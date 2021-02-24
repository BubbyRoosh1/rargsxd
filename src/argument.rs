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
    pub (crate) name: String,
    /// Short name (- if not ArgType::Word) to check for when parsing.
    pub (crate) short: char,
    /// What's printed when self.print_help() is called.
    pub (crate) help: String,
    /// Type of argument to parse for.
    pub (crate) typ: ArgType,

    pub (crate) required: bool,
    pub (crate) set: bool,
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

    pub (crate) fn set(&mut self) {
        self.set = true;
    }
}
