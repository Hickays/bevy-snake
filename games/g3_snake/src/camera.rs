use bevy::prelude::*;

/// 相机插件: 只负责 2D 相机的初始化
pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, startup_spawn);
}

fn startup_spawn(mut commands: Commands) {
    commands.spawn(Camera2d);
}
