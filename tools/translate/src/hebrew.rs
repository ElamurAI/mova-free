//! The Hebrew side: the Leningrad Codex as MACULA's UD trees (`tanakh-wlc-macula`, words in text order), the
//! negation of `macula prag` (`prag.tsv`), MACULA's verb frames (MISC `Frame=A0:…;A1:…`), and the reader from
//! Hebrew into the meta-world.
//!
//! The reader is learned, not written: on the training verses, each Hebrew word is aligned (IBM Model 1 with MACULA's glosses as a prior, a place
//! prior) to the meta tokens of the same verse read from JPS 1917; the meta tokens aligned to a Hebrew word, with the
//! unaligned ones after them, are that word's chunk (וַיְהִי → «it came to pass»; אֵת → nothing). A Hebrew word reads
//! as its most frequent chunk in the most specific context seen twice: lemma, part of speech, verb type, person and
//! number; lemma, part of speech and verb type; lemma. A lemma never seen reads as MACULA's English gloss through the
//! shared lexicon. The meta tokens keep the Hebrew tree: a chunk's first token hangs where its Hebrew word's head's
//! chunk is.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use anyhow::{Context, Result};
use en::gram::{Rel, Tag};

use crate::bible::{VKey, vkey};
use crate::ibm::{self, Ibm1, Vocab};
use crate::meta::{Lexicon, MTok};

#[derive(Clone, Debug, Default)]
pub struct HTok {
    /// the word's form as written (accents and points), and its XPOS
    pub form: String,
    pub xpos: String,
    pub lemma: String,
    pub upos: String,
    /// verse-level, 1-based, 0 — root
    pub head: usize,
    pub rel: String,
    pub feats: String,
    pub gloss: String,
    pub typ: String,
    pub mid: String,
    /// MACULA's Strong's number
    pub strong: Option<u32>,
    /// MACULA's frame: (role, referenced word ids)
    pub frame: Vec<(String, Vec<String>)>,
}

#[derive(Clone, Debug, Default)]
pub struct HVerse {
    pub toks: Vec<HTok>,
    /// the verse as written (its sentences' `# text`, joined by a space)
    pub text: String,
    /// words per written word (a multiword token's range, or 1)
    pub groups: Vec<usize>,
    /// a negative word in some sentence of the verse (`macula prag`)
    pub neg: bool,
}

fn misc(m: &str, key: &str) -> String {
    m.split('|').find_map(|kv| kv.strip_prefix(key).and_then(|v| v.strip_prefix('='))).unwrap_or("").to_string()
}

fn feat(f: &str, key: &str) -> String {
    misc(f, key)
}

