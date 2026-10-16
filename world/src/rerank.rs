//! Reranking parse candidates with the absurdity matrix: the parser's `k` best trees, each penalized by the
//! absurdity of its subject/object readings (`absurd::judge`, level-1 matrix), the best `score − λ·penalty` wins.
//! λ is chosen on a dev split and reported on a test split; LAS against gold, paired bootstrap over sentences.
//!
//!   world absurd-rerank <dev.conllu> <test.conllu> [--k 8]
//!
//! Penalty of a tree: Σ over judged arguments of w[score], w = [0, 0, 1, 2, 4]. λ = ∞ means: the least absurd
//! candidate, ties by parser score. Deterministic (fixed bootstrap seed, BTreeMap only).

use std::path::Path;

use anyhow::{Result, bail};
use en::conllu::Sentence;

use crate::absurd::{Matrix, judge};
use crate::sense::Node;

const W: [f64; 5] = [0.0, 0.0, 1.0, 2.0, 4.0];
const LAMBDAS: [f64; 10] = [0.0, 0.1, 0.3, 1.0, 3.0, 10.0, 30.0, 100.0, 1000.0, f64::INFINITY];

struct Cand {
    score: f64,
    penalty: f64,
    /// correct (head, rel) words against gold
    correct: usize,
}

struct Item {
    n: usize,
    greedy: usize,
    cands: Vec<Cand>,
}

fn items(path: &Path, m: &Matrix, k: usize) -> Result<Vec<Item>> {
    let a = crate::tree::annotator()?;
    let gold = en::conllu::read(path)?;
    let mut out = Vec::new();
    for s in &gold {
        let forms: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
        if forms.is_empty() {
            continue;
        }
        let ok = |ws: &[en::annotate::Word]| ws.iter().zip(&s.tokens).filter(|(w, g)| w.head == g.head && w.rel == g.rel).count();
        let greedy = ok(&a.annotate(&forms));
        let cands = a
            .annotate_kbest(&forms, k)
            .into_iter()
            .map(|(score, ws)| {
                let nodes: Vec<Node> = ws.iter().map(|w| Node { form: w.form.clone(), lemma: w.lemma.to_lowercase(), upos: Some(w.upos), head: w.head, rel: w.rel }).collect();
                let (js, _) = judge(m, &nodes);
                Cand { score: score as f64, penalty: js.iter().map(|j| W[j.score as usize]).sum(), correct: ok(&ws) }
            })
            .collect();
        out.push(Item { n: s.tokens.len(), greedy, cands });
    }
    Ok(out)
}

fn pick(it: &Item, lambda: f64) -> usize {
    let key = |c: &Cand| if lambda.is_infinite() { (-c.penalty, c.score) } else { (c.score - lambda * c.penalty, 0.0) };
    let mut best = 0;
    for i in 1..it.cands.len() {
        let (a, b) = (key(&it.cands[i]), key(&it.cands[best]));
        if a.0 > b.0 || (a.0 == b.0 && a.1 > b.1) {
            best = i;
        }
    }
    best
}

fn las(its: &[Item], f: &dyn Fn(&Item) -> usize) -> f64 {
    let (c, n): (usize, usize) = its.iter().fold((0, 0), |(c, n), it| (c + f(it), n + it.n));
    100.0 * c as f64 / n.max(1) as f64
}

/// Paired bootstrap: share of resamples where the system is not better than the baseline (one-sided p).
fn bootstrap(its: &[Item], base: &dyn Fn(&Item) -> usize, sys: &dyn Fn(&Item) -> usize) -> f64 {
    let d: Vec<i64> = its.iter().map(|it| sys(it) as i64 - base(it) as i64).collect();
    let mut x: u64 = 0x9e3779b97f4a7c15;
    let mut worse = 0;
    let r = 1000;
    for _ in 0..r {
        let mut sum = 0i64;
        for _ in 0..d.len() {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            sum += d[(x % d.len() as u64) as usize];
        }
        worse += (sum <= 0) as usize;
    }
    worse as f64 / r as f64
}

