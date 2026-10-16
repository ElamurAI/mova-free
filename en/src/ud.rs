//! Full UD annotation on top of the graph: UPOS and FEATS are backoff tables over what the graph already knows
//! (PTB tag, form, relation, tree). We don't write them into the graph: they are derived from it, so a node
//! stays 12 bytes.
//!
//! Metrics are as in CoNLL 2018 (Zeman et al.) on gold tokens:
//! - UPOS, XPOS, UFeats (universal features only), AllTags, Lemmas;
//! - UAS; LAS on the universal part of the relation (without subtypes);
//! - CLAS, MLAS and BLEX on content words (without function aux, case, cc, clf, cop, det, mark and without punct).

use crate::conllu::Sentence;
use crate::gram::{Feat, Feats, Rel, Tag, UPos};
use crate::hash::{FastMap, key, str_key};

/// Minimum examples for a table key to fire (otherwise a coarser key).
const MIN: u32 = 2;

/// What the graph knows about a token — everything UPOS and FEATS are derived from.
pub struct View<'a> {
    pub form: &'a str,
    pub tag: Tag,
    pub rel: Rel,
    /// child flags: subject, aux, passive (aux:pass or nsubj:pass), fixed
    pub kids: u8,
    /// verb–subject agreement key (`agreement`); 0 = none
    pub agr: u64,
}

fn kid_flags(rels: impl Iterator<Item = Rel>) -> u8 {
    let mut f = 0u8;
    for r in rels {
        match r {
            Rel::Nsubj | Rel::Csubj | Rel::Expl | Rel::NsubjOuter | Rel::CsubjOuter => f |= 1,
            Rel::NsubjPass | Rel::CsubjPass => f |= 1 | 4,
            Rel::Aux => f |= 2,
            Rel::AuxPass => f |= 2 | 4,
            Rel::Fixed => f |= 8,
            _ => {}
        }
    }
    f
}

/// Child flags of each sentence token by the tree (heads 1..n, 0 = root). A coordinated conjunct
/// (conj) without its own subject or aux takes them from the first conjunct: "has seen and heard" — heard is perfect too,
/// "was seen and heard" — passive.
pub fn kids_of(heads: &[usize], rels: &[Rel]) -> Vec<u8> {
    let n = heads.len();
    let mut k: Vec<u8> = (1..=n).map(|h| kid_flags((0..n).filter(|&d| heads[d] == h).map(|d| rels[d]))).collect();
    let own = k.clone();
    for d in 0..n {
        // first conjunct: up the conj chain
        let mut h = d;
        for _ in 0..n {
            if rels[h] != Rel::Conj || heads[h] == 0 || heads[h] > n {
                break;
            }
            h = heads[h] - 1;
        }
        if h != d {
            if own[d] & 1 == 0 {
                k[d] |= own[h] & 1;
            }
            if own[d] & 2 == 0 {
                k[d] |= own[h] & (2 | 4);
            }
        }
    }
    k
}

/// Children of each token (0-based indices).
pub fn children(heads: &[usize]) -> Vec<Vec<usize>> {
    let mut ch = vec![Vec::new(); heads.len()];
    for (d, &h) in heads.iter().enumerate() {
        if h > 0 && h <= heads.len() {
            ch[h - 1].push(d);
        }
    }
    ch
}

