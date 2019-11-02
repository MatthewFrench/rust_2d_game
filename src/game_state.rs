use crate::bullet::Bullet;
use crate::common::Direction;
use crate::player::Player;
use ggez::event::KeyCode;
use ggez::input::keyboard::KeyMods;
use ggez::nalgebra as na;
use ggez::{event, graphics, Context};

pub struct GameState {
    bullets: Vec<Bullet>,
    player1: Player,
    player2: Player,
}

impl GameState {
    pub fn new() -> ggez::GameResult<GameState> {
        let mut player1 = Player {
            x: 200,
            y: 240,
            direction: Direction::Right,
            shooter_id: 0,
            color: graphics::Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            },
            ..Default::default()
        };
        let mut player2 = Player {
            x: 600,
            y: 240,
            direction: Direction::Left,
            shooter_id: 1,
            color: graphics::Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            },
            ..Default::default()
        };
        let bullets: Vec<Bullet> = Vec::new();
        let s = GameState {
            bullets,
            player1,
            player2,
        };
        Ok(s)
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
        self.player1.shoot(&mut self.bullets);
        self.player2.shoot(&mut self.bullets);
        for bullet in &mut self.bullets {
            bullet.move_bullet();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let mb = &mut graphics::MeshBuilder::new();
        self.player1.draw_player(mb);
        self.player2.draw_player(mb);
        for bullet in &mut self.bullets {
            bullet.draw_bullet(mb);
        }
        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::new())?;
        graphics::present(ctx)?;
        Ok(())
    }
}