fn report(name: &str, its: &[Item], lambda: f64) -> String {
    let top = |it: &Item| it.cands.first().map_or(0, |c| c.correct);
    let sys = |it: &Item| it.cands.get(pick(it, lambda)).map_or(0, |c| c.correct);
    let changed = its.iter().filter(|it| pick(it, lambda) != 0).count();
    let better = its.iter().filter(|it| sys(it) > top(it)).count();
    let worse = its.iter().filter(|it| sys(it) < top(it)).count();
    let oracle = las(its, &|it| it.cands.iter().map(|c| c.correct).max().unwrap_or(0));
    format!(
        "{name}: sentences {}  LAS greedy {:.2}  beam top-1 {:.2}  reranked (λ={lambda}) {:.2}  oracle of k {:.2}\n  changed {changed} sentences: better {better}, worse {worse}; paired bootstrap p(reranked ≤ top-1) = {:.3}\n",
        its.len(),
        las(its, &|it| it.greedy),
        las(its, &top),
        las(its, &sys),
        oracle,
        bootstrap(its, &top, &sys)
    )
}

/// `world absurd-rerank`.
pub fn run(dev: &Path, test: &Path, k: usize) -> Result<()> {
    if k < 2 {
        bail!("k must be at least 2");
    }
    let m = Matrix::global();
    let d = items(dev, &m, k)?;
    let t = items(test, &m, k)?;
    println!("matrix {}  k {k}", m.name());
    let mut best = (f64::MIN, 0.0);
    for &l in &LAMBDAS {
        let v = las(&d, &|it| it.cands.get(pick(it, l)).map_or(0, |c| c.correct));
        println!("  dev λ={l:<6} LAS {v:.2}  changed {}", d.iter().filter(|it| pick(it, l) != 0).count());
        if v > best.0 + 1e-9 {
            best = (v, l);
        }
    }
    print!("{}", report("dev", &d, best.1));
    print!("{}", report("test", &t, best.1));
    Ok(())
}

/// Split a CoNLL-U file into even / odd sentences (dev / test for a corpus without a split).
pub fn split(input: &Path, even: &Path, odd: &Path) -> Result<()> {
    let ss: Vec<Sentence> = en::conllu::read(input)?;
    let text = std::fs::read_to_string(input)?;
    let blocks: Vec<&str> = text.split("\n\n").filter(|b| b.lines().any(|l| l.split('\t').count() == 10)).collect();
    if blocks.len() != ss.len() {
        bail!("{}: {} blocks vs {} sentences", input.display(), blocks.len(), ss.len());
    }
    let pick = |r: usize| blocks.iter().enumerate().filter(|(i, _)| i % 2 == r).map(|(_, b)| format!("{}\n\n", b.trim_matches('\n'))).collect::<String>();
    std::fs::write(even, pick(0))?;
    std::fs::write(odd, pick(1))?;
    Ok(())
}

/// A label repair driven by the absurdity matrix (applied to the parser's tree).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Repair {
    /// obj after the verb, absurd as an object (OBJ ≥ 3) but a normal doer (SUBJ ≤ 1), the verb has no subject:
    /// an inverted subject ("'Run!' cried the hare") → nsubj
    InvertedSubject,
    /// a noun of measure or time after an intransitive verb ("went a long way") → obl:unmarked
    BareOblique,
    /// an archaic adverb read as a noun ("Whence came this stranger?") → ADV, advmod
    ArchaicAdverb,
    /// "there" read as the subject of an inverted clause ("there lived a king") → expl
    Expletive,
    /// a noun between commas right after a speech verb ("I say, young man, …") → vocative
    Vocative,
    /// a speech verb with the quoted noun read as the subject and the speaker as the object ("'Good morning,'
    /// said the fox": morning nsubj, fox obj) → the quoted noun ccomp, the speaker nsubj
    SpeechSwap,
}

impl Repair {
    pub fn name(self) -> &'static str {
        match self {
            Repair::InvertedSubject => "inverted-subject (obj → nsubj)",
            Repair::BareOblique => "bare-oblique (obj → obl:unmarked)",
            Repair::ArchaicAdverb => "archaic-adverb (noun → ADV advmod)",
            Repair::Vocative => "vocative (obj between commas after a speech verb → vocative)",
            Repair::Expletive => "expletive (there nsubj → expl)",
            Repair::SpeechSwap => "speech-swap (quote nsubj → ccomp, speaker obj → nsubj)",
        }
    }
}

