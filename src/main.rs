#![windows_subsystem = "windows"] // hide the console window

mod config;
use crate::config::Config;
mod game;
use crate::game::Game;
mod display;
use crate::display::Display;
use crate::display::screen::Screen;
use crate::display::sound::Sound;

fn main()
{
    let config = match Config::new("config.json")
    {
        Ok(c) => c,
        Err(_) =>
        {
            eprintln!("Could not open the config file.");
            return;
        }
    };

    let mut game = match Game::new(&config)
    {
        Ok(g) => g,
        Err(msg) =>
        {
            eprintln!("{msg}");
            return;
        }
    };
    
    let screen = match Screen::new("Super snake", &config)
    {
        Ok(s) => s,
        Err(msg) =>
        {
            eprintln!("{msg}");
            return;
        }
    };

    let sound = match Sound::new(&"assets/music".to_string())
    {
        Ok(s) => Some(s),
        Err(msg) => 
        {
            eprintln!("Could not create the sound. {msg}");
            None
        }
    };

    let mut display = Display::new(&mut game, screen, sound);

    let _ = display.run();
}
