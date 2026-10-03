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
use en::gram::{Rel, Tag, UPos};

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
    fn scale(&self, s: f64) -> Lin {
        Lin::default().add(self, s)
    }
    /// Replace solved unknowns by their expressions (over number leaves).
    fn subst(&self, sol: &BTreeMap<usize, Lin>) -> Lin {
        let mut r = Lin::k(self.c);
        for (k, a) in &self.x {
            match sol.get(k) {
                Some(e) => r = r.add(e, *a),
                None => r = r.add(&Lin::var(*k), *a),
            }
        }
        r
    }
    /// Unknowns (not number leaves) still in the expression.
    fn unknowns(&self) -> impl Iterator<Item = usize> + '_ {
        self.x.keys().copied().filter(|&k| k < LEAF)
    }
}

/// Variables from `LEAF` on are the numbers of the text (leaf j = `LEAF + j`): the answer stays an expression over
/// them, so the world knows which numbers it used and can explain «26 = 12 + 14».
const LEAF: usize = 1 << 20;

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
    /// the noun the number counts when it measures another («pages» of «8 pages of math homework»)
    unit: String,
    val: Lin,
    kind: Kind,
    verb: String,
    class: String,
    ord: Option<String>,
    other: Option<String>,
    /// other nouns of the number's clause: purpose and place («to build a house», «at the park»)
    ctx: Vec<String>,
    /// the other side of a transfer (the recipient of «gave», the source of «picked from»), not a told number
    derived: bool,
    /// the owner is a source outside the story's pile («cut 8 roses from her garden»): not in the all-owner sums
    outside: bool,
}

fn num_val(form: &str) -> Option<f64> {
    let f = form.trim_start_matches('$').replace(',', "");
    if let Ok(v) = f.parse::<f64>() {
        return Some(v);
    }
    const W: &[(&str, f64)] = &[("one", 1.0), ("two", 2.0), ("three", 3.0), ("four", 4.0), ("five", 5.0), ("six", 6.0), ("seven", 7.0), ("eight", 8.0), ("nine", 9.0), ("ten", 10.0), ("twelve", 12.0), ("dozen", 12.0)];
    W.iter().find(|(k, _)| *k == form.to_lowercase()).map(|x| x.1)
}

/// Class of the unknown change the world puts before a told state that its events do not explain.
const CHANGE: &str = "change";

const ORD: &[&str] = &["first", "second", "third", "fourth", "last"];

/// A time stamp word: an ordinal, a part of the day or a day of the week (level 1, relations.md).
fn time_word(l: &str) -> bool {
    ORD.contains(&l) || ["order", "day", "week"].iter().any(|k| global::relation(l, k).is_some())
}

