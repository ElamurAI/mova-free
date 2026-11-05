//! CoNLL-U in and out, vocabularies, word features, batches.

use std::collections::BTreeMap;
use std::fmt::Write as _;

#[derive(Clone, Debug)]
pub struct Word {
    pub form: String,
    /// LEMMA and FEATS as read (the parser does not predict them; `annotate` keeps or refills them)
    pub lemma: String,
    pub feats: String,
    pub upos: String,
    pub xpos: String,
    pub head: usize,
    pub rel: String,
    /// MISC as read; training reads the agreement vote from it (`Vote`, `VoteHead`, `VoteAlt`, see [`vote`]).
    /// `write` does not carry it (the parser's output has no vote).
    pub misc: String,
}

#[derive(Clone, Debug)]
pub struct Sent {
    pub comments: Vec<String>,
    pub words: Vec<Word>,
    /// pragmatic roles read back from the reader (`prag roles`): (dependent, predicate, role id), 1-based
    pub roles: Vec<(usize, usize, u8)>,
}

impl Sent {
    pub fn id(&self) -> &str {
        self.comments.iter().find_map(|c| c.strip_prefix("# sent_id = ")).unwrap_or("")
    }
}

/// The reader's roles (`prag::roles::ROLES`, same order); id 0 is "none".
pub const ROLES: [&str; 7] = ["agent", "patient", "recipient", "place", "attr", "obl", "quote"];

/// Attach `prag roles` rows (sent_id, dep, pred, role) to the sentences; returns how many were attached.
pub fn attach_roles(sents: &mut [Sent], tsv: &str) -> usize {
    let mut by: BTreeMap<&str, Vec<(usize, usize, u8)>> = BTreeMap::new();
    for l in tsv.lines().skip(1) {
        let c: Vec<&str> = l.split('\t').collect();
        if let (4, Some(r)) = (c.len(), ROLES.iter().position(|x| *x == c.get(3).copied().unwrap_or(""))) {
            if let (Ok(d), Ok(p)) = (c[1].parse(), c[2].parse()) {
                by.entry(c[0]).or_default().push((d, p, r as u8 + 1));
            }
        }
    }
    let mut n = 0;
    for s in sents.iter_mut() {
        if let Some(v) = by.get(s.id()) {
            s.roles = v.iter().copied().filter(|&(d, p, _)| d >= 1 && p >= 1 && d <= s.words.len() && p <= s.words.len()).collect();
            n += s.roles.len();
        }
    }
    n
}

/// Plain word lines only: multiword ranges ("3-4") and empty nodes ("5.1") are skipped.
pub fn read(src: &str) -> Vec<Sent> {
    let mut out = Vec::new();
    let mut cur = Sent { comments: vec![], words: vec![], roles: vec![] };
    for l in src.lines().chain(std::iter::once("")) {
        if l.is_empty() {
            if !cur.words.is_empty() {
                out.push(std::mem::replace(&mut cur, Sent { comments: vec![], words: vec![], roles: vec![] }));
            }
            cur.comments.clear();
        } else if l.starts_with('#') {
            cur.comments.push(l.to_string());
        } else {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() < 8 || c[0].contains('-') || c[0].contains('.') {
                continue;
            }
            cur.words.push(Word { form: c[1].into(), lemma: c[2].into(), feats: c[5].into(), upos: c[3].into(), xpos: c[4].into(), head: c[6].parse().unwrap_or(0), rel: c[7].into(), misc: c.get(9).map_or("_", |m| m).into() });
        }
    }
    out
}

