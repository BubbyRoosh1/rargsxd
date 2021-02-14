//! # Example
//!
//! ```rust
//! use rargsxd::*;
//!
//!let mut args = ArgParser::new("program_lol");
//!args.author("BubbyRoosh")
//!    .version("0.1.0")
//!    .copyright("Copyright (C) 2021 BubbyRoosh")
//!    .info("Example for simple arg parsing crate OwO")
//!    .require_args(true) // Requires args to be passed, otherwise prints help and exits
//!    .args(
//!        vec!(
//!            Arg::new("test")
//!                .short("t")
//!                .help("This is a test flag")
//!                .flag(false),
//!            Arg::new("monke")
//!                .short("m")
//!                .help("This is a test option")
//!                .option("oo"),
//!        )
//!    )
//!    .parse();
//!
//!// If "-t" or "--test" is passed, this will run
//!if args.get_flag("test").unwrap() {
//!    println!("Hello, world!");
//!}
//!
//!// This will be "oo" unless "--monke" or "-m" is passed with a string argument
//!println!("{}", args.get_option("monke").unwrap());
//! ```

// Copyright (C) 2021 BubbyRoosh
use std::{env, process};

#[derive(Clone, PartialEq)]
pub enum ArgType {
    /// Only used for initialization. Will panic if there's any unknown ArgTypes when initializing.
    Unknown,
    Flag(bool),
    Option_(String),
}

impl ArgType {
    pub fn new(string: Option<String>) -> Self {
        match string {
            Some(s) => Self::Option_(s),
            None => Self::Flag(false),
        }
    }
}

#[derive(Clone)]
pub struct Arg {
    name: String,
    short: String,
    help: String,
    typ: ArgType,
}

impl Arg {
    pub fn new(name: &str) -> Self {
        let name = String::from(name);
        Self {
            name,
            short: String::new(),
            help: String::new(),
            typ: ArgType::Unknown,
        }
    }

    pub fn flag(&mut self, val: bool) -> &mut Self {
        self.typ = ArgType::Flag(val);
        self
    }

    pub fn option(&mut self, val: &str) -> &mut Self {
        self.typ = ArgType::Option_(String::from(val));
        self
    }

    pub fn help(&mut self, help: &str) -> &mut Self {
        self.help = String::from(help);
        self
    }

    pub fn short(&mut self, short: &str) -> &mut Self {
        self.short = String::from(short);
        self
    }
}

pub struct ArgParser {
    name: String,
    author: String,
    version: String,
    copyright: String,
    info: String,
    usage: String,
    flags: Vec<Arg>,
    options: Vec<Arg>,
    require_args: bool,
}

impl ArgParser {
    pub fn parse(&mut self) -> &mut Self {
        let args: Vec<_> = env::args().collect();
        self.parse_args(args);
        self
    }

    pub fn parse_args(&mut self, args: Vec<String>) -> &mut Self {
        if args.len() == 1 && self.require_args {
            self.print_help();
            process::exit(1);
        }

        for (idx, arg) in args.iter().enumerate() {
            if let Some(arg) = arg.strip_prefix("--") {
                if arg == "help" {
                    self.print_help();
                    process::exit(0);
                } else if arg == "version" {
                    println!("{} {}", self.name, self.version);
                    process::exit(0);
                }
                for flag in self.flags.iter_mut() {
                    if flag.name == arg {
                        // In theory this will always be a Flag because of the args() method
                        if let ArgType::Flag(boolean) = flag.typ {
                            flag.flag(!boolean);
                        }
                    }
                }
                for option in self.options.iter_mut() {
                    if option.name == arg {
                        let next = args.get(idx + 1);
                        if let Some(next) = next {
                            if !next.starts_with('-') {
                                option.option(&next);
                            }
                        }
                    }
                }
            } else if let Some(arg) = arg.strip_prefix('-') {
                if arg == "h" {self.print_help();process::exit(1);}
                else if arg == "v" {println!("{} {}", self.name, self.version);process::exit(0);}
                for flag in self.flags.iter_mut() {
                    if flag.short == arg {
                        // In theory this will always be a Flag because of the args() method
                        if let ArgType::Flag(boolean) = flag.typ {
                            flag.flag(!boolean);
                        }
                    }
                }
                for option in self.options.iter_mut() {
                    if option.short == arg {
                        let next = args.get(idx + 1);
                        if let Some(next) = next {
                            if !next.starts_with('-') {
                                option.option(&next);
                            }
                        }
                    }
                }
            }
        }
        self
    }

    pub fn get_option(&self, name: &str) -> Option<String> {
        for option in self.options.clone() {
            if option.name == name {
                if let ArgType::Option_(string) = option.typ {
                    return Some(string);
                }
                break;
            }
        }
        None
    }

    pub fn get_flag(&self, name: &str) -> Option<bool> {
        for flag in self.flags.clone() {
            if flag.name == name {
                if let ArgType::Flag(boolean) = flag.typ {
                    return Some(boolean);
                }
                break;
            }
        }
        None
    }

    pub fn new(name: &str) -> Self {
        let mut s = Self {
            name: String::from(name),
            author: String::new(),
            version: String::new(),
            copyright: String::new(),
            info: String::new(),
            usage: format!("{} [flags] [options]", name),
            flags: Vec::new(),
            options: Vec::new(),
            require_args: false,
        };
        s.args(vec!(
            Arg::new("help")
                .short("h")
                .help("Prints the help dialog")
                .flag(false),
            Arg::new("version")
                .short("v")
                .help("Prints the version")
                .flag(false),
        ));
        s
    }

    pub fn print_help(&self) {
        println!("{} {}\n{}\n{}\n{}", self.name, self.version, self.author, self.info, self.copyright);
        println!("\nUsage:\n\t{}", self.usage);

        if !self.flags.is_empty() {
            println!("\nFlags:");
            self.flags.iter().for_each(|flag| {
                println!("\t-{}, --{}\t{}", flag.short, flag.name, flag.help);
            });
        }

        if !self.options.is_empty() {
            println!("\nOptions:");
            self.options.iter().for_each(|opt| {
                println!("\t-{}, --{}\t{}", opt.short, opt.name, opt.help);
            });
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = String::from(name);
        self
    }

    pub fn author(&mut self, author: &str) -> &mut Self {
        self.author = String::from(author);
        self
    }

    pub fn version(&mut self, version: &str) -> &mut Self {
        self.version = String::from(version);
        self
    }

    pub fn copyright(&mut self, copyright: &str) -> &mut Self {
        self.copyright = String::from(copyright);
        self
    }

    pub fn info(&mut self, info: &str) -> &mut Self {
        self.info = String::from(info);
        self
    }

    pub fn args(&mut self, args: Vec<&mut Arg>) -> &mut Self {
        for arg in args {
            match arg.typ {
                ArgType::Unknown => panic!("No Args can have type Unknown!"),
                ArgType::Flag(_) => self.flags.push(arg.clone()),
                ArgType::Option_(_) => self.options.push(arg.clone()),
            }

        }
        self
    }

    pub fn require_args(&mut self, require: bool) -> &mut Self {
        self.require_args = require;
        self
    }
}
