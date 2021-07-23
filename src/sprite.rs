use asefile::AsepriteFile;
use bevy::{
    prelude::*,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};

/// The sprite of an animation frame. Refers to an item in a sprite atlas.
#[derive(Debug)]
pub struct Sprite {
    // TODO: Add support for pivot points
    pub atlas: Handle<TextureAtlas>,
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
