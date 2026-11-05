//! The legacy central manuals (feature `worlds`, also enabled by `peta`) live under `peta/world/legacy/worlds`,
//! which public checkouts do not have. With the feature and the folder, `cfg(mova_legacy_worlds)` compiles them
//! in; with the feature but without the folder the build warns and carries no manuals, as without the feature, so
//! `cargo check --all-features` works on a public checkout.
use std::path::Path;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(mova_legacy_worlds)");
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var_os("CARGO_FEATURE_WORLDS").is_none() {
        return;
    }
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../peta/world/legacy/worlds");
    if dir.is_dir() {
        println!("cargo:rerun-if-changed={}", dir.display());
        println!("cargo:rustc-cfg=mova_legacy_worlds");
    } else {
        println!("cargo:warning=feature `worlds`: peta/world/legacy/worlds is absent (a public checkout), so the legacy manuals are not built in");
    }
}
