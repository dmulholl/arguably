extern crate arguably;
use arguably::ArgParser;

fn main() {
    let mut parser = ArgParser::new()
        .helptext("help!")
        .version("v1.0")
        .option("file f")
        .flag("quiet q");

    if let Err(err) = parser.parse() {
        err.exit();
    }

    println!(
        "quiet: {} ({})",
        parser.found("quiet").unwrap(),
        parser.count("quiet").unwrap()
    );

    if let Some(filename) = parser.value("file").unwrap() {
        println!("file: {}", filename);
    }

    for arg in parser.args() {
        println!("arg: {}", arg);
    }
}
