use crate::graph::plugin::Plugin;
use crate::image::{self, Image};
use crate::graph::plugin::{control, context::Context};

enum Property {
    Width,
    Height,
}

const CONTROLS: [control::Description; 2] = [
    control::Description::new("Width", control::Kind::Integer),
    control::Description::new("Height", control::Kind::Integer),
];

pub fn create() -> Plugin {
    Plugin {
        image_description,
        process,
        controls_desc: &CONTROLS,
    }
}

fn image_description(ctx: Context) -> image::Description {
    let w = ctx.get_prop_int(Property::Width as usize);
    let h = ctx.get_prop_int(Property::Height as usize);
    image::Description::new(w, h, 4)
}

fn process(_: Context, out: &mut Image) {
    let h = out.height;
    let w = out.width;
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
}
