//! # Example
//!
//! ```rust
//! use rargsxd::*;
//!
//! let args = vec!("testword".to_string(), "--testflag".to_string(), "-o".to_string(), "monke".to_string());
//! let mut parser = ArgParser::new("program_lol");
//! parser.author("BubbyRoosh")
//!     .version("0.1.0")
//!     .copyright("Copyright (C) 2021 BubbyRoosh")
//!     .info("Example for simple arg parsing crate OwO")
//!     .require_args(true)
//!     .args(
//!         vec!(
//!             Arg::new("testflag")
//!                 .short('t')
//!                 .help("This is a test flag.")
//!                 .flag(false),
//!             Arg::new("testoption")
//!                 .short('o')
//!                 .help("This is a test option.")
//!                 .option("option"),
//!             Arg::new("testword")
//!                 .help("This is a test word.")
//!                 .word(WordType::Boolean(false)),
//!         )
//!     ).parse_vec(args); // .parse() uses std::env::args() so the args vec won't need to be passed.
//!
//! assert!(parser.get_flag("testflag").unwrap());
//! assert!(parser.get_word("testword").unwrap().as_bool().unwrap());
//! assert_eq!(parser.get_option("testoption").unwrap(), "monke");
//! ```

// Copyright (C) 2021 BubbyRoosh
use std::{env, process};
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum WordType {
    /// ArgType's flag but for words.
    Boolean(bool),
    /// ArgType's option but for words.
    String_(String),
}

impl WordType {
    /// Creates a new WordType with `b` boolean
    pub fn boolean(b: bool) -> Self {
        Self::Boolean(b)
    }

    /// Creates a new WordType with `s` &str
    pub fn string(s: &str) -> Self {
        Self::String_(String::from(s))
    }

    /// Gets the WordType if it's WordType::Boolean
    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Boolean(b) = self {
            return Some(*b);
        }
        None
    }

    /// Gets the WordType if it's WordType::String_
    pub fn as_string(&self) -> Option<String> {
        if let Self::String_(s) = self {
            return Some(s.clone());
        }
        None
    }
}

/// Type of argument to check for.
#[derive(Clone, PartialEq)]
pub enum ArgType {
    /// Only used for initialization. Will panic if there's any unknown ArgTypes when initializing.
    Unknown,
    /// Toggles if the name is found when parsing.
    Flag(bool),
    /// Value is set to the next argument if the name is found when parsing.
    Option_(String),
    /// ^^ those but without -/--
    Word(WordType),
}

impl ArgType {
    /// Creates a new ArgType::Option_ with `opt` &str as a default.
    pub fn option(opt: &str) -> Self {
        Self::Option_(String::from(opt))
    }

    /// Creates a new ArgType::Flag with `f` bool as a default.
    pub fn flag(f: bool) -> Self {
        Self::Flag(f)
    }

    /// Creates a new ArgType::Word with `wt` WordType as a default.
    pub fn word(wt: WordType) -> Self {
        Self::Word(wt)
    }
}

/// An argument.
#[derive(Clone)]
pub struct Arg {
    /// Long name (-- if not ArgType::Word) to check for when parsing.
    name: String,
    /// Short name (- if not ArgType::Word) to check for when parsing.
    short: char,
    /// What's printed when self.print_help() is called.
    help: String,
    /// Type of argument to parse for.
    typ: ArgType,

    required: bool,
    set: bool,
}

impl Arg {
    /// Creates a new argument with `namee` &str.
    pub fn new(namee: &str) -> Self {
        let name = String::from(namee);
        Self {
            name,
            short: namee.chars().nth(0).unwrap(),
            help: String::new(),
            typ: ArgType::Unknown,
            required: false,
            set: false,
        }
    }

    /// Makes the argument's type ArgType::Flag, giving it `val` bool.
    pub fn flag(&mut self, val: bool) -> &mut Self {
        self.typ = ArgType::flag(val);
        self
    }

    /// Makes the argument's type ArgType::Option_, giving it `val` &str.
    pub fn option(&mut self, val: &str) -> &mut Self {
        self.typ = ArgType::option(val);
        self
    }

    /// Makes the argument's type ArgType::Word, giving it `wt` WordType.
    pub fn word(&mut self, wt: WordType) -> &mut Self {
        self.typ = ArgType::word(wt);
        self
    }

    /// Sets the output when the help menu is printed.
    pub fn help(&mut self, help: &str) -> &mut Self {
        self.help = String::from(help);
        self
    }

    /// Sets the argument's short name with `short` char.
    pub fn short(&mut self, short: char) -> &mut Self {
        self.short = short;
        self
    }

    /// Sets whether or not to print help/exit if this argument isn't passed.
    pub fn required(&mut self, required: bool) -> &mut Self {
        self.required = required;
        self
    }

