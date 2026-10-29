//! 输入
use bevy::prelude::*;

use crate::core::action::GameAction;

/// 方向输入消息
#[derive(Message)]
pub enum DirectionInput {
    Up,
    Down,
    Left,
    Right,
}

impl DirectionInput {
    /// 方向转为方向向量
    pub fn as_ivec2(&self) -> IVec2 {
        match self {
            DirectionInput::Up => IVec2::Y,
            DirectionInput::Down => -IVec2::Y,
            DirectionInput::Left => -IVec2::X,
            DirectionInput::Right => IVec2::X,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_message::<DirectionInput>();

    app.add_systems(Update, (emit_direction_input, emit_game_action));
}

/// 发送方向消息
fn emit_direction_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut direction_input: MessageWriter<DirectionInput>,
    mut game_action: MessageWriter<GameAction>,
) {
    for key in keys.get_just_pressed() {
        let direction = match key {
            KeyCode::KeyW | KeyCode::ArrowUp => DirectionInput::Up,
            KeyCode::KeyS | KeyCode::ArrowDown => DirectionInput::Down,
            KeyCode::KeyA | KeyCode::ArrowLeft => DirectionInput::Left,
            KeyCode::KeyD | KeyCode::ArrowRight => DirectionInput::Right,
            _ => continue,
        };
        game_action.write(GameAction::StartGame);
        direction_input.write(direction);
    }
}

/// 发送游戏行为消息
fn emit_game_action(keys: Res<ButtonInput<KeyCode>>, mut game_action: MessageWriter<GameAction>) {
    for key in keys.get_just_pressed() {
        let action = match key {
            // KeyCode::Space => GameAction::StartGame,
            KeyCode::Space => GameAction::TogglePause,
            KeyCode::KeyR => GameAction::Retry,
            _ => continue,
        };
        game_action.write(action);
    }
}