pub fn write(sents: &[Sent]) -> String {
    let mut s = String::new();
    for x in sents {
        for c in &x.comments {
            writeln!(s, "{c}").unwrap();
        }
        for (i, w) in x.words.iter().enumerate() {
            writeln!(s, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t_\t_", i + 1, w.form, w.lemma, w.upos, w.xpos, w.feats, w.head, w.rel).unwrap();
        }
        s.push('\n');
    }
    s
}

/// A closed list of names with ids (0 is reserved where a padding/unknown value is needed).
#[derive(Clone, Debug, Default)]
pub struct Voc {
    pub names: Vec<String>,
    pub ids: BTreeMap<String, u32>,
}

impl Voc {
    pub fn with(reserved: &[&str]) -> Voc {
        let mut v = Voc::default();
        for r in reserved {
            v.add(r);
        }
        v
    }
    pub fn add(&mut self, s: &str) -> u32 {
        if let Some(&i) = self.ids.get(s) {
            return i;
        }
        let i = self.names.len() as u32;
        self.names.push(s.to_string());
        self.ids.insert(s.to_string(), i);
        i
    }
    pub fn get(&self, s: &str) -> Option<u32> {
        self.ids.get(s).copied()
    }
    pub fn len(&self) -> usize {
        self.names.len()
    }
    pub fn save(&self) -> String {
        self.names.join("\n")
    }
    pub fn load(s: &str) -> Voc {
        let mut v = Voc::default();
        for l in s.split('\n') {
            v.add(l);
        }
        v
    }
}

pub const UNK: u32 = 1;
pub const ROOT: u32 = 2;
/// Hashed affix buckets.
pub const AFFIX: usize = 1 << 15;

/// FNV-1a: affixes hash into fixed buckets (no table to store; same on every machine).
fn fnv(s: &str, salt: u64) -> u32 {
    let mut h: u64 = 0xcbf29ce484222325 ^ salt;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    (h % (AFFIX as u64 - 3)) as u32 + 3
}

/// Word shape: 0 pad, 1 lower, 2 Capitalized, 3 UPPER, 4 digits, 5 has digit, 6 punctuation, 7 other, 8 root.
pub fn shape(w: &str) -> u32 {
    let cs: Vec<char> = w.chars().collect();
    if cs.iter().all(|c| c.is_ascii_digit() || matches!(c, ',' | '.')) && cs.iter().any(|c| c.is_ascii_digit()) {
        4
    } else if cs.iter().any(|c| c.is_ascii_digit()) {
        5
    } else if cs.iter().all(|c| !c.is_alphanumeric()) {
        6
    } else if cs.iter().all(|c| !c.is_lowercase()) && cs.len() > 1 {
        3
    } else if cs[0].is_uppercase() {
        2
    } else if cs.iter().all(|c| c.is_lowercase() || *c == '-' || *c == '\'') {
        1
    } else {
        7
    }
}

/// Input features of one word: word id, prefix-3 and suffix-3 buckets, shape.
pub fn feats(words: &Voc, w: &str) -> [u32; 4] {
    let lw = w.to_lowercase();
    let cs: Vec<char> = lw.chars().collect();
    let pre: String = cs.iter().take(3).collect();
    let suf: String = cs[cs.len().saturating_sub(3)..].iter().collect();
    [words.get(&lw).unwrap_or(UNK), fnv(&pre, 1), fnv(&suf, 2), shape(w)]
}

/// The most character n-grams one word contributes (`GP_SUBWORD`); a longer word keeps its first ones, shorter n
/// first («<in», «int», …, then 4-grams, …).
pub const NGRAM_CAP: usize = 32;

/// Default number of hashed n-gram buckets of a subword model (`GP_NGRAM_BUCKETS`).
pub const NGRAM_BUCKETS: usize = 1 << 18;

/// Hashed character n-grams of length 3–6 of the lowercased word with boundary marks («<word>»), fastText-style
/// (Bojanowski et al. 2017, TACL 5, section 3.2: a word is the bag of its character n-grams, hashed with FNV-1a into
/// K buckets). Bucket ids are 1..buckets; 0 is padding. At most [`NGRAM_CAP`] per word.
pub fn ngrams(w: &str, buckets: usize) -> Vec<u32> {
    let cs: Vec<char> = std::iter::once('<').chain(w.to_lowercase().chars()).chain(std::iter::once('>')).collect();
    let mut out = Vec::new();
    for n in 3..=6 {
        for i in 0..(cs.len() + 1).saturating_sub(n) {
            if out.len() == NGRAM_CAP {
                return out;
            }
            let g: String = cs[i..i + n].iter().collect();
            out.push(fnv_mod(&g, 3, buckets));
        }
    }
    out
}

/// FNV-1a into 1..buckets (0 is reserved for padding).
fn fnv_mod(s: &str, salt: u64, buckets: usize) -> u32 {
    let mut h: u64 = 0xcbf29ce484222325 ^ salt;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    (h % (buckets as u64 - 1)) as u32 + 1
}

/// The agreement vote over a word's (head, relation), as `cycle db-agree` keeps it in MISC: `Vote` — the share of the
/// chosen (head, relation) pair, `VoteHead` — the share of the chosen head whatever the relation (defaults to
/// `Vote`), `VoteAlt` — the other heads with their shares (`5:0.17,7:0.05`). A word with no `Vote` was not voted on
/// (a gold or single-version tree) and is fully trusted.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vote {
    pub pair: f32,
    pub head: f32,
    pub alts: Vec<(usize, f32)>,
}

