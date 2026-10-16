//! External measurement (not a gate): `%!assert`, `%!test`, `%!error` blocks from GNU Octave source files. Octave is
//! GPL, so the files live only in `$MOVA_DATA/raw/tests-octave/` (`OCTAVE_TESTS`) and never enter the repo; the
//! Octave implementation code is not read — the tool takes only the `%!` lines. Each block is a separate
//! `mlab run` process with a 10 s limit (a hang is a finding too). In scope — blocks all of whose calls are mlab builtins.
//!
//! `cargo test --release --test octave_ext -- --ignored --nocapture` → report and `blocks.tsv` in `OCTAVE_OUT`.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap())
}

/// The data directory: $MOVA_DATA, else ~/mova-data.
fn mova_data() -> PathBuf {
    std::env::var("MOVA_DATA").map(PathBuf::from).unwrap_or_else(|_| home().join("mova-data"))
}

fn files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut v: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path())).collect();
    v.sort();
    for p in v {
        if p.is_dir() {
            files(&p, out);
        } else if matches!(p.extension().and_then(|e| e.to_str()), Some("m" | "cc")) {
            out.push(p);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Assert,
    Test,
    Error,
}

struct Block {
    kind: Kind,
    code: String,
}

/// Blocks of a file: `%!shared` and `%!function` are prepended to every following test of the file.
fn blocks(text: &str) -> Vec<Block> {
    let mut raw: Vec<(String, Vec<String>)> = Vec::new();
    for l in text.lines() {
        let Some(rest) = l.strip_prefix("%!") else { continue };
        let kw: String = rest.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
        let known = ["assert", "test", "error", "shared", "function", "endfunction", "xtest", "testif", "warning", "demo", "fail", "fixme"];
        if !kw.is_empty() && known.contains(&kw.as_str()) && (rest.len() == kw.len() || !rest[kw.len()..].starts_with(|c: char| c.is_alphanumeric())) {
            raw.push((kw.clone(), vec![rest[kw.len()..].to_string()]));
        } else if let Some(last) = raw.last_mut() {
            last.1.push(rest.to_string());
        }
    }
    let mut shared = String::new();
    let mut funcs = String::new();
    let mut out = Vec::new();
    let mut in_func: Option<String> = None;
    for (kw, lines) in raw {
        if let Some(f) = in_func.as_mut() {
            if kw == "endfunction" {
                funcs += f;
                funcs += "endfunction\n";
                in_func = None;
                continue;
            }
        }
        match kw.as_str() {
            "shared" => {
                // «%!shared a, b» + initialization lines
                let vars: Vec<&str> = lines[0].split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
                shared = vars.iter().map(|v| format!("{v} = [];\n")).collect::<String>() + &lines[1..].join("\n") + "\n";
            }
            "function" => in_func = Some(format!("function{}\n", lines.join("\n"))),
            "assert" => out.push(Block { kind: Kind::Assert, code: format!("{shared}assert{}\n{funcs}", lines.join("\n")) }),
            "test" => out.push(Block { kind: Kind::Test, code: format!("{shared}{}\n{funcs}", lines.join("\n")) }),
            "error" => {
                // «%!error <pattern> code», «%!error id=… code»
                let mut first = lines[0].trim_start().to_string();
                if first.starts_with("id=") {
                    first = first.split_once(' ').map_or(String::new(), |x| x.1.trim_start().to_string());
                }
                if first.starts_with('<') {
                    first = first.find('>').map_or(String::new(), |i| first[i + 1..].to_string());
                }
                let body = std::iter::once(first).chain(lines[1..].iter().cloned()).collect::<Vec<_>>().join("\n");
                out.push(Block { kind: Kind::Error, code: format!("{shared}{body}\n{funcs}") });
            }
            _ => {}
        }
    }
    out
}

/// Call names `name (` / `name(` in code (without strings and comments — crude, but enough for a filter).
fn calls(code: &str) -> Vec<String> {
    let mut out = Vec::new();
    let b = code.as_bytes();
    let mut i = 0;
    let mut in_str = false;
    while i < b.len() {
        let c = b[i] as char;
        if c == '"' {
            in_str = !in_str;
        }
        if !in_str && (c.is_ascii_alphabetic()) && (i == 0 || !(b[i - 1] as char).is_ascii_alphanumeric() && b[i - 1] != b'_' && b[i - 1] != b'.') {
            let s = i;
            while i < b.len() && ((b[i] as char).is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            let mut j = i;
            while j < b.len() && b[j] == b' ' {
                j += 1;
            }
            if j < b.len() && b[j] == b'(' {
                out.push(code[s..i].to_string());
            }
            continue;
        }
        i += 1;
    }
    out
}

fn assigned(code: &str) -> HashSet<String> {
    let mut s = HashSet::new();
    for l in code.lines() {
        let l = l.trim_start();
        if let Some(name) = l.strip_prefix("function") {
            // function [a, b] = name (args) / function name (args)
            let head = name.split('(').next().unwrap_or("");
            let nm = head.rsplit('=').next().unwrap_or("").trim();
            s.insert(nm.to_string());
            for a in name.split('(').nth(1).unwrap_or("").split(')').next().unwrap_or("").split(',') {
                s.insert(a.trim().to_string());
            }
            continue;
        }
        if let Some((lhs, rhs)) = l.split_once('=') {
            if !rhs.starts_with('=') && !lhs.ends_with(['<', '>', '~', '!', '=']) {
                for v in lhs.trim_matches(|c| c == '[' || c == ']' || c == ' ').split([',', ' ']) {
                    let v = v.split('(').next().unwrap_or("").trim();
                    if !v.is_empty() {
                        s.insert(v.to_string());
                    }
                }
            }
        }
        if let Some(r) = l.strip_prefix("for ") {
            s.insert(r.split('=').next().unwrap_or("").trim().to_string());
        }
    }
    s
}

#[test]
#[ignore]
fn octave_test_blocks_external_measure() {
    let root = std::env::var("OCTAVE_TESTS").map(PathBuf::from).unwrap_or_else(|_| mova_data().join("raw/tests-octave/octave/octave-11.3.0"));
    let outdir = std::env::var("OCTAVE_OUT").map(PathBuf::from).unwrap_or_else(|_| mova_data().join("runs/tests3-2026-09-26/octave"));
    std::fs::create_dir_all(&outdir).unwrap();
    let exe = env!("CARGO_BIN_EXE_mlab");
    let list = Command::new(exe).arg("builtins").output().unwrap();
    let mut known: HashSet<String> = String::from_utf8_lossy(&list.stdout).split_whitespace().map(str::to_string).collect();
    for k in ["if", "for", "while", "switch", "function", "assert", "error", "end", "return"] {
        known.insert(k.to_string());
    }
    let mut fs = Vec::new();
    files(&root, &mut fs);
    let tmp = outdir.join("block.m");
    let mut tsv = String::from("file\tkind\tin_scope\tstatus\tdetail\n");
    let (mut total, mut scope, mut pass, mut fail_assert, mut fail_other, mut hang, mut crash) = (0, 0, 0, 0, 0, 0, 0);
    let only_scope = std::env::var("OCTAVE_ALL").is_err();
    for f in &fs {
        let text = std::fs::read_to_string(f).unwrap_or_default();
        let rel = f.strip_prefix(&root).unwrap_or(f).display().to_string();
        for (k, b) in blocks(&text).into_iter().enumerate() {
            total += 1;
            let vars = assigned(&b.code);
            let unknown: Vec<String> = calls(&b.code).into_iter().filter(|c| !known.contains(c) && !vars.contains(c)).collect();
            let in_scope = unknown.is_empty();
            if !in_scope && only_scope {
                continue;
            }
            scope += in_scope as usize;
            std::fs::write(&tmp, &b.code).unwrap();
            let o = Command::new("timeout").args(["10", exe, "run"]).arg(&tmp).output().unwrap();
            let err = String::from_utf8_lossy(&o.stderr).to_string() + &String::from_utf8_lossy(&o.stdout);
            let first_err = err.lines().find(|l| l.starts_with("error:") || l.contains("panicked")).unwrap_or("").to_string();
            let ok_exit = o.status.success() && first_err.is_empty();
            let status = if o.status.code() == Some(124) {
                hang += 1;
                "hang"
            } else if o.status.code().is_none() || err.contains("panicked") {
                crash += 1;
                std::fs::write(outdir.join(format!("crash-{crash}.m")), &b.code).unwrap();
                "crash"
            } else if b.kind == Kind::Error {
                if ok_exit {
                    fail_other += 1;
                    "no-error"
                } else {
                    pass += 1;
                    "pass"
                }
            } else if ok_exit {
                pass += 1;
                "pass"
            } else if first_err.contains("assert") {
                fail_assert += 1;
                "assert"
            } else {
                fail_other += 1;
                "error"
            };
            tsv += &format!("{rel}#{k}\t{:?}\t{in_scope}\t{status}\t{}\n", b.kind, first_err.replace('\t', " ").chars().take(160).collect::<String>());
        }
    }
    std::fs::write(outdir.join("blocks.tsv"), &tsv).unwrap();
    let _ = std::fs::remove_file(&tmp);
    println!("files {}, blocks {total}, in scope {scope}: pass {pass}, assert {fail_assert}, other error {fail_other}, hang {hang}, crash {crash}", fs.len());
}
