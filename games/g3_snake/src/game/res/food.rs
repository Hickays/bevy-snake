use bevy::prelude::*;

#[derive(Resource)]
pub struct SnakeFood {
    food_list: Vec<IVec2>,
}

impl SnakeFood {
    pub fn new() -> SnakeFood {
        SnakeFood {
            food_list: Vec::new(),
        }
    }
}