pub fn vote(misc: &str) -> Option<Vote> {
    let (mut pair, mut head, mut alts) = (None, None, Vec::new());
    for kv in misc.split('|') {
        match kv.split_once('=') {
            Some(("Vote", v)) => pair = v.parse::<f32>().ok(),
            Some(("VoteHead", v)) => head = v.parse::<f32>().ok(),
            Some(("VoteAlt", v)) => {
                alts = v.split(',').filter_map(|a| a.split_once(':')).filter_map(|(h, s)| Some((h.parse().ok()?, s.parse().ok()?))).collect()
            }
            _ => {}
        }
    }
    let pair = pair.filter(|p| p.is_finite())?.clamp(0., 1.);
    Some(Vote { pair, head: head.filter(|h| h.is_finite()).map_or(pair, |h| h.clamp(0., 1.)), alts })
}

/// Model vocabularies, built from the training data.
pub struct Vocs {
    pub words: Voc,
    pub xpos: Voc,
    pub upos: Voc,
    pub rels: Voc,
}

impl Vocs {
    pub fn build(train: &[Sent], min_count: usize) -> Vocs {
        let mut cnt: BTreeMap<String, usize> = BTreeMap::new();
        for w in train.iter().flat_map(|s| &s.words) {
            *cnt.entry(w.form.to_lowercase()).or_default() += 1;
        }
        let mut words = Voc::with(&["<pad>", "<unk>", "<root>"]);
        for (w, n) in &cnt {
            if *n >= min_count {
                words.add(w);
            }
        }
        let (mut xpos, mut upos, mut rels) = (Voc::with(&["_"]), Voc::with(&["_"]), Voc::with(&["_"]));
        for w in train.iter().flat_map(|s| &s.words) {
            xpos.add(&w.xpos);
            upos.add(&w.upos);
            rels.add(&w.rel);
        }
        Vocs { words, xpos, upos, rels }
    }
}

/// A batch: B sentences padded to T = 1 + longest (position 0 is the root).
pub struct Batch {
    pub b: usize,
    pub t: usize,
    /// (B*T*4) input features
    pub inp: Vec<u32>,
    /// (B*T) 1 for real words (not root, not pad)
    pub mask: Vec<f32>,
    /// gold per real word, in row order b*T + i: head, rel, xpos, upos
    pub rows: Vec<u32>,
    pub heads: Vec<u32>,
    pub rels: Vec<u32>,
    pub xpos: Vec<u32>,
    pub upos: Vec<u32>,
    /// (B*T) 1 where the key may be attended / be a head (root and words)
    pub keys: Vec<f32>,
    /// (B*T) pragmatic token feature: its role as a filler (0–7) + 8 when it is a predicate
    pub role_tok: Vec<u32>,
    /// (B*T*T) the role linking dependent i to predicate j (0 — none)
    pub role_arc: Vec<u32>,
    /// n-gram buckets per position, `g` of them (B*T*g, 0 — padding); `g` is 0 for a model without subword input
    pub ngrams: Vec<u32>,
    pub g: usize,
    /// per row (as `rows`): the vote share of the gold head and of the gold (head, relation); 1 when not voted
    pub arc_conf: Vec<f32>,
    pub rel_conf: Vec<f32>,
    /// per row: the other heads the vote saw, with their shares (empty when not voted)
    pub alts: Vec<Vec<(u32, f32)>>,
}

