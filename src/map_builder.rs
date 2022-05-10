use crate::prelude::*;
const NUM_ISLAND: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub islands: Vec<Rect>,
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            islands: Vec::new(),
            player_start: Point::zero(),
        };
        mb.fill(CellType::Vacuum);
        mb.build_random_isles(rng);
        mb.connect_bridges(rng);
        println!("p AT {:?}", mb.player_start);
        mb.player_start = mb.islands[0].center();
        println!("p AT {:?}", mb.player_start);
        mb
    }
    fn fill(&mut self, tile: CellType) {
        self.map.cells.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_isles(&mut self, rng: &mut RandomNumberGenerator) {
        while self.islands.len() < NUM_ISLAND {
            let new_isle = Rect::with_size(
                rng.range(1, CELLS_WIDTH - 10),
                rng.range(1, CELLS_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let overlaps = self
                .islands
                .iter()
                .filter(|i| i.intersect(&new_isle))
                .count();
            if overlaps == 0 {
                new_isle.for_each(|p| {
                    if p.x > 0 && p.x < CELLS_WIDTH && p.y > 0 && p.y < CELLS_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.cells[idx] = CellType::Platform;
                    }
                });
                self.islands.push(new_isle)
            }
        }
    }
    fn connect_bridges(&mut self, rng: &mut RandomNumberGenerator) {
        let mut xs = self.islands.clone();
        xs.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in xs.iter().enumerate().skip(1) {
            let prev = xs[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                self.horiz_bridge(prev.x, new.x, prev.y);
                self.vert_bridge(prev.y, new.y, new.x);
            } else {
                self.vert_bridge(prev.y, new.y, prev.x);
                self.horiz_bridge(prev.x, new.x, new.y);
            }
        }
    }
    fn vert_bridge(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.cells[idx as usize] = CellType::Platform;
            }
        }
    }
    fn horiz_bridge(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.cells[idx as usize] = CellType::Platform;
            }
        }
    }
}
