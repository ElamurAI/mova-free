//! The SLM analyses and changes any component of itself: a component exposes its knobs as an abstract configuration
//! (name → value, with the values worth trying) and scores itself per item; the loop proposes deltas (one knob to a
//! neighbouring value), tests them in parallel on dev and held-out, keeps every result in the experiment journal
//! (shared with `world selfplay`, keyed by code version, component, base configuration and data), orders the
//! proposals by what the journal says helped before, and resets — the shipped configuration is never changed in
//! place; adoption (with `--adopt`) is in memory and reported.
//!
//!   world tune domains --dev <set.jsonl> --held <set.jsonl> [--gens N] [--threads T] [--adopt]
//!   world tune expressions --dev <dev.cupt> --held <test.cupt> …   (PARSEME 1.3 EN)

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Result, bail};

use crate::induce::{Journal, boot, code_key, fnv, verdict};

/// A component under tuning.
pub struct Component<'a> {
    pub name: &'static str,
    pub knobs: Vec<(&'static str, f64, Vec<f64>)>,
    pub score_dev: Box<dyn Fn(&BTreeMap<String, f64>) -> Vec<i64> + Sync + 'a>,
    pub score_held: Box<dyn Fn(&BTreeMap<String, f64>) -> Vec<i64> + Sync + 'a>,
}

fn show(cfg: &BTreeMap<String, f64>) -> String {
    cfg.iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join(" ")
}

/// Run the loop on a component.
pub fn tune(c: &Component, gens: usize, threads: usize, adopt: bool, data_key: &str) -> Result<BTreeMap<String, f64>> {
    let mut base: BTreeMap<String, f64> = c.knobs.iter().map(|(k, v, _)| (k.to_string(), *v)).collect();
    let mut journal = Journal::open(PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("runs/selfplay-journal.jsonl"));
    let code = code_key();
    for g in 0..gens {
        let bd = (c.score_dev)(&base);
        let bh = (c.score_held)(&base);
        println!("{} generation {}: base {}  dev {} held {}", c.name, g + 1, show(&base), bd.iter().sum::<i64>(), bh.iter().sum::<i64>());
        // deltas: each knob to its neighbouring values
        let mut deltas: Vec<(String, BTreeMap<String, f64>)> = Vec::new();
        for (k, _, vals) in &c.knobs {
            let cur = base[*k];
            let pos = vals.iter().position(|v| (*v - cur).abs() < 1e-9);
            let neigh: Vec<f64> = match pos {
                Some(p) => [p.checked_sub(1), Some(p + 1)].into_iter().flatten().filter_map(|i| vals.get(i).copied()).collect(),
                None => vals.clone(),
            };
            for v in neigh {
                let mut cfg = base.clone();
                cfg.insert(k.to_string(), v);
                deltas.push((format!("{k}: {cur} → {v}"), cfg));
            }
        }
        // order by the journal: knobs whose changes helped before come first
        let helped = |d: &str| journal.seen.values().filter(|v| v["component"] == c.name && v["knob"].as_str() == d.split(':').next() && v["verdict"] == "positive").count();
        deltas.sort_by(|a, b| helped(&b.0).cmp(&helped(&a.0)).then(a.0.cmp(&b.0)));
        let base_key = format!("{:x}", fnv(&show(&base)));
        let key = |d: &str| format!("{code}/{}/{base_key}/{data_key}/{d}", c.name);
        let res: std::sync::Mutex<Vec<(usize, i64, i64, f64, bool)>> = std::sync::Mutex::new(Vec::new());
        let next = std::sync::atomic::AtomicUsize::new(0);
        std::thread::scope(|sc| {
            for _ in 0..threads.max(1) {
                sc.spawn(|| loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let Some((d, cfg)) = deltas.get(i) else { break };
                    if let Some(v) = journal.seen.get(&key(d)) {
                        res.lock().unwrap().push((i, v["dev"].as_i64().unwrap_or(0), v["held"].as_i64().unwrap_or(0), v["p"].as_f64().unwrap_or(1.0), true));
                        continue;
                    }
                    let cd = (c.score_dev)(cfg);
                    let ch = (c.score_held)(cfg);
                    let diff: Vec<i64> = cd.iter().zip(&bd).map(|(a, b)| a - b).collect();
                    let gh: i64 = ch.iter().zip(&bh).map(|(a, b)| a - b).sum();
                    res.lock().unwrap().push((i, diff.iter().sum(), gh, boot(&diff), false));
                });
            }
        });
        let mut res = res.into_inner().unwrap();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        for (i, gd, gh, p, remembered) in &res {
            if !remembered {
                let d = &deltas[*i].0;
                journal.add(serde_json::json!({"key": key(d), "code": code, "time": now, "component": c.name, "kind": "knob", "knob": d.split(':').next(), "delta": d, "features": [d.split(':').next()], "dev": gd, "held": gh, "p": p, "verdict": verdict(*gd, *gh, *p)}))?;
            }
        }
        res.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let reused = res.iter().filter(|x| x.4).count();
        println!("  {} deltas ({} remembered from the journal):", res.len(), reused);
        for (i, gd, gh, p, rem) in &res {
            println!("    {:8} dev {gd:+4} held {gh:+4} p {p:.3}  {}{}", verdict(*gd, *gh, *p), deltas[*i].0, if *rem { "  [remembered]" } else { "" });
        }
        if !adopt {
            break;
        }
        let Some((i, gd, gh, p, _)) = res.iter().find(|x| x.1 > 0 && x.2 >= 0 && x.3 < 0.05).cloned() else {
            println!("  no delta significant on dev and safe on held-out — stop");
            break;
        };
        println!("  adopt (dev {gd:+}, held {gh:+}, p {p:.3}): {}", deltas[i].0);
        base = deltas[i].1.clone();
    }
    println!("{} final configuration (not shipped; review): {}", c.name, show(&base));
    Ok(base)
}

