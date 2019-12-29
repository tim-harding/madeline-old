use crate::control;
use crate::image::{self, Image};
use crate::plugin::{self, *};
use crate::utils::{Vec2I, Vec2U};
use std::cmp::min;

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

        let fg = match inputs[1] {
            Some(fg) => fg,
            // Pass through background input
            None => return Ok(bg.clone()),
        };

        let tx = controls[Parameters::Translate as usize].as_vec();
        let bg_sz: Vec2I = bg.desc().size.into();
        let fg_sz: Vec2I = fg.desc().size.into();

        let lo = Vec2I::min(tx, Default::default());
        let hi = Vec2I::max(bg_sz, fg_sz + tx);

        let size: Vec2U = (hi - lo).into();
        let desc = image::Desc::new(size, 4);
        let mut out = Image::from_desc(desc);

        for y in 0..size.y {
            for x in 0..size.x {
                let pos = Vec2U::new(x, y);
                let pos_i: Vec2I = pos.into();

                let bg_pos = pos_i - lo;
                let fg_pos = pos_i + tx;
            }
        }

        Ok(out)
    }

    fn desc(&self) -> &'static plugin::Desc {
        &DESC
    }
}
