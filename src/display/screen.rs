use crate::game::{Color, Game};
use minifb::{Key, Window, WindowOptions};

use std::ops::Range;
use bmp::Image;

type Area = (Range<usize>, Range<usize>);

use itertools::Itertools;
use crate::config::Config;
use crate::display::screen::DisplayState::{GameOver, Playing, Stop};

#[derive(PartialEq)]
pub enum DisplayState
{
    Playing,
    GameOver,
    Stop,
}

extern crate bmp;

pub struct Screen
{
    game_area: Area,
    down_bar: Area,
    window: Window,
    pixels: Vec<u32>,
    delay: u64,
    bitmaps: Vec<Image>,
}

const INITIAL_DELAY: u64 = 80000;

impl Screen
{
    pub fn new(name: &str, config: &Config) -> Result<Screen, String>
    {
        let game_area =
            (
                (0..config.screen_width as usize),
                (0..config.screen_height as usize),
            );

        let down_bar =
            (
                (0..config.screen_width as usize),

                (config.screen_height as usize..(config.screen_height as usize) +
                    config.screen_height as usize / 4),
            );

        let (game_xs, _game_ys) = game_area.clone();
        let (_down_bar_xs, down_bar_ys) = down_bar.clone();
        let window_width = game_xs.end;
        let window_height = down_bar_ys.end;

        let window_creation = Window::new(name, window_width, window_height,
                                          WindowOptions::default());

        let window = match window_creation
        {
            Ok(win) => win,
            Err(err) =>
                {
                    let err_msg = format!("Could not create display: {}\n", err.to_string())
                        .to_string();
                    return Err(err_msg);
                }
        };

        let mut bitmaps = vec![];
        for i in 0..=10
        {
            let filename = if i == 10
            {
                "assets/game_over.bmp".to_string()
            }
            else
            {
                format!("assets/{i}.bmp")
            };

            match bmp::open(filename)
            {
                Ok(bmp) => bitmaps.push(bmp),
                Err(e) => return Err(e.to_string()),
            };
        }

        let display = Screen
        {
            game_area,
            down_bar,
            window,
            pixels: vec![0; window_width * window_height],
            delay: 80000,
            bitmaps,
        };

        Ok(display)
    }

    fn get_input(&self) -> Option<Key>
    {
        let keys = [Key::Left, Key::Right, Key::Up, Key::Down];
        for key in keys
        {
            if self.window.is_key_down(key)
            {
                return Some(key);
            }
        }
        None
    }

