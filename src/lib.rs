#![warn(missing_docs)]
//! Utilities for loading [`Aseprite`] files directly into a [`Bevy`] application.
//!
//! Provides an [`AssetLoader`] which reads .aseprite files directly into memory,
//! without an intermediate import step. The loader adds [`Resources`] generated
//! by the files' data. The added resources include several types:
//!
//! - [`Texture`] data, which contains the file's images.
//! - [Animation] data.
//! - [Tileset] data (from files created in Aseprite v1.3 beta).
//!
//! [`Texture`]: https://docs.rs/bevy/0.5.0/bevy/render/texture/index.html
//! [`AssetLoader`]: https://docs.rs/bevy/0.5.0/bevy/asset/trait.AssetLoader.html
//! [`Bevy`]: https://bevyengine.org/
//! [`Aseprite`]: https://www.aseprite.org/
//! [`Resources`]: https://bevyengine.org/learn/book/getting-started/resources/
pub mod animate;
mod animation;
pub mod animation_index;
mod ase;
pub mod loader;
mod processing;
pub mod slice;
mod sprite;
#[cfg(test)]
mod tests;
mod tileset;
pub mod timer;

pub use animation::{Animation, Frame};
pub use ase::AseId;
pub use asefile::UserData;
pub use tileset::{TileSize, Tileset, TilesetAseKey, TilesetId};