/// The Hebrew verses of a MACULA directory (`*.conllu`), and the negation column of `prag.tsv` if given.
pub fn read(dir: &Path, prag: Option<&Path>) -> Result<BTreeMap<VKey, HVerse>> {
    let mut neg_of: BTreeMap<String, bool> = BTreeMap::new();
    if let Some(p) = prag {
        let t = std::fs::read_to_string(p).with_context(|| format!("{}", p.display()))?;
        for l in t.lines().skip(1) {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() > 10 {
                neg_of.insert(c[0].to_string(), c[10] == "1");
            }
        }
    }
    let mut files: Vec<_> = std::fs::read_dir(dir)?.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "conllu")).collect();
    files.sort();
    let mut out: BTreeMap<VKey, HVerse> = BTreeMap::new();
    for f in files {
        let text = std::fs::read_to_string(&f)?;
        let (mut sid, mut verse) = (String::new(), None);
        let mut sent: Vec<HTok> = Vec::new();
        let mut vtext = String::new();
        let mut groups: Vec<usize> = Vec::new();
        // the last word of the open multiword token
        let mut range_end = 0usize;
        let flush = |out: &mut BTreeMap<VKey, HVerse>, verse: Option<VKey>, sid: &str, sent: &mut Vec<HTok>, vtext: &mut String, groups: &mut Vec<usize>| {
            if let Some(k) = verse {
                let v = out.entry(k).or_default();
                if !vtext.is_empty() {
                    if !v.text.is_empty() {
                        v.text.push(' ');
                    }
                    v.text.push_str(vtext);
                }
                v.groups.append(groups);
                let off = v.toks.len();
                v.toks.extend(sent.drain(..).map(|mut t| {
                    if t.head > 0 {
                        t.head += off;
                    }
                    t
                }));
                v.neg |= neg_of.get(sid).copied().unwrap_or(false);
            }
            sent.clear();
            vtext.clear();
            groups.clear();
        };
        for line in text.lines() {
            if line.is_empty() {
                flush(&mut out, verse, &sid, &mut sent, &mut vtext, &mut groups);
                continue;
            }
            if let Some(s) = line.strip_prefix("# text = ") {
                vtext = s.to_string();
                continue;
            }
            if let Some(s) = line.strip_prefix("# sent_id = ") {
                sid = s.to_string();
                range_end = 0;
                continue;
            }
            if let Some(s) = line.strip_prefix("# verse = ") {
                verse = vkey(s);
                continue;
            }
            if line.starts_with('#') {
                continue;
            }
            let c: Vec<&str> = line.split('\t').collect();
            if c.len() >= 10 {
                if let Some((a, b)) = c[0].split_once('-') {
                    if let (Ok(a), Ok(b)) = (a.parse::<usize>(), b.parse::<usize>()) {
                        groups.push(b + 1 - a);
                        range_end = b;
                    }
                    continue;
                }
            }
            if c.len() < 10 || c[0].contains('.') {
                continue;
            }
            if c[0].parse::<usize>().is_ok_and(|id| id > range_end) {
                groups.push(1);
            }
            let m = c[9];
            let frame = misc(m, "Frame")
                .split(';')
                .filter_map(|x| x.split_once(':'))
                .map(|(r, ids)| (r.to_string(), ids.split(',').map(|i| i.trim_start_matches('o').to_string()).collect()))
                .collect();
            sent.push(HTok {
                form: c[1].to_string(),
                xpos: c[4].to_string(),
                lemma: c[2].to_string(),
                upos: c[3].to_string(),
                head: c[6].parse().unwrap_or(0),
                rel: c[7].to_string(),
                feats: c[5].to_string(),
                gloss: misc(m, "Gloss"),
                typ: misc(m, "Type"),
                mid: misc(m, "MId").trim_start_matches('o').to_string(),
                strong: crate::strong::number(&misc(m, "Strong")),
                frame,
            });
        }
        flush(&mut out, verse, &sid, &mut sent, &mut vtext, &mut groups);
    }
    Ok(out)
}

fn ctx_keys(t: &HTok) -> [String; 3] {
    let (p, n) = (feat(&t.feats, "Person"), feat(&t.feats, "Number"));
    [format!("{}|{}|{}|{p}|{n}", t.lemma, t.upos, t.typ), format!("{}|{}|{}", t.lemma, t.upos, t.typ), t.lemma.clone()]
}

/// A chunk entry: concept, tag, relation, and the index of its head inside the chunk (None — the chunk's head).
type Entry = (String, Tag, Rel, Option<usize>);

fn chunk_key(c: &[Entry]) -> String {
    c.iter().map(|(x, t, r, h)| format!("{x}/{t}/{r}/{}", h.map_or("-".into(), |v| v.to_string()))).collect::<Vec<_>>().join(" ")
}

fn chunk_of(s: &str) -> Vec<Entry> {
    s.split(' ')
        .filter(|x| !x.is_empty())
        .filter_map(|e| {
            let mut it = e.rsplitn(4, '/');
            let h = it.next()?;
            let r = Rel::parse(it.next()?)?;
            let t = Tag::parse(it.next()?)?;
            let x = it.next()?.to_string();
            Some((x, t, r, h.parse().ok()))
        })
        .collect()
}

/// The words of a MACULA gloss («he.created» → he, created; «(et)» → none).
pub fn gloss_words(g: &str) -> Vec<String> {
    g.split(['.', ' ', '_']).filter(|w| !w.is_empty() && !w.starts_with('(')).map(str::to_string).collect()
}

