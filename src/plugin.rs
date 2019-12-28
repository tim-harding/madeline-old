mod plugins;
pub use plugins::*;

mod desc;
pub use desc::*;

pub mod builtin;

use crate::image::Image;
use crate::plugin;

pub type Inputs<'a> = &'a [Option<&'a Image>];

pub trait Plugin {
    fn render(&self, inputs: Inputs) -> Result<Image, String>;
    fn desc(&self) -> &'static plugin::Desc;
}
