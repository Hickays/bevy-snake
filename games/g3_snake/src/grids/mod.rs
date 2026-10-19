use std::collections::VecDeque;

use bevy::{input::keyboard::KeyboardInput, platform::collections::HashMap, prelude::*};

mod grid;
mod snake;

pub struct GridsPlugin;

impl Plugin for GridsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup_system);
        app.add_systems(Update, apply_snake_grid);
        app.add_systems(Update, apply_snake_body);
        app.add_systems(Update, handle_move);
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
        // info!("")
        sprite.color = snake_grid.get_color()
    }
}

fn apply_snake_body(
    mut query: Query<&mut grid::SnakeGrid>,
    grid_index: ResMut<SnakeGridIndex>,
    snake_data: Res<SnakeData>,
) {
    if snake_data.is_changed() {
        info!("bl: {:?}", &snake_data.body_list);
        for body in &snake_data.body_list {
            let Some(entity) = grid_index.snake_grids.get(body) else {
                info!("gi: {:?}", grid_index.snake_grids);
                continue;
            };
            
            let Ok(mut e) = query.get_mut(*entity) else {
                continue;
            };
            info!("设置: {:?} 为 {:?}", body, e.get_color());
            
            e.as_snake_body();

        }
    }

    // for mut snake_grid in query {}
}

fn handle_move(keys: Res<ButtonInput<KeyCode>>, mut snake_data: ResMut<SnakeData>) {
    if keys.just_pressed(KeyCode::Space) {
        let b = &mut snake_data.body_list;
        b.push_front(IVec2 { x: 210, y: -126 });
        info!("更新: {:?}", b);
    };
}

#[derive(Resource)]
struct SnakeGridIndex {
    // 记录坐标和坐标下的网格实体, 坐标采用原点在左上的计算方式
    snake_grids: HashMap<IVec2, Entity>,
}

#[derive(Resource)]
struct SnakeData {
    pub body_list: VecDeque<IVec2>,
}
