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
mod argument;
mod parser;

pub use argument::*;
pub use parser::*;


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
        parser.args(
                vec!(
                    Arg::new("testflag")
                        .flag(false),

                    Arg::new("testoption")
                        .short('o')
                        .option("option"),

                    Arg::new("combinedtestflag")
                        .short('f')
                        .flag(false),

                    Arg::new("combinedtestoption")
                        .short('a')
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
        parser.args(
                vec!(
                    Arg::new("testword")
                        .word(WordType::Boolean(false)),

                    Arg::new("anothertestword")
                        .word(WordType::string("")),
                )
            ).parse_vec(args);

        assert!(parser.get_word("testword").unwrap().as_bool().unwrap());
        assert_eq!(parser.get_word("anothertestword").unwrap().as_string().unwrap(), "wordargument");
    }

    #[test]
    fn extra() {
        let args = vec!(
            "--monke".to_string(),
            "oo oo".to_string(),
            "extra".to_string(),
        );
        let mut parser = ArgParser::new("program_lol");
        parser.args(
                vec!(
                    Arg::new("monke")
                        .option(""),
                )
            ).parse_vec(args);

        assert_eq!(parser.get_option("monke").unwrap(), "oo oo");
        assert!(parser.extra.contains(&String::from("extra")));
    }
}
