use arguably::ArgParser;

#[test]
fn flag_empty_input() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_vec(vec![]);
    assert_eq!(parser.found("flag"), false);
    assert_eq!(parser.count("flag"), 0);
}

#[test]
fn flag_missing() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_vec(vec!["foo", "bar"]);
    assert_eq!(parser.found("flag"), false);
    assert_eq!(parser.count("flag"), 0);
}

#[test]
fn flag_present_long() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_vec(vec!["--flag"]);
    assert_eq!(parser.found("flag"), true);
    assert_eq!(parser.count("flag"), 1);
}

#[test]
fn flag_present_short() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_vec(vec!["-f"]);
    assert_eq!(parser.found("flag"), true);
    assert_eq!(parser.count("flag"), 1);
}

#[test]
fn flag_multiple_long() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_vec(vec!["--flag", "--flag", "--flag"]);
    assert_eq!(parser.found("flag"), true);
    assert_eq!(parser.count("flag"), 3);
}

#[test]
fn flag_multiple_short() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_vec(vec!["-f", "-f", "-f"]);
    assert_eq!(parser.found("flag"), true);
    assert_eq!(parser.count("flag"), 3);
}

#[test]
fn flag_multiple_mixed() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_vec(vec!["--flag", "-f", "-f"]);
    assert_eq!(parser.found("flag"), true);
    assert_eq!(parser.count("flag"), 3);
}

#[test]
fn flag_multiple_condensed() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_vec(vec!["-fff", "--flag"]);
    assert_eq!(parser.found("flag"), true);
    assert_eq!(parser.count("flag"), 4);
}

#[test]
fn option_empty_input() {
    let mut parser = ArgParser::new().option("opt o", "default");
    let _ = parser.parse_vec(vec![]);
    assert_eq!(parser.found("opt"), false);
    assert!(parser.value("opt") == "default");
}

#[test]
fn option_missing() {
    let mut parser = ArgParser::new().option("opt o", "default");
    let _ = parser.parse_vec(vec!["foo", "bar"]);
    assert_eq!(parser.found("opt"), false);
    assert!(parser.value("opt") == "default");
}

#[test]
fn option_present_long() {
    let mut parser = ArgParser::new().option("opt o", "default");
    let _ = parser.parse_vec(vec!["--opt", "foo"]);
    assert_eq!(parser.found("opt"), true);
    assert!(parser.value("opt") == "foo");
}

#[test]
fn option_present_short() {
    let mut parser = ArgParser::new().option("opt o", "default");
    let _ = parser.parse_vec(vec!["-o", "foo"]);
    assert_eq!(parser.found("opt"), true);
    assert!(parser.value("opt") == "foo");
}

#[test]
fn option_multiple_values() {
    let mut parser = ArgParser::new().option("opt o", "default");
    let _ = parser.parse_vec(vec!["-o", "foo", "--opt", "bar"]);
    assert_eq!(parser.found("opt"), true);
    assert_eq!(parser.count("opt"), 2);
    assert_eq!(parser.value("opt"), "bar");
    assert_eq!(parser.values("opt"), vec!["foo", "bar"]);
}

#[test]
fn command_missing() {
    let mut parser = ArgParser::new().command("cmd", ArgParser::new());
    let _ = parser.parse_vec(vec!["foo", "bar"]);
    assert!(parser.cmd_name.is_none());
}

#[test]
fn command_found() {
    let mut parser = ArgParser::new().command("cmd", ArgParser::new());
    let _ = parser.parse_vec(vec!["cmd", "foo", "bar"]);
    assert_eq!(parser.cmd_name.unwrap(), "cmd");
}

#[test]
fn arguments_empty_input() {
    let mut parser = ArgParser::new();
    let _ = parser.parse_vec(vec![]);
    assert_eq!(parser.args.len(), 0);
}

#[test]
fn arguments_found() {
    let mut parser = ArgParser::new();
    let _ = parser.parse_vec(vec!["foo", "bar"]);
    assert_eq!(parser.args, vec!["foo", "bar"]);
}

