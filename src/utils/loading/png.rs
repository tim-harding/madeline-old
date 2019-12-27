use crate::image::{self, Image};
use std::io;
use std::fs::File;
use crate::utils::Vec2I;
use ::png::{OutputInfo, Decoder, ColorType::*};

pub fn load(file: &File) -> Result<Image, String> {
    let (img_data, info) = extract(file).map_err(|e| format!("{}", e))?;
    let channel_count = match info.color_type {
        Grayscale => 1,
        GrayscaleAlpha => 2,
        RGB => 3,
        RGBA => 4,
        
        // TODO: Translate indexed colors
        Indexed => {
            return Err("Indexed PNG encoding not supported.".to_string());
        }
    };

    let size = Vec2I::new(info.width as usize, info.height as usize);
    let desc = image::Desc::new(size, channel_count);
    let mut image = Image::from_desc(desc);
    for (y, line) in image.lines_mut().enumerate() {
        for (x, pixel) in line.enumerate() {
            let pixel_index = y * size.x + x;
            let offset = pixel_index * channel_count;
            for (i, channel) in pixel.iter_mut().enumerate() {
                *channel = img_data[offset + i] as f32;
            }
        }
    }

    Ok(image)
}

fn extract(file: &File) -> io::Result<(Vec<u8>, OutputInfo)> {
    let decoder = Decoder::new(file);
    let (info, mut reader) = decoder.read_info()?;
    let mut img_data = vec![0; info.buffer_size()];
    reader.next_frame(&mut img_data)?;
    Ok((img_data, info))
}