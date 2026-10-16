//! Triggers: English words hooked to compiled code, explained by the worlds.
//!
//! A word such as «minute» is not only text to the small ones: it fires a trigger — a function of the skeleton that
//! knows a minute is sixty seconds and can count with it. The world of the trigger's topic explains it: the sentences
//! of the world that use the trigger's words are the trigger's explanation, with their sources. So Mova can both
//! compute and say why. The concept lives in the code; the world is its explanation.
//!
//!   world trigger "<English sentence>"   — which triggers fire, what they compute, what explains them

use std::collections::BTreeMap;

use anyhow::Result;

/// A trigger: its name, the world that explains it, the words it hooks, what it does.
pub struct Trigger {
    pub name: &'static str,
    pub world: &'static str,
    pub words: &'static [&'static str],
    pub what: &'static str,
}

/// Units of time: singular, plural, seconds.
pub const TIME_UNITS: [(&str, &str, f64); 14] = [
    ("nanosecond", "nanoseconds", 1e-9),
    ("microsecond", "microseconds", 1e-6),
    ("millisecond", "milliseconds", 1e-3),
    ("second", "seconds", 1.0),
    ("minute", "minutes", 60.0),
    ("hour", "hours", 3600.0),
    ("day", "days", 86400.0),
    ("week", "weeks", 604_800.0),
    ("fortnight", "fortnights", 1_209_600.0),
    ("month", "months", 2_629_746.0),
    ("year", "years", 31_556_952.0),
    ("decade", "decades", 315_569_520.0),
    ("century", "centuries", 3_155_695_200.0),
    ("millennium", "millennia", 31_556_952_000.0),
];

/// Units of frequency: singular, plural, cycles a second.
pub const FREQ_UNITS: [(&str, &str, f64); 4] = [("hertz", "hertz", 1.0), ("kilohertz", "kilohertz", 1e3), ("megahertz", "megahertz", 1e6), ("gigahertz", "gigahertz", 1e9)];

pub const TRIGGERS: [Trigger; 5] = [
    Trigger { name: "duration", world: "time", words: &["second", "minute", "hour", "day", "week", "month", "year", "millisecond", "nanosecond", "century"], what: "a number and a unit of time become a length of time in seconds; lengths convert between units" },
    Trigger { name: "cycles", world: "time", words: &["hertz", "gigahertz", "megahertz", "cycle", "clock", "frequency"], what: "a frequency and a length of time give a number of clock cycles" },
    Trigger { name: "clock", world: "time", words: &["now", "time", "today", "date", "clock"], what: "the skeleton's clock: the time and the date now, in the operator's time zone" },
    Trigger { name: "self", world: "time", words: &["message", "task", "work", "training", "learning", "research", "science", "world"], what: "the words call up what the small ones lived through: when it last happened and how long it took" },
    Trigger { name: "calendar", world: "time", words: &["weekday", "calendar", "leap", "january", "monday"], what: "the civil calendar: the day of the week of a date, leap years" },
];

// ---- The self: events of the skeleton and the words they activate, both ways ------------------------------------
//
// A message in the chat activates the word «message»; the word «message» calls up the real messages. Doing a task
// activates «task» and «work»; training a state, «training», «learning» and «work»; searching for ideas and checking
// them, «research», «science», «hypothesis» and «experiment». Every activation is written with its time and how long
// it took (`<hot>/activations.tsv`), so the words of the world of time — «a minute», «long», «just now» — are tied to
// what the small ones lived through.

/// Events of the skeleton and the words each activates.
pub const EVENTS: [(&str, &[&str]); 5] = [
    ("message", &["message", "chat", "conversation", "words", "said"]),
    ("task", &["task", "work", "command", "job", "done"]),
    ("training", &["training", "learning", "work", "round", "state", "lesson"]),
    ("research", &["research", "science", "hypothesis", "experiment", "search", "idea", "test", "evidence"]),
    ("gathering", &["world", "knowledge", "reading", "sources", "principle", "book"]),
];

fn activations_path() -> std::path::PathBuf {
    crate::state::root().join("activations.tsv")
}

