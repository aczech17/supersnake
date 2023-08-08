use minifb::{Key, Window, WindowOptions};
use crate::game::Game;

pub struct Display
{
    //game_width: i64,
    //game_height: i64,
    window: Window,
    pixels: Vec<u32>,
}

impl Display
{
    pub fn new(name: &str, width: usize, height: usize) -> Result<Display, String>
    {
        let window = Window::new(name, width, height, WindowOptions::default());
        let mut window = match window
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
            window,
            pixels: vec![0; width * height],
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

    fn draw(&mut self, game: &Game)
    {
        let background_color = game.get_background_color();

        for pixel in &mut self.pixels
        {
            let (r, g, b) = background_color;
            let pixel_val = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);

            *pixel = pixel_val;
        }

        let (width, _height) = game.get_resolution();


        let mut all_cells = game.get_snake_cells().clone();
        let point_cell = game.get_point_cell().clone();
        all_cells.push(point_cell);

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
                    let index = y * width as usize + x;
                    let (r, g, b) = color;
                    let pixel_val = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                    self.pixels[index] = pixel_val;
                }
            }
        }
    }

    pub fn display_loop(&mut self, game: &mut Game)
    {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape)
        {
            let input = self.get_input();

            let res = game.go(input);
            if let Some(points) = res
            {
                println!("{}", points);
                return;
            }

            self.draw(game);

            let (width, height) = self.window.get_size();
            self.window.update_with_buffer(&self.pixels, width, height)
                .unwrap();
        }
    }
}