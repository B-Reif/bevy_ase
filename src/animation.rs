use asefile::{AsepriteFile, Tag};
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Animation {
    pub(crate) file: PathBuf,
    pub(crate) tag: Option<String>,
    pub(crate) sprites: Vec<usize>,
}
impl Animation {
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
        Animation {
            file: name.clone(),
            tag: Some(tag.name().to_owned()),
            sprites: (tag.from_frame()..tag.to_frame() + 1)
                .map(|f| sprite_offset + f as usize)
                .collect(),
        }
    }
}
