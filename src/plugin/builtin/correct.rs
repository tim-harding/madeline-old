use crate::image::Image;
use crate::plugin::*;
use crate::utils::Value;

enum Parameters {
    HueRotation,
    Saturation,
    Exposure,
    Gamma,
    InvertGamma,
}

const KR: f32 = 0.2126;
const KG: f32 = 0.7152;
const KB: f32 = 0.0722;

pub fn create() -> Plugin {
    let controls = [
        control::Desc::new("hue_rotation", Value::Real(0.0)),
        control::Desc::new("saturation", Value::Real(1.0)),
        control::Desc::new("exposure", Value::Real(0.0)),
        control::Desc::new("gamma", Value::Real(1.0)),
        control::Desc::new("invert_gamma", Value::Boolean(false)),
    ];
    let desc = plugin::Desc::new("correct", &["bg"], &controls);
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
    let src_g = src_channels
        .next()
        .ok_or("Source green channel not found.")?;
    let src_b = src_channels
        .next()
        .ok_or("Source blue channel not found.")?;

    let mut dst_channels = out.channels_mut();
    let dst_r = dst_channels.next().ok_or("Target red channel not found.")?;
    let dst_g = dst_channels
        .next()
        .ok_or("Target green channel not found.")?;
    let dst_b = dst_channels
        .next()
        .ok_or("Target blue channel not found.")?;

    let hue_rotation = controls[Parameters::HueRotation as usize].as_real();
    let saturation = controls[Parameters::Saturation as usize].as_real();
    let exposure = controls[Parameters::Exposure as usize].as_real();
    let gamma = controls[Parameters::Gamma as usize].as_real();
    let invert_gamma = controls[Parameters::InvertGamma as usize].as_bool();

    let gamma = if invert_gamma { gamma.recip() } else { gamma };
    let brightness = 2.0f32.powf(exposure);

    let cos = hue_rotation.cos();
    let sin = hue_rotation.sin();

    for (((((src_r, src_g), src_b), dst_r), dst_g), dst_b) in src_r
        .elements()
        .zip(src_g.elements())
        .zip(src_b.elements())
        .zip(dst_r.elements_mut())
        .zip(dst_g.elements_mut())
        .zip(dst_b.elements_mut())
    {
        let mut y = src_r * KR + src_g * KG + src_b * KB;
        let mut u = 0.5 * (src_b - y) / (1.0 - KB);
        let mut v = 0.5 * (src_r - y) / (1.0 - KR);

        y *= brightness;

        y = if roughly_one(gamma) {
            y
        } else if y < 1.0 {
            fast_pow(y, gamma)
        } else {
            y.powf(gamma)
        };

        u = u * cos - v * sin;
        v = u * sin + v * cos;

        u *= saturation;
        v *= saturation;

        *dst_r = y + (2.0 - 2.0 * KR) * v;
        *dst_g = y - (KB / KG * (2.0 - 2.0 * KB)) * u - (KR / KG * (2.0 - 2.0 * KR)) * v;
        *dst_b = y + (2.0 - 2.0 * KB) * u;
    }

    Ok(out)
}

fn roughly_one(x: f32) -> bool {
    (x - 1.0).abs() < std::f32::EPSILON
}

// Inaccurate for HDR
fn fast_pow(x: f32, n: f32) -> f32 {
    let int = n as i32;
    let less = x.powi(int);
    let more = x.powi(int + 1);
    let frac = n.fract();
    less * (1.0 - frac) + more * frac
}
