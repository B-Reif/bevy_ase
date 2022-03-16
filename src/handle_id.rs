//! Provides functions to generate [HandleId](bevy::asset::HandleId)s for assets generated from aseprite files.
//!
//! This crate uses asset labels to identify generated sub-assets. The following label forms are supported:
//!
//! | Label                      | Asset type                                                                                    |
//! | -------------------------- | ------------------------------------------------------------------------  |
//! | `Animation/{tag_name}`     | [`Animation`][crate::asset::animation::Animation] with a given tag name*. |
//! | `Atlas`                    | [`TextureAtlas`][bevy::prelude::TextureAtlas] for the entire sprite.      |
//! | `FrameImage{index}`        | [`Image`][bevy::prelude::Image] for a given frame index.                  |
//! | `Slice/{name}`             | [`Slice`][crate::asset::slice::Slice] with a given slice name*.           |
//! | `Tileset{tileset_id}`      | [`Tileset`][crate::asset::tileset::Tileset] for a given tileset id.       |
//! | `TilesetImage{tileset_id}` | [`Image`][bevy::prelude::Image]  for the tileset with the given id.       |
//!
//! # Examples
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_ase::handle_id;
//!
//! fn get_frame_image(image_assets: Res<Assets<Image>>) {
//!     let my_file_path = "assets/my_file.aseprite";
//!
//!     // Use `handle_id` functions to generate a path to a sub-asset:
//!     let frame_image_handle_id = handle_id::frame_image(my_file_path, 0);
//!
//!     // Or use a path with a label:
//!     assert_eq!(
//!         frame_image_handle_id,
//!         "assets/my_file.aseprite#FrameImage0".into()
//!     );
//!
//!     let my_image_asset: Option<&Image> = image_assets.get(frame_image_handle_id);
//! }
//! ```
//!
//! # * Warning!
//!
//! This crate keys slices and animations by name. When using slices and animations data with this crate,
//! ensure that they have unique names within the same file. If there is a name collision, only one of the named
//! assets will be set, and any other slice or animation assets by the same name will be dropped!
//!
use bevy::asset::AssetPath;
use bevy::asset::HandleId;
use std::path::PathBuf;

fn make(path: &str, kind: &str, suffix: Option<&str>) -> HandleId {
    let mut label = kind.to_string();
    if let Some(suffix) = suffix {
        label.push_str(suffix);
    }
    let asset_path = AssetPath::new(PathBuf::from(path), Some(label));
    asset_path.into()
}

/// Makes a `HandleId` for an [`Animation`][crate::asset::animation::Animation].
///
/// The path of each tag's [`Animation`][crate::asset::animation::Animation]
/// takes the form `{file_path}#Animation/{tag_name}`.
///
/// For files with multiple tags sharing the same name,
/// only one animation asset by that name will be stored.
/// Other animations by the same tag name are dropped.
/// When using animations with this crate, make sure each tag in a file has a unique name.
///
/// # Examples
/// ```
/// use bevy_ase::handle_id;
///
/// let my_file_path = "assets/my_ase_file.aseprite";
/// let tag_name = "my_tag";
///
/// assert_eq!(
///     handle_id::animation(my_file_path, tag_name),
///     "assets/my_ase_file.aseprite#Animation/my_tag".into()
/// );
/// ```
pub fn animation(path: &str, tag_name: &str) -> HandleId {
    make(path, "Animation/", Some(tag_name))
}

/// Makes a `HandleId` for a frame's [`Image`][bevy::prelude::Image].
///
/// The path of each frame's [`Image`][bevy::prelude::Image]
/// takes the form `{file_path}#FrameImage{index}`.
///
/// # Examples
/// ```
/// use bevy_ase::handle_id;
///
/// let my_file_path = "assets/my_ase_file.aseprite";
/// let frame: u32 = 2;
///
/// assert_eq!(
///   handle_id::frame_image(my_file_path, frame),
///   "assets/my_ase_file.aseprite#FrameImage2".into()
/// );
/// ```
pub fn frame_image(path: &str, frame: u32) -> HandleId {
    make(path, "FrameImage", Some(&frame.to_string()))
}

/// Makes a `HandleId` for a [`TextureAtlas`][bevy::prelude::TextureAtlas].
///
/// The path of the [`TextureAtlas`][bevy::prelude::TextureAtlas] takes the form `{file_path}#Atlas`.
///
/// # Examples
/// ```
/// use bevy_ase::handle_id;
///
/// let my_file_path = "assets/my_ase_file.aseprite";
///
/// assert_eq!(
///   handle_id::atlas(my_file_path),
///   "assets/my_ase_file.aseprite#Atlas".into()
/// );
/// ```
pub fn atlas(path: &str) -> HandleId {
    make(path, "Atlas", None)
}

/// Makes a `HandleId` for a [`Tileset`][crate::asset::Tileset].
///
/// The path of each [`Tileset`][crate::asset::Tileset]
/// takes the form `{file_path}#Tileset{tileset_id}`.
///
/// # Examples
/// ```
/// use bevy_ase::handle_id;
///
/// let my_file_path = "assets/my_ase_file.aseprite";
/// let my_tileset_id: u32 = 5;
///
/// assert_eq!(
///   handle_id::tileset(my_file_path, my_tileset_id),
///   "assets/my_ase_file.aseprite#Tileset5".into()
/// );
/// ```
pub fn tileset(path: &str, tileset_id: u32) -> HandleId {
    make(path, "Tileset", Some(&tileset_id.to_string()))
}

/// Makes a `HandleId` for a tileset [`Image`][bevy::prelude::Image].
///
/// The path of each tileset's [`Image`][bevy::prelude::Image]
/// takes the form `{file_path}#TilesetImage{tileset_id}`.
///
/// # Examples
/// ```
/// use bevy_ase::handle_id;
///
/// let my_file_path = "assets/my_ase_file.aseprite";
/// let tileset_id: u32 = 1;
///
/// assert_eq!(
///   handle_id::tileset_image(my_file_path, tileset_id),
///   "assets/my_ase_file.aseprite#TilesetImage1".into()
/// );
/// ```
pub fn tileset_image(path: &str, tileset_id: u32) -> HandleId {
    make(path, "TilesetImage", Some(&tileset_id.to_string()))
}

/// Makes a `HandleId` for a [`Slice`][crate::asset::slice::Slice].
///
/// The path of each [`Slice`][crate::asset::slice::Slice]
/// takes the form `{file_path}#Slice/{slice_name}`.
///
/// For files with multiple slices sharing the same name,
/// only one slice asset by that name will be stored.
/// Other slices by the same name are dropped.
/// When using slices with this crate, make sure each slice in a file has a unique name.
///
/// # Examples
/// ```
/// use bevy_ase::handle_id;
///
/// let my_file_path = "assets/my_ase_file.aseprite";
/// let slice_name = "my_slice";
///
/// assert_eq!(
///     handle_id::slice(my_file_path, slice_name),
///     "assets/my_ase_file.aseprite#Slice/my_slice".into()
/// );
/// ```
pub fn slice(path: &str, name: &str) -> HandleId {
    make(path, "Slice/", Some(name))
}
