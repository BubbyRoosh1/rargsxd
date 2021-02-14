# Example:

```rust
use rargsxd::*;

fn main() {
    let mut args = ArgParser::new("program_lol");
    args.author("BubbyRoosh")
        .version("0.1.0")
        .copyright("Copyright (C) 2021 BubbyRoosh")
        .info("Example for simple arg parsing crate OwO")
        .require_args(true) // Makes the program print help and exit if there are no arguments passed
        .args(
            vec!(
                Arg::new("test")
                    .short("t")
                    .help("This is a test flag")
                    .flag(false),
                Arg::new("monke")
                    .short("m")
                    .help("This is a test option")
                    .option("oo"),
            )
        )
        .parse();

    // If "-t" or "--test" is passed, this will run
    if args.get_flag("test").unwrap() {
        println!("Hello, world!");
    }

    // This will be "oo" unless "--monke" or "-m" is passed with a string argument
    println!("{}", args.get_option("monke").unwrap());
}
```
