//! The SLM derives its own tree-repair rules: transformation-based, error-driven learning (after Brill 1995) of
//! label rules on top of the parser, with level-1 knowledge as features (the absurdity matrix, idioms, verb classes,
//! noun lists). Every rule reads as a sentence with its evidence; nothing is hand-written.
//!
//!   world induce <train.conllu>... --dev <dev.conllu> --test <test.conllu>... --out <dir> [--iters 40] [--min 8]
//!
//! 1. Parse the gold sentences with the bare parser (no hand repairs).
//! 2. For each word whose head is right but label wrong, every conjunction of 1–3 feature atoms (plus its current
//!    label) is a candidate rule "label → gold label".
//! 3. Greedy: score every candidate on all words — fixed (label becomes right), broken (a right label becomes
//!    wrong); take the best `fixed − broken` with precision ≥ 0.85 and at least `--min` fixes, apply it, repeat.
//! 4. Report the rule list with train counts, then its effect on dev and test (LAS, per-rule fixed/broken,
//!    paired bootstrap).

use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Result;
use en::gram::{Rel, Tag, UPos};

use crate::absurd::Matrix;

/// A feature atom: (feature, value), both small integers; the names are for reading the rules.
type Atom = (u8, u16);

const F_UPOS: u8 = 0;
const F_HEAD_UPOS: u8 = 1;
const F_HEAD_FINITE: u8 = 2;
const F_AFTER_HEAD: u8 = 3;
const F_HEAD_HAS_SUBJ: u8 = 4;
const F_HEAD_INTRANS: u8 = 5;
const F_HEAD_SPEECH: u8 = 6;
const F_HEAD_LIGHT: u8 = 7;
const F_OBJ_BIN: u8 = 8;
const F_SUBJ_BIN: u8 = 9;
const F_IDIOM: u8 = 10;
const F_MEASURE: u8 = 11;
const F_ARCHAIC: u8 = 12;
const F_ANIMACY: u8 = 13;
const F_PREV_PUNCT: u8 = 14;
const F_NEXT_PUNCT: u8 = 15;
const F_HEAD_HAS_OBJ: u8 = 16;
/// the word has a case marker (a `case` child: "at length", "of course") — a subject or object never has one
const F_HAS_CASE: u8 = 17;
/// a quote mark stands between the word and its head ("“It is a place,” replied the man")
const F_QUOTE_BETWEEN: u8 = 18;

fn feature_name(f: u8) -> &'static str {
    match f {
        F_UPOS => "word is",
        F_HEAD_UPOS => "head is",
        F_HEAD_FINITE => "head finite",
        F_AFTER_HEAD => "after head",
        F_HEAD_HAS_SUBJ => "head has a subject",
        F_HEAD_INTRANS => "head takes no object (matrix)",
        F_HEAD_SPEECH => "head is a speech/motion verb (level 1)",
        F_HEAD_LIGHT => "head is a light verb",
        F_OBJ_BIN => "matrix OBJ",
        F_SUBJ_BIN => "matrix SUBJ (real/tale)",
        F_IDIOM => "idiom kind",
        F_MEASURE => "noun of measure/time",
        F_ARCHAIC => "archaic adverb",
        F_ANIMACY => "animacy",
        F_PREV_PUNCT => "previous token is , or quote",
        F_NEXT_PUNCT => "next token is , or quote",
        F_HEAD_HAS_OBJ => "head has another object",
        F_HAS_CASE => "word has a preposition (case)",
        F_QUOTE_BETWEEN => "a quote between word and head",
        _ => "?",
    }
}

fn bin(s: Option<u8>) -> u16 {
    match s {
        None => 0,
        Some(0 | 1) => 1,
        Some(2) => 2,
        Some(_) => 3,
    }
}

fn value_name(f: u8, v: u16) -> String {
    match f {
        F_UPOS | F_HEAD_UPOS => format!("{:?}", UPos::ALL.get(v as usize).copied().unwrap_or(UPos::X)),
        F_OBJ_BIN | F_SUBJ_BIN => ["unknown", "0–1", "2", "3–4"][v as usize].to_string(),
        F_IDIOM => ["none", "idiom", "light-verb", "collocation", "phrasal-verb", "candidate"][v as usize].to_string(),
        F_ANIMACY => ["unknown", "animate", "inanimate"][v as usize].to_string(),
        _ => if v == 1 { "yes".into() } else { "no".into() },
    }
}

struct Word {
    form: String,
    lemma: String,
    upos: UPos,
    tag: Tag,
    head: usize,
    rel: Rel,
}

struct Sent {
    w: Vec<Word>,
    gold: Vec<(usize, Rel)>,
}

fn upos_id(u: UPos) -> u16 {
    UPos::ALL.iter().position(|x| *x == u).unwrap_or(0) as u16
}

/// Feature groups that can be hidden from the inducer (`--without`, ablation of a piece of knowledge).
fn hidden_features() -> &'static Vec<u8> {
    static H: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();
    H.get_or_init(|| {
        let mut out = Vec::new();
        for g in std::env::var("INDUCE_WITHOUT").unwrap_or_default().split(',').map(str::trim) {
            out.extend_from_slice(match g {
                "matrix" => &[F_OBJ_BIN, F_SUBJ_BIN, F_HEAD_INTRANS][..],
                "idioms" => &[F_IDIOM][..],
                "verbclass" => &[F_HEAD_SPEECH, F_HEAD_LIGHT][..],
                "nouns" => &[F_MEASURE, F_ARCHAIC, F_ANIMACY][..],
                "syntax" => &[F_HEAD_FINITE, F_AFTER_HEAD, F_HEAD_HAS_SUBJ, F_HEAD_HAS_OBJ][..],
                "punct" => &[F_PREV_PUNCT, F_NEXT_PUNCT][..],
                _ => &[][..],
            });
        }
        out
    })
}

/// Feature atoms of word `i` (without the label, which is part of every rule).
fn atoms(m: &Matrix, s: &[Word], i: usize) -> Vec<Atom> {
    let mut a = atoms_all(m, s, i);
    let h = hidden_features();
    if !h.is_empty() {
        a.retain(|(f, _)| !h.contains(f));
    }
    a
}

fn atoms_all(m: &Matrix, s: &[Word], i: usize) -> Vec<Atom> {
    let w = &s[i];
    let mut a: Vec<Atom> = vec![(F_UPOS, upos_id(w.upos))];
    let punct = |j: usize| s.get(j).is_some_and(|x| matches!(x.form.as_str(), "," | "\"" | "“" | "”" | "'" | "‘" | "’"));
    a.push((F_PREV_PUNCT, (i > 0 && punct(i - 1)) as u16));
    a.push((F_NEXT_PUNCT, punct(i + 1) as u16));
    a.push((F_MEASURE, global::selection("measure_time").contains(&w.lemma.as_str()) as u16));
    a.push((F_ARCHAIC, global::selection("archaic_adverbs").contains(&w.lemma.as_str()) as u16));
    let an = match crate::sense::animacy(&w.lemma) {
        crate::sense::Anim::Unknown => 0,
        crate::sense::Anim::Animate(_) => 1,
        crate::sense::Anim::Inanimate(_) => 2,
    };
    a.push((F_ANIMACY, an));
    // the possessive 's is a `case` too in UD ("Bailey 's officials"), but it does not make a subject oblique
    a.push((F_HAS_CASE, s.iter().any(|x| x.head == i + 1 && x.rel == Rel::Case && !matches!(x.form.as_str(), "'s" | "’s" | "'" | "’")) as u16));
    if w.head > 0 {
        let h = w.head - 1;
        let (lo, hi) = (i.min(h), i.max(h));
        a.push((F_QUOTE_BETWEEN, s[lo + 1..hi].iter().any(|x| matches!(x.form.as_str(), "\"" | "“" | "”" | "‘" | "’" | "``" | "''")) as u16));
        let hw = &s[h];
        let verb = hw.lemma.as_str();
        a.push((F_HEAD_UPOS, upos_id(hw.upos)));
        a.push((F_HEAD_FINITE, matches!(hw.tag, Tag::VBD | Tag::VBZ | Tag::VBP) as u16));
        a.push((F_AFTER_HEAD, (i > h) as u16));
        let kids = || s.iter().enumerate().filter(move |(_, x)| x.head == h + 1);
        a.push((F_HEAD_HAS_SUBJ, kids().any(|(j, x)| j != i && matches!(x.rel, Rel::Nsubj | Rel::NsubjPass | Rel::Csubj) && x.lemma != "there") as u16));
        a.push((F_HEAD_HAS_OBJ, kids().any(|(j, x)| j != i && x.rel == Rel::Obj) as u16));
        if hw.upos == UPos::VERB {
            a.push((F_HEAD_INTRANS, global::absurd_intransitive(verb) as u16));
            a.push((F_HEAD_SPEECH, global::selection("patient_not_animate").contains(&verb) as u16));
            a.push((F_HEAD_LIGHT, ["make", "take", "give", "have", "do", "pay", "get", "put"].contains(&verb) as u16));
            let tale = global::absurdity_in("tale", verb, &w.lemma).map(|x| x[0]);
            let subj = match (m.score(verb, &w.lemma, 0), tale) {
                (Some(x), Some(y)) => Some(x.min(y)),
                (x, y) => x.or(y),
            };
            a.push((F_OBJ_BIN, bin(m.score(verb, &w.lemma, 1))));
            a.push((F_SUBJ_BIN, bin(subj)));
            let k = match global::idiom(verb, &w.lemma) {
                None => 0,
                Some("idiom") => 1,
                Some("light-verb") => 2,
                Some("collocation") => 3,
                Some("phrasal-verb") => 4,
                Some(_) => 5,
            };
            a.push((F_IDIOM, k));
        }
    }
    a
}

