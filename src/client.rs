use error::{Error, Result};
use url_builder::UrlBuilder;

use std::io::Read;

use clap::ArgMatches;
use hyper;
use hyper::client::Response;

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
        let mut res = self.request()?;

        let output = if self.args.is_present("headers") {
            format!("{}", res.headers)
        } else {
            let mut buf = String::new();
            let _ = res.read_to_string(&mut buf)?;
            buf
        };

        Ok(output)
    }

    fn request(&self) -> Result<Response> {
        let url = self.build_url()?;

        match self.args.value_of("METHOD").unwrap() {
            "get" => self.http.get(&url).send().map_err(Error::from),
            "post" => self.http.post(&url).send().map_err(Error::from),

            // clap shouldn't allow invalid values, so this must be a bug.
            _ => {
                panic!("This shouldn't be possible! Please file an issue with the exact command \
                        you ran here: https://github.com/saghm/rural/issues/new")
            }
        }
    }

    fn build_url(&self) -> Result<String> {
        let url = self.args.value_of("URL").unwrap();

        if !self.args.is_present("params") {
            return Ok(String::from(url));
        }

        let mut builder = UrlBuilder::parse(url)?;
        builder.add_params(self.args.values_of("params").unwrap())?;

        Ok(builder.build())
    }
}
