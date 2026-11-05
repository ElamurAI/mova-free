//! Bounded, source-bound relation catalogue audit; never production substitution.
mod corpus;

use anyhow::{Result, bail};
use std::{env, path::Path};

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 5 || args[0] != "scan" {
        bail!("usage: mova-correspondence scan REPO REGISTRY CATALOG NEW_JSON");
    }
    let report = corpus::scan(
        Path::new(&args[1]),
        Path::new(&args[2]),
        Path::new(&args[3]),
    )?;
    mova_modelcheck::write_new(Path::new(&args[4]), &serde_json::to_vec_pretty(&report)?)?;
    println!(
        "{}",
        serde_json::json!({"output":args[4],"coverage":report["coverage"],"proof":false,"production_substitution":false})
    );
    Ok(())
}
