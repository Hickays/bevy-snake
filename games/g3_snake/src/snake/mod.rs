use std::collections::VecDeque;

use bevy::prelude::*;

use crate::input::DirectionInput;

pub(super) fn plugin(app: &mut App) {}

fn handle_direction_input(
    mut snake_state: ResMut<SnakeState>,
    mut direction_input: MessageReader<DirectionInput>,
) {
    for direction in direction_input.read() {
        snake_state.try_buffer_direction(direction.as_ivec2());
    }
}

#[derive(Resource)]
pub struct SnakeState {
    /// 身体列表
    segment_list: IVec2,

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
}
