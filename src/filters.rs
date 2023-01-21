//! Implementation of the filters for Unicode characters.

use lexopt::prelude::*;

pub enum Filter {
    Name(String),

    Range(char, char),
}

impl Filter {
    pub fn matches(&self, chr: char, name: &str) -> bool {
        match self {
            Filter::Name(n) => name.contains(n),
            Filter::Range(s, e) => (*s..=*e).contains(&chr),
        }
    }
}

/// Parse command-line arguments and return a list of filters.
pub fn parse_args() -> Result<Vec<Filter>, lexopt::Error> {
    let mut filters = Vec::new();
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                return Ok(vec![]);
            }

            Short('r') | Long("range") => {
                let (start, end) = parser.value()?.parse_with(parse_range)?;
                filters.push(Filter::Range(start, end));
            }

            Value(value) => {
                let value = value.to_string_lossy().to_uppercase();
                filters.push(Filter::Name(value));
            }

            _ => return Err(arg.unexpected()),
        }
    }

    Ok(filters)
}

fn parse_range(value: &str) -> Result<(char, char), String> {
    macro_rules! p {
        ($e:expr) => {
            match $e {
                value => match u32::from_str_radix(value, 16) {
                    Ok(v) => match char::from_u32(v) {
                        Some(c) => c,
                        None => {
                            return Err(format!("invalid character U+{:X}", v));
                        }
                    },
                    Err(e) => {
                        return Err(format!("invalid value {:?}: {}", value, e));
                    }
                },
            }
        };
    }

    if let Some((start, end)) = value.split_once('-') {
        return Ok((p!(start), p!(end)));
    }

    if let Some((start, end)) = value.split_once('+') {
        let start = p!(start);

        let end = end
            .parse::<u32>()
            .map_err(|e| format!("invalid value {:?}: {}", value, e))
            .and_then(|offset| {
                char::from_u32(start as u32 + offset)
                    .ok_or_else(|| format!("invalid character U+{:X}", start as u32 + offset))
            });

        return Ok((start, end?));
    }

    Err(format!("invalid range: {:?}", value))
}
