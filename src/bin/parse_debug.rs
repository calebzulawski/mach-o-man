use mach_o_man::{Error, Header};
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Error> {
    for file in std::env::args().skip(1) {
        let f = File::open(&file)?;
        let mut reader = BufReader::new(f);
        let header = Header::from_reader(&mut reader);
        println!("{}:\n{:?}", file, header);
    }
    Ok(())
}
