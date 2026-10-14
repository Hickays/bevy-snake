use bevy::prelude::*;

mod window;
mod square;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(
        // 自定义 window 窗口
        window::build_custom_window_plugin(),
    ));

    app.add_plugins(square::TestBlockPlugin);

    app.run();
}
