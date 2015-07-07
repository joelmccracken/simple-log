extern crate clap;
use clap::App;

fn main() {
  let _ = App::new("fake").version("v1.0-beta").get_matches();
}
