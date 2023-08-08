mod game;

use minifb::{Key, Window, WindowOptions};
use crate::game::{Color, Game};


fn get_input(window: &Window) -> Option<Key>
{
    let keys = [Key::Left, Key::Right, Key::Up, Key::Down];
    for key in keys
    {
        if window.is_key_down(key)
        {
            return Some(key);
        }
    }
    None
}

fn draw(game: &Game, pixels: &mut Vec<u32>)
{
    let background_color = game.get_background_color();
    for pixel in &mut *pixels
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
                pixels[index] = pixel_val;
            }
        }
    }
}

const WIDTH: i64 = 800;
const HEIGHT: i64 = 600;
const YELLOW: Color = (255, 255, 0);
const GREEN: Color = (0, 255, 0);
const GRAY: Color = (128, 128, 128);

fn main()
{
    let mut game = Game::new(WIDTH, HEIGHT, 10, 3,
                             YELLOW, GREEN, GRAY)
        .unwrap();

    let mut pixels = vec![0; (WIDTH * HEIGHT) as usize];
    let mut window = Window::new
    (
        "HUJ",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default(),
    )
        .expect("Spierdoliło się okno.");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    while window.is_open() && !window.is_key_down(Key::Escape)
    {
        let input = get_input(&window);
        let res = game.go(input);
        if let Some(points) = res
        {
            println!("{}", points);
            return;
        }

        draw(&game, &mut pixels);
        window.update_with_buffer(&pixels, WIDTH as usize, HEIGHT as usize)
            .unwrap();
    }
}

