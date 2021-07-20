use crate::ase::AseId;
use std::fmt::Display;

/// Identifier for a [Slice] within an Aseprite file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SliceId(u32);
impl SliceId {
    pub fn new(inner: u32) -> Self {
        Self(inner)
    }
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
    pub fn new(ase_id: AseId, slice_id: SliceId) -> Self {
        Self { ase_id, slice_id }
    }
    pub fn ase_id(&self) -> &AseId {
        &self.ase_id
    }
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
