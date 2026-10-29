use bevy::prelude::*;

mod core;
mod food;
mod game;
mod input;
mod snake;
// mod status;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(core::window::build_custom_window_plugin()));

    app.add_plugins(game::plugin);

    app.run();
}
