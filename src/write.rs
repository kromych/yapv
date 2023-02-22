use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crossbeam::channel::Receiver;

pub fn write_loop(outfile: Option<String>, write_rx: Receiver<Vec<u8>>) -> std::io::Result<()> {
    let mut writer: Box<dyn Write> = if let Some(outfile) = outfile {
        Box::new(BufWriter::new(File::create(outfile)?))
    } else {
        Box::new(BufWriter::new(std::io::stdout()))
    };

    loop {
        // TODO receive bytes
        let buffer = write_rx.recv().unwrap();

        if buffer.is_empty() {
            break;
        }

        if let Err(e) = writer.write_all(&buffer) {
            if e.kind() == std::io::ErrorKind::BrokenPipe {
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
