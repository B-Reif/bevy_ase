use crate::asset::{Animation, Frame};
use std::time::Duration;

impl From<&Frame> for benimator::Frame {
    fn from(f: &Frame) -> Self {
        benimator::Frame::new(
            f.sprite.atlas_index as usize,
            Duration::from_millis(f.duration_ms as u64),
        )
    }
}
impl From<&Animation> for benimator::Animation {
    fn from(a: &Animation) -> Self {
        let frames = a.frames().iter().map(|f| f.into());
        benimator::Animation::from_frames(frames)
    }
}
