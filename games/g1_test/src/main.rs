use bevy::{
    DefaultPlugins,
    app::{App, PluginGroup, Startup},
    ecs::system::Commands,
    log::info,
    window::{Window, WindowPlugin},
};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: String::from("My Bevy App"),
            resolution: (800, 600).into(),
            resizable: true,
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_systems(Startup, startup_system);
    app.run();
}

fn startup_system(mut _commands: Commands) {
    info!("Hello, world!");
}
