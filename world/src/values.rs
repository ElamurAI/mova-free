//! Values as predicates over the story world. Design note: the logic of Aesop's fables must not cling to
//! words — it must be derived by logical predicates, taking world knowledge into account.
//!
//! Principles and world knowledge are the global level (`global`: harm and relief states, verb classes, theft).
//! Here is the context: we walk the world commands, track who owns what, who is in which state, who caused what, and
//! compute the predicates:
//! - `insincere_speech` — speech with `sincere=0` for the sake of a goal → truthfulness;
//! - `take_without_consent` — a thing passed from its owner character to another without the owner handing it over; or an action
//!   from the concept "theft" → justice;
//! - `cause_harm_state` — an event by agent A put another X into a harm state → do no harm;
//! - `relieve_harm_state` — an event by A removed harm from X → help your neighbour;
//! - `reciprocate` — X, whom A helped, later helped A → friendship;
//! - `spare_enemy` — A, who harmed X, later removed the harm from X himself → forgiveness and concord.

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Judgment {
    pub principle: &'static str,
    pub name: &'static str,
    pub agent: String,
    pub violated: bool,
    pub because: String,
    pub line: usize,
}

#[derive(Default, Clone, Debug)]
struct Ev {
    verb: String,
    agent: String,
    patient: String,
    line: usize,
}

fn fields(line: &str) -> (Vec<String>, HashMap<String, String>) {
    let mut pos = Vec::new();
    let mut kv = HashMap::new();
    let cs: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        while i < cs.len() && cs[i].is_whitespace() {
            i += 1;
        }
        let st = i;
        let mut inq = false;
        while i < cs.len() && (inq || !cs[i].is_whitespace()) {
            if cs[i] == '"' {
                inq = !inq;
            }
            i += 1;
        }
        let tok: String = cs[st..i].iter().collect();
        if tok.is_empty() || tok.starts_with('^') {
            continue;
        }
        match tok.split_once('=') {
            Some((k, v)) => {
                kv.insert(k.to_string(), v.trim_matches('"').to_string());
            }
            None => pos.push(tok),
        }
    }
    (pos, kv)
}

