mod map;
mod player; // or monster-path
mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const CELLS_HEIGHT: i32 = SCREEN_HEIGHT - 2;
    pub const CELLS_WIDTH: i32 = SCREEN_WIDTH - 2;
    pub use crate::map::*;
    pub use crate::player::*;
}
use prelude::*;
#[derive(Debug)]
struct State {
    map: Map,
    player: Player,
}
impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(CELLS_WIDTH / 2, CELLS_HEIGHT / 2)),
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("TD-Engine-ASCII-Sim")
        .with_fps_cap(30.0)
        .build()?;
    main_loop(context, State::new())
}
