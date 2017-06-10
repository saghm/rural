use error::{Error, Result};

use clap::Values;
use reqwest::{Client, Method, Response, Url};
use reqwest::header::Headers;
use regex::{Captures, Regex};
use serde_json;

type Json = serde_json::Map<String, serde_json::Value>;

pub struct Request<'a> {
    url: &'a str,
    json: &'a Json,
    headers: &'a Headers,
    form: bool,
}

impl<'a> Request<'a> {
    pub fn new(url: &str, form: bool) -> Result<RequestBuilder> {
        Ok(RequestBuilder {
               url: Url::parse(url).map_err(Error::from)?,
               json: Json::new(),
               headers: Headers::new(),
               form: form,
           })
    }

    pub fn send(&self, method: &str, client: &Client) -> Result<Response> {
        let mut builder = match method {
            "delete" => client.request(Method::Delete, self.url),
            "get" => client.get(self.url),
            "head" => client.request(Method::Head, self.url),
            "options" => client.request(Method::Options, self.url),
            "patch" => client.request(Method::Patch, self.url),
            "post" => client.post(self.url),
            "put" => client.request(Method::Put, self.url),

            // clap shouldn't allow invalid values, so this must be a bug.
            _ => {
                panic!("An unexpected error occured! Please file an issue with the exact command \
                        you ran here: https://github.com/saghm/rural/issues/new")
            }
        };

        if self.form {
            builder = builder.form(self.json);
        } else {
            builder = builder.json(self.json);
        }

        builder.headers(self.headers.clone()).send().map_err(Error::from)
    }
}

pub struct RequestBuilder {
    url: Url,
    json: Json,
    form: bool,
    headers: Headers,
}

impl RequestBuilder {
    pub fn add_params(&mut self, values: Option<Values>) -> Result<&mut Self> {
        if let Some(vals) = values {
            for param in vals {
                self.add_param(param)?;
            }
        }

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
            } else if let Some(header_pair) = get_header(param) {
                self.headers.set_raw(String::from(&header_pair[1]),
                                     vec![(&header_pair[2]).as_bytes().to_vec()]);
            } else if let Some(body_pair) = get_body_param(param) {
                self.json.insert(String::from(&body_pair[1]),
                                 serde_json::Value::String(String::from(&body_pair[2])));
            } else {
                return Err(Error::argument_error(param));
            }
        }

        Ok(self)
    }

    pub fn build(&self) -> Request {
        Request {
            url: self.url.as_str(),
            json: &self.json,
            headers: &self.headers,
            form: self.form,
        }
    }
}

fn get_body_param(text: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(.+)=(.+)").unwrap();
    }

    RE.captures(text)
}

fn get_header(text: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(.*):(.+)").unwrap();
    }

    RE.captures(text)
}

fn get_json_param(text: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(.+):=(.+)").unwrap();
    }

    RE.captures(text)
}

fn get_query_param(text: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(.*)==(.+)").unwrap();
    }

    RE.captures(text)
}

#[cfg(test)]
mod tests {
    use super::Request;

    use std::collections::HashMap;
    use std::io::Read;

    use reqwest::{Client, Method, StatusCode};
    use reqwest::header::Allow;
    use serde_json;

    lazy_static!{
        static ref CLIENT: Client = Client::new().unwrap();
    }

    #[test]
    fn simple_get_http() {
        let res = Request::new("http://httpbin.org/status/200", false)
            .unwrap()
            .build()
            .send("get", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);
    }

    #[test]
    fn simple_get_https() {
        let res = Request::new("https://httpbin.org/status/200", false)
            .unwrap()
            .build()
            .send("get", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);
    }

