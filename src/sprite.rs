use asefile::AsepriteFile;
use bevy::{
    prelude::Texture,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};

pub(crate) struct Sprite<T> {
    pub(crate) frame: u32,
    pub(crate) texture: T,
    pub(crate) duration: u32,
}
impl Sprite<Texture> {
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
