extern crate clap;
use clap::{App,Arg};

fn main() {
    let matches = App::new("simple-log").version("v0.0.1")
        .arg(Arg::with_name("LOG FILE")
             .short("l")
             .long("logfile")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("AUTH TOKEN")
             .short("t")
             .long("token")
             .takes_value(true))
        .get_matches();

    let logfile_path = matches.value_of("LOG FILE").unwrap();
    let auth_token   = matches.value_of("AUTH TOKEN");
}

