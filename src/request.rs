use error::{Error, Result};

use clap::Values;
use reqwest::{self, Url};
use reqwest::{Client, Response};
use regex::{Captures, Regex};

pub struct Request<'a> {
    url: &'a str,
    body: &'a [(String, String)],
}

impl<'a> Request<'a> {
    pub fn new(url: &str) -> Result<RequestBuilder> {
        Ok(RequestBuilder {
            url: Url::parse(url).map_err(reqwest::Error::from)?,
            body: Vec::new(),
        })
    }

    pub fn send(&self, method: &str, client: &Client) -> Result<Response> {
        let builder = match method {
            "get" => client.get(self.url),
            "post" => client.post(self.url),

            // clap shouldn't allow invalid values, so this must be a bug.
            _ => {
                panic!("This shouldn't be possible! Please file an issue with the exact command \
                        you ran here: https://github.com/saghm/rural/issues/new")
            }
        };

        builder.form(&self.body).send().map_err(Error::from)
    }
}

pub struct RequestBuilder {
    url: Url,
    body: Vec<(String, String)>,
}

impl RequestBuilder {
    pub fn add_params(&mut self, values: Option<Values>) -> Result<&mut Self> {
        match values {
            Some(vals) => {
                for param in vals {
                    self.add_param(param)?;
                }
            }
            None => {}
        };

        Ok(self)
    }

    fn add_param(&mut self, param: &str) -> Result<&mut Self> {
        {
            let mut querystring = self.url.query_pairs_mut();

            if let Some(query_pair) = get_query_param(param) {
                querystring.append_pair(&query_pair[1], &query_pair[2]);
            } else if let Some(body_pair) = get_body_param(param) {
                self.body.push((String::from(&body_pair[1]), String::from(&body_pair[2])));
            } else {
                return Err(Error::argument_error(param));
            }
        }

        Ok(self)
    }

    pub fn build(&self) -> Request {
        Request {
            url: self.url.as_str(),
            body: &self.body,
        }
    }
}

fn get_query_param(text: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([^&=]+)=([^&=]+)").unwrap();
    }

    RE.captures(text)
}

fn get_body_param(text: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([^&=]+)==([^&=]+)").unwrap();
    }

    RE.captures(text)
}
