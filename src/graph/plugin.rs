pub mod context;
pub use context::Context;

pub mod control;
pub use control::Control;

use crate::image;
use image::Image;

pub type PluginId = usize;

type ImageDescriptionFunc = fn(Context) -> image::Description;
type RenderFunc = fn(Context, &mut Image);

pub struct Plugin {
    image_description_func: ImageDescriptionFunc,
    render_func: RenderFunc,
    controls_desc: &'static [control::Description],
}

impl Plugin {
    pub fn controls(&self) -> Vec<Control> {
        let mut out = Vec::new();
        for desc in self.controls_desc {
            out.push(Control::from(&desc.kind));
        }
        out
    }

    pub fn image_description(&self, ctx: Context) -> image::Description {
        (self.image_description_func)(ctx)
    }

    pub fn render(&self, ctx: Context, image: &mut Image) {
        (self.render_func)(ctx, image)
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

    pub fn with_id(&self, id: PluginId) -> &Plugin {
        &self.plugins[id]
    }
}
