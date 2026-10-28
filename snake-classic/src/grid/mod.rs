//! 网格
use bevy::prelude::*;

use std::collections::HashMap;

pub mod message;

use crate::config::GameConfig;

/// 网格相关
pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GridMap::default());
    app.add_message::<message::SyncCellTypeMessage>();
    app.add_systems(Startup, startup_spawn);
    app.add_systems(Update, (handle_sync_cell_type_message, sync_cell).chain());
}

/// 网格类型: 空格, 蛇体节格, 食物格
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CellType {
    Empty,
    Snake,
    Food,
}

impl CellType {
    fn get_color(&self) -> Color {
        match self {
            CellType::Empty => Color::srgb_u32(0x_444444),
            CellType::Food => Color::srgb_u32(0x_ff4242),
            CellType::Snake => Color::srgb_u32(0x_aaaaaa),
        }
    }
}

/// 网格组件, 记录真实坐标, 转换坐标, 网格类型
#[derive(Component, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,

    pub grid_pos: IVec2,
    world_pos: Vec2,
}

/// 网格地图资源, 保存坐标和网格实体的对应关系
#[derive(Resource, Default, Deref)]
pub struct GridMap(HashMap<IVec2, Entity>);

/// 创建网格
fn startup_spawn(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    mut grid_map: ResMut<GridMap>,
) {
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

        grid_map.0.insert(cell.grid_pos, entity);
    }
}

pub fn ready(cells: Query<&mut Cell>) {
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
fn sync_cell(query: Query<(&mut Sprite, &Cell), Changed<Cell>>) {
    for (mut sprite, cell) in query {
        sprite.color = cell.cell_type.get_color()
    }
}

/// 处理同步网格的消息
fn handle_sync_cell_type_message(
    mut messages: MessageReader<message::SyncCellTypeMessage>,
    grid_map: Res<GridMap>,
    mut cells: Query<&mut Cell>,
) {
    for message in messages.read() {
        for (pos, cell_type) in &message.changes {
            let Some(entity) = grid_map.get(pos) else {
                continue;
            };

            let Ok(mut cell) = cells.get_mut(*entity) else {
                continue;
            };

            if &cell.cell_type != cell_type {
                cell.cell_type = *cell_type
            }
        }
    }
}
