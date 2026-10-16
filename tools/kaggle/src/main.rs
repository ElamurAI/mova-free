//! kaggle — Mova on Kaggle competitions, deterministic and explained: every prediction names the rule behind it.
//!
//!   kaggle titanic <data dir> [--out submission.csv]
//!
//! Titanic: three models, each measured by 10-fold cross-validation on train (fixed folds by passenger id):
//! 1. gender — women survive (the competition's own baseline);
//! 2. a decision list learned greedily (CN2-style): conjunctions of up to two features over sex, class, title from
//!    the name, age, family size, fare, port, deck; each rule takes the majority of the passengers it covers;
//! 3. woman-child groups — passengers sharing a surname and fare, or a ticket, travel together; women and boys
//!    (title Master) live or die with their group: a female whose whole group of women and boys died in train is
//!    predicted to die, a boy whose group's women and boys all survived is predicted to live; everyone else as the
//!    gender model (men die, women live).

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, bail};

#[derive(Clone, Debug)]
struct P {
    id: u32,
    survived: Option<bool>,
    class: u8,
    name: String,
    female: bool,
    age: Option<f64>,
    sibsp: u32,
    parch: u32,
    ticket: String,
    fare: Option<f64>,
    cabin: String,
    port: String,
}

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

fn load(path: &Path) -> Result<Vec<P>> {
    let t = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    let mut lines = t.lines();
    let head = split_csv(lines.next().context("empty file")?);
    let col = |n: &str| head.iter().position(|h| h == n);
    let mut out = Vec::new();
    for l in lines.filter(|l| !l.trim().is_empty()) {
        let c = split_csv(l);
        let get = |n: &str| col(n).and_then(|i| c.get(i)).cloned().unwrap_or_default();
        out.push(P {
            id: get("PassengerId").parse()?,
            survived: col("Survived").map(|_| get("Survived") == "1"),
            class: get("Pclass").parse()?,
            name: get("Name"),
            female: get("Sex") == "female",
            age: get("Age").parse().ok(),
            sibsp: get("SibSp").parse().unwrap_or(0),
            parch: get("Parch").parse().unwrap_or(0),
            ticket: get("Ticket"),
            fare: get("Fare").parse().ok(),
            cabin: get("Cabin"),
            port: get("Embarked"),
        });
    }
    Ok(out)
}

/// The title from "Surname, Title. Given names".
fn title(p: &P) -> &'static str {
    let t = p.name.split(", ").nth(1).and_then(|r| r.split('.').next()).unwrap_or("").trim();
    match t {
        "Mr" => "Mr",
        "Mrs" | "Mme" => "Mrs",
        "Miss" | "Mlle" | "Ms" => "Miss",
        "Master" => "Master",
        _ => "rare",
    }
}

fn surname(p: &P) -> &str {
    p.name.split(',').next().unwrap_or("")
}

/// Discrete features as (name, value) — the vocabulary of the decision list.
fn feats(p: &P) -> Vec<(&'static str, String)> {
    let age = match p.age {
        None => "unknown",
        Some(a) if a < 13.0 => "child",
        Some(a) if a < 20.0 => "teen",
        Some(a) if a < 50.0 => "adult",
        Some(_) => "old",
    };
    let fam = match p.sibsp + p.parch {
        0 => "alone",
        1..=3 => "small",
        _ => "large",
    };
    let fare = match p.fare {
        None => "unknown",
        Some(f) if f < 8.0 => "low",
        Some(f) if f < 15.0 => "mid",
        Some(f) if f < 50.0 => "high",
        Some(_) => "top",
    };
    let deck = p.cabin.chars().next().map(|c| c.to_string()).unwrap_or_else(|| "none".into());
    vec![
        ("sex", if p.female { "female".into() } else { "male".into() }),
        ("class", p.class.to_string()),
        ("title", title(p).into()),
        ("age", age.into()),
        ("family", fam.into()),
        ("fare", fare.into()),
        ("port", if p.port.is_empty() { "unknown".into() } else { p.port.clone() }),
        ("deck", deck),
    ]
}

