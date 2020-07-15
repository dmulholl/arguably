extern crate arguably;
use arguably::ArgParser;

#[test]
fn flag_empty_input() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_args(vec![]);
    assert_eq!(parser.found("flag").unwrap(), false);
    assert_eq!(parser.count("flag").unwrap(), 0);
}

#[test]
fn flag_missing() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_args(vec!["foo", "bar"]);
    assert_eq!(parser.found("flag").unwrap(), false);
    assert_eq!(parser.count("flag").unwrap(), 0);
}

#[test]
fn flag_present_long() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_args(vec!["--flag"]);
    assert_eq!(parser.found("flag").unwrap(), true);
    assert_eq!(parser.count("flag").unwrap(), 1);
}

#[test]
fn flag_present_short() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_args(vec!["-f"]);
    assert_eq!(parser.found("flag").unwrap(), true);
    assert_eq!(parser.count("flag").unwrap(), 1);
}

#[test]
fn flag_multiple_long() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_args(vec!["--flag", "--flag", "--flag"]);
    assert_eq!(parser.found("flag").unwrap(), true);
    assert_eq!(parser.count("flag").unwrap(), 3);
}

#[test]
fn flag_multiple_short() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_args(vec!["-f", "-f", "-f"]);
    assert_eq!(parser.found("flag").unwrap(), true);
    assert_eq!(parser.count("flag").unwrap(), 3);
}

#[test]
fn flag_multiple_mixed() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_args(vec!["--flag", "-f", "-f"]);
    assert_eq!(parser.found("flag").unwrap(), true);
    assert_eq!(parser.count("flag").unwrap(), 3);
}

#[test]
fn flag_multiple_condensed() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_args(vec!["-fff", "--flag"]);
    assert_eq!(parser.found("flag").unwrap(), true);
    assert_eq!(parser.count("flag").unwrap(), 4);
}

#[test]
fn flag_value() {
    let mut parser = ArgParser::new().flag("flag f");
    let _ = parser.parse_args(vec!["-f"]);
    assert!(parser.value("flag").is_err());
}

#[test]
fn option_empty_input() {
    let mut parser = ArgParser::new().option("opt o");
    let _ = parser.parse_args(vec![]);
    assert_eq!(parser.found("opt").unwrap(), false);
    assert!(parser.value("opt").unwrap().is_none());
}

#[test]
fn option_missing() {
    let mut parser = ArgParser::new().option("opt o");
    let _ = parser.parse_args(vec!["foo", "bar"]);
    assert_eq!(parser.found("opt").unwrap(), false);
    assert!(parser.value("opt").unwrap().is_none());
}

#[test]
fn option_present_long() {
    let mut parser = ArgParser::new().option("opt o");
    let _ = parser.parse_args(vec!["--opt", "foo"]);
    assert_eq!(parser.found("opt").unwrap(), true);
    assert_eq!(parser.value("opt").unwrap().unwrap(), "foo");
}

#[test]
fn option_present_short() {
    let mut parser = ArgParser::new().option("opt o");
    let _ = parser.parse_args(vec!["-o", "foo"]);
    assert_eq!(parser.found("opt").unwrap(), true);
    assert_eq!(parser.value("opt").unwrap().unwrap(), "foo");
}

#[test]
fn option_multiple_values() {
    let mut parser = ArgParser::new().option("opt o");
    let _ = parser.parse_args(vec!["-o", "foo", "--opt", "bar"]);
    assert_eq!(parser.found("opt").unwrap(), true);
    assert_eq!(parser.count("opt").unwrap(), 2);
    assert_eq!(parser.value("opt").unwrap().unwrap(), "bar");
    assert_eq!(parser.values("opt").unwrap(), vec!["foo", "bar"]);
}

#[test]
fn command_missing() {
    let mut parser = ArgParser::new().command("cmd", ArgParser::new());
    let _ = parser.parse_args(vec!["foo", "bar"]);
    assert!(parser.cmd_name.is_none());
}

#[test]
fn command_found() {
    let mut parser = ArgParser::new().command("cmd", ArgParser::new());
    let _ = parser.parse_args(vec!["cmd", "foo", "bar"]);
    assert_eq!(parser.cmd_name.unwrap(), "cmd");
}

#[test]
fn arguments_empty_input() {
    let mut parser = ArgParser::new();
    let _ = parser.parse_args(vec![]);
    assert_eq!(parser.args.len(), 0);
}

#[test]
fn arguments_found() {
    let mut parser = ArgParser::new();
    let _ = parser.parse_args(vec!["foo", "bar"]);
    assert_eq!(parser.args, vec!["foo", "bar"]);
}

