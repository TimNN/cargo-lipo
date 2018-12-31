use crate::{Invocation, Result};
use failure::bail;
use log::{info, debug};
use std::path::Path;

pub(crate) struct Meta<'a> {
    packages: Vec<Package<'a>>,
    target_dir: &'a Path,
}

pub(crate) struct Package<'a> {
    name: &'a str,
    lib_name: &'a str,
}

impl<'a> Meta<'a> {
    #[allow(clippy::useless_let_if_seq)] // multiple variables are initialized
    pub(crate) fn new(
        invocation: &'a Invocation,
        meta: &'a cargo_metadata::Metadata,
    ) -> Result<Meta<'a>> {
        let package_names: Vec<_>;
        let staticlib_required;

        if !invocation.packages.is_empty() {
            package_names = invocation.packages.iter().map(|p| p.as_str()).collect();
            staticlib_required = true;
        } else {
            package_names = meta.workspace_members.iter().map(|m| m.name()).collect();
            // Require a staticlib for single-member workspaces unless `--all` was specified.
            staticlib_required = meta.workspace_members.len() == 1 && !invocation.all;
        }

        debug!(
            "Considering package(s) {:?}, `staticlib` target {}",
            package_names,
            if staticlib_required { "required" } else { "not required" }
        );

        let mut packages = vec![];

        for &name in &package_names {
            let package = match meta.packages.iter().find(|p| p.name == name) {
                Some(p) => p,
                None => bail!("No package metadata found for {:?}", name),
            };

            let lib_targets: Vec<_> = package
                .targets
                .iter()
                .filter(|t| t.kind.iter().any(|k| k == "staticlib"))
                .collect();

            match lib_targets.as_slice() {
                [] => {
                    if !staticlib_required {
                        debug!("Ignoring {:?} because it does not have a `staticlib` target", name);
                        continue;
                    }
                    bail!("No library target found for {:?}", name);
                }
                [target] => {
                    if target.crate_types.iter().any(|ct| ct == "staticlib") {
                        packages.push((package, target.name.as_str()));
                    } else {
                        if !staticlib_required {
                            debug!(
                                "Ignoring {:?} because it does not have a `staticlib` crate type",
                                name
                            );
                            continue;
                        }
                        bail!("No staticlib crate type found for {:?}", name);
                    }
                }
                _ => bail!("Found multiple lib targets for {:?}", name),
            }
        }

        let packages = packages
            .iter()
            .map(|(p, lib_name)| Package { name: p.name.as_str(), lib_name })
            .collect::<Vec<_>>();

        let package_names = packages.iter().map(|p| p.name).collect::<Vec<_>>();

        if packages.is_empty() {
            bail!(
                "Did not find any packages with a `staticlib` target, considered {:?}",
                package_names
            );
        }

        info!("Will build universal library for {:?}", package_names);

        Ok(Meta { packages, target_dir: Path::new(&meta.target_directory) })
    }

    pub(crate) fn packages(&self) -> &[Package<'a>] {
        &self.packages
    }

    pub(crate) fn target_dir(&self) -> &'a Path {
        self.target_dir
    }
}

impl<'a> Package<'a> {
    pub(crate) fn name(&self) -> &'a str {
        self.name
    }

    pub(crate) fn lib_name(&self) -> &'a str {
        self.lib_name
    }
}
