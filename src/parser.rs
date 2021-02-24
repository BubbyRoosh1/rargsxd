use super::argument::*;

use std::{env, process};
use std::collections::HashMap;

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
        let mut args: Vec<_> = env::args().collect();
        args.remove(0);
        self.parse_vec(args);
        self
    }

    /// Parses a given Vec<String>.
    pub fn parse_vec(&mut self, args: Vec<String>) -> &mut Self {
        if args.len() == 0 && self.require_args {
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
                else if arg == "version" {self.version_exit()}

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
                } else {
                    self.unexpected(&format!("--{}", arg));
                }

            } else if let Some(arg) = arg.strip_prefix('-') {
                arg.chars().into_iter().for_each(|ch| {
                    if ch == 'h' {self.help_exit()}
                    else if ch == 'v' {self.version_exit()}

                    for arg in self.args.values_mut() {
                        if arg.short == ch {
                            match arg.typ {
                                ArgType::Flag(boolean) => {arg.flag(!boolean);arg.set();},
                                ArgType::Option_(_) => {
                                    if let Some(next) = args.get(idx + 1) {
                                        if !next.starts_with('-') {
                                            arg.option(&next);
                                            arg.set();
                                        } else {
                                            println!("{}", arg.name);
                                            // Couldn't do this here because multiple mutable calls
                                            // to self
                                            //self.unexpected(next);

                                            eprintln!("Unexpected argument: \"{}\"", next);
                                            self.print_help();
                                            process::exit(1);
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

        let flags: Vec<&Arg> = self.args
            .iter()
            .filter(|(_, arg)| if let ArgType::Flag(_) = arg.typ{true} else {false})
            .map(|(_, arg)| arg)
            .collect();

        let options: Vec<&Arg> = self.args
            .iter()
            .filter(|(_, arg)| if let ArgType::Option_(_) = arg.typ{true} else {false})
            .map(|(_, arg)| arg)
            .collect();

        let words: Vec<&Arg> = self.args
            .iter()
            .filter(|(_, arg)| if let ArgType::Word(_) = arg.typ{true} else {false})
            .map(|(_, arg)| arg)
            .collect();

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

    fn version_exit(&self) {
        println!("{} {}", self.name, self.version);
        process::exit(1);
    }

    fn unexpected(&self, arg: &str) {
        eprintln!("Unexpected argument: \"{}\"", arg);
        self.help_exit();
    }
}
