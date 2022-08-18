use std::path::Path;

use bevy::prelude::*;
use bevy_ase::{
    self,
    asset::{AseAsset, Tileset},
    loader,
    loader::Loader,
};
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(loader::AseLoaderDefaultPlugin)
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_sprites))
        .add_system_set(
            SystemSet::on_update(AppState::Loading).with_system(check_loading_sprites),
        )
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_camera))
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_tiles))
        .run()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Game,
}

// Collect the tilemap sprite and send it to the loader.
pub fn load_sprites(asset_server: Res<AssetServer>, mut aseloader: ResMut<Loader>) {
    info!("Loading assets");
    let h: Handle<AseAsset> = asset_server.load(Path::new("sprites/tiles.aseprite"));
    aseloader.add(h.clone());
}

// Wait until all sprites are loaded.
pub fn check_loading_sprites(mut state: ResMut<State<AppState>>, aseloader: Res<loader::Loader>) {
    if aseloader.is_loaded() {
        info!("All Aseprite files loaded");
        state.set(AppState::Game).expect("Failed to set game state");
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle({
        let mut b = Camera2dBundle::default();
        b.projection.scale = 1.0 / 3.0; // scale to 3x
        b
    });
}

fn set_tiles(commands: &mut Commands, map_entity: Entity, tile_storage: &mut TileStorage) {
    for y in 0..3 {
        let y_offset = 7 - (y * 3);
        for x in 0..3 {
            let texture_index = y_offset + x;
            let tile_pos = TilePos::new(x as u32, y as u32);
            let tile = commands
                .spawn_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(map_entity),
                    texture: TileTexture(texture_index),
                    ..TileBundle::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile));
        }
    }
}

fn spawn_tiles(
    mut commands: Commands,
    tilesets: Res<Assets<Tileset>>,
) {
    for (_, tileset) in tilesets.iter() {
        let texture_handle = tileset.texture.clone();

        let map_size = TilemapSize { x: 3, y: 3 };
        let map_entity = commands.spawn().id();
        let mut tile_storage = TileStorage::empty(map_size);

        set_tiles(&mut commands, map_entity, &mut tile_storage);

        commands
            .entity(map_entity)
            .insert_bundle(TilemapBundle {
                grid_size: (&tileset.tile_size).into(),
                size: map_size,
                storage: tile_storage,
                texture: TilemapTexture(texture_handle),
                tile_size: (&tileset.tile_size).into(),
                ..Default::default()
            });
    }
}
