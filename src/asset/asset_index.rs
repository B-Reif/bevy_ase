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
    pub(crate) animations: HashMap<String, Vec<Handle<Animation>>>,
    pub(crate) slices: HashMap<String, Vec<Handle<Slice>>>,
    pub(crate) tilesets: HashMap<String, Vec<Handle<Tileset>>>,
    pub(crate) textures: HashMap<u32, Handle<Image>>,
}
impl AseAssetMap {
    /// Returns all animations with the given tag name.
    pub fn animations(&self, tag_name: &str) -> Option<&Vec<Handle<Animation>>> {
        self.animations.get(tag_name)
    }
    /// Returns all slices with the given name.
    pub fn slices(&self, slice_name: &str) -> Option<&Vec<Handle<Slice>>> {
        self.slices.get(slice_name)
    }
    /// Returns all tilesets with the given name.
    pub fn tilesets(&self, tileset_name: &str) -> Option<&Vec<Handle<Tileset>>> {
        self.tilesets.get(tileset_name)
    }
    /// Returns the texture for the given frame index.
    pub fn texture(&self, frame_index: u32) -> Option<&Handle<Image>> {
        self.textures.get(&frame_index)
    }

    // Insert API
    pub(crate) fn insert_animation(&mut self, tag_name: String, handle: Handle<Animation>) {
        let anims = self.animations.entry(tag_name).or_default();
        anims.push(handle);
    }
    pub(crate) fn insert_tileset(&mut self, tileset_name: String, handle: Handle<Tileset>) {
        let tilesets = self.tilesets.entry(tileset_name).or_default();
        tilesets.push(handle);
    }
    pub(crate) fn insert_slice(&mut self, slice_name: String, handle: Handle<Slice>) {
        let slices = self.slices.entry(slice_name).or_default();
        slices.push(handle);
    }
    pub(crate) fn insert_texture(&mut self, frame_index: u32, handle: Handle<Image>) {
        self.textures.insert(frame_index, handle);
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
    pub(crate) fn get_mut(&mut self, path: PathBuf) -> &mut AseAssetMap {
        let entry = self.0.entry(path);
        entry.or_default()
    }
    /// Returns the first animation in an Ase file with the given tag name.
    pub fn animation(&self, path: &Path, tag_name: &str) -> Option<Handle<Animation>> {
        self.get(path)?.animations(tag_name).and_then(clone_first)
    }
    /// Returns the first slice in an Ase file with the given name.
    pub fn slice(&self, path: &Path, slice_name: &str) -> Option<Handle<Slice>> {
        self.get(path)?.slices(slice_name).and_then(clone_first)
    }
    /// Returns the first tileset in an Ase file with the given name.
    pub fn tileset(&self, path: &Path, tileset_name: &str) -> Option<Handle<Tileset>> {
        self.get(path)?.tilesets(tileset_name).and_then(clone_first)
    }
}
