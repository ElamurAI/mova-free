//! Volumes and the first layer, text ↔ tokens.
//!
//! A volume is one file (or one document of a treebank) as a sequence of pieces: literal layout (separators, ids,
//! lesson keys) and text blocks (a paragraph, a line, a treebank sentence). A block is tokenized by `en::tok`; the
//! literal whitespace before every token is kept as a gap, so the block comes back byte for byte. Blocks are cut into
//! sentences at the tokens . ! ? (with closing quotes) followed by a space and a capital or an opening quote — the
//! rule of `prag::frames::split_sentences`, on tokens.

use std::path::Path;

use anyhow::{Context, Result, bail};
use en::annotate::{Annotator, Word};

use crate::class::{Class, Layer, lit_bits, pos_bits};
use crate::tally::Residual;

#[derive(Clone, Debug)]
pub enum Piece {
    /// literal text that is not language (class `Meta`) or layout between blocks (class `Layout`)
    Lit(String, Class),
    Text(usize),
}

#[derive(Clone, Debug)]
pub struct Block {
    pub text: String,
    /// a treebank's tree of the block (a whole sentence), if any
    pub gold: Option<Vec<Word>>,
    /// one sentence, never cut (a treebank sentence)
    pub whole: bool,
    /// inline markup inside the block's text (a page's link targets, emphasis marks, escapes): at a byte offset of
    /// `text`, the literal written before it; `text` alone goes up the layers, the marks are layout (`Class::Markup`)
    pub marks: Vec<(usize, String)>,
}

impl Block {
    /// The block as written: its text with the inline marks put back.
    pub fn written(&self) -> String {
        with_marks(&self.text, &self.marks)
    }
}

/// A text with inline marks put back at their byte offsets (in order; several at one offset in their order).
pub fn with_marks(text: &str, marks: &[(usize, String)]) -> String {
    if marks.is_empty() {
        return text.to_string();
    }
    let mut s = String::with_capacity(text.len() + marks.iter().map(|m| m.1.len()).sum::<usize>());
    let mut at = 0;
    for (k, m) in marks {
        let k = (*k).min(text.len());
        if k > at {
            s.push_str(&text[at..k]);
            at = k;
        }
        s.push_str(m);
    }
    s.push_str(&text[at..]);
    s
}

#[derive(Clone, Debug)]
pub struct Volume {
    pub name: String,
    pub pieces: Vec<Piece>,
    pub blocks: Vec<Block>,
    /// the separator between blocks that costs nothing (a decoder writes it by itself)
    pub canon_sep: String,
}

impl Volume {
    pub fn text(&self) -> String {
        let mut s = String::new();
        for p in &self.pieces {
            match p {
                Piece::Lit(x, _) => s.push_str(x),
                Piece::Text(b) => s.push_str(&self.blocks[*b].written()),
            }
        }
        s
    }
}

fn lines_keep(text: &str) -> Vec<&str> {
    text.split_inclusive('\n').collect()
}

/// A plain-text book: paragraphs (runs of non-blank lines) are blocks; the blank lines between them are layout.
pub fn read_txt(path: &Path) -> Result<Volume> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    Ok(txt_volume(&path.display().to_string(), &text))
}

pub fn txt_volume(name: &str, text: &str) -> Volume {
    let mut v = Volume { name: name.to_string(), pieces: Vec::new(), blocks: Vec::new(), canon_sep: "\n\n".into() };
    let mut lit = String::new();
    let mut para = String::new();
    let flush_para = |v: &mut Volume, para: &mut String, lit: &mut String| {
        if para.is_empty() {
            return;
        }
        // the paragraph's last line break goes to the layout after it
        let body = para.strip_suffix('\n').map(|s| s.strip_suffix('\r').unwrap_or(s)).unwrap_or(para.as_str()).to_string();
        let tail = para[body.len()..].to_string();
        v.blocks.push(Block { text: body, gold: None, whole: false, marks: Vec::new() });
        v.pieces.push(Piece::Text(v.blocks.len() - 1));
        lit.push_str(&tail);
        para.clear();
    };
    for line in lines_keep(text) {
        if line.trim().is_empty() {
            flush_para(&mut v, &mut para, &mut lit);
            lit.push_str(line);
        } else {
            if !lit.is_empty() {
                v.pieces.push(Piece::Lit(std::mem::take(&mut lit), Class::Layout));
            }
            para.push_str(line);
        }
    }
    flush_para(&mut v, &mut para, &mut lit);
    if !lit.is_empty() {
        v.pieces.push(Piece::Lit(lit, Class::Layout));
    }
    v
}

