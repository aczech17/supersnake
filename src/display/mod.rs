use crate::display::screen::Screen;
use crate::display::screen::DisplayState::{GameOver, Stop};
use crate::display::sound::Sound;
use crate::game::Game;

pub mod screen;
pub mod sound;

pub struct Display<'a>
{
    game: &'a mut Game,
    screen: Screen,
    sound: Option<Sound>,
}

impl <'a> Display<'a>
{
    pub fn new(game: &'a mut Game, screen: Screen, sound: Option<Sound>) -> Display
    {
        Display
        {
            game,
            screen,
            sound,
        }
    }

    pub fn run(&mut self) -> Result<(), String>
    {
        loop
        {
            match &mut self.sound
            {
                Some(s) => s.play()?,
                None => {},
            };

            let status = self.screen.draw(&mut self.game)?;
            if status == GameOver
            {
                self.sound.take();
            }
            if status == Stop
            {
                break;
            }
        }

        Ok(())
    }
}