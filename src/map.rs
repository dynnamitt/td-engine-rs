use crate::prelude::*;
const NUM_CELLS: usize = (CELLS_HEIGHT * CELLS_WIDTH) as usize;

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
            cells: vec![CellType::Platform; NUM_CELLS],
        }
    }
    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..CELLS_HEIGHT {
            (0..CELLS_WIDTH).for_each(|x| {
                // use ODD cells as Space(not vacuum)
                let csp = cell_scr_point(x, y);
                match self.cells[map_idx(x, y)] {
                    CellType::Platform => {
                        ctx.set(csp.x, csp.y, YELLOW, BLACK, to_cp437('.'));
                    }
                    CellType::Vacuum => {
                        ctx.set(csp.x, csp.y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }) // end .for_each
        } // end for y in
    } // end render

    pub fn in_bounds(&self, p: Point) -> bool {
        p.x >= 0 && p.x < CELLS_WIDTH && p.y >= 0 && p.y < CELLS_HEIGHT
    }

    pub fn can_enter(&self, p: Point) -> bool {
        self.in_bounds(p) && self.cells[map_idx(p.x, p.y)] == CellType::Platform
    }

    pub fn try_idx(&self, p: Point) -> Option<usize> {
        if !self.in_bounds(p) {
            None
        } else {
            Some(map_idx(p.x, p.y))
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * CELLS_WIDTH) + x) as usize
}

// simulate a hex-grid
pub fn cell_scr_point(x: i32, y: i32) -> Point {
    let odd_row_indent = (y % 2 != 0) as i32;
    let odd_cell_intersp = (x % 2 != 0) as i32;
    Point::new(x + odd_row_indent + odd_cell_intersp, y)
}
