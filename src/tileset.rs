use asefile::{AsepriteFile, TilesetImageError};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};
use std::{collections::HashMap, fmt, iter::FromIterator};

use crate::ase::AseId;

pub(crate) type TilesetResult<T> = std::result::Result<T, TilesetError>;

#[derive(Debug)]
pub enum TilesetError {
    MissingId(TilesetId),
    NoPixels(TilesetId),
}
impl fmt::Display for TilesetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TilesetError::MissingId(tileset_id) => {
                write!(f, "No tileset found with id: {}", tileset_id)
            }
            TilesetError::NoPixels(tileset_id) => {
                write!(f, "No pixel data for tileset with id: {}", tileset_id)
            }
        }
    }
}
impl From<&TilesetImageError> for TilesetError {
    fn from(e: &TilesetImageError) -> Self {
        match e {
            TilesetImageError::MissingTilesetId(id) => Self::MissingId(id.into()),
            TilesetImageError::NoPixelsInTileset(id) => Self::NoPixels(id.into()),
        }
    }
}
impl From<TilesetImageError> for TilesetError {
    fn from(e: TilesetImageError) -> Self {
        Self::from(&e)
    }
}

fn texture_from(ase: &AsepriteFile, tileset: &asefile::Tileset) -> TilesetResult<Texture> {
    let tileset_id = tileset.id();
    let image = ase.tileset_image(&tileset_id)?;
    let size = Extent3d {
        width: image.width(),
        height: image.height(),
        depth: 1,
    };
    Ok(Texture::new_fill(
        size,
        TextureDimension::D2,
        image.as_raw(),
        TextureFormat::Rgba8UnormSrgb,
    ))
}

/// Identifier for a [Tileset] within an Aseprite file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TilesetId(u32);
impl TilesetId {
    /// Creates a new [TilesetId] from an inner u32 value.
    pub fn new(inner: u32) -> Self {
        Self(inner)
    }
    /// Returns a reference to the id's inner u32 value.
    pub fn inner(&self) -> &u32 {
        &self.0
    }
}
impl From<&asefile::TilesetId> for TilesetId {
    fn from(ase_id: &asefile::TilesetId) -> Self {
        Self(*ase_id.value())
    }
}
impl fmt::Display for TilesetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TilesetId({})", self.0)
    }
}

/// Unique identifier for a [Tileset] with an [AseId] and a [TilesetId].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TilesetAseKey {
    ase_id: AseId,
    tileset_id: TilesetId,
}
impl TilesetAseKey {
    /// Creates a new [TilesetAseKey] from an [AseId] and a [TilesetId].
    pub fn new(ase_id: AseId, tileset_id: TilesetId) -> Self {
        Self { ase_id, tileset_id }
    }
    /// Returns a reference to the key's [AseId].
    pub fn ase_id(&self) -> &AseId {
        &self.ase_id
    }
    /// Returns a reference to the key's [TilesetId].
    pub fn tileset_id(&self) -> &TilesetId {
        &self.tileset_id
    }
}
impl fmt::Display for TilesetAseKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TilesetKey({}, {})", self.ase_id, self.tileset_id)
    }
}

/// Width and height of a tile in pixels.
#[derive(Debug)]
pub struct TileSize {
    pub width: u16,
    pub height: u16,
}
impl TileSize {
    fn from_ase(ase_size: &asefile::TileSize) -> Self {
        Self {
            width: *ase_size.width(),
            height: *ase_size.height(),
        }
    }
}

/// Data and texture from an Aseprite tileset.
#[derive(Debug, TypeUuid)]
#[uuid = "0e2dbd05-dbad-46c9-a943-395f83dfa4ba"]
pub struct Tileset {
    /// A unique identifier for this tileset.
    pub key: TilesetAseKey,
    /// Number of tiles in this tilset.
    pub tile_count: u32,
    /// Pixel size of this tileset's tiles.
    pub tile_size: TileSize,
    /// Name of this tileset.
    pub name: String,
    /// A handle to the tileset's texture. See also the [`Self::texture_size()`] method.
    pub texture: Handle<Texture>,
}
impl Tileset {
    /// Returns the size of the [Tileset]'s texture.
    /// This has width = tile_size.width and height = tile_size.height * tile_count
    /// (e.g. all tiles are stored in a vertical strip).
    pub fn texture_size(&self) -> Vec2 {
        let TileSize { width, height } = self.tile_size;
        let tile_size = Vec2::new(width.into(), height.into());
        let tile_count = self.tile_count as f32;
        Vec2::new(tile_size.x, tile_size.y * tile_count)
    }
}

#[derive(Debug)]
pub(crate) struct TilesetData<T> {
    pub(crate) id: TilesetId,
    pub(crate) tile_count: u32,
    pub(crate) tile_size: TileSize,
    pub(crate) name: String,
    pub(crate) texture: T,
}
impl<T> TilesetData<T> {
    fn from_ase<F>(f: F, ase: &AsepriteFile, ase_tileset: &asefile::Tileset) -> TilesetResult<Self>
    where
        F: FnOnce(&AsepriteFile, &asefile::Tileset) -> TilesetResult<T>,
    {
        let ase_id = *ase_tileset.id();
        let texture = f(ase, ase_tileset)?;
        let ase_size = ase_tileset.tile_size();
        Ok(Self {
            id: TilesetId(*ase_id.value()),
            tile_count: *ase_tileset.tile_count(),
            tile_size: TileSize::from_ase(ase_size),
            name: ase_tileset.name().to_string(),
            texture,
        })
    }
}
impl TilesetData<Texture> {
    pub(crate) fn from_ase_with_texture(
        ase: &AsepriteFile,
        ase_tileset: &asefile::Tileset,
    ) -> TilesetResult<Self> {
        TilesetData::<Texture>::from_ase(texture_from, ase, ase_tileset)
    }
    pub(crate) fn move_into_bevy(
        self,
        ase_id: &AseId,
        textures: &mut Assets<Texture>,
        tilesets: &mut Assets<Tileset>,
    ) -> Handle<Tileset> {
        let TilesetData {
            id,
            tile_count,
            tile_size,
            name,
            texture,
        } = self;
        let tex_handle = textures.add(texture);
        let tileset = Tileset {
            key: TilesetAseKey::new(*ase_id, id),
            name,
            texture: tex_handle,
            tile_count,
            tile_size,
        };
        tilesets.add(tileset)
    }
}

#[derive(Debug)]
pub(crate) struct TilesetsById<T>(HashMap<TilesetId, TilesetData<T>>);
impl<T> TilesetsById<T> {}
impl<T> FromIterator<(TilesetId, TilesetData<T>)> for TilesetsById<T> {
    fn from_iter<I: IntoIterator<Item = (TilesetId, TilesetData<T>)>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}