/// Deterministic xorshift for shuffles and word dropout.
pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    pub fn unit(&mut self) -> f32 {
        (self.next() >> 40) as f32 / (1u64 << 24) as f32
    }
}

/// `prag_drop`: the share of sentences whose roles are hidden (the parser must also work on the first pass, with
/// no pragmatics yet).
pub fn batch(sents: &[&Sent], v: &Vocs, drop: f32, rng: &mut Rng) -> Batch {
    batch_prag(sents, v, drop, 0.0, rng)
}

pub fn batch_prag(sents: &[&Sent], v: &Vocs, drop: f32, prag_drop: f32, rng: &mut Rng) -> Batch {
    batch_full(sents, v, drop, prag_drop, 0, rng)
}

/// A batch for a model with `ngram` hashed n-gram buckets (0 — no subword input), at inference.
pub fn batch_for(sents: &[&Sent], v: &Vocs, ngram: usize, rng: &mut Rng) -> Batch {
    batch_full(sents, v, 0., 0., ngram, rng)
}

/// Everything a batch can carry. `ngram` > 0 adds the character n-grams of every word ([`ngrams`]); word dropout
/// hides the word id only, so a dropped word is still seen through its spelling. The random sequence does not depend
/// on `ngram` or on the votes.
pub fn batch_full(sents: &[&Sent], v: &Vocs, drop: f32, prag_drop: f32, ngram: usize, rng: &mut Rng) -> Batch {
    let b = sents.len();
    let t = 1 + sents.iter().map(|s| s.words.len()).max().unwrap_or(0);
    let grams: Vec<Vec<Vec<u32>>> = if ngram > 0 { sents.iter().map(|s| s.words.iter().map(|w| ngrams(&w.form, ngram)).collect()).collect() } else { Vec::new() };
    let g = grams.iter().flatten().map(Vec::len).max().unwrap_or(0).max((ngram > 0) as usize);
    let mut x = Batch { b, t, inp: vec![0; b * t * 4], mask: vec![0.; b * t], rows: vec![], heads: vec![], rels: vec![], xpos: vec![], upos: vec![], keys: vec![0.; b * t], role_tok: vec![0; b * t], role_arc: vec![0; b * t * t], ngrams: vec![0; b * t * g], g, arc_conf: vec![], rel_conf: vec![], alts: vec![] };
    for (k, s) in sents.iter().enumerate() {
        let r0 = k * t;
        if !s.roles.is_empty() && !(prag_drop > 0. && rng.unit() < prag_drop) {
            for &(d, p, r) in &s.roles {
                x.role_arc[(k * t + d) * t + p] = r as u32;
                if x.role_tok[r0 + d] % 8 == 0 {
                    x.role_tok[r0 + d] += r as u32;
                }
                if x.role_tok[r0 + p] < 8 {
                    x.role_tok[r0 + p] += 8;
                }
            }
        }
        x.inp[r0 * 4..r0 * 4 + 4].copy_from_slice(&[ROOT, 1, 2, 8]);
        x.keys[r0] = 1.;
        for (i, w) in s.words.iter().enumerate() {
            let r = r0 + i + 1;
            let mut f = feats(&v.words, &w.form);
            if drop > 0. && rng.unit() < drop {
                f[0] = UNK;
            }
            x.inp[r * 4..r * 4 + 4].copy_from_slice(&f);
            x.mask[r] = 1.;
            x.keys[r] = 1.;
            x.rows.push(r as u32);
            x.heads.push(w.head.min(s.words.len()) as u32);
            x.rels.push(v.rels.get(&w.rel).unwrap_or(0));
            x.xpos.push(v.xpos.get(&w.xpos).unwrap_or(0));
            x.upos.push(v.upos.get(&w.upos).unwrap_or(0));
            if let Some(gs) = grams.get(k).map(|s| &s[i]) {
                x.ngrams[r * g..r * g + gs.len()].copy_from_slice(gs);
            }
            match vote(&w.misc) {
                Some(vt) => {
                    x.arc_conf.push(vt.head);
                    x.rel_conf.push(vt.pair);
                    x.alts.push(vt.alts.iter().filter(|a| a.0 <= s.words.len() && a.0 != w.head).map(|&(h, sh)| (h as u32, sh)).collect());
                }
                None => {
                    x.arc_conf.push(1.);
                    x.rel_conf.push(1.);
                    x.alts.push(Vec::new());
                }
            }
        }
    }
    x
}

