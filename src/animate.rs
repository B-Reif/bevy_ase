use std::path::{Path, PathBuf};

use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};

use crate::timer::{self, GameTime, GameTimer};

pub struct SpriteAnimatorPlugin;

impl Plugin for SpriteAnimatorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<AnimationInfo>()
            .add_asset::<Animation>()
            .add_system(sprite_animator.system().after(timer::Label));
    }
}

/// A sprite-based animation.
#[derive(Debug, TypeUuid)]
#[uuid = "49c1ff21-7abe-4167-b25b-f3730763e348"]
pub struct Animation {
    frames: Vec<Frame>,
}

/// A single frame in an [Animation].
#[derive(Debug)]
pub struct Frame {
    pub sprite: Sprite,
    pub duration_ms: u32,
}

/// The sprite of an animation frame. Refers to an item in a sprite atlas.
#[derive(Debug)]
pub struct Sprite {
    // TODO: Add support for pivot points
    pub atlas: Handle<TextureAtlas>,
    pub atlas_index: u32,
}

impl Animation {
    pub fn new(frames: Vec<Frame>) -> Self {
        Animation { frames }
    }

    pub fn num_frames(&self) -> u32 {
        self.frames.len() as u32
    }

    pub fn frame(&self, frame: u32) -> &Frame {
        &self.frames[frame as usize]
    }

    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    /// Returns next frame number after the given frame. The second result is
    /// `true` if we wrapped around.
    pub fn frame_after(&self, frame: u32) -> (u32, bool) {
        let frame = frame as usize;
        let num_frames = self.frames.len();
        if frame < num_frames - 1 {
            (frame as u32 + 1, false)
        } else {
            (0, true)
        }
    }
}

/// Resource for looking up [Animation] handles by name.
///
/// This is populated by the animation loader.
#[derive(Debug, Default)]
pub struct AnimationInfo {
    animations: Vec<Handle<Animation>>,
    names: HashMap<PathBuf, (usize, HashMap<String, usize>)>,
}

impl AnimationInfo {
    /// Lookup by path relative to the assets directory.
    pub fn lookup(&self, path: &Path) -> Option<Handle<Animation>> {
        self.names
            .get(path)
            .map(|(idx, _)| self.animations[*idx].clone())
    }

    /// Lookup by path and tag within that animation file.
    pub fn lookup_with_tag(&self, path: &Path, tag: &str) -> Option<Handle<Animation>> {
        self.names
            .get(path)
            .and_then(|(_, tags)| tags.get(tag).map(|idx| self.animations[*idx].clone()))
    }

    pub(crate) fn add_anim(&mut self, path: PathBuf, tag: Option<String>, hdl: Handle<Animation>) {
        let idx = self.animations.len();
        self.animations.push(hdl);
        let mut entry = self
            .names
            .entry(path)
            .or_insert((usize::MAX, HashMap::default()));
        if let Some(tag) = tag {
            entry.1.insert(tag, idx);
        } else {
            entry.0 = idx;
        }
    }
}

// ----------------------------------------------------------------------------

pub struct PlayAnimation {
    pub current_frame: u32,
    pub frame_end: GameTime,
    pub playing: bool,
    pub looping: bool,
    pub animation: Handle<Animation>,
}

impl PlayAnimation {
    pub fn new(animation: Handle<Animation>) -> Self {
        PlayAnimation {
            animation,
            current_frame: 0,
            frame_end: GameTime::ZERO, // uninitialized
            playing: true,
            looping: true,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Bundle)]
pub struct AnimationBundle {
    pub play: PlayAnimation,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

// ----------------------------------------------------------------------------

/// System for sprite-based animation
pub fn sprite_animator(
    time: Res<GameTimer>,
    animations: Res<Assets<Animation>>,
    mut query: Query<(&mut TextureAtlasSprite, &mut PlayAnimation)>,
) {
    let t = time.time();
    for (mut sprite, mut play) in query.iter_mut() {
        if !play.playing {
            continue;
        }
        if play.frame_end >= t {
            continue;
        }

        // Frame changed, or entity needs initializing
        let anim = if let Some(anim) = animations.get(&play.animation) {
            anim
        } else {
            continue;
        };
        let mut frame = play.current_frame;
        let mut init = false;
        // Advance frames until we reach current game time.
        while play.frame_end < t {
            // Special support for newly-created sprites.
            if play.frame_end == GameTime::ZERO {
                play.frame_end = t;
                play.frame_end.add_millis(anim.frame(frame).duration_ms);
                init = true;
                break;
            }

            let (next_frame, wrapped) = anim.frame_after(frame);
            if wrapped && !play.looping {
                // Animation finished, make sure we use the shortcut next time
                play.playing = false;
                break;
            }
            frame = next_frame;
            play.frame_end.add_millis(anim.frame(frame).duration_ms);
        }

        if init || frame != play.current_frame {
            play.current_frame = frame;
            sprite.index = anim.frame(frame).sprite.atlas_index;
        }
    }
}
