//! Mova on tables (Kaggle-style tasks): the SLM reads records into its world and learns its own decision list,
//! with every prediction explained by the rule that made it.
//!
//! - **Reading.** Each record is an entity; its columns are attributes. Numbers are cut into bins at quantiles the
//!   model picks itself from train; short categories stay as they are; a column of personal names is read with
//!   level-1 knowledge of English honorifics ("Master" is a boy, "Mrs" a married woman…) and gives the title and the
//!   family name. Records that share a value of an identifying column (a ticket, a family name with the same fare)
//!   are linked in the world as travelling together.
//! - **Relations as features.** For a record, what its linked records did (only those with a known outcome in the
//!   training part) becomes an attribute: "the women and boys of my group all had the outcome" — so the learner can
//!   find group behaviour itself, it is not written in.
//! - **Learning.** A decision list learned greedily (CN2-style, Laplace accuracy, conjunctions of up to two
//!   attributes, at least `min` records per rule), measured by 10-fold cross-validation with fixed folds.
//! - **Teachers.** Predictions of other models for the unlabelled records (`--teacher`) can be added as
//!   pseudo-labels: the teacher's knowledge is learned by Mova's own rules, never copied into the answer.
//!
//!   world tabular <train.csv> <test.csv> --target <col> --id <col> [--teacher <csv>] [--out <submission.csv>]

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, bail};

