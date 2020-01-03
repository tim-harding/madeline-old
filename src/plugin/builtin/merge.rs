use crate::control;
use crate::image::Image;
use crate::plugin::{self, *};
use crate::utils::{Vec2I, Vec2U};
use std::iter::repeat;

enum Parameters {
    Translate,
}

const NAME: &'static str = "merge";
const INPUTS: [&'static str; 2] = ["bg", "fg"];
const CONTROLS: [control::Desc; 1] = [control::Desc::new("translate", control::Kind::Vec2)];
const DESC: plugin::Desc = plugin::Desc::new(NAME, &INPUTS, &CONTROLS);

pub struct Merge {}

impl Merge {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for Merge {
    fn render(&mut self, inputs: Inputs, controls: Controls) -> Result<Image, String> {
        let bg = match inputs[0] {
            Some(bg) => bg,
            None => return Err(String::from("Invalid background input")),
        };

        let mut out = bg.clone();

        let fg = match inputs[1] {
            Some(fg) => fg,
            // Pass through background input
            None => return Ok(out),
        };

        let tx = controls[Parameters::Translate as usize].as_vec();

        if fg.channel_count() == 4 {
            // Alpha blended
            for ((fg_chan, out_chan), alpha_chan) in fg
                .channels()
                .take(3)
                .zip(out.channels_mut())
                .zip(repeat(&fg[3]))
            {
                for (y, (fg_line, alpha_line)) in
                    fg_chan.lines().zip(alpha_chan.lines()).enumerate()
                {
                    for (x, (fg_e, alpha_e)) in fg_line.zip(alpha_line).enumerate() {
                        let pos = tx + Vec2U::new(x, y).into();
                        let bg_e = out_chan.element(pos);
                        let value = *fg_e * alpha_e + bg_e * (1.0 - *alpha_e);
                        out_chan.set_element(pos, value);
                    }
                }
            }
            for (y, fg_line) in fg[3].lines().enumerate() {
                for (x, fg_e) in fg_line.enumerate() {
                    let pos = tx + Vec2U::new(x, y).into();
                    let bg_e = out[3].element(pos);
                    let value = 1.0 - (1.0 - fg_e) * (1.0 - bg_e);
                    out[3].set_element(pos, value);
                }
            }
        } else {
            // Straight copy
            for (fg_c, out_c) in fg.channels().zip(out.channels_mut()) {
                for (y, fg_line) in fg_c.lines().enumerate() {
                    for (x, fg_e) in fg_line.enumerate() {
                        let pos = tx + Vec2U::new(x, y).into();
                        out_c.set_element(pos, *fg_e);
                    }
                }
            }
        }

        Ok(out)
    }

    fn desc(&self) -> &'static plugin::Desc {
        &DESC
    }
}
