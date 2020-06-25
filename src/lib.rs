//! A minimalist library for parsing command line arguments.
//!
//! ## Features
//!
//! * Long-form boolean flags with single-character shortcuts: `--flag`, `-f`.
//! * Long-form string-valued options with single-character shortcuts: `--option <arg>`, `-o <arg>`.
//! * Condensed short-form options: `-abc <arg> <arg>`.
//! * Automatic `--help` and `--version` flags.
//! * Support for multivalued options.
//! * Support for git-style command interfaces with arbitrarily-nested commands.
//!
//! ## Example
//!
//! ```
//! # use arguably::ArgParser;
//! let mut parser = ArgParser::new()
//!     .helptext("Usage: foobar...")
//!     .version("1.0")
//!     .flag("foo f")
//!     .option("bar b");
//!
//! if let Err(err) = parser.parse() {
//!     err.exit();
//! }
//!
//! if parser.found("foo").unwrap() {
//!     println!("Found --foo/-f flag.");
//! }
//!
//! if let Some(value) = parser.value("bar").unwrap() {
//!     println!("Found --bar/-b option with value: {}", value);
//! }
//!
//! for arg in parser.args() {
//!     println!("Arg: {}", arg);
//! }
//! ```

use std::collections::HashMap;
use std::fmt;
use std::error;


/// Error types returned by the library.
#[derive(Debug)]
pub enum Error {
    /// Returned when the parser detects an unregistered flag, option, or command name,
    /// either among the command line arguments or in an API call.
    BadName(String),

    /// Returned when the parser detects an option with a missing value.
    MissingValue(String),

    /// Returned when the parser detects a help command with a missing argument.
    MissingHelpArg,

    /// Returned when the command line arguments are not valid unicode strings.
    NotUnicode,
}


impl error::Error for Error {}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadName(msg) =>  write!(f, "Error: {}", msg),
            Error::MissingValue(msg) =>  write!(f, "Error: {}", msg),
            Error::MissingHelpArg => write!(f, "Error: missing argument for the help command"),
            Error::NotUnicode => write!(f, "Error: arguments are not valid unicode strings"),
        }
    }
}


impl Error {
    /// Prints an error message to `stderr` and exits with a non-zero status code.
    pub fn exit(self) -> ! {
        eprintln!("{}.", self);
        std::process::exit(1);
    }
}


/// An ArgParser instance can be intialized using the builder pattern.
///
/// ```
/// # use arguably::ArgParser;
/// let parser = ArgParser::new()
///     .helptext("Usage: appname...")
///     .version("1.0")
///     .flag("foo f")
///     .option("bar b");
/// ```
pub struct ArgParser {
    helptext: Option<String>,
    version: Option<String>,
    arguments: Vec<String>,
    options: Vec<Opt>,
    option_map: HashMap<String, usize>,
    flags: Vec<Flag>,
    flag_map: HashMap<String, usize>,
    commands: Vec<ArgParser>,
    command_map: HashMap<String, usize>,
    command_name: Option<String>,
    callback: Option<fn(&str, &ArgParser)>,
    auto_help_cmd: bool,
}


impl ArgParser {
    /// Creates a new ArgParser instance.
    pub fn new() -> ArgParser {
        ArgParser {
            helptext: None,
            version: None,
            arguments: Vec::new(),
            options: Vec::new(),
            option_map: HashMap::new(),
            flags: Vec::new(),
            flag_map: HashMap::new(),
            commands: Vec::new(),
            command_map: HashMap::new(),
            command_name: None,
            callback: None,
            auto_help_cmd: false,
        }
    }

    /// Sets the parser's helptext string. Supplying a helptext string activates support
    /// for an automatic `--help` flag, also a `-h` shortcut if not registered by another
    /// option.
    ///
    /// ```
    /// # use arguably::ArgParser;
    /// let parser = ArgParser::new()
    ///     .helptext("Usage: appname...");
    /// ```
    pub fn helptext<S>(mut self, text: S) -> Self where S: Into<String> {
        self.helptext = Some(text.into());
        self
    }

