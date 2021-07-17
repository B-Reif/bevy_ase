use std::{fmt, path::PathBuf};

use asefile::AsepriteFile;
use bevy::utils::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AseId(u32);
impl AseId {
    pub fn new(inner: u32) -> Self {
        Self(inner)
    }
    pub fn inner(&self) -> &u32 {
        &self.0
    }
}
impl fmt::Display for AseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AseId({})", self.0)
    }
}

pub(crate) struct AseKeyed {
    pub(crate) id: AseId,
    pub(crate) path: PathBuf,
    pub(crate) file: AsepriteFile,
}

pub(crate) struct AsesById(HashMap<AseId, AseKeyed>);
impl AsesById {
    pub(crate) fn inner(&self) -> &HashMap<AseId, AseKeyed> {
        &self.0
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, AseId, AseKeyed> {
        self.0.iter()
    }
}
impl From<Vec<(PathBuf, AsepriteFile)>> for AsesById {
    fn from(vec: Vec<(PathBuf, AsepriteFile)>) -> Self {
        Self(
            vec.into_iter()
                .enumerate()
                .map(|(idx, (path, file))| {
                    let id = AseId::new(idx as u32);
                    let value = AseKeyed { id, path, file };
                    (id, value)
                })
                .collect(),
        )
    }
}