/// One CSV line with quoted fields.
fn split_csv(l: &str) -> Vec<String> {
    let (mut out, mut cur, mut q) = (Vec::new(), String::new(), false);
    let mut it = l.chars().peekable();
    while let Some(c) = it.next() {
        match (c, q) {
            ('"', true) if it.peek() == Some(&'"') => {
                cur.push('"');
                it.next();
            }
            ('"', _) => q = !q,
            (',', false) => out.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out
}

/// A table: header and rows of strings.
pub struct Table {
    pub head: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub fn read_csv(path: &Path) -> Result<Table> {
    let t = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    let mut lines = t.lines().filter(|l| !l.trim().is_empty());
    let head = split_csv(lines.next().context("empty table")?);
    Ok(Table { head, rows: lines.map(split_csv).collect() })
}

/// Level-1 knowledge of English honorifics: title → what it says about the person.
const HONORIFICS: &[(&str, &str)] = &[
    ("Mr", "man"),
    ("Mrs", "married woman"),
    ("Mme", "married woman"),
    ("Miss", "unmarried woman or girl"),
    ("Mlle", "unmarried woman or girl"),
    ("Ms", "woman"),
    ("Master", "boy"),
    ("Dr", "doctor"),
    ("Rev", "clergyman"),
    ("Col", "officer"),
    ("Major", "officer"),
    ("Capt", "officer"),
    ("Sir", "nobleman"),
    ("Lady", "noblewoman"),
    ("the Countess", "noblewoman"),
    ("Don", "nobleman"),
    ("Dona", "noblewoman"),
    ("Jonkheer", "nobleman"),
];

/// "Surname, Title. Given names" → (family name, what the title says), if it reads as a name.
fn read_name(s: &str) -> Option<(String, &'static str)> {
    let (family, rest) = s.split_once(", ")?;
    let t = rest.split('.').next()?.trim();
    let meaning = HONORIFICS.iter().find(|(h, _)| *h == t).map(|x| x.1).unwrap_or("other title");
    Some((family.trim().to_string(), meaning))
}

/// The world of the records: attributes per record (name → value) and links between records.
pub struct World {
    pub attrs: Vec<BTreeMap<String, String>>,
    pub links: Vec<Vec<usize>>,
    /// a woman or a child (from the name or the age), for group features
    pub wc: Vec<bool>,
}

/// Read the records (train then test rows) into a world. `skip`: the target and id columns.
pub fn read_world(head: &[String], rows: &[&Vec<String>], skip: &[&str]) -> World {
    let n = rows.len();
    let col = |name: &str| head.iter().position(|h| h == name);
    let mut attrs: Vec<BTreeMap<String, String>> = vec![BTreeMap::new(); n];
    let mut keys: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    let mut wc = vec![false; n];
    for (ci, h) in head.iter().enumerate() {
        if skip.contains(&h.as_str()) {
            continue;
        }
        let vals: Vec<&str> = rows.iter().map(|r| r.get(ci).map(String::as_str).unwrap_or("")).collect();
        let nums: Vec<f64> = vals.iter().filter_map(|v| v.parse::<f64>().ok()).collect();
        let names = vals.iter().filter(|v| read_name(v).is_some()).count();
        let distinct: std::collections::BTreeSet<&str> = vals.iter().copied().collect();
        if names * 2 > n {
            // a column of personal names: the title's meaning and the family name
            for (i, v) in vals.iter().enumerate() {
                if let Some((fam, m)) = read_name(v) {
                    attrs[i].insert(format!("{h}: title"), m.to_string());
                    if m == "boy" || m.contains("woman") {
                        wc[i] = true;
                    }
                    let fare = col("Fare").and_then(|f| rows[i].get(f)).cloned().unwrap_or_default();
                    keys.entry(format!("family {fam} {fare}")).or_default().push(i);
                }
            }
        } else if nums.len() * 10 >= n * 9 && distinct.len() > 12 {
            // a number: quartile bins chosen from the data
            let mut s = nums.clone();
            s.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let q = |p: f64| s[((s.len() - 1) as f64 * p) as usize];
            let cuts = [q(0.25), q(0.5), q(0.75)];
            for (i, v) in vals.iter().enumerate() {
                let b = match v.parse::<f64>() {
                    Ok(x) => format!("q{}", cuts.iter().filter(|c| x > **c).count() + 1),
                    Err(_) => "unknown".into(),
                };
                attrs[i].insert(h.clone(), b);
            }
        } else if distinct.len() <= 12 {
            for (i, v) in vals.iter().enumerate() {
                attrs[i].insert(h.clone(), if v.is_empty() { "unknown".into() } else { v.to_string() });
            }
        } else {
            // an identifier-like column: shared values link records; its first letter may be a category (a deck)
            for (i, v) in vals.iter().enumerate() {
                if !v.is_empty() {
                    keys.entry(format!("{h} {v}")).or_default().push(i);
                }
                let first: String = v.chars().next().filter(|c| c.is_ascii_alphabetic()).map(|c| c.to_string()).unwrap_or_else(|| "none".into());
                if vals.iter().filter(|x| x.chars().next().is_some_and(|c| c.is_ascii_alphabetic())).count() * 5 < n {
                    attrs[i].insert(format!("{h}: letter"), first);
                }
            }
        }
    }
    // a child by age, if there is an age column
    if let Some(a) = col("Age") {
        for (i, r) in rows.iter().enumerate() {
            if r.get(a).and_then(|v| v.parse::<f64>().ok()).is_some_and(|x| x < 13.0) {
                wc[i] = true;
            }
        }
    }
    let mut links = vec![Vec::new(); n];
    for ids in keys.values().filter(|v| v.len() > 1 && v.len() <= 12) {
        for &a in ids {
            for &b in ids {
                if a != b && !links[a].contains(&b) {
                    links[a].push(b);
                }
            }
        }
    }
    World { attrs, links, wc }
}

/// The relation attribute of record `i`: what its linked women and children did, known outcomes only.
fn group_attr(w: &World, i: usize, known: &BTreeMap<usize, bool>) -> String {
    let out: Vec<bool> = w.links[i].iter().filter(|j| w.wc[**j]).filter_map(|j| known.get(j).copied()).collect();
    match (out.iter().filter(|x| **x).count(), out.iter().filter(|x| !**x).count()) {
        (0, 0) => "none known".into(),
        (_, 0) => "all had it".into(),
        (0, _) => "none had it".into(),
        _ => "mixed".into(),
    }
}

type Cond = Vec<(String, String)>;

/// A learned decision list.
pub struct List {
    pub rules: Vec<(Cond, bool, usize, usize)>,
    pub default: bool,
}

fn covers(c: &Cond, f: &BTreeMap<String, String>) -> bool {
    c.iter().all(|(k, v)| f.get(k) == Some(v))
}

/// Greedy decision list over records (attributes, outcome).
pub fn learn(data: &[(BTreeMap<String, String>, bool)], min: usize) -> List {
    learn_k(data, min, 2)
}

/// Greedy decision list with conjunctions of up to `k` (1–3) attributes.
pub fn learn_k(data: &[(BTreeMap<String, String>, bool)], min: usize, k: usize) -> List {
    learn_kc(data, min, k, 14)
}

/// Greedy decision list with conjunctions of up to `k` attributes and at most `cap` rules.
pub fn learn_kc(data: &[(BTreeMap<String, String>, bool)], min: usize, k: usize, cap: usize) -> List {
    let mut left: Vec<usize> = (0..data.len()).collect();
    let mut rules = Vec::new();
    while left.len() >= min && rules.len() < cap {
        let pos = left.iter().filter(|&&i| data[i].1).count();
        let base = pos.max(left.len() - pos) as f64 / left.len() as f64;
        let mut cands: BTreeMap<Cond, (usize, usize)> = BTreeMap::new();
        for &i in &left {
            let f: Vec<(&String, &String)> = data[i].0.iter().collect();
            let at = |x: usize| (f[x].0.clone(), f[x].1.clone());
            for a in 0..f.len() {
                let e = cands.entry(vec![at(a)]).or_default();
                if data[i].1 { e.0 += 1 } else { e.1 += 1 }
                if k < 2 {
                    continue;
                }
                for b in a + 1..f.len() {
                    let e = cands.entry(vec![at(a), at(b)]).or_default();
                    if data[i].1 { e.0 += 1 } else { e.1 += 1 }
                    if k < 3 {
                        continue;
                    }
                    for c in b + 1..f.len() {
                        let e = cands.entry(vec![at(a), at(b), at(c)]).or_default();
                        if data[i].1 { e.0 += 1 } else { e.1 += 1 }
                    }
                }
            }
        }
        let best = cands
            .into_iter()
            .filter(|(_, (p, n))| p + n >= min)
            .map(|(c, (p, n))| ((p.max(n) + 1) as f64 / (p + n + 2) as f64, c, p, n))
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(b.1.len().cmp(&a.1.len())).then(b.1.cmp(&a.1)));
        let Some((lap, c, p, n)) = best else { break };
        if lap < base + 0.02 {
            break;
        }
        left.retain(|&i| !covers(&c, &data[i].0));
        rules.push((c, p > n, p, n));
    }
    let pos = left.iter().filter(|&&i| data[i].1).count();
    List { rules, default: pos * 2 > left.len() }
}

pub fn predict(l: &List, f: &BTreeMap<String, String>) -> (bool, String) {
    for (c, v, p, n) in &l.rules {
        if covers(c, f) {
            let cs: Vec<String> = c.iter().map(|(k, v)| format!("{k} = {v}")).collect();
            return (*v, format!("{} ({p} yes, {n} no in training)", cs.join(" and ")));
        }
    }
    (l.default, "the default".into())
}

/// Records with the relation attribute computed from `known` outcomes.
fn with_group(w: &World, i: usize, known: &BTreeMap<usize, bool>) -> BTreeMap<String, String> {
    let mut f = w.attrs[i].clone();
    f.insert("group of women and children".into(), group_attr(w, i, known));
    f
}

/// `world tabular`.
pub fn run(train: &Path, test: &Path, target: &str, id: &str, teacher: Option<&Path>, out: Option<&Path>) -> Result<()> {
    let tr = read_csv(train)?;
    let te = read_csv(test)?;
    let ti = tr.head.iter().position(|h| h == target).with_context(|| format!("no column {target}"))?;
    let ii = te.head.iter().position(|h| h == id).with_context(|| format!("no column {id}"))?;
    // test rows aligned to the train header (the target column missing)
    let te_rows: Vec<Vec<String>> = te.rows.iter().map(|r| tr.head.iter().map(|h| te.head.iter().position(|x| x == h).and_then(|j| r.get(j)).cloned().unwrap_or_default()).collect()).collect();
    let all: Vec<&Vec<String>> = tr.rows.iter().chain(te_rows.iter()).collect();
    let w = read_world(&tr.head, &all, &[target, id]);
    let y: Vec<bool> = tr.rows.iter().map(|r| r.get(ti).is_some_and(|v| v == "1")).collect();
    let n = tr.rows.len();
    println!("read {} training and {} test records; {} attributes; {} records linked into groups", n, te_rows.len(), w.attrs[0].len(), w.links.iter().filter(|l| !l.is_empty()).count());
    // teacher pseudo-labels for the test records
    let mut pseudo: BTreeMap<usize, bool> = BTreeMap::new();
    if let Some(t) = teacher {
        let tt = read_csv(t)?;
        let (tid, tval) = (tt.head.iter().position(|h| h == id).context("teacher: no id")?, tt.head.iter().position(|h| h == target).context("teacher: no target")?);
        let by: BTreeMap<String, bool> = tt.rows.iter().map(|r| (r[tid].clone(), r[tval] == "1")).collect();
        for (k, r) in te.rows.iter().enumerate() {
            if let Some(v) = by.get(&r[ii]) {
                pseudo.insert(n + k, *v);
            }
        }
        println!("teacher: {} pseudo-labels for test records", pseudo.len());
    }
    // Mova tunes herself: the rule support `min` and the conjunction size `k` chosen by cross-validation, each
    // setting scored on the same 10 folds (group attribute only from the fold's training outcomes)
    let cv_of = |min: usize, k: usize| -> usize {
        let mut ok = 0;
        for fold in 0..10 {
            let known: BTreeMap<usize, bool> = (0..n).filter(|i| i % 10 != fold).map(|i| (i, y[i])).chain(pseudo.iter().map(|(k, v)| (*k, *v))).collect();
            let data: Vec<(BTreeMap<String, String>, bool)> = known.iter().map(|(&i, &v)| (with_group(&w, i, &{ let mut k2 = known.clone(); k2.remove(&i); k2 }), v)).collect();
            let l = learn_k(&data, min, k);
            for i in (0..n).filter(|i| i % 10 == fold) {
                ok += usize::from(predict(&l, &with_group(&w, i, &known)).0 == y[i]);
            }
        }
        ok
    };
    let mut best = (0usize, 10usize, 2usize);
    for k in 1..=3 {
        for min in [5usize, 10, 15, 20, 30] {
            let ok = cv_of(min, k);
            println!("  setting min {min:>2}, up to {k} attribute(s): CV {:.2}%", 100.0 * ok as f64 / n as f64);
            if ok > best.0 {
                best = (ok, min, k);
            }
        }
    }
    let (ok, min_best, k_best) = best;
    // the honest number: nested cross-validation — in every outer fold the setting is chosen on the other nine folds
    // only, then the fold is predicted
    let mut honest = 0;
    let settings: Vec<(usize, usize)> = (1..=3).flat_map(|k| [5usize, 10, 15, 20, 30].into_iter().map(move |m| (m, k))).collect();
    for outer in 0..10 {
        let inner: Vec<usize> = (0..n).filter(|i| i % 10 != outer).collect();
        let mut pick = (0usize, 10usize, 2usize);
        for &(m, k) in &settings {
            let mut okk = 0;
            for fold in (0..10).filter(|f| *f != outer) {
                let known: BTreeMap<usize, bool> = inner.iter().filter(|i| *i % 10 != fold).map(|&i| (i, y[i])).chain(pseudo.iter().map(|(a, b)| (*a, *b))).collect();
                let data: Vec<(BTreeMap<String, String>, bool)> = known.iter().map(|(&i, &v)| (with_group(&w, i, &{ let mut k2 = known.clone(); k2.remove(&i); k2 }), v)).collect();
                let l = learn_k(&data, m, k);
                for &i in inner.iter().filter(|i| *i % 10 == fold) {
                    okk += usize::from(predict(&l, &with_group(&w, i, &known)).0 == y[i]);
                }
            }
            if okk > pick.0 {
                pick = (okk, m, k);
            }
        }
        let known: BTreeMap<usize, bool> = inner.iter().map(|&i| (i, y[i])).chain(pseudo.iter().map(|(a, b)| (*a, *b))).collect();
        let data: Vec<(BTreeMap<String, String>, bool)> = known.iter().map(|(&i, &v)| (with_group(&w, i, &{ let mut k2 = known.clone(); k2.remove(&i); k2 }), v)).collect();
        let l = learn_k(&data, pick.1, pick.2);
        for i in (0..n).filter(|i| i % 10 == outer) {
            honest += usize::from(predict(&l, &with_group(&w, i, &known)).0 == y[i]);
        }
    }
    println!("nested cross-validation (the honest estimate, setting chosen inside each fold): {:.2}%", 100.0 * honest as f64 / n as f64);
    println!("10-fold cross-validation on train: {:.2}% with min {min_best}, up to {k_best} attribute(s) (chosen by the same folds — optimistic by the choice)", 100.0 * ok as f64 / n as f64);
    // the final list on all training records (and the teacher's pseudo-labels)
    let known: BTreeMap<usize, bool> = (0..n).map(|i| (i, y[i])).chain(pseudo.iter().map(|(k, v)| (*k, *v))).collect();
    let data: Vec<(BTreeMap<String, String>, bool)> = known.iter().map(|(&i, &v)| (with_group(&w, i, &{ let mut k2 = known.clone(); k2.remove(&i); k2 }), v)).collect();
    let l = learn_k(&data, min_best, k_best);
    println!("Mova's rules:");
    for (c, v, p, q) in &l.rules {
        let cs: Vec<String> = c.iter().map(|(k, v)| format!("{k} = {v}")).collect();
        println!("  if {} → {} ({p} yes, {q} no)", cs.join(" and "), if *v { "yes" } else { "no" });
    }
    println!("  otherwise → {}", if l.default { "yes" } else { "no" });
    if let Some(o) = out {
        let train_known: BTreeMap<usize, bool> = (0..n).map(|i| (i, y[i])).collect();
        let mut csv = format!("{id},{target}\n");
        let mut why = String::new();
        let mut agree = 0;
        for (k, r) in te.rows.iter().enumerate() {
            let (v, reason) = predict(&l, &with_group(&w, n + k, &train_known));
            if pseudo.get(&(n + k)) == Some(&v) {
                agree += 1;
            }
            csv.push_str(&format!("{},{}\n", r[ii], u8::from(v)));
            why.push_str(&format!("{}\t{}\t{reason}\n", r[ii], u8::from(v)));
        }
        std::fs::write(o, csv)?;
        std::fs::write(o.with_extension("why.tsv"), why)?;
        println!("Mova's answers → {} (reasons in {}){}", o.display(), o.with_extension("why.tsv").display(), if pseudo.is_empty() { String::new() } else { format!("; agrees with the teacher on {agree} of {}", te.rows.len()) });
    }
    Ok(())
}

pub fn main_args(args: &[String]) -> Result<()> {
    let usage = "usage: world tabular <train.csv> <test.csv> --target <col> --id <col> [--teacher <csv>] [--out <csv>]";
    let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
    let pos: Vec<&String> = args.iter().skip(1).filter(|a| !a.starts_with("--")).collect();
    let pos: Vec<&String> = pos.into_iter().filter(|a| ![opt("--target"), opt("--id"), opt("--teacher"), opt("--out")].contains(&Some((*a).clone()))).collect();
    if pos.len() < 2 {
        bail!("{usage}");
    }
    run(Path::new(pos[0]), Path::new(pos[1]), &opt("--target").context(usage)?, &opt("--id").context(usage)?, opt("--teacher").as_deref().map(Path::new), opt("--out").as_deref().map(Path::new))
}

/// A list for a rare class: only rules that predict `true` with a Laplace precision of at least `prec` on at least
/// `min` records, greedily, the default `false`. When the wanted records are few, a majority list keeps nothing; this
/// one keeps what it is sure of.
pub fn learn_rare(data: &[(BTreeMap<String, String>, bool)], min: usize, k: usize, prec: f64) -> List {
    let mut left: Vec<usize> = (0..data.len()).collect();
    let mut rules = Vec::new();
    while rules.len() < 14 {
        let mut cands: BTreeMap<Cond, (usize, usize)> = BTreeMap::new();
        for &i in &left {
            let f: Vec<(&String, &String)> = data[i].0.iter().collect();
            let at = |x: usize| (f[x].0.clone(), f[x].1.clone());
            for a in 0..f.len() {
                let e = cands.entry(vec![at(a)]).or_default();
                if data[i].1 { e.0 += 1 } else { e.1 += 1 }
                if k >= 2 {
                    for b in a + 1..f.len() {
                        let e = cands.entry(vec![at(a), at(b)]).or_default();
                        if data[i].1 { e.0 += 1 } else { e.1 += 1 }
                    }
                }
            }
        }
        let best = cands
            .into_iter()
            .filter(|(_, (p, n))| p + n >= min)
            .map(|(c, (p, n))| ((p + 1) as f64 / (p + n + 2) as f64, c, p, n))
            .filter(|x| x.0 >= prec)
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.2.cmp(&b.2)).then(b.1.cmp(&a.1)));
        let Some((_, c, p, n)) = best else { break };
        left.retain(|&i| !covers(&c, &data[i].0));
        rules.push((c, true, p, n));
    }
    List { rules, default: false }
}
