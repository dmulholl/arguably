use std::collections::HashMap;


#[derive(Debug)]
pub enum Error {
    BadName(String),
    MissingValue(String),
}


impl Error {
    pub fn exit(&self) -> ! {
        match self {
            Error::BadName(msg) =>  {
                eprintln!("Error: {}.", msg);
                std::process::exit(1);
            },
            Error::MissingValue(msg) =>  {
                eprintln!("Error: {}.", msg);
                std::process::exit(1);
            },
        }
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


#[derive(Debug)]
struct Opt {
    values: Vec<String>,
}


#[derive(Debug)]
struct Flag {
    count: usize,
}


#[derive(Debug)]
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
}


impl ArgParser {
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
        }
    }

    // ---------
    // Builders.
    // ---------

    pub fn helptext(mut self, text: &str) -> Self {
        self.helptext = Some(text.trim().to_string());
        self
    }

    pub fn version(mut self, text: &str) -> Self {
        self.version = Some(text.trim().to_string());
        self
    }

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

    pub fn command(mut self, name: &str, cmd_parser: ArgParser) -> Self {
        self.commands.push(cmd_parser);
        let index = self.commands.len() - 1;
        for alias in name.split_whitespace() {
            self.command_map.insert(alias.to_string(), index);
        }
        self
    }

    // --------
    // Getters.
    // --------

    pub fn value(&self, name: &str) -> Result<Option<String>, Error> {
        if let Some(index) = self.option_map.get(name) {
            if let Some(value) = self.options[*index].values.last() {
                return Ok(Some(value.to_string()));
            }
            return Ok(None);
        }
        Err(Error::BadName(format!("'{}' is not a registered option name", name)))
    }

    pub fn values(&self, name: &str) -> Result<Vec<String>, Error> {
        if let Some(index) = self.option_map.get(name) {
            return Ok(self.options[*index].values.clone());
        }
        Err(Error::BadName(format!("'{}' is not a registered option name", name)))
    }

    pub fn count(&self, name: &str) -> Result<usize, Error> {
        if let Some(index) = self.flag_map.get(name) {
            return Ok(self.flags[*index].count);
        }
        if let Some(index) = self.option_map.get(name) {
            return Ok(self.options[*index].values.len());
        }
        Err(Error::BadName(format!("'{}' is not a registered name", name)))
    }

    pub fn found(&self, name: &str) -> Result<bool, Error> {
        match self.count(name) {
            Ok(count) => Ok(count > 0),
            Err(err) => Err(err),
        }
    }

    pub fn has_args(&self) -> bool {
        self.arguments.len() > 0
    }

    pub fn num_args(&self) -> usize {
        self.arguments.len()
    }

    pub fn args(&self) -> Vec<String> {
        self.arguments.clone()
    }

    pub fn has_cmd(&self) -> bool {
        self.command_name.is_some()
    }

    pub fn cmd_name(&self) -> Option<&str> {
        self.command_name.as_deref()
    }

    pub fn cmd_parser(&self) -> Option<&ArgParser> {
        if let Some(name) = self.command_name.as_ref() {
            let index = self.command_map.get(name).unwrap();
            return Some(&self.commands[*index]);
        }
        None
    }

    // ------------------
    // Parsing machinery.
    // ------------------

    pub fn parse(&mut self) -> Result<(), Error> {
        let strings = std::env::args().skip(1).collect();
        let mut stream = ArgStream::new(strings);
        self.parse_argstream(&mut stream)?;
        Ok(())
    }

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
                let index = self.command_map.get(&arg).unwrap();
                self.commands[*index].parse_argstream(argstream)?;
                self.command_name = Some(arg);
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
        }

        else if let Some(index) = self.option_map.get(&arg[2..]) {
            if argstream.has_next() {
                self.options[*index].values.push(argstream.next());
            } else {
                return Err(Error::MissingValue(format!("missing value for {}", arg)));
            }
        }

        else if arg == "--help" && self.helptext.is_some() {
            println!("{}", self.helptext.as_ref().unwrap());
            std::process::exit(0);
        }

        else if arg == "--version" && self.version.is_some() {
            println!("{}", self.version.as_ref().unwrap());
            std::process::exit(0);
        }

        else {
            return Err(Error::BadName(format!("{} is not a recognised option name", arg)));
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
                    return Err(
                        Error::MissingValue(format!("missing value for {}", arg))
                    );
                }
            } else if c == 'h' && self.helptext.is_some() {
                println!("{}", self.helptext.as_ref().unwrap());
                std::process::exit(0);
            } else if c == 'v' && self.version.is_some() {
                println!("{}", self.version.as_ref().unwrap());
                std::process::exit(0);
            } else {
                return Err(
                    Error::BadName(format!("'{}' in {} is not a recognised option name", c, arg))
                );
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