/// Idioms whose English is not their words': וַיְהִי alone at a clause's start (no subject of its own) is *it came
/// to pass*; עָשָׂה with כֵּן is *did so*.
fn idioms(lex: &Lexicon, v: &HVerse) -> BTreeMap<usize, Vec<Entry>> {
    let n = v.toks.len();
    let mut out = BTreeMap::new();
    for i in 0..n {
        let t = &v.toks[i];
        let kids: Vec<usize> = (0..n).filter(|&j| v.toks[j].head == i + 1).collect();
        if t.lemma == "הָיָה" && t.typ == "wayyiqtol" && feat(&t.feats, "Person") == "3" && !kids.iter().any(|&j| v.toks[j].rel == "nsubj") && !kids.iter().any(|&j| v.toks[j].rel == "cop") {
            let ctp = lex.phrases.get("come to pass").cloned().unwrap_or_else(|| "come_to_pass".to_string());
            out.insert(i, vec![(lex.concept("it").to_string(), Tag::PRP, Rel::Nsubj, Some(1)), (ctp, Tag::VBD, Rel::Root, None)]);
        }
        if t.lemma == "עָשָׂה" && kids.iter().any(|&j| v.toks[j].lemma == "כֵּן") {
            out.insert(i, vec![(lex.concept("do").to_string(), Tag::VBD, Rel::Root, None)]);
        }
    }
    out
}

/// The plan of a Hebrew verse's English grammar: the order to read its words in (a possessive suffix before its
/// noun, a subject after its verb moved before the verb, after the verb's conjunction), the words to insert before a
/// word's chunk («of» before a construct dependent, «the» before a construct head whose chain is definite), and the
/// possessive concept of each suffix by person, number and gender.
#[allow(clippy::type_complexity)]
fn grammar_plan(lex: &Lexicon, v: &HVerse) -> (Vec<usize>, BTreeMap<usize, Vec<(String, Tag, Rel)>>, BTreeMap<usize, String>) {
    let n = v.toks.len();
    let kids: Vec<Vec<usize>> = (0..n).map(|h| (0..n).filter(|&i| v.toks[i].head == h + 1).collect()).collect();
    let has = |i: usize, rel: &str| kids[i].iter().any(|&c| v.toks[c].rel == rel);
    // a word is definite when it is a name, has the article or a possessive suffix, or ends a definite chain
    fn definite(v: &HVerse, kids: &[Vec<usize>], i: usize, depth: usize) -> bool {
        let t = &v.toks[i];
        if t.upos == "PROPN" || depth > 8 {
            return t.upos == "PROPN";
        }
        kids[i].iter().any(|&c| matches!(v.toks[c].rel.as_str(), "det" | "nmod:poss")) || kids[i].iter().any(|&c| v.toks[c].rel == "compound:smixut" && definite(v, kids, c, depth + 1))
    }
    let mut pre: BTreeMap<usize, Vec<(String, Tag, Rel)>> = BTreeMap::new();
    let mut poss: BTreeMap<usize, String> = BTreeMap::new();
    for i in 0..n {
        let t = &v.toks[i];
        if t.rel == "compound:smixut" {
            pre.entry(i).or_default().push((lex.concept("of").to_string(), Tag::IN, Rel::Case));
        }
        if t.upos == "NOUN" && !has(i, "det") && !has(i, "nmod:poss") && kids[i].iter().any(|&c| v.toks[c].rel == "compound:smixut" && definite(v, &kids, c, 0)) {
            pre.entry(i).or_default().push((lex.concept("the").to_string(), Tag::DT, Rel::Det));
        }
        if t.upos == "PRON" && t.rel == "nmod:poss" {
            let (p, num, g) = (feat(&t.feats, "Person"), feat(&t.feats, "Number"), feat(&t.feats, "Gender"));
            let w = match (p.as_str(), num.as_str(), g.as_str()) {
                ("1", "Sing", _) => "my",
                ("1", _, _) => "our",
                ("2", "Sing", _) => "thy",
                ("2", _, _) => "your",
                ("3", "Plur", _) => "their",
                ("3", _, "Fem") => "her",
                _ => "his",
            };
            poss.insert(i, lex.concept(w).to_string());
        }
    }
    // order: possessive suffixes before their noun; a subject after its verb before the verb
    let mut order: Vec<usize> = (0..n).collect();
    let pos = |order: &Vec<usize>, x: usize| order.iter().position(|&y| y == x).unwrap();
    for &i in poss.keys() {
        let h = v.toks[i].head;
        if h > 0 && h - 1 < i {
            let (pi, ph) = (pos(&order, i), pos(&order, h - 1));
            let x = order.remove(pi);
            order.insert(ph, x);
        }
    }
    let subtree = |r: usize| -> BTreeSet<usize> {
        let mut s = BTreeSet::from([r]);
        let mut grew = true;
        while grew {
            grew = false;
            for i in 0..n {
                let h = v.toks[i].head;
                if h > 0 && s.contains(&(h - 1)) && s.insert(i) {
                    grew = true;
                }
            }
        }
        s
    };
    for vb in 0..n {
        if !matches!(v.toks[vb].upos.as_str(), "VERB" | "AUX") {
            continue;
        }
        let Some(&sj) = kids[vb].iter().find(|&&c| v.toks[c].rel == "nsubj" && c > vb) else { continue };
        let st = subtree(sj);
        if st.contains(&vb) {
            continue;
        }
        let moved: Vec<usize> = order.iter().copied().filter(|x| st.contains(x)).collect();
        order.retain(|x| !st.contains(x));
        // before the verb, after the verb's own conjunction right before it
        let mut at = pos(&order, vb);
        while at > 0 && v.toks[order[at - 1]].rel == "cc" && v.toks[order[at - 1]].head == vb + 1 {
            at -= 1;
        }
        let at = if at < pos(&order, vb) { pos(&order, vb) } else { at };
        for (j, x) in moved.into_iter().enumerate() {
            order.insert(at + j, x);
        }
    }
    (order, pre, poss)
}

