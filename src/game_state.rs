extern crate nalgebra as na;
extern crate ncollide2d;
use crate::bullet::Bullet;
use crate::common::{GAME_HEIGHT, GAME_WIDTH, PLAYER_SHOOT_DELAY, PLAYER_SIZE, PLAYER_SPEED};
use crate::player::Player;
use ggez::event::KeyCode;
use ggez::graphics::Color;
use ggez::input::keyboard::KeyMods;
use ggez::{event, graphics, Context};
use na::{Point2, Vector2};
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{
    BodyPartHandle, BodyStatus, ColliderDesc, DefaultBodySet, DefaultColliderSet, Ground,
    RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld, MechanicalWorld};
use std::f64::consts::PI;

pub struct GameState {
    bullets: Vec<Bullet>,
    player1: Player,
    player2: Player,
    bullet_image_width: f32,
    bullet_image_height: f32,
    bullet_batch: graphics::spritebatch::SpriteBatch,
    arena_image: graphics::Image,
    mechanical_world: DefaultMechanicalWorld<f64>,
    geometrical_world: DefaultGeometricalWorld<f64>,
    bodies: DefaultBodySet<f64>,
    colliders: DefaultColliderSet<f64>,
    forces: DefaultForceGeneratorSet<f64>,
    constraints: DefaultJointConstraintSet<f64>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> ggez::GameResult<GameState> {
        let mut player1 = Player {
            position: Point2::new(200.0, 240.0),
            direction: 0.0,
            shooter_id: 0,
            color: graphics::Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            moving: false,
            pressing_up: false,
            pressing_down: false,
            pressing_left: false,
            pressing_right: false,
            pressing_shoot: false,
            speed: PLAYER_SPEED,
            size: PLAYER_SIZE,
            health: 100.0,
            image: graphics::Image::new(ctx, "/Player 1.png")?, /*graphics::Image::solid(
                                                                    ctx,
                                                                    20,
                                                                    graphics::Color {
                                                                        r: 1.0,
                                                                        g: 0.0,
                                                                        b: 0.0,
                                                                        a: 1.0,
                                                                    },
                                                                )?*/
            shoot_timer: PLAYER_SHOOT_DELAY, //let image1 = graphics::Image::new(ctx, "/dragon1.png")?;
            bullet_color: Color {
                r: 0.6,
                g: 1.0,
                b: 0.6,
                a: 1.0,
            },
        };
        let mut player2 = Player {
            position: Point2::new(600.0, 240.0),
            direction: PI,
            shooter_id: 1,
            color: graphics::Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            moving: false,
            pressing_up: false,
            pressing_down: false,
            pressing_left: false,
            pressing_right: false,
            pressing_shoot: false,
            speed: PLAYER_SPEED,
            size: PLAYER_SIZE,
            health: 100.0,
            image: graphics::Image::new(ctx, "/Player 2.png")?,
            /*graphics::Image::solid(
                ctx,
                20,
                graphics::Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                },
            )?*/
            shoot_timer: PLAYER_SHOOT_DELAY, //let image1 = graphics::Image::new(ctx, "/dragon1.png")?;
            bullet_color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
        };
        let bullets: Vec<Bullet> = Vec::new();
        let bullet_image = graphics::Image::new(ctx, "/Fireball.png")?;
        let mut s = GameState {
            bullets,
            player1,
            player2,
            bullet_image_width: bullet_image.width() as f32,
            bullet_image_height: bullet_image.height() as f32,
            bullet_batch: graphics::spritebatch::SpriteBatch::new(bullet_image),
            arena_image: graphics::Image::new(ctx, "/Arena.png")?,
            mechanical_world: DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0)),
            geometrical_world: DefaultGeometricalWorld::<f64>::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            constraints: DefaultJointConstraintSet::new(),
            forces: DefaultForceGeneratorSet::new(),
        };
        // Advanced CCD physics accuracy
        s.mechanical_world.integration_parameters.max_ccd_substeps = 2;
        GameState::create_ground(&mut s);
        Ok(s)
    }

    pub fn create_ground(game_state: &mut GameState) {
        let create_wall = |game_state: &mut GameState, width: f64, height: f64, x: f64, y: f64| {
            game_state.colliders.insert(
                ColliderDesc::new(ShapeHandle::new(Cuboid::new(Vector2::new(width, height))))
                    .density(1.0)
                    .material(MaterialHandle::new(BasicMaterial::new(1.0, 0.0)))
                    .build(BodyPartHandle(
                        game_state.bodies.insert(
                            RigidBodyDesc::new()
                                .status(BodyStatus::Static)
                                .translation(Vector2::new(x, y))
                                .build(),
                        ),
                        0,
                    )),
            );
        };

        // Top
        create_wall(game_state, GAME_WIDTH, 1.0, GAME_WIDTH / 2.0, -0.5);

        // Bottom
        create_wall(
            game_state,
            GAME_WIDTH,
            1.0,
            GAME_WIDTH / 2.0,
            GAME_HEIGHT + 0.5,
        );

        // Left
        create_wall(game_state, 1.0, GAME_HEIGHT, -0.5, GAME_HEIGHT / 2.0);

        // Right
        create_wall(
            game_state,
            1.0,
            GAME_HEIGHT,
            GAME_WIDTH + 0.5,
            GAME_HEIGHT / 2.0,
        );
    }
}

