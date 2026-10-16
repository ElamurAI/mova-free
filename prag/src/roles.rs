//! Pragmatic roles as positions, for the parser to read back: pragmatics on top of the AST — the MMM parses, the
//! pragmatics is made, the same parse runs again with the pragmatics as input, until it converges.
//!
//! The reader's frames name their fillers by head lemma (`Ent.id`), not by token. Here each frame is put back on the
//! sentence: the predicate is the token whose lemma is the frame's verb (the nearest unused one for repeated verbs);
//! a filler is the token with its head lemma, the one closest to the predicate. Output: (dependent, predicate, role),
//! 1-based, for every filled slot of every event or speech frame.

use en::annotate::Word;
use en::gram::UPos;

use crate::frames::{frames_of_tree, Ctx, Kind, Tree};

pub const ROLES: [&str; 7] = ["agent", "patient", "recipient", "place", "attr", "obl", "quote"];

/// (dependent, predicate, role) for one sentence read alone (1-based positions).
pub fn roles(words: Vec<Word>) -> Vec<(usize, usize, &'static str)> {
    roles_in(words, &mut Ctx::default())
}

/// The document a school sentence belongs to (pragmatics is not measured on one sentence): a lesson
/// (`school:<course>:<lesson>:<n>` → course:lesson), and for the books course the whole book across its lessons
/// (`043-willows-ch5-part3…` → books:willows).
pub fn doc_key(id: &str) -> (String, u32, u32) {
    let parts: Vec<&str> = id.split(':').collect();
    let n: u32 = parts.last().and_then(|x| x.parse().ok()).unwrap_or(0);
    if parts.len() >= 4 && parts[0] == "school" {
        let lesson = parts[2];
        let num: u32 = lesson.split('-').next().and_then(|x| x.parse().ok()).unwrap_or(0);
        if parts[1] == "books" {
            let book = lesson.split('-').nth(1).unwrap_or(lesson);
            return (format!("books:{book}"), num, n);
        }
        return (format!("{}:{}", parts[1], lesson), num, n);
    }
    (id.rsplit_once(':').map_or(id, |x| x.0).to_string(), 0, n)
}

