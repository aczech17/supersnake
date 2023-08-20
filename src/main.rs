#![windows_subsystem = "windows"] // hide the console window

mod config;
use crate::config::Config;
mod game;
use crate::game::Game;
mod display;
use crate::display::Display;

fn main()
{
    let config = Config::new("config.json").unwrap();

    let mut game = Game::new(&config)
        .unwrap();

    let mut display = Display::new("Super Snake", &config, &mut game, "assets/music");

    display.run();
}
