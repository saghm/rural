use error::Result;

use std::io::Read;

use clap::ArgMatches;
use hyper;

pub struct Client<'a> {
    args: ArgMatches<'a>,
    http: hyper::Client,
}

impl<'a> Client<'a> {
    pub fn new(args: ArgMatches<'a>) -> Self {
        Client {
            args: args,
            http: hyper::Client::new(),
        }
    }

    pub fn execute(&self) -> Result<String> {
        let url = self.args.value_of("URL").unwrap();
        let mut res = self.http.get(url).send()?;

        let output = if self.args.is_present("headers") {
            format!("{}", res.headers)
        } else {
            let mut buf = String::new();
            let _ = res.read_to_string(&mut buf)?;
            buf
        };

        Ok(output)
    }
}
