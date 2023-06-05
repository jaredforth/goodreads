[![Build Status](https://travis-ci.com/jaredforth/goodreads.svg?token=mH2pScYxqRkBEzpBQAu6&branch=main)](https://travis-ci.com/jaredforth/goodreads)
[![Build status](https://ci.appveyor.com/api/projects/status/w75cp0q4qr0hngf8?svg=true)](https://ci.appveyor.com/project/jaredforth/goodreads)
[![Crate](https://img.shields.io/crates/v/goodreads.svg)](https://crates.io/crates/goodreads)
[![API](https://docs.rs/goodreads/badge.svg)](https://docs.rs/goodreads)
![Crates.io](https://img.shields.io/crates/d/goodreads)

# goodreads

A library for deserializing a Goodreads library export.

Documentation:
-   [API Reference](https://docs.rs/goodreads)


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
goodreads = "0.1"
```

## Examples

An example for converting a Goodreads CSV export into a a `Book` is in the
`examples` directory. It can be run using 
```sh
cargo run --example goodreads_to_rust
```

## License

**goodreads** is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.
