use crate::graph::image::Image;
use std::fs::File;
use std::path::Path;
use std::io;
use ::png::{Self, ColorType::*};

pub fn image_from_png(path: &Path) -> io::Result<Image> {
    let decoder = Decoder::new(File::open(path)?);
    let (info, mut reader) = decoder.read_info()?;
    let mut img_data = vec![0; info.buffer_size()];
    reader.next_frame(&mut img_data)?;

    let channel_count = match info.color_type {
        RGB => 3,
        RGBA => 4,
        Grayscale => 1,
        GrayscaleAlpha => 2,
        
        // TODO: Translate indexed colors
        Indexed => {
            let kind = io::ErrorKind::Other;
            let text = "Indexed PNG encoding not supported.";
            let error = io::Error::new(kind, text);
            return io::Result::Err(error);
        }
    };

    let w = info.width as usize;
    let h = info.height as usize;
    let mut image = Image::new(w, h, channel_count);
    for y in 0..h {
        for x in 0..w {
            let pixel_index = y * w + x;
            let offset = pixel_index * channel_count;
            for channel_index in 0..channel_count {
                let channel = image.channel(channel_index);
                channel.raw()[pixel_index] = img_data[offset + channel_index] as f32;
            }
        }
    }

    Ok(image)
}
