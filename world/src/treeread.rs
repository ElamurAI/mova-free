//! The tree reader of word problems (design note, 15.10: the maths student reads through the tree too). Each told number
//! is read off the dependency tree: what it counts (the noun it modifies), who holds or acts on it (the subject of its
//! verb; a pronoun is the last name), the verb and its effect on the holder (the skeleton's verb classes: get +,
//! give ±, lose −, have =), a rate («each», «per», «every») and a comparison («3 fewer … than Steven»). The question is
//! read the same way: what is counted, about whom, compared with what, and whether it asks for what is left, the
//! total, or each one's share. Then a few schemas, in order, each answering only when its pieces are all there:
//! comparison, total, rate, what is left, the unknown change. Every answer says which schema and which numbers.
//!
//!   world treeread "<problem>"   — the reading and the answer
//!
//! Measured 15.10 on SVAMP train (700, alone): 185 right, 200 wrong, 315 no answer (the old world reader: 132 / 425 /
//! 143). Per schema: each 19/19, the change 33/52, comparison 77/158, total 15/33, part of a whole 30/74, left 9/39,
//! rate 2/8. Put before the learned operation in the book's way (MOVA_TREESCHEMAS, two-fold check, base 1116): each
//! 1116, each+the change 1119, + comparison 1095, + part of a whole 1050, + total 1059 — off by default; the learned
//! operation chooser is already more precise than these hand schemas.

use crate::frames::parse_words;
use en::annotate::Word;
use en::gram::{Rel, UPos};

/// One told number as the tree reads it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Q {
    pub v: i64,
    pub thing: String,
    pub holder: String,
    pub verb: String,
    /// the verb's effect on its holder: '+', '-', '=', '±' (gives: less for the giver), '?' unknown
    pub effect: char,
    pub rate: bool,
    /// «N more/fewer … than X»: (+1 more / −1 fewer, X)
    pub comp: Option<(i64, String)>,
    /// «in the morning», «on Monday»: the time or place it belongs to
    pub when: String,
    pub sent: usize,
    /// every content word that tells this quantity apart: the thing, its adjectives, the holder, the verb, the time,
    /// and the nouns of the verb's other phrases («to build a tower», «to africa»)
    pub tags: Vec<String>,
    /// told in the question itself
    pub in_q: bool,
}

/// The question as the tree reads it.
#[derive(Clone, Debug, Default)]
pub struct Ask {
    /// the content words before «than» and after it: what each side of a comparison is
    pub left_tags: Vec<String>,
    pub right_tags: Vec<String>,
    pub thing: String,
    pub holder: String,
    pub verb: String,
    pub than: String,
    pub more: bool,
    pub left: bool,
    pub total: bool,
    pub each: bool,
}

fn lemma(w: &Word) -> String {
    w.lemma.to_lowercase()
}

fn num(w: &Word) -> Option<i64> {
    let f = w.form.replace([',', '$'], "");
    f.parse::<f64>().ok().filter(|x| x.fract() == 0.0).map(|x| x as i64)
}

fn kids(w: &[Word], i: usize) -> impl Iterator<Item = usize> + '_ {
    (0..w.len()).filter(move |&k| w[k].head == i + 1)
}

/// The verb a word hangs under (itself if it is one), climbing heads.
fn verb_of(w: &[Word], mut i: usize) -> Option<usize> {
    for _ in 0..w.len() {
        if matches!(w[i].upos, UPos::VERB | UPos::AUX) || w[i].head == 0 {
            return Some(i);
        }
        i = w[i].head - 1;
    }
    None
}

const TIMES: [&str; 14] = ["morning", "afternoon", "evening", "night", "monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday", "yesterday", "today", "week"];

/// The subject of a verb (through conjoined verbs to the first one), a pronoun replaced by the last name seen.
fn subject(w: &[Word], v: usize, last: &str) -> String {
    let mut v = v;
    for _ in 0..4 {
        // several subjects (a parse slip in «how many apples does Tom have»): a name or a pronoun first
        let subs: Vec<usize> = kids(w, v).filter(|&k| matches!(w[k].rel.base(), Rel::Nsubj | Rel::Expl)).collect();
        if let Some(s) = subs.iter().copied().find(|&k| matches!(w[k].upos, UPos::PROPN | UPos::PRON)).or_else(|| subs.first().copied()) {
            let l = lemma(&w[s]);
            if w[s].upos == UPos::PRON && !last.is_empty() && matches!(l.as_str(), "he" | "she" | "they" | "it") {
                return last.to_string();
            }
            // «Jack's mom»: the possessor names whose it is
            if let Some(p) = kids(w, s).find(|&k| w[k].rel == Rel::NmodPoss) {
                return lemma(&w[p]);
            }
            return l;
        }
        if w[v].rel == Rel::Conj && w[v].head > 0 {
            v = w[v].head - 1;
        } else {
            break;
        }
    }
    last.to_string()
}

