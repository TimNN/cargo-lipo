use crate::{Invocation, Result};
use crate::meta::Meta;
use failure::{bail, ResultExt};
use log::warn;
use std::env;
use std::process::Command;

pub(crate) fn integ(meta: &Meta, mut invocation: Invocation) -> Result<()> {
    if is_release_configuration() {
        invocation.release = true;
    }

    let cargo = crate::cargo::Cargo::new(&invocation);

    match env::var("ACTION").with_context(|e| format!("Failed to read $ACTION: {}", e))?.as_str() {
        "build" | "install" => {
            crate::lipo::build(&cargo, meta, &targets_from_env()?)?;
        }
        action => warn!("Unsupported XCode action: {:?}", action),
    }

    Ok(())
}

fn targets_from_env() -> Result<Vec<String>> {
    let archs = env::var("ARCHS").with_context(|e| format!("Failed to read $ARCHS: {}", e))?;
    let (platform, abi) = match env::var("PLATFORM_NAME").as_ref().map(String::as_str) {
        Ok("macosx") => ("apple-darwin", None),
        // There's a new "iphonesimulator" platform that requires a special abi suffix
        // We just look for "sim" here as a minimal distinguishing check from "iphoneos".
        Ok(s) if s.contains("sim") => ("apple-ios", Some("sim")),
        _ => ("apple-ios", None)
    };
    Ok(archs
        .split(" ")
        .map(|a| a.trim())
        .filter(|a| !a.is_empty())
        .map(|a| map_arch_to_target(a, platform, abi))
        .collect::<Result<Vec<_>>>()
        .with_context(|e| format!("Failed to parse $ARCHS: {}", e))?)
}

fn is_release_configuration() -> bool {
    env::var("CONFIGURATION").map(|v| v == "Release").unwrap_or(false)
}

fn map_arch_to_target(arch: &str, platform: &str, abi: Option<&str>) -> Result<String> {
    let mapped_arch = match arch {
        "armv7" => "armv7",
        "arm64" => "aarch64",
        "i386" => "i386",
        "x86_64" => "x86_64",
        _ => bail!("Unknown arch: {:?}", arch),
    };
    match abi {
        // the "sim" abi is only used with aarch64 builds
        Some("sim") => if mapped_arch == "aarch64" {
            Ok(format!("{}-{}-{}", mapped_arch, platform, "sim"))
        } else {
            Ok(format!("{}-{}", mapped_arch, platform))
        },
        Some(abi) => Ok(format!("{}-{}-{}", mapped_arch, platform, abi)),
        None => Ok(format!("{}-{}", mapped_arch, platform)),
    }
}

pub(crate) fn sanitize_env(cmd: &mut Command) {
    cmd.env_clear();
    cmd.envs(env::vars_os().filter(|&(ref name, _)| match name.to_str() {
        Some(name) => !(name.ends_with("DEPLOYMENT_TARGET") || name.starts_with("SDK")),
        None => false,
    }));
}
