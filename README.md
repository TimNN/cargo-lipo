# `cargo lipo` [![Build Status](https://travis-ci.org/TimNN/cargo-lipo.svg?branch=master)](https://travis-ci.org/TimNN/cargo-lipo) [![Crates.io](https://img.shields.io/crates/v/cargo-lipo.svg)](https://crates.io/crates/cargo-lipo)

Provides a `cargo lipo` subcommand which automatically creates a universal library for use with your iOS application.

## Usage

From anywhere you would usually run `cargo` you can now run `cargo lipo` or `cargo lipo --release` to create a universal library for ios, which can be found in `$target/universal/{release|debug}/$lib_name.a`.

Make sure you have a library target in your `Cargo.toml` with a crate type of `staticlib`:

```toml
[lib]
name = "..."
crate-type = ["staticlib"]
```

## Installation

Install `cargo lipo` with `cargo install cargo-lipo`. `cargo lipo` can be build with rust 1.8 and later.

You also need a rust compiler which can compile for the iOS targets. If you use [rustup](https://www.rustup.rs/) all you should have to do is

```sh
rustup target add aarch64-apple-ios
rustup target add armv7-apple-ios
rustup target add i386-apple-ios
rustup target add x86_64-apple-ios
```

## Troubleshooting

 Cargo fails with ``error: can't find crate for `std` ``: Your rust compiler most likely does not support cross-compiling to iOS.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