/// Verb agreement key (VERB, AUX) — for the FEATS table. EWT 2.18 writes Number and Person on VBZ,
/// VBP and VBD from the subject (subject "you" → Number=Sing|Person=2); MD gets only VerbForm=Fin. Subject:
/// - its own nsubj (nsubj:pass, nsubj:outer) or csubj, otherwise expl;
/// - for aux, aux:pass and cop — the head's subject;
/// - for conj without its own subject — the subject of the first conjunct;
/// - a relative pronoun (PronType=Rel) — its antecedent, the head of acl:relcl.
///
/// The key holds: subject class, its Number and Person, whether it is a coordination ("John and Mary"); for conj —
/// mood and form of the first conjunct ("wait and see" — an imperative coordination).
pub fn agreement(i: usize, upos: &[UPos], feats: &[Feats], heads: &[usize], rels: &[Rel], ch: &[Vec<usize>]) -> u64 {
    let n = heads.len();
    let up = |p: usize| if heads[p] > 0 && heads[p] <= n { Some(heads[p] - 1) } else { None };
    let subj_of = |mut p: usize| -> Option<usize> {
        for _ in 0..n {
            let own = ch[p]
                .iter()
                .copied()
                .find(|&c| matches!(rels[c], Rel::Nsubj | Rel::NsubjPass | Rel::NsubjOuter | Rel::Csubj | Rel::CsubjPass | Rel::CsubjOuter))
                .or_else(|| ch[p].iter().copied().find(|&c| rels[c] == Rel::Expl));
            if own.is_some() {
                return own;
            }
            match up(p) {
                Some(h) if rels[p] == Rel::Conj => p = h,
                _ => return None,
            }
        }
        None
    };
    let pred = if matches!(rels[i], Rel::Aux | Rel::AuxPass | Rel::Cop) { up(i).unwrap_or(i) } else { i };
    let mut s = subj_of(pred);
    if let Some(x) = s {
        if feats[x].has(Feat::PronTypeRel) {
            if let Some(p) = up(x) {
                if rels[p] == Rel::AclRelcl {
                    s = up(p).or(s);
                }
            }
        }
    }
    let (class, num, pers, coord) = match s {
        None => (0, 0, 0, 0),
        Some(x) => {
            let class = match (rels[x], upos[x]) {
                (Rel::Csubj | Rel::CsubjPass | Rel::CsubjOuter, _) => 6,
                (Rel::Expl, _) => 7,
                (_, UPos::PRON) => 1,
                (_, UPos::NOUN) => 2,
                (_, UPos::PROPN) => 3,
                (_, UPos::DET) => 4,
                (_, UPos::NUM) => 5,
                _ => 8,
            };
            let f = feats[x];
            let num = [Feat::NumberSing, Feat::NumberPlur, Feat::NumberPtan].iter().position(|&v| f.has(v)).map_or(0, |k| k + 1);
            let pers = [Feat::Person1, Feat::Person2, Feat::Person3].iter().position(|&v| f.has(v)).map_or(0, |k| k + 1);
            (class, num as u64, pers as u64, ch[x].iter().any(|&c| rels[c] == Rel::Conj) as u64)
        }
    };
    let mut first = i;
    for _ in 0..n {
        match up(first) {
            Some(h) if rels[first] == Rel::Conj => first = h,
            _ => break,
        }
    }
    let vf = if first == i {
        0
    } else {
        let f = feats[first];
        [Feat::MoodImp, Feat::VerbFormInf, Feat::VerbFormFin, Feat::VerbFormPart, Feat::VerbFormGer].iter().position(|&v| f.has(v)).map_or(6, |k| k as u64 + 1)
    };
    key(&[20, class, num, pers, coord, vf])
}

/// Shape of a number word: digits, word, Roman, mixed.
fn shape(w: &str) -> u64 {
    let digits = w.chars().filter(char::is_ascii_digit).count();
    let alpha = w.chars().filter(|c| c.is_alphabetic()).count();
    if digits > 0 && alpha == 0 {
        1
    } else if digits > 0 {
        2
    } else if w.len() > 0 && w.chars().all(|c| "IVXLCDM".contains(c)) {
        3
    } else {
        0
    }
}

/// Shape that accounts for the decimal point: "10.2" in EWT is mostly NumType=Frac (104 vs 32).
fn num_shape(w: &str) -> u64 {
    let s = shape(w);
    let b = w.as_bytes();
    if s == 1 && b.windows(3).any(|x| x[0].is_ascii_digit() && x[1] == b'.' && x[2].is_ascii_digit()) {
        4
    } else {
        s
    }
}

fn upos_keys(v: &View) -> [u64; 4] {
    let (t, r, w) = (v.tag as u64, v.rel as u64, str_key(&v.form.to_lowercase()));
    [key(&[1, t, w, r]), key(&[2, t, w]), key(&[3, t, r, v.kids as u64]), key(&[4, t])]
}

