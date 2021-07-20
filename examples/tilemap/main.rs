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
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_camera.system()))
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

fn layer_settings_from(map_size: UVec2, chunk_size: UVec2, tileset: &Tileset) -> LayerSettings {
    let Tileset {
        tile_count,
        tile_size,
        ..
    } = tileset;
    let tile_size = Vec2::new((*tile_size.width()).into(), (*tile_size.height()).into());
    let tile_count = *tile_count as f32;
    let texture_size = Vec2::new(tile_size.x, tile_size.y * tile_count);
    LayerSettings::new(map_size, chunk_size, tile_size, texture_size)
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle({
        let mut b = OrthographicCameraBundle::new_2d();
        b.orthographic_projection.scale = 1.0 / 3.0; // scale to 3x
        b
    });
}

fn set_tiles(layer_builder: &mut LayerBuilder<TileBundle>) {
    for y in 0..3 {
        let y_offset = 7 - (y * 3);
        for x in 0..3 {
            let texture_index = y_offset + x;
            let tile = Tile {
                texture_index,
                ..Tile::default()
            };
            let tile_pos = UVec2::new(x as u32, y as u32);
            layer_builder.set_tile(tile_pos, tile.into()).unwrap();
        }
    }
}

fn spawn_tiles(
    mut commands: Commands,
    tilesets: Res<Assets<Tileset>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    for (_, tileset) in tilesets.iter() {
        let texture_handle = tileset.texture.clone();
        let material_handle = materials.add(ColorMaterial::texture(texture_handle));
        let settings = layer_settings_from(UVec2::new(3, 3), UVec2::new(3, 3), tileset);

        let (mut layer_builder, layer_entity) =
            LayerBuilder::<TileBundle>::new(&mut commands, settings, 0u16, 0u16);

        set_tiles(&mut layer_builder);

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
