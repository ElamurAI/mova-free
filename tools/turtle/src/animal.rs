//! Sea Turtle as one of Mova's animals: its configuration, its hands for the deputy, and its English.
//!
//! The configuration (`turtle.example.json`) names the Turtle's journal, the folder its drafts go to and the
//! Nautilus layers it may make questions from. Operations of the handler `turtle` (trusted parameters, never the
//! prompt): `check` (a course: `world knowledge check`'s rule and the marks), `made` (what the journal says it made)
//! and `gap` (gap questions from a configured layer into a new draft folder).
use crate::{check, make};
use anyhow::{Context, Result, bail};
use mova_octopus::realize::{noun, noun_count, paragraph, sentence};
use mova_octopus::{Action, CapabilityHandler, Deputy};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub const OPERATIONS: [&str; 3] = ["check", "made", "gap"];

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayerSource {
    pub path: PathBuf,
    /// how its source is named in a lesson ("the command course")
    pub source: String,
    /// the course name of its draft lessons
    pub course: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub journal: PathBuf,
    pub drafts: PathBuf,
    #[serde(default)]
    pub layers: BTreeMap<String, LayerSource>,
    /// the courses of the standing loop (`turtle work`), names or folders
    #[serde(default)]
    pub courses: Vec<String>,
    /// Mova's annotator model (`en.bin`) that reads the slots of gap questions; `gap` refuses to run without it
    #[serde(default)]
    pub model: Option<PathBuf>,
}

impl Config {
    pub fn load(p: &Path) -> Result<Config> {
        let c: Config = serde_json::from_slice(&std::fs::read(p).with_context(|| format!("{}", p.display()))?)?;
        anyhow::ensure!(c.journal.is_absolute() && c.drafts.is_absolute(), "absolute journal and drafts required");
        Ok(c)
    }
}

pub fn say_check(r: &check::Report) -> String {
    let n = r.lessons.len() as u64;
    let name = &r.course;
    let mut out = vec![sentence(&format!(
        "I checked the {name} course: {} with {}",
        noun_count(n, "lesson"),
        noun_count(r.questions as u64, "question")
    ))];
    if r.with_problems == 0 {
        out.push(sentence("every lesson keeps the school's rules and its right answers are marked at varied places"));
    } else {
        out.push(sentence(&format!("{} {} a problem", noun_count(r.with_problems as u64, "lesson"), if r.with_problems == 1 { "has" } else { "have" })));
        for p in r.lessons.iter().flat_map(|l| l.problems.iter()).take(3) {
            out.push(sentence(p.trim_start_matches("✗ ")));
        }
    }
    paragraph(&out)
}

pub fn say_made(v: &Value) -> String {
    let q = v["questions"].as_u64().unwrap_or(0);
    if v["makings"].as_u64().unwrap_or(0) == 0 {
        return sentence("I have not made any questions yet");
    }
    sentence(&format!(
        "I have made {} in {}, in {} of drafts",
        noun_count(q, "question"),
        noun_count(v["lessons"].as_u64().unwrap_or(0), "lesson"),
        noun_count(v["makings"].as_u64().unwrap_or(0), "set")
    ))
}

pub fn say_gap(v: &Value, source: &str) -> String {
    paragraph(&[
        sentence(&format!(
            "I have made {} from {source} in {}, as a draft for review",
            {
                let q = v["questions"].as_u64().unwrap_or(0);
                format!("{q} gap {}", noun(q, "question"))
            },
            noun_count(v["lessons"].as_u64().unwrap_or(0), "lesson")
        )),
        sentence("No teacher wrote any of them"),
    ])
}

pub fn check_capabilities(d: &mova_octopus::Config, c: &Config) -> Result<(), String> {
    let mut n = 0;
    for (name, cap) in d.capabilities.iter().filter(|(_, c)| c.handler == "turtle") {
        let op = cap.parameters["operation"].as_str().ok_or(format!("{name}: operation required"))?;
        match op {
            "check" if cap.parameters["course"].as_str().is_some() => {}
            "made" => {}
            "gap" if cap.parameters["layer"].as_str().is_some_and(|l| c.layers.contains_key(l)) => {}
            _ => return Err(format!("{name}: operation {op} needs its course or a configured layer")),
        }
        if !cap.slots.is_empty() {
            return Err(format!("{name}: turtle capabilities take no slots"));
        }
        // Only reading operations may answer a question (`IntentRule::observes`).
        if cap.intent.observes && matches!(op, "check" | "gap") {
            return Err(format!("{name}: operation {op} acts; it cannot be declared observing"));
        }
        n += 1;
    }
    if n == 0 {
        return Err("no turtle capability configured".into());
    }
    Ok(())
}

