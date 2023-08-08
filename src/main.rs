mod game;
mod display;

use crate::display::Display;
use crate::game::{Color, Game};

const WIDTH: i64 = 400;
const HEIGHT: i64 = 400;
const YELLOW: Color = (255, 255, 0);
const GREEN: Color = (0, 255, 0);
//const GRAY: Color = (128, 128, 128);
const NAVY_BLUE: Color = (0, 0, 128);

fn main()
{
    let mut game = Game::new(WIDTH, HEIGHT, 10, 3,
                             YELLOW, GREEN, NAVY_BLUE)
        .unwrap();

    let mut display = Display::new("Snake", WIDTH as usize, HEIGHT as usize + 100)
        .unwrap();

    display.display_loop(&mut game);
}

