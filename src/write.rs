use std::fs::File;
use std::io::{self, BufWriter, Result, Write, ErrorKind};

pub fn write(outfile: &str, buffer: &[u8]) -> Result<bool> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    if let Err(e) = writer.write_all(&buffer) {
        if e.kind() == ErrorKind::BrokenPipe {
            // Stop processing
            return Ok(false)
        }

        return Err(e);
        // eprintln!("Error: {}", e.to_string());
        // std::process::exit(1);
    }

    // Keep going

    Ok(true)
}
