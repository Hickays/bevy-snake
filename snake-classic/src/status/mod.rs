//! 游戏状态
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameState>();
}

#[derive(States, Default, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GameState {
    #[default]
    Ready,
    Running,
    Paused,
    GameOver,
}

#[derive(Resource)]
pub struct GameStatus {}