/// Batches over a chosen subset (`idx` into `sents`), same rule as `plan`; returns indices into `sents`.
pub fn plan_of(sents: &[Sent], idx: &[usize], tokens: usize) -> Vec<Vec<usize>> {
    let mut v: Vec<usize> = idx.to_vec();
    v.sort_by_key(|&i| (sents[i].words.len(), i));
    let mut out = Vec::new();
    let mut cur: Vec<usize> = Vec::new();
    for i in v {
        let t = sents[i].words.len() + 1;
        if !cur.is_empty() && (cur.len() + 1) * t > tokens {
            out.push(std::mem::take(&mut cur));
        }
        cur.push(i);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Batches of about `tokens` words, sentences of similar length together.
pub fn plan(sents: &[Sent], tokens: usize) -> Vec<Vec<usize>> {
    let mut idx: Vec<usize> = (0..sents.len()).collect();
    idx.sort_by_key(|&i| (sents[i].words.len(), i));
    let mut out = Vec::new();
    let mut cur: Vec<usize> = Vec::new();
    for i in idx {
        let t = sents[i].words.len() + 1;
        if !cur.is_empty() && (cur.len() + 1) * t > tokens {
            out.push(std::mem::take(&mut cur));
        }
        cur.push(i);
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn features_line_up_with_words() {
        let src = "1\tThe\tthe\tDET\tDT\t_\t2\tdet\t_\t_\n2\tcat\tcat\tNOUN\tNN\t_\t3\tnsubj\t_\t_\n3\tsleeps\tsleep\tVERB\tVBZ\t_\t0\troot\t_\t_\n\n1\tThe\tthe\tDET\tDT\t_\t2\tdet\t_\t_\n2\tcat\tcat\tNOUN\tNN\t_\t0\troot\t_\t_\n\n";
        let s = read(src);
        let v = Vocs::build(&s, 2);
        let refs: Vec<&Sent> = s.iter().collect();
        let b = batch(&refs, &v, 0., &mut Rng(1));
        assert_eq!((b.b, b.t), (2, 4));
        let the = v.words.get("the").unwrap();
        let cat = v.words.get("cat").unwrap();
        assert_eq!(b.inp[4], the);
        assert_eq!(b.inp[8], cat);
        assert_eq!(b.inp[12], UNK); // "sleeps" seen once
        assert_eq!(b.inp[4 * 4 + 4], the);
        assert_eq!(b.rows, vec![1, 2, 3, 5, 6]);
        assert_eq!(b.heads, vec![2, 3, 0, 2, 0]);
        let xp: Vec<&str> = b.xpos.iter().map(|&i| v.xpos.names[i as usize].as_str()).collect();
        assert_eq!(xp, vec!["DT", "NN", "VBZ", "DT", "NN"]);
    }

    #[test]
    fn ngrams_and_votes() {
        // «<cat>»: 3-grams <ca cat at>, 4-grams <cat cat>, 5-gram <cat>; nothing longer
        let g = ngrams("Cat", NGRAM_BUCKETS);
        assert_eq!(g.len(), 6);
        assert!(g.iter().all(|&b| b >= 1 && (b as usize) < NGRAM_BUCKETS));
        assert_eq!(g, ngrams("cat", NGRAM_BUCKETS), "case is the shape's business");
        assert_ne!(ngrams("cats", NGRAM_BUCKETS)[..3], ngrams("dogs", NGRAM_BUCKETS)[..3]);
        assert_eq!(ngrams("cats", NGRAM_BUCKETS)[1], g[1], "shared n-grams share a bucket («cat»)");
        assert_eq!(ngrams(&"x".repeat(40), NGRAM_BUCKETS).len(), NGRAM_CAP);
        assert_eq!(ngrams("a", NGRAM_BUCKETS).len(), 1);
        assert_eq!(vote("_"), None);
        assert_eq!(vote("SpaceAfter=No|Vote=0.83|VoteHead=0.9|VoteAlt=5:0.1,0:0.07"), Some(Vote { pair: 0.83, head: 0.9, alts: vec![(5, 0.1), (0, 0.07)] }));
        assert_eq!(vote("Vote=0.5").map(|v| v.head), Some(0.5));
        assert_eq!(vote("Vote=nan"), None);
        // the batch: a voted word carries its shares, an unvoted one is trusted, ngrams line up with positions
        let src = "1\tThe\tthe\tDET\tDT\t_\t2\tdet\t_\tVote=0.6|VoteHead=0.8|VoteAlt=0:0.2,9:0.1\n2\tcat\tcat\tNOUN\tNN\t_\t0\troot\t_\t_\n\n";
        let s = read(src);
        let v = Vocs::build(&s, 1);
        let plain = batch(&[&s[0]], &v, 0., &mut Rng(1));
        assert_eq!((plain.g, plain.ngrams.len()), (0, 0));
        assert_eq!((plain.arc_conf.clone(), plain.rel_conf.clone()), (vec![0.8, 1.], vec![0.6, 1.]));
        assert_eq!(plain.alts, vec![vec![(0, 0.2)], vec![]], "an alternative outside the sentence is dropped");
        let sub = batch_for(&[&s[0]], &v, 1 << 10, &mut Rng(1));
        assert_eq!(sub.g, 6);
        assert_eq!(&sub.ngrams[..6], &[0; 6], "the root has no spelling");
        assert_eq!(&sub.ngrams[12..18], ngrams("cat", 1 << 10).as_slice());
        assert_eq!(sub.inp, plain.inp, "n-grams do not change the other inputs");
    }

    /// L5 and F1 of `languages/en-x-emode/13-improvements.md`: a retagged word is lemmatized again for its new tag, and
    /// an archaic pronoun takes its features from the EModE table, not from the most frequent PRP of the corpus («it»).
    #[test]
    fn retagged_words_are_filled_again() {
        if en::dict::N_LEMMAS == 0 {
            return;
        }
        let corpus = "1\tit\tit\tPRON\tPRP\tCase=Nom|Gender=Neut|Number=Sing|Person=3|PronType=Prs\t2\tnsubj\t_\t_\n2\truns\trun\tVERB\tVBZ\tMood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin\t0\troot\t_\t_\n3\tit\tit\tPRON\tPRP\tCase=Nom|Gender=Neut|Number=Sing|Person=3|PronType=Prs\t2\tobj\t_\t_\n\n";
        let lex = Lex::learn(&read(corpus));
        let s = ["He", "that", "giveth", "unto", "thee", "and", "thou", "saw", "it"];
        // F1: «thou» retagged PRP gets the table's features, «thee» too, with the lemma «thou»
        let (f, l) = lex.fill(&s, 6, "thou", "PRON", "PRP", "nsubj");
        assert_eq!((f.as_str(), l.as_str()), ("Case=Nom|Number=Sing|Person=2|PronType=Prs|Style=Arch", "thou"));
        assert_eq!(lex.fill(&s, 4, "thee", "PRON", "PRP", "obl").1, "thou");
        // a word that keeps its tags: the table too (an old draft's «thou» with the features of «it»)
        let (f, l) = Lex::table(&s, 6, "PRON", "PRP", "nsubj", "Case=Nom|Gender=Neut|Number=Sing|Person=3|PronType=Prs").unwrap();
        assert_eq!((f.as_str(), l.as_str()), ("Case=Nom|Number=Sing|Person=2|PronType=Prs|Style=Arch", "thou"));
        assert!(Lex::table(&s, 8, "PRON", "PRP", "obj", "_").is_none());
        // the table only when its tag agrees: «thee» made a noun is no pronoun of the table
        assert_eq!(lex.fill(&s, 4, "thee", "NOUN", "NN", "obl").1, "thee");
        // L5: «giveth» from NOUN to VERB is «give»; a modern «saw» from NOUN to VBD is «see»
        assert_eq!(lex.fill(&s, 2, "giveth", "VERB", "VBZ", "acl:relcl").1, "give");
        assert_eq!(lex.fill(&s, 7, "saw", "VERB", "VBD", "root").1, "see");
        // a verb keeps an informative old lemma when the new tag gives only the form back
        assert_eq!(lex.fill(&["he", "smote", "him"], 1, "smite", "VERB", "VBP", "root").1, "smite");
        assert_eq!(lex.fill(&["a", "saw"], 1, "see", "NOUN", "NN", "root").1, "saw");
        // negative control: what the corpus knows comes from the corpus; a modern pronoun gets the corpus's features
        let (f, l) = lex.fill(&s, 8, "it", "PRON", "PRP", "obj");
        assert_eq!((f.as_str(), l.as_str()), ("Case=Nom|Gender=Neut|Number=Sing|Person=3|PronType=Prs", "it"));
    }
}

/// LEMMA/FEATS for retagged words and the allowed relations, learned from a corpus (`GP_LEX`): the most frequent
/// (FEATS, lemma) for (form, UPOS, XPOS), else FEATS for (UPOS, XPOS) with the word lemmatized again for its new tag
/// (`en::morph`). Ties go to the smaller. The Early Modern English table (`en::emode`) comes first: its words take
/// lemma and FEATS from it, never from the tag's most frequent FEATS («thou» would get those of «it»).
#[derive(Default)]
pub struct Lex {
    by_form: BTreeMap<(String, String, String), BTreeMap<(String, String), u32>>,
    by_tag: BTreeMap<(String, String), BTreeMap<String, u32>>,
    pub rels: std::collections::BTreeSet<String>,
}

impl Lex {
    pub fn learn(sents: &[Sent]) -> Lex {
        let mut l = Lex::default();
        for w in sents.iter().flat_map(|s| &s.words) {
            *l.by_form.entry((w.form.to_lowercase(), w.upos.clone(), w.xpos.clone())).or_default().entry((w.feats.clone(), w.lemma.clone())).or_default() += 1;
            *l.by_tag.entry((w.upos.clone(), w.xpos.clone())).or_default().entry(w.feats.clone()).or_default() += 1;
            l.rels.insert(w.rel.clone());
        }
        l
    }

    fn top<K: Clone + Ord>(m: &BTreeMap<K, u32>) -> Option<K> {
        m.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))).map(|(k, _)| k.clone())
    }

    /// F1: (FEATS, LEMMA) of word `i` from the EModE table (`en::emode`), when the table knows the word with these
    /// tags; `feats` — the annotation's, for the entries that keep them. `annotate` asks it for every word, retagged or
    /// not: a draft may carry the features of «it» on «thou».
    pub fn table(forms: &[&str], i: usize, upos: &str, xpos: &str, rel: &str, feats: &str) -> Option<(String, String)> {
        use en::gram::{Feats, Rel, Tag, UPos};
        let (t, u, r) = (Tag::parse(xpos)?, UPos::parse(upos)?, Rel::parse(rel)?);
        let (l, u2, f, _) = en::emode::fix(forms, i, t, u, r, Feats::parse(feats).0)?;
        (u2 == u).then(|| (f.to_string(), l))
    }

    /// (FEATS, LEMMA) of word `i` of `forms`, retagged to `upos`/`xpos` with relation `rel`; `lemma` is the old one.
    pub fn fill(&self, forms: &[&str], i: usize, lemma: &str, upos: &str, xpos: &str, rel: &str) -> (String, String) {
        use en::gram::Tag;
        let form = forms[i];
        let by_tag = self.by_tag.get(&(upos.to_string(), xpos.to_string())).and_then(Self::top).unwrap_or_else(|| "_".into());
        let tag = Tag::parse(xpos);
        if let Some(x) = Self::table(forms, i, upos, xpos, rel, &by_tag) {
            return x;
        }
        if let Some(x) = self.by_form.get(&(form.to_lowercase(), upos.to_string(), xpos.to_string())).and_then(Self::top) {
            return x;
        }
        // L5: the lemma for the new tag («giveth» NOUN → VERB: give, not giveth); a verb whose new lemma would only
        // repeat its form keeps the old one («smote» VBD → VBP stays smite)
        let new = tag.map_or_else(|| lemma.to_string(), |t| en::morph::Morph.lemmatize(form, t));
        let verb = matches!(upos, "VERB" | "AUX");
        let lemma = if verb && new.to_lowercase() == form.to_lowercase() && lemma.to_lowercase() != form.to_lowercase() { lemma.to_string() } else { new };
        (by_tag, lemma)
    }

    /// A relation the corpus knows: as is, else its universal part, else `dep`.
    pub fn rel(&self, r: &str) -> String {
        if self.rels.is_empty() || self.rels.contains(r) {
            return r.to_string();
        }
        let base = r.split(':').next().unwrap_or(r);
        if self.rels.contains(base) { base.to_string() } else { "dep".to_string() }
    }
}

