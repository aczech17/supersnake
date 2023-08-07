use crate::game::cell::Cell;
use crate::game::snake::Snake;

mod cell;
mod snake;

pub(crate) type Color = (u8, u8, u8);
type Input = minifb::Key;
type Points = u64;

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
    points: Points,
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
    ) -> Result<Game, String>
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

        let conditions = screen_width % cell_size == 0 && screen_height % cell_size == 0;
        if !conditions
        {
            let error_message = format!
            (
                "Bad dimensions. \n\
                 screen_width: {}\n\
                screen_height: {}\n\
                cell_size: {}\n",
            screen_width, screen_height, cell_size)
                .to_string();

            return Err(error_message);
        }

        let game = Game
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
        };

        Ok(game)
    }

    pub(crate) fn get_snake_cells(&self) -> &Vec<Cell>
    {
        self.snake.get_cells()
    }

    pub(crate) fn get_background_color(&self) -> Color
    {
        self.background_color
    }

    pub(crate) fn go(&mut self, input: Option<Input>) -> Option<Points>
    {
        self.snake.go(input);

        match self.snake.is_tangled()
        {
            true => Some(self.points),
            false => None,
        }
    }

    pub(crate) fn get_resolution(&self) -> (i64, i64)
    {
        (self.screen_width, self.screen_height)
    }
}