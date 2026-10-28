use bevy::prelude::*;

use crate::{camera, config, food, grid, input, snake, status, ui};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(camera::plugin);
    app.add_plugins(config::plugin);
    app.add_plugins(input::plugin);
    app.add_plugins(grid::plugin);
    app.add_plugins(snake::plugin);
    app.add_plugins(food::plugin);
    app.add_plugins(status::plugin);
    app.add_plugins(ui::plugin);

    app.add_systems(
        OnEnter(status::GameState::Ready),
        (grid::ready, snake::ready, food::ready).chain(),
    );
}
