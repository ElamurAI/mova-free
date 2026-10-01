//! NIST StRD (Statistical Reference Datasets, public domain): certified values for linear
//! regression, one-way analysis of variance and descriptive statistics. The `.dat` files are as is, from the NIST site
//! (`tests/data/ext/nist/`). Metric — LRE (log relative error, McCullough 1998): the number of correct
//! significant digits, −log10(|q − c| / |c|), capped at 15 (15 digits are certified). mlab code is built from the file's
//! data and executed by the interpreter; thresholds are a lower bound below which the test turns red.
//!
//! `cargo test --release --test ext_nist -- --nocapture` prints the LRE table.

use mlab::Interp;

const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/ext/nist");

fn read(name: &str) -> String {
    std::fs::read_to_string(format!("{DIR}/{name}.dat")).unwrap_or_else(|e| panic!("{name}.dat: {e}"))
}

/// «Data (lines A to B)» from the file header → data lines.
fn data_rows(text: &str) -> Vec<Vec<f64>> {
    let lines: Vec<&str> = text.lines().collect();
    let hdr = lines.iter().find(|l| l.contains("Data") && l.contains("lines")).expect("line «Data (lines A to B)»");
    let hdr = &hdr[hdr.find("lines").unwrap()..];
    let nums: Vec<usize> = hdr.split(|c: char| !c.is_ascii_digit()).filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
    let (a, b) = (nums[0], nums[1]);
    lines[a - 1..b]
        .iter()
        .filter_map(|l| {
            let v: Vec<f64> = l.split_whitespace().filter_map(|t| t.parse().ok()).collect();
            (!v.is_empty()).then_some(v)
        })
        .collect()
}

fn num(t: &str) -> f64 {
    t.trim().replace('D', "E").parse().unwrap_or_else(|_| panic!("number «{t}»"))
}

/// The number after a label in the line containing `label` (the last field of the line).
fn after(text: &str, label: &str) -> f64 {
    text.lines()
        .filter(|l| l.contains(label))
        .find_map(|l| l.split_whitespace().rev().find_map(|t| t.replace('D', "E").parse().ok()))
        .unwrap_or_else(|| panic!("label «{label}» with a number at the end of the line"))
}

fn lre(q: f64, c: f64) -> f64 {
    if q == c {
        return 15.0;
    }
    if !q.is_finite() {
        return 0.0;
    }
    let e = if c == 0.0 { q.abs() } else { (q - c).abs() / c.abs() };
    (-e.log10()).clamp(0.0, 15.0)
}

fn mat_literal(rows: &[Vec<f64>]) -> String {
    let mut s = String::from("[");
    for r in rows {
        for v in r {
            s += &format!("{v:e} ");
        }
        s += ";\n";
    }
    s + "]"
}

fn run(code: &str) -> Vec<f64> {
    let mut it = Interp::capture();
    it.run(code);
    let out = it.take_output();
    let vals: Vec<f64> = out.lines().filter_map(|l| l.trim().parse().ok()).collect();
    assert!(!out.contains("error:"), "mlab: {out}");
    vals
}

struct Row {
    name: &'static str,
    what: String,
    lre: f64,
}

fn min_lre(rows: &[Row], name: &str) -> f64 {
    rows.iter().filter(|r| r.name == name).map(|r| r.lre).fold(15.0, f64::min)
}

// ---------- linear regression ----------

/// (dataset, min. LRE of estimates and SE via fitlm, min. LRE via `\`)
const LLS: &[(&str, f64, f64)] = &[
    ("Norris", 13.0, 12.0),
    ("Pontius", 12.0, 13.0),
    ("NoInt1", 14.0, 14.0),
    ("NoInt2", 14.0, 14.0),
    // Filip: `\` honestly says «rank deficient» (as MATLAB does) — checked by `filip_backslash_warns`
    ("Filip", 7.0, 0.0),
    ("Longley", 12.5, 11.0),
    ("Wampler1", 9.5, 9.5),
    ("Wampler2", 12.5, 12.0),
    ("Wampler3", 9.0, 9.5),
    ("Wampler4", 7.5, 8.0),
    ("Wampler5", 5.5, 6.0),
];

