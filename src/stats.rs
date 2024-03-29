//! YAPV's stat module documentation
//! # Stat module

mod timer;
use timer::Timer;

use crossbeam::channel::Receiver;

use crossterm::cursor;
use crossterm::style;
use crossterm::style::Color;
use crossterm::style::PrintStyledContent;
use crossterm::style::Stylize;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;

use std::io::Stderr;
use std::io::Write;

pub fn stats_loop(silent: bool, stats_rx: Receiver<usize>) -> std::io::Result<()> {
    let mut total_bytes = 0;
    let start = std::time::Instant::now();
    let mut timer = Timer::new();
    let mut stderr = std::io::stderr();

    loop {
        let num_bytes = stats_rx.recv().unwrap();
        timer.update();
        let rate_per_second = num_bytes as f64 / timer.delta.as_secs_f64();

        total_bytes += num_bytes;

        if !silent && timer.ready {
            timer.ready = false;
            output_progress(
                &mut stderr,
                total_bytes,
                start.elapsed().as_secs().as_time(),
                rate_per_second,
            );
        }

        if num_bytes == 0 {
            break;
        }
    }

    if !silent {
        eprintln!();
    }

    Ok(())
}

fn output_progress(stderr: &mut Stderr, bytes: usize, elapsed: String, rate: f64) {
    let bytes = style::style(format!("{bytes} bytes ")).with(Color::Red);
    let elapsed = style::style(elapsed).with(Color::Green);
    let rate = style::style(format!(" [{rate:.0} b/s]")).with(Color::Blue);

    let _ = crossterm::execute!(
        stderr,
        cursor::MoveToColumn(0),
        Clear(ClearType::CurrentLine),
        PrintStyledContent(bytes),
        PrintStyledContent(elapsed),
        PrintStyledContent(rate)
    );
    let _ = stderr.flush();
}

/// TimeOutput adds a `fn as_time(&self) -> String` method for u64
///
/// # Example
/// Here is an example of how to use it
///
/// ```rust
///use yapv::stats::TimeOutput;
///assert_eq!(154_u64.as_time(), String::from("0:02:34"))
/// ```
pub trait TimeOutput {
    fn as_time(&self) -> String;
}

impl TimeOutput for u64 {
    /// Renders the u64 value into a time string
    fn as_time(&self) -> String {
        let (hours, left) = (*self / 3600, *self % 3600);
        let (minutes, seconds) = (left / 60, left % 60);

        format!("{hours}:{minutes:02}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::TimeOutput;

    #[test]
    fn as_time_format() {
        let pairs = vec![
            (5_u64, "0:00:05"),
            (60_u64, "0:01:00"),
            (154_u64, "0:02:34"),
            (3603_u64, "1:00:03"),
        ];

        for (input, output) in pairs {
            assert_eq!(input.as_time().as_str(), output)
        }
    }
}
