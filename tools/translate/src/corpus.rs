//! An author's verses (or any text in units) through the standard English stack (tools/roundtrip's layers): text → tokens (with the
//! literal gaps) → UD tree (`en`) → frames (`prag`, the discourse carried through each book) → world (per book). A
//! verse keeps its tokens, its tree (the sentences' trees joined, heads shifted) and the events and entities its
//! sentences have in the book's world.

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::Result;
use en::annotate::{Annotator, Word};
use roundtrip::frames::F;
use roundtrip::tally::Residual;
use roundtrip::tree::TNode;
use roundtrip::{vol, world};

use crate::bible::VKey;

#[derive(Clone, Debug, Default)]
pub struct PVerse {
    pub text: String,
    /// the verse's tokens, every sentence's in order
    pub forms: Vec<String>,
    /// the literal before each token (gaps[0] — before the verse)
    pub gaps: Vec<String>,
    /// the literal after the last token
    pub tail: String,
    /// the verse's tree: its sentences' trees in order, heads shifted (each sentence keeps its root, head 0)
    pub words: Vec<Word>,
    /// tokens per sentence (the standard stack's sentences)
    pub sent_lens: Vec<usize>,
    /// the words the meta-world is read from: the parse, or (`--meta-tree frames`) its heads and relations taken from
    /// the frames' anchors where they have one
    pub mw: Vec<Word>,
    /// the events of the verse in the book's world: verb lemma, negated
    pub events: Vec<(String, bool)>,
    /// the entities those events name (world ids)
    pub ents: Vec<String>,
    /// the verse's logical facts, event by event in order (`logic`)
    pub facts: Vec<crate::logic::Event>,
}

impl PVerse {
    /// Space-after bits over the whole verse.
    pub fn space(&self) -> Vec<bool> {
        (0..self.forms.len()).map(|k| self.gaps.get(k + 1).is_some_and(|g| !g.is_empty())).collect()
    }
    /// The words the meta-world is read from: `mw` when the frames gave it a tree of its own, else the parse.
    pub fn meta_words(&self) -> &[Word] {
        if self.mw.is_empty() { &self.words } else { &self.mw }
    }
    /// Keep only what training needs (the tree and the facts): the text, its tokens and gaps go.
    pub fn compact(&mut self) {
        self.text = String::new();
        self.forms = Vec::new();
        self.gaps = Vec::new();
        self.tail = String::new();
        self.events = Vec::new();
        self.ents = Vec::new();
    }
    pub fn tree(&self) -> Vec<TNode> {
        roundtrip::tree::of_words(&self.words)
    }
}

pub struct Corpus {
    pub name: String,
    pub kind: crate::sources::Kind,
    pub verses: BTreeMap<VKey, PVerse>,
}

/// Join sentences' trees into one: heads shifted by the words before, roots stay 0.
pub fn join_trees(sents: &[Vec<Word>]) -> Vec<Word> {
    let mut out = Vec::new();
    for s in sents {
        let off = out.len();
        out.extend(s.iter().map(|w| Word { head: if w.head == 0 { 0 } else { w.head + off }, ..w.clone() }));
    }
    out
}

/// Parse a run of tokens (no text, as a decoder writes them): cut into sentences by the standard rule on the
/// rule spaces, each sentence annotated, the trees joined.
pub fn parse_tokens(a: &Annotator, forms: &[String]) -> Vec<Word> {
    let space: Vec<bool> = (0..forms.len()).map(|k| forms.get(k + 1).is_some_and(|n| roundtrip::tree::rule_space(&forms[k], n))).collect();
    let mut sents = Vec::new();
    let mut k = 0;
    for n in vol::cut(forms, &space) {
        sents.push(a.annotate(&forms[k..k + n]));
        k += n;
    }
    join_trees(&sents)
}

/// A sentence's tree through its frames' anchors: each predicate hangs on the predicate of the frame above with the
/// clause's relation, each role said in a clause hangs on its predicate with the role as relation (agent → nsubj,
/// patient → obj, the others → obl); every other word keeps the parse's head. A change that would close a cycle is
/// not made.
pub fn anchored(w: &[Word], anchors: &[prag::frames::Anchor]) -> Vec<Word> {
    use en::gram::Rel;
    use prag::frames::Slot;
    let mut out = w.to_vec();
    let n = out.len();
    let set = |out: &mut Vec<Word>, x: usize, h: usize, rel: Rel| {
        if x >= n || h >= n || x == h {
            return;
        }
        // h must not be under x
        let mut y = h + 1;
        for _ in 0..=n {
            if y == 0 {
                break;
            }
            if y == x + 1 {
                return;
            }
            y = out[y - 1].head;
        }
        out[x].head = h + 1;
        out[x].rel = rel;
    };
    for an in anchors {
        let Some(p) = an.pred else { continue };
        if let Some((h, rel)) = an.link {
            set(&mut out, p, h, rel);
        }
        for (slot, t) in &an.roles {
            let rel = match slot {
                Slot::Agent => Rel::Nsubj,
                Slot::Patient => Rel::Obj,
                _ => Rel::Obl,
            };
            set(&mut out, *t, p, rel);
        }
    }
    out
}

