//! 网格单元
use std::collections::HashMap;

use bevy::prelude::*;

/// 网格组件, 记录真实坐标, 转换坐标, 网格类型
#[derive(Component, Clone, Copy)]
pub struct Cell {
    pub cell_type: CellType,

    pub grid_pos: IVec2,
    pub world_pos: Vec2,
}

/// 网格类型: 空格, 蛇体节格, 食物格
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CellType {
    Empty,
    Snake,
    Food,
}

impl CellType {
    pub fn get_color(&self) -> Color {
        match self {
            CellType::Empty => Color::srgb_u32(0x_444444),
            CellType::Food => Color::srgb_u32(0x_ff4242),
            CellType::Snake => Color::srgb_u32(0x_aaaaaa),
        }
    }
}

/// 网格地图资源, 保存坐标和网格实体的对应关系
#[derive(Resource, Default)]
pub struct GridMap {
    entitys: HashMap<IVec2, Entity>,
    inited: bool,
}

impl GridMap {
    pub fn get_entity(&self, grid_pos: &IVec2) -> Option<&Entity> {
        self.entitys.get(grid_pos)
    }
    pub fn add_entity(&mut self, pos: IVec2, entity: Entity) {
        self.entitys.insert(pos, entity);
    }
    pub fn set_inited(&mut self, inited: bool) {
        self.inited = inited;
    }
    pub fn is_inited(&self) -> bool {
        self.inited
    }
}
