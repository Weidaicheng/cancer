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

/// A command flag type
#[derive(Debug)]
pub struct Flag {
    /// Flag short identifier
    ///
    /// # Example
    ///
    /// f
    short: String,
    /// Flag long identifier
    ///
    /// # Example
    ///
    /// ferris
    long: String,
    /// Flag description
    ///
    /// # Example
    ///
    /// say hello from ferris
    description: String,
    /// Represents this flag is provided or not, `false` by default
    has: bool,
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
        command.add_flag(
            HELP_SHORT,
            HELP_LONG,
            format!("help for {}", PKG_NAME).as_str(),
        );
        command.add_flag(
            VERSION_SHORT,
            VERSION_LONG,
            format!("version for {}", PKG_NAME).as_str(),
        );

        command
    }

    /// Add a new flag for command with the arguments provided and the `has` filed set to `false`
    ///
    /// # Arguments
    ///
    /// `short` - A string slice that holds the short identifier of the flag
    /// `long` - A string slice that holds the long identifier of the flag
    /// `description` - A string slice that holds the description of the flag
    ///
    /// # Examples
    ///
    /// ```
    /// command.add_flag("f", "ferris", "say hello from ferris");
    /// ```
    pub fn add_flag(&mut self, short: &str, long: &str, description: &str) {
        self.flags.push(Flag {
            short: String::from(short),
            long: String::from(long),
            description: String::from(description),
            has: false,
        });
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
                    flag.has = true;
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
            if flag.has && flag.short == HELP_SHORT {
                self.help();
                exit = true;
                break;
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
            if flag.has && flag.short == VERSION_SHORT {
                self.version();
                exit = true;
                break;
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
