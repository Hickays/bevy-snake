//! 游戏配置
use bevy::prelude::*;

pub mod window;

// pub const GRID_SIZE: u16 = 21;
// pub const CELL_SIZE: f32 = 20.;
// pub const GRID_GAP: f32 = 1.;

/// 游戏配置
#[derive(Resource)]
pub struct GameConfig {
    /// 网格范围
    pub grid_extent: UVec2,
    /// 单元格范围
    pub cell_size: f32,
    /// 单元格间距
    pub cell_gap: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            grid_extent: UVec2::splat(21),
            cell_size: 20.,
            cell_gap: 1.,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
  app.insert_resource(GameConfig::default());
}