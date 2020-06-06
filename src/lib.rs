use std::collections::HashMap;




#[derive(Debug)]
pub enum Error {
    BadName(String),
    MissingValue(String),
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
    options: Vec<Opt>,
    option_map: HashMap<String, usize>,
    flags: Vec<Flag>,
    flag_map: HashMap<String, usize>,
    arguments: Vec<String>,
}


impl ArgParser {

    pub fn new() -> ArgParser {
        ArgParser {
            helptext: None,
            version: None,
            options: Vec::new(),
            option_map: HashMap::new(),
            flags: Vec::new(),
            flag_map: HashMap::new(),
            arguments: Vec::new(),
        }
    }

    // --------
    // Setters.
    // --------

    pub fn helptext(&mut self, text: &str) {
        self.helptext = Some(text.to_string());
    }

    pub fn version(&mut self, text: &str) {
        self.version = Some(text.to_string());
    }

    pub fn option(&mut self, name: &str) {
        self.options.push(Opt {
            values: Vec::new(),
        });
        let index = self.options.len() - 1;
        for alias in name.split_whitespace() {
            self.option_map.insert(alias.to_string(), index);
        }
    }

    pub fn flag(&mut self, name: &str) {
        self.flags.push(Flag {
            count: 0,
        });
        let index = self.flags.len() - 1;
        for alias in name.split_whitespace() {
            self.flag_map.insert(alias.to_string(), index);
        }
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
        Err(Error::BadName(name.to_string()))
    }

    pub fn values(&self, name: &str) -> Result<Vec<String>, Error> {
        if let Some(index) = self.option_map.get(name) {
            return Ok(self.options[*index].values.clone());
        }
        Err(Error::BadName(name.to_string()))
    }

    pub fn count(&self, name: &str) -> Result<usize, Error> {
        if let Some(index) = self.flag_map.get(name) {
            return Ok(self.flags[*index].count);
        }
        if let Some(index) = self.option_map.get(name) {
            return Ok(self.options[*index].values.len());
        }
        Err(Error::BadName(name.to_string()))
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
        // let mut is_first_arg = true;

        while argstream.has_next() {
            let arg = argstream.next();

            if arg == "--" {
                while argstream.has_next() {
                    self.arguments.push(argstream.next());
                }
            }

            else if arg.starts_with("--") {
                if arg.contains("=") {
                    self.parse_equals(&arg)?;
                } else {
                    self.parse_double_dash(&arg, argstream)?;
                }
            }

            else if arg.starts_with("-") {
                if arg == "-" || arg.chars().nth(1).unwrap().is_numeric() {
                    self.arguments.push(arg);
                } else if arg.contains("=") {
                    self.parse_equals(&arg)?;
                } else {
                    self.parse_single_dash(&arg, argstream)?;
                }
            }

            // Is the argument a registered command?

            // No matches so add the argument to our list of positionals.
            else {
                self.arguments.push(arg);
            }

            // is_first_arg = false;
        }

        Ok(())
    }

    fn parse_double_dash(&mut self, arg: &str, argstream: &mut ArgStream) -> Result<(), Error> {

        // Is the argument a registered flag?
        if let Some(index) = self.flag_map.get(&arg[2..]) {
            self.flags[*index].count += 1;
        }

        // Is the argument a registered option name?
        else if let Some(index) = self.option_map.get(&arg[2..]) {
            if argstream.has_next() {
                self.options[*index].values.push(argstream.next());
            } else {
                return Err(Error::MissingValue(format!("missing value for {} option", arg)));
            }
        }

        // Is the argument an automatic --help flag?
        else if arg == "--help" && self.helptext.is_some() {
            println!("{}", self.helptext.as_ref().unwrap());
            std::process::exit(0);
        }

        // Is the argument an automatic --version flag?
        else if arg == "--version" && self.version.is_some() {
            println!("{}", self.version.as_ref().unwrap());
            std::process::exit(0);
        }

        // The argument isn't a registered name.
        else {
            return Err(Error::BadName(format!("{} is not a recognised option name", arg)));
        }

        Ok(())
    }

    fn parse_single_dash(&mut self, arg: &str, argstream: &mut ArgStream) -> Result<(), Error> {
        for c in arg.chars().skip(1) {
            if let Some(index) = self.flag_map.get(&c.to_string()) {
                self.flags[*index].count += 1;
            } else if let Some(index) = self.option_map.get(&c.to_string()) {
                if argstream.has_next() {
                    self.options[*index].values.push(argstream.next());
                } else {
                    return Err(Error::MissingValue(format!("missing value for -{} option", c)));
                }
            } else if c == 'h' && self.helptext.is_some() {
                println!("{}", self.helptext.as_ref().unwrap());
                std::process::exit(0);
            } else if c == 'v' && self.version.is_some() {
                println!("{}", self.version.as_ref().unwrap());
                std::process::exit(0);
            } else {
                return Err(Error::BadName(format!("-{} is not a recognised option name", c)));
            }
        }
        Ok(())
    }

    fn parse_equals(&mut self, arg: &str) -> Result<(), Error> {
        let splits: Vec<&str> = arg.splitn(2, '=').collect();
        let name = splits[0];
        let value = splits[1];

        if let Some(index) = self.option_map.get(name.trim_start_matches('-')) {
            if value == "" {
                return Err(Error::MissingValue(format!("missing value for {} option", name)));
            } else {
                self.options[*index].values.push(value.to_string());
                return Ok(());
            }
        }
        return Err(Error::BadName(format!("{} is not a recognised option name", name)));
    }

}



