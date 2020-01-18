use crate::image::{self, Image};
use crate::utils::{Vec2I, Vec2U};
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

    let size = Vec2U::new(info.width as usize, info.height as usize);
    let desc = image::Desc::new(size, channel_count);
    let mut image = Image::from_desc(desc);
    for y in 0..size.y {
        for x in 0..size.x {
            let start = (y * size.x + x) * channel_count;
            let end = start + channel_count;
            let pixel = &img_data[start..end];
            for (i, channel) in pixel.iter().enumerate() {
                let value = *channel as f32 / 255.0;
                let pos: Vec2I = Vec2U::new(x, y).into();
                image[i].set_element(pos, value);
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

    let writer = &mut BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, x, y);
    encoder.set_color(ColorType::RGBA);
    encoder.set_depth(BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|e| format!("{}", e))?;

    let element_count = size.area() * image.channel_count();
    let mut data = vec![0; element_count];
    for (channel_i, channel) in image.channels().enumerate() {
        for (element_i, element) in channel.elements().enumerate() {
            let i = element_i * image.channel_count() + channel_i;
            data[i] = (element.clamp(0.0, 1.0) * 255.0) as u8;
        }
    }

    writer
        .write_image_data(data.as_slice())
        .map_err(|e| format!("{}", e))
}
