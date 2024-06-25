use std::{fs, path::Path};
use clap::{Arg, ArgGroup, ArgMatches, Command, ValueHint};

pub struct Cli {
    pub matches: ArgMatches,
}

impl Cli {
    pub fn new() -> Self {
        let matches = Command::new("YKKLanguage")
            .author("ykk2b")
            .version("0.0.1-beta.2")
            .about("Minimalistic programming language")
            .group(ArgGroup::new("main"))
            .arg(
                Arg::new("run")
                    .short('r')
                    .long("run")
                    .aliases(["dev", "start"])
                    .value_name("FILE")
                    .help("Interpret the FILE")
                    .value_hint(ValueHint::FilePath)
                    .display_order(0)
                    .group("main"),
            )
            .subcommand(Command::new("--no-optimize").about("disable analyzing"))
            .get_matches();

        Self { matches }
    }

    pub fn run(&self) -> String {
        let file_path_str = self
            .matches
            .get_one::<String>("run")
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "No file specified"))
            .unwrap()
            .to_string();

        let file_path = Path::new(&file_path_str);
        fs::read_to_string(file_path).unwrap()
    }

    pub fn no_optimize(&self) -> bool {
        self.matches.subcommand_matches("--no-optimize").is_some()
    }
}
