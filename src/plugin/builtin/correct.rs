use crate::image::Image;
use crate::plugin::*;

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
        let kr = 0.2126;
        let kg = 0.7152;
        let kb = 0.0722;

        let y = src_r * kr + src_g * kg + src_b * kb;
        let u = 0.5 * (src_b - y) / (1.0 - kb);
        let v = 0.5 * (src_r - y) / (1.0 - kr);

        let r = y + (2.0 - 2.0 * kr) * v;
        let g = y - (kb / kg * (2.0 - 2.0 * kb)) * u - (kr / kg * (2.0 - 2.0 * kr)) * v; 
        let b = y + (2.0 - 2.0 * kb) * u;

        *dst_r = r;
        *dst_g = g;
        *dst_b = b;
    }

    Ok(out)
}
