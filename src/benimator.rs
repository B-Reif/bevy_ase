use crate::asset::{Animation, Frame};
use std::time::Duration;

impl From<&Frame> for benimator::Frame {
    fn from(f: &Frame) -> Self {
        benimator::Frame {
            duration: Duration::from_millis(f.duration_ms as u64),
            index: f.sprite.atlas_index,
        }
    }
}
impl From<&Animation> for benimator::SpriteSheetAnimation {
    fn from(a: &Animation) -> Self {
        let frames = a.frames().iter().map(|f| f.into()).collect();
        benimator::SpriteSheetAnimation::from_frames(frames)
    }
}
