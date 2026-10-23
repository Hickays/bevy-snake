use bevy::prelude::*;

mod camera;
mod config;
mod grid;
mod input;
mod snake;
mod food;
// mod grids;

// mod game;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(config::window::build_custom_window_plugin()));

    app.add_plugins(camera::plugin);
    app.add_plugins(config::plugin);
    app.add_plugins(input::plugin);
    app.add_plugins(grid::plugin);
    app.add_plugins(snake::plugin);
    app.add_plugins(food::plugin);
    // app.add_plugins(game::SnakePlugin);

    app.run();
}
