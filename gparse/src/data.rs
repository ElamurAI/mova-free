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
            cur.words.push(Word { form: c[1].into(), lemma: c[2].into(), feats: c[5].into(), upos: c[3].into(), xpos: c[4].into(), head: c[6].parse().unwrap_or(0), rel: c[7].into() });
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
    let b = sents.len();
    let t = 1 + sents.iter().map(|s| s.words.len()).max().unwrap_or(0);
    let mut x = Batch { b, t, inp: vec![0; b * t * 4], mask: vec![0.; b * t], rows: vec![], heads: vec![], rels: vec![], xpos: vec![], upos: vec![], keys: vec![0.; b * t], role_tok: vec![0; b * t], role_arc: vec![0; b * t * t] };
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
}

/// LEMMA/FEATS for retagged words and the allowed relations, learned from a corpus (`GP_LEX`): the most frequent
/// (FEATS, lemma) for (form, UPOS, XPOS), else FEATS for (UPOS, XPOS) with the old lemma. Ties go to the smaller.
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

    pub fn fill(&self, form: &str, lemma: &str, upos: &str, xpos: &str) -> (String, String) {
        if let Some(x) = self.by_form.get(&(form.to_lowercase(), upos.to_string(), xpos.to_string())).and_then(Self::top) {
            return x;
        }
        (self.by_tag.get(&(upos.to_string(), xpos.to_string())).and_then(Self::top).unwrap_or_else(|| "_".into()), lemma.to_string())
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
