use bevy::prelude::*;

mod change_color;
mod movement;
mod spawn;

/// 方块插件: 只负责把方块相关的系统注册到 App 上
pub struct SquarePlugin;

impl Plugin for SquarePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                // 创建矩形
                spawn::spawn_square,
            ),
        );

        // 移动矩形
        app.add_systems(Update, movement::move_square);
        // 点击改变颜色
        app.add_observer(change_color::init_square_color);
        app.add_observer(change_color::click_square);
    }
}
