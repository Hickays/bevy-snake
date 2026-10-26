//! 贪吃蛇
use bevy::prelude::*;
use std::{
    collections::{VecDeque},
    ops::Div,
};

mod timer;

use crate::{
    config::GameConfig,
    food::SnakeFoodState,
    grid::{CellType, message::SyncCellTypeMessage},
    input::DirectionInput,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, startup_spawn_snake);
    app.insert_resource(timer::SnakeMoveTimer::new());
    app.add_systems(Update, (handle_direction_input, handle_snake_move).chain());
}

/// 初始化蛇
fn startup_spawn_snake(
    mut commands: Commands,
    game_config: Res<GameConfig>,
    mut sync_cell_message: MessageWriter<SyncCellTypeMessage>,
) {
    let mut segment_list = VecDeque::new();
    segment_list.push_front(game_config.grid_extent.div(2).as_ivec2());
    segment_list.push_front(game_config.grid_extent.div(2).as_ivec2());
    segment_list.push_front(game_config.grid_extent.div(2).as_ivec2());

    sync_cell_message.write(SyncCellTypeMessage::from_vec(
        &segment_list,
        CellType::Snake,
    ));

    commands.insert_resource(SnakeState {
        segment_list: segment_list.clone(),
        current_direction: IVec2::X,
        direction_queue: VecDeque::new(),
    });
}

/// 接收方向消息
fn handle_direction_input(
    mut snake_state: ResMut<SnakeState>,
    mut direction_input: MessageReader<DirectionInput>,
) {
    for direction in direction_input.read() {
        // info!("获取输入 {}", direction.as_ivec2());
        snake_state.try_buffer_direction(direction.as_ivec2());
    }
}

/// 移动结果
pub enum MoveOutcome {
    Advanced { enter: IVec2, leave: Option<IVec2> },
    AteFood { food: IVec2 },
    Collided,
}

/// 处理蛇移动
fn handle_snake_move(
    mut snake_state: ResMut<SnakeState>,
    time: Res<Time<Real>>,
    mut timer: ResMut<timer::SnakeMoveTimer>,
    game_config: Res<GameConfig>,
    mut sync_cell_message: MessageWriter<SyncCellTypeMessage>,
    food_state: ResMut<SnakeFoodState>,
) {
    timer.tick(time.delta());

    if !timer.just_finished() {
        return;
    }

    match snake_state.step_forward(&game_config, food_state) {
        MoveOutcome::Advanced { enter, leave } => {
            let mut message_data = vec![(enter, CellType::Snake)];
            if let Some(leave) = leave {
                message_data.push((leave, CellType::Empty));
            }

            sync_cell_message.write(SyncCellTypeMessage::new(message_data));
        }
        MoveOutcome::AteFood { food } => {
            sync_cell_message.write(SyncCellTypeMessage::new(vec![(food, CellType::Snake)]));
        }
        MoveOutcome::Collided => info!("撞到自己了!"),
    };
}

/// 蛇状态
#[derive(Resource)]
pub struct SnakeState {
    /// 身体列表
    segment_list: VecDeque<IVec2>,
    /// 当前方向
    pub current_direction: IVec2,
    /// 输入队列, 接收输入时从后往前比较, 如果相反则覆盖
    direction_queue: VecDeque<IVec2>,
}

impl SnakeState {
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
    pub fn step_forward(
        &mut self,
        game_config: &GameConfig,
        mut food_state: ResMut<SnakeFoodState>,
    ) -> MoveOutcome {
        let _ = game_config;
        // 获取头的位置, 随后基于头位置 + 方向计算头的下一步位置
        let head_pos = self.segment_list.front().unwrap().clone();

        // 获取并更新方向
        let new_direction = self
            .direction_queue
            .pop_front()
            .unwrap_or(self.current_direction);
        self.current_direction = new_direction;

        let head_new_pos = head_pos + new_direction;

        // 根据边界范围修改头位置
        let boundary = game_config.grid_extent;
        let new_head_pos = (head_new_pos + boundary.as_ivec2()).rem_euclid(boundary.as_ivec2());

        // 新头在体节内, 表示撞到自己了
        if self.segment_list.contains(&new_head_pos) {
            return MoveOutcome::Collided;
        }

        // 将新头加入体节列表
        self.segment_list.push_front(head_new_pos);

        let mut leave = None;

        // 没吃到食物
        if !food_state.try_eat_food(&head_new_pos) {
            // 去掉尾巴
            if let Some(tail) = self.segment_list.pop_back() {
                // 如果体节里没有和尾巴相同的坐标, 则标记离开尾巴位置
                if !self.segment_list.contains(&tail) {
                    leave = Some(tail); // 标记离开
                }
            };
        } else {
            // 吃到了食物, 标记前进位置
            return MoveOutcome::AteFood { food: new_head_pos };
        }

        // 没吃到食物, 标记前进位置和离开位置
        MoveOutcome::Advanced {
            enter: new_head_pos,
            leave,
        }
    }
}
