//! Run over real formulas: arXiv metadata (OAI-PMH arXivRaw, `.xml.gz`) or `.tex`. Computes coverage
//! (share of formulas without `Unknown`), top unknown commands, errors by kind, the round-trip gate,
//! (with feature `math`) the share that converts to `math::Expr`, and speed. Only reads sources.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use latex::gate::{self, RoundTrip};
use latex::print::Fault;
use latex::{Macros, extract, parse_with};

#[derive(Default)]
pub struct Opts {
    pub out: Option<PathBuf>,
    pub limit: Option<usize>,
    pub fault: Option<Fault>,
    pub bridge: bool,
}

#[derive(Default)]
struct Stats {
    files: usize,
    file_errors: Vec<String>,
    records: usize,
    dup_records: usize,
    texts: usize,
    formulas: usize,
    distinct: HashSet<u64>,
    distinct_clean: HashSet<u64>,
    unterminated: usize,
    parsed: usize,
    clean: usize,
    errors: HashMap<&'static str, (usize, Vec<String>)>,
    unknown: HashMap<String, (usize, String)>,
    /// formulas with at least one unknown — keyed by the name of the first unknown
    rt_ok: usize,
    rt_fail: usize,
    /// negative control: formulas the fault touches structurally, and how many of them are red
    fault_touched: usize,
    fault_touched_red: usize,
    rt_samples: Vec<String>,
    bridge_ok: usize,
    bridge_rel: usize,
    bridge_why: HashMap<String, usize>,
    bridge_samples: Vec<String>,
    t_parse: Duration,
    t_gate: Duration,
    t_total: Duration,
    bytes: usize,
    formula_bytes: usize,
}

fn hash(s: &str) -> u64 {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

fn collect(paths: &[PathBuf], out: &mut Vec<PathBuf>) {
    for p in paths {
        if p.is_dir() {
            let mut v: Vec<PathBuf> = match std::fs::read_dir(p) {
                Ok(rd) => rd.filter_map(|e| e.ok().map(|e| e.path())).collect(),
                Err(_) => continue,
            };
            v.sort();
            collect(&v, out);
        } else {
            let s = p.to_string_lossy();
            if s.ends_with(".xml.gz") || s.ends_with(".xml") || s.ends_with(".tex") {
                out.push(p.clone());
            }
        }
    }
}

fn read(p: &Path) -> Result<String, String> {
    let bytes = std::fs::read(p).map_err(|e| format!("{}: {e}", p.display()))?;
    if p.to_string_lossy().ends_with(".gz") {
        #[cfg(feature = "gz")]
        {
            let mut s = String::new();
            flate2::read::GzDecoder::new(&bytes[..]).read_to_string(&mut s).map_err(|e| format!("{}: gzip: {e}", p.display()))?;
            return Ok(s);
        }
        #[cfg(not(feature = "gz"))]
        return Err(format!("{}: .gz without feature gz", p.display()));
    }
    String::from_utf8(bytes).map_err(|e| format!("{}: not UTF-8: {e}", p.display()))
}

fn tag<'a>(rec: &'a str, name: &str) -> Option<&'a str> {
    let open = format!("<{name}>");
    let close = format!("</{name}>");
    let a = rec.find(&open)? + open.len();
    let b = rec[a..].find(&close)? + a;
    Some(&rec[a..b])
}

