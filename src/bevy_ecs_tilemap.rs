use crate::asset::TileSize;
use bevy::math::Vec2;
use bevy_ecs_tilemap::prelude::*;

impl From<TileSize> for Vec2 {
    fn from(tile_size: TileSize) -> Self {
        Vec2::new(tile_size.width as f32, tile_size.height as f32)
    }
}

impl From<&TileSize> for TilemapTileSize {
    fn from(tile_size: &TileSize) -> Self {
        Self {
            x: tile_size.width as f32,
            y: tile_size.height as f32,
        }
    }
}

impl From<&TileSize> for TilemapGridSize {
    fn from(tile_size: &TileSize) -> Self {
        Self {
            x: tile_size.width as f32,
            y: tile_size.height as f32,
        }
    }
}
