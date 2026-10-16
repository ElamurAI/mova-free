//! The snake's test bench. Design note: with a smaller role for the Opus checker and its answers cached, it becomes
//! quite possible to test many architectures and ideas from papers and books quickly.
//!
//! One command runs the measurements (no LLM — everything on frozen sets and cached verdicts) and appends a row to
//! `$MOVA_DATA/runs/bench.tsv`: time, commit, metric after metric. Comparing an idea — branch → `bench` → row vs the previous one.
//!
//!   bench [--quick]     --quick — without GSM8K (the longest measurement)

use std::path::PathBuf;
use std::process::Command;
use std::time::Instant;

use anyhow::{Context, Result};

fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
}

/// The data directory: $MOVA_DATA, else ~/mova-data.
fn mova_data() -> PathBuf {
    std::env::var("MOVA_DATA").map(PathBuf::from).unwrap_or_else(|_| home().join("mova-data"))
}

/// Run a measurement and extract numbers by line prefixes: (metric name, prefix, index of the number in the line).
fn run(name: &str, cmd: &str, args: &[&str], env: &[(&str, &str)], picks: &[(&str, &str, usize)]) -> Result<Vec<(String, String)>> {
    let t0 = Instant::now();
    let root = std::env::var("MOVA_ROOT").map(PathBuf::from).unwrap_or_else(|_| home().join("Mova"));
    let out = Command::new(root.join(cmd)).args(args).envs(env.iter().copied()).current_dir(&root).output().with_context(|| format!("{name}: {cmd}"))?;
    let text = String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    let mut res = Vec::new();
    for (key, prefix, k) in picks {
        let v = text.lines().find(|l| l.trim_start().starts_with(prefix)).and_then(|l| {
            let nums: Vec<String> = l.split(|c: char| !(c.is_ascii_digit() || c == '.')).filter(|x| !x.is_empty() && x.chars().any(|c| c.is_ascii_digit())).map(String::from).collect();
            nums.get(*k).cloned()
        }).unwrap_or_else(|| "?".into());
        res.push((key.to_string(), v));
    }
    res.push((format!("{name}_s"), format!("{:.0}", t0.elapsed().as_secs_f64())));
    eprintln!("{name}: {:?}", res);
    Ok(res)
}

fn main() -> Result<()> {
    let quick = std::env::args().any(|a| a == "--quick");
    let b = mova_data().join("runs/read-books-2026-09-30");
    let q = mova_data().join("runs/qgen-2026-09-30");
    let scripts = mova_data().join("runs/quant-2026-09-30/scripts.jsonl");
    let s = |p: PathBuf| p.display().to_string();
    let mut cols: Vec<(String, String)> = Vec::new();
    let nolex = [("MATH_NOLEX", "1")];
    cols.extend(run("svamp", "target/release/mathsolve", &["steps", "--set", "svamp", "--exact", "--epochs", "20", "--beam", "32", "--rerank", "5", "--world", &s(scripts.clone())], &nolex, &[
        ("svamp_%", "svamp test:", 3),
        ("svamp_answered_correct", "v2: answered — correct", 1),
        ("svamp_answered_wrong", "v2: answered — wrong", 1),
        ("world_correct_agrees", "world: correct, agrees with judge", 0),
        ("world_correct_judge_no", "world: correct, judge not", 0),
    ])?);
    if !quick {
        cols.extend(run("gsm8k", "target/release/mathsolve", &["steps", "--set", "gsm8k", "--exact", "--epochs", "20", "--beam", "32", "--rerank", "5"], &nolex, &[("gsm8k_%", "gsm8k test:", 4)])?);
    }
    cols.extend(run("ftqa", "target/release/world", &["read-events"], &[], &[("ftqa_rougeL", "FairytaleQA test:", 0)])?);
    let mut rt = vec!["read-train".to_string()];
    for (bk, qa) in [("pg2591", "grimm"), ("pg11339", "aesop"), ("pg27200", "andersen"), ("pg503", "blue")] {
        rt.push(s(b.join(bk)));
        rt.push(s(q.join(qa)));
    }
    let rt_ref: Vec<&str> = rt.iter().map(|x| x.as_str()).collect();
    cols.extend(run("tales", "target/release/world", &rt_ref, &[], &[
        ("tales_EM", "learned search:", 2),
        ("tales_F1", "learned search:", 4),
        ("tales_pairs_EM", "pair trigger", 1),
    ])?);
    let fables: Vec<String> = ["2451", "2355", "2366"].iter().map(|d| s(mova_data().join(format!("runs/world-2026-09-26/cmds-{d}.txt")))).collect();
    let mut verdicts = 0;
    let mut gaps = 0;
    for f in &fables {
        let root = std::env::var("MOVA_ROOT").map(PathBuf::from).unwrap_or_else(|_| home().join("Mova"));
        let o = Command::new(root.join("target/release/world")).args(["values", f]).output()?;
        let t = String::from_utf8_lossy(&o.stdout);
        verdicts += t.lines().filter(|l| l.starts_with('✓') || l.starts_with('✗')).count();
        gaps += t.lines().filter(|l| l.trim_start().starts_with('·')).count();
    }
    cols.push(("fables_verdicts".into(), verdicts.to_string()));
    cols.push(("fables_gaps".into(), gaps.to_string()));
    // row into the table
    let commit = String::from_utf8_lossy(&Command::new("git").args(["-C", &s(home().join("Mova")), "rev-parse", "--short", "HEAD"]).output()?.stdout).trim().to_string();
    let date = String::from_utf8_lossy(&Command::new("date").args(["-u", "+%Y-%m-%dT%H:%MZ"]).output()?.stdout).trim().to_string();
    // BENCH_OUT — a different file (ablations write separately and get deleted)
    let path = std::env::var("BENCH_OUT").map(PathBuf::from).unwrap_or_else(|_| mova_data().join("runs/bench.tsv"));
    let header = format!("time\tcommit\t{}", cols.iter().map(|c| c.0.clone()).collect::<Vec<_>>().join("\t"));
    let row = format!("{date}\t{commit}\t{}", cols.iter().map(|c| c.1.clone()).collect::<Vec<_>>().join("\t"));
    // history is never overwritten: if the column set changed, append a new header; old rows stay
    let prev = std::fs::read_to_string(&path).unwrap_or_default();
    let last_header = prev.lines().filter(|l| l.starts_with("time\t")).last();
    let mut text = prev.clone();
    if last_header != Some(header.as_str()) { text.push_str(&header); text.push('\n'); }
    text.push_str(&row);
    text.push('\n');
    std::fs::write(&path, &text)?;
    println!("{header}\n{row}");
    Ok(())
}
