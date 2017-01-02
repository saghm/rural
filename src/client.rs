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

    pub fn execute(&self) -> Result<String> {
        let method = self.args.value_of("METHOD").unwrap();
        let url = self.args.value_of("URL").unwrap();
        let params = self.args.values_of("PARAM");
        let form = self.args.is_present("form");

        let mut res = Request::new(url, form)?
            .add_params(params)?
            .build()
            .send(method, &self.http)?;

        let output = if self.args.is_present("headers") {
            format!("{}", res.headers())
        } else {
            let mut buf = String::new();
            let _ = res.read_to_string(&mut buf)?;
            buf
        };

        Ok(output)
    }
}
