//! Types for slice data.
pub use asefile::{Slice9, SliceKey};
use bevy::reflect::TypeUuid;

/// A slice is a region of an Ase sprite with a name and optional user data.
#[derive(Debug, TypeUuid)]
#[uuid = "d12e0ddb-b47b-4d50-ae12-73eb970feae2"]
pub struct Slice {
    /// The name of the slice. Not guaranteed to be unique.
    pub name: String,
    /// A set of [asefile::SliceKey] structs. Together, these describe the shape and position of a slice during animation.
    pub keys: Vec<asefile::SliceKey>,
    /// Optional [asefile::UserData] associated with this slice.
    pub user_data: Option<asefile::UserData>,
}
impl Slice {
    pub(crate) fn from_ase(ase_slice: &asefile::Slice) -> Self {
        let asefile::Slice {
            name,
            keys,
            user_data,
        } = ase_slice;

        Self {
            name: name.to_string(),
            keys: keys.to_vec(),
            user_data: user_data.clone(),
        }
    }
}
