use std::collections::VecDeque;

use bevy::{platform::collections::HashMap, prelude::*};

mod grid;
mod snake;

pub struct GridsPlugin;

impl Plugin for GridsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system);
        app.add_systems(Update, apply_snake_grid);
    }
}

const GRID_SIZE: u16 = 20;
const GRID_GAP: u16 = 1;

fn startup_system(mut commands: Commands) {
    let mut positions = vec![];
    for x in -10..=10 {
        for y in -10..=10 {
            positions.push(IVec2::new(x, y) * (GRID_SIZE + GRID_GAP) as i32);
        }
    }

    let mut snake_grids = HashMap::new();

    for position in positions {
        let entity = commands
            .spawn((
                Sprite {
                    custom_size: Some(Vec2::splat(GRID_SIZE as f32)),
                    ..Default::default()
                },
                Transform::from_translation(position.extend(0).as_vec3()),
                grid::SnakeGrid::new(),
                snake::Snake::new(),
            ))
            .id();
        snake_grids.insert(position, entity);
    }

    commands.insert_resource(SnakeGridIndex { snake_grids });
    let mut body_list = VecDeque::new();
    body_list.push_front(IVec2::ZERO);
    commands.insert_resource(SnakeData { body_list });
}

fn apply_snake_grid(query: Query<(&mut Sprite, &grid::SnakeGrid), Changed<grid::SnakeGrid>>) {
    for (mut sprite, snake_grid) in query {
        info!("改变颜色");
        sprite.color = snake_grid.get_color()
    }
}

fn apply_snake_body(query: Query<&mut grid::SnakeGrid, Changed<SnakeData>>) {
  for mut snake_grid in query {
    
  }
}

#[derive(Resource)]
struct SnakeGridIndex {
    // 记录坐标和坐标下的网格实体, 坐标采用原点在左上的计算方式
    snake_grids: HashMap<IVec2, Entity>,
}

#[derive(Resource)]
struct SnakeData {
    body_list: VecDeque<IVec2>,
}
