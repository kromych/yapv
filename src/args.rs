use std::env;
use clap::{App, Arg};

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool
}

impl Args {
    pub fn parse() -> Self {
        let matches = App::new("Yet Another Pipe Viewer")
            .arg(Arg::with_name("infile").help("Read from a file rather than stdin"))
            .arg(
                Arg::with_name("outfile")
                    .short("o")
                    .long("outfile")
                    .takes_value(true)
                    .help("Write to a file rather than stdout"),
            )
            .arg(
                Arg::with_name("silent")
                    .help("Be silent. Also can set YAPV_SILENT for the effect")
                    .short("s")
                    .long("silent"),
            )
            .get_matches();

        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();
        let silent = if matches.is_present("silent") {
            true
        } else {
            !env::var("YAPV_SILENT").unwrap_or_default().is_empty()
        };

        Self {
            infile,
            outfile,
            silent
        }
    }
}