    fn set(&mut self) {
        self.set = true;
    }
}

/// Main parser struct.
pub struct ArgParser {
    /// Name of the program.
    name: String,
    /// Name of the author.
    author: String,
    /// Version of the program.
    version: String,
    /// Copyright (if any)
    copyright: String,
    /// Description/info on the program.
    info: String,
    /// Usage (defaults to "{} [flags] [options]", name)
    usage: String,
    args: HashMap<String, Arg>,
    /// Prints help and exits if no args are passed when parsing.
    require_args: bool,
}

impl ArgParser {
    /// Parses std::env::args().
    pub fn parse(&mut self) -> &mut Self {
        let args: Vec<_> = env::args().collect();
        self.parse_vec(args);
        self
    }

    /// Parses a given Vec<String>.
    pub fn parse_vec(&mut self, args: Vec<String>) -> &mut Self {
        if args.len() == 1 && self.require_args {
            self.print_help();
            process::exit(1);
        }

        for (idx, arg) in args.iter().enumerate() {
            if let Some(arg) = self.args.get_mut(arg) {
                if let ArgType::Word(w) = arg.clone().typ {
                    match w {
                        WordType::Boolean(boolean) => {arg.word(WordType::Boolean(!boolean));},
                        WordType::String_(_) => {
                            let next = args.get(idx + 1);
                            if let Some(next) = next {
                                if !next.starts_with('-') {
                                    arg.word(WordType::String_(next.clone()));
                                    arg.set();
                                }
                            }
                        },
                    }
                }
                continue;
            }

            if let Some(arg) = arg.strip_prefix("--") {
                if arg == "help" {self.help_exit()}
                else if arg == "version" {println!("{} {}", self.name, self.version);process::exit(0);}

                if let Some(arg) = self.args.get_mut(arg) {
                    match arg.typ {
                        ArgType::Flag(boolean) => {arg.flag(!boolean);arg.set();},
                        ArgType::Option_(_) => {
                            if let Some(next) = args.get(idx + 1) {
                                if !next.starts_with('-') {
                                    arg.option(&next);
                                    arg.set();
                                }
                            }
                        },
                        _ => {},
                    }
                }
            } else if let Some(arg) = arg.strip_prefix('-') {
                arg.chars().into_iter().for_each(|ch| {
                    if ch == 'h' {self.help_exit()}
                    else if ch == 'v' {println!("{} {}", self.name, self.version);process::exit(0);}

                    for arg in self.args.values_mut() {
                        if arg.short == ch {
                            match arg.typ {
                                ArgType::Flag(boolean) => {arg.flag(!boolean);arg.set();},
                                ArgType::Option_(_) => {
                                    if let Some(next) = args.get(idx + 1) {
                                        if !next.starts_with('-') {
                                            arg.option(&next);
                                            arg.set();
                                        }
                                    }
                                },
                                _ => {},
                            }
                        }
                    }
                });
            }
        }

        self.args.iter().for_each(|(_, arg)| {
            if arg.required && !arg.set {
                println!("Didn't find \"{}\"\n", arg.name);
                self.help_exit();
            }
        });

        self
    }

    /// Gets an option argument's output by name.
    pub fn get_option(&self, name: &str) -> Option<String> {
        if let Some(arg) = self.args.get(name) {
            if let ArgType::Option_(string) = arg.clone().typ {
                return Some(string);
            }
        }
        None
    }

    /// Gets a flag argument's output by name.
    pub fn get_flag(&self, name: &str) -> Option<bool> {
        if let Some(arg) = self.args.get(name) {
            if let ArgType::Flag(boolean) = arg.typ {
                return Some(boolean);
            }
        }
        None
    }

    /// Gets a word argument's output by name.
    pub fn get_word(&self, name: &str) -> Option<WordType> {
        if let Some(arg) = self.args.get(name) {
            if let ArgType::Word(wt) = arg.clone().typ {
                return Some(wt);
            }
        }
        None
    }

    /// Creates a new ArgParser with `name` &str.
    pub fn new(name: &str) -> Self {
        let mut s = Self {
            name: String::from(name),
            author: String::new(),
            version: String::new(),
            copyright: String::new(),
            info: String::new(),
            usage: format!("{} [flags] [options]", name),
            args: HashMap::new(),
            require_args: false,
        };

        s.args(vec!(
            Arg::new("help")
                .short('h')
                .help("Prints the help dialog")
                .flag(false),
            Arg::new("version")
                .short('v')
                .help("Prints the version")
                .flag(false),
        ));
        s
    }

