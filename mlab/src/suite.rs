//! Scenario suite: parsing the `data/suite-v1.txt` file and running it (each scenario in a fresh interpreter).
//! Format: «### id | category | title», code, «---», expected: «> line» (exact) or «>* prefix».

use crate::interp::Interp;

#[derive(Debug, Clone)]
pub struct Scenario {
    pub id: String,
    pub category: String,
    pub title: String,
    pub code: String,
    pub expect: Vec<Expect>,
}

#[derive(Debug, Clone)]
pub enum Expect {
    Exact(String),
    Prefix(String),
}

pub fn parse(text: &str) -> Vec<Scenario> {
    let mut out = Vec::new();
    let mut cur: Option<Scenario> = None;
    let mut in_expect = false;
    let mut code_lines: Vec<String> = Vec::new();
    let finish = |cur: &mut Option<Scenario>, code_lines: &mut Vec<String>, out: &mut Vec<Scenario>| {
        if let Some(mut s) = cur.take() {
            // strip trailing empty code lines
            while code_lines.last().map_or(false, |l| l.trim().is_empty()) {
                code_lines.pop();
            }
            s.code = code_lines.join("\n") + "\n";
            out.push(s);
        }
        code_lines.clear();
    };
    for line in text.lines() {
        if let Some(h) = line.strip_prefix("### ") {
            finish(&mut cur, &mut code_lines, &mut out);
            let parts: Vec<&str> = h.split('|').map(|p| p.trim()).collect();
            cur = Some(Scenario {
                id: parts.first().unwrap_or(&"").to_string(),
                category: parts.get(1).unwrap_or(&"").to_string(),
                title: parts.get(2).unwrap_or(&"").to_string(),
                code: String::new(),
                expect: Vec::new(),
            });
            in_expect = false;
            continue;
        }
        let Some(s) = cur.as_mut() else { continue };
        if !in_expect {
            if line == "---" {
                in_expect = true;
            } else {
                code_lines.push(line.to_string());
            }
            continue;
        }
        if let Some(p) = line.strip_prefix(">* ") {
            s.expect.push(Expect::Prefix(p.to_string()));
        } else if line == ">" {
            s.expect.push(Expect::Exact(String::new()));
        } else if let Some(p) = line.strip_prefix("> ") {
            s.expect.push(Expect::Exact(p.to_string()));
        }
        // other lines (empty ones between blocks) are ignored
    }
    finish(&mut cur, &mut code_lines, &mut out);
    out
}

#[derive(Debug, Clone)]
pub struct Outcome {
    pub id: String,
    pub category: String,
    pub pass: bool,
    pub got: String,
    pub first_diff: Option<(usize, String, String)>,
}

pub fn check(expect: &[Expect], got: &str) -> Option<(usize, String, String)> {
    let lines: Vec<&str> = got.strip_suffix('\n').unwrap_or(got).split('\n').collect();
    let lines: Vec<&str> = if got.is_empty() { vec![] } else { lines };
    let n = expect.len().max(lines.len());
    for k in 0..n {
        let g = lines.get(k).copied();
        let e = expect.get(k);
        let ok = match (e, g) {
            (Some(Expect::Exact(e)), Some(g)) => e == g,
            (Some(Expect::Prefix(p)), Some(g)) => g.starts_with(p.as_str()),
            _ => false,
        };
        if !ok {
            let es = match e {
                Some(Expect::Exact(e)) => e.clone(),
                Some(Expect::Prefix(p)) => format!("{p}…"),
                None => "<end>".into(),
            };
            return Some((k + 1, es, g.unwrap_or("<end>").to_string()));
        }
    }
    None
}

/// Running the suite; `fault` — negative control (a broken builtin or an injected error before the gate).
pub fn run(scenarios: &[Scenario], fault: Option<&str>) -> Vec<Outcome> {
    run_opts(scenarios, fault, false)
}

/// The same, `vm` — execution on the register VM v1 (`vm.rs`). The file system is in memory, fresh for each scenario.
pub fn run_opts(scenarios: &[Scenario], fault: Option<&str>, vm: bool) -> Vec<Outcome> {
    run_fs(scenarios, fault, vm, &|| Box::new(vfs::Vfs::new()))
}

/// The same with a chosen file system (`--fs`): `mk_fs` gives a fresh one for each scenario.
pub fn run_fs(
    scenarios: &[Scenario],
    fault: Option<&str>,
    vm: bool,
    mk_fs: &dyn Fn() -> Box<dyn vfs::FileSystem>,
) -> Vec<Outcome> {
    scenarios
        .iter()
        .map(|s| {
            let mut it = Interp::capture().with_fs(mk_fs());
            it.fault = fault.map(|f| f.to_string());
            it.vm_mode = vm;
            it.run(&s.code);
            let got = it.take_output();
            let first_diff = check(&s.expect, &got);
            Outcome { id: s.id.clone(), category: s.category.clone(), pass: first_diff.is_none(), got, first_diff }
        })
        .collect()
}

/// VM equivalence gate report.
#[derive(Debug, Default)]
pub struct GateReport {
    pub total: usize,
    pub same: usize,
    /// First mismatch: scenario, line, tree output, VM output.
    pub mismatch: Option<(String, usize, String, String)>,
    /// Instructions compiled / handed over to the tree (static), fallback triggers (dynamic).
    pub native: usize,
    pub fallback: usize,
    pub fallback_runs: u64,
    pub dyn_runs: u64,
    /// Scenarios executed fully in the VM (without a single fallback).
    pub fully_native: usize,
}

/// VM equivalence gate: each scenario — tree-walker and VM with the same `fault`; the output (including
/// warnings and errors) must match byte for byte. The first mismatch — red and stop (fail-fast).
pub fn vm_gate(scenarios: &[Scenario], fault: Option<&str>) -> GateReport {
    let mut rep = GateReport::default();
    for s in scenarios {
        let mut a = Interp::capture();
        a.fault = fault.map(|f| f.to_string());
        a.run(&s.code);
        let tree = a.take_output();
        let mut b = Interp::capture();
        b.fault = fault.map(|f| f.to_string());
        b.vm_mode = true;
        b.run(&s.code);
        let vm = b.take_output();
        rep.total += 1;
        rep.native += b.vm_stats.native;
        rep.fallback += b.vm_stats.fallback;
        rep.fallback_runs += b.vm_stats.fallback_runs;
        rep.dyn_runs += b.vm_stats.dyn_runs;
        if b.vm_stats.fallback == 0 && b.vm_stats.dyn_runs == 0 {
            rep.fully_native += 1;
        }
        if tree == vm {
            rep.same += 1;
            continue;
        }
        let (tl, vl): (Vec<&str>, Vec<&str>) = (tree.split('\n').collect(), vm.split('\n').collect());
        let k = (0..tl.len().max(vl.len())).find(|&k| tl.get(k) != vl.get(k)).unwrap_or(0);
        rep.mismatch = Some((
            s.id.clone(),
            k + 1,
            tl.get(k).map_or("<end>".into(), |x| x.to_string()),
            vl.get(k).map_or("<end>".into(), |x| x.to_string()),
        ));
        break;
    }
    rep
}
