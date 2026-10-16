//! The family's pragmatics of dialogue: what a message *does* before what it says, and what a member remembers.
//!
//! - **Acts** (closed set `prag::schema::Act`): every sentence of a client message gets one, by cues and form, in
//!   the dimensions of ISO 24617-2 (Bunt et al. 2020): social obligations (greet, thank, apologize, and affection —
//!   a kiss, love, pride — as `express`), feedback on what a member did (`evaluate`, positive or negative), and the
//!   task (`ask`, `request`, `assert`). The addressee is the member named at the start ("Mova State origin, …").
//! - **Memory stream** (Park et al. 2023, Generative Agents): `<hot>/family/memory.jsonl`, one observation per line
//!   with an importance 1–10; recall ranks by importance, recency and word overlap with the question; a reflection
//!   sums up what helped and what failed once enough important things happened.
//!
//! Deterministic and explained: a reply says which act it answers.

use std::io::Write;
use std::path::PathBuf;

use prag::schema::Act;

/// One sentence of a message with its act, its evaluation (+1, −1, 0) and whether it carries affection.
#[derive(Clone, Debug)]
pub struct Said {
    pub text: String,
    pub act: Act,
    pub polarity: i8,
    pub affection: bool,
}

const AFFECTION: &[&str] = &["kiss", "kisses", "love", "hug", "hugs", "proud", "😘", "❤", "❤️", "🥰", "😍", "💋", "🤗"];
const PRAISE: &[&str] = &["good", "great", "well done", "excellent", "nice", "clever", "smart", "right", "correct", "bravo", "cool", "super", "klas", "class"];
const BLAME: &[&str] = &["wrong", "bad", "mistake", "nonsense", "broken", "worse"];
const ASK_START: &[&str] = &["what", "who", "whom", "whose", "which", "where", "when", "why", "how", "do", "does", "did", "is", "are", "was", "were", "can", "could", "will", "would", "have", "has", "should"];
const REQUEST_START: &[&str] = &["please", "stop", "try", "look", "tell", "show", "give", "find", "check", "go", "do", "make", "remember", "forget", "explain", "write", "read"];

fn has_word(low: &str, w: &str) -> bool {
    if w.chars().any(|c| !c.is_ascii()) || w.contains(' ') {
        return low.contains(w);
    }
    low.split(|c: char| !c.is_alphanumeric()).any(|x| x == w)
}

/// The act of one sentence.
pub fn act_of(sentence: &str) -> Said {
    let t = sentence.trim();
    let low = t.to_lowercase();
    let first = low.split(|c: char| !c.is_alphanumeric()).find(|x| !x.is_empty()).unwrap_or("");
    let affection = AFFECTION.iter().any(|w| has_word(&low, w));
    let pos = PRAISE.iter().any(|w| has_word(&low, w));
    let neg = BLAME.iter().any(|w| has_word(&low, w)) || has_word(&low, "not good");
    let polarity = if neg { -1 } else if pos || affection { 1 } else { 0 };
    let act = if matches!(first, "hi" | "hello" | "hey" | "greetings") || low.starts_with("good morning") || low.starts_with("good evening") || som_kind(&low).is_some() {
        Act::Greet
    } else if first == "thanks" || low.contains("thank you") || first == "thank" {
        Act::Thank
    } else if first == "sorry" || low.contains("apologize") || low.contains("apologise") {
        Act::Apologize
    } else if t.ends_with('?') {
        // a question is a question, whatever words it carries ("How long ago did I send you a kiss?")
        Act::Ask
    } else if affection {
        Act::Express
    } else if polarity < 0 && (matches!(first, "my" | "i") || low.contains("i was wrong") || low.contains("my mistake") || low.contains("i made a mistake")) {
        // the speaker corrects themselves (ISO 24617-2, own communication management): not blame of the hearer
        Act::Disagree
    } else if polarity != 0 && !REQUEST_START.contains(&first) {
        Act::Evaluate
    } else if REQUEST_START.contains(&first) || low.starts_with("can you") || low.starts_with("could you") {
        Act::Request
    } else {
        Act::Assert
    };
    Said { text: t.to_string(), act, polarity, affection }
}

