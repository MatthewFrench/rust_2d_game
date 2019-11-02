pub const GAME_WIDTH: i32 = 800;
pub const GAME_HEIGHT: i32 = 600;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
