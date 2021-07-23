use crate::animate::AnimationInfo;
use crate::animation::Animation;
use crate::processing::{self, ResourceData};
use crate::slice::Slice;
use crate::tileset::Tileset;
use asefile::AsepriteFile;
use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadState, LoadedAsset},
    ecs::system::Res,
    prelude::*,
    reflect::TypeUuid,
    tasks::AsyncComputeTaskPool,
};
use std::ops::DerefMut;
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
};

pub struct AseLoaderPlugin;

impl Plugin for AseLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Loader>()
            .add_asset::<AseAsset>()
            .add_asset::<Tileset>()
            .init_asset_loader::<AseAssetLoader>()
            .add_system(ase_importer.system());
    }
}

/// Handle type for ase assets. Used to add assets to [Loader].
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_ase::loader::AseAsset;
///
/// // Convert an untyped handle into an AseAsset handle.
/// pub fn to_typed(handle: HandleUntyped) -> Handle<AseAsset> {
///    handle.clone().typed::<AseAsset>()
/// }
/// ```
#[derive(Debug, TypeUuid)]
#[uuid = "053511cb-7843-47a3-b5b6-c3279dc7cf6f"]
pub struct AseAsset {
    data: AseData,
    name: PathBuf,
}

#[derive(Debug)]
pub enum AseData {
    Loaded(AsepriteFile),
    Processed,
}

#[derive(Default)]
pub struct AseAssetLoader;

impl AssetLoader for AseAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            debug!("Loading/parsing asefile: {}", load_context.path().display());
            let ase = AseAsset {
                data: AseData::Loaded(AsepriteFile::read(bytes)?),
                name: load_context.path().to_owned(),
            };
            load_context.set_default_asset(LoadedAsset::new(ase));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["aseprite", "ase"]
    }
}
/// Provides methods for loading [AseAsset].
///
/// The [AseLoaderPlugin] adds this as a resource by default.
/// To load Aseprite files, or check their loading status, a system can accept the [Loader] as a parameter.
///
/// # Examples
///
/// ```
/// // Adds a Loader instance to the app's resources.
/// // The AseLoaderPlugin already does this by default.
/// fn build(&self, app: &mut AppBuilder) {
///     app.init_resource::<Loader>();
/// }
/// ```

pub struct Loader {
    todo_handles: Vec<Handle<AseAsset>>,
    in_progress: Arc<AtomicU32>,
    done: Arc<Mutex<Vec<processing::ResourceData>>>,
}