/// A rule: from label, to label, sorted atoms (1–3).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Rule {
    from: Rel,
    to: Rel,
    atoms: Vec<Atom>,
}

impl Rule {
    /// The same labels and at least this rule's conditions: `other` fires on a subset of this rule's words (a
    /// narrower variant of it).
    pub fn covers(&self, other: &Rule) -> bool {
        self.from == other.from && self.to == other.to && self.atoms.iter().all(|a| other.atoms.contains(a))
    }

    /// The machine line of the rule (scope "domain"), as in `rules-induced.tsv`.
    pub fn machine(&self) -> String {
        format!("domain\t{}\t{}\t{}", self.from, self.to, self.atoms.iter().map(|(a, v)| format!("{a}:{v}")).collect::<Vec<_>>().join(","))
    }

    pub fn show(&self) -> String {
        let conds: Vec<String> = self.atoms.iter().map(|&(f, v)| format!("{} = {}", feature_name(f), value_name(f, v))).collect();
        format!("{} → {} IF {}", self.from, self.to, conds.join(" AND "))
    }

    fn matches(&self, rel: Rel, a: &[Atom]) -> bool {
        rel == self.from && self.atoms.iter().all(|x| a.contains(x))
    }
}

fn subsets(a: &[Atom], out: &mut Vec<Vec<Atom>>) {
    out.clear();
    let n = a.len();
    for i in 0..n {
        out.push(vec![a[i]]);
        for j in i + 1..n {
            out.push(vec![a[i], a[j]]);
            for k in j + 1..n {
                out.push(vec![a[i], a[j], a[k]]);
            }
        }
    }
    for s in out.iter_mut() {
        s.sort();
    }
}

fn load(paths: &[PathBuf]) -> Result<Vec<Sent>> {
    let a = crate::tree::annotator()?;
    let mut out = Vec::new();
    for p in paths {
        for s in en::conllu::read(p)? {
            let forms: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
            if forms.is_empty() {
                continue;
            }
            let ws = a.annotate(&forms);
            let w = ws.into_iter().map(|x| Word { form: x.form, lemma: x.lemma.to_lowercase(), upos: x.upos, tag: x.tag, head: x.head, rel: x.rel }).collect();
            out.push(Sent { w, gold: s.tokens.iter().map(|t| (t.head, t.rel)).collect() });
        }
    }
    Ok(out)
}

/// Count fixes and breaks of every candidate over all words; candidates come from the label errors.
fn score(m: &Matrix, data: &[Sent], min: usize) -> Vec<(Rule, usize, usize)> {
    let mut fix: HashMap<Rule, usize> = HashMap::new();
    let mut buf = Vec::new();
    let feats: Vec<Vec<Vec<Atom>>> = data.iter().map(|s| (0..s.w.len()).map(|i| atoms(m, &s.w, i)).collect()).collect();
    for (s, fs) in data.iter().zip(&feats) {
        for (i, w) in s.w.iter().enumerate() {
            let (gh, gr) = s.gold[i];
            if w.head != gh || w.rel == gr {
                continue;
            }
            subsets(&fs[i], &mut buf);
            for a in &buf {
                *fix.entry(Rule { from: w.rel, to: gr, atoms: a.clone() }).or_default() += 1;
            }
        }
    }
    fix.retain(|_, n| *n >= min);
    // breaks: words whose label is right now and that a candidate would change
    let mut brk: HashMap<&Rule, usize> = HashMap::new();
    let by_from: HashMap<(Rel, Vec<Atom>), Vec<&Rule>> = fix.keys().fold(HashMap::new(), |mut acc, r| {
        acc.entry((r.from, r.atoms.clone())).or_default().push(r);
        acc
    });
    for (s, fs) in data.iter().zip(&feats) {
        for (i, w) in s.w.iter().enumerate() {
            if w.rel != s.gold[i].1 || w.head != s.gold[i].0 {
                continue;
            }
            subsets(&fs[i], &mut buf);
            for a in &buf {
                if let Some(rs) = by_from.get(&(w.rel, a.clone())) {
                    for r in rs {
                        if r.to != w.rel {
                            *brk.entry(*r).or_default() += 1;
                        }
                    }
                }
            }
        }
    }
    let mut out: Vec<(Rule, usize, usize)> = fix.iter().map(|(r, &f)| (r.clone(), f, brk.get(r).copied().unwrap_or(0))).collect();
    // deterministic order: net, then fewer atoms, then the rule itself
    out.sort_by(|a, b| (b.1 as i64 - b.2 as i64).cmp(&(a.1 as i64 - a.2 as i64)).then(a.0.atoms.len().cmp(&b.0.atoms.len())).then(a.0.cmp(&b.0)));
    out
}

fn apply(m: &Matrix, data: &mut [Sent], r: &Rule) -> (usize, usize, Vec<i64>) {
    let (mut f, mut b) = (0, 0);
    let mut diffs = Vec::with_capacity(data.len());
    for s in data.iter_mut() {
        let fs: Vec<Vec<Atom>> = (0..s.w.len()).map(|i| atoms(m, &s.w, i)).collect();
        let mut d = 0i64;
        for i in 0..s.w.len() {
            if r.matches(s.w[i].rel, &fs[i]) {
                let (gh, gr) = s.gold[i];
                let was = s.w[i].head == gh && s.w[i].rel == gr;
                let now = s.w[i].head == gh && r.to == gr;
                f += (!was && now) as usize;
                b += (was && !now) as usize;
                d += now as i64 - was as i64;
                s.w[i].rel = r.to;
            }
        }
        diffs.push(d);
    }
    (f, b, diffs)
}

fn las(data: &[Sent]) -> f64 {
    let (mut c, mut n) = (0usize, 0usize);
    for s in data {
        for (i, w) in s.w.iter().enumerate() {
            n += 1;
            c += (w.head == s.gold[i].0 && w.rel == s.gold[i].1) as usize;
        }
    }
    100.0 * c as f64 / n.max(1) as f64
}

/// The paired bootstrap p (see `boot`), for other modules.
pub fn boot_pub(d: &[i64]) -> f64 {
    boot(d)
}

pub(crate) fn boot(d: &[i64]) -> f64 {
    let mut x: u64 = 0x9e3779b97f4a7c15;
    let mut worse = 0;
    for _ in 0..1000 {
        let mut sum = 0i64;
        for _ in 0..d.len() {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            sum += d[(x % d.len() as u64) as usize];
        }
        worse += (sum <= 0) as usize;
    }
    worse as f64 / 1000.0
}

fn eval_set(m: &Matrix, name: &str, data: &mut [Sent], rules: &[(Rule, usize, usize)], rep: &mut String) {
    let before = las(data);
    let mut diffs = vec![0i64; data.len()];
    let mut per = Vec::new();
    for (r, _, _) in rules {
        let (f, b, d) = apply(m, data, r);
        for (x, y) in diffs.iter_mut().zip(d) {
            *x += y;
        }
        per.push((f, b));
    }
    *rep += &format!("{name}: LAS {before:.2} → {:.2}  paired bootstrap p = {:.3}\n", las(data), boot(&diffs));
    for ((r, _, _), (f, b)) in rules.iter().zip(per) {
        if f + b > 0 {
            *rep += &format!("    fixed {f:3} broken {b:3}  {}\n", r.show());
        }
    }
}

