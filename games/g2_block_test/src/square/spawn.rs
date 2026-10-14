use bevy::prelude::*;

pub fn spawn_square(mut command: Commands) {
    command.spawn((
        Sprite {
            color: Color::srgb_u32(0x_ff0000),
            custom_size: Some(vec2(100., 100.)),
            ..Default::default()
        },
        Transform {
            translation: vec2(0., 0.).extend(0.),
            ..Default::default()
        },
    ));
}
