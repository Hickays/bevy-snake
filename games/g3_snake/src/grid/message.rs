use std::collections::HashMap;

use bevy::prelude::*;

use crate::grid::CellType;

#[derive(Message)]
pub struct SyncCellTypeMessage(HashMap<IVec2, CellType>);

impl SyncCellTypeMessage {
    pub fn new(data: HashMap<IVec2, CellType>) -> SyncCellTypeMessage {
        SyncCellTypeMessage(data)
    }

    pub fn from_vec<'a, I>(data: I, cell_type: CellType) -> SyncCellTypeMessage
    where
        I: IntoIterator<Item = &'a IVec2>,
    {
        SyncCellTypeMessage(data.into_iter().map(|&pos| (pos, cell_type)).collect())
    }

    pub fn get_data(&self) -> &HashMap<IVec2, CellType> {
        &self.0
    }
}
