//! Valuing a component by ablation. Design note: to check the quality of a commercial algorithm and its training
//! bases, make a copy of the system without that component, measure, report whether it helps, and delete the
//! result — then it is clear whom the money should go to.
//!
//!   value <name> --exclude <substring of seed paths>[,…] [--env K=V]… [--quick]
//!
//! 1) system copy — a git worktree in $MOVA_DATA/work/value-<name> (its own target);
//! 2) build with GLOBAL_EXCLUDE (the component's seeds are not built) and the env that disables the module;
//! 3) bench (tools/bench) in the copy and in the main system — the same measurements;
//! 4) report docs/value/<name>.md: metric, with component, without, delta, conclusion;
//! 5) the copy and intermediate results are deleted — only the report remains.

use std::path::PathBuf;
use std::process::Command;

use anyhow::{Context, Result, bail};

fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
}

/// The data directory: $MOVA_DATA, else ~/mova-data.
fn mova_data() -> PathBuf {
    std::env::var("MOVA_DATA").map(PathBuf::from).unwrap_or_else(|_| home().join("mova-data"))
}

fn sh(dir: &PathBuf, prog: &str, args: &[&str], env: &[(String, String)]) -> Result<String> {
    let o = Command::new(prog).args(args).current_dir(dir).envs(env.iter().map(|(a, b)| (a.as_str(), b.as_str()))).output().with_context(|| format!("{prog} {args:?}"))?;
    if !o.status.success() {
        bail!("{prog} {args:?}: {}", String::from_utf8_lossy(&o.stderr).lines().rev().take(5).collect::<Vec<_>>().join(" | "));
    }
    Ok(String::from_utf8_lossy(&o.stdout).to_string())
}

fn bench(root: &PathBuf, out: &PathBuf, quick: bool, env: &[(String, String)]) -> Result<Vec<(String, String)>> {
    let mut e = env.to_vec();
    e.push(("MOVA_ROOT".into(), root.display().to_string()));
    e.push(("BENCH_OUT".into(), out.display().to_string()));
    let mut args = vec![];
    if quick { args.push("--quick"); }
    let bin = home().join("Mova/target/release/bench");
    sh(root, &bin.display().to_string(), &args, &e)?;
    let text = std::fs::read_to_string(out)?;
    let lines: Vec<&str> = text.lines().collect();
    let h: Vec<&str> = lines.iter().filter(|l| l.starts_with("time\t")).last().context("header")?.split('\t').collect();
    let r: Vec<&str> = lines.last().context("row")?.split('\t').collect();
    Ok(h.iter().zip(r.iter()).skip(2).map(|(a, b)| (a.to_string(), b.to_string())).collect())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let name = args.first().context("usage: value <name> --exclude <substrings> [--env K=V] [--quick]")?.clone();
    let exclude = args.iter().position(|a| a == "--exclude").and_then(|i| args.get(i + 1)).cloned().unwrap_or_default();
    let quick = args.iter().any(|a| a == "--quick");
    let envs: Vec<(String, String)> = args.windows(2).filter(|w| w[0] == "--env").filter_map(|w| w[1].split_once('=').map(|(a, b)| (a.to_string(), b.to_string()))).collect();
    let main_root = home().join("Mova");
    let copy = mova_data().join(format!("work/value-{name}"));
    let branch = format!("value-{name}");
    let runs = mova_data().join(format!("runs/value-{name}"));
    std::fs::create_dir_all(&runs)?;
    // 1) copy
    let _ = sh(&main_root, "git", &["worktree", "remove", "--force", &copy.display().to_string()], &[]);
    let _ = sh(&main_root, "git", &["branch", "-D", &branch], &[]);
    sh(&main_root, "git", &["worktree", "add", "-q", &copy.display().to_string(), "-b", &branch, "HEAD"], &[])?;
    // 2) build the copy without the component
    let mut benv = envs.clone();
    benv.push(("GLOBAL_EXCLUDE".into(), exclude.clone()));
    let cargo = home().join(".cargo/bin/cargo").display().to_string();
    for c in ["global", "math", "world"] {
        sh(&copy.join(c), &cargo, &["build", "--release", "-q"], &benv)?;
    }
    // 3) bench: without the component (copy) and with it (main system)
    let without = bench(&copy, &runs.join("without.tsv"), quick, &envs)?;
    let with = bench(&main_root, &runs.join("with.tsv"), quick, &[])?;
    // 4) report
    let mut rep = format!("# Value of component «{name}»\n\nAblation: a copy of the system without this component, measurements, a report on whether it helps, then the result is deleted. The copy without the component was built with `GLOBAL_EXCLUDE={exclude}`{}; measurements — `tools/bench{}`; the copy and intermediate results were deleted.\n\n| metric | with component | without | delta |\n|---|---|---|---|\n", if envs.is_empty() { String::new() } else { format!(" and {envs:?}") }, if quick { " --quick" } else { "" });
    let mut gains = 0;
    let mut losses = 0;
    for ((k, a), (_, b)) in with.iter().zip(without.iter()) {
        if k.ends_with("_s") { continue; }
        let d = match (a.parse::<f64>(), b.parse::<f64>()) { (Ok(x), Ok(y)) => { if x > y + 1e-9 { gains += 1 } else if x < y - 1e-9 { losses += 1 } format!("{:+.3}", x - y) } _ => "?".into() };
        rep.push_str(&format!("| {k} | {a} | {b} | {d} |\n"));
    }
    rep.push_str(&format!("\n**Conclusion:** metrics where the component helps — {gains}; where it hurts — {losses}. (For gaps and wrong answers lower is better: read the delta according to the metric's meaning.)\n"));
    let out = main_root.join(format!("docs/value/{name}.md"));
    std::fs::create_dir_all(out.parent().unwrap())?;
    std::fs::write(&out, &rep)?;
    println!("{rep}");
    // 5) delete the copy and intermediate files
    sh(&main_root, "git", &["worktree", "remove", "--force", &copy.display().to_string()], &[])?;
    sh(&main_root, "git", &["branch", "-D", &branch], &[])?;
    std::fs::remove_dir_all(&runs)?;
    println!("copy and intermediate results deleted; report — {}", out.display());
    Ok(())
}
