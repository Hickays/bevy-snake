use bevy::prelude::*;

mod camera;
mod config;
mod square;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(
        // 自定义 window 窗口
        config::window::build_custom_window_plugin(),
    ));

    // 相机
    app.add_plugins(camera::CameraPlugin);
    // 方块
    app.add_plugins(square::SquarePlugin);

    app.run();
}
