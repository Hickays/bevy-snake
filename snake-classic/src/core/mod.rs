//! 核心包
use bevy::prelude::*;

pub mod action;
pub mod camera;
pub mod config;
pub mod grid;
pub mod state;
pub mod stats;
pub mod window;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(config::plugin);
    app.add_plugins(state::plugin);
    app.add_plugins(stats::plugin);
    app.add_plugins(camera::plugin);
    app.add_plugins(grid::plugin);
    app.add_plugins(action::plugin);
}
