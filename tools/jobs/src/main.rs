//! Heavy job runner. Design note: spiders and other modules should work via callbacks or queues, like
//! node.js; every job should carry estimates, heuristics and measurements of RAM and CPU ticks; recursion runs as
//! subtasks; then the system can see that a task eats too much CPU, memory leaks, or recursion loops.
//!
//!   jobs run --name N [--mem-gb G] [--secs S] [--est-mem-gb E] [--est-secs T] -- command args…
//!   jobs log                  — job log (estimate vs actual)
//!
//! Every second: peak memory (RSS of the whole process tree) and CPU time. Exceeding --mem-gb or --secs
//! stops only this job (SIGTERM, SIGKILL after 5 s), not the whole container. Log —
//! `$MOVA_DATA/runs/jobs.jsonl`: name, estimate, actual, stop reason.

use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};

fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
}

/// The data directory: $MOVA_DATA, else ~/mova-data.
fn mova_data() -> PathBuf {
    std::env::var("MOVA_DATA").map(PathBuf::from).unwrap_or_else(|_| home().join("mova-data"))
}

/// Children of a process (recursively), from /proc/<pid>/task/*/children.
fn tree(pid: u32) -> Vec<u32> {
    let mut out = vec![pid];
    let mut i = 0;
    while i < out.len() {
        let p = out[i];
        if let Ok(ts) = std::fs::read_dir(format!("/proc/{p}/task")) {
            for t in ts.flatten() {
                if let Ok(c) = std::fs::read_to_string(t.path().join("children")) {
                    out.extend(c.split_whitespace().filter_map(|x| x.parse::<u32>().ok()));
                }
            }
        }
        i += 1;
    }
    out
}

/// RSS (KB) and CPU time (s) of a process tree.
fn usage(pid: u32) -> (u64, f64) {
    let tick = 100.0; // CLK_TCK on Linux
    let mut rss = 0;
    let mut cpu = 0.0;
    for p in tree(pid) {
        if let Ok(s) = std::fs::read_to_string(format!("/proc/{p}/status")) {
            rss += s.lines().find(|l| l.starts_with("VmRSS:")).and_then(|l| l.split_whitespace().nth(1)).and_then(|x| x.parse::<u64>().ok()).unwrap_or(0);
        }
        if let Ok(s) = std::fs::read_to_string(format!("/proc/{p}/stat")) {
            // fields after ")": utime is the 12th, stime the 13th (0-based)
            if let Some(rest) = s.rsplit(')').next() {
                let f: Vec<&str> = rest.split_whitespace().collect();
                let ut: f64 = f.get(11).and_then(|x| x.parse().ok()).unwrap_or(0.0);
                let st: f64 = f.get(12).and_then(|x| x.parse().ok()).unwrap_or(0.0);
                cpu += (ut + st) / tick;
            }
        }
    }
    (rss, cpu)
}

fn flag(args: &[String], k: &str) -> Option<f64> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let log = mova_data().join("runs/jobs.jsonl");
    match args.first().map(|s| s.as_str()) {
        Some("log") => {
            print!("{}", std::fs::read_to_string(&log).unwrap_or_default());
            return Ok(());
        }
        Some("run") => {}
        _ => bail!("usage: jobs run --name N [--mem-gb G] [--secs S] [--est-mem-gb E] [--est-secs T] -- command …  |  jobs log"),
    }
    let sep = args.iter().position(|a| a == "--").context("missing \"--\" before the command")?;
    let opts = &args[..sep];
    let cmd = &args[sep + 1..];
    let name = opts.iter().position(|a| a == "--name").and_then(|i| opts.get(i + 1)).cloned().unwrap_or_else(|| cmd.first().cloned().unwrap_or_default());
    let mem_kb = (flag(opts, "--mem-gb").unwrap_or(16.0) * 1024.0 * 1024.0) as u64;
    let secs = flag(opts, "--secs").unwrap_or(7200.0);
    let (est_mem, est_secs) = (flag(opts, "--est-mem-gb"), flag(opts, "--est-secs"));
    let t0 = Instant::now();
    let mut child = Command::new(&cmd[0]).args(&cmd[1..]).spawn().with_context(|| format!("spawn {}", cmd[0]))?;
    let pid = child.id();
    let (mut peak, mut cpu, mut reason) = (0u64, 0.0f64, String::from("ok"));
    let code = loop {
        if let Some(st) = child.try_wait()? { break st.code().unwrap_or(-1); }
        let (rss, c) = usage(pid);
        peak = peak.max(rss);
        cpu = cpu.max(c);
        let over = if rss > mem_kb { Some(format!("memory {} MB > limit {} MB", rss / 1024, mem_kb / 1024)) } else if t0.elapsed().as_secs_f64() > secs { Some(format!("time > {secs} s")) } else { None };
        if let Some(r) = over {
            eprintln!("jobs: stopping \"{name}\" — {r}");
            for p in tree(pid).into_iter().rev() { let _ = Command::new("kill").arg("-TERM").arg(p.to_string()).status(); }
            std::thread::sleep(Duration::from_secs(5));
            for p in tree(pid).into_iter().rev() { let _ = Command::new("kill").arg("-KILL").arg(p.to_string()).status(); }
            let _ = child.wait();
            reason = r;
            break -9;
        }
        std::thread::sleep(Duration::from_millis(1000));
    };
    let wall = t0.elapsed().as_secs_f64();
    let rec = format!(
        "{{\"name\":{:?},\"cmd\":{:?},\"est_mem_gb\":{},\"est_secs\":{},\"peak_mem_gb\":{:.2},\"cpu_secs\":{:.0},\"wall_secs\":{:.0},\"code\":{code},\"reason\":{:?}}}",
        name, cmd.join(" "), est_mem.map(|x| x.to_string()).unwrap_or("null".into()), est_secs.map(|x| x.to_string()).unwrap_or("null".into()), peak as f64 / 1048576.0, cpu, wall, reason
    );
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&log)?;
    writeln!(f, "{rec}")?;
    eprintln!("jobs: \"{name}\" — peak memory {:.2} GB{}, CPU {:.0} s, wall {:.0} s{}, {reason}", peak as f64 / 1048576.0, est_mem.map(|e| format!(" (estimate {e} GB)")).unwrap_or_default(), cpu, wall, est_secs.map(|e| format!(" (estimate {e} s)")).unwrap_or_default());
    std::process::exit(code);
}
