extern crate piston_window;

use piston_window::*;

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
}

struct Bullet {
    x: i32,
    y: i32,
    direction: Direction,
    shooter_id: u32,
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Bullet Game!", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let player1 = Player {
        x: 200,
        y: 240,
        direction: Direction::Right,
        shooter_id: 0,
        moving: false,
    };
    let player2 = Player {
        x: 440,
        y: 240,
        direction: Direction::Left,
        shooter_id: 1,
        moving: false,
    };

    let bullets: Vec<Bullet> = Vec::new();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            //if key == Key::C {
            //}

            println!("Pressed keyboard key '{:?}'", key);
        };
        if let Some(Button::Keyboard(key)) = event.release_args() {
            //if key == Key::C {
            //}

            println!("Released keyboard key '{:?}'", key);
        };
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);

            // Player 1
            rectangle(
                [1.0, 0.0, 0.0, 1.0], // red
                [player1.x as f64 - 10.0, player1.y as f64 - 10.0, 20.0, 20.0],
                context.transform,
                graphics,
            );

            // Player 2
            rectangle(
                [0.0, 1.0, 0.0, 1.0], // green
                [player2.x as f64 - 10.0, player2.y as f64 - 10.0, 20.0, 20.0],
                context.transform,
                graphics,
            );
        });

        if let Some(u) = event.update_args() {
            // Logic Update
        }
    }
}
