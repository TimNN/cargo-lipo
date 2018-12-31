use crate::{Invocation, Result};
use std::env;
use std::ffi::OsString;
use std::process::Command;

pub(crate) struct Cargo<'a> {
    cargo: OsString,
    invocation: &'a Invocation,
}

impl<'a> Cargo<'a> {
    pub(crate) fn new(invocation: &'a Invocation) -> Cargo {
        Cargo { cargo: env::var_os("CARGO").unwrap_or_else(|| "cargo".into()), invocation }
    }

    pub(crate) fn build_lib(&self, name: &str, target: &str) -> Result<()> {
        crate::exec::run(self.prepare_build_lib(name, target))
    }

    pub(crate) fn clean(&self, name: &str, target: &str) -> Result<()> {
        crate::exec::run(self.prepare_build_lib(name, target))
    }

    pub(crate) fn profile(&self) -> &str {
        if self.invocation.release {
            "release"
        } else {
            "debug"
        }
    }

    fn prepare(&self) -> Command {
        let mut cmd = Command::new(&self.cargo);

        cmd.arg("--color").arg(self.invocation.color.value());

        if self.invocation.verbose > 1 {
            cmd.arg(format!("-{}", "v".repeat(self.invocation.verbose as usize)));
        }

        if self.invocation.locked {
            cmd.arg("--locked");
        }

        if self.invocation.frozen {
            cmd.arg("--frozen");
        }

        cmd
    }

    fn prepare_target(&self, sub_command: &str, name: &str, target: &str) -> Command {
        let mut cmd = self.prepare();
        cmd.arg(sub_command);

        if let Some(ref path) = self.invocation.manifest_path {
            cmd.arg("--manifest-path").arg(path);
        }

        cmd.arg("-p").arg(name).arg("--target").arg(target);

        if self.invocation.release {
            cmd.arg("--release");
        }

        cmd
    }

    fn prepare_build_lib(&self, name: &str, target: &str) -> Command {
        let mut cmd = self.prepare_target("build", name, target);

        cmd.arg("--lib");

        if self.invocation.all_features {
            cmd.arg("--all-features");
        }

        if self.invocation.no_default_features {
            cmd.arg("--no-default-features");
        }

        if let Some(ref features) = self.invocation.features {
            cmd.arg("--features").arg(features);
        }

        if let Some(jobs) = self.invocation.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        cmd
    }
}
