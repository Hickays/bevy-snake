use bevy::prelude::*;

mod camera;
mod config;
mod grids;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(config::window::build_custom_window_plugin()));

    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(grids::GridsPlugin);

    app.run();
}
