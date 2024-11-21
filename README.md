# cookie_parser

## Overview

This package/CLI allows you to parse Cookie/Set-Cookie header contents into a more computer-readable struct. The parser is built on pest

Crate: [cookie_parser](https://crates.io/crates/cookie_parser)

## Usage

### Installation

```sh
cargo add cookie_parser
```

### Using as a library

In order to know more information about usage of this crate as a library, please, refer to [Doc](/doc.md) or [docs.rs](https://docs.rs/cookie_parser/latest/cookie_parser/)

### Using as a CLI

This library exposes a thin CLI.

```
Cookie/Set-Cookie Parser CLI

USAGE:
         cookie_parser <COMMAND>

COMMANDS:
        parse-cookie Parses Cookie header contents.
                Options:
                        --file,-f <FILE>     File with contents that should be parsed.

        parse-set-cookie Parses Set-Cookie header contents.
                Options:
                        --file,-f <FILE>     File with contents that should be parsed.

        credits Show credits.

        help    Show help.
```

You can find examples of input in `examples/` directory

## Contributing & development

Build the library:

```sh
make build
```

Run tests:

```sh
make test
```

For more makefile commands, refer to:

```sh
make help
```

## License

MIT License, 2024, Artem Tarasenko