use crate::Result;
use crate::cargo::Cargo;
use crate::meta::Meta;
use failure::ResultExt;
use log::{info, warn};
use std::fs;
use std::process::Command;
use std::path::Path;

fn should_update_output(
    output: impl AsRef<Path>,
    inputs: impl IntoIterator<Item = impl AsRef<Path>>,
) -> std::io::Result<bool> {
    let output_metadata = match fs::metadata(output) {
        Ok(metadata) => metadata,
        Err(_) => return Ok(true),
    };
    let output_mtime = output_metadata.modified()?;
    for input in inputs {
        let input_mtime = fs::metadata(input)?.modified()?;
        if input_mtime > output_mtime {
            return Ok(true);
        }
    }
    Ok(false)
}

pub(crate) fn build(cargo: &Cargo, meta: &Meta, targets: &[impl AsRef<str>]) -> Result<()> {
    for package in meta.packages() {
        let lib_name = format!("lib{}.a", package.lib_name());
        let mut inputs = vec![];

        for target in targets {
            let target = target.as_ref();
            info!("Building {:?} for {:?}", package.name(), target);

            cargo.build_lib(package.name(), target).with_context(|e| {
                format!("Failed to build {:?} for {:?}: {}", package.name(), target, e)
            })?;

            let mut input = meta.target_dir().to_owned();
            input.push(target);
            input.push(cargo.profile());
            input.push(&lib_name);

            inputs.push(input);
        }

        let mut output = meta.target_dir().to_owned();
        output.push("universal");
        output.push(cargo.profile());

        fs::create_dir_all(&output).with_context(|e| {
            format!("Creating output directory \"{}\" failed: {}", output.display(), e)
        })?;

        output.push(&lib_name);

        match should_update_output(&output, &inputs) {
            Ok(true) => {}
            Ok(false) => {
                info!(
                    "Universal library is up-to-date, skipping lipo invocation for {}",
                    package.name()
                );
                continue;
            }
            Err(e) => {
                warn!(
                    "Failed to check if universal library for {:?} is up-to-date: {}",
                    package.name(),
                    e
                )
            }
        }
        let mut cmd = Command::new("lipo");
        cmd.arg("-create").arg("-output").arg(output);
        cmd.args(inputs);

        info!("Creating universal library for {}", package.name());

        crate::exec::run(cmd)?;
    }

    Ok(())
}
