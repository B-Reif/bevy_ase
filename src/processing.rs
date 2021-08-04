use crate::asset::{
    animation::{Animation, AnimationData, Frame, Sprite, SpriteData},
    slice::Slice,
    tileset::{TilesetData, TilesetResult},
    AseAssetMap, Tileset,
};
use crate::loader::AseAssetResources;
use asefile::AsepriteFile;
use bevy::sprite::TextureAtlasBuilder;
use bevy::{prelude::*, utils::HashMap};
use std::path::{Path, PathBuf};

fn tilesets_from(ase: &AsepriteFile) -> TilesetResult<Vec<TilesetData<Texture>>> {
    ase.tilesets()
        .map()
        .values()
        .map(|ts| TilesetData::<Texture>::from_ase_with_texture(&ase, ts))
        .collect()
}

fn move_slices(
    slice_vec: Vec<Slice>,
    slices: &mut Assets<Slice>,
    mut file_assets: &mut Option<&mut AseAssetMap>,
) {
    for s in slice_vec {
        if let Some(ref mut file_assets) = file_assets {
            let slice_name = s.name.clone();
            let handle = slices.add(s);
            file_assets.insert_slice(slice_name, handle);
        } else {
            slices.add(s);
        }
    }
}

struct TilesetImportResources<'a> {
    textures: &'a mut Assets<Texture>,
    tilesets: &'a mut Assets<Tileset>,
}

fn move_tilesets(
    tileset_data: Vec<TilesetData<Texture>>,
    resources: TilesetImportResources,
    mut file_assets: &mut Option<&mut AseAssetMap>,
) {
    let TilesetImportResources { textures, tilesets } = resources;
    for ts in tileset_data.into_iter() {
        // ts.move_into_bevy(textures, tilesets);
        let TilesetData {
            tile_count,
            tile_size,
            name,
            texture,
        } = ts;
        let tex_handle = textures.add(texture);
        let tileset = Tileset {
            name,
            texture: tex_handle,
            tile_count,
            tile_size,
        };
        if let Some(ref mut file_assets) = file_assets {
            let tileset_name = tileset.name.clone();
            let handle = tilesets.add(tileset);
            file_assets.insert_tileset(tileset_name, handle);
        } else {
            tilesets.add(tileset);
        }
    }
}

// Data used to move animations into Bevy.
struct AnimationImportData<'a> {
    animation_data: Vec<AnimationData>,
    sprite_data: Vec<SpriteData<Handle<Texture>>>,
    atlas: &'a TextureAtlas,
    atlas_handle: Handle<TextureAtlas>,
}

fn move_animations(
    data: AnimationImportData,
    animations: &mut Assets<Animation>,
    file_assets: &mut Option<&mut AseAssetMap>,
) {
    let AnimationImportData {
        animation_data,
        sprite_data,
        atlas,
        atlas_handle,
    } = data;

    for anim_data in animation_data.into_iter() {
        let mut frames = Vec::with_capacity(anim_data.sprites.len());
        for sprite_id in &anim_data.sprites {
            let sprite_id = *sprite_id;
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
        let handle = animations.add(Animation::new(frames, atlas_handle.clone()));
        if let Some(tag_name) = anim_data.tag_name {
            if let Some(ref mut file_assets) = file_assets {
                file_assets.insert_animation(tag_name, handle);
            }
        }
    }
}

struct SpriteImportResources<'a> {
    textures: &'a mut Assets<Texture>,
    atlases: &'a mut Assets<TextureAtlas>,
}

fn move_sprites(
    sprites: Vec<SpriteData<Texture>>,
    resources: SpriteImportResources,
    file_assets: &mut Option<&mut AseAssetMap>,
) -> (Vec<SpriteData<Handle<Texture>>>, Handle<TextureAtlas>) {
    let SpriteImportResources { textures, atlases } = resources;
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
                if let Some(ref mut file_assets) = file_assets {
                    file_assets.insert_texture(frame, tex_handle.clone());
                }
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

pub(crate) struct ResourceDataByFile(HashMap<PathBuf, ResourceData>);
impl ResourceDataByFile {
    pub(crate) fn new(ases: Vec<(PathBuf, AsepriteFile)>) -> Self {
        let inner = ases
            .into_iter()
            .map(|(path, ase)| {
                let data = ResourceData::new(&path, &ase);
                (path, data)
            })
            .collect();
        Self(inner)
    }
    pub(crate) fn move_into_resources(self, resources: &mut AseAssetResources) {
        for (path, data) in self.0.into_iter() {
            data.move_into_resources(path, resources);
        }
    }
}

pub(crate) struct ResourceData {
    pub(crate) sprites: Vec<SpriteData<Texture>>,
    pub(crate) anims: Vec<AnimationData>,
    pub(crate) tilesets: Vec<TilesetData<Texture>>,
    pub(crate) slices: Vec<Slice>,
}
impl ResourceData {
    pub(crate) fn new(path: &Path, file: &AsepriteFile) -> Self {
        let mut tmp_sprites: Vec<SpriteData<Texture>> = Vec::new();
        let mut tmp_anim_info: Vec<AnimationData> = Vec::new();
        let mut slices: Vec<Slice> = Vec::new();
        let mut tilesets: Vec<TilesetData<Texture>> = Vec::new();
        debug!("Processing Aseprite file: {}", path.display());
        let sprite_offset = tmp_sprites.len();
        for frame in 0..file.num_frames() {
            tmp_sprites.push(SpriteData::<Texture>::new(&file, frame));
        }
        tmp_anim_info.push(AnimationData::new(&path, &file, sprite_offset));
        for tag_id in 0..file.num_tags() {
            let tag = file.tag(tag_id);
            tmp_anim_info.push(AnimationData::from_tag(&path, sprite_offset, tag));
        }
        let mut ase_tilesets =
            tilesets_from(&file).expect("Internal error: Failed to add tilesets from Ase file");
        tilesets.append(&mut ase_tilesets);
        for ase_slice in file.slices().iter() {
            // let slice_id = SliceId::new(idx as u32);
            let slice = crate::asset::slice::Slice::from_ase(ase_slice);
            slices.push(slice);
        }
        Self {
            sprites: tmp_sprites,
            anims: tmp_anim_info,
            tilesets,
            slices,
        }
    }
    pub(crate) fn move_into_resources(self, path: PathBuf, resources: &mut AseAssetResources) {
        let data = self;
        let (textures, animations, atlases, tilesets, slices, index) = resources;

        let mut file_assets = index.as_deref_mut().map(|idk| idk.get_mut(path));

        if let Some(slices) = slices {
            move_slices(data.slices, slices, &mut file_assets);
        }

        if let Some(tilesets) = tilesets {
            let resources = TilesetImportResources { textures, tilesets };
            move_tilesets(data.tilesets, resources, &mut file_assets);
        }

        // Move sprites
        if let Some(atlases) = atlases {
            let resources = SpriteImportResources { textures, atlases };
            let (sprites, atlas_handle) = move_sprites(data.sprites, resources, &mut file_assets);
            let atlas = atlases.get(&atlas_handle).unwrap();
            // Move animations
            if let Some(animations) = animations {
                let data = AnimationImportData {
                    animation_data: data.anims,
                    sprite_data: sprites,
                    atlas,
                    atlas_handle,
                };

                move_animations(data, animations, &mut file_assets);
            }
        }
    }
}
