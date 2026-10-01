//! LLM tests for LaTeX parsing: 100 cases from an independent author (Opus, did not see the code),
//! `tests/data/vmm/latex.tsv`, frozen by hash before the first run. Kinds:
//! same / diff — trees equal / different (`Node ==` without positions); err — parse error; ok — no error, no
//! `Unknown` and a green round trip; count — number of formulas in the fragment (`\n` — newline); macro —
//! a fragment with definitions and one formula equals the formula without macros; value — the formula's value via
//! the bridge to `math::Expr` (only with `--features math`). Reds are only reported (README, "Independent tests").

use latex::{Macros, extract, gate, parse, parse_with};

const DATA: &str = include_str!("data/vmm/latex.tsv");
const FROZEN: &str = "7f53c952033496e3490bf0bbdbb5c0698d5e3d3fedb303669f0cbff5ea292abc"; // re-hashed after translation (comment lines only)

/// (id, class, why)
const KNOWN_RED: &[(&str, &str, &str)] = &[(
    "vmm-tex-088",
    "test error",
    "prompt format: \"\\n\" is both a line break and the start of a command; \"…commented\\nand \\(c\\)\" is ambiguous (\\nand), without a break the comment swallows \\(c\\)",
)];

fn sha256(data: &[u8]) -> String {
    let tmp = std::env::temp_dir().join(format!("latex-vmm-{}.bin", std::process::id()));
    std::fs::write(&tmp, data).unwrap();
    let out = std::process::Command::new("sha256sum").arg(&tmp).output().expect("sha256sum");
    let _ = std::fs::remove_file(&tmp);
    String::from_utf8_lossy(&out.stdout).split_whitespace().next().unwrap_or("").to_string()
}

#[cfg(feature = "math")]
fn value(a: &str, b: &str) -> Result<bool, String> {
    let n = parse(a).map_err(|e| format!("parse: {e:?}"))?;
    let e = latex::tomath::to_expr(&n).map_err(|e| format!("bridge: {}", e.kind()))?;
    let got = math::fcheck::eval_f64(&e).ok_or("evaluation")?;
    let want: f64 = match b.strip_prefix('~') {
        Some(d) => d.parse().map_err(|_| "expected")?,
        None => match b.split_once('/') {
            Some((p, q)) => p.trim().parse::<f64>().map_err(|_| "expected")? / q.trim().parse::<f64>().map_err(|_| "expected")?,
            None => b.parse().map_err(|_| "expected")?,
        },
    };
    let tol = if b.starts_with('~') { 1e-11 } else { 1e-12 };
    Ok((got - want).abs() <= tol * want.abs().max(1.0))
}

/// One case: Ok(true) — green, Ok(false) — red, Err — red with a reason. `None` — not
/// run in this build (value without feature math).
fn case(kind: &str, a: &str, b: &str) -> Option<Result<bool, String>> {
    // "\n" is a newline only when not followed by a letter: \newcommand, \norm, \neq are commands, not breaks
    let nl = |s: &str| {
        let mut out = String::new();
        let cs: Vec<char> = s.chars().collect();
        let mut i = 0;
        while i < cs.len() {
            if cs[i] == '\\' && cs.get(i + 1) == Some(&'n') && !cs.get(i + 2).is_some_and(|c| c.is_ascii_alphabetic()) {
                out.push('\n');
                i += 2;
            } else {
                out.push(cs[i]);
                i += 1;
            }
        }
        out
    };
    Some(match kind {
        "same" | "diff" => match (parse(a), parse(b)) {
            (Ok(x), Ok(y)) => Ok((x == y) == (kind == "same")),
            (x, y) => Err(format!("parse error: a {:?}, b {:?}", x.err().map(|e| e.kind.code()), y.err().map(|e| e.kind.code()))),
        },
        "err" => Ok(parse(a).is_err()),
        "ok" => match parse(a) {
            Ok(n) if n.has_unknown() => Err(format!("unknown: {:?}", n.unknowns().iter().map(|u| u.0.clone()).collect::<Vec<_>>())),
            Ok(n) => Ok(gate::check(&n).is_ok()),
            Err(e) => Err(format!("parse error {}", e.kind.code())),
        },
        "count" => {
            let doc = nl(a);
            let (fs, _) = extract(&doc);
            let want: usize = b.trim().parse().unwrap_or(usize::MAX);
            if fs.len() == want { Ok(true) } else { Err(format!("found {}, expected {want}", fs.len())) }
        }
        "macro" => {
            let doc = nl(a);
            let mut m = Macros::new();
            m.scan(&doc);
            let (fs, _) = extract(&doc);
            let Some(f) = fs.iter().find(|f| !f.body.contains("\\newcommand") && !f.body.contains("\\def")) else {
                return Some(Err("formula not found".into()));
            };
            match (parse_with(f.body, &m), parse(b)) {
                (Ok(x), Ok(y)) => Ok(x == y),
                (x, y) => Err(format!("parse error: {:?} / {:?}", x.err().map(|e| e.kind.code()), y.err().map(|e| e.kind.code()))),
            }
        }
        #[cfg(feature = "math")]
        "value" => value(a, b),
        #[cfg(not(feature = "math"))]
        "value" => return None,
        other => Err(format!("unknown kind {other}")),
    })
}

fn run(data: &str) -> (usize, usize, Vec<(String, String, String)>) {
    let (mut n, mut ran) = (0, 0);
    let mut red = Vec::new();
    for l in data.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let f: Vec<&str> = l.split('\t').collect();
        n += 1;
        let Some(r) = case(f[1], f[2], f[3]) else { continue };
        ran += 1;
        match r {
            Ok(true) => {}
            Ok(false) => red.push((f[0].to_string(), f[1].to_string(), "wrong verdict".to_string())),
            Err(e) => red.push((f[0].to_string(), f[1].to_string(), e)),
        }
    }
    (n, ran, red)
}

#[test]
fn vmm_latex_frozen_and_classified() {
    assert_eq!(sha256(DATA.as_bytes()), FROZEN, "latex.tsv changed after freezing");
    let (n, ran, red) = run(DATA);
    assert_eq!(n, 100);
    println!("ran {ran} of {n}, red {}", red.len());
    for r in &red {
        println!("✗ {}\t{}\t{}", r.0, r.1, r.2);
    }
    let mut ids: Vec<&str> = red.iter().map(|r| r.0.as_str()).collect();
    ids.sort();
    let mut want: Vec<&str> = KNOWN_RED.iter().filter(|k| cfg!(feature = "math") || !k.1.starts_with("value")).map(|k| k.0).collect();
    want.sort();
    assert_eq!(ids, want, "the set of reds changed");
}

/// Negative control: same ↔ diff and ok → err must turn red.
#[test]
fn vmm_latex_negative_control() {
    let bad = DATA.replace("\tsame\t", "\tdiff\t");
    let (_, _, red) = run(&bad);
    assert!(red.len() >= 15, "swapping same → diff gave only {} reds", red.len());
}
