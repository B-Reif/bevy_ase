use std::path::Path;

use bevy::{prelude::*, reflect::TypeUuid, sprite::SpriteSheetBundle};
use bevy_ase::{
    self,
    asset::{Animation, AseAsset},
    loader::{self, Loader},
};

#[derive(TypeUuid, Deref)]
#[uuid = "33fd3d9b-dd1e-4d38-9b82-30751b29c72c"]
pub struct SpriteSheetAnimation(benimator::Animation);

#[derive(Default, Component, Deref, DerefMut)]
pub struct SpriteSheetAnimationState(benimator::State);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(loader::AseLoaderDefaultPlugin)
        .add_asset::<SpriteSheetAnimation>()
        .add_state(AppState::Loading)
        .add_system_set(SystemSet::on_enter(AppState::Loading).with_system(load_sprites))
        .add_system_set(
            SystemSet::on_update(AppState::Loading).with_system(check_loading_sprites),
        )
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(spawn_sprites))
        .add_system_set(SystemSet::on_update(AppState::Game).with_system(animate))
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
    mut sprite_sheet_animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
    commands.spawn_bundle({
        let mut b = Camera2dBundle::default();
        b.projection.scale = 1.0 / 3.0; // scale to 3x
        b
    });

    let anims = animations.iter().enumerate();
    for (idx, (_id, anim)) in anims {
        let texture_atlas = anim.atlas();
        // The "benimator" feature provides a From implementation to convert animations.
        let anim: benimator::Animation = anim.into();
        let anim_handle = sprite_sheet_animations.add(SpriteSheetAnimation(anim));
        let x_position = idx as f32 * 50.0;

        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas,
                transform: Transform::from_xyz(x_position, 0.0, 0.0),
                ..Default::default()
            })
            .insert(anim_handle)
            .insert(SpriteSheetAnimationState::default());
    }
}

pub fn animate(
    time: Res<Time>,
    animations: Res<Assets<SpriteSheetAnimation>>,
    mut query: Query<(
        &mut SpriteSheetAnimationState,
        &mut TextureAtlasSprite,
        &Handle<SpriteSheetAnimation>,
    )>,
) {
    for (mut state, mut texture, handle) in query.iter_mut() {
        let animation = match animations.get(handle) {
            Some(anim) => anim,
            None => continue,
        };
        state.update(animation, time.delta());
        texture.index = state.frame_index();
    }
}
