use bevy::prelude::*;

/// 构建自定义 window 窗口
pub fn build_custom_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "snake".into(),
            resolution: (800, 600).into(),
            resizable: true,
            ..Default::default()
        }),
        ..Default::default()
    }
}
