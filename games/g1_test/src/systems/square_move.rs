use bevy::prelude::*;

pub fn move_square(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    // 查询所有带 Sprite 的实体
    mut query: Query<(&Sprite, &mut Transform)>,
    windows: Query<&Window>,
) {
    //! 使用按键移动方块

    // for mut transform in &mut query {
    //     transform.translation.x += 2. * time.delta_secs();
    // }

    let mut direction = Vec2::ZERO;
    let Ok(window) = windows.single() else { return };

    for key in keys.get_pressed() {
        direction += match key {
            KeyCode::KeyW | KeyCode::ArrowUp => Vec2::Y,
            KeyCode::KeyS | KeyCode::ArrowDown => -Vec2::Y,
            KeyCode::KeyA | KeyCode::ArrowLeft => -Vec2::X,
            KeyCode::KeyD | KeyCode::ArrowRight => Vec2::X,
            _ => Vec2::ZERO,
        }
    }

    if direction == Vec2::ZERO {
        return;
    }

    let direction = direction.normalize();

    // 加速
    let speed_up = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    let speed = if speed_up { 300. } else { 200. };

    let window_size = window.size();
    let min = Vec2::new(window_size.x / -2., window_size.y / -2.);
    let max = Vec2::new(window_size.x / 2., window_size.y / 2.);

    for (square, mut transform) in &mut query {
        // let square_position = transform.translation.truncate();

        let new_x = transform.translation.x + direction.x * speed * time.delta_secs();
        let new_y = transform.translation.y + direction.y * speed * time.delta_secs();

        // info!("尝试移动到: ({}, {})", new_x, new_y);

        let half_size = square.custom_size.unwrap_or(Vec2::ZERO) / 2.;
        if !(min + half_size).cmple(max - half_size).all() {
            continue;
        }
        let new_position = Vec2::new(new_x, new_y).clamp(min + half_size, max - half_size);

        transform.translation = new_position.extend(transform.translation.z);
    }
}