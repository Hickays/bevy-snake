use bevy::prelude::*;

#[derive(Component)]
pub struct Snake {
    body_list: Vec<Vec2>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            body_list: vec![Vec2::ZERO],
        }
    }
}
