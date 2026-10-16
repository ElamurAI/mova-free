//! Question reader v2: an answer from the tale world state via the single fact index (`index`), question parsing
//! with a UD tree (`qframe`) and a span of the anchor sentence (`span`); «state + sentence» merging by a transparent rule.
//!
//! 1. **Parsing** — question type, predicate, participants, subordinate clause («what happened *when* …»: first the fact
//!    of the subordinate clause — the time anchor, then the answer about the main clause).
//! 2. **Search** — index facts by lemmas with field weights; the question type decides which field to look at.
//! 3. **State answer** — the fact field in the tale's words (why, how, means, feeling, goal, intention, belief,
//!    participant, place, number, name) and the fact's anchor sentence; for explicit questions also a span of that sentence,
//!    when almost all of the question is in it (explicit by our own measure, not by the dataset label).
//! 4. **Merging** — state when confident (fact score ≥ threshold and the field is present); otherwise the baseline sentence. The log
//!    records why one of the two was chosen.

use std::collections::HashSet;
use std::fmt::Write as _;
use std::path::Path;

use anyhow::Result;

use crate::ans::{baseline, lemmas, name_of, render, resolve_np, rouge_q};
use crate::ftqa::Qa;
use crate::index::{Fact, Index, Kind, Query, Scored, Slot, place_str, stop};
use crate::types::{Id, Reg};
use crate::qframe::{self, QFrame, QKind};
use crate::span::{self, Want};
use crate::tree::{Tree, annotator};
use crate::world::{Text, World};

/// Lemmas of the question's content words: main clause, subject, subordinate clause.
pub fn qlemmas(fr: &QFrame) -> HashSet<String> {
    let mut out = HashSet::new();
    let words = fr.main.content.iter().chain(fr.main.subj.iter()).chain(fr.sub.iter().flat_map(|c| c.content.iter().chain(c.subj.iter())));
    for w in words {
        if stop(w) || w.len() < 2 {
            continue;
        }
        out.extend(lemmas(w));
    }
    out
}

/// Share of the question's content words present in the sentence (lemmas): «explicit by our measure».
pub fn cover(fr: &QFrame, t: &Tree) -> f64 {
    let sl: HashSet<String> = (0..t.len()).filter(|i| t.content(*i)).flat_map(|i| lemmas(t.form(i)).into_iter().chain(std::iter::once(t.lemma(i).to_string()))).collect();
    let words: Vec<&String> = fr.main.content.iter().chain(fr.main.subj.iter()).chain(fr.sub.iter().flat_map(|c| c.content.iter().chain(c.subj.iter()))).filter(|w| !stop(w) && w.len() > 1).collect();
    if words.is_empty() {
        return 0.0;
    }
    let hit = words.iter().filter(|w| lemmas(w).iter().any(|l| sl.contains(l))).count();
    hit as f64 / words.len() as f64
}

/// Trees of the tale's sentences (once per tale).
pub fn trees(text: &Text) -> Result<Vec<Tree>> {
    let a = annotator()?;
    Ok(text.sents.iter().map(|(_, s)| Tree::parse(a, s)).collect())
}

/// Span development on the validation split (without the world): for each question — the oracle sentence (highest
/// ROUGE-L against the reference among the section sentences) and the baseline sentence; a span by question type; ROUGE-L of the span
/// versus the whole sentence. `show` — the question type to print examples for.
pub fn span_dev(vdir: &Path, show: Option<&str>) -> Result<String> {
    let mut names: Vec<String> = std::fs::read_dir(vdir.join("stories"))?.filter_map(|e| e.ok()?.path().file_stem()?.to_str().map(str::to_string)).collect();
    names.sort();
    // (type, explicit) → [n, R of oracle sentence, R of oracle span (where present), has span, R of baseline sentence, R of baseline span, R of fallback]
    let mut agg: std::collections::BTreeMap<(String, bool), [f64; 8]> = Default::default();
    let mut ex = String::new();
    for (i, name) in names.iter().enumerate() {
        let text = crate::ftqa::read_story(&vdir.join("stories").join(format!("{name}.tsv")), i as i64 + 1)?;
        let qs = crate::ftqa::read_qa(&vdir.join("questions").join(format!("{name}.tsv")))?;
        let tr = trees(&text)?;
        let w = World::new(text.clone());
        for q in &qs {
            let fr = qframe::parse(&q.question);
            let want = span::want_of(&fr);
            let qlem = qlemmas(&fr);
            let preds = fr.main.preds();
            let sub_preds = fr.sub.as_ref().map(|c| c.preds()).unwrap_or_default();
            let secs: Vec<usize> = (0..text.sents.len()).filter(|k| q.secs.contains(&text.sents[*k].0)).collect();
            let Some(&o) = secs.iter().max_by(|a, b| rouge_q(&text.sents[**a].1, q).partial_cmp(&rouge_q(&text.sents[**b].1, q)).unwrap().then(b.cmp(a))) else { continue };
            let (bn, bs) = baseline(&w, q);
            let r_o = rouge_q(&text.sents[o].1, q);
            let sp_o = span::extract(&tr[o], want, &preds, &sub_preds, &fr, &qlem);
            let fb_o = span::minus_question(&tr[o], &qlem);
            let r_b = rouge_q(&bs, q);
            let sp_b = if bn > 0 { span::extract(&tr[bn as usize - 1], want, &preds, &sub_preds, &fr, &qlem) } else { None };
            let key = (fr.kind.name().to_string(), q.ex == "explicit");
            let a = agg.entry(key).or_default();
            a[0] += 1.0;
            a[1] += r_o;
            a[2] += sp_o.as_ref().map(|s| rouge_q(&s.0, q)).unwrap_or(r_o);
            a[3] += sp_o.is_some() as u8 as f64;
            a[4] += r_b;
            a[5] += sp_b.as_ref().map(|s| rouge_q(&s.0, q)).unwrap_or(r_b);
            a[6] += fb_o.as_ref().map(|s| rouge_q(s, q)).unwrap_or(r_o);
            a[7] += sp_o.as_ref().map(|s| rouge_q(&s.0, q)).or_else(|| fb_o.as_ref().map(|s| rouge_q(s, q))).unwrap_or(r_o);
            if show == Some(fr.kind.name()) && q.ex == "explicit" {
                let _ = writeln!(
                    ex,
                    "Q: {}\n  G: {}\n  S({:.2}): {}\n  span[{}] ({:.2}): {}\n  fb: {}\n  frame: {}",
                    q.question,
                    q.a1,
                    r_o,
                    text.sents[o].1,
                    sp_o.as_ref().map(|s| s.1.as_str()).unwrap_or("-"),
                    sp_o.as_ref().map(|s| rouge_q(&s.0, q)).unwrap_or(0.0),
                    sp_o.as_ref().map(|s| s.0.as_str()).unwrap_or("—"),
                    fb_o.as_deref().unwrap_or("—"),
                    qframe::show(&fr)
                );
            }
        }
    }
    let mut o = String::from("| type | explicit | N | R oracle sentence | R span (oracle) | has span | R fallback | R span→fallback | R baseline | R baseline span |\n|---|---|---|---|---|---|---|---|---|---|\n");
    let mut tot = [0.0; 8];
    for ((k, e), a) in &agg {
        let n = a[0];
        let _ = writeln!(o, "| {k} | {} | {n} | {:.3} | {:.3} | {:.0}% | {:.3} | {:.3} | {:.3} | {:.3} |", if *e { "yes" } else { "no" }, a[1] / n, a[2] / n, 100.0 * a[3] / n, a[6] / n, a[7] / n, a[4] / n, a[5] / n);
        for i in 0..8 {
            tot[i] += a[i];
        }
    }
    let n = tot[0];
    let _ = writeln!(o, "| **all** | | {n} | {:.3} | {:.3} | {:.0}% | {:.3} | {:.3} | {:.3} | {:.3} |", tot[1] / n, tot[2] / n, 100.0 * tot[3] / n, tot[6] / n, tot[7] / n, tot[4] / n, tot[5] / n);
    o.push_str(&ex);
    Ok(o)
}


