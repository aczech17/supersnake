use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub struct BMP
{
    width: usize,
    height: usize,
    pixels: Vec<(u8, u8, u8)>,
}

impl BMP
{
    pub fn new(filename: &String) -> Result<BMP, String>
    {
        let mut file = match File::open(filename)
        {
            Ok(f) => f,
            Err(e) =>
            {
                let err_msg = format!("Could not open the file {}.", filename);
                return Err(err_msg);
            }
        };

        let mut content = Vec::new();
        match file.read_to_end(&mut content)
        {
            Ok(_) => {},
            Err(e) =>
            {
                let err_msg = format!("Could not read the file. {}", e.to_string());
                return Err(err_msg);
            }
        }

        if content.len() == 0
        {
            return Err("Pusty plik".to_string());
        }


        let width = Self::bytes_to_u32(&content[18..22]);
        let height = Self::bytes_to_u32(&content[22..26]);
        let offset = Self::bytes_to_u32(&content[10..14]);
        let padding = width % 4;


        let mut pixels = Vec::new();
        let mut index = offset;
        for y in 0..height
        {
            for x in 0..width
            {
                let (r, g, b) =
                    (content[index as usize], content[index as usize + 1], content[index as usize + 2]);
                pixels.push((r, g, b));
                index += 3;
            }
            index += padding;
        }

        //println!("{} {} {} {}", width, height, offset, pixels.len());

        let bmp = BMP
        {
            width: width as usize,
            height: height as usize,
            pixels,
        };

        Ok(bmp)
    }

    pub fn get_width(&self) -> usize
    {
        self.width
    }

    pub fn get_height(&self) -> usize
    {
        self.height
    }

    pub fn get_pixels(&self) -> &Vec<(u8, u8, u8)>
    {
        &self.pixels
    }

    fn bytes_to_u32(bytes: &[u8]) -> u32
    {
        ((bytes[3] as u32) << 24) |
        ((bytes[2] as u32) << 16) |
        ((bytes[1] as u32) << 8)  |
        (bytes[0] as u32)
    }
}

#[cfg(test)]
mod test
{
    use crate::bmp::BMP;

    #[test]
    fn test_bytes_to_u32()
    {
        let bytes = vec![0xAB, 0xCD, 0xEF, 0x11];
        let desired_value = 0xABCDEF11;
        let value = BMP::bytes_to_u32(&bytes);
        assert_eq!(value, desired_value);
    }
}