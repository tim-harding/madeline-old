use crate::control::Control;
use crate::image::{self, Image};
use crate::plugin;

type ImageDescFunc = fn(&[Control]) -> image::Desc;
type RenderFunc = fn(&[Control], &mut Image);
type PlugDesc = &'static plugin::Desc;

pub struct Plugin {
    pub desc: PlugDesc,
    image_desc_func: ImageDescFunc,
    render_func: RenderFunc,
}

impl Plugin {
    pub const fn new(
        desc: PlugDesc,
        image_desc_func: ImageDescFunc,
        render_func: RenderFunc,
    ) -> Self {
        Self {
            desc,
            image_desc_func,
            render_func,
        }
    }

    pub fn controls(&self) -> Vec<Control> {
        let mut out = Vec::new();
        for desc in self.desc.controls {
            out.push(Control::from(&desc.kind));
        }
        out
    }

    pub fn image_desc(&self, controls: &[Control]) -> image::Desc {
        (self.image_desc_func)(controls)
    }

    pub fn render(&self, controls: &[Control], image: &mut Image) {
        (self.render_func)(controls, image)
    }
}
