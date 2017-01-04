use error::Result;
use request::Request;

use std::io::Read;

use clap::ArgMatches;
use reqwest;

pub struct Client<'a> {
    args: ArgMatches<'a>,
    http: reqwest::Client,
}

impl<'a> Client<'a> {
    pub fn new(args: ArgMatches<'a>) -> Result<Self> {
        Ok(Client {
            args: args,
            http: reqwest::Client::new()?,
        })
    }

    // Unwraps are okay because clap guarantees that the required arguments are present.
    pub fn execute(&self) -> Result<String> {
        let method = self.args.value_of("METHOD").unwrap();
        let url = self.args.value_of("URL").unwrap();
        let params = self.args.values_of("PARAM");
        let form = self.args.is_present("form");

        let mut res = Request::new(url, form)?
            .add_params(params)?
            .build()
            .send(method, &self.http)?;

        let mut buf = String::new();

        if self.args.is_present("headers") || self.args.is_present("both") ||
           self.args.value_of("METHOD").unwrap() == "head" {
            buf.push_str(&format!("{} {}\n\n{}", res.version(), res.status(), res.headers()));
        }

        if !self.args.is_present("headers") {
            if !buf.is_empty() {
                buf.push('\n')
            }
            
            let _ = res.read_to_string(&mut buf)?;
        }

        Ok(buf)
    }
}
