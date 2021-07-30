//! Index for assets created by this library.
use super::{animation::Animation, slice::Slice, tileset::Tileset};
use bevy::utils::HashMap;
use bevy::{asset::Asset, prelude::*};
use std::path::{Path, PathBuf};

/// Provides map access to an Ase file's [Animation], [Slice], and [Tileset] assets,
/// using their string names as keys.
///
/// Internally maps [String] keys to [Vec] values of each asset type.
/// There may be more than one asset with the same name.
/// If just one asset is expected, compose the result with `first()`.
///
/// # Examples
///
/// ```
/// use bevy_ase::asset::{AseAssetMap, Animation};
/// use bevy::asset::Handle;
/// // Get the first animation in this file with the name "foo".
/// fn get_foo(ase_asset_map: AseAssetMap) -> Option<Handle<Animation>> {
///     ase_asset_map.animations("foo")?.first().map(Handle::clone)
/// }
/// ```
#[derive(Default)]
pub struct AseAssetMap {
    pub(crate) animations: HashMap<String, Vec<Handle<Animation>>>,
    pub(crate) slices: HashMap<String, Vec<Handle<Slice>>>,
    pub(crate) tilesets: HashMap<String, Vec<Handle<Tileset>>>,
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

    // Insert API
    pub(crate) fn insert_animation(&mut self, tag_name: String, handle: Handle<Animation>) {
        let anims = self.animations.entry(tag_name).or_default();
        anims.push(handle);
    }
    pub(crate) fn insert_tileset(&mut self, tileset_name: String, handle: Handle<Tileset>) {
        let tilesets = self.tilesets.entry(tileset_name).or_default();
        tilesets.push(handle);
    }
}

#[allow(clippy::ptr_arg)]
fn clone_first<T: Asset>(vec: &Vec<Handle<T>>) -> Option<Handle<T>> {
    vec.first().map(Handle::clone)
}

/// Provides map access to Ase assets, keyed by the Ase file's path.
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
/// // Compose with [AseAssetMap] methods to get individual assets:
/// fn get_foo_bar(ase_file_map: AseFileMap) -> Option<Handle<Animation>> {
///     ase_file_map
///         .get(Path::new("sprites/foo.aseprite"))?
///         .animations("bar")?
///         .first()
///         .map(Handle::clone)
/// }
///
/// // Or use a shortcut method to index both the file and the asset, and then clone the [Handle].
/// // This is equivalent to the above:
/// fn get_foo_bar_short(ase_file_map: AseFileMap) -> Option<Handle<Animation>> {
///     ase_file_map.animation(Path::new("sprites/foo.aseprite"), "bar")
/// }
/// ```
pub struct AseFileMap(pub(crate) HashMap<PathBuf, AseAssetMap>);
impl AseFileMap {
    /// Returns the asset map for the file with the given path.
    pub fn get(&self, path: &Path) -> Option<&AseAssetMap> {
        self.0.get(path)
    }
    pub(crate) fn get_mut(&mut self, path: &Path) -> Option<&mut AseAssetMap> {
        self.0.get_mut(path)
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
