use crate::animate::{Animation, AnimationInfo};
use crate::processing::{self};
use crate::tileset::Tileset;
use asefile::AsepriteFile;
use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadState, LoadedAsset},
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

pub struct AsepriteLoaderPlugin;

impl Plugin for AsepriteLoaderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<AsepriteLoader>()
            .add_asset::<Tileset>()
            .add_asset::<AsepriteAsset>()
            .init_asset_loader::<AsepriteAssetLoader>()
            .add_system(aseprite_loader.system());
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "053511cb-7843-47a3-b5b6-c3279dc7cf6f"]
pub struct AsepriteAsset {
    data: LoadedAsepriteFile,
    name: PathBuf,
}

#[derive(Debug)]
pub enum LoadedAsepriteFile {
    Loaded(AsepriteFile),
    Processed,
}

#[derive(Default)]
pub struct AsepriteAssetLoader;

impl AssetLoader for AsepriteAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            debug!("Loading/parsing asefile: {}", load_context.path().display());
            let ase = AsepriteAsset {
                data: LoadedAsepriteFile::Loaded(AsepriteFile::read(bytes)?),
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

// #[derive(Debug)]
pub struct AsepriteLoader {
    todo_handles: Vec<Handle<AsepriteAsset>>,
    done: Arc<Mutex<Vec<processing::AseAssets>>>,
    in_progress: Arc<AtomicU32>,
}

impl Default for AsepriteLoader {
    fn default() -> Self {
        AsepriteLoader {
            todo_handles: Vec::new(),
            in_progress: Arc::new(AtomicU32::new(0)),
            done: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl AsepriteLoader {
    pub fn add(&mut self, handle: Handle<AsepriteAsset>) {
        self.todo_handles.push(handle);
    }

    fn all_todo_handles_ready(&self, asset_server: &AssetServer) -> bool {
        if let LoadState::Loaded =
            asset_server.get_group_load_state(self.todo_handles.iter().map(|handle| handle.id))
        {
            true
        } else {
            false
        }
    }

    fn spawn_tasks(&mut self, pool: &AsyncComputeTaskPool, aseprites: &mut Assets<AsepriteAsset>) {
        if self.todo_handles.is_empty() {
            return;
        }

        let in_progress = self.in_progress.clone();
        in_progress.fetch_add(1, Ordering::SeqCst);

        let mut handles = Vec::new();
        std::mem::swap(&mut handles, &mut self.todo_handles);

        let mut inputs: Vec<(PathBuf, AsepriteFile)> = Vec::with_capacity(handles.len());
        for h in &handles {
            let ase_asset = aseprites
                .get_mut(h.clone_weak())
                .expect("Failed to get aseprite from handle");

            // We actually remove the AsepriteFile from the AsepriteAsset so
            // the memory can be freed after we're done processing. If the file
            // was changed we get the new data from the asset loader.
            //
            // TODO: Add support for changed-on disk events.
            let mut loaded_ase = LoadedAsepriteFile::Processed;
            std::mem::swap(&mut ase_asset.data, &mut loaded_ase);

            if let LoadedAsepriteFile::Loaded(ase) = loaded_ase {
                inputs.push((ase_asset.name.clone(), ase));
            }
        }

        let output = self.done.clone();
        let task = pool.spawn(async move {
            let processed = processing::AseAssets::new(inputs);
            let mut out = output.lock().expect("Failed to get lock");
            out.push(processed);
        });
        task.detach();
    }

    fn process_finished(&mut self, mut resources: AseAssetResources) {
        let results = {
            let mut lock = self.done.try_lock();
            if let Ok(ref mut data) = lock {
                let mut results = Vec::new();
                std::mem::swap(&mut results, &mut *data);
                results
            } else {
                return;
            }
        };
        if results.is_empty() {
            return;
        }
        for r in results {
            r.move_into_bevy(&mut resources);
            self.in_progress.fetch_sub(1, Ordering::SeqCst);
        }
    }

    pub fn check_pending(&self) -> u32 {
        self.in_progress.load(Ordering::SeqCst)
    }

    pub fn is_loaded(&self) -> bool {
        self.todo_handles.is_empty() && self.check_pending() == 0
    }
}

pub(crate) struct AseAssetResources<'a> {
    pub animations: Option<&'a mut Assets<Animation>>,
    pub anim_info: Option<&'a mut AnimationInfo>,
    pub textures: &'a mut Assets<Texture>,
    pub atlases: &'a mut Assets<TextureAtlas>,
    pub tilesets: &'a mut Assets<Tileset>,
}

pub fn aseprite_loader(
    mut loader: ResMut<AsepriteLoader>,
    task_pool: ResMut<AsyncComputeTaskPool>,
    mut aseassets: ResMut<Assets<AsepriteAsset>>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut animations: Option<ResMut<Assets<Animation>>>,
    mut anim_info: Option<ResMut<AnimationInfo>>,
    mut tilesets: ResMut<Assets<Tileset>>,
) {
    let pending = loader.check_pending();
    if pending > 0 {
        debug!("Processing asefiles (batches: {})", pending);
    }
    if loader.all_todo_handles_ready(&asset_server) {
        loader.spawn_tasks(&task_pool, &mut aseassets);
    }
    let animations = animations.as_mut().map(DerefMut::deref_mut);
    let anim_info = anim_info.as_mut().map(DerefMut::deref_mut);
    let resources = AseAssetResources {
        anim_info,
        animations,
        textures: &mut textures,
        atlases: &mut atlases,
        tilesets: &mut tilesets,
    };
    loader.process_finished(resources);
}
