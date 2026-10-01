//! Sandbox for real execution — only for verifying the task set (references) and for the judge control
//! at the end. Execution results never enter the training loop.
//!
//! Safety:
//! 1. static filter — any name from `DENY` among the program's identifiers (mlab lexer; comments and
//!    strings are not counted, but `feval`/`str2func`/`eval` with a string are blocked separately) — the program is not run;
//! 2. a separate `coder exec-one` process under `ulimit -v` (memory) and `ulimit -t` (CPU), wall-clock limit 2 s —
//!    after that the process is killed.

use std::io::{Read as _, Write as _};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// File, system and "meta" calls: they never occur in the sandbox.
pub const DENY: &[&str] = &[
    "readtable", "writetable", "readmatrix", "writematrix", "fopen", "fclose", "fread", "fwrite", "fgetl", "fgets",
    "fscanf", "fileread", "writelines", "tempname", "tempdir", "system", "unix", "dos", "shell", "exist", "delete",
    "mkdir", "rmdir", "cd", "dir", "ls", "pwd", "save", "load", "input", "keyboard", "eval", "evalin", "evalc",
    "assignin", "str2func", "getenv", "setenv", "urlread", "webread", "websave", "copyfile", "movefile", "diary",
];

pub const WALL_SECS: f64 = 2.0;
pub const MEM_KB: u64 = 1_048_576; // 1 GiB of virtual memory

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Exec {
    /// executed; output (merged mlab stream)
    Ran(String),
    /// rejected statically: the names found
    Denied(Vec<String>),
    /// time limit exceeded
    Timeout,
    /// process crashed (memory, signal)
    Crashed(String),
}

/// Static check: identifiers from `DENY` (via the mlab lexer; if the lexer fails — by words).
pub fn denied(src: &str) -> Vec<String> {
    let mut hits: Vec<String> = Vec::new();
    let words: Vec<String> = match mlab::lexer::lex(src) {
        Ok(toks) => toks
            .iter()
            .filter_map(|t| match &t.tok {
                mlab::lexer::Tok::Ident(s) => Some(s.clone()),
                _ => None,
            })
            .collect(),
        Err(_) => src.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).map(str::to_string).collect(),
    };
    for w in &words {
        if DENY.contains(&w.as_str()) && !hits.contains(w) {
            hits.push(w.clone());
        }
    }
    // feval with a string can call anything — block it
    let flat: String = src.split_whitespace().collect();
    if flat.contains("feval('") || flat.contains("feval(\"") {
        hits.push("feval(string)".into());
    }
    hits
}

/// Run a program in a separate process with limits.
pub fn run(src: &str) -> Result<Exec> {
    let d = denied(src);
    if !d.is_empty() {
        return Ok(Exec::Denied(d));
    }
    let exe = std::env::current_exe()?;
    let script = format!("ulimit -v {MEM_KB}; ulimit -t 3; exec \"$0\" exec-one");
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(&script)
        .arg(&exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("sh exec-one")?;
    child.stdin.take().context("stdin")?.write_all(src.as_bytes())?;
    let t0 = Instant::now();
    loop {
        if let Some(st) = child.try_wait()? {
            let mut out = String::new();
            child.stdout.take().context("stdout")?.read_to_string(&mut out)?;
            if !st.success() {
                let mut err = String::new();
                child.stderr.take().context("stderr")?.read_to_string(&mut err)?;
                return Ok(Exec::Crashed(format!("{st}: {}", err.chars().take(200).collect::<String>())));
            }
            return Ok(Exec::Ran(out));
        }
        if t0.elapsed().as_secs_f64() > WALL_SECS {
            let _ = child.kill();
            let _ = child.wait();
            return Ok(Exec::Timeout);
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// Child process body: program from stdin → mlab output to stdout (warning/error in the same stream).
pub fn exec_one() -> Result<()> {
    let mut src = String::new();
    std::io::stdin().read_to_string(&mut src)?;
    let out = mlab::run_capture(&src);
    std::io::stdout().write_all(out.as_bytes())?;
    Ok(())
}

/// Output matches the expected one (line by line; trailing spaces on a line are ignored — as in the judged tasks).
pub fn same_output(got: &str, expected: &[String]) -> bool {
    let g: Vec<&str> = got.strip_suffix('\n').unwrap_or(got).split('\n').collect();
    let g: Vec<&str> = if got.is_empty() { vec![] } else { g };
    g.len() == expected.len() && g.iter().zip(expected).all(|(a, b)| a.trim_end() == b.trim_end())
}