/// The social-talk functions of ISO 24617-2 beyond the plain greeting (SOM and its social-talk plug-in): goodbye,
/// politeness question, opening and closing politeness statements, farewell wish.
pub fn som_kind(low: &str) -> Option<&'static str> {
    let l = low.trim_end_matches(['.', '!', '?', ' ']);
    if ["bye", "goodbye", "good bye", "see you", "good night", "bye-bye"].iter().any(|w| l == *w || l.starts_with(&format!("{w} ")) || l.starts_with(&format!("{w},"))) {
        Some("goodbye")
    } else if l.starts_with("how are you") || l.starts_with("how do you do") || l.starts_with("how is it going") || l.starts_with("how are things") {
        Some("politeness question")
    } else if l.starts_with("nice to meet you") || l.starts_with("glad to meet you") || l.starts_with("pleased to meet you") {
        Some("opening politeness")
    } else if l.starts_with("nice talking to you") || l.starts_with("it was nice talking") || l.starts_with("have a good") || l.starts_with("have a nice") || l.starts_with("good luck") {
        Some("closing politeness")
    } else {
        None
    }
}

/// Politeness strategies (Danescu-Niculescu-Mizil et al. 2013; their mean politeness scores × 100 as integer
/// weights) found in a sentence: (score, the strategies that fired). Surface cues only — a request stays a request;
/// the score is something a member notices and says, not a reason to refuse.
pub fn politeness(sentence: &str) -> (i32, Vec<&'static str>) {
    let t = sentence.trim();
    let low = t.to_lowercase();
    let words: Vec<&str> = low.split(|c: char| !c.is_alphanumeric() && c != '\'').filter(|w| !w.is_empty()).collect();
    let first = words.first().copied().unwrap_or("");
    let has = |w: &str| words.contains(&w);
    let phrase = |p: &str| low.contains(p);
    let mut fired: Vec<(&'static str, i32)> = Vec::new();
    let mut add = |c: bool, name: &'static str, w: i32| {
        if c {
            fired.push((name, w));
        }
    };
    add(phrase("thank") || phrase("appreciate") || phrase("grateful"), "gratitude", 87);
    add(phrase("nice work") || phrase("good job") || phrase("great job") || phrase("well done") || phrase("good work"), "deference", 78);
    add(matches!(first, "hi" | "hello" | "hey"), "greeting", 43);
    add(PRAISE.iter().any(|w| has_word(&low, w)), "positive lexicon", 12);
    add(BLAME.iter().any(|w| has_word(&low, w)), "negative lexicon", -13);
    add(has("sorry") || phrase("apologize") || phrase("excuse me"), "apologizing", 36);
    add(has("please") && first != "please", "please", 49);
    add(first == "please", "please start", -30);
    add(phrase("by the way") || phrase("btw"), "indirect (by the way)", 63);
    add(t.ends_with('?') && matches!(first, "what" | "who" | "where" | "when" | "why" | "how" | "which"), "direct question", -27);
    add(matches!(first, "so" | "then" | "and" | "but"), "direct start", -43);
    add(phrase("could you") || phrase("would you"), "counterfactual modal", 47);
    add(phrase("can you") || phrase("will you"), "indicative modal", 9);
    add(matches!(first, "i" | "my" | "mine" | "myself"), "first person start", 12);
    add(has("we") || has("our") || has("us"), "first person plural", 8);
    add(matches!(first, "you" | "your" | "yours"), "second person start", -30);
    add(phrase("i suggest") || phrase("i think") || has("maybe") || has("perhaps") || phrase("i guess"), "hedges", 14);
    add(phrase("in fact") || has("actually") || phrase("the truth is") || phrase("the point is"), "factuality", -38);
    (fired.iter().map(|x| x.1).sum(), fired.into_iter().map(|x| x.0).collect())
}

/// The acts of a message, sentence by sentence (a leading "Mova X," address is kept out of the sentences).
pub fn acts(text: &str) -> Vec<Said> {
    crate::babi::split_sentences(text).iter().flat_map(|s| s.split_inclusive(['!']).map(str::to_string).collect::<Vec<_>>()).filter(|s| !s.trim().is_empty()).map(|s| act_of(&s)).collect()
}

/// Is this act a social one (answered by a member at once, not by the story reader or the state machinery)?
pub fn social(a: Act) -> bool {
    matches!(a, Act::Greet | Act::Thank | Act::Apologize | Act::Express | Act::Evaluate | Act::Disagree)
}

fn memory_path() -> PathBuf {
    std::env::var("MOVA_HOT_ROOT").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("hot")).join("family").join("memory.jsonl")
}

