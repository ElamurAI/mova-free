//! CLI transport only. Composition never executes Cargo, scripts or hooks.
use anyhow::{Result, bail};
use mova_module_overlay::{check, load, save_snapshot, stage};
use std::path::Path;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        ["snapshot", upstream, revision, output] => {
            save_snapshot(Path::new(upstream), revision, Path::new(output))?;
        }
        ["check", upstream, custom, manifest] => {
            let (manifest, expected) = load(Path::new(custom), manifest)?;
            let report = check(Path::new(upstream), Path::new(custom), &manifest, &expected)?;
            println!("{}", serde_json::to_string_pretty(&report)?);
            if !report.compatible {
                std::process::exit(3);
            }
        }
        ["stage", upstream, custom, manifest, output] => {
            let (manifest, expected) = load(Path::new(custom), manifest)?;
            let receipt = stage(
                Path::new(upstream),
                Path::new(custom),
                &manifest,
                &expected,
                Path::new(output),
            )?;
            println!("{}", serde_json::to_string_pretty(&receipt)?);
        }
        _ => bail!(
            "usage: mova-module-overlay snapshot PUBLIC_SOURCE REVISION_LABEL NEW_JSON | check PUBLIC_SOURCE CUSTOM_ROOT MANIFEST_REL | stage PUBLIC_SOURCE CUSTOM_ROOT MANIFEST_REL NEW_TREE"
        ),
    }
    Ok(())
}
