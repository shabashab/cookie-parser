# Cookie Parser

A Rust library for parsing Cookie and Set-Cookie HTTP headers with support for all standard cookie attributes.

## Features

- Parse Cookie header strings into structured data
- Parse Set-Cookie header strings with full attribute support
- Support for cookie extensions

## Usage

Install using cargo:

```sh
cargo add cookie_parser
```

### Parsing Cookie Headers

```rust
use cookie_parser::{parse_cookie_string, CookiePair};

fn main() {
    let cookies = parse_cookie_string("session=abc123; user=john_doe")?;
    
    for cookie in cookies {
        println!("Name: {}, Value: {}", cookie.name, cookie.value);
    }
}
```

### Parsing Set-Cookie Headers

```rust
use cookie_parser::{parse_set_cookie, SetCookie};

fn main() {
    let set_cookie = parse_set_cookie(
        "session=abc123; HttpOnly; Secure; Path=/; Domain=example.com; Max-Age=3600"
    )?;
    
    println!("Cookie name: {}", set_cookie.pair.name);
    println!("Cookie value: {}", set_cookie.pair.value);
    println!("Is secure: {}", set_cookie.secure);
    println!("Is HTTP-only: {}", set_cookie.http_only);
    
    if let Some(domain) = set_cookie.domain {
        println!("Domain: {}", domain);
    }
}
```

## API Reference

### Structures

#### `CookiePair`

Represents a name-value pair in a cookie.

```rust
pub struct CookiePair {
    pub name: String,
    pub value: String,
}
```

#### `SetCookie`

Represents a complete Set-Cookie header with all possible attributes.

```rust
pub struct SetCookie {
    pub pair: CookiePair,
    pub secure: bool,
    pub http_only: bool,
    pub max_age: Option<String>,
    pub domain: Option<String>,
    pub expires: Option<String>,
    pub path: Option<String>,
    pub extensions: Vec<String>,
}
```

### Functions

#### `parse_cookie_string`

```rust
pub fn parse_cookie_string(input: &str) -> Result<Vec<CookiePair>, CookieParseError>
```

Parses a Cookie header string into a vector of cookie pairs.

#### `parse_set_cookie`

```rust
pub fn parse_set_cookie(input: &str) -> Result<SetCookie, CookieParseError>
```

Parses a Set-Cookie header string into a structured `SetCookie` object.

### Errors

The library uses a custom error type `CookieParseError` with the following variants:

- `ErrorCookieStringSyntax`: Invalid cookie string syntax
- `ErrorCookieStringEmpty`: Cookie string is empty

## License

MIT