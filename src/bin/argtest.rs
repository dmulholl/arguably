extern crate arguably;


fn main() {
    let mut parser = arguably::ArgParser::new();
    parser.helptext("help!");
    parser.version("v1.0");
    parser.option("file f");
    parser.flag("quiet q");

    if let Err(err) = parser.parse() {
        println!("{:?}", err);
    } else {
        println!("{:#?}", parser);
    }



    // let foo = vec!["foo", "bar", "baz"];
    // parser.parse_slices(foo);
}

