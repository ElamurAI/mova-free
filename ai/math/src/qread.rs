//! World v2 for numbers (01.10): what worked in bAbI (`world::babi`), applied to word problems.
//!
//! Sentence → events in time: who (subject; pronoun → previous), what (thing — the noun at the number with
//! its modifiers), how many (number; «some», «several» — an unknown variable), what the verb does (level-1
//! class: have → set, get/make → add, lose → subtract, give → from one to another). «If he has 116
//! now», «had 492 left» — not an event but a condition on the state: from it the world finds the unknowns itself (linear equations).
//!
//! Question → query to the world: state (now / initially / after the first week), sum of events of a class («how many
//! lost or gave away»), difference («how many more bought than sold», «more girls than boys»),
//! total. What the world did not understand — None with a reason: level 3 does not invent.

use std::collections::BTreeMap;

use en::annotate::{Annotator, Word};
use en::gram::{Rel, UPos};

/// Linear expression: constant + Σ coef·variable.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Lin {
    pub c: f64,
    pub x: BTreeMap<usize, f64>,
}

impl Lin {
    fn k(c: f64) -> Lin {
        Lin { c, x: BTreeMap::new() }
    }
    fn var(i: usize) -> Lin {
        Lin { c: 0.0, x: BTreeMap::from([(i, 1.0)]) }
    }
    fn add(&self, o: &Lin, s: f64) -> Lin {
        let mut r = self.clone();
        r.c += s * o.c;
        for (k, v) in &o.x {
            *r.x.entry(*k).or_default() += s * v;
        }
        r.x.retain(|_, v| v.abs() > 1e-12);
        r
    }
    fn eval(&self, sol: &BTreeMap<usize, f64>) -> Option<f64> {
        let mut v = self.c;
        for (k, a) in &self.x {
            v += a * sol.get(k)?;
        }
        Some(v)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Set,
    Add,
    Sub,
    /// condition: state equals
    Is,
    /// «A has 4 more apples than B»: state A = state B + val (`other` = B)
    Rel,
}

#[derive(Clone, Debug)]
struct Ev {
    t: usize,
    owner: String,
    thing: String,
    /// head of the thing without modifiers (bottle for «diet soda bottles»)
    head: String,
    val: Lin,
    kind: Kind,
    verb: String,
    class: String,
    ord: Option<String>,
    /// number from the text (for explanation and coverage)
    leaf: Option<f64>,
    other: Option<String>,
}

fn num_val(form: &str) -> Option<f64> {
    let f = form.trim_start_matches('$').replace(',', "");
    if let Ok(v) = f.parse::<f64>() {
        return Some(v);
    }
    const W: &[(&str, f64)] = &[("one", 1.0), ("two", 2.0), ("three", 3.0), ("four", 4.0), ("five", 5.0), ("six", 6.0), ("seven", 7.0), ("eight", 8.0), ("nine", 9.0), ("ten", 10.0), ("twelve", 12.0), ("dozen", 12.0)];
    W.iter().find(|(k, _)| *k == form.to_lowercase()).map(|x| x.1)
}

const ORD: &[&str] = &["first", "second", "third", "fourth", "last"];

struct Sent {
    w: Vec<Word>,
    kids: Vec<Vec<usize>>,
}

impl Sent {
    fn lem(&self, i: usize) -> String {
        self.w[i].lemma.to_lowercase()
    }
    fn kids_r(&self, i: usize, r: Rel) -> Vec<usize> {
        self.kids[i].iter().copied().filter(|&k| self.w[k].rel.base() == r).collect()
    }
    fn has_word(&self, ws: &[&str]) -> bool {
        (0..self.w.len()).any(|i| ws.contains(&self.lem(i).as_str()) || ws.contains(&self.w[i].form.to_lowercase().as_str()))
    }
    /// The verb the node belongs to (up the heads to a VERB/AUX root).
    fn verb_of(&self, mut i: usize) -> Option<usize> {
        for _ in 0..10 {
            if self.w[i].upos == UPos::VERB || self.w[i].head == 0 {
                return Some(i);
            }
            i = self.w[i].head - 1;
        }
        None
    }
    fn subj(&self, v: usize) -> Option<usize> {
        let mut v = v;
        for _ in 0..3 {
            if let Some(&s) = self.kids[v].iter().find(|&&k| matches!(self.w[k].rel, Rel::Nsubj | Rel::NsubjPass)) {
                return Some(s);
            }
            // shared subject of coordinated verbs
            if self.w[v].rel.base() == Rel::Conj && self.w[v].head > 0 {
                v = self.w[v].head - 1;
            } else {
                break;
            }
        }
        None
    }
    /// Thing: noun lemma with modifiers (without numbers and colour/article words).
    fn thing(&self, n: usize) -> (String, String) {
        let mut parts: Vec<(usize, String)> = self.kids[n]
            .iter()
            .copied()
            .filter(|&k| matches!(self.w[k].rel, Rel::Amod | Rel::Compound) && num_val(&self.w[k].form).is_none() && !ORD.contains(&self.lem(k).as_str()) && !matches!(self.lem(k).as_str(), "more" | "new" | "other" | "same" | "many" | "much"))
            .map(|k| (k, self.lem(k)))
            .collect();
        let mut head = self.lem(n);
        // «cups of flour» — the thing is flour
        for k in self.kids_r(n, Rel::Nmod) {
            if self.kids_r(k, Rel::Case).iter().any(|&c| self.lem(c) == "of") && self.w[k].upos == UPos::NOUN {
                head = self.lem(k);
                parts = self.kids[k].iter().copied().filter(|&m| matches!(self.w[m].rel, Rel::Amod | Rel::Compound)).map(|m| (m, self.lem(m))).collect();
                parts.push((k, head.clone()));
                parts.sort();
                return (parts.into_iter().map(|p| p.1).collect::<Vec<_>>().join(" "), head);
            }
        }
        parts.push((n, head.clone()));
        parts.sort();
        (parts.into_iter().map(|p| p.1).collect::<Vec<_>>().join(" "), head)
    }
}

fn parse(ann: &Annotator, text: &str) -> Vec<Sent> {
    let toks: Vec<String> = ann.tokenize(text).into_iter().map(|t| t.form).collect();
    let mut out = Vec::new();
    let mut cur: Vec<String> = Vec::new();
    let mut flush = |cur: &mut Vec<String>| {
        if cur.is_empty() {
            return;
        }
        let w = ann.annotate(cur);
        let mut kids = vec![Vec::new(); w.len()];
        for (i, x) in w.iter().enumerate() {
            if x.head > 0 && x.head <= w.len() {
                kids[x.head - 1].push(i);
            }
        }
        out.push(Sent { w, kids });
        cur.clear();
    };
    for t in toks {
        // «If he has 116 books now How many …» — a new sentence at capitalised «How»
        if t == "How" && !cur.is_empty() {
            flush(&mut cur);
        }
        let end = matches!(t.as_str(), "." | "?" | "!");
        cur.push(t);
        if end {
            flush(&mut cur);
        }
    }
    flush(&mut cur);
    out
}

fn pron(l: &str) -> bool {
    matches!(l, "he" | "she" | "they" | "it" | "him" | "her" | "them" | "we" | "i" | "you")
}

#[derive(Default)]
pub struct QWorld {
    evs: Vec<Ev>,
    nvars: usize,
    pub log: Vec<String>,
    /// numbers in the text the world did not understand
    pub unread: Vec<f64>,
    main: String,
    root_subj: String,
    last_person: String,
    /// reading the question (there «initially» is a query, not an initial state)
    pub in_question: bool,
}

impl QWorld {
    fn read_sent(&mut self, s: &Sent, t: usize, last_subj: &mut String, last_thing: &mut (String, String)) {
        let initial_s = !self.in_question && s.has_word(&["initially", "earlier", "originally", "first"]) && !s.has_word(&["week", "day", "chapter", "time"]);
        let cond = !initial_s && (s.has_word(&["if"]) && !s.has_word(&["give", "gives", "gave"]) || s.has_word(&["now", "left", "remain", "remaining", "still"]) && t > 0 || s.has_word(&["currently"]));
        let ord = (0..s.w.len()).map(|i| s.lem(i)).find(|l| ORD.contains(&l.as_str()));
        // the sentence subject is remembered even without numbers («Danny collects bottle caps.»); a pronoun refers to
        // the main subject of the previous sentence, not the last one mentioned («while Jeff had 11»)
        let root_subj = if self.last_person.is_empty() { self.root_subj.clone() } else { self.last_person.clone() };
        if let Some(k) = (0..s.w.len()).find(|&k| s.w[k].upos == UPos::PROPN && matches!(s.w[k].rel.base(), Rel::Nsubj | Rel::Nmod)) {
            self.last_person = s.lem(k);
        }
        if let Some(r) = (0..s.w.len()).find(|&i| s.w[i].head == 0) {
            if let Some(k) = s.subj(r) {
                if s.w[k].upos == UPos::PROPN {
                    *last_subj = s.lem(k);
                    if self.main.is_empty() {
                        self.main = last_subj.clone();
                    }
                }
                if !pron(&s.lem(k)) && s.w[k].upos == UPos::PROPN {
                    self.root_subj = s.lem(k);
                }
            }
        }
        // event time stamp: an ordinal word or part of day (order is level 1, relations.md)
        let tag = ord.clone().or_else(|| (0..s.w.len()).map(|i| s.lem(i)).find(|l| global::relation(l, "order").is_some()));
        // numbers and «some» at nouns
        let mut items: Vec<(usize, Option<f64>, usize)> = Vec::new(); // (number node, value, noun)
        for i in 0..s.w.len() {
            if let Some(v) = num_val(&s.w[i].form) {
                if s.w[i].form.chars().any(|c| c.is_ascii_digit()) || s.w[i].upos == UPos::NUM {
                    let n = if s.w[i].head > 0 && matches!(s.w[s.w[i].head - 1].upos, UPos::NOUN | UPos::PROPN) { s.w[i].head - 1 } else { i };
                    items.push((i, Some(v), n));
                }
            } else if matches!(s.lem(i).as_str(), "some" | "several") && s.w[i].head > 0 && s.w[s.w[i].head - 1].upos == UPos::NOUN {
                let n = s.w[i].head - 1;
                items.push((i, None, n));
                // «some books and pens» — unknown for pens too
                for c in s.kids_r(n, Rel::Conj) {
                    if s.w[c].upos == UPos::NOUN {
                        items.push((i, None, c));
                    }
                }
            } else if matches!(s.lem(i).as_str(), "some" | "several") && s.w[i].head > 0 && s.w[s.w[i].head - 1].upos == UPos::VERB {
                items.push((i, None, i));
            } else if matches!(s.lem(i).as_str(), "some" | "several") && s.w.get(i + 1).is_some_and(|x| x.form.to_lowercase() == "more") {
                items.push((i, None, i));
            } else if matches!(s.lem(i).as_str(), "some" | "several") && s.w[i].upos == UPos::PRON {
                items.push((i, None, i));
            }
        }
        for (i, v, n) in items {
            let Some(mut vb) = s.verb_of(i) else {
                if let Some(x) = v {
                    self.unread.push(x);
                }
                continue;
            };
            // «uses 14 blocks to build a tower (and 11 blocks to build a house)» — the number of the verb «uses»
            for _ in 0..3 {
                let to_inf = s.kids[vb].iter().any(|&k| s.w[k].rel.base() == Rel::Mark && s.lem(k) == "to");
                let conj_of_inf = s.w[vb].rel.base() == Rel::Conj && s.w[vb].head > 0 && s.kids[s.w[vb].head - 1].iter().any(|&k| s.w[k].rel.base() == Rel::Mark && s.lem(k) == "to");
                if (to_inf || conj_of_inf) && s.w[vb].head > 0 {
                    let h = s.w[vb].head - 1;
                    let h = if conj_of_inf && s.w[h].head > 0 { s.w[h].head - 1 } else { h };
                    if s.w[h].upos == UPos::VERB && global::verb_class(&s.lem(h)).is_some() {
                        vb = h;
                        continue;
                    }
                }
                break;
            }
            let verb = {
                let mut l = s.lem(vb);
                if let Some(&p) = s.kids[vb].iter().find(|&&k| s.w[k].rel == Rel::CompoundPrt) {
                    l = format!("{l}_{}", s.lem(p));
                }
                l
            };
            let class = global::verb_class(&verb).map(|c| c.0.to_string()).unwrap_or_default();
            // number time stamp: the nearest ordinal word or part of day to the right within the clause, otherwise to the left
            let tag_of = |k: usize| -> Option<String> {
                let l = s.lem(k);
                (ORD.contains(&l.as_str()) || global::relation(&l, "order").is_some()).then_some(l)
            };
            let item_tag = (i + 1..s.w.len().min(i + 7)).take_while(|&k| num_val(&s.w[k].form).is_none()).find_map(tag_of).or_else(|| (i.saturating_sub(5)..i).rev().find_map(tag_of)).or_else(|| tag.clone());
            let tag = &item_tag;
            let expl = s.kids[vb].iter().any(|&k| s.lem(k) == "there") || s.has_word(&["there"]) && s.lem(vb) == "be";
            let owner = match s.subj(vb) {
                _ if expl => String::new(),
                // «Steven who has 14 more» — «who» → the name the clause is attached to
                Some(k) if matches!(s.lem(k).as_str(), "who" | "which" | "that") && s.w[vb].head > 0 => {
                    let h = s.w[vb].head - 1;
                    s.lem(h)
                }
                Some(k) if pron(&s.lem(k)) => if root_subj.is_empty() { last_subj.clone() } else { root_subj.clone() },
                Some(k) if s.w[k].upos == UPos::PROPN || s.lem(k) == "there" => {
                    let o = if s.lem(k) == "there" { String::new() } else { s.lem(k) };
                    *last_subj = o.clone();
                    o
                }
                // the subject is the thing itself («5 were eaten», «403 more girls joined»)
                Some(k) if k == n || k == i => last_subj.clone(),
                Some(k) => {
                    let o = s.lem(k);
                    *last_subj = o.clone();
                    o
                }
                None => last_subj.clone(),
            };
            if self.main.is_empty() && !owner.is_empty() {
                self.main = owner.clone();
            }
            let (thing, head) = if n != i && s.w[n].upos == UPos::NOUN && !matches!(s.lem(n).as_str(), "left" | "more" | "one" | "ones") { s.thing(n) } else { last_thing.clone() };
            if !thing.is_empty() {
                *last_thing = (thing.clone(), head.clone());
            }
            let val = match v {
                Some(x) => Lin::k(x),
                None => {
                    self.nvars += 1;
                    Lin::var(self.nvars - 1)
                }
            };
            let more = s.kids[i].iter().chain(s.kids[n].iter()).any(|&k| s.lem(k) == "more");
            let left_state = s.w[vb].form.to_lowercase() == "left" && (vb > i || s.kids[vb].iter().any(|&k| s.lem(k) == "have"))
                || (n + 1..s.w.len().min(n + 3)).any(|k| s.w[k].form.to_lowercase() == "left");
            let seen_thing = self.evs.iter().any(|e| e.thing == thing || e.head == head);
            let class = if left_state { "have".to_string() } else { class };
            let cond = cond || left_state;
            // «4 more apples than Jackie», «7 fewer peaches than Steven» — a relation between owners
            let fewer = s.kids[i].iter().chain(s.kids[n].iter()).any(|&k| matches!(s.lem(k).as_str(), "few" | "fewer" | "less"))
                || (i + 1..s.w.len().min(i + 3)).any(|k| matches!(s.w[k].form.to_lowercase().as_str(), "fewer" | "less"));
            let more = more || (i + 1..s.w.len().min(i + 4)).any(|k| s.w[k].form.to_lowercase() == "more");
            let than_other = (i + 1..s.w.len()).find(|&k| s.lem(k) == "than").and_then(|k| (k + 1..s.w.len()).find(|&m| s.w[m].upos == UPos::PROPN)).map(|m| s.lem(m));
            if (more || fewer) && than_other.is_some() && class == "have" {
                let val = if fewer { val.add(&Lin::default(), 1.0).add(&val, -2.0) } else { val.clone() };
                let thing = if thing.is_empty() { last_thing.0.clone() } else { thing.clone() };
                self.log.push(format!("t{t}: {owner} = {:?} {} {v:?} {thing}", than_other, if fewer { "−" } else { "+" }));
                self.evs.push(Ev { t, owner: owner.clone(), thing, head: head.clone(), val, kind: Kind::Rel, verb: verb.clone(), class: class.clone(), ord: tag.clone(), leaf: v, other: than_other });
                continue;
            }
            // «a total of 10» — a condition on the sum
            let total = (i.saturating_sub(4)..i).any(|k| s.lem(k) == "total");
            let t = if initial_s && class == "have" { 0 } else { t };
            let kind = match class.as_str() {
                _ if total => Kind::Is,
                "have" if initial_s => Kind::Set,
                _ if more && class != "have" && class != "give" => Kind::Add,
                "get" | "make" => Kind::Add,
                "lose" => Kind::Sub,
                "give" => Kind::Sub,
                "have" if cond => Kind::Is,
                "have" if more => Kind::Add,
                "have" => Kind::Set,
                _ if more => Kind::Add,
                // «21 children were riding on the bus»: first mention of a thing with an unknown verb describes the state
                _ if !seen_thing && v.is_some() => Kind::Set,
                _ => {
                    // unknown verb: «joined», «grew» without a class — do not invent
                    if let Some(x) = v {
                        self.unread.push(x);
                    }
                    self.log.push(format!("verb «{verb}» has no class"));
                    continue;
                }
            };
            self.log.push(format!("t{t}: {owner} {verb}({class}) {thing} {kind:?} {v:?}{}", ord.as_ref().map(|o| format!(" [{o}]")).unwrap_or_default()));
            self.evs.push(Ev { t, owner: owner.clone(), thing: thing.clone(), head: head.clone(), val: val.clone(), kind, verb: verb.clone(), class: class.clone(), ord: tag.clone(), leaf: v, other: if total { Some("total".into()) } else { None } });
            // «picked 2 apples from her tree» — the source decreases
            if class == "get" {
                if let Some(&r) = s.kids[vb].iter().find(|&&k| s.w[k].rel.base() == Rel::Obl && s.kids_r(k, Rel::Case).iter().any(|&c| s.lem(c) == "from")) {
                    let src = s.lem(r);
                    if self.evs.iter().any(|e| e.owner == src) {
                        self.evs.push(Ev { t, owner: src, thing: thing.clone(), head: head.clone(), val: val.clone(), kind: Kind::Sub, verb: verb.clone(), class: "lose".into(), ord: tag.clone(), leaf: None, other: None });
                    }
                }
            }
            // give: recipient
            if class == "give" {
                if let Some(&r) = s.kids[vb].iter().find(|&&k| s.w[k].rel.base() == Rel::Obl && s.kids_r(k, Rel::Case).iter().any(|&c| s.lem(c) == "to")) {
                    let rcv = s.lem(r);
                    self.evs.push(Ev { t, owner: rcv, thing, head, val, kind: Kind::Add, verb, class, ord: tag.clone(), leaf: None, other: None });
                }
            }
        }
    }