fn lls_case(name: &'static str, out: &mut Vec<Row>) -> (f64, f64) {
    let text = read(name);
    // certified parameters: lines «Bk  estimate  SE»
    let mut est = Vec::new();
    let mut se = Vec::new();
    let mut has_b0 = false;
    for l in text.lines() {
        let f: Vec<&str> = l.split_whitespace().collect();
        if f.len() == 3 && f[0].starts_with('B') && f[0][1..].chars().all(|c| c.is_ascii_digit()) {
            has_b0 |= f[0] == "B0";
            est.push(num(f[1]));
            se.push(num(f[2]));
        }
    }
    let rsd = after(&text, "Standard Deviation  ");
    let r2 = after(&text, "R-Squared");
    let rows = data_rows(&text);
    let npred = rows[0].len() - 1;
    // model: several predictors — linear with an intercept (Longley); one — a polynomial of degree p − 1
    // (with B0) or y = B1·x (NoInt)
    let xdef = if npred > 1 {
        "X = D(:, 2:end);".to_string()
    } else {
        let deg = if has_b0 { est.len() - 1 } else { est.len() };
        format!("X = D(:, 2) .^ (1:{deg});")
    };
    let icpt = if has_b0 { "true" } else { "false" };
    let ones = if has_b0 { "[ones(rows(X), 1) X]" } else { "X" };
    let code = format!(
        "D = {};\ny = D(:, 1);\n{xdef}\n[C, S] = fitlm(X, y, 'Intercept', {icpt}, 'Display', 'off');\n\
         printf('%.17g\\n', C.Estimate, C.SE, S.RMSE, S.R2);\nb = {ones} \\ y;\nprintf('%.17g\\n', b);\n",
        mat_literal(&rows)
    );
    let v = run(&code);
    let p = est.len();
    assert_eq!(v.len(), 2 * p + 2 + p, "{name}: output {v:?}");
    let mut worst_fit: f64 = 15.0;
    let mut worst_bs: f64 = 15.0;
    for k in 0..p {
        let b = if has_b0 { k } else { k + 1 };
        let (a, s, d) = (lre(v[k], est[k]), lre(v[p + k], se[k]), lre(v[2 * p + 2 + k], est[k]));
        out.push(Row { name, what: format!("B{b} fitlm"), lre: a });
        out.push(Row { name, what: format!("SE(B{b}) fitlm"), lre: s });
        out.push(Row { name, what: format!("B{b} \\"), lre: d });
        worst_fit = worst_fit.min(a).min(s);
        worst_bs = worst_bs.min(d);
    }
    let (a, b) = (lre(v[2 * p], rsd), lre(v[2 * p + 1], r2));
    out.push(Row { name, what: "resid SD".into(), lre: a });
    out.push(Row { name, what: "R²".into(), lre: b });
    (worst_fit.min(a).min(b), worst_bs)
}

// ---------- ANOVA ----------

const ANOVA: &[(&str, f64)] = &[
    ("SiRstv", 13.0),
    ("SmLs01", 14.5),
    ("SmLs02", 14.5),
    ("SmLs03", 13.5),
    ("AtmWtAg", 10.0),
    ("SmLs04", 10.0),
    ("SmLs05", 9.5),
    ("SmLs06", 9.5),
    // 10^12 + 0.x: in double the data already carry an error of ~6·10^-5 on a deviation of ~0.1 — ceiling ~4 digits
    ("SmLs07", 3.5),
    ("SmLs08", 3.5),
    ("SmLs09", 3.5),
];

fn anova_case(name: &'static str, out: &mut Vec<Row>) -> f64 {
    let text = read(name);
    let field = |label: &str, k: usize| -> f64 {
        let l = text.lines().find(|l| l.trim_start().starts_with(label)).unwrap_or_else(|| panic!("{name}: {label}"));
        let f: Vec<&str> = l.split_whitespace().collect();
        // «Between Instrument df SS MS F»: numbers come after the source name
        let nums: Vec<f64> = f.iter().filter_map(|t| t.parse::<f64>().ok()).collect();
        nums[k]
    };
    let (df1, ssb, msb, fstat) = (field("Between", 0), field("Between", 1), field("Between", 2), field("Between", 3));
    let (df2, ssw, msw) = (field("Within", 0), field("Within", 1), field("Within", 2));
    let r2 = after(&text, "Certified R-Squared");
    let rsd = after(&text, "Standard Deviation");
    let rows = data_rows(&text);
    let code = format!(
        "D = {};\n[p, t] = anova1(D(:, 2), D(:, 1), 'off');\n\
         printf('%.17g\\n', t.df(1), t.df(2), t.SS(1), t.SS(2), t.MS(1), t.MS(2), t.F(1), t.SS(1) / (t.SS(1) + t.SS(2)), sqrt(t.MS(2)));\n",
        mat_literal(&rows)
    );
    let v = run(&code);
    assert_eq!(v.len(), 9, "{name}: {v:?}");
    assert_eq!((v[0], v[1]), (df1, df2), "{name}: degrees of freedom");
    let pairs = [("SS between", v[2], ssb), ("SS within", v[3], ssw), ("MS between", v[4], msb), ("MS within", v[5], msw), ("F", v[6], fstat), ("R²", v[7], r2), ("resid SD", v[8], rsd)];
    let mut worst: f64 = 15.0;
    for (w, q, c) in pairs {
        let l = lre(q, c);
        worst = worst.min(l);
        out.push(Row { name, what: w.into(), lre: l });
    }
    worst
}

