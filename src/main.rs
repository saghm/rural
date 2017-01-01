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
        .arg(Arg::with_name("URL")
            .help("URL to request")
            .required(true)
            .index(1))
        .arg(Arg::with_name("headers")
            .short("d")
            .long("headers")
            .help("Print response headers instead of body"))
        .get_matches();

    let client = Client::new();
    let url = matches.value_of("URL").unwrap();

    let mut res = client.get(url).send()?;

    if matches.is_present("headers") {
        println!("{}", res.headers);
    } else {
        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf)?;
        println!("{}", buf);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}
