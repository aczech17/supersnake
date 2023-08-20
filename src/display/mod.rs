use crate::config::Config;
use crate::display::screen::Screen;
use crate::display::screen::DisplayState::{GameOver, Stop};
use crate::display::sound::Sound;
use crate::game::Game;

mod screen;
mod sound;

pub struct Display<'a>
{
    game: &'a mut Game,
    screen: Screen,
    sound: Option<Sound>,
}

impl <'a> Display<'a>
{
    pub fn new(window_name: &str, config: &Config, game: &'a mut Game, music_path: &str) -> Self
    {
        let screen = Screen::new(window_name, config)
            .unwrap();

        let sound = Sound::new(&music_path.to_string())
            .unwrap();

        let sound = Some(sound);

        Display
        {
            game,
            screen,
            sound,
        }
    }

    pub fn run(&mut self)
    {
        loop
        {
            match &mut self.sound
            {
                Some(s) => s.play(),
                None => {},
            };

            let status = self.screen.draw(&mut self.game).unwrap();
            if status == GameOver
            {
                self.sound.take();
            }
            if status == Stop
            {
                break;
            }
        }
    }
}