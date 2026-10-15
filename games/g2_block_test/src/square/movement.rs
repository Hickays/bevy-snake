use bevy::prelude::*;

/// 按 WASD / 方向键移动方块, 按住 Shift 加速, 并限制在窗口内
pub fn move_square(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    squares: Query<(&Sprite, &mut Transform)>,
    windows: Query<&Window>,
) {
    // 计算方向
    let mut direction = Vec2::ZERO;
    for key in keys.get_pressed() {
        direction += match key {
            KeyCode::KeyW | KeyCode::ArrowUp => Vec2::Y,
            KeyCode::KeyS | KeyCode::ArrowDown => -Vec2::Y,
            KeyCode::KeyA | KeyCode::ArrowLeft => -Vec2::X,
            KeyCode::KeyD | KeyCode::ArrowRight => Vec2::X,
            _ => Vec2::ZERO,
        }
    }

    // 归一化, 避免斜向移动更快
    let direction = direction.normalize_or_zero();
    if direction == Vec2::ZERO {
        return;
    }

    // 是否加速, 并计算速度
    let speed_up = keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let speed = if speed_up { 500. } else { 300. };

    // 获取窗口尺寸
    let Ok(window) = windows.single() else {
        return;
    };
    let window_size = window.size();

    // 本帧的移动量
    let displacement = direction * speed * time.delta_secs();

    for (square, mut transform) in squares {
        // 获取矩形尺寸
        let Some(square_size) = square.custom_size else {
            continue;
        };

        // 计算窗口边界 (矩形不能超出的部分: 窗口尺寸 - 矩形尺寸的一半)
        let window_boundary = (window_size - square_size) / 2.;
        if window_boundary.x < 0. || window_boundary.y < 0. {
            continue;
        }

        // 计算新坐标
        let new_position = transform.translation.truncate() + displacement;

        // 限制在边界内移动
        let new_position = new_position.clamp(-window_boundary, window_boundary);

        // 设置新坐标
        transform.translation = new_position.extend(transform.translation.z);
    }
}
