//! `yomhadin` — Yom HaDin, the Last Judgment, on the command line. A study of reasoning, not legal advice.
//!
//! ```text
//! yomhadin jurisdictions                      every jurisdiction: place, era, body, status, rules
//! yomhadin extract [<jurisdiction> …]         read the rules out of the texts (sources in jurisdiction.tsv)
//! yomhadin rules [<jurisdiction>] [--authority <pat>] [--force <f>] [--max N]
//! yomhadin judge "<case>" [--lens L] [--max N] [--depth D] [--in a,b] [--forum J] [--given S] [--flagged]
//!          [--weights a=1,b=0.5] [--protocol DIR | --no-protocol]
//!          sections: all, compare, forum, modern, history, slices, bounds, flips, values, goals, corrupt (bribed),
//!          outlier (with --given), fiction (legal devices), absurd (measurable absurdity);
//!          detail lenses: paths, honest (majority), strict, lenient, loopholes, detail
//! yomhadin sample                             the sentences of the gold sample (text only, for writing the gold)
//! yomhadin measure [--gold F] [--cases F]     rule counts, precision, controls, lattice sizes, two runs compared
//! ```

use anyhow::{Context, Result, bail};
use std::collections::BTreeSet;
use std::path::PathBuf;

use yomhadin::court;
use yomhadin::extract;
use yomhadin::juris::{self, Jurisdiction};
use yomhadin::measure;
use yomhadin::report::{self, Opts};
use yomhadin::rule::write_rules;

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1).cloned())
}

fn positional(args: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut skip = false;
    for a in args {
        if skip {
            skip = false;
            continue;
        }
        if a.starts_with("--") {
            skip = !BOOL_FLAGS.contains(&a.as_str());
            continue;
        }
        out.push(a.clone());
    }
    out
}

const BOOL_FLAGS: [&str; 2] = ["--flagged", "--no-protocol"];

fn opts_of(rest: &[String]) -> Result<Opts> {
    let mut weights = std::collections::BTreeMap::new();
    if let Some(w) = flag(rest, "--weights") {
        for kv in w.split(',') {
            let (k, v) = kv.split_once('=').context("--weights a=1,b=0.5")?;
            weights.insert(k.to_string(), v.parse::<f64>().context("a weight is a number")?);
        }
    }
    Ok(Opts {
        lens: flag(rest, "--lens").unwrap_or_else(|| "all".into()),
        max: flag(rest, "--max").and_then(|m| m.parse().ok()).unwrap_or(5),
        depth: flag(rest, "--depth").and_then(|m| m.parse().ok()).unwrap_or(3),
        only: flag(rest, "--in").map(|s| s.split(',').map(str::to_string).collect()).unwrap_or_default(),
        forum: flag(rest, "--forum"),
        given: flag(rest, "--given").and_then(|g| g.parse().ok()),
        flagged: rest.iter().any(|a| a == "--flagged"),
        weights,
    })
}

fn crate_file(name: &str) -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"))).join(name)
}

const EXTRACTED: [&str; 2] = ["torah", "yerushalmi"];

