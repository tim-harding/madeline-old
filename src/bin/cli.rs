use madeline::control::Control;
use madeline::image::Image;
use madeline::plugin::{
    builtin::{Loader, Merge},
    Plugin,
};
use madeline::utils::io;
use std::path::Path;

fn main() {
    match render() {
        Ok(_) => {}
        Err(e) => println!("{:?}", e),
    }
}

fn render() -> Result<(), String> {
    let mut loader = Loader::new();
    let mut merge = Merge::new();

    let kitty = {
        let inputs: Vec<Option<&Image>> = vec![];
        let filename_control = Control::Text("data/kitty.png".to_string());
        let controls = vec![&filename_control];
        loader.render(inputs.as_slice(), controls.as_slice())
    }?;

    let tree = {
        let inputs: Vec<Option<&Image>> = vec![];
        let filename_control = Control::Text("data/tree.png".to_string());
        let controls = vec![&filename_control];
        loader.render(inputs.as_slice(), controls.as_slice())
    }?;

    let comp = {
        let inputs: Vec<Option<&Image>> = vec![Some(&tree), Some(&kitty)];
        let tx_control = Control::Vec2(Default::default());
        let controls = vec![&tx_control];
        merge.render(inputs.as_slice(), controls.as_slice())
    }?;

    io::save(Path::new("data/merge.png"), &comp)
}
