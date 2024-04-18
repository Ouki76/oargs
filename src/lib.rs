/// Struct representing an argument.
pub struct Arg {
    /// Name of the argument.
    pub name: String,
    /// Short form of the argument (e.g., "-s").
    pub short: String,
    /// Long form of the argument (e.g., "--long").
    pub long: String,
    /// Additional arguments associated with this argument.
    pub args: Vec<String>,
}

impl Clone for Arg {
    /// Implements cloning for the Arg struct.
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            short: self.short.clone(),
            long: self.long.clone(),
            args: self.args.clone(),
        }
    }
}

/// Macro for creating an argument.
#[macro_export]
macro_rules! arg {
    ({ name: $name:expr, short: $short:expr, long: $long:expr }) => {
        oargs::Arg {
            name: String::from($name),
            short: String::from($short),
            long: String::from($long),
            args: Vec::new(),
        }
    };
}

/// Struct representing a collection of arguments.
pub struct Args {
    args: Vec<Arg>,
    parsed: Vec<String>,
}

impl Args {
    /// Creates a new instance of Args.
    pub fn new(args: Vec<Arg>, parsed: Vec<String>) -> Self {
        Args { args, parsed }
    }

    /// Retrieves an argument by its name.
    pub fn get(&self, name: &str) -> Option<Arg> {
        if let Some(arg) = self.args.iter().find(|arg| arg.name == name) {
            if let Some(index) = self
                .parsed
                .iter()
                .position(|s| s == &format!("-{}", &arg.short) || s == &format!("--{}", &arg.long))
            {
                let mut cloned_arg = arg.clone();
                cloned_arg.args.extend(
                    self.parsed
                        .iter()
                        .skip(index + 1)
                        .take_while(|s| !s.starts_with('-'))
                        .cloned(),
                );
                Some(cloned_arg)
            } else {
                Some(arg.clone())
            }
        } else {
            None
        }
    }

    /// Checks if the parsed arguments contain a specific argument.
    pub fn contains(&self, name: &str) -> bool {
        if let Some(arg) = self.args.iter().find(|arg| arg.name == name) {
            self.parsed.contains(&format!("-{}", &arg.short))
                || self.parsed.contains(&format!("--{}", &arg.long))
        } else {
            false
        }
    }
}