/// `id<TAB>text` lines: the id is metadata, the text a block.
pub fn read_tsv(path: &Path) -> Result<Volume> {
    let text = std::fs::read_to_string(path).with_context(|| format!("{}", path.display()))?;
    let mut v = Volume { name: path.display().to_string(), pieces: Vec::new(), blocks: Vec::new(), canon_sep: "\n".into() };
    for line in lines_keep(&text) {
        let body = line.strip_suffix('\n').unwrap_or(line);
        let nl = &line[body.len()..];
        match body.rfind('\t') {
            Some(t) => {
                v.pieces.push(Piece::Lit(body[..=t].to_string(), Class::Meta));
                v.blocks.push(Block { text: body[t + 1..].to_string(), gold: None, whole: false, marks: Vec::new() });
                v.pieces.push(Piece::Text(v.blocks.len() - 1));
            }
            None => v.pieces.push(Piece::Lit(body.to_string(), Class::Meta)),
        }
        if !nl.is_empty() {
            v.pieces.push(Piece::Lit(nl.to_string(), Class::Layout));
        }
    }
    Ok(v)
}

/// The keys of a lesson whose values are text.
const TEXT_KEYS: [&str; 4] = ["title", "explain", "fact", "exercise"];

/// A school lesson: the values of `title:`, `explain:`, `fact:`, `exercise:` are blocks; keys and other lines are metadata.
pub fn lesson_volume(name: &str, text: &str) -> Volume {
    let mut v = Volume { name: name.to_string(), pieces: Vec::new(), blocks: Vec::new(), canon_sep: "\n".into() };
    for line in lines_keep(text) {
        let body = line.strip_suffix('\n').unwrap_or(line);
        let nl = &line[body.len()..];
        let key = body.split_once(": ").filter(|(k, _)| TEXT_KEYS.contains(k));
        match key {
            Some((k, val)) => {
                v.pieces.push(Piece::Lit(format!("{k}: "), Class::Meta));
                v.blocks.push(Block { text: val.to_string(), gold: None, whole: false, marks: Vec::new() });
                v.pieces.push(Piece::Text(v.blocks.len() - 1));
            }
            None => v.pieces.push(Piece::Lit(body.to_string(), Class::Meta)),
        }
        if !nl.is_empty() {
            v.pieces.push(Piece::Lit(nl.to_string(), Class::Layout));
        }
    }
    v
}

/// Every `stride`-th lesson of a school directory (`<course>/*.lesson`, sorted).
pub fn read_lessons(dir: &Path, stride: usize) -> Result<Vec<Volume>> {
    let mut files = Vec::new();
    let mut courses: Vec<_> = std::fs::read_dir(dir)?.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.is_dir()).collect();
    courses.sort();
    for c in courses {
        let mut ls: Vec<_> = std::fs::read_dir(&c)?.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "lesson")).collect();
        ls.sort();
        files.extend(ls);
    }
    let mut out = Vec::new();
    for (i, f) in files.iter().enumerate() {
        if i % stride.max(1) != 0 {
            continue;
        }
        let text = std::fs::read_to_string(f).with_context(|| format!("{}", f.display()))?;
        let name = f.strip_prefix(dir).unwrap_or(f).display().to_string();
        out.push(lesson_volume(&name, &text));
    }
    Ok(out)
}