/// Parse `rules.machine.tsv` lines: (scope, rule).
pub fn parse_rules(text: &str) -> Vec<(String, Rule)> {
    text.lines()
        .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        .filter_map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            let atoms = c.get(3)?.split(',').filter_map(|a| a.split_once(':')).filter_map(|(f, v)| Some((f.parse().ok()?, v.parse().ok()?))).collect();
            Some((c[0].to_string(), Rule { from: Rel::parse(c.get(1)?)?, to: Rel::parse(c.get(2)?)?, atoms }))
        })
        .collect()
}

/// The folder of the hot layer the SLM writes (`MOVA_HOT`, default `data/hot`).
pub fn hot_dir() -> PathBuf {
    std::env::var("MOVA_HOT").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("hot/active"))
}

/// The induced rule sets: the cold layer compiled into the binary (`world/data/rules-induced.tsv`, from `world
/// induce`), or a hot layer read at start-up from the file named by `WORLD_RULES` (new rules tried in the working
/// pipeline without recompiling; they move into the cold layer only after the gates).
pub fn shipped(domain: &str) -> Vec<Rule> {
    static RULES: std::sync::OnceLock<Vec<(String, Rule)>> = std::sync::OnceLock::new();
    let all = RULES.get_or_init(|| match std::env::var("WORLD_RULES").ok().and_then(|p| std::fs::read_to_string(&p).ok()) {
        Some(hot) => parse_rules(&hot),
        None => parse_rules(include_str!("../data/rules-induced.tsv")),
    });
    // the tale domain uses every rule; any other domain only the general ones
    all.iter().filter(|(s, _)| domain == "tale" || s == "general").map(|(_, r)| r.clone()).collect()
}

/// The cold layer only: the rules compiled into the binary, whatever the hot layer of this process is.
pub fn cold_rules(domain: &str) -> Vec<Rule> {
    parse_rules(include_str!("../data/rules-induced.tsv")).into_iter().filter(|(s, _)| domain == "tale" || s == "general").map(|(_, r)| r).collect()
}

/// Apply rules in order to annotated words (each rule over the whole sentence, features recomputed after it).
pub fn apply_words(m: &Matrix, ws: &mut [en::annotate::Word], rules: &[Rule]) -> usize {
    let mut s: Vec<Word> = ws.iter().map(|x| Word { form: x.form.clone(), lemma: x.lemma.to_lowercase(), upos: x.upos, tag: x.tag, head: x.head, rel: x.rel }).collect();
    let mut n = 0;
    for r in rules {
        // features only for words that carry the rule's label (the rest cannot match)
        let hits: Vec<usize> = (0..s.len()).filter(|&i| s[i].rel == r.from && r.matches(s[i].rel, &atoms(m, &s, i))).collect();
        for i in hits {
            s[i].rel = r.to;
            n += 1;
        }
    }
    for (w, x) in ws.iter_mut().zip(&s) {
        w.rel = x.rel;
    }
    n
}

/// `world induce`.
pub fn run(train: &[PathBuf], dev: &Path, test: &[PathBuf], out: &Path, iters: usize, min: usize) -> Result<()> {
    let m = Matrix::global();
    let mut tr = load(train)?;
    let mut dv = load(&[dev.to_path_buf()])?;
    let mut ts: Vec<(String, Vec<Sent>)> = test.iter().map(|p| Ok((p.display().to_string(), load(std::slice::from_ref(p))?))).collect::<Result<_>>()?;
    std::fs::create_dir_all(out)?;
    let mut rules: Vec<(Rule, usize, usize)> = Vec::new();
    println!("train sentences {}  LAS {:.2}", tr.len(), las(&tr));
    for it in 0..iters {
        let cands = score(&m, &tr, min);
        let Some((r, f, b)) = cands.into_iter().find(|(_, f, b)| *f as f64 / (*f + *b) as f64 >= 0.85) else { break };
        if f < min + b {
            break;
        }
        let (af, ab, _) = apply(&m, &mut tr, &r);
        println!("rule {:2}: {}  (train fixed {f} broken {b}; applied {af}/{ab})", it + 1, r.show());
        rules.push((r, f, b));
    }
    println!("train LAS after {:.2}", las(&tr));
    let mut rep = String::new();
    // domain rules: all induced rules (the training domain); general rules: those that do not hurt the dev set
    let mut dv_copy = load(&[dev.to_path_buf()])?;
    let mut general = Vec::new();
    for (r, f0, b0) in &rules {
        let (f, b, _) = apply(&m, &mut dv_copy, r);
        if f >= b {
            general.push((r.clone(), *f0, *b0));
        }
    }
    rep += &format!("== domain rules (all {}):\n", rules.len());
    eval_set(&m, "dev", &mut dv, &rules, &mut rep);
    for (name, data) in ts.iter_mut() {
        eval_set(&m, name, data, &rules, &mut rep);
    }
    rep += &format!("== general rules (not hurting dev: {} of {}):\n", general.len(), rules.len());
    for p in test {
        let mut data = load(std::slice::from_ref(p))?;
        eval_set(&m, &p.display().to_string(), &mut data, &general, &mut rep);
    }
    print!("{rep}");
    let mut f = std::fs::File::create(out.join("rules.tsv"))?;
    writeln!(f, "# rules induced by world induce (transformation-based, error-driven); train fixed/broken; scope: domain (all) or general (not hurting dev)\nfixed\tbroken\tscope\trule")?;
    for (r, fx, b) in &rules {
        let scope = if general.iter().any(|g| g.0 == *r) { "general" } else { "domain" };
        writeln!(f, "{fx}\t{b}\t{scope}\t{}", r.show())?;
    }
    std::fs::write(out.join("report.txt"), rep)?;
    let mut mf = std::fs::File::create(out.join("rules.machine.tsv"))?;
    writeln!(mf, "# scope\tfrom\tto\tatoms (feature:value) — world induce; load with induce::parse_rules")?;
    for (r, _, _) in &rules {
        let scope = if general.iter().any(|g| g.0 == *r) { "general" } else { "domain" };
        writeln!(mf, "{scope}\t{}\t{}\t{}", r.from, r.to, r.atoms.iter().map(|(f, v)| format!("{f}:{v}")).collect::<Vec<_>>().join(","))?;
    }
    let _ = BTreeMap::<u8, u8>::new();
    Ok(())
}

/// `world rediscover <books.txt-list> [--sentences N] [--iters K]`: hide each shipped rule in turn, parse book
/// sentences with all rules (the teacher) and without the hidden one (the student), induce rules that turn the
/// student into the teacher — no gold — and check whether the hidden rule comes back (the same rule, or one with
/// the same labels that does the same work on ≥ 90% of the words it changed).
pub fn rediscover(list: &Path, sentences_n: usize, iters: usize) -> Result<()> {
    let m = Matrix::global();
    let a = crate::tree::annotator()?;
    let rules = shipped("tale");
    // sentences: up to 20 from each book, in list order, until N (deterministic)
    let mut words_all: Vec<Vec<en::annotate::Word>> = Vec::new();
    for b in std::fs::read_to_string(list)?.lines() {
        if words_all.len() >= sentences_n {
            break;
        }
        let Ok(t) = std::fs::read_to_string(b) else { continue };
        for s in crate::events::sentences(crate::events::book_body(&t)).into_iter().step_by(7).take(20) {
            let forms: Vec<String> = a.tokenize(&s).into_iter().map(|t| t.form).collect();
            if !forms.is_empty() {
                words_all.push(a.annotate(&forms));
            }
        }
    }
    words_all.truncate(sentences_n);
    println!("sentences {}  rules {}", words_all.len(), rules.len());
    let mut found = 0;
    for (h, hidden) in rules.iter().enumerate() {
        let student_rules: Vec<Rule> = rules.iter().enumerate().filter(|(i, _)| *i != h).map(|(_, r)| r.clone()).collect();
        let mut data: Vec<Sent> = Vec::new();
        let mut changed = 0usize;
        for ws in &words_all {
            let mut t = ws.clone();
            apply_words(&m, &mut t, &rules);
            let mut st = ws.clone();
            apply_words(&m, &mut st, &student_rules);
            changed += t.iter().zip(&st).filter(|(x, y)| x.rel != y.rel).count();
            let w = st.into_iter().map(|x| Word { form: x.form, lemma: x.lemma.to_lowercase(), upos: x.upos, tag: x.tag, head: x.head, rel: x.rel }).collect();
            data.push(Sent { w, gold: t.iter().map(|x| (x.head, x.rel)).collect() });
        }
        if changed == 0 {
            println!("rule {:2} hidden: changes nothing on these sentences — skipped  ({})", h + 1, hidden.show());
            continue;
        }
        let mut induced = Vec::new();
        for _ in 0..iters {
            let cands = score(&m, &data, 3);
            let Some((r, f, b)) = cands.into_iter().find(|(_, f, b)| *f as f64 / (*f + *b) as f64 >= 0.85) else { break };
            if f < 3 + b {
                break;
            }
            apply(&m, &mut data, &r);
            induced.push((r, f, b));
        }
        let exact = induced.iter().any(|(r, _, _)| r == hidden);
        let equiv = induced.iter().find(|(r, f, _)| r.from == hidden.from && r.to == hidden.to && *f * 10 >= changed * 9);
        let verdict = if exact { "REDISCOVERED (same rule)" } else if equiv.is_some() { "REDISCOVERED (equivalent)" } else { "not found" };
        found += (exact || equiv.is_some()) as usize;
        println!("rule {:2} hidden ({changed} words differ): {verdict}\n    hidden:  {}", h + 1, hidden.show());
        for (r, f, b) in &induced {
            println!("    induced: {}  (fixed {f}, broken {b})", r.show());
        }
    }
    println!("rediscovered {found} of {}", rules.len());
    Ok(())
}


