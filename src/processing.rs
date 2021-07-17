use crate::{
    animation::Animation,
    ase::{AseKeyed, AsesById},
    tileset::{self, TilesetKey, TilesetResult, TilesetsById},
    Tileset,
};
use asefile::AsepriteFile;
use bevy::{
    prelude::*,
    render::texture::{Extent3d, TextureDimension, TextureFormat},
};
use std::{collections::HashMap, path::PathBuf};

pub(crate) struct TilesetsByKey<T>(HashMap<TilesetKey, Tileset<T>>);
impl TilesetsByKey<Texture> {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn add_ase(&mut self, ase: &AseKeyed) -> TilesetResult<()> {
        let ase_id = &ase.id;
        let kv_pairs: TilesetResult<HashMap<TilesetKey, Tileset<Texture>>> = ase
            .file
            .tilesets()
            .map()
            .values()
            .map(|ts| {
                let value = Tileset::<Texture>::from_ase_with_texture(&ase.file, ts)?;
                let key = TilesetKey::new(ase_id, value.id());
                Ok((key, value))
            })
            .collect();
        self.0.extend(kv_pairs?);
        Ok(())
    }
}

pub(crate) struct AseAssets<T> {
    pub(crate) sprites: Vec<Sprite<T>>,
    pub(crate) anims: Vec<Animation>,
    pub(crate) tilesets: TilesetsByKey<T>,
}
impl AseAssets<Texture> {
    pub(crate) fn new(ases: Vec<(PathBuf, AsepriteFile)>) -> Self {
        let ases_by_id = AsesById::from(ases);
        let mut tmp_sprites: Vec<Sprite<Texture>> = Vec::new();
        let mut tmp_anim_info: Vec<Animation> = Vec::new();
        let mut tilesets = TilesetsByKey::<Texture>::new();
        for (_id, ase_keyed) in ases_by_id.iter() {
            tilesets.add_ase(ase_keyed);
        }
        // for (name, ase) in &ases {
        //     debug!("Processing Aseprite file: {}", name.display());
        //     let sprite_offset = tmp_sprites.len();

        //     for frame in 0..ase.num_frames() {
        //         tmp_sprites.push(Sprite::<Texture>::new(ase, frame));
        //     }

        //     tmp_anim_info.push(Animation::new(name, ase, sprite_offset));

        //     for tag_id in 0..ase.num_tags() {
        //         let tag = ase.tag(tag_id);
        //         tmp_anim_info.push(Animation::from_tag(name, sprite_offset, tag));
        //     }
        // }
        Self {
            sprites: tmp_sprites,
            anims: tmp_anim_info,
            tilesets,
        }
    }
}

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