/// A treebank: the `# text` of every sentence is a block with its tree; one volume per document (the `sent_id`
/// without its last field), every `stride`-th document.
pub fn read_conllu(path: &Path, stride: usize) -> Result<Vec<Volume>> {
    let sents = en::conllu::read(path)?;
    // documents by the sent_id without its last field, their sentences by the number in that field
    let mut by: std::collections::BTreeMap<String, Vec<(u64, en::conllu::Sentence)>> = std::collections::BTreeMap::new();
    for s in sents {
        let (doc, k) = s.id.rsplit_once(':').map(|(d, k)| (d.to_string(), k.parse::<u64>().unwrap_or(0))).unwrap_or_default();
        by.entry(doc).or_default().push((k, s));
    }
    let docs: Vec<(String, Vec<en::conllu::Sentence>)> = by
        .into_iter()
        .map(|(d, mut v)| {
            v.sort_by_key(|x| x.0);
            (d, v.into_iter().map(|x| x.1).collect())
        })
        .collect();
    let mut out = Vec::new();
    for (i, (doc, ss)) in docs.into_iter().enumerate() {
        if i % stride.max(1) != 0 {
            continue;
        }
        let mut v = Volume { name: doc, pieces: Vec::new(), blocks: Vec::new(), canon_sep: "\n".into() };
        for s in ss {
            if s.text.is_empty() {
                continue;
            }
            let ok = s.tokens.iter().all(|t| t.tag.is_some() && t.upos.is_some());
            let gold = ok.then(|| {
                s.tokens
                    .iter()
                    .map(|t| Word { form: t.form.clone(), lemma: t.lemma.clone(), upos: t.upos.unwrap(), tag: t.tag.unwrap(), feats: t.feats, head: t.head, rel: t.rel })
                    .collect()
            });
            v.blocks.push(Block { text: s.text.clone(), gold, whole: true, marks: Vec::new() });
            v.pieces.push(Piece::Text(v.blocks.len() - 1));
            v.pieces.push(Piece::Lit("\n".into(), Class::Layout));
        }
        out.push(v);
    }
    Ok(out)
}

/// A sentence at the token layer.
#[derive(Clone, Debug, Default)]
pub struct Sent {
    pub forms: Vec<String>,
    /// the literal before each token (gaps[0] — before the sentence)
    pub gaps: Vec<String>,
    pub block: usize,
    pub gold: Option<Vec<Word>>,
}

impl Sent {
    /// The space-after bits (the token layer's own representation): a non-empty gap before the next token.
    pub fn space(&self) -> Vec<bool> {
        (0..self.forms.len()).map(|k| self.gaps.get(k + 1).is_some_and(|g| !g.is_empty())).collect()
    }
    /// The sentence as written (without the gap before it).
    pub fn original(&self) -> String {
        let mut s = String::new();
        for (k, f) in self.forms.iter().enumerate() {
            if k > 0 {
                s.push_str(&self.gaps[k]);
            }
            s.push_str(f);
        }
        s
    }
}

/// A block cut into sentences, with the literal after its last token.
#[derive(Clone, Debug, Default)]
pub struct Seg {
    pub sents: Vec<Sent>,
    pub tail: String,
}

fn terminal(f: &str) -> bool {
    !f.is_empty() && f.chars().all(|c| matches!(c, '.' | '!' | '?'))
}

fn closer(f: &str) -> bool {
    matches!(f, "'" | "\"" | "\u{2019}" | "\u{201d}" | ")" | "]" | "''")
}

fn opener(f: &str) -> bool {
    matches!(f, "'" | "\"" | "\u{2018}" | "\u{201c}" | "(" | "[" | "``")
}

