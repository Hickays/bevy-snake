use bevy::prelude::*;

use std::collections::HashMap;
use std::collections::VecDeque;

pub mod food;
use crate::config::GRID_SIZE;
use crate::game::res::food::SnakeFood;

use super::entity::Cell;
use super::entity::CellType;

#[derive(Resource, Deref, DerefMut)]
pub struct SnakeMoveTimer(Timer);

impl SnakeMoveTimer {
    pub fn new() -> SnakeMoveTimer {
        SnakeMoveTimer(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

#[derive(Resource)]
pub struct SnakeState {
    /// body 列表
    pub body_list: VecDeque<IVec2>,
    /// 食物列表
    // pub food_list: VecDeque<IVec2>,
    /// 当前方向
    pub current_direction: IVec2,
    /// 输入队列, 接收输入时从后往前比较, 如果相反则覆盖
    direction_queue: VecDeque<IVec2>,
}

fn is_opposite_direction(direction1: IVec2, direction2: IVec2) -> bool {
    direction1 + direction2 == IVec2::ZERO
}
fn is_same_direction(direction1: IVec2, direction2: IVec2) -> bool {
    direction1 == direction2
}

impl SnakeState {
    pub fn new() -> SnakeState {
        let mut body_list = VecDeque::new();
        body_list.push_front(IVec2::ZERO);
        body_list.push_front(IVec2::ZERO);
        body_list.push_front(IVec2::ZERO);
        body_list.push_front(IVec2::ZERO);
        SnakeState {
            body_list,
            current_direction: IVec2::ZERO,
            direction_queue: VecDeque::new(),
        }
    }

    /// 尝试将输入加入缓冲队列
    pub fn try_buffer_direction(&mut self, direction: IVec2) {
        if direction == IVec2::ZERO {
            return;
        }

        // 获取输入队列最后一个输入, 如果输入队列为空则获取当前方向
        let before_direction = self
            .direction_queue
            .back()
            .unwrap_or(&self.current_direction);

        let queue_len = self.direction_queue.len();

        // 如果输入方向和 当前方向 / 最后输入方向 相同则忽略
        if is_same_direction(*before_direction, direction) {
            return;
        }

        // 如果输入方向和 当前方向 / 最后输入方向 相反
        if is_opposite_direction(*before_direction, direction) {
            // 如果是和 最后输入方向 相反则覆盖, queue_len > 0 表示取到的是最后输入方向
            if queue_len > 0 {
                self.direction_queue.pop_back();
                self.direction_queue.push_back(direction);
            }
            // 如果是和 当前方向 相反则忽略
            return;
        }

        // 如果有输入缓冲队列有空位则加入队列
        if queue_len < 2 {
            self.direction_queue.push_back(direction);
        }
    }

    /// 前进一步
    pub fn step_forward(&mut self, mut snake_food: ResMut<SnakeFood>) {
        // 获取头的位置, 随后基于头位置 + 方向计算头的下一步位置
        let Some(head_pos) = self.body_list.front() else {
            return;
        };
        let head_pos = head_pos.clone();

        // 获取并更新方向
        let new_direction = self
            .take_next_direcction()
            .unwrap_or(self.current_direction);
        self.current_direction = new_direction;

        let head_new_pos = head_pos + new_direction;

        // 根据边界范围修改头位置
        let boundary = IVec2::splat(GRID_SIZE as i32 / 2);
        let head_new_pos =
            (head_new_pos + boundary).rem_euclid(IVec2::splat(GRID_SIZE as i32)) - boundary;

        if !snake_food.try_eat_food(&head_new_pos) {
          // 去掉尾巴
          self.body_list.pop_back();
        }

        // 将头加入
        self.body_list.push_front(head_new_pos);
    }

    /// 拿一个输入
    fn take_next_direcction(&mut self) -> Option<IVec2> {
        self.direction_queue.pop_front()
    }

    fn create_rand_food(&mut self) {}
}

#[derive(Resource)]
// 网格地图
pub struct GridMap {
    cells: HashMap<IVec2, Entity>,
}

impl GridMap {
    pub fn new() -> GridMap {
        GridMap {
            cells: HashMap::new(),
        }
    }

    pub fn add_cell(&mut self, grid_pos: IVec2, entity: Entity) {
        self.cells.insert(grid_pos, entity);
    }

    // pub fn get_cell(&self, grid_pos: IVec2) -> Option<&Entity> {
    //     self.cells.get(&grid_pos)
    // }
}

pub fn update_snake(
    mut cells: Query<&mut Cell>,
    snake_state: Res<SnakeState>,
    grid_map: Res<GridMap>,
) {
    if !snake_state.is_changed() {
        return;
    }

    let body_list = &snake_state.body_list;

    for (_, &entity) in &grid_map.cells {
        let Ok(mut cell) = cells.get_mut(entity) else {
            continue;
        };

        let is_body = body_list.contains(&cell.grid_pos);

        match (is_body, cell.is_type(CellType::Snake)) {
            (true, false) => cell.as_type(CellType::Snake),
            (false, true) => cell.as_type(CellType::Empty),
            _ => continue,
        }
    }
}