/// Structural features for the same nonempty, in-range batches used by predict_inner.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct InferenceCost {
    pub sentences: usize,
    pub words: usize,
    pub skipped: usize,
    pub batches: usize,
    pub padded_tokens: u64,
    pub attention_cells: u64,
    pub decode_cubes: u64,
}
pub fn inference_cost(sents: &[Sent], tokens: usize, max_len: usize) -> InferenceCost {
    let idx: Vec<usize> = (0..sents.len()).filter(|&i| !sents[i].words.is_empty() && sents[i].words.len() < max_len).collect();
    let plans = plan_of(sents, &idx, tokens);
    let mut c = InferenceCost {
        sentences: idx.len(), words: idx.iter().map(|&i|sents[i].words.len()).sum(),
        skipped: sents.len()-idx.len(), batches: plans.len(), ..Default::default()
    };
    for batch in plans {
        let t = batch.iter().map(|&i|sents[i].words.len()+1).max().unwrap() as u64;
        c.padded_tokens += batch.len() as u64*t;
        c.attention_cells += batch.len() as u64*t*t;
        c.decode_cubes += batch.iter().map(|&i|(sents[i].words.len() as u64+1).pow(3)).sum::<u64>();
    }
    c
}
#[cfg(test)]
mod cost_tests {
    use super::*;
    #[test]
    fn cost_counts_real_batch_shapes_and_filters_skipped_sentences() {
        let src = "1\tA\t_\tNOUN\tNN\t_\t0\troot\t_\t_\n\n1\tA\t_\tNOUN\tNN\t_\t0\troot\t_\t_\n2\tB\t_\tNOUN\tNN\t_\t1\tdep\t_\t_\n\n";
        let mut s = read(src);
        s.push(Sent { comments: Vec::new(), words: Vec::new(), roles: Vec::new() });
        let c = inference_cost(&s,6,256);
        assert_eq!((c.sentences,c.words,c.skipped,c.batches,c.padded_tokens,c.attention_cells,c.decode_cubes),(2,3,1,1,6,18,35));
        assert_eq!(inference_cost(&s,2,256).padded_tokens,5);
        assert_eq!(inference_cost(&s,6,2).sentences,1);
    }
}

#[cfg(test)]
mod wave30 {
    use super::*;
    #[test]
    fn projection_roles_and_zero_budget() {
        let mut s = read("# sent_id = a\n1-2\tskip\t_\t_\t_\t_\t0\t_\n1\tX\tx\tNOUN\tNN\t_\tbad\troot\n2.1\tskip\t_\t_\t_\t_\t0\t_\n");
        assert_eq!(s.len(), 1);
        assert_eq!(s[0].words.len(), 1);
        assert_eq!(s[0].words[0].head, 0);
        assert_eq!(shape(""), 6);
        assert_eq!(
            attach_roles(
                &mut s,
                "id\tdep\tpred\trole\na\t1\t1\tagent\na\t0\t1\tpatient\na\t2\t1\tpatient\n"
            ),
            1
        );
        let v = Vocs::build(&s, 1);
        let x = batch(&[&s[0]], &v, 0., &mut Rng(0));
        assert_eq!(x.role_tok, vec![0, 9]);
        assert_eq!(x.role_arc, vec![0, 0, 0, 1]);
        assert_eq!(plan(&s, 0), vec![vec![0]]);
        assert_eq!(Voc::load("").names, vec![""]);
        assert_eq!(read(&write(&s))[0].words[0].form, "X");
    }
}
