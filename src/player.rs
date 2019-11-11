use crate::bullet::Bullet;
use crate::common::{GAME_HEIGHT, GAME_WIDTH};
use ggez::input::mouse::position;
//use ggez::nalgebra as na;
extern crate nalgebra as na;
use self::na::Isometry2;
use ggez::{graphics, Context};
use na::{Point2, Rotation2, UnitComplex, Vector2};
use ncollide2d::shape::{Ball, ShapeHandle};
use nphysics2d::algebra::Velocity2;
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, RigidBodyDesc,
};
use std::f32::consts::PI;

pub struct Player {
    pub position: Point2<f32>,
    pub shooter_id: u32,
    pub moving: bool,
    pub pressing_up: bool,
    pub pressing_down: bool,
    pub pressing_left: bool,
    pub pressing_right: bool,
    pub pressing_shoot: bool,
    pub direction: f32,
    pub speed: f32,
    pub size: f32,
    pub color: graphics::Color,
    pub image: graphics::Image,
    pub health: f32,
}

impl Player {
    pub fn shoot(
        &mut self,
        bullets: &mut Vec<Bullet>,
        bodies: &mut DefaultBodySet<f32>,
        colliders: &mut DefaultColliderSet<f32>,
    ) {
        if self.pressing_shoot {
            let diameter = 10.0;
            let speed = 10.0;

            // Create rigid body for bullet
            let mut rb_desc = RigidBodyDesc::new()
                .position(Isometry2::new(
                    Vector2::new(self.position.x, self.position.y),
                    0.0,
                ))
                .velocity(Velocity2::new(Vector2::new(speed, 0.0), self.direction))
                .mass(diameter / 2.0);
            let rigid_body = rb_desc.build();
            // Add it to simulation bodies set
            let rigid_body_handle = bodies.insert(rigid_body);
            // Create shape for bullet
            let shape = ShapeHandle::new(Ball::new(1.0));
            // Create collider with shape and attach it to rigid body
            let collider = ColliderDesc::new(shape)
                .density(1.0)
                .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
                .build(BodyPartHandle(rigid_body_handle, 0));
            // Add collider to simulation colliders set
            let collider_handle = colliders.insert(collider);

            let bullet = Bullet {
                shooter_id: self.shooter_id,
                diameter,
                body_handle: rigid_body_handle,
                collider_handle,
            };
            bullets.push(bullet);
        }
    }
    pub fn move_player(&mut self) {
        if self.pressing_left {
            self.direction -= PI / 36.0;
        }
        if self.pressing_right {
            self.direction += PI / 36.0;
        }
        if self.pressing_up {
            let push_to = Vector2::new(self.speed, 0.0);
            self.position += UnitComplex::new(self.direction).transform_vector(&push_to);
        }
        if self.pressing_down {
            let push_to = Vector2::new(self.speed, 0.0);
            self.position -= UnitComplex::new(self.direction).transform_vector(&push_to);
        }
        /*
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
        */
        if self.position.x < self.size / 2.0 {
            self.position.x = self.size / 2.0;
            /*
            self.direction = match self.direction {
                Direction::Left => Direction::Right,
                Direction::UpLeft => Direction::UpRight,
                Direction::DownLeft => Direction::DownRight,
                _ => self.direction,
            }
            */
        }
        if self.position.x > GAME_WIDTH - self.size / 2.0 {
            self.position.x = GAME_WIDTH - self.size / 2.0;
            /*
            self.direction = match self.direction {
                Direction::Right => Direction::Left,
                Direction::UpRight => Direction::UpLeft,
                Direction::DownRight => Direction::DownLeft,
                _ => self.direction,
            }
            */
        }
        if self.position.y < self.size / 2.0 {
            self.position.y = self.size / 2.0;
            /*
            self.direction = match self.direction {
                Direction::Up => Direction::Down,
                Direction::UpLeft => Direction::DownLeft,
                Direction::UpRight => Direction::DownRight,
                _ => self.direction,
            }
            */
        }
        if self.position.y > GAME_HEIGHT - self.size / 2.0 {
            self.position.y = GAME_HEIGHT - self.size / 2.0;
            /*
            self.direction = match self.direction {
                Direction::Down => Direction::Up,
                Direction::DownRight => Direction::UpRight,
                Direction::DownLeft => Direction::UpLeft,
                _ => self.direction,
            }
            */
        }
    }
    pub fn draw_player(&mut self, ctx: &mut Context) -> ggez::GameResult {
        graphics::draw(
            ctx,
            &self.image,
            graphics::DrawParam::new()
                .dest([self.position.x, self.position.y])
                .offset([0.5, 0.5])
                .rotation(self.direction)
                .scale([1.0, 1.0]),
        )?;
        Ok(())
    }
}