/// An event happened: write it with the words it activates (time, event, words, seconds it took, detail).
pub fn activate(event: &str, secs: f64, detail: &str) {
    let Some((_, ws)) = EVENTS.iter().find(|e| e.0 == event) else { return };
    let line = format!("{}\t{event}\t{}\t{secs:.3}\t{}\n", crate::talk::now(), ws.join(","), detail.replace(['\t', '\n'], " ").chars().take(200).collect::<String>());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(activations_path()) {
        let _ = std::io::Write::write_all(&mut f, line.as_bytes());
    }
}

/// A task in progress: when it ends (the value is dropped), the event «task» is written with how long it took.
pub struct Task {
    t0: std::time::Instant,
    detail: String,
}

impl Task {
    pub fn new(detail: &str) -> Task {
        Task { t0: std::time::Instant::now(), detail: detail.to_string() }
    }
}

impl Drop for Task {
    fn drop(&mut self) {
        activate("task", self.t0.elapsed().as_secs_f64(), &self.detail);
    }
}

/// The last activations of an event: (unix time, seconds it took, detail), newest first.
pub fn episodes(event: &str, k: usize) -> Vec<(u64, f64, String)> {
    let t = std::fs::read_to_string(activations_path()).unwrap_or_default();
    let mut v: Vec<(u64, f64, String)> = t.lines().filter_map(|l| {
        let c: Vec<&str> = l.splitn(5, '\t').collect();
        (c.len() == 5 && c[1] == event).then(|| (c[0].parse().unwrap_or(0), c[3].parse().unwrap_or(0.0), c[4].to_string()))
    }).collect();
    v.reverse();
    v.truncate(k);
    v
}

/// The words of a text that call up events the small ones lived through: (event, what it says about the last one),
/// given «now» and the last lived event of each kind (event, time, seconds, detail).
pub fn recall_events_at(text: &str, now: u64, lived: &[(String, u64, f64, String)]) -> Vec<(String, String)> {
    let low: Vec<String> = text.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(String::from).collect();
    let mut out = Vec::new();
    for (ev, ws) in EVENTS {
        if !ws.iter().any(|w| low.iter().any(|x| x == w || x.strip_suffix('s') == Some(w))) {
            continue;
        }
        let Some((_, t, secs, detail)) = lived.iter().find(|x| x.0 == ev) else { continue };
        let last = (*t, *secs, detail.clone());
        let ago = show_duration(now.saturating_sub(last.0) as f64);
        let took = if last.1 > 0.0 { format!(", it took {}", show_duration(last.1)) } else { String::new() };
        out.push((ev.to_string(), format!("the last {ev} was {ago} ago{took}: {}", last.2)));
    }
    out
}

// ---- Expecting: what usually comes next, and surprise ------------------------------------------------------------
//
// A mind does not wait for an event to happen in order to think about it: it turns over in advance what may come, the
// bad and the good, and has a reaction ready. The small ones learn from what they lived (`activations.tsv`) which
// event usually follows which and after how long; before each round they say what they expect, with the reaction
// prepared for each outcome; an event they gave almost no chance surprises them — and a surprise is where the new
// comes from.

/// What usually follows an event: (next event, how often in thousandths, the median wait in seconds), most likely first.
pub fn expect_after(event: &str, lived: &[(u64, String)]) -> Vec<(String, u32, u64)> {
    let mut after: BTreeMap<String, Vec<u64>> = BTreeMap::new();
    let mut total = 0u32;
    for w in lived.windows(2) {
        if w[0].1 == event && w[1].1 != "message" && w[1].1 != "task" {
            after.entry(w[1].1.clone()).or_default().push(w[1].0.saturating_sub(w[0].0));
            total += 1;
        }
    }
    let mut v: Vec<(String, u32, u64)> = after
        .into_iter()
        .map(|(e, mut gaps)| {
            gaps.sort_unstable();
            let n = gaps.len() as u32;
            (e, n * 1000 / total.max(1), gaps[gaps.len() / 2])
        })
        .collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    v
}

/// The lived events in order (time, event), without the chat's chatter.
pub fn lived_events() -> Vec<(u64, String)> {
    std::fs::read_to_string(activations_path())
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let c: Vec<&str> = l.splitn(5, '\t').collect();
            (c.len() == 5 && !matches!(c[1], "message" | "task")).then(|| (c[0].parse().unwrap_or(0), c[1].to_string()))
        })
        .collect()
}

