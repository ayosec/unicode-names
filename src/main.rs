//! # unicode-names
//!
//! Search Unicode characters using substrings against their names.

use std::env;
use std::io::{stdout, BufWriter, Write};

mod charstable;

fn main() {
    let patterns: Vec<_> = env::args().skip(1).map(|arg| arg.to_uppercase()).collect();

    if patterns.is_empty() {
        eprintln!("Usage: unicode-names [patterns]+");
        return;
    }

    let mut stdout = BufWriter::new(stdout());

    for (chr, name) in charstable::Table::new() {
        if patterns.iter().all(|pat| name.contains(pat)) {
            let res = writeln!(&mut stdout, " {chr:}\tU+{:<6X} {name}", chr as u32);
            if res.is_err() {
                return;
            }
        }
    }
}
