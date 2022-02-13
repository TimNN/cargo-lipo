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

`cargo-lipo` easily integrates with Xcode. For XCode 13, here is a recipe for the integration, assuming that your `myproject` crate (that has the static library) is a sibling of your XCode project directory and that your build library is `libmystatic.a`.

1. Add a new *"Run Script"* phase to your *"Build Phases"*. Place it **before** *"Compile Sources"*. Name it "build Rust static library". Make it run on every build (uncheck dependency analysis), and give it an output file of `$(PROJECT_DIR)/../target/universal/$(CONFIGURATION:lower)/libmystatic.a`.  Add something like the following to the script:

    ```bash
    # The $PATH used by Xcode likely won't contain Cargo, fix that.
    # This assumes a default `rustup` setup.
    export PATH="$HOME/.cargo/bin:$PATH"

    # --xcode-integ determines --release and --targets from Xcode's env vars.
    # Depending your setup, specify the rustup toolchain explicitly.
    cargo lipo --xcode-integ --manifest-path ../myproject/Cargo.toml
    ```

2. Run cargo lipo manually to build both debug and release, then update the *"Link Binary with Libraries"* phase:
   1. First add your debug library by clicking the <kbd>+</kbd>, choosing *"Add Other..."*, and navigating to `../myproject/target/universal/debug` and selecting your library.
   2. Next add your release library similarly.
   3. After adding both libraries to XCode, delete the actual static library files (not their references in the XCode project).  Yes, really.  If you don't, XCode won't think they need rebuilding so it won't run your script.

3. Next, go back to your *"Build Settings"* and add a *"Library Search Path"* of `$(PROJECT_DIR)../myproject/target/universal/$(CONFIGURATION:lower)`.  This will provide the right search paths for both debug and release.

4. Finally, add a second *"Run Script"* build phase but leave this one at the bottom (so it runs after the build is complete).  Name this one "delete Rust static libraries" and make it run on every build.  (While it may seem counter-intuitive to delete the library we just built, it's needed to make Xcode's new build system run the build script each time.  Back in the days when there were just two target platforms -- x86_64 and arm64 -- the output of cargo lipo would have both and a rebuild wouldn't be necessary.  But now that there are actually three target platforms -- x86_64, arm64, and arm64-simulator -- the built library can only have one of the arm64 variants. Every time you change the target platform, the library will need to be rebuilt, but Xcode can't detect that because it's looking at mod dates not at target platforms.  So we always delete the built library after it's linked, in order to force it to be rebuilt with the correct target the next time through.) The content of this phase should be something like:
   ```bash
   # Delete the built libraries that were just linked.
   # If this isn't done, XCode won't try to rebuild them
   # by running the build scripts, because it won't think
   # they are out of date.
   rm -fv ../myproject/target/universal/*/*.a
   ```
5. If you are planning to do `Archive` builds in the XCode application, you also need to go into your *"Build Settings"* and set *Enable Bitcode* to **`No`**.  This is because Rust uses a different LLVM than Xcode does, and the in-application XCode `Archive` build process does a bitcode verification which will fail on Rust libraries with an error message such as: `Invalid value (Producer: 'LLVM13.0.0-rust-1.57.0-stable' Reader: 'LLVM APPLE_1_1300.0.29.30_0') for architecture arm64`

A final note about XCode integration: because all XCode builds are "one target at a time", there's really no need to use `cargo lipo` at all when building for iOS.  For an example of an Xcode product that invokes cargo directly, look at the apps in the [`rust-on-ios` project](https://github.com/brotskydotcom/rust-on-ios).

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
