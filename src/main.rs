extern crate chrono;

use std::io::prelude::*;
use std::fs::File;
use std::io;
use chrono::*;

fn log_time(filename: &'static str) -> io::Result<()> {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    let bytes = formatted.as_bytes();
    let mut f = try!(File::create(filename));
    try!(f.write_all(bytes));
    Ok(())
}

fn main() {
    match log_time("log.txt") {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file.")
    }
}
