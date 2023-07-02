use std::env;
use std::io::{self, Read, Write, Result, ErrorKind};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let silent = env::var("SILENT").unwrap_or_default().to_lowercase() == "true";
    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let bytes_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => panic!("error reading stdin: {}", err),
        };
        total_bytes += bytes_read;
        if !silent {
            eprint!("{}", total_bytes);
        }

        if let Err(e) = io::stdout().write_all(&buffer[..bytes_read]) {
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
