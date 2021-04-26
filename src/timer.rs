use bevy::prelude::*;
use std::time::Duration;

/// Millisecond-accurate game time stamp.
// 1 year = 31_536_000_000 ms, 2^63 ms = ~250 million years
// We can live with that :)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameTime(u64); // Use NonZeroU64? Then Option<GameTime> could use 0.

impl GameTime {
    pub const ZERO: Self = GameTime(0);

    pub fn from_millis(millis: u64) -> GameTime {
        GameTime(millis)
    }

    pub fn add_millis(&mut self, millis: u32) {
        self.0 += millis as u64;
    }
}

pub struct GameTimer {
    duration: Duration,
    time: GameTime,
}

impl Default for GameTimer {
    fn default() -> Self {
        GameTimer {
            duration: Duration::from_millis(1),
            time: GameTime(1),
        }
    }
}

impl GameTimer {
    pub fn add_dt(&mut self, dt: Duration) {
        self.duration += dt;
        self.time = GameTime(self.duration.as_millis() as u64);
    }

    pub fn time(&self) -> GameTime {
        self.time
    }
}

pub fn game_timer(time: Res<Time>, mut game_time: ResMut<GameTimer>) {
    game_time.add_dt(time.delta());
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub struct Label;

/// Adds [GameTimer] resource which advances a [GameTime] value at millisecond
/// accuracy. This means the time stamp can be represented as a 64 bit value.
///
/// A freshly-constructed timer starts at 1ms. This frees up the 0ms value to
/// be used as an unitialized time stamp.
pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GameTimer>()
            .add_system(game_timer.system().label(Label));
    }
}
