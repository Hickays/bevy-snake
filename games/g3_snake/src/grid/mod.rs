use std::collections::HashMap;

use bevy::prelude::*;

use crate::config::GameConfig;
pub mod message;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GridMap::default());
    app.add_message::<message::SyncCellTypeMessage>();
    app.add_systems(Startup, startup_spawn);
    app.add_systems(Update, (handle_sync_cell_type_message, sync_cell).chain());
}

#[derive(Clone, Copy, PartialEq, Eq)]
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

#[derive(Component, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,

    pub grid_pos: IVec2,
    world_pos: Vec2,
}

#[derive(Resource, Default, Deref)]
pub struct GridMap(HashMap<IVec2, Entity>);

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

fn sync_cell(query: Query<(&mut Sprite, &Cell), Changed<Cell>>) {
    for (mut sprite, cell) in query {
        sprite.color = cell.cell_type.get_color()
    }
}

fn handle_sync_cell_type_message(
    mut messages: MessageReader<message::SyncCellTypeMessage>,
    grid_map: Res<GridMap>,
    mut cells: Query<&mut Cell>,
) {
    for message in messages.read() {
        info!("收到同步消息");
        for (pos, cell_type) in message.get_data() {
            let Some(entity) = grid_map.get(pos) else {
                continue;
            };

            let Ok(mut cell) = cells.get_mut(*entity) else {
                continue;
            };

            info!("同步网格");

            if &cell.cell_type != cell_type {
                cell.cell_type = *cell_type
            }
        }
    }
}