/// FEATS keys from most specific to coarsest; 0 = no key (no agreement).
fn feats_keys(v: &View, u: UPos) -> [u64; 7] {
    let (t, r, w, u, k, s) = (v.tag as u64, v.rel as u64, str_key(&v.form.to_lowercase()), u as u64, v.kids as u64, num_shape(v.form));
    let (a0, a1) = if v.agr == 0 { (0, 0) } else { (key(&[10, u, t, w, r, k, v.agr]), key(&[16, u, t, r, k, v.agr])) };
    [a0, a1, key(&[11, u, t, w, r, k]), key(&[12, u, t, w]), key(&[13, u, t, r, k, s]), key(&[14, u, t, s]), key(&[15, u])]
}

/// UPOS and FEATS tables.
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Ud {
    upos: FastMap<u64, [u32; UPos::N]>,
    feats: FastMap<u64, FastMap<Feats, u32>>,
}

impl Ud {
    /// Training on gold sentences (with UPOS and PTB tags).
    pub fn train(sents: &[Sentence]) -> Ud {
        let mut m = Ud::default();
        for s in sents.iter().filter(|s| s.tagged() && s.tokens.iter().all(|t| t.upos.is_some())) {
            let feats = s.has_feats;
            let heads: Vec<usize> = s.tokens.iter().map(|t| t.head).collect();
            let rels: Vec<Rel> = s.tokens.iter().map(|t| t.rel).collect();
            let kids = kids_of(&heads, &rels);
            let ch = children(&heads);
            let upos: Vec<UPos> = s.tokens.iter().map(|t| t.upos.expect("upos")).collect();
            let gold: Vec<Feats> = s.tokens.iter().map(|t| t.feats).collect();
            for (i, t) in s.tokens.iter().enumerate() {
                let u = upos[i];
                let agr = if feats && matches!(u, UPos::VERB | UPos::AUX) { agreement(i, &upos, &gold, &heads, &rels, &ch) } else { 0 };
                let v = View { form: &t.form, tag: t.tag.expect("tagged"), rel: t.rel, kids: kids[i], agr };
                for k in upos_keys(&v) {
                    m.upos.entry(k).or_insert([0; UPos::N])[u.idx()] += 1;
                }
                if feats {
                    for k in feats_keys(&v, u).into_iter().filter(|&k| k != 0) {
                        *m.feats.entry(k).or_default().entry(t.feats).or_default() += 1;
                    }
                }
            }
        }
        m
    }

    pub fn upos(&self, v: &View) -> UPos {
        for k in upos_keys(v) {
            if let Some(c) = self.upos.get(&k) {
                if c.iter().sum::<u32>() >= MIN {
                    let (i, _) = c.iter().enumerate().max_by_key(|x| (x.1, std::cmp::Reverse(x.0))).expect("17");
                    return UPos::ALL[i];
                }
            }
        }
        UPos::X
    }

    pub fn feats(&self, v: &View, u: UPos) -> Feats {
        for k in feats_keys(v, u).into_iter().filter(|&k| k != 0) {
            if let Some(c) = self.feats.get(&k) {
                if c.values().sum::<u32>() >= MIN {
                    return *c.iter().max_by_key(|x| (x.1, std::cmp::Reverse(*x.0))).expect("non-empty").0;
                }
            }
        }
        Feats::default()
    }

    /// Sentence FEATS: first each word from the tables, then VERB and AUX again — with the agreement key
    /// (`agreement`) on first-pass features. Conjuncts come after the first conjunct, so they take its
    /// already refined features.
    pub fn feats_sentence<S: AsRef<str>>(&self, forms: &[S], tags: &[Tag], upos: &[UPos], heads: &[usize], rels: &[Rel]) -> Vec<Feats> {
        let kids = kids_of(heads, rels);
        let ch = children(heads);
        let view = |i: usize, agr: u64| View { form: forms[i].as_ref(), tag: tags[i], rel: rels[i], kids: kids[i], agr };
        let mut f: Vec<Feats> = (0..forms.len()).map(|i| self.feats(&view(i, 0), upos[i])).collect();
        if std::env::var("EN_AGR").as_deref() == Ok("0") {
            return f;
        }
        for i in 0..forms.len() {
            if matches!(upos[i], UPos::VERB | UPos::AUX) {
                let agr = agreement(i, upos, &f, heads, rels, &ch);
                f[i] = self.feats(&view(i, agr), upos[i]);
            }
        }
        f
    }
}

