use bevy::prelude::*;

use crate::grids::grid::GridType::SnakeBody;

enum GridType {
    Cell,
    Food,
    SnakeBody,
}

#[derive(Component)]
pub struct SnakeGrid {
    grid_type: GridType,
    
}

impl SnakeGrid {
    pub fn new() -> SnakeGrid {
        SnakeGrid {
            grid_type: GridType::Cell,
        }
    }

    pub fn get_color(&self) -> Color {
        match &self.grid_type {
            GridType::Cell => Color::srgb_u32(0x_ffffff),
            GridType::Food => Color::srgb_u32(0x_ff0000),
            GridType::SnakeBody => Color::srgb_u32(0x_000000),
        }
    }

    pub fn as_cell(&mut self) {
        self.grid_type = GridType::Cell
    }

    pub fn as_food(&mut self) {
        self.grid_type = GridType::Food
    }

    pub fn as_snake_body(&mut self) {
        self.grid_type = GridType::SnakeBody
    }
}