    #[test]
    fn get_querystring_params() {
        let mut res = Request::new("http://httpbin.org/response-headers?bass=john&drums=keith",
                                   false)
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
    fn get_manual_params() {
        let mut res = Request::new("http://httpbin.org/response-headers", false)
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
        let res = Request::new("http://httpbin.org/status/200", false)
            .unwrap()
            .build()
            .send("post", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);
    }

    #[test]
    fn simple_post_https() {
        let res = Request::new("https://httpbin.org/status/200", false)
            .unwrap()
            .build()
            .send("post", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);
    }

    #[test]
    fn simple_headers() {
        let mut res = Request::new("http://httpbin.org/headers", false)
            .unwrap()
            .add_param("bass:john")
            .unwrap()
            .add_param("drums:keith")
            .unwrap()
            .build()
            .send("get", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let json: HashMap<String, serde_json::Value> = serde_json::from_str(&buf).unwrap();
        let headers = json["headers"].as_object().unwrap();

        assert_eq!(headers["Bass"].as_str(), Some("john"));
        assert_eq!(headers["Drums"].as_str(), Some("keith"));
    }

    #[test]
    fn post_json() {
        let mut res = Request::new("http://httpbin.org/post", false)
            .unwrap()
            .add_param("bass=john")
            .unwrap()
            .add_param("drums=keith")
            .unwrap()
            .add_param("others:=[\"pete\", \"roger\"]")
            .unwrap()
            .add_param("band==the who")
            .unwrap()
            .add_param("song==bargain")
            .unwrap()
            .add_param("keyboard:the rabbit")
            .unwrap()
            .add_param("keyboard-also:pete")
            .unwrap()
            .build()
            .send("post", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let outer_json: HashMap<String, serde_json::Value> = serde_json::from_str(&buf).unwrap();

        let args = outer_json["args"].as_object().unwrap();
        assert_eq!(args["band"].as_str(), Some("the who"));
        assert_eq!(args["song"].as_str(), Some("bargain"));

        let inner_json = outer_json["json"].as_object().unwrap();
        assert_eq!(inner_json["bass"].as_str(), Some("john"));
        assert_eq!(inner_json["drums"].as_str(), Some("keith"));

        let others = inner_json["others"].as_array().unwrap();
        assert_eq!(others.len(), 2);
        assert_eq!(others[0].as_str(), Some("pete"));
        assert_eq!(others[1].as_str(), Some("roger"));

        let headers = outer_json["headers"].as_object().unwrap();
        assert_eq!(headers["Keyboard"].as_str(), Some("the rabbit"));
        assert_eq!(headers["Keyboard-Also"].as_str(), Some("pete"));
    }

    #[test]
    fn post_form() {
        let mut res = Request::new("http://httpbin.org/post", true)
            .unwrap()
            .add_param("bass=john")
            .unwrap()
            .add_param("drums=keith")
            .unwrap()
            .add_param("band==the who")
            .unwrap()
            .add_param("song==bargain")
            .unwrap()
            .add_param("keyboard:the rabbit")
            .unwrap()
            .add_param("keyboard-also:pete")
            .unwrap()
            .build()
            .send("post", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let outer_json: HashMap<String, serde_json::Value> = serde_json::from_str(&buf).unwrap();

        let args = outer_json["args"].as_object().unwrap();
        assert_eq!(args["band"].as_str(), Some("the who"));
        assert_eq!(args["song"].as_str(), Some("bargain"));

        let inner_json = outer_json["form"].as_object().unwrap();
        assert_eq!(inner_json["bass"].as_str(), Some("john"));
        assert_eq!(inner_json["drums"].as_str(), Some("keith"));

        let headers = outer_json["headers"].as_object().unwrap();
        assert_eq!(headers["Keyboard"].as_str(), Some("the rabbit"));
        assert_eq!(headers["Keyboard-Also"].as_str(), Some("pete"));
    }

    #[test]
    fn delete_json() {
        let mut res = Request::new("http://httpbin.org/delete", false)
            .unwrap()
            .add_param("bass=john")
            .unwrap()
            .add_param("drums=keith")
            .unwrap()
            .add_param("others:=[\"pete\", \"roger\"]")
            .unwrap()
            .add_param("band==the who")
            .unwrap()
            .add_param("song==bargain")
            .unwrap()
            .add_param("keyboard:the rabbit")
            .unwrap()
            .add_param("keyboard-also:pete")
            .unwrap()
            .build()
            .send("delete", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let outer_json: HashMap<String, serde_json::Value> = serde_json::from_str(&buf).unwrap();

        let args = outer_json["args"].as_object().unwrap();
        assert_eq!(args["band"].as_str(), Some("the who"));
        assert_eq!(args["song"].as_str(), Some("bargain"));

        let inner_json = outer_json["json"].as_object().unwrap();
        assert_eq!(inner_json["bass"].as_str(), Some("john"));
        assert_eq!(inner_json["drums"].as_str(), Some("keith"));

        let others = inner_json["others"].as_array().unwrap();
        assert_eq!(others.len(), 2);
        assert_eq!(others[0].as_str(), Some("pete"));
        assert_eq!(others[1].as_str(), Some("roger"));

        let headers = outer_json["headers"].as_object().unwrap();
        assert_eq!(headers["Keyboard"].as_str(), Some("the rabbit"));
        assert_eq!(headers["Keyboard-Also"].as_str(), Some("pete"));
    }

    #[test]
    fn delete_form() {
        let mut res = Request::new("http://httpbin.org/delete", true)
            .unwrap()
            .add_param("bass=john")
            .unwrap()
            .add_param("drums=keith")
            .unwrap()
            .add_param("band==the who")
            .unwrap()
            .add_param("song==bargain")
            .unwrap()
            .add_param("keyboard:the rabbit")
            .unwrap()
            .add_param("keyboard-also:pete")
            .unwrap()
            .build()
            .send("delete", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let outer_json: HashMap<String, serde_json::Value> = serde_json::from_str(&buf).unwrap();

        let args = outer_json["args"].as_object().unwrap();
        assert_eq!(args["band"].as_str(), Some("the who"));
        assert_eq!(args["song"].as_str(), Some("bargain"));

        let inner_json = outer_json["form"].as_object().unwrap();
        assert_eq!(inner_json["bass"].as_str(), Some("john"));
        assert_eq!(inner_json["drums"].as_str(), Some("keith"));

        let headers = outer_json["headers"].as_object().unwrap();
        assert_eq!(headers["Keyboard"].as_str(), Some("the rabbit"));
        assert_eq!(headers["Keyboard-Also"].as_str(), Some("pete"));
    }

    #[test]
    fn put_json() {
        let mut res = Request::new("http://httpbin.org/put", false)
            .unwrap()
            .add_param("bass=john")
            .unwrap()
            .add_param("drums=keith")
            .unwrap()
            .add_param("others:=[\"pete\", \"roger\"]")
            .unwrap()
            .add_param("band==the who")
            .unwrap()
            .add_param("song==bargain")
            .unwrap()
            .add_param("keyboard:the rabbit")
            .unwrap()
            .add_param("keyboard-also:pete")
            .unwrap()
            .build()
            .send("put", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let outer_json: HashMap<String, serde_json::Value> = serde_json::from_str(&buf).unwrap();

        let args = outer_json["args"].as_object().unwrap();
        assert_eq!(args["band"].as_str(), Some("the who"));
        assert_eq!(args["song"].as_str(), Some("bargain"));

        let inner_json = outer_json["json"].as_object().unwrap();
        assert_eq!(inner_json["bass"].as_str(), Some("john"));
        assert_eq!(inner_json["drums"].as_str(), Some("keith"));

        let others = inner_json["others"].as_array().unwrap();
        assert_eq!(others.len(), 2);
        assert_eq!(others[0].as_str(), Some("pete"));
        assert_eq!(others[1].as_str(), Some("roger"));

        let headers = outer_json["headers"].as_object().unwrap();
        assert_eq!(headers["Keyboard"].as_str(), Some("the rabbit"));
        assert_eq!(headers["Keyboard-Also"].as_str(), Some("pete"));
    }

    #[test]
    fn put_form() {
        let mut res = Request::new("http://httpbin.org/put", true)
            .unwrap()
            .add_param("bass=john")
            .unwrap()
            .add_param("drums=keith")
            .unwrap()
            .add_param("band==the who")
            .unwrap()
            .add_param("song==bargain")
            .unwrap()
            .add_param("keyboard:the rabbit")
            .unwrap()
            .add_param("keyboard-also:pete")
            .unwrap()
            .build()
            .send("put", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let outer_json: HashMap<String, serde_json::Value> = serde_json::from_str(&buf).unwrap();

        let args = outer_json["args"].as_object().unwrap();
        assert_eq!(args["band"].as_str(), Some("the who"));
        assert_eq!(args["song"].as_str(), Some("bargain"));

        let inner_json = outer_json["form"].as_object().unwrap();
        assert_eq!(inner_json["bass"].as_str(), Some("john"));
        assert_eq!(inner_json["drums"].as_str(), Some("keith"));

        let headers = outer_json["headers"].as_object().unwrap();
        assert_eq!(headers["Keyboard"].as_str(), Some("the rabbit"));
        assert_eq!(headers["Keyboard-Also"].as_str(), Some("pete"));
    }


    #[test]
    fn patch_json() {
        let mut res = Request::new("http://httpbin.org/patch", false)
            .unwrap()
            .add_param("bass=john")
            .unwrap()
            .add_param("drums=keith")
            .unwrap()
            .add_param("others:=[\"pete\", \"roger\"]")
            .unwrap()
            .add_param("band==the who")
            .unwrap()
            .add_param("song==bargain")
            .unwrap()
            .add_param("keyboard:the rabbit")
            .unwrap()
            .add_param("keyboard-also:pete")
            .unwrap()
            .build()
            .send("patch", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let outer_json: HashMap<String, serde_json::Value> = serde_json::from_str(&buf).unwrap();

        let args = outer_json["args"].as_object().unwrap();
        assert_eq!(args["band"].as_str(), Some("the who"));
        assert_eq!(args["song"].as_str(), Some("bargain"));

        let inner_json = outer_json["json"].as_object().unwrap();
        assert_eq!(inner_json["bass"].as_str(), Some("john"));
        assert_eq!(inner_json["drums"].as_str(), Some("keith"));

        let others = inner_json["others"].as_array().unwrap();
        assert_eq!(others.len(), 2);
        assert_eq!(others[0].as_str(), Some("pete"));
        assert_eq!(others[1].as_str(), Some("roger"));

        let headers = outer_json["headers"].as_object().unwrap();
        assert_eq!(headers["Keyboard"].as_str(), Some("the rabbit"));
        assert_eq!(headers["Keyboard-Also"].as_str(), Some("pete"));
    }

    #[test]
    fn patch_form() {
        let mut res = Request::new("http://httpbin.org/patch", true)
            .unwrap()
            .add_param("bass=john")
            .unwrap()
            .add_param("drums=keith")
            .unwrap()
            .add_param("band==the who")
            .unwrap()
            .add_param("song==bargain")
            .unwrap()
            .add_param("keyboard:the rabbit")
            .unwrap()
            .add_param("keyboard-also:pete")
            .unwrap()
            .build()
            .send("patch", &CLIENT)
            .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        let outer_json: HashMap<String, serde_json::Value> = serde_json::from_str(&buf).unwrap();

        let args = outer_json["args"].as_object().unwrap();
        assert_eq!(args["band"].as_str(), Some("the who"));
        assert_eq!(args["song"].as_str(), Some("bargain"));

        let inner_json = outer_json["form"].as_object().unwrap();
        assert_eq!(inner_json["bass"].as_str(), Some("john"));
        assert_eq!(inner_json["drums"].as_str(), Some("keith"));

        let headers = outer_json["headers"].as_object().unwrap();
        assert_eq!(headers["Keyboard"].as_str(), Some("the rabbit"));
        assert_eq!(headers["Keyboard-Also"].as_str(), Some("pete"));
    }

    #[test]
    fn head() {
        let mut res = Request::new("http://httpbin.org/response-headers?bass=john&drums=keith",
                                   false)
                .unwrap()
                .build()
                .send("head", &CLIENT)
                .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        assert!(buf.is_empty());
    }

    #[test]
    fn options() {
        let mut res = Request::new("http://httpbin.org/response-headers?bass=john&drums=keith",
                                   false)
                .unwrap()
                .build()
                .send("options", &CLIENT)
                .unwrap();

        assert_eq!(*res.status(), StatusCode::Ok);

        let mut buf = String::new();
        let _ = res.read_to_string(&mut buf).unwrap();
        assert!(buf.is_empty());

        let ref allowed_methods = res.headers().get::<Allow>().unwrap().0;
        assert_eq!(allowed_methods.len(), 4);
        assert!(allowed_methods.contains(&Method::Get));
        assert!(allowed_methods.contains(&Method::Head));
        assert!(allowed_methods.contains(&Method::Options));
        assert!(allowed_methods.contains(&Method::Post));
    }
}

