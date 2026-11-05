//! The Whales' English: short sentences from structured results, realized deterministically with
//! Mova's realizer (`mova_octopus::realize`: number agreement, plurals from Mova's morphology,
//! lists, «I could not … because …»). Every function states only what its input says; the
//! structured JSON travels alongside the sentence. No language model is involved.
use crate::animal::Outcome;
use mova_octopus::realize::{be, could_not, list, noun, noun_count, paragraph, sentence, thousands, utc};
use serde_json::Value;

/// A source as the answer names it: its id, with its title when the catalog has one.
fn named(id: &str, title: Option<&str>) -> String {
    match title.filter(|t| !t.trim().is_empty()) {
        Some(t) => format!("{id} (“{}”)", t.trim()),
        None => id.to_string(),
    }
}
fn named_v(v: &Value) -> String {
    named(v["id"].as_str().or_else(|| v["source"].as_str()).unwrap_or("?"), v["title"].as_str())
}
fn words(v: &Value) -> String {
    v.as_u64().map(|n| format!("{} words", thousands(n))).unwrap_or_else(|| "an unmeasured length".into())
}

/// What a finished attempt means.
fn attempt(noun_: &str, f: &Value) -> String {
    let who = named_v(f);
    let when = utc(f["finished_unix"].as_u64().unwrap_or(0));
    let reason = f["reason"].as_str().unwrap_or("no reason was recorded").trim_end_matches('.').to_string();
    match f["state"].as_str() {
        Some("completed") => sentence(&format!("I described {who} at {when}: {}", words(&f["words"]))),
        Some("quarantined") => could_not("describe", &who, &format!("the teacher's reply failed my checks ({reason}); I keep it for review and do not ask again")),
        Some("local_only") => sentence(&format!("I did not send {who} to the teacher: its licence keeps the {noun_} local")),
        Some("preparation_failed") | Some("launch_failed") => {
            could_not("describe", &who, &format!("I could not prepare the work ({reason}); no teacher was asked"))
        }
        _ => sentence(&format!("My attempt to describe {who} is uncertain ({reason}); I will not repeat it on my own")),
    }
}

/// The answer to a describe request: what was queued and what was not, and why.
pub fn describe(noun_: &str, o: &[Outcome]) -> String {
    let mut out = vec![];
    let queued: Vec<&Outcome> = o.iter().filter(|x| x.state == "queued").collect();
    if !queued.is_empty() {
        let names: Vec<String> = queued.iter().map(|x| named(x.id.as_deref().unwrap_or(&x.source), x.title.as_deref())).collect();
        let jobs: Vec<&str> = queued.iter().filter_map(|x| x.job.as_deref()).collect();
        out.push(sentence(&format!("I have queued {} for description ({}): {}", noun_count(queued.len() as u64, noun_), list(&jobs), list(&names))));
        let d = queued[0].detail.clone().unwrap_or_default();
        let ahead = d["ahead"].as_u64().unwrap_or(0);
        let running = d["running"].as_u64().unwrap_or(0);
        out.push(sentence(&if d["paused"] == true {
            "The Whales are paused, so nothing new starts until they are resumed".to_string()
        } else if ahead > 0 {
            format!("{} {} ahead of it", noun_count(ahead, "request"), be(ahead))
        } else if running > 0 {
            "It starts when a running description is done".to_string()
        } else {
            "I am starting on it now".to_string()
        }));
        if queued.len() > 1 {
            out.push(sentence("The others wait behind it in the order you gave"));
        }
    }
    for x in o.iter().filter(|x| x.state != "queued") {
        let who = named(x.id.as_deref().unwrap_or(&x.source), x.title.as_deref());
        let job = x.job.as_deref().unwrap_or("the queue");
        let d = x.detail.clone().unwrap_or_default();
        out.push(match x.state.as_str() {
            "running" => sentence(&format!("I am describing {who} right now ({job})")),
            "waiting" => sentence(&format!("{who} is already waiting ({job})")),
            "described" => sentence(&format!("I have already described {who} ({}); ask me to show it", words(&d["words"]))),
            "failed_before" => sentence(&format!("I tried {who} before and will not try again on my own. {}", attempt(noun_, &d))),
            "unknown" => could_not("describe", &x.source, &format!("it is not a {noun_} in my catalog")),
            "unreadable" => could_not("describe", &who, &format!("{}", d.as_str().unwrap_or("its source cannot be read").trim_end_matches('.'))),
            "full" => could_not("queue", &who, &format!("{} requests are already waiting", d.as_u64().unwrap_or(0))),
            _ => could_not("queue", &who, "I am stopping"),
        });
    }
    paragraph(&out)
}

