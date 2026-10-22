use std::ops::DerefMut;

use bevy::prelude::*;
use rand::seq::{IndexedMutRandom, IndexedRandom};

use crate::game::entity::{Cell, CellType};
use crate::game::message::CreateFoodMessage;
use crate::game::res::GridMap;

#[derive(Resource)]
pub struct SnakeFood {
    food_list: Vec<IVec2>,
}

impl SnakeFood {
    pub fn new() -> SnakeFood {
        SnakeFood {
            food_list: Vec::new(),
        }
    }

    pub fn try_eat_food(&mut self, food_pos: &IVec2) -> bool {
        info!("尝试吃: {food_pos:?}, [{:?}]", &self.food_list);
        if !self.food_list.contains(food_pos) {
            return false;
        }

        self.food_list.retain(|p| p == food_pos);
        info!("吃到了");
        return true;
    }

    pub fn add_food(&mut self, food_pos: IVec2) {
        self.food_list.push(food_pos);
    }
}

pub fn create_food(
    mut snake_food: ResMut<SnakeFood>,
    // grid_map: Res<GridMap>,
    mut cells: Query<&mut Cell>,
    mut create_food_message: MessageReader<CreateFoodMessage>,
) {
    for _ in create_food_message.read() {
        let mut empty_cells = cells
            .iter_mut()
            .filter(|cell| cell.is_type(CellType::Empty))
            .collect::<Vec<Mut<Cell>>>();

        let Some(empty_cell) = empty_cells.choose_mut(&mut rand::rng()) else {
            continue;
        };
        empty_cell.as_type(CellType::Food);
        snake_food.add_food(empty_cell.grid_pos.clone());
    }
}
