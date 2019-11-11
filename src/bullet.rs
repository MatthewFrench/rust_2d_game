use crate::common::{GAME_HEIGHT, GAME_WIDTH};
use ggez::graphics::Rect;
//use ggez::nalgebra as na;
extern crate nalgebra as na;
use self::na::{Isometry2, Translation2};
use ggez::{graphics, Context};
use mint;
use na::{Point2, Rotation2, UnitComplex, Vector2};
use nphysics2d::object::{
    DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle, DefaultColliderSet,
};
use std::borrow::Borrow;
use std::f32::consts::PI;

pub struct Bullet {
    pub shooter_id: u32,
    pub diameter: f32,
    pub body_handle: DefaultBodyHandle,
    pub collider_handle: DefaultColliderHandle,
}

impl Bullet {
    pub fn move_bullet(&mut self) {
        /*
        let push_to = Vector2::new(self.speed, 0.0);
        self.position += UnitComplex::new(self.direction).transform_vector(&push_to);
        if self.position.x < self.diameter / 2.0 {
            self.position.x = self.diameter / 2.0;
            self.direction += PI / 2.0;
        }
        if self.position.x > GAME_WIDTH - self.diameter / 2.0 {
            self.position.x = GAME_WIDTH - self.diameter / 2.0;
            self.direction += PI / 2.0;
        }
        if self.position.y < self.diameter / 2.0 {
            self.position.y = self.diameter / 2.0;
            self.direction += PI / 2.0;
        }
        if self.position.y > GAME_HEIGHT - self.diameter / 2.0 {
            self.position.y = GAME_HEIGHT - self.diameter / 2.0;
            self.direction += PI / 2.0;
        }
        self.rotation += PI / 180.0;
        */
    }
    pub fn draw_bullet(
        &mut self,
        ctx: &mut Context,
        bullet_batch: &mut ggez::graphics::spritebatch::SpriteBatch,
        bullet_image_width: f32,
        bullet_image_height: f32,
        colliders: &mut DefaultColliderSet<f32>,
    ) -> ggez::GameResult {
        let collider_optional = colliders.get(self.collider_handle);
        if let Some(collider) = collider_optional {
            let position: &Isometry2<f32> = collider.position();
            let d = graphics::DrawParam::new()
                .dest([position.translation.x, position.translation.y])
                .offset([0.5, 0.5])
                .rotation(position.rotation.re)
                .scale([
                    self.diameter / bullet_image_width,
                    self.diameter / bullet_image_height,
                ]);
            bullet_batch.add(d);
        }
        Ok(())
    }
}
