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

    pub(crate) fn get_cells(&self) -> &Vec<Cell>
    {
        &self.cells
    }

    pub(crate) fn is_tangled(&self) -> bool
    {
        let head = &self.cells[0];
        for cell in self.cells.iter().skip(1)
        {
            if cell.overlap(head)
            {return true;}
        }
        false
    }

    pub(crate) fn change_head(&mut self, mut new_head: Cell)
    {
        let snake_color = self.cells[1].get_color();
        let head = &mut self.cells[0];
        let head_direction = head.get_direction().clone();
        let head_color = head.get_color();

        head.set_color(snake_color);

        new_head.set_direction(head_direction);
        new_head.set_color(head_color);

        let mut new_cells = vec![new_head];
        new_cells.append(&mut self.cells);

        self.cells = new_cells;
    }

    pub(crate) fn is_collecting_point(&self, point_cell: &Cell) -> bool
    {
        // Checking for the next head position.

        let head = &self.cells[0];
        let mut new_head = head.clone();
        new_head.step();
        if new_head.overlap(point_cell)
        {
            return true;
        }
        false
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
}
