// TODO: handle parsing cli commands

use std::fs::read_to_string;

use clap::{Arg, ArgGroup, Command, ValueHint};

pub struct Cli {
    pub run: Option<String>,
}

impl Cli {
    pub fn new() -> Cli {
        let matches = Command::new("YKKLanguage")
            .author("ykk2b")
            .version("0.0.1-beta.1")
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
            .get_matches();

            Cli {
            run: matches
                .clone()
                .get_one::<String>("run")
                .map(|s| s.to_string()),
        }
    }

    pub fn input(&self) -> Result<String, std::io::Error> {
        match &self.run {
            Some(file_path) => read_to_string(file_path),
            None => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no file specified",
            )),
        }
    }
}
