# Test Data

This directory contains Cargo projects to test `cargo-lipo` with:

* `simple/`: Contains a single `staticlib`
* `workspace/`: A workspace with multiple projects
    * `buildutil`: To be used as a build dependency. Has itself a build script.
    * `normal`: A "normal" rust library crate.
    * `static1`: A simple `staticlib`, depends on `normal`.
    * `static2build`: A `staticlib` with a build script. Library and build script both depend on `buildutil`.
    * `static3bin`: Contains `staticlib`, that is also an `rlib`, as well as a `bin` target.
