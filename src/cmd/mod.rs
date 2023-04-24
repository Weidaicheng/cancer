use std::env;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

const FLAG_SHORT_START: &str = "-";
const FLAG_LONG_START: &str = "--";

const HELP_SHORT: &str = "h";
const HELP_LONG: &str = "help";
const VERSION_SHORT: &str = "v";
const VERSION_LONG: &str = "version";

/// A command type
pub struct Command {
    /// Command description
    ///
    /// # Example
    ///
    /// gives a friendly hello
    description: String,
    /// Command usage
    ///
    /// # Example
    ///
    /// hello TEXt
    usage: String,
    /// Command flags
    flags: Vec<Flag>,
    /// Command execution logic
    ///
    /// # Arguments
    ///
    /// `text` - An optional string that holds input text
    /// `flags` - A vector of Flag that holds added flags without help or version
    ///
    /// # Example
    ///
    /// ```
    /// |text, flags| { println!("hello, {}!", text.unwrap()); }
    /// ```
    run: fn(text: Option<String>, flags: Vec<&Flag>),
}

#[derive(Debug)]
pub enum FlagValue {
    Bool(bool),
    String(String),
    Int(i32),
    Float(f32),
}

/// A command flag type
#[derive(Debug)]
pub struct Flag {
    /// Flag short identifier
    ///
    /// # Example
    ///
    /// f
    pub short: String,
    /// Flag long identifier
    ///
    /// # Example
    ///
    /// ferris
    pub long: String,
    /// Flag description
    ///
    /// # Example
    ///
    /// say hello from ferris
    description: String,
    /// Flag value
    pub value: FlagValue,
}

impl Command {
    /// Returns a command with the arguments provided and help and version flags
    ///
    /// # Arguments
    ///
    /// * `description` - A string slice that holds the description of the command
    /// * `usage` - A string slice that holds the usage of the command
    /// * `run` - A function with an optional string parameter that holds the logic of the command, this function will be called on command execution
    ///
    /// # Examples
    ///
    /// ```
    /// let mut command = Command::new("gives a friendly hello", "hello TEXT", |text, flags| {
    ///     println!("hello, {}!", text.unwrap());
    /// });
    /// ```
    pub fn new(description: &str, usage: &str, run: fn(Option<String>, Vec<&Flag>)) -> Self {
        let mut command = Command {
            description: String::from(description),
            usage: String::from(usage),
            run,
            flags: vec![],
        };
        command.add_boolean_flag(
            HELP_SHORT,
            HELP_LONG,
            format!("help for {}", PKG_NAME).as_str(),
        );
        command.add_boolean_flag(
            VERSION_SHORT,
            VERSION_LONG,
            format!("version for {}", PKG_NAME).as_str(),
        );

        command
    }
}

impl Command {
    /// Add a new flag for command with the arguments provided
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier of the flag
    ///
    /// `long` - A string slice that holds the long identifier of the flag
    ///
    /// `description` - A string slice that holds the description of the flag
    ///
    /// `default_value` - A enum that holds the default value of the flag
    ///
    /// # Examples
    ///
    /// ```
    /// command.add_flag("f", "ferris", "say hello from ferris", FlagValue::Bool(false));
    /// ```
    fn add_flag(&mut self, short: &str, long: &str, description: &str, default_value: FlagValue) {
        self.flags.push(Flag {
            short: String::from(short),
            long: String::from(long),
            description: String::from(description),
            value: default_value,
        });
    }

    /// Add a boolean flag for command with the arguments provided and value set to `false` by default
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier of the flag
    ///
    /// `long` - A string slice that holds the long identifier of the flag
    ///
    /// `description` - A string slice that holds the description of the flag
    ///
    /// # Examples
    ///
    /// ```
    /// command.add_boolean_flag("f", "ferris", "say hello from ferris");
    /// ```
    pub fn add_boolean_flag(&mut self, short: &str, long: &str, description: &str) {
        self.add_flag(short, long, description, FlagValue::Bool(false));
    }

    /// Add a string flag for command with the arguments provided and value set to `""` by default
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier of the flag
    ///
    /// `long` - A string slice that holds the long identifier of the flag
    ///
    /// `description` - A string slice that holds the description of the flag
    ///
    /// # Examples
    ///
    /// ```
    /// command.add_string_flag("f", "ferris", "say hello from ferris");
    /// ```
    pub fn add_string_flag(&mut self, short: &str, long: &str, description: &str) {
        self.add_flag(
            short,
            long,
            description,
            FlagValue::String(String::from("")),
        );
    }

    /// Add a int flag for command with the arguments provided and value set to `0` by default
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier of the flag
    ///
    /// `long` - A string slice that holds the long identifier of the flag
    ///
    /// `description` - A string slice that holds the description of the flag
    ///
    /// # Examples
    ///
    /// ```
    /// command.add_int_flag("f", "ferris", "say hello from ferris");
    /// ```
    pub fn add_int_flag(&mut self, short: &str, long: &str, description: &str) {
        self.add_flag(short, long, description, FlagValue::Int(0));
    }

