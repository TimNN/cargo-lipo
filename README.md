# `cargo lipo` [![Build Status](https://img.shields.io/github/workflow/status/TimNN/cargo-lipo/Test/master)](https://github.com/TimNN/cargo-lipo/actions) [![Crates.io](https://img.shields.io/crates/v/cargo-lipo.svg)](https://crates.io/crates/cargo-lipo)

Provides a `cargo lipo` subcommand which automatically creates a universal library for use with your iOS application.

## Maintenance Status

Please consider this project deprecated / passively maintained. This is partly because I am not currently working on any iOS projects, and partly because I believe that there exists a better alternative to using `lipo`:

One can use architecture (and OS) specific environment variables in Xcode. The OS specific part could be configured in the Xcode project editor last time I tried, but the architecture specific part needed to be added by manually editing the `project.pbxproj` file, for example like this:

```plain
    "LIBRARY_SEARCH_PATHS[sdk=iphoneos*]" = ../path/to/target/debug/<...>;
    "LIBRARY_SEARCH_PATHS[sdk=macosx11.1][arch=arm64]" = ../path/to/target/<...>;
    "LIBRARY_SEARCH_PATHS[sdk=macosx11.1][arch=x86_64]" = ../path/to/target/<...>;
```

Thus, I believe that a future iOS support crate should offer primarily two features:

* Something similar to the current `--xcode-integ` flag.
* Something which can do the `project.pbxproj` editing.

## Usage

From anywhere you would usually run `cargo` you can now run `cargo lipo` or `cargo lipo --release` to create a universal library for ios, which can be found in `$target/universal/{release|debug}/$lib_name.a`.

Make sure you have a library target in your `Cargo.toml` with a crate type of `staticlib`:

```toml
[lib]
name = "..."
crate-type = ["staticlib"]
```

### Xcode Integration

`cargo-lipo` easily integrates with Xcode. Although note that this functionality has only been added recently and may not yet be perfect (the Xcode build process is somewhat of a blackbox to me).

1. In your *"Build Settings"* change *"Enable Bitcode"* to **`No`**.

2. Add a new *"Run Script"* phase to your *"Build Phases"*. Place it **before** *"Compile Sources"*. Add something like the following to the script:

    ```bash
    # The $PATH used by Xcode likely won't contain Cargo, fix that.
    # This assumes a default `rustup` setup.
    export PATH="$HOME/.cargo/bin:$PATH"

    # --xcode-integ determines --release and --targets from Xcode's env vars.
    # Depending your setup, specify the rustup toolchain explicitly.
    cargo lipo --xcode-integ --manifest-path ../something/Cargo.toml
    ```

3. Build the project once, then update the *"Link Binary with Libraries"* phase: Click the <kbd>+</kbd>, then choose *"Add Other..."*. Navigate to `your-cargo-project/target/universal/{debug-or-release}` and select your library(s).

4. Go back to your *"Build Settings"* and add *"Library Search Paths"* for *"Debug"* and *"Release"*, pointing to `your-cargo-project/target/universal/{debug-or-release}`.

## Installation

Install `cargo lipo` with `cargo install cargo-lipo`. `cargo lipo` should always be buildable with the latest stable Rust version. For the minimum supported version check `.travis.yml`.

You also need a rust compiler which can compile for the iOS targets. If you use [rustup](https://www.rustup.rs/) all you should have to do is

```sh
# 64 bit targets (real device & simulator):
rustup target add aarch64-apple-ios x86_64-apple-ios
# 32 bit targets (you probably don't need these):
rustup target add armv7-apple-ios i386-apple-ios
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
