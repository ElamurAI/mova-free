//! LLM tests for the exact expression language (`calc`): 100 expressions with exact answers from an independent author
//! (Opus, never saw the code), `tests/data/vmm/calc.tsv`, frozen by hash before the first run. The expected value is
//! also evaluated by `calc` (these are literals — integer, fraction, true/false, list), so we compare values, not notation.
//! ERR — the expression must produce an error. Reds — only analysed ones (README section "Independent tests").

use math::calc;

const DATA: &str = include_str!("data/vmm/calc.tsv");
const FROZEN: &str = "72700011d95e6e7ff0d16ed298a12cedeec07e64a53391870e7d2212cf28b254";

/// (id, class, why)
const KNOWN_RED: &[(&str, &str, &str)] = &[];

fn eval(src: &str) -> Result<calc::V, String> {
    let e = calc::parse(src)?;
    let mut env = calc::Env::new(math::nt::Limits::with_secs(20.0));
    calc::eval(&e, &mut env)
}

fn sha256(data: &[u8]) -> String {
    let tmp = std::env::temp_dir().join(format!("math-vmm-{}.bin", std::process::id()));
    std::fs::write(&tmp, data).unwrap();
    let out = std::process::Command::new("sha256sum").arg(&tmp).output().expect("sha256sum");
    let _ = std::fs::remove_file(&tmp);
    String::from_utf8_lossy(&out.stdout).split_whitespace().next().unwrap_or("").to_string()
}

/// (id, expression, expected, what came out) for the reds.
fn run(data: &str) -> (usize, Vec<(String, String, String, String)>) {
    let mut red = Vec::new();
    let mut n = 0;
    for l in data.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let f: Vec<&str> = l.split('\t').collect();
        let (id, expr, want) = (f[0], f[1], f[2].trim());
        n += 1;
        let got = eval(expr);
        let ok = match (&got, want) {
            (Err(_), "ERR") => true,
            (Ok(_), "ERR") | (Err(_), _) => false,
            (Ok(v), w) => match eval(w) {
                Ok(wv) => wv.exact() == v.exact(),
                Err(e) => panic!("{id}: expected «{w}» does not parse: {e}"),
            },
        };
        if !ok {
            let g = match got {
                Ok(v) => v.exact(),
                Err(e) => format!("ERR: {e}"),
            };
            red.push((id.to_string(), expr.to_string(), want.to_string(), g));
        }
    }
    (n, red)
}

#[test]
fn vmm_calc_frozen_and_classified() {
    assert_eq!(sha256(DATA.as_bytes()), FROZEN, "calc.tsv changed after freezing");
    let (n, red) = run(DATA);
    assert_eq!(n, 100);
    for r in &red {
        println!("✗ {}\t{}\texpected {}\tgot {}", r.0, r.1, r.2, r.3);
    }
    let mut ids: Vec<&str> = red.iter().map(|r| r.0.as_str()).collect();
    ids.sort();
    let mut want: Vec<&str> = KNOWN_RED.iter().map(|k| k.0).collect();
    want.sort();
    assert_eq!(ids, want, "the set of reds changed");
}

/// Negative control: a wrong expected value (1/2 → 1/3 in the first row, «true» → «false») must turn red.
#[test]
fn vmm_calc_negative_control() {
    let bad = DATA.replacen("\t1/2\t", "\t1/3\t", 1).replacen("\ttrue\t", "\tfalse\t", 1);
    let (_, red) = run(&bad);
    assert_eq!(red.len(), 2, "{red:?}");
}
