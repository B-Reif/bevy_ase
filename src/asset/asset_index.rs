//! Index for assets created by this library.
use super::{animation::Animation, slice::Slice, tileset::Tileset};
use bevy::utils::HashMap;
use bevy::{asset::Asset, prelude::*};
use std::path::{Path, PathBuf};

/// Provides a map to [Handles](Handle) for an Ase file's assets.
///
/// Instances of this type are owned by [AseFileMap]. To access them during runtime,
/// use the AseFileMap as a system parameter, and index each AseAssetMap by
/// using the file's path as a key.
///
/// # Examples
///
/// ```
/// use bevy_ase::asset::{AseAssetMap, Animation, Tileset};
/// use bevy::asset::Handle;
/// // Get all animations in this file with the name "foo".
/// fn get_foo_animations(ase_asset_map: &AseAssetMap) -> Option<&Vec<Handle<Animation>>> {
///     ase_asset_map.animations("foo")
/// }
///
/// // Get the first tileset in this file with the name "bar".
/// fn get_bar_tileset(ase_asset_map: &AseAssetMap) -> Option<Handle<Tileset>> {
///     ase_asset_map.tilesets("foo")?.first().map(Handle::clone)
/// }
/// ```
///
/// # Notes
///
/// The owning AseFileMap instance provides convenience methods to index a file
/// and an asset simultaneously. These methods also clone the Handle value before returning.
///
/// [Image](bevy::render::texture::Image) assets are mapped to their frame index. This map does not include Images
/// rendered from [Tileset] assets. To access a Tileset's Image, use the texture field
/// on a tileset asset.
///
/// [Animation], [Slice], and Tileset assets are mapped to their string name. There may be
/// more than one asset with the same name. If just one asset is expected,
/// compose the result with `first()`.
#[derive(Default, Debug)]
pub struct AseAssetMap {
    pub(crate) animations: HashMap<String, Handle<Animation>>,
    pub(crate) slices: HashMap<String, Handle<Slice>>,
    pub(crate) tilesets: HashMap<u32, Handle<Tileset>>,
    pub(crate) textures: HashMap<u32, Handle<Image>>,
    pub(crate) atlas: Handle<TextureAtlas>,
}
impl AseAssetMap {
    /// Returns the animation with the given tag name.
    pub fn animations(&self, tag_name: &str) -> Option<&Handle<Animation>> {
        self.animations.get(tag_name)
    }
    /// Returns the slice with the given name.
    pub fn slices(&self, slice_name: &str) -> Option<&Handle<Slice>> {
        self.slices.get(slice_name)
    }
    /// Returns the tileset with the given id.
    pub fn tilesets(&self, tileset_id: u32) -> Option<&Handle<Tileset>> {
        self.tilesets.get(&tileset_id)
    }
    /// Returns the texture for the given frame index.
    pub fn texture(&self, frame_index: u32) -> Option<&Handle<Image>> {
        self.textures.get(&frame_index)
    }
    /// Returns the texture atlas for the file.
    pub fn atlas(&self) -> &Handle<TextureAtlas> {
        &self.atlas
    }

    // Insert API
    pub(crate) fn insert_animation(&mut self, tag_name: String, handle: Handle<Animation>) {
        self.animations.insert(tag_name, handle);
    }
    pub(crate) fn insert_tileset(&mut self, tileset_id: u32, handle: Handle<Tileset>) {
        self.tilesets.insert(tileset_id, handle);
    }
    pub(crate) fn insert_slice(&mut self, slice_name: String, handle: Handle<Slice>) {
        self.slices.insert(slice_name, handle);
    }
    pub(crate) fn insert_texture(&mut self, frame_index: u32, handle: Handle<Image>) {
        self.textures.insert(frame_index, handle);
    }
    pub(crate) fn insert_atlas(&mut self, handle: Handle<TextureAtlas>) {
        self.atlas = handle;
    }
}

#[allow(clippy::ptr_arg)]
fn clone_first<T: Asset>(vec: &Vec<Handle<T>>) -> Option<Handle<T>> {
    vec.first().map(Handle::clone)
}

/// Resource type. Provides map access to Ase asset [Handles](Handle),
/// keyed by the Ase file's path.
///
/// Internally maps [PathBuf] keys to [AseAssetMap] values.
/// Each asset map stores [Handle] values for that file's assets.
///
/// # Examples
///
/// ```
/// use std::path::Path;
/// use bevy::prelude::*;
/// use bevy_ase::asset::{Animation, AseFileMap, AseAssetMap};
///
/// // Use access methods to index both the file and an asset, and get a new handle.
/// fn get_foo_bar(ase_file_map: AseFileMap) -> Option<Handle<Animation>> {
///     ase_file_map.animation(Path::new("sprites/foo.aseprite"), "bar")
/// }
///
/// // Or compose with [AseAssetMap] methods to get individual assets.
/// // This is equivalent to the above:
/// fn get_foo_bar_long(ase_file_map: AseFileMap) -> Option<Handle<Animation>> {
///     ase_file_map
///         .get(Path::new("sprites/foo.aseprite"))?
///         .animations("bar")?
///         .first()
///         .map(Handle::clone)
/// }
///
/// ```

#[derive(Default, Debug)]
pub struct AseFileMap(pub(crate) HashMap<PathBuf, AseAssetMap>);
impl AseFileMap {
    /// Returns the asset map for the file with the given path.
    pub fn get(&self, path: &Path) -> Option<&AseAssetMap> {
        self.0.get(path)
    }
    pub(crate) fn get_mut(&mut self, path: &Path) -> &mut AseAssetMap {
        let entry = self.0.entry(path.to_path_buf());
        entry.or_default()
    }
    /// Returns the first animation in an Ase file with the given tag name.
    pub fn animation(&self, path: &Path, tag_name: &str) -> Option<Handle<Animation>> {
        self.get(path)?.animations(tag_name).cloned()
    }
    /// Returns the first slice in an Ase file with the given name.
    pub fn slice(&self, path: &Path, slice_name: &str) -> Option<Handle<Slice>> {
        self.get(path)?.slices(slice_name).cloned()
    }
    /// Returns the first tileset in an Ase file with the given name.
    pub fn tileset(&self, path: &Path, tileset_id: u32) -> Option<Handle<Tileset>> {
        self.get(path)?.tilesets(tileset_id).cloned()
    }
}