#[derive(Clone, Debug, Default)]
pub struct Reader {
    tables: Vec<BTreeMap<String, BTreeMap<String, u32>>>,
    /// Hebrew words aligned in training, words left out
    pub aligned: u64,
    pub null: u64,
    /// meta tokens aligned by Strong's number
    pub strong: u64,
    /// read with the English grammar plan (`grammar_plan`)
    pub grammar: bool,
    /// the Hebrew word that owns each meta token of each training pair (its chunk), for the Hebrew encoder
    pub owners: Vec<Vec<Option<usize>>>,
}

const MIN: [u32; 3] = [2, 2, 1];

impl Reader {
    /// Learn the reader on (Hebrew verse, the meta-world read from the English of the same verse). `shuffle`: a
    /// negative control, each Hebrew verse paired with another verse's meta-world.
    /// Each pair may carry the Strong's number of every meta token (eBible's KJV): a meta token with a number is
    /// aligned to the Hebrew word with the same number (the nearest by relative place), the rest by IBM Model 1.
    pub fn train(lex: &Lexicon, pairs: &[(&HVerse, Vec<MTok>, Vec<Option<u32>>)], shuffle: bool) -> Reader {
        let n = pairs.len();
        let pick = |i: usize| if shuffle { (i + 7919) % n.max(1) } else { i };
        let tgt = |i: usize| &pairs[pick(i)].1;
        let fv = Vocab::of(pairs.iter().flat_map(|(h, _, _)| h.toks.iter().map(|t| &t.lemma)));
        let ev = Vocab::of(pairs.iter().flat_map(|(_, m, _)| m.iter().map(|x| &x.concept)));
        let ids: Vec<(Vec<u32>, Vec<u32>)> = (0..n).map(|i| (pairs[i].0.toks.iter().map(|t| fv.id(&t.lemma)).collect(), tgt(i).iter().map(|x| ev.id(&x.concept)).collect())).collect();
        // MACULA's glosses as a prior: each Hebrew lemma's gloss words, read as concepts
        let mut prior: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
        for (h, _, _) in pairs {
            for t in &h.toks {
                let f = fv.id(&t.lemma);
                for w in gloss_words(&t.gloss) {
                    let e = ev.id(lex.concept(&w.to_lowercase()));
                    if e > 0 {
                        prior.entry(f).or_default().push(e);
                    }
                }
            }
        }
        for v in prior.values_mut() {
            v.sort_unstable();
            v.dedup();
        }
        let model = Ibm1::train_prior(&ids, fv.ids.len(), 5, &prior, 2.0);
        let mut r = Reader { tables: vec![BTreeMap::new(); 3], aligned: 0, null: 0, strong: 0, grammar: false, owners: Vec::with_capacity(n) };
        for i in 0..n {
            let (h, m) = (pairs[i].0, tgt(i));
            let _ = &m;
            let mut al = ibm::align(&model, &ids[i].0, &ids[i].1);
            let strongs = &pairs[pick(i)].2;
            let (nh, nm) = (h.toks.len().max(1) as f64, m.len().max(1) as f64);
            for (j, sj) in strongs.iter().enumerate() {
                let Some(sn) = sj else { continue };
                let best = (0..h.toks.len()).filter(|&k| h.toks[k].strong == Some(*sn)).min_by(|&a, &b| {
                    let d = |k: usize| ((k as f64 + 0.5) / nh - (j as f64 + 0.5) / nm).abs();
                    d(a).partial_cmp(&d(b)).unwrap().then(a.cmp(&b))
                });
                if let Some(k) = best {
                    if j < al.len() {
                        al[j] = Some(k);
                        r.strong += 1;
                    }
                }
            }
            // chunks: aligned meta tokens, the unaligned ones with the aligned token before them (or after, at the start)
            let mut owner: Vec<Option<usize>> = al.clone();
            let mut last = None;
            for j in 0..owner.len() {
                match owner[j] {
                    Some(_) => last = owner[j],
                    None => owner[j] = last,
                }
            }
            let first = owner.iter().flatten().next().copied();
            for o in owner.iter_mut() {
                if o.is_none() {
                    *o = first;
                }
            }
            r.owners.push(owner.clone());
            for (k, t) in h.toks.iter().enumerate() {
                let js: Vec<usize> = (0..m.len()).filter(|&j| owner[j] == Some(k)).collect();
                if js.is_empty() {
                    r.null += 1;
                } else {
                    r.aligned += 1;
                }
                // the chunk's head: the token whose meta head is outside the chunk
                let pos = |j: usize| js.iter().position(|&x| x == j);
                let entries: Vec<Entry> = js
                    .iter()
                    .map(|&j| {
                        let x = &m[j];
                        let inner = if x.head == 0 { None } else { pos(x.head - 1) };
                        (x.concept.clone(), x.tag, x.rel, inner)
                    })
                    .collect();
                let key = chunk_key(&entries);
                for (l, ck) in ctx_keys(t).into_iter().enumerate() {
                    *r.tables[l].entry(ck).or_default().entry(key.clone()).or_default() += 1;
                }
            }
        }
        r
    }

