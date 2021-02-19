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
//!                 .help("This is a test option.")
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

#[derive(Clone, PartialEq)]
pub enum WordType {
    Boolean(bool),
    String_(String),
}

impl WordType {
    pub fn boolean(b: bool) -> Self {
        Self::Boolean(b)
    }

    pub fn string(s: &str) -> Self {
        Self::String_(String::from(s))
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let Self::Boolean(b) = self {
            return Some(*b);
        }
        None
    }

    pub fn as_string(&self) -> Option<String> {
        if let Self::String_(s) = self {
            return Some(s.clone());
        }
        None
    }
}

#[derive(Clone, PartialEq)]
pub enum ArgType {
    /// Only used for initialization. Will panic if there's any unknown ArgTypes when initializing.
    Unknown,
    Flag(bool),
    Option_(String),
    Word(WordType),
}

impl ArgType {
    pub fn option(opt: &str) -> Self {
        Self::Option_(String::from(opt))
    }

    pub fn flag(f: bool) -> Self {
        Self::Flag(f)
    }

    pub fn word(wt: WordType) -> Self {
        Self::Word(wt)
    }
}

#[derive(Clone)]
pub struct Arg {
    name: String,
    short: char,
    help: String,
    typ: ArgType,
}

impl Arg {
    pub fn new(namee: &str) -> Self {
        let name = String::from(namee);
        Self {
            name,
            short: namee.chars().nth(0).unwrap(),
            help: String::new(),
            typ: ArgType::Unknown,
        }
    }

    pub fn flag(&mut self, val: bool) -> &mut Self {
        self.typ = ArgType::flag(val);
        self
    }

    pub fn option(&mut self, val: &str) -> &mut Self {
        self.typ = ArgType::option(val);
        self
    }

    pub fn word(&mut self, wt: WordType) -> &mut Self {
        self.typ = ArgType::word(wt);
        self
    }

    pub fn help(&mut self, help: &str) -> &mut Self {
        self.help = String::from(help);
        self
    }

    pub fn short(&mut self, short: char) -> &mut Self {
        self.short = short;
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
    words: Vec<Arg>,
    require_args: bool,
}

impl ArgParser {
    pub fn parse(&mut self) -> &mut Self {
        let args: Vec<_> = env::args().collect();
        self.parse_vec(args);
        self
    }

    pub fn parse_vec(&mut self, args: Vec<String>) -> &mut Self {
        if args.len() == 1 && self.require_args {
            self.print_help();
            process::exit(1);
        }

        for (idx, arg) in args.iter().enumerate() {
            for word in self.words.iter_mut() {
                if word.name == *arg {
                    if let ArgType::Word(w) = word.clone().typ {
                        match w {
                            WordType::Boolean(boolean) => {word.word(WordType::Boolean(!boolean));},
                            WordType::String_(_) => {
                                let next = args.get(idx + 1);
                                if let Some(next) = next {
                                    if !next.starts_with('-') {
                                        word.word(WordType::String_(next.clone()));
                                    }
                                }
                            },
                        }
                    }
                }
            }
            if let Some(arg) = arg.strip_prefix("--") {
                if arg == "help" {self.print_help();process::exit(0);}
                else if arg == "version" {println!("{} {}", self.name, self.version);process::exit(0);}

                for flag in self.flags.iter_mut() {
                    if flag.name == arg {
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

                arg.chars().into_iter().for_each(|ch| {
                    if ch == 'h' {self.print_help();process::exit(1);}
                    else if ch == 'v' {println!("{} {}", self.name, self.version);process::exit(0);}

                    for flag in self.flags.iter_mut() {
                        if flag.short == ch {
                            if let ArgType::Flag(boolean) = flag.typ {
                                flag.flag(!boolean);
                            }
                        }
                    }
                    for option in self.options.iter_mut() {
                        if option.short == ch {
                            let next = args.get(idx + 1);
                            if let Some(next) = next {
                                if !next.starts_with('-') {
                                    option.option(&next);
                                }
                            }
                        }
                    }
                });
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

    pub fn get_word(&self, name: &str) -> Option<WordType> {
        for word in self.words.clone() {
            if word.name == name {
                if let ArgType::Word(res) = word.typ {
                    return Some(res);
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
            words: Vec::new(),
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

    pub fn usage(&mut self, usage: &str) -> &mut Self {
        self.usage = String::from(usage);
        self
    }

    pub fn args(&mut self, args: Vec<&mut Arg>) -> &mut Self {
        for arg in args {
            match arg.typ {
                ArgType::Unknown => panic!("No Args can have type Unknown!"),
                ArgType::Flag(_) => self.flags.push(arg.clone()),
                ArgType::Option_(_) => self.options.push(arg.clone()),
                ArgType::Word(_) => self.words.push(arg.clone()),
            }

        }
        self
    }

    pub fn require_args(&mut self, require: bool) -> &mut Self {
        self.require_args = require;
        self
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
