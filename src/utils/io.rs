use crate::image::Image;
use std::{
    fs::File,
    path::Path,
};

mod png;

const EXT_ERR: &str = "File extension not recognized";

pub fn load(path: &Path) -> Result<Image, String> {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some(ext) => {
                let file = File::open(path).map_err(|e| format!("{}", e))?;
                match ext {
                    "png" => png::load(&file),
                    _ => Err(EXT_ERR.to_string()),
                }
            }
            None => Err(EXT_ERR.to_string()),
        },
        None => Err(EXT_ERR.to_string()),
    }
}

pub fn save(path: &Path, image: &Image) -> Result<(), String> {
    match path.extension() {
        Some(ext) => match ext.to_str() {
            Some(ext) => {
                let file = File::create(path).map_err(|e| format!("{}", e))?;
                match ext {
                    "png" => png::save(&file, image),
                    _ => Err(EXT_ERR.to_string()),
                }
            }
            None => Err(EXT_ERR.to_string()),
        },
        None => Err(EXT_ERR.to_string()),
    }
}