/// Tokens of a block with their gaps, cut into sentences.
pub fn segment(a: &Annotator, text: &str, block: usize, whole: bool, gold: Option<&Vec<Word>>) -> Result<Seg> {
    let toks = a.tokenize(text);
    let mut forms = Vec::with_capacity(toks.len());
    let mut gaps = Vec::with_capacity(toks.len());
    let mut pos = 0;
    for t in &toks {
        let rest = &text[pos..];
        let start = match rest.find(t.form.as_str()) {
            Some(s) => s,
            None => bail!("token «{}» not found in the text after byte {pos}: «{}»", t.form, rest.chars().take(60).collect::<String>()),
        };
        gaps.push(rest[..start].to_string());
        forms.push(t.form.clone());
        pos += start + t.form.len();
    }
    let tail = text[pos..].to_string();
    let mut seg = Seg { sents: Vec::new(), tail };
    let space: Vec<bool> = (0..forms.len()).map(|k| gaps.get(k + 1).is_some_and(|g| !g.is_empty())).collect();
    let lens = if whole { vec![forms.len()] } else { cut(&forms, &space) };
    let mut k = 0;
    for n in lens {
        if n == 0 {
            continue;
        }
        seg.sents.push(Sent { forms: forms[k..k + n].to_vec(), gaps: gaps[k..k + n].to_vec(), block, gold: None });
        k += n;
    }
    if whole {
        if let (Some(g), Some(s)) = (gold, seg.sents.first_mut()) {
            if g.len() == s.forms.len() && g.iter().zip(&s.forms).all(|(w, f)| &w.form == f) {
                s.gold = Some(g.clone());
            }
        }
    }
    Ok(seg)
}

/// Sentence lengths of a run of tokens (forms, space-after bits): a sentence ends at . ! ? (with closing quotes
/// after it) when the next token follows a space and starts with a capital or an opening quote.
pub fn cut(forms: &[String], space: &[bool]) -> Vec<usize> {
    let mut out = Vec::new();
    let mut start = 0;
    for k in 0..forms.len() {
        let mut j = k;
        while j > 0 && closer(&forms[j]) && !terminal(&forms[j]) {
            j -= 1;
        }
        let end = terminal(&forms[j]) && j + 3 > k;
        let next_ok = forms.get(k + 1).is_some_and(|n| n.chars().next().is_some_and(char::is_uppercase) || opener(n)) && space.get(k).copied().unwrap_or(false);
        if end && next_ok {
            out.push(k + 1 - start);
            start = k + 1;
        }
    }
    if start < forms.len() {
        out.push(forms.len() - start);
    }
    out
}

/// The class of a gap literal the canonical decoder would not write.
fn gap_class(g: &str) -> Class {
    if g.contains('\n') { Class::Layout } else { Class::Spacing }
}

/// The token layer's residual: the literal gaps the canonical decoder would not write, by (block, sentence in the
/// block, token), and the blocks' tails. The layout pieces of the volume are residual too (counted, kept in `Volume`).
#[derive(Clone, Debug, Default)]
pub struct R1 {
    pub gaps: std::collections::BTreeMap<(usize, usize, usize), String>,
    pub tails: std::collections::BTreeMap<usize, String>,
}

/// Exact decode of a volume's blocks from the sentences' tokens (forms, space bits) and the residual.
pub fn decode_blocks(nblocks: usize, sents: &[Vec<(Vec<String>, Vec<bool>)>], r: &R1) -> Vec<String> {
    (0..nblocks)
        .map(|b| {
            let mut s = String::new();
            for (si, (forms, space)) in sents[b].iter().enumerate() {
                for (k, f) in forms.iter().enumerate() {
                    let canon = if k == 0 { if si == 0 { "" } else { " " } } else if space[k - 1] { " " } else { "" };
                    s.push_str(r.gaps.get(&(b, si, k)).map_or(canon, String::as_str));
                    s.push_str(f);
                }
            }
            if let Some(t) = r.tails.get(&b) {
                s.push_str(t);
            }
            s
        })
        .collect()
}

