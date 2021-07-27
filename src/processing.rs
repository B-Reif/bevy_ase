use crate::asset::{
    animation::{Animation, AnimationData, Frame, Sprite, SpriteData},
    ase::{AsepriteFileWithId, AsesById},
    slice::{Slice, SliceAseKey, SliceId},
    tileset::{TilesetAseKey, TilesetData, TilesetResult},
    Tileset,
};
use crate::loader::AseAssetResources;
use asefile::AsepriteFile;
use bevy::prelude::*;
use bevy::sprite::TextureAtlasBuilder;
use bevy::utils::HashMap;
use std::path::PathBuf;

pub(crate) struct TilesetsByKey<T>(pub HashMap<TilesetAseKey, TilesetData<T>>);
impl TilesetsByKey<Texture> {
    fn new() -> Self {
        Self(HashMap::default())
    }
    fn add_ase(&mut self, ase: &AsepriteFileWithId) -> TilesetResult<()> {
        let ase_id = &ase.id;
        let kv_pairs: TilesetResult<HashMap<TilesetAseKey, TilesetData<Texture>>> = ase
            .file
            .tilesets()
            .map()
            .values()
            .map(|ts| {
                let value = TilesetData::<Texture>::from_ase_with_texture(&ase.file, ts)?;
                let key = TilesetAseKey::new(*ase_id, value.id);
                Ok((key, value))
            })
            .collect();
        self.0.extend(kv_pairs?);
        Ok(())
    }
    pub(crate) fn into_iter(
        self,
    ) -> std::collections::hash_map::IntoIter<TilesetAseKey, TilesetData<Texture>> {
        self.0.into_iter()
    }
}

fn move_slices(slice_vec: Vec<Slice>, slices: &mut Assets<Slice>) {
    for s in slice_vec {
        slices.add(s);
    }
}

fn move_tilesets(
    tilesets_by_key: TilesetsByKey<Texture>,
    textures: &mut Assets<Texture>,
    tilesets: &mut Assets<Tileset>,
) {
    for (key, ts) in tilesets_by_key.into_iter() {
        let ase_id = key.ase_id();
        ts.move_into_bevy(ase_id, textures, tilesets);
    }
}

fn move_animations(
    animation_data: Vec<AnimationData>,
    sprite_data: Vec<SpriteData<Handle<Texture>>>,
    atlas: &TextureAtlas,
    atlas_handle: Handle<TextureAtlas>,
    animation_assets: &mut Assets<Animation>,
) {
    for tmp_anim in animation_data {
        let mut frames = Vec::with_capacity(tmp_anim.sprites.len());
        for sprite_id in tmp_anim.sprites {
            let tmp_sprite = &sprite_data[sprite_id];
            let atlas_index = atlas
                .get_texture_index(&tmp_sprite.texture)
                .expect("Failed to get texture from atlas");
            frames.push(Frame {
                sprite: Sprite {
                    atlas_index: atlas_index as u32,
                },
                duration_ms: tmp_sprite.duration,
            });
        }
        let _handle = animation_assets.add(Animation::new(frames, atlas_handle.clone()));
    }
}

fn move_sprites(
    sprites: Vec<SpriteData<Texture>>,
    textures: &mut Assets<Texture>,
    atlases: &mut Assets<TextureAtlas>,
) -> (Vec<SpriteData<Handle<Texture>>>, Handle<TextureAtlas>) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let sprite_handles: Vec<SpriteData<Handle<Texture>>> = sprites
        .into_iter()
        .map(
            |SpriteData {
                 frame,
                 texture: tex,
                 duration,
             }| {
                let tex_handle = textures.add(tex);
                let texture = textures
                    .get(&tex_handle)
                    .expect("Failed to get texture from handle");
                texture_atlas_builder.add_texture(tex_handle.clone_weak(), texture);
                SpriteData {
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
    (sprite_handles, atlas_handle)
}

pub(crate) struct ResourceData {
    pub(crate) sprites: Vec<SpriteData<Texture>>,
    pub(crate) anims: Vec<AnimationData>,
    pub(crate) tilesets: TilesetsByKey<Texture>,
    pub(crate) slices: Vec<Slice>,
}
impl ResourceData {
    pub(crate) fn new(ases: Vec<(PathBuf, AsepriteFile)>) -> Self {
        let ases_by_id = AsesById::from(ases);
        let mut tmp_sprites: Vec<SpriteData<Texture>> = Vec::new();
        let mut tmp_anim_info: Vec<AnimationData> = Vec::new();
        let mut slices: Vec<Slice> = Vec::new();
        let mut tilesets = TilesetsByKey::<Texture>::new();
        for (ase_id, ase_keyed) in ases_by_id.iter() {
            let AsepriteFileWithId {
                path: name, file, ..
            } = ase_keyed;
            debug!("Processing Aseprite file: {}", name.display());
            let sprite_offset = tmp_sprites.len();
            for frame in 0..file.num_frames() {
                tmp_sprites.push(SpriteData::<Texture>::new(file, frame));
            }
            tmp_anim_info.push(AnimationData::new(name, file, sprite_offset));
            for tag_id in 0..file.num_tags() {
                let tag = file.tag(tag_id);
                tmp_anim_info.push(AnimationData::from_tag(name, sprite_offset, tag));
            }
            tilesets
                .add_ase(ase_keyed)
                .expect("Failed to add tilesets from Ase file");
            for (idx, ase_slice) in file.slices().iter().enumerate() {
                let slice_id = SliceId::new(idx as u32);
                let key = SliceAseKey::new(*ase_id, slice_id);
                let slice = crate::asset::slice::Slice::from_ase(ase_slice, key);
                slices.push(slice);
            }
        }
        Self {
            sprites: tmp_sprites,
            anims: tmp_anim_info,
            tilesets,
            slices,
        }
    }
    pub(crate) fn move_into_resources(self, resources: &mut AseAssetResources) {
        let AseAssetResources {
            animations,
            textures,
            atlases,
            tilesets,
            slices,
        } = resources;

        if let Some(slices) = slices {
            move_slices(self.slices, slices);
        }

        if let Some(tilesets) = tilesets {
            move_tilesets(self.tilesets, textures, tilesets);
        }

        // Move sprites
        if let Some(atlases) = atlases {
            let (sprites, atlas_handle) = move_sprites(self.sprites, textures, atlases);
            let atlas = atlases.get(&atlas_handle).unwrap();

            // Move animations
            if let Some(animations) = animations {
                move_animations(self.anims, sprites, atlas, atlas_handle, animations);
            }
        }
    }
}
