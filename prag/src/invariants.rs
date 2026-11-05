//! Invariants over the frames of a corpus: rows the reader should never give, whatever the sentence. Each check reads
//! one sentence's tree, its frames and their anchors (`Ctx::anchors` right after `frames_of_tree`), except the VerbNet
//! spread, which needs the whole corpus (`Spread`). The rows they flag are for a teacher to look at; a check that
//! flags a correct frame is a bug of the check (each test has its negative control).
//!
//! - `copula-no-who`: a copula frame (verb «be», a Fact is / is a) with no subject;
//! - `time-in-where`: a word of time (a day, a month, a season, «morning», «year» …) as the place;
//! - `vn-spread`: one verb (lemma + particle) with one syntactic frame given different VerbNet classes;
//! - `unsaid-own`: a role marked «not said here» while the clause has its own dependent for it;
//! - `who-is-what`: the agent and the patient of one row are the same entity (a reflexive aside).
//!
//! Deterministic: ordered maps, ties to the smaller class id.

use std::collections::BTreeMap;

use crate::frames::{Anchor, By, Frame, Kind, Link, Rel, Slot, Tree, UPos, time_word};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Check {
    CopulaNoWho,
    TimeInWhere,
    VnSpread,
    UnsaidOwn,
    WhoIsWhat,
}

impl Check {
    pub const ALL: [Check; 5] = [Check::CopulaNoWho, Check::TimeInWhere, Check::VnSpread, Check::UnsaidOwn, Check::WhoIsWhat];

    pub fn name(self) -> &'static str {
        match self {
            Check::CopulaNoWho => "copula-no-who",
            Check::TimeInWhere => "time-in-where",
            Check::VnSpread => "vn-spread",
            Check::UnsaidOwn => "unsaid-own",
            Check::WhoIsWhat => "who-is-what",
        }
    }
}

/// A flagged row: the check, the frame's number in the sentence (1-based, as the site's tables count), what is wrong.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hit {
    pub check: Check,
    pub row: usize,
    pub detail: String,
}

fn is_copula(f: &Frame) -> bool {
    matches!(f.kind, Kind::Fact(Link::Is) | Kind::Fact(Link::IsA)) || f.kind != Kind::Bare && f.verb == "be"
}

