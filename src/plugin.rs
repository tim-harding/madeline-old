mod plugins;
pub use plugins::*;

mod desc;
pub use desc::*;

mod builtin;

use crate::plugin;
use crate::image::Image;

pub type Inputs<'a> = &'a [Option<&'a Image>];

pub trait Plugin {
    fn render(&self, inputs: Inputs) -> Result<Image, String>;
    fn desc(&self) -> &'static plugin::Desc;
}
