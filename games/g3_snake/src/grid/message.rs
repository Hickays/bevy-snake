use bevy::prelude::*;

use crate::grid::CellType;

/// 用于同步网格类型的消息
#[derive(Message)]
pub struct SyncCellTypeMessage(Vec<(IVec2, CellType)>);

impl SyncCellTypeMessage {
    pub fn new(data: Vec<(IVec2, CellType)>) -> SyncCellTypeMessage {
        SyncCellTypeMessage(data)
    }

    pub fn from_vec<'a, I>(data: I, cell_type: CellType) -> SyncCellTypeMessage
    where
        I: IntoIterator<Item = &'a IVec2>,
    {
        SyncCellTypeMessage(data.into_iter().map(|&pos| (pos, cell_type)).collect())
    }

    pub fn get_data(&self) -> &Vec<(IVec2, CellType)> {
        &self.0
    }
}
