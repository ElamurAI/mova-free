//! mova-hard-cases — the loop from reading failures to learning (see the README).
use anyhow::{bail, Result};
use hard_cases::config::Config;
use hard_cases::{control, eel, harvest, judge, wave, witness};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::Path;

const USAGE: &str = "usage: mova-hard-cases COMMAND CONFIG …
  list CONFIG                         the one list: cases by source, licence track, parser view
  wave CONFIG [--dry]                 the next Fish priority-lane wave (tasks, pages, pager configs)
  lane CONFIG                         the waves and their pagers' progress
  harvest CONFIG                      validated Fish replies → learn/<track>/<split>.conllu
  witness CONFIG TRACK                Stanza, Trankit, spaCy trf on the track's training trees (GPU lease)
  eel CONFIG TRACK                    one hard-case Eel round for the track, when its data is ready
  sample CONFIG [NAME]                the fixed measurement sample (built once)
  measure CONFIG MODEL_DIR [LABEL] [NAME]   pass rate, silver LAS and EWT LAS of a model on the sample
  recheck CONFIG [--force]            the list re-judged with the deployed gparse
  watch CONFIG [SECONDS] [NAME]       measure and recheck whenever the deployed gparse changes (STOP file stops)
  guard CONFIG                        lane priority (generic pagers yield) and the shared limit guard";

fn print(v: &Value) {
    println!("{}", serde_json::to_string_pretty(v).unwrap_or_default());
}

fn list(cfg: &Config) -> Result<Value> {
    let st = hard_case_list::state(&cfg.list)?;
    let mut by: BTreeMap<String, usize> = BTreeMap::new();
    for c in st.values() {
        let track = cfg.policy.track(&c.found.licence);
        let admitted = cfg.policy.admits(&c.found.why);
        *by.entry(format!("{} | {} | en {} | gparse {} | {}", c.found.source, track, c.status("en"), c.status("gparse"), if admitted { "lane kinds" } else { "other kinds" })).or_default() += 1;
    }
    Ok(json!({"list": cfg.list, "cases": st.len(), "by": by}))
}

fn run(a: &[String]) -> Result<()> {
    if a.len() < 2 {
        bail!("{USAGE}");
    }
    let cfg = Config::load(Path::new(&a[1]))?;
    let rest = &a[2..];
    let flag = |f: &str| rest.iter().any(|x| x == f);
    let pos: Vec<&String> = rest.iter().filter(|x| !x.starts_with("--")).collect();
    match a[0].as_str() {
        "list" => print(&list(&cfg)?),
        "wave" => print(&wave::build(&cfg, flag("--dry"))?),
        "lane" => print(&wave::status(&cfg)?),
        "harvest" => print(&harvest::harvest(&cfg)?),
        "witness" => print(&witness::run(&cfg, pos.first().map_or("internal", |s| s.as_str()))?),
        "eel" => print(&eel::round(&cfg, pos.first().map_or("internal", |s| s.as_str()))?),
        "sample" => {
            let s = judge::sample(&cfg, pos.first().map_or("sample", |s| s.as_str()))?;
            print(&json!({"cases": s.len(), "measure_split": s.iter().filter(|c| c.split == "measure").count(), "with_stanza": s.iter().filter(|c| c.stanza.is_some()).count()}));
        }
        "measure" => {
            let Some(m) = pos.first() else { bail!("{USAGE}") };
            print(&judge::measure(&cfg, Path::new(m.as_str()), pos.get(2).map_or("sample", |s| s.as_str()), pos.get(1).map_or("manual", |s| s.as_str()))?);
        }
        "recheck" => print(&judge::recheck(&cfg, None, flag("--force"))?),
        "watch" => judge::watch(&cfg, pos.get(1).map_or("sample", |s| s.as_str()), pos.first().and_then(|s| s.parse().ok()).unwrap_or(600))?,
        "guard" => control::guard(&cfg)?,
        _ => bail!("{USAGE}"),
    }
    Ok(())
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = run(&a) {
        eprintln!("mova-hard-cases: {e:#}");
        std::process::exit(1);
    }
}
