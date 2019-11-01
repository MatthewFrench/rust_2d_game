use ggez;
use ggez::event;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

struct Player {
    x: i32,
    y: i32,
    direction: Direction,
    shooter_id: u32,
    moving: bool,
    pressing_up: bool,
    pressing_down: bool,
    pressing_left: bool,
    pressing_right: bool,
}

struct Bullet {
    x: i32,
    y: i32,
    direction: Direction,
    shooter_id: u32,
}

struct MainState {
    bullets: Vec<Bullet>,
    player1: Player,
    player2: Player,
}

impl MainState {
    fn new() -> ggez::GameResult<MainState> {
        let mut player1 = Player {
            x: 200,
            y: 240,
            direction: Direction::Right,
            shooter_id: 0,
            moving: false,
            pressing_up: false,
            pressing_down: false,
            pressing_left: false,
            pressing_right: false,
        };
        let mut player2 = Player {
            x: 600,
            y: 240,
            direction: Direction::Left,
            shooter_id: 1,
            moving: false,
            pressing_up: false,
            pressing_down: false,
            pressing_left: false,
            pressing_right: false,
        };
        let bullets: Vec<Bullet> = Vec::new();
        let s = MainState {
            bullets,
            player1,
            player2,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        if _repeat {
            return;
        }
        match keycode {
            KeyCode::Up => self.player1.pressing_up = true,
            KeyCode::Down => self.player1.pressing_down = true,
            KeyCode::Left => self.player1.pressing_left = true,
            KeyCode::Right => self.player1.pressing_right = true,
            _ => (),
        };
        println!(
            "Pressed keyboard key '{:?}' with keymods '{:?}' repeated: '{:?}'",
            keycode, _keymod, _repeat
        );
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Up => self.player1.pressing_up = false,
            KeyCode::Down => self.player1.pressing_down = false,
            KeyCode::Left => self.player1.pressing_left = false,
            KeyCode::Right => self.player1.pressing_right = false,
            _ => (),
        };
        println!(
            "Released keyboard key '{:?}' with keymods '{:?}'",
            keycode, _keymod
        );
    }
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.player1.direction = match (
            self.player1.pressing_down,
            self.player1.pressing_up,
            self.player1.pressing_left,
            self.player1.pressing_right,
        ) {
            (false, true, false, false) => Direction::Up,
            (true, false, false, false) => Direction::Down,
            (false, false, true, false) => Direction::Left,
            (false, false, false, true) => Direction::Right,
            (false, true, true, false) => Direction::UpLeft,
            (false, true, false, true) => Direction::UpRight,
            (true, false, true, false) => Direction::DownLeft,
            (true, false, false, true) => Direction::DownRight,
            _ => Direction::Left,
        };
        self.player1.moving = self.player1.pressing_down
            || self.player1.pressing_left
            || self.player1.pressing_right
            || self.player1.pressing_up;
        if self.player1.moving {
            match self.player1.direction {
                Direction::Up => self.player1.y -= 1,
                Direction::Down => self.player1.y += 1,
                Direction::Left => self.player1.x -= 1,
                Direction::Right => self.player1.x += 1,
                Direction::UpLeft => {
                    self.player1.y -= 1;
                    self.player1.x -= 1
                }
                Direction::UpRight => {
                    self.player1.y -= 1;
                    self.player1.x += 1
                }
                Direction::DownLeft => {
                    self.player1.y += 1;
                    self.player1.x -= 1
                }
                Direction::DownRight => {
                    self.player1.y += 1;
                    self.player1.x += 1
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let player1_rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(-10.0, -10.0, 10.0, 10.0),
            graphics::Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        )?;
        graphics::draw(
            ctx,
            &player1_rectangle,
            (na::Point2::new(
                self.player1.x as f32,
                self.player1.y as f32,
            ),),
        )?;

        let player2_rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(-10.0, -10.0, 10.0, 10.0),
            graphics::Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
        )?;
        graphics::draw(
            ctx,
            &player2_rectangle,
            (na::Point2::new(
                self.player2.x as f32,
                self.player2.y as f32,
            ),),
        )?;
        /*
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    na::Point2::new(self.pos_x, 380.0),
                    100.0,
                    2.0,
                    graphics::WHITE,
                )?;
                graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        */
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("Bullet Game!", "Matthew French");
    let (ctx, event_loop) = &mut cb
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
