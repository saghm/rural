extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate serde_json;

mod client;
mod error;
mod request;

use client::Client;

use clap::{Arg, App};

// Shamelessly stolen from burntsushi (okay, maybe with a *little* shame).
macro_rules! eprintln {
    ($($tt:tt)*) => {{
        use std::io::Write;
        let _ = writeln!(&mut ::std::io::stderr(), $($tt)*);
    }}
}

fn main() {
    let matches = App::new("rural")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Saghm Rossi <saghmrossi@gmail.com>")
        .about("Command-line HTTP client")
        .arg(Arg::with_name("METHOD")
            .help("HTTP request method to use")
            .required(true)
            .index(1)
            .possible_values(&["get", "post"]))
        .arg(Arg::with_name("URL")
            .help("URL to request")
            .required(true)
            .index(2))
        .arg(Arg::with_name("PARAM")
            .help("querystring parameter (i.e `key=value`) or body parameter (i.e `key==value`")
            .index(3)
            .multiple(true))
        .arg(Arg::with_name("headers")
            .help("Print response headers instead of body")
            .short("d")
            .long("headers"))
        .arg(Arg::with_name("form")
            .help("Send POST data as a form rather than JSON")
            .short("f")
            .long("form"))
        .get_matches();

    let client = Client::new(matches);

    match client.and_then(|c| c.execute()) {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("{}", err),
    }
}
