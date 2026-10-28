use bevy::prelude::*;

mod camera;
mod config;
mod food;
mod game;
mod grid;
mod input;
mod snake;
mod status;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(config::window::build_custom_window_plugin()));

    app.add_plugins(game::plugin);

    app.run();
}
