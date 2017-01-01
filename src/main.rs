extern crate clap;
extern crate hyper;

mod client;
mod error;

use client::Client;

use clap::{Arg, App};

fn main() {
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

    let client = Client::new(matches);

    match client.execute() {
        Ok(output) => println!("{}", output),
        Err(err) => println!("{}", err),
    }
}
