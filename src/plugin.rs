mod plugins;
pub use plugins::*;

mod desc;
pub use desc::*;

mod builtin;

use crate::control::Control;
use crate::image::{self, Channel};
use crate::plugin;
use crate::utils::Vec2I;
use std::fmt::{self, Debug};

pub type ControlsRef<'a> = &'a [&'a Control];
pub type ChannelsRef<'a> = &'a mut [&'a mut Channel];
pub type Size<'a> = &'a Vec2I;

type ImageDescFunc = fn(ControlsRef) -> image::Desc;
type RenderFunc = fn(ChannelsRef, ControlsRef, Size);
type PlugDesc = &'static plugin::Desc;

#[derive(Clone)]
pub struct Plugin {
    desc: PlugDesc,
    image_desc_func: ImageDescFunc,
    render_func: RenderFunc,
}

impl Debug for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.desc)
    }
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
        for desc in self.desc.controls() {
            out.push(Control::from(&desc.kind()));
        }
        out
    }

    pub fn image_desc<'a>(&'a self, controls: ControlsRef<'a>) -> image::Desc {
        (self.image_desc_func)(controls)
    }

    pub fn render<'a>(
        &'a self,
        channels: ChannelsRef<'a>,
        controls: ControlsRef<'a>,
        size: Size<'a>,
    ) {
        (self.render_func)(channels, controls, size)
    }

    pub fn plug_desc(&self) -> PlugDesc {
        self.desc
    }
}
