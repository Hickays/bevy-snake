use bevy::prelude::*;

pub fn click_square(
    // mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    squares: Query<(&Transform, &Sprite)>,
) {
    //! 通过手动计算实现点击方块功能
    
    // 左键按下瞬间触发, 比拾取 (按下后松开触发) 触发时机更早
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else {
        info!("未找到或找到多个窗口");
        return;
    };
    let Ok((camera, camera_transform)) = cameras.single() else {
        info!("未找到或找到多个相机");
        return;
    };
    let Some(cursor_position) = window.cursor_position() else {
        info!("当前窗口没有鼠标光标位置");
        return;
    };

    // info!("光标位置: {:?}", cursor_position);

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        info!("光标位置无法转换为世界坐标");
        return;
    };

    // info!("世界坐标: {:?}", world_position);

    for (transform, square) in &squares {
        let Some(size) = square.custom_size else {
            info!("Sprite 没有自定义大小");
            continue;
        };

        let half_size = size / 2.0;
        let min = transform.translation.truncate() - half_size;
        let max = transform.translation.truncate() + half_size;

        if world_position.x >= min.x
            && world_position.x <= max.x
            && world_position.y >= min.y
            && world_position.y <= max.y
        {
            info!("点击了方块! (手动计算)");
        }
    }
}
