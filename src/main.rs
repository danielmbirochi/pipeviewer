use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let matches = App::new("pipeviewer")
        .arg(
            Arg::with_name("silent")
                .short('s')
                .long("silent")
                .help("Silences the progress output"),
        )
        .arg(
            Arg::with_name("infile")
                .short('i')
                .long("infile")
                .takes_value(true)
                .help("Read from a file instead of stdin"),
        )
        .arg(
            Arg::with_name("outfile")
                .short('o')
                .long("outfile")
                .takes_value(true)
                .help("Write to a file instead of stdout"),
        )
        .get_matches();

    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    let silent = if matches.is_present("silent") {
        true
    } else {
        env::var("SILENT").unwrap_or_default().to_lowercase() == "true"
    };

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
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => panic!("error reading stdin: {}", err),
        };
        total_bytes += bytes_read;

        if let Err(e) = writer.write_all(&buffer[..bytes_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }
    if !silent {
        eprintln!("{} bytes read", total_bytes);
    }
    Ok(())
}