/// The markup of a block's pages written from the markup before it (a page repeats its siblings' front matter, links
/// and markers): each literal is a copy of an earlier one with its common prefix and suffix kept and the middle
/// stored, or stored whole when that is cheaper. Bits: a flag (copy or whole); the earlier literal — one bit when it is
/// the one after the literal the last copy took (a page copied line after line from the page before), else a bit and
/// its number; the edit — one bit when the copy is the same, else a bit, two lengths and the middle's bytes. The
/// candidates are the last `RECENT` literals, the one after the last copy's, and the last ones that begin or end with
/// the same `KEY` bytes. Decodable in the order the literals are coded; `Copier::decode` is the decoder, checked on
/// every literal.
#[derive(Clone, Debug, Default)]
pub struct Copier {
    hist: Vec<String>,
    by_head: std::collections::BTreeMap<Vec<u8>, usize>,
    by_tail: std::collections::BTreeMap<Vec<u8>, usize>,
    /// the literal the last copy took
    last: Option<usize>,
    /// literals, copied with nothing stored, bits stored whole, bits of the code
    pub pieces: u64,
    pub exact: u64,
    pub literal_bits: f64,
    pub code_bits: f64,
}

/// A literal's code: a copy of literal `from` with `pre` bytes of its start, `suf` of its end and `mid` between, or
/// stored whole.
#[derive(Clone, Debug, PartialEq)]
pub enum Copy {
    Whole(String),
    From { from: usize, pre: usize, suf: usize, mid: String },
}

const RECENT: usize = 48;
const KEY: usize = 10;

fn head(s: &str) -> Vec<u8> {
    s.as_bytes()[..s.len().min(KEY)].to_vec()
}

fn tail(s: &str) -> Vec<u8> {
    s.as_bytes()[s.len().saturating_sub(KEY)..].to_vec()
}

impl Copier {
    /// A copier that starts with a dictionary of literals (learned from training pages): they can be copied, they are
    /// not counted.
    pub fn seeded(dict: &[String]) -> Copier {
        let mut c = Copier::default();
        for x in dict {
            c.push(x);
        }
        c
    }

    /// The bits of a literal, coded against the literals before it; the literal joins them.
    pub fn code(&mut self, x: &str) -> (f64, Copy) {
        let whole = lit_bits(x);
        let mut best = (1.0 + whole, Copy::Whole(x.to_string()));
        let n = self.hist.len();
        let next = self.last.map(|j| j + 1).filter(|&j| j < n);
        let mut cands: Vec<usize> = next.into_iter().collect();
        cands.extend((n.saturating_sub(RECENT)..n).rev());
        cands.extend(self.by_head.get(&head(x)).copied());
        cands.extend(self.by_tail.get(&tail(x)).copied());
        for &j in &cands {
            let y = &self.hist[j];
            let pre = x.bytes().zip(y.bytes()).take_while(|(a, b)| a == b).count();
            let pre = (0..=pre).rev().find(|&k| x.is_char_boundary(k) && y.is_char_boundary(k)).unwrap_or(0);
            let room = x.len().min(y.len()) - pre;
            let suf = x.bytes().rev().zip(y.bytes().rev()).take(room).take_while(|(a, b)| a == b).count();
            let suf = (0..=suf).rev().find(|&k| x.is_char_boundary(x.len() - k) && y.is_char_boundary(y.len() - k)).unwrap_or(0);
            let mid = &x[pre..x.len() - suf];
            let refb = if Some(j) == next { 1.0 } else { 1.0 + pos_bits(n) };
            let edit = if x == y { 1.0 } else { 1.0 + 2.0 * pos_bits(y.len()) + lit_bits(mid) };
            let bits = 1.0 + refb + edit;
            if bits < best.0 {
                best = (bits, Copy::From { from: j, pre, suf, mid: mid.to_string() });
            }
        }
        debug_assert_eq!(self.decode(&best.1), x);
        if self.decode(&best.1) != x {
            best = (1.0 + whole, Copy::Whole(x.to_string()));
        }
        if let Copy::From { from, .. } = &best.1 {
            self.last = Some(*from);
        }
        self.pieces += 1;
        self.exact += matches!(&best.1, Copy::From { mid, .. } if mid.is_empty()) as u64;
        self.literal_bits += whole;
        self.code_bits += best.0;
        self.push(x);
        best
    }
    /// A literal joins the history without being coded (prose that went up the layers, for later copies).
    pub fn remember(&mut self, x: &str) {
        self.push(x);
    }
    fn push(&mut self, x: &str) {
        let k = self.hist.len();
        self.by_head.insert(head(x), k);
        self.by_tail.insert(tail(x), k);
        self.hist.push(x.to_string());
    }
    /// The literal of a code, from the literals before it.
    pub fn decode(&self, c: &Copy) -> String {
        match c {
            Copy::Whole(s) => s.clone(),
            Copy::From { from, pre, suf, mid } => {
                let y = &self.hist[*from];
                format!("{}{mid}{}", &y[..*pre], &y[y.len() - suf..])
            }
        }
    }
}

