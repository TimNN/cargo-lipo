# `cargo lipo` [![Build Status](https://travis-ci.org/TimNN/cargo-lipo.svg?branch=master)](https://travis-ci.org/TimNN/cargo-lipo) [![Crates.io](https://img.shields.io/crates/v/cargo-lipo.svg)](https://crates.io/crates/cargo-lipo)

Provides a `cargo lipo` subcommand which automatically creates a universal library for use with your iOS application.

## Usage

From anywhere you would usually run `cargo` you can now run `cargo lipo` or `cargo lipo --release` to create a universal library for ios, which can be found in `$target/universal/{release|debug}/$lib_name.a`.

## Installation

Install `cargo lipo` with `cargo install cargo-lipo`. `cargo lipo` can be build with rust 1.7 and later.

You also need a rust compiler which can compile for the iOS targets. If you use [rustup](https://www.rustup.rs/) all you should have to do is

```sh
rustup target add aarch64-apple-ios
rustup target add armv7-apple-ios
rustup target add armv7s-apple-ios
rustup target add i386-apple-ios
rustup target add x86_64-apple-ios
```

If you use a recent version of [multirust](https://github.com/brson/multirust)

```sh
multirust add-target aarch64-apple-ios
multirust add-target armv7-apple-ios
multirust add-target armv7s-apple-ios
multirust add-target i386-apple-ios
multirust add-target x86_64-apple-ios
```

should work.

**Note:** both will only work on stable starting with the 1.8 release.

Alternatively you can build a rust compiler with iOS support yourself.

## Troubleshooting

 Cargo fails with ``error: can't find crate for `std` ``: Your rust compiler most likely does not support cross-compiling to iOS.
