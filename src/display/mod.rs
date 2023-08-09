use crate::game::{Color, Game};
use minifb::{Key, Window, WindowOptions};

use std::ops::Range;
type Area = (Range<usize>, Range<usize>);

use itertools::Itertools;

pub struct Display
{
    game_area: Area,
    down_bar: Area,
    window: Window,
    pixels: Vec<u32>,
}

impl Display
{
    pub fn new(name: &str, game_area: Area, down_bar: Area) -> Result<Display, String>
    {
        let (game_xs, _game_ys) = game_area.clone();
        let (_down_bar_xs, down_bar_ys) = down_bar.clone();
        let window_width = game_xs.end;
        let window_height = down_bar_ys.end;

        let window_creation = Window::new(name, window_width, window_height,
                                 WindowOptions::default());

        let mut window = match window_creation
        {
            Ok(win) => win,
            Err(err) =>
            {
                let err_msg = format!("Could not create display: {}\n", err.to_string())
                    .to_string();
                return Err(err_msg);
            }
        };

        // Limit fps
        window.limit_update_rate(Some(std::time::Duration::from_micros(40000)));

        let display = Display
        {
            game_area,
            down_bar,
            window,
            pixels: vec![0; window_width * window_height],
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
            let index = (y * game_width + x) as usize;
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

    fn draw_game(&mut self, game: &Game)
    {
        self.draw_game_background(game);
        self.draw_cells(game);
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

    fn draw(&mut self, game: &Game)
    {
        self.draw_game(game);
        self.draw_down_bar();
    }

    pub fn display_loop(&mut self, game: &mut Game) -> Result<(), String>
    {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape)
        {
            let input = self.get_input();

            let res = game.go(input);
            if let Some(points) = res
            {
                println!("{}", points);
                return Ok(());
            }

            self.draw(game);

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
        }

        Ok(())
    }
}