/// How rare an outcome has been: (times it happened, times it could have) for an outcome counted in the details of an
/// event («a rule of knowledge held» among knowledge rounds).
pub fn outcome_rate(event: &str, word: &str) -> (usize, usize) {
    let t = std::fs::read_to_string(activations_path()).unwrap_or_default();
    let rows: Vec<&str> = t.lines().filter(|l| l.split('\t').nth(1) == Some(event)).collect();
    (rows.iter().filter(|l| l.contains(word)).count(), rows.len())
}

/// The expectation said before a round, with a reaction prepared for the bad and the good outcome.
pub fn expectation_line() -> String {
    let lived = lived_events();
    let next = expect_after("training", &lived);
    let (held, tried) = outcome_rate("research", "a safe idea was found");
    let (learned, rounds) = outcome_rate("research", "a rule of knowledge held");
    let nx = next.first().map(|(e, p, g)| format!("after a round usually comes {e} ({}%, in about {})", p / 10, show_duration(*g as f64))).unwrap_or_else(|| "I have not lived enough rounds to expect".into());
    format!(
        "I expect: {nx}. A new safe idea for my parser came {held} of {tried} times, a rule of knowledge held {learned} of {rounds}. Ready if nothing comes: I try the next ideas in their order and keep the failed ones as dormant hypotheses. Ready if something holds: I write it into my brain and tell you at once."
    )
}

/// Is this outcome a surprise — something that happened at most once in twenty chances before?
pub fn surprising(event: &str, word: &str) -> bool {
    let (n, of) = outcome_rate(event, word);
    of >= 20 && n * 20 <= of
}

const NUMBER_WORDS: [(&str, f64); 36] = [
    ("zero", 0.0), ("seventeen", 17.0), ("nineteen", 19.0),
    ("seventy", 70.0), ("eighty", 80.0), ("ninety", 90.0), ("thirteen", 13.0), ("fourteen", 14.0), ("sixteen", 16.0), ("eighteen", 18.0), ("twenty-four", 24.0),
    ("a", 1.0), ("an", 1.0), ("one", 1.0), ("two", 2.0), ("three", 3.0), ("four", 4.0), ("five", 5.0), ("six", 6.0), ("seven", 7.0),
    ("eight", 8.0), ("nine", 9.0), ("ten", 10.0), ("eleven", 11.0), ("twelve", 12.0), ("fifteen", 15.0), ("twenty", 20.0),
    ("thirty", 30.0), ("forty", 40.0), ("fifty", 50.0), ("sixty", 60.0), ("hundred", 100.0), ("thousand", 1000.0), ("half", 0.5),
    ("million", 1e6), ("billion", 1e9),
];

/// A number written as a word («sixty»), if it is one.
pub fn number_word(w: &str) -> Option<f64> {
    let w = w.to_lowercase();
    // «twenty-four»: the parts added
    if let Some((a, b)) = w.split_once('-') {
        return Some(number_word(a)? + number_word(b)?);
    }
    NUMBER_WORDS.iter().find(|x| x.0 == w && !matches!(x.0, "a" | "an")).map(|x| x.1)
}

fn number(w: &str) -> Option<f64> {
    let w = w.to_lowercase();
    w.replace(',', "").parse::<f64>().ok().or_else(|| NUMBER_WORDS.iter().find(|x| x.0 == w).map(|x| x.1))
}

fn words(text: &str) -> Vec<String> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '.' || c == ',')).map(|w| w.trim_end_matches(['.', ',']).to_string()).filter(|w| !w.is_empty()).collect()
}

/// A quantity in the text: (the words, the number or none, the unit's index, the unit's size).
fn quantities(text: &str, units: &[(&str, &str, f64)]) -> Vec<(String, Option<f64>, usize, f64)> {
    let ws = words(text);
    let mut out = Vec::new();
    for (i, w) in ws.iter().enumerate() {
        let lw = w.to_lowercase();
        let Some(u) = units.iter().position(|u| u.0 == lw || u.1 == lw) else { continue };
        let n = i.checked_sub(1).and_then(|j| number(&ws[j]));
        let shown = if n.is_some() { format!("{} {w}", ws[i - 1]) } else { w.clone() };
        out.push((shown, n, u, units[u].2));
    }
    out
}