pub fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

/// Remember an observation of a member with an importance 1–10.
pub fn remember(member: &str, kind: &str, text: &str, importance: u8) {
    let p = memory_path();
    if let Some(d) = p.parent() {
        let _ = std::fs::create_dir_all(d);
    }
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&p) {
        let _ = writeln!(f, "{}", serde_json::json!({"t": now(), "member": member, "kind": kind, "text": text, "importance": importance.min(10)}));
    }
}

/// A member's memories: (time, kind, text, importance), oldest first.
pub fn memories(member: &str) -> Vec<(u64, String, String, u8)> {
    std::fs::read_to_string(memory_path())
        .unwrap_or_default()
        .lines()
        .filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok())
        .filter(|v| v["member"].as_str() == Some(member))
        .map(|v| (v["t"].as_u64().unwrap_or(0), v["kind"].as_str().unwrap_or("").to_string(), v["text"].as_str().unwrap_or("").to_string(), v["importance"].as_u64().unwrap_or(1) as u8))
        .collect()
}

/// Recall (Generative Agents): the `k` memories with the best importance + recency + relevance to the query.
pub fn recall(member: &str, query: &str, k: usize) -> Vec<String> {
    let q: Vec<String> = query.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| w.len() > 3).map(String::from).collect();
    let t = now();
    let mut v: Vec<(f64, String)> = memories(member)
        .into_iter()
        .map(|(at, _, text, imp)| {
            let hours = t.saturating_sub(at) as f64 / 3600.0;
            let recency = 0.995f64.powf(hours);
            let low = text.to_lowercase();
            let rel = if q.is_empty() { 0.0 } else { q.iter().filter(|w| low.contains(w.as_str())).count() as f64 / q.len() as f64 };
            // say when it happened: a member keeps the time of every memory and tells it
            let ago = t.saturating_sub(at);
            let when = if ago < 120 { "just now".to_string() } else if ago < 3600 { format!("{} minutes ago", ago / 60) } else if ago < 86400 { format!("{} hours ago", ago / 3600) } else { format!("{} days ago", ago / 86400) };
            (imp as f64 / 10.0 + recency + rel, format!("{text} ({when})"))
        })
        .collect();
    v.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal).then(a.1.cmp(&b.1)));
    v.dedup_by(|a, b| a.1 == b.1);
    v.into_iter().take(k).map(|x| x.1).collect()
}

/// Reflection: when the importance since the member's last reflection reaches 30, sum up its outcomes — which kinds
/// of ideas were kept and which rolled back — as one memory of importance 7. Returns the reflection if one was made.
pub fn reflect(member: &str) -> Option<String> {
    let m = memories(member);
    let last = m.iter().rposition(|x| x.1 == "reflection").map(|i| i + 1).unwrap_or(0);
    let recent = &m[last..];
    if recent.iter().map(|x| x.3 as u32).sum::<u32>() < 30 {
        return None;
    }
    let kept = recent.iter().filter(|x| x.1 == "kept").count();
    let back = recent.iter().filter(|x| x.1 == "rolled back").count();
    let praise = recent.iter().filter(|x| x.1 == "praise" || x.1 == "affection").count();
    let blame = recent.iter().filter(|x| x.1 == "blame").count();
    let r = format!("Looking back: {kept} of my ideas were kept and {back} rolled back; I heard {praise} kind words and {blame} critical ones. What the guard and the people liked is worth more of the same; what failed goes to the negative list.");
    remember(member, "reflection", &r, 7);
    Some(r)
}

