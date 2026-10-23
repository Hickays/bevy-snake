use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct SnakeMoveTimer(Timer);

impl SnakeMoveTimer {
    pub fn new() -> SnakeMoveTimer {
        SnakeMoveTimer(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}
