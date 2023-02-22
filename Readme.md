# Yet Another Pipe Viewer

This is an educational project that follows the Packt's book on system programming in Rust.
The utility is similar to pv, and outputs number of bytes processed and statistics.

Command line parameters:

```text
Yet Another Pipe Viewer 

USAGE:
    yapv [FLAGS] [OPTIONS] [infile]

FLAGS:
    -h, --help       Prints help information
    -s, --silent     Be silent. Also can set YAPV_SILENT for the effect
    -V, --version    Prints version information

OPTIONS:
    -o, --outfile <outfile>    Write to a file rather than stdout

ARGS:
    <infile>    Read from a file rather than stdin

```

Here is a fun example to give you an idea:

```sh

yes | target/release/yapv -o /dev/null

```