    /// Sets the parser's version string. Supplying a version string activates support
    /// for an automatic `--version` flag, also a `-v` shortcut if not registered by another
    /// option.
    ///
    /// ```
    /// # use arguably::ArgParser;
    /// let parser = ArgParser::new()
    ///     .version("1.0");
    /// ```
    pub fn version<S>(mut self, text: S) -> Self where S: Into<String> {
        self.version = Some(text.into());
        self
    }

    /// Registers a new option. The `name` parameter accepts an unlimited number of
    /// space-separated aliases and single-character shortcuts.
    ///
    /// ```
    /// # use arguably::ArgParser;
    /// let parser = ArgParser::new()
    ///     .option("name1 name2 n");
    /// ```
    pub fn option(mut self, name: &str) -> Self {
        self.options.push(Opt {
            values: Vec::new(),
        });
        let index = self.options.len() - 1;
        for alias in name.split_whitespace() {
            self.option_map.insert(alias.to_string(), index);
        }
        self
    }

    /// Registers a new flag. The `name` parameter accepts an unlimited number of
    /// space-separated aliases and single-character shortcuts.
    ///
    /// ```
    /// # use arguably::ArgParser;
    /// let parser = ArgParser::new()
    ///     .flag("name1 name2 n");
    /// ```
    pub fn flag(mut self, name: &str) -> Self {
        self.flags.push(Flag {
            count: 0,
        });
        let index = self.flags.len() - 1;
        for alias in name.split_whitespace() {
            self.flag_map.insert(alias.to_string(), index);
        }
        self
    }

    /// Registers a new command. The name parameter accepts an unlimited number of
    /// space-separated aliases. The command's helptext, flags, and options can be
    /// registered on the command's ArgParser instance.
    ///
    /// ```
    /// # use arguably::ArgParser;
    /// let mut parser = ArgParser::new()
    ///     .helptext("Usage: appname...")
    ///     .command("cmdname", ArgParser::new()
    ///         .helptext("Usage: appname cmdname...")
    ///         .flag("cmdflag")
    ///     );
    /// ```
    pub fn command(mut self, name: &str, cmd_parser: ArgParser) -> Self {
        self.commands.push(cmd_parser);
        let index = self.commands.len() - 1;
        for alias in name.split_whitespace() {
            self.command_map.insert(alias.to_string(), index);
        }
        self
    }

    /// Registers a callback function on a command parser. If the command is found the
    /// callback will be called and passed the command name and a reference to the
    /// command's parser instance.
    pub fn callback(mut self, f: fn(&str, &ArgParser)) -> Self {
        self.callback = Some(f);
        self
    }

    /// Turns on an automatic `help` command for printing subcommand helptext.
    pub fn enable_help_command(mut self) -> Self {
        self.auto_help_cmd = true;
        self
    }

    /// Returns the value of the named option. Returns `Error::BadName` if `name` is not a
    /// registered option name. Returns `None` if the option was not found.
    pub fn value(&self, name: &str) -> Result<Option<String>, Error> {
        if let Some(index) = self.option_map.get(name) {
            if let Some(value) = self.options[*index].values.last() {
                return Ok(Some(value.to_string()));
            }
            return Ok(None);
        }
        Err(Error::BadName(format!("'{}' is not a registered option name", name)))
    }

    /// Returns the named option's list of values. Returns `Error::BadName` if `name` is not a
    /// registered option name.
    pub fn values(&self, name: &str) -> Result<Vec<String>, Error> {
        if let Some(index) = self.option_map.get(name) {
            return Ok(self.options[*index].values.clone());
        }
        Err(Error::BadName(format!("'{}' is not a registered option name", name)))
    }

