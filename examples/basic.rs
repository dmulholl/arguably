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

    println!("Option --bar/-b has value: {}", parser.value("bar"));

    for arg in parser.args {
        println!("Arg: {}", arg);
    }
}