/// A hideable rule: an induced rule or a kind of hand-written repair.
#[derive(Clone, Debug, PartialEq)]
enum Hideable {
    Induced(Rule),
    Hand(crate::rerank::Repair, Vec<(Rel, Rel)>),
}

impl Hideable {
    fn show(&self) -> String {
        match self {
            Hideable::Induced(r) => r.show(),
            Hideable::Hand(k, _) => format!("hand-written: {}", k.name()),
        }
    }
    fn labels(&self) -> Vec<(Rel, Rel)> {
        match self {
            Hideable::Induced(r) => vec![(r.from, r.to)],
            Hideable::Hand(_, l) => l.clone(),
        }
    }
}

/// `world rediscover-random <book-list> [--hide 2] [--trials 20] [--seed 1] [--sentences N] [--iters K]`: in each
/// trial hide `--hide` random rules from the pool (the shipped induced rules and the hand-written repair kinds),
/// parse with all rules (teacher) and without the hidden ones (student), induce without gold, and check each hidden
/// rule: rediscovered if an induced rule has the same rule (induced pool) or the same labels and fixes ≥ 90% of the
/// words that differ in those labels.
pub fn rediscover_random(list: &Path, hide: usize, trials: usize, seed: u64, sentences_n: usize, iters: usize) -> Result<()> {
    use crate::rerank::Repair;
    let m = Matrix::global();
    let a = crate::tree::annotator()?;
    let induced = shipped("tale");
    let mut pool: Vec<Hideable> = induced.iter().cloned().map(Hideable::Induced).collect();
    pool.push(Hideable::Hand(Repair::InvertedSubject, vec![(Rel::Obj, Rel::Nsubj)]));
    pool.push(Hideable::Hand(Repair::BareOblique, vec![(Rel::Obj, Rel::OblUnmarked)]));
    pool.push(Hideable::Hand(Repair::SpeechSwap, vec![(Rel::Nsubj, Rel::Ccomp), (Rel::Obj, Rel::Nsubj)]));
    pool.push(Hideable::Hand(Repair::Expletive, vec![(Rel::Nsubj, Rel::Expl)]));
    let mut words_all: Vec<Vec<en::annotate::Word>> = Vec::new();
    for b in std::fs::read_to_string(list)?.lines() {
        if words_all.len() >= sentences_n {
            break;
        }
        let Ok(t) = std::fs::read_to_string(b) else { continue };
        for s in crate::events::sentences(crate::events::book_body(&t)).into_iter().step_by(7).take(20) {
            let forms: Vec<String> = a.tokenize(&s).into_iter().map(|t| t.form).collect();
            if !forms.is_empty() {
                words_all.push(a.annotate(&forms));
            }
        }
    }
    words_all.truncate(sentences_n);
    println!("sentences {}  pool {} rules  hide {hide} per trial, {trials} trials, seed {seed}", words_all.len(), pool.len());
    let mut x = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
    let mut rnd = |n: usize| {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        (x % n as u64) as usize
    };
    let (mut total, mut found, mut skipped) = (0usize, 0usize, 0usize);
    let mut per_rule: Vec<(usize, usize)> = vec![(0, 0); pool.len()];
    for t in 0..trials {
        let mut hidden: Vec<usize> = Vec::new();
        while hidden.len() < hide.min(pool.len()) {
            let k = rnd(pool.len());
            if !hidden.contains(&k) {
                hidden.push(k);
            }
        }
        hidden.sort();
        let hand_off: Vec<Repair> = hidden.iter().filter_map(|&k| if let Hideable::Hand(r, _) = &pool[k] { Some(*r) } else { None }).collect();
        let student_rules: Vec<Rule> = induced.iter().filter(|r| !hidden.iter().any(|&k| pool[k] == Hideable::Induced((*r).clone()))).cloned().collect();
        let mut data: Vec<Sent> = Vec::new();
        let mut diff_by_label: std::collections::BTreeMap<(Rel, Rel), usize> = Default::default();
        for ws in &words_all {
            let mut te = ws.clone();
            crate::rerank::hand_pass_without(&m, &mut te, false, &[]);
            apply_words(&m, &mut te, &induced);
            let mut st = ws.clone();
            crate::rerank::hand_pass_without(&m, &mut st, false, &hand_off);
            apply_words(&m, &mut st, &student_rules);
            for (p, q) in st.iter().zip(&te) {
                if p.rel != q.rel {
                    *diff_by_label.entry((p.rel, q.rel)).or_default() += 1;
                }
            }
            let w = st.into_iter().map(|x| Word { form: x.form, lemma: x.lemma.to_lowercase(), upos: x.upos, tag: x.tag, head: x.head, rel: x.rel }).collect();
            data.push(Sent { w, gold: te.iter().map(|x| (x.head, x.rel)).collect() });
        }
        let mut got = Vec::new();
        for _ in 0..iters {
            let cands = score(&m, &data, 3);
            let Some((r, f, b)) = cands.into_iter().find(|(_, f, b)| *f as f64 / (*f + *b) as f64 >= 0.85) else { break };
            if f < 3 + b {
                break;
            }
            apply(&m, &mut data, &r);
            got.push((r, f, b));
        }
        println!("trial {:2}: hidden {}", t + 1, hidden.iter().map(|k| format!("#{}", k + 1)).collect::<Vec<_>>().join(", "));
        for &k in &hidden {
            let h = &pool[k];
            let labels = h.labels();
            let need: usize = labels.iter().map(|l| diff_by_label.get(l).copied().unwrap_or(0)).sum();
            if need == 0 {
                println!("    {} — changes nothing here (shadowed by another rule or no such words); skipped", h.show());
                skipped += 1;
                continue;
            }
            total += 1;
            per_rule[k].0 += 1;
            let same = matches!(h, Hideable::Induced(r) if got.iter().any(|(g, _, _)| g == r));
            let fixed: usize = got.iter().filter(|(g, _, _)| labels.contains(&(g.from, g.to))).map(|(_, f, _)| *f).sum();
            let ok = same || fixed * 10 >= need * 9;
            found += ok as usize;
            per_rule[k].1 += ok as usize;
            println!("    {} {} ({need} words differ, induced rules with its labels fixed {fixed})", if ok { "REDISCOVERED" } else { "not found  " }, h.show());
        }
        for (r, f, b) in &got {
            println!("      induced: {}  (fixed {f}, broken {b})", r.show());
        }
    }
    println!("rediscovered {found} of {total} hidden rules that changed something ({skipped} hidden rules changed nothing)");
    for (k, (n, f)) in per_rule.iter().enumerate().filter(|(_, x)| x.0 > 0) {
        println!("  #{:2} {f}/{n}  {}", k + 1, pool[k].show());
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------
// Self-play: the SLM proposes cheap modifications of itself (deltas over the rule set), tests each in parallel on
// cached parses, reports, and resets — the default rule set is never changed in place.

/// A delta over a rule set.
#[derive(Clone, Debug)]
pub enum Delta {
    Remove(usize),
    /// drop one condition atom of rule `i`
    Relax(usize, usize),
    /// add one condition atom to rule `i` (it fires less: on the words where it was wrong more than right)
    Specialize(usize, Atom),
    Add(Rule),
}

impl Delta {
    fn apply(&self, base: &[Rule]) -> Vec<Rule> {
        let mut r = base.to_vec();
        match self {
            Delta::Remove(i) => {
                r.remove(*i);
            }
            Delta::Relax(i, a) => {
                r[*i].atoms.remove(*a);
            }
            Delta::Specialize(i, a) => {
                r[*i].atoms.push(*a);
                r[*i].atoms.sort();
            }
            Delta::Add(x) => r.push(x.clone()),
        }
        r
    }
    fn show(&self, base: &[Rule]) -> String {
        match self {
            Delta::Remove(i) => format!("remove: {}", base[*i].show()),
            Delta::Relax(i, a) => format!("relax ({} = {} dropped): {}", feature_name(base[*i].atoms[*a].0), value_name(base[*i].atoms[*a].0, base[*i].atoms[*a].1), base[*i].show()),
            Delta::Specialize(i, a) => format!("narrow (+ {} = {}): {}", feature_name(a.0), value_name(a.0, a.1), base[*i].show()),
            Delta::Add(x) => format!("add: {}", x.show()),
        }
    }
}

/// Where each rule of `rules` fires on the data, applied in order as `apply_words` does: (rule index, the word's
/// atoms at that moment, whether the new label is the gold one).
fn fires(m: &Matrix, data: &[Cached], rules: &[Rule]) -> Vec<(usize, Vec<Atom>, bool)> {
    let mut out = Vec::new();
    for c in data {
        let mut s: Vec<Word> = c.ws.iter().map(|x| Word { form: x.form.clone(), lemma: x.lemma.to_lowercase(), upos: x.upos, tag: x.tag, head: x.head, rel: x.rel }).collect();
        for (ri, r) in rules.iter().enumerate() {
            let hits: Vec<(usize, Vec<Atom>)> = (0..s.len()).filter(|&i| s[i].rel == r.from).map(|i| (i, atoms(m, &s, i))).filter(|(_, a)| r.matches(r.from, a)).collect();
            for (i, a) in hits {
                s[i].rel = r.to;
                out.push((ri, a, c.gold.get(i).is_some_and(|g| g.1 == r.to)));
            }
        }
    }
    out
}

/// Narrowing candidates: for each rule, up to `k` atoms that would keep more of its right firings than wrong ones
/// (benefit = wrong firings dropped − right firings dropped > 0), best first.
fn narrowings(m: &Matrix, data: &[Cached], rules: &[Rule], k: usize) -> Vec<Delta> {
    let f = fires(m, data, rules);
    let mut out = Vec::new();
    for (ri, r) in rules.iter().enumerate() {
        let mine: Vec<&(usize, Vec<Atom>, bool)> = f.iter().filter(|x| x.0 == ri).collect();
        let (rt, wr) = (mine.iter().filter(|x| x.2).count() as i64, mine.iter().filter(|x| !x.2).count() as i64);
        if wr == 0 {
            continue;
        }
        let mut cnt: BTreeMap<Atom, (i64, i64)> = BTreeMap::new();
        for x in &mine {
            for a in &x.1 {
                if !r.atoms.iter().any(|b| b.0 == a.0) {
                    let e = cnt.entry(*a).or_default();
                    if x.2 { e.0 += 1 } else { e.1 += 1 }
                }
            }
        }
        let mut c: Vec<(i64, Atom)> = cnt.into_iter().map(|(a, (ra, wa))| ((wr - wa) - (rt - ra), a)).filter(|(b, _)| *b > 0).collect();
        c.sort_by(|x, y| y.0.cmp(&x.0).then(x.1.cmp(&y.1)));
        out.extend(c.into_iter().take(k).map(|(_, a)| Delta::Specialize(ri, a)));
    }
    out
}

/// Cached parse: words after the lexical repairs, with gold heads and relations.
struct Cached {
    ws: Vec<en::annotate::Word>,
    gold: Vec<(usize, Rel)>,
}

fn cache(paths: &[PathBuf]) -> Result<Vec<Cached>> {
    let a = crate::tree::annotator()?;
    let m = Matrix::global();
    let mut out = Vec::new();
    for p in paths {
        for s in en::conllu::read(p)? {
            let forms: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
            if forms.is_empty() {
                continue;
            }
            let mut ws = a.annotate(&forms);
            crate::rerank::hand_pass_without(&m, &mut ws, true, &[]);
            out.push(Cached { ws, gold: s.tokens.iter().map(|t| (t.head, t.rel)).collect() });
        }
    }
    Ok(out)
}

/// Per-sentence correct counts under a rule set.
fn correct_per(m: &Matrix, data: &[Cached], rules: &[Rule]) -> Vec<i64> {
    data.iter()
        .map(|c| {
            let mut ws = c.ws.clone();
            apply_words(m, &mut ws, rules);
            ws.iter().zip(&c.gold).filter(|(w, g)| w.head == g.0 && w.rel == g.1).count() as i64
        })
        .collect()
}

/// The experiment journal (shared by all runs): what the SLM tried, on which base and data, and what came out.
/// Before testing a delta it looks the same experiment up and reuses the remembered result.
pub(crate) struct Journal {
    path: PathBuf,
    pub(crate) seen: BTreeMap<String, serde_json::Value>,
}

/// The code version: the running binary's build time and size — after a rebuild old results are not about this code.
pub(crate) fn code_key() -> String {
    let m = std::env::current_exe().ok().and_then(|p| std::fs::metadata(p).ok());
    let t = m.as_ref().and_then(|m| m.modified().ok()).and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_secs()).unwrap_or(0);
    format!("{:x}", fnv(&format!("{t}:{}", m.map(|m| m.len()).unwrap_or(0))))
}

/// Journal entries older than this are dropped when the journal is opened (periodic cleaning).
const JOURNAL_DAYS: u64 = 7;

impl Journal {
    /// Open the journal, keeping only entries of the current code version and younger than `JOURNAL_DAYS`; the file
    /// is rewritten without the dropped entries.
    pub(crate) fn open(path: PathBuf) -> Journal {
        let mut seen = BTreeMap::new();
        let code = code_key();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let (mut kept, mut dropped) = (Vec::new(), 0usize);
        if let Ok(t) = std::fs::read_to_string(&path) {
            for l in t.lines() {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(l) {
                    let fresh = v["code"].as_str() == Some(code.as_str()) && now.saturating_sub(v["time"].as_u64().unwrap_or(0)) < JOURNAL_DAYS * 86400;
                    if !fresh {
                        dropped += 1;
                        continue;
                    }
                    if let Some(k) = v["key"].as_str() {
                        seen.insert(k.to_string(), v.clone());
                    }
                    kept.push(l.to_string());
                }
            }
            if dropped > 0 {
                let _ = std::fs::write(&path, kept.iter().map(|l| format!("{l}\n")).collect::<String>());
                println!("experiment journal: dropped {dropped} entries of another code version or older than {JOURNAL_DAYS} days");
            }
        }
        Journal { path, seen }
    }
    pub(crate) fn add(&mut self, v: serde_json::Value) -> Result<()> {
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&self.path)?;
        writeln!(f, "{v}")?;
        if let Some(k) = v["key"].as_str() {
            self.seen.insert(k.to_string(), v);
        }
        Ok(())
    }
}

