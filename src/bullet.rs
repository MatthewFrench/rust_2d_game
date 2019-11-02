use crate::common::{Direction, GAME_HEIGHT, GAME_WIDTH};
use ggez::graphics;
use ggez::nalgebra as na;

pub struct Bullet {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub shooter_id: u32,
    pub speed: i32,
    pub size: i32,
    pub color: graphics::Color,
}

impl Default for Bullet {
    fn default() -> Bullet {
        Bullet {
            x: 0,
            y: 0,
            direction: Direction::Left,
            shooter_id: 0,
            speed: 10,
            size: 10,
            color: graphics::Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }
}

impl Bullet {
    pub fn move_bullet(&mut self) {
        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Left => self.x -= self.speed,
            Direction::Right => self.x += self.speed,
            Direction::UpLeft => {
                self.y -= self.speed;
                self.x -= self.speed;
            }
            Direction::UpRight => {
                self.y -= self.speed;
                self.x += self.speed;
            }
            Direction::DownLeft => {
                self.y += self.speed;
                self.x -= self.speed;
            }
            Direction::DownRight => {
                self.y += self.speed;
                self.x += self.speed;
            }
        }
        if self.x < self.size / 2 {
            self.x = self.size / 2;
            self.direction = match self.direction {
                Direction::Left => Direction::Right,
                Direction::UpLeft => Direction::UpRight,
                Direction::DownLeft => Direction::DownRight,
                _ => self.direction,
            }
        }
        if self.x > GAME_WIDTH + self.size / 2 {
            self.x = GAME_WIDTH + self.size / 2;
            self.direction = match self.direction {
                Direction::Right => Direction::Left,
                Direction::UpRight => Direction::UpLeft,
                Direction::DownRight => Direction::DownLeft,
                _ => self.direction,
            }
        }
        if self.y < self.size / 2 {
            self.y = self.size / 2;
            self.direction = match self.direction {
                Direction::Up => Direction::Down,
                Direction::UpLeft => Direction::DownLeft,
                Direction::UpRight => Direction::DownRight,
                _ => self.direction,
            }
        }
        if self.y > GAME_HEIGHT + self.size / 2 {
            self.y = GAME_HEIGHT + self.size / 2;
            self.direction = match self.direction {
                Direction::Down => Direction::Up,
                Direction::DownRight => Direction::UpRight,
                Direction::DownLeft => Direction::UpLeft,
                _ => self.direction,
            }
        }
    }
    pub fn draw_bullet(&mut self, mesh: &mut graphics::MeshBuilder) {
        mesh.circle(
            graphics::DrawMode::fill(),
            na::Point2::new(self.x as f32, self.y as f32),
            self.size as f32 / 2.0,
            0.1,
            self.color,
        );
    }
}
