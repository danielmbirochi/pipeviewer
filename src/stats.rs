pub fn stats(silent: bool, bytes_read: usize, total_bytes: &mut usize, last: bool) {
    *total_bytes += bytes_read;

    if !silent {
        eprint!("\r{}", total_bytes);
        if last {
            eprintln!()
        }
    }
}
