use ggez;
use ggez::event;

mod bullet;
mod common;
mod game_state;
mod player;
use crate::common::{GAME_HEIGHT, GAME_WIDTH};
use game_state::GameState;

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("Bullet Game!", "Matthew French");
    let (ctx, event_loop) = &mut cb
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(GAME_WIDTH as f32, GAME_HEIGHT as f32),
        )
        .build()?;
    let state = &mut GameState::new()?;
    event::run(ctx, event_loop, state)
}
