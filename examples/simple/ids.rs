use bevy_ase::animation_index::AnimationId;
use strum::{EnumIter, EnumProperty, IntoEnumIterator};

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumProperty, EnumIter)]
pub enum AnimId {
    #[strum(props(file = "sprites/hello.aseprite"))]
    Dummy,
    #[strum(props(file = "sprites/hello.aseprite", tag = "Blue"))]
    DummySad,
}

impl AnimationId for AnimId {
    fn as_key(&self) -> (&str, Option<&str>) {
        let path = self
            .get_str("file")
            .expect("Attribute \"file\" is required");
        let tag = self.get_str("tag");
        (path, tag)
    }

    fn list_all() -> Box<dyn Iterator<Item = Self>> {
        Box::new(AnimId::iter())
    }
}
