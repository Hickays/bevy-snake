//! 食物
use bevy::prelude::*;

use rand::seq::IndexedMutRandom;

mod message;
mod timer;

use crate::{
    core::grid::{Cell, CellType},
    core::state::GameState,
};

pub use message::*;

pub(super) fn plugin(app: &mut App) {
    app.add_message::<message::FoodCreateMessage>();
    app.insert_resource(timer::FoodCreateTimer::new());
    app.insert_resource(SnakeFoodState::default());

    app.add_systems(OnEnter(GameState::Ready), startup_create_food);

    app.add_systems(Update, create_food);

    app.add_systems(
        Update,
        (interval_create_food).run_if(in_state(GameState::Playing)),
    );
}

pub fn reset_food_for_ready(mut snake_food: ResMut<SnakeFoodState>, mut timer: ResMut<timer::FoodCreateTimer>) {
    timer.reset();
    snake_food.reset();
}

/// 食物状态资源
#[derive(Resource, Default, Deref, DerefMut)]
pub struct SnakeFoodState {
    pub foods: Vec<IVec2>,
}

impl SnakeFoodState {
    fn reset(&mut self) {
        self.foods.clear();
    }

    /// 尝试吃食物, 吃到后返回 true
    pub fn try_eat_food(&mut self, food_pos: &IVec2) -> bool {
        // info!("尝试吃: {food_pos:?}, [{:?}]", &self.foods);
        if !self.foods.contains(food_pos) {
            return false;
        }

        self.foods.retain(|p| p != food_pos);
        // info!("吃到了");
        return true;
    }
}

/// 初始化发送一次创建食物消息
pub fn startup_create_food(mut create_food_message: MessageWriter<FoodCreateMessage>) {
    create_food_message.write(FoodCreateMessage());
}

/// 每隔 FoodCreateTimer 发送一次创建食物消息
fn interval_create_food(
    time: Res<Time<Real>>,
    mut timer: ResMut<timer::FoodCreateTimer>,
    mut create_food_message: MessageWriter<FoodCreateMessage>,
) {
    timer.tick(time.delta());
    if timer.just_finished() {
        create_food_message.write(FoodCreateMessage());
    }
}

/// 创建食物
fn create_food(
    mut snake_food: ResMut<SnakeFoodState>,
    // grid_map: Res<GridMap>,
    mut cells: Query<&mut Cell>,
    mut create_food_message: MessageReader<FoodCreateMessage>,
) {
    // 接收创建食物消息
    for _ in create_food_message.read() {
        // 上限十个食物
        if snake_food.len() >= 10 {
            continue;
        }

        // 获取空网格
        let mut empty_cells = cells
            .iter_mut()
            .filter(|cell| cell.cell_type == CellType::Empty)
            .collect::<Vec<Mut<Cell>>>();

        // 随机选择一个空网格, 放入食物
        if let Some(empty_cell) = empty_cells.choose_mut(&mut rand::rng()) {
            empty_cell.cell_type = CellType::Food;
            snake_food.push(empty_cell.grid_pos.clone());
        };
    }
}