// ---------- descriptive statistics ----------

const UNIV: &[(&str, f64)] = &[
    ("PiDigits", 13.0),
    ("Lottery", 14.5),
    ("Lew", 14.5),
    ("Mavro", 13.0),
    ("Michelso", 13.0),
    ("NumAcc1", 14.5),
    ("NumAcc2", 13.5),
    ("NumAcc3", 9.0),
    ("NumAcc4", 8.0),
];

fn univ_case(name: &'static str, out: &mut Vec<Row>) -> f64 {
    let text = read(name);
    let (mean, sd, r1) = (after(&text, "Sample Mean"), after(&text, "Sample Standard Deviation"), after(&text, "Autocorrelation"));
    let rows = data_rows(&text);
    let code = format!(
        "y = {};\nm = mean(y);\nr = sum((y(1:end-1) - m) .* (y(2:end) - m)) / sum((y - m) .^ 2);\nprintf('%.17g\\n', m, std(y), r);\n",
        mat_literal(&rows)
    );
    let v = run(&code);
    assert_eq!(v.len(), 3, "{name}: {v:?}");
    let mut worst: f64 = 15.0;
    for (w, q, c) in [("mean", v[0], mean), ("std", v[1], sd), ("r(1)", v[2], r1)] {
        let l = lre(q, c);
        worst = worst.min(l);
        out.push(Row { name, what: w.into(), lre: l });
    }
    worst
}

#[test]
fn nist_strd_certified_values() {
    let mut rows = Vec::new();
    let mut bad = Vec::new();
    let mut report = String::from("dataset\tmin. LRE (fitlm / anova1 / mean-std-r1)\tmin. LRE (\\)\tthreshold\n");
    for &(name, need, need_bs) in LLS {
        let (fit, bs) = lls_case(name, &mut rows);
        report += &format!("{name}\t{fit:.1}\t{bs:.1}\t{need} / {need_bs}\n");
        if fit < need || bs < need_bs {
            bad.push(format!("{name}: {fit:.1} / {bs:.1} < {need} / {need_bs}"));
        }
    }
    for &(name, need) in ANOVA {
        let w = anova_case(name, &mut rows);
        report += &format!("{name}\t{w:.1}\t—\t{need}\n");
        if w < need {
            bad.push(format!("{name}: {w:.1} < {need}"));
        }
    }
    for &(name, need) in UNIV {
        let w = univ_case(name, &mut rows);
        report += &format!("{name}\t{w:.1}\t—\t{need}\n");
        if w < need {
            bad.push(format!("{name}: {w:.1} < {need}"));
        }
    }
    println!("{report}");
    if std::env::var("NIST_DETAIL").is_ok() {
        for r in &rows {
            println!("{}\t{}\t{:.1}", r.name, r.what, r.lre);
        }
    }
    let _ = min_lre(&rows, "Norris");
    assert!(bad.is_empty(), "below threshold: {bad:?}");
}

/// Negative control: a broken `mean` (+1 to the first result) must drop the LRE of descriptive
/// statistics below the threshold — a gate that cannot turn red is not a gate.
#[test]
fn nist_negative_control_broken_mean_is_red() {
    let text = read("PiDigits");
    let rows = data_rows(&text);
    let mean = after(&text, "Sample Mean");
    let mut it = Interp::capture();
    it.fault = Some("mean".into());
    it.run(&format!("y = {};\nprintf('%.17g\\n', mean(y));\n", mat_literal(&rows)));
    let q: f64 = it.take_output().trim().parse().unwrap();
    assert!(lre(q, mean) < 2.0, "broken mean must give LRE < 2, got {}", lre(q, mean));
}

/// Filip (a degree-10 polynomial on raw powers x ∈ [−9, −3], condition number ~10^15): `\` must honestly
/// warn about rank deficiency rather than silently give wrong coefficients (before tests3 that is exactly what happened — 0 digits, silently).
/// fitlm (QR without column pivoting) gives ≥ 7 digits — checked by `nist_strd_certified_values`.
#[test]
fn filip_backslash_warns() {
    let rows = data_rows(&read("Filip"));
    let mut it = Interp::capture();
    it.run(&format!("D = {};\nX = [ones(82, 1) D(:, 2) .^ (1:10)];\nb = X \\ D(:, 1);\n", mat_literal(&rows)));
    let out = it.take_output();
    assert!(out.contains("warning: rank deficient"), "expected a warning, got: {out}");
}
