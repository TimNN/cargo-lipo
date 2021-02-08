use crate::Result;
use crate::cargo::Cargo;
use crate::meta::Meta;
use failure::ResultExt;
use fat_macho::FatWriter;
use log::info;
use std::fs;

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

        info!("Creating universal library for {}", package.name());
        let mut writer = FatWriter::new();
        for input in &inputs {
            let content = fs::read(&input)
                .with_context(|e| format!("Read file \"{}\" failed: {}", input.display(), e))?;
            writer
                .add(content)
                .with_context(|e| format!("Add file \"{}\" failed: {}", input.display(), e))?;
        }
        writer.write_to_file(output)?;
    }

    Ok(())
}
