extern crate arguably;
use arguably::ArgParser;

fn main() {
    let mut parser = ArgParser::new()
        .helptext("Usage: foobar...")
        .version("1.0")
        .option("bar b", "default")
        .flag("foo f");

    if let Err(err) = parser.parse() {
        err.exit();
    }

    if parser.found("foo") {
        println!("Flag --foo/-f found.");
    }

    if parser.found("bar") {
        println!("Option --bar/-b found with value: {}", parser.value("bar"));
    } else {
        println!("Option --bar/-b has default value: {}", parser.value("bar"));
    }

    for arg in parser.args {
        println!("Arg: {}", arg);
    }
}
