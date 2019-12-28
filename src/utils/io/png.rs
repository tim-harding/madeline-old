use crate::image::{self, Image};
use crate::utils::Vec2I;
use ::png::{BitDepth, ColorType, Decoder, OutputInfo};
use std::fs::File;
use std::io::{self, BufWriter};

pub fn load(file: &File) -> Result<Image, String> {
    let (img_data, info) = extract(file).map_err(|e| format!("{}", e))?;
    let channel_count = match info.color_type {
        ColorType::Grayscale => 1,
        ColorType::GrayscaleAlpha => 2,
        ColorType::RGB => 3,
        ColorType::RGBA => 4,
        ColorType::Indexed => {
            // TODO: Translate indexed colors
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

pub fn save(file: &File, image: &Image) -> Result<(), String> {
    let size = image.desc().size;
    let x = size.x as u32;
    let y = size.y as u32;

    let ref mut writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, x, y);
    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|e| format!("{}", e))?;

    let data = image
        .pixels()
        .flat_map(|px| px.iter())
        .map(|c| (*c * 255.0) as u8)
        .collect::<Vec<u8>>();
    writer
        .write_image_data(data.as_slice())
        .map_err(|e| format!("{}", e))
}