pub fn parse_book(a: &Annotator, verses: &[(VKey, String)], frames_tree: bool) -> Result<Vec<(VKey, PVerse)>> {
    let mut ctx = prag::frames::Ctx::default();
    let mut out: Vec<(VKey, PVerse)> = Vec::with_capacity(verses.len());
    let mut all_fs: Vec<Vec<F>> = Vec::new();
    let mut verse_of: Vec<usize> = Vec::new();
    for (k, text) in verses {
        let seg = vol::segment(a, text, 0, false, None)?;
        let mut pv = PVerse { text: text.clone(), tail: seg.tail.clone(), ..Default::default() };
        let mut trees = Vec::new();
        let mut mtrees = Vec::new();
        for s in &seg.sents {
            pv.forms.extend(s.forms.iter().cloned());
            pv.gaps.extend(s.gaps.iter().cloned());
            pv.sent_lens.push(s.forms.len());
            let w = a.annotate(&s.forms);
            // as roundtrip's frames::read, with the anchors kept
            let t = prag::frames::Tree::of_words(w.to_vec());
            let fs: Vec<F> = if t.is_empty() {
                Vec::new()
            } else {
                let (fs, _, _) = prag::frames::frames_of_tree(&t, &mut ctx, false);
                fs.iter().map(F::of).collect()
            };
            // the frames' tree only when asked (else the parse, `meta_words`, with no second copy)
            if frames_tree {
                mtrees.push(if t.is_empty() { w.clone() } else { anchored(&w, &ctx.anchors) });
            }
            pv.facts.extend(crate::logic::events(&fs, &w));
            ctx.new_line();
            all_fs.push(fs);
            verse_of.push(out.len());
            trees.push(w);
        }
        pv.words = join_trees(&trees);
        if frames_tree {
            pv.mw = join_trees(&mtrees);
        }
        out.push((*k, pv));
    }
    let (w, _) = world::encode(&all_fs, &mut Residual::default());
    let wfs = world::decode_in(&w, &all_fs);
    for (i, fs) in wfs.iter().enumerate() {
        let pv = &mut out[verse_of[i]].1;
        for f in fs {
            if f.kind == prag::frames::Kind::Bare {
                continue;
            }
            pv.events.push((f.verb.clone(), f.neg));
            pv.ents.extend(f.ents().iter().map(|e| e.id.clone()));
        }
    }
    Ok(out)
}

/// Parse the verses `keep` selects, book by book in parallel; the result does not depend on the threads.
pub fn parse(a: &Annotator, name: &str, kind: crate::sources::Kind, pool: &BTreeMap<VKey, String>, keep: &dyn Fn(VKey) -> bool, threads: usize, frames_tree: bool) -> Result<Corpus> {
    parse_grouped(a, name, kind, pool, keep, &|k| kind.group(k), threads, frames_tree)
}

/// `parse` with the discourse groups given (`group`: the verses of one group are read as one discourse, one world;
/// a block of chapters for `translate many`).
#[allow(clippy::too_many_arguments)]
pub fn parse_grouped(a: &Annotator, name: &str, kind: crate::sources::Kind, pool: &BTreeMap<VKey, String>, keep: &dyn Fn(VKey) -> bool, group: &dyn Fn(VKey) -> (u8, u16), threads: usize, frames_tree: bool) -> Result<Corpus> {
    let mut books: BTreeMap<(u8, u16), Vec<(VKey, String)>> = BTreeMap::new();
    for (k, t) in pool {
        if keep(*k) {
            books.entry(group(*k)).or_default().push((*k, t.clone()));
        }
    }
    let books: Vec<Vec<(VKey, String)>> = books.into_values().collect();
    let next = AtomicUsize::new(0);
    let res: Mutex<Vec<Option<Result<Vec<(VKey, PVerse)>>>>> = Mutex::new((0..books.len()).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    if i >= books.len() {
                        break;
                    }
                    let r = parse_book(a, &books[i], frames_tree);
                    res.lock().unwrap()[i] = Some(r);
                }
            });
        }
    });
    let mut verses = BTreeMap::new();
    for r in res.into_inner().unwrap() {
        for (k, v) in r.unwrap()? {
            verses.insert(k, v);
        }
    }
    Ok(Corpus { name: name.to_string(), kind, verses })
}

/// Run a function over items in parallel, results in the items' order.
pub fn par_map<T: Sync, R: Send>(items: &[T], threads: usize, f: &(dyn Fn(&T) -> R + Sync)) -> Vec<R> {
    let next = AtomicUsize::new(0);
    let res: Mutex<Vec<Option<R>>> = Mutex::new((0..items.len()).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    if i >= items.len() {
                        break;
                    }
                    let r = f(&items[i]);
                    res.lock().unwrap()[i] = Some(r);
                }
            });
        }
    });
    res.into_inner().unwrap().into_iter().map(|x| x.unwrap()).collect()
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn ordered_parallel_and_compaction() {
        let items: Vec<usize> = (0..17).collect();
        for t in [0, 1, 4] {
            assert_eq!(
                par_map(&items, t, &|x| x * x),
                items.iter().map(|x| x * x).collect::<Vec<_>>()
            );
        }
        assert!(par_map::<usize, usize>(&[], 2, &|x| *x).is_empty());
        let mut v = PVerse {
            text: "a b".into(),
            forms: vec!["a".into(), "b".into()],
            gaps: vec!["".into(), "\n".into()],
            sent_lens: vec![2],
            ..Default::default()
        };
        assert_eq!(v.space(), vec![true, false]);
        v.compact();
        assert!(v.text.is_empty() && v.forms.is_empty() && v.gaps.is_empty());
        assert_eq!(v.sent_lens, vec![2]);
    }
}
