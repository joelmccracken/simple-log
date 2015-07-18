#![feature(no_std)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io;

#[macro_use]
extern crate nickel;
use nickel::Nickel;

extern crate chrono;
use chrono::{DateTime, Local};

extern crate clap;
use clap::{App, Arg};

fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_in_log(filename: String, bytes: &[u8]) -> io::Result<()> {
    let mut file =
        match OpenOptions::new().append(true).write(true).create(true).open(filename)
            {











            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(err) => {
                return ::std::result::Result::Err(::std::convert::From::from(err))
            }
        };
    match file.write_all(bytes) {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => {
            return ::std::result::Result::Err(::std::convert::From::from(err))
        }
    };
    Ok(())
}
fn log_time(filename: String) -> io::Result<String> {
    let entry = formatted_time_entry();
    {
        let bytes = entry.as_bytes();
        match record_entry_in_log(filename, &bytes) {
            ::std::result::Result::Ok(val) => val,
            ::std::result::Result::Err(err) => {
                return ::std::result::Result::Err(::std::convert::From::from(err))
            }
        };
    }
    Ok(entry)
}
fn do_log_time(logfile_path: String, auth_token: Option<String>) -> String {
    match log_time(logfile_path) {
        Ok(entry) =>
        ::std::fmt::format(::std::fmt::Arguments::new_v1({
                                                             static __STATIC_FMTSTR:
                                                                    &'static [&'static str]
                                                                    =
                                                                 &["Entry Logged: "];
                                                             __STATIC_FMTSTR
                                                         },
                                                         &match (&entry,) {
                                                              (__arg0,) =>
                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                           ::std::fmt::Display::fmt)],
                                                          })),
        Err(e) =>
        ::std::fmt::format(::std::fmt::Arguments::new_v1({
                                                             static __STATIC_FMTSTR:
                                                                    &'static [&'static str]
                                                                    =
                                                                 &["Error: "];
                                                             __STATIC_FMTSTR
                                                         },
                                                         &match (&e,) {
                                                              (__arg0,) =>
                                                              [::std::fmt::ArgumentV1::new(__arg0,
                                                                                           ::std::fmt::Display::fmt)],
                                                          })),
    }
}
fn main() {
    let matches =
        App::new("simple-log").version("v0.0.1").arg(Arg::with_name("LOG FILE").short("l").long("logfile").required(true).takes_value(true)).arg(Arg::with_name("AUTH TOKEN").short("t").long("token").takes_value(true)).get_matches();
    let logfile_path = matches.value_of("LOG FILE").unwrap().to_string();
    let auth_token = matches.value_of("AUTH TOKEN");
    let auth_token_string =
        match auth_token {
            Some(str) => Some(str.to_string()),
            None => None,
        };
    let thing = "hi there".to_string();
    let mut server = Nickel::new();
    server.utilize({
                       use nickel::HttpRouter;
                       let mut router = ::nickel::Router::new();
                       {
                           router.get("**",
                                      {
                                          use nickel::{MiddlewareResult,
                                                       Responder, Response,
                                                       Request};
                                          #[inline(always)]
                                          fn restrict<'a, R: Responder>(r: R, res: Response<'a>) -> MiddlewareResult<'a> {
                                              res.send(r)
                                          }
                                          #[inline(always)]
                                          fn restrict_closure<F>(f: F) -> F
                                              where F: for<'r, 'b, 'a>Fn(&'r mut Request<'b, 'a, 'b>, Response<'a>) ->
                                              MiddlewareResult<'a> + Send + Sync
                                          {
                                                  f
                                          }
                                          restrict_closure(move |_req, _res| {
                                              restrict({
                                                  do_log_time(thing,
                                                              Some(thing))
                                              }, _res)
                                          })
                                      });
                           router
                       }
    });
    server.listen("127.0.0.1:6767");
}