/// UD annotation of one word (predicted or gold) — what the metrics compare.
#[derive(Clone, Debug)]
pub struct Word {
    pub upos: Option<UPos>,
    pub tag: Option<Tag>,
    pub feats: Feats,
    pub lemma: String,
    pub head: usize,
    pub rel: Rel,
}

fn functional(r: Rel) -> bool {
    matches!(r.base(), Rel::Aux | Rel::Case | Rel::Cc | Rel::Clf | Rel::Cop | Rel::Det | Rel::Mark)
}

fn content(r: Rel) -> bool {
    !functional(r) && r.base() != Rel::Punct
}

/// CoNLL 2018 metric counters.
#[derive(Default, Clone, Copy, Debug)]
pub struct Scores {
    pub n: usize,
    pub upos: usize,
    /// tokens in treebanks with FEATS and with lemmas
    pub feats_n: usize,
    pub lemma_n: usize,
    /// tokens with a gold PTB tag and how many of them were guessed
    pub xpos_n: usize,
    pub xpos: usize,
    pub ufeats: usize,
    pub feats: usize,
    pub alltags: usize,
    pub lemma: usize,
    pub uas: usize,
    pub las: usize,
    pub las_full: usize,
    /// content words: gold, predicted; matches for CLAS, MLAS, BLEX
    pub gold_c: usize,
    pub pred_c: usize,
    pub clas: usize,
    pub mlas: usize,
    pub blex: usize,
}

/// Sum of counters (sentence → corpus; bootstrap sample).
impl std::ops::AddAssign<&Scores> for Scores {
    fn add_assign(&mut self, o: &Scores) {
        self.n += o.n;
        self.upos += o.upos;
        self.feats_n += o.feats_n;
        self.lemma_n += o.lemma_n;
        self.xpos_n += o.xpos_n;
        self.xpos += o.xpos;
        self.ufeats += o.ufeats;
        self.feats += o.feats;
        self.alltags += o.alltags;
        self.lemma += o.lemma;
        self.uas += o.uas;
        self.las += o.las;
        self.las_full += o.las_full;
        self.gold_c += o.gold_c;
        self.pred_c += o.pred_c;
        self.clas += o.clas;
        self.mlas += o.mlas;
        self.blex += o.blex;
    }
}

impl Scores {
    /// Percentages: UPOS, UFeats, LAS and MLAS (content-word F1) — the four metrics of the context experiment.
    pub fn main4(&self) -> [f64; 4] {
        let p = |k: usize, n: usize| if n == 0 { 0.0 } else { 100.0 * k as f64 / n as f64 };
        [p(self.upos, self.n), p(self.ufeats, self.feats_n), p(self.las, self.n), p(2 * self.mlas, self.gold_c + self.pred_c)]
    }

    /// `feats`, `lemmas` — whether the treebank annotates these layers (otherwise their metrics are not counted).
    pub fn add(&mut self, gold: &[Word], pred: &[Word], feats: bool, lemmas: bool) {
        let n = gold.len();
        // function children of each word: (relation, UPOS, universal features)
        let fkids = |ws: &[Word], h: usize| -> Vec<(Rel, Option<UPos>, Feats)> {
            let mut v: Vec<_> = ws.iter().filter(|w| w.head == h && functional(w.rel)).map(|w| (w.rel.base(), w.upos, w.feats.universal())).collect();
            v.sort();
            v
        };
        for i in 0..n {
            let (g, p) = (&gold[i], &pred[i]);
            self.n += 1;
            let up = g.upos.is_some() && g.upos == p.upos;
            self.upos += up as usize;
            let uf = feats && g.feats.universal() == p.feats.universal();
            if feats {
                self.feats_n += 1;
                self.ufeats += uf as usize;
                self.feats += (g.feats == p.feats) as usize;
            }
            if g.tag.is_some() {
                self.xpos_n += 1;
                let xp = g.tag == p.tag;
                self.xpos += xp as usize;
                self.alltags += (xp && up && uf) as usize;
            }
            if lemmas {
                self.lemma_n += 1;
                self.lemma += (g.lemma == p.lemma) as usize;
            }
            let head_ok = g.head == p.head;
            self.uas += head_ok as usize;
            let las = head_ok && g.rel.base() == p.rel.base();
            self.las += las as usize;
            self.las_full += (head_ok && g.rel == p.rel) as usize;
            self.gold_c += content(g.rel) as usize;
            self.pred_c += content(p.rel) as usize;
            if content(g.rel) && las {
                self.clas += 1;
                self.blex += (lemmas && g.lemma == p.lemma) as usize;
                self.mlas += (up && uf && fkids(gold, i + 1) == fkids(pred, i + 1)) as usize;
            }
        }
    }