/// The per-sentence checks (all but `vn-spread`). `anchors` are the sentence's, one per frame (fewer when
/// `PRAG_OFF=anchors`: `unsaid-own` then checks nothing).
pub fn sentence(t: &Tree, fs: &[Frame], anchors: &[Anchor]) -> Vec<Hit> {
    let mut out = Vec::new();
    for (i, f) in fs.iter().enumerate() {
        let row = i + 1;
        if is_copula(f) && f.agent.is_none() {
            let what = f.patient.as_ref().map(|e| e.text.clone()).or_else(|| f.attrs.first().cloned()).unwrap_or_default();
            out.push(Hit { check: Check::CopulaNoWho, row, detail: format!("{} with no subject; predicate «{what}»", f.verb) });
        }
        if let Some(p) = f.place.as_ref().filter(|p| time_word(&p.id, p.named)) {
            out.push(Hit { check: Check::TimeInWhere, row, detail: format!("where = «{}» ({}); a time", p.text, p.id) });
        }
        if let (Some(a), Some(p)) = (&f.agent, &f.patient) {
            // a reflexive is the same entity by right; two pronouns left unresolved, or «I», «you», «we», say nothing
            let refl = |x: &str| x.ends_with("self") || x.ends_with("selves");
            let reflexive = p.pron.as_deref().is_some_and(refl) || refl(&p.text.to_lowercase());
            let bare = |e: &crate::frames::Ent| ["i", "you", "we", "me", "us", "one"].contains(&e.id.as_str()) || crate::frames::UNRESOLVED_PUB.contains(&e.id.as_str()) && e.pron.as_deref().is_some_and(|x| x.eq_ignore_ascii_case(&e.text));
            if !a.id.is_empty() && a.id.eq_ignore_ascii_case(&p.id) && !reflexive && !(bare(a) && bare(p)) {
                out.push(Hit { check: Check::WhoIsWhat, row, detail: format!("who «{}» = what «{}» ({})", a.text, p.text, a.id) });
            }
        }
        let Some(pred) = anchors.get(i).and_then(|a| a.pred) else { continue };
        // the clause's own dependents not already said in a role of the frame
        let used: Vec<usize> = anchors[i].roles.iter().map(|(_, k)| *k).collect();
        let own = |want: &dyn Fn(usize) -> bool| t.kids(pred).iter().copied().find(|&k| !used.contains(&k) && want(k));
        for &s in &f.unsaid {
            let dep = match s {
                // (an «obl:agent» is the doer only in a passive: the parser gives it to active «with» phrases too)
                Slot::Agent => own(&|k| t.w[k].rel == Rel::OblAgent && f.tam.passive || t.base(k) == Rel::Nsubj && t.w[k].rel != Rel::NsubjPass),
                Slot::Patient => own(&|k| t.w[k].rel == Rel::NsubjPass || t.base(k) == Rel::Obj && !time_word(t.lemma(k), t.upos(k) == UPos::PROPN)),
                Slot::Recipient => own(&|k| t.base(k) == Rel::Iobj),
                _ => None,
            };
            if let Some(k) = dep {
                let filled = f.slot(s).map(|e| e.text.clone()).unwrap_or_default();
                out.push(Hit { check: Check::UnsaidOwn, row, detail: format!("{s:?} «{filled}» not said here, but the clause has {} «{}»", t.w[k].rel, t.form(k)) });
            }
        }
    }
    out
}

/// The VerbNet classes the corpus gives each verb in each syntactic frame (`How::sig`, the clause as the matcher saw
/// it): verb, frame → class → the rows (sentence id, row) that got it.
#[derive(Default)]
pub struct Spread {
    by: BTreeMap<(String, String), BTreeMap<String, Vec<(String, usize)>>>,
}

impl Spread {
    pub fn add(&mut self, id: &str, fs: &[Frame]) {
        for (i, f) in fs.iter().enumerate() {
            if f.how.by == By::VerbNet && !f.how.sig.is_empty() {
                self.by.entry((f.verb.clone(), f.how.sig.clone())).or_default().entry(f.how.class.clone()).or_default().push((id.to_string(), i + 1));
            }
        }
    }

    /// Verb-and-frame groups, and those with more than one class.
    pub fn counts(&self) -> (usize, usize) {
        (self.by.len(), self.by.values().filter(|c| c.len() > 1).count())
    }

