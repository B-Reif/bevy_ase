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

/// Provides asset types for working with Aseprite data.
pub mod asset;
/// Provides systems and resources for loading Aseprite files.
///
/// The default loader configuration provided by [AseLoaderDefaultPlugin] contains
/// asset types and processing for all Aseprite data types provided by this library.
pub mod loader;
mod processing;
mod sprite;
#[cfg(test)]
mod tests;

pub use asefile::UserData;