    /// Returns the number of times the named flag or option was found. Returns `Error::BadName`
    /// if `name` is not a registered flag or option name.
    pub fn count(&self, name: &str) -> Result<usize, Error> {
        if let Some(index) = self.flag_map.get(name) {
            return Ok(self.flags[*index].count);
        }
        if let Some(index) = self.option_map.get(name) {
            return Ok(self.options[*index].values.len());
        }
        Err(Error::BadName(format!("'{}' is not a registered flag or option name", name)))
    }

    /// Returns `true` if the named flag or option was found. Returns `Error::BadName` if `name`
    /// is not a registered flag or option name.
    pub fn found(&self, name: &str) -> Result<bool, Error> {
        match self.count(name) {
            Ok(count) => Ok(count > 0),
            Err(err) => Err(err),
        }
    }

    /// Returns `true` if one or more positional arguments have been found.
    pub fn has_args(&self) -> bool {
        self.arguments.len() > 0
    }

    /// Returns the number of positional arguments.
    pub fn num_args(&self) -> usize {
        self.arguments.len()
    }

    /// Returns the positional arguments.
    pub fn args(&self) -> Vec<String> {
        self.arguments.clone()
    }

    /// Returns `true` if a command was found.
    pub fn has_cmd(&self) -> bool {
        self.command_name.is_some()
    }

    /// If a command was found, returns the command's name.
    pub fn cmd_name(&self) -> Option<&str> {
        self.command_name.as_deref()
    }

    /// If a command was found, returns a reference to the command's ArgParser instance.
    pub fn cmd_parser(&self) -> Option<&ArgParser> {
        if let Some(name) = self.command_name.as_ref() {
            let index = self.command_map.get(name).unwrap();
            return Some(&self.commands[*index]);
        }
        None
    }

    /// Parse the program's command line arguments.
    ///
    /// ```
    /// # let mut parser = arguably::ArgParser::new();
    /// if let Err(err) = parser.parse() {
    ///     err.exit();
    /// }
    /// ```
    pub fn parse(&mut self) -> Result<(), Error> {
        let mut strings = Vec::<String>::new();
        for os_string in std::env::args_os().skip(1) {
            if let Ok(string) = os_string.into_string() {
                strings.push(string);
            } else {
                return Err(Error::NotUnicode);
            }
        }
        let mut stream = ArgStream::new(strings);
        self.parse_argstream(&mut stream)?;
        Ok(())
    }

    /// Parse a vector of strings. This function is intended for testing only.
    pub fn parse_args(&mut self, args: Vec<&str>) -> Result<(), Error> {
        let strings = args.iter().map(|s| s.to_string()).collect();
        let mut stream = ArgStream::new(strings);
        self.parse_argstream(&mut stream)?;
        Ok(())
    }

    fn parse_argstream(&mut self, argstream: &mut ArgStream) -> Result<(), Error> {
        let mut is_first_arg = true;

        while argstream.has_next() {
            let arg = argstream.next();

            if arg == "--" {
                while argstream.has_next() {
                    self.arguments.push(argstream.next());
                }
            }

            else if arg.starts_with("--") {
                if arg.contains("=") {
                    self.handle_equals_opt(&arg)?;
                } else {
                    self.handle_long_opt(&arg, argstream)?;
                }
            }

            else if arg.starts_with("-") {
                if arg == "-" || arg.chars().nth(1).unwrap().is_numeric() {
                    self.arguments.push(arg);
                } else if arg.contains("=") {
                    self.handle_equals_opt(&arg)?;
                } else {
                    self.handle_short_opt(&arg, argstream)?;
                }
            }

            else if is_first_arg && self.command_map.contains_key(&arg) {
                self.command_name = Some(arg.to_string());
                let index = self.command_map.get(&arg).unwrap();
                let cmd_parser = &mut self.commands[*index];
                cmd_parser.parse_argstream(argstream)?;
                if let Some(callback) = cmd_parser.callback {
                    callback(&arg, cmd_parser);
                }
            }

            else if is_first_arg && arg == "help" && self.auto_help_cmd {
                if argstream.has_next() {
                    let name = argstream.next();
                    if let Some(index) = self.command_map.get(&name) {
                        let cmd_parser = &mut self.commands[*index];
                        let helptext = cmd_parser.helptext.as_deref().unwrap_or("").trim();
                        println!("{}", helptext);
                        std::process::exit(0);
                    } else {
                        return Err(
                            Error::BadName(format!("'{}' is not a recognised command name", &name))
                        );
                    }
                } else {
                    return Err(Error::MissingHelpArg);
                }
            }

            else {
                self.arguments.push(arg);
            }

            is_first_arg = false;
        }

        Ok(())
    }