pub(crate) fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

pub(crate) fn verdict(dev: i64, held: i64, p: f64) -> &'static str {
    if dev > 0 && held >= 0 && p < 0.05 {
        "positive"
    } else if dev < 0 || held < 0 {
        "negative"
    } else {
        "neutral"
    }
}

/// The causal summary of the journal: by delta type and by the features the changed rule uses — how often a change
/// of that kind helped, hurt or did nothing.
fn causal_summary(j: &Journal) -> String {
    let mut by: BTreeMap<String, [usize; 3]> = BTreeMap::new();
    for v in j.seen.values() {
        let (Some(kind), Some(vd)) = (v["kind"].as_str(), v["verdict"].as_str()) else { continue };
        let slot = match vd { "positive" => 0, "negative" => 1, _ => 2 };
        by.entry(format!("{kind}")).or_insert([0; 3])[slot] += 1;
        for f in v["features"].as_array().into_iter().flatten().filter_map(|x| x.as_str()) {
            by.entry(format!("{kind} a rule with \"{f}\"")).or_insert([0; 3])[slot] += 1;
        }
    }
    let mut rows: Vec<(String, [usize; 3])> = by.into_iter().collect();
    rows.sort_by(|a, b| (b.1[0] + b.1[1] + b.1[2]).cmp(&(a.1[0] + a.1[1] + a.1[2])).then(a.0.cmp(&b.0)));
    rows.iter().take(14).map(|(k, c)| format!("    {k}: +{} −{} ={}\n", c[0], c[1], c[2])).collect()
}