/// `world tune <component> …`.
pub fn run(component: &str, dev: &Path, held: &Path, gens: usize, threads: usize, adopt: bool) -> Result<()> {
    let data_key = format!("{:x}", fnv(&format!("{}:{}|{}:{}", dev.display(), std::fs::metadata(dev).map(|m| m.len()).unwrap_or(0), held.display(), std::fs::metadata(held).map(|m| m.len()).unwrap_or(0))));
    match component {
        "domains" => {
            let d = crate::domains::load_set(dev)?;
            let h = crate::domains::load_set(held)?;
            let c = Component {
                name: "domains",
                knobs: crate::domains::knobs(),
                score_dev: Box::new(|cfg| crate::domains::score_set(&d, cfg)),
                score_held: Box::new(|cfg| crate::domains::score_set(&h, cfg)),
            };
            let fin = tune(&c, gens, threads, adopt, &data_key)?;
            // the SLM writes its own hot layer: the tuned knobs go to <MOVA_HOT>/domains.cfg (WORLD_DOMAINS_CFG)
            if adopt {
                let hot = crate::induce::hot_dir();
                std::fs::create_dir_all(&hot)?;
                let body: String = fin.iter().map(|(k, v)| format!("{k}={v}\n")).collect();
                crate::store::write_atomic(&hot.join("domains.cfg"), format!("# hot layer written by world tune domains --adopt\n{body}").as_bytes())?;
                println!("hot layer: {}", hot.join("domains.cfg").display());
            }
            Ok(())
        }
        "expressions" => {
            let d = crate::mwe::read_cupt(dev)?;
            let h = crate::mwe::read_cupt(held)?;
            let base: BTreeMap<String, f64> = crate::mwe::knobs().into_iter().map(|(k, v, _)| (k.to_string(), v)).collect();
            let (p0, r0) = crate::mwe::pr(&h, &base);
            println!("expressions on held-out, shipped configuration: precision {p0:.1}% recall {r0:.1}%");
            let c = Component {
                name: "expressions",
                knobs: crate::mwe::knobs(),
                score_dev: Box::new(|cfg| crate::mwe::score(&d, cfg)),
                score_held: Box::new(|cfg| crate::mwe::score(&h, cfg)),
            };
            let fin = tune(&c, gens, threads, adopt, &data_key)?;
            let (p1, r1) = crate::mwe::pr(&h, &fin);
            println!("expressions on held-out, final configuration: precision {p1:.1}% recall {r1:.1}%");
            Ok(())
        }
        _ => bail!("unknown component {component}; known: domains, expressions"),
    }
}