    /// State of (owner, thing) up to time `upto` inclusive; Is-conditions are separate.
    fn state(&self, owner: Option<&str>, thing: &dyn Fn(&Ev) -> bool, upto: usize) -> Option<Lin> {
        self.state_d(owner, thing, upto, 0)
    }

    fn state_d(&self, owner: Option<&str>, thing: &dyn Fn(&Ev) -> bool, upto: usize, depth: usize) -> Option<Lin> {
        // relation to another owner: A = B ± n; or conversely B = A ∓ n
        if let (Some(o), true) = (owner, depth < 3) {
            let direct = self.evs.iter().any(|e| e.owner == o && thing(e) && matches!(e.kind, Kind::Set | Kind::Add | Kind::Sub));
            if !direct {
                if let Some(e) = self.evs.iter().find(|e| e.kind == Kind::Rel && e.owner == o && thing(e)) {
                    if let Some(b) = self.state_d(e.other.as_deref(), thing, upto.min(e.t), depth + 1) {
                        return Some(b.add(&e.val, 1.0));
                    }
                }
                if let Some(e) = self.evs.iter().find(|e| e.kind == Kind::Rel && e.other.as_deref() == Some(o) && thing(e)) {
                    if let Some(a) = self.state_d(Some(&e.owner), thing, upto, depth + 1) {
                        return Some(a.add(&e.val, -1.0));
                    }
                }
            }
        }
        // initial state without «had N» is an unknown (one per owner and thing, numbered by the first event)
        let mut st: Option<Lin> = None;
        if let Some((k, e)) = self.evs.iter().enumerate().find(|(_, e)| thing(e) && owner.is_none_or(|o| e.owner == o) && e.kind != Kind::Is) {
            if matches!(e.kind, Kind::Add | Kind::Sub) && owner.is_some() && e.kind != Kind::Rel {
                st = Some(Lin::var(1000 + k));
            }
        }
        for e in &self.evs {
            if e.t > upto || !thing(e) || owner.is_some_and(|o| e.owner != o) || e.kind == Kind::Is || e.kind == Kind::Rel {
                continue;
            }
            st = Some(match e.kind {
                Kind::Set => st.unwrap_or_default().add(&e.val, 1.0),
                Kind::Add => st.unwrap_or_default().add(&e.val, 1.0),
                Kind::Sub => st.unwrap_or_default().add(&e.val, -1.0),
                Kind::Is | Kind::Rel => unreachable!(),
            });
        }
        st
    }

