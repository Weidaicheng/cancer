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
    /// # Example
    ///
    /// ```
    /// |text| { println!("hello, {}!", text.unwrap()); }
    /// ```
    run: fn(Option<String>),
}

/// A command flag type
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
    /// let mut command = Command::new("gives a friendly hello", "hello TEXT", |text| {
    ///     println!("hello, {}!", text.unwrap());
    /// });
    /// ```
    pub fn new(description: &str, usage: &str, run: fn(Option<String>)) -> Self {
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
    /// let mut command = Command::new("gives a friendly hello", "hello TEXT", |text| {
    ///     println!("hello, {}!", text.unwrap());
    /// });
    /// command.execute();
    /// ```
    pub fn execute(&mut self) {
        let args = self.get_args();

        let args = self.set_flags(args.iter().map(|x| &x[..]).collect());

        if self.help_exit() {
            return;
        }
        if self.version_exit() {
            return;
        }

        let input = &args[1];
        (self.run)(Some(String::from(input)));
    }

    /// Get args from env
    fn get_args(&self) -> Vec<String> {
        let mut args: Vec<String> = env::args().collect();
        if args.len() <= 1 {
            args.push(format!("{}{}", FLAG_SHORT_START, HELP_SHORT));
        }
        args
    }

    /// Set flags by giving args and returns modified args vector without any flag
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
    fn set_flags(&mut self, mut args: Vec<&str>) -> Vec<String> {
        for i in 0..args.len() {
            let arg = args[i];
            if !(arg.starts_with(FLAG_SHORT_START) || arg.starts_with(FLAG_LONG_START)) {
                continue;
            }
            for mut flag in self.flags.iter_mut() {
                if *arg == format!("{}{}", FLAG_SHORT_START, flag.short)
                    || *arg == format!("{}{}", FLAG_LONG_START, flag.long)
                {
                    flag.has = true;
                    args.remove(i);
                }
            }
        }

        let args: Vec<String> = args.iter().map(|x| String::from(*x)).collect();
        args
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
