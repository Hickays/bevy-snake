use bevy::prelude::*;

mod message;
mod timer;
pub use message::*;
use rand::seq::IndexedMutRandom;

use crate::grid::{Cell, CellType};

pub(super) fn plugin(app: &mut App) {
    app.add_message::<message::CreateFoodMessage>();
    app.insert_resource(timer::FoodCreateTimer::new());
    app.insert_resource(SnakeFoodState::default());

    app.add_systems(Startup, startup_create_food);
    app.add_systems(Update, create_food);
    app.add_systems(Update, interval_create_food);
}

#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakeFoodState {
    pub food_list: Vec<IVec2>,
}

impl SnakeFoodState {
    pub fn try_eat_food(&mut self, food_pos: &IVec2) -> bool {
        // info!("尝试吃: {food_pos:?}, [{:?}]", &self.food_list);
        if !self.food_list.contains(food_pos) {
            return false;
        }

        self.food_list.retain(|p| p != food_pos);
        // info!("吃到了");
        return true;
    }
}

pub fn startup_create_food(mut create_food_message: MessageWriter<CreateFoodMessage>) {
    create_food_message.write(CreateFoodMessage());
}

/// 间隔创建食物
fn interval_create_food(
    time: Res<Time<Real>>,
    mut timer: ResMut<timer::FoodCreateTimer>,
    mut create_food_message: MessageWriter<CreateFoodMessage>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        create_food_message.write(CreateFoodMessage());
    }
}

fn create_food(
    mut snake_food: ResMut<SnakeFoodState>,
    // grid_map: Res<GridMap>,
    mut cells: Query<&mut Cell>,
    mut create_food_message: MessageReader<CreateFoodMessage>,
) {
    for _ in create_food_message.read() {
        if snake_food.len() >= 10 {
            continue;
        }

        let mut empty_cells = cells
            .iter_mut()
            .filter(|cell| cell.cell_type == CellType::Empty)
            .collect::<Vec<Mut<Cell>>>();

        if let Some(empty_cell) = empty_cells.choose_mut(&mut rand::rng()) {
            empty_cell.cell_type = CellType::Food;
            snake_food.push(empty_cell.grid_pos.clone());
        };
    }
}
