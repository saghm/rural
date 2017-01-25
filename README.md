[![crates.io](https://img.shields.io/crates/v/rural.svg)](https://crates.io/crates/rural) [![Build Status](https://travis-ci.org/saghm/rural.svg)](https://travis-ci.org/saghm/rural)

# rural - User-friendly command-line HTTP tool in Rust

## Why rural?

Most developers tend to use `curl` when they need to make HTTP requests from the command line. In terms of speed and breadth of features, curl pretty much has it all. However, in terms of the command-line API, curl can sometimes leave something to be desired. For example, the `-G` or `--get` flag can be used to make a GET request, but there is no `--post` option, and `-P` is used to instruct the server to connect to an open FTP port instead of sending the reponse back directly.

One alternative to using `curl` for command-line HTTP requests is HTTPie. HTTPie has an excellent command-line API, including some superb syntatic sugar for various parameter types and headers. However, being written in Python, HTTPie is quite a bit slower than curl.

Rural aims to provide the ease-of-use of HTTPie at a speed comparable to that of curl. To that end, rural purposely implements a similar API to HTTPie, including the syntatic sugar for parameters and headers.

## Status

Rural currently implements most of the common features needed for making HTTP requests in the command line. However, many less common features have not been implemented. (A brief look at `man curl` shows just how much functionality there is to include). If there's a feature you'd like to be implemented, feel free to [open an issue](https://github.com/saghm/rural/issues), or even better, a [pull request](https://github.com/saghm/rural/pulls)! Contributions are always quite welcome.

To date, no specific optimization has been done on rural. From a set of extremely unscientific tests, rural currently runs at the same order of magnitude as curl in terms of speed (which is an order of magnitude faster than HTTPie).

## Installation

If you don't already have Rust installed, you can get the Rust toolchain installer [here](https://rustup.rs/).

Once Rust is installed, run `cargo install rural`. Make sure you add `$HOME/.cargo/bin` to your PATH if you instructed rustup not to do so itself.

## Usage

### Method types

Rural currently supports making GET, POST, PUT, DELETE, HEAD, OPTIONS, and PATCH requests. To make a request, invoke rural with the request method (in lowercase) and the URL:

```sh
rural get http://example.com
rural post http://example.com
rural put http://example.com
rural delete http://example.com
rural head http://example.com
rural options http://example.com
rural patch http://example.com
```

### HTTPS

Rural requires OpenSSL to make HTTPS requests:

```sh
rural get https://example.com
```

### Output

#### Sections

To see the response headers instead of the body, use the `--headers` flag (`-d` for short):

```sh
rural --headers get http://example.com
rural -d get http://example.com
```

Note that whenever the headers are printed out, the HTTP version number and response status code are printed by default. To suppress this extra output and just get the headers, add the `--suppress-info` flag as well (`-s` for short):

```sh
rural --headers --suppress-info get http://example.com
rural -ds get http://example.com
```

To print both the reponse headers and the body, use the `--both` flag (`-b` for short):

```sh
rural --both get http://example.com
rural -b get http://example.com
```

`--suppress-info` works for `--both` as well:

```sh
rural --both --suppress-info get http://example.com
rural -bs get http://example.com
```

#### Output file

To save the response body to a file, use the `--out` argument with the desired output file name (`-o` for short):

```sh
rural get http://example.com --out output.json
rural get http://example.com -o output.html
```

#### Colors

By default, rural will colorize the response headers, the HTTP info string, and any JSON in the response body. To suppress this, use the `--no-color` flag (`-n` for short):

```sh
rural --no-color http://example.com
rural -n http://example.com
```

NOTE: The library used to colorize the JSON uses ANSI color escape sequences, which will *not* work correctly on Windows. Because of this, rural disables colorized output on Windows.

### Parameters

Rural supports supplying GET parameters in the querystring or by using the syntax `key==value`:

```sh
rural get 'http://example.com?bass=john&drums=keith'
rural get http://example.com bass==john drums==keith
```

To supply body parameters, use the syntax `key=value`:

```sh
rural post http://example.com bass=john drums=keith
```

Rural defaults to sending body parameters as JSON. To instead send them as form parameters, use `--form` (`-f` for short):

```sh
rural --form post http://example.com bass=john drums=keith
rural -f post http://example.com bass=john drums=keith
```

Body parameters can also be sent using literal JSON rather than plaintext. To do this, use the syntax `key:=value` (note that JSON values will generally have to be wrapped in quotes due to how most shells interpret some of the characters, as shown below):

```sh
rural post http://example.com who:='{ "bass": "john", "drums": "keith", "others": ["pete", "roger"] }'
```

Note that body parameters can also be specified for other types of requests besides POST:

```sh
rural put http://example.com bass=john
rural delete http://example.com drums=keith
```

### Headers

HTTP headers (either standard or custom) can be provided using the syntax `name:value`:

```sh
rural get http://example.com bass:john drums:keith
```

## License

Rural is licensed under the MIT LICENSE.