struct Sent {
    w: Vec<Word>,
    kids: Vec<Vec<usize>>,
    /// «new» is part of the thing when the story contrasts new and old ones («57 new games and 39 old games»);
    /// otherwise it only marks an addition («got 4 new customers»)
    keep_new: bool,
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
    /// The noun a number counts. English numbers precede their noun («12 more people», «5 baseball cards»), so a
    /// head noun to the right wins; when the parse attaches the number elsewhere, the noun right after it (over
    /// adjectives like «more», «new», «red») is taken; the last noun of a compound chain is the head.
    fn counted_noun(&self, i: usize) -> usize {
        let h = self.w[i].head;
        if h > i + 1 && matches!(self.w[h - 1].upos, UPos::NOUN | UPos::PROPN) {
            // a compound modifier («chocolate» of «chocolate chip cookies») leads to its head
            let mut k = h - 1;
            while self.w[k].rel == Rel::Compound && self.w[k].head > k + 1 && self.w[self.w[k].head - 1].upos == UPos::NOUN {
                k = self.w[k].head - 1;
            }
            return k;
        }
        // partitive «16 of the books», «56 of them» («25 of us» are people)
        if self.w.get(i + 1).is_some_and(|x| x.form.to_lowercase() == "of") {
            if self.w.get(i + 2).is_some_and(|x| matches!(x.form.to_lowercase().as_str(), "us" | "you")) {
                return i + 2;
            }
            if let Some(k) = (i + 2..self.w.len().min(i + 4)).find(|&k| matches!(self.w[k].upos, UPos::NOUN | UPos::PRON)) {
                if self.w[k].upos == UPos::NOUN {
                    return k;
                }
            }
        }
        let mut found = None;
        let mut k = i + 1;
        while k < self.w.len().min(i + 6) {
            match self.w[k].upos {
                // «push - ups»: a hyphenated compound continues to its last part
                UPos::NOUN if self.w.get(k + 1).is_some_and(|x| x.form == "-") && self.w.get(k + 2).is_some_and(|x| x.upos == UPos::NOUN) => {
                    k += 2;
                    continue;
                }
                UPos::NOUN if time_word(&self.lem(k)) => break,
                UPos::NOUN => found = Some(k),
                UPos::ADJ | UPos::ADV if found.is_none() => {}
                _ => break,
            }
            k += 1;
        }
        if let Some(k) = found {
            return k;
        }
        i
    }
    /// Context nouns of a number: the nouns after its counted noun up to the next number. «71 bottle caps and 24
    /// wrappers at the park» — a bare coordinated item shares the trailing phrase of the last item.
    fn ctx_nouns(&self, i: usize, n: usize, starts: &[usize]) -> Vec<String> {
        let from = n.max(i) + 1;
        let next = starts.iter().copied().find(|&k| k > i).unwrap_or(self.w.len());
        let bare = from >= next || self.w.get(from).is_some_and(|x| matches!(x.upos, UPos::CCONJ | UPos::PUNCT)) && starts.iter().any(|&k| k > i);
        let (a, b) = if bare {
            let last = starts.iter().copied().max().unwrap_or(i);
            let ln = self.counted_noun(last).max(last);
            (ln + 1, self.w.len())
        } else {
            (from, next)
        };
        (a..b).filter(|&k| matches!(self.w[k].upos, UPos::NOUN | UPos::PROPN) && !time_word(&self.lem(k))).map(|k| self.lem(k)).collect()
    }
    /// The unit a number is counted per: a noun with «each»/«every» in the clause (not the counted noun, not
    /// counted itself), «per Y», or «a Y» right after the counted noun or in a phrase («on a block»).
    fn per_noun(&self, i: usize, n: usize) -> Option<String> {
        // «a total of 42 pieces» is not a rate
        if (i.saturating_sub(3)..i).any(|k| self.lem(k) == "total") {
            return None;
        }
        let counted_by_num = |y: usize| self.kids[y].iter().any(|&c| num_val(&self.w[c].form).is_some());
        let nums: Vec<usize> = (0..self.w.len()).filter(|&k| num_val(&self.w[k].form).is_some() && (self.w[k].form.chars().any(|c| c.is_ascii_digit()) || self.w[k].upos == UPos::NUM)).collect();
        // «each Y» belongs to the nearest number and the items coordinated with it («Each basket has 19 red
        // peaches and 4 green peaches»), not to every number of the sentence («30 pencils … 5 pencils in each row»)
        // (a tie goes to the number after: «each necklace takes 2 beads»)
        let nearest = |k: usize| nums.iter().copied().min_by_key(|&m| ((m as isize - k as isize).unsigned_abs(), m < k));
        let mine = |k: usize| -> bool {
            let Some(m) = nearest(k) else { return false };
            if m == i {
                return true;
            }
            let (a, b) = (m.min(i), m.max(i));
            (a..b).any(|x| self.w[x].upos == UPos::CCONJ) && !(a + 1..b).any(|x| matches!(self.w[x].upos, UPos::VERB | UPos::AUX))
        };
        // «each of his 29 bookshelves»: the unit is the noun of the «of» phrase (its own number counts the units)
        // (the number inside the «of» phrase counts the units; the rate is the nearest other number)
        let each_of = (0..self.w.len()).find(|&k| matches!(self.lem(k).as_str(), "each" | "every") && self.w.get(k + 1).is_some_and(|x| x.form == "of")).and_then(|k| (k + 2..self.w.len().min(k + 6)).find(|&m| self.w[m].upos == UPos::NOUN).map(|y| (k, y)));
        if let Some((k, y)) = each_of.filter(|&(_, y)| y != n) {
            let outside = nums.iter().copied().filter(|&m| !(k..=y).contains(&m)).min_by_key(|&m| ((m as isize - k as isize).unsigned_abs(), m < k));
            let coord = |m: usize| {
                let (a, b) = (m.min(i), m.max(i));
                (a..b).any(|x| self.w[x].upos == UPos::CCONJ) && !(a + 1..b).any(|x| matches!(self.w[x].upos, UPos::VERB | UPos::AUX))
            };
            if outside.is_some_and(|m| m == i || coord(m)) {
                return Some(self.noun_end(y));
            }
        }
        let each_y = |k: usize| matches!(self.lem(k).as_str(), "each" | "every") && self.w[k].head > 0 && self.w[self.w[k].head - 1].upos == UPos::NOUN && self.w[k].head - 1 != n && !counted_by_num(self.w[k].head - 1);
        // («$ 62 off each t-shirt and $ 99 off each jersey»: the number's own «each» first)
        let own = (0..self.w.len()).find(|&k| each_y(k) && nearest(k) == Some(i));
        if let Some(k) = own.or_else(|| (0..self.w.len()).find(|&k| each_y(k) && mine(k))) {
            let y = self.w[k].head - 1;
            // «31 packs of pencils each one having 6 pencils»: «one» is the noun before
            if self.lem(y) == "one" {
                return (0..k).rev().find(|&m| self.w[m].upos == UPos::NOUN).map(|m| self.lem(m));
            }
            return Some(self.noun_end(y));
        }
        let mut from = n.max(i) + 1;
        // «15 water bottles and 54 soda bottles a day»: a bare coordinated item shares the last item's phrase
        if self.w.get(from).is_some_and(|x| matches!(x.upos, UPos::CCONJ | UPos::PUNCT)) {
            if let Some(&last) = nums.last().filter(|&&l| l > i && !(i + 1..l).any(|x| matches!(self.w[x].upos, UPos::VERB | UPos::AUX))) {
                from = self.counted_noun(last).max(last) + 1;
            }
        }
        for k in from..self.w.len().min(from + 4) {
            let l = self.lem(k);
            if matches!(l.as_str(), "per" | "a" | "an") {
                if let Some(y) = (k + 1..self.w.len().min(k + 3)).find(|&m| self.w[m].upos == UPos::NOUN) {
                    let in_pp = l == "per" || k == from || k > 0 && matches!(self.lem(k - 1).as_str(), "in" | "on" | "for");
                    if in_pp {
                        return Some(self.noun_end(y));
                    }
                }
            }
            if num_val(&self.w[k].form).is_some() {
                break;
            }
        }
        None
    }
    /// The head lemma of a noun phrase starting at `y`: over compounds and hyphens («t - shirt» → shirt).
    fn noun_end(&self, mut y: usize) -> String {
        loop {
            if self.w.get(y + 1).is_some_and(|x| x.form == "-") && self.w.get(y + 2).is_some_and(|x| x.upos == UPos::NOUN) {
                y += 2;
            } else if self.w[y].rel == Rel::Compound && self.w[y].head > y + 1 && self.w[self.w[y].head - 1].upos == UPos::NOUN {
                y = self.w[y].head - 1;
            } else {
                return self.lem(y);
            }
        }
    }
    /// «the rest are boys», «the rest were bottles of diet soda»: the predicate noun after a form of «be».
    fn rest_predicate(&self, i: usize) -> Option<usize> {
        let mut k = i + 1;
        if self.w.get(k).is_none_or(|x| x.lemma != "be") {
            return None;
        }
        k += 1;
        while k < self.w.len() && matches!(self.w[k].upos, UPos::DET | UPos::ADJ) {
            k += 1;
        }
        (k < self.w.len() && self.w[k].upos == UPos::NOUN).then_some(k)
    }
    /// Whether a noun is a counted thing (has a number or «some» on it) rather than an owner.
    fn counted(&self, k: usize) -> bool {
        self.kids[k].iter().any(|&c| num_val(&self.w[c].form).is_some() || matches!(self.lem(c).as_str(), "some" | "several"))
    }
    /// Thing: noun lemma with modifiers (without numbers and colour/article words).
    fn thing(&self, n: usize) -> (String, String) {
        let mut parts: Vec<(usize, String)> = self.kids[n]
            .iter()
            .copied()
            .filter(|&k| matches!(self.w[k].rel, Rel::Amod | Rel::Compound) && num_val(&self.w[k].form).is_none() && !ORD.contains(&self.lem(k).as_str()) && !matches!(self.lem(k).as_str(), "more" | "other" | "same" | "many" | "much" | "few" | "fewer" | "less") && (self.keep_new || self.lem(k) != "new"))
            .map(|k| (k, self.lem(k)))
            .collect();
        // nested compounds: «chocolate» → «chip» → «cookies»
        let nested: Vec<(usize, String)> = parts.iter().flat_map(|&(k, _)| self.kids_r(k, Rel::Compound)).map(|m| (m, self.lem(m))).collect();
        parts.extend(nested);
        // hyphenated «push - ups»
        if n >= 2 && self.w[n - 1].form == "-" && self.w[n - 2].upos == UPos::NOUN && !parts.iter().any(|p| p.0 == n - 2) {
            parts.push((n - 2, self.lem(n - 2)));
        }
        let mut head = self.lem(n);
        // «cups of flour» when the parse missed the «of» phrase: the noun right after «of»
        if self.w.get(n + 1).is_some_and(|x| x.form.to_lowercase() == "of") && !self.kids_r(n, Rel::Nmod).iter().any(|&k| self.kids_r(k, Rel::Case).iter().any(|&c| self.lem(c) == "of")) {
            if let Some(k) = (n + 2..self.w.len().min(n + 5)).take_while(|&k| !matches!(self.w[k].upos, UPos::VERB | UPos::AUX | UPos::PUNCT | UPos::ADP)).filter(|&k| self.w[k].upos == UPos::NOUN).last() {
                let head = self.lem(k);
                let mut parts: Vec<(usize, String)> = (n + 2..k).filter(|&m| matches!(self.w[m].upos, UPos::NOUN | UPos::ADJ)).map(|m| (m, self.lem(m))).collect();
                parts.push((k, head.clone()));
                return (parts.into_iter().map(|p| p.1).collect::<Vec<_>>().join(" "), head);
            }
        }
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
        out.push(Sent { w, kids, keep_new: false });
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
    /// the last subject that is a thing, not a person (the referent of «it»)
    last_it: String,
    /// values of the leaf variables: the numbers of the text in reading order, then compound leaves (rates)
    pub leaves: Vec<f64>,
    /// which numbers of the text each leaf is made of, and how it reads («12», «71 × 141»)
    leaf_src: Vec<Vec<usize>>,
    leaf_desc: Vec<String>,
    /// how many numbers of the text were read as leaves
    nums: usize,
    /// the first number of the question (numbers of the question are never distractors)
    q_leaf0: usize,
    /// every number of the text must be in the answer (a two-step reading through rates)
    all_used: bool,
    /// rates «71 flowers in each pot»: (thing counted, per what, the rate leaf)
    rates: Vec<(String, String, Lin)>,
    /// the answer as an expression over the leaves (after solving the unknowns)
    pub expr: Option<Lin>,
    /// relations between two things of an owner: thing A = thing B + value
    cmps: Vec<(String, String, String, Lin)>,
    /// relations between two times of an owner's thing: (owner, head, time A, time B, value): A = B + value
    time_cmps: Vec<(String, String, String, String, Lin)>,
    /// a whole and its parts (things): whole = Σ parts
    wholes: Vec<(String, Vec<String>)>,
}

impl QWorld {
    fn leaf(&mut self, v: f64) -> Lin {
        self.leaves.push(v);
        self.leaf_src.push(vec![self.nums]);
        self.leaf_desc.push(fmt_num(v));
        self.nums += 1;
        Lin::var(LEAF + self.leaves.len() - 1)
    }
    /// Totals at rates as states: «5 sets of tables, each set has 10 chairs» → the owner of the sets has 10 × 5
    /// chairs. Only for told, unchanged counts of the units; false if nothing was made.
    fn materialize(&mut self) -> bool {
        let mut made = false;
        for (x, y, r) in self.rates.clone() {
            let xh = x.rsplit(' ').next().unwrap_or(&x).to_string();
            if self.evs.iter().any(|e| e.unit == xh || e.head == xh) {
                continue;
            }
            let ys: Vec<Ev> = self.evs.iter().filter(|e| (e.head == y || e.unit == y) && !e.derived).cloned().collect();
            let owners: std::collections::BTreeSet<&str> = ys.iter().map(|e| e.owner.as_str()).collect();
            if ys.is_empty() || ys.iter().any(|e| e.kind != Kind::Set) || owners.len() != 1 {
                continue;
            }
            let c = ys.iter().fold(Lin::default(), |a, e| a.add(&e.val, 1.0));
            let Some(val) = self.compound(&r, &c, true) else { continue };
            let e0 = &ys[0];
            self.log.push(format!("t{}: {} has {x} at the rate per {y}", e0.t, e0.owner));
            let ev = Ev { t: e0.t, owner: e0.owner.clone(), thing: x.clone(), head: xh.clone(), unit: xh, val, kind: Kind::Set, verb: "have".into(), class: "have".into(), ord: e0.ord.clone(), other: None, ctx: e0.ctx.clone(), derived: false, outside: false };
            self.evs.push(ev);
            made = true;
        }
        if made {
            self.rates.clear();
        }
        made
    }
    /// Value and description of an expression over leaves (None if it has unknowns or a scaled leaf).
    fn closed(&self, e: &Lin) -> Option<(f64, String, Vec<usize>)> {
        if e.unknowns().next().is_some() || e.c.abs() > 1e-12 || e.x.values().any(|a| (a.abs() - 1.0).abs() > 1e-9) || e.x.is_empty() {
            return None;
        }
        let v = e.x.iter().map(|(k, a)| a * self.leaves[k - LEAF]).sum();
        let mut d = show_expr(e, &self.leaf_desc);
        if e.x.len() > 1 {
            d = format!("({d})");
        }
        let src = e.x.keys().flat_map(|k| self.leaf_src[k - LEAF].clone()).collect();
        Some((v, d, src))
    }
    /// A leaf made of two closed expressions: «71 × 141», «28 ÷ 7» (rates are products and quotients).
    fn compound(&mut self, a: &Lin, b: &Lin, mul: bool) -> Option<Lin> {
        let (va, da, sa) = self.closed(a)?;
        let (vb, db, sb) = self.closed(b)?;
        if !mul && vb.abs() < 1e-12 {
            return None;
        }
        self.leaves.push(if mul { va * vb } else { va / vb });
        self.leaf_src.push(sa.into_iter().chain(sb).collect());
        self.leaf_desc.push(format!("{da} {} {db}", if mul { "×" } else { "÷" }));
        Some(Lin::var(LEAF + self.leaves.len() - 1))
    }
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
        let tag = ord.clone().or_else(|| (0..s.w.len()).map(|i| s.lem(i)).find(|l| time_word(l)));
        // numbers and «some» at nouns
        let mut items: Vec<(usize, Option<f64>, usize)> = Vec::new(); // (number node, value, noun)
        for i in 0..s.w.len() {
            if let Some(v) = num_val(&s.w[i].form) {
                if s.w[i].form.chars().any(|c| c.is_ascii_digit()) || s.w[i].upos == UPos::NUM {
                    items.push((i, Some(v), s.counted_noun(i)));
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
            } else if s.lem(i) == "more" && s.w[i].head > i + 1 && s.w[s.w[i].head - 1].upos == UPos::NOUN && !s.kids[s.w[i].head - 1].iter().any(|&k| num_val(&s.w[k].form).is_some() || matches!(s.lem(k).as_str(), "some" | "several" | "many" | "much")) && i > 0 && !matches!(s.lem(i - 1).as_str(), "some" | "several" | "many" | "much") {
                // «and yet more pages of reading homework»: more of a thing, an untold amount
                items.push((i, None, s.w[i].head - 1));
            } else if s.lem(i) == "rest" && i > 0 && s.lem(i - 1) == "the" || matches!(s.lem(i).as_str(), "other" | "others") && s.w[i].upos != UPos::ADJ && matches!(s.w[i].rel.base(), Rel::Nsubj | Rel::Obj) {
                // «the rest stay home», «others suggested bacon»: the remaining part, an unknown
                items.push((i, None, i));
            }
        }
        // a sentence without numbers names the topic («Danny collects bottle caps.»): later «ones» are bottle caps
        if items.is_empty() {
            if let Some(o) = (0..s.w.len()).find(|&i| s.w[i].head == 0).and_then(|r| s.kids_r(r, Rel::Obj).first().copied()).filter(|&o| s.w[o].upos == UPos::NOUN) {
                *last_thing = s.thing(o);
            }
        }
        let starts: Vec<usize> = items.iter().map(|x| x.0).collect();
        for (i, v, n) in items {
            let mut ctx = s.ctx_nouns(i, n, &starts);
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
            // «6 of them got tired of waiting and left»: «get» with an adjective is «become», the change is the
            // coordinated verb
            if s.lem(vb) == "get" && s.kids[vb].iter().any(|&k| s.w[k].upos == UPos::ADJ && matches!(s.w[k].rel.base(), Rel::Xcomp | Rel::Obj | Rel::Advmod)) {
                if let Some(c) = s.kids_r(vb, Rel::Conj).into_iter().find(|&c| s.w[c].upos == UPos::VERB) {
                    vb = c;
                }
            }
            // a denied event did not happen («didn't recycle 2 of them») — the world does not read negation
            if s.kids[vb].iter().any(|&k| matches!(s.lem(k).as_str(), "not" | "never" | "n't")) {
                if let Some(x) = v {
                    self.unread.push(x);
                }
                self.log.push("negated event".into());
                continue;
            }
            // «… and 3 people got off»: a clause misparsed as a noun with a reduced relative; after «and» the verb
            // right behind the counted noun is the clause's own verb
            if n != i && (i >= 1 && s.w[i - 1].upos == UPos::CCONJ || i >= 2 && s.w[i - 2].upos == UPos::CCONJ) {
                if let Some(c) = s.kids_r(n, Rel::Acl).into_iter().find(|&c| c > n && c <= n + 2 && s.w[c].upos == UPos::VERB) {
                    vb = c;
                }
            }
            // the verb's own object, when it is not the counted noun («played tag with 5 kids»)
            for k in s.kids_r(vb, Rel::Obj) {
                if k != n && k != i && s.w[k].upos == UPos::NOUN && !s.counted(k) && !ctx.contains(&s.lem(k)) {
                    ctx.push(s.lem(k));
                }
            }
            // phrasal verb: «got on», «got off», «gave away» — the particle counts when level 1 knows the pair
            let verb = {
                let mut l = s.lem(vb);
                // a copular clause («8 green peaches are in the basket»): the verb is «be»
                if s.w[vb].upos != UPos::VERB && s.kids[vb].iter().any(|&k| s.w[k].rel == Rel::Cop && s.lem(k) == "be") {
                    l = "be".into();
                }
                let prt = s.kids[vb].iter().copied().find(|&k| s.w[k].rel == Rel::CompoundPrt).or_else(|| (vb + 1 < s.w.len() && matches!(s.lem(vb + 1).as_str(), "on" | "off" | "up" | "down" | "away" | "back" | "out")).then_some(vb + 1));
                if let Some(p) = prt {
                    let pl = format!("{l}_{}", s.lem(p));
                    if s.w[p].rel == Rel::CompoundPrt || global::verb_class(&pl).is_some() {
                        l = pl;
                    }
                }
                l
            };
            // the class from level 1; verbs that are concepts of increase («put in», «join»: things.md) gain
            let class = global::verb_class(&verb).map(|c| c.0.to_string()).unwrap_or_else(|| {
                let inc = ["put_in", "join_group", "learn_gain"].iter().any(|c| global::concept_words(c).contains(&verb.as_str()));
                if inc { "get".into() } else { String::new() }
            });
            // number time stamp: the nearest ordinal word or part of day to the right within the clause, otherwise to the left
            let tag_of = |k: usize| -> Option<String> {
                let l = s.lem(k);
                time_word(&l).then_some(l)
            };
            // — but an ordinal adjective belongs to its noun: «the first chapter is 35 pages long» (the subject's)
            let subj_n = s.subj(vb);
            let ord_of_np = (0..s.w.len()).find(|&k| ORD.contains(&s.lem(k).as_str()) && s.w[k].head > 0 && (Some(s.w[k].head - 1) == subj_n || s.w[k].head - 1 == n)).map(|k| s.lem(k));
            let item_tag = ord_of_np.or_else(|| (i + 1..s.w.len().min(i + 7)).take_while(|&k| num_val(&s.w[k].form).is_none()).find_map(tag_of).or_else(|| (i.saturating_sub(5)..i).rev().find_map(tag_of))).or_else(|| tag.clone());
            let tag = &item_tag;
            // existential «there were 7 roses» (not the place adverb «bought 3 more balloons there»)
            let expl = s.kids[vb].iter().any(|&k| s.lem(k) == "there" && (s.w[k].rel == Rel::Expl || k < vb)) || s.has_word(&["there"]) && s.lem(vb) == "be";
            // «There were 78 dollars in Olivia's wallet»: the place's possessor owns them
            let place_owner = || -> Option<String> {
                let place = (0..s.w.len()).find(|&k| s.w[k].upos == UPos::NOUN && s.kids_r(k, Rel::Case).iter().any(|&c| s.lem(c) == "in"))?;
                let p = s.kids[place].iter().copied().find(|&c| s.w[c].rel == Rel::NmodPoss)?;
                if s.w[p].upos == UPos::PROPN {
                    Some(s.lem(p))
                } else if pron(&s.lem(p)) && !root_subj.is_empty() {
                    Some(root_subj.clone())
                } else {
                    None
                }
            };
            // «There are 46 rulers in the drawer»: the place holds them
            let place_noun = || -> Option<String> {
                let k = (0..s.w.len()).find(|&k| s.w[k].upos == UPos::NOUN && k != n && s.kids_r(k, Rel::Case).iter().any(|&c| matches!(s.lem(c).as_str(), "in" | "on")))?;
                Some(s.lem(k))
            };
            let owner = match s.subj(vb) {
                // otherwise the place, or the one owner the story has of this thing («there were 159 dollars left»)
                _ if expl => place_owner().or_else(place_noun).or_else(|| {
                    let hd = if n != i && s.w[n].upos == UPos::NOUN { s.lem(n) } else { last_thing.1.clone() };
                    let os: std::collections::BTreeSet<&str> = self.evs.iter().filter(|e| e.head == hd && !e.outside && !e.derived && e.kind != Kind::Rel).map(|e| e.owner.as_str()).collect();
                    (os.len() == 1).then(|| os.into_iter().next().unwrap().to_string())
                }).unwrap_or_default(),
                // «Steven who has 14 more» — «who» → the name the clause is attached to
                Some(k) if matches!(s.lem(k).as_str(), "who" | "which" | "that") && s.w[vb].head > 0 => {
                    // the antecedent: the clause's head, or the nearest name before «who» when the parse missed it
                    let h = s.w[vb].head - 1;
                    if s.lem(k) == "who" && s.w[h].upos != UPos::PROPN {
                        (0..k).rev().find(|&m| s.w[m].upos == UPos::PROPN).map(|m| s.lem(m)).unwrap_or_else(|| s.lem(h))
                    } else {
                        s.lem(h)
                    }
                }
                // «it» is the last thing-like subject, not a person («Robin's hair was 14 inches long. It grew by 8»)
                Some(k) if s.lem(k) == "it" && !self.last_it.is_empty() => self.last_it.clone(),
                Some(k) if pron(&s.lem(k)) => {
                    if !root_subj.is_empty() {
                        root_subj.clone()
                    } else if !last_subj.is_empty() {
                        last_subj.clone()
                    } else {
                        // no antecedent yet: the name in the same sentence («For Gwen's birthday she received»)
                        (0..s.w.len()).find(|&m| s.w[m].upos == UPos::PROPN).map(|m| s.lem(m)).unwrap_or_default()
                    }
                }
                Some(k) if s.w[k].upos == UPos::PROPN || s.lem(k) == "there" => {
                    let o = if s.lem(k) == "there" { String::new() } else { s.lem(k) };
                    *last_subj = o.clone();
                    o
                }
                // the subject is the thing itself («5 were eaten», «403 more girls joined», «3 birds and 2 storks»)
                // — unless a place the story counts is named («3 new apples grew on the tree»)
                // («418 more girls joined the school»: the group joined)
                Some(k) if k == n || k == i || s.counted(k) || matches!(s.lem(k).as_str(), "some" | "several") => s
                    .kids_r(vb, Rel::Obl)
                    .into_iter()
                    .filter(|&o| s.kids_r(o, Rel::Case).iter().any(|&c| matches!(s.lem(c).as_str(), "on" | "in")))
                    .chain(s.kids_r(vb, Rel::Obj))
                    .find(|&o| self.evs.iter().any(|e| e.owner == s.lem(o)))
                    .map(|o| s.lem(o))
                    .or_else(place_owner)
                    .unwrap_or_else(|| last_subj.clone()),
                Some(k) => {
                    // a measure of a possessed thing belongs to the possessor («Robin's hair was 14 inches long»,
                    // «Marco's strawberries weighed 15 pounds»: units from level 1, things.md)
                    let unit_n = n != i && global::concept_words("measure_unit").contains(&s.lem(n).as_str());
                    let poss = s.kids[k].iter().copied().find(|&c| s.w[c].rel == Rel::NmodPoss && s.w[c].upos == UPos::PROPN);
                    let o = match poss {
                        Some(p) if unit_n => s.lem(p),
                        _ => s.lem(k),
                    };
                    *last_subj = o.clone();
                    if s.w[k].upos == UPos::NOUN {
                        self.last_it = o.clone();
                    }
                    o
                }
                None => last_subj.clone(),
            };
            if self.main.is_empty() && !owner.is_empty() {
                self.main = owner.clone();
            }
            let dollar = s.w[i].form.starts_with('$') || i > 0 && s.w[i - 1].form == "$";
            let (thing, head) = if dollar {
                ("dollar".to_string(), "dollar".to_string())
            } else if n != i && s.w[n].upos == UPos::NOUN && !matches!(s.lem(n).as_str(), "left" | "more" | "one" | "ones") {
                s.thing(n)
            } else if let Some(p) = (s.lem(i) == "rest").then(|| s.rest_predicate(i)).flatten() {
                // «the rest are boys»: the predicate names the rest
                s.thing(p)
            } else {
                last_thing.clone()
            };
            // (a later bare number refers to a thing counted with a number, not to «some pies»)
            if !thing.is_empty() && (v.is_some() || last_thing.0.is_empty()) {
                *last_thing = (thing.clone(), head.clone());
            }
            let unit = if dollar {
                head.clone()
            } else if n != i && s.w[n].upos == UPos::NOUN {
                s.lem(n)
            } else {
                // a bare number («lost 24 of them») counts in the unit of its thing («33 pieces of candy»)
                self.evs.iter().rev().find(|e| e.thing == thing).map(|e| e.unit.clone()).unwrap_or_else(|| head.clone())
            };
            let val = match v {
                Some(x) => self.leaf(x),
                None => {
                    self.nvars += 1;
                    Lin::var(self.nvars - 1)
                }
            };
            let more = s.kids[i].iter().chain(s.kids[n].iter()).any(|&k| s.lem(k) == "more");
            // «left»: a state when it follows the counted noun of another verb («has 210 cards left») or is a
            // predicate («are left», «has left over»); a departure when it is the clause's own verb («5 customers left»)
            let left_verb = s.w[vb].form.to_lowercase() == "left";
            let left_state = left_verb && vb > i && (0..i).any(|k| s.lem(k) == "have" && matches!(s.w[k].upos, UPos::VERB | UPos::AUX))
                || left_verb && (s.kids[vb].iter().any(|&k| matches!(s.w[k].rel.base(), Rel::Aux | Rel::Cop) && matches!(s.lem(k).as_str(), "have" | "be")) || s.w[vb].head > 0 && matches!(s.lem(s.w[vb].head - 1).as_str(), "have" | "be"))
                || (n + 1..s.w.len().min(n + 3)).any(|k| k != vb && s.w[k].form.to_lowercase() == "left");
            // first mention of this owner's thing («Allan brought 5 balloons and Jake brought 3 balloons»)
            // (an ownerless clause — «If you read 19 of the books» — does not start a new pile of a counted thing)
            let seen_thing = self.evs.iter().any(|e| (e.owner == owner || owner.is_empty()) && (e.thing == thing || e.head == head));
            let class = if left_state { "have".to_string() } else { class };
            // «bought a candy bar for $ 6»: money given for something is paid, it leaves the buyer
            let price = (dollar || global::concept_words("money").contains(&head.as_str())) && class == "get" && (i.saturating_sub(2)..i).any(|k| s.lem(k) == "for");
            let class = if price { "lose".to_string() } else { class };
            // «Tim took 25 rulers from the drawer»: taking from a source is getting
            let class = if class == "have" && s.lem(vb) == "take" && s.kids[vb].iter().chain(s.kids[n].iter()).any(|&k| matches!(s.w[k].rel.base(), Rel::Obl | Rel::Nmod) && s.kids_r(k, Rel::Case).iter().any(|&c| s.lem(c) == "from")) { "get".to_string() } else { class };
            let cond = cond || left_state;
            // «4 more apples than Jackie», «7 fewer peaches than Steven» — a relation between owners
            // a rate: «Each pot has 71 flowers», «10 seeds in each flower bed», «$ 5 off each jersey», «6 shirts a
            // minute», «4 pages per chapter» — not an event but a quantity per unit (level 1, units.md: rate)
            if let (Some(y), Some(_), false) = (s.per_noun(i, n), v, self.in_question) {
                if !thing.is_empty() {
                    self.log.push(format!("rate: {v:?} {thing} per {y}"));
                    self.rates.push((thing.clone(), y, val.clone()));
                    continue;
                }
            }
            // the comparative word lies between the number and «than» (or right after the number): «more», a lesser
            // word (level 1, units.md), or any other comparative form («farther»)
            let lesser = global::concept_words("lesser");
            let than_k = (i + 1..s.w.len()).find(|&k| s.lem(k) == "than");
            let next_num = starts.iter().copied().find(|&k| k > i).unwrap_or(s.w.len());
            let cmp_end = than_k.unwrap_or(s.w.len()).min(next_num).min(i + 6);
            let cmp_form = |k: usize| s.w[k].form.to_lowercase();
            let fewer = s.kids[i].iter().chain(s.kids[n].iter()).any(|&k| matches!(s.lem(k).as_str(), "few" | "fewer" | "less"))
                || (i + 1..cmp_end).any(|k| lesser.contains(&cmp_form(k).as_str()));
            let more = more
                || (i + 1..cmp_end).any(|k| cmp_form(k) == "more")
                || than_k.is_some() && (i + 1..cmp_end).any(|k| matches!(s.w[k].tag, Tag::JJR | Tag::RBR) && !lesser.contains(&cmp_form(k).as_str()));
            // the other side: a name, or a noun the story already counts as an owner («than the grasshopper»)
            let than_other = than_k
                .and_then(|k| (k + 1..s.w.len()).take_while(|&m| num_val(&s.w[m].form).is_none()).find(|&m| s.w[m].upos == UPos::PROPN || s.w[m].upos == UPos::NOUN && self.evs.iter().any(|e| e.owner == s.lem(m))))
                .map(|m| s.lem(m));
            let than_after = than_k.is_some();
            // «8 more kids on monday than on tuesday»: a relation between two times of the same activity
            let than_time = than_k.filter(|_| more || fewer).and_then(|k| (k + 1..s.w.len().min(k + 4)).find(|&m| global::relation(&s.lem(m), "week").is_some() || global::relation(&s.lem(m), "order").is_some())).map(|m| s.lem(m));
            if let (Some(tb), Some(ta)) = (than_time.clone(), tag.clone().filter(|ta| Some(ta) != than_time.as_ref())) {
                let val = if fewer { val.scale(-1.0) } else { val.clone() };
                // a time the story did not count yet is an unknown of the same activity
                for tg in [&ta, &tb] {
                    if !self.evs.iter().any(|e| e.head == head && e.owner == owner && e.ord.as_deref() == Some(tg.as_str())) {
                        self.nvars += 1;
                        let x = Lin::var(self.nvars - 1);
                        self.evs.push(Ev { t, owner: owner.clone(), thing: thing.clone(), head: head.clone(), unit: unit.clone(), val: x, kind: Kind::Set, verb: verb.clone(), class: class.clone(), ord: Some(tg.clone()), other: None, ctx: ctx.clone(), derived: false, outside: false });
                    }
                }
                self.log.push(format!("t{t}: {owner} {thing} [{ta}] = [{tb}] {} {v:?}", if fewer { "−" } else { "+" }));
                self.time_cmps.push((owner.clone(), head.clone(), ta, tb, val));
                continue;
            }
            // «228 more girls than boys», «6 more pages of reading homework than math homework»: a relation between
            // two things (a constraint for the solver); a thing never counted before becomes an unknown
            let than_thing = than_k.filter(|_| than_other.is_none() && (more || fewer)).and_then(|k| {
                let mut m = k + 1;
                while m < s.w.len() && matches!(s.w[m].upos, UPos::DET | UPos::ADJ) {
                    m += 1;
                }
                while m + 1 < s.w.len() && s.w[m].upos == UPos::NOUN && s.w[m + 1].upos == UPos::NOUN {
                    m += 1;
                }
                (m < s.w.len() && s.w[m].upos == UPos::NOUN && !time_word(&s.lem(m)) && !thing.is_empty()).then(|| s.thing(m))
            });
            if let Some((tb, _)) = than_thing.filter(|(tb, _)| *tb != thing) {
                let val = if fewer { val.scale(-1.0) } else { val.clone() };
                if !self.evs.iter().any(|e| e.thing == thing && (e.owner == owner || owner.is_empty())) {
                    self.nvars += 1;
                    let x = Lin::var(self.nvars - 1);
                    self.evs.push(Ev { t, owner: owner.clone(), thing: thing.clone(), head: head.clone(), unit: unit.clone(), val: x, kind: Kind::Set, verb: verb.clone(), class: "have".into(), ord: tag.clone(), other: None, ctx: ctx.clone(), derived: false, outside: false });
                }
                self.log.push(format!("t{t}: {thing} = {tb} {} {v:?}", if fewer { "−" } else { "+" }));
                self.cmps.push((owner.clone(), thing.clone(), tb, val));
                continue;
            }
            if (more || fewer) && than_after && than_other.is_none() {
                // «12 more kids on monday than on tuesday», «than those that suggested bacon»: other side unknown
                if let Some(x) = v {
                    self.unread.push(x);
                }
                self.log.push("comparison with an unread side".into());
                continue;
            }
            if (more || fewer) && than_other.is_some() {
                let val = if fewer { val.add(&Lin::default(), 1.0).add(&val, -2.0) } else { val.clone() };
                let thing = if thing.is_empty() { last_thing.0.clone() } else { thing.clone() };
                self.log.push(format!("t{t}: {owner} = {:?} {} {v:?} {thing}", than_other, if fewer { "−" } else { "+" }));
                self.evs.push(Ev { t, owner: owner.clone(), thing, head: head.clone(), val, kind: Kind::Rel, verb: verb.clone(), class: class.clone(), ord: tag.clone(), other: than_other, ctx: ctx.clone(), unit: unit.clone(), derived: false, outside: false });
                continue;
            }
            // «a total of 10» — a condition on the sum
            // «a total of 10», «has 828521 kids in all», «… altogether» — a condition on the sum of all parts
            let total = (i.saturating_sub(4)..i).any(|k| s.lem(k) == "total") || s.has_word(&["together"]) && class == "have"
                || (i + 1..s.w.len()).any(|k| matches!(s.lem(k).as_str(), "altogether" | "combined") || s.lem(k) == "all" && k > 0 && s.lem(k - 1) == "in" || s.lem(k) == "total" && k > 0 && s.lem(k - 1) == "in");
            let t = if initial_s && class == "have" { 0 } else { t };
            let kind = match class.as_str() {
                _ if total => Kind::Is,
                "have" if initial_s => Kind::Set,
                _ if more && !matches!(class.as_str(), "have" | "give" | "lose") => Kind::Add,
                "get" | "make" => Kind::Add,
                "lose" => Kind::Sub,
                "give" => Kind::Sub,
                "have" if cond => Kind::Is,
                "have" if more => Kind::Add,
                "have" => Kind::Set,
                _ if more => Kind::Add,
                // «21 children were riding on the bus»: first mention of a thing with an unknown verb describes the state
                _ if !seen_thing && (v.is_some() || matches!(class.as_str(), "" | "move")) => Kind::Set,
                // a verb without a quantity class, but another time or place than before: a separate part
                // («played tag with 5 kids on tuesday … with 6 kids on monday», «flew to africa … to asia»)
                _ if v.is_some() && matches!(class.as_str(), "" | "move") && {
                    let prior: Vec<&Ev> = self.evs.iter().filter(|e| e.owner == owner && (e.thing == thing || e.head == head)).collect();
                    !prior.is_empty() && prior.iter().all(|e| e.ord.is_some() && tag.is_some() && e.ord != *tag || e.verb == verb && !e.ctx.is_empty() && !ctx.is_empty() && e.ctx.iter().all(|c| !ctx.contains(c)))
                } => Kind::Set,
                // «the rest stay home»: the remaining part of a thing already counted
                _ if matches!(s.lem(i).as_str(), "rest" | "other" | "others") => Kind::Set,
                _ => {
                    // unknown verb: «joined», «grew» without a class — do not invent
                    if let Some(x) = v {
                        self.unread.push(x);
                    }
                    self.log.push(format!("verb «{verb}» has no class"));
                    continue;
                }
            };
            // a second state of the same owner's same thing at the same time («had 39 cards, and 9 were torn»): a
            // part or a restatement the world cannot place
            if kind == Kind::Set && v.is_some() && !total && self.evs.iter().any(|e| e.kind == Kind::Set && e.owner == owner && e.thing == thing && e.ord == *tag && !e.derived) {
                if let Some(x) = v {
                    self.unread.push(x);
                }
                self.log.push(format!("second state of {owner}'s {thing}"));
                continue;
            }
            // a condition on a thing never mentioned before is just its state («If there are 13 red peaches»)
            let kind = if kind == Kind::Is && !total && !left_state && !self.evs.iter().any(|e| e.thing == thing && (e.owner == owner || e.owner.is_empty() || owner.is_empty())) { Kind::Set } else { kind };
            // harvest: «the tree had 7 apples … picked 4» — picking takes from the counted stock
            let from = |k: usize| matches!(s.w[k].rel.base(), Rel::Obl | Rel::Nmod) && s.kids_r(k, Rel::Case).iter().any(|&c| s.lem(c) == "from");
            // («bought 24 of Sally's baseball cards»: the possessor is the source)
            let of_poss = (s.w.get(i + 1).is_some_and(|x| x.form == "of")).then(|| (i + 2..s.w.len().min(i + 5)).find(|&k| s.w[k].upos == UPos::PROPN && s.w.get(k + 1).is_some_and(|x| x.form == "'s"))).flatten();
            let from_src = s.kids[vb].iter().chain(s.kids[n].iter()).copied().find(|&k| from(k)).or(of_poss);
            let (mut owner, mut kind, mut outside) = (owner, kind, false);
            // «cut 8 roses from her garden»: a removal from the source, not a loss of the subject's own
            if class == "lose" && kind == Kind::Sub {
                if let Some(r) = from_src {
                    owner = if pron(&s.lem(r)) { root_subj.clone() } else { s.lem(r) };
                    outside = true;
                }
            }
            if class == "get" && kind == Kind::Add && from_src.is_none() && global::concept_words("harvest").contains(&verb.as_str()) {
                // (a stock that grows somewhere: another owner's — the tree's — or one told with its place, «in his
                // garden»; a florist's own roses picked are more roses)
                if let Some(st) = self.evs.iter().rev().find(|e| (e.head == head || e.thing == thing) && matches!(e.kind, Kind::Set | Kind::Is) && !e.derived && e.other.is_none()).filter(|st| st.owner != owner || !st.ctx.is_empty()) {
                    owner = st.owner.clone();
                    kind = Kind::Sub;
                }
            }
            // «Jason washed cars over the weekend and now has 33 dollars»: a told state that the known events do not
            // reach means an untold change (an unknown) just before it
            if kind == Kind::Is && !total && v.is_some() {
                let (th, hd) = (thing.clone(), head.clone());
                let f = move |x: &Ev| x.thing == th || x.head == hd;
                let st = if owner.is_empty() { self.state(None, &f, t) } else { self.state(Some(&owner), &f, t) };
                if st.is_some_and(|st| st.unknowns().next().is_none()) {
                    self.nvars += 1;
                    let x = Lin::var(self.nvars - 1);
                    self.log.push(format!("t{t}: {owner} untold change of {thing}"));
                    self.evs.push(Ev { t, owner: owner.clone(), thing: thing.clone(), head: head.clone(), val: x, kind: Kind::Add, verb: verb.clone(), class: CHANGE.into(), ord: tag.clone(), other: None, ctx: ctx.clone(), unit: unit.clone(), derived: false, outside: false });
                }
            }
            self.log.push(format!("t{t}: {owner} {verb}({class}) {thing} {kind:?} {v:?}{}", tag.as_ref().map(|o| format!(" [{o}]")).unwrap_or_default()));
            self.evs.push(Ev { t, owner: owner.clone(), thing: thing.clone(), head: head.clone(), val: val.clone(), kind, verb: verb.clone(), class: class.clone(), ord: tag.clone(), other: if total { Some("total".into()) } else { None }, ctx: ctx.clone(), unit: unit.clone(), derived: false, outside });
            // «A school has 485 pupils. There are 232 girls and the rest are boys.»: the rest completes the parts of the
            // whole counted just before (a thing other than the parts)
            if s.lem(i) == "rest" && v.is_none() {
                let parts: Vec<String> = self.evs.iter().filter(|e| e.t == t && !e.derived && e.kind == Kind::Set).map(|e| e.thing.clone()).collect();
                if let Some(wh) = self.evs.iter().rev().find(|e| e.t + 1 == t && e.kind == Kind::Set && !e.derived && !parts.iter().any(|p| p.ends_with(&e.head))) {
                    self.log.push(format!("t{t}: {} = {parts:?}", wh.thing));
                    self.wholes.push((wh.thing.clone(), parts));
                }
            }
            // «picked 2 apples from her tree» — the source decreases
            if class == "get" && kind == Kind::Add {
                if let Some(r) = from_src {
                    let src = if pron(&s.lem(r)) { root_subj.clone() } else { s.lem(r) };
                    // a source the story did not count before is outside its piles («collected 148 dollars from an atm»)
                    let outside = !self.evs.iter().any(|e| e.owner == src);
                    self.evs.push(Ev { t, owner: src, thing: thing.clone(), head: head.clone(), val: val.clone(), kind: Kind::Sub, verb: verb.clone(), class: "lose".into(), ord: tag.clone(), other: None, ctx: ctx.clone(), unit: unit.clone(), derived: true, outside });
                }
            }
            // give: recipient
            if class == "give" {
                // «to Jeff» on the number itself («some more to Jeff»), else on the verb; or the indirect object
                let to = |k: usize| s.w[k].rel.base() == Rel::Obl && s.kids_r(k, Rel::Case).iter().any(|&c| s.lem(c) == "to");
                let r = s.kids[i].iter().chain(s.kids[n].iter()).copied().find(|&k| to(k)).or_else(|| s.kids[vb].iter().copied().find(|&k| to(k))).or_else(|| s.kids_r(vb, Rel::Iobj).first().copied())
                    // «Annie gives Angela 4 more»: a name as the object of «give» is the receiver
                    .or_else(|| s.kids_r(vb, Rel::Obj).into_iter().find(|&k| k != n && k != i && s.w[k].upos == UPos::PROPN));
                if let Some(r) = r {
                    let rcv = if pron(&s.lem(r)) { root_subj.clone() } else { s.lem(r) };
                    self.evs.push(Ev { t, owner: rcv, thing, head, val, kind: Kind::Add, verb, class, ord: tag.clone(), other: None, ctx: ctx.clone(), unit: unit.clone(), derived: true, outside: false });
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
                // a relation told at one time does not carry over changes of the other owner after it
                let moved = |b: &str, t: usize| self.evs.iter().any(|x| x.owner == b && thing(x) && matches!(x.kind, Kind::Add | Kind::Sub) && x.t > t);
                if let Some(e) = self.evs.iter().find(|e| e.kind == Kind::Rel && e.owner == o && thing(e)) {
                    if e.other.as_deref().is_some_and(|b| moved(b, e.t)) || moved(o, e.t) {
                        return None;
                    }
                    if let Some(b) = self.state_d(e.other.as_deref(), thing, upto, depth + 1) {
                        return Some(b.add(&e.val, 1.0));
                    }
                }
                if let Some(e) = self.evs.iter().find(|e| e.kind == Kind::Rel && e.other.as_deref() == Some(o) && thing(e)) {
                    if moved(&e.owner, e.t) || moved(o, e.t) {
                        return None;
                    }
                    if let Some(a) = self.state_d(Some(&e.owner), thing, upto, depth + 1) {
                        return Some(a.add(&e.val, -1.0));
                    }
                }
            }
        }
        // initial state without «had N» (one unknown per owner and thing, numbered by the first event): unknown when
        // the story loses before it has, or when a told state can fix it; a first gain otherwise starts the
        // possession («Paul got a box of 531 crayons»), and making creates from nothing («Haley grew 9 trees»)
        let mut st: Option<Lin> = None;
        if let (Some((k, e)), Some(o)) = (self.initial_ev(owner, thing), owner) {
            let told_state = self.evs.iter().any(|x| x.kind == Kind::Is && x.owner == o && x.other.is_none() && thing(x));
            if e.kind == Kind::Sub || e.kind == Kind::Add && e.class != "make" && told_state {
                st = Some(Lin::var(1000 + k));
            }
        }
        for e in &self.evs {
            if e.t > upto || !thing(e) || owner.is_some_and(|o| e.owner != o) || owner.is_none() && e.outside || e.kind == Kind::Is || e.kind == Kind::Rel {
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

    /// The first event of an owner's thing when no «had N» is told at or before it (that is the initial state).
    fn initial_ev(&self, owner: Option<&str>, thing: &dyn Fn(&Ev) -> bool) -> Option<(usize, &Ev)> {
        let mine = |x: &Ev| thing(x) && owner.is_none_or(|o| x.owner == o);
        let (k, e) = self.evs.iter().enumerate().find(|(_, x)| mine(x) && x.kind != Kind::Is)?;
        let set_before = self.evs.iter().any(|x| mine(x) && x.kind == Kind::Set && x.t <= e.t);
        (matches!(e.kind, Kind::Add | Kind::Sub) && !set_before).then_some((k, e))
    }

    /// Solve the unknowns from «state = number» conditions: each unknown becomes an expression over the leaves.
    fn solve(&self) -> BTreeMap<usize, Lin> {
        let mut eqs: Vec<Lin> = Vec::new();
        for e in &self.evs {
            if e.kind != Kind::Is {
                continue;
            }
            // the exact thing when the story has it («ripe apples» apart from «unripe apples»), else by head
            let th = e.thing.clone();
            let hd = e.head.clone();
            let exact = self.evs.iter().any(|x| x.kind != Kind::Is && x.thing == th);
            let f = move |x: &Ev| x.thing == th || !exact && x.head == hd;
            let upto = if e.other.as_deref() == Some("total") { usize::MAX } else { e.t };
            let st = if e.other.as_deref() == Some("total") || e.owner.is_empty() { self.state(None, &f, upto) } else { self.state(Some(&e.owner), &f, upto).or_else(|| self.state(None, &f, upto)) };
            if let Some(st) = st {
                eqs.push(st.add(&e.val, -1.0));
            }
        }
        for (o, a, b, val) in &self.cmps {
            let (a, b) = (a.clone(), b.clone());
            let o = (!o.is_empty()).then_some(o.as_str());
            let sa = self.state(o, &move |x: &Ev| x.thing == a, usize::MAX);
            let sb = self.state(o, &move |x: &Ev| x.thing == b, usize::MAX);
            if let (Some(sa), Some(sb)) = (sa, sb) {
                eqs.push(sa.add(&sb, -1.0).add(val, -1.0));
            }
        }
        for (o, hd, ta, tb, val) in &self.time_cmps {
            let at = |tg: &str| -> Option<Lin> {
                let es: Vec<&Ev> = self.evs.iter().filter(|e| e.owner == *o && e.head == *hd && e.ord.as_deref() == Some(tg) && !e.derived && matches!(e.kind, Kind::Set | Kind::Add | Kind::Sub)).collect();
                (!es.is_empty()).then(|| es.iter().fold(Lin::default(), |a, e| a.add(&e.val, if e.kind == Kind::Sub { -1.0 } else { 1.0 })))
            };
            if let (Some(a), Some(b)) = (at(ta), at(tb)) {
                eqs.push(a.add(&b, -1.0).add(val, -1.0));
            }
        }
        for (wh, parts) in &self.wholes {
            let whc = wh.clone();
            let sw = self.state(None, &move |x: &Ev| x.thing == whc, usize::MAX);
            let sp: Option<Vec<Lin>> = parts.iter().map(|p| { let p = p.clone(); self.state(None, &move |x: &Ev| x.thing == p, usize::MAX) }).collect();
            if let (Some(sw), Some(sp)) = (sw, sp) {
                eqs.push(sp.iter().fold(sw, |a, b| a.add(b, -1.0)));
            }
        }
        let mut sol: BTreeMap<usize, Lin> = BTreeMap::new();
        // one unknown at a time: an equation with a single unknown left
        for _ in 0..4 {
            for q in &eqs {
                let q = q.subst(&sol);
                let rest: Vec<usize> = q.unknowns().collect();
                if rest.len() == 1 {
                    let u = rest[0];
                    let a = q.x[&u];
                    let mut r = q.clone();
                    r.x.remove(&u);
                    sol.insert(u, r.scale(-1.0 / a));
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

/// Dependency trees of a text, one token per line (for debugging the reader).
pub fn trees(ann: &Annotator, text: &str) -> String {
    let mut out = String::new();
    for s in parse(ann, text) {
        for (i, x) in s.w.iter().enumerate() {
            out.push_str(&format!("{:>3} {:<14} {:<12} {:?} {} {:?}\n", i + 1, x.form, x.lemma, x.upos, x.head, x.rel));
        }
        out.push('\n');
    }
    out
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

fn answer_parsed(mut bs: Vec<Sent>, mut qs: Vec<Sent>) -> (Option<f64>, QWorld) {
    let old = bs.iter().chain(qs.iter()).any(|s| s.has_word(&["old"]));
    for s in bs.iter_mut().chain(qs.iter_mut()) {
        s.keep_new = old;
    }
    let mut w = QWorld::default();
    let e = query(&mut w, &bs, &qs);
    let v = e.as_ref().filter(|e| e.unknowns().next().is_none()).map(|e| {
        let mut v = e.c;
        for (k, a) in &e.x {
            v += a * w.leaves[k - LEAF];
        }
        v
    });
    if let (Some(e), Some(v)) = (&e, v) {
        w.log.push(format!("answer: {} = {}", show_expr(e, &w.leaf_desc), fmt_num(v)));
    } else if e.is_some() {
        w.log.push("answer: unknowns left".into());
    }
    w.expr = e.filter(|_| v.is_some());
    (v, w)
}

/// One side of «how many more A than B».
#[derive(Clone, Debug, Default, PartialEq)]
struct Desc {
    thing: Option<(String, String)>,
    owner: Option<String>,
    verb: Option<String>,
    tags: Vec<String>,
    ctx: Vec<String>,
}

impl Desc {
    fn inherit(&mut self, o: &Desc) {
        if self.thing.is_none() {
            self.thing = o.thing.clone();
        }
        if self.owner.is_none() {
            self.owner = o.owner.clone();
        }
        if self.verb.is_none() {
            self.verb = o.verb.clone();
        }
        if self.ctx.is_empty() && self.tags.is_empty() {
            self.ctx = o.ctx.clone();
        }
    }
}

/// Read a side of a comparison: the verb (not an infinitive of purpose, not an auxiliary), the owner (a name the
/// story knows or a pronoun), the thing (the first other plain noun), time words, and context nouns (in a
/// prepositional phrase or a purpose clause).
fn side_desc(q: &Sent, r: std::ops::Range<usize>, known: &dyn Fn(&str) -> bool) -> Desc {
    let mut d = Desc::default();
    let mut purpose = false;
    // nouns already inside the thing («bottle» of «bottle caps», «flour» of «cups of flour»)
    let mut used = vec![false; q.w.len()];
    for i in r.clone() {
        if used[i] {
            continue;
        }
        let l = q.lem(i);
        let f = q.w[i].form.to_lowercase();
        if l == "to" && q.w[i].upos == UPos::PART {
            purpose = true;
        }
        if time_word(&l) {
            d.tags.push(l);
            continue;
        }
        match q.w[i].upos {
            // perfect «have you read»: the participle is the verb
            UPos::VERB if l == "have" && (i + 1..r.end).any(|k| q.w[k].tag == Tag::VBN && q.w[k].form.to_lowercase() != "left") => {}
            // «does he have left» — the state left over
            UPos::VERB if f == "left" && (0..q.w.len()).any(|k| q.lem(k) == "have") => {}
            UPos::VERB if !purpose && d.verb.is_none() && !matches!(l.as_str(), "do" | "be") => d.verb = Some(l),
            UPos::AUX if l == "have" && d.verb.is_none() && !q.kids[i].is_empty() => d.verb = Some(l),
            UPos::PRON if pron(&l) && d.owner.is_none() && matches!(q.w[i].rel.base(), Rel::Nsubj) => d.owner = Some(l),
            // a known owner inside a prepositional phrase is a place or source («from her mom», «at the atm»)
            UPos::PROPN | UPos::NOUN if (known(&l) || known(&f)) && q.kids_r(i, Rel::Case).iter().any(|&c| q.lem(c) != "than") => d.ctx.push(l),
            UPos::PROPN | UPos::NOUN if known(&l) || known(&f) => d.owner = Some(if known(&l) { l } else { f }),
            UPos::NOUN if matches!(l.as_str(), "more" | "less") => {}
            // a compound modifier waits for its head noun
            UPos::NOUN if q.w[i].rel == Rel::Compound && q.w[i].head > i + 1 && r.contains(&(q.w[i].head - 1)) => {}
            UPos::NOUN => {
                let in_pp = q.kids_r(i, Rel::Case).iter().any(|&c| q.lem(c) != "than") || purpose;
                if in_pp {
                    d.ctx.push(l);
                } else if d.thing.is_none() {
                    d.thing = Some(q.thing(i));
                    for k in q.kids_r(i, Rel::Nmod) {
                        used[k] = true;
                    }
                }
            }
            _ => {}
        }
    }
    d
}

/// Strict gate (precision first): why the world should abstain, or None if it trusts its answer.
/// It must have read every number; the answer must be a positive sum/difference of at least two numbers of the
/// text, each used once (an answer equal to one number of the text is a misreading in word problems).
pub fn abstain(w: &QWorld, v: Option<f64>) -> Option<&'static str> {
    let Some(v) = v else { return Some("no answer") };
    let Some(e) = &w.expr else { return Some("no expression") };
    if !w.unread.is_empty() {
        return Some("unread numbers");
    }
    if v <= 1e-9 {
        return Some("answer not positive");
    }
    // counts of things are whole: a fraction means a misread rate or share
    if (v - v.round()).abs() > 1e-6 {
        return Some("not a whole number");
    }
    if e.x.values().any(|a| (a.abs() - 1.0).abs() > 1e-9) {
        return Some("a number used twice");
    }
    let mut src: Vec<usize> = e.x.keys().filter(|&&k| k >= LEAF).flat_map(|k| w.leaf_src[k - LEAF].clone()).collect();
    if src.len() < 2 || e.c.abs() > 1e-9 {
        return Some("fewer than two numbers");
    }
    src.sort();
    // (in a two-step reading through rates a count may serve twice: «5 sets of tables, 10 chairs each — how many
    // more chairs than tables» = 10 × 5 − 5; within one leaf kind a number is still used once)
    let dup = if w.all_used {
        let part = |compound: bool| -> bool {
            let mut v: Vec<usize> = e.x.keys().filter(|&&k| k >= LEAF && (w.leaf_src[k - LEAF].len() > 1) == compound).flat_map(|k| w.leaf_src[k - LEAF].clone()).collect();
            v.sort();
            v.windows(2).any(|p| p[0] == p[1])
        };
        part(true) || part(false)
    } else {
        src.windows(2).any(|p| p[0] == p[1])
    };
    if dup {
        return Some("a number used twice");
    }
    if (w.q_leaf0..w.nums).any(|j| !src.contains(&j)) {
        return Some("a number of the question unused");
    }
    if w.all_used && (0..w.nums).any(|j| !src.contains(&j)) {
        return Some("a number unused in a two-step reading");
    }
    None
}

fn fmt_num(v: f64) -> String {
    if (v - v.round()).abs() < 1e-9 { format!("{}", v.round()) } else { format!("{v}") }
}

/// «12 + 14» from the expression over leaves.
fn show_expr(e: &Lin, leaves: &[String]) -> String {
    let mut s = String::new();
    for (k, a) in &e.x {
        let x = if *k >= LEAF { leaves[k - LEAF].clone() } else { "?".into() };
        let sign = if *a < 0.0 { " − " } else if s.is_empty() { "" } else { " + " };
        let m = if (a.abs() - 1.0).abs() < 1e-9 { String::new() } else { format!("{}·", a.abs()) };
        s.push_str(&format!("{sign}{m}{x}"));
    }
    if e.c.abs() > 1e-12 || s.is_empty() {
        s.push_str(&format!(" + {}", e.c));
    }
    s
}

/// Questions about rates (units: «each», «per»): the total of a thing at a rate times the count of units; the
/// rate asked from a total and a count («how many books in each bookshelf»); the count of units from a total and
/// the rate («how many flower beds»). Anything else with a rate — abstain.
fn rate_query(w: &mut QWorld, q: &Sent, sol: &BTreeMap<usize, Lin>) -> Option<Lin> {
    let ql: Vec<String> = (0..q.w.len()).map(|i| q.lem(i)).collect();
    let how = (0..q.w.len()).find(|&i| (ql[i] == "many" || ql[i] == "much") && i > 0 && ql[i - 1] == "how")?;
    // one product or quotient answers only a plain question: a comparison, a remainder or a need on top of the
    // rate is another step («How many chairs are left unoccupied?», «How many more chairs than tables?»)
    if ql.iter().chain((0..q.w.len()).map(|i| q.w[i].form.to_lowercase()).collect::<Vec<_>>().iter()).any(|l| matches!(l.as_str(), "more" | "than" | "fewer" | "less" | "left" | "remain" | "extra" | "need" | "still")) {
        w.log.push("rate with another step — not read".into());
        return None;
    }
    let qn = (how + 1..q.w.len()).take_while(|&k| matches!(q.w[k].upos, UPos::NOUN | UPos::ADJ | UPos::PROPN) || ql[k] == "more").filter(|&k| q.w[k].upos == UPos::NOUN).find(|&k| q.w[k].rel != Rel::Compound);
    let money = global::concept_words("money");
    // the asked thing: the noun after «how many» (its unit too: «slices» of «slices of pizza»), or money for
    // «how much … cost / make / earn»
    let (qthing, qhead, qunit) = match qn {
        Some(k) => {
            let (t, h) = q.thing(k);
            (t, h, ql[k].clone())
        }
        None if ql[how] == "much" => ("dollar".to_string(), "dollar".to_string(), "dollar".to_string()),
        None => return None,
    };
    let is_money = |h: &str| money.contains(&h);
    let same = |a: &str, h: &str| a == h || is_money(h) && is_money(a);
    let asked = |e_head: &str, unit: &str| same(e_head, &qhead) || same(unit, &qhead) || same(e_head, &qunit) || same(unit, &qunit);
    let qtags: Vec<String> = ql.iter().filter(|l| time_word(l)).cloned().collect();
    // the unit asked per: «in each bookshelf», «each pack», «per day»
    let q_per = (0..q.w.len()).find(|&k| matches!(ql[k].as_str(), "each" | "every" | "per") && k > how).and_then(|k| {
        let y = if ql[k] == "per" { (k + 1..q.w.len()).find(|&m| q.w[m].upos == UPos::NOUN) } else if q.w[k].head > 0 && q.w[q.w[k].head - 1].upos == UPos::NOUN { Some(q.w[k].head - 1) } else { (k + 1..q.w.len().min(k + 3)).find(|&m| q.w[m].upos == UPos::NOUN) };
        y.map(|m| ql[m].clone())
    });
    let mods: Vec<String> = qthing.split(' ').filter(|m| *m != qhead).map(String::from).collect();
    // «pieces of candies»: the units counted are those of candy
    let unit_mods: Vec<String> = if qunit != qhead { mods.iter().cloned().chain(std::iter::once(qhead.clone())).collect() } else { mods.clone() };
    let rates = w.rates.clone();
    // a count or total of a thing told in the story (a total, or the told amounts — not both), narrowed by the
    // question's modifiers and time words; a thing told only as a rate per another is that rate × its count
    fn amount(w: &mut QWorld, h: &str, mods: &[String], qtags: &[String], rates: &[(String, String, Lin)], sol: &BTreeMap<usize, Lin>, depth: usize) -> Option<Lin> {
        let money = global::concept_words("money");
        // a category counts its members when the story has no such noun («each person» — his 4 friends)
        let cat = global::concept_words(h);
        let direct = w.evs.iter().any(|e| e.head == h || e.unit == h);
        let same = |a: &str| a == h || money.contains(&h) && money.contains(&a) || !direct && cat.contains(&a);
        let total = w.evs.iter().rev().find(|e| e.kind == Kind::Is && e.other.as_deref() == Some("total") && (same(&e.head) || same(&e.unit))).map(|e| e.val.clone());
        let mut es: Vec<&Ev> = w.evs.iter().filter(|e| (same(&e.head) || same(&e.unit)) && !e.derived && !e.outside && matches!(e.kind, Kind::Set | Kind::Add | Kind::Sub)).collect();
        if total.is_some() {
            return if es.is_empty() { total.map(|t| t.subst(sol)) } else { None };
        }
        for m in mods {
            if es.iter().any(|e| e.ctx.iter().any(|c| c == m) || e.thing.split(' ').any(|p| p == m)) {
                es.retain(|e| e.ctx.iter().any(|c| c == m) || e.thing.split(' ').any(|p| p == m));
            }
        }
        if es.iter().any(|e| e.ord.as_ref().is_some_and(|t| qtags.contains(t))) {
            es.retain(|e| e.ord.as_ref().is_some_and(|t| qtags.contains(t)));
        }
        if !es.is_empty() {
            // units are counted whatever happens to them («gave crackers to his 4 friends» — 4 friends); a total
            // of things is what is left of them
            let unit_count = es.iter().all(|e| e.kind == Kind::Sub);
            return Some(es.iter().fold(Lin::default(), |a, e| a.add(&e.val, if e.kind == Kind::Sub && !unit_count { -1.0 } else { 1.0 })).subst(sol));
        }
        // «544 pots in each of the 10 gardens»: pots = 544 × 10
        if depth < 2 {
            if let [(_, z, r)] = rates.iter().filter(|(x, _, _)| x.rsplit(' ').next() == Some(h)).collect::<Vec<_>>().as_slice() {
                let (z, r) = (z.clone(), r.clone());
                let c = amount(w, &z, &[], qtags, rates, sol, depth + 1)?;
                return w.compound(&r, &c, true);
            }
        }
        None
    }
    if let Some(y) = &q_per {
        // the rate asked: total ÷ count
        if rates.iter().any(|(x, ry, _)| asked(x.rsplit(' ').next().unwrap_or(x), "") && ry == y) {
            return None;
        }
        // (a total changed by an untold amount or told again later is not the shared total)
        if w.evs.iter().any(|e| asked(&e.head, &e.unit) && (e.class == CHANGE || e.kind == Kind::Is && e.other.is_none())) {
            return None;
        }
        let t = amount(w, &qhead, &mods, &qtags, &rates, sol, 0).or_else(|| amount(w, &qunit, &mods, &qtags, &rates, sol, 0))?;
        let c = amount(w, y, &[], &qtags, &rates, sol, 0)?;
        w.log.push(format!("rate asked: {qhead} per {y} = total ÷ count"));
        return w.compound(&t, &c, false);
    }
    // «How many books and magazines in total?» at rates per the same unit: (23 + 61) × 29
    let conj: Vec<String> = qn.map(|k| q.kids_r(k, Rel::Conj).into_iter().filter(|&c| q.w[c].upos == UPos::NOUN).map(|c| ql[c].clone()).collect()).unwrap_or_default();
    if !conj.is_empty() {
        let mut heads = vec![qhead.clone()];
        heads.extend(conj);
        let per: Vec<Option<&(String, String, Lin)>> = heads.iter().map(|h| rates.iter().find(|(x, _, _)| x.rsplit(' ').next() == Some(h.as_str()))).collect();
        let Some(Some((_, y, _))) = per.first().cloned() else { return None };
        if per.iter().any(|r| r.is_none_or(|(_, ry, _)| ry != y)) || w.evs.iter().any(|e| heads.contains(&e.head) && !e.derived) {
            return None;
        }
        let r = per.iter().flatten().fold(Lin::default(), |a, (_, _, r)| a.add(r, 1.0));
        let y = y.clone();
        let c = amount(w, &y, &[], &qtags, &rates, sol, 0)?;
        w.log.push(format!("totals at rates: {heads:?} per {y} × count of {y}"));
        return w.compound(&r, &c, true);
    }
    let mut for_x: Vec<&(String, String, Lin)> = rates.iter().filter(|(x, _, _)| asked(x.rsplit(' ').next().unwrap_or(x), "")).collect();
    // the question's modifiers choose among rates («how many green peaches»); otherwise rates of different kinds
    // per the same unit add up («19 red peaches and 4 green peaches in each basket»)
    if for_x.len() > 1 && !mods.is_empty() {
        for_x.retain(|(x, _, _)| mods.iter().all(|m| x.split(' ').any(|p| p == m)));
    }
    let kinds: std::collections::BTreeSet<&str> = for_x.iter().map(|(x, _, _)| x.as_str()).collect();
    let units: std::collections::BTreeSet<&str> = for_x.iter().map(|(_, y, _)| y.as_str()).collect();
    if !for_x.is_empty() && units.len() == 1 && kinds.len() == for_x.len() {
        // the total at a rate: rate × count of units (the thing itself must not be counted otherwise)
        if w.evs.iter().any(|e| (same(&e.unit, &qhead) || same(&e.unit, &qunit)) && !e.derived) {
            return None;
        }
        let y = for_x[0].1.clone();
        let r = for_x.iter().fold(Lin::default(), |a, (_, _, r)| a.add(r, 1.0));
        let mods = &unit_mods;
        let c = amount(w, &y, &mods, &qtags, &rates, sol, 0)?;
        w.log.push(format!("total at a rate: {qhead} per {y} × count of {y}"));
        return w.compound(&r, &c, true);
    }
    // the count of units: total ÷ rate (several rates per this unit — the question's words choose one)
    let mut for_y: Vec<&(String, String, Lin)> = rates.iter().filter(|(_, y, _)| *y == qhead || *y == qunit).collect();
    if for_y.len() > 1 {
        for_y.retain(|(x, _, _)| x.split(' ').any(|p| ql.iter().any(|l| l == p) && p != x.rsplit(' ').next().unwrap_or("")));
    }
    if for_y.len() > 1 {
        let told: Vec<&(String, String, Lin)> = for_y.iter().copied().filter(|(x, _, _)| {
            let xh = x.rsplit(' ').next().unwrap_or(x);
            w.evs.iter().any(|e| (e.head == xh || e.unit == xh) && !e.derived)
        }).collect();
        for_y = told;
    }
    if let [(x, _, r)] = for_y.as_slice() {
        let xh = x.rsplit(' ').next().unwrap_or(x).to_string();
        let xmods: Vec<String> = x.split(' ').filter(|m| *m != xh).map(String::from).collect();
        let r = r.clone();
        let t = amount(w, &xh, &xmods, &qtags, &rates, sol, 0)?;
        w.log.push(format!("count of units: {qhead} = total {xh} ÷ rate"));
        return w.compound(&t, &r, false);
    }
    w.log.push("rate: question not understood".into());
    None
}

/// Read the story and the question; the answer as an expression (unknowns substituted where solved).
fn query(w: &mut QWorld, bs: &[Sent], qs: &[Sent]) -> Option<Lin> {
    let (mut ls, mut lt) = (String::new(), (String::new(), String::new()));
    for (t, s) in bs.iter().enumerate() {
        w.read_sent(s, t, &mut ls, &mut lt);
    }
    let Some(q) = qs.last() else { return None };
    // rates and shares are not this world's type (steps handles them)
    let all_l: Vec<String> = bs.iter().chain(qs.iter()).flat_map(|s| (0..s.w.len()).map(|i| s.lem(i)).collect::<Vec<_>>()).collect();
    if let Some(x) = all_l.iter().find(|l| matches!(l.as_str(), "times" | "twice" | "half" | "average" | "rate")) {
        w.log.push(format!("rate/share «{x}» — not my type"));
        return None;
    }
    // «sold all but 4 of them» — an exception the world does not read
    if all_l.windows(2).any(|p| p[0] == "all" && p[1] == "but") {
        w.log.push("«all but» — not read".into());
        return None;
    }
    // «how many dozens of dollars» — a unit conversion the world does not read
    if (0..q.w.len()).any(|i| matches!(q.lem(i).as_str(), "dozen" | "dozens")) {
        w.log.push("conversion — not read".into());
        return None;
    }
    // only «how many / how much / how far / how long» questions are queries to this world
    if !(0..q.w.len()).any(|i| q.lem(i) == "how") {
        w.log.push("not a «how» question".into());
        return None;
    }
    // numbers from the question itself («How many will she have left if she gives away 64 games?») are events too
    let t_end = bs.len();
    w.in_question = true;
    w.q_leaf0 = w.nums;
    let ev0 = w.evs.len();
    for s in qs {
        if (0..s.w.len()).any(|i| num_val(&s.w[i].form).is_some() && s.w[i].form.chars().any(|c| c.is_ascii_digit())) {
            w.read_sent(s, t_end, &mut ls, &mut lt);
        }
    }
    // a state told in the question about a thing the story already counts («how many good carrots if 38 were
    // bad?») asks for the other part — not read; a one-sentence problem tells its states in the question
    if !bs.is_empty() && w.evs[ev0..].iter().any(|e| matches!(e.kind, Kind::Set | Kind::Is) && e.class != CHANGE && w.evs[..ev0].iter().any(|b| b.head == e.head)) {
        w.log.push("a state told in the question — not read".into());
        return None;
    }
    let t_end = t_end + 1;
    let sol = w.solve();
    let mut sol = sol;
    if !w.rates.is_empty() || (0..q.w.len()).any(|i| matches!(q.lem(i).as_str(), "each" | "every" | "per")) {
        let r = rate_query(w, q, &sol);
        // a rate with another step on top («How many more chairs than tables?»): the totals at the rates become
        // states of the story, and the question is read as usual — but then every number must be used
        if r.is_some() || !w.log.last().is_some_and(|l| l.starts_with("rate with another step")) || !w.materialize() {
            return r;
        }
        w.all_used = true;
        sol = w.solve();
    }
    let ql: Vec<String> = (0..q.w.len()).map(|i| q.lem(i)).collect();
    let qf: Vec<String> = (0..q.w.len()).map(|i| q.w[i].form.to_lowercase()).collect();
    let has = |x: &str| ql.iter().any(|l| l == x) || qf.iter().any(|l| l == x);
    // the question's thing: the noun after how many / much
    // (right after «how many», over adjectives and «more»: «How much more did Edward spend on books» has none)
    // (the head of a compound: «action figures» → figures)
    let how = (0..q.w.len()).find(|&i| (ql[i] == "many" || ql[i] == "much") && i > 0 && ql[i - 1] == "how");
    let compound_head = |mut k: usize| {
        while k + 1 < q.w.len() && q.w[k + 1].upos == UPos::NOUN && q.w[k].rel == Rel::Compound {
            k += 1;
        }
        k
    };
    let span = |i: usize| (i + 1..q.w.len()).take_while(|&k| matches!(q.w[k].upos, UPos::NOUN | UPos::ADJ | UPos::ADV | UPos::PROPN | UPos::CCONJ) || ql[k] == "more" || ql[k] == ",");
    let qthing_i = how.and_then(|i| span(i).find(|&k| q.w[k].upos == UPos::NOUN)).map(compound_head);
    // «How many green and yellow peaches»: coordinated adjectives name several things with one head
    let adj_conj: Vec<String> = match (how, qthing_i) {
        (Some(i), Some(n)) if (i + 1..n).any(|k| q.w[k].upos == UPos::CCONJ) => (i + 1..n).filter(|&k| q.w[k].upos == UPos::ADJ && !matches!(ql[k].as_str(), "more" | "many")).map(|k| format!("{} {}", ql[k], ql[n])).collect(),
        _ => Vec::new(),
    };
    let qthing: Option<(String, String)> = match adj_conj.first() {
        Some(t) => qthing_i.map(|n| (t.clone(), ql[n].clone())),
        None => qthing_i.map(|k| q.thing(k)),
    };
    // verbs of the question; after do-support («did Fred earn») the next known verb counts even if mistagged
    // (only the question clause: «If she gave away 3 and bought 48 more, how many would she have?» asks a state)
    let q_from = how.map(|h| h - 1).unwrap_or(0);
    let mut qverb = (q_from..q.w.len()).filter(|&i| q.w[i].upos == UPos::VERB).map(|i| q.lem(i)).collect::<Vec<_>>();
    if let Some(d) = (q_from..q.w.len()).find(|&i| ql[i] == "do") {
        if let Some(k) = (d + 1..q.w.len().min(d + 5)).find(|&k| q.w[k].upos != UPos::VERB && global::verb_class(&ql[k]).is_some() && !matches!(q.w[k].upos, UPos::PROPN | UPos::PRON | UPos::DET)) {
            qverb.push(ql[k].clone());
        }
    }
    // the question's subject: not the counted thing («How many marbles does Josh have» — Josh)
    let qnoun = how.and_then(|i| (i + 1..q.w.len()).find(|&k| q.w[k].upos == UPos::NOUN)).map(compound_head);
    let qsubj_i = (0..q.w.len()).find(|&i| matches!(q.w[i].rel, Rel::Nsubj) && Some(i) != qnoun && !matches!(q.lem(i).as_str(), "many" | "much"));
    let qsubj = qsubj_i.map(|i| q.lem(i));
    // a named subject the story never gave anything to («How many cakes would baker still have?» when the
    // baker only appears as «Baker's friend»): the world does not know whose state is asked
    if let Some(k) = qsubj_i {
        let l = q.lem(k);
        let in_story = bs.iter().any(|s| (0..s.w.len()).any(|i| s.lem(i) == l));
        if !pron(&l) && in_story && !w.evs.iter().any(|e| e.owner == l || e.other.as_deref() == Some(l.as_str())) && !w.evs.iter().any(|e| e.head == l || e.thing == l) {
            w.log.push(format!("question subject «{l}» owns nothing in the story"));
            return None;
        }
    }
    let known = |n: &str| w.evs.iter().any(|e| e.owner == n || e.other.as_deref() == Some(n));
    // a person of the story who owns nothing there («How many cakes would baker still have?» after «Baker's
    // friend bought 137 cakes from him»): the world cannot tell whose state is asked
    let person = |l: &str| bs.iter().any(|s| (0..s.w.len()).any(|i| s.lem(i) == l && (s.w[i].upos == UPos::PROPN || s.w[i].rel == Rel::NmodPoss)));
    let whose = |k: usize| matches!(q.w[k].rel.base(), Rel::Nsubj | Rel::Conj) || q.w[k].rel == Rel::NmodPoss;
    if let Some(k) = (0..q.w.len()).find(|&k| matches!(q.w[k].upos, UPos::NOUN | UPos::PROPN) && whose(k) && Some(k) != qnoun && !known(&ql[k]) && !time_word(&ql[k]) && person(&ql[k]) && !w.evs.iter().any(|e| e.head == ql[k] || e.ctx.contains(&ql[k]))) {
        w.log.push(format!("asked about «{}» who owns nothing", ql[k]));
        return None;
    }
    // an existential question («How many boys are there in that school?») asks the existential owner when the
    // story has the thing there
    let exist_q = has("there") && qthing.as_ref().is_some_and(|(_, hd)| w.evs.iter().any(|e| e.owner.is_empty() && e.head == *hd && !e.derived));
    let story_owners: std::collections::BTreeSet<&str> = w.evs.iter().filter(|e| !e.derived && !e.outside && !e.owner.is_empty() && e.kind != Kind::Rel).map(|e| e.owner.as_str()).collect();
    // «How many rulers are now in the drawer?» — a place the story counts things in
    let place_q = (0..q.w.len()).find(|&k| q.w[k].upos == UPos::NOUN && Some(k) != qnoun && q.kids_r(k, Rel::Case).iter().any(|&c| matches!(ql[c].as_str(), "in" | "on")) && known(&ql[k])).map(|k| ql[k].clone());
    let owner_q: Option<String> = match &qsubj {
        _ if place_q.is_some() && (has("there") || qsubj_i.is_some_and(|k| Some(k) == qnoun) || qsubj.is_none()) => place_q.clone(),
        // «how many pieces do they have left?» after «Debby had 32 … her sister had 42»: all of them
        Some(s) if matches!(s.as_str(), "they" | "we") && story_owners.len() > 1 => None,
        Some(s) if pron(s) => Some(w.main.clone()),
        Some(s) if known(s) => Some(s.clone()),
        _ if exist_q => Some(String::new()),
        // lowercase name («did paco have») or subject not found: a known name among the question's words
        _ => qf.iter().chain(ql.iter()).find(|x| !x.is_empty() && known(x) && x.as_str() != "").cloned().or_else(|| qsubj.as_ref().filter(|s| pron(s)).map(|_| w.main.clone())),
    };
    // «How many roses are there in the vase now?» — the existential owner of «There were 10 roses in the vase»
    let owner_q = owner_q.or_else(|| (has("there") && w.evs.iter().any(|e| e.owner.is_empty() && !e.derived && e.kind != Kind::Rel) && w.evs.iter().any(|e| !e.owner.is_empty() && !e.derived)).then(String::new));
    let evs_things: Vec<String> = w.evs.iter().map(|e| e.thing.clone()).collect();
    let first_head = w.evs.first().map(|e| e.head.clone()).unwrap_or_default();
    // «regular soda and diet soda» — several things together
    let qconj: Vec<String> = qnoun.map(|k| {
        let mut v: Vec<String> = adj_conj.iter().skip(1).cloned().collect();
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
    // one thing: the exact thing if the story has it («salty cookie»), otherwise by head («bottles» = all bottles)
    let money = global::concept_words("money");
    let by_head = |hd: &str| w.evs.iter().any(|e| e.head == hd || e.unit == hd || e.thing.ends_with(&format!(" {hd}")));
    let thing_one = |qt: &Option<(String, String)>| -> Box<dyn Fn(&Ev) -> bool> {
        match qt.clone() {
            // «how much money» — dollars, cents (level 1, units.md)
            Some((_, hd)) if money.contains(&hd.as_str()) => Box::new(move |e: &Ev| money.contains(&e.head.as_str())),
            Some((th, hd)) if th != hd && evs_things.contains(&th) => Box::new(move |e: &Ev| e.thing == th),
            // a modifier the story never uses («how many good games» when the story only says «games»): the
            // question asks for a part the world did not read
            Some((th, hd)) if th != hd && !th.split(' ').filter(|m| *m != hd).all(|m| w.evs.iter().any(|e| e.thing.split(' ').any(|p| p == m) || e.ctx.iter().any(|c| c == m))) => Box::new(|_: &Ev| false),
            Some((_, hd)) if by_head(&hd) => Box::new(move |e: &Ev| e.head == hd || e.unit == hd || e.thing.ends_with(&format!(" {hd}"))),
            // a category the story has only members of («people» = parents, pupils, teachers; level 1, things.md)
            Some((_, hd)) if !global::concept_words(&hd).is_empty() => {
                let ws = global::concept_words(&hd);
                Box::new(move |e: &Ev| ws.contains(&e.head.as_str()))
            }
            Some((_, hd)) => Box::new(move |e: &Ev| e.head == hd || e.unit == hd || e.thing.ends_with(&format!(" {hd}"))),
            // «How much did they make?» — the thing of the first event
            None => {
                let fh = first_head.clone();
                Box::new(move |e: &Ev| e.head == fh)
            }
        }
    };
    let thing_f = |qt: &Option<(String, String)>| -> Box<dyn Fn(&Ev) -> bool> {
        if !qconj.is_empty() && qt.as_ref().is_some_and(|x| evs_things.contains(&x.0)) {
            let mut set = qconj.clone();
            set.push(qt.as_ref().unwrap().0.clone());
            return Box::new(move |e: &Ev| set.contains(&e.thing));
        }
        thing_one(qt)
    };
    let ev = |l: &Lin| Some(l.subst(&sol));
    let past_q = has("did") || (0..q.w.len()).any(|i| q.w[i].upos == UPos::VERB && (q.w[i].tag == Tag::VBD || q.w[i].tag == Tag::VBN && q.kids_r(i, Rel::Aux).iter().all(|&a| q.lem(a) == "have")));
    // requirements («the recipe calls for 12 cups», «how many more does she need to add») are not modelled
    if has("need") {
        w.log.push("a requirement question — not my type".into());
        return None;
    }
    // 1. «how many more A than B»: each side is a description (thing, owner, verb, time, context nouns); what one
    // side leaves out it shares with the other («more emails in the morning than in the afternoon»)
    if let Some(ti) = ql.iter().position(|l| l == "than") {
        if has("more") || has("less") || has("fewer") {
            let mut da = side_desc(q, 0..ti, &known);
            let mut db = side_desc(q, ti + 1..q.w.len(), &known);
            // «spend on books than pens»: a bare noun after «than» mirrors the left side's prepositional phrase
            if qthing.is_none() && da.thing.is_none() && !da.ctx.is_empty() && db.ctx.is_empty() {
                if let Some((t, _)) = db.thing.take() {
                    db.ctx.push(t.rsplit(' ').next().unwrap_or("").to_string());
                }
            }
            if da.thing.is_none() {
                da.thing = qthing.clone();
            }
            da.inherit(&db);
            db.inherit(&da);
            for d in [&mut da, &mut db] {
                if d.owner.as_deref().is_some_and(|o| matches!(o, "they" | "we")) && story_owners.len() != 1 {
                    d.owner = None;
                } else if d.owner.as_deref().is_some_and(pron) {
                    d.owner = Some(w.main.clone());
                }
                // a place the question names («in the series») holds both sides
                if d.owner.is_none() {
                    d.owner = place_q.clone();
                }
            }
            let side = |d: &Desc| -> Option<Lin> {
                let tf = thing_one(&d.thing);
                let owner_ok = |e: &Ev| d.owner.as_ref().is_none_or(|o| e.owner == *o);
                let tag_ok = |e: &Ev| d.tags.is_empty() || e.ord.as_ref().is_some_and(|t| d.tags.contains(t));
                let cls = d.verb.as_deref().and_then(global::verb_class).map(|c| c.0).unwrap_or("");
                // a verb with a quantity class, or a past verb naming parts of the story («did Zachary do»);
                // a present verb without a class asks for the state («are sitting on the fence»)
                let event_verb = d.verb.as_ref().filter(|v| !matches!(v.as_str(), "have" | "be") && (!matches!(cls, "have" | "" | "move") || past_q && w.evs.iter().any(|e| e.verb == **v)));
                if let Some(v) = event_verb {
                    let exact = w.evs.iter().any(|e| e.verb == *v && tf(e));
                    let mut cand: Vec<&Ev> = w
                        .evs
                        .iter()
                        .filter(|e| !matches!(e.kind, Kind::Is | Kind::Rel) && !e.derived)
                        .filter(|e| if exact { e.verb == *v } else { e.class == cls })
                        .filter(|e| e.kind != Kind::Set || e.class.is_empty() || e.class == "move")
                        .filter(|e| tf(e) && owner_ok(e) && tag_ok(e))
                        .collect();
                    // context nouns narrow the events when the story has them («to build the tower»)
                    for c in &d.ctx {
                        if cand.iter().any(|e| e.ctx.contains(c)) {
                            cand.retain(|e| e.ctx.contains(c));
                        }
                    }
                    if !cand.is_empty() {
                        return Some(cand.iter().fold(Lin::default(), |s, e| s.add(&e.val, 1.0)));
                    }
                    // an untold change of the owner answers for a gain or a loss verb («spend at the supermarket»)
                    let ch: Vec<&Ev> = w.evs.iter().filter(|e| e.class == CHANGE && tf(e) && (owner_ok(e) || e.owner.is_empty())).collect();
                    if let ([e], true) = (ch.as_slice(), matches!(cls, "get" | "make" | "lose" | "give")) {
                        return Some(e.val.scale(if matches!(cls, "get" | "make") { 1.0 } else { -1.0 }));
                    }
                    if d.owner.as_ref().is_some_and(|o| w.evs.iter().any(|e| e.kind == Kind::Rel && (e.owner == *o || e.other.as_deref() == Some(o.as_str())))) {
                        return w.state(d.owner.as_deref(), &*tf, t_end);
                    }
                    return None;
                }
                // a state: time words select the told parts («the first chapter»); context nouns are only places
                if !d.tags.is_empty() {
                    let tagged = |e: &Ev| tf(e) && e.ord.as_ref().is_some_and(|t| d.tags.contains(t));
                    return w.evs.iter().any(|e| tagged(e)).then(|| w.state(d.owner.as_deref(), &tagged, t_end)).flatten();
                }
                w.state(d.owner.as_deref(), &*tf, t_end)
            };
            if da == db {
                w.log.push(format!("comparison: sides are the same {da:?}"));
                return None;
            }
            let (a, b) = (side(&da), side(&db));
            w.log.push(format!("difference: {da:?} vs {db:?}"));
            if let (Some(a), Some(b)) = (a, b) {
                let d = a.add(&b, -1.0);
                return ev(&d).map(|x| if has("less") || has("fewer") { x.scale(-1.0) } else { x });
            }
            w.log.push("comparison: side not found".into());
            return None;
        }
    }
    if has("than") {
        w.log.push("«than» without «more»/«fewer» — not read".into());
        return None;
    }
    // «How many push-ups did Zachary and David do altogether?» (no change verb): the coordinated owners' states
    let q_owners: Vec<String> = qsubj_i.map(|k| std::iter::once(k).chain(q.kids_r(k, Rel::Conj)).map(|m| q.lem(m)).filter(|o| known(o)).collect()).unwrap_or_default();
    if q_owners.len() >= 2 && qverb.iter().all(|v| global::verb_class(v).is_none_or(|c| matches!(c.0, "have" | "move"))) {
        let tf = thing_f(&qthing);
        let mut sum = Lin::default();
        for o in &q_owners {
            sum = sum.add(&w.state(Some(o), &*tf, t_end)?, 1.0);
        }
        w.log.push(format!("state of {q_owners:?}"));
        return ev(&sum);
    }
    // 2. sum of events of a class: «how many … lost or given away», «how many did he sell»
    // motion («before starting to jog») is not a change of quantity
    let ev_classes: Vec<String> = qverb.iter().filter_map(|v| global::verb_class(v).map(|c| c.0.to_string())).filter(|c| c != "have" && c != "move").collect();
    let did_have0 = has("did") && qverb.iter().any(|v| v == "have" || v == "weigh") && w.evs.iter().any(|e| e.kind == Kind::Is);
    let initial_q0 = did_have0 || has("initially") || has("originally") || has("begin") || has("beginning") || has("start") || has("before") || ql.windows(2).any(|x| x == ["at", "first"]);
    // «How many cards did Nell give to Jeff?» — sum on the recipient's side
    let q_to = (0..q.w.len()).find(|&i| q.w[i].rel.base() == Rel::Obl && q.w[i].upos == UPos::PROPN && q.kids_r(i, Rel::Case).iter().any(|&c| ql[c] == "to")).map(|i| ql[i].clone());
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
            return ev(&sum);
        }
        w.log.push(format!("nothing given to {r}"));
        return None;
    }
    // context nouns of the question: not the asked thing, not an owner, not a time word
    let qctx: Vec<String> = (0..q.w.len())
        .filter(|&k| q.w[k].upos == UPos::NOUN && Some(k) != qnoun && qnoun.is_none_or(|n| q.w[k].head != n + 1 && q.w[n].head != k + 1) && !known(&ql[k]) && !time_word(&ql[k]))
        .map(|k| ql[k].clone())
        .collect();
    // «How many customers left?» — «left» as the question's own verb is a departure, not the state «left over»
    let left_dep_q = (0..q.w.len()).any(|i| qf[i] == "left" && q.w[i].upos == UPos::VERB && q.w[i].head == 0) && !has("have") && !has("be");
    // a verb of the question that names a part of the story exactly («how many kids stayed home»)
    // — asked in the past («stayed», «did … play»), not the present state («are sitting»)
    let unclassed_q = past_q && qverb.iter().any(|v| global::verb_class(v).is_none_or(|c| c.0 == "move") && w.evs.iter().any(|e| e.verb == *v));
    if !initial_q0 && (!ev_classes.is_empty() || unclassed_q) && (!has("left") || left_dep_q) && !has("now") && !has("remain") {
        let mut sum = Lin::default();
        let mut any = false;
        let tf = thing_f(&qthing);
        // time stamp in the question («in the evening», «in the first week»)
        let qtags: Vec<String> = ql.iter().filter(|l| time_word(l)).cloned().collect();
        // first the question's verb itself («did he sell»), only then the whole class (give = give, sell, …)
        let set_ok = |e: &Ev| e.kind != Kind::Set || e.class.is_empty() || e.class == "move";
        let by_verb = w.evs.iter().any(|e| qverb.contains(&e.verb) && e.kind != Kind::Is && set_ok(e) && e.kind != Kind::Rel && tf(e));
        let mut cand: Vec<&Ev> = Vec::new();
        for e in &w.evs {
            if e.derived {
                continue; // the other side of a transfer
            }
            let verb_ok = if by_verb { qverb.contains(&e.verb) } else { ev_classes.contains(&e.class) };
            if verb_ok && !matches!(e.kind, Kind::Is | Kind::Rel) && (e.kind != Kind::Set || by_verb && set_ok(e)) && tf(e) && owner_q.as_ref().is_none_or(|o| e.owner == *o) && (qtags.is_empty() || e.ord.as_ref().is_some_and(|t| qtags.contains(t))) {
                cand.push(e);
            }
        }
        // context nouns of the question narrow the events when the story has them («did she play tag with»)
        for c in &qctx {
            if cand.iter().any(|e| e.ctx.contains(c)) {
                cand.retain(|e| e.ctx.contains(c));
            }
        }
        for e in cand {
            sum = sum.add(&e.val, 1.0);
            any = true;
        }
        if any {
            w.log.push(format!("sum events {ev_classes:?}"));
            return ev(&sum);
        }
        // the change the story left untold («had 40 apples … had 39 left. How many did he use?»): the world put an
        // unknown change before the state it was told; a gain is asked with get/make, a loss with lose/give
        let ch: Vec<&Ev> = w.evs.iter().filter(|e| e.class == CHANGE && tf(e) && owner_q.as_ref().is_none_or(|o| e.owner == *o)).collect();
        if let ([e], false) = (ch.as_slice(), ev_classes.is_empty()) {
            let gain = ev_classes.iter().any(|c| c == "get" || c == "make");
            w.log.push("untold change".into());
            return ev(&e.val.scale(if gain { 1.0 } else { -1.0 }));
        }
        // the question asks for a change the story does not show: the state would answer something else
        if !ev_classes.is_empty() {
            w.log.push("no such change in the story".into());
            return None;
        }
    }
    // a past question about an action the story never tells («How many bales did he store?»): not a state
    if past_q && qverb.iter().any(|v| !matches!(v.as_str(), "have" | "be" | "do" | "leave") && global::verb_class(v).is_none_or(|c| c.0 != "have") && !w.evs.iter().any(|e| e.verb == *v)) && !initial_q0 {
        w.log.push("an untold action asked".into());
        return None;
    }
    // 3. state: after the first week / initially / now / total
    let tf0 = thing_f(&qthing);
    let did_have = has("did") && qverb.iter().any(|v| v == "have") && w.evs.iter().any(|e| e.kind == Kind::Is);
    let initial_q = did_have || has("initially") || has("originally") || has("begin") || has("beginning") || has("start") || has("before") || ql.windows(2).any(|x| x == ["at", "first"]);
    // «after the first week», «after yesterday's picking»: the state up to the events of that time
    let qtime = ql.iter().find(|l| time_word(l) && w.evs.iter().any(|e| e.ord.as_deref() == Some(l.as_str()))).cloned();
    let upto = if initial_q {
        usize::MAX - 1
    } else if let Some(o) = &qtime {
        w.evs.iter().find(|e| e.ord.as_deref() == Some(o.as_str())).map(|e| e.t).unwrap_or(t_end)
    } else {
        t_end
    };
    let tf: Box<dyn Fn(&Ev) -> bool> = match qtime.clone() {
        Some(o) if !initial_q => Box::new(move |e: &Ev| tf0(e) && !(e.t == upto && e.ord.is_some() && e.ord.as_deref() != Some(o.as_str()))),
        _ => tf0,
    };
    let all_q = has("altogether") || has("total") || has("together") || has("combined") || has("all") || qsubj.as_deref().is_some_and(|s| matches!(s, "they" | "we")) && owner_q.is_none();
    // («How many tickets does Angela have in all?» — all of Angela's)
    let named_subj = qsubj_i.is_some_and(|k| q.w[k].upos == UPos::PROPN && q.kids_r(k, Rel::Conj).is_empty() && known(&ql[k]));
    let owner = if all_q && !named_subj { None } else { owner_q.clone() };
    // no owner asked but several in the story: whose state is meant is not said
    let owners: std::collections::BTreeSet<&str> = w.evs.iter().filter(|e| tf(e) && !e.derived && !e.outside && e.kind != Kind::Rel && e.other.is_none()).map(|e| e.owner.as_str()).collect();
    if owner.is_none() && !all_q && owners.len() > 1 {
        w.log.push(format!("whose? {owners:?}"));
        return None;
    }
    if upto == usize::MAX - 1 {
        let o = owner_q.clone().or_else(|| Some(w.main.clone()));
        let st = w.state(o.as_deref(), &|e: &Ev| tf(e) && e.kind != Kind::Add && e.kind != Kind::Sub, t_end).or_else(|| {
            // no «had N» — unknown initial value (numbered as in state)
            w.evs.iter().position(|e| tf(e) && o.as_ref().is_none_or(|x| e.owner == *x) && e.kind != Kind::Is).map(|k| Lin::var(1000 + k))
        });
        w.log.push(format!("state initially {o:?} {qthing:?}"));
        return st.and_then(|x| ev(&x));
    }
    // «How many balloons did Allan and Jake have»: the sum of the coordinated owners' states
    if q_owners.len() >= 2 {
        let mut sum = Lin::default();
        for o in &q_owners {
            sum = sum.add(&w.state(Some(o), &*tf, upto)?, 1.0);
        }
        w.log.push(format!("state of {q_owners:?} up to t{upto}"));
        return ev(&sum);
    }
    match w.state(owner.as_deref(), &*tf, upto) {
        Some(st) => {
            w.log.push(format!("state {owner:?} {qthing:?} up to t{upto}"));
            ev(&st)
        }
        None => {
            w.log.push("state: no events".into());
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    /// The trained `en` annotator; tests are skipped when the model file is not on this machine.
    fn ann() -> Option<&'static Annotator> {
        static A: OnceLock<Option<Annotator>> = OnceLock::new();
        A.get_or_init(|| Annotator::load(std::path::Path::new(crate::understand::MODEL)).ok()).as_ref()
    }

    /// The trusted answer (None when the strict gate abstains) and the log.
    fn solve(text: &str) -> Option<(Option<f64>, String)> {
        let (v, w) = answer_text(ann()?, text);
        let v = if abstain(&w, v).is_some() { None } else { v };
        Some((v, w.log.join(" | ")))
    }

    fn check(text: &str, want: Option<f64>) {
        if let Some((v, log)) = solve(text) {
            assert_eq!(v, want, "{text}\n{log}");
        }
    }

    #[test]
    fn explains_with_the_numbers_used() {
        if let Some((v, log)) = solve("Tom had 5 apples. He bought 3 more apples. How many apples does Tom have now?") {
            assert_eq!(v, Some(8.0), "{log}");
            assert!(log.contains("answer: 5 + 3 = 8"), "{log}");
        }
    }

    #[test]
    fn gate_abstains_on_a_bare_number() {
        // negative control: the question repeats a told number — no operation, no trusted answer
        check("Tom has 5 apples. How many apples does Tom have?", None);
    }

    #[test]
    fn left_as_departure_and_as_state() {
        check("A waiter had 9 customers. 4 customers left. How many customers does he have now?", Some(5.0));
        // a told state the events do not reach: an untold change
        check("Ann had 12 pens. Now she has 7 pens left. How many pens did she lose?", Some(5.0));
    }

    #[test]
    fn comparison_sides_share_what_they_leave_out() {
        check("Sam received 6 letters in the morning and 2 letters in the evening. How many more letters did Sam receive in the morning than in the evening?", Some(4.0));
    }

    #[test]
    fn relations_between_owners() {
        check("Lily has 4 more stamps than Max. Max has 9 stamps. How many stamps does Lily have?", Some(13.0));
        // a lesser comparative (level 1)
        check("The frog jumped 20 inches. The toad jumped 6 inches less than the frog. How far did the toad jump?", Some(14.0));
    }

    #[test]
    fn relation_between_two_times() {
        check("Ann played tag with 6 kids on monday. She played tag with 2 more kids on monday than on friday. How many kids did she play with on friday?", Some(4.0));
    }

    #[test]
    fn comparison_with_an_unread_side_abstains() {
        check("30 students suggested pizza. 6 more students suggested pizza than those that suggested pasta. How many students suggested pasta?", None);
    }

    #[test]
    fn harvest_takes_from_the_counted_stock() {
        check("The tree had 10 pears. Mia picked 3 pears. How many pears are on the tree now?", Some(7.0));
    }

    #[test]
    fn money_paid_for_something_leaves() {
        check("Ben has $ 20. He bought a book for $ 8. How much money does Ben have left?", Some(12.0));
    }

    #[test]
    fn rates_total_rate_and_count() {
        check("Each box has 4 toys. Jim has 3 boxes. How many toys does Jim have?", Some(12.0));
        check("Bryan has 7 shelves. He has a total of 28 books. How many books are there in each shelf?", Some(4.0));
        check("Paige put 10 seeds in each flower bed. She planted 60 seeds altogether. How many flower beds did she have?", Some(6.0));
    }

    #[test]
    fn rates_of_several_kinds_add_up() {
        check("Each basket has 19 red peaches and 4 green peaches. There are 15 baskets. How many peaches are in the baskets altogether?", Some(345.0));
        check("Each basket has 19 red peaches and 4 green peaches. There are 15 baskets. How many green peaches are in the baskets altogether?", Some(60.0));
    }

    #[test]
    fn places_hold_existential_things() {
        check("There are 46 rulers in the drawer. Tim took 25 rulers from the drawer. How many rulers are now in the drawer?", Some(21.0));
    }

    #[test]
    fn negated_events_abstain() {
        // negative control: the world does not read negation
        check("Wendy had 11 bags. She did not recycle 2 bags. How many bags does Wendy have?", None);
    }

    #[test]
    fn question_numbers_must_be_used() {
        check("Kelly has 106 games. How many games will she have left if she gives away 64 games?", Some(42.0));
    }

    #[test]
    fn a_fraction_of_things_abstains() {
        // negative control: 10 toys in boxes of 4 is not a whole count
        check("Each box has 4 toys. Jim has 10 toys. How many boxes does Jim have?", None);
    }

    #[test]
    fn unknown_initial_state_is_solved() {
        check("Kim had some marbles. She lost 4 marbles. Now she has 6 marbles. How many marbles did Kim have at first?", Some(10.0));
    }

    #[test]
    fn the_rest_of_a_total() {
        check("During the break 12 kids go to camp and the rest stay home. The town has 30 kids in all. How many kids stayed home?", Some(18.0));
    }
}
