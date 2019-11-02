use crate::bullet::Bullet;
use crate::common::Direction;
use ggez::graphics;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub shooter_id: u32,
    pub moving: bool,
    pub pressing_up: bool,
    pub pressing_down: bool,
    pub pressing_left: bool,
    pub pressing_right: bool,
    pub pressing_shoot: bool,
    pub speed: i32,
    pub size: i32,
    pub color: graphics::Color,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            x: 0,
            y: 0,
            direction: Direction::Left,
            shooter_id: 0,
            moving: false,
            pressing_up: false,
            pressing_down: false,
            pressing_left: false,
            pressing_right: false,
            pressing_shoot: false,
            speed: 3,
            size: 20,
            color: graphics::Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
        }
    }
}

impl Player {
    pub fn shoot(&mut self, bullets: &mut Vec<Bullet>) {
        if self.pressing_shoot {
            let bullet = Bullet {
                x: self.x,
                y: self.y,
                direction: self.direction,
                shooter_id: self.shooter_id,
                color: self.color,
                ..Default::default()
            };
            bullets.push(bullet);
        }
    }
    pub fn move_player(&mut self) {
        self.direction = match (
            self.pressing_down,
            self.pressing_up,
            self.pressing_left,
            self.pressing_right,
        ) {
            (false, true, false, false) => Direction::Up,
            (true, false, false, false) => Direction::Down,
            (false, false, true, false) => Direction::Left,
            (false, false, false, true) => Direction::Right,
            (false, true, true, false) => Direction::UpLeft,
            (false, true, false, true) => Direction::UpRight,
            (true, false, true, false) => Direction::DownLeft,
            (true, false, false, true) => Direction::DownRight,
            _ => self.direction,
        };
        self.moving =
            self.pressing_down || self.pressing_left || self.pressing_right || self.pressing_up;
        if self.moving {
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
        }
        if self.x < 0 {
            self.x = 0;
        }
    }
    pub fn draw_player(&mut self, mesh: &mut graphics::MeshBuilder) {
        mesh.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                (self.x - self.size / 2) as f32,
                (self.y - self.size / 2) as f32,
                self.size as f32,
                self.size as f32,
            ),
            self.color,
        );
    }
}
