# `cargo lipo` [![Build Status](https://travis-ci.org/TimNN/cargo-lipo.svg?branch=master)](https://travis-ci.org/TimNN/cargo-lipo) [![Crates.io](https://img.shields.io/crates/v/cargo-lipo.svg)](https://crates.io/crates/cargo-lipo)

Provides a `cargo lipo` subcommand which automatically creates a universal library for use with your iOS application.

*Note:* You still need to have a `rustc` which can cross-compile to iOS on your path. If cargo fails with ``error: can't find crate for `std` `` your rust compiler most likely does not support cross-compiling to iOS.

*Note:* While `cargo lipo` can be compiled on stable rust, it requires at least cargo version `0.5.0` to run which currently only ships with the beta.

## Installation

Until we get a `cargo install` command you will need to checkout this repository, run `cargo build --release` and the make sure that `target/release/cargo-lipo` is somewhere on your `$PATH`.

## Usage

From anywhere you would usually run `cargo` you can now run `cargo lipo` or `cargo lipo --release` to create a universal library for ios, which can be found in `$target/universal/{release|debug}/$lib_name.a`.