/// `world selfplay --dev <conllu>... --held <conllu>... [--gens 3] [--threads 24] [--adopt] [--out <dir>]`.
pub fn selfplay(dev: &[PathBuf], held: &[PathBuf], gens: usize, threads: usize, adopt: bool, out: &Path) -> Result<()> {
    let m = Matrix::global();
    let dv = cache(dev)?;
    let hd = cache(held)?;
    let nd: i64 = dv.iter().map(|c| c.gold.len() as i64).sum();
    let nh: i64 = hd.iter().map(|c| c.gold.len() as i64).sum();
    std::fs::create_dir_all(out)?;
    let mut log = std::io::BufWriter::new(std::fs::File::create(out.join("selfplay.jsonl"))?);
    let mut base = shipped("tale");
    println!("dev {} sentences, held-out {} sentences; base {} rules", dv.len(), hd.len(), base.len());
    let mut adopted: Vec<String> = Vec::new();
    let stage = crate::state::begin_stage("selfplay")?;
    let data_key = format!("{:x}", fnv(&dev.iter().chain(held).map(|p| format!("{}:{}", p.display(), std::fs::metadata(p).map(|m| m.len()).unwrap_or(0))).collect::<Vec<_>>().join("|")));
    let mut journal = Journal::open(PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("runs/selfplay-journal.jsonl"));
    println!("experiment journal: {} experiments remembered", journal.seen.len());
    for g in 0..gens {
        let bd = correct_per(&m, &dv, &base);
        let bh = correct_per(&m, &hd, &base);
        let (sd, sh): (i64, i64) = (bd.iter().sum(), bh.iter().sum());
        println!("generation {}: base dev LAS {:.2}, held-out LAS {:.2}", g + 1, 100.0 * sd as f64 / nd as f64, 100.0 * sh as f64 / nh as f64);
        // deltas: remove each rule, relax each atom, add the best new rules induced from the dev errors
        let mut deltas: Vec<Delta> = Vec::new();
        for i in 0..base.len() {
            deltas.push(Delta::Remove(i));
            for a in 0..base[i].atoms.len() {
                if base[i].atoms.len() > 1 {
                    deltas.push(Delta::Relax(i, a));
                }
            }
        }
        let mut sents: Vec<Sent> = dv
            .iter()
            .map(|c| {
                let mut ws = c.ws.clone();
                apply_words(&m, &mut ws, &base);
                Sent { w: ws.into_iter().map(|x| Word { form: x.form, lemma: x.lemma.to_lowercase(), upos: x.upos, tag: x.tag, head: x.head, rel: x.rel }).collect(), gold: c.gold.clone() }
            })
            .collect();
        for (r, f, b) in score(&m, &sents, 5).into_iter().filter(|(_, f, b)| *f as f64 / (*f + *b) as f64 >= 0.7).take(20) {
            let _ = (f, b);
            deltas.push(Delta::Add(r));
        }
        sents.clear();
        // remembered experiments are not run again: the journal tells what came out and when
        let base_key = format!("{:x}", fnv(&base.iter().map(Rule::show).collect::<Vec<_>>().join("|")));
        let code = code_key();
        let key_of = |d: &Delta| format!("{code}/{base_key}/{data_key}/{}", d.show(&base));
        let mut results_v: Vec<(usize, i64, i64, f64)> = Vec::new();
        let mut todo: Vec<usize> = Vec::new();
        let mut reused = 0usize;
        for (k, d) in deltas.iter().enumerate() {
            match journal.seen.get(&key_of(d)) {
                Some(v) => {
                    reused += 1;
                    results_v.push((k, v["dev"].as_i64().unwrap_or(0), v["held"].as_i64().unwrap_or(0), v["p"].as_f64().unwrap_or(1.0)));
                }
                None => todo.push(k),
            }
        }
        if reused > 0 {
            println!("  {reused} deltas remembered from the journal (not run again), {} new", todo.len());
        }
        // evaluate in parallel; each worker builds its own modified copy — the base is never touched
        let results: std::sync::Mutex<Vec<(usize, i64, i64, f64)>> = std::sync::Mutex::new(results_v);
        let next = std::sync::atomic::AtomicUsize::new(0);
        std::thread::scope(|sc| {
            for _ in 0..threads.max(1) {
                sc.spawn(|| loop {
                    let t = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    let Some(&k) = todo.get(t) else { break };
                    let d = &deltas[k];
                    let rules = d.apply(&base);
                    let cd = correct_per(&m, &dv, &rules);
                    let ch = correct_per(&m, &hd, &rules);
                    let diff: Vec<i64> = cd.iter().zip(&bd).map(|(a, b)| a - b).collect();
                    let gd: i64 = diff.iter().sum();
                    let gh: i64 = ch.iter().zip(&bh).map(|(a, b)| a - b).sum();
                    results.lock().unwrap().push((k, gd, gh, boot(&diff)));
                });
            }
        });
        let mut res = results.into_inner().unwrap();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        for (k, gd, gh, p) in res.iter().filter(|x| todo.contains(&x.0)) {
            let d = &deltas[*k];
            let (kind, rule) = match d {
                Delta::Remove(i) => ("remove", base[*i].clone()),
                Delta::Relax(i, _) => ("relax", base[*i].clone()),
                Delta::Specialize(i, _) => ("narrow", base[*i].clone()),
                Delta::Add(r) => ("add", r.clone()),
            };
            let feats: Vec<&str> = rule.atoms.iter().map(|(f, _)| feature_name(*f)).collect();
            journal.add(serde_json::json!({"key": key_of(d), "code": code_key(), "time": now, "kind": kind, "delta": d.show(&base), "features": feats, "dev": gd, "held": gh, "p": p, "verdict": verdict(*gd, *gh, *p)}))?;
        }
        res.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        println!("  {} deltas tested; best by dev gain (words):", res.len());
        for (k, gd, gh, p) in res.iter().take(12) {
            let mem = journal.seen.get(&key_of(&deltas[*k])).and_then(|v| v["time"].as_u64()).filter(|_| !todo.contains(k)).map(|t| format!(" [remembered, tried {} s ago]", now.saturating_sub(t))).unwrap_or_default();
            println!("    {:8} dev {gd:+4}  held {gh:+4}  p {p:.3}  {}{mem}", verdict(*gd, *gh, *p), deltas[*k].show(&base));
        }
        for (k, gd, gh, p) in &res {
            writeln!(log, "{}", serde_json::json!({"generation": g + 1, "delta": deltas[*k].show(&base), "dev_gain_words": gd, "held_gain_words": gh, "p_dev": p}))?;
        }
        log.flush()?;
        if !adopt {
            break;
        }
        // adopt the best delta significant on dev and not hurting the held-out set (in memory only)
        let Some((k, gd, gh, p)) = res.iter().find(|(_, gd, gh, p)| *gd > 0 && *gh >= 0 && *p < 0.05).cloned() else {
            println!("  no delta both significant on dev and safe on held-out — stop");
            break;
        };
        println!("  adopt (dev {gd:+}, held {gh:+}, p {p:.3}): {}", deltas[k].show(&base));
        adopted.push(format!("{} (dev {gd:+}, held {gh:+}, p {p:.3})", deltas[k].show(&base)));
        base = deltas[k].apply(&base);
    }
    println!("what the journal says about kinds of change (positive / negative / neutral):\n{}", causal_summary(&journal));
    let mut f = std::fs::File::create(out.join("rules-proposed.tsv"))?;
    writeln!(f, "# proposed by world selfplay (not shipped; review, then copy into world/data/rules-induced.tsv)")?;
    for r in &base {
        writeln!(f, "domain\t{}\t{}\t{}", r.from, r.to, r.atoms.iter().map(|(a, v)| format!("{a}:{v}")).collect::<Vec<_>>().join(","))?;
    }
    // the SLM writes its own hot layer: the adopted rule set goes to <MOVA_HOT>/rules-induced.tsv (used when
    // WORLD_RULES points there; moved into the cold layer only after the gates)
    drop(stage);
    if adopt {
        let hot = hot_dir();
        std::fs::create_dir_all(&hot)?;
        let body: String = base.iter().map(|r| format!("domain\t{}\t{}\t{}\n", r.from, r.to, r.atoms.iter().map(|(a, v)| format!("{a}:{v}")).collect::<Vec<_>>().join(","))).collect();
        crate::store::write_atomic(&hot.join("rules-induced.tsv"), format!("# hot layer written by world selfplay --adopt\n{body}").as_bytes())?;
        println!("hot layer: {}", hot.join("rules-induced.tsv").display());
        if !adopted.is_empty() {
            // the state changes: serialise the brain, record the new state with its letter, deserialise under the new rules
            let letter = format!("Selfplay adopted {} changes: {}.\nDecision: keep them in the hot layer and test them in work; move them into the skeleton only after the gates.", adopted.len(), adopted.join("; "));
            let id = crate::state::transition(&format!("selfplay adopted {} changes", adopted.len()), Some(&letter), &[], &[])?;
            println!("new state {id}: brain serialised, letter written, brain deserialised under the new rules");
        }
    }
    Ok(())
}