    fn chunk(&self, lex: &Lexicon, t: &HTok) -> (Vec<Entry>, String) {
        for (l, ck) in ctx_keys(t).into_iter().enumerate() {
            if let Some(tb) = self.tables[l].get(&ck) {
                let total: u32 = tb.values().sum();
                if total >= MIN[l] {
                    let (best, n) = tb.iter().fold((String::new(), 0u32), |acc, (c, &v)| if v > acc.1 { (c.clone(), v) } else { acc });
                    return (chunk_of(&best), format!("{}: «{best}» {n} of {total}", t.lemma));
                }
            }
        }
        // MACULA's gloss through the lexicon
        let tag = match t.upos.as_str() {
            "VERB" | "AUX" => Tag::VBD,
            "PROPN" => Tag::NNP,
            "ADJ" => Tag::JJ,
            "ADV" => Tag::RB,
            "ADP" => Tag::IN,
            _ => Tag::NN,
        };
        let words: Vec<String> = gloss_words(&t.gloss).iter().map(|w| lex.concept(&w.to_lowercase()).to_string()).collect();
        let e = words.iter().enumerate().map(|(i, w)| (w.clone(), if i == 0 { tag } else { Tag::NN }, Rel::Dep, if i == 0 { None } else { Some(0) })).collect();
        (e, format!("{}: gloss «{}»", t.lemma, t.gloss))
    }

