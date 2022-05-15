use crate::prelude::*;
const NUM_CELLS: usize = (CELLS_HEIGHT * CELLS_WIDTH) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CellType {
    Vacuum,
    Platform,
}

#[derive(Debug)]
pub struct Map {
    pub cells: Vec<CellType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            cells: vec![CellType::Platform; NUM_CELLS],
        }
    }
    pub fn render(&self, ctx: &mut BTerm, cam: &Camera) {
        ctx.set_active_console(0);
        for y in cam.top_y..cam.bottom_y {
            (cam.left_x..cam.right_x).for_each(|x| {
                if self.in_bounds(Point::new(x, y)) {
                    let csp = cell_scr_point(x, y);
                    match self.cells[map_idx(x, y)] {
                        CellType::Platform => {
                            ctx.set(
                                csp.x - cam.left_x,
                                csp.y - cam.top_y,
                                DIMGRAY,
                                GREEN,
                                to_cp437('.'),
                            );
                        }
                        CellType::Vacuum => {
                            ctx.set(
                                csp.x - cam.left_x,
                                csp.y - cam.top_y,
                                WEBGRAY,
                                BLACK,
                                to_cp437('#'),
                            );
                        }
                    }
                } // end if
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

// simulate a hex-grid , if
pub fn cell_scr_point(x: i32, y: i32) -> Point {
    let odd_row_indent = (y % 2 != 0) as i32;
    let odd_cell_intersp = (x % 2 != 0) as i32;
    // Point::new(x + odd_row_indent + odd_cell_intersp, y)
    Point::new(x, y)
}