/// The token layer's residual of a volume: every literal piece that is not the canonical separator, every gap that is
/// not "" or " ", a gap before a sentence that is not the canonical one (nothing at a block start, one space after a
/// sentence), and a block's tail.
pub fn residual(v: &Volume, segs: &[Seg], res: &mut Residual) -> R1 {
    residual_with(v, segs, res, None)
}

/// The same, with the markup (`Class::Markup` pieces and inline marks) coded by a copier when one is given.
pub fn residual_with(v: &Volume, segs: &[Seg], res: &mut Residual, mut copier: Option<&mut Copier>) -> R1 {
    let mut r1 = R1::default();
    // prose a page repeats is copied from the prose before it (`Class::Repeat`)
    let mut prose = Copier::default();
    let has_repeat = v.pieces.iter().any(|p| matches!(p, Piece::Lit(_, Class::Repeat)));
    for p in &v.pieces {
        if has_repeat {
            match p {
                Piece::Text(b) => prose.remember(&v.blocks[*b].written()),
                Piece::Lit(x, Class::Repeat) => {
                    let (bits, code) = prose.code(x);
                    let x2 = x.clone();
                    res.add_k(Layer::Tokens, Class::Repeat, bits, if matches!(code, Copy::Whole(_)) { "stored".into() } else { "copied".into() }, || format!("{:?}", x2.chars().take(60).collect::<String>()));
                    continue;
                }
                _ => {}
            }
        }
        if let Piece::Lit(x, c) = p {
            // a page's CoNLL-U and frames table are rebuilt from the layers: their residual is the site decoder's
            if matches!(c, Class::Conllu | Class::FrTable | Class::Given) {
                continue;
            }
            if let (Class::Markup, Some(cp)) = (c, copier.as_deref_mut()) {
                let (bits, code) = cp.code(x);
                let x2 = x.clone();
                res.add_k(Layer::Tokens, Class::Markup, bits, if matches!(code, Copy::Whole(_)) { "stored".into() } else { "copied".into() }, || format!("{:?}", x2.chars().take(50).collect::<String>()));
                continue;
            }
            if *c == Class::Meta || *x != v.canon_sep && x != "\n" {
                let x2 = x.clone();
                res.add(Layer::Tokens, *c, lit_bits(x), || format!("{:?}", x2.chars().take(50).collect::<String>()));
            }
        }
    }
    for (b, seg) in segs.iter().enumerate() {
        let ntok: usize = seg.sents.iter().map(|s| s.forms.len()).sum();
        for (_, m) in &v.blocks[b].marks {
            let m2 = m.clone();
            let (bits, kind) = match copier.as_deref_mut() {
                Some(cp) => {
                    let (b, code) = cp.code(m);
                    (b, if matches!(code, Copy::Whole(_)) { "inline, stored" } else { "inline, copied" })
                }
                None => (lit_bits(m), "inline"),
            };
            res.add_k(Layer::Tokens, Class::Markup, bits + pos_bits(ntok + 1), kind.into(), || format!("inline {:?}", m2.chars().take(50).collect::<String>()));
        }
        for (si, s) in seg.sents.iter().enumerate() {
            let canon0 = if si == 0 { "" } else { " " };
            if s.gaps[0] != canon0 {
                let g = s.gaps[0].clone();
                r1.gaps.insert((b, si, 0), g.clone());
                res.add(Layer::Tokens, gap_class(&g), lit_bits(&g) + pos_bits(ntok), || format!("before «{}»: {:?}", s.forms[0], g));
            }
            for k in 1..s.forms.len() {
                let g = &s.gaps[k];
                if !g.is_empty() && g != " " {
                    let g2 = g.clone();
                    r1.gaps.insert((b, si, k), g.clone());
                    res.add(Layer::Tokens, gap_class(g), lit_bits(g) + pos_bits(ntok), || format!("«{}»{:?}«{}»", s.forms[k - 1], g2, s.forms[k]));
                }
            }
        }
        if !seg.tail.is_empty() {
            let t = seg.tail.clone();
            r1.tails.insert(b, t.clone());
            res.add(Layer::Tokens, gap_class(&t), lit_bits(&t), || format!("block tail {t:?}"));
        }
    }
    r1
}

