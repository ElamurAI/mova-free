//! The command school: an anonymized corpus of commands sent to Crab and an exam that replays
//! them through Mova's understanding (plan only, nothing executed) against expected readings.
use crate::{Config, Deputy, Report};
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

/// Replace private or volatile tokens so a command can be kept and shared as an example:
/// absolute paths, e-mail addresses, URLs and long hexadecimal or digit runs (hashes, ids, pids).
/// Words, names of roles and ordinary numbers stay; the command keeps its English shape.
pub fn anonymize(text: &str) -> String {
    let mut out = Vec::new();
    for raw in text.split_inclusive(char::is_whitespace) {
        let word = raw.trim_end_matches(char::is_whitespace);
        let space = &raw[word.len()..];
        let core = word.trim_matches(|c: char| ",.;:!?()[]{}\"'«»".contains(c));
        let (lead, trail) = match word.find(core) {
            Some(i) if !core.is_empty() => (&word[..i], &word[i + core.len()..]),
            _ => ("", ""),
        };
        let hexy = core.len() >= 12
            && core.chars().all(|c| c.is_ascii_hexdigit() || c == '-' || c == '_')
            && core.chars().filter(|c| c.is_ascii_digit()).count() >= 4;
        let digits = core.len() >= 5 && core.chars().all(|c| c.is_ascii_digit());
        let replaced = if core.is_empty() {
            None
        } else if core.starts_with('/') || core.starts_with("~/") {
            Some("<path>")
        } else if core.contains("://") {
            Some("<url>")
        } else if core.contains('@') && core.contains('.') && !core.starts_with('@') {
            Some("<email>")
        } else if hexy || digits {
            Some("<id>")
        } else {
            None
        };
        match replaced {
            Some(r) => out.push(format!("{lead}{r}{trail}{space}")),
            None => out.push(raw.to_owned()),
        }
    }
    out.concat()
}

/// How Crab read a command: the configured capabilities it planned, the refusal stage, or a plan
/// that was understood but whose execution failed. An execution failure is not a misunderstanding:
/// it keeps the planned capabilities and is never examined as a refusal.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Reading {
    Plan { capabilities: Vec<String> },
    Refuse { stage: String },
    ExecutedFailed { capabilities: Vec<String> },
}
pub fn reading(r: &Report) -> Reading {
    let capabilities = || r.plan.iter().map(|a| a.capability.clone()).collect();
    match &r.failure {
        Some(f) if f.stage == "execution_failure" && !r.plan.is_empty() => {
            Reading::ExecutedFailed { capabilities: capabilities() }
        }
        Some(f) => Reading::Refuse { stage: f.stage.clone() },
        None => Reading::Plan { capabilities: capabilities() },
    }
}

/// One corpus line. `expected` is a reviewed reading (seeds, corrected entries); `observed` is
/// what Crab did when the command arrived. Only reviewed entries are examined.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entry {
    pub text: String,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub expected: Option<Reading>,
    #[serde(default)]
    pub observed: Option<Reading>,
    pub origin: String,
    /// For a generated exercise: the reviewed text it was derived from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_from: Option<String>,
}

/// Append an anonymized observed command to the corpus (append-only JSONL, 0600).
pub fn record(corpus: &Path, prompt: &str, priority: Option<&str>, r: &Report) -> Result<()> {
    let e = Entry {
        text: anonymize(prompt),
        priority: priority.map(str::to_owned),
        expected: None,
        observed: Some(reading(r)),
        origin: "crab-ask".into(),
        derived_from: None,
    };
    if let Some(dir) = corpus.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut f = OpenOptions::new().create(true).append(true).mode(0o600).open(corpus)?;
    let mut line = serde_json::to_vec(&e)?;
    line.push(b'\n');
    f.write_all(&line)?;
    Ok(())
}

pub fn load(corpus: &Path) -> Result<Vec<Entry>> {
    let f = fs::File::open(corpus).with_context(|| format!("open {}", corpus.display()))?;
    let mut v = Vec::new();
    for (i, line) in BufReader::new(f).lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        v.push(serde_json::from_str(&line).with_context(|| format!("corpus line {}", i + 1))?);
    }
    Ok(v)
}

