use bevy::prelude::*;

pub fn move_square(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut squares: Query<(&Sprite, &mut Transform)>,
    windows: Query<&Window>,
) {
    let (direction, speed) = {
        let mut direction = Vec2::ZERO;

        // 计算方向
        for key in keys.get_pressed() {
            direction += match key {
                KeyCode::KeyW | KeyCode::ArrowUp => Vec2::Y,
                KeyCode::KeyS | KeyCode::ArrowDown => -Vec2::Y,
                KeyCode::KeyA | KeyCode::ArrowLeft => -Vec2::X,
                KeyCode::KeyD | KeyCode::ArrowRight => Vec2::X,
                _ => Vec2::ZERO,
            }
        }

        let direction = direction.normalize_or_zero();

        let speed_up = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

        let speed = match () {
            _ if direction == Vec2::ZERO => 0.,
            _ if speed_up => 300.,
            _ => 200.,
        };

        (direction, speed)
    };

    if speed <= 0. {
        return;
    };

    for (_, mut transform) in &mut squares {
        let direction = direction * speed * time.delta_secs();
        let new_position = direction.extend(transform.translation.z);
        transform.translation += new_position;
    }
}