impl event::EventHandler for GameState {
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
            KeyCode::Up => self.player2.pressing_up = true,
            KeyCode::Down => self.player2.pressing_down = true,
            KeyCode::Left => self.player2.pressing_left = true,
            KeyCode::Right => self.player2.pressing_right = true,
            KeyCode::Space => self.player2.pressing_shoot = true,
            KeyCode::E => self.player1.pressing_up = true,
            KeyCode::D => self.player1.pressing_down = true,
            KeyCode::S => self.player1.pressing_left = true,
            KeyCode::F => self.player1.pressing_right = true,
            KeyCode::A => self.player1.pressing_shoot = true,
            _ => (),
        };
        println!(
            "Pressed keyboard key '{:?}' with keymods '{:?}' repeated: '{:?}'",
            keycode, _keymod, _repeat
        );
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        match keycode {
            KeyCode::Up => self.player2.pressing_up = false,
            KeyCode::Down => self.player2.pressing_down = false,
            KeyCode::Left => self.player2.pressing_left = false,
            KeyCode::Right => self.player2.pressing_right = false,
            KeyCode::Space => self.player2.pressing_shoot = false,
            KeyCode::E => self.player1.pressing_up = false,
            KeyCode::D => self.player1.pressing_down = false,
            KeyCode::S => self.player1.pressing_left = false,
            KeyCode::F => self.player1.pressing_right = false,
            KeyCode::A => self.player1.pressing_shoot = false,
            _ => (),
        };
        println!(
            "Released keyboard key '{:?}' with keymods '{:?}'",
            keycode, _keymod
        );
    }
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.player1.move_player();
        self.player2.move_player();
        self.player1
            .shoot(&mut self.bullets, &mut self.bodies, &mut self.colliders);
        self.player2
            .shoot(&mut self.bullets, &mut self.bodies, &mut self.colliders);
        // Run the simulation.
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.constraints,
            &mut self.forces,
        );
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        graphics::draw(
            ctx,
            &self.arena_image,
            graphics::DrawParam::new().dest([0.0, 0.0]).scale([
                GAME_WIDTH as f32 / self.arena_image.width() as f32,
                GAME_HEIGHT as f32 / self.arena_image.height() as f32,
            ]),
        )?;
        self.player1.draw_player(ctx)?;
        self.player2.draw_player(ctx)?;
        for bullet in &mut self.bullets {
            bullet.draw_bullet(
                ctx,
                &mut self.bullet_batch,
                self.bullet_image_width,
                self.bullet_image_height,
                &mut self.colliders,
            )?;
        }
        graphics::draw(ctx, &self.bullet_batch, graphics::DrawParam::new())?;
        self.bullet_batch.clear();
        graphics::present(ctx)?;
        Ok(())
    }
}