/// Repairs for one tree: (word index, new relation, rule, why).
///
/// - inverted subject: an obj noun after its verb, the verb has no subject, and the verb takes no object
///   (intransitive by the matrix, or in `patient_not_animate`) or the cell is absurd as an object (OBJ ≥ 3); the
///   noun is a normal doer (SUBJ ≤ 1 in the real or the fairy-tale layer, or animate by level-1 categories when the cell is unknown) and not a noun of
///   measure or time; the verb is finite and has no xcomp/ccomp after the noun → nsubj ("'Run!' cried the hare",
///   "then came a knight");
/// - bare oblique: an obj noun of measure or time (`measure_time`) after an intransitive verb → obl:unmarked
///   ("went a long way", "waited a while").
pub fn repairs(m: &Matrix, t: &[Node], tags: &[en::gram::Tag]) -> Vec<(usize, en::gram::Rel, Repair, String)> {
    use en::gram::{Rel, Tag, UPos};
    let mut out = Vec::new();
    for (i, w) in t.iter().enumerate() {
        if matches!(w.upos, Some(UPos::NOUN | UPos::PROPN)) && w.rel != Rel::Advmod && global::selection("archaic_adverbs").contains(&w.lemma.as_str()) {
            out.push((i, Rel::Advmod, Repair::ArchaicAdverb, format!("{} is an archaic adverb (level 1), not a noun", w.lemma)));
        }
    }
    if !out.is_empty() {
        return out;
    }
    // the doer score of a noun for a verb: the lower of the real and the fairy-tale layer
    let doer_score = |verb: &str, noun: &str| match (m.score(verb, noun, 0), global::absurdity_in("tale", verb, noun).map(|x| x[0])) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (a, b) => a.or(b),
    };
    for (i, w) in t.iter().enumerate() {
        if w.rel != Rel::Obj || !matches!(w.upos, Some(UPos::NOUN | UPos::PROPN)) || w.head == 0 {
            continue;
        }
        let v = w.head - 1;
        if t[v].upos != Some(UPos::VERB) || i < v {
            continue;
        }
        let verb = t[v].lemma.as_str();
        let speech = global::selection("patient_not_animate").contains(&verb);
        let is_there = |x: &Node| x.lemma == "there";
        // "there" as nsubj of an inverted clause is an expletive and does not block ("there lived a king")
        let there = t.iter().position(|x| x.head == v + 1 && x.rel == Rel::Nsubj && is_there(x));
        let subj = t.iter().position(|x| x.head == v + 1 && matches!(x.rel, Rel::Nsubj | Rel::NsubjPass | Rel::Csubj) && !is_there(x));
        let intrans = global::absurd_intransitive(verb) || speech;
        let measure = global::selection("measure_time").contains(&w.lemma.as_str());
        let o = m.score(verb, &w.lemma, 1);
        if measure && w.upos == Some(UPos::NOUN) && (global::absurd_intransitive(verb) || o.is_some_and(|o| o >= 3)) {
            out.push((i, Rel::OblUnmarked, Repair::BareOblique, format!("{verb} does not take {} as an object (matrix), it is a noun of measure or time", w.lemma)));
            continue;
        }
        // inversion needs a finite verb ("said", "came"), not a participle or infinitive ("becoming a sand",
        // "going the same road"), and no xcomp after the noun ("make the monkey do", "hear the mermaids singing");
        // a ccomp after the noun blocks too, except after a speech verb (the rest of the quotation)
        // "quoth" is always finite (archaic past), whatever the tagger says
        let finite = verb == "quoth" || tags.get(v).is_some_and(|g| matches!(g, Tag::VBD | Tag::VBZ | Tag::VBP));
        // tried 02.10: "I say, young man" → vocative (obj between commas after a speech verb); on the tales dev split
        // it fixed 0 and broke 1 — rejected; the Repair::Vocative name is kept for the journal
        let clause_after = t.iter().enumerate().any(|(j, x)| j > i && x.head == v + 1 && (x.rel == Rel::Xcomp || (x.rel == Rel::Ccomp && !speech)));
        if measure || !finite || clause_after || !(intrans || o.is_some_and(|o| o >= 3)) {
            continue;
        }
        let s = doer_score(verb, &w.lemma);
        let doer = match s {
            Some(s) => s <= 1,
            // names after a speech verb are speakers; other unknown nouns need level-1 animacy
            None => (w.upos == Some(UPos::PROPN) && speech) || matches!(crate::sense::animacy(&w.lemma), crate::sense::Anim::Animate(_)),
        };
        if !doer {
            continue;
        }
        let why = format!("{verb} {} — {} as the doer {}", if intrans { "takes no object" } else { "cannot take it as an object" }, w.lemma, s.map_or("(animate or a name after a speech verb)".to_string(), |s| format!("SUBJ={s}")));
        match subj {
            None => {
                if let Some(th) = there {
                    out.push((th, Rel::Expl, Repair::Expletive, format!("there before an inverted subject ({})", w.lemma)));
                }
                out.push((i, Rel::Nsubj, Repair::InvertedSubject, why));
            }
            // a speech verb whose subject is the quoted noun before it, absurd as a speaker: swap
            Some(q) if speech && q < v && matches!(t[q].upos, Some(UPos::NOUN)) && doer_score(verb, &t[q].lemma).is_some_and(|x| x >= 3) => {
                out.push((q, Rel::Ccomp, Repair::SpeechSwap, format!("{} cannot speak ({verb}|{} SUBJ ≥ 3): it is the quoted words", t[q].lemma, t[q].lemma)));
                out.push((i, Rel::Nsubj, Repair::SpeechSwap, why));
            }
            Some(_) => {}
        }
    }
    out
}

