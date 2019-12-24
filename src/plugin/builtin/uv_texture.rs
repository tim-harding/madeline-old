use crate::control::{self, Control};
use crate::image::{self, Channel};
use crate::plugin::{self, *};
use crate::utils::Vector2Int;

enum Property {
    Width,
    Height,
}

const NAME: &'static str = "UV";
const INPUTS: [Input; 0] = [];
const CONTROLS: [control::Desc; 2] = [
    control::Desc::new("Width", control::Kind::Integer),
    control::Desc::new("Height", control::Kind::Integer),
];
const DESC: plugin::Desc = plugin::Desc::new(NAME, &INPUTS, &CONTROLS);

pub const PLUGIN: Plugin = Plugin::new(&DESC, image_desc, render);

fn image_desc(controls: ControlsRef) -> image::Desc {
    let w = controls[Property::Width as usize].as_int();
    let h = controls[Property::Height as usize].as_int();
    let size = Vector2Int::new(w, h);
    image::Desc::new(size, image::Format::Rg)
}

fn render(channels: ChannelsRef, _: ControlsRef, size: Size) {
    let h_f = size.y as f32;
    let w_f = size.x as f32;
    for y in 0..size.y {
        for x in 0..size.x {
            let index = y * size.x + x;
            channels[0].set(index, (x as f32) / w_f);
            channels[1].set(index, (y as f32) / h_f);
        }
    }
}
