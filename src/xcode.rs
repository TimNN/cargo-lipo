use crate::{Invocation, Result};
use crate::meta::Meta;
use failure::{bail, ResultExt};
use log::{warn, info};
use std::env;
use std::process::Command;

pub(crate) fn integ(meta: &Meta, mut invocation: Invocation) -> Result<()> {
    if is_release_configuration() {
        invocation.release = true;
    }

    let cargo = crate::cargo::Cargo::new(&invocation);

    match env::var("ACTION").with_context(|e| format!("Failed to read $ACTION: {}", e))?.as_str() {
        "build" => {
            crate::lipo::build(&cargo, meta, &targets_from_env()?)?;
        }
        "clean" => {
            for package in meta.packages() {
                for target in targets_from_env()? {
                    info!("Cleaning {:?} for {:?}", package.name(), target);
                    cargo.clean(package.name(), target)?;
                }
            }
        }
        action => warn!("Unsupported XCode action: {:?}", action),
    }

    Ok(())
}

fn targets_from_env() -> Result<Vec<&'static str>> {
    if only_active_arch() {
        current_target().map(|t| vec![t])
    } else {
        let archs = env::var("ARCHS").with_context(|e| format!("Failed to read $ARCHS: {}", e))?;
        Ok(archs
            .split(" ")
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
            .map(map_arch_to_target)
            .collect::<Result<Vec<_>>>()
            .with_context(|e| format!("Failed to parse $ARCHS: {}", e))?)
    }
}

fn current_target() -> Result<&'static str> {
    let arch = env::var("CURRENT_ARCH")
        .with_context(|e| format!("Failed to read $CURRENT_ARCH: {}", e))?;
    Ok(map_arch_to_target(&arch)
        .with_context(|e| format!("Failed to parse $CURRENT_ARCH: {}", e))?)
}

fn only_active_arch() -> bool {
    env::var("ONLY_ACTIVE_ARCH").map(|v| v == "YES").unwrap_or(false)
}

fn is_release_configuration() -> bool {
    env::var("CONFIGURATION").map(|v| v == "Release").unwrap_or(false)
}

fn map_arch_to_target(arch: &str) -> Result<&'static str> {
    match arch {
        "armv7" => Ok("armv7-apple-ios"),
        "arm64" => Ok("aarch64-apple-ios"),
        "i386" => Ok("i386-apple-ios"),
        "x86_64" => Ok("x86_64-apple-ios"),
        _ => bail!("Unknown arch: {:?}", arch),
    }
}

pub(crate) fn sanitize_env(cmd: &mut Command) {
    cmd.env_clear();
    cmd.envs(env::vars_os().filter(|&(ref name, _)| match name.to_str() {
        Some(name) => !(name.ends_with("DEPLOYMENT_TARGET") || name.starts_with("SDK")),
        None => false,
    }));
}
