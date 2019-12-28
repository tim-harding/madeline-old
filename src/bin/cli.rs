use madeline::utils::{test_images, io};
use madeline::plugin::{Plugin, builtin::Multiply};
use std::path::Path;

fn main() {
    let red = test_images::solid_color(256, 0.8, 0.2, 0.2);
    let checker = test_images::checker(256);
    let multiply = Multiply::new();
    let inputs = [Some(&red), Some(&checker)];
    match multiply.render(&inputs) {
        Ok(out) => {
            let path = Path::new("data/multiplied.png");
            match io::save(path, &out) {
                Ok(_) => println!("Saved successfully"),
                Err(e) => println!("{:?}", e),
            }
        },
        Err(e) => println!("{:?}", e),
    }
}