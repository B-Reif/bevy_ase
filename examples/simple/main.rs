use std::path::Path;

use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_ase::{
    self,
    animate::{self, AnimationInfo},
    animation_index::{AnimationId, AnimationIndex},
    loader::{self, AseAsset, Loader},
    timer, Animation,
};

mod ids;

use ids::AnimId;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(timer::GameTimePlugin)
        .add_plugin(animate::SpriteAnimatorPlugin)
        .add_plugin(loader::AseLoaderPlugin)
        .init_resource::<AnimationIndex<AnimId>>()
        .add_system(exit_on_esc_system.system())
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_sprites.system()))
        .add_system_set(
            SystemSet::on_update(AppState::Loading).with_system(check_loading_sprites.system()),
        )
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_sprites.system()))
        .run()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    Game,
}

// Collect all sprites and send them to the loader.
pub fn load_sprites(asset_server: Res<AssetServer>, mut aseloader: ResMut<Loader>) {
    info!("Loading assets");
    let handles = asset_server.load_folder(Path::new("sprites")).unwrap();
    for h in &handles {
        aseloader.add(h.clone().typed::<AseAsset>());
    }
}

// Wait until all sprites are loaded.
pub fn check_loading_sprites(
    mut state: ResMut<State<AppState>>,
    mut anim_ids: ResMut<AnimationIndex<AnimId>>,
    animations: Res<Assets<Animation>>,
    anim_info: Res<AnimationInfo>,
    ase_loader: Res<Loader>,
) {
    if ase_loader.is_loaded() {
        anim_ids.initialize(AnimId::list_all(), &anim_info, &animations);
        info!("All Aseprite files loaded");
        state.set(AppState::Game).unwrap()
    }
}

// Create some sprites.
pub fn spawn_sprites(mut commands: Commands, anim_ids: Res<AnimationIndex<AnimId>>) {
    //commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle({
        let mut b = OrthographicCameraBundle::new_2d();
        b.orthographic_projection.scale = 1.0 / 3.0; // scale to 3x
        b
    });

    commands.spawn_bundle(anim_ids.get(AnimId::Dummy));
    commands.spawn_bundle({
        let mut b = anim_ids.get(AnimId::DummySad);
        b.sprite.transform.translation = Vec3::new(50.0, 0.0, 0.0);
        // b.sprite.transform.scale = Vec3::splat(3.0);
        b
    });
}
