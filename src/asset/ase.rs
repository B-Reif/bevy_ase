use asefile::AsepriteFile;
use bevy::reflect::TypeUuid;
use std::path::PathBuf;

/// Handle type for ase assets.
///
/// [crate::loader::Loader] processes [AseAsset] instances and stores their data
/// as various other data types in bevy's Assets resources.
///
/// Once an AseAsset has been processed into other resource types, its data is dropped.
///
/// # Examples
///
/// ```
/// use bevy::prelude::*;
/// use bevy_ase::asset::AseAsset;
///
/// // Convert an untyped handle into an AseAsset handle.
/// pub fn to_typed(handle: HandleUntyped) -> Handle<AseAsset> {
///    handle.clone().typed::<AseAsset>()
/// }
/// ```
#[derive(Debug, TypeUuid)]
#[uuid = "053511cb-7843-47a3-b5b6-c3279dc7cf6f"]
pub struct AseAsset {
    pub(crate) data: AseData,
    pub(crate) name: PathBuf,
}
impl AseAsset {
    /// Returns a reference to the asset's file data, if this asset has not yet been processed.
    pub fn file(&self) -> Option<&AsepriteFile> {
        if let AseData::Loaded(file) = &self.data {
            Some(file)
        } else {
            None
        }
    }
}

/// Contains Aseprite file data before processing.
///
/// During processing, Loaded data is moved into other asset types.
/// Afterward, the Loaded instances are replaced with Processed instances.
#[derive(Debug)]
pub(crate) enum AseData {
    Loaded(AsepriteFile),
    Processed,
}