/// Replay every reviewed entry through `review` (no execution) and compare with its expectation.
pub fn exam(deputy: &Deputy, entries: &[Entry]) -> Value {
    let (mut pass, mut fail) = (0usize, Vec::new());
    let mut by_kind: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    let mut unreviewed = 0usize;
    for e in entries {
        let Some(want) = &e.expected else {
            unreviewed += 1;
            continue;
        };
        let got = reading(&deputy.review(&e.text));
        let kind = match want {
            Reading::Plan { .. } => "plan",
            Reading::Refuse { .. } => "refuse",
            // A reviewed expectation is a reading, never an execution outcome.
            Reading::ExecutedFailed { .. } => "invalid_expectation",
        };
        let slot = by_kind.entry(kind).or_default();
        slot.1 += 1;
        if &got == want {
            pass += 1;
            slot.0 += 1;
        } else {
            fail.push(json!({"text":e.text,"expected":want,"got":got}));
        }
    }
    json!({"schema":1,"examined":pass+fail.len(),"passed":pass,"unreviewed":unreviewed,
        "by_kind":by_kind.iter().map(|(k,(p,n))| (k.to_string(), json!({"passed":p,"examined":n}))).collect::<BTreeMap<_,_>>(),
        "failures":fail,"executed":false})
}

/// Check that a corpus file is anonymized: no entry text changes under `anonymize`.
pub fn audit(entries: &[Entry]) -> Result<()> {
    for e in entries {
        ensure!(anonymize(&e.text) == e.text, "entry is not anonymized: {:?}", e.text);
    }
    Ok(())
}

/// Group a review queue by (prompt, failure stage), reading legacy one-file-per-build candidates
/// and deduplicated files alike. Read-only: nothing in the queue is moved or rewritten.
pub fn group_candidates(dir: &Path) -> Result<Value> {
    let mut groups: BTreeMap<String, Value> = BTreeMap::new();
    let mut names: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    names.sort();
    for p in names {
        let meta = fs::metadata(&p)?;
        ensure!(meta.len() <= 4_000_000, "oversized review candidate {}", p.display());
        let v: Value = serde_json::from_slice(&fs::read(&p)?).with_context(|| format!("{}", p.display()))?;
        let (Some(ps), Some(stage)) = (v["prompt_sha256"].as_str(), v["failure"]["stage"].as_str()) else { continue };
        let key = crate::dedupe_key(ps, stage);
        let g = groups.entry(key.clone()).or_insert_with(|| {
            json!({"dedupe_key":key,"prompt":v["prompt"],"stage":stage,"files":[],"builds":[]})
        });
        g["files"].as_array_mut().unwrap().push(json!(p.file_name().map(|n| n.to_string_lossy().into_owned())));
        let builds = match v["reproduced_by"].as_array() {
            Some(b) => b.clone(),
            None => vec![json!({"candidate_id":v["candidate_id"],"source_sha256":v["source_sha256"],"reason":v["failure"]["reason"],"seen":1})],
        };
        let list = g["builds"].as_array_mut().unwrap();
        for b in builds {
            if !list.iter().any(|x| x["candidate_id"] == b["candidate_id"]) {
                list.push(b);
            }
        }
    }
    Ok(json!({"schema":1,"groups":groups.len(),"items":groups.into_values().collect::<Vec<_>>()}))
}

/// Observed commands that no reviewed corpus answers yet, once per text, with every distinct
/// observed reading and how often the text arrived. This is the reviewer's work list.
pub fn review_queue(observed: &[Entry], reviewed: &[Entry]) -> Value {
    let done: std::collections::BTreeSet<&str> =
        reviewed.iter().filter(|e| e.expected.is_some()).map(|e| e.text.trim()).collect();
    let mut open: BTreeMap<&str, (usize, Vec<&Reading>)> = BTreeMap::new();
    for e in observed {
        let t = e.text.trim();
        if done.contains(t) || e.expected.is_some() {
            continue;
        }
        let slot = open.entry(t).or_default();
        slot.0 += 1;
        if let Some(r) = &e.observed {
            if !slot.1.contains(&r) {
                slot.1.push(r);
            }
        }
    }
    json!({"schema":1,"open":open.len(),"items":open.iter().map(|(t,(n,rs))| json!({"text":t,"arrivals":n,"observed":rs})).collect::<Vec<_>>()})
}

