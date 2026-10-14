use bevy::prelude::*;
mod movement;
mod spawn;
pub struct TestBlockPlugin;

impl Plugin for TestBlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_system,
                // 创建矩形
                spawn::spawn_square,
            ),
        );

        app.add_systems(Update, (movement::move_square));
    }
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}
