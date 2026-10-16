//! Generator: fact → sentence, deterministically.
//! Head members (predicate, subject, objects, adverbials, particles) are chunks; attributes sit in the chunk
//! of their head: adjectives before it, genitives after; the preposition, quantity word and "не" come from
//! the relation and the flag. Chunk order is a Markov chain over classes (bigrams with <s> and </s>), trained
//! on parsed sentences; the best permutation is found by dynamic programming over subsets. Forms come from VESUM synthesis.

use std::collections::HashMap;

use crate::gram::*;
use crate::morph::{Morph, norm};
use crate::uk::{Fact, Member};

/// Chunk class for ordering: the relation without details, except the object's case.
pub fn class(rel: &str) -> &str {
    if rel.starts_with("adverbial:") {
        "adverbial:prep"
    } else if rel.starts_with("quantity:") {
        "quantity"
    } else {
        rel
    }
}

fn is_modifier(m: &Member) -> bool {
    m.rel.starts_with("attribute:")
}

#[derive(Default)]
pub struct OrderModel {
    bigram: HashMap<(String, String), f64>,
    from: HashMap<String, f64>,
}

impl OrderModel {
    /// Surface order of chunk classes in a parsed sentence — by the position of the head word.
    pub fn observe(&mut self, f: &Fact) {
        let mut heads: Vec<(usize, &str)> =
            f.members.iter().filter(|m| !is_modifier(m)).filter_map(|m| m.word.map(|w| (w, class(&m.rel)))).collect();
        heads.sort();
        let mut prev = "<s>";
        for c in heads.iter().map(|(_, c)| *c).chain(["</s>"]) {
            *self.bigram.entry((prev.to_string(), c.to_string())).or_default() += 1.0;
            *self.from.entry(prev.to_string()).or_default() += 1.0;
            prev = c;
        }
    }

    fn logp(&self, a: &str, b: &str) -> f64 {
        const ALPHA: f64 = 0.1;
        const CLASSES: f64 = 32.0;
        let n = self.bigram.get(&(a.to_string(), b.to_string())).copied().unwrap_or(0.0);
        let d = self.from.get(a).copied().unwrap_or(0.0);
        ((n + ALPHA) / (d + ALPHA * CLASSES)).ln()
    }

    /// Most probable class order: dynamic programming over subsets (up to 12 chunks).
    fn best_order(&self, classes: &[&str]) -> Vec<usize> {
        let n = classes.len();
        let full = (1usize << n) - 1;
        let mut dp = vec![vec![f64::NEG_INFINITY; n]; 1 << n];
        let mut back = vec![vec![usize::MAX; n]; 1 << n];
        for i in 0..n {
            dp[1 << i][i] = self.logp("<s>", classes[i]);
        }
        for mask in 1..=full {
            for last in 0..n {
                let cur = dp[mask][last];
                if cur == f64::NEG_INFINITY {
                    continue;
                }
                for next in 0..n {
                    if mask & (1 << next) != 0 {
                        continue;
                    }
                    let v = cur + self.logp(classes[last], classes[next]);
                    let m2 = mask | (1 << next);
                    if v > dp[m2][next] {
                        dp[m2][next] = v;
                        back[m2][next] = last;
                    }
                }
            }
        }
        let mut last = (0..n)
            .max_by(|&a, &b| {
                (dp[full][a] + self.logp(classes[a], "</s>")).total_cmp(&(dp[full][b] + self.logp(classes[b], "</s>")))
            })
            .unwrap_or(0);
        let (mut mask, mut order) = (full, Vec::with_capacity(n));
        loop {
            order.push(last);
            let prev = back[mask][last];
            mask &= !(1 << last);
            if prev == usize::MAX {
                break;
            }
            last = prev;
        }
        order.reverse();
        order
    }
}

/// Frequencies for choosing among VESUM variants ("одружимося" vs "одружимось", "рибок" vs "рибки"):
/// first — how many times the form appeared for exactly this (lemma, grammar) in training parses,
/// then — the overall corpus frequency of the form; on a tie — the first one in the dictionary.
#[derive(Default)]
pub struct Freq {
    pub cond: HashMap<(String, u32), HashMap<String, u32>>,
    pub word: HashMap<String, u32>,
}

impl Freq {
    pub fn new() -> Self {
        Self::default()
    }
    /// Members of a training fact: (lemma, grammar) → normalized form from the sentence.
    pub fn observe(&mut self, f: &Fact, words: &[String]) {
        for mem in &f.members {
            if let Some(w) = mem.word.and_then(|w| words.get(w)) {
                let key = (mem.lemma.clone(), mem.gram.0 & !(1 << NEG) & !(1 << IMPLICIT));
                *self.cond.entry(key).or_default().entry(w.clone()).or_default() += 1;
            }
        }
    }
    fn score(&self, lemma: &str, gram: u32, form: &str) -> (u32, u32) {
        let n = norm(form);
        let c = self.cond.get(&(lemma.to_string(), gram)).and_then(|m| m.get(&n)).copied().unwrap_or(0);
        (c, self.word.get(&n).copied().unwrap_or(0))
    }
}

