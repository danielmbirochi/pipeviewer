use clap::{App, Arg};
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("pipeviewer")
            .arg(
                Arg::with_name("silent")
                    .short('s')
                    .long("silent")
                    .help("Silences the progress output"),
            )
            .arg(
                Arg::with_name("infile")
                    .short('i')
                    .long("infile")
                    .takes_value(true)
                    .help("Read from a file instead of stdin"),
            )
            .arg(
                Arg::with_name("outfile")
                    .short('o')
                    .long("outfile")
                    .takes_value(true)
                    .help("Write to a file instead of stdout"),
            )
            .get_matches();

        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();
        let silent = if matches.is_present("silent") {
            true
        } else {
            env::var("SILENT").unwrap_or_default().to_lowercase() == "true"
        };

        Self {
            infile,
            outfile,
            silent,
        }
    }
}