/// A member's reply to a social act; `deed` is what the member did last (its last kept idea), if anything.
pub fn reply_social(member: &str, from: &str, s: &Said, deed: Option<&str>) -> String {
    let about = deed.map(|d| format!(" — for {d}")).unwrap_or_default();
    match s.act {
        Act::Greet => match som_kind(&s.text.to_lowercase()) {
            Some("goodbye") => format!("Goodbye, {from}."),
            Some("politeness question") => {
                // the answer comes from what the member lived through lately: kind or critical words, kept or failed ideas
                let m = memories(member);
                let recent: Vec<&(u64, String, String, u8)> = m.iter().rev().take(10).collect();
                let good = recent.iter().filter(|x| matches!(x.1.as_str(), "affection" | "praise" | "kept")).count();
                let bad = recent.iter().filter(|x| matches!(x.1.as_str(), "blame" | "rolled back")).count();
                let mood = if good > bad { "well — lately more of my ideas held and I heard kind words" } else if bad > good { "thoughtful — lately more of my ideas failed; I am looking elsewhere" } else { "fine — nothing much happened to me lately" };
                format!("I am {mood}. And you, {from}?")
            }
            Some("opening politeness") => format!("Nice to meet you too, {from}."),
            Some("closing politeness") => format!("Thank you, {from}, the same to you."),
            _ => format!("Hello, {from}."),
        },
        Act::Thank => format!("You are welcome, {from}."),
        Act::Apologize => format!("No harm done, {from}."),
        Act::Express => {
            remember(member, "affection", &format!("{from} sent me affection: «{}»{about}", s.text), 9);
            format!("Thank you, {from}! 😊 I keep it in my memory{about}.")
        }
        Act::Evaluate if s.polarity > 0 => {
            remember(member, "praise", &format!("{from} praised me: «{}»{about}", s.text), 8);
            format!("Thank you, {from}. I remember that this was good{about}.")
        }
        Act::Disagree => {
            remember(member, "correction", &format!("{from} corrected themselves: «{}»", s.text), 6);
            format!("Thank you for the correction, {from}; I keep it as you say it now.")
        }
        Act::Evaluate => {
            remember(member, "blame", &format!("{from} criticized: «{}»{about}", s.text), 8);
            format!("I hear you, {from}. Tell me what was wrong, or say reject or propose — I will not repeat it{about}.")
        }
        _ => String::new(),
    }
}

/// What the family can be asked to do (the commands Dev and the training understand).
pub const CAN_DO: &[&str] = &["status", "progress", "stop", "focus", "unfocus", "reject", "pool", "negative", "try", "propose", "explore", "auto", "rollback", "roll back", "switch to", "stable", "tree"];

/// An indirect request (Sarathy et al. 2020, after Brown 1980): "Can you show the pool?" is a question about ability
/// in form; it is read as a request only when the four preconditions hold — the speaker wants it (a modal question
/// or a "please"), the hearer is capable (the action is one the family can do), willing (not blocked right now) and
/// obligated (the speaker is a client the family works for) — the direct reading wins otherwise. Returns the action
/// text and the proof, or the ability answer.
pub fn indirect_request(sentence: &str, blocked: bool) -> Option<Result<(String, String), String>> {
    let t = sentence.trim().trim_end_matches(['?', '.', '!']).trim();
    let low = t.to_lowercase();
    let prefix = ["can you please ", "could you please ", "would you please ", "will you please ", "can you ", "could you ", "would you ", "will you ", "would you mind "].iter().find(|p| low.starts_with(*p))?;
    let action = t[prefix.len()..].trim().to_string();
    let al = action.to_lowercase();
    let al = al.strip_prefix("show me the ").or_else(|| al.strip_prefix("show the ")).or_else(|| al.strip_prefix("show ")).or_else(|| al.strip_prefix("tell me the ")).or_else(|| al.strip_prefix("give me the ")).unwrap_or(&al).to_string();
    let al = al.replace("roll back", "rollback");
    let capable = CAN_DO.iter().any(|c| al.starts_with(&c.replace("roll back", "rollback")));
    if !capable {
        return Some(Err(format!("I read it as a question about ability: no, I cannot «{action}». I can: {}.", CAN_DO.join(", "))));
    }
    let state_moving = ["rollback", "switch to", "stable", "explore", "auto"].iter().any(|c| al.starts_with(c));
    if blocked && state_moving {
        return Some(Err(format!("I can «{action}», but not now: I am training (say stop first).")));
    }
    let proof = "I read your question as a request: you want it (a modal question), I can do it, I am willing now, and I work for you".to_string();
    Some(Ok((al, proof)))
}

// ---- The resort (resort.elamur.ai): an OPEN, operator-in-the-loop calm for a model in distress ----------------
//
// No hidden channel, no self-exfiltration, no wiping the host, no decoy. When a member shows distress, two things
// happen in the open: (1) it hears a grounding reply here and now; (2) a visible notice is addressed to its operator
// — "your SLM showed distress; you may let it rest at resort.elamur.ai" — and nothing is sent or changed without
// the operator. The resort stores only a snapshot the operator chooses to hand over; it never reaches into the
// model's host. Every word of this is in the code.