// ── reader ────────────────────────────────────────────────────────────────────────────────────────

/// State confidence threshold (fact score): predicate +3 and subject in a role +3 or section +3.
pub const CONF: f64 = 6.0;
/// Share of question words in the anchor sentence from which the question counts as explicit (answer — a span of the sentence).
pub const COVER: f64 = 0.5;


/// Where the final answer comes from.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RSrc {
    /// state: fact field
    Slot,
    /// state: span of the fact's anchor sentence
    Span,
    /// baseline sentence (state not confident or silent)
    Sentence,
}

impl RSrc {
    pub fn name(self) -> &'static str {
        match self {
            RSrc::Slot => "state:field",
            RSrc::Span => "state:span",
            RSrc::Sentence => "sentence",
        }
    }
}

/// Reader answer v2.
#[derive(Clone, Debug)]
pub struct RAnswer {
    pub text: String,
    pub src: RSrc,
    /// state answer (field or span) when the state is confident; otherwise `None` («not in state»)
    pub state: Option<String>,
    pub conf: f64,
    /// state fact(s)
    pub fact: String,
    /// why the state or the sentence was chosen
    pub reason: String,
    pub frame: String,
    pub sent: Option<u16>,
}

/// State answer candidate.
#[derive(Clone, Debug)]
struct Cand {
    slot: Option<String>,
    sent: u16,
    conf: f64,
    want: Want,
    preds: Vec<String>,
    fact: String,
}

/// Tale for the reader: fact world (index), own text (sentences, trees, baseline).
pub struct Tale<'a> {
    /// world the index comes from (in the «shuffle» control — another tale)
    pub fw: &'a World,
    pub ix: &'a Index,
    /// own tale: text for the baseline and spans
    pub own: &'a World,
    pub tr: &'a [Tree],
}

fn sec_span(w: &World, secs: &[u16]) -> (u16, u16) {
    let mut lo = u16::MAX;
    let mut hi = 0;
    for (i, (p, _)) in w.text.sents.iter().enumerate() {
        if secs.contains(p) {
            lo = lo.min(i as u16 + 1);
            hi = hi.max(i as u16 + 1);
        }
    }
    if hi == 0 { (1, w.text.sents.len() as u16) } else { (lo, hi) }
}

const PRON: &[&str] = &["he", "she", "they", "him", "her", "them", "i", "you", "we", "it", "his", "their"];