/// One generation of self-play from a given rule set on small sets: the best delta that is significant on dev and does
/// not hurt held-out — (description, rules after, dev gain, held gain, p). Used by the family's state members.
/// A change found by `explore`, transferable to another rule set: add a rule, or remove a rule.
#[derive(Clone, Debug)]
pub enum Change {
    Add(Rule),
    Remove(Rule),
    /// a rule changed in place (relaxed or narrowed): old → new, keeping its position in the order
    Replace(Rule, Rule),
}

impl Change {
    /// Apply to another rule set (removing a rule that is not there changes nothing).
    pub fn apply_to(&self, rules: &[Rule]) -> Vec<Rule> {
        match self {
            Change::Add(r) => {
                let mut v = rules.to_vec();
                if !v.contains(r) {
                    v.push(r.clone());
                }
                v
            }
            Change::Remove(r) => rules.iter().filter(|x| *x != r).cloned().collect(),
            Change::Replace(old, new) => rules.iter().map(|x| if x == old { new.clone() } else { x.clone() }).collect(),
        }
    }
    /// Already true of a rule set: the rule is there, or a more general rule with the same labels (its conditions a
    /// subset of the new rule's) is there — the new one adds nothing (add); the rule is absent (remove).
    pub fn holds_in(&self, rules: &[Rule]) -> bool {
        match self {
            Change::Add(r) => rules.iter().any(|x| x.from == r.from && x.to == r.to && x.atoms.iter().all(|a| r.atoms.contains(a))),
            Change::Remove(r) => !rules.contains(r),
            Change::Replace(old, new) => !rules.contains(old) || rules.contains(new),
        }
    }
}

pub fn explore_change(base: Vec<Rule>, dev: &Path, held: &Path, skip: &[Rule]) -> Result<Option<(String, Change, i64, i64, f64)>> {
    let r = explore_inner(base.clone(), dev, held, skip)?;
    Ok(r.map(|(desc, d, gd, gh, p)| {
        let ch = match d {
            Delta::Add(r) => Change::Add(r),
            Delta::Remove(i) => Change::Remove(base[i].clone()),
            d @ (Delta::Relax(i, _) | Delta::Specialize(i, _)) => Change::Replace(base[i].clone(), d.apply(&base)[i].clone()),
        };
        (desc, ch, gd, gh, p)
    }))
}

pub fn explore(base: Vec<Rule>, dev: &Path, held: &Path) -> Result<Option<(String, Vec<Rule>, i64, i64, f64)>> {
    let b2 = base.clone();
    Ok(explore_inner(base, dev, held, &[])?.map(|(desc, d, gd, gh, p)| (desc, d.apply(&b2), gd, gh, p)))
}

/// Up to `k` useful changes (positive on dev, not hurting held-out), best first.
pub fn explore_changes(base: Vec<Rule>, dev: &Path, held: &Path, skip: &[Rule], k: usize) -> Result<Vec<(String, Change, i64, i64, f64)>> {
    let all = explore_all(base.clone(), dev, held, skip, 40)?;
    Ok(all
        .into_iter()
        .take(k)
        .map(|(desc, d, gd, gh, p)| {
            let ch = match d {
                Delta::Add(r) => Change::Add(r),
                Delta::Remove(i) => Change::Remove(base[i].clone()),
                ref d @ (Delta::Relax(i, _) | Delta::Specialize(i, _)) => Change::Replace(base[i].clone(), d.apply(&base)[i].clone()),
            };
            (desc, ch, gd, gh, p)
        })
        .collect())
}

fn explore_inner(base: Vec<Rule>, dev: &Path, held: &Path, skip: &[Rule]) -> Result<Option<(String, Delta, i64, i64, f64)>> {
    Ok(explore_all(base, dev, held, skip, 15)?.into_iter().next())
}

/// All useful deltas (positive on dev, not hurting held-out), best dev gain first. `skip`: the target rule set — an
/// added rule already there is no idea. `cands`: how many candidate added rules to test.
fn explore_all(base: Vec<Rule>, dev: &Path, held: &Path, skip: &[Rule], cands: usize) -> Result<Vec<(String, Delta, i64, i64, f64)>> {
    let m = Matrix::global();
    let dv = cache(&[dev.to_path_buf()])?;
    let hd = cache(&[held.to_path_buf()])?;
    let bd = correct_per(&m, &dv, &base);
    let bh = correct_per(&m, &hd, &base);
    let mut deltas: Vec<Delta> = Vec::new();
    for i in 0..base.len() {
        deltas.push(Delta::Remove(i));
        if base[i].atoms.len() > 1 {
            for a in 0..base[i].atoms.len() {
                deltas.push(Delta::Relax(i, a));
            }
        }
    }
    deltas.extend(narrowings(&m, &dv, &base, 3));
    let debug = std::env::var("INDUCE_DEBUG").is_ok();
    let sents: Vec<Sent> = dv
        .iter()
        .map(|c| {
            let mut ws = c.ws.clone();
            apply_words(&m, &mut ws, &base);
            Sent { w: ws.into_iter().map(|x| Word { form: x.form, lemma: x.lemma.to_lowercase(), upos: x.upos, tag: x.tag, head: x.head, rel: x.rel }).collect(), gold: c.gold.clone() }
        })
        .collect();
    for (r, f, b) in score(&m, &sents, 3).into_iter().filter(|(_, f, b)| *f as f64 / (*f + *b) as f64 >= 0.7).filter(|(r, _, _)| !skip.contains(r)).take(cands) {
        let _ = (f, b);
        deltas.push(Delta::Add(r));
    }
    let mut found: Vec<(String, Delta, i64, i64, f64)> = Vec::new();
    for d in &deltas {
        let rules = d.apply(&base);
        let cd = correct_per(&m, &dv, &rules);
        let diff: Vec<i64> = cd.iter().zip(&bd).map(|(a, b)| a - b).collect();
        let gd: i64 = diff.iter().sum();
        if debug && !matches!(d, Delta::Add(_)) {
            eprintln!("delta {} → dev {gd:+}", d.show(&base));
        }
        if gd <= 0 {
            continue;
        }
        let ch = correct_per(&m, &hd, &rules);
        let gh: i64 = ch.iter().zip(&bh).map(|(a, b)| a - b).sum();
        if gh >= 0 {
            found.push((d.show(&base), d.clone(), gd, gh, boot(&diff)));
        }
    }
    found.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)));
    Ok(found)
}

/// Grammar constraints of UD English a labelling may not break: a head with two subjects or two objects, a subject or
/// object with a case marker, a subject or object set off by punctuation on both sides with no dependents of its own
/// (a vocative: "say, March, when…", "look, sir!"). Returns how many are broken.
pub fn violations(ws: &[en::annotate::Word]) -> i64 {
    let mut v = 0i64;
    let core = |r: Rel| matches!(r, Rel::Nsubj | Rel::NsubjPass | Rel::Obj | Rel::Iobj);
    let mut subj = vec![0i64; ws.len() + 1];
    let mut obj = vec![0i64; ws.len() + 1];
    for w in ws {
        if matches!(w.rel, Rel::Nsubj | Rel::NsubjPass | Rel::Csubj | Rel::CsubjPass) {
            subj[w.head.min(ws.len())] += 1;
        }
        if w.rel == Rel::Obj {
            obj[w.head.min(ws.len())] += 1;
        }
    }
    v += subj.iter().chain(obj.iter()).map(|&n| (n - 1).max(0)).sum::<i64>();
    let edge = |j: Option<usize>| j.is_none_or(|j| ws.get(j).is_none_or(|x| matches!(x.form.as_str(), "," | "!" | "?" | "." | ";" | "\"" | "“" | "”" | "'" | "‘" | "’")));
    for (i, w) in ws.iter().enumerate() {
        if !core(w.rel) {
            continue;
        }
        let kids: Vec<&en::annotate::Word> = ws.iter().filter(|x| x.head == i + 1).collect();
        if kids.iter().any(|x| x.rel == Rel::Case && !matches!(x.form.as_str(), "'s" | "’s" | "'" | "’")) {
            v += 1;
        }
        let bare = kids.iter().all(|x| matches!(x.rel, Rel::Flat | Rel::Compound | Rel::Punct));
        if bare && matches!(w.upos, UPos::PROPN | UPos::NOUN) && edge(i.checked_sub(1)) && edge(Some(i + 1)) {
            v += 1;
        }
    }
    v
}

