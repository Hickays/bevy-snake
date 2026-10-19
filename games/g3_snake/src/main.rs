use bevy::prelude::*;

mod camera;
mod config;
// mod grids;

mod game;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(config::window::build_custom_window_plugin()));

    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(game::SnakePlugin);

    app.run();
}
