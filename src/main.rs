mod game;
mod display;

use crate::display::Display;
use crate::game::{Color, Game};

const WIDTH: i64 = 800;
const HEIGHT: i64 = 600;
const YELLOW: Color = (255, 255, 0);
const GREEN: Color = (0, 255, 0);
const GRAY: Color = (128, 128, 128);

fn main()
{
    let mut game = Game::new(WIDTH, HEIGHT, 10, 3,
                             YELLOW, GREEN, GRAY)
        .unwrap();

    let mut display = Display::new("Snake", WIDTH as usize, HEIGHT as usize)
        .unwrap();

    display.display_loop(&mut game);
}

