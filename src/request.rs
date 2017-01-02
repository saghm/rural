use error::{Error, Result};

use clap::Values;
use reqwest::{self, Url};
use reqwest::{Client, Response};
use regex::{Captures, Regex};
use serde_json;

type Json = serde_json::Map<String, serde_json::Value>;

pub struct Request<'a> {
    url: &'a str,
    body: &'a [(String, String)],
    json: &'a Json,
}

impl<'a> Request<'a> {
    pub fn new(url: &str) -> Result<RequestBuilder> {
        Ok(RequestBuilder {
            url: Url::parse(url).map_err(reqwest::Error::from)?,
            body: Vec::new(),
            json: Json::new(),
        })
    }

    pub fn send(&self, method: &str, client: &Client) -> Result<Response> {
        let mut builder = match method {
            "get" => client.get(self.url),
            "post" => client.post(self.url),

            // clap shouldn't allow invalid values, so this must be a bug.
            _ => {
                panic!("This shouldn't be possible! Please file an issue with the exact command \
                        you ran here: https://github.com/saghm/rural/issues/new")
            }
        };

        builder = builder.form(&self.body);

        if !self.json.is_empty() {
            builder = builder.json(self.json);
        }

        builder.send().map_err(Error::from)
    }
}

pub struct RequestBuilder {
    url: Url,
    body: Vec<(String, String)>,
    json: Json,
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

            if let Some(json_pair) = get_json_param(param) {
                let json_value = serde_json::from_str(&json_pair[2])?;
                self.json.insert(String::from(&json_pair[1]), json_value);
            } else if let Some(query_pair) = get_query_param(param) {
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
            json: &self.json,
        }
    }
}

fn get_body_param(text: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(.+)=(.+)").unwrap();
    }

    RE.captures(text)
}

fn get_query_param(text: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(.*)==(.+)").unwrap();
    }

    RE.captures(text)
}

fn get_json_param(text: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(.+):=(.+)").unwrap();
    }

    RE.captures(text)
}


#[cfg(test)]
mod tests {
    use super::Request;

    use std::collections::HashMap;
    use std::io::Read;


    use reqwest::{Client, StatusCode};
    use serde_json;

    lazy_static!{
        static ref CLIENT: Client = Client::new().unwrap();
    }

    #[test]
    fn simple_get_http() {
        let res = Request::new("http://httpbin.org/status/200")
            .unwrap()
            .build()
            .send("get", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);
    }

    #[test]
    fn simple_get_https() {
        let res = Request::new("https://httpbin.org/status/200")
            .unwrap()
            .build()
            .send("get", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);
    }

    #[test]
    fn get_querystring_params_http() {
        let mut res = Request::new("http://httpbin.org/response-headers?bass=john&drums=keith")
            .unwrap()
            .build()
            .send("get", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let json: HashMap<String, String> = serde_json::from_str(&buf).unwrap();

        assert_eq!(json["bass"], "john");
        assert_eq!(json["drums"], "keith");
    }

    #[test]
    fn get_manual_params_http() {
        let mut res = Request::new("http://httpbin.org/response-headers")
            .unwrap()
            .add_param("bass==john")
            .unwrap()
            .add_param("drums==keith")
            .unwrap()
            .build()
            .send("get", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let json: HashMap<String, String> = serde_json::from_str(&buf).unwrap();

        assert_eq!(json["bass"], "john");
        assert_eq!(json["drums"], "keith");
    }

    #[test]
    fn simple_post_http() {
        let res = Request::new("http://httpbin.org/status/200")
            .unwrap()
            .build()
            .send("post", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);
    }

    #[test]
    fn simple_post_https() {
        let res = Request::new("https://httpbin.org/status/200")
            .unwrap()
            .build()
            .send("post", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);
    }

    #[test]
    fn post_params_http() {
        let mut res = Request::new("http://httpbin.org/post")
            .unwrap()
            .add_param("bass=john")
            .unwrap()
            .add_param("drums=keith")
            .unwrap()
            .build()
            .send("post", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let outer_json: HashMap<String, serde_json::Value> = serde_json::from_str(&buf).unwrap();
        let inner_json = outer_json["form"].as_object().unwrap();

        assert_eq!(inner_json["bass"].as_str(), Some("john"));
        assert_eq!(inner_json["drums"].as_str(), Some("keith"));
    }
}
