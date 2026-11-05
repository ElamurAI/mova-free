//! mova-json CLI: transport over the library; every subcommand is deterministic and fails fast.

use mova_json::merge::{merge3_file, rebind_groupant, rebind_manifest};
use mova_json::*;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "usage:
  mova-json check FILE
  mova-json get FILE PATH [--raw]
  mova-json set FILE PATH VALUE [--str]
  mova-json del FILE PATH
  mova-json push FILE PATH VALUE... [--str]
  mova-json insert-before FILE PATH ANCHOR VALUE... [--str]
  mova-json remove-value FILE PATH VALUE [--str]
  mova-json replace-str FILE OLD NEW
  mova-json rebind REPO MANIFEST... --paths P... [--add]
  mova-json merge3 FILE... [--rebind-all]
  mova-json rebind-groupant REPO BASE_COMMIT FILE...
  mova-json jsonl-count FILE PATH
  mova-json jsonl-filter FILE PATH=VALUE
PATH: a.b[2].c[name=x] (\\ escapes; \"\" or . is the root). VALUE: JSON text, or a plain string with --str.
Writing commands accept --reformat for files whose layout is not reproduced byte for byte.
`--` ends flags (for --str values that start with --).";

struct Args {
    pos: Vec<String>,
    paths: Vec<String>,
    str_values: bool,
    raw: bool,
    reformat: bool,
    add: bool,
    rebind_all: bool,
}

fn parse_args(argv: Vec<String>) -> Result<Args, String> {
    let mut a = Args { pos: vec![], paths: vec![], str_values: false, raw: false, reformat: false, add: false, rebind_all: false };
    let mut it = argv.into_iter();
    let mut in_paths = false;
    while let Some(x) = it.next() {
        match x.as_str() {
            "--" => {
                let rest: Vec<String> = it.by_ref().collect();
                if in_paths { a.paths.extend(rest) } else { a.pos.extend(rest) }
                break;
            }
            "--str" => a.str_values = true,
            "--raw" => a.raw = true,
            "--reformat" => a.reformat = true,
            "--add" => a.add = true,
            "--rebind-all" => a.rebind_all = true,
            "--paths" => in_paths = true,
            f if f.starts_with("--") => return Err(format!("unknown flag {f}")),
            _ if in_paths => a.paths.push(x),
            _ => a.pos.push(x),
        }
    }
    Ok(a)
}

fn value_arg(s: &str, as_str: bool) -> Res<Value> {
    if as_str {
        Ok(Value::string(s))
    } else {
        parse(s).map_err(|e| format!("value {s:?}: {e} (use --str for a plain string)"))
    }
}

fn edit(a: &Args, f: impl FnOnce(&mut Value, &[Seg]) -> Res<String>) -> Res<()> {
    let file = Path::new(&a.pos[1]);
    let path = parse_path(&a.pos[2])?;
    let mut doc = load_for_edit(file, a.reformat)?;
    let msg = f(&mut doc.value, &path)?;
    let w = save(file, &doc)?;
    println!("{msg}{}", if w { "" } else { " (file unchanged)" });
    Ok(())
}

fn run(a: Args) -> Result<u8, String> {
    let n = a.pos.len();
    let need = |k: usize, exact: bool| -> Res<()> {
        if (exact && n != k) || n < k {
            Err(USAGE.to_string())
        } else {
            Ok(())
        }
    };
    let cmd = a.pos.first().map(String::as_str).unwrap_or("");
    match cmd {
        "check" => {
            need(2, true)?;
            let d = load_file(Path::new(&a.pos[1]))?;
            println!("layout {:?} trailing_newline {} byte_identical {}", d.style.layout, d.style.newline, d.canonical);
            Ok(if d.canonical { 0 } else { 4 })
        }
        "get" => {
            need(3, true)?;
            let d = load_file(Path::new(&a.pos[1]))?;
            let v = get(&d.value, &parse_path(&a.pos[2])?)?;
            match (a.raw, v.as_str()) {
                (true, Some(s)) => println!("{s}"),
                _ => println!("{}", render(v, Style { layout: d.style.layout, newline: false })),
            }
            Ok(0)
        }
        "set" => {
            need(4, true)?;
            let v = value_arg(&a.pos[3], a.str_values)?;
            edit(&a, |root, p| set(root, p, v).map(|_| "set".to_string()))?;
            Ok(0)
        }
        "del" => {
            need(3, true)?;
            edit(&a, |root, p| del(root, p).map(|v| format!("deleted {}", compact(&v))))?;
            Ok(0)
        }
        "push" => {
            need(4, false)?;
            let vals = a.pos[3..].iter().map(|s| value_arg(s, a.str_values)).collect::<Res<Vec<_>>>()?;
            let k = vals.len();
            edit(&a, |root, p| push(root, p, vals).map(|_| format!("pushed {k}")))?;
            Ok(0)
        }
        "insert-before" => {
            need(5, false)?;
            let anchor = value_arg(&a.pos[3], a.str_values)?;
            let vals = a.pos[4..].iter().map(|s| value_arg(s, a.str_values)).collect::<Res<Vec<_>>>()?;
            let k = vals.len();
            edit(&a, |root, p| insert_before(root, p, &anchor, vals).map(|i| format!("inserted {k} at index {i}")))?;
            Ok(0)
        }
        "remove-value" => {
            need(4, true)?;
            let v = value_arg(&a.pos[3], a.str_values)?;
            edit(&a, |root, p| remove_value(root, p, &v).map(|k| format!("removed {k}")))?;
            Ok(0)
        }
        "replace-str" => {
            need(4, true)?;
            if a.pos[2].is_empty() {
                return Err("OLD must not be empty".into());
            }
            let file = Path::new(&a.pos[1]);
            let mut doc = load_for_edit(file, a.reformat)?;
            let (strings, occ) = replace_str(&mut doc.value, &a.pos[2], &a.pos[3]);
            if strings > 0 {
                save(file, &doc)?;
            }
            println!("{occ} occurrences in {strings} strings");
            Ok(0)
        }
        "rebind" => {
            need(3, false)?;
            if a.paths.is_empty() {
                return Err("rebind needs --paths P...".into());
            }
            let repo = PathBuf::from(&a.pos[1]);
            for m in &a.pos[2..] {
                let k = rebind_manifest(&repo, Path::new(m), &a.paths, a.add, a.reformat)?;
                println!("{m} {k}");
            }
            Ok(0)
        }
        "merge3" => {
            need(2, false)?;
            let mut status = 0;
            for f in &a.pos[1..] {
                let r = merge3_file(f, a.rebind_all, a.reformat)?;
                for x in &r.resolved {
                    println!("{f}: resolved {x}");
                }
                for c in &r.conflicts {
                    println!("{f}: CONFLICT {}: {}", c.at, c.what);
                }
                for p in &r.unresolved_paths {
                    println!("{f}: binding not found on disk: {p}");
                }
                if r.conflicts.is_empty() {
                    println!("{f}: merged{}{}", if r.written { "" } else { " (file unchanged)" }, if a.rebind_all { format!(", {} bindings refreshed", r.rebound) } else { String::new() });
                } else {
                    println!("{f}: {} conflicts, file not written", r.conflicts.len());
                    status = 3;
                }
            }
            Ok(status)
        }
        "rebind-groupant" => {
            need(4, false)?;
            let repo = PathBuf::from(&a.pos[1]);
            for f in &a.pos[3..] {
                for line in rebind_groupant(&repo, &a.pos[2], f, a.reformat)? {
                    println!("{line}");
                }
            }
            Ok(0)
        }
        "jsonl-count" => {
            need(3, true)?;
            let text = std::fs::read_to_string(&a.pos[1]).map_err(|e| format!("{}: {e}", a.pos[1]))?;
            let (c, invalid) = jsonl_count(&text, &parse_path(&a.pos[2])?);
            for (k, v) in sorted_counts(&c) {
                println!("{k}\t{v}");
            }
            if !invalid.is_empty() {
                eprintln!("{} invalid lines (first at line {})", invalid.len(), invalid[0]);
            }
            Ok(0)
        }
        "jsonl-filter" => {
            need(3, true)?;
            let text = std::fs::read_to_string(&a.pos[1]).map_err(|e| format!("{}: {e}", a.pos[1]))?;
            let (p, want) = split_assignment(&a.pos[2])?;
            for l in jsonl_filter(&text, &parse_path(p)?, want) {
                println!("{l}");
            }
            Ok(0)
        }
        _ => Err(USAGE.to_string()),
    }
}

fn main() -> ExitCode {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    match parse_args(argv).and_then(run) {
        Ok(c) => ExitCode::from(c),
        Err(e) if e == USAGE => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
        Err(e) => {
            eprintln!("mova-json: {e}");
            ExitCode::from(1)
        }
    }
}
