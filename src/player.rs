use crate::bullet::Bullet;
use crate::common::{
    FIREBALL_SIZE, FIREBALL_SPEED, FIREBALL__ROTATION_SPEED, GAME_FPS, GAME_HEIGHT, GAME_WIDTH,
    PLAYER_SHOOT_DELAY,
};
extern crate nalgebra as na;
use self::na::Isometry2;
use ggez::graphics::Color;
use ggez::{graphics, Context};
use na::{Point2, Rotation2, UnitComplex, Vector2};
use ncollide2d::shape::{Ball, ShapeHandle};
use nphysics2d::algebra::Velocity2;
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{
    BodyPartHandle, ColliderDesc, DefaultBodySet, DefaultColliderSet, RigidBodyDesc,
};
use std::f64::consts::PI;

pub struct Player {
    pub position: Point2<f64>,
    pub shooter_id: u32,
    pub moving: bool,
    pub pressing_up: bool,
    pub pressing_down: bool,
    pub pressing_left: bool,
    pub pressing_right: bool,
    pub pressing_shoot: bool,
    pub direction: f64,
    pub speed: f64,
    pub size: f64,
    pub color: graphics::Color,
    pub image: graphics::Image,
    pub health: f32,
    pub shoot_timer: f64,
    pub bullet_color: Color,
}

impl Player {
    pub fn shoot(
        &mut self,
        bullets: &mut Vec<Bullet>,
        bodies: &mut DefaultBodySet<f64>,
        colliders: &mut DefaultColliderSet<f64>,
    ) {
        self.shoot_timer += 1.0;
        if self.pressing_shoot && self.shoot_timer >= PLAYER_SHOOT_DELAY {
            self.shoot_timer = 0.0;
            let diameter = FIREBALL_SIZE;
            let speed = FIREBALL_SPEED * GAME_FPS;
            // Create rigid body for bullet
            let rb_desc = RigidBodyDesc::new()
                .position(Isometry2::new(
                    Vector2::new(self.position.x, self.position.y),
                    0.0,
                ))
                .velocity(Velocity2::new(
                    UnitComplex::new(self.direction).transform_vector(&Vector2::new(speed, 0.0)),
                    FIREBALL__ROTATION_SPEED * GAME_FPS,
                ))
                .mass(1.0);
            let rigid_body = rb_desc.build();
            // Add it to simulation bodies set
            let rigid_body_handle = bodies.insert(rigid_body);
            // Create shape for bullet
            let shape = ShapeHandle::new(Ball::new(diameter * 0.7 / 2.0));
            // Create collider with shape and attach it to rigid body
            let collider = ColliderDesc::new(shape)
                .ccd_enabled(true)
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
                color: self.bullet_color,
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
            self.position +=
                UnitComplex::new(self.direction).transform_vector(&Vector2::new(self.speed, 0.0));
        }
        if self.pressing_down {
            self.position -=
                UnitComplex::new(self.direction).transform_vector(&Vector2::new(self.speed, 0.0));
        }
        if self.position.x < self.size / 2.0 {
            self.position.x = self.size / 2.0;
        }
        if self.position.x > GAME_WIDTH - self.size / 2.0 {
            self.position.x = GAME_WIDTH - self.size / 2.0;
        }
        if self.position.y < self.size / 2.0 {
            self.position.y = self.size / 2.0;
        }
        if self.position.y > GAME_HEIGHT - self.size / 2.0 {
            self.position.y = GAME_HEIGHT - self.size / 2.0;
        }
    }
    pub fn draw_player(&mut self, ctx: &mut Context) -> ggez::GameResult {
        graphics::draw(
            ctx,
            &self.image,
            graphics::DrawParam::new()
                .dest([self.position.x as f32, self.position.y as f32])
                .offset([0.5, 0.5])
                .rotation((self.direction - PI / 2.0) as f32)
                .scale([
                    self.size as f32 / self.image.width() as f32,
                    self.size as f32 / self.image.height() as f32,
                ]),
        )?;
        Ok(())
    }
}
