use std::collections::VecDeque;

use bevy::prelude::*;

use super::entity::Cell;
use super::entity::CellType;

#[derive(Resource)]
pub struct SnakeState {
    pub body_list: VecDeque<IVec2>,
}

impl SnakeState {
    pub fn new() -> SnakeState {
        let mut body_list = VecDeque::new();
        body_list.push_front(IVec2::ZERO);
        SnakeState { body_list }
    }
}

pub fn update_snake(mut cells: Query<&mut Cell>, snake_state: Res<SnakeState>) {
    if !snake_state.is_changed() {
        return;
    }

    for mut cell in &mut cells {
      info!("更新蛇");
        if cell.is_type(CellType::Snake) {
            cell.as_type(CellType::Empty);
        }
        for body in &snake_state.body_list {
            if cell.eq_grid_pos(body.clone()) {
                cell.as_type(CellType::Snake);
            }
        }
    }
}
