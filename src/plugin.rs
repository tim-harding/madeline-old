mod plugins;
pub use plugins::*;

mod desc;
pub use desc::*;

mod builtin;

use crate::plugin;
use crate::image::Image;

pub trait Plugin {
    fn render(&self) -> Image;
    fn desc(&self) -> &'static plugin::Desc;
}