/// `world absurd-repair <dev.conllu> <test.conllu>`: per-rule applications, arcs fixed and broken, LAS before and after.
pub fn run_repair(dev: &Path, test: &Path, show: usize) -> Result<()> {
    let m = Matrix::global();
    let a = crate::tree::annotator()?;
    for (name, path) in [("dev", dev), ("test", test)] {
        let gold = en::conllu::read(path)?;
        let mut rules: std::collections::BTreeMap<Repair, (usize, usize, usize)> = Default::default();
        let (mut n, mut before, mut after) = (0usize, 0usize, 0usize);
        let mut diffs: Vec<i64> = Vec::new();
        let mut shown = 0;
        for s in &gold {
            let forms: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
            if forms.is_empty() {
                continue;
            }
            let ws = a.annotate(&forms);
            let mut t: Vec<Node> = ws.iter().map(|w| Node { form: w.form.clone(), lemma: w.lemma.to_lowercase(), upos: Some(w.upos), head: w.head, rel: w.rel }).collect();
            let ok = |t: &[Node]| t.iter().zip(&s.tokens).filter(|(w, g)| w.head == g.head && w.rel == g.rel).count();
            let b = ok(&t);
            let tags: Vec<en::gram::Tag> = ws.iter().map(|w| w.tag).collect();
            // two passes, as in `repair_words`: lexical repairs first, then the role repairs
            for (i, rel, r, why) in { let first = repairs(&m, &t, &tags); let lexical = first.iter().any(|x| x.2 == Repair::ArchaicAdverb); let mut all = first.clone(); if lexical { let mut t2 = t.clone(); for (i, rel, _, _) in &first { t2[*i].rel = *rel; t2[*i].upos = Some(en::gram::UPos::ADV); } all.extend(repairs(&m, &t2, &tags)); } all } {
                let was = t[i].head == s.tokens[i].head && t[i].rel == s.tokens[i].rel;
                let now = t[i].head == s.tokens[i].head && rel == s.tokens[i].rel;
                let e = rules.entry(r).or_default();
                e.0 += 1;
                e.1 += (!was && now) as usize;
                e.2 += (was && !now) as usize;
                if shown < show && name == "dev" {
                    shown += 1;
                    println!("  {} {}: {} — {why}; gold {}", r.name(), if now && !was { "FIX" } else if was && !now { "BREAK" } else { "same" }, s.text, s.tokens[i].rel);
                }
                t[i].rel = rel;
            }
            let aft = ok(&t);
            n += s.tokens.len();
            before += b;
            after += aft;
            diffs.push(aft as i64 - b as i64);
        }
        let mut x: u64 = 0x9e3779b97f4a7c15;
        let mut worse = 0;
        for _ in 0..1000 {
            let mut sum = 0i64;
            for _ in 0..diffs.len() {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                sum += diffs[(x % diffs.len() as u64) as usize];
            }
            worse += (sum <= 0) as usize;
        }
        println!("{name} {}: LAS {:.2} → {:.2}  paired bootstrap p = {:.3}", path.display(), 100.0 * before as f64 / n as f64, 100.0 * after as f64 / n as f64, worse as f64 / 1000.0);
        for (r, (k, f, b)) in &rules {
            println!("  {:36} applied {k:4}  fixed {f:4}  broke {b:4}", r.name());
        }
    }
    Ok(())
}

