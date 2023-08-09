mod game;
mod display;
mod bmp;

use crate::display::Display;
use crate::game::{Color, Game};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;
const YELLOW: Color = (255, 255, 0);
const GREEN: Color = (0, 255, 0);
//const GRAY: Color = (128, 128, 128);
const NAVY_BLUE: Color = (0, 0, 128);

fn main()
{
    let mut game = Game::new(WIDTH, HEIGHT, 10, 3,
                             YELLOW, GREEN, NAVY_BLUE)
        .unwrap();

    let game_area = ((0..WIDTH as usize), (0..HEIGHT as usize));
    let down_bar = ((0..WIDTH as usize), (HEIGHT as usize..(HEIGHT as usize) + 100));
    let mut display = Display::new("Snake", game_area, down_bar)
        .unwrap();

    display.display_loop(&mut game)
        .unwrap();
}