type Cond = Vec<(&'static str, String)>;

/// A decision list: rules in order, the first that covers a passenger decides; the default last.
struct List {
    rules: Vec<(Cond, bool, usize, usize)>,
    default: bool,
}

fn covers(c: &Cond, f: &[(&'static str, String)]) -> bool {
    c.iter().all(|x| f.contains(x))
}

/// Greedy learning: at each step the condition (one or two features) with the highest Laplace accuracy among
/// the uncovered passengers (at least `min` covered) becomes a rule; stop when no rule beats the default by 2 points.
fn learn(train: &[P], min: usize) -> List {
    let fx: Vec<Vec<(&'static str, String)>> = train.iter().map(feats).collect();
    let y: Vec<bool> = train.iter().map(|p| p.survived.unwrap_or(false)).collect();
    let mut left: Vec<usize> = (0..train.len()).collect();
    let mut rules = Vec::new();
    loop {
        let base_pos = left.iter().filter(|&&i| y[i]).count();
        let base_acc = base_pos.max(left.len() - base_pos) as f64 / left.len().max(1) as f64;
        let mut cands: BTreeMap<Cond, (usize, usize)> = BTreeMap::new();
        for &i in &left {
            let f = &fx[i];
            for a in 0..f.len() {
                let e = cands.entry(vec![f[a].clone()]).or_default();
                if y[i] { e.0 += 1 } else { e.1 += 1 }
                for b in a + 1..f.len() {
                    let e = cands.entry(vec![f[a].clone(), f[b].clone()]).or_default();
                    if y[i] { e.0 += 1 } else { e.1 += 1 }
                }
            }
        }
        let best = cands
            .into_iter()
            .filter(|(_, (p, n))| p + n >= min)
            .map(|(c, (p, n))| {
                let lap = (p.max(n) + 1) as f64 / (p + n + 2) as f64;
                (lap, c, p, n)
            })
            .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(b.1.len().cmp(&a.1.len())).then(b.1.cmp(&a.1)));
        let Some((lap, c, p, n)) = best else { break };
        if lap < base_acc + 0.02 || rules.len() >= 12 {
            break;
        }
        left.retain(|&i| !covers(&c, &fx[i]));
        rules.push((c, p > n, p, n));
        if left.len() < min {
            break;
        }
    }
    let pos = left.iter().filter(|&&i| y[i]).count();
    List { rules, default: pos * 2 > left.len() }
}

fn predict_list(l: &List, p: &P) -> (bool, String) {
    let f = feats(p);
    for (c, v, pos, neg) in &l.rules {
        if covers(c, &f) {
            let cs: Vec<String> = c.iter().map(|(k, v)| format!("{k}={v}")).collect();
            return (*v, format!("rule {} ({pos} survived, {neg} died in train)", cs.join(" & ")));
        }
    }
    (l.default, "default".into())
}

/// Group keys of a passenger: surname + fare, and the ticket (a group shares either).
fn groups(all: &[&P]) -> BTreeMap<u32, Vec<u32>> {
    let mut by: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    for p in all {
        by.entry(format!("n:{}:{}", surname(p), p.fare.map(|f| format!("{f:.2}")).unwrap_or_default())).or_default().push(p.id);
        by.entry(format!("t:{}", p.ticket)).or_default().push(p.id);
    }
    let mut out: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for ids in by.values().filter(|v| v.len() > 1) {
        for &a in ids {
            let e = out.entry(a).or_default();
            for &b in ids {
                if b != a && !e.contains(&b) {
                    e.push(b);
                }
            }
        }
    }
    out
}

/// Woman-child groups: the gender model corrected by what the group's women and boys did in train.
fn predict_wcg(train: &[P], all: &[&P], p: &P) -> (bool, String) {
    let g = groups(all);
    let known: BTreeMap<u32, &P> = train.iter().map(|x| (x.id, x)).collect();
    let wc = |x: &P| x.female || title(x) == "Master";
    let mates: Vec<&P> = g.get(&p.id).map(|v| v.iter().filter_map(|id| known.get(id).copied()).filter(|x| wc(x)).collect()).unwrap_or_default();
    let lived = mates.iter().filter(|x| x.survived == Some(true)).count();
    let died = mates.iter().filter(|x| x.survived == Some(false)).count();
    if p.female && died > 0 && lived == 0 {
        return (false, format!("woman whose group's {died} women and boys all died"));
    }
    if title(p) == "Master" && lived > 0 && died == 0 {
        return (true, format!("boy whose group's {lived} women and boys all lived"));
    }
    (p.female, if p.female { "woman (gender)".into() } else { "man or boy (gender)".into() })
}

fn cv<F: Fn(&[P], &P) -> bool>(train: &[P], k: usize, f: F) -> f64 {
    let mut ok = 0;
    for fold in 0..k {
        let (test, tr): (Vec<P>, Vec<P>) = train.iter().cloned().partition(|p| (p.id as usize) % k == fold);
        for p in &test {
            ok += usize::from(f(&tr, p) == p.survived.unwrap_or(false));
        }
    }
    100.0 * ok as f64 / train.len() as f64
}

fn titanic(dir: &Path, out: Option<&Path>) -> Result<()> {
    let train = load(&dir.join("train.csv"))?;
    let test = load(&dir.join("test.csv"))?;
    println!("train {} passengers, test {}", train.len(), test.len());
    let g = cv(&train, 10, |_, p| p.female);
    let d = cv(&train, 10, |tr, p| predict_list(&learn(tr, 10), p).0);
    // in a fold, the groups are found over everyone (train and test names are public), outcomes only from the fold's train
    let everyone: Vec<P> = train.iter().chain(test.iter()).cloned().collect();
    let w = cv(&train, 10, |tr, p| {
        let all: Vec<&P> = everyone.iter().collect();
        predict_wcg(tr, &all, p).0
    });
    println!("10-fold cross-validation on train: gender {g:.2}%, decision list {d:.2}%, woman-child groups {w:.2}%");
    let list = learn(&train, 10);
    println!("decision list on all train:");
    for (c, v, p, n) in &list.rules {
        let cs: Vec<String> = c.iter().map(|(k, v)| format!("{k}={v}")).collect();
        println!("  if {} → {} ({p} survived, {n} died)", cs.join(" & "), if *v { "lives" } else { "dies" });
    }
    println!("  otherwise → {}", if list.default { "lives" } else { "dies" });
    if let Some(o) = out {
        let all: Vec<&P> = everyone.iter().collect();
        let mut csv = String::from("PassengerId,Survived\n");
        let mut why: BTreeMap<String, usize> = BTreeMap::new();
        for p in &test {
            let (s, r) = predict_wcg(&train, &all, p);
            *why.entry(r.split(" whose").next().unwrap_or(&r).to_string() + if r.contains("whose") { " (group)" } else { "" }).or_default() += 1;
            csv.push_str(&format!("{},{}\n", p.id, u8::from(s)));
        }
        std::fs::write(o, csv)?;
        println!("submission (woman-child groups) → {}: {}", o.display(), why.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
    }
    Ok(())
}

fn main() -> Result<()> {
    let a: Vec<String> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("titanic") => {
            let dir = a.get(2).context("usage: kaggle titanic <data dir> [--out submission.csv]")?;
            let out = a.iter().position(|x| x == "--out").and_then(|i| a.get(i + 1));
            titanic(Path::new(dir), out.map(Path::new))
        }
        _ => bail!("usage: kaggle titanic <data dir> [--out submission.csv]"),
    }
}
