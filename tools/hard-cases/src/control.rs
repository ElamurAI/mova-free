//! The lane's place among the Fish: priority and the shared limit.
//!
//! **Priority.** While a lane pager is working (a fresh heartbeat, a state that is not terminal),
//! the generic pagers it stands in for (`yield_roots`) hold an owned `PAUSE`, so the lane's hard
//! cases take their share of the Fish first and the pool stays at its size. The generic pagers stop
//! at their next job boundary (an accepted attempt finishes); when no lane pager works, only the
//! exact owned `PAUSE` bytes are removed. A foreign `PAUSE` (the operator's) is never touched.
//!
//! **Limit.** The lane pagers share the ChatGPT limit with the generic ones. A lane pager that turns
//! `uncertain` stops every lane pager and every `stop_roots` root (the generic pagers and their
//! producer) at their next job boundary; a generic root stopped by its own limit guard stops the
//! lane pagers. No retry, no automatic resume.
use crate::config::Config;
use crate::read_json;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Control {
    /// Generic pager roots that pause while the lane works.
    #[serde(default)]
    pub yield_roots: Vec<PathBuf>,
    /// Roots stopped when a lane pager turns uncertain (generic pagers, their producer).
    #[serde(default)]
    pub stop_roots: Vec<PathBuf>,
    #[serde(default = "poll")]
    pub poll_seconds: u64,
    /// A lane heartbeat older than this does not count as working.
    #[serde(default = "fresh")]
    pub fresh_seconds: u64,
}

fn poll() -> u64 {
    20
}
fn fresh() -> u64 {
    120
}

const TERMINAL: [&str; 4] = ["plan_exhausted", "stopped", "deadline_reached", "uncertain"];

/// The owned PAUSE bytes.
pub fn marker(cfg: &Config) -> String {
    format!("hard-lane priority: {}\n", cfg.lane_root.display())
}

/// The roots of every lane pager of every complete wave.
pub fn lane_roots(cfg: &Config) -> Result<Vec<PathBuf>> {
    let mut out = vec![];
    let Ok(d) = std::fs::read_dir(&cfg.lane_root) else { return Ok(out) };
    let mut waves: Vec<PathBuf> = d.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.join("wave.json").exists()).collect();
    waves.sort();
    for w in waves {
        for pc in read_json(&w.join("wave.json"))?["pagers"].as_array().cloned().unwrap_or_default() {
            if let Some(r) = read_json(Path::new(pc.as_str().unwrap_or("")))?["root"].as_str() {
                out.push(PathBuf::from(r));
            }
        }
    }
    Ok(out)
}

/// A pager's state, and whether it is working now.
pub fn pager(root: &Path, now: u64, fresh: u64) -> (String, bool) {
    let s = read_json(&root.join("status.json")).unwrap_or(json!({}));
    let state = s["state"].as_str().unwrap_or("not started").to_string();
    let beat = s["heartbeat_unix"].as_u64().unwrap_or(0);
    let working = s.get("state").is_some() && !TERMINAL.contains(&state.as_str()) && now.saturating_sub(beat) <= fresh && !root.join("STOP").exists();
    (state, working)
}

fn stop(root: &Path, why: &str) -> Result<bool> {
    let p = root.join("STOP");
    if p.exists() {
        return Ok(false);
    }
    std::fs::create_dir_all(root)?;
    std::fs::write(&p, format!("hard-lane guard: {why}\n"))?;
    Ok(true)
}

/// One pass: returns what it did.
pub fn step(cfg: &Config, c: &Control, now: u64) -> Result<Value> {
    let lanes = lane_roots(cfg)?;
    let mut working = 0;
    let mut events = vec![];
    for r in &lanes {
        let (state, w) = pager(r, now, c.fresh_seconds);
        working += usize::from(w);
        if state == "uncertain" {
            for x in lanes.iter().chain(&c.stop_roots) {
                if stop(x, &format!("lane pager {} uncertain", r.display()))? {
                    events.push(json!({"stop": x}));
                }
            }
            return Ok(json!({"uncertain": r, "events": events, "exit": true}));
        }
    }
    // A generic root stopped by its own limit guard stops the lane.
    for x in &c.stop_roots {
        let s = std::fs::read_to_string(x.join("STOP")).unwrap_or_default();
        if s.starts_with("limit-guard") {
            for r in &lanes {
                if stop(r, &format!("generic root {} stopped by its limit guard", x.display()))? {
                    events.push(json!({"stop": r}));
                }
            }
        }
    }
    let m = marker(cfg);
    for y in &c.yield_roots {
        let p = y.join("PAUSE");
        let have = std::fs::read_to_string(&p).ok();
        if working > 0 && have.is_none() {
            std::fs::write(&p, &m)?;
            events.push(json!({"pause": y}));
        } else if working == 0 && have.as_deref() == Some(m.as_str()) {
            std::fs::remove_file(&p)?;
            events.push(json!({"resume": y}));
        }
    }
    Ok(json!({"lane_pagers": lanes.len(), "working": working, "events": events, "exit": false}))
}

