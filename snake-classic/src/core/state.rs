//! 状态
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameState>();
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    /// 就绪
    #[default]
    Ready,
    /// 运行中
    Playing,
    /// 暂停
    Paused,
    /// 结束
    GameOver,
}
