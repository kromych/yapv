use std::env;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Result, Write, BufReader, BufWriter};

use clap::{App, Arg};

const CHUNK_SIZE: usize = 16 * 1204;

fn main() -> Result<()> {
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

    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    let silent = if matches.is_present("silent") {
        true
    } else {
        !env::var("YAPV_SILENT").unwrap_or_default().is_empty()
    };

    //dbg!(infile, outfile, silent);

    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(..) => break,
        };

        total_bytes += num_read;

        if !silent {
            eprint!("\rread:{}", total_bytes);
        }

        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }

            return Err(e);
            // eprintln!("Error: {}", e.to_string());
            // std::process::exit(1);
        }
    }

    Ok(())
}
