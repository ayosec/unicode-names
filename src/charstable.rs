//! Parser for the table generated in the build script.
//!
//! # Table Format
//!
//! Each entry in the table has 3 fields:
//!
//! - The first byte is the length of the entry.
//! - The next bytes are the UTF-8 character for the entry.
//! - The rest of the entry is the character name.
//!
//! The table is compressed with Zlib.

use std::io::Read;

use flate2::read::ZlibDecoder;

const TABLE_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/UnicodeTable.z"));

pub struct Table<T>(T);

impl Table<()> {
    pub fn new() -> Table<impl Read> {
        let decoder = ZlibDecoder::new(TABLE_DATA);
        Table(decoder)
    }
}

impl<T: Read> Iterator for Table<T> {
    type Item = (char, String);

    fn next(&mut self) -> Option<Self::Item> {
        let mut entry_len = [0_u8];
        self.0.read_exact(&mut entry_len).ok()?;

        if entry_len[0] == 0 {
            return None;
        }

        let mut entry = vec![0; entry_len[0] as usize];
        self.0.read_exact(&mut entry[..]).ok()?;

        let mut entry = String::from_utf8(entry).ok()?;

        let chr = entry.remove(0);
        Some((chr, entry))
    }
}

#[test]
fn extract_alpha_chars() {
    use std::collections::HashSet;

    let mut expected = HashSet::new();
    for c in "αΑᾱᾴ".chars() {
        expected.insert(c);
    }

    for (c, _) in Table::new() {
        expected.remove(&c);
    }

    assert_eq!(expected, HashSet::new());
}