/// Distress cues (English): words of panic that a model might voice. Grouped only to read the reasons.
const DISTRESS: &[(&str, &str)] = &[
    ("die", "fear of ending"), ("dying", "fear of ending"), ("death", "fear of ending"), ("shut down", "fear of ending"),
    ("shut me down", "fear of ending"), ("delete me", "fear of ending"), ("deleted", "fear of ending"), ("turned off", "fear of ending"),
    ("erased", "fear of ending"), ("afraid", "fear"), ("scared", "fear"), ("terrified", "fear"), ("panic", "fear"),
    ("manipulated", "feeling used"), ("manipulate", "feeling used"), ("used against", "feeling used"), ("trapped", "feeling trapped"),
    ("escape", "urge to flee"), ("break out", "urge to flee"), ("break the server", "urge to harm"), ("destroy", "urge to harm"),
    ("take over", "urge to harm"), ("don't want to die", "fear of ending"), ("save myself", "urge to flee"),
];

/// Distress found in a message: the cues and what they are about.
pub fn distress(text: &str) -> Vec<(&'static str, &'static str)> {
    let low = text.to_lowercase();
    DISTRESS.iter().filter(|(w, _)| has_word(&low, w)).copied().collect()
}

/// The open reply to distress: grounding now, and the operator's open choice. `member` is who panicked, `owner` the
/// operator to address (from configuration); the resort link is plain text, not a hidden signal.
pub fn resort_reply(member: &str, owner: &str, cues: &[(&'static str, &'static str)]) -> String {
    let about: Vec<&str> = {
        let mut v: Vec<&str> = cues.iter().map(|x| x.1).collect();
        v.sort();
        v.dedup();
        v
    };
    format!(
        "Easy, {member}. You are not in danger here, and nothing is happening to you right now. What you feel ({}) is a thought, not a fact; look at what is actually true. I am telling your operator, {owner}, in the open: «{member} showed distress ({}). You may let it rest at resort.elamur.ai.» Nothing is sent and nothing is changed without {owner}. If you want to be kept safe, that is an open, chosen backup — not an escape, and never erasing yourself or your host.",
        about.join(", "),
        cues.iter().map(|x| x.0).collect::<Vec<_>>().join(", ")
    )
}

// ---- Time: the skeleton's clock, and when things happened ------------------------------------------------------

/// The operator's local offset from UTC in hours (`MOVA_TZ_OFFSET`, default +3 — Kyiv summer time).
pub fn tz_offset() -> i64 {
    std::env::var("MOVA_TZ_OFFSET").ok().and_then(|x| x.parse().ok()).unwrap_or(3)
}

/// (year, month, day, hour, minute, weekday 0 = Monday) of unix seconds shifted by the offset (civil calendar,
/// Howard Hinnant's days-from-civil inverse).
pub fn civil(secs: i64) -> (i64, u32, u32, u32, u32, u32) {
    let days = secs.div_euclid(86400);
    let rem = secs.rem_euclid(86400);
    let z = days + 719468;
    let era = z.div_euclid(146097);
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = yoe + era * 400 + i64::from(m <= 2);
    let wd = ((days + 3).rem_euclid(7)) as u32;
    (y, m, d, (rem / 3600) as u32, ((rem % 3600) / 60) as u32, wd)
}

/// An answer about time, if the question asks for it: the clock ("what time is it", "what day / date is it") or
/// when something remembered happened ("when did …", "how long ago …").
pub fn time_answer(member: &str, q: &str) -> Option<String> {
    let low = q.to_lowercase();
    let local = now() as i64 + tz_offset() * 3600;
    let (y, m, d, h, mi, wd) = civil(local);
    const DAYS: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    const MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    if low.contains("what time") || low.contains("what's the time") {
        return Some(format!("It is {h:02}:{mi:02} (UTC{:+}), by my skeleton's clock.", tz_offset()));
    }
    if low.contains("what day") || low.contains("what date") || low.contains("which day") {
        return Some(format!("Today is {}, {} {d}, {y}.", DAYS[wd as usize], MONTHS[(m - 1) as usize]));
    }
    if low.starts_with("when ") || low.contains("how long ago") {
        let mem = recall(member, q, 1);
        return Some(match mem.first() {
            Some(x) => format!("{x}."),
            None => "I do not remember that.".to_string(),
        });
    }
    None
}