    /// Add a float flag for command with the arguments provided and value set to `0.0` by default
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier of the flag
    ///
    /// `long` - A string slice that holds the long identifier of the flag
    ///
    /// `description` - A string slice that holds the description of the flag
    ///
    /// # Examples
    ///
    /// ```
    /// command.add_float_flag("f", "ferris", "say hello from ferris");
    /// ```
    pub fn add_float_flag(&mut self, short: &str, long: &str, description: &str) {
        self.add_flag(short, long, description, FlagValue::Float(0.0));
    }
}

impl Command {
    /// Execute command
    ///
    /// # Example
    ///
    /// ```
    /// let mut command = Command::new("gives a friendly hello", "hello TEXT", |text, flags| {
    ///     todo!();
    /// });
    /// command.execute();
    /// ```
    pub fn execute(&mut self) {
        let args = self.get_args();

        let args = self.update_flags(args.iter().map(|x| &x[..]).collect());

        if self.help_exit() {
            return;
        }
        if self.version_exit() {
            return;
        }

        let input = &args[1];
        let flags = self.get_flags();
        (self.run)(Some(String::from(input)), flags);
    }

    /// Get args from env
    fn get_args(&self) -> Vec<String> {
        let mut args: Vec<String> = env::args().collect();
        if args.len() <= 1 {
            args.push(format!("{}{}", FLAG_SHORT_START, HELP_SHORT));
        }
        args
    }

    /// Update flags value by giving args and returns simple args vector without any flag
    ///
    /// # Arguments
    ///
    /// `args` - A vector of string slice that holds arguments
    ///
    /// # Return
    ///
    /// A vector of string that without any flag from args
    ///
    /// # Example
    ///
    /// ```
    /// let args = vec!["target/debug/hello", "-f", "world"];
    /// let args = self.set_flags(args);
    /// dbg!(&args);
    /// // output:
    /// // [src/cmd/mod.rs:95] &args = [
    /// //     "target/debug/hello",
    /// //     "world",
    /// // ]
    /// ```
    fn update_flags(&mut self, args: Vec<&str>) -> Vec<String> {
        let mut simple_args: Vec<String> = vec![];

        for arg in args {
            if !(arg.starts_with(FLAG_SHORT_START) || arg.starts_with(FLAG_LONG_START)) {
                simple_args.push(String::from(arg));
                continue;
            }
            for mut flag in self.flags.iter_mut() {
                if *arg == format!("{}{}", FLAG_SHORT_START, flag.short)
                    || *arg == format!("{}{}", FLAG_LONG_START, flag.long)
                {
                    flag.value = FlagValue::Bool(true);
                }
            }
        }

        simple_args
    }

    /// Returns added flags without help or version
    ///
    /// # Example
    ///
    /// ```
    /// // returns empty flag vector when not providing any flag
    /// let mut command = Command::new("gives a friendly hello", "hello TEXT", |text, flags| {
    ///     todo!();
    /// });
    /// let flags = command.get_flags();
    /// assert_eq!(0, flags.len());
    ///
    /// // returns non empty flag vector when providing flag
    /// let mut command = Command::new("gives a friendly hello", "hello TEXT", |text, flags| {
    ///     todo!();
    /// });
    /// command.add_flag("f", "ferris", "say hello from ferris");
    /// let flags = command.get_flags();
    /// assert_eq!(1, flags.len());
    /// ```
    fn get_flags(&self) -> Vec<&Flag> {
        let mut simple_flags: Vec<&Flag> = vec![];

        for flag in self.flags.iter() {
            if !(flag.short == HELP_SHORT || flag.short == VERSION_SHORT) {
                simple_flags.push(flag);
            }
        }

        simple_flags
    }

    /// Check if help needed to display and exit,
    ///
    /// by if arguments contains `-h` or `--help`
    fn help_exit(&self) -> bool {
        let mut exit = false;

        for flag in self.flags.iter() {
            if flag.short == HELP_SHORT {
                match flag.value {
                    FlagValue::Bool(value) => {
                        if value {
                            self.help();
                            exit = true;
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }

        exit
    }

    /// Check if version needed to display and exit,
    ///
    /// by if arguments contains `-v` or `--version`
    fn version_exit(&self) -> bool {
        let mut exit = false;

        for flag in self.flags.iter() {
            if flag.short == VERSION_SHORT {
                match flag.value {
                    FlagValue::Bool(value) => {
                        if value {
                            self.version();
                            exit = true;
                            break;
                        }
                    }
                    _ => (),
                }
            }
        }

        exit
    }
}

impl Command {
    /// Print help document
    ///
    /// # Example
    ///
    /// command description
    ///
    /// Usage:
    ///
    ///       command TEXT
    ///
    /// Flags:
    ///
    ///       -h, --help        help for command
    ///       -v, --version     version for command
    pub fn help(&self) {
        println!("{}", self.description);
        println!();
        println!("Usage:");
        println!("  {}", self.usage);
        println!();
        println!("Flags:");
        for flag in self.flags.iter() {
            println!(
                "  {}{}, {}{}\t{}",
                FLAG_SHORT_START, flag.short, FLAG_LONG_START, flag.long, flag.description
            )
        }
    }

    /// Print version information
    ///
    /// # Example
    ///
    /// command version 1.0.0
    pub fn version(&self) {
        println!("{} version {}", PKG_NAME, PKG_VERSION);
    }
}