/// XML unescaping: `&lt;` `&gt;` `&amp;` `&quot;` `&apos;` `&#NN;` `&#xHH;`.
pub fn unescape(s: &str) -> String {
    if !s.contains('&') {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let Some(j) = rest.find(';').filter(|&j| j <= 10) else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let ent = &rest[1..j];
        let ch = match ent {
            "lt" => Some('<'),
            "gt" => Some('>'),
            "amp" => Some('&'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ if ent.starts_with("#x") => u32::from_str_radix(&ent[2..], 16).ok().and_then(char::from_u32),
            _ if ent.starts_with('#') => ent[1..].parse::<u32>().ok().and_then(char::from_u32),
            _ => None,
        };
        match ch {
            Some(c) => {
                out.push(c);
                rest = &rest[j + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

pub fn run(paths: &[PathBuf], o: &Opts) -> Result<String, String> {
    let mut files = Vec::new();
    collect(paths, &mut files);
    if files.is_empty() {
        return Err("no .xml.gz / .xml / .tex files".into());
    }
    let mut st = Stats::default();
    let mut seen_ids: HashSet<String> = HashSet::new();
    let start = Instant::now();
    'files: for f in &files {
        let text = match read(f) {
            Ok(t) => t,
            Err(e) => {
                st.file_errors.push(e);
                continue;
            }
        };
        st.files += 1;
        st.bytes += text.len();
        if f.to_string_lossy().ends_with(".tex") {
            let mut m = Macros::new();
            m.scan(&text);
            st.texts += 1;
            if process(&text, &m, o, &mut st) {
                break 'files;
            }
            continue;
        }
        let empty = Macros::new();
        let mut rest = text.as_str();
        while let Some(a) = rest.find("<record>") {
            let Some(b) = rest[a..].find("</record>") else { break };
            let rec = &rest[a..a + b];
            rest = &rest[a + b + 9..];
            let Some(id) = tag(rec, "id") else { continue };
            if !seen_ids.insert(id.to_string()) {
                st.dup_records += 1;
                continue;
            }
            st.records += 1;
            for name in ["title", "abstract"] {
                if let Some(t) = tag(rec, name) {
                    let t = unescape(t);
                    st.texts += 1;
                    if process(&t, &empty, o, &mut st) {
                        break 'files;
                    }
                }
            }
        }
    }
    st.t_total = start.elapsed();
    let report = report(&st, o, &files);
    if let Some(dir) = &o.out {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        write_tables(&st, dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        std::fs::write(dir.join("summary.md"), &report).map_err(|e| e.to_string())?;
    }
    Ok(report)
}

/// true — the formula limit was reached.
fn process(text: &str, macros: &Macros, o: &Opts, st: &mut Stats) -> bool {
    let (formulas, bad) = extract(text);
    st.unterminated += bad.len();
    for f in formulas {
        if o.limit.is_some_and(|l| st.formulas >= l) {
            return true;
        }
        st.formulas += 1;
        st.formula_bytes += f.body.len();
        let h = hash(f.body);
        st.distinct.insert(h);
        let t0 = Instant::now();
        let r = parse_with(f.body, macros);
        st.t_parse += t0.elapsed();
        let tree = match r {
            Ok(t) => t,
            Err(e) => {
                let ent = st.errors.entry(e.kind.code()).or_default();
                ent.0 += 1;
                if ent.1.len() < 12 {
                    ent.1.push(format!("{} | {}", e, one_line(&e.context(f.body))));
                }
                continue;
            }
        };
        st.parsed += 1;
        let unknowns = tree.unknowns();
        if unknowns.is_empty() {
            st.clean += 1;
            st.distinct_clean.insert(h);
        }
        for (name, _) in unknowns {
            let ent = st.unknown.entry(name).or_insert_with(|| (0, one_line(f.body)));
            ent.0 += 1;
        }
        let t1 = Instant::now();
        let rt = match o.fault {
            Some(fault) => gate::check_faulty(&tree, fault),
            None => gate::check(&tree),
        };
        st.t_gate += t1.elapsed();
        if let Some(fault) = o.fault {
            if gate::affected(&tree, fault) {
                st.fault_touched += 1;
                if !rt.is_ok() {
                    st.fault_touched_red += 1;
                }
            }
        }
        match rt {
            RoundTrip::Ok => st.rt_ok += 1,
            other => {
                st.rt_fail += 1;
                if st.rt_samples.len() < 40 {
                    let (printed, why) = match &other {
                        RoundTrip::Mismatch { printed, .. } => (printed.clone(), "different tree".to_string()),
                        RoundTrip::ReparseError { printed, error } => (printed.clone(), error.to_string()),
                        RoundTrip::Ok => unreachable!(),
                    };
                    st.rt_samples.push(format!("{}\t{}\t{}", one_line(f.body), one_line(&printed), why));
                }
            }
        }
        if o.bridge {
            bridge(&tree, f.body, st);
        }
    }
    false
}

#[cfg(feature = "math")]
fn bridge(tree: &latex::Node, src: &str, st: &mut Stats) {
    use latex::tomath::{from_expr, to_expr, to_relation};
    match to_expr(tree) {
        Ok(e) => {
            st.bridge_ok += 1;
            if st.bridge_samples.len() < 60 && src.len() > 12 && st.bridge_ok % 997 == 0 {
                let back = from_expr(&e).map(|n| latex::to_latex(&n)).unwrap_or_default();
                st.bridge_samples.push(format!("{}\t{e}\t{back}", one_line(src)));
            }
        }
        Err(e) if e.kind() == "relation" => match to_relation(tree) {
            Ok(_) => st.bridge_rel += 1,
            Err(e2) => *st.bridge_why.entry(e2.kind().to_string()).or_default() += 1,
        },
        Err(e) => *st.bridge_why.entry(e.kind().to_string()).or_default() += 1,
    }
}

#[cfg(not(feature = "math"))]
fn bridge(_: &latex::Node, _: &str, _: &mut Stats) {}

fn one_line(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn pct(a: usize, b: usize) -> f64 {
    if b == 0 { 0.0 } else { 100.0 * a as f64 / b as f64 }
}

fn report(st: &Stats, o: &Opts, files: &[PathBuf]) -> String {
    let mut r = String::new();
    let _ = writeln!(r, "# latex: run over real formulas");
    let _ = writeln!(r);
    let _ = writeln!(r, "- files: {} of {} (read errors: {})", st.files, files.len(), st.file_errors.len());
    for e in st.file_errors.iter().take(5) {
        let _ = writeln!(r, "  - {e}");
    }
    let _ = writeln!(r, "- arXiv records: {} (duplicates from other sets skipped: {}), texts: {}, {:.1} MB", st.records, st.dup_records, st.texts, st.bytes as f64 / 1e6);
    let _ = writeln!(r, "- formulas: {} (distinct: {}); unterminated `$`/`\\(`… during extraction: {}", st.formulas, st.distinct.len(), st.unterminated);
    let errs: usize = st.errors.values().map(|e| e.0).sum();
    let _ = writeln!(r, "- parsed: {} ({:.2} %); parse errors: {} ({:.2} %)", st.parsed, pct(st.parsed, st.formulas), errs, pct(errs, st.formulas));
    let _ = writeln!(
        r,
        "- **without `Unknown`: {} of {} ({:.2} %)**; distinct formulas without `Unknown`: {:.2} %",
        st.clean,
        st.formulas,
        pct(st.clean, st.formulas),
        pct(st.distinct_clean.len(), st.distinct.len())
    );
    let gate_name = match o.fault {
        Some(f) => format!("round-trip gate WITH PRINT FAULT {f:?} (negative control)"),
        None => "round-trip gate".to_string(),
    };
    let _ = writeln!(r, "- {gate_name}: green {} of {} parsed ({:.3} %), red {}", st.rt_ok, st.parsed, pct(st.rt_ok, st.parsed), st.rt_fail);
    if o.fault.is_some() {
        let _ = writeln!(
            r,
            "  - the fault touches (by tree structure) {} formulas; red among them {}; red among untouched {} — {}",
            st.fault_touched,
            st.fault_touched_red,
            st.rt_fail - st.fault_touched_red,
            if st.fault_touched == st.fault_touched_red && st.rt_fail == st.fault_touched_red { "exact match" } else { "does NOT match" }
        );
    }
    if o.bridge {
        let _ = writeln!(
            r,
            "- bridge to `math::Expr`: expression {} ({:.2} %), relation chain {} ({:.2} %), total {:.2} % of {} parsed; reasons for the rest:",
            st.bridge_ok,
            pct(st.bridge_ok, st.parsed),
            st.bridge_rel,
            pct(st.bridge_rel, st.parsed),
            pct(st.bridge_ok + st.bridge_rel, st.parsed),
            st.parsed
        );
        let mut why: Vec<_> = st.bridge_why.iter().collect();
        why.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        for (w, n) in why.iter().take(12) {
            let _ = writeln!(r, "  - {n} — {w}");
        }
    }
    let secs = st.t_parse.as_secs_f64();
    let gsecs = st.t_gate.as_secs_f64();
    let _ = writeln!(
        r,
        "- speed (1 thread): parsing {:.0} formulas/s ({:.2} s for {} formulas, {:.1} MB/s of formula text); gate (print + reparse + compare) {:.0} formulas/s; whole run (gzip, XML, extraction, parsing, gate{}) {:.1} s",
        st.formulas as f64 / secs.max(1e-9),
        secs,
        st.formulas,
        st.formula_bytes as f64 / 1e6 / secs.max(1e-9),
        st.parsed as f64 / gsecs.max(1e-9),
        if o.bridge { ", bridge" } else { "" },
        st.t_total.as_secs_f64()
    );
    let _ = writeln!(r);
    let _ = writeln!(r, "## Parse errors by kind");
    let mut ev: Vec<_> = st.errors.iter().collect();
    ev.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    for (code, (n, ex)) in &ev {
        let _ = writeln!(r, "- `{code}` — {n}; example: {}", ex.first().map(String::as_str).unwrap_or(""));
    }
    let _ = writeln!(r);
    let _ = writeln!(r, "## Top 30 unknown commands (support queue)");
    let _ = writeln!(r);
    let _ = writeln!(r, "| # | command | formulas | example |");
    let _ = writeln!(r, "|---|---|---|---|");
    for (i, (name, (n, ex))) in top_unknown(st).iter().take(30).enumerate() {
        let ex: String = ex.chars().take(70).collect();
        let _ = writeln!(r, "| {} | `{}` | {} | `{}` |", i + 1, name, n, ex.replace('|', "\\|"));
    }
    r
}

fn top_unknown(st: &Stats) -> Vec<(&String, &(usize, String))> {
    let mut v: Vec<_> = st.unknown.iter().collect();
    v.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
    v
}

fn write_tables(st: &Stats, dir: &Path) -> std::io::Result<()> {
    let mut u = String::from("command\toccurrences\texample\n");
    for (name, (n, ex)) in top_unknown(st) {
        let _ = writeln!(u, "{name}\t{n}\t{ex}");
    }
    std::fs::write(dir.join("unknown.tsv"), u)?;
    let mut e = String::from("code\tcount\texample\n");
    let mut ev: Vec<_> = st.errors.iter().collect();
    ev.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    for (code, (n, ex)) in ev {
        for x in ex {
            let _ = writeln!(e, "{code}\t{n}\t{x}");
        }
    }
    std::fs::write(dir.join("errors.tsv"), e)?;
    let mut rt = String::from("source\tprinted\twhy\n");
    for s in &st.rt_samples {
        let _ = writeln!(rt, "{s}");
    }
    std::fs::write(dir.join("roundtrip-fail.tsv"), rt)?;
    if !st.bridge_samples.is_empty() {
        let mut b = String::from("source\tmath::Expr\tback to LaTeX\n");
        for x in &st.bridge_samples {
            let _ = writeln!(b, "{x}");
        }
        std::fs::write(dir.join("bridge-sample.tsv"), b)?;
    }
    Ok(())
}
