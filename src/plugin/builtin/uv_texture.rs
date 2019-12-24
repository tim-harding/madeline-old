use crate::control::{self, Control};
use crate::image::{self, Image};
use crate::plugin::{self, Plugin};

enum Property {
    Width,
    Height,
}

const NAME: &'static str = "UV";
const INPUTS: [&'static str; 0] = [];
const CONTROLS: [control::Desc; 2] = [
    control::Desc::new("Width", control::Kind::Integer),
    control::Desc::new("Height", control::Kind::Integer),
];
const DESC: plugin::Desc = plugin::Desc::new(NAME, &INPUTS, &CONTROLS);

pub const PLUGIN: Plugin = Plugin::new(&DESC, image_desc, render);

fn image_desc(controls: &[Control]) -> image::Desc {
    let w = controls[Property::Width as usize].as_int();
    let h = controls[Property::Height as usize].as_int();
    image::Desc::new(w, h, 4)
}

fn render(_: &[Control], out: &mut Image) {
    // use image kind api
    /*
    let h = out.desc.height;
    let w = out.desc.width;
    let h_f = h as f32;
    let w_f = w as f32;
    for y in 0..h {
        for x in 0..w {
            let index = y * w + x;
            let u = (x as f32) / w_f;
            let v = (y as f32) / h_f;
            out.channels[0].pixels[index] = u;
            out.channels[1].pixels[index] = v;
            out.channels[2].pixels[index] = 0.0;
            out.channels[3].pixels[index] = 1.0;
        }
    }
    */
}
