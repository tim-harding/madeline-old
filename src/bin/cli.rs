use engine::{engine::Engine, mdl, utils::io};
use std::path::Path;

fn main() -> Result<(), String> {
    let matches = ::clap::App::new("Madeline")
        .version("0.1")
        .author("Tim Harding <tim@timharding.co>")
        .about("Node-based image compositor")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .arg(
            ::clap::Arg::with_name("comp_file")
                .short("c")
                .long("comp")
                .value_name("FILE")
                .takes_value(true)
                .help("Sets the .mdl file to render"),
        )
        .arg(
            ::clap::Arg::with_name("output")
                .short("o")
                .long("out")
                .value_name("FILE")
                .takes_value(true)
                .help("Sets the output image file"),
        )
        .arg(
            ::clap::Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("Starts a Madeline console session"),
        )
        .get_matches();

    let parser = mdl::Parser::default();
    let mut engine = Engine::new();
    match matches.value_of("comp_file") {
        Some(comp) => {
            let src = std::fs::read_to_string(comp)
                .map_err(|_| "Could not load comp file".to_string())?;
            for line in src.lines() {
                match parser.parse(line) {
                    Ok(statement) => mdl::apply(&mut engine, &statement)?,
                    Err(e) => print!("{}", e),
                }
            }
            let comp = engine.render()?;
            let out = matches.value_of("output").unwrap_or("data/merge.png");
            io::save(Path::new(out), comp)
        }
        None => {
            let mut line = String::new();
            while let Ok(_) = std::io::stdin().read_line(&mut line) {
                match parser.parse(&line) {
                    Ok(statement) => {
                        if let Err(e) = mdl::apply(&mut engine, &statement) {
                            println!("{}", e);
                        }
                    }
                    Err(e) => print!("{}", e),
                };
                println!();
                line.clear();
            }
            Ok(())
        }
    }
}
