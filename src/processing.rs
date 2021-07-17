use crate::animate::{AnimationInfo, Frame};
use crate::Tileset;
use crate::{
    animate::Animation,
    animation::AnimationData,
    ase::{AseKeyed, AsesById},
    sprite::Sprite,
    tileset::{TilesetData, TilesetKey, TilesetResult},
};
use asefile::AsepriteFile;
use bevy::prelude::*;
use bevy::sprite::TextureAtlasBuilder;
use std::time::Instant;
use std::{collections::HashMap, path::PathBuf};

pub(crate) struct TilesetsByKey<T>(pub HashMap<TilesetKey, TilesetData<T>>);
impl TilesetsByKey<Texture> {
    fn new() -> Self {
        Self(HashMap::new())
    }
    fn add_ase(&mut self, ase: &AseKeyed) -> TilesetResult<()> {
        let ase_id = &ase.id;
        let kv_pairs: TilesetResult<HashMap<TilesetKey, TilesetData<Texture>>> = ase
            .file
            .tilesets()
            .map()
            .values()
            .map(|ts| {
                let value = TilesetData::<Texture>::from_ase_with_texture(&ase.file, ts)?;
                let key = TilesetKey::new(ase_id, &value.id);
                Ok((key, value))
            })
            .collect();
        self.0.extend(kv_pairs?);
        Ok(())
    }
    pub(crate) fn into_iter(
        self,
    ) -> std::collections::hash_map::IntoIter<TilesetKey, TilesetData<Texture>> {
        self.0.into_iter()
    }
}

pub(crate) struct AseAssets {
    pub(crate) sprites: Vec<Sprite<Texture>>,
    pub(crate) anims: Vec<AnimationData>,
    pub(crate) tilesets: TilesetsByKey<Texture>,
}
impl AseAssets {
    pub(crate) fn new(ases: Vec<(PathBuf, AsepriteFile)>) -> Self {
        let ases_by_id = AsesById::from(ases);
        let mut tmp_sprites: Vec<Sprite<Texture>> = Vec::new();
        let mut tmp_anim_info: Vec<AnimationData> = Vec::new();
        let mut tilesets = TilesetsByKey::<Texture>::new();
        for (_id, ase_keyed) in ases_by_id.iter() {
            let file = &ase_keyed.file;
            let name = &ase_keyed.path;
            debug!("Processing Aseprite file: {}", name.display());
            let sprite_offset = tmp_sprites.len();
            for frame in 0..file.num_frames() {
                tmp_sprites.push(Sprite::<Texture>::new(file, frame));
            }
            tmp_anim_info.push(AnimationData::new(name, file, sprite_offset));
            for tag_id in 0..file.num_tags() {
                let tag = file.tag(tag_id);
                tmp_anim_info.push(AnimationData::from_tag(name, sprite_offset, tag));
            }
            tilesets.add_ase(ase_keyed).unwrap();
        }
        Self {
            sprites: tmp_sprites,
            anims: tmp_anim_info,
            tilesets,
        }
    }
    pub(crate) fn move_into_bevy(
        self,
        animations: &mut Assets<Animation>,
        anim_info: &mut AnimationInfo,
        textures: &mut Assets<Texture>,
        atlases: &mut Assets<TextureAtlas>,
        tilesets: &mut Assets<Tileset>,
    ) {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();

        let start = Instant::now();
        for (key, ts) in self.tilesets.into_iter() {
            let ase_id = key.ase_id();
            ts.move_into_bevy(ase_id, textures, tilesets);
        }
        let tmp_sprites: Vec<crate::sprite::Sprite<Handle<Texture>>> = self
            .sprites
            .into_iter()
            .map(
                |crate::sprite::Sprite {
                     frame,
                     texture: tex,
                     duration,
                 }| {
                    let tex_handle = textures.add(tex);
                    let texture = textures.get(&tex_handle).unwrap();
                    texture_atlas_builder.add_texture(tex_handle.clone_weak(), texture);
                    crate::sprite::Sprite {
                        texture: tex_handle,
                        frame,
                        duration,
                    }
                },
            )
            .collect();
        let atlas = texture_atlas_builder
            .finish(textures)
            .expect("Creating texture atlas failed");
        let atlas_handle = atlases.add(atlas);
        let atlas = atlases.get(&atlas_handle).unwrap();
        debug!("Creating atlas took: {}", start.elapsed().as_secs_f32());

        for tmp_anim in self.anims {
            let mut frames = Vec::with_capacity(tmp_anim.sprites.len());
            for sprite_id in tmp_anim.sprites {
                let tmp_sprite = &tmp_sprites[sprite_id];
                let atlas_index = atlas.get_texture_index(&tmp_sprite.texture).unwrap();
                frames.push(Frame {
                    sprite: crate::animate::Sprite {
                        atlas: atlas_handle.clone(),
                        atlas_index: atlas_index as u32,
                    },
                    duration_ms: tmp_sprite.duration,
                });
            }
            let handle = animations.add(Animation::new(frames));
            anim_info.add_anim(tmp_anim.file, tmp_anim.tag, handle);
        }
    }
}