pub struct Hands<'a> {
    pub config: &'a Config,
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

impl CapabilityHandler for Hands<'_> {
    fn execute(&self, p: &Value, _: &Action, invocation_id: &str) -> Result<Value> {
        match p["operation"].as_str() {
            Some("check") => {
                let r = check::course(p["course"].as_str().context("course")?)?;
                Ok(json!({"operation":"check","answer":say_check(&r),"result":{"course":r.course,"lessons":r.lessons.len(),"questions":r.questions,"with_problems":r.with_problems,
                    "problems":r.lessons.iter().flat_map(|l| l.problems.clone()).take(32).collect::<Vec<_>>()}}))
            }
            Some("made") => {
                let v = make::made(&self.config.journal)?;
                Ok(json!({"operation":"made","answer":say_made(&v),"result":v}))
            }
            Some("gap") => {
                let key = p["layer"].as_str().context("layer")?;
                let l = self.config.layers.get(key).with_context(|| format!("layer {key} is not configured"))?;
                let out = self.config.drafts.join(format!("{}-{invocation_id}", l.course));
                let model = self.config.model.as_deref().context("the gap maker reads its slots with Mova's annotator: set `model` (an en.bin) in the Turtle's configuration")?;
                let v = make::make_gap_course(&l.path, &l.course, &l.source, &out, &self.config.journal, now(), model)?;
                Ok(json!({"operation":"gap","answer":say_gap(&v, &l.source),"result":v}))
            }
            other => bail!("unknown turtle operation {other:?}"),
        }
    }
}

/// Answer one prompt through the deputy with the Turtle's hands; the English answer is realized from the report.
pub fn answer(deputy: &Deputy, config: &Config, id: &str, prompt: &str) -> Result<Value> {
    let hands = Hands { config };
    let overrides: BTreeMap<String, &dyn CapabilityHandler> = BTreeMap::from([("turtle".to_owned(), &hands as &dyn CapabilityHandler)]);
    let r = deputy.run_with(prompt, id, true, &overrides)?;
    Ok(json!({"answer":nautilus::animal::report(&r),"plan":r.plan,"failure":r.failure,"results":r.results}))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_example_configurations_hold() {
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        let c = Config::load(&here.join("turtle.example.json")).unwrap();
        let d: mova_octopus::Config = serde_json::from_slice(&std::fs::read(here.join("deputy.example.json")).unwrap()).unwrap();
        check_capabilities(&d, &c).unwrap();
        let t: nautilus::teacher::Config = serde_json::from_slice(&std::fs::read(here.join("teachers.example.json")).unwrap()).unwrap();
        t.validate().unwrap();
        assert_eq!((t.profiles[0].provider.as_str(), t.profiles[0].model.as_str()), ("claude-cli", "opus"));
        // the teacher's own mova-terminal profile, and the profile named in the teacher configuration
        let tp = terminal::Profile::load(&here.join("terminal.example.json")).unwrap();
        assert_eq!(tp.animal, t.profiles[0].terminal);
        let w: Value = serde_json::from_slice(&std::fs::read(here.join("crab-workers.example.json")).unwrap()).unwrap();
        assert!(w["workers"].as_array().unwrap().iter().all(|w| w["autostart"] == false));
    }
    #[test]
    fn english() {
        assert_eq!(say_made(&json!({"makings":0,"lessons":0,"questions":0})), "I have not made any questions yet.");
        assert_eq!(say_made(&json!({"makings":1,"lessons":8,"questions":151})), "I have made 151 questions in 8 lessons, in 1 set of drafts.");
        let r = check::Report { course: "commands".into(), lessons: vec![check::LessonReport::default(); 30], questions: 1200, with_problems: 0 };
        assert_eq!(say_check(&r), "I checked the commands course: 30 lessons with 1200 questions. Every lesson keeps the school's rules and its right answers are marked at varied places.");
        let mut bad = r.clone();
        bad.with_problems = 1;
        bad.lessons[0].problems = vec!["✗ 3 (a.lesson): facts 5, exercises 11, tests 8; every question needs exactly three wrong answers".into()];
        assert!(say_check(&bad).contains("1 lesson has a problem. 3 (a.lesson): facts 5"), "{}", say_check(&bad));
        assert_eq!(say_gap(&json!({"questions":2,"lessons":1}), "the command course"), "I have made 2 gap questions from the command course in 1 lesson, as a draft for review. No teacher wrote any of them.");
    }
}
