use bevy::prelude::*;

/// 创建食物定时器
#[derive(Resource, Deref, DerefMut)]
pub struct FoodCreateTimer(Timer);

impl FoodCreateTimer {
    /// 每三秒发送一次
    pub fn new() -> FoodCreateTimer {
        FoodCreateTimer(Timer::from_seconds(3., TimerMode::Repeating))
    }
}
