use bevy::prelude::*;

use crate::{config::{CELL_SIZE, GRID_GAP, GRID_SIZE}, game::res::GridMap};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Empty,
    Food,
    Snake,
}

#[derive(Component, Clone, Copy)]
pub struct Cell {
    pub grid_pos: IVec2,
    world_pos: Vec2,
    cell_type: CellType,
}

impl Cell {
    pub fn is_type(&self, cell_type: CellType) -> bool {
        self.cell_type == cell_type
    }

    pub fn as_type(&mut self, cell_type: CellType) {
        self.cell_type = cell_type
    }
}

pub fn spawn_grid(mut commands: Commands, mut grid_map: ResMut<GridMap>) {
    for cell in build_cell_matrix() {
        let entity = commands.spawn((
            cell,
            Sprite {
                custom_size: Some(Vec2::splat(CELL_SIZE)),
                // color:
                ..Default::default()
            },
            Transform::from_translation(cell.world_pos.clone().extend(0.)),
        )).id();

        grid_map.add_cell(cell.grid_pos, entity);
    }
}

pub fn update_cell(query: Query<(&mut Sprite, &Cell), Changed<Cell>>) {
    for (mut sprite, cell) in query {
        sprite.color = match cell.cell_type {
            CellType::Empty => Color::srgb_u32(0x_444444),
            CellType::Food => Color::srgb_u32(0x_ff4242),
            CellType::Snake => Color::srgb_u32(0x_aaaaaa),
        }
    }
}

fn build_cell_matrix() -> Vec<Cell> {
    let mut matrix = Vec::new();
    let half_size = GRID_SIZE as i32 / 2;
    let word_offset = Vec2::splat(CELL_SIZE + GRID_GAP);
    for x in -half_size..=half_size {
        for y in -half_size..=half_size {
            let grid_pos = IVec2::new(x, y);
            matrix.push(Cell {
                grid_pos,
                world_pos: grid_pos.as_vec2() * word_offset,
                cell_type: CellType::Empty,
            });
        }
    }
    matrix
}