/// "How many … have you described?"
pub fn count(v: &Value) -> String {
    let n = v["noun"].as_str().unwrap_or("source");
    let described = v["described"].as_u64().unwrap_or(0);
    let mut out = vec![sentence(&format!("I have described {}", noun_count(described, n)))];
    let (running, waiting) = (v["running"].as_u64().unwrap_or(0), v["waiting"].as_u64().unwrap_or(0));
    if running + waiting > 0 {
        out.push(sentence(&format!("{} {} being described now and {waiting} waiting", noun_count(running, n), be(running))));
    }
    let t = &v["totals"];
    let failed: u64 = ["quarantined", "uncertain", "preparation_failed", "launch_failed"].iter().filter_map(|k| t[k].as_u64()).sum();
    let local = t["local_only"].as_u64().unwrap_or(0);
    if failed > 0 {
        out.push(sentence(&format!("{} of my attempts here did not produce a description and wait for review", failed)));
    }
    if local > 0 {
        out.push(sentence(&format!("I kept {} local because of {}", noun_count(local, n), if local == 1 { "its licence" } else { "their licences" })));
    }
    if v["paused"] == true {
        out.push(sentence("The Whales are paused"));
    }
    paragraph(&out)
}

/// The status of the whole service.
pub fn status(v: &Value) -> String {
    let n = v["noun"].as_str().unwrap_or("source");
    let empty = vec![];
    let running = v["running"].as_array().unwrap_or(&empty);
    let waiting = v["waiting"].as_array().unwrap_or(&empty);
    let mut out = vec![];
    if running.is_empty() {
        out.push(sentence(&format!("I am not describing any {n} now")));
    } else {
        let names: Vec<String> = running
            .iter()
            .map(|r| format!("{} (since {})", named_v(r), utc(r["started_unix"].as_u64().unwrap_or(0))))
            .collect();
        out.push(sentence(&format!("I am describing {}", list(&names))));
    }
    let w = waiting.len() as u64;
    out.push(sentence(&if w == 0 { "no request is waiting".to_string() } else { format!("{} {} waiting", noun_count(w, "request"), be(w)) }));
    if v["paused"] == true {
        out.push(sentence("The Whales are paused: no new description starts until they are resumed"));
    }
    if let Some(f) = v.get("last").filter(|f| !f.is_null()) {
        out.push(format!("Last: {}", attempt(n, f)));
    }
    paragraph(&out)
}

/// The status of one source.
pub fn source_status(n: &str, v: &Value) -> String {
    let who = named_v(v);
    match v["state"].as_str() {
        Some("unknown") => sentence(&format!("{} is not a {n} in my catalog", v["source"].as_str().unwrap_or("?"))),
        Some("running") => sentence(&format!("I am describing {who} now (since {})", utc(v["started_unix"].as_u64().unwrap_or(0)))),
        Some("waiting") => {
            let ahead = v["ahead"].as_u64().unwrap_or(0);
            let mut s = if ahead == 0 {
                format!("{who} is waiting; it is next")
            } else {
                format!("{who} is waiting; {} {} ahead of it", noun_count(ahead, "request"), be(ahead))
            };
            if v["paused"] == true {
                s.push_str(", and the Whales are paused");
            }
            sentence(&s)
        }
        Some("finished") => attempt(n, &v["result"]),
        Some("described") => paragraph(&[
            sentence(&format!("I described {who} at {}: {}", utc(v["written_unix"].as_u64().unwrap_or(0)), words(&v["words"]))),
            sentence(&format!("The bundle is in {}", v["manifest"].as_str().and_then(|m| m.strip_suffix("/manifest.json")).unwrap_or("the store"))),
        ]),
        _ => sentence(&format!("I have not described {who} yet")),
    }
}

/// A described source: length, opening and where the bundle is.
pub fn show(_n: &str, v: &Value) -> String {
    let who = named_v(v);
    let mut out = vec![sentence(&format!(
        "My description of {who} has {}; I wrote it at {}",
        words(&v["words"]),
        utc(v["written_unix"].as_u64().unwrap_or(0))
    ))];
    if let Some(o) = v["opening"].as_str().filter(|o| !o.is_empty()) {
        out.push(format!("It begins: “{o}”"));
    }
    if let Some(a) = v["artifact"].as_str() {
        out.push(sentence(&format!("The full text is {a}")));
    }
    out.push(sentence(&format!("The bundle is in {}", v["bundle"].as_str().unwrap_or("the store"))));
    paragraph(&out)
}

/// Pause and resume.
pub fn marker(v: &Value) -> String {
    let running = v["running"].as_u64().unwrap_or(0);
    let waiting = v["waiting"].as_u64().unwrap_or(0);
    let n = v["noun"].as_str().unwrap_or("source");
    match (v["paused"] == true, v["was_paused"] == true) {
        (true, true) => sentence("The Whales were already paused"),
        (true, false) => paragraph(&[
            sentence("I have paused the Whales: no new description starts"),
            if running > 0 {
                sentence(&format!("{} {} already being described and may finish", noun_count(running, n), be(running)))
            } else {
                String::new()
            },
        ]),
        (false, false) => sentence("The Whales were not paused"),
        (false, true) => sentence(&format!(
            "I have resumed the Whales; {}",
            if waiting == 0 { "no request is waiting".to_string() } else { format!("{waiting} waiting {} can start", noun(waiting, "request")) }
        )),
    }
}

