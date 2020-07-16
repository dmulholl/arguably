extern crate arguably;
use arguably::ArgParser;

fn main() {
    let mut parser = ArgParser::new()
        .helptext("Usage: foobar...")
        .version("1.0")
        .command("boo", ArgParser::new()
            .helptext("Usage: foobar boo...")
            .callback(cmd_boo)
        );

    if let Err(err) = parser.parse() {
        err.exit();
    }
}

fn cmd_boo(_cmd_name: &str, _cmd_parser: &ArgParser) {
    println!("boo!");
}
