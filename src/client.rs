use error::Result;
use request::Request;

use std::fs::OpenOptions;
use std::io::{Read, Write};

use clap::ArgMatches;
use colored::Colorize;
use json_color::Colorizer;

pub struct Client<'a> {
    args: ArgMatches<'a>,
    http: ::reqwest::Client,
    colorizer: Colorizer,
}

impl<'a> Client<'a> {
    pub fn new(args: ArgMatches<'a>) -> Self {
        Client {
            args,
            http: ::reqwest::Client::new(),
            colorizer: Colorizer::arbitrary(),
        }
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

        if self.args.is_present("headers")
            || self.args.is_present("both")
            || self.args.is_present("out")
            || self.args.value_of("METHOD").unwrap() == "head"
        {
            if !self.args.is_present("suppress-info") {
                let mut status_key = "Status".to_string();
                let mut status_val = format!("{}", res.status());

                if !cfg!(target_os = "windows") && !self.args.is_present("no-color") {
                    status_key = status_key.blue().to_string();
                    status_val = status_val.yellow().to_string();
                }

                buf.push_str(&format!("{}: {}\n", status_key, status_val));
            }

            if cfg!(target_os = "windows") || self.args.is_present("no-color") {
                buf.push_str(&format!("{}", res.headers()));
            } else {
                for (i, header) in res.headers().iter().enumerate() {
                    if i != 0 {
                        buf.push('\n');
                    }

                    buf.push_str(&header.name().cyan().to_string());
                    buf.push_str(": ");
                    buf.push_str(&header.value_string().yellow().to_string());
                }
            }
        }

        if let Some(file_name) = self.args.value_of("out") {
            let mut body = String::new();
            let _ = res.read_to_string(&mut body)?;

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(file_name)?;
            write!(file, "{}", body)?;
        } else if !self.args.is_present("headers") {
            if !buf.is_empty() {
                buf.push_str("\n\n");
            }

            let mut bytes = Vec::new();
            let _ = res.read_to_end(&mut bytes)?;
            let mut body = String::from_utf8_lossy(&bytes).into_owned();

            if !cfg!(target_os = "windows") && !self.args.is_present("no-color") {
                if let Ok(colored_json) = self.colorizer.colorize_json_str(&body) {
                    body = colored_json;
                }
            }

            buf.push_str(&body);
        }

        Ok(buf)
    }
}
