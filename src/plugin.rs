mod plugins;
pub use plugins::*;

mod desc;
pub use desc::*;

use crate::control::Control;

pub mod builtin;

use crate::image::Image;
use crate::plugin;

pub type Inputs<'a> = &'a [Option<&'a Image>];
pub type Controls<'a> = &'a [&'a Control];

pub trait Plugin {
    fn render(&mut self, inputs: Inputs, controls: Controls) -> Result<Image, String>;
    fn desc(&self) -> &'static plugin::Desc;
}