/// One reviewer decision about an observed command (JSONL, written by the reviewer).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Decision {
    pub text: String,
    pub expected: Reading,
    /// "passed" admits the text to the reviewed corpus; anything else keeps it out.
    pub privacy_review: String,
    #[serde(default)]
    pub note: Option<String>,
}

/// Admit reviewed observed commands into the reviewed corpus (append-only). A decision is taken
/// only when its text was really observed, it passed privacy review, it is anonymized, its
/// expectation is a reading (a plan or a refusal) and the text is not reviewed yet. Admission is
/// for the exam and the school; it never authorizes training (`training_authorized` stays false).
pub fn promote(observed: &[Entry], reviewed: &[Entry], decisions: &[Decision]) -> (Vec<Entry>, Vec<Value>) {
    let seen: std::collections::BTreeSet<&str> = observed.iter().map(|e| e.text.trim()).collect();
    let mut done: std::collections::BTreeSet<String> =
        reviewed.iter().filter(|e| e.expected.is_some()).map(|e| e.text.trim().to_owned()).collect();
    let (mut admitted, mut refused) = (Vec::new(), Vec::new());
    for d in decisions {
        let t = d.text.trim();
        let why = if d.privacy_review != "passed" {
            Some("privacy review did not pass")
        } else if !seen.contains(t) {
            Some("text was not observed")
        } else if anonymize(t) != t {
            Some("text is not anonymized")
        } else if matches!(d.expected, Reading::ExecutedFailed { .. }) {
            Some("an expectation is a plan or a refusal, not an execution outcome")
        } else if done.contains(t) {
            Some("already reviewed")
        } else {
            None
        };
        match why {
            Some(w) => refused.push(json!({"text":t,"reason":w})),
            None => {
                done.insert(t.to_owned());
                admitted.push(Entry {
                    text: t.to_owned(),
                    priority: None,
                    expected: Some(d.expected.clone()),
                    observed: None,
                    origin: "reviewed-observed".into(),
                    derived_from: None,
                });
            }
        }
    }
    (admitted, refused)
}

fn capitalized(w: &str) -> bool {
    w.chars().next().is_some_and(char::is_uppercase)
}
fn core(w: &str) -> &str {
    w.trim_matches(|c: char| !c.is_alphanumeric())
}
fn title(s: &str) -> String {
    let mut c = s.chars();
    c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
}
/// Toggle the determiner before the capitalized name that ends in word `i`: "Is Eel running?" ↔
/// "Is the Eel running?", "Start the Sea Turtle." ↔ "Start Sea Turtle.".
fn toggle_the(words: &[&str], i: usize) -> Option<String> {
    let mut start = i;
    while start > 1 && capitalized(words[start - 1]) && core(words[start - 1]) == words[start - 1] {
        start -= 1;
    }
    let mut out: Vec<String> = words.iter().map(|s| s.to_string()).collect();
    if start > 0 && core(words[start - 1]).eq_ignore_ascii_case("the") {
        if start - 1 == 0 {
            return None; // a sentence-initial "The" is left alone
        }
        out.remove(start - 1);
    } else if start == 0 {
        return None;
    } else {
        out.insert(start, "the".into());
    }
    Some(out.join(" "))
}