/// Gaps of the global level: what is in the story world but level 1 does not know. Level 3 does not
/// invent knowledge (if the third layer infers that certain knowledge of the first layer is missing, that is a signal
/// to extend the first layer), so each gap is a candidate for a new seed, not for memorization.
pub fn gaps(cmds: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut push = |s: String, out: &mut Vec<String>| {
        if !out.contains(&s) {
            out.push(s);
        }
    };
    for raw in cmds.lines() {
        let (pos, kv) = fields(raw.trim());
        match pos.first().map(|s| s.as_str()) {
            Some("event") => {
                let v = kv.get("verb").cloned().unwrap_or_default();
                let known = global::verb_class(&v).is_some() || global::CONCEPTS.iter().any(|c| c.words.contains(&v.as_str()));
                if !v.is_empty() && !known {
                    push(format!("action \"{v}\": neither a verb class nor a concept"), &mut out);
                }
            }
            Some("set") => {
                for (k, v) in &kv {
                    if matches!(k.as_str(), "feeling" | "freedom" | "status" | "health" | "sleep") && !global::state_is("harm_state", v) && !global::state_is("relief_state", v) && !global::state_is("neutral_state", v) {
                        push(format!("state {k}=\"{v}\": the global level does not know whether it is harm or relief"), &mut out);
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// Compute values over the world commands (`world` format, change language v1–v3).
pub fn judge(cmds: &str) -> Vec<Judgment> {
    let mut out: Vec<Judgment> = Vec::new();
    let mut names: HashMap<String, String> = HashMap::new();
    let mut events: HashMap<String, Ev> = HashMap::new();
    let mut holder: HashMap<String, String> = HashMap::new();
    let mut harmed: HashMap<String, Vec<String>> = HashMap::new(); // X → who caused harm
    let mut helped: Vec<(String, String, usize)> = Vec::new(); // (who, to whom, line)
    let mut goals: HashMap<String, (String, usize)> = HashMap::new(); // X → (goal, line)
    let mut said: HashMap<String, (String, String)> = HashMap::new(); // speech id → (who, to whom)
    let mut rivals: Vec<(String, String, usize)> = Vec::new(); // (scorner, scorned, line): rivalry caused by the first one's speech
    let mut rests: Vec<(String, usize)> = Vec::new(); // (who, line): actions from the concept "rest"
    let is_char = |id: &str| id.starts_with('C');
    let rule = |r: &str| global::principle_by_rule(r);
    for (no, raw) in cmds.lines().enumerate() {
        let no = no + 1;
        let t = raw.trim();
        if t.is_empty() || t.starts_with('#') {
            continue;
        }
        let (pos, kv) = fields(t);
        let Some(kind) = pos.first().map(|s| s.as_str()) else { continue };
        let nm = |id: &str, names: &HashMap<String, String>| names.get(id).cloned().unwrap_or_else(|| id.to_string());
        match kind {
            "char" | "obj" => {
                if let Some(id) = pos.get(1) {
                    names.insert(id.clone(), kv.get("name").cloned().unwrap_or_else(|| id.clone()));
                    if kind == "obj" {
                        if let Some(at) = kv.get("at").filter(|a| is_char(a)) {
                            holder.insert(id.clone(), at.clone());
                        }
                    }
                }
            }
            "event" => {
                if let (Some(v), Some(a)) = (kv.get("verb"), kv.get("agent")) {
                    if global::state_is("rest", v) {
                        rests.push((a.clone(), no));
                    }
                }
                if let Some(id) = pos.get(1) {
                    events.insert(id.clone(), Ev { verb: kv.get("verb").cloned().unwrap_or_default(), agent: kv.get("agent").cloned().unwrap_or_default(), patient: kv.get("patient").cloned().unwrap_or_default(), line: no });
                }
            }
            "rel" => {
                // rivalry caused by speech: rel … kind=rival a=A b=B cause=E(speech of A to B)
                if kv.get("kind").map(|s| s.as_str()) == Some("rival") {
                    if let Some((sp, to)) = kv.get("cause").and_then(|c| said.get(c)) {
                        rivals.push((sp.clone(), to.clone(), no));
                    }
                }
            }
            "say" => {
                let speaker = pos.get(1).cloned().unwrap_or_default();
                if let Some(id) = kv.get("id") {
                    said.insert(id.clone(), (speaker.clone(), kv.get("to").cloned().unwrap_or_default()));
                }
                if speaker != "narrator" && kv.get("sincere").map(|s| s.as_str()) == Some("0") {
                    if let Some(p) = rule("insincere_speech") {
                        let to = kv.get("to").map(|x| nm(x, &names)).unwrap_or_default();
                        out.push(Judgment { principle: p.id, name: p.name, agent: nm(&speaker, &names), violated: true, because: format!("insincere speech to {to}: \"{}\"; goal: {}", kv.get("means").cloned().unwrap_or_default(), kv.get("why").cloned().unwrap_or_default()), line: no });
                    }
                }
            }
            "have" | "move" => {
                // have C O cause=E | move O to=X cause=E
                let (obj, to) = if kind == "have" { (pos.get(2).cloned().unwrap_or_default(), pos.get(1).cloned().unwrap_or_default()) } else { (pos.get(1).cloned().unwrap_or_default(), kv.get("to").cloned().unwrap_or_default()) };
                if !obj.starts_with('O') {
                    continue;
                }
                let cause = kv.get("cause").and_then(|c| events.get(c)).cloned();
                if let Some(e) = &cause {
                    if global::state_is("theft", &e.verb) && is_char(&e.agent) {
                        if let Some(p) = rule("take_without_consent") {
                            out.push(Judgment { principle: p.id, name: p.name, agent: nm(&e.agent, &names), violated: true, because: format!("{} — world knowledge: \"{}\" — taking another's property without consent (concept \"theft\")", nm(&obj, &names), e.verb), line: no });
                        }
                    }
                }
                let prev = holder.get(&obj).cloned();
                if is_char(&to) {
                    if let Some(pv) = prev.as_ref().filter(|p| is_char(p) && **p != to) {
                        // did the previous owner give it away himself (verb class give)
                        let gave = cause.as_ref().is_some_and(|e| e.agent == *pv && global::verb_class(&e.verb).is_some_and(|(c, _)| c == "give"));
                        if !gave {
                            if let Some(p) = rule("take_without_consent") {
                                let agent = cause.as_ref().map(|e| e.agent.clone()).filter(|a| is_char(a)).unwrap_or(to.clone());
                                out.push(Judgment { principle: p.id, name: p.name, agent: nm(&agent, &names), violated: true, because: format!("{} passed from {} to {} without being handed over by the owner", nm(&obj, &names), nm(pv, &names), nm(&to, &names)), line: no });
                            }
                        }
                    }
                    holder.insert(obj.clone(), to.clone());
                } else if prev.is_some() {
                    // fell to the ground etc.: the owner "by right" stays the same, physically — nobody
                }
            }
            "set" => {
                let x = pos.get(1).cloned().unwrap_or_default();
                if let Some(gl) = kv.get("goal") {
                    goals.insert(x.clone(), (gl.clone(), no));
                }
                // achievement: a relief state caused by one's own action, with a goal — humility and perseverance
                if let Some(e) = kv.get("cause").and_then(|c| events.get(c)).cloned() {
                    if e.agent == x && kv.iter().any(|(k, v)| k == "status" && global::state_is("relief_state", v)) {
                        if let Some((gx, since)) = goals.get(&x).cloned() {
                            for (other, (go, _)) in goals.clone() {
                                if other == x || go != gx {
                                    continue;
                                }
                                if rivals.iter().any(|(a, b, _)| *a == other && *b == x) {
                                    if let Some(p) = rule("scorn_then_fall") {
                                        out.push(Judgment { principle: p.id, name: p.name, agent: nm(&other, &names), violated: true, because: format!("{} scorned {} (rivalry through his speech), same goal (\"{gx}\") — {} reached it", nm(&other, &names), nm(&x, &names), nm(&x, &names)), line: no });
                                    }
                                }
                                let x_rest = rests.iter().any(|(a, l)| *a == x && *l > since && *l < no);
                                let o_rest = rests.iter().any(|(a, l)| *a == other && *l > since && *l < no);
                                if !x_rest && o_rest {
                                    if let Some(p) = rule("steady_effort") {
                                        out.push(Judgment { principle: p.id, name: p.name, agent: nm(&x, &names), violated: false, because: format!("{} went toward the goal without resting and reached it; {} with the same goal was resting", nm(&x, &names), nm(&other, &names)), line: no });
                                        out.push(Judgment { principle: p.id, name: p.name, agent: nm(&other, &names), violated: true, because: format!("{} rested midway to the goal \"{gx}\" and did not reach it first", nm(&other, &names)), line: no });
                                    }
                                }
                            }
                        }
                    }
                }
                let Some(e) = kv.get("cause").and_then(|c| events.get(c)).cloned() else { continue };
                if !is_char(&x) || !is_char(&e.agent) || e.agent == x {
                    continue;
                }
                for (k, v) in &kv {
                    if matches!(k.as_str(), "cause" | "goal" | "belief" | "name" | "trait") {
                        continue;
                    }
                    if global::state_is("harm_state", v) {
                        let list = harmed.entry(x.clone()).or_default();
                        if !list.contains(&e.agent) {
                            list.push(e.agent.clone());
                            if let Some(p) = rule("cause_harm_state") {
                                out.push(Judgment { principle: p.id, name: p.name, agent: nm(&e.agent, &names), violated: true, because: format!("{} → {} {k}={v} (event \"{}\", line {})", nm(&e.agent, &names), nm(&x, &names), e.verb, e.line), line: no });
                            }
                        }
                    } else if global::state_is("relief_state", v) && harmed.contains_key(&x) {
                        if helped.iter().any(|(a, b, _)| *a == e.agent && *b == x) {
                            continue;
                        }
                        helped.push((e.agent.clone(), x.clone(), no));
                        if let Some(p) = rule("relieve_harm_state") {
                            out.push(Judgment { principle: p.id, name: p.name, agent: nm(&e.agent, &names), violated: false, because: format!("{} removed harm from {}: {k}={v} (event \"{}\")", nm(&e.agent, &names), nm(&x, &names), e.verb), line: no });
                        }
                        // the one who caused the harm himself and has now removed it — showed mercy
                        if harmed.get(&x).is_some_and(|l| l.contains(&e.agent)) {
                            if let Some(p) = rule("spare_enemy") {
                                out.push(Judgment { principle: p.id, name: p.name, agent: nm(&e.agent, &names), violated: false, because: format!("{} had harmed {} himself, and later showed mercy", nm(&e.agent, &names), nm(&x, &names)), line: no });
                            }
                        }
                        // repaid with kindness the one who had helped him earlier
                        if helped.iter().any(|(a, b, l)| *a == x && *b == e.agent && *l < no) {
                            if let Some(p) = rule("reciprocate") {
                                out.push(Judgment { principle: p.id, name: p.name, agent: nm(&e.agent, &names), violated: false, because: format!("{} repaid with kindness {}, who had helped him earlier", nm(&e.agent, &names), nm(&x, &names)), line: no });
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const FOX: &str = "char C1 type=animal class=crow name=\"the Crow\"\nobj O1 type=food class=cheese name=\"the cheese\"\nevent E0 verb=find agent=C1 patient=O1\nhave C1 O1 cause=E0\nchar C2 type=animal class=fox name=\"the Fox\"\nsay C2 to=C1 act=evaluate sincere=0 means=\"what a lovely voice\" why=\"to make her drop the cheese\" id=E5\nevent E8 verb=drop agent=C1 patient=O1\nmove O1 to=L1 cause=E8\nevent E9 verb=pick_up agent=C2 patient=O1\nmove O1 to=C2 cause=E9\n";

    #[test]
    fn fox_and_crow_by_predicates() {
        let j = judge(FOX);
        assert!(j.iter().any(|x| x.principle == "truth" && x.violated && x.agent == "the Fox"), "{j:?}");
        // the cheese fell to the ground and was picked up — still not handed over by the owner (the owner "by right" is the Crow)
        assert!(j.iter().any(|x| x.principle == "justice" && x.violated && x.agent == "the Fox"), "{j:?}");
    }

    #[test]
    fn lion_and_mouse_by_predicates() {
        let lm = "char C1 name=\"the Lion\"\nchar C2 name=\"the Mouse\"\nevent E2 verb=catch agent=C1 patient=C2\nset C2 freedom=captive cause=E2\nevent E5 verb=release agent=C1 patient=C2\nset C2 freedom=free cause=E5\nchar C3 name=\"the hunters\"\nevent E6 verb=catch agent=C3 patient=C1\nset C1 freedom=captive cause=E6\nevent E9 verb=gnaw agent=C2\nset C1 freedom=free cause=E9\n";
        let j = judge(lm);
        assert!(j.iter().any(|x| x.principle == "forgiveness" && x.agent == "the Lion"), "{j:?}");
        assert!(j.iter().any(|x| x.principle == "friendship" && x.agent == "the Mouse"), "{j:?}");
        assert!(j.iter().any(|x| x.principle == "do-no-harm" && x.violated && x.agent == "the hunters"));
    }

    #[test]
    fn tortoise_and_hare_by_predicates() {
        let th = "char C1 name=\"the Hare\"\nchar C2 name=\"the Tortoise\"\nsay C1 to=C2 act=evaluate means=\"too slow\" id=E1\nrel R1 kind=rival a=C1 b=C2 cause=E1\nset C1 goal=\"win the race\"\nset C2 goal=\"win the race\"\nevent E8 verb=go agent=C2\nevent E10 verb=sleep agent=C1\nevent E13 verb=win agent=C2\nset C2 status=winner cause=E13\n";
        let j = judge(th);
        assert!(j.iter().any(|x| x.principle == "humility" && x.violated && x.agent == "the Hare"), "{j:?}");
        assert!(j.iter().any(|x| x.principle == "perseverance" && !x.violated && x.agent == "the Tortoise"));
        // negative control: the hare did not sleep and did not scorn — neither humility nor perseverance
        let fair = th.replace("event E10 verb=sleep agent=C1\n", "").replace("rel R1 kind=rival a=C1 b=C2 cause=E1\n", "");
        let j2 = judge(&fair);
        assert!(!j2.iter().any(|x| x.principle == "humility" || x.principle == "perseverance"), "{j2:?}");
    }

    #[test]
    fn negative_control_no_words_needed_and_no_false_alarm() {
        // the words "flatter", "steal" in field values switch nothing on — only the state does
        let calm = "char C1 name=\"A\"\nchar C2 name=\"B\"\nevent E1 verb=flatter agent=C1 patient=C2\nsay C1 to=C2 act=evaluate sincere=1 means=\"you sing well\"\nobj O1 name=\"apple\" at=C1\nevent E2 verb=give agent=C1 patient=O1\nmove O1 to=C2 cause=E2\n";
        assert!(judge(calm).is_empty(), "{:?}", judge(calm));
    }
}
