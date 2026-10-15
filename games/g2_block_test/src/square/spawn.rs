use super::change_color::SquareColor;
use bevy::prelude::*;

/// 矩形边长
const SQUARE_SIZE: f32 = 100.;

pub fn spawn_square(mut commands: Commands) {
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::splat(SQUARE_SIZE)),
            ..Default::default()
        },
        SquareColor::new(),
        Pickable::default(),
        Transform::from_translation(Vec3::ZERO),
    ));
}