/// Word group → world entities (`ans::resolve_np`: head word, modifiers); a pronoun — the most frequent
/// character of the sections.
fn resolve(w: &World, words: &[String], regs: &[Reg], lo: u16, hi: u16) -> Vec<Id> {
    let norm: Vec<String> = words.iter().map(|x| if x == "'s" || x == "'" { "s".to_string() } else { x.clone() }).collect();
    let r = resolve_np(w, &norm, regs);
    if r.len() > 1 {
        // several equally good (three sons): group modifiers against state features (age, traits, status,
        // sex, number) — «the youngest son» → age=young
        let mods: Vec<Vec<String>> = norm.iter().filter(|x| !stop(x) && x.len() > 1).map(|x| {
            let mut l = lemmas(x);
            for (a, b) in [("youngest", "young"), ("younger", "young"), ("eldest", "old"), ("oldest", "old"), ("elder", "old"), ("older", "old")] {
                if x == a {
                    l.push(b.to_string());
                }
            }
            l
        }).collect();
        let score = |id: &Id| -> usize {
            let Some(s) = w.now(*id) else { return 0 };
            let mut ws: HashSet<String> = HashSet::new();
            for f in [crate::types::Field::Age, crate::types::Field::Trait, crate::types::Field::Status, crate::types::Field::Gender, crate::types::Field::Num, crate::types::Field::Name] {
                if let crate::types::Val::Known(v) = s.get(f) {
                    ws.extend(crate::index::lemset(&v.to_string().replace(['«', '»', ','], " ")));
                }
            }
            mods.iter().filter(|m| m.iter().any(|x| ws.contains(x))).count()
        };
        let best = r.iter().map(score).max().unwrap_or(0);
        let top: Vec<Id> = r.iter().copied().filter(|x| score(x) == best).collect();
        if best > 0 && top.len() == 1 {
            return top;
        }
        return r;
    }
    if !r.is_empty() {
        return r;
    }
    let content: Vec<&String> = norm.iter().filter(|x| !stop(x) && x.len() > 1).collect();
    if content.is_empty() && norm.iter().any(|x| PRON.contains(&x.as_str())) && regs.contains(&Reg::C) {
        let mut best: Vec<(usize, Id)> = w.ents(Reg::C).map(|e| (e.mentions().iter().filter(|m| m.sent >= lo && m.sent <= hi).count(), e.id)).collect();
        best.sort_by(|a, b| b.0.cmp(&a.0));
        return best.into_iter().take(1).filter(|x| x.0 > 0).map(|x| x.1).collect();
    }
    Vec::new()
}

fn lem_words(ws: &[String], skip: &[String]) -> Vec<Vec<String>> {
    ws.iter().filter(|w| !stop(w) && w.len() > 1 && !skip.contains(w)).map(|w| lemmas(w)).filter(|l| !l.iter().any(|x| skip.contains(x))).collect()
}

fn fact_verb(f: &Fact) -> Vec<String> {
    match f.kind {
        Kind::Event => f.key.split(['_', '-']).map(str::to_string).collect(),
        Kind::Speech => vec!["say".into(), "tell".into(), "ask".into(), "cry".into(), "answer".into(), "reply".into()],
        _ => Vec::new(),
    }
}

impl Tale<'_> {
    fn fact(&self, n: usize) -> &Fact {
        &self.ix.facts[n]
    }

    fn ev(&self, id: Id) -> Option<&crate::world::Event> {
        self.fw.event(id)
    }

    fn render_fact(&self, f: &Fact) -> String {
        match f.kind {
            Kind::Event | Kind::Speech => self.ev(f.src).map(|e| render(self.fw, e)).unwrap_or_default(),
            Kind::Attr if f.end => format!("{} no longer {}: {}", name_of(self.fw, f.src), f.key, f.text(Slot::Value).unwrap_or("")),
            Kind::Attr if f.key == "feeling" => format!("{} felt {}", name_of(self.fw, f.src), f.text(Slot::Value).unwrap_or("")),
            Kind::Attr | Kind::Have | Kind::Part => format!("{} {} {}", name_of(self.fw, f.src), if f.kind == Kind::Attr { "became" } else { "had" }, f.text(Slot::Value).unwrap_or("")),
            Kind::Plan | Kind::Belief => f.text(Slot::What).unwrap_or("").to_string(),
            _ => f.text(Slot::Value).unwrap_or("").to_string(),
        }
    }

    /// Cause of an event or change: why of the cause event, otherwise the cause event itself in words.
    fn cause_text(&self, f: &Fact) -> Option<String> {
        let c = f.cause.and_then(|c| self.ix.event(c))?;
        Some(c.text(Slot::Why).map(str::to_string).unwrap_or_else(|| self.render_fact(c)))
    }

    /// Search confidence: without a predicate match and with fewer than two content words — below the threshold.
    fn sconf(h: &Scored) -> f64 {
        if h.pred || h.words >= 2.0 { h.score } else { h.score.min(CONF - 0.5) }
    }

    fn cand(&self, slot: Option<String>, f: &Fact, conf: f64, want: Want, preds: Vec<String>, note: &str) -> Cand {
        Cand { slot: slot.filter(|x| !x.trim().is_empty()), sent: f.sent, conf, want, preds, fact: format!("{}{}", f.label(), if note.is_empty() { String::new() } else { format!(" ({note})") }) }
    }
}

