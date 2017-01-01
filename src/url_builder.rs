use error::{Error, Result};

use clap::Values;
use hyper::{self, Url};
use regex::{Captures, Regex};

pub struct UrlBuilder {
    url: Url,
}

impl UrlBuilder {
    pub fn parse(url: &str) -> Result<Self> {
        Ok(UrlBuilder { url: Url::parse(url).map_err(hyper::Error::from)? })
    }

    pub fn add_params(&mut self, values: Values) -> Result<()> {
        for param in values {
            self.add_param(param)?;
        }

        Ok(())
    }

    fn add_param(&mut self, param: &str) -> Result<()> {
        let mut querystring = self.url.query_pairs_mut();

        if let Some(captures) = get_query_param(param) {
            querystring.append_pair(&captures[1], &captures[2]);
            return Ok(());
        }

        if let Some(body_pair) = get_body_param(param) {
            // TODO: Implement body parameter functionality
            return Ok(());
        }

        Err(Error::argument_error(param))
    }

    pub fn build(self) -> String {
        self.url.into_string()
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