impl Default for Loader {
    fn default() -> Self {
        Self {
            todo_handles: Vec::new(),
            in_progress: Arc::new(AtomicU32::new(0)),
            done: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Loader {
    /// Adds an [AseAsset] to the [Loader] for loading.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_ase::loader::{AseAsset, Loader};
    /// use std::path::Path;
    ///
    /// // System function which sends ase assets in the "sprites" folder to the loader.
    /// pub fn load_sprites(asset_server: Res<AssetServer>, mut aseloader: ResMut<Loader>) {
    ///     let handles = asset_server.load_folder(Path::new("sprites")).unwrap();
    ///     for h in &handles {
    ///         aseloader.add(h.clone().typed::<AseAsset>());
    ///     }
    /// }
    /// ```
    pub fn add(&mut self, handle: Handle<AseAsset>) {
        self.todo_handles.push(handle);
    }

    /// Returns the number of [AseAsset] handles currently being processed.
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_ase::loader::Loader;
    /// // System function which checks how many assets are processing.
    /// pub fn check_loading_sprites(ase_loader: Res<Loader>) {
    ///     info!("{} ase assets currently processing", ase_loader.pending_count());
    /// }
    /// ```
    pub fn pending_count(&self) -> u32 {
        self.in_progress.load(Ordering::SeqCst)
    }

    /// Returns true when no [AseAsset] handles are loading or being processed.
    pub fn is_loaded(&self) -> bool {
        self.todo_handles.is_empty() && self.pending_count() == 0
    }

    fn all_todo_handles_ready(&self, asset_server: &AssetServer) -> bool {
        let handles = self.todo_handles.iter().map(|h| h.id);
        asset_server.get_group_load_state(handles) == LoadState::Loaded
    }

    fn spawn_tasks(&mut self, pool: &AsyncComputeTaskPool, aseprites: &mut Assets<AseAsset>) {
        if self.todo_handles.is_empty() {
            return;
        }

        let in_progress = self.in_progress.clone();
        in_progress.fetch_add(1, Ordering::SeqCst);

        let mut handles = Vec::new();
        std::mem::swap(&mut handles, &mut self.todo_handles);

        let mut ase_files: Vec<(PathBuf, AsepriteFile)> = Vec::with_capacity(handles.len());
        for h in &handles {
            let ase_asset = aseprites
                .get_mut(h.clone_weak())
                .expect("Failed to get aseprite from handle");

            // We actually remove the AsepriteFile from the AsepriteAsset so
            // the memory can be freed after we're done processing. If the file
            // was changed we get the new data from the asset loader.
            //
            // TODO: Add support for changed-on disk events.
            let mut loaded_ase = AseData::Processed;
            std::mem::swap(&mut ase_asset.data, &mut loaded_ase);

            if let AseData::Loaded(ase) = loaded_ase {
                ase_files.push((ase_asset.name.clone(), ase));
            }
        }

        let output = self.done.clone();
        let task = pool.spawn(async move {
            let processed = processing::ResourceData::new(ase_files);
            let mut out = output.lock().expect("Failed to get lock");
            out.push(processed);
        });
        task.detach();
    }

    fn take_finished(&mut self) -> Option<Vec<ResourceData>> {
        let results = {
            let mut lock = self.done.try_lock();
            if let Ok(ref mut data) = lock {
                let mut results = Vec::new();
                std::mem::swap(&mut results, &mut *data);
                results
            } else {
                return None;
            }
        };
        if results.is_empty() {
            return None;
        }
        Some(results)
    }

    fn move_finished_into_resources(&mut self, mut resources: AseAssetResources) {
        if let Some(finished) = self.take_finished() {
            for ase in finished {
                ase.move_into_resources(&mut resources);
                self.in_progress.fetch_sub(1, Ordering::SeqCst);
            }
        }
    }
}

pub(crate) struct AseAssetResources<'a> {
    pub animations: Option<&'a mut Assets<Animation>>,
    pub anim_info: Option<&'a mut AnimationInfo>,
    pub textures: Option<&'a mut Assets<Texture>>,
    pub atlases: Option<&'a mut Assets<TextureAtlas>>,
    pub tilesets: Option<&'a mut Assets<Tileset>>,
    pub slices: Option<&'a mut Assets<Slice>>,
}

/// System function for moving loaded Aseprite assets into Resoures.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_ase::loader::ase_importer;
///
/// // Creates a Bevy app and adds the ase_importer system.
/// // This system is already added by default in AseLoaderPlugin.
/// fn main() {
///     App::build().add_system(ase_importer.system());
/// }
/// ```
pub fn ase_importer(
    mut loader: ResMut<Loader>,
    task_pool: ResMut<AsyncComputeTaskPool>,
    mut aseassets: ResMut<Assets<AseAsset>>,
    asset_server: Res<AssetServer>,
    mut textures: Option<ResMut<Assets<Texture>>>,
    mut atlases: Option<ResMut<Assets<TextureAtlas>>>,
    mut animations: Option<ResMut<Assets<Animation>>>,
    mut anim_info: Option<ResMut<AnimationInfo>>,
    mut tilesets: Option<ResMut<Assets<Tileset>>>,
    mut slices: Option<ResMut<Assets<Slice>>>,
) {
    let pending = loader.pending_count();
    if pending > 0 {
        debug!("Processing asefiles (batches: {})", pending);
    }
    if loader.all_todo_handles_ready(&asset_server) {
        loader.spawn_tasks(&task_pool, &mut aseassets);
    }
    let textures = textures.as_mut().map(DerefMut::deref_mut);
    let atlases = atlases.as_mut().map(DerefMut::deref_mut);
    let animations = animations.as_mut().map(DerefMut::deref_mut);
    let anim_info = anim_info.as_mut().map(DerefMut::deref_mut);
    let tilesets = tilesets.as_mut().map(DerefMut::deref_mut);
    let slices = slices.as_mut().map(DerefMut::deref_mut);
    let resources = AseAssetResources {
        animations,
        anim_info,
        textures,
        atlases,
        tilesets,
        slices,
    };
    loader.move_finished_into_resources(resources);
}
