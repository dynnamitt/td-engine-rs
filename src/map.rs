use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellType {
    Vacuum,
    Platform,
}

#[derive(Debug)]
pub struct Map {
    cells: Vec<CellType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            cells: vec![CellType::Platform; NUM_TILES],
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_HEIGHT {
            (0..SCREEN_WIDTH).for_each(|x| match self.cells[map_idx(x, y)] {
                CellType::Platform => {
                    ctx.set(x, y, YELLOW, BLACK, to_cp437('.'));
                }
                CellType::Vacuum => {
                    ctx.set(x, y, GREEN, BLACK, to_cp437('#'));
                }
            })
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
