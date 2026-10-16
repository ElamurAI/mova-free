//! Sense check of dependency trees — a cheap parse-error detector from level-1 common sense only (crate `global`).
//! A tree that makes "the cheese" the eater of "the mouse" implies something absurd, so it is probably a parse error.
//! Knowledge: categories from `global/seeds/wikidata/core.md` and `things.md`, selectional lists from
//! `global/seeds/shortcuts/selection.md` (which verbs need an animate agent, an edible or an animate undergoer).
//! Precision first: a noun is judged only if its categories agree; unknown nouns, pronouns and names never flag.
//!
//!   world sense-check <in.conllu> [<gold.conllu>] [--show N]
//!
//! With gold: a flag is right if a flagged noun's head or relation differs from gold (strict), or the verb's (lenient);
//! recall over all subject/object arc errors (nsubj, nsubj:pass, obj, obl:agent in the parse or in gold).

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Result, bail};
use en::conllu::Sentence;
use en::gram::{Rel, UPos};

const SEED: &str = "global/seeds/shortcuts/selection.md";

/// One word of a tree (0-based index; `head` is 1-based, 0 = root, as in CoNLL-U).
#[derive(Clone, Debug)]
pub struct Node {
    pub form: String,
    pub lemma: String,
    pub upos: Option<UPos>,
    pub head: usize,
    pub rel: Rel,
}

pub fn nodes(s: &Sentence) -> Vec<Node> {
    s.tokens
        .iter()
        .map(|t| {
            let lemma = if t.lemma.is_empty() || t.lemma == "_" { t.form.to_lowercase() } else { t.lemma.to_lowercase() };
            Node { form: t.form.clone(), lemma, upos: t.upos, head: t.head, rel: t.rel }
        })
        .collect()
}

