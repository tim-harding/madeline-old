use madeline::image::Image;
use madeline::control::Control;
use madeline::plugin::{builtin::{Multiply, Loader}, Plugin};
use madeline::utils::{io, test_images};
use std::path::Path;

fn main() {
    match render() {
        Ok(_) => { },
        Err(e) => println!("{:?}", e),
    }
}

fn render() -> Result<(), String> {
    let mut loader = Loader::new();
    let inputs: Vec<Option<&Image>> = vec![];
    let filename_control = Control::Text("data/kitty.png".to_string());
    let controls = vec![&filename_control];
    let kitty = loader.render(inputs.as_slice(), controls.as_slice())?;
    let checker = test_images::checker(kitty.desc().size);
    let mut multiply = Multiply::new();
    let inputs = [Some(&kitty), Some(&checker)];
    let controls: Vec<&Control> = vec![];
    multiply.render(&inputs, controls.as_slice()).and_then(|out| {
        io::save(Path::new("data/kitty_checker.png"), &out)
    })
}