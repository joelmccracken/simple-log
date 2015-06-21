#[macro_use] extern crate nickel;
extern crate chrono;

use std::io::prelude::*;
use std::fs::{File,OpenOptions};
use std::io;
use chrono::{DateTime,Local};

use nickel::Nickel;

fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut file = try!(OpenOptions::new().
                        append(true).
                        write(true).
                        create(true).
                        open(filename));
    try!(file.write_all(bytes));
    Ok(())
}

fn log_time(filename: &'static str) -> io::Result<()> {
    let entry = formatted_time_entry();
    let bytes = entry.as_bytes();

    try!(record_entry_in_log(filename, &bytes));
    Ok(())
}

fn main2() {
    match log_time("log.txt") {
        Ok(..) => println!("File created!"),
        Err(e) => println!("Error: {}", e)
    }
}


fn say_hello() -> &'static str {
    "Hello dear world!"
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            say_hello()
        }
    });

    server.listen("127.0.0.1:6767");
}
