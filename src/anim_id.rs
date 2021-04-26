//! This module allows us to declare animation IDs in code and avoid having to
//! deal with strings all over the place. It also makes animation lookup a bit
//! faster because we don't have to hash the string.
//!
//! The ID type must have a way to enumerate through all possible IDs so we can
//! pre-load the dictionary. There might be a way to make this more lazy?
//!
//! To avoid having to implement the `AnimationId` trait manually, you should
//! use something like `strum`. See the examples directory for details.

use bevy::{prelude::*, utils::HashMap};
use std::{fmt::Debug, hash::Hash, path::Path};

use crate::animate::{Animation, AnimationBundle, AnimationInfo, PlayAnimation};

pub struct AnimationById<Id> {
    map: HashMap<Id, AnimInfo>,
}

struct AnimInfo {
    anim_handle: Handle<Animation>,
    atlas_sprites: Vec<u32>, // frame to atlas sprite mapping, len >= 1
    atlas_handle: Handle<TextureAtlas>,
}

impl<Id> Default for AnimationById<Id> {
    fn default() -> Self {
        AnimationById {
            map: HashMap::default(),
        }
    }
}

impl<Id> AnimationById<Id>
where
    Id: Eq + Hash + AnimationId + Debug,
{
    pub fn initialize(
        &mut self,
        ids: impl Iterator<Item = Id>,
        anim_info: &AnimationInfo,
        animations: &Assets<Animation>,
    ) {
        for id in ids {
            let (path, tag) = id.name();
            let path = Path::new(path);

            let anim_handle = if let Some(tag) = tag {
                anim_info.lookup_with_tag(&path, tag)
            } else {
                anim_info.lookup(&path)
            };

            if let Some(h) = anim_handle {
                let anim = animations.get(&h).unwrap();
                self.map.insert(
                    id,
                    AnimInfo {
                        anim_handle: h.clone(),
                        atlas_sprites: anim
                            .frames()
                            .iter()
                            .map(|frame| frame.sprite.atlas_index)
                            .collect(),
                        atlas_handle: anim.frame(0).sprite.atlas.clone(),
                    },
                );
            } else {
                warn!(
                    "Could not find animation for: {:?} (path:{}, tag:{:?})",
                    id,
                    path.display(),
                    tag
                );
            }
        }
    }

    pub fn get(&self, id: Id) -> AnimationBundle {
        // TODO: Error handling: log error + return dummy bundle
        let info = {
            let info = self.map.get(&id);
            if info.is_none() {
                error!("Animation ID not found: {:?}", id);
            }
            info.unwrap()
        };
        //if let Some(info) = self.map.get(&id).unwrap();
        debug_assert!(!info.atlas_sprites.is_empty());
        AnimationBundle {
            sprite: SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(info.atlas_sprites[0]),
                texture_atlas: info.atlas_handle.clone(),
                ..Default::default()
            },
            play: PlayAnimation::new(info.anim_handle.clone()),
        }
    }

    pub fn sprite(&self, id: Id, frame: u32) -> SpriteSheetBundle {
        // TODO: Error handling: log error + return dummy bundle
        let info = self.map.get(&id).unwrap();
        SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(info.atlas_sprites[frame as usize]),
            texture_atlas: info.atlas_handle.clone(),
            ..Default::default()
        }
    }
}

pub trait AnimationId {
    fn name(&self) -> (&str, Option<&str>);
    fn list_all() -> Box<dyn Iterator<Item = Self>>;
}
