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
    pub fn update(&mut self, _ctx: &mut BTerm, map: &Map, cam: &mut Camera) {
        let input = INPUT.lock();
        let delta = match true {
            _ if input.is_key_pressed(VirtualKeyCode::Left) => Point::new(-1, 0),
            _ if input.is_key_pressed(VirtualKeyCode::Right) => Point::new(1, 0),
            _ if input.is_key_pressed(VirtualKeyCode::Up) => Point::new(0, -1),
            _ if input.is_key_pressed(VirtualKeyCode::Down) => Point::new(0, 1),
            _ => Point::zero(),
        };
        if delta.x != 0 || delta.y != 0 {
            let new_pos = self.map_pos + delta;
            if map.can_enter(new_pos) {
                self.map_pos = new_pos;
                cam.on_player_move(new_pos);
            }
        }
    }
}
