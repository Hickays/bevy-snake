use bevy::prelude::*;

mod entity;
mod message;
mod res;

use message::CreateFoodMessage;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, entity::spawn_grid);
        app.add_systems(Update, entity::update_cell);

        app.insert_resource(res::GridMap::new());
        app.insert_resource(res::SnakeState::new());
        app.insert_resource(res::SnakeMoveTimer::new());
        app.insert_resource(res::food::SnakeFood::new());

        app.add_message::<message::CreateFoodMessage>();

        app.add_systems(
            Update,
            (
                handle_key_input,
                move_snake,
                res::update_snake,
                res::food::create_food,
            )
                .chain(),
        );
    }
}

fn handle_key_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut snake_state: ResMut<res::SnakeState>,

    mut create_food_message: MessageWriter<CreateFoodMessage>,
) {
    for key in keys.get_just_pressed() {
        let direction = match key {
            KeyCode::KeyW => IVec2::Y,
            KeyCode::KeyS => -IVec2::Y,
            KeyCode::KeyA => -IVec2::X,
            KeyCode::KeyD => IVec2::X,
            _ => IVec2::ZERO,
        };

        if direction != IVec2::ZERO {
            snake_state.try_buffer_direction(direction);
            create_food_message.write(CreateFoodMessage {});
        }
    }
}

fn move_snake(
    mut snake_state: ResMut<res::SnakeState>,
    time: Res<Time<Real>>,
    mut timer: ResMut<res::SnakeMoveTimer>,
    mut food: ResMut<res::food::SnakeFood>
) {
    timer.tick(time.delta());

    // 指定时刻才进行移动
    if !timer.just_finished() {
        return;
    }

    // 前进一步
    snake_state.step_forward(food);
}
