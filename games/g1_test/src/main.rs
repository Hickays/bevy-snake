use bevy::prelude::*;
use g1_test::{SquareColor, build_square};
mod systems;

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
    app.add_systems(Update, systems::move_square);
    app.add_systems(Update, systems::multi_square_click);
    // app.add_systems(Update, systems::picking_square_click);
    app.add_observer(systems::picking_square_click);
    app.run();
}

fn startup_system(mut commands: Commands) {
    //! 初始化
    info!("Hello, world!");
    commands.spawn(Camera2d); // 相机

    let square_color = SquareColor::new();

    // 创建一个方块
    commands
        .spawn((
            build_square(100, square_color.current()),
            Transform::from_translation(Vec3::ZERO),
            Pickable::default(),
            square_color,
        ))
        .observe(|_click: On<PointerClick>| {
            info!("点击了方块! (拾取)");
        });
}
