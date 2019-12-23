pub mod context;
pub use context::Context;

pub mod control;
pub use control::Control;

use crate::image;
use image::Image;

pub type PluginId = usize;

type ImageDescriptionFunc = fn(Context) -> image::Description;
type ProcessFunc = fn(Context, &mut Image);

pub struct Plugin {
    pub image_description: ImageDescriptionFunc,
    pub process: ProcessFunc,
    pub controls_desc: &'static [control::Description],
}

impl Plugin {
    pub fn controls(&self) -> Vec<Control> {
        let mut out = Vec::new();
        for desc in self.controls_desc {
            out.push(Control::from(&desc.kind));
        }
        out
    }
}

pub struct Plugins {
    plugins: Vec<Plugin>,
}

impl Plugins {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add(&mut self, plugin: Plugin) {
        self.plugins.push(plugin);
    }
}
