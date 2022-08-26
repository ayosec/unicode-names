//! # unicode-names
//!
//! Search Unicode characters using substrings against their names.

use std::env;

mod charstable;

fn main() {
    let patterns: Vec<_> = env::args().skip(1).map(|arg| arg.to_uppercase()).collect();

    if patterns.is_empty() {
        eprintln!("Usage: unicode-names [patterns]+");
        return;
    }

    for (chr, name) in charstable::Table::new() {
        if patterns.iter().all(|pat| name.contains(pat)) {
            println!(" {chr:}\tU+{:<6X} {name}", chr as u32);
        }
    }
}
