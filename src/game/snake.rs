use crate::game::cell::{Cell, Direction};
use crate::game::cell::Direction::{DOWN, LEFT, RIGHT, UP};

use minifb::Key;
use crate::game::{Color, Input};


pub struct Snake
{
    cells: Vec<Cell>,
}

impl Snake
{
    pub fn new
    (
        screen_width: i64,
        screen_height: i64,
        initial_cell_count: i64,
        cell_size: i64,
        head_color: Color,
        snake_color: Color,
    ) -> Snake
    {
        let mut cells = Vec::new();
        for i in 0..initial_cell_count
        {
            let left = screen_width / 2;
            let top = screen_height / 2 - (initial_cell_count - 1 - i) * cell_size;
            let color = if i == 0
            {
                head_color
            }
            else
            {
                snake_color
            };

            let cell = Cell::new(left, top, cell_size, UP, screen_width, screen_height, color);
            cells.push(cell);
        }

        Snake { cells }
    }

    fn step(&mut self)
    {
        for i in (0..self.cells.len()).rev()
        {
            self.cells[i].step();

            if i < self.cells.len() - 1
            {
                let direction = self.cells[i].get_direction().clone();
                self.cells[i + 1].set_direction(direction);
            }
        }

    }

    fn turn(&mut self, new_direction: Direction)
    {
        let head = &mut self.cells[0];
        let head_direction = head.get_direction();
        match (head_direction, new_direction)
        {
            (DOWN, UP) | (UP, DOWN) | (LEFT, RIGHT) | (RIGHT, LEFT) => return,
            (_, new_dir) => head.set_direction(new_dir),
        }
    }

    pub fn go(&mut self, input: Option<Input>)
    {
        self.step();

        // Check for turning the snake.
        let new_direction = match input
        {
            // For example.
            Some(Key::Up) => Some(UP),
            Some(Key::Down) => Some(DOWN),
            Some(Key::Left) => Some(LEFT),
            Some(Key::Right) => Some(RIGHT),
            _ => None,
        };

        if let Some(dir) = new_direction
        {
            self.turn(dir);
        }
    }

    pub(crate) fn get_cells(&self) -> &Vec<Cell>
    {
        &self.cells
    }
}
