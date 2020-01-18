use madeline::{mdl, utils::io};

use std::path::Path;

fn main() -> Result<(), String> {
    let ast = std::fs::read_to_string("data/test_comp.mdl").map_err(|_| "".to_string())?;
    let content = mdl::parse(&ast)?;
    let mut engine = mdl::unpack(&content)?;
    let comp = engine.render()?;
    io::save(Path::new("data/merge.png"), comp)
}