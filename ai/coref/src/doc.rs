//! CoNLL-U documents for coreference: sentences with a tree (children, depth, subtree bounds), speaker and addressee
//! (`# speaker`, `# addressee`), quotes (quotation number for each word) and the raw MISC `Entity=`.
//!
//! `Entity=` is gold; only the scorer reads it (`corefud::read_mentions`). The resolver (`sieve`) sees
//! only the tree, forms, FEATS and speaker comments — see the test `resolver_ignores_gold_entities`.

use anyhow::{Result, bail};
use en::conllu::{Col, Doc as UdDoc, Token};
use en::gram::{Feat, Rel, UPos};

/// A sentence of a document.
#[derive(Clone, Debug, Default)]
pub struct Sent {
    /// Sentence number in the file (index into `UdDoc::sents`).
    pub file_idx: usize,
    pub id: String,
    pub toks: Vec<Token>,
    /// Children of each word (0-based), in word order.
    pub kids: Vec<Vec<usize>>,
    /// Word depth: the root is 0.
    pub depth: Vec<u32>,
    /// Subtree bounds of each word (0-based, inclusive).
    pub sub: Vec<(usize, usize)>,
    pub speaker: Option<String>,
    pub addressee: Option<String>,
    /// Quotation number for each word: 0 means outside quotes.
    pub quote: Vec<u32>,
    /// Raw `Entity=` value from the MISC of each word (gold).
    pub entity: Vec<Option<String>>,
    /// A new paragraph starts before the sentence (`# newpar`).
    pub newpar: bool,
}

impl Sent {
    pub fn len(&self) -> usize {
        self.toks.len()
    }
    pub fn is_empty(&self) -> bool {
        self.toks.is_empty()
    }
    /// Head of word `i` (0-based); `None` for the root.
    pub fn parent(&self, i: usize) -> Option<usize> {
        match self.toks[i].head {
            0 => None,
            h => Some(h - 1),
        }
    }
    pub fn upos(&self, i: usize) -> Option<UPos> {
        self.toks[i].upos
    }
    pub fn rel(&self, i: usize) -> Rel {
        self.toks[i].rel
    }
    pub fn has(&self, i: usize, f: Feat) -> bool {
        self.toks[i].feats.has(f)
    }
    /// Lowercased form.
    pub fn low(&self, i: usize) -> String {
        self.toks[i].form.to_lowercase()
    }
    /// Lowercased lemma (the form if there is no lemma).
    pub fn lem(&self, i: usize) -> String {
        let l = &self.toks[i].lemma;
        if l.is_empty() || l == "_" { self.low(i) } else { l.to_lowercase() }
    }
    /// `a` is an ancestor of `b` or `b` itself.
    pub fn dominates(&self, a: usize, b: usize) -> bool {
        let mut x = Some(b);
        let mut guard = 0;
        while let Some(i) = x {
            if i == a {
                return true;
            }
            x = self.parent(i);
            guard += 1;
            if guard > self.toks.len() {
                return false;
            }
        }
        false
    }
    /// Text of the word span [a, b] with spacing as in the text.
    pub fn text(&self, a: usize, b: usize) -> String {
        let mut s = String::new();
        for i in a..=b {
            s.push_str(&self.toks[i].form);
            if i < b && self.toks[i].space_after {
                s.push(' ');
            }
        }
        s
    }
    /// Children of word `i` with the given relation (without subtype).
    pub fn kids_rel(&self, i: usize, r: Rel) -> impl Iterator<Item = usize> + '_ {
        self.kids[i].iter().copied().filter(move |&k| self.toks[k].rel.base() == r)
    }
}

/// A document (`# newdoc id = …`).
#[derive(Clone, Debug, Default)]
pub struct Document {
    pub id: String,
    pub sents: Vec<Sent>,
}

