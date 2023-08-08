#[derive(Clone, Copy)]
pub(crate) enum Direction
{
    UP, DOWN, LEFT, RIGHT, STOP,
}
use Direction::{UP, DOWN, LEFT, RIGHT};

type Color = (u8, u8, u8);

#[derive(Clone)]
pub(crate) struct Cell
{
    left: i64,
    top: i64,
    size: i64,
    direction: Direction,
    screen_width: i64,
    screen_height: i64,
    color: Color,
}

impl Cell
{
    pub(crate) fn new
    (
        left: i64,
        top: i64,
        size: i64,
        direction: Direction,
        screen_width: i64,
        screen_height: i64,
        color: Color,
    )
        -> Cell
    {
        Cell
        {
            left,
            top,
            size,
            direction,
            screen_width,
            screen_height,
            color,
        }
    }

    pub(crate) fn step(&mut self)
    {
        match self.direction
        {
            UP =>
            {
                let mut new_top = self.top - self.size;
                if new_top < 0
                {
                    new_top = self.screen_height - self.size;
                }
                self.top = new_top;
            }
            DOWN =>
            {
                let mut new_top = self.top + self.size;
                let new_bottom = new_top + self.size;
                if new_bottom > self.screen_height
                {
                    new_top = 0;
                }
                self.top = new_top;
            }
            LEFT =>
            {
                let mut new_left = self.left - self.size;
                if new_left < 0
                {
                    new_left = self.screen_width - self.size;
                }
                self.left = new_left;
            }
            RIGHT =>
            {
                let mut new_left = self.left + self.size;
                let new_right = new_left + self.size;
                if new_right > self.screen_width
                {
                    new_left = 0;
                }
                self.left = new_left;
            }
            _ => {}
        }
    }

    pub(crate) fn set_direction(&mut self, direction: Direction)
    {
        self.direction = direction;
    }

    pub(crate) fn get_direction(&self) -> &Direction
    {
        &self.direction
    }

    pub(crate) fn get_top(&self) -> i64
    {
        self.top
    }

    pub(crate) fn get_bottom(&self) -> i64
    {
        self.top + self.size
    }

    pub(crate) fn get_left(&self) -> i64
    {
        self.left
    }

    pub(crate) fn get_right(&self) -> i64
    {
        self.left + self.size
    }

    pub(crate) fn get_color(&self) -> Color
    {
        self.color
    }

    pub(crate) fn overlap(&self, other: &Cell) -> bool
    {
        self.left == other.left && self.top == other.top
    }

    pub(crate) fn set_color(&mut self, new_color: Color)
    {
        self.color = new_color
    }

    /*
    pub(crate) fn get_size(&self) -> i64
    {
        self.size
    }
    */
}

