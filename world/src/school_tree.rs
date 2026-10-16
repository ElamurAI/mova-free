//! The tree mathematician's reading (design note, 15.10: «take the tree mathematician, throw the old one out; take the old
//! one's ideas if there is something»). A problem is read into the same `Reading` the world plays, but along the
//! dependency tree instead of a window of words:
//!
//! - clauses are the tree's predicates (the root, conjoined and adverbial verbs), not the stretches between «and»s;
//! - who acts is the clause's subject (a conjoined verb without one shares its head's), the verb is the predicate's
//!   lemma, a passive («is given … by Zoe») is read from its auxiliary;
//! - the receiver is the indirect object or the «to» oblique, the source the «from» / «by» oblique;
//! - a number counts the noun it modifies («cups of flour» → flour); «N more T» is the noun's «more»;
//! - a comparison is the «than» / «as … as» phrase hanging in the clause.
//!
//! The old reader's ideas stay: the skeleton words (`on`, an ablation registry) for lines, calendars, clocks, prices,
//! rates, packs, parts and the rest; pronouns resolved by the names' sex; relations waiting for a count; a thing
//! carried over from the actor's last one.

use super::*;
use en::annotate::Word;
use en::gram::{Rel, UPos};

/// A sentence read by the tree: the words (forms lower case) and its clauses.
struct TSent {
    w: Vec<Word>,
    low: Vec<String>,
}

impl TSent {
    fn new(s: &str) -> TSent {
        let w = crate::frames::parse_words(s.trim());
        let low = w.iter().map(|x| x.form.to_lowercase()).collect();
        TSent { w, low }
    }

    fn kids(&self, i: usize) -> impl Iterator<Item = usize> + '_ {
        (0..self.w.len()).filter(move |&k| self.w[k].head == i + 1)
    }

    fn head(&self, i: usize) -> Option<usize> {
        let h = self.w[i].head;
        (h > 0 && h <= self.w.len()).then(|| h - 1)
    }

    fn base(&self, i: usize) -> Rel {
        self.w[i].rel.base()
    }

    /// The preposition («case») on a word.
    fn case_of(&self, i: usize) -> Option<String> {
        self.kids(i).find(|&k| self.base(k) == Rel::Case).map(|k| self.low[k].clone())
    }

    fn punct(&self, i: usize) -> bool {
        self.w[i].upos == UPos::PUNCT
    }

    /// A predicate heads a clause: the root, and a verb (or a word with its own subject or copula) conjoined to,
    /// or hanging as an adverbial clause, under another.
    fn is_pred(&self, i: usize) -> bool {
        if self.w[i].head == 0 {
            return true;
        }
        let own = self.kids(i).any(|k| matches!(self.base(k), Rel::Nsubj | Rel::Cop | Rel::Expl));
        matches!(self.base(i), Rel::Conj | Rel::Advcl | Rel::Parataxis) && (self.w[i].upos == UPos::VERB || own)
    }

    /// The clauses in the order of their first word: (predicate, its words in order), punctuation left out.
    fn clauses(&self) -> Vec<(usize, Vec<usize>)> {
        let preds: Vec<usize> = (0..self.w.len()).filter(|&i| self.is_pred(i)).collect();
        let mut by: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for i in 0..self.w.len() {
            if self.punct(i) {
                continue;
            }
            let mut j = i;
            for _ in 0..self.w.len() {
                if preds.contains(&j) {
                    break;
                }
                match self.head(j) {
                    Some(h) => j = h,
                    None => break,
                }
            }
            // a «cc» word («and», «then») belongs before its clause, not to it
            if self.base(i) == Rel::Cc && preds.contains(&j) && j != i {
                continue;
            }
            by.entry(j).or_default().push(i);
        }
        let mut v: Vec<(usize, Vec<usize>)> = by.into_iter().filter(|(p, _)| preds.contains(p)).collect();
        v.sort_by_key(|x| x.1.first().copied().unwrap_or(0));
        v
    }

    /// The subject of a clause's predicate: its own, or (a conjoined one) its head's.
    fn subject(&self, p: usize) -> Option<usize> {
        let mut p = p;
        for _ in 0..4 {
            if let Some(s) = self.kids(p).find(|&k| matches!(self.base(k), Rel::Nsubj | Rel::Expl)) {
                return Some(s);
            }
            if self.base(p) == Rel::Conj {
                p = self.head(p)?;
            } else {
                return None;
            }
        }
        None
    }

    /// Passive: «is given», «was handed».
    fn passive(&self, p: usize) -> bool {
        self.kids(p).any(|k| self.w[k].rel == Rel::AuxPass || self.w[k].rel == Rel::NsubjPass)
    }

    /// The noun a number counts: its head when it modifies one (through a measure: «2 cups of flour» → flour).
    fn counted(&self, i: usize) -> Option<usize> {
        let h = if self.w[i].rel == Rel::Nummod { self.head(i)? } else { return None };
        // «2 more»: «more» is no thing
        if !matches!(self.w[h].upos, UPos::NOUN | UPos::PROPN) || matches!(self.low[h].as_str(), "more" | "fewer" | "less" | "other" | "extra" | "out" | "away" | "back" | "up" | "down" | "off" | "over" | "left") {
            return None;
        }
        if MEASURES.contains(&self.low[h].as_str()) {
            if let Some(k) = self.kids(h).find(|&k| self.case_of(k).as_deref() == Some("of")) {
                return Some(k);
            }
        }
        Some(h)
    }
}