/// Animacy of a noun from level-1 categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anim {
    Animate(&'static str),
    Inanimate(&'static str),
    Unknown,
}

pub fn animacy(lemma: &str) -> Anim {
    let cats = global::categories_of(lemma);
    let in_list = |l: &str, c: &&str| global::selection(l).contains(c);
    let an: Vec<&str> = if global::selection("not_animate").contains(&lemma) { vec![] } else { cats.iter().copied().filter(|c| in_list("animate_cats", c)).collect() };
    let inan: Vec<&str> = if global::selection("not_inanimate").contains(&lemma) { vec![] } else { cats.iter().copied().filter(|c| in_list("inanimate_cats", c)).collect() };
    match (an.first(), inan.first()) {
        (Some(a), None) => Anim::Animate(a),
        (None, Some(i)) => Anim::Inanimate(i),
        _ => Anim::Unknown,
    }
}

fn cat_name(c: &str) -> String {
    c.trim_start_matches("wd_").replace('_', " ")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Role {
    Agent,
    Patient,
}

/// Does the noun break a level-1 preference of the verb in this role? `Some(reason)` if it does.
pub fn violation(verb: &str, role: Role, lemma: &str) -> Option<String> {
    let a = animacy(lemma);
    match (role, a) {
        (Role::Agent, Anim::Inanimate(c)) if global::selection("agent_animate").contains(&verb) => {
            Some(format!("{lemma}: {} — cannot be the agent of {verb} (level 1: {verb} needs an animate agent)", cat_name(c)))
        }
        (Role::Patient, Anim::Inanimate(c)) if global::selection("patient_edible").contains(&verb) && global::selection("non_edible_cats").contains(&c) => {
            Some(format!("{lemma}: {} — cannot be what is eaten (level 1: {verb} needs something edible)", cat_name(c)))
        }
        (Role::Patient, Anim::Animate(c)) if global::selection("patient_not_animate").contains(&verb) => {
            Some(format!("{lemma}: {} — cannot be the object of {verb} (level 1: one does not {verb} a creature; likely an inverted subject: \"{verb} the {lemma}\")", cat_name(c)))
        }
        (Role::Patient, Anim::Inanimate(c)) if global::selection("patient_animate").contains(&verb) => {
            Some(format!("{lemma}: {} — cannot be the one who undergoes {verb} (level 1: {verb} needs a person or creature)", cat_name(c)))
        }
        _ => None,
    }
}

fn fits(verb: &str, role: Role, lemma: &str) -> String {
    let a = animacy(lemma);
    let c = match a {
        Anim::Animate(c) | Anim::Inanimate(c) => cat_name(c),
        Anim::Unknown => "?".into(),
    };
    let atomic = if role == Role::Agent && global::capable(lemma).contains(&verb) { format!(", ATOMIC: {lemma} can {verb}") } else { String::new() };
    match role {
        Role::Agent => format!("{lemma}: {c} — fits as the agent of {verb}{atomic}"),
        Role::Patient => format!("{lemma}: {c} — fits as what undergoes {verb}"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    RoleSwap,
    ImplausibleSubject,
    ImplausibleObject,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::RoleSwap => "role-swap",
            Kind::ImplausibleSubject => "implausible-subject",
            Kind::ImplausibleObject => "implausible-object",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Flag {
    pub kind: Kind,
    /// verb index (0-based)
    pub verb: usize,
    /// flagged nouns (0-based)
    pub words: Vec<usize>,
    pub why: String,
}

fn is_noun(n: &Node) -> bool {
    n.upos == Some(UPos::NOUN)
}

/// Agent and patient nouns of verb `v` (0-based): active nsubj / passive obl:agent, active obj / passive nsubj:pass.
pub fn arguments(t: &[Node], v: usize) -> (Option<usize>, Option<usize>) {
    let kids: Vec<usize> = (0..t.len()).filter(|&i| t[i].head == v + 1).collect();
    let passive = kids.iter().any(|&i| t[i].rel == Rel::AuxPass);
    let first = |pred: &dyn Fn(Rel) -> bool| kids.iter().copied().find(|&i| pred(t[i].rel) && is_noun(&t[i]));
    let agent = if passive { first(&|r| r == Rel::OblAgent) } else { first(&|r| r == Rel::Nsubj) };
    // ccomp is not a patient: a nominal ccomp is a quoted fragment or a copular clause ("said that he was a good man");
    // tried 02.10, the negative control on gold went red (6 false alarms on the silver tales, 2 on EWT train)
    let patient = if passive { first(&|r| r == Rel::NsubjPass || r == Rel::Nsubj) } else { first(&|r| r == Rel::Obj || r == Rel::NsubjPass) };
    (agent, patient)
}

/// Sense check of one tree.
pub fn check(t: &[Node]) -> Vec<Flag> {
    let mut out = Vec::new();
    for (v, vn) in t.iter().enumerate() {
        if vn.upos != Some(UPos::VERB) {
            continue;
        }
        let (agent, patient) = arguments(t, v);
        let verb = vn.lemma.as_str();
        let va = agent.and_then(|a| violation(verb, Role::Agent, &t[a].lemma));
        let vp = patient.and_then(|p| violation(verb, Role::Patient, &t[p].lemma));
        if va.is_none() && vp.is_none() {
            continue;
        }
        // role swap: both nouns known, the current reading breaks a preference, the swapped one breaks none
        if let (Some(a), Some(p)) = (agent, patient) {
            let (la, lp) = (&t[a].lemma, &t[p].lemma);
            let known = animacy(la) != Anim::Unknown && animacy(lp) != Anim::Unknown;
            if known && violation(verb, Role::Agent, lp).is_none() && violation(verb, Role::Patient, la).is_none() {
                let now = [va.clone(), vp.clone()].into_iter().flatten().collect::<Vec<_>>().join("; ");
                out.push(Flag { kind: Kind::RoleSwap, verb: v, words: vec![a, p], why: format!("{now}; swapped: {}; {}", fits(verb, Role::Agent, lp), fits(verb, Role::Patient, la)) });
                continue;
            }
        }
        if let (Some(a), Some(why)) = (agent, va) {
            out.push(Flag { kind: Kind::ImplausibleSubject, verb: v, words: vec![a], why });
        }
        if let (Some(p), Some(why)) = (patient, vp) {
            out.push(Flag { kind: Kind::ImplausibleObject, verb: v, words: vec![p], why });
        }
    }
    out
}

fn sub_obj(r: Rel) -> bool {
    matches!(r, Rel::Nsubj | Rel::NsubjPass | Rel::Obj | Rel::OblAgent)
}

fn show_tree(t: &[Node], f: &Flag) -> String {
    let w = |i: usize| format!("{}/{}<-{}", t[i].form, t[i].rel, if t[i].head == 0 { "ROOT".to_string() } else { t[t[i].head - 1].form.clone() });
    let mut s = vec![format!("verb {}", t[f.verb].form)];
    s.extend(f.words.iter().map(|&i| w(i)));
    s.join(", ")
}

/// `world sense-check`: counts, precision/recall against gold, examples.
pub fn run(input: &Path, gold: Option<&Path>, show: usize) -> Result<()> {
    let sys = en::conllu::read(input)?;
    let gold = match gold {
        Some(g) => {
            let gs = en::conllu::read(g)?;
            // sentences are matched by sent_id (a parse may cover a subset of gold); without ids — by position
            let by_id: BTreeMap<&str, usize> = gs.iter().enumerate().filter(|(_, s)| !s.id.is_empty()).map(|(i, s)| (s.id.as_str(), i)).collect();
            let idx: Vec<Option<usize>> = sys.iter().enumerate().map(|(i, s)| if by_id.is_empty() { (i < gs.len()).then_some(i) } else { by_id.get(s.id.as_str()).copied() }).collect();
            let found = idx.iter().filter(|x| x.is_some()).count();
            if found == 0 {
                bail!("no sentence of the parse is found in gold (by sent_id or position)");
            }
            Some(idx.into_iter().map(|i| i.map(|i| gs[i].clone())).collect::<Vec<_>>())
        }
        None => None,
    };
    let mut by_kind: BTreeMap<&str, (usize, usize, usize)> = BTreeMap::new(); // kind → (flags, strict right, lenient right)
    let (mut errs, mut caught, mut checkable, mut caught_c, mut misaligned) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let mut shown = 0usize;
    let mut verbs_seen = 0usize;
    // diagnostics of the errors the check cannot see: nouns, nouns with known animacy, their parse heads
    let (mut e_noun, mut e_anim, mut e_inan) = (0usize, 0usize, 0usize);
    let mut e_heads: BTreeMap<String, usize> = BTreeMap::new();
    for (si, s) in sys.iter().enumerate() {
        let t = nodes(s);
        verbs_seen += t.iter().filter(|n| n.upos == Some(UPos::VERB)).count();
        let flags = check(&t);
        let g = gold.as_ref().map(|gs| gs[si].as_ref().map(nodes));
        let g = match g {
            Some(None) => {
                misaligned += 1;
                None
            }
            Some(Some(g)) if g.len() != t.len() || g.iter().zip(&t).any(|(a, b)| a.form != b.form) => {
                misaligned += 1;
                None
            }
            Some(Some(g)) => Some(g),
            None => None,
        };
        let wrong = |i: usize| g.as_ref().map(|g| g[i].head != t[i].head || g[i].rel != t[i].rel);
        for f in &flags {
            let e = by_kind.entry(f.kind.name()).or_default();
            e.0 += 1;
            let strict = f.words.iter().any(|&i| wrong(i) == Some(true));
            let lenient = strict || wrong(f.verb) == Some(true) || g.as_ref().is_some_and(|g| g[f.verb].upos != t[f.verb].upos);
            e.1 += strict as usize;
            e.2 += lenient as usize;
            if shown < show {
                shown += 1;
                let verdict = match (&g, strict, lenient) {
                    (None, _, _) => String::new(),
                    (Some(_), true, _) => " [gold: parse error at the flagged noun]".into(),
                    (Some(_), false, true) => " [gold: parse error at the verb]".into(),
                    (Some(_), false, false) => " [gold: parse correct — false alarm]".into(),
                };
                let text = if s.text.is_empty() { t.iter().map(|n| n.form.as_str()).collect::<Vec<_>>().join(" ") } else { s.text.clone() };
                println!("{} {}{verdict}\n  {text}\n  {}\n  why: {}", s.id, f.kind.name(), show_tree(&t, f), f.why);
            }
        }
        if let Some(g) = &g {
            for i in 0..t.len() {
                if !(sub_obj(t[i].rel) || sub_obj(g[i].rel)) || (t[i].head == g[i].head && t[i].rel == g[i].rel) {
                    continue;
                }
                errs += 1;
                if is_noun(&t[i]) {
                    e_noun += 1;
                    match animacy(&t[i].lemma) {
                        Anim::Animate(_) => e_anim += 1,
                        Anim::Inanimate(_) => e_inan += 1,
                        Anim::Unknown => {}
                    }
                    if animacy(&t[i].lemma) != Anim::Unknown && std::env::var("SENSE_ERRORS").is_ok() {
                        let hd = |n: &[Node], i: usize| if n[i].head == 0 { "ROOT".to_string() } else { n[n[i].head - 1].lemma.clone() };
                        println!("ERR {} {:?} {}: parse {}<-{} gold {}<-{}", s.id, animacy(&t[i].lemma), t[i].lemma, t[i].rel, hd(&t, i), g[i].rel, hd(g, i));
                    }
                    if animacy(&t[i].lemma) != Anim::Unknown && t[i].head > 0 {
                        *e_heads.entry(format!("{}<-{}", t[i].lemma, t[t[i].head - 1].lemma)).or_default() += 1;
                    }
                }
                let hit = flags.iter().any(|f| f.words.contains(&i));
                caught += hit as usize;
                // checkable: a noun with known animacy under a verb the seed has preferences for
                let h = t[i].head;
                let verb_known = h > 0 && ["agent_animate", "patient_edible", "patient_animate", "patient_not_animate"].iter().any(|l| global::selection(l).contains(&t[h - 1].lemma.as_str()));
                if is_noun(&t[i]) && animacy(&t[i].lemma) != Anim::Unknown && verb_known {
                    checkable += 1;
                    caught_c += hit as usize;
                }
            }
        }
    }
    let total: usize = by_kind.values().map(|x| x.0).sum();
    println!("sentences {}  verbs {}  flags {total}  (knowledge: {SEED})", sys.len(), verbs_seen);
    for (k, (n, st, le)) in &by_kind {
        if gold.is_some() {
            println!("  {k:20} {n:4}  strict right {st}  lenient right {le}");
        } else {
            println!("  {k:20} {n:4}");
        }
    }
    if gold.is_some() {
        let st: usize = by_kind.values().map(|x| x.1).sum();
        let le: usize = by_kind.values().map(|x| x.2).sum();
        let pct = |a: usize, b: usize| if b == 0 { 0.0 } else { 100.0 * a as f64 / b as f64 };
        println!("precision strict {st}/{total} = {:.1}%  lenient {le}/{total} = {:.1}%", pct(st, total), pct(le, total));
        println!("recall: subject/object arc errors {errs}, flagged {caught} = {:.2}%; checkable (known noun under a seed verb) {checkable}, flagged {caught_c} = {:.1}%", pct(caught, errs), pct(caught_c, checkable));
        println!("  of the errors: nouns {e_noun}, with known animacy {} (animate {e_anim}, inanimate {e_inan})", e_anim + e_inan);
        if show > 0 {
            let mut hs: Vec<(&String, &usize)> = e_heads.iter().collect();
            hs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            println!("  known-animacy error nouns (noun<-parse head): {}", hs.iter().take(30).map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", "));
        }
        if misaligned > 0 {
            println!("misaligned sentences (skipped for scoring): {misaligned}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// (form, lemma, upos, head, rel)
    fn tree(ws: &[(&str, &str, UPos, usize, Rel)]) -> Vec<Node> {
        ws.iter().map(|&(f, l, u, h, r)| Node { form: f.into(), lemma: l.into(), upos: Some(u), head: h, rel: r }).collect()
    }

    #[test]
    fn cheese_eats_mouse_is_flagged() {
        let t = tree(&[("The", "the", UPos::DET, 2, Rel::Det), ("cheese", "cheese", UPos::NOUN, 3, Rel::Nsubj), ("eats", "eat", UPos::VERB, 0, Rel::Root), ("the", "the", UPos::DET, 5, Rel::Det), ("mouse", "mouse", UPos::NOUN, 3, Rel::Obj)]);
        let f = check(&t);
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].kind, Kind::RoleSwap);
        assert_eq!(f[0].words, vec![1, 4]);
        assert!(f[0].why.starts_with("cheese: food — cannot be the agent of eat"), "{}", f[0].why);
    }

    #[test]
    fn mouse_eats_cheese_is_not_flagged() {
        // negative control: the plausible sentence
        let t = tree(&[("The", "the", UPos::DET, 2, Rel::Det), ("mouse", "mouse", UPos::NOUN, 3, Rel::Nsubj), ("eats", "eat", UPos::VERB, 0, Rel::Root), ("the", "the", UPos::DET, 5, Rel::Det), ("cheese", "cheese", UPos::NOUN, 3, Rel::Obj)]);
        assert!(check(&t).is_empty());
        // unknown nouns never flag (precision first)
        let t = tree(&[("Zorbs", "zorb", UPos::NOUN, 2, Rel::Nsubj), ("eat", "eat", UPos::VERB, 0, Rel::Root), ("glim", "glim", UPos::NOUN, 2, Rel::Obj)]);
        assert!(check(&t).is_empty());
        // metonymy exception: "the hospital decided"
        let t = tree(&[("hospital", "hospital", UPos::NOUN, 2, Rel::Nsubj), ("decided", "decide", UPos::VERB, 0, Rel::Root)]);
        assert!(check(&t).is_empty());
    }

    #[test]
    fn passive_and_objects() {
        // "The mouse was eaten by the cheese" — role swap through obl:agent
        let t = tree(&[("mouse", "mouse", UPos::NOUN, 3, Rel::NsubjPass), ("was", "be", UPos::AUX, 3, Rel::AuxPass), ("eaten", "eat", UPos::VERB, 0, Rel::Root), ("by", "by", UPos::ADP, 5, Rel::Case), ("cheese", "cheese", UPos::NOUN, 3, Rel::OblAgent)]);
        let f = check(&t);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].kind, Kind::RoleSwap);
        // "The man ate the table" — implausible object, no swap (a table cannot eat)
        let t = tree(&[("man", "man", UPos::NOUN, 2, Rel::Nsubj), ("ate", "eat", UPos::VERB, 0, Rel::Root), ("table", "table", UPos::NOUN, 2, Rel::Obj)]);
        let f = check(&t);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].kind, Kind::ImplausibleObject);
        // "The man ate a bowl of soup" — a container is not judged
        let t = tree(&[("man", "man", UPos::NOUN, 2, Rel::Nsubj), ("ate", "eat", UPos::VERB, 0, Rel::Root), ("bowl", "bowl", UPos::NOUN, 2, Rel::Obj)]);
        assert!(check(&t).is_empty());
        // "'Run!' cried the hare" with hare as obj — the inverted subject; "the hare cried" — no flag
        let t = tree(&[("cried", "cry", UPos::VERB, 0, Rel::Root), ("the", "the", UPos::DET, 3, Rel::Det), ("hare", "hare", UPos::NOUN, 1, Rel::Obj)]);
        let f = check(&t);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].kind, Kind::ImplausibleObject);
        let t = tree(&[("the", "the", UPos::DET, 2, Rel::Det), ("hare", "hare", UPos::NOUN, 3, Rel::Nsubj), ("cried", "cry", UPos::VERB, 0, Rel::Root)]);
        assert!(check(&t).is_empty());
    }
}
