use crate::control::{self, Control};
use crate::image::Image;
use crate::plugin::*;
use crate::utils::io;
use std::path::PathBuf;

enum Parameters {}

pub fn create() -> Plugin {
    let desc = plugin::Desc::new("correct", &["bg"], &[]);
    Plugin::new(render, desc)
}

fn render(inputs: Inputs, controls: Controls) -> Result<Image, String> {
    let bg = match inputs[0] {
        Some(bg) => bg,
        None => return Err("Invalid background input".to_string()),
    };

    let mut out = bg.clone();

    let mut src_channels = bg.channels();
    let src_r = src_channels.next().ok_or("Source red channel not found.")?;
    let src_g = src_channels.next().ok_or("Source green channel not found.")?;
    let src_b = src_channels.next().ok_or("Source blue channel not found.")?;

    let mut dst_channels = out.channels_mut();
    let dst_r = dst_channels.next().ok_or("Target red channel not found.")?;
    let dst_g = dst_channels.next().ok_or("Target green channel not found.")?;
    let dst_b = dst_channels.next().ok_or("Target blue channel not found.")?;

    for (((((src_r, src_g), src_b), dst_r), dst_g), dst_b) in src_r
        .elements()
        .zip(src_g.elements())
        .zip(src_b.elements())
        .zip(dst_r.elements_mut())
        .zip(dst_g.elements_mut())
        .zip(dst_b.elements_mut())
    {
        let y = src_r * 0.2126 + src_g * 0.7152 + src_b * 0.0722;
        *dst_r = y;
        *dst_g = y;
        *dst_b = y;
    }

    Ok(out)
}
