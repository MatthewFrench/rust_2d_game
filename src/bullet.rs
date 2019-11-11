use crate::common::{GAME_HEIGHT, GAME_WIDTH};
use ggez::graphics::Rect;
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
    pub diameter: f64,
    pub body_handle: DefaultBodyHandle,
    pub collider_handle: DefaultColliderHandle,
}

impl Bullet {
    pub fn draw_bullet(
        &mut self,
        ctx: &mut Context,
        bullet_batch: &mut ggez::graphics::spritebatch::SpriteBatch,
        bullet_image_width: f32,
        bullet_image_height: f32,
        colliders: &mut DefaultColliderSet<f64>,
    ) -> ggez::GameResult {
        let collider_optional = colliders.get(self.collider_handle);
        if let Some(collider) = collider_optional {
            let position: &Isometry2<f64> = collider.position();
            let d = graphics::DrawParam::new()
                .dest([position.translation.x as f32, position.translation.y as f32])
                .offset([0.5, 0.5])
                .rotation(position.rotation.re as f32)
                .scale([
                    self.diameter as f32 / bullet_image_width,
                    self.diameter as f32 / bullet_image_height,
                ]);
            bullet_batch.add(d);
        }
        Ok(())
    }
}
