# rural - User-friendly command-line HTTP tool in Rust

## Installation

Assuming you have a reasonably recent version of Rust/Cargo installed, simply run `cargo install rural`.

## Usage

Rural currently supports making GET and POST requests. To make a request, invoke rural with the request method (in lowercase) and the URL:

```sh
rural get http://example.com
rural post http://example.com
```

Rural requires OpenSSL to make HTTPS requests:

```sh
rural get https://example.com
```

To see the response headers returned, use the `--headers` flag (`-d` for short):

```sh
rural --headers get http://example.com
rural -d get http://example.com
```

Rural supports supplying GET parameters in the querystring or by using the syntax `key==value`:

```sh
rural get 'http://example.com?bass=john&drums=keith'
rural get http://example.com bass==john drums==keith
```

To supply POST parameters, use the syntax `key=value`:

```sh
rural post http://example.com bass=john drums=keith
```

Rural defaults to sending POST parameters as JSON. To instead send them as form parameters, use `--form` (`-f` for short):

```sh
rural --form post http://example.com bass=john drums=keith
rural -f post http://example.com bass=john drums=keith
```

POST parameters can also be sent using literal JSON rather than plaintext. To do this, use the syntax `key:=value` (note that JSON values will generally have to be wrapped in quotes due to how most shells interpret some of the characters, as shown below):

```sh
rural post http://example.com who:='{ "bass": "john", "drums": "keith", "others": ["pete", "roger"] }'
```

HTTP headers (either standard or custom) can be provided using the syntax `name:value`:

```sh
rural get http://example.com bass:john drums:keith
```
