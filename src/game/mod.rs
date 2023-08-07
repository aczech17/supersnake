use crate::game::cell::Cell;
use crate::game::snake::Snake;

mod cell;
mod snake;

pub(crate) type Color = (u8, u8, u8);
type Input = minifb::Key;

pub(crate) struct Game
{
    screen_width: i64,
    screen_height: i64,
    cell_size: i64,
    initial_cell_count: i64,
    head_color: Color,
    snake_color: Color,
    background_color: Color,

    snake: Snake,
    points: u64,
}

impl Game
{
    pub(crate) fn new
    (
        screen_width: i64,
        screen_height: i64,
        cell_size: i64,
        initial_cell_count: i64,
        head_color: Color,
        snake_color: Color,
        background_color: Color,
    )
        -> Game
    {
        let snake = Snake::new
        (
            screen_width,
            screen_height,
            initial_cell_count,
            cell_size,
            head_color,
            snake_color
        );

        Game
        {
            screen_width,
            screen_height,
            cell_size,
            initial_cell_count,
            head_color,
            snake_color,
            background_color,
            snake,
            points: 0,
        }
    }

    pub(crate) fn get_snake_cells(&self) -> &Vec<Cell>
    {
        self.snake.get_cells()
    }

    pub(crate) fn get_background_color(&self) -> Color
    {
        self.background_color
    }

    pub(crate) fn go(&mut self, input: Option<Input>)
    {
        self.snake.go(input);
    }

}