//! Types for slice data.
use crate::ase::AseId;
pub use asefile::{Slice9, SliceKey, SliceOrigin, SlicePivot, SliceSize};
use bevy::reflect::TypeUuid;
use std::fmt::Display;

/// Identifier for a [Slice] within an Aseprite file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SliceId(u32);
impl SliceId {
    /// Creates a new [SliceId] with an inner [u32] value.
    pub fn new(inner: u32) -> Self {
        Self(inner)
    }
    /// Returns a reference to the id's inner value.
    pub fn inner(&self) -> &u32 {
        &self.0
    }
}
impl Display for SliceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SliceId({})", self.inner())
    }
}

/// Unique identifier for a [Slice] with an [AseId] and a [SliceId].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SliceAseKey {
    ase_id: AseId,
    slice_id: SliceId,
}
impl SliceAseKey {
    /// Creates a new [SliceAseKey] from an [AseId] and a [SliceId].
    pub fn new(ase_id: AseId, slice_id: SliceId) -> Self {
        Self { ase_id, slice_id }
    }
    /// Returns a reference to the key's [AseId].
    pub fn ase_id(&self) -> &AseId {
        &self.ase_id
    }
    /// Returns a reference to the key's [SliceId].
    pub fn slice_id(&self) -> &SliceId {
        &self.slice_id
    }
}
impl Display for SliceAseKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SliceKey ({}, {})", self.ase_id(), self.slice_id())
    }
}

/// A slice is a region of an Ase sprite with a name and optional user data.
#[derive(Debug, TypeUuid)]
#[uuid = "d12e0ddb-b47b-4d50-ae12-73eb970feae2"]
pub struct Slice {
    /// The key field uniquely identifies a Slice with an [AseId] and a [SliceId].
    pub key: SliceAseKey,
    /// The name of the slice. Not guaranteed to be unique.
    pub name: String,
    /// A set of [asefile::SliceKey] structs. Together, these describe the shape and position of a slice during animation.
    pub keys: Vec<asefile::SliceKey>,
    /// Optional [asefile::UserData] associated with this slice.
    pub user_data: Option<asefile::UserData>,
}
impl Slice {
    pub(crate) fn from_ase(ase_slice: &asefile::Slice, key: SliceAseKey) -> Self {
        let asefile::Slice {
            name,
            keys,
            user_data,
        } = ase_slice;

        Self {
            key,
            name: name.to_string(),
            keys: keys.to_vec(),
            user_data: user_data.clone(),
        }
    }
}
