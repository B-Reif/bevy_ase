use asefile::{AsepriteFile, Tag};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
    sprite::TextureAtlas,
};
use std::path::{Path, PathBuf};

/// A sprite-based animation.
#[derive(Debug, TypeUuid)]
#[uuid = "49c1ff21-7abe-4167-b25b-f3730763e348"]
pub struct Animation {
    frames: Vec<Frame>,
    atlas: Handle<TextureAtlas>,
}
impl Animation {
    /// Creates a new Animation with a [Frame] vec and a [TextureAtlas] handle.
    pub fn new(frames: Vec<Frame>, atlas: Handle<TextureAtlas>) -> Self {
        Animation { frames, atlas }
    }

    /// Returns a reference to the animation's [Frame] vec.
    pub fn frames(&self) -> &[Frame] {
        &self.frames
    }

    /// Returns a cloned handle to the animation's [TextureAtlas].
    pub fn atlas(&self) -> Handle<TextureAtlas> {
        self.atlas.clone()
    }
}

/// The sprite of an animation frame. Refers to an item in a sprite atlas.
#[derive(Debug)]
pub struct Sprite {
    /// The index into the TextureAtlas for this sprite.
    pub atlas_index: u32,
}

pub(crate) struct SpriteData<T> {
    pub(crate) frame: u32,
    pub(crate) texture: T,
    pub(crate) duration: u32,
}
impl SpriteData<Texture> {
    pub(crate) fn new(ase: &AsepriteFile, frame: u32) -> Self {
        let img = ase.frame(frame).image();
        let size = Extent3d {
            width: ase.width() as u32,
            height: ase.height() as u32,
            depth: 1,
        };
        let texture = Texture::new_fill(
            size,
            TextureDimension::D2,
            img.as_raw(),
            TextureFormat::Rgba8UnormSrgb,
        );
        Self {
            frame,
            texture,
            duration: ase.frame(frame).duration(),
        }
    }
}

/// A single frame in an [Animation].
#[derive(Debug)]
pub struct Frame {
    /// The [Sprite] shown during this frame.
    pub sprite: Sprite,
    /// The duration of this frame in milliseconds.
    pub duration_ms: u32,
}

#[derive(Debug)]
pub(crate) struct AnimationData {
    pub(crate) file: PathBuf,
    pub(crate) tag: Option<String>,
    pub(crate) sprites: Vec<usize>,
}
impl AnimationData {
    pub(crate) fn new(name: &Path, ase: &AsepriteFile, sprite_offset: usize) -> Self {
        Self {
            file: name.to_path_buf(),
            tag: None,
            sprites: (0..ase.num_frames())
                .map(|f| sprite_offset + f as usize)
                .collect(),
        }
    }
    pub(crate) fn from_tag(name: &Path, sprite_offset: usize, tag: &Tag) -> Self {
        AnimationData {
            file: name.to_path_buf(),
            tag: Some(tag.name().to_owned()),
            sprites: (tag.from_frame()..tag.to_frame() + 1)
                .map(|f| sprite_offset + f as usize)
                .collect(),
        }
    }
}