/// Read a problem into a world along its trees.
pub(super) fn read_tree(text: &str) -> Reading {
    let text = &if on("numbers") { crate::numbers::normalize(&resolve_pronouns(text)).0 } else { resolve_pronouns(text) };
    let text = &if on("dozen pair") { groups_of_words(text) } else { text.to_string() };
    let mut r = Reading::default();
    let mut last: Option<Ev> = None;
    let mut last_subj = String::new();
    // «2 chairs per table»: (count for one, the unit), waiting for how many units
    let mut per_unit: Option<(i64, String)> = None;
    let holding_lemma = |l: &str| matches!(l, "have" | "hold" | "be");
    let (mut fp, mut pending, mut used): (String, Vec<i64>, BTreeSet<i64>) = (String::new(), Vec::new(), BTreeSet::new());
    for sentence in text.split_inclusive(['.', '?', '!']) {
        if sentence.trim().is_empty() {
            continue;
        }
        let t = TSent::new(sentence);
        if t.w.is_empty() {
            continue;
        }
        let low = &t.low;
        let raw_cap: Vec<bool> = {
            // the case as written: the tree's forms are as typed, the first word's capital says nothing
            sentence.split(|c: char| !c.is_alphanumeric() && c != '\'').filter(|x| !x.is_empty()).map(|x| x.chars().next().is_some_and(char::is_uppercase)).collect()
        };
        let _ = raw_cap;
        let is_name = |i: usize| -> bool {
            KNOWN_NAMES.iter().any(|n| n.eq_ignore_ascii_case(&low[i]))
                || (t.w[i].upos == UPos::PROPN && !SENTENCE_OPENERS.contains(&low[i].as_str()) && crate::triggers::number_word(&low[i]).is_none() && on("names"))
        };
        let num = |i: usize| -> Option<i64> { low[i].parse().ok().or_else(|| crate::triggers::number_word(&low[i]).map(|x| x as i64)) };
        let thing_of = |i: usize| -> String {
            if let Some(h) = t.counted(i) {
                if let Some(s) = school_norm(&low[h]) {
                    return s;
                }
            }
            // a number standing alone («takes 2 out», «gives Lily six»): the thing is the one before
            if t.w[i].rel != Rel::Nummod && !(low.get(i + 1).is_some_and(|w| w == "of") || low.get(i + 1).is_some_and(|w| matches!(w.as_str(), "more" | "fewer" | "big" | "small" | "new" | "old"))) {
                return String::new();
            }
            if on("things") { thing_phrase(low, i + 1) } else { low.get(i + 1).and_then(|w| school_norm(w)).unwrap_or_default() }
        };
        let text_low = sentence.to_lowercase();
        settle(&r, &mut fp, &mut pending, &mut used);
        pending = (0..low.len()).filter_map(num).collect();
        if sentence.trim_end().ends_with('?') {
            pending.clear();
            if (on("each with") && text_low.contains("each with")) || (on("such") && text_low.contains("such")) {
                let ns: Vec<i64> = (0..low.len()).filter_map(num).collect();
                if ns.len() >= 2 {
                    r.groups.push((ns[0], ns[1]));
                }
            }
            let named: Vec<String> = (0..low.len()).filter(|&i| is_name(i)).map(|i| low[i].clone()).collect();
            let together = r.ask_all_words(&text_low);
            if named.len() >= 2 && together {
                r.ask_some = named;
            }
            r.ask_left = (on("left") && text_low.contains("left")) || (on("remain") && text_low.contains("remain"));
            r.ask_all = r.ask_all_words(&text_low);
            r.ask_each = on("each") && text_low.contains("each");
            r.ask_spent = on("cost") && ["pay", "spend", "spent", "cost"].iter().any(|w| low.iter().any(|x| x == w));
            r.ask_full = on("full") && (low.iter().any(|x| x == "full") && !text_low.contains("outside") || low.iter().any(|x| x == "fill"));
            r.ask_over = on("left over") && (text_low.contains("left over") || text_low.contains("remain"));
            if on("week") {
                if let Some(k) = low.iter().position(|w| matches!(w.as_str(), "day" | "days" | "week" | "weeks")) {
                    let n = if k > 0 { if low[k - 1] == "a" { Some(1) } else { num(k - 1) } } else { None };
                    r.days = n.map(|n| if low[k].starts_with("week") { 7 * n } else { n });
                }
            }
            if on("line") {
                r.ask_front = text_low.contains("in front");
                r.ask_behind = low.iter().any(|w| w == "behind");
            }
            if on("clock") && low.iter().any(|w| w == "minutes") {
                if let Some(k) = low.iter().position(|w| w == "hours" || w == "hour") {
                    r.ask_minutes = if k > 0 { num(k - 1).or((low[k - 1] == "an").then_some(1)) } else { None };
                }
            }
            if on("years") {
                if let Some(k) = low.iter().position(|w| w == "years" || w == "year") {
                    if let Some(n) = if k > 0 { num(k - 1) } else { None } {
                        r.shift = if low.get(k + 1).is_some_and(|w| w == "ago") { -n } else if k > 1 && low[k - 2] == "in" { n } else { 0 };
                    }
                }
            }
            if on("things") {
                // what is counted: the noun «many» / «much» modifies, or the words after it
                if let Some(k) = low.iter().position(|w| w == "many" || w == "much") {
                    let tr = t.head(k).filter(|&h| matches!(t.w[h].upos, UPos::NOUN)).and_then(|h| school_norm(&low[h]));
                    let ph = thing_phrase(low, k + 1);
                    let th = if !ph.is_empty() { Some(ph) } else { tr };
                    if let Some(th) = th.filter(|x| !x.is_empty()) {
                        r.ask_thing = Some(th);
                    }
                }
                if let Some(k) = low.iter().position(|w| w == "than") {
                    let th = thing_phrase(low, k + 1);
                    if !th.is_empty() && !(k + 1 < low.len() && is_name(k + 1)) {
                        r.than_thing = Some(th);
                    }
                }
            }
            let qn: Vec<i64> = (0..low.len()).filter_map(num).collect();
            if qn.len() == 1 {
                r.q_num = Some(qn[0]);
            }
            if let Some(k) = low.iter().position(|w| on("than") && w == "than") {
                // what «than» compares with: the word it marks in the tree, else the next content word
                let marked = t.head(k).filter(|&h| h > k && !t.punct(h));
                r.ask_than = marked.map(|h| school_norm(&low[h]).unwrap_or(low[h].clone())).or_else(|| low[k + 1..].iter().find(|w| !matches!(w.as_str(), "in" | "on" | "the" | "a" | "at")).map(|w| school_norm(w).unwrap_or(w.clone())));
            }
            let than_at = low.iter().position(|w| w == "than").unwrap_or(usize::MAX);
            // whose: the question's subject when it is a name, else the first name before «than»
            let root = t.w.iter().position(|x| x.head == 0).unwrap_or(0);
            let subj_name = t.kids(root).filter(|&k| matches!(t.base(k), Rel::Nsubj)).find(|&k| is_name(k) && k < than_at);
            let nouns_act = std::env::var("MOVA_TREE_NOUNS").as_deref() != Ok("0");
            // a noun subject that is not what is counted («does the pond hold»)
            // «how many fewer balloons are on the desk than on the shelf»: the place before «than»
            let place_q = (0..than_at.min(low.len())).find(|&i| matches!(t.case_of(i).as_deref(), Some("on" | "in" | "at")) && t.w[i].upos == UPos::NOUN).and_then(|i| school_norm(&low[i]));
            let place_q = place_q.filter(|_| than_at != usize::MAX);
            let subj_noun = t.kids(root).filter(|&k| matches!(t.base(k), Rel::Nsubj) && t.w[k].upos == UPos::NOUN && !t.kids(k).any(|j| matches!(low[j].as_str(), "many" | "much"))).find(|&k| k < than_at).filter(|_| nouns_act);
            r.ask_whose = subj_name.or_else(|| (0..low.len()).find(|&i| is_name(i) && i < than_at)).map(|i| low[i].clone()).or_else(|| place_q.clone()).or_else(|| subj_noun.and_then(|k| school_norm(&low[k]))).or_else(|| {
                (r.ask_than.is_none()).then(|| (0..low.len()).find(|&i| matches!(low[i].as_str(), "in" | "on") && low.get(i + 1).is_some_and(|w| w == "the")).and_then(|i| low.get(i + 2)).map(|w| school_norm(w).unwrap_or(w.clone()))).flatten()
            }).or_else(|| {
                if !(low.first().is_some_and(|w| w == "how") && low.get(1).is_some_and(|w| matches!(w.as_str(), "long" | "tall" | "heavy" | "old" | "wide" | "high" | "far"))) {
                    return None;
                }
                let k = (0..low.len().saturating_sub(1)).find(|&i| low[i] == "is" && low[i + 1] == "the")?;
                let words: Vec<String> = low[k + 2..].iter().take_while(|w| !matches!(w.as_str(), "now" | "then" | "?")).cloned().collect();
                (!words.is_empty()).then(|| object_name(&words))
            });
            continue;
        }
        // ordinals: «6th», «21st»
        let ords: Vec<(usize, i64)> = (0..low.len()).filter_map(|i| {
            let w = &low[i];
            let d = w.trim_end_matches(|c: char| c.is_ascii_alphabetic());
            (d.len() < w.len() && !d.is_empty() && matches!(&w[d.len()..], "st" | "nd" | "rd" | "th")).then(|| d.parse().ok().map(|v| (i, v))).flatten()
        }).collect();
        if on("line") && low.iter().any(|w| matches!(w.as_str(), "line" | "queue" | "row")) {
            if let Some(n) = (0..low.len()).find_map(num) {
                r.line = Some(n);
            }
            for (k, &(i, v)) in ords.iter().enumerate() {
                let until = ords.get(k + 1).map(|x| x.0).unwrap_or(low.len());
                if low[i..until].iter().any(|w| w == "back") { r.back = Some(v) } else { r.front = Some(v) }
            }
            if !ords.is_empty() || r.line.is_some() {
                continue;
            }
        }
        if on("line") && !ords.is_empty() && r.line.is_some() && low.iter().any(|w| w == "is") {
            r.front = Some(ords[0].1);
            continue;
        }
        if on("calendar") && ords.len() >= 2 {
            r.span = Some((ords[0].1, ords[1].1));
            continue;
        }
        if on("clock") && low.iter().any(|w| w == "clock" || w == "o'clock") {
            let ns: Vec<i64> = (0..low.len()).filter_map(num).collect();
            if ns.len() >= 2 {
                r.clock = Some((ns[0], ns[1]));
                continue;
            }
        }
        let clauses = t.clauses();
        // «now he has 5»: the end of the story, told
        if on("now") && low.first().is_some_and(|w| w == "now") {
            let p = clauses.first().map(|c| c.0).unwrap_or(0);
            let who = t.subject(p).filter(|&s| is_name(s)).or_else(|| (0..low.len()).find(|&i| is_name(i)));
            if let (Some(i), Some(n)) = (who, (0..low.len()).find_map(num)) {
                r.now = Some((low[i].clone(), n));
                continue;
            }
        }
        // «Ben does the same», «so did Ben», «Ben did too»: the last action, with this one acting
        if ((on("the same") && text_low.contains("the same")) || (on("so did") && ["so did", "so does", "so do"].iter().any(|x| text_low.trim_start().starts_with(x))) || (on("too") && text_low.contains(" too"))) && last.is_some() {
            if let (Some(mut e), Some(i)) = (last.clone(), (0..low.len()).find(|&i| is_name(i) || low[i] == "friend")) {
                e.subj = low[i].clone();
                r.events.push(e);
            }
            continue;
        }
        if ((on("equally") && text_low.contains("equally")) || (on("equal") && text_low.contains("equal "))) && !(on("rest") && text_low.contains("rest")) {
            let ns: Vec<i64> = (0..low.len()).filter_map(num).collect();
            if ns.len() >= 2 {
                r.share = Some((ns[0], ns[1]));
            }
            continue;
        }
        settle(&r, &mut fp, &mut pending, &mut used);
        pending.clear();
        for (p, clause) in &clauses {
            let p = *p;
            if clause.is_empty() {
                continue;
            }
            settle(&r, &mut fp, &mut pending, &mut used);
            pending = clause.iter().filter_map(|&i| num(i)).collect();
            let a = clause[0];
            let has = |w: &str| clause.iter().any(|&i| low[i] == w);
            let plemma = t.w[p].lemma.to_lowercase();
            // the subject: a name (or a noun that is not the counted one); the predicate's lemma, or the copula's
            // the subject; when the tree has none that is a name («Before that Max had 8»), a name before the
            // predicate in the clause
            let subj_i = t.subject(p).filter(|&x| !matches!(low[x].as_str(), "that" | "this")).or_else(|| clause.iter().copied().find(|&k| k < p && is_name(k) && t.base(k) != Rel::Nmod));
            let counted_subj = subj_i.is_some_and(|s| t.kids(s).any(|k| t.w[k].rel == Rel::Nummod && num(k).is_some()));
            // who acts or holds: a name, or a noun subject that is not what is counted («the pond holds», «a tank
            // holds»; MOVA_TREE_NOUNS=0: only the old few, friend, box, bag, shelf)
            let nouns_act = std::env::var("MOVA_TREE_NOUNS").as_deref() != Ok("0");
            let actor_i = subj_i.filter(|&s| is_name(s) || (!counted_subj && t.w[s].upos == UPos::NOUN && (matches!(low[s].as_str(), "friend" | "box" | "bag" | "shelf") || (nouns_act && !matches!(low[s].as_str(), "each" | "some" | "all" | "both")))));
            let is_cop = t.kids(p).any(|k| t.base(k) == Rel::Cop);
            let verb_lemma = if is_cop { "be".to_string() } else { plemma.clone() };
            let holds_here = holding_lemma(&verb_lemma) && !t.passive(p);
            // a holding told against another's: «Ben has 3 more apples than Anna», «twice as many … as Anna»
            if holds_here {
                let rel = clause.iter().copied().rev().find(|&i| {
                    is_name(i) && match t.case_of(i).as_deref() {
                        Some("than") => on("than"),
                        Some("as") => on("as many") && (has("many") || has("much")),
                        _ => false,
                    }
                });
                if let (Some(ai), Some(ri)) = (actor_i, rel) {
                    let k = clause.iter().copied().find_map(num);
                    let rel3: Option<(i64, i64, i64)> = if t.case_of(ri).as_deref() == Some("than") {
                        let d = k.unwrap_or(0);
                        if has("more") || has("older") || has("longer") || has("taller") { Some((1, 1, d)) } else if has("fewer") || has("less") || has("younger") || has("shorter") { Some((1, 1, -d)) } else { None }
                    } else if on("twice") && has("twice") {
                        Some((2, 1, 0))
                    } else if on("half") && has("half") {
                        Some((1, 2, 0))
                    } else if has("times") {
                        k.map(|k| (k, 1, 0))
                    } else {
                        None
                    };
                    if let Some((pp, q, d)) = rel3 {
                        let thing = r.holds.iter().find(|h| h.0 == low[ri]).map(|h| h.1.clone()).unwrap_or_else(|| {
                            // the compared noun: the one «more» / «many» stands on, else after the number
                            clause.iter().copied().find(|&i| t.w[i].upos == UPos::NOUN && t.kids(i).any(|k| matches!(low[k].as_str(), "more" | "fewer" | "many" | "less" | "much")) ).and_then(|i| school_norm(&low[i]))
                                .or_else(|| clause.iter().copied().find(|&i| num(i).is_some()).map(|i| thing_of(i)))
                                .unwrap_or_default()
                        });
                        match r.holds.iter().find(|h| h.0 == low[ri]).map(|h| h.2) {
                            Some(b) => r.holds.push((low[ai].clone(), thing, b * pp / q + d)),
                            None if on("relations") => r.rels.push((low[ai].clone(), low[ri].clone(), thing, pp, q, d)),
                            None => {}
                        }
                        last_subj = low[ai].clone();
                        continue;
                    }
                }
            }
            let subj_name = || actor_i.map(|i| school_norm(&low[i]).unwrap_or(low[i].clone())).unwrap_or_else(|| last_subj.clone());
            // the receiver: the indirect object, the «to» oblique (not «each of»), a name between the verb and number
            let to_of = || -> String {
                if let Some(k) = t.kids(p).find(|&k| t.base(k) == Rel::Iobj) {
                    return school_norm(&low[k]).unwrap_or(low[k].clone());
                }
                // «gives Lily six», «hands Sam 1 balloon», «gives Lily half of her stamps»: two objects, the
                // first a name — the tree may call both «obj»
                let objs: Vec<usize> = t.kids(p).filter(|&k| matches!(t.base(k), Rel::Obj | Rel::Nsubj) && k > p).collect();
                if objs.len() >= 2 && is_name(objs[0]) {
                    return school_norm(&low[objs[0]]).unwrap_or(low[objs[0]].clone());
                }
                // a name after the verb and before the number or the part, whatever the tree calls it («gives Lily
                // half»: compound; «gives Lily six»: nmod) — the old reader's idea, on the tree's order
                let first_n = clause.iter().copied().find(|&j| j > p && (num(j).is_some() || matches!(low[j].as_str(), "half" | "rest" | "third" | "quarter" | "each")));
                if let Some(fnum) = first_n {
                    if let Some(k) = clause.iter().copied().find(|&k| k > p && k < fnum && (is_name(k) || low[k] == "friend") && !matches!(t.base(k), Rel::Nsubj)) {
                        return school_norm(&low[k]).unwrap_or(low[k].clone());
                    }
                }
                if let Some(&k) = objs.first() {
                    // a name object right after the verb, with a number or a part after it in the clause
                    if is_name(k) && clause.iter().any(|&j| j > k && (num(j).is_some() || matches!(low[j].as_str(), "half" | "rest" | "third" | "quarter"))) {
                        return school_norm(&low[k]).unwrap_or(low[k].clone());
                    }
                }
                if let Some(k) = clause.iter().copied().find(|&k| on("to") && t.case_of(k).as_deref() == Some("to") && !matches!(low[k].as_str(), "each")) {
                    return school_norm(&low[k]).unwrap_or(low[k].clone());
                }
                String::new()
            };
            // parts: «gives half of his apples to Sam», «doubles her apples», «… the rest to Ben»
            let doubles = on("double") && clause.iter().any(|&i| matches!(low[i].as_str(), "double" | "doubles" | "doubled"));
            let part = clause.iter().copied().find_map(|i| {
                let den = match low[i].as_str() { "half" => 2, "third" | "thirds" => 3, "quarter" | "quarters" | "fourth" | "fourths" => 4, _ => return None };
                if den > 2 && !on("third") {
                    return None;
                }
                let k = if i > 0 { num(i - 1).unwrap_or(1) } else { 1 };
                Some((k, den))
            });
            let rest = on("rest") && has("rest");
            if rest && (has("equally") || has("among")) {
                if let Some(k) = clause.iter().copied().find_map(num) {
                    r.share_rest = Some((subj_name(), k));
                }
                continue;
            }
            if (on("half") && part.is_some() && !holds_here) || rest || doubles {
                let subj = subj_name();
                let verb = if doubles { "double".to_string() } else if holding_lemma(&verb_lemma) || verb_lemma.is_empty() { last.as_ref().map(|e| e.verb.clone()).unwrap_or_default() } else { verb_lemma.clone() };
                let to = to_of();
                let thing = r.holds.last().map(|h| h.1.clone()).unwrap_or_default();
                let frac = if rest || doubles { (1, 1) } else { part.unwrap_or((1, 1)) };
                let e = Ev { verb, subj: subj.clone(), to, n: 0, thing, more: doubles, frac: Some(frac), keep: if has("rest") { r.kept } else { 0 }, unknown: false };
                r.kept = 0;
                last = Some(e.clone());
                last_subj = subj;
                r.events.push(e);
                continue;
            }
            // «Tom had some apples»: the start unknown; «found some more», «lost a few of them»: the change unknown
            if on("some") && clause.iter().all(|&i| num(i).is_none()) && clause.windows(2).any(|q| low[q[0]] == "some" || (low[q[0]] == "a" && low[q[1]] == "few")) {
                let subj = subj_name();
                if holding_lemma(&verb_lemma) {
                    r.unknown_start = Some(subj.clone());
                    let thing = clause.iter().rev().find_map(|&i| school_norm(&low[i])).unwrap_or_default();
                    r.holds.push((subj.clone(), thing, 0));
                } else {
                    let e = Ev { verb: verb_lemma.clone(), subj: subj.clone(), more: has("more"), unknown: true, ..Default::default() };
                    last = Some(e.clone());
                    r.events.push(e);
                }
                last_subj = subj;
                continue;
            }
            // «The red ribbon is 14 cm long», «The blue ribbon is 2 cm shorter»: things measured are actors too
            if on("measure") && actor_i.is_none_or(|x| !is_name(x)) && is_cop {
                if let Some(s) = subj_i.filter(|&s| t.w[s].upos == UPos::NOUN) {
                    let mut words: Vec<String> = t.kids(s).filter(|&k| matches!(t.base(k), Rel::Amod | Rel::Compound)).map(|k| low[k].clone()).collect();
                    words.push(low[s].clone());
                    let name = object_name(&words);
                    let n = clause.iter().copied().find_map(num);
                    let cmp = clause.iter().find_map(|&i| match low[i].as_str() { "longer" | "taller" | "heavier" | "older" => Some(1), "shorter" | "lighter" | "younger" => Some(-1), _ => None });
                    if let Some(n) = n {
                        let v = match cmp {
                            Some(sg) if !has("than") => r.holds.last().map(|h| h.2 + sg * n),
                            _ => Some(n),
                        };
                        if let Some(v) = v {
                            r.holds.push((name.clone(), String::new(), v));
                            last_subj = name;
                            continue;
                        }
                    }
                }
            }
            let Some(&ni) = clause.iter().find(|&&i| num(i).is_some()) else { continue };
            let n = num(ni).unwrap_or(0);
            let thing = thing_of(ni);
            // «2 chairs per table … 4 tables»: so many for one unit, and later how many units
            if on("each") {
                let c = t.counted(ni);
                if let Some(u) = c.and_then(|c| t.kids(c).find(|&k| matches!(t.case_of(k).as_deref(), Some("per")) || (t.base(k) == Rel::Nmod && t.kids(k).any(|j| matches!(low[j].as_str(), "per"))))) {
                    if let Some(un) = school_norm(&low[u]) {
                        per_unit = Some((n, un));
                    }
                }
                if let Some((k, un)) = per_unit.clone() {
                    if thing == un && c.is_some() {
                        r.groups.push((k, n));
                        per_unit = None;
                        continue;
                    }
                }
            }
            // «holds 2 times as much water», «twice as many» with no one named to compare: against the last holding
            if holds_here && (has("as") || has("than")) && !clause.iter().any(|&i| is_name(i) && matches!(t.case_of(i).as_deref(), Some("as" | "than"))) {
                let factor = if has("times") { Some((n, 1)) } else { None };
                if let (Some((pp, q)), Some(last_h)) = (factor, r.holds.last().cloned()) {
                    let subj = subj_name();
                    r.holds.push((subj.clone(), last_h.1.clone(), last_h.2 * pp / q));
                    last_subj = subj;
                    continue;
                }
            }
            // prices: «A pencil costs 3 cents»; «buys 4 books at 8 cents each»
            if on("cost") && clause.iter().any(|&i| low[i].starts_with("cent")) {
                if matches!(plemma.as_str(), "cost") {
                    let item = subj_i.and_then(|s| school_norm(&low[s])).or_else(|| clause.iter().copied().filter(|&i| i < p).rev().find_map(|i| school_norm(&low[i]).filter(|w| !matches!(w.as_str(), "a" | "one" | "each"))));
                    if let Some(item) = item {
                        r.prices.insert(item, n);
                    }
                    continue;
                }
                if let Some(ci) = clause.iter().copied().find(|&i| low[i].starts_with("cent") && matches!(t.case_of(i).as_deref(), Some("at" | "for"))) {
                    let k = n;
                    let price = t.kids(ci).find_map(|x| num(x)).unwrap_or(0);
                    let subj = subj_name();
                    let e = Ev { verb: "pay".into(), subj: subj.clone(), n: k * price, thing: "cent".into(), ..Default::default() };
                    last = Some(e.clone());
                    last_subj = subj;
                    r.events.push(e);
                    continue;
                }
            }
            // a rate: «reads 3 pages every day», «saves 4 cents a day»
            if on("week") && clause.iter().any(|&i| low[i] == "day" && t.kids(i).any(|k| matches!(low[k].as_str(), "every" | "each" | "a" | "per"))) {
                r.rate = Some(n);
                last_subj = subj_name();
                continue;
            }
            // «26 eggs are packed in boxes of 6», «puts 26 eggs into boxes that hold 6 each»
            if on("full") && (has("into") || has("packed")) && clause.iter().any(|&i| matches!(low[i].as_str(), "boxes" | "bags" | "packs")) {
                let ns: Vec<i64> = clause.iter().filter_map(|&i| num(i)).collect();
                if ns.len() >= 2 {
                    r.pack = Some((ns[0], ns[1]));
                    continue;
                }
            }
            // «Each pack holds 4 cards» after «buys 3 packs of cards»: what was bought is the packs' contents
            if on("pack") {
                if let Some(e) = r.events.last_mut().filter(|e| matches!(e.thing.as_str(), "pack" | "box" | "bag" | "packet")) {
                    if clause.iter().any(|&i| school_norm(&low[i]).as_deref() == Some(e.thing.as_str())) {
                        e.n *= n;
                        e.thing = thing.clone();
                        continue;
                    }
                }
            }
            // «gives 2 apples to each of his 3 friends», «gives each of her 3 friends 2 apples»
            if on("each of") && actor_i.is_some() && clause.windows(2).any(|q| low[q[0]] == "each" && low[q[1]] == "of") && !holds_here {
                let total: i64 = clause.iter().filter_map(|&i| num(i)).product();
                let subj = subj_name();
                let th = r.holds.iter().find(|h| h.0 == subj).map(|h| h.1.clone()).unwrap_or_else(|| thing.clone());
                let e = Ev { verb: verb_lemma.clone(), subj: subj.clone(), n: total, thing: th, ..Default::default() };
                last = Some(e.clone());
                last_subj = subj;
                r.events.push(e);
                continue;
            }
            // groups: «N tables with M pencils on each table», «N boxes with M in each box»
            if clause.iter().any(|&i| on(&low[i]) && matches!(low[i].as_str(), "each" | "every")) && !(subj_i.is_some_and(|s| low[s] == "each" || t.kids(s).any(|k| low[k] == "each"))) {
                let ns: Vec<i64> = clause.iter().filter_map(|&i| num(i)).collect();
                if ns.len() >= 2 {
                    r.groups.push((ns[0], ns[1]));
                    continue;
                }
            }
            // «there are N T on the table»: the place holds them
            if on("there") && t.kids(p).any(|k| t.base(k) == Rel::Expl) {
                // each count with its own place: «9 balloons on the shelf and 7 balloons on the desk»
                let place_of = |j: usize| -> Option<String> {
                    let c = t.counted(j).unwrap_or(j);
                    // its own place in the tree, else the place phrase that follows it before the next count (the
                    // tree may hang «on the desk» on the verb)
                    let next_num = clause.iter().copied().find(|&k| k > j && num(k).is_some()).unwrap_or(usize::MAX);
                    t.kids(c).find(|&k| matches!(t.case_of(k).as_deref(), Some("on" | "in" | "at")))
                        .or_else(|| clause.iter().copied().find(|&k| k > j && k < next_num && matches!(t.case_of(k).as_deref(), Some("on" | "in" | "at"))))
                        .map(|k| school_norm(&low[k]).unwrap_or(low[k].clone()))
                };
                let counted = t.counted(ni).unwrap_or(ni);
                let place = place_of(ni).or_else(|| clause.iter().copied().find(|&i| i != counted && matches!(t.case_of(i).as_deref(), Some("on" | "in" | "at"))).map(|i| school_norm(&low[i]).unwrap_or(low[i].clone()))).unwrap_or_else(|| "it".into());
                r.holds.push((place.clone(), thing.clone(), n));
                for j in clause.iter().copied().filter(|&j| j != ni && num(j).is_some()) {
                    let th = thing_of(j);
                    r.holds.push((place_of(j).unwrap_or_else(|| place.clone()), if th.is_empty() { thing.clone() } else { th }, num(j).unwrap_or(0)));
                }
                continue;
            }
            // «each bag has M» after «N bags»; «each of N children has M»
            if on("each") && subj_i.is_some_and(|s| low[s] == "each" || t.kids(s).any(|k| low[k] == "each")) {
                let ns: Vec<i64> = clause.iter().filter_map(|&i| num(i)).collect();
                if ns.len() >= 2 {
                    r.groups.push((ns[0], ns[1]));
                } else if let Some(k) = r.holds.last().map(|h| h.2).or_else(|| r.events.last().map(|e| e.n)) {
                    r.groups.push((k, n));
                }
                continue;
            }
            // «16 pencils broke»: the number counts the subject — the last actor's loss
            if counted_subj && subj_i.is_some_and(|s| t.counted(ni) == Some(s)) && !holding_lemma(&verb_lemma) {
                let e = Ev { verb: verb_lemma.clone(), subj: last_subj.clone(), to: String::new(), n, thing: thing.clone(), ..Default::default() };
                last = Some(e.clone());
                r.events.push(e);
                continue;
            }
            // «A pack holds 4 cards» after «Tom buys 3 packs»: a group
            if subj_i.is_some_and(|s| t.kids(s).any(|k| matches!(low[k].as_str(), "a" | "one"))) && matches!(plemma.as_str(), "hold" | "have" | "contain") && actor_i.is_none() {
                if let Some(k) = r.events.last().map(|e| e.n).or_else(|| r.holds.last().map(|h| h.2)) {
                    r.groups.push((k, n));
                    continue;
                }
            }
            let subj = if counted_subj && actor_i.is_none() { last_subj.clone() } else { subj_name() };
            let passive = t.passive(p);
            // the clause's other numbers on conjoined nouns («6 big balloons and 9 small balloons», «7 pairs …, and
            // next to them 4 single ones»): the same reading, each with its count
            let more_nums: Vec<(i64, String)> = clause.iter().copied().filter(|&j| j != ni && num(j).is_some()).filter(|&j| {
                let c = t.counted(j).unwrap_or(j);
                t.base(c) == Rel::Conj || t.base(j) == Rel::Conj
            }).map(|j| (num(j).unwrap_or(0), thing_of(j))).collect();
            if holds_here {
                r.holds.push((subj.clone(), thing.clone(), n));
                for (m, th) in &more_nums {
                    r.holds.push((subj.clone(), if th.is_empty() || th == "one" { thing.clone() } else { th.clone() }, *m));
                }
                last_subj = subj;
                continue;
            }
            let verb = verb_lemma.clone();
            let to = to_of();
            if on("keep") && verb == "keep" {
                r.kept = n;
                last_subj = subj;
                continue;
            }
            // «receives 1 from Zoe», «is given 2 by Zoe»: the other one passes them to the actor
            let source = clause.iter().copied().find(|&k| match t.case_of(k).as_deref() {
                Some("from") => on("from"),
                Some("by") => on("by") && passive,
                _ => false,
            }).map(|k| school_norm(&low[k]).unwrap_or(low[k].clone()));
            let more = on("more") && (t.counted(ni).is_some_and(|h| t.kids(h).any(|k| low[k] == "more")) || low.get(ni + 1).is_some_and(|w| w == "more"));
            let e = match source {
                Some(src) if to.is_empty() => Ev { verb: "give".into(), subj: src, to: subj.clone(), n, thing, ..Default::default() },
                _ => Ev { verb, subj: subj.clone(), to, n, thing, more, ..Default::default() },
            };
            let _ = a;
            for (m, th) in &more_nums {
                let mut e2 = e.clone();
                e2.n = *m;
                if !th.is_empty() && th != "one" {
                    e2.thing = th.clone();
                }
                r.events.push(e2);
            }
            last = Some(e.clone());
            last_subj = subj;
            r.events.insert(r.events.len() - more_nums.len(), e);
        }
    }
    // the waiting relations, read either way until nothing more follows
    loop {
        let mut changed = false;
        for (a, b, thing, p, q, d) in r.rels.clone() {
            let ha = r.holds.iter().find(|h| h.0 == a).map(|h| h.2);
            let hb = r.holds.iter().find(|h| h.0 == b).map(|h| h.2);
            match (ha, hb) {
                (None, Some(vb)) => {
                    r.holds.push((a.clone(), thing.clone(), vb * p / q + d));
                    changed = true;
                }
                (Some(va), None) if p != 0 => {
                    r.holds.push((b.clone(), thing.clone(), (va - d) * q / p));
                    changed = true;
                }
                _ => {}
            }
        }
        if !changed {
            break;
        }
    }
    if !r.rels.is_empty() {
        used.extend(r.rels.iter().map(|x| x.5.abs()));
    }
    settle(&r, &mut fp, &mut pending, &mut used);
    r.used = used;
    if on("things") {
        let mut last_of: BTreeMap<String, String> = BTreeMap::new();
        let mut last_any = String::new();
        for (who, th, _) in &r.holds {
            if !th.is_empty() {
                last_of.insert(who.clone(), th.clone());
                last_any = th.clone();
            }
        }
        for e in r.events.iter_mut() {
            if e.thing.is_empty() || matches!(e.thing.as_str(), "them" | "it" | "one" | "some") {
                if let Some(th) = last_of.get(&e.subj).cloned().or_else(|| (!last_any.is_empty()).then(|| last_any.clone())) {
                    e.thing = th;
                }
            } else {
                last_of.insert(e.subj.clone(), e.thing.clone());
                last_any = e.thing.clone();
            }
        }
    }
    if !r.prices.is_empty() {
        for e in r.events.iter_mut() {
            let item = if matches!(e.thing.as_str(), "them" | "it" | "") && r.prices.len() == 1 { r.prices.keys().next().cloned().unwrap_or_default() } else { e.thing.clone() };
            if let Some(p) = r.prices.get(&item) {
                *e = Ev { verb: "pay".into(), subj: e.subj.clone(), n: e.n * p, thing: "cent".into(), ..Default::default() };
            }
        }
    }
    r
}
