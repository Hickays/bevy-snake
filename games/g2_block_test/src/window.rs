use bevy::prelude::*;

pub fn build_custom_window_plugin() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "block test".into(),
            resolution: (800, 600).into(),
            resizable: true,
            ..Default::default()
        }),
        ..Default::default()
    }
}
