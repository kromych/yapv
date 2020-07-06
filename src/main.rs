use std::env;
use std::io::{self, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1204;

fn main() -> Result<()> {
    let silent = !env::var("YAPV_SILENT").unwrap_or_default().is_empty();
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match std::io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(..) => break,
        };

        total_bytes += num_read;

        if !silent {
            eprint!("\rread:{}", total_bytes);
        }

        if let Err(e) = io::stdout().write_all(&buffer[..num_read]) {
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
