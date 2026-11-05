//! The default of `$MOVA_BIG` (the heavy-data directory) is a fact of one machine, so it is not in the public code:
//! it comes from the line `MOVA_BIG=<dir>` of the private `peta/paths.env`, absent from public checkouts. Without
//! that file the default is empty, and `cycle` asks for `MOVA_BIG` at run time.

fn main() {
    let f = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../peta/paths.env");
    // only an existing file is watched: cargo treats a watched absent path as always changed
    if f.is_file() {
        println!("cargo:rerun-if-changed={}", f.display());
    }
    let v = std::fs::read_to_string(&f)
        .ok()
        .and_then(|t| t.lines().find_map(|l| l.trim().strip_prefix("MOVA_BIG=").map(|v| v.trim().to_string())))
        .unwrap_or_default();
    println!("cargo:rustc-env=MOVA_BIG_DEFAULT={v}");
}
