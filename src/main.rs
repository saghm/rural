extern crate clap;
extern crate hyper;

mod error;

use error::Result;

use std::io::Read;

use clap::{Arg, App};
use hyper::Client;

fn run() -> Result<()> {
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
    let url = matches.value_of("url").unwrap();

    let mut res = client.get(url).send()?;
    let mut buf = String::new();
    let _ = res.read_to_string(&mut buf)?;
    println!("{}", buf);

    Ok(())
}


fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}