    /// Read a Hebrew verse into the meta-world, with the reason of each word's chunk.
    pub fn read(&self, lex: &Lexicon, v: &HVerse) -> (Vec<MTok>, Vec<String>) {
        self.read_with(lex, v, self.grammar)
    }

    /// `grammar`: what Hebrew says by its structure and English by words — the construct chain's «of» and «the»,
    /// a possessive suffix as a possessive before its noun, the subject before its verb — read into the meta-world.
    pub fn read_with(&self, lex: &Lexicon, v: &HVerse, grammar: bool) -> (Vec<MTok>, Vec<String>) {
        let mut out: Vec<MTok> = Vec::new();
        let mut why = Vec::new();
        let n = v.toks.len();
        let mut head_of: Vec<Option<usize>> = vec![None; n];
        let mut pending: Vec<(usize, usize)> = Vec::new();
        let (order, pre, poss) = if grammar { grammar_plan(lex, v) } else { ((0..n).collect(), BTreeMap::new(), BTreeMap::new()) };
        let forced = if grammar { idioms(lex, v) } else { BTreeMap::new() };
        // words inserted before a word's chunk hang on that chunk's head (resolved below)
        let mut inserted: Vec<(usize, usize)> = Vec::new();
        for &k in &order {
            let t = &v.toks[k];
            let (mut c, w) = match (poss.get(&k), forced.get(&k)) {
                (Some(p), _) => (vec![(p.clone(), Tag::PRPS, Rel::NmodPoss, None)], format!("{}: possessive suffix «{p}»", t.lemma)),
                (None, Some(f)) => (f.clone(), format!("{}: idiom", t.lemma)),
                (None, None) => self.chunk(lex, t),
            };
            // «the» goes after a chunk's own leading «of» («of the host», not «the of host»)
            let the = lex.concept("the").to_string();
            if pre.get(&k).is_some_and(|p| p.iter().any(|x| x.0 == the)) && c.first().is_some_and(|e| e.0 == lex.concept("of")) && c.len() > 1 {
                let hpos = c.iter().position(|e| e.3.is_none()).unwrap_or(0);
                let hpos = if hpos >= 1 { hpos + 1 } else { hpos };
                for e in c.iter_mut() {
                    if let Some(i) = e.3.as_mut() {
                        if *i >= 1 {
                            *i += 1;
                        }
                    }
                }
                c.insert(1, (the.clone(), Tag::DT, Rel::Det, Some(hpos)));
            }
            // an inserted «of» or «the» the chunk itself (or the word just before) already says is not said twice
            for (x, tag, rel) in pre.get(&k).cloned().unwrap_or_default() {
                if c.first().is_some_and(|e| e.0 == x) || c.get(1).is_some_and(|e| e.0 == x && x == the) || out.last().is_some_and(|m: &MTok| m.concept == x) {
                    continue;
                }
                inserted.push((out.len(), k));
                out.push(MTok { concept: x, tag, head: 0, rel, span: (k, k + 1) });
            }
            why.push(w);
            let base = out.len();
            for (x, tag, rel, inner) in c {
                let head = inner.map_or(0, |i| base + i + 1);
                if inner.is_none() {
                    if head_of[k].is_none() {
                        head_of[k] = Some(out.len());
                    }
                    pending.push((out.len(), k));
                }
                out.push(MTok { concept: x, tag, head, rel, span: (k, k + 1) });
            }
        }
        // chunk heads hang on the chunk of their Hebrew head (climbing past words with no chunk)
        for (m, k) in pending {
            let mut h = v.toks[k].head;
            let mut guard = 0;
            let mut target = 0;
            while h > 0 && guard < v.toks.len() {
                if let Some(x) = head_of[h - 1] {
                    if x != m {
                        target = x + 1;
                    }
                    break;
                }
                h = v.toks[h - 1].head;
                guard += 1;
            }
            out[m].head = target;
        }
        for (m, k) in inserted {
            out[m].head = head_of[k].map_or(0, |x| x + 1);
        }
        // the verse's own start: a mark the first conjunction's chunk brought («, and») goes
        if grammar && out.first().is_some_and(|m| m.concept == ",") {
            out.remove(0);
            for m in out.iter_mut() {
                m.head = match m.head {
                    0 | 1 => 0,
                    h => h - 1,
                };
            }
        }
        (out, why)
    }

