extern crate arguably;
use arguably::ArgParser;

fn main() {
    let mut parser = ArgParser::new()
        .helptext("Usage: foobar...")
        .version("1.0")
        .flag("foo f")
        .option("bar b");

    if let Err(err) = parser.parse() {
        err.exit();
    }

    if parser.found("foo") {
        println!("Found --foo/-f flag.");
    }

    if let Some(value) = parser.value("bar") {
        println!("Found --bar/-b option with value: {}", value);
    }

    for arg in parser.args {
        println!("Arg: {}", arg);
    }
}
