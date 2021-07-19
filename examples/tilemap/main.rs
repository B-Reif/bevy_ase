use std::path::Path;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_asefile::{
    self,
    aseloader::{self, AsepriteAsset, AsepriteLoader},
    timer, Tileset,
};
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(timer::GameTimePlugin)
        .add_plugin(aseloader::AsepriteLoaderPlugin)
        .add_system(exit_on_esc_system.system())
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_sprites.system()))
        .add_system_set(
            SystemSet::on_update(AppState::Loading).with_system(check_loading_sprites.system()),
        )
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_tiles.system()))
        .run()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Game,
}

// Collect all sprites and send them to the loader.
pub fn load_sprites(asset_server: Res<AssetServer>, mut aseloader: ResMut<AsepriteLoader>) {
    info!("Loading assets");
    let handles = asset_server
        .load_folder(Path::new("sprites"))
        .expect("Failed to load sprites");
    for h in &handles {
        aseloader.add(h.clone().typed::<AsepriteAsset>());
    }
}

// Wait until all sprites are loaded.
pub fn check_loading_sprites(mut state: ResMut<State<AppState>>, aseloader: Res<AsepriteLoader>) {
    if aseloader.is_loaded() {
        info!("All Aseprite files loaded");
        state.set(AppState::Game).expect("Failed to set game state");
    }
}

fn spawn_tiles(
    mut commands: Commands,
    tilesets: Res<Assets<Tileset>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    commands.spawn_bundle({
        let mut b = OrthographicCameraBundle::new_2d();
        b.orthographic_projection.scale = 1.0 / 3.0; // scale to 3x
        b
    });

    for (_, tileset) in tilesets.iter() {
        let Tileset {
            tile_count,
            tile_size,
            texture,
            ..
        } = tileset;
        let tile_size = Vec2::new((*tile_size.width()).into(), (*tile_size.height()).into());
        let tile_count = *tile_count as f32;
        let texture_size = Vec2::new(tile_size.x, tile_size.y * tile_count);
        let texture_handle = texture.clone();
        let material_handle = materials.add(ColorMaterial::texture(texture_handle));

        let (mut layer_builder, layer_entity) = LayerBuilder::<TileBundle>::new(
            &mut commands,
            LayerSettings::new(UVec2::new(3, 3), UVec2::new(3, 3), tile_size, texture_size),
            0u16,
            0u16,
        );
        for x in 0..=2 {
            for y in 0..=2 {
                let tile_pos = UVec2::new(x, y);
                let texture_index = (7 - (y * 3)) + x;
                let tile = Tile {
                    texture_index: texture_index as u16,
                    ..Tile::default()
                };
                layer_builder.set_tile(tile_pos, tile.into()).unwrap();
            }
        }

        map_query.build_layer(&mut commands, layer_builder, material_handle);

        let map_entity = commands.spawn().id();
        let mut map = Map::new(0u16, map_entity);
        map.add_layer(&mut commands, 0u16, layer_entity);
        commands
            .entity(map_entity)
            .insert(map)
            .insert(GlobalTransform::default());
    }
}
