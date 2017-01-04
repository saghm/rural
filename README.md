[![crates.io](https://img.shields.io/crates/v/rural.svg)](https://crates.io/crates/rural) [![Build Status](https://travis-ci.org/saghm/rural.svg)](https://travis-ci.org/saghm/rural)

# rural - User-friendly command-line HTTP tool in Rust

## Installation

Assuming you have a reasonably recent version of Rust/Cargo installed, simply run `cargo install rural`.

## Usage

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

Rural requires OpenSSL to make HTTPS requests:

```sh
rural get https://example.com
```

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

HTTP headers (either standard or custom) can be provided using the syntax `name:value`:

```sh
rural get http://example.com bass:john drums:keith
```
