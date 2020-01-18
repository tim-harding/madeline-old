use madeline::{mdl, utils::io};
use std::path::Path;
use std::env;

fn main() -> Result<(), String> {
    if env::args().count() != 2 {
        return Err("Usage: madeline {comp.mdl}".into());
    }
    let mdl = env::args().last().unwrap();
    let ast = std::fs::read_to_string(mdl).map_err(|_| "".to_string())?;
    let content = mdl::parse(&ast)?;
    let mut engine = mdl::unpack(&content)?;
    let comp = engine.render()?;
    io::save(Path::new("data/merge.png"), comp)
}