/// Apply the matrix-driven label repairs to annotated words in place (the working pipeline: reader trees,
/// event statistics). Returns the number of repaired words.
pub fn repair_words(ws: &mut [en::annotate::Word]) -> usize {
    let mode = std::env::var("WORLD_REPAIR").unwrap_or_else(|_| "induced".into());
    let domain = std::env::var("WORLD_DOMAIN").unwrap_or_else(|_| "real".into());
    repair_words_with(ws, &mode, &domain)
}

/// The repair pipeline: `hand` — the hand-written repairs; `induced` — archaic adverbs (lexical) then the rules
/// the SLM induced (`world induce`; the tale domain uses its domain rules too); `both` — hand, then induced.
pub fn repair_words_with(ws: &mut [en::annotate::Word], mode: &str, domain: &str) -> usize {
    let m = Matrix::global();
    let mut n = 0;
    if mode == "induced" || mode == "both" {
        if mode == "induced" {
            n += hand_pass(&m, ws, true);
        } else {
            n += hand_pass(&m, ws, false);
        }
        return n + crate::induce::apply_words(&m, ws, &crate::induce::shipped(domain));
    }
    n + hand_pass(&m, ws, false)
}

/// The hand-written repairs (two passes); `lexical_only` keeps only the archaic-adverb repair.
fn hand_pass(m: &Matrix, ws: &mut [en::annotate::Word], lexical_only: bool) -> usize {
    hand_pass_without(m, ws, lexical_only, &[])
}

/// The hand-written repairs without some kinds (for hiding rules in `world rediscover`).
pub fn hand_pass_without(m: &Matrix, ws: &mut [en::annotate::Word], lexical_only: bool, without: &[Repair]) -> usize {
    let m = m;
    let mut n = 0;
    // lexical repairs first (archaic adverbs), then the role repairs on the corrected tree
    for _ in 0..2 {
        let nodes: Vec<Node> = ws.iter().map(|w| Node { form: w.form.clone(), lemma: w.lemma.to_lowercase(), upos: Some(w.upos), head: w.head, rel: w.rel }).collect();
        let tags: Vec<en::gram::Tag> = ws.iter().map(|w| w.tag).collect();
        let mut rs = repairs(m, &nodes, &tags);
        if lexical_only {
            rs.retain(|x| x.2 == Repair::ArchaicAdverb);
        }
        rs.retain(|x| !without.contains(&x.2));
        for (i, rel, r, _) in &rs {
            ws[*i].rel = *rel;
            if *r == Repair::ArchaicAdverb {
                ws[*i].upos = en::gram::UPos::ADV;
                ws[*i].tag = en::gram::Tag::RB;
            }
        }
        n += rs.len();
        if rs.is_empty() {
            break;
        }
    }
    n
}

/// `world repair-eval <gold.conllu>... [--domain real|tale]`: LAS of the parser with no repairs, hand repairs,
/// induced rules and both, on gold tokens; paired bootstrap of each against no repairs.
pub fn repair_eval(paths: &[std::path::PathBuf], domain: &str) -> Result<()> {
    let a = crate::tree::annotator()?;
    for p in paths {
        let gold = en::conllu::read(p)?;
        let mut rows: Vec<(&str, usize, Vec<i64>)> = Vec::new();
        let mut n = 0usize;
        let mut base: Vec<usize> = Vec::new();
        for mode in ["none", "hand", "induced", "both"] {
            let mut correct = 0usize;
            let mut per = Vec::new();
            for s in &gold {
                let forms: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
                if forms.is_empty() {
                    continue;
                }
                let mut ws = a.annotate(&forms);
                if mode != "none" {
                    repair_words_with(&mut ws, mode, domain);
                }
                let c = ws.iter().zip(&s.tokens).filter(|(w, g)| w.head == g.head && w.rel == g.rel).count();
                correct += c;
                per.push(c);
                if mode == "none" {
                    n += s.tokens.len();
                }
            }
            if mode == "none" {
                base = per.clone();
            }
            let d: Vec<i64> = per.iter().zip(&base).map(|(x, y)| *x as i64 - *y as i64).collect();
            rows.push((mode, correct, d));
        }
        println!("{} (domain {domain})", p.display());
        for (mode, c, d) in &rows {
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
            println!("  {mode:8} LAS {:.2}{}", 100.0 * *c as f64 / n as f64, if *mode == "none" { String::new() } else { format!("  p(≤ none) = {:.3}", worse as f64 / 1000.0) });
        }
    }
    Ok(())
}
