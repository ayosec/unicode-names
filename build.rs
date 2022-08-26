//! Build script to download the `UnicodeData.txt` file, and generate the table
//! with the character names.

use std::env;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

use flate2::write::ZlibEncoder;
use flate2::Compression;
use twox_hash::XxHash64;

const SOURCE_URL: &str = "https://unicode.org/Public/14.0.0/ucd/UnicodeData.txt";

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // The file name contains a hash generated from the URL, so if the URL is
    // changed the file should be downloaded again.
    let ucd = {
        let mut hasher = XxHash64::with_seed(0);
        SOURCE_URL.len().hash(&mut hasher);
        SOURCE_URL.hash(&mut hasher);

        out_dir.join(format!("UnicodeData-{:x}.txt", hasher.finish()))
    };

    // Download UnicodeData.txt if it is missing.
    if !ucd.exists() {
        let mut writer = BufWriter::new(File::create(&ucd).expect("Create UnicodeData.txt file"));
        let response = ureq::get(SOURCE_URL)
            .call()
            .expect("Request for UnicodeData.txt");
        if !(200..300).contains(&response.status()) {
            panic!("Response with status {:?}", response.status_text());
        }

        // Limit download to 10M
        let mut reader = response.into_reader().take(10 * (1 << 20));
        io::copy(&mut reader, &mut writer).expect("Download UnicodeData.txt");
    }

    // Generate a table with the character name. Each entry in the table has the
    // following fields:
    //
    //  1 byte              Length, in bytes, of the entry.
    //  1 UTF char          Unicode character for the entry.
    //  Rest of the bytes   Character name.

    let table = BufWriter::new(File::create(out_dir.join("UnicodeTable.z")).unwrap());
    let mut table = ZlibEncoder::new(table, Compression::best());

    let ucd = BufReader::new(File::open(ucd).expect("Open UnicodeData.txt"));
    for line in ucd.lines() {
        let line = line.unwrap();
        let mut fields = line.split(';');
        let (code, name) = match (fields.next(), fields.next()) {
            (Some(code), Some(name)) => (code, name),
            _ => continue,
        };

        let code = u32::from_str_radix(code, 16).expect("Parse code as hexadecimal value");
        let chr_entry = match char::from_u32(code) {
            Some(c) => c,
            None => continue,
        };

        let entry_len = u8::try_from(chr_entry.len_utf8() + name.len()).unwrap();

        table.write_all(&[entry_len]).unwrap();
        write!(&mut table, "{chr_entry}{name}").unwrap();
    }

    table.finish().unwrap();

    // Only rerun if the build script is modified.
    println!("cargo:rerun-if-changed=build.rs");
}
