use ggez;
use ggez::event;

mod bullet;
mod common;
mod game_state;
mod player;
use crate::common::{GAME_HEIGHT, GAME_WIDTH};
use game_state::GameState;
use ggez::conf::WindowSetup;
use std::env;
use std::path;

pub fn main() -> ggez::GameResult {
    // Allow loading resources from source directory
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let mut cb =
        ggez::ContextBuilder::new("Bullet Game!", "Matthew French").window_setup(WindowSetup {
            title: String::from("Fireball Game"),
            ..WindowSetup::default()
        });
    let (ctx, event_loop) = &mut cb
        .add_resource_path(resource_dir)
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(GAME_WIDTH as f32, GAME_HEIGHT as f32),
        )
        .build()?;
    let state = &mut GameState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
