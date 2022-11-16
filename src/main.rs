use clap::Parser;
use std::{
    fs::File,
    io::{self, Read, Write},
};

const BUFFER_SIZE: usize = 8 * 1024;

/// Copy standard input to each FILE, and also to standard output.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// append to the given FILEs, do not overwrite
    #[arg(short, long, default_value_t = false)]
    append: bool,

    /// ignore interrupt signals
    #[arg(short, long, default_value_t = false)]
    ignore_interrupts: bool,

    /// output files
    file: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    if args.ignore_interrupts {
        ctrlc::set_handler(|| {}).unwrap();
    }

    let mut output = args
        .file
        .iter()
        .map(|path| {
            File::options()
                .write(true)
                .create(true)
                .append(args.append)
                .open(&path)
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut buf = vec![0; BUFFER_SIZE];
    loop {
        let bytes_read = stdin.read(&mut buf).expect("Error reading from stdin");
        if bytes_read == 0 {
            break;
        }
        let buf = &buf[..bytes_read];
        for file in &mut output {
            file.write_all(buf).expect("Error writing to a file");
        }
        stdout.write_all(buf).expect("Error writing to stdout");
    }
}
