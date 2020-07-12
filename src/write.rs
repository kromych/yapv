use std::fs::File;
use std::io::{self, BufWriter, ErrorKind, Result, Write};
use std::sync::mpsc::Receiver;

pub fn write_loop(outfile: &str, write_rx: Receiver<Vec<u8>>) -> Result<()> {
    let mut writer: Box<dyn Write> = if !outfile.is_empty() {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(io::stdout()))
    };

    loop {
        // TODO receive bytes
        let buffer = write_rx.recv().unwrap();

        if buffer.is_empty() {
            break;
        }

        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == ErrorKind::BrokenPipe {
                // Stop processing
                return Ok(());
            }

            return Err(e);
            // eprintln!("Error: {}", e.to_string());
            // std::process::exit(1);
        }
    }

    Ok(())
}
