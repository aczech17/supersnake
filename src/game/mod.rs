use crate::game::cell::Cell;
use crate::game::snake::Snake;

mod cell;
mod snake;

use rand::Rng;
use crate::game::cell::Direction::STOP;

pub(crate) type Color = (u8, u8, u8);
type Input = minifb::Key;
type Points = u64;

pub struct Game
{
    screen_width: i64,
    screen_height: i64,
    cell_size: i64,
    //initial_cell_count: i64,
    //head_color: Color,
    snake_color: Color,
    background_color: Color,

    snake: Snake,
    point_cell: Cell,
    points: Points,
}

impl Game
{
    pub(crate) fn new
    (
        screen_width: usize,
        screen_height: usize,
        cell_size: i64,
        initial_cell_count: i64,
        head_color: Color,
        snake_color: Color,
        background_color: Color,
    ) -> Result<Game, String>
    {
        let screen_width = screen_width as i64;
        let screen_height = screen_height as i64;
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

        let point_cell = Self::make_random_cell(snake.get_cells(), screen_width,
                                                screen_height, cell_size, snake_color);

        let game = Game
        {
            screen_width,
            screen_height,
            cell_size,
            //initial_cell_count,
            //head_color,
            snake_color,
            background_color,
            snake,
            point_cell,
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

    /*
    pub(crate) fn get_resolution(&self) -> (i64, i64)
    {
        (self.screen_width, self.screen_height)
    }
    */


    fn make_random_cell(snake_cells: &Vec<Cell>, screen_width: i64,
                        screen_height: i64, cell_size: i64, cell_color: Color) -> Cell
    {
        let max_iteration_count = 10;
        let mut rng = rand::thread_rng();

        for _ in 0..max_iteration_count
        {
            let r = rng.gen_range(0..screen_width / cell_size);
            let left = r * cell_size;

            let r = rng.gen_range(0..screen_height / cell_size);
            let top = r * cell_size;

            let new_cell = Cell::new(left, top, cell_size, STOP, screen_width,
                                 screen_height, cell_color);

            for cell in snake_cells
            {
                if cell.overlap(&new_cell)
                    { continue }

                return new_cell;
            }
        }

        panic!("Max iteration count ({}) reached.", max_iteration_count);
    }

    pub(crate) fn get_point_cell(&self) -> &Cell
    {
        &self.point_cell
    }

    pub fn get_points(&self) -> Points
    {
        self.points
    }

    pub(crate) fn go(&mut self, input: Option<Input>) -> bool
    {

        self.snake.go(input);

        if self.snake.is_collecting_point(&self.point_cell)
        {
            self.snake.change_head(self.point_cell.clone());

            self.points += 1;
            println!("{}", self.points);

            self.point_cell = Self::make_random_cell(self.snake.get_cells(), self.screen_width,
                                                     self.screen_height, self.cell_size, self.snake_color);
        }

        if self.snake.is_tangled()
        {
            return false;
        }
        true
    }
}