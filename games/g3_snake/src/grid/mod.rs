use std::collections::HashMap;

use bevy::prelude::*;

use crate::config::GameConfig;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(GridMap::default());
    app.add_systems(Startup, startup_spawn);
    app.add_systems(Update, sync_cell);
}

#[derive(Clone, Copy)]
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
    cell_type: CellType,

    grid_pos: IVec2,
    world_pos: Vec2,
}

#[derive(Resource, Default)]
pub struct GridMap(HashMap<IVec2, Entity>);

fn startup_spawn(mut commands: Commands, game_config: Res<GameConfig>, mut grid_map: ResMut<GridMap>) {
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
