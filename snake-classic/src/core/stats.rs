//! 统计数据
use bevy::prelude::*;

use crate::core::state::GameState;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GameStatus>();

    app.add_systems(OnEnter(GameState::Ready), reset_status_for_ready);
}

fn reset_status_for_ready(mut game_status: ResMut<GameStatus>) {
    game_status.reset()
}

#[derive(Resource, Default)]
pub struct GameStatus {
    pub score: u16,
}

impl GameStatus {
    pub fn add_score(&mut self) {
        self.score += 1;
    }

    fn reset(&mut self) {
        self.score = 0;
    }
}
