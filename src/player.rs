use crate::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub map_pos: Point,
}
impl Player {
    pub fn new(cell_pos: Point) -> Self {
        Self { map_pos: cell_pos }
    }
    pub fn render(&self, ctx: &mut BTerm, cam: &Camera) {
        ctx.set_active_console(1);
        let csp = cell_scr_point(self.map_pos.x - cam.left_x, self.map_pos.y - cam.top_y);
        ctx.set(csp.x, csp.y, WHITE, BLACK, to_cp437('@'));
    }
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, cam: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };
            let new_pos = self.map_pos + delta;
            if map.can_enter(new_pos) {
                self.map_pos = new_pos;
                cam.on_player_move(new_pos);
            }
        }
    }
}