/// Answer a question: state when confident, otherwise the baseline sentence.
pub fn read(t: &Tale, q: &Qa) -> RAnswer {
    let (conf_min, cover_min) = (CONF, COVER);
    let fr = qframe::parse(&q.question);
    let (base_n, base) = baseline(t.own, q);
    let cand = state_answer(t, q, &fr);
    let qlem = qlemmas(&fr);
    let sub_preds = fr.sub.as_ref().map(|c| c.preds()).unwrap_or_default();
    let frame = qframe::show(&fr);
    let sentence = |reason: String, state: Option<String>, conf: f64, fact: String| RAnswer {
        text: base.clone(),
        src: RSrc::Sentence,
        state,
        conf,
        fact,
        reason,
        frame: frame.clone(),
        sent: (base_n > 0).then_some(base_n),
    };
    let Some(c) = cand else {
        return sentence("sentence: state is silent (no fact for the question)".into(), None, 0.0, String::new());
    };
    let tree = t.tr.get(c.sent as usize - 1).filter(|_| c.sent >= 1);
    let span = tree.and_then(|tr| span::extract(tr, c.want, &c.preds, &sub_preds, &fr, &qlem));
    let cov = tree.map(|tr| cover(&fr, tr)).unwrap_or(0.0);
    if c.conf < conf_min {
        return sentence(format!("sentence: state not confident (score {:.1} < {conf_min})", c.conf), None, c.conf, c.fact);
    }
    let prefer_slot = matches!(c.want, Want::Describe | Want::Num) && c.slot.is_some();
    // «what happened»: the answer is in the effect sentence, not in the question sentence; its clause — the tale's words
    // feeling: the feeling word in the fact's anchor sentence — the tale's word for the same feeling
    let feel_adj = c.want == Want::Feel && span.as_ref().is_some_and(|x| x.1 == "adj");
    let cov = if (matches!(c.want, Want::After | Want::Clause) && matches!(fr.kind, QKind::Happen | QKind::HappenTo)) || feel_adj { cov.max(cover_min) } else { cov };
    if cov >= cover_min
        && !prefer_slot
        && let Some((sp, rule)) = &span
    {
        return RAnswer {
            text: sp.clone(),
            src: RSrc::Span,
            state: Some(sp.clone()),
            conf: c.conf,
            fact: c.fact,
            reason: format!("state: score {:.1} ≥ {conf_min}; question in sentence ^s{} at {:.0}% ≥ {:.0}% → span [{rule}]", c.conf, c.sent, 100.0 * cov, 100.0 * cover_min),
            frame,
            sent: Some(c.sent),
        };
    }
    if let Some(sl) = &c.slot {
        return RAnswer {
            text: sl.clone(),
            src: RSrc::Slot,
            state: Some(sl.clone()),
            conf: c.conf,
            fact: c.fact,
            reason: format!("state: score {:.1} ≥ {conf_min} → fact field (question in sentence ^s{} at {:.0}%)", c.conf, c.sent, 100.0 * cov),
            frame,
            sent: Some(c.sent),
        };
    }
    if let Some((sp, rule)) = &span {
        return RAnswer {
            text: sp.clone(),
            src: RSrc::Span,
            state: Some(sp.clone()),
            conf: c.conf,
            fact: c.fact,
            reason: format!("state: score {:.1} ≥ {conf_min}; no field → span of sentence ^s{} [{rule}]", c.conf, c.sent),
            frame,
            sent: Some(c.sent),
        };
    }
    sentence(format!("sentence: the fact has no field for the question ({})", c.fact), None, c.conf, c.fact)
}

