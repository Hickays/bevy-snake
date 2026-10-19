use bevy::prelude::*;

mod entity;
mod res;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, entity::spawn_grid);
        app.add_systems(Update, entity::update_cell);

        app.insert_resource(res::SnakeState::new());
        app.add_systems(Update, res::update_snake);

        app.add_systems(Update, move_snake);
    }
}

fn move_snake(keys: Res<ButtonInput<KeyCode>>, mut snake_state: ResMut<res::SnakeState>) {
    let mut direction = IVec2::ZERO;

    for key in keys.get_just_pressed() {
        direction += match key {
            KeyCode::KeyW => IVec2::Y,
            KeyCode::KeyS => -IVec2::Y,
            KeyCode::KeyA => -IVec2::X,
            KeyCode::KeyD => IVec2::X,
            _ => IVec2::ZERO,
        };
        continue;
    }

    if direction == IVec2::ZERO {
        return;
    }

    let Some(back) = snake_state.body_list.pop_back() else {
        return;
    };
    snake_state.body_list.push_front(back + direction);
}
