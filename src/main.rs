extern crate clap;
extern crate hyper;

mod error;

use error::Result;

use std::io::Read;

use clap::{Arg, App};
use hyper::Client;

fn main() {
    let matches = App::new("rural")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Saghm Rossi <saghmrossi@gmail.com>")
        .about("Command-line HTTP client")
        .arg(Arg::with_name("url")
            .short("u")
            .long("url")
            .value_name("URL")
            .help("URL to request")
            .required(true))
        .get_matches();

    let client = Client::new();
    let mut res = client.get(matches.value_of("url").unwrap()).send().unwrap();
    let mut buf = String::new();
    let _ = res.read_to_string(&mut buf).unwrap();

    println!("{}", buf);
}
