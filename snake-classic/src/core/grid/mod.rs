//! 网格
use bevy::prelude::*;

use crate::core::config::GameConfig;

mod cell;
pub use cell::{Cell, CellType, GridMap};

/// 网格相关
pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GridMap>();

    app.add_systems(Update, sync_cell_sprites);
}

/// 准备阶段重置网格状态
pub fn reset_grid_for_ready(
    cells: Query<&mut Cell>,
    mut commands: Commands,
    game_config: Res<GameConfig>,
    mut grid_map: ResMut<GridMap>,
) {
    if !grid_map.is_inited() {
        for cell in build_cells(&game_config) {
            let entity = commands
                .spawn((
                    Sprite {
                        color: cell.cell_type.get_color(),
                        custom_size: Some(Vec2::splat(game_config.cell_size)),
                        ..Default::default()
                    },
                    Transform::from_translation(cell.world_pos.extend(0.)),
                    cell,
                ))
                .id();

            grid_map.add_entity(cell.grid_pos, entity);
        }
        grid_map.set_inited(true);
    }

    for mut cell in cells {
        cell.cell_type = CellType::Empty
    }
}

/// 构建网格
fn build_cells(config: &GameConfig) -> Vec<Cell> {
    let [width, height] = config.grid_extent.to_array();
    let mut matrix = Vec::with_capacity((width * height) as usize);

    let world_offset = Vec2::splat(config.cell_size + config.cell_gap);

    let origin_offset = world_offset * (config.grid_extent.as_vec2() - 1.) / 2.;

    for x in 0..width {
        for y in 0..height {
            let grid_pos = IVec2::new(x as i32, y as i32);
            matrix.push(Cell {
                grid_pos,
                world_pos: grid_pos.as_vec2() * world_offset - origin_offset,
                cell_type: CellType::Empty,
            });
        }
    }
    matrix
}

/// 同步网格 (主要同步颜色)
fn sync_cell_sprites(query: Query<(&mut Sprite, &Cell), Changed<Cell>>) {
    for (mut sprite, cell) in query {
        sprite.color = cell.cell_type.get_color()
    }
}
