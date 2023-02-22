use clap::Parser;
use yapv::args::Args;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        silent,
    } = args;
    let silent = silent || !std::env::var("YAPV_SILENT").unwrap_or_default().is_empty();

    let (stats_tx, stats_rx) = crossbeam::channel::unbounded();
    let (write_tx, write_rx) = crossbeam::channel::bounded(1024);

    let read_handle = std::thread::spawn(move || yapv::read::read_loop(infile, stats_tx, write_tx));
    let stats_handle = std::thread::spawn(move || yapv::stats::stats_loop(silent, stats_rx));
    let write_handle = std::thread::spawn(move || yapv::write::write_loop(outfile, write_rx));

    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    read_io_result?;
    stats_io_result?;
    write_io_result?;

    Ok(())
}
