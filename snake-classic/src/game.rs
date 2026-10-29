use bevy::prelude::*;

use crate::{core, food, input, snake, ui};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(core::plugin);
    app.add_plugins(input::plugin);
    app.add_plugins(snake::plugin);
    app.add_plugins(food::plugin);
    app.add_plugins(ui::plugin);

    app.add_systems(
        OnEnter(core::state::GameState::Ready),
        (
            core::grid::reset_grid_for_ready,
            snake::reset_snake_for_ready,
            food::reset_food_for_ready,
        )
            .chain(),
    );
}
