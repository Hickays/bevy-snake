//! 游戏状态
use bevy::prelude::*;

use crate::input;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Ready), resert_status);
    app.add_systems(Update, handle_game_action);
}

fn resert_status(mut game_status: ResMut<GameStatus>) {
    // info!("重置分数");
    game_status.reset();
}

fn handle_game_action(
    mut game_actions: MessageReader<input::GameAction>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
) {
    let game_state = state.get();
    for game_action in game_actions.read() {
        match game_action {
            input::GameAction::StartGame => {
                if matches!(game_state, GameState::Ready) {
                    next.set(GameState::Playing);
                }
            }
            input::GameAction::Retry => {
                if !matches!(game_state, GameState::Ready) {
                    next.set(GameState::Ready);
                }
            }
            input::GameAction::TogglePause => {
                match game_state {
                    GameState::Paused => next.set(GameState::Playing),
                    GameState::Playing => next.set(GameState::Paused),
                    _ => {},
                };
            }
        }
    }
}

