use crate::sprite::Sprite;
use asefile::{AsepriteFile, Tag};
use bevy::prelude::*;
use bevy::{reflect::TypeUuid, sprite::TextureAtlas};
use std::path::PathBuf;

/// A sprite-based animation.
#[derive(Debug, TypeUuid)]
#[uuid = "49c1ff21-7abe-4167-b25b-f3730763e348"]
pub struct Animation {
    frames: Vec<Frame>,
    atlas: Handle<TextureAtlas>,
}
impl Animation {
    pub fn new(frames: Vec<Frame>, atlas: Handle<TextureAtlas>) -> Self {
        Animation { frames, atlas }
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

    pub fn atlas(&self) -> Handle<TextureAtlas> {
        self.atlas.clone()
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

/// A single frame in an [Animation].
#[derive(Debug)]
pub struct Frame {
    pub sprite: Sprite,
    pub duration_ms: u32,
}

#[derive(Debug)]
pub(crate) struct AnimationData {
    pub(crate) file: PathBuf,
    pub(crate) tag: Option<String>,
    pub(crate) sprites: Vec<usize>,
}
impl AnimationData {
    pub(crate) fn new(name: &PathBuf, ase: &AsepriteFile, sprite_offset: usize) -> Self {
        Self {
            file: name.clone(),
            tag: None,
            sprites: (0..ase.num_frames())
                .map(|f| sprite_offset + f as usize)
                .collect(),
        }
    }
    pub(crate) fn from_tag(name: &PathBuf, sprite_offset: usize, tag: &Tag) -> Self {
        AnimationData {
            file: name.clone(),
            tag: Some(tag.name().to_owned()),
            sprites: (tag.from_frame()..tag.to_frame() + 1)
                .map(|f| sprite_offset + f as usize)
                .collect(),
        }
    }
}
