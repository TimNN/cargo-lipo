extern crate clap;
extern crate serde_json as json;

use clap::{App, SubCommand};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{self, Command, ExitStatus};
use std::result::Result as StdResult;

use msg_error::{Error, Result};

#[macro_use] mod json_macros;
#[macro_use] mod msg_error;

static IOS_TRIPLES: &'static [&'static str] = &[
    "aarch64-apple-ios",
    "armv7-apple-ios",
    "armv7s-apple-ios",
    "i386-apple-ios",
    "x86_64-apple-ios",
];

fn main() {
    if let Err(err) = real_main() {
        println!("{}", err);
        process::exit(1);
    }
}

fn real_main() -> Result<()> {
    let matches = build_app().get_matches();
    let matches = trm!("Invalid invocation"; matches.subcommand_matches("lipo").ok_or("subcommand required"));

    let release = matches.is_present("release");
    let verbose = matches.is_present("verbose");

    let lib_name = try!(find_lib_name(verbose));

    let triples: Vec<&str> = match matches.values_of("targets") {
        Some(values) => values.collect(),
        None => IOS_TRIPLES.to_vec(),
    };

    let features = matches.value_of("features").unwrap_or("");

    let color = matches.value_of("color").unwrap_or("auto");

    for triple in &triples {
        try!(build_triple(triple, release, verbose, features, color));
    }

    let target_path = try!(find_target_path(verbose));
    let target_subdir = if release { "release" } else { "debug" };

    let out_dir = target_path.join("universal").join(&target_subdir);
    let out = out_dir.join(&lib_name);

    trm!("Failed to create output directory"; fs::create_dir_all(&out_dir));

    let mut cmd = Command::new("lipo");
    cmd.args(&["-create", "-output"]);
    cmd.arg(out.as_os_str());

    for triple in &triples {
        cmd.arg(target_path.join(triple).join(target_subdir).join(&lib_name).as_os_str());
    }

    let status = trm!("Failed to execute lipo"; cmd.status());

    trm!("lipo exited unsuccessfully"; exit_to_result(status));

    Ok(())
}

fn build_app<'a, 'b>() -> App<'a, 'b> {
    App::new("cargo-lipo")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Tim Neumann <mail@timnn.me>")
        .about("This binary should only be executed as a custom cargo subcommand (ie. `cargo lipo`)")
        .bin_name("cargo")
        .subcommand(SubCommand::with_name("lipo")
            .version(env!("CARGO_PKG_VERSION"))
            .author("Tim Neumann <mail@timnn.me>")
            .about("Automatically create universal libraries")
            .args_from_usage("--release 'Compiles in release mode'
                              --targets=[TRIPLE1,TRIPLE2] 'Build for the target triples'
                              --features=[FEATURES] 'Space-separated list of features to also build'
                              --color=[WHEN] 'Coloring: auto, always, never'
                              -v --verbose 'Print additional information'")
        )
}

/// Invoke `cargo build` for the given triple.
fn build_triple(triple: &str, release: bool, verbose: bool, features: &str, color: &str) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.args(&["build", "--target", triple, "--lib", "--features", features, "--color", color]);

    if release { cmd.arg("--release"); }
    if verbose { cmd.arg("--verbose"); }

    log_command(&cmd, verbose);

    let status = trm!("Failed to build library for {}", triple; cmd.status());
    trm!("Cargo exited unsuccessfully"; exit_to_result(status));

    Ok(())
}

/// Find the name of the staticlibrary to build as defined in the project's `Cargo.toml`.
fn find_lib_name(verbose: bool) -> Result<String> {
    static ERR: &'static str = "Failed to parse `cargo read-manifest` output";

    let value = trm!(ERR; cargo_json_value("read-manifest", verbose));

    let targets = trm!(ERR; json_get!(Array, value.targets));

    let mut lib_name = None;

    for target in targets {
        let kinds = trm!(ERR; json_get!(Array, target.kind));

        for kind in kinds {
            let kind = trm!(ERR; kind.as_string().ok_or("kind is not a string"));

            if kind == "staticlib" {
                if let Some(_) = lib_name {
                    return Err(Error::new(ERR, "Multiple targets with kind `staticlib` found"));
                }

                lib_name = Some(trm!(ERR; json_get!(String, target.name)));
            }
        }
    }

    match lib_name {
        Some(name) => Ok(String::from("lib") + &name.replace("-", "_") + ".a"),
        None => Err(Error::new(ERR, "No target with kind `staticlib` found")),
    }
}

/// Find the path to the project's `target` directory.
fn find_target_path(verbose: bool) -> Result<PathBuf> {
    static ERR: &'static str = "Failed to parse `cargo locate-project`";
    static ERR2: &'static str = "Failed to verify target directory";

    let value = trm!(ERR; cargo_json_value("metadata", verbose));

    let target_directory = trm!(ERR; json_get!(String, value.target_directory));
    let target: &Path = target_directory.as_ref();

    let meta = trm!(ERR2; fs::metadata(&target));

    if !meta.is_dir() {
        Err(Error::new(ERR2, "not a directory"))
    } else {
        Ok(target.to_path_buf())
    }
}

/// Create a `serde_json::Value` from the output of the given cargo subcomand.
fn cargo_json_value(subcommand: &str, verbose: bool) -> Result<json::Value> {
    let mut cmd = Command::new("cargo");
    cmd.arg(subcommand);

    log_command(&cmd, verbose);

    let output = trm!("Failed to execute cargo"; cmd.output());

    trm!("Cargo exited unsuccessfully"; exit_to_result(output.status));

    let json = trm!("Invalid json"; String::from_utf8(output.stdout));
    let value = trm!("Invalid json"; json::from_str(&json));

    Ok(value)
}

/// Convert an `ExitStatus` into a `Result`.
fn exit_to_result(exit: ExitStatus) -> StdResult<(), String> {
    if exit.success() {
        Ok(())
    } else {
        Err(exit.to_string())
    }
}

/// Log the given command to stdout if running in verbose mode.
fn log_command(cmd: &Command, verbose: bool) {
    if verbose {
        println!("Executing: {:?}", cmd);
    }
}
