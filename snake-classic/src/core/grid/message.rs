use bevy::prelude::*;

use crate::grid::CellType;

/// 用于同步网格类型的消息
#[derive(Message, Default)]
pub struct SyncCellTypeMessage {
    pub changes: Vec<(IVec2, CellType)>,
}

impl SyncCellTypeMessage {
    // pub fn new(changes: Vec<(IVec2, CellType)>) -> Self {
    //     Self { changes }
    // }

    pub fn cell(mut self, cell: IVec2, cell_type: CellType) -> Self {
        self.changes.push((cell, cell_type));
        self
    }

    pub fn cells(mut self, cells: impl IntoIterator<Item = IVec2>, cell_type: CellType) -> Self {
        self.changes
            .extend(cells.into_iter().map(|cell| (cell, cell_type)));
        self
    }
}
