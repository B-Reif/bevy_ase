use crate::loader::AseAssetResources;
use crate::{
    asset::{
        animation::{self, Animation, AnimationData, Frame, SpriteData},
        slice::Slice,
        tileset::{TilesetData, TilesetResult},
        AseAssetMap, Tileset,
    },
    handle_id,
};
use asefile::AsepriteFile;
use bevy::render::texture::ImageSampler;
use bevy::sprite::TextureAtlasBuilder;
use bevy::{prelude::*, utils::HashMap};
use std::path::{Path, PathBuf};

fn tilesets_from(ase: &AsepriteFile) -> TilesetResult<Vec<TilesetData<Image>>> {
    let f = |t| TilesetData::<Image>::from_ase_with_texture(ase, t);
    ase.tilesets().iter().map(f).collect()
}

fn move_slices(
    path: &str,
    slice_vec: Vec<Slice>,
    slices: &mut Assets<Slice>,
    file_assets: &mut AseAssetMap,
) {
    for s in slice_vec {
        let slice_id = handle_id::slice(path, &s.name);
        let slice_name = s.name.clone();
        let handle = slices.set(slice_id, s);
        file_assets.insert_slice(slice_name, handle);
    }
}

struct TilesetImportResources<'a> {
    textures: &'a mut Assets<Image>,
    tilesets: &'a mut Assets<Tileset>,
}

fn move_tilesets(
    path: &str,
    tileset_data: Vec<TilesetData<Image>>,
    resources: TilesetImportResources,
    file_assets: &mut AseAssetMap,
) {
    let TilesetImportResources { textures, tilesets } = resources;
    for ts in tileset_data.into_iter() {
        let TilesetData {
            id,
            tile_count,
            tile_size,
            name,
            texture,
        } = ts;
        let image_handle_id = handle_id::tileset_image(path, id);
        let tex_handle = textures.set(image_handle_id, texture);
        let tileset = Tileset {
            id,
            name,
            texture: tex_handle,
            tile_count,
            tile_size,
        };
        let tileset_handle_id = handle_id::tileset(path, id);
        let handle = tilesets.set(tileset_handle_id, tileset);
        file_assets.insert_tileset(id, handle);
    }
}

// Data used to move animations into Bevy.
struct AnimationImportData<'a> {
    animation_data: Vec<AnimationData>,
    sprite_data: Vec<SpriteData<Handle<Image>>>,
    atlas: &'a TextureAtlas,
    atlas_handle: Handle<TextureAtlas>,
}

fn move_animations(
    path: &str,
    data: AnimationImportData,
    animations: &mut Assets<Animation>,
    file_assets: &mut AseAssetMap,
) {
    let AnimationImportData {
        animation_data,
        sprite_data,
        atlas,
        atlas_handle,
    } = data;

    for anim_data in animation_data.into_iter() {
        if let Some(tag_name) = anim_data.tag_name {
            let mut frames = Vec::with_capacity(anim_data.sprites.len());
            for sprite_id in &anim_data.sprites {
                let sprite_id = *sprite_id;
                let tmp_sprite = &sprite_data[sprite_id];
                let atlas_index = atlas
                    .get_texture_index(&tmp_sprite.texture)
                    .expect("Failed to get texture from atlas");
                frames.push(Frame {
                    sprite: animation::Sprite {
                        atlas_index: atlas_index as u32,
                    },
                    duration_ms: tmp_sprite.duration,
                });
            }
            let anim_id = handle_id::animation(path, &tag_name);
            let asset = Animation::new(frames, atlas_handle.clone());
            let handle = animations.set(anim_id, asset);
            file_assets.insert_animation(tag_name, handle);
        }
    }
}

struct SpriteImportResources<'a> {
    images: &'a mut Assets<Image>,
    atlases: &'a mut Assets<TextureAtlas>,
}

