#![windows_subsystem = "windows"] // hide the console window

mod game;
mod display;
mod config;

use crate::config::Config;
use crate::display::Display;
use crate::game::Game;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main()
{
    let config = Config::new("config.json").unwrap();

    let mut game = Game::new(WIDTH, HEIGHT, 10, 3,
                             config.head_color, config.snake_color, config.background_color)
        .unwrap();

    let game_area = ((0..WIDTH as usize), (0..HEIGHT as usize));
    let down_bar = ((0..WIDTH as usize), (HEIGHT as usize..(HEIGHT as usize) + 100));
    let mut display = Display::new("Super Snake", game_area, down_bar)
        .unwrap();

    display.display_loop(&mut game)
        .unwrap();
}