/// Runs until `<lane_root>/STOP` or a lane pager turns uncertain.
pub fn guard(cfg: &Config) -> Result<()> {
    let c = cfg.control.as_ref().ok_or_else(|| anyhow::anyhow!("the configuration has no control section"))?;
    loop {
        if cfg.lane_root.join("STOP").exists() {
            eprintln!("hard-lane guard: STOP found");
            return Ok(());
        }
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();
        let v = step(cfg, c, now)?;
        if !v["events"].as_array().is_none_or(Vec::is_empty) {
            eprintln!("{} {v}", now);
        }
        if v["exit"] == true {
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_secs(c.poll_seconds.clamp(5, 300)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_work_pauses_generic_pagers_and_uncertainty_stops_all() {
        let d = std::env::temp_dir().join(format!("hard-control-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        let lane = d.join("lane");
        let w = lane.join("wave-0001");
        std::fs::create_dir_all(w.join("workers/h00")).unwrap();
        std::fs::write(w.join("h00-config.json"), json!({"root": w.join("workers/h00")}).to_string()).unwrap();
        std::fs::write(w.join("wave.json"), json!({"pagers": [w.join("h00-config.json")]}).to_string()).unwrap();
        let (t06, t07, mat) = (d.join("t06"), d.join("t07"), d.join("materializer"));
        for x in [&t06, &t07, &mat] {
            std::fs::create_dir_all(x).unwrap();
        }
        let cfg: Config = serde_json::from_value(json!({"schema": "mova-hard-cases-v1", "list": d.join("list"), "lane_root": lane, "sources": {}, "en_model": "/x", "prag_silver": "/x", "gparse": {"executable": "/x", "model_dir": "/x"}, "fish": null, "witness": null})).unwrap();
        let c = Control { yield_roots: vec![t06.clone(), t07.clone()], stop_roots: vec![t06.clone(), t07.clone(), mat.clone()], poll_seconds: 20, fresh_seconds: 120 };
        let status = |state: &str, beat: u64| std::fs::write(w.join("workers/h00/status.json"), json!({"state": state, "heartbeat_unix": beat}).to_string()).unwrap();
        // the operator's own PAUSE on t07 stays whatever happens
        std::fs::write(t07.join("PAUSE"), "operator\n").unwrap();
        status("active", 1000);
        step(&cfg, &c, 1010).unwrap();
        assert_eq!(std::fs::read_to_string(t06.join("PAUSE")).unwrap(), marker(&cfg));
        assert_eq!(std::fs::read_to_string(t07.join("PAUSE")).unwrap(), "operator\n");
        // a stale heartbeat is not work: the owned pause goes, the foreign one stays
        step(&cfg, &c, 5000).unwrap();
        assert!(!t06.join("PAUSE").exists());
        assert!(t07.join("PAUSE").exists());
        status("plan_exhausted", 5000);
        step(&cfg, &c, 5001).unwrap();
        assert!(!t06.join("PAUSE").exists());
        // red: an uncertain lane pager stops the lane and the generic roots
        status("uncertain", 6000);
        let v = step(&cfg, &c, 6001).unwrap();
        assert_eq!(v["exit"], true);
        for x in [&t06, &t07, &mat, &w.join("workers/h00")] {
            assert!(x.join("STOP").exists(), "{}", x.display());
        }
        // a generic limit-guard stop reaches the lane
        let _ = std::fs::remove_file(w.join("workers/h00/STOP"));
        status("active", 7000);
        std::fs::write(mat.join("STOP"), "limit-guard: t03 uncertain\n").unwrap();
        step(&cfg, &c, 7001).unwrap();
        assert!(w.join("workers/h00/STOP").exists());
        let _ = std::fs::remove_dir_all(d);
    }
}
