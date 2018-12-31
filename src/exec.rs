use crate::Result;
use failure::{bail, ResultExt};
use log::debug;
use std::process::Command;

pub fn run(mut cmd: Command) -> Result<()> {
    debug!("Running: {:?}", cmd);

    let status = cmd.status().with_context(|e| format!("Executing {:?} failed: {}", cmd, e))?;

    if !status.success() {
        bail!("Executing {:?} finished with error status: {}", cmd, status);
    }

    Ok(())
}