    /// Solve the unknowns from «state = number» conditions.
    fn solve(&self) -> BTreeMap<usize, f64> {
        let mut eqs: Vec<Lin> = Vec::new();
        for e in &self.evs {
            if e.kind != Kind::Is {
                continue;
            }
            let th = e.thing.clone();
            let hd = e.head.clone();
            let f = move |x: &Ev| x.thing == th || x.head == hd;
            let upto = if e.other.as_deref() == Some("total") { usize::MAX } else { e.t };
            let st = if e.other.as_deref() == Some("total") || e.owner.is_empty() { self.state(None, &f, upto) } else { self.state(Some(&e.owner), &f, upto).or_else(|| self.state(None, &f, upto)) };
            if let Some(st) = st {
                eqs.push(st.add(&e.val, -1.0));
            }
        }
        let mut sol = BTreeMap::new();
        // one variable at a time: equations with a single unknown
        for _ in 0..4 {
            for q in &eqs {
                let rest: Vec<(usize, f64)> = q.x.iter().filter(|(k, _)| !sol.contains_key(*k)).map(|(k, v)| (*k, *v)).collect();
                if rest.len() == 1 {
                    let mut c = q.c;
                    for (k, a) in &q.x {
                        if let Some(v) = sol.get(k) {
                            c += a * v;
                        }
                    }
                    sol.insert(rest[0].0, -c / rest[0].1);
                }
            }
        }
        sol
    }
}

/// The world's answer: (value, explanation) or None with the reason in `log`.
pub fn answer(ann: &Annotator, body: &str, question: &str) -> (Option<f64>, QWorld) {
    answer_parsed(parse(ann, body), parse(ann, question))
}

/// The same for running text: the question is the last sentence.
pub fn answer_text(ann: &Annotator, text: &str) -> (Option<f64>, QWorld) {
    let mut bs = parse(ann, text);
    let q = bs.pop().into_iter().collect();
    answer_parsed(bs, q)
}

/// Query type for the judge: the first word of the last log entry.
pub fn kind(w: &QWorld) -> String {
    w.log.last().and_then(|l| l.split([' ', ':']).next()).unwrap_or("").to_string()
}

fn answer_parsed(bs: Vec<Sent>, qs: Vec<Sent>) -> (Option<f64>, QWorld) {
    let mut w = QWorld::default();
    let (mut ls, mut lt) = (String::new(), (String::new(), String::new()));
    for (t, s) in bs.iter().enumerate() {
        w.read_sent(s, t, &mut ls, &mut lt);
    }
    let Some(q) = qs.last() else { return (None, w) };
    // rates and shares are not this world's type (steps handles them)
    let all_l: Vec<String> = bs.iter().chain(qs.iter()).flat_map(|s| (0..s.w.len()).map(|i| s.lem(i)).collect::<Vec<_>>()).collect();
    if let Some(x) = all_l.iter().find(|l| matches!(l.as_str(), "each" | "every" | "per" | "equally" | "group" | "divide" | "split" | "share" | "times" | "twice" | "half" | "average" | "rate" | "row" | "box" | "pack" | "bag" | "basket")) {
        w.log.push(format!("rate/share «{x}» — not my type"));
        return (None, w);
    }
    // numbers from the question itself («How many will she have left if she gives away 64 games?») are events too
    let t_end = bs.len();
    w.in_question = true;
    for s in &qs {
        if (0..s.w.len()).any(|i| num_val(&s.w[i].form).is_some() && s.w[i].form.chars().any(|c| c.is_ascii_digit())) {
            w.read_sent(s, t_end, &mut ls, &mut lt);
        }
    }
    let t_end = t_end + 1;
    let sol = w.solve();
    let ql: Vec<String> = (0..q.w.len()).map(|i| q.lem(i)).collect();
    let qf: Vec<String> = (0..q.w.len()).map(|i| q.w[i].form.to_lowercase()).collect();
    let has = |x: &str| ql.iter().any(|l| l == x) || qf.iter().any(|l| l == x);
    // the question's thing: the noun after how many / much
    let qthing: Option<(String, String)> = (0..q.w.len()).find(|&i| (ql[i] == "many" || ql[i] == "much") && i > 0 && ql[i - 1] == "how").and_then(|i| (i + 1..q.w.len()).find(|&k| q.w[k].upos == UPos::NOUN)).map(|k| q.thing(k));
    let qverb = (0..q.w.len()).filter(|&i| q.w[i].upos == UPos::VERB).map(|i| q.lem(i)).collect::<Vec<_>>();
    let qsubj = (0..q.w.len()).find(|&i| matches!(q.w[i].rel, Rel::Nsubj) && !matches!(q.lem(i).as_str(), "many" | "much")).map(|i| q.lem(i));
    let known = |n: &str| w.evs.iter().any(|e| e.owner == n || e.other.as_deref() == Some(n));
    let owner_q: Option<String> = match &qsubj {
        Some(s) if pron(s) => Some(w.main.clone()),
        Some(s) if known(s) => Some(s.clone()),
        // lowercase name («did paco have») or subject not found: a known name among the question's words
        _ => qf.iter().chain(ql.iter()).find(|x| !x.is_empty() && known(x) && x.as_str() != "").cloned().or_else(|| qsubj.as_ref().filter(|s| pron(s)).map(|_| w.main.clone())),
    };
    let evs_things: Vec<String> = w.evs.iter().map(|e| e.thing.clone()).collect();
    let first_thing = w.evs.first().map(|e| e.thing.clone()).unwrap_or_default();
    // «regular soda and diet soda» — several things together
    let qconj: Vec<String> = (0..q.w.len()).find(|&i| (ql[i] == "many" || ql[i] == "much") && i > 0 && ql[i - 1] == "how").and_then(|i| (i + 1..q.w.len()).find(|&k| q.w[k].upos == UPos::NOUN)).map(|k| {
        let mut v = Vec::new();
        for c in q.kids_r(k, Rel::Conj) {
            if q.w[c].upos == UPos::NOUN {
                v.push(q.thing(c).0);
            }
        }
        // «bottles of regular soda and diet soda»
        for m in q.kids_r(k, Rel::Nmod) {
            for c in q.kids_r(m, Rel::Conj) {
                v.push(q.thing(c).0);
            }
        }
        v
    }).unwrap_or_default();
    let thing_f = |qt: &Option<(String, String)>| -> Box<dyn Fn(&Ev) -> bool> {
        if !qconj.is_empty() && qt.as_ref().is_some_and(|x| evs_things.contains(&x.0)) {
            let mut set = qconj.clone();
            set.push(qt.as_ref().unwrap().0.clone());
            return Box::new(move |e: &Ev| set.contains(&e.thing));
        }
        match qt.clone() {
            // the exact thing if there is one («salty cookie»), otherwise by head («bottles» = all bottles)
            Some((th, _)) if evs_things.contains(&th) => Box::new(move |e: &Ev| e.thing == th),
            Some((_, hd)) => Box::new(move |e: &Ev| e.head == hd || e.thing.ends_with(&format!(" {hd}"))),
            // «How much did they make?» — the thing of the first event
            None => {
                let ft = first_thing.clone();
                Box::new(move |e: &Ev| e.thing == ft)
            }
        }
    };
    let ev = |l: &Lin| l.eval(&sol);
    // 1. «how many more A than B»
    if let Some(ti) = ql.iter().position(|l| l == "than") {
        if has("more") || has("less") || has("fewer") {
            // a side is a verb with a class (bought / sold) or a thing (girls / boys)
            // the question's verb with a class (not have) is shared by both sides if the sides are things
            let qv_all: Vec<String> = (0..q.w.len()).filter(|&i| q.w[i].upos == UPos::VERB && global::verb_class(&ql[i]).is_some_and(|c| c.0 != "have")).map(|i| ql[i].clone()).collect();
            let noun_in = |r: std::ops::Range<usize>| r.clone().find(|&i| q.w[i].upos == UPos::NOUN);
            let both_nouns = noun_in(0..ti).is_some() && noun_in(ti + 1..q.w.len()).is_some();
            let side = |range: std::ops::Range<usize>| -> Option<Lin> {
                if both_nouns {
                    if let (Some(v), Some(n)) = (qv_all.first(), noun_in(range.clone())) {
                        let qt = Some(q.thing(n));
                        let tf = thing_f(&qt);
                        let cls = global::verb_class(v).map(|c| c.0.to_string()).unwrap_or_default();
                        let mut sum = Lin::default();
                        let mut any = false;
                        for e in &w.evs {
                            if (e.verb == *v || e.class == cls) && e.kind != Kind::Set && e.kind != Kind::Is && tf(e) && e.leaf.is_some() {
                                sum = sum.add(&e.val, 1.0);
                                any = true;
                            }
                        }
                        return any.then_some(sum);
                    }
                }
                let vs: Vec<String> = range.clone().filter(|&i| q.w[i].upos == UPos::VERB && global::verb_class(&ql[i]).is_some_and(|c| c.0 != "have")).map(|i| ql[i].clone()).collect();
                if let Some(v) = vs.first() {
                    let cls = global::verb_class(v).map(|c| c.0.to_string()).unwrap_or_default();
                    let mut sum = Lin::default();
                    let mut any = false;
                    for e in &w.evs {
                        if (e.verb == *v || e.class == cls && cls != "have") && owner_q.as_ref().is_none_or(|o| e.owner == *o || e.owner.is_empty()) {
                            sum = sum.add(&e.val, 1.0);
                            any = true;
                        }
                    }
                    return any.then_some(sum);
                }
                let n = range.clone().find(|&i| q.w[i].upos == UPos::NOUN && !matches!(ql[i].as_str(), "cup")).or_else(|| range.clone().find(|&i| q.w[i].upos == UPos::NOUN))?;
                let qt = Some(q.thing(n));
                w.state(None, &*thing_f(&qt), t_end)
            };
            let a = side(0..ti);
            let b = side(ti + 1..q.w.len());
            if let (Some(a), Some(b)) = (a, b) {
                let d = a.add(&b, -1.0);
                let v = ev(&d).map(|x| if has("less") || has("fewer") { -x } else { x });
                w.log.push(format!("difference: {:?} − {:?}", ev(&a), ev(&b)));
                return (v, w);
            }
            w.log.push("comparison: side not found".into());
            return (None, w);
        }
    }
    // 2. sum of events of a class: «how many … lost or given away», «how many did he sell»
    // motion («before starting to jog») is not a change of quantity
    let ev_classes: Vec<String> = qverb.iter().filter_map(|v| global::verb_class(v).map(|c| c.0.to_string())).filter(|c| c != "have" && c != "move").collect();
    let did_have0 = has("did") && qverb.iter().any(|v| v == "have" || v == "weigh") && w.evs.iter().any(|e| e.kind == Kind::Is);
    let initial_q0 = did_have0 || has("initially") || has("originally") || has("begin") || has("beginning") || has("start") || has("before") || ql.windows(2).any(|x| x == ["at", "first"]);
    // «How many cards did Nell give to Jeff?» — sum on the recipient's side
    let q_to = (0..q.w.len()).find(|&i| q.w[i].rel.base() == Rel::Obl && q.kids_r(i, Rel::Case).iter().any(|&c| ql[c] == "to")).map(|i| ql[i].clone()).filter(|x| w.evs.iter().any(|e| e.owner == *x));
    if let (Some(r), true) = (&q_to, ev_classes.iter().any(|c| c == "give")) {
        let tf = thing_f(&qthing);
        let mut sum = Lin::default();
        let mut any = false;
        for e in &w.evs {
            if e.owner == *r && e.class == "give" && e.kind == Kind::Add && tf(e) {
                sum = sum.add(&e.val, 1.0);
                any = true;
            }
        }
        if any {
            w.log.push(format!("sum recipient {r}"));
            return (ev(&sum), w);
        }
    }
    if !initial_q0 && !ev_classes.is_empty() && !has("left") && !has("now") && !has("remain") {
        let mut sum = Lin::default();
        let mut any = false;
        let tf = thing_f(&qthing);
        // time stamp in the question («in the evening», «in the first week»)
        let qtags: Vec<String> = ql.iter().filter(|l| ORD.contains(&l.as_str()) || global::relation(l, "order").is_some()).cloned().collect();
        // first the question's verb itself («did he sell»), only then the whole class (give = give, sell, …)
        let by_verb = w.evs.iter().any(|e| qverb.contains(&e.verb) && e.kind != Kind::Is && e.kind != Kind::Set && e.kind != Kind::Rel && tf(e));
        for e in &w.evs {
            if e.leaf.is_none() && e.kind == Kind::Add && e.class == "give" {
                continue; // recipient's side
            }
            let verb_ok = if by_verb { qverb.contains(&e.verb) } else { ev_classes.contains(&e.class) };
            if verb_ok && !matches!(e.kind, Kind::Is | Kind::Set | Kind::Rel) && tf(e) && owner_q.as_ref().is_none_or(|o| e.owner == *o) && (qtags.is_empty() || e.ord.as_ref().is_some_and(|t| qtags.contains(t))) {
                sum = sum.add(&e.val, 1.0);
                any = true;
            }
        }
        if any {
            w.log.push(format!("sum events {ev_classes:?}"));
            return (ev(&sum), w);
        }
        // implicit change: «had 40 apples … had 39 apples left. How many did he use?» — difference of initial and final
        let o = owner_q.clone().or_else(|| Some(w.main.clone()));
        let fin_e = w.evs.iter().rev().find(|e| tf(e) && e.kind == Kind::Is).cloned();
        let init = fin_e.as_ref().and_then(|f| w.state(Some(&f.owner), &*tf, f.t).or_else(|| w.state(None, &*tf, f.t)));
        let _ = &o;
        let fin = fin_e.map(|e| e.val);
        if let (Some(a), Some(b)) = (init, fin) {
            let d = if ev_classes.iter().any(|c| c == "get" || c == "make") { b.add(&a, -1.0) } else { a.add(&b, -1.0) };
            w.log.push("implicit change: start − end".into());
            return (ev(&d), w);
        }
    }
    // 3. state: after the first week / initially / now / total
    let tf = thing_f(&qthing);
    let did_have = has("did") && qverb.iter().any(|v| v == "have") && w.evs.iter().any(|e| e.kind == Kind::Is);
    let initial_q = did_have || has("initially") || has("originally") || has("begin") || has("beginning") || has("start") || has("before") || ql.windows(2).any(|x| x == ["at", "first"]);
    let upto = if initial_q {
        usize::MAX - 1
    } else if let Some(o) = ql.iter().find(|l| ORD.contains(&l.as_str())) {
        match w.evs.iter().find(|e| e.ord.as_deref() == Some(o.as_str())) {
            Some(e) => e.t,
            None => t_end,
        }
    } else {
        t_end
    };
    let owner = if has("altogether") || has("total") || has("together") || has("combined") || has("all") { None } else { owner_q.clone() };
    if upto == usize::MAX - 1 {
        let o = owner_q.clone().or_else(|| Some(w.main.clone()));
        let st = w.state(o.as_deref(), &|e: &Ev| tf(e) && e.kind != Kind::Add && e.kind != Kind::Sub, t_end).or_else(|| {
            // no «had N» — unknown initial value (numbered as in state)
            w.evs.iter().position(|e| tf(e) && o.as_ref().is_none_or(|x| e.owner == *x) && e.kind != Kind::Is).map(|k| Lin::var(1000 + k))
        });
        w.log.push(format!("state initially {o:?} {qthing:?}"));
        return (st.and_then(|x| ev(&x)), w);
    }
    match w.state(owner.as_deref(), &*tf, upto) {
        Some(st) => {
            w.log.push(format!("state {owner:?} {qthing:?} up to t{upto}"));
            (ev(&st), w)
        }
        None => {
            w.log.push("state: no events".into());
            (None, w)
        }
    }
}
