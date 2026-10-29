//! 贪吃蛇
use bevy::prelude::*;
use std::collections::VecDeque;

mod timer;

use crate::{
    core::{
        config::GameConfig,
        grid::{Cell, CellType, GridMap},
        state::GameState,
        stats::GameStatus,
    },
    food::SnakeFoodState,
    input::DirectionInput,
    // status::{GameState, GameStatus},
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(timer::SnakeMoveTimer::new());
    app.init_resource::<SnakeState>();
    app.add_systems(
        Update,
        (handle_direction_input, handle_snake_move)
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}

/// 接收方向消息
fn handle_direction_input(
    mut snake_state: ResMut<SnakeState>,
    mut direction_input: MessageReader<DirectionInput>,
) {
    for direction in direction_input.read() {
        snake_state.try_buffer_direction(direction.as_ivec2());
    }
}

/// 移动结果
pub enum MoveOutcome {
    Advanced { enter: IVec2, leave: Option<IVec2> },
    AteFood { food: IVec2 },
    Collided,
    None,
}

pub fn reset_snake_for_ready(
    mut snake_state: ResMut<SnakeState>,
    mut timer: ResMut<timer::SnakeMoveTimer>,
    game_config: Res<GameConfig>,
    grid_map: Res<GridMap>,
    mut cells: Query<&mut Cell>,
) {
    timer.reset();
    snake_state.reset((game_config.grid_extent / UVec2::splat(2)).as_ivec2());

    for segment in &snake_state.segment_list {
        let Some(entity) = grid_map.get_entity(segment) else {
            continue;
        };
        let Ok(mut cell) = cells.get_mut(entity.entity()) else {
            continue;
        };
        cell.cell_type = CellType::Snake;
    }
}

/// 处理蛇移动
fn handle_snake_move(
    mut snake_state: ResMut<SnakeState>,
    time: Res<Time<Real>>,
    mut timer: ResMut<timer::SnakeMoveTimer>,
    game_config: Res<GameConfig>,
    grid_map: Res<GridMap>,
    mut cells: Query<&mut Cell>,
    mut food_state: ResMut<SnakeFoodState>,
    mut game_status: ResMut<GameStatus>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    timer.tick(time.delta());

    if !timer.just_finished() {
        return;
    }

    let mut changed_cells = Vec::<(IVec2, CellType)>::new();

    match snake_state.step_forward(&game_config, &food_state.foods) {
        // 前进
        MoveOutcome::Advanced { enter, leave } => {
            if let Some(leave) = leave {
                changed_cells.push((leave, CellType::Empty));
            }

            changed_cells.push((enter, CellType::Snake));
        }
        // 吃到食物
        MoveOutcome::AteFood { food } => {
            food_state.try_eat_food(&food);
            game_status.add_score();
            changed_cells.push((food, CellType::Snake));
        }
        // 撞墙
        MoveOutcome::Collided => {
            game_state.set(GameState::GameOver);
        }
        MoveOutcome::None => {}
    };

    if !changed_cells.is_empty() {
        for (pos, cell_type) in changed_cells {
            let Some(entity) = grid_map.get_entity(&pos) else {
                continue;
            };
            let Ok(mut cell) = cells.get_mut(entity.entity()) else {
                continue;
            };
            cell.cell_type = cell_type;
        }
    }
}

/// 蛇状态
#[derive(Resource, Default)]
pub struct SnakeState {
    /// 身体列表
    segment_list: VecDeque<IVec2>,
    /// 当前方向
    pub current_direction: IVec2,
    /// 输入队列, 接收输入时从后往前比较, 如果相反则覆盖
    direction_queue: VecDeque<IVec2>,
}

impl SnakeState {
    fn reset(&mut self, birth_pos: IVec2) {
        let mut segment_list = VecDeque::new();
        segment_list.push_front(birth_pos);
        segment_list.push_front(birth_pos);
        segment_list.push_front(birth_pos);

        self.segment_list = segment_list;
        self.current_direction = IVec2::ZERO;
        self.direction_queue = VecDeque::new();
        info!("生成体");
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
        if *before_direction == direction {
            return;
        }

        // 如果输入方向和 当前方向 / 最后输入方向 相反
        if before_direction + direction == IVec2::ZERO {
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
    pub fn step_forward(&mut self, game_config: &GameConfig, foods: &Vec<IVec2>) -> MoveOutcome {
        // 获取并更新方向
        let new_direction = self
            .direction_queue
            .pop_front()
            .unwrap_or(self.current_direction);
        self.current_direction = new_direction;

        if new_direction == IVec2::ZERO {
            return MoveOutcome::None;
        };

        // 获取头的位置, 随后基于头位置 + 方向计算头的下一步位置
        let head_pos = self.segment_list.front().unwrap().clone();

        let new_head_pos = head_pos + new_direction;

        // 根据边界范围修改头位置
        let boundary = game_config.grid_extent;
        let new_head_pos = (new_head_pos + boundary.as_ivec2()).rem_euclid(boundary.as_ivec2());

        // 新头在体节内, 表示撞到自己了
        if self.segment_list.contains(&new_head_pos) {
            return MoveOutcome::Collided;
        }

        // 本次移动吃到了食物
        let ate_food = foods.contains(&new_head_pos);

        // 将新头加入体节列表
        self.segment_list.push_front(new_head_pos);

        // 吃到了食物, 标记食物位置
        if ate_food {
            return MoveOutcome::AteFood { food: new_head_pos };
        }

        let mut leave = None;

        // 没吃到食物, 去掉尾巴
        if let Some(tail) = self.segment_list.pop_back() {
            // 如果体节里没有和尾巴相同的坐标, 则标记离开尾巴位置
            if !self.segment_list.contains(&tail) {
                leave = Some(tail); // 标记离开
            }
        };

        // 没吃到食物, 标记前进位置和离开位置
        MoveOutcome::Advanced {
            enter: new_head_pos,
            leave,
        }
    }
}
