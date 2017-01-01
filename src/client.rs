use error::{Error, Result};

use std::io::Read;

use clap::ArgMatches;
use hyper::{self, Url};
use hyper::client::Response;
use regex::{Captures, Regex};

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
        let url_str = self.args.value_of("URL").unwrap();

        if !self.args.is_present("params") {
            return Ok(String::from(url_str));
        }

        let mut url = Url::parse(url_str).map_err(hyper::Error::from)?;

        {
            let mut querystring = url.query_pairs_mut();

            for param in self.args.values_of("params").unwrap() {
                if let Some(captures) = get_query_param(param) {
                    querystring.append_pair(&captures[1], &captures[2]);
                    continue;
                }

                if let Some(body_pair) = get_body_param(param) {
                    // TODO: Implement body parameter functionality
                    continue;
                }

                return Err(Error::argument_error(param));
            }
        }

        Ok(url.into_string())
    }
}

fn get_query_param(text: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([^=]+)=([^=]+)").unwrap();
    }

    RE.captures(text)
}

fn get_body_param(text: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([^=]+)==([^=]+)").unwrap();
    }

    RE.captures(text).map(|c| format!("{}={}", &c[1], &c[2]))
}
