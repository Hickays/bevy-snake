//! 行为
use bevy::prelude::*;

use crate::core::state::GameState;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<GameAction>();

    app.add_systems(Update, handle_game_action);
}

/// 游戏行为消息
#[derive(Message, Debug)]
pub enum GameAction {
    StartGame,
    TogglePause,
    Retry,
}

fn handle_game_action(
    mut game_actions: MessageReader<GameAction>,
    state: Res<State<GameState>>,
    mut next: ResMut<NextState<GameState>>,
) {
    let game_state = state.get();
    for game_action in game_actions.read() {
        match game_action {
            GameAction::StartGame => {
                if matches!(game_state, GameState::Ready) {
                    next.set(GameState::Playing);
                }
            }
            GameAction::Retry => {
                if !matches!(game_state, GameState::Ready) {
                    next.set(GameState::Ready);
                }
            }
            GameAction::TogglePause => {
                match game_state {
                    GameState::Paused => next.set(GameState::Playing),
                    GameState::Playing => next.set(GameState::Paused),
                    _ => {}
                };
            }
        }
    }
}