/// The effect of a verb on its subject (skeleton classes), «have» for copulas and existentials.
fn effect(verb: &str) -> char {
    match global::verb_class(verb) {
        Some((_, e)) => e,
        None => '?',
    }
}

/// Read a problem: its told numbers and its question.
pub fn read(text: &str) -> (Vec<Q>, Ask) {
    let text = crate::numbers::normalize(text).0;
    let mut qs = Vec::new();
    let mut ask = Ask::default();
    let mut last = String::new();
    for (si, s) in text.split_inclusive(['.', '!', '?']).enumerate() {
        let w = parse_words(s.trim());
        if w.is_empty() {
            continue;
        }
        let question = s.trim_end().ends_with('?');
        if question {
            let root = w.iter().position(|x| x.head == 0).unwrap_or(0);
            let low: Vec<String> = w.iter().map(lemma).collect();
            // what is counted: the noun «many»/«much» modifies
            if let Some(m) = (0..w.len()).find(|&i| matches!(low[i].as_str(), "many" | "much")) {
                let h = w[m].head;
                if h > 0 && matches!(w[h - 1].upos, UPos::NOUN | UPos::PROPN) {
                    ask.thing = lemma(&w[h - 1]);
                }
            }
            ask.verb = lemma(&w[root]);
            ask.holder = subject(&w, root, &last);
            if ask.holder == ask.thing {
                ask.holder.clear();
            }
            ask.more = low.iter().any(|l| matches!(l.as_str(), "more" | "less" | "fewer" | "longer" | "shorter" | "heavier" | "older" | "younger" | "taller" | "further" | "farther"));
            if let Some(t) = (0..w.len()).find(|&i| low[i] == "than") {
                // the word «than» introduces: its head, or the next noun/name
                let h = w[t].head;
                let x = if h > 0 && h - 1 > t { h - 1 } else { (t + 1..w.len()).find(|&k| matches!(w[k].upos, UPos::NOUN | UPos::PROPN | UPos::PRON)).unwrap_or(t) };
                ask.than = if w[x].upos == UPos::PRON && !last.is_empty() { last.clone() } else { lemma(&w[x]) };
            }
            let content = |r: std::ops::Range<usize>| -> Vec<String> {
                r.filter(|&i| matches!(w[i].upos, UPos::NOUN | UPos::PROPN | UPos::ADJ | UPos::VERB) && !matches!(low[i].as_str(), "many" | "much" | "more" | "less" | "fewer" | "have" | "do" | "be")).map(|i| low[i].clone()).collect()
            };
            match (0..w.len()).find(|&i| low[i] == "than") {
                Some(t) => {
                    ask.left_tags = content(0..t);
                    ask.right_tags = content(t + 1..w.len());
                }
                None => ask.left_tags = content(0..w.len()),
            }
            ask.left = low.iter().any(|l| matches!(l.as_str(), "left" | "remain" | "now" | "still"));
            ask.total = low.iter().any(|l| matches!(l.as_str(), "total" | "altogether" | "together" | "combined" | "all" | "sum"));
            ask.each = low.iter().any(|l| matches!(l.as_str(), "each" | "every" | "per"));
        }
        for i in 0..w.len() {
            let Some(v) = num(&w[i]) else { continue };
            if w[i].upos == UPos::NUM || w[i].upos == UPos::NOUN || w[i].upos == UPos::SYM || w[i].upos == UPos::ADJ || w[i].upos == UPos::PROPN {
                // the noun it counts
                let n = if w[i].rel == Rel::Nummod && w[i].head > 0 { w[i].head - 1 } else { i };
                let thing = if n != i { lemma(&w[n]) } else { String::new() };
                let vb = verb_of(&w, n);
                let verb = vb.map(|x| lemma(&w[x])).unwrap_or_default();
                let holder = vb.map(|x| subject(&w, x, &last)).unwrap_or_else(|| last.clone());
                let around: Vec<usize> = kids(&w, n).chain(kids(&w, i)).collect();
                let rate = around.iter().any(|&k| matches!(lemma(&w[k]).as_str(), "each" | "every" | "per" | "apiece" | "a"))
                    && around.iter().any(|&k| matches!(lemma(&w[k]).as_str(), "each" | "every" | "per" | "apiece"));
                let mut comp = None;
                if let Some(k) = around.iter().copied().find(|&k| matches!(lemma(&w[k]).as_str(), "more" | "fewer" | "less")) {
                    let dir = if lemma(&w[k]) == "more" { 1 } else { -1 };
                    // «than X»: anywhere after
                    let than = (k..w.len()).find(|&t| lemma(&w[t]) == "than").and_then(|t| (t + 1..w.len()).find(|&x| matches!(w[x].upos, UPos::NOUN | UPos::PROPN | UPos::PRON))).map(|x| lemma(&w[x])).unwrap_or_default();
                    // «3 more apples» with no «than» is a gain, not a comparison
                    if !than.is_empty() {
                        comp = Some((dir, than));
                    }
                }
                let when = vb
                    .map(|x| kids(&w, x).filter(|&k| w[k].rel.base() == Rel::Obl).map(|k| lemma(&w[k])).find(|l| TIMES.contains(&l.as_str())).unwrap_or_default())
                    .unwrap_or_default();
                let eff = if matches!(verb.as_str(), "be" | "have") { '=' } else { effect(&verb) };
                // the tags: the thing and its adjectives, holder, verb, time, and the nouns under the verb's other
                // phrases (purpose, place)
                let mut tags: Vec<String> = vec![thing.clone(), holder.clone(), verb.clone(), when.clone()];
                tags.extend(kids(&w, n).filter(|&k| matches!(w[k].rel.base(), Rel::Amod | Rel::Compound)).map(|k| lemma(&w[k])));
                if let Some(x) = vb {
                    for k in kids(&w, x).filter(|&k| matches!(w[k].rel.base(), Rel::Obl | Rel::Advcl | Rel::Xcomp | Rel::Obj) && k != n) {
                        tags.push(lemma(&w[k]));
                        tags.extend(kids(&w, k).filter(|&j| matches!(w[j].upos, UPos::NOUN | UPos::PROPN)).map(|j| lemma(&w[j])));
                    }
                }
                tags.retain(|t| !t.is_empty());
                qs.push(Q { v, thing, holder: holder.clone(), verb, effect: eff, rate, comp, when, sent: si, tags, in_q: question });
            }
        }
        // the last name told: who «he», «she» will be
        if let Some(x) = w.iter().find(|x| x.upos == UPos::PROPN && x.rel.base() == Rel::Nsubj) {
            last = lemma(x);
        }
    }
    (qs, ask)
}