fn do_extract(js: &[Jurisdiction], ids: &[String]) -> Result<()> {
    for j in js {
        if !ids.is_empty() && !ids.contains(&j.id) {
            continue;
        }
        let srcs = j.sources();
        if srcs.is_empty() {
            continue;
        }
        let mut sents = Vec::new();
        let mut tree_files: Vec<PathBuf> = Vec::new();
        for (kind, spec) in &srcs {
            let row = j.meta["source"].iter().find(|r| r.first() == Some(kind) && r.get(1) == Some(spec)).unwrap();
            match kind.as_str() {
                "pool" => {
                    let pool = extract::expand(row.get(2).context("pool source needs a pool path")?)?;
                    let auth = |addr: &str| -> String {
                        let p: Vec<&str> = addr.split('.').collect();
                        if p.len() == 4 {
                            if p[3] == "1" { "Mishnah (anonymous)".into() } else { "Halakhah (anonymous)".into() }
                        } else {
                            "Torah".into()
                        }
                    };
                    sents.extend(extract::read_pool(&pool, spec, &auth)?);
                    if let Some(t) = row.get(3) {
                        tree_files.push(extract::expand(t)?);
                    }
                }
                "text" => sents.extend(extract::read_text(&j.dir.join(spec))?),
                k => bail!("{}: unknown source kind {k}", j.id),
            }
        }
        let ids: BTreeSet<String> = sents.iter().map(|s| s.id.clone()).collect();
        let mut trees = std::collections::BTreeMap::new();
        for t in &tree_files {
            trees.extend(extract::read_trees(t, &ids)?);
        }
        let (rules, db, slm) = extract::extract(&j.id, &sents, &trees)?;
        write_rules(&j.dir.join("rules.tsv"), &rules)?;
        eprintln!("{}: {} sentences ({} agreed trees from the parse database, {} parsed by the en SLM now) → {} rules", j.id, sents.len(), db, slm, rules.len());
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("help");
    let rest = &args[args.len().min(1)..];
    let root = juris::root();
    match cmd {
        "jurisdictions" => {
            let js = juris::load_all(&root)?;
            for j in &js {
                println!("{:<14} {:<10} {:<7} {:>5} rules  {} — {}; {}; {}", j.id, j.kind(), j.status(), j.rules.len(), j.name(), j.get("place").cloned().unwrap_or_default(), j.get("era").cloned().unwrap_or_default(), j.get("body").cloned().unwrap_or_default());
                if let Some(a) = j.get("admits") {
                    println!("{:<14} admits: {a}; legal devices: {}", "", j.get("devices").cloned().unwrap_or_else(|| if j.devices.is_empty() { "none listed".into() } else { "yes".into() }));
                }
            }
        }
        "extract" => {
            let js = juris::load_all(&root)?;
            do_extract(&js, &positional(rest))?;
        }
        "rules" => {
            let js = juris::load_all(&root)?;
            let pos = positional(rest);
            let auth = flag(rest, "--authority").map(|a| a.to_lowercase());
            let force = flag(rest, "--force");
            let max: usize = flag(rest, "--max").and_then(|m| m.parse().ok()).unwrap_or(usize::MAX);
            for j in &js {
                if !pos.is_empty() && !pos.contains(&j.id) {
                    continue;
                }
                // by authority, in the jurisdiction's order of precedence
                let mut by: std::collections::BTreeMap<(u8, String), Vec<&yomhadin::rule::Rule>> = std::collections::BTreeMap::new();
                for r in &j.rules {
                    if auth.as_ref().is_some_and(|a| !r.authority.to_lowercase().contains(a)) || force.as_ref().is_some_and(|f| r.force.name() != f) {
                        continue;
                    }
                    let (_, rank) = j.authority(&r.authority);
                    by.entry((rank, r.authority.clone())).or_default().push(r);
                }
                println!("== {} — {} ({} rules)", j.id, j.name(), j.rules.len());
                for ((rank, a), rs) in by {
                    println!("  -- {a} (rank {rank}, {} rules)", rs.len());
                    for r in rs.iter().take(max) {
                        let conds: Vec<String> = r.conds.iter().map(|g| g.iter().map(|c| c.show()).collect::<Vec<_>>().join(" or ")).collect();
                        let excs: Vec<String> = r.excs.iter().map(|g| g.iter().map(|c| c.show()).collect::<Vec<_>>().join(" and ")).collect();
                        println!(
                            "    {} [{}] {}: {}{}{}{}{}",
                            r.id,
                            r.source,
                            r.force.name(),
                            r.acts.iter().map(|c| c.show()).collect::<Vec<_>>().join(", "),
                            if conds.is_empty() { String::new() } else { format!(" | if {}", conds.join("; ")) },
                            if excs.is_empty() { String::new() } else { format!(" | except {}", excs.join("; ")) },
                            if r.penalty.is_empty() { String::new() } else { format!(" | penalty: {}", r.penalty) },
                            if r.dispute.is_empty() { String::new() } else { format!(" | {}", r.dispute) }
                        );
                    }
                }
            }
        }
        "judge" => {
            let pos = positional(rest);
            let text = pos.first().context("give a case in quotes")?;
            let js = juris::load_all(&root)?;
            let opts = opts_of(rest)?;
            let case = court::read_case(text)?;
            let rep = report::report(&js, &case, &opts);
            print!("{rep}");
            if !rest.iter().any(|a| a == "--no-protocol") {
                let dir = match flag(rest, "--protocol") {
                    Some(d) => PathBuf::from(d),
                    None => PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/yom-hadin/protocols"),
                };
                let p = report::protocol(&js, &case, &opts, &rep);
                std::fs::create_dir_all(&dir)?;
                let name = format!("{:016x}.txt", report::fnv(format!("{}\n{}", case.text, opts.canon()).as_bytes()));
                std::fs::write(dir.join(&name), p)?;
                eprintln!("protocol: {}", dir.join(&name).display());
            }
        }
        "sample" => {
            // the sentences of the gold (to write a gold for new ones, list rules with `rules`)
            let order = measure::gold_order(&crate_file("data/gold.tsv"))?;
            let js = juris::load_all(&root)?;
            for sent in &order {
                if let Some(r) = js.iter().flat_map(|j| j.rules.iter()).find(|r| &r.sent == sent) {
                    println!("{}\t{}\t{}", r.sent, r.source, r.text);
                } else {
                    println!("{sent}\t-\t(no rule extracted now)");
                }
            }
        }
        "measure" => {
            let js = juris::load_all(&root)?;
            println!("== Rules (2026-10-18)\n{}", measure::counts(&js));
            let gold_path = flag(rest, "--gold").map(PathBuf::from).unwrap_or_else(|| crate_file("data/gold.tsv"));
            if gold_path.exists() {
                let gold = measure::read_gold(&gold_path)?;
                let order = measure::gold_order(&gold_path)?;
                let s = measure::sample(&js, &gold);
                println!("== Precision on the {} hand-checked sentences: every rule extracted from them", order.len());
                println!("  real:                         {}", measure::precision(&s, &gold, &order, 0));
                println!("  control, gold of the next sentence: {}", measure::precision(&s, &gold, &order, 1));
                for id in EXTRACTED {
                    let part: Vec<_> = s.iter().filter(|r| r.jur == id).copied().collect();
                    let g: std::collections::BTreeMap<String, measure::Gold> = gold.iter().filter(|(k, _)| part.iter().any(|r| &r.sent == *k) || (id == "torah" && k.starts_with("jps:")) || (id == "yerushalmi" && k.starts_with("yer:"))).map(|(k, v)| (k.clone(), v.clone())).collect();
                    println!("  {id:<12}                  {}; recall {}", measure::precision(&part, &gold, &order, 0), measure::recall(&part, &g));
                }
                println!("  recall, all:                  {}", measure::recall(&s, &gold));
            }
            let cases_path = flag(rest, "--cases").map(PathBuf::from).unwrap_or_else(|| crate_file("data/cases.tsv"));
            let cases: Vec<(String, String)> = std::fs::read_to_string(&cases_path).with_context(|| format!("{}", cases_path.display()))?.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).filter_map(|l| l.split_once('\t')).map(|(a, b)| (a.to_string(), b.to_string())).collect();
            println!("== Lattices of the example cases");
            println!("{:<10} {:<10} {:<12} {:>5} {:>6} {:>10} {:>8} {:>5} {:>4} {:>6}  honest verdict", "case", "defendant", "jurisdiction", "rules", "points", "paths", "verdicts", "sens", "🐭0", "absurd");
            let opts = Opts::default();
            let mut texts = Vec::new();
            for (id, text) in &cases {
                let case = court::read_case(text)?;
                texts.push(report::report(&js, &case, &opts));
                for d in court::defendants(&case) {
                    for g in report::judge_all(&js, &case, &d, &opts, false) {
                        let m = court::mice(&g.tips);
                        let acq = m.iter().filter(|(v, _)| v.acquittal()).map(|(_, &n)| n).min();
                        let fl = yomhadin::absurd::flags(&g.j, &case, &d, opts.depth);
                        let ab = fl.first().map(|f| format!("{}:{}", f.score, fl.len())).unwrap_or_else(|| "0".into());
                        println!("{:<10} {:<10} {:<12} {:>5} {:>6} {:>10} {:>8} {:>5.2} {:>4} {:>6}  {}", id, d, g.j.id, g.l.rules.len(), g.l.points.len(), g.l.total, g.l.counts.len(), court::sensitivity(&g.l, &g.tips), acq.map(|n| n.to_string()).unwrap_or_else(|| ">3".into()), ab, g.honest.0.label);
                    }
                }
            }
            // two runs in one process: the reports must be the same byte for byte
            let mut same = true;
            for ((_, text), first) in cases.iter().zip(&texts) {
                let case = court::read_case(text)?;
                same &= &report::report(&js, &case, &opts) == first;
            }
            println!("== Two runs identical: {same}");
        }
        _ => {
            println!("yomhadin jurisdictions | extract [<jur> …] | rules [<jur>] [--authority P] [--force F] [--max N] | judge \"<case>\" [--lens L] [--max N] [--depth D] [--in a,b] | sample | measure [--gold F] [--cases F]");
        }
    }
    Ok(())
}

#[cfg(test)]
mod wave31_tests {
    use super::*;

    #[test]
    fn argument_consumption_and_defaults() {
        let args: Vec<String> = [
            "case",
            "--flagged",
            "--unknown",
            "eaten",
            "--depth",
            "bad",
            "--weights",
            "x=2,x=3",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
        assert_eq!(positional(&args), ["case"]);
        let opts = opts_of(&args).unwrap();
        assert_eq!(opts.depth, 3);
        assert!(opts.flagged);
        assert_eq!(opts.weight("x"), 3.0);
        assert_eq!(opts.weight("missing"), 1.0);
        assert!(opts_of(&["--weights".into(), "bad".into()]).is_err());
        assert_eq!(
            flag(&["--max".into(), "--depth".into()], "--max"),
            Some("--depth".into())
        );
    }
}