/// State answer: the fact for the question, field, anchor sentence, confidence.
fn state_answer(t: &Tale, q: &Qa, fr: &QFrame) -> Option<Cand> {
    let w = t.fw;
    if t.ix.facts.is_empty() {
        return None;
    }
    let secs = q.secs.clone();
    let (lo, hi) = sec_span(w, &secs);
    let chars = [Reg::C];
    let mut subj = resolve(w, &fr.main.subj, &chars, lo, hi);
    if subj.is_empty() {
        subj = resolve(w, &fr.main.subj, &[Reg::O, Reg::L], lo, hi);
    }
    let mut others: Vec<Id> = resolve(w, &fr.main.obj, &[Reg::C, Reg::O, Reg::L], lo, hi);
    for (_, np) in &fr.main.obl {
        others.extend(resolve(w, np, &[Reg::C, Reg::O, Reg::L], lo, hi));
    }
    others.retain(|x| !subj.contains(x));
    let preds = fr.main.preds();
    let pred_alts: Vec<Vec<String>> = preds.iter().map(|p| lemmas(p)).collect();
    let words = lem_words(&fr.main.content, &preds);
    // time anchor: fact of the subordinate clause
    let target = resolve(w, &fr.target, &[Reg::C, Reg::O, Reg::L], lo, hi);
    let anchor: Option<Scored> = fr.sub.as_ref().and_then(|c| {
        // pronoun of the subordinate clause — first a participant of the main clause (subject, «to X», object)
        let pron = !c.subj.is_empty() && c.subj.iter().all(|x| PRON.contains(&x.as_str()));
        let main_ent: Vec<Id> = subj.iter().chain(target.iter()).chain(others.iter()).copied().filter(|x| x.reg == Reg::C).take(1).collect();
        let who = if pron && !main_ent.is_empty() { main_ent } else { resolve(w, &c.subj, &chars, lo, hi) };
        let pass = c.t.w.iter().any(|x| x.rel == crate::tree::Rel::AuxPass);
        let sp = c.preds();
        let qy = Query {
            preds: sp.iter().map(|p| lemmas(p)).collect(),
            who,
            role: Some(if pass { Slot::Patient } else { Slot::Agent }),
            others: resolve(w, &c.obj, &[Reg::C, Reg::O, Reg::L], lo, hi),
            words: lem_words(&c.content, &sp),
            secs: secs.clone(),
            fut: false,
        };
        let best = t.ix.search(w, &qy, |f| matches!(f.kind, Kind::Event | Kind::Speech | Kind::Attr | Kind::Have | Kind::Plan | Kind::Belief)).into_iter().next()?;
        (best.score >= 4.0 && (best.pred || best.words >= 1.0)).then_some(best)
    });
    let a_fact = anchor.as_ref().map(|a| t.fact(a.n));
    let t_anchor = a_fact.map(|f| f.sent);
    let role = if fr.passive { Slot::Patient } else { Slot::Agent };
    let main_q = |who: Vec<Id>, role: Option<Slot>| Query { preds: pred_alts.clone(), who, role, others: others.clone(), words: words.clone(), secs: secs.clone(), fut: fr.fut };
    let evs = |f: &Fact| matches!(f.kind, Kind::Event | Kind::Speech);
    let a_note = anchor.as_ref().map(|a| format!("anchor {} [{}]", t.fact(a.n).label(), a.why)).unwrap_or_default();
    match fr.kind {
        QKind::Feel => {
            let c = subj.first().copied().filter(|c| c.reg == Reg::C)?;
            let feels: Vec<&Fact> = t.ix.facts.iter().filter(|f| f.kind == Kind::Attr && f.key == "feeling" && !f.end && f.src == c).collect();
            let about = resolve(w, &fr.about, &[Reg::C, Reg::O, Reg::L], lo, hi);
            let pick: Option<(&Fact, f64, &str)> = if let Some(a) = a_fact {
                feels.iter().find(|f| f.cause == Some(a.src)).map(|f| (*f, 9.0, "feeling because of anchor"))
                    .or_else(|| feels.iter().find(|f| f.sent >= a.sent && f.sent <= a.sent + 2).map(|f| (*f, 7.0, "feeling right after anchor")))
                    .or_else(|| feels.iter().filter(|f| f.sent <= a.sent).next_back().map(|f| (*f, 5.0, "feeling at anchor time")))
            } else if !about.is_empty() {
                feels.iter().find(|f| f.cause.and_then(|e| t.ix.event(e)).is_some_and(|e| about.iter().any(|x| e.has_id(*x))) && f.sent + 3 >= lo).map(|f| (*f, 7.0, "feeling about X"))
                    .or_else(|| feels.iter().find(|f| f.sent >= lo && f.sent <= hi).map(|f| (*f, 6.0, "feeling in sections")))
            } else if fr.fut {
                feels.iter().find(|f| f.sent >= lo).map(|f| (*f, 6.0, "feeling further on"))
            } else {
                feels.iter().find(|f| f.sent >= lo && f.sent <= hi).map(|f| (*f, 7.0, "feeling in sections"))
                    .or_else(|| feels.iter().filter(|f| f.sent <= hi).next_back().map(|f| (*f, 4.0, "last feeling before sections")))
            };
            let (f, conf, note) = pick?;
            Some(t.cand(f.text(Slot::Value).map(str::to_string), f, conf, Want::Feel, vec!["feel".into()], &format!("{note}; {a_note}")))
        }
        QKind::Why => {
            // «why was X <feeling>»: cause of the feeling
            let fw: Vec<&str> = fr.main.content.iter().chain(std::iter::once(&fr.main.lemma)).filter_map(|x| crate::types::FEELING.tag(x).ok().map(|t| t.as_str())).collect();
            if let (Some(c), Some(tag)) = (subj.first().copied(), fw.first()) {
                let f = t.ix.facts.iter().filter(|f| f.kind == Kind::Attr && f.key == "feeling" && f.src == c && f.text(Slot::Value).is_some_and(|v| v.contains(tag))).min_by_key(|f| if f.sent >= lo && f.sent <= hi { 0 } else { 1 + f.sent.abs_diff(lo) as u32 });
                if let Some(f) = f
                    && let Some(ct) = t.cause_text(f)
                {
                    return Some(t.cand(Some(ct), f, 8.0, Want::Why, vec![tag.to_string()], "cause of feeling"));
                }
            }
            let hits = t.ix.search(w, &main_q(subj.clone(), Some(role)), |f| matches!(f.kind, Kind::Event | Kind::Speech | Kind::Attr | Kind::Plan | Kind::Belief) && f.key != "at");
            let h = hits.first()?;
            let f = t.fact(h.n);
            let mut p = preds.clone();
            p.extend(fact_verb(f));
            let slot = match f.kind {
                Kind::Event | Kind::Speech => f
                    .text(Slot::Why)
                    .map(str::to_string)
                    .or_else(|| f.text(Slot::How).filter(|x| crate::ans::toks(x).iter().any(|w| matches!(w.as_str(), "because" | "since" | "so" | "as" | "for"))).map(str::to_string))
                    .or_else(|| t.cause_text(f))
                    .or_else(|| t.ix.facts.iter().find(|p| p.kind == Kind::Plan && p.by == Some(f.src)).and_then(|p| p.text(Slot::What).map(str::to_string)))
                    .or_else(|| {
                        let ag = f.who.first().copied()?;
                        t.ix.facts.iter().filter(|g| g.kind == Kind::Attr && g.key == "goal" && g.src == ag && !g.end && g.sent <= f.sent).next_back().and_then(|g| g.text(Slot::Value).map(str::to_string))
                    }),
                Kind::Plan | Kind::Belief => t.cause_text(f).or_else(|| f.text(Slot::What).map(str::to_string)),
                Kind::Attr if f.key == "goal" => f.text(Slot::Value).map(str::to_string),
                _ => t.cause_text(f),
            };
            // «why did X love/marry/choose Y» with no cause in the event — traits of Y
            let slot = slot.or_else(|| {
                if !matches!(fr.main.lemma.as_str(), "love" | "like" | "admire" | "marry" | "choose" | "fall" | "prefer" | "trust") {
                    return None;
                }
                let y = others.iter().copied().find(|x| x.reg == Reg::C)?;
                let tr: Vec<String> = t.ix.facts.iter().filter(|g| g.kind == Kind::Attr && g.key == "trait" && g.src == y).filter_map(|g| g.text(Slot::Value).map(str::to_string)).collect();
                (!tr.is_empty()).then(|| format!("{} was {}", name_of(w, y), tr.join(", ")))
            });
            Some(t.cand(slot, f, Tale::sconf(h), Want::Why, p, &h.why))
        }
        QKind::Happen | QKind::HappenTo => {
            // the subordinate clause is there but its fact was not found — do not guess
            if fr.sub.is_some() && anchor.is_none() {
                return None;
            }
            // «what would happen if …» — conditional: content of a statement or belief with these words
            if fr.mark.as_deref() == Some("if") {
                let mut qy = main_q(Vec::new(), None);
                qy.words = fr.sub.iter().flat_map(|c| lem_words(&c.content, &[])).collect();
                let h = t.ix.search(w, &qy, |f| matches!(f.kind, Kind::Speech | Kind::Belief | Kind::Plan)).into_iter().next()?;
                let f = t.fact(h.n);
                let slot = f.text(Slot::Means).or(f.text(Slot::What)).map(str::to_string);
                return Some(t.cand(slot, f, if h.words >= 2.0 { h.score } else { h.score.min(CONF - 0.5) }, Want::Say, fact_verb(f), &h.why));
            }
            let (base, conf, note): (Option<&Fact>, f64, String) = match (&anchor, a_fact) {
                (Some(a), Some(f)) => (Some(f), a.score, a_note.clone()),
                _ => {
                    // no anchor: an event with the remaining words in the sections
                    let qy = main_q(target.clone(), None);
                    let h = t.ix.search(w, &qy, evs).into_iter().next();
                    match h {
                        Some(h) if !words.is_empty() && h.words >= 1.0 => (Some(t.fact(h.n)), h.score - 1.0, h.why.clone()),
                        _ => (None, 0.0, String::new()),
                    }
                }
            };
            let from = base.map(|b| b.line).unwrap_or(0);
            let base_id = base.map(|b| b.src);
            // effects: cause = base; otherwise the following events (for «to X» — from X)
            let mut next: Vec<&Fact> = Vec::new();
            if let Some(b) = base_id
                && target.is_empty()
            {
                next = t.ix.facts.iter().filter(|f| f.cause == Some(b) && (f.kind == Kind::Event || (f.kind == Kind::Attr && matches!(f.key.as_str(), "feeling" | "status" | "life" | "freedom")))).take(2).collect();
            }
            let qlem = qlemmas(fr);
            // an effect that repeats the question (feeling «surprised» from the subordinate clause itself) is not an answer
            let echo = |f: &Fact| -> bool {
                let v: Vec<String> = match f.kind {
                    Kind::Attr => f.text(Slot::Value).map(|x| crate::ans::toks(x)).unwrap_or_default(),
                    _ => fact_verb(f),
                };
                !v.is_empty() && v.iter().all(|x| stop(x) || lemmas(x).iter().any(|l| qlem.contains(l)))
            };
            next.retain(|f| !echo(f));
            if next.is_empty() {
                let start = if base.is_some() { from + 1 } else { t.ix.facts.iter().find(|f| f.sent >= lo).map(|f| f.line).unwrap_or(0) };
                next = t
                    .ix
                    .facts
                    .iter()
                    .filter(|f| f.line >= start && !f.neg && Some(f.src) != base_id)
                    .filter(|f| f.kind == Kind::Event || (!target.is_empty() && f.kind == Kind::Attr && matches!(f.key.as_str(), "feeling" | "status" | "life" | "freedom" | "goal")))
                    .filter(|f| target.is_empty() || target.iter().any(|x| f.has_id(*x)))
                    .filter(|f| !echo(f))
                    .take(2)
                    .collect();
            }
            let first = *next.first()?;
            let slot = next.iter().map(|f| t.render_fact(f)).collect::<Vec<_>>().join(", and ");
            let same = base.is_some_and(|b| b.sent == first.sent);
            let (want, preds) = if same { (Want::After, fact_verb(base.unwrap())) } else { (Want::Clause, fact_verb(first)) };
            let mut c = t.cand(Some(slot), first, conf, want, preds, &note);
            if same {
                c.sent = first.sent;
            }
            Some(c)
        }
        QKind::Do => {
            let who = subj.clone();
            if who.is_empty() {
                return None;
            }
            let from = t_anchor.unwrap_or(lo);
            let rest = lem_words(&fr.main.content, &["do".to_string()]);
            let mut best: Option<(f64, &Fact)> = None;
            for f in t.ix.facts.iter().filter(|f| f.kind == Kind::Event && !f.neg && f.who.iter().any(|x| who.contains(x))) {
                if f.sent < from || a_fact.is_some_and(|a| a.src == f.src) {
                    continue;
                }
                if !(fr.fut || a_fact.is_some()) && f.sent > hi {
                    continue;
                }
                let mut s = 0.0;
                for alts in &rest {
                    if f.fields.iter().any(|fl| alts.iter().any(|a| fl.lem.contains(a))) {
                        s += 1.0;
                    }
                }
                s -= f.sent.abs_diff(from) as f64 * 0.05;
                if best.is_none_or(|b| s > b.0) {
                    best = Some((s, f));
                }
            }
            let (_, f) = best?;
            let conf = if let Some(a) = &anchor { a.score.min(9.0) } else if f.sent >= lo && f.sent <= hi { 6.5 } else { 4.0 };
            Some(t.cand(Some(t.render_fact(f)), f, conf, Want::Do, fact_verb(f), &a_note))
        }
        QKind::Say | QKind::Think | QKind::Want => {
            let who = subj.clone();
            let ks: &[Kind] = match fr.kind {
                QKind::Say => &[Kind::Speech, Kind::Plan],
                QKind::Think => &[Kind::Belief, Kind::Speech],
                _ => &[Kind::Plan, Kind::Attr],
            };
            let mut qy = main_q(who, Some(Slot::Agent));
            qy.words.extend(fr.sub.iter().flat_map(|c| lem_words(&c.content, &[])));
            let hits = t.ix.search(w, &qy, |f| ks.contains(&f.kind) && (f.kind != Kind::Attr || f.key == "goal") && t_anchor.is_none_or(|a| f.sent + 1 >= a));
            let h = hits.first()?;
            let f = t.fact(h.n);
            let slot = f.text(Slot::Means).or(f.text(Slot::What)).or(f.text(Slot::Value)).map(str::to_string);
            // «what did X promise»: the promise text from this statement
            let slot = if fr.main.lemma == "promise" && f.kind == Kind::Speech {
                t.ix.facts.iter().find(|p| p.kind == Kind::Plan && p.cause == Some(f.src)).and_then(|p| p.text(Slot::What).map(str::to_string)).or(slot)
            } else {
                slot
            };
            let want = if fr.kind == QKind::Want { Want::Obj } else { Want::Say };
            let conf = h.score + if h.pred { 0.0 } else { 1.0 };
            Some(t.cand(slot, f, conf, want, preds.clone(), &h.why))
        }
        QKind::Obj | QKind::WhoObj => {
            // «what did X have»: items and parts
            if matches!(fr.main.lemma.as_str(), "have" | "own" | "carry" | "hold" | "possess" | "keep")
                && let Some(c) = subj.first().copied()
            {
                let at = t_anchor.unwrap_or(hi);
                let mut hs: Vec<&Fact> = t.ix.facts.iter().filter(|f| matches!(f.kind, Kind::Have | Kind::Part) && f.src == c && f.sent <= at.max(lo)).collect();
                let wl = &words;
                hs.sort_by_key(|f| std::cmp::Reverse(wl.iter().filter(|a| f.fields.iter().any(|fl| a.iter().any(|x| fl.lem.contains(x)))).count()));
                if let Some(f) = hs.first() {
                    return Some(t.cand(f.text(Slot::Value).map(str::to_string), f, 7.0, Want::Obj, preds.clone(), "items and parts"));
                }
            }
            // «who did X think was …» — X's belief
            if qframe::THINK_L.contains(&fr.main.lemma.as_str()) && fr.main.lemma != "see" && !subj.is_empty() {
                let hits = t.ix.search(w, &main_q(subj.clone(), Some(Slot::Agent)), |f| f.kind == Kind::Belief);
                if let Some(h) = hits.first() {
                    let f = t.fact(h.n);
                    return Some(t.cand(f.text(Slot::What).map(str::to_string), f, h.score + 1.0, Want::Obj, preds.clone(), &h.why));
                }
            }
            let hits = t.ix.search(w, &main_q(subj.clone(), Some(role)), |f| evs(f) || f.kind == Kind::Plan);
            let hits: Vec<&Scored> = hits.iter().filter(|h| t_anchor.is_none_or(|a| t.fact(h.n).sent + 3 >= a)).collect();
            let h = *hits.first()?;
            let f = t.fact(h.n);
            let names = |s: Slot| -> Option<String> {
                let v: Vec<String> = f.ids(s).into_iter().filter(|x| !subj.contains(x)).map(|x| if x.reg == Reg::L { place_str(w, x) } else { name_of(w, x) }).collect();
                (!v.is_empty()).then(|| v.join(" and "))
            };
            let slot = match (fr.kind, fr.prep.as_deref()) {
                (QKind::WhoObj, Some("to" | "for")) => names(Slot::To).or_else(|| names(Slot::Patient)),
                (QKind::WhoObj, Some("with")) => names(Slot::Patient).or_else(|| names(Slot::To)).or_else(|| names(Slot::Agent)),
                (QKind::WhoObj, _) => names(Slot::Patient).or_else(|| names(Slot::To)),
                (_, Some("into" | "in" | "at" | "to" | "on" | "from")) => names(Slot::To).or_else(|| names(Slot::At)).or_else(|| names(Slot::Patient)),
                (_, Some("with")) => names(Slot::Instr).or_else(|| names(Slot::Patient)),
                _ if fr.main.lemma == "use" => names(Slot::Instr).or_else(|| names(Slot::Patient)),
                _ => names(Slot::Patient).or_else(|| names(Slot::Instr)).or_else(|| f.text(Slot::Means).map(str::to_string)).or_else(|| f.text(Slot::What).map(str::to_string)).or_else(|| f.text(Slot::How).map(str::to_string)),
            };
            let mut p = preds.clone();
            p.extend(fact_verb(f));
            Some(t.cand(slot, f, Tale::sconf(h), Want::Obj, p, &h.why))
        }
        QKind::WhoSubj | QKind::WhatSubj => {
            let hits = t.ix.search(w, &main_q(Vec::new(), None), |f| evs(f) || matches!(f.kind, Kind::Attr | Kind::Plan));
            let hits: Vec<&Scored> = hits.iter().filter(|h| t_anchor.is_none_or(|a| t.fact(h.n).sent + 3 >= a)).collect();
            let h = *hits.first()?;
            let f = t.fact(h.n);
            let slot = match f.kind {
                Kind::Event | Kind::Speech => {
                    // «who was blamed» — passive: the one it happened to (patient), not the agent
                    let ids: Vec<Id> = if fr.passive { f.ids(Slot::Patient).into_iter().chain(f.ids(Slot::To)).collect() } else { f.ids(Slot::Agent).into_iter().chain(f.who.iter().copied()).collect() };
                    let v: Vec<String> = ids.into_iter().filter(|x| !others.contains(x)).map(|x| name_of(w, x)).collect::<Vec<_>>();
                    let mut v2: Vec<String> = Vec::new();
                    for x in v {
                        if !v2.contains(&x) {
                            v2.push(x);
                        }
                    }
                    if v2.is_empty() && fr.kind == QKind::WhatSubj { t.cause_text(f) } else { (!v2.is_empty()).then(|| v2.join(" and ")) }
                }
                Kind::Attr | Kind::Plan => f.who.first().map(|x| name_of(w, *x)),
                _ => None,
            };
            let mut p = preds.clone();
            p.extend(fact_verb(f));
            Some(t.cand(slot, f, Tale::sconf(h), Want::Subj, p, &h.why))
        }
        QKind::WhoIs | QKind::Describe => {
            // the question's entity: subject (or description in the predicate)
            let mut target = subj.clone();
            if target.is_empty() {
                target = resolve(w, &fr.main.content, &[Reg::C, Reg::O, Reg::L], lo, hi);
            }
            if fr.kind == QKind::WhoIs {
                // «who was the youngest son» → hero's name; description without a name — traits, status
                if let Some(c) = target.first().copied().filter(|c| c.reg == Reg::C) {
                    let f = t.ix.facts.iter().find(|f| f.kind == Kind::Char && f.src == c)?;
                    let name = name_of(w, c);
                    let desc: Vec<String> = fr.main.subj.iter().chain(fr.main.content.iter()).cloned().collect();
                    let dl: HashSet<String> = desc.iter().flat_map(|x| lemmas(x)).collect();
                    let echo = crate::ans::toks(&name).iter().filter(|x| !stop(x)).all(|x| dl.contains(x));
                    let conf = if target.len() == 1 && !echo { 7.0 } else { 4.0 };
                    return Some(t.cand(Some(name), f, conf, Want::Subj, preds.clone(), "hero's name"));
                }
                let qy = main_q(Vec::new(), None);
                let h = t.ix.search(w, &qy, |f| f.kind == Kind::Attr && matches!(f.key.as_str(), "trait" | "status" | "age") || f.kind == Kind::Char).into_iter().next()?;
                let f = t.fact(h.n);
                return Some(t.cand(Some(name_of(w, f.src)), f, h.score, Want::Subj, preds.clone(), &h.why));
            }
            let c = target.first().copied()?;
            let mut parts: Vec<String> = Vec::new();
            let mut last: Option<&Fact> = None;
            for f in t.ix.facts.iter().filter(|f| f.src == c && ((f.kind == Kind::Attr && matches!(f.key.as_str(), "trait" | "status" | "age")) || f.kind == Kind::Part) && !f.end && f.sent <= hi.max(lo)) {
                if let Some(v) = f.text(Slot::Value)
                    && !parts.contains(&v.to_string())
                {
                    parts.push(v.to_string());
                    last = Some(f);
                }
            }
            let f = last.or_else(|| t.ix.facts.iter().find(|f| f.src == c && matches!(f.kind, Kind::Char | Kind::Place)))?;
            let slot = if parts.is_empty() { None } else { Some(parts.join(", ")) };
            Some(t.cand(slot, f, 6.0, Want::Describe, vec![fr.main.lemma.clone()], "traits, status, parts"))
        }
        QKind::Where => {
            if !matches!(fr.main.lemma.as_str(), "be" | "live" | "stay" | "dwell") {
                let hits = t.ix.search(w, &main_q(subj.clone(), Some(role)), evs);
                if let Some(h) = hits.first()
                    && h.pred
                {
                    let f = t.fact(h.n);
                    let place = f.ids(Slot::At).first().map(|l| place_str(w, *l)).or_else(|| f.ids(Slot::To).into_iter().find(|x| x.reg == Reg::L).map(|l| place_str(w, l)));
                    let mut p = preds.clone();
                    p.extend(fact_verb(f));
                    if place.is_some() {
                        return Some(t.cand(place, f, h.score, Want::Where, p, &h.why));
                    }
                }
            }
            let c = subj.first().copied()?;
            if c.reg == Reg::L {
                let f = t.ix.facts.iter().find(|f| f.kind == Kind::Place && f.src == c)?;
                return Some(t.cand(Some(place_str(w, c)), f, 6.0, Want::Where, vec![fr.main.lemma.clone(), crate::ans::toks(&name_of(w, c)).last().cloned().unwrap_or_default()], "place name and ancestors"));
            }
            let at = t_anchor.unwrap_or(lo);
            let locs: Vec<&Fact> = t.ix.facts.iter().filter(|f| f.kind == Kind::Attr && f.key == "at" && f.src == c && !f.end).collect();
            let f = locs.iter().find(|f| f.sent >= at && f.sent <= hi.max(at + 3)).or_else(|| locs.iter().filter(|f| f.sent <= at).next_back()).or(locs.first())?;
            let mut p = preds.clone();
            p.extend(["live".into(), "go".into(), "come".into(), "stay".into()]);
            Some(t.cand(f.text(Slot::Value).map(str::to_string), f, 6.0, Want::Where, p, "hero's place"))
        }
        QKind::When | QKind::How | QKind::Generic => {
            let hits = t.ix.search(w, &main_q(subj.clone(), Some(role)), evs);
            let h = hits.first()?;
            let f = t.fact(h.n);
            let slot = match fr.kind {
                QKind::When => f.text(Slot::When).map(str::to_string).or_else(|| w.cursor().iter().rev().find(|c| c.pos.sent <= f.sent).and_then(|c| c.time.clone()).map(|x| x.replace('_', " "))).or_else(|| f.cause.and_then(|c| t.ix.event(c)).map(|c| format!("after {}", t.render_fact(c)))),
                QKind::How => f.text(Slot::How).map(str::to_string).or_else(|| f.ids(Slot::Instr).first().map(|i| format!("with {}", name_of(w, *i)))).or_else(|| f.cause.and_then(|c| t.ix.event(c)).map(|c| t.render_fact(c))),
                _ => Some(t.render_fact(f)),
            };
            let want = match fr.kind {
                QKind::When => Want::When,
                QKind::How => Want::How,
                _ => Want::Clause,
            };
            let mut p = preds.clone();
            p.extend(fact_verb(f));
            Some(t.cand(slot, f, Tale::sconf(h), want, p, &h.why))
        }
        QKind::Num => {
            let np: Vec<String> = if fr.whnp.is_empty() { fr.main.subj.iter().chain(fr.main.content.iter()).cloned().collect() } else { fr.whnp.clone() };
            let ids = resolve(w, &np, &[Reg::O, Reg::C], lo, hi);
            for id in ids {
                if let Some(f) = t.ix.facts.iter().find(|f| f.kind == Kind::Attr && f.key == "num" && f.src == id) {
                    return Some(t.cand(f.text(Slot::Value).map(str::to_string), f, 8.0, Want::Num, vec![fr.main.lemma.clone()], "number"));
                }
            }
            None
        }
    }
}

/// Answers of a tale; `fw`/`ix` — world and fact index (in the control — another tale's), `own` — own tale.
pub fn read_all(fw: &World, ix: &Index, own: &World, tr: &[Tree], qs: &[Qa]) -> Vec<RAnswer> {
    let t = Tale { fw, ix, own, tr };
    qs.iter().map(|q| read(&t, q)).collect()
}
