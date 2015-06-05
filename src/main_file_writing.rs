use std::io::prelude::*;
use std::fs::File;
use std::io;

fn log_something(filename: &'static str, string: &'static [u8; 12]) -> io::Result<()> {
    let mut f = try!(File::create(filename));
    try!(f.write_all(string));
    Ok(())
}

fn main() {
    match log_something("log.txt", b"ITS ALIVE!!!") {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file.")
    }
}