    fn color_to_pixel(color: Color) -> u32
    {
        let (r, g, b) = color;
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    fn draw_game_background(&mut self, game: &Game)
    {
        let background_color = game.get_background_color();

        let (game_xs, game_ys) = self.game_area.clone();
        let game_width = game_xs.end - game_xs.start;

        for (x, y) in game_xs.cartesian_product(game_ys)
        {
            let index = y * game_width + x;
            let pixel_val = Self::color_to_pixel(background_color);
            self.pixels[index] = pixel_val;
        }
    }

    fn draw_cells(&mut self, game: &Game)
    {
        // Join snake cells and point cell to display all of them.
        let mut all_cells = game.get_snake_cells().clone();
        let point_cell = game.get_point_cell().clone();
        all_cells.push(point_cell);

        let (game_xs, _game_ys) = self.game_area.clone();
        let game_width = game_xs.end - game_xs.start;
        for cell in all_cells
        {
            let left = cell.get_left() as usize;
            let down = cell.get_bottom() as usize;
            let right = cell.get_right() as usize;
            let up = cell.get_top() as usize;
            let color = cell.get_color();

            for x in left..right
            {
                for y in up..down
                {
                    let index = y * game_width as usize + x;
                    let (r, g, b) = color;
                    let pixel_val = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                    self.pixels[index] = pixel_val;
                }
            }
        }
    }

    fn draw_game_area(&mut self, game: &Game) -> Result<(), String>
    {
        self.draw_game_background(game);
        self.draw_cells(game);

        Ok(())
    }

    fn draw_down_bar(&mut self)
    {
        let down_bar_color = 0; // black

        let (xs, ys) = self.down_bar.clone();
        let bar_width = xs.end - xs.start;
        for (x, y) in xs.cartesian_product(ys)
        {
            let index = y * bar_width + x;
            self.pixels[index] = down_bar_color;
        }
    }

    fn draw_digit(&mut self, digit: char, digit_position: usize) -> Result<(), String>
    {
        let bitmap_index = digit as usize - 48;
        let bmp = &self.bitmaps[bitmap_index];

        let bmp_xs = 0..bmp.get_width();
        let bmp_ys = 0..bmp.get_height();

        let (down_bar_xs, down_bar_ys) = self.down_bar.clone();
        let down_bar_left = down_bar_xs.start;
        let down_bar_top = down_bar_ys.start;
        let screen_width = down_bar_xs.end - down_bar_xs.start;

        for (bmp_x, bmp_y) in bmp_xs.cartesian_product(bmp_ys)
        {
            let color = bmp.get_pixel(bmp_x, bmp_y);
            let bmp_value = Self::color_to_pixel((color.r, color.g, color.b));

            let screen_x = down_bar_left + digit_position + bmp_x as usize;
            let screen_y = down_bar_top + bmp_y as usize;
            let screen_index = screen_y * screen_width + screen_x;
            self.pixels[screen_index] = bmp_value;
        }

        Ok(())
    }

    fn draw_points(&mut self, game: &Game) -> Result<(), String>
    {
        let digit_width = self.bitmaps[0].get_width() as usize;
        let points = game.get_points().to_string();
        let mut digit_position = 0;
        for digit in points.chars()
        {
            self.draw_digit(digit, digit_position)?;
            digit_position += digit_width;
        }

        Ok(())
    }

    fn draw_game(&mut self, game: &Game) -> Result<(), String>
    {
        self.draw_game_area(game)?;
        self.draw_down_bar();
        self.draw_points(game)?;

        Ok(())
    }

    fn draw_game_over(&mut self)
    {
        let (game_area_xs, game_area_ys) = self.game_area.clone();
        let game_area_width = game_area_xs.end - game_area_xs.start;

        let game_over_img = &self.bitmaps[10];

        for (x, y) in game_area_xs.cartesian_product(game_area_ys)
        {
            let pixel = game_over_img.get_pixel(x as u32, y as u32);
            let pixel_value = Self::color_to_pixel((pixel.r, pixel.g, pixel.b));
            let index = y * game_area_width + x;
            self.pixels[index] = pixel_value;
        }
    }

    fn game_pace_to_delay(pace: u64) -> u64
    {
        if INITIAL_DELAY >= pace
        {INITIAL_DELAY - pace}
        else { 0 }
    }

    pub fn draw(&mut self, game: &mut Game) -> Result<DisplayState, String>
    {
        let pace = game.get_pace();
        self.delay = Self::game_pace_to_delay(pace);

        // Limit fps;
        self.window.limit_update_rate(Some(std::time::Duration::from_micros(self.delay)));


        let input = self.get_input();

        if game.is_running()
        {
            game.go(input);
            self.draw_game(game)?;
        }
        else
        {
            self.draw_game_over();
        }

        let (width, height) = self.window.get_size();

        let display_result = self.window.update_with_buffer(&self.pixels, width, height);
        match display_result
        {
            Ok(_) => {},
            Err(err) =>
                {
                    let err_msg = format!("Could not display to the window. {}", err.to_string())
                        .to_string();
                    return Err(err_msg);
                }
        }

        let window_open = self.window.is_open() && !self.window.is_key_down(Key::Escape);
        if !window_open
        {
            return Ok(Stop);
        }

        return match game.is_running()
        {
            true => Ok(Playing),
            false => Ok(GameOver),
        };
    }
}
