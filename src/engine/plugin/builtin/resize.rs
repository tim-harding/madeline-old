use crate::{
    control,
    image::{Channel, Image},
    plugin::{self, *},
    utils::{Value, Vec2U},
};
use rayon::prelude::*;
use std::cmp::{max, min};

enum Parameters {
    SizeX,
    SizeY,
}

pub fn create() -> Plugin {
    let controls = [
        control::Desc::new("width", Value::Integer(512)),
        control::Desc::new("height", Value::Integer(512)),
    ];
    let desc = plugin::Desc::new("resize", &["bg"], &controls);
    Plugin::new(render, desc)
}

fn render(inputs: Inputs, controls: Controls) -> Result<Image, String> {
    let bg = match inputs[0] {
        Some(bg) => Ok(bg),
        None => Err(String::from("Invalid background input")),
    }?;

    let sx = controls[Parameters::SizeX as usize].as_uint();
    let sy = controls[Parameters::SizeY as usize].as_uint();

    let h_buf = scale_axis(&bg, sx);
    let v_buf = scale_axis(&h_buf, sy);

    Ok(v_buf)
}

fn scale_axis(src: &Image, dim: usize) -> Image {
    if dim > src.desc().size.x {
        upscale_axis(src, dim)
    } else {
        downscale_axis(src, dim)
    }
}

fn upscale_axis(src: &Image, dim: usize) -> Image {
    let scale_factor = src.desc().size.x as f32 / dim as f32;
    let dst_size = Vec2U::new(src.desc().size.y, dim);
    src.par_channels()
        .map(|src| {
            let mut dst = Channel::black(dst_size);
            for x in 0..dst_size.x {
                for y in 0..dst_size.y {
                    // dst_y is bigger, downscale to src_y
                    let pivot = y as f32 * scale_factor;
                    let frac = pivot.fract();

                    let pivot = pivot as isize;
                    let srcn = max(0, pivot - 1) as usize;
                    let pivot = pivot as usize;
                    let clip = src.size().x - 1;
                    let src0 = min(clip, pivot);
                    let src1 = min(clip, pivot + 1);
                    let src2 = min(clip, pivot + 2);

                    // dst_x maps to src_y directly
                    let srcy = x * src.size().x;

                    let y0 = src[srcn + srcy];
                    let y1 = src[src0 + srcy];
                    let y2 = src[src1 + srcy];
                    let y3 = src[src2 + srcy];

                    let a0 = -0.5 * y0 + 1.5 * y1 - 1.5 * y2 + 0.5 * y3;
                    let a1 = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
                    let a2 = -0.5 * y0 + 0.5 * y2;
                    let a3 = y1;

                    let frac2 = frac * frac;
                    let out_index = y * dst_size.x + x;
                    dst[out_index] = a0 * frac2 * frac + a1 * frac2 + a2 * frac + a3;
                }
            }
            dst
        })
        .collect::<Image>()
}

fn downscale_axis(src: &Image, dim: usize) -> Image {
    let dst_size = Vec2U::new(src.desc().size.y, dim);
    let width = src.desc().size.x as f32 / dim as f32;
    src.par_channels()
        .map(|src| {
            // Starting with x, which is out-of-order. However, since
            // dst is flipped over y=x, this yields in-order access to
            // the src buffer.
            let mut dst = Channel::black(dst_size);
            for x in 0..dst_size.x {
                for y in 0..dst_size.y {
                    let center = y as f32 * width;
                    let lo = (center - width).floor() as isize;
                    let hi = (center + width).ceil() as isize;

                    let (px, filter) = (lo..hi).fold((0.0, 0.0), |acc, i| {
                        let rel = i as f32 - center;
                        let clipped = min(src.size().x - 1, max(0, i) as usize);
                        let sample_index = x * src.size().x + clipped;
                        let sample = src[sample_index];

                        let filter = filter(rel, width);
                        (acc.0 + sample * filter, acc.1 + filter)
                    });

                    let out_index = y * dst_size.x + x;
                    dst[out_index] = px / filter;
                }
            }
            dst
        })
        .collect::<Image>()
}

fn filter(src: f32, radius: f32) -> f32 {
    let sawtooth = 1.0 - src.abs() / radius;
    let x = if sawtooth < 0.0 {
        0.0
    } else if sawtooth > 1.0 {
        1.0
    } else {
        sawtooth
    };

    // Smoothstep
    let rcp = 1.0 - x;
    rcp * x * x + x * (1.0 - rcp * rcp)
}