/// A length of time in words: the largest unit that gives at least one, and the seconds.
pub fn show_duration(secs: f64) -> String {
    let u = TIME_UNITS.iter().rev().find(|u| secs >= u.2).unwrap_or(&TIME_UNITS[0]);
    let n = secs / u.2;
    let num = if (n - n.round()).abs() < 1e-9 { format!("{}", n.round()) } else { format!("{n:.2}") };
    format!("{num} {}", if num == "1" { u.0 } else { u.1 })
}

/// What fires on a text, reading the outside world (the clock and the lived events) once and passing them in.
pub fn fire(text: &str) -> Vec<(&'static Trigger, String)> {
    let lived: Vec<(String, u64, f64, String)> = EVENTS.iter().flat_map(|(e, _)| episodes(e, 1).into_iter().map(move |x| (e.to_string(), x.0, x.1, x.2))).collect();
    fire_at(text, crate::talk::now(), crate::talk::tz_offset(), &lived)
}

/// The determinant: what fires on a text given «now», the time zone and the lived events — a pure function, the same
/// on every system for the same inputs.
pub fn fire_at(text: &str, now: u64, tz: i64, lived: &[(String, u64, f64, String)]) -> Vec<(&'static Trigger, String)> {
    let mut out = Vec::new();
    let low = text.to_lowercase();
    let durs = quantities(text, &TIME_UNITS);
    let freqs = quantities(text, &FREQ_UNITS);
    let with_n: Vec<&(String, Option<f64>, usize, f64)> = durs.iter().filter(|d| d.1.is_some()).collect();
    if !with_n.is_empty() {
        let mut said: Vec<String> = with_n.iter().map(|d| format!("{} = {} seconds", d.0, d.1.unwrap() * d.3)).collect();
        // a unit without a number is the unit asked for («how many seconds are in three hours»)
        if let Some(target) = durs.iter().find(|d| d.1.is_none()) {
            let total: f64 = with_n.iter().map(|d| d.1.unwrap() * d.3).sum();
            let n = total / target.3;
            said.push(format!("that is {} {}", if (n - n.round()).abs() < 1e-9 { format!("{}", n.round()) } else { format!("{n:.3}") }, TIME_UNITS[target.2].1));
        } else if with_n.len() > 1 {
            let total: f64 = with_n.iter().map(|d| d.1.unwrap() * d.3).sum();
            said.push(format!("together {}", show_duration(total)));
        }
        out.push((&TRIGGERS[0], said.join("; ")));
    }
    if let (Some(f), Some(d)) = (freqs.iter().find(|f| f.1.is_some()), with_n.first()) {
        let cycles = f.1.unwrap() * f.3 * d.1.unwrap() * d.3;
        out.push((&TRIGGERS[1], format!("{} for {} is {} clock cycles", f.0, d.0, if cycles >= 1.0 { format!("{}", cycles.round()) } else { format!("{cycles:.3}") })));
    } else if let Some(f) = freqs.iter().find(|f| f.1.is_some()) {
        let period = 1.0 / (f.1.unwrap() * f.3);
        out.push((&TRIGGERS[1], format!("{} is {} cycles a second; one cycle lasts {period:.3e} seconds", f.0, f.1.unwrap() * f.3)));
    }
    let asks = |w: &str| low.split(|c: char| !c.is_alphanumeric()).any(|x| x == w);
    if asks("now") || low.contains("what time") || asks("today") {
        let local = now as i64 + tz * 3600;
        let (y, m, d, h, mi, _) = crate::talk::civil(local);
        out.push((&TRIGGERS[2], format!("now is {y}-{m:02}-{d:02} {h:02}:{mi:02} (UTC{tz:+})")));
    }
    if let Some(y) = words(text).iter().filter_map(|w| w.parse::<i64>().ok()).find(|y| (1583..=9999).contains(y)).filter(|_| asks("leap") || asks("year")) {
        let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
        out.push((&TRIGGERS[4], format!("{y} is {}a leap year ({} days)", if leap { "" } else { "not " }, if leap { 366 } else { 365 })));
    }
    for (_, said) in recall_events_at(text, now, lived) {
        out.push((&TRIGGERS[3], said));
    }
    out
}