fn move_sprites(
    path: &str,
    sprites: Vec<SpriteData<Image>>,
    resources: SpriteImportResources,
    file_assets: &mut AseAssetMap,
) -> (Vec<SpriteData<Handle<Image>>>, Handle<TextureAtlas>) {
    let SpriteImportResources { images, atlases } = resources;
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let sprite_handles: Vec<SpriteData<Handle<Image>>> = sprites
        .into_iter()
        .map(
            |SpriteData {
                 frame,
                 texture: image,
                 duration,
             }| {
                let image_handle_id = handle_id::frame_image(path, frame);
                let image_handle = images.set(image_handle_id, image);
                file_assets.insert_texture(frame, image_handle.clone());
                // Expect: We just inserted this image above
                let image = images.get(&image_handle).expect("Image missing");
                texture_atlas_builder.add_texture(image_handle.clone_weak(), image);
                SpriteData {
                    texture: image_handle,
                    frame,
                    duration,
                }
            },
        )
        .collect();
    let atlas = texture_atlas_builder
        .finish(images)
        .expect("Creating texture atlas failed");
    // Since we likely are dealing with pixel art, default to nearest image sampling
    images
        .get_mut(&atlas.texture)
        .expect("Texture atlas texture missing")
        .sampler_descriptor = ImageSampler::nearest();
    let atlas_handle_id = handle_id::atlas(path);
    let atlas_handle = atlases.set(atlas_handle_id, atlas);
    file_assets.insert_atlas(atlas_handle.clone());
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
    pub(crate) sprites: Vec<SpriteData<Image>>,
    pub(crate) anims: Vec<AnimationData>,
    pub(crate) tilesets: Vec<TilesetData<Image>>,
    pub(crate) slices: Vec<Slice>,
}
impl ResourceData {
    pub(crate) fn new(path: &Path, file: &AsepriteFile) -> Self {
        let mut tmp_sprites: Vec<SpriteData<Image>> = Vec::new();
        let mut tmp_anim_info: Vec<AnimationData> = Vec::new();
        let mut slices: Vec<Slice> = Vec::new();
        let mut tilesets: Vec<TilesetData<Image>> = Vec::new();
        debug!("Processing Aseprite file: {}", path.display());
        let sprite_offset = tmp_sprites.len();
        for frame in 0..file.num_frames() {
            tmp_sprites.push(SpriteData::<Image>::new(file, frame));
        }
        tmp_anim_info.push(AnimationData::new(file, sprite_offset));
        for tag_id in 0..file.num_tags() {
            let tag = file.tag(tag_id);
            tmp_anim_info.push(AnimationData::from_tag(sprite_offset, tag));
        }
        let mut ase_tilesets =
            tilesets_from(file).expect("Internal error: Failed to add tilesets from Ase file");
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
    pub(crate) fn move_into_resources(self, path_buf: PathBuf, resources: &mut AseAssetResources) {
        let data = self;
        let path_str = path_buf.to_str().expect("Expected valid Unicode path!");
        let (textures, animations, atlases, tilesets, slices, index) = resources;

        let file_assets = index
            .as_deref_mut()
            .map(|ase_file_map| ase_file_map.get_mut(&path_buf))
            .expect("Expected a file map!");

        if let Some(slices) = slices {
            move_slices(path_str, data.slices, slices, file_assets);
        }

        if let Some(tilesets) = tilesets {
            let resources = TilesetImportResources { textures, tilesets };
            move_tilesets(path_str, data.tilesets, resources, file_assets);
        }

        // Move sprites
        if let Some(atlases) = atlases {
            let resources = SpriteImportResources {
                images: textures,
                atlases,
            };

            let (sprites, atlas_handle) =
                move_sprites(path_str, data.sprites, resources, file_assets);
            let atlas = atlases.get(&atlas_handle).unwrap();
            // Move animations
            if let Some(animations) = animations {
                let data = AnimationImportData {
                    animation_data: data.anims,
                    sprite_data: sprites,
                    atlas,
                    atlas_handle,
                };

                move_animations(path_str, data, animations, file_assets);
            }
        }
    }
}
