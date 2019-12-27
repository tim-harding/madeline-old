use crate::image::Image;
use super::Plugin;
use std::collections::HashMap;

pub struct Loader {
    images: HashMap<PathBuf, Image>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
        }
    }
}