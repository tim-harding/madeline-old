use std::collections::HashMap;
use std::collections::hash_map::Entry;
use crate::graph::image::Image;
use crate::graph::png_loader::image_from_png;
use std::path::PathBuf;

pub struct ReaderCache { 
    entries: HashMap<PathBuf, Image>,
}

impl ReaderCache {
    pub fn new() -> ReaderCache {
        ReaderCache {
            entries: HashMap::new(),
        }
    }

    pub fn get_or_read(&mut self, path: PathBuf) -> Option<&Image> {
        match self.entries.entry(path.clone()) {
            Entry::Occupied(entry) => Some(&*entry.into_mut()),
            Entry::Vacant(entry) => {
                match image_from_png(&path) {
                    Ok(image) => {
                        Some(&*entry.insert(image))
                    },
                    Err(_) => None,
                }
            },
        }
    }
}