/// Canonical decode of a sentence's tokens: one space where the space-after bit is set.
pub fn join(forms: &[String], space: &[bool]) -> String {
    let mut s = String::new();
    for (k, f) in forms.iter().enumerate() {
        s.push_str(f);
        if k + 1 < forms.len() && space[k] {
            s.push(' ');
        }
    }
    s
}

/// Exact decode of a volume from its pieces and decoded blocks.
pub fn volume_text(v: &Volume, blocks: &[String]) -> String {
    let mut s = String::new();
    for p in &v.pieces {
        match p {
            Piece::Lit(x, _) => s.push_str(x),
            Piece::Text(b) => s.push_str(&with_marks(&blocks[*b], &v.blocks[*b].marks)),
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn txt_pieces_give_the_text_back() {
        let t = "CHAPTER I.\n\nAlice was\nbeginning to get tired.\n\n\n  So she\n";
        let v = txt_volume("t", t);
        assert_eq!(v.text(), t);
        assert_eq!(v.blocks.len(), 3);
        assert_eq!(v.blocks[1].text, "Alice was\nbeginning to get tired.");
        let l = lesson_volume("l", "course: x\nfact: Cats purr.\nexplain: Dogs bark.");
        assert_eq!(l.text(), "course: x\nfact: Cats purr.\nexplain: Dogs bark.");
        assert_eq!(l.blocks.len(), 2);
    }
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn copier_unicode_and_gap_reconstruction() {
        let mut cp = Copier::default();
        for s in [
            "",
            "α long literal with a suffix",
            "α long literal with a suffix",
            "β long literal with a suffix",
            "short",
        ] {
            let (bits, code) = cp.code(s);
            assert_eq!(cp.decode(&code), s);
            assert!(bits.is_finite() && bits <= 1.0 + lit_bits(s));
        }
        assert_eq!(cp.pieces, 5);
        let v = txt_volume("x", "  Alpha\tβ!\n");
        let seg = Seg {
            sents: vec![Sent {
                forms: vec!["Alpha".into(), "β!".into()],
                gaps: vec!["  ".into(), "\t".into()],
                ..Default::default()
            }],
            tail: String::new(),
        };
        let mut res = Residual::default();
        let r = residual(&v, &[seg.clone()], &mut res);
        let decoded = decode_blocks(
            1,
            &[vec![(seg.sents[0].forms.clone(), seg.sents[0].space())]],
            &r,
        );
        assert_eq!(volume_text(&v, &decoded), "  Alpha\tβ!\n");
        assert_eq!(
            with_marks("αβ", &[(0, "[".into()), (4, "]".into())]),
            "[αβ]"
        );
        let forms = ["A", ".", "\"", "B"].map(String::from);
        assert_eq!(cut(&forms, &[false, false, true, false]), [3, 1]);
    }
}
