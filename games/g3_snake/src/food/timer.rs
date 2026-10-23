use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct FoodCreateTimer(Timer);

impl FoodCreateTimer {
    pub fn new() -> FoodCreateTimer {
        FoodCreateTimer(Timer::from_seconds(3., TimerMode::Repeating))
    }
}