/// Command-school exercises derived from reviewed plans, with their expected readings:
/// determiner variants of every name ("Is Eel running?" → "Is the Eel running?") and the same
/// sentence about every other entity whose capability has the same handler and parameters except
/// the named target ("Is Eel running?" → "Is the Sea Turtle running?" → turtle-status). Each
/// exercise is labeled with the reviewed text it comes from; the exam replays it like any other.
pub fn exercises(config: &Config, reviewed: &[Entry]) -> Vec<Entry> {
    let mut out: BTreeMap<String, Entry> = BTreeMap::new();
    let have: std::collections::BTreeSet<&str> = reviewed.iter().map(|e| e.text.trim()).collect();
    let shape = |c: &crate::Capability| {
        let mut p = c.parameters.clone();
        for k in ["worker", "dolphin"] {
            if let Some(o) = p.as_object_mut() {
                o.remove(k);
            }
        }
        (c.handler.clone(), p.to_string(), c.intent.observations.clone())
    };
    let mut push = |text: String, cap: &str, from: &str| {
        if !have.contains(text.as_str()) && !out.contains_key(&text) {
            out.insert(
                text.clone(),
                Entry {
                    text,
                    priority: None,
                    expected: Some(Reading::Plan { capabilities: vec![cap.to_owned()] }),
                    observed: None,
                    origin: "school-generated".into(),
                    derived_from: Some(from.to_owned()),
                },
            );
        }
    };
    for e in reviewed {
        let Some(Reading::Plan { capabilities }) = &e.expected else { continue };
        let [cap] = capabilities.as_slice() else { continue };
        let Some(c) = config.capabilities.get(cap) else { continue };
        let text = e.text.trim();
        let words: Vec<&str> = text.split(' ').collect();
        // The one name of this capability's entity said in the text, written as a name.
        let hits: Vec<usize> = (0..words.len())
            .filter(|&i| capitalized(words[i]) && c.intent.entities.iter().any(|a| core(words[i]).eq_ignore_ascii_case(a)))
            .collect();
        let [i] = hits.as_slice() else { continue };
        if let Some(t) = toggle_the(&words, *i) {
            push(t, cap, text);
        }
        // A name with a head noun ("the Sol Dolphin") is replaced only by a member of the same
        // configured group, so the head noun stays true ("the Opus Dolphin", never "the Eel Dolphin").
        let headed = words.get(i + 1).is_some_and(|n| capitalized(n));
        let same_group = |oc: &crate::Capability| {
            config.group_aliases.values().any(|m| {
                c.intent.entities.iter().any(|a| m.contains(a)) && oc.intent.entities.iter().any(|a| m.contains(a))
            })
        };
        for (other, oc) in &config.capabilities {
            if other == cap || shape(oc) != shape(c) || oc.intent.collective || (headed && !same_group(oc)) {
                continue;
            }
            let alias = &oc.intent.entities[0];
            let mut w2: Vec<String> = words.iter().map(|s| s.to_string()).collect();
            let old = core(words[*i]);
            w2[*i] = words[*i].replacen(old, &title(alias), 1);
            let t = w2.join(" ");
            let ws: Vec<&str> = t.split(' ').collect();
            push(t.clone(), other, text);
            if let Some(t2) = toggle_the(&ws, *i) {
                push(t2, other, text);
            }
        }
    }
    out.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn anonymize_keeps_english_and_hides_private_tokens() {
        let t = "Please restart Eel (pid 247740) and read /srv/example/runs/x/log.txt, \
                 then mail ops@example.org about f61e5c2b3f318f6f110c35357201 at https://h.example/a; keep 3 workers.";
        let a = anonymize(t);
        assert_eq!(
            a,
            "Please restart Eel (pid <id>) and read <path>, \
                 then mail <email> about <id> at <url>; keep 3 workers."
        );
        assert_eq!(anonymize(&a), a);
        assert_eq!(anonymize("Is Crab running normally?"), "Is Crab running normally?");
        assert_eq!(anonymize("Start Eel at 2026 please."), "Start Eel at 2026 please.");
    }
    fn entry(t: &str, expected: Option<Reading>, observed: Option<Reading>, origin: &str) -> Entry {
        Entry { text: t.into(), priority: None, expected, observed, origin: origin.into(), derived_from: None }
    }
    fn plan(c: &str) -> Reading {
        Reading::Plan { capabilities: vec![c.into()] }
    }
    #[test]
    fn determiner_toggles_before_the_whole_name() {
        let w: Vec<&str> = "Is Eel running?".split(' ').collect();
        assert_eq!(toggle_the(&w, 1).as_deref(), Some("Is the Eel running?"));
        let w: Vec<&str> = "Start the Sea Turtle.".split(' ').collect();
        assert_eq!(toggle_the(&w, 3).as_deref(), Some("Start Sea Turtle."));
        let w: Vec<&str> = "Is the Sol Dolphin running?".split(' ').collect();
        assert_eq!(toggle_the(&w, 2).as_deref(), Some("Is Sol Dolphin running?"));
        let w: Vec<&str> = "Eel is running.".split(' ').collect();
        assert_eq!(toggle_the(&w, 0), None);
    }
    #[test]
    fn promote_keeps_the_privacy_gate_and_needs_an_observation() {
        let observed = vec![
            entry("Is the documenter running?", None, Some(Reading::Refuse { stage: "unsupported_capability".into() }), "crab-ask"),
            entry("Start the Nautilus.", None, Some(Reading::ExecutedFailed { capabilities: vec!["nautilus-start".into()] }), "crab-ask"),
        ];
        let reviewed = vec![entry("Start Eel.", Some(plan("eel-start")), None, "seed")];
        let d = |t: &str, e: Reading, p: &str| Decision { text: t.into(), expected: e, privacy_review: p.into(), note: None };
        let (ok, no) = promote(
            &observed,
            &reviewed,
            &[
                d("Is the documenter running?", plan("dolphin-docs-status"), "passed"),
                d("Is the documenter running?", plan("dolphin-docs-status"), "passed"),
                d("Start the Nautilus.", plan("nautilus-start"), "required"),
                d("Start Eel.", plan("eel-start"), "passed"),
                d("Never said.", plan("eel-start"), "passed"),
                d("Start the Nautilus.", Reading::ExecutedFailed { capabilities: vec![] }, "passed"),
            ],
        );
        assert_eq!(ok.len(), 1);
        assert_eq!(ok[0].origin, "reviewed-observed");
        let reasons: Vec<&str> = no.iter().map(|v| v["reason"].as_str().unwrap()).collect();
        assert_eq!(
            reasons,
            ["already reviewed", "privacy review did not pass", "text was not observed", "text was not observed",
             "an expectation is a plan or a refusal, not an execution outcome"]
        );
        let q = review_queue(&observed, &[reviewed, ok].concat());
        assert_eq!(q["open"], 1);
        assert_eq!(q["items"][0]["observed"][0]["kind"], "executed_failed");
    }
    #[test]
    fn exercises_vary_determiners_and_same_shaped_entities() {
        let cap = |worker: &str, entity: &str, op: &str| {
            json!({"handler":"crab_control","parameters":{"operation":op,"worker":worker},
                "intent":{"request_predicates":["check"],"observations":["status","run"],"entities":[entity],"observes":op == "worker_status"}})
        };
        let config: Config = serde_json::from_value(json!({"model":"/m","model_sha256":"0","ledger":"/l","followup_queue":"/q",
            "capabilities":{"eel-status":cap("eel","eel","worker_status"),"turtle-status":cap("turtle","turtle","worker_status"),
                "eel-stop":cap("eel","eel","worker_stop")}})).unwrap();
        let reviewed = vec![entry("Is Eel running?", Some(plan("eel-status")), None, "seed")];
        let got: Vec<(String, Reading)> =
            exercises(&config, &reviewed).into_iter().map(|e| (e.text, e.expected.unwrap())).collect();
        assert_eq!(
            got,
            [
                ("Is Turtle running?".to_string(), plan("turtle-status")),
                ("Is the Eel running?".to_string(), plan("eel-status")),
                ("Is the Turtle running?".to_string(), plan("turtle-status")),
            ]
        );
        // Negative control: a refusal or a multi-capability plan yields no exercise.
        let none = vec![entry("Is Eel running?", Some(Reading::Refuse { stage: "x".into() }), None, "seed")];
        assert!(exercises(&config, &none).is_empty());
    }
    #[test]
    fn audit_refuses_raw_paths() {
        let e = |t: &str| Entry { text: t.into(), priority: None, expected: None, observed: None, origin: "seed".into(), derived_from: None };
        assert!(audit(&[e("Start Eel.")]).is_ok());
        assert!(audit(&[e("Read /home/someone/secret.")]).is_err());
    }
}