    /// The world of a Hebrew verse in meta concepts: its events (the verbs' chunk heads that are verbs) and the
    /// entities MACULA's frames name (A0, A1, A2).
    pub fn world(&self, lex: &Lexicon, v: &HVerse) -> (Vec<String>, Vec<String>) {
        let mid: BTreeMap<&str, usize> = v.toks.iter().enumerate().map(|(i, t)| (t.mid.as_str(), i)).collect();
        let head_concept = |k: usize| -> Option<(String, Tag)> {
            let (c, _) = self.chunk(lex, &v.toks[k]);
            c.into_iter().find(|e| e.3.is_none()).map(|e| (e.0, e.1))
        };
        let (mut ev, mut en) = (Vec::new(), Vec::new());
        let mut seen: BTreeSet<usize> = BTreeSet::new();
        for (k, t) in v.toks.iter().enumerate() {
            if t.upos != "VERB" {
                continue;
            }
            if let Some((c, tag)) = head_concept(k) {
                if tag.is_verb() {
                    ev.push(c);
                }
            }
            for (role, ids) in &t.frame {
                if !matches!(role.as_str(), "A0" | "A1" | "A2") {
                    continue;
                }
                for id in ids {
                    if let Some(&i) = mid.get(id.as_str()) {
                        if seen.insert(i) {
                            if let Some((c, _)) = head_concept(i) {
                                en.push(c);
                            }
                        }
                    }
                }
            }
        }
        (ev, en)
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn gloss_chunks_and_owned_alignment() {
        let lex = Lexicon::default();
        let h = HVerse {
            toks: vec![HTok {
                lemma: "house".into(),
                upos: "NOUN".into(),
                gloss: "a.house".into(),
                strong: Some(1),
                ..Default::default()
            }],
            ..Default::default()
        };
        let empty = Reader::train(&lex, &[], false);
        let (fallback, _) = empty.read(&lex, &h);
        assert_eq!(
            fallback
                .iter()
                .map(|x| x.concept.as_str())
                .collect::<Vec<_>>(),
            ["a", "house"]
        );
        assert_eq!(fallback.iter().map(|x| x.head).collect::<Vec<_>>(), [0, 1]);
        let m = vec![MTok {
            concept: "home".into(),
            tag: Tag::NN,
            head: 0,
            rel: Rel::Root,
            span: (0, 1),
        }];
        let trained = Reader::train(&lex, &[(&h, m.clone(), vec![Some(1)]); 1], false);
        assert_eq!(trained.owners, vec![vec![Some(0)]]);
        assert_eq!((trained.aligned, trained.null, trained.strong), (1, 0, 1));
        assert_eq!(trained.read(&lex, &h).0[0].concept, "home");
        let entries = vec![("a/b".into(), Tag::NN, Rel::Dep, None)];
        assert_eq!(chunk_of(&chunk_key(&entries)), entries);
        assert!(chunk_of("malformed").is_empty());
        assert_eq!(gloss_words("(et).he_created"), ["he", "created"]);
    }
}
