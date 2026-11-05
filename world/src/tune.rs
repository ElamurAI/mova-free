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

/// Enumerate one-knob neighbors in declaration order.
/// Every knob name must exist in base. Candidate values are kept verbatim, including duplicates;
/// the first value within absolute distance 1e-9 selects its adjacent entries.
pub fn neighboring_configs(
    knobs: &[(&'static str, f64, Vec<f64>)],
    base: &BTreeMap<String, f64>,
) -> Vec<(String, BTreeMap<String, f64>)> {
    let mut deltas: Vec<(String, BTreeMap<String, f64>)> = Vec::new();
    for (k, _, vals) in knobs {
        let cur = base[*k];
        let pos = vals.iter().position(|v| (*v - cur).abs() < 1e-9);
        let neigh: Vec<f64> = match pos {
            Some(p) => [p.checked_sub(1), Some(p + 1)]
                .into_iter()
                .flatten()
                .filter_map(|i| vals.get(i).copied())
                .collect(),
            None => vals.clone(),
        };
        for v in neigh {
            let mut cfg = base.clone();
            cfg.insert(k.to_string(), v);
            deltas.push((format!("{k}: {cur} → {v}"), cfg));
        }
    }
    deltas
}

/// Select the greatest dev gain among candidates with positive dev gain,
/// nonnegative validation gain and p < 0.05; ties use the lower candidate index.
/// This is a pure selection rule, not evidence of an untouched final test set.
pub fn select_candidate(
    results: &[(usize, i64, i64, f64, bool)],
) -> Option<(usize, i64, i64, f64, bool)> {
    results
        .iter()
        .filter(|x| x.1 > 0 && x.2 >= 0 && x.3 < 0.05)
        .min_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)))
        .copied()
}

/// Run the loop on a component.
pub fn tune(c: &Component, gens: usize, threads: usize, adopt: bool, data_key: &str) -> Result<BTreeMap<String, f64>> {
    let mut base: BTreeMap<String, f64> = c.knobs.iter().map(|(k, v, _)| (k.to_string(), *v)).collect();
    let mut journal = Journal::open(PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/selfplay-journal.jsonl"));
    let code = code_key();
    for g in 0..gens {
        let bd = (c.score_dev)(&base);
        let bh = (c.score_held)(&base);
        println!("{} generation {}: base {}  dev {} held {}", c.name, g + 1, show(&base), bd.iter().sum::<i64>(), bh.iter().sum::<i64>());
        // deltas: each knob to its neighbouring values
        let mut deltas = neighboring_configs(&c.knobs, &base);
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
        let Some((i, gd, gh, p, _)) = select_candidate(&res) else {
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

#[cfg(test)]
mod wave14_models {
    use super::*;

    #[test]
    fn neighbors_follow_first_match_and_preserve_other_fields() {
        for len in 0..=5 {
            let values: Vec<_> = (0..len).map(|v| v as f64).collect();
            for current in -1..=len {
                let base = BTreeMap::from([("x".into(), current as f64), ("y".into(), 17.0)]);
                let got = neighboring_configs(&[("x", 99.0, values.clone())], &base);
                let expected: Vec<_> = values
                    .iter()
                    .copied()
                    .filter(|&v| current < 0 || current >= len || (v - current as f64).abs() == 1.0)
                    .collect();
                assert_eq!(
                    got.iter().map(|(_, c)| c["x"]).collect::<Vec<_>>(),
                    expected,
                    "adjacency at len={len}, current={current}"
                );
                for (label, c) in got {
                    assert_eq!(c["y"], 17.0);
                    assert_eq!(c.len(), 2);
                    assert_eq!(label, format!("x: {current} → {}", c["x"]));
                }
                assert_eq!(base["x"], current as f64);
            }
        }
    }

    #[test]
    fn tolerance_duplicates_empty_and_nonfinite_domain_are_explicit() {
        let base = BTreeMap::from([("x".into(), 1.0)]);
        let got = neighboring_configs(&[("x", 0.0, vec![1.0 + 0.5e-9, 1.0, 4.0])], &base);
        assert_eq!(got.len(), 1, "first tolerance match wins");
        assert_eq!(got[0].1["x"], 1.0, "duplicate/no-op neighbor is retained");
        assert!(neighboring_configs(&[], &base).is_empty());
        let got = neighboring_configs(&[("x", 0.0, vec![f64::NAN, f64::INFINITY])], &base);
        assert_eq!(got.len(), 2);
        assert!(got[0].1["x"].is_nan());
        assert!(got[1].1["x"].is_infinite());
    }

    #[test]
    fn selection_matches_independent_sorted_finite_oracle() {
        for gd in -1..=2 {
            for gh in -1..=1 {
                for p in [0.0, 0.049, 0.05, 1.0, f64::NAN] {
                    let rows = [
                        (3, gd, gh, p, false),
                        (1, 1, 0, 0.01, true),
                        (0, 9, -1, 0.0, false),
                        (2, 1, 0, 0.01, false),
                    ];
                    let mut expected = None;
                    for gain in (1..=9).rev() {
                        for id in 0..=3 {
                            if let Some(r) = rows
                                .iter()
                                .find(|r| r.0 == id && r.1 == gain && r.2 >= 0 && r.3 < 0.05)
                            {
                                expected = Some(*r);
                                break;
                            }
                        }
                        if expected.is_some() {
                            break;
                        }
                    }
                    assert_eq!(
                        select_candidate(&rows),
                        expected,
                        "safe maximum with index tie"
                    );
                    let reversed: Vec<_> = rows.into_iter().rev().collect();
                    assert_eq!(select_candidate(&reversed), expected);
                }
            }
        }
    }

    #[test]
    fn rejection_thresholds_and_configuration_order() {
        for r in [
            (0, 0, 0, 0.0, false),
            (0, 1, -1, 0.0, false),
            (0, 1, 0, 0.05, false),
            (0, 1, 0, f64::NAN, false),
        ] {
            assert!(
                select_candidate(&[r]).is_none(),
                "ineligible result must not be adopted"
            );
        }
        assert!(select_candidate(&[]).is_none());
        assert_eq!(
            show(&BTreeMap::from([("z".into(), 2.0), ("a".into(), 1.0)])),
            "a=1 z=2"
        );
    }
}
