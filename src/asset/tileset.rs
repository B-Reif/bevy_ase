use asefile::{AsepriteFile, TilesetImageError};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};
use std::fmt;

pub(crate) type TilesetResult<T> = std::result::Result<T, TilesetError>;

#[derive(Debug)]
pub enum TilesetError {
    MissingId(asefile::TilesetId),
    NoPixels(asefile::TilesetId),
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
            TilesetImageError::MissingTilesetId(id) => Self::MissingId(*id),
            TilesetImageError::NoPixelsInTileset(id) => Self::NoPixels(*id),
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

/// Width and height of a tile in pixels.
#[derive(Debug)]
pub struct TileSize {
    /// Width of a tile in pixels.
    pub width: u16,
    /// Height of a tile in pixels.
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
        let tile_count = self.tile_count as f32;
        Vec2::new(width as f32, height as f32 * tile_count)
    }
}

#[derive(Debug)]
pub(crate) struct TilesetData<T> {
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
        let texture = f(ase, ase_tileset)?;
        let ase_size = ase_tileset.tile_size();
        Ok(Self {
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
        textures: &mut Assets<Texture>,
        tilesets: &mut Assets<Tileset>,
    ) -> Handle<Tileset> {
        let TilesetData {
            tile_count,
            tile_size,
            name,
            texture,
        } = self;
        let tex_handle = textures.add(texture);
        let tileset = Tileset {
            name,
            texture: tex_handle,
            tile_count,
            tile_size,
        };
        tilesets.add(tileset)
    }
}
