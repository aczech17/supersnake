use std::fs::File;
use std::io::Read;
use serde::{Deserialize, Serialize};
use crate::game::Color;

#[derive(Serialize, Deserialize)]
pub struct Config
{
    pub(crate) head_color: Color,
    pub(crate) snake_color: Color,
    pub(crate) background_color: Color,
    pub(crate) screen_width: i64,
    pub(crate) screen_height: i64,
    pub(crate) cell_size: i64,
    pub(crate) initial_cell_count: i64,
}

impl Config
{
    pub fn new(filename: &str) -> Result<Config, String>
    {
        let mut config_file = match File::open(filename)
        {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };
        let mut config_file_content = String::new();

        match config_file.read_to_string(&mut config_file_content)
        {
            Ok(_) => {},
            Err(_) => return Err("Could not parse json file".to_string()),
        }

        let config: Config = match serde_json::from_str(&config_file_content)
        {
            Ok(c) => c,
            Err(e) => return Err(e.to_string()),
        };

        Ok(config)
    }

}