    /// Table row: percentages; CLAS, MLAS, BLEX are F1 over content words.
    pub fn row(&self) -> String {
        let p = |k: usize, n: usize| if n == 0 { "—".to_string() } else { format!("{:.2}", 100.0 * k as f64 / n as f64) };
        let f1 = |k: usize, applies: bool| {
            if !applies {
                return "—".to_string();
            }
            let (pr, rc) = (k as f64 / self.pred_c.max(1) as f64, k as f64 / self.gold_c.max(1) as f64);
            if pr + rc == 0.0 { "0.00".to_string() } else { format!("{:.2}", 100.0 * 2.0 * pr * rc / (pr + rc)) }
        };
        format!(
            "{} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
            p(self.upos, self.n),
            p(self.xpos, self.xpos_n),
            p(self.ufeats, self.feats_n),
            p(self.feats, self.feats_n),
            if self.feats_n == self.n { p(self.alltags, self.xpos_n) } else { "—".to_string() },
            p(self.lemma, self.lemma_n),
            p(self.uas, self.n),
            p(self.las, self.n),
            p(self.las_full, self.n),
            f1(self.clas, true),
            f1(self.mlas, self.feats_n == self.n),
            f1(self.blex, self.lemma_n == self.n)
        )
    }

    pub const HEADER: &'static str =
        "| test | tokens | UPOS | XPOS | UFeats | Feats | AllTags | Lemmas | UAS | LAS | LAS with subtypes | CLAS | MLAS | BLEX |";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gram::Feat;

    #[test]
    fn feats_round_trip_in_conllu_order() {
        let s = "Case=Nom|Number=Sing|Person=1|PronType=Prs";
        let (f, unk) = Feats::parse(s);
        assert_eq!((f.to_string().as_str(), unk), (s, 0));
        // Number < NumForm < NumType — CoNLL-U order, case-insensitive
        let s = "Number=Sing|NumForm=Word|NumType=Frac";
        assert_eq!(Feats::parse(s).0.to_string(), s);
        // an unknown pair is dropped and counted
        assert_eq!(Feats::parse("Number=Sing|Foo=Bar"), (Feats(1 << Feat::NumberSing.idx()), 1));
        // UFeats does not see language-specific features
        assert_eq!(Feats::parse("NumForm=Digit|NumType=Card").0.universal(), Feats::parse("NumType=Card").0);
    }

    #[test]
    fn metrics_see_wrong_label_and_function_child() {
        let w = |upos, head, rel| Word { upos: Some(upos), tag: None, feats: Feats::default(), lemma: String::new(), head, rel };
        let gold = vec![w(UPos::DET, 2, Rel::Det), w(UPos::NOUN, 3, Rel::Nsubj), w(UPos::VERB, 0, Rel::Root)];
        let mut s = Scores::default();
        s.add(&gold, &gold, true, true);
        assert_eq!((s.las, s.clas, s.mlas), (3, 2, 2));
        // negative control: a wrong label on a function word breaks the head's LAS and MLAS, but not CLAS

        let mut bad = gold.clone();
        bad[0].upos = Some(UPos::PRON);
        let mut s = Scores::default();
        s.add(&gold, &bad, true, true);
        assert_eq!((s.upos, s.clas, s.mlas), (2, 2, 1));
    }
}
