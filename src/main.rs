//! # unicode-names
//!
//! Search Unicode characters using substrings against their names.

use std::io::{stdout, BufWriter, Write};
use std::process::ExitCode;

mod charstable;
mod filters;

const HELP: &str = include_str!("HELP.txt");

fn main() -> ExitCode {
    let filters = match filters::parse_args() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return ExitCode::FAILURE;
        }
    };

    if filters.is_empty() {
        help();
        return ExitCode::SUCCESS;
    }

    let mut stdout = BufWriter::new(stdout());

    for (chr, name) in charstable::Table::new() {
        if filters.iter().all(|filter| filter.matches(chr, &name)) {
            let res = writeln!(&mut stdout, " {chr:}\tU+{:<6X} {name}", chr as u32);
            if res.is_err() {
                return ExitCode::from(2);
            }
        }
    }

    ExitCode::SUCCESS
}

fn help() {
    println!(
        "{} {}\n\n{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        HELP
    );
}
