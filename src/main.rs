#![allow(clippy::needless_pass_by_value, clippy::new_ret_no_self, clippy::single_char_pattern)]
#![deny(unused_must_use)]

use log::{error, trace};
use std::cmp;
use std::ffi::OsString;
use std::path::PathBuf;
use structopt::StructOpt;

mod cargo;
mod exec;
mod lipo;
mod meta;
mod xcode;

type Result<T> = std::result::Result<T, failure::Error>;

/// Automatically create universal libraries.
#[derive(StructOpt, Debug)]
#[structopt(name = "cargo-lipo", bin_name = "cargo")]
#[structopt(author = "")]
#[structopt(raw(setting = "clap::AppSettings::SubcommandRequiredElseHelp"))]
struct CargoInvocation {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Automatically create universal libraries.
    #[structopt(name = "lipo")]
    #[structopt(author = "")]
    Invocation(Invocation),
}

#[derive(StructOpt, Debug)]
struct Invocation {
    /// Coloring: auto, always, never
    #[structopt(long, default_value = "auto", value_name = "WHEN")]
    color: Coloring,

    /// Use verbose output (-vv very verbose output)
    #[structopt(long, short)]
    #[structopt(parse(from_occurrences))]
    verbose: u32,

    /// Determine `targets` and `release` from the environment provided by XCode to build scripts.
    #[structopt(long = "xcode-integ")]
    #[structopt(conflicts_with = "release", conflicts_with = "targets")]
    xcode_integ: bool,

    /// Don't run `cargo clean` when XCode cleans the project.
    #[structopt(long = "xcode-ignore-clean")]
    #[structopt(requires = "xcode_integ")]
    xcode_ignore_clean: bool,

    /// Don't remove environment variables that can cause issues with build scripts when calling
    /// cargo.
    #[structopt(long = "no-sanitize-env")]
    no_sanitize_env: bool,

    /// Build artifacts in release mode, with optimizations
    #[structopt(long)]
    release: bool,

    /// Require Cargo.lock and cache are up to date
    #[structopt(long)]
    frozen: bool,

    /// Require Cargo.lock is up to date
    #[structopt(long)]
    locked: bool,

    /// Number of parallel jobs to be used by Cargo, defaults to # of CPUs
    #[structopt(long, short, value_name = "N")]
    jobs: Option<u32>,

    /// Build all packages in the workspace (that have a `staticlib` target)
    #[structopt(long)]
    all: bool,

    /// Package(s) to build (must have a `staticlib` target)
    #[structopt(short, long = "package", value_name = "NAME")]
    #[structopt(conflicts_with = "all")]
    packages: Vec<String>,

    /// Activate all available features
    #[structopt(long = "all-features")]
    all_features: bool,

    /// Path to Cargo.toml
    #[structopt(long = "no-default-features")]
    no_default_features: bool,

    /// Space-separated list of features to activate
    #[structopt(long, value_name = "FEATURES")]
    #[structopt(parse(from_os_str))]
    features: Option<OsString>,

    /// Path to Cargo.toml
    #[structopt(long = "manifest-path", value_name = "PATH")]
    #[structopt(parse(from_os_str))]
    manifest_path: Option<PathBuf>,

    /// Build for the target triples
    #[structopt(long, value_name = "TARGET1,TARGET2")]
    #[structopt(value_delimiter = ",")]
    #[structopt(default_value = "aarch64-apple-ios,x86_64-apple-ios")]
    targets: Vec<String>,
}

fn main() {
    let cargo = CargoInvocation::from_args();
    let Command::Invocation(invocation) = cargo.cmd;

    env_logger::Builder::from_default_env()
        .default_format_timestamp(false)
        .write_style(invocation.color.log_style())
        .filter_module(
            "cargo_lipo",
            [log::LevelFilter::Info, log::LevelFilter::Debug, log::LevelFilter::Trace]
                [cmp::min(invocation.verbose, 2) as usize],
        )
        .init();

    trace!("Invocation: {:#?}", invocation);

    if let Err(e) = run(invocation) {
        error!("{}", e);
        std::process::exit(1);
    }
}

fn run(invocation: Invocation) -> Result<()> {
    let meta = cargo_metadata::metadata(invocation.manifest_path.as_ref().map(|p| p.as_ref()))
        .map_err(failure::SyncFailure::new)?;

    trace!("Metadata: {:#?}", meta);

    let meta = meta::Meta::new(&invocation, &meta)?;

    if invocation.xcode_integ {
        xcode::integ(&meta, invocation)
    } else {
        lipo::build(&cargo::Cargo::new(&invocation), &meta, &invocation.targets)
    }
}

#[derive(Copy, Clone, Debug)]
enum Coloring {
    Auto,
    Always,
    Never,
}

impl Coloring {
    pub fn value(self) -> &'static str {
        match self {
            Coloring::Auto => "auto",
            Coloring::Always => "always",
            Coloring::Never => "never",
        }
    }

    pub fn log_style(self) -> env_logger::WriteStyle {
        match self {
            Coloring::Auto => env_logger::WriteStyle::Auto,
            Coloring::Always => env_logger::WriteStyle::Always,
            Coloring::Never => env_logger::WriteStyle::Never,
        }
    }
}

impl std::str::FromStr for Coloring {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Coloring::Auto),
            "always" => Ok(Coloring::Always),
            "never" => Ok(Coloring::Never),
            _ => Err(format!("Invalid coloring: '{}'", s)),
        }
    }
}
