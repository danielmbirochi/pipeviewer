use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    let silent = env::var("SILENT").unwrap_or_default().to_lowercase() == "true";
    let mut total_bytes = 0;

    loop {
        let mut buffer = [0; CHUNK_SIZE];
        let bytes_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(err) => panic!("error reading stdin: {}", err),
        };
        total_bytes += bytes_read;
        io::stdout().write_all(&buffer[..bytes_read]).unwrap();
    }
    if !silent {
        eprintln!("{} bytes read", total_bytes);
    }
}