/// Words whose label differs between two rule sets on a big sample, evenly spread, at most `max`: (forms, word,
/// head, old label, new label), and how many changed in all.
pub fn changed_words(r0: &[Rule], r1: &[Rule], big: &[Vec<en::annotate::Word>], max: usize) -> (Vec<(Vec<String>, usize, usize, Rel, Rel)>, usize) {
    let m = Matrix::global();
    let mut all = Vec::new();
    for ws in big {
        let (mut a, mut b) = (ws.clone(), ws.clone());
        apply_words(&m, &mut a, r0);
        apply_words(&m, &mut b, r1);
        for i in 0..a.len() {
            if a[i].rel != b[i].rel {
                all.push((a.iter().map(|w| w.form.clone()).collect::<Vec<_>>(), i, a[i].head, a[i].rel, b[i].rel));
            }
        }
    }
    let n = all.len();
    if n <= max {
        return (all, n);
    }
    let step = n as f64 / max as f64;
    ((0..max).map(|k| all[(k as f64 * step) as usize].clone()).collect(), n)
}

/// Per sentence of a big sample under `rules`: (absurd arcs — matrix score 4, sensible arcs — score 0–1). The judge
/// `jm` is the matrix of the sample's domain; features come from the global matrix as everywhere in induction.
fn absurd_per(m: &Matrix, jm: &Matrix, data: &[Vec<en::annotate::Word>], rules: &[Rule]) -> Vec<(i64, i64)> {
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(4).min(10);
    let chunk = data.len().div_ceil(threads).max(1);
    let mut out = vec![(0i64, 0i64); data.len()];
    std::thread::scope(|sc| {
        for (part, slot) in data.chunks(chunk).zip(out.chunks_mut(chunk)) {
            sc.spawn(move || {
                for (ws0, o) in part.iter().zip(slot.iter_mut()) {
                    let mut ws = ws0.clone();
                    apply_words(m, &mut ws, rules);
                    let nodes: Vec<crate::sense::Node> = ws.iter().map(|w| crate::sense::Node { form: w.form.clone(), lemma: w.lemma.to_lowercase(), upos: Some(w.upos), head: w.head, rel: w.rel }).collect();
                    let (js, _) = crate::absurd::judge(jm, &nodes);
                    *o = (js.iter().filter(|j| j.score >= 4).count() as i64, js.iter().filter(|j| j.score <= 1).count() as i64);
                }
            });
        }
    });
    out
}

/// Exploring on a big sample without gold (`big::sample`): deltas of the base rules (remove, relax) and new rules
/// induced from pseudo-gold — an argument the matrix finds absurd (score 4) in its role but sensible (0–1) in the
/// other is relabelled (nsubj ↔ obj) — judged by the matrix over the whole sample. A delta is an idea when it
/// removes absurd arcs (paired bootstrap over sentences) and at least half of them come back as sensible ones (no
/// gain by hiding arcs from the judge), and it does not hurt the gold held-out set. Returns (description, change,
/// absurd arcs removed, gold held-out gain, p), most removed first.
pub fn explore_big(base: Vec<Rule>, big: &[Vec<en::annotate::Word>], domain: &str, held: &Path, skip: &[Rule], k: usize) -> Result<Vec<(String, Change, i64, i64, f64)>> {
    let m = Matrix::global();
    let jm = Matrix::global_in(domain);
    let hd = cache(&[held.to_path_buf()])?;
    let bh = correct_per(&m, &hd, &base);
    let b0 = absurd_per(&m, &jm, big, &base);
    let viol = |rules: &[Rule]| -> i64 {
        big.iter().map(|ws0| {
            let mut ws = ws0.clone();
            apply_words(&m, &mut ws, rules);
            violations(&ws)
        }).sum()
    };
    let v0 = viol(&base);
    let mut deltas: Vec<Delta> = Vec::new();
    for i in 0..base.len() {
        deltas.push(Delta::Remove(i));
        if base[i].atoms.len() > 1 {
            for a in 0..base[i].atoms.len() {
                deltas.push(Delta::Relax(i, a));
            }
        }
    }
    // pseudo-gold from the matrix
    let mut sents: Vec<Sent> = Vec::new();
    for ws0 in big {
        let mut ws = ws0.clone();
        apply_words(&m, &mut ws, &base);
        let nodes: Vec<crate::sense::Node> = ws.iter().map(|w| crate::sense::Node { form: w.form.clone(), lemma: w.lemma.to_lowercase(), upos: Some(w.upos), head: w.head, rel: w.rel }).collect();
        let (js, _) = crate::absurd::judge(&jm, &nodes);
        if std::env::var("BIG_SHOW").is_ok() {
            for j in js.iter().filter(|j| j.score >= 4) {
                let text: Vec<&str> = ws.iter().map(|w| w.form.as_str()).collect();
                eprintln!("absurd: {} —[{} as {}, swapped {:?}]→ {} | {}", ws[j.verb].lemma, ws[j.noun].lemma, ws[j.noun].rel, j.swapped, ws[j.noun].upos, text.join(" "));
            }
        }
        let mut gold: Vec<(usize, Rel)> = ws.iter().map(|w| (w.head, w.rel)).collect();
        let mut any = false;
        for j in js.iter().filter(|j| j.score >= 4 && j.swapped.is_some_and(|s| s <= 1)) {
            if matches!(ws[j.noun].rel, Rel::Nsubj | Rel::Obj) {
                gold[j.noun].1 = if j.role == 0 { Rel::Obj } else { Rel::Nsubj };
                any = true;
            }
        }
        if any {
            sents.push(Sent { w: ws.into_iter().map(|x| Word { form: x.form, lemma: x.lemma.to_lowercase(), upos: x.upos, tag: x.tag, head: x.head, rel: x.rel }).collect(), gold });
        }
    }
    // a new rule may not test the matrix itself: the matrix is its judge here (no circular gains)
    let no_matrix = |r: &Rule| r.atoms.iter().all(|(f, _)| !feature_name(*f).contains("matrix"));
    for (r, _, _) in score(&m, &sents, 10).into_iter().filter(|(_, f, b)| *f as f64 / (*f + *b) as f64 >= 0.7).filter(|(r, _, _)| !skip.contains(r) && no_matrix(r)).take(30) {
        deltas.push(Delta::Add(r));
    }
    let debug = std::env::var("INDUCE_DEBUG").is_ok();
    if debug {
        let (a, s): (i64, i64) = b0.iter().fold((0, 0), |x, y| (x.0 + y.0, x.1 + y.1));
        eprintln!("big: {} sentences, absurd arcs {a}, sensible {s}; pseudo-gold sentences {}; deltas {}", big.len(), sents.len(), deltas.len());
    }
    let mut found = Vec::new();
    for d in &deltas {
        let rules = d.apply(&base);
        let b1 = absurd_per(&m, &jm, big, &rules);
        let diff: Vec<i64> = b1.iter().zip(&b0).map(|(x, y)| y.0 - x.0).collect();
        let removed: i64 = diff.iter().sum();
        let sensible: i64 = b1.iter().zip(&b0).map(|(x, y)| x.1 - y.1).sum();
        if debug && (removed != 0 || sensible != 0) {
            eprintln!("  big delta {} → absurd −{removed}, sensible {sensible:+}", d.show(&base));
        }
        if removed <= 0 || 2 * sensible < removed {
            continue;
        }
        // a fix the matrix likes may still break the grammar (two subjects, a vocative as subject): not allowed
        let dv = viol(&rules) - v0;
        let gh: i64 = correct_per(&m, &hd, &rules).iter().zip(&bh).map(|(a, b)| a - b).sum();
        if debug {
            eprintln!("    passes the matrix; grammar violations {dv:+}; gold held-out {gh:+}");
        }
        if gh < 0 || dv > 0 {
            continue;
        }
        let ch = match d {
            Delta::Add(r) => Change::Add(r.clone()),
            Delta::Remove(i) => Change::Remove(base[*i].clone()),
            Delta::Relax(i, _) | Delta::Specialize(i, _) => Change::Replace(base[*i].clone(), d.apply(&base)[*i].clone()),
        };
        found.push((format!("{} [big: absurd −{removed}, sensible {sensible:+}, grammar {dv:+}]", d.show(&base)), ch, removed, gh, boot(&diff)));
    }
    found.sort_by(|a, b| b.2.cmp(&a.2).then(a.0.cmp(&b.0)));
    found.truncate(k);
    Ok(found)
}

/// Absurd and sensible arcs of a big sample under a rule set, and the per-sentence absurd counts (for the guard).
pub fn big_score(rules: &[Rule], big: &[Vec<en::annotate::Word>], domain: &str) -> Vec<(i64, i64)> {
    absurd_per(&Matrix::global(), &Matrix::global_in(domain), big, rules)
}