/// Roles for the sentences of documents read in order with one discourse state per document — the temporary world:
/// who has been mentioned and how salient, what pronouns point to, who spoke last. Input: (sent id, words);
/// output in the same order.
pub fn roles_docs(sents: Vec<(String, Vec<Word>)>) -> Vec<(String, Vec<(usize, usize, &'static str)>)> {
    let mut order: Vec<usize> = (0..sents.len()).collect();
    order.sort_by_key(|&i| doc_key(&sents[i].0));
    let mut out: Vec<Option<Vec<(usize, usize, &'static str)>>> = vec![None; sents.len()];
    let mut ctx = Ctx::default();
    let mut cur = String::new();
    for i in order {
        let key = doc_key(&sents[i].0).0;
        if key != cur {
            ctx = Ctx::default();
            cur = key;
        }
        out[i] = Some(roles_in(sents[i].1.clone(), &mut ctx));
        ctx.new_line();
    }
    sents.into_iter().zip(out).map(|((id, _), r)| (id, r.unwrap_or_default())).collect()
}

/// The world before each sentence of the documents (same reading as `roles_docs`): the salient entities and the
/// last speaker (`Ctx::summary`), plus the last few events as «agent verb patient». Output in input order.
pub fn world_docs(sents: Vec<(String, Vec<Word>)>, events: usize) -> Vec<(String, String)> {
    let mut order: Vec<usize> = (0..sents.len()).collect();
    order.sort_by_key(|&i| doc_key(&sents[i].0));
    let mut out: Vec<String> = vec![String::new(); sents.len()];
    let mut ctx = Ctx::default();
    let mut cur = String::new();
    let mut recent: std::collections::VecDeque<String> = std::collections::VecDeque::new();
    for i in order {
        let key = doc_key(&sents[i].0).0;
        if key != cur {
            ctx = Ctx::default();
            recent.clear();
            cur = key;
        }
        let ev = if recent.is_empty() { "none yet".to_string() } else { recent.iter().cloned().collect::<Vec<_>>().join("; ") };
        out[i] = format!("{} | recent events: {ev}", ctx.summary(6));
        let t = Tree::of_words(sents[i].1.clone());
        if !t.is_empty() {
            let (frames, _, _) = frames_of_tree(&t, &mut ctx, false);
            for f in frames.iter().filter(|f| matches!(f.kind, Kind::Event | Kind::Say(_)) && !f.verb.is_empty()) {
                let a = f.agent.as_ref().map(|e| e.show()).unwrap_or_default();
                let p = f.patient.as_ref().map(|e| e.show()).unwrap_or_default();
                let e = format!("{a} {}{} {p}", if f.neg { "not " } else { "" }, f.verb).trim().to_string();
                recent.push_back(e);
                while recent.len() > events {
                    recent.pop_front();
                }
            }
        }
        ctx.new_line();
    }
    sents.into_iter().zip(out).map(|((id, _), w)| (id, w)).collect()
}

fn roles_in(words: Vec<Word>, ctx: &mut Ctx) -> Vec<(usize, usize, &'static str)> {
    let t = Tree::of_words(words);
    if t.is_empty() {
        return Vec::new();
    }
    let (frames, _, _) = frames_of_tree(&t, ctx, false);
    let w = &t.w;
    let lemma = |i: usize| w[i].lemma.to_lowercase();
    let mut used_pred = vec![false; w.len()];
    let mut out = Vec::new();
    for f in &frames {
        if !matches!(f.kind, Kind::Event | Kind::Say(_)) || f.verb.is_empty() {
            continue;
        }
        let head = f.verb.split(' ').next().unwrap_or(&f.verb).to_lowercase();
        let Some(p) = (0..w.len()).find(|&i| !used_pred[i] && lemma(i) == head && matches!(w[i].upos, UPos::VERB | UPos::AUX | UPos::NOUN | UPos::ADJ)) else { continue };
        used_pred[p] = true;
        let mut put = |id: &str, role: &'static str| {
            let id = id.to_lowercase();
            // a pronoun the world resolved keeps its own token (its antecedent is in an earlier sentence)
            if let Some(d) = (0..w.len()).filter(|&i| i != p && (lemma(i) == id || w[i].form.to_lowercase() == id)).min_by_key(|&i| i.abs_diff(p)) {
                out.push((d + 1, p + 1, role));
            }
        };
        for (slot, role) in [(&f.agent, "agent"), (&f.patient, "patient"), (&f.recipient, "recipient"), (&f.place, "place")] {
            if let Some(e) = slot {
                put(e.pron.as_deref().unwrap_or(&e.id), role);
            }
        }
        for a in &f.attrs {
            put(a, "attr");
        }
        for (_, e) in &f.obls {
            put(&e.id, "obl");
        }
    }
    out.sort();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use en::gram::{Feats, Rel, Tag};

    fn w(form: &str, lemma: &str, upos: UPos, tag: Tag, head: usize, rel: Rel) -> Word {
        Word { form: form.into(), lemma: lemma.into(), upos, tag, feats: Feats::default(), head, rel }
    }

    #[test]
    fn the_cat_ate_the_fish() {
        let s = vec![
            w("The", "the", UPos::DET, Tag::DT, 2, Rel::Det),
            w("cat", "cat", UPos::NOUN, Tag::NN, 3, Rel::Nsubj),
            w("ate", "eat", UPos::VERB, Tag::VBD, 0, Rel::Root),
            w("the", "the", UPos::DET, Tag::DT, 5, Rel::Det),
            w("fish", "fish", UPos::NOUN, Tag::NN, 3, Rel::Obj),
            w(".", ".", UPos::PUNCT, Tag::Stop, 3, Rel::Punct),
        ];
        let r = roles(s);
        assert!(r.contains(&(2, 3, "agent")), "{r:?}");
        assert!(r.contains(&(5, 3, "patient")), "{r:?}");
        // negative control: a determiner gets no role
        assert!(!r.iter().any(|x| x.0 == 1));
    }

    #[test]
    fn documents_group_lessons_and_books() {
        assert_eq!(doc_key("school:space:003-mercury.lesson:4"), ("space:003-mercury.lesson".to_string(), 3, 4));
        assert_eq!(doc_key("school:books:043-willows-ch5-part3-sardines.lesson:30").0, "books:willows");
        // negative control: two books stay apart
        assert_ne!(doc_key("school:books:018-alice-the-mock-turtles.lesson:1").0, doc_key("school:books:043-willows-ch5.lesson:1").0);
    }
}