fn same_thing(a: &str, b: &str) -> bool {
    !a.is_empty() && !b.is_empty() && (a == b || a.trim_end_matches('s') == b.trim_end_matches('s') || a.contains(b) || b.contains(a))
}

/// The answer and its reason, or None when no schema has all its pieces.
pub fn answer(text: &str) -> Option<(i64, String)> {
    let (qs, a) = read(text);
    let of_thing: Vec<&Q> = qs.iter().filter(|q| a.thing.is_empty() || same_thing(&q.thing, &a.thing) || q.thing.is_empty()).collect();
    // 1. comparison: «how many more A than B» — the quantity that fits A's words best against the one that fits B's
    if a.more && !a.right_tags.is_empty() {
        let fit = |q: &Q, tags: &[String]| tags.iter().filter(|t| q.tags.iter().any(|x| same_thing(x, t))).count();
        let pick = |tags: &[String], not: Option<&Q>| -> Option<&Q> {
            let mut best: Option<(&Q, usize)> = None;
            for q in qs.iter().filter(|q| !q.in_q && Some(*q) != not && q.comp.is_none()) {
                let f = fit(q, tags);
                if f > 0 && best.is_none_or(|b| f > b.1) {
                    best = Some((q, f));
                }
            }
            best.map(|b| b.0)
        };
        if let Some(y) = pick(&a.right_tags, None) {
            if let Some(x) = pick(&a.left_tags, Some(y)) {
                return Some(((x.v - y.v).abs(), format!("comparison: {} ({}) against {} ({})", x.v, x.tags.join(" "), y.v, y.tags.join(" "))));
            }
        }
        // a comparison told in the problem: «A has 5 more than B» asked back
        if let Some(c) = of_thing.iter().find(|q| q.comp.is_some()) {
            return Some((c.v, format!("comparison told: {} {}", c.v, c.verb)));
        }
    }
    // 1b. part and whole: a total told («together», «in all», «total»), the asked part is the total less the others
    {
        let text_low = text.to_lowercase();
        let body = text_low.rsplit_once(['.', '!']).map(|x| x.0).unwrap_or(&text_low);
        if !a.total && ["together", "in all", "total", "altogether", "combined", "in total"].iter().any(|w| body.contains(w)) && qs.len() >= 2 {
            let whole = qs.iter().filter(|q| !q.in_q).max_by_key(|q| q.v)?;
            let parts: i64 = qs.iter().filter(|q| !q.in_q && *q != whole && !q.rate).map(|q| q.v).sum();
            if parts < whole.v {
                return Some((whole.v - parts, format!("part of a whole: {} − {}", whole.v, parts)));
            }
        }
    }
    let plain: Vec<&Q> = of_thing.iter().copied().filter(|q| !q.rate && q.comp.is_none()).collect();
    // 2. total: «how many … in all / altogether»
    if a.total && plain.len() >= 2 {
        let s: i64 = plain.iter().map(|q| q.v).sum();
        return Some((s, format!("total: {}", plain.iter().map(|q| q.v.to_string()).collect::<Vec<_>>().join(" + "))));
    }
    // 3. rate: a quantity per one, times a count — or a total shared out «each»
    if let Some(r) = qs.iter().find(|q| q.rate) {
        if let Some(c) = qs.iter().find(|q| !q.rate && *q != r) {
            return Some((r.v * c.v, format!("rate: {} each × {}", r.v, c.v)));
        }
    }
    if a.each && qs.len() == 2 {
        let (big, small) = if qs[0].v >= qs[1].v { (&qs[0], &qs[1]) } else { (&qs[1], &qs[0]) };
        if small.v > 0 && big.v % small.v == 0 {
            return Some((big.v / small.v, format!("each: {} ÷ {}", big.v, small.v)));
        }
    }
    // 4. what is left: the holder's start, changed by its gains and losses
    if a.left || a.verb == "have" {
        let mine: Vec<&Q> = plain.iter().copied().filter(|q| a.holder.is_empty() || q.holder == a.holder || q.holder.is_empty()).collect();
        let mine: Vec<&Q> = if mine.iter().any(|q| q.effect == '=') { mine } else { plain.clone() };
        if let Some(start) = mine.iter().find(|q| q.effect == '=') {
            let mut v = start.v;
            let mut why = format!("left: {}", start.v);
            for q in mine.iter().filter(|q| *q != start) {
                match q.effect {
                    '+' => {
                        v += q.v;
                        why += &format!(" + {} ({})", q.v, q.verb);
                    }
                    '-' | '±' => {
                        v -= q.v;
                        why += &format!(" − {} ({})", q.v, q.verb);
                    }
                    _ => {}
                }
            }
            if mine.len() >= 2 && v >= 0 {
                return Some((v, why));
            }
        }
    }
    // 5. the unknown change: a start and what is left told, the asked verb's amount is the difference
    if matches!(effect(&a.verb), '-' | '±' | '+') && plain.len() == 2 {
        let (x, y) = (plain[0].v, plain[1].v);
        return Some(((x - y).abs(), format!("the change: |{x} − {y}|")));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_and_answers_by_schema() {
        // these need the en model on disk; skip quietly where it is missing
        if crate::tree::annotator().is_err() {
            return;
        }
        assert_eq!(answer("Jack received 10 emails in the morning. He received 3 emails in the afternoon. How many more emails did Jack receive in the morning than in the afternoon?").map(|x| x.0), Some(7));
        assert_eq!(answer("Tom has 5 apples. He buys 3 more apples. How many apples does Tom have now?").map(|x| x.0), Some(8));
        // negative control: a problem with no schema's pieces gets no answer
        assert_eq!(answer("The sky is blue. Why?"), None);
    }
}