    fn handle_long_opt(&mut self, arg: &str, argstream: &mut ArgStream) -> Result<(), Error> {
        if let Some(index) = self.flag_map.get(&arg[2..]) {
            self.flags[*index].count += 1;
        } else if let Some(index) = self.option_map.get(&arg[2..]) {
            if argstream.has_next() {
                self.options[*index].values.push(argstream.next());
            } else {
                return Err(Error::MissingValue(format!("missing value for {}", arg)));
            }
        } else if arg == "--help" && self.helptext.is_some() {
            println!("{}", self.helptext.as_ref().unwrap().trim());
            std::process::exit(0);
        } else if arg == "--version" && self.version.is_some() {
            println!("{}", self.version.as_ref().unwrap().trim());
            std::process::exit(0);
        } else {
            return Err(Error::BadName(format!("{} is not a recognised flag or option name", arg)));
        }
        Ok(())
    }

    fn handle_short_opt(&mut self, arg: &str, argstream: &mut ArgStream) -> Result<(), Error> {
        for c in arg.chars().skip(1) {
            if let Some(index) = self.flag_map.get(&c.to_string()) {
                self.flags[*index].count += 1;
            } else if let Some(index) = self.option_map.get(&c.to_string()) {
                if argstream.has_next() {
                    self.options[*index].values.push(argstream.next());
                } else {
                    let msg = if arg.chars().count() > 2 {
                        format!("missing value for '{}' in {}", c, arg)
                    } else {
                        format!("missing value for {}", arg)
                    };
                    return Err(Error::MissingValue(msg));
                }
            } else if c == 'h' && self.helptext.is_some() {
                println!("{}", self.helptext.as_ref().unwrap().trim());
                std::process::exit(0);
            } else if c == 'v' && self.version.is_some() {
                println!("{}", self.version.as_ref().unwrap().trim());
                std::process::exit(0);
            } else {
                let msg = if arg.chars().count() > 2 {
                    format!("'{}' in {} is not a recognised flag or option name", c, arg)
                } else {
                    format!("{} is not a recognised flag or option name", arg)
                };
                return Err(Error::BadName(msg));
            }
        }
        Ok(())
    }

    fn handle_equals_opt(&mut self, arg: &str) -> Result<(), Error> {
        let splits: Vec<&str> = arg.splitn(2, '=').collect();
        let name = splits[0];
        let value = splits[1];

        if let Some(index) = self.option_map.get(name.trim_start_matches('-')) {
            if value == "" {
                return Err(Error::MissingValue(format!("missing value for {}", name)));
            } else {
                self.options[*index].values.push(value.to_string());
                return Ok(());
            }
        }
        return Err(Error::BadName(format!("{} is not a recognised option name", name)));
    }
}


struct ArgStream {
    args: Vec<String>,
    index: usize,
}


impl ArgStream {
    fn new(args: Vec<String>) -> ArgStream {
        ArgStream {
            args: args,
            index: 0,
        }
    }

    fn has_next(&self) -> bool {
        self.index < self.args.len()
    }

    fn next(&mut self) -> String {
        self.index += 1;
        self.args[self.index - 1].clone()
    }
}


struct Opt {
    values: Vec<String>,
}


struct Flag {
    count: usize,
}