    /// Every row whose class is not its group's most frequent (ties to the smaller class id): (sentence id, row,
    /// detail), in the order of the groups.
    pub fn hits(&self) -> Vec<(String, usize, String)> {
        let mut out = Vec::new();
        for ((verb, sig), cs) in &self.by {
            if cs.len() < 2 {
                continue;
            }
            let top = cs.iter().max_by(|a, b| a.1.len().cmp(&b.1.len()).then(b.0.cmp(a.0))).map(|(c, _)| c.clone()).unwrap_or_default();
            let all = cs.iter().map(|(c, v)| format!("{c}×{}", v.len())).collect::<Vec<_>>().join(", ");
            for (c, rows) in cs.iter().filter(|(c, _)| **c != top) {
                for (id, row) in rows {
                    out.push((id.clone(), *row, format!("{verb} «{sig}»: {c}, most get {top} ({all})")));
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frames::{Ctx, frames_of_tree};
    use en::annotate::Word;
    use en::gram::{Feats, Tag, UPos};

    /// A tree from rows `form lemma UPOS XPOS head rel`.
    fn tree(rows: &[&str]) -> Tree {
        Tree::of_words(
            rows.iter()
                .map(|r| {
                    let c: Vec<&str> = r.split_whitespace().collect();
                    Word { form: c[0].into(), lemma: c[1].into(), upos: UPos::parse(c[2]).unwrap(), tag: Tag::parse(c[3]).unwrap(), feats: Feats::default(), head: c[4].parse().unwrap(), rel: Rel::parse(c[5]).unwrap() }
                })
                .collect(),
        )
    }

    fn read(t: &Tree) -> (Vec<Frame>, Vec<Hit>) {
        let mut ctx = Ctx::default();
        let (fs, _, _) = frames_of_tree(t, &mut ctx, false);
        let hits = sentence(t, &fs, &ctx.anchors);
        (fs, hits)
    }

    fn has(h: &[Hit], c: Check) -> bool {
        h.iter().any(|x| x.check == c)
    }

    #[test]
    fn copula_without_subject() {
        // «That film was garbage.» with the subject parsed as a compound: flagged
        let bad = tree(&["That that DET DT 2 det", "film film NOUN NN 4 compound", "was be AUX VBD 4 cop", "garbage garbage NOUN NN 0 root", ". . PUNCT . 4 punct"]);
        assert!(has(&read(&bad).1, Check::CopulaNoWho));
        // negative control: the right tree has its subject
        let good = tree(&["That that DET DT 2 det", "film film NOUN NN 4 nsubj", "was be AUX VBD 4 cop", "garbage garbage NOUN NN 0 root", ". . PUNCT . 4 punct"]);
        assert!(!has(&read(&good).1, Check::CopulaNoWho));
    }

    #[test]
    fn time_as_place() {
        let mut f = Frame::new(Kind::Event, "come".into());
        f.place = Some(crate::frames::Ent { id: "tuesday".into(), text: "Tuesdays".into(), named: true, ..Default::default() });
        assert!(has(&sentence(&Tree::default(), &[f.clone()], &[]), Check::TimeInWhere));
        // negative control: a real place
        f.place = Some(crate::frames::Ent { id: "station".into(), text: "the station".into(), ..Default::default() });
        assert!(!has(&sentence(&Tree::default(), &[f], &[]), Check::TimeInWhere));
        // and the reader itself puts «on Tuesdays» in no place
        let t = tree(&["The the DET DT 2 det", "truck truck NOUN NN 3 nsubj", "comes come VERB VBZ 0 root", "on on ADP IN 5 case", "Tuesdays tuesday PROPN NNPS 3 obl", ". . PUNCT . 3 punct"]);
        let (fs, h) = read(&t);
        assert!(fs[0].place.is_none() && !has(&h, Check::TimeInWhere), "{:?}", fs[0].show(0));
        // negative control: «march» as a common noun is no time
        assert!(!time_word("march", false) && time_word("march", true));
    }

    #[test]
    fn same_entity_twice() {
        let mut f = Frame::new(Kind::Fact(Link::Is), "be".into());
        let things = crate::frames::Ent { id: "thing".into(), text: "things".into(), ..Default::default() };
        f.agent = Some(things.clone());
        f.patient = Some(things.clone());
        assert!(has(&sentence(&Tree::default(), &[f.clone()], &[]), Check::WhoIsWhat));
        // negative controls: a reflexive, and two different entities
        f.patient = Some(crate::frames::Ent { pron: Some("themselves".into()), ..things.clone() });
        assert!(!has(&sentence(&Tree::default(), &[f.clone()], &[]), Check::WhoIsWhat));
        let you = crate::frames::Ent { id: "you".into(), text: "you".into(), ..Default::default() };
        f.agent = Some(you.clone());
        f.patient = Some(crate::frames::Ent { text: "yourself".into(), ..you.clone() });
        assert!(!has(&sentence(&Tree::default(), &[f.clone()], &[]), Check::WhoIsWhat));
        let he = |w: &str| crate::frames::Ent { id: "he".into(), text: w.into(), pron: Some(w.into()), ..Default::default() };
        f.agent = Some(he("he"));
        f.patient = Some(he("him"));
        assert!(!has(&sentence(&Tree::default(), &[f.clone()], &[]), Check::WhoIsWhat));
        f.agent = Some(things.clone());
        f.patient = Some(crate::frames::Ent { id: "garbage".into(), text: "garbage".into(), ..Default::default() });
        assert!(!has(&sentence(&Tree::default(), &[f], &[]), Check::WhoIsWhat));
        // the reader: «things that are untrue» gives no row with things twice
        let t = tree(&["Nonsense nonsense NOUN NN 0 root", "; ; PUNCT : 3 punct", "things thing NOUN NNS 1 conj", "that that PRON WDT 6 nsubj", "are be AUX VBP 6 cop", "untrue untrue ADJ JJ 3 acl:relcl", ". . PUNCT . 1 punct"]);
        assert!(!has(&read(&t).1, Check::WhoIsWhat));
    }

    #[test]
    fn unsaid_with_own_dependent() {
        // a frame that says its patient is inherited while the clause has an object of its own: flagged
        let t = tree(&["We we PRON PRP 2 nsubj", "sorted sort VERB VBD 0 root", "the the DET DT 4 det", "garbage garbage NOUN NN 2 obj", ". . PUNCT . 2 punct"]);
        let mut ctx = Ctx::default();
        let (mut fs, _, _) = frames_of_tree(&t, &mut ctx, false);
        assert!(!has(&sentence(&t, &fs, &ctx.anchors), Check::UnsaidOwn), "negative control: the reader's own frame");
        fs[0].unsaid.push(Slot::Patient);
        let mut an = ctx.anchors.clone();
        an[0].roles.retain(|(s, _)| *s != Slot::Patient);
        assert!(has(&sentence(&t, &fs, &an), Check::UnsaidOwn));
        // negative control: «things you throw away» — the patient is the noun, the clause has no object
        let t = tree(&["things thing NOUN NNS 0 root", "you you PRON PRP 3 nsubj", "throw throw VERB VBP 1 acl:relcl", "away away ADV RB 3 advmod", ". . PUNCT . 1 punct"]);
        let (fs, h) = read(&t);
        assert!(fs[0].unsaid.contains(&Slot::Patient) && !has(&h, Check::UnsaidOwn), "{}", fs[0].show(0));
    }

    #[test]
    fn reader_fixes() {
        // «Don't talk garbage»: a negative imperative, said to «you»
        let t = tree(&["Do do AUX VB 3 aux", "n't not PART RB 3 advmod", "talk talk VERB VB 0 root", "garbage garbage NOUN NN 3 obj", "! ! PUNCT . 3 punct"]);
        let (fs, _) = read(&t);
        assert!(fs[0].tam.imperative && fs[0].neg && fs[0].agent.as_ref().is_some_and(|a| a.id == "you"), "{} {}", fs[0].show(0), fs[0].tam.show());
        // negative control: a question with do-support is no imperative
        let t = tree(&["Do do AUX VB 3 aux", "you you PRON PRP 3 nsubj", "talk talk VERB VB 0 root", "garbage garbage NOUN NN 3 obj", "? ? PUNCT . 3 punct"]);
        assert!(!read(&t).0[0].tam.imperative);
        // «sorted the garbage into paper, plastic and food»: where it ends up, not a place
        let t = tree(&["We we PRON PRP 2 nsubj", "sorted sort VERB VBD 0 root", "the the DET DT 4 det", "garbage garbage NOUN NN 2 obj", "into into ADP IN 6 case", "paper paper NOUN NN 2 obl", ". . PUNCT . 2 punct"]);
        let (fs, _) = read(&t);
        assert_eq!(fs[0].goal(), Some("goal"), "{}", fs[0].show_how(0));
        // negative control: «put the peel in the garbage» keeps its place
        let t = tree(&["Put put VERB VB 0 root", "the the DET DT 3 det", "peel peel NOUN NN 1 obj", "in in ADP IN 6 case", "the the DET DT 6 det", "garbage garbage NOUN NN 1 obl", ". . PUNCT . 1 punct"]);
        let fs = read(&t).0;
        assert!(fs[0].place.as_ref().is_some_and(|p| p.id == "garbage") && fs[0].goal().is_none());
        // «things you throw away» and «a family throws away the garbage»: one verb, one frame, one class
        let rel = tree(&["things thing NOUN NNS 0 root", "you you PRON PRP 3 nsubj", "throw throw VERB VBP 1 acl:relcl", "away away ADV RB 3 advmod", ". . PUNCT . 1 punct"]);
        let main = tree(&["A a DET DT 2 det", "family family NOUN NN 3 nsubj", "throws throw VERB VBZ 0 root", "away away ADV RB 3 advmod", "the the DET DT 6 det", "garbage garbage NOUN NN 3 obj", ". . PUNCT . 3 punct"]);
        let (a, b) = (read(&rel).0, read(&main).0);
        assert_eq!((a[0].how.class.as_str(), a[0].how.sig.as_str()), (b[0].how.class.as_str(), b[0].how.sig.as_str()), "{} | {}", a[0].show_how(0), b[0].show_how(0));
    }

    #[test]
    fn spread_of_classes() {
        let fr = |class: &str| {
            let mut f = Frame::new(Kind::Event, "throw away".into());
            f.how.by = By::VerbNet;
            f.how.class = class.into();
            f.how.sig = "NP V NP".into();
            f
        };
        let mut s = Spread::default();
        s.add("a", &[fr("throw-17.1")]);
        s.add("b", &[fr("throw-17.1")]);
        // negative control: one class for the group
        assert_eq!(s.counts(), (1, 0));
        assert!(s.hits().is_empty());
        s.add("c", &[fr("amuse-31.1")]);
        assert_eq!(s.counts(), (1, 1));
        let h = s.hits();
        assert_eq!(h.len(), 1);
        assert_eq!((h[0].0.as_str(), h[0].1), ("c", 1));
    }
}

#[cfg(test)]
mod wave12_models {
    use crate::frames::{Anchor, By, Ent, Frame, Kind, Link, Rel, Slot, Tree};
    use crate::invariants::{Check, Spread, sentence};
    use en::annotate::Word;
    use en::gram::{Feats, Tag, UPos};

    #[test]
    fn finite_spread_plurality_ties_rows_and_groups() {
        // Independent finite frequency oracle: a wins ties, then b, then c.
        for a in 0..=3 {
            for b in 0..=3 {
                for c in 0..=3 {
                    let counts = [a, b, c];
                    let mut spread = Spread::default();
                    let mut rows = Vec::new();
                    for (class, count) in counts.iter().enumerate() {
                        for _ in 0..*count {
                            let mut f = Frame::new(Kind::Event, "carry".into());
                            f.how.by = By::VerbNet;
                            f.how.sig = "NP V NP".into();
                            f.how.class = ["a", "b", "c"][class].into();
                            rows.push(f);
                        }
                    }
                    let mut ignored = Frame::new(Kind::Event, "ignored".into());
                    ignored.how.by = By::VerbNet;
                    rows.insert(0, ignored); // empty signature must not enter the denominator
                    spread.add("s", &rows);
                    let present = counts.iter().filter(|n| **n > 0).count();
                    assert_eq!(
                        spread.counts(),
                        (usize::from(present > 0), usize::from(present > 1))
                    );
                    let maximum = *counts.iter().max().unwrap();
                    let winner = counts.iter().position(|n| *n == maximum).unwrap();
                    let expected: Vec<_> = rows
                        .iter()
                        .enumerate()
                        .skip(1)
                        .filter(|(_, f)| f.how.class != ["a", "b", "c"][winner])
                        .map(|(i, _)| ("s".to_string(), i + 1))
                        .collect();
                    let actual: Vec<_> = spread
                        .hits()
                        .into_iter()
                        .map(|(id, row, _)| (id, row))
                        .collect();
                    assert_eq!(actual, expected);
                }
            }
        }
        let mut spread = Spread::default();
        for (verb, sig) in [("carry", "NP V"), ("carry", "NP V NP"), ("take", "NP V")] {
            let mut f = Frame::new(Kind::Event, verb.into());
            f.how.by = By::VerbNet;
            f.how.sig = sig.into();
            f.how.class = "a".into();
            spread.add("same", &[f]);
        }
        assert_eq!(spread.counts(), (3, 0));
        assert!(spread.hits().is_empty());
    }

    #[test]
    fn finite_unsaid_role_dependency_and_anchor_exclusions() {
        for (relation, slot, passive_required) in [
            (Rel::Nsubj, Slot::Agent, false),
            (Rel::OblAgent, Slot::Agent, true),
            (Rel::NsubjPass, Slot::Patient, false),
            (Rel::Obj, Slot::Patient, false),
            (Rel::Iobj, Slot::Recipient, false),
        ] {
            for passive in [false, true] {
                for used in [false, true] {
                    for present in [false, true] {
                        let word = |head, rel, text: &str| Word {
                            form: text.into(),
                            lemma: text.into(),
                            upos: UPos::NOUN,
                            tag: Tag::NN,
                            feats: Feats::default(),
                            head,
                            rel,
                        };
                        let t = Tree::of_words(vec![
                            word(0, Rel::Root, "carry"),
                            word(1, relation, "parcel"),
                        ]);
                        let mut f = Frame::new(Kind::Event, "carry".into());
                        f.tam.passive = passive;
                        f.unsaid.push(slot);
                        let anchors = if present {
                            vec![Anchor {
                                pred: Some(0),
                                roles: if used { vec![(slot, 1)] } else { vec![] },
                                ..Default::default()
                            }]
                        } else {
                            vec![]
                        };
                        let hits = sentence(
                            &t,
                            &[Frame::new(Kind::Bare, "".into()), f],
                            &[Anchor::default()]
                                .into_iter()
                                .chain(anchors)
                                .collect::<Vec<_>>(),
                        );
                        let expected = present && !used && (!passive_required || passive);
                        assert_eq!(hits.len(), usize::from(expected));
                        if expected {
                            assert_eq!((hits[0].check, hits[0].row), (Check::UnsaidOwn, 2));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn finite_copula_and_entity_negative_controls() {
        for kind in [
            Kind::Bare,
            Kind::Event,
            Kind::Fact(Link::Is),
            Kind::Fact(Link::Has),
        ] {
            for verb in ["be", "carry"] {
                for subject in [false, true] {
                    let mut f = Frame::new(kind, verb.into());
                    if subject {
                        f.agent = Some(Ent {
                            id: "alice".into(),
                            text: "Alice".into(),
                            ..Default::default()
                        });
                    }
                    let expected = !subject
                        && (kind == Kind::Fact(Link::Is) || kind != Kind::Bare && verb == "be");
                    let hits = sentence(&Tree::default(), &[f], &[]);
                    assert_eq!(hits.len(), usize::from(expected));
                    if expected {
                        assert_eq!(hits[0].check, Check::CopulaNoWho);
                    }
                }
            }
        }
        for (id, text, pron, expected) in [
            ("alice", "Alice", None, true),
            ("alice", "herself", None, false),
            ("alice", "Alice", Some("herself"), false),
            ("bob", "Bob", None, false),
            ("", "", None, false),
        ] {
            let mut f = Frame::new(Kind::Event, "see".into());
            f.agent = Some(Ent {
                id: "Alice".into(),
                text: "Alice".into(),
                ..Default::default()
            });
            f.patient = Some(Ent {
                id: id.into(),
                text: text.into(),
                pron: pron.map(str::to_string),
                ..Default::default()
            });
            assert_eq!(
                sentence(&Tree::default(), &[f], &[]).len(),
                usize::from(expected)
            );
        }
    }
}