fn build_sent(ud: &UdDoc, k: usize) -> Result<Sent> {
    let fs = &ud.sents[k];
    let toks = fs.sent.tokens.clone();
    let n = toks.len();
    let mut kids = vec![Vec::new(); n];
    for (i, t) in toks.iter().enumerate() {
        if t.head > n {
            bail!("{}: word {} has head {} outside the sentence", fs.label(), i + 1, t.head);
        }
        if t.head > 0 {
            kids[t.head - 1].push(i);
        }
    }
    let mut depth = vec![0u32; n];
    for i in 0..n {
        let (mut d, mut x) = (0u32, toks[i].head);
        while x != 0 {
            d += 1;
            if d as usize > n {
                bail!("{}: cycle in the tree near word {}", fs.label(), i + 1);
            }
            x = toks[x - 1].head;
        }
        depth[i] = d;
    }
    // subtree bounds: from the deepest words up to the root
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(depth[i]));
    let mut sub: Vec<(usize, usize)> = (0..n).map(|i| (i, i)).collect();
    for &i in &order {
        if toks[i].head > 0 {
            let p = toks[i].head - 1;
            sub[p].0 = sub[p].0.min(sub[i].0);
            sub[p].1 = sub[p].1.max(sub[i].1);
        }
    }
    let mut speaker = None;
    let mut addressee = None;
    let mut newpar = false;
    for c in &fs.comments {
        if let Some(v) = c.strip_prefix("# speaker = ") {
            speaker = Some(v.trim().to_string());
        } else if let Some(v) = c.strip_prefix("# addressee = ") {
            addressee = Some(v.trim().to_string());
        } else if c.starts_with("# newpar") && !c.starts_with("# newpar_") {
            newpar = true;
        } else if c.starts_with("# newdoc") {
            newpar = true;
        }
    }
    let mut entity = Vec::with_capacity(n);
    for (ri, row) in fs.rows.iter().enumerate() {
        let misc = row.get(Col::Misc);
        let e = misc.split('|').find_map(|p| p.strip_prefix("Entity=")).map(str::to_string);
        if row.is_word() {
            entity.push(e);
        } else if e.is_some() {
            bail!("{}: Entity= on line {} (multiword token or empty node) — not supported", fs.label(), ri + 1);
        }
    }
    Ok(Sent { file_idx: k, id: fs.sent.id.clone(), toks, kids, depth, sub, speaker, addressee, quote: vec![0; n], entity, newpar })
}

/// Quotes: `"` toggles, `“` opens, `”` closes; the state resets at a new paragraph.
fn mark_quotes(sents: &mut [Sent]) {
    let mut open = false;
    let mut qid = 0u32;
    for s in sents.iter_mut() {
        if s.newpar {
            open = false;
        }
        for i in 0..s.toks.len() {
            let f = s.toks[i].form.as_str();
            let (opens, closes) = match f {
                "\"" | "``" | "''" => (!open, open),
                "“" => (true, false),
                "”" => (false, true),
                _ => (false, false),
            };
            if opens && !open {
                open = true;
                qid += 1;
                s.quote[i] = qid;
                continue;
            }
            if closes && open {
                s.quote[i] = qid;
                open = false;
                continue;
            }
            s.quote[i] = if open { qid } else { 0 };
        }
    }
}

/// Documents of a file: split at `# newdoc`; a file without newdoc is one document.
pub fn documents(ud: &UdDoc) -> Result<Vec<Document>> {
    let mut out: Vec<Document> = Vec::new();
    for k in 0..ud.sents.len() {
        let fs = &ud.sents[k];
        if fs.sent.tokens.is_empty() {
            continue;
        }
        let nd = fs.comments.iter().find_map(|c| c.strip_prefix("# newdoc id = ").or_else(|| c.strip_prefix("# newdoc id=")));
        let is_new = fs.comments.iter().any(|c| c.starts_with("# newdoc"));
        if is_new || out.is_empty() {
            out.push(Document { id: nd.map(str::to_string).unwrap_or_else(|| format!("doc{}", out.len() + 1)), sents: Vec::new() });
        }
        let s = build_sent(ud, k)?;
        out.last_mut().unwrap().sents.push(s);
    }
    for d in &mut out {
        mark_quotes(&mut d.sents);
    }
    Ok(out)
}

/// Trees from another file (output of `en annotate` on the same tokens): columns LEMMA, UPOS, XPOS, FEATS,
/// HEAD, DEPREL are taken from `trees`, the rest (comments, MISC with Entity) from `gold`. Sentences and words must
/// match in order and form, otherwise it is an error (fail-fast).
pub fn retree(gold: &UdDoc, trees: &UdDoc) -> Result<UdDoc> {
    let gs: Vec<usize> = (0..gold.sents.len()).filter(|&k| !gold.sents[k].sent.tokens.is_empty()).collect();
    let ts: Vec<usize> = (0..trees.sents.len()).filter(|&k| !trees.sents[k].sent.tokens.is_empty()).collect();
    if gs.len() != ts.len() {
        bail!("sentences: gold {}, trees {}", gs.len(), ts.len());
    }
    let mut out = gold.clone();
    for (&g, &t) in gs.iter().zip(&ts) {
        let (gf, tf) = (&gold.sents[g], &trees.sents[t]);
        if gf.words.len() != tf.words.len() {
            bail!("{}: words in gold {}, in trees {}", gf.label(), gf.words.len(), tf.words.len());
        }
        for w in 0..gf.words.len() {
            let (gr, tr) = (gf.word(w), tf.word(w));
            if gr.get(Col::Form) != tr.get(Col::Form) {
                bail!("{}: word {} — \"{}\" in gold, \"{}\" in trees", gf.label(), w + 1, gr.get(Col::Form), tr.get(Col::Form));
            }
            let o = out.sents[g].word_mut(w);
            for c in [Col::Lemma, Col::Upos, Col::Xpos, Col::Feats, Col::Head, Col::Deprel] {
                o.set(c, tr.get(c));
            }
            o.set(Col::Deps, "_");
        }
        out.sents[g].refresh()?;
    }
    Ok(out)
}
