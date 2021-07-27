use std::path::Path;

use bevy::{input::system::exit_on_esc_system, prelude::*, sprite::entity::SpriteSheetBundle};
use bevy_ase::{
    self,
    asset::{Animation, AseAsset},
    loader::{self, Loader},
};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(loader::AseLoaderDefaultPlugin)
        .add_plugin(benimator::AnimationPlugin)
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

// Collect the sprite and send it to the loader.
pub fn load_sprites(asset_server: Res<AssetServer>, mut aseloader: ResMut<Loader>) {
    info!("Loading assets");
    let h: Handle<AseAsset> = asset_server.load(Path::new("sprites/hello.aseprite"));
    aseloader.add(h.clone());
}

// Wait until all sprites are loaded.
pub fn check_loading_sprites(mut state: ResMut<State<AppState>>, ase_loader: Res<Loader>) {
    if ase_loader.is_loaded() {
        info!("All Aseprite files loaded");
        state.set(AppState::Game).unwrap()
    }
}

// Create some sprites.
pub fn spawn_sprites(
    mut commands: Commands,
    animations: Res<Assets<Animation>>,
    mut sprite_sheet_animations: ResMut<Assets<benimator::SpriteSheetAnimation>>,
) {
    commands.spawn_bundle({
        let mut b = OrthographicCameraBundle::new_2d();
        b.orthographic_projection.scale = 1.0 / 3.0; // scale to 3x
        b
    });

    let anims = animations.iter().enumerate();
    for (idx, (_id, anim)) in anims {
        let texture_atlas = anim.atlas();
        // The "benimator" feature provides a From implementation to convert animations.
        let anim: benimator::SpriteSheetAnimation = anim.into();
        let anim_handle = sprite_sheet_animations.add(anim);
        let x_position = idx as f32 * 50.0;

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas,
                transform: Transform::from_xyz(x_position, 0.0, 0.0),
                ..Default::default()
            })
            .insert(anim_handle)
            .insert(benimator::Play);
    }
}