/// The first sentences of a description, without Markdown headings, at most `max` characters,
/// cut at a sentence end when one exists.
pub fn opening(text: &str, max: usize) -> String {
    let body: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).collect();
    let flat = body.join(" ");
    let cut: String = flat.chars().take(max).collect();
    if cut.len() == flat.len() {
        return cut;
    }
    match cut.rfind(['.', '!', '?']).filter(|&i| i > max / 4) {
        Some(i) => cut[..=i].to_string(),
        None => format!("{}…", cut.trim_end()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    fn o(state: &str, id: &str, detail: Value) -> Outcome {
        Outcome { source: id.into(), id: Some(id.into()), title: None, state: state.into(), job: Some("octopus-r1-0".into()), detail: Some(detail) }
    }
    #[test]
    fn describe_answers() {
        assert_eq!(
            describe("paper", &[o("queued", "smith-2024-tree", json!({"ahead":0,"running":0,"paused":false}))]),
            "I have queued 1 paper for description (octopus-r1-0): smith-2024-tree. I am starting on it now."
        );
        assert_eq!(
            describe("paper", &[o("queued", "a-2001-x", json!({"ahead":2,"running":1,"paused":false}))]),
            "I have queued 1 paper for description (octopus-r1-0): a-2001-x. 2 requests are ahead of it."
        );
        assert!(describe("paper", &[o("queued", "a-2001-x", json!({"ahead":0,"running":1}))]).ends_with("It starts when a running description is done."));
        assert!(describe("paper", &[o("queued", "a-2001-x", json!({"ahead":0,"running":0,"paused":true}))]).contains("paused"));
        let mut u = o("unknown", "b-2002-y", json!(null));
        u.id = None;
        assert_eq!(describe("paper", &[u]), "I could not describe b-2002-y because it is not a paper in my catalog.");
        assert_eq!(
            describe("paper", &[o("described", "c-2003-z", json!({"words":4512}))]),
            "I have already described c-2003-z (4,512 words); ask me to show it."
        );
        let f = json!({"id":"d-2004-w","state":"quarantined","reason":"length gate: /description must contain 4000..5000 words","finished_unix":0});
        assert!(describe("paper", &[o("failed_before", "d-2004-w", f)]).contains("failed my checks (length gate: /description must contain 4000..5000 words)"));
    }
    #[test]
    fn count_status_marker_and_opening() {
        assert_eq!(count(&json!({"described":3,"running":1,"waiting":2,"totals":{},"noun":"paper"})), "I have described 3 papers. 1 paper is being described now and 2 waiting.");
        assert_eq!(count(&json!({"described":1,"running":0,"waiting":0,"totals":{"quarantined":1,"local_only":2},"noun":"paper","paused":true})),
            "I have described 1 paper. 1 of my attempts here did not produce a description and wait for review. I kept 2 papers local because of their licences. The Whales are paused.");
        assert_eq!(status(&json!({"running":[],"waiting":[],"noun":"paper"})), "I am not describing any paper now. No request is waiting.");
        assert_eq!(
            status(&json!({"running":[{"id":"a-2001-x","title":"Trees","started_unix":60}],"waiting":[{"id":"b"}],"paused":true,"noun":"paper"})),
            "I am describing a-2001-x (“Trees”) (since 1970-01-01 00:01 UTC). 1 request is waiting. The Whales are paused: no new description starts until they are resumed."
        );
        assert_eq!(source_status("paper", &json!({"source":"x-1-y","state":"unknown"})), "X-1-y is not a paper in my catalog.");
        assert_eq!(source_status("paper", &json!({"id":"x-1-y","state":"waiting","ahead":2})), "X-1-y is waiting; 2 requests are ahead of it.");
        assert_eq!(source_status("paper", &json!({"id":"x-1-y","state":"not_described"})), "I have not described x-1-y yet.");
        assert_eq!(marker(&json!({"paused":true,"was_paused":false,"running":1,"waiting":0,"noun":"paper"})), "I have paused the Whales: no new description starts. 1 paper is already being described and may finish.");
        assert_eq!(marker(&json!({"paused":false,"was_paused":true,"running":0,"waiting":2})), "I have resumed the Whales; 2 waiting requests can start.");
        assert_eq!(marker(&json!({"paused":true,"was_paused":true})), "The Whales were already paused.");
        assert_eq!(opening("# Title\n\nFirst sentence here. Second one is longer than the limit allows.", 40), "First sentence here.");
        assert_eq!(opening("Short.", 40), "Short.");
    }
}