/// Form from (lemma, grammar). After a preposition a 3rd-person pronoun takes the н-form ("в них", not "в їх").
fn form(m: &Morph, freq: &Freq, mem: &Member, after_prep: bool) -> Option<String> {
    let key = Gram(mem.gram.0 & !(1 << NEG) & !(1 << IMPLICIT));
    let forms = m.synth.get(&(mem.lemma.clone(), key))?;
    let n_form = |f: &&String| f.starts_with('н') || f.starts_with('Н');
    let pool: Vec<&String> = if after_prep && mem.gram.flag(PRON) && mem.gram.person() == 3 && forms.iter().any(|f| n_form(&f)) {
        forms.iter().filter(n_form).collect()
    } else {
        forms.iter().collect()
    };
    let score = |f: &&String| freq.score(&mem.lemma, key.0, f);
    let best = pool.iter().copied().fold(None::<&String>, |b, f| match b {
        Some(b) if score(&b) >= score(&f) => Some(b),
        _ => Some(f),
    })?;
    Some(best.clone())
}

/// Chunk: [preposition/quantity word] ["не"] [adjective attributes] head [genitive attributes with their chunks].
/// `used` — attributes already placed in another chunk (each is placed once).
fn chunk(h: &Member, f: &Fact, m: &Morph, freq: &Freq, used: &mut Vec<bool>, depth: usize) -> Option<Vec<String>> {
    if depth > 4 {
        return None;
    }
    let mut words = Vec::new();
    let prep = h.rel.strip_prefix("adverbial:").and_then(|r| r.split_once('+')).map(|(p, _)| p);
    if let Some(prep) = prep {
        words.push(prep.to_string());
    }
    if let Some(q) = h.rel.strip_prefix("quantity:") {
        words.push(q.to_string());
    }
    if h.gram.flag(NEG) {
        words.push("не".into());
    }
    let own = format!("attribute:{}", h.rel);
    let mut pre: Vec<usize> = (0..f.members.len()).filter(|&i| !used[i] && f.members[i].rel == own && f.members[i].gram.pos() == ADJ).collect();
    pre.sort_by_key(|&i| !f.members[i].gram.flag(PRON)); // "свій старий ноутбук": pronominal ones first
    for i in pre {
        used[i] = true;
        words.push(form(m, freq, &f.members[i], false)?);
    }
    words.push(form(m, freq, h, prep.is_some())?);
    let post: Vec<usize> = (0..f.members.len()).filter(|&i| !used[i] && f.members[i].rel == own && f.members[i].gram.pos() != ADJ).collect();
    for i in post {
        used[i] = true;
        words.extend(chunk(&f.members[i], f, m, freq, used, depth + 1)?);
    }
    Some(words)
}

pub fn generate(f: &Fact, m: &Morph, freq: &Freq, om: &OrderModel) -> Option<String> {
    let heads: Vec<&Member> = f.members.iter().filter(|x| !is_modifier(x) && !x.gram.flag(IMPLICIT)).collect();
    if heads.is_empty() || heads.len() > 12 {
        return None;
    }
    let mut used = vec![false; f.members.len()];
    let chunks: Vec<Vec<String>> = heads.iter().map(|h| chunk(h, f, m, freq, &mut used, 0)).collect::<Option<_>>()?;
    let classes: Vec<&str> = heads.iter().map(|h| class(&h.rel)).collect();
    let order = om.best_order(&classes);
    let mut parts: Vec<String> = Vec::new();
    for (k, &i) in order.iter().enumerate() {
        // "Київ — столиця": verbless sentence, both nouns in the nominative, subject is not a pronoun
        if k > 0 && f.gram == Gram(0) {
            let prev = heads[order[k - 1]];
            let cur = heads[i];
            let pair = |a: &Member, b: &Member| a.rel == "subject" && b.rel == "predicate";
            if (pair(prev, cur) || pair(cur, prev))
                && prev.gram.pos() == NOUN
                && cur.gram.pos() == NOUN
                && !prev.gram.flag(PRON)
                && !cur.gram.flag(PRON)
            {
                parts.push("—".into());
            }
        }
        parts.extend(chunks[i].iter().cloned());
    }
    let mut s = parts.join(" ");
    if let Some(first) = s.chars().next() {
        s = first.to_uppercase().collect::<String>() + &s[first.len_utf8()..];
    }
    s.push(if f.gram.tense() == IMPR { '!' } else { '.' });
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member(rel: &str, word: usize, lemma: &str, tags: &str) -> Member {
        Member { rel: rel.into(), word: Some(word), lemma: lemma.into(), gram: Gram::from_vesum(tags) }
    }

    #[test]
    fn subject_verb_object() {
        let mut m = Morph::default();
        let facts = [("мама|noun", "noun:anim:f:v_naz", "мама"), ("читати|verb", "verb:imperf:pres:s:3", "читає"), ("книжка|noun", "noun:inanim:f:v_zna", "книжку")];
        for (l, t, f) in facts {
            m.synth.insert((l.into(), Gram::from_vesum(t)), vec![f.into()]);
        }
        let f = Fact {
            gram: Gram::from_vesum("verb:imperf:pres:s:3"),
            members: vec![
                member("predicate", 1, "читати|verb", "verb:imperf:pres:s:3"),
                member("subject", 0, "мама|noun", "noun:anim:f:v_naz"),
                member("object:v_zna", 2, "книжка|noun", "noun:inanim:f:v_zna"),
            ],
        };
        let mut om = OrderModel::default();
        om.observe(&f);
        assert_eq!(generate(&f, &m, &Freq::new(), &om).as_deref(), Some("Мама читає книжку."));
    }
}