    /// Prints the help dialog.
    pub fn print_help(&self) {
        println!("{} {}\n{}\n{}\n{}", self.name, self.version, self.author, self.info, self.copyright);
        println!("\nUsage:\n\t{}", self.usage);

        let flags: Vec<&Arg> = self.args.iter().filter(|(_, arg)| if let ArgType::Flag(_) = arg.typ{true} else {false}).map(|(_, arg)| arg).collect();
        let options: Vec<&Arg> = self.args.iter().filter(|(_, arg)| if let ArgType::Option_(_) = arg.typ{true} else {false}).map(|(_, arg)| arg).collect();
        let words: Vec<&Arg> = self.args.iter().filter(|(_, arg)| if let ArgType::Word(_) = arg.typ{true} else {false}).map(|(_, arg)| arg).collect();

        if !flags.is_empty() {
            println!("\nFlags:");
            flags.iter().for_each(|flag| {
                println!("\t-{}, --{}\t{}", flag.short, flag.name, flag.help);
            });
        }

        if !options.is_empty() {
            println!("\nOptions:");
            options.iter().for_each(|opt| {
                println!("\t-{}, --{}\t{}", opt.short, opt.name, opt.help);
            });
        }

        if !words.is_empty() {
            println!("\nWords:");
            words.iter().for_each(|opt| {
                println!("\t{}\t{}", opt.name, opt.help);
            });
        }
    }

    /// Sets the name of the program.
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = String::from(name);
        self
    }

    /// Sets the name of the author of the program.
    pub fn author(&mut self, author: &str) -> &mut Self {
        self.author = String::from(author);
        self
    }

    /// Sets the version of the program.
    pub fn version(&mut self, version: &str) -> &mut Self {
        self.version = String::from(version);
        self
    }

    /// Sets the copyright (if any) of the program.
    pub fn copyright(&mut self, copyright: &str) -> &mut Self {
        self.copyright = String::from(copyright);
        self
    }

    /// Sets the info of the program.
    pub fn info(&mut self, info: &str) -> &mut Self {
        self.info = String::from(info);
        self
    }

    /// Sets the usage of the program.
    pub fn usage(&mut self, usage: &str) -> &mut Self {
        self.usage = String::from(usage);
        self
    }

    /// Gives the parser `args` Vec<&mut Arg>.
    pub fn args(&mut self, args: Vec<&mut Arg>) -> &mut Self {
        for arg in args {
            match arg.typ {
                ArgType::Unknown => panic!("No Args can have type Unknown!"),
                _ => {self.args.insert(arg.name.clone(), arg.clone());}
            }

        }
        self
    }

    /// Sets whether or not the program should exit when no arguments are passed.
    pub fn require_args(&mut self, require: bool) -> &mut Self {
        self.require_args = require;
        self
    }

    fn help_exit(&self) {
        self.print_help();
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dash() {
        let args = vec!("--testflag".to_string(),
            "-o".to_string(), "monke".to_string(),
            "-fa".to_string(), "option".to_string(),
        );

        let mut parser = ArgParser::new("program_lol");
        parser.author("BubbyRoosh")
            .version("0.1.0")
            .copyright("Copyright (C) 2021 BubbyRoosh")
            .info("Example for simple arg parsing crate OwO")
            .args(
                vec!(
                    Arg::new("testflag")
                        .short('t')
                        .help("This is a test flag.")
                        .flag(false),

                    Arg::new("testoption")
                        .short('o')
                        .help("This is a test option.")
                        .option("option"),

                    Arg::new("combinedtestflag")
                        .short('f')
                        .help("This is another test flag.")
                        .flag(false),

                    Arg::new("combinedtestoption")
                        .short('a')
                        .help("This is another test option.")
                        .option("monke"),
                )
            ).parse_vec(args);

        assert!(parser.get_flag("testflag").unwrap());
        assert_eq!(parser.get_option("testoption").unwrap(), "monke");

        assert!(parser.get_flag("combinedtestflag").unwrap());
        assert_eq!(parser.get_option("combinedtestoption").unwrap(), "option");
    }

    #[test]
    fn parse_word() {
        let args = vec!(
            "testword".to_string(),
            "anothertestword".to_string(),
            "wordargument".to_string(),
        );

        let mut parser = ArgParser::new("program_lol");
        parser.author("BubbyRoosh")
            .version("0.1.0")
            .copyright("Copyright (C) 2021 BubbyRoosh")
            .info("Example for simple arg parsing crate OwO")
            .args(
                vec!(
                    Arg::new("testword")
                        .help("This is a test word argument.")
                        .word(WordType::Boolean(false)),

                    Arg::new("anothertestword")
                        .help("This is a *another* test word argument.")
                        .word(WordType::string("")),
                )
            ).parse_vec(args);

        assert!(parser.get_word("testword").unwrap().as_bool().unwrap());
        assert_eq!(parser.get_word("anothertestword").unwrap().as_string().unwrap(), "wordargument");
    }
}