/// The sentences of a trigger's world that explain it: those using most of its words (and the words of the text),
/// best first.
pub fn explain(t: &Trigger, text: &str, k: usize) -> Vec<String> {
    // the nerve graph: the text and the trigger's words spread to the sentences of its world
    let found = crate::nerves::graph().sentences(text, t.world, k);
    if !found.is_empty() {
        return found;
    }
    explain_by_overlap(t, text, k)
}

/// The world's sentences that share most words with the trigger (when the graph finds nothing).
fn explain_by_overlap(t: &Trigger, text: &str, k: usize) -> Vec<String> {
    let Some((_, _, manual)) = crate::state::CENTRAL_MANUALS.iter().find(|m| m.0 == t.world) else { return Vec::new() };
    let low = text.to_lowercase();
    let hooks: Vec<&str> = t.words.iter().copied().filter(|w| low.contains(w)).collect();
    let hooks = if hooks.is_empty() { t.words.to_vec() } else { hooks };
    let all = crate::nerves::sentences_of(manual);
    let mut scored: Vec<(usize, usize, &str)> = all
        .iter()
        .map(|l| {
            let ll = l.to_lowercase();
            (hooks.iter().filter(|w| ll.contains(*w)).count(), l.len(), l.as_str())
        })
        .filter(|x| x.0 > 0 && x.1 < 160)
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(b.2)));
    scored.into_iter().take(k).map(|x| x.2.to_string()).collect()
}

/// `world trigger "<English sentence>"`.
pub fn main_args(args: &[String]) -> Result<()> {
    if args.get(1).map(String::as_str) == Some("--expect") {
        println!("{}", expectation_line());
        return Ok(());
    }
    let text = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
    let fired = fire(&text);
    if fired.is_empty() {
        println!("No trigger fires on «{text}».");
    }
    for (t, said) in fired {
        println!("{} ({}): {said}.\n  Why — my world of {} says:", t.name, t.what, t.world);
        for s in explain(t, &text, 3) {
            println!("  · {s}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations_convert() {
        let f = fire("How many seconds are in three hours?");
        assert!(f[0].1.contains("that is 10800 seconds"), "{}", f[0].1);
        let f = fire("The round took 14 minutes and 30 seconds.");
        assert!(f[0].1.contains("together 14.50 minutes"), "{}", f[0].1);
        assert_eq!(show_duration(86400.0), "1 day");
    }

    #[test]
    fn cycles_and_leap_years() {
        let f = fire("A 3 GHz processor runs for 1 nanosecond.");
        assert!(f.is_empty() || !f.iter().any(|x| x.0.name == "cycles"), "GHz is not written out here");
        let f = fire("A processor at 3 gigahertz runs for 2 nanoseconds.");
        assert!(f.iter().any(|x| x.0.name == "cycles" && x.1.contains("6 clock cycles")), "{:?}", f.iter().map(|x| &x.1).collect::<Vec<_>>());
        let f = fire("Is the year 1900 a leap year?");
        assert!(f.iter().any(|x| x.1.contains("1900 is not a leap year")));
        let f = fire("Was 2000 a leap year?");
        assert!(f.iter().any(|x| x.1.contains("2000 is a leap year")));
    }

    #[test]
    fn the_dynamic_determinant_is_pure() {
        let lived = vec![("message".to_string(), 1_000_000, 0.0, "the operator: hello".to_string())];
        let a = fire_at("What time is it now? When was the last message?", 1_000_600, 3, &lived);
        let b = fire_at("What time is it now? When was the last message?", 1_000_600, 3, &lived);
        let show = |v: &[(&Trigger, String)]| v.iter().map(|x| x.1.clone()).collect::<Vec<_>>();
        assert_eq!(show(&a), show(&b));
        assert!(show(&a).iter().any(|x| x.contains("10 minutes ago")), "{:?}", show(&a));
        let c = fire_at("What time is it now?", 1_000_660, 3, &lived);
        assert_ne!(show(&a)[0], show(&c)[0], "another «now» is another input and gives another answer");
    }

    #[test]
    fn negative_control_no_trigger() {
        assert!(fire("The fox ate the cheese.").is_empty());
    }
}
