use crate::asset::{TileSize, Tileset};
use bevy::math::{UVec2, Vec2};
use bevy_ecs_tilemap::prelude::*;

impl From<TileSize> for Vec2 {
    fn from(tile_size: TileSize) -> Self {
        Vec2::new(tile_size.width as f32, tile_size.height as f32)
    }
}

impl Tileset {
    /// Creates new [LayerSettings] using the [Tileset's](Tileset) own tile size and texture size.
    ///
    /// # Examples
    ///
    /// ```
    /// #[cfg(feature = "bevy_ecs_tilemap")]
    /// use bevy_ase::asset::Tileset;
    /// use bevy_ecs_tilemap::LayerSettings;
    /// use bevy::math::UVec2;
    ///
    /// // Create new layer settings from a tileset, with specified map size and chunk size.
    /// fn my_layer_settings(tileset: &Tileset) -> LayerSettings {
    ///     let map_size = UVec2::new(30, 30);
    ///     let chunk_size = UVec2::new(15, 15);
    ///     tileset.layer_settings(map_size, chunk_size)       
    /// }
    /// ```
    pub fn layer_settings(&self, map_size: UVec2, chunk_size: UVec2) -> LayerSettings {
        LayerSettings::new(
            map_size,
            chunk_size,
            Vec2::new(self.tile_size.width as f32, self.tile_size.height as f32),
            self.texture_size(),
        )
    }
}
