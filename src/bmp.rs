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
    pub fn new(filename: String) -> Option<BMP>
    {
        let mut file = File::open(filename).ok()?;

        let mut header = Vec::with_capacity(14);
        file.read_exact(&mut header).ok()?;

        let width = Self::bytes_to_u32(&header[18..22]);
        let height = Self::bytes_to_u32(&header[22..26]);
        let offset = Self::bytes_to_u32(&header[10..14]);

        file.seek(SeekFrom::Start(offset as u64)).ok()?;


        let mut pixel_bytes = Vec::new();
        file.read(&mut pixel_bytes).ok()?;

        let mut pixels = Vec::new();
        for i in (0..pixel_bytes.len()).step_by(3)
        {
            let (r, g, b) = (pixel_bytes[i], pixel_bytes[i+1], pixel_bytes[i+2]);
            pixels.push((r, g, b));
        }

        let bmp = BMP
        {
            width: width as usize,
            height: height as usize,
            pixels,
        };

        Some(bmp)
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
        ((bytes[0] as u32) << 24) |
        ((bytes[1] as u32) << 16) |
        ((bytes[2] as u32) << 8)  |
        (bytes[3] as u32)
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