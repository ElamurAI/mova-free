//! The site decoder of a page's derived pieces (`site_md`): its ```` ```conllu ```` blocks and its frames tables
//! are written from the layers, not stored.
//!
//! - **CoNLL-U.** The `# text = S` line points to the prose block that holds S (or S is stored); the rows are the
//!   `en` annotation of S (form, lemma, UPOS, PTB tag, FEATS, head, relation, `SpaceAfter=No` from S's spacing),
//!   and only the fields where the page's tree differs are stored: a field patch per word (lemma 17 bits or its
//!   string, UPOS 5, tag 6, a FEATS pair 7, head log2(n+1), relation 6, MISC its string, and log2(n+1) to name the
//!   word). Other comment lines (`# sent_id`) are stored. A CoNLL-U whose words are not `en`'s tokens of S, or one
//!   right to left (Hebrew, MACULA's), is stored whole.
//! - **The frames table.** `prag::frames_md::table` of the frames `prag` reads from the page's tree above it, with
//!   one discourse state carried through the block's trees in order; a bit per table says whether the state starts
//!   afresh at its tree (the page builder reads a chapter, a document or a text with one state), a bit whether the
//!   note under the table is left out (a page gives it once). What still differs is a line script (lines dropped,
//!   lines stored, each with its position). A table of another kind (the Hebrew verbs' frames) is stored whole.
//!
//! The decoder reads only the codes, the block's prose and the `en` model: the gate compares what it writes with
//! the page byte for byte. The residual is counted in `Class::Conllu` (tree layer) and `Class::FrTable` (frames
//! layer); `Tally::derived` keeps, per kind of piece, how many there are, how many need no stored byte, and the bits
//! of storing them whole against the bits of the code.

use std::collections::{BTreeMap, BTreeSet};

use en::annotate::{Annotator, Word};
use en::gram::{Feats, Rel, Tag, UPos};
use prag::frames::{Ctx, Frame, Tree, frames_of_tree};

use crate::class::{Class, Layer, lemma_bits, lit_bits, pos_bits};
use crate::tally::{Derived, Tally};
use crate::vol::{Copier, Piece, Volume};
use super::site_heb;

/// Whether a volume is a site block: it has markup or pieces the site decoder writes.
pub fn is_site(v: &Volume) -> bool {
    v.pieces.iter().any(|p| matches!(p, Piece::Lit(_, Class::Conllu | Class::FrTable | Class::Markup | Class::Given | Class::Repeat))) || v.blocks.iter().any(|b| !b.marks.is_empty())
}

const FENCE: &str = "```conllu\n";
const CLOSE: &str = "```\n";
const FIELDS: [&str; 10] = ["id", "form", "lemma", "upos", "xpos", "feats", "head", "deprel", "deps", "misc"];

/// Where the sentence of a CoNLL-U is: a prose block, whole or a span of it, or stored.
#[derive(Clone, Debug, PartialEq)]
enum SRef {
    Block(usize),
    /// a piece the volume is given (`Class::Given`)
    Given(usize),
    Span(usize, usize, usize),
    Lit(String),
}

#[derive(Clone, Debug, PartialEq)]
enum Item {
    /// a line stored as written
    Lit(String),
    /// `# text = S`
    Text(SRef),
    /// the k-th word of the annotation of S, with field patches
    Row(usize, Vec<(usize, String)>),
    /// the k-th row the Hebrew lexicon writes for the verse (`site_heb`), with its patches
    HRow(usize, Vec<site_heb::Patch>),
}

#[derive(Clone, Debug, PartialEq)]
struct Conllu {
    fence: Option<String>,
    items: Vec<Item>,
    close: Option<String>,
    /// the Hebrew tokens given whole (their analysis), by number
    hgiven: BTreeMap<usize, site_heb::Analysis>,
}

/// A line script: lines of the rebuilt text dropped, lines stored with their position in the output.
#[derive(Clone, Debug, Default, PartialEq)]
struct Script {
    drop: BTreeSet<usize>,
    put: Vec<(usize, String)>,
}

#[derive(Clone, Debug, PartialEq)]
enum Table {
    Lit(String),
    /// MACULA's verb frames rebuilt from the Hebrew tree, and a line script
    Hebrew(Script),
    Rebuilt { reset: bool, short: bool, script: Script },
}

#[derive(Clone, Debug, PartialEq)]
enum Code {
    Conllu(Conllu),
    Table(Table),
}

/// The sentence of a reference.
fn sentence(v: &Volume, r: &SRef) -> String {
    match r {
        SRef::Block(b) => v.blocks[*b].text.clone(),
        SRef::Given(p) => match &v.pieces[*p] {
            Piece::Lit(x, _) => x.trim_end_matches('\n').to_string(),
            _ => String::new(),
        },
        SRef::Span(b, x, y) => v.blocks[*b].text[*x..*y].to_string(),
        SRef::Lit(s) => s.clone(),
    }
}

/// `en`'s tokens of a sentence, their space-after bits, and its annotation.
fn annotate(a: &Annotator, s: &str) -> Option<(Vec<Word>, Vec<bool>)> {
    let toks = a.tokenize(s);
    let mut forms = Vec::with_capacity(toks.len());
    let mut space = Vec::with_capacity(toks.len());
    let mut pos = 0;
    for t in &toks {
        let k = s[pos..].find(t.form.as_str())?;
        if !forms.is_empty() {
            space.push(k > 0);
        }
        forms.push(t.form.clone());
        pos += k + t.form.len();
    }
    space.push(pos < s.len());
    if forms.is_empty() {
        return None;
    }
    Some((a.annotate(&forms), space))
}

fn or_(x: &str) -> String {
    if x.is_empty() { "_".into() } else { x.to_string() }
}

/// The rows `en` gives for a sentence: the fields of each word.
fn canon_rows(ws: &[Word], space: &[bool]) -> Vec<[String; 10]> {
    let n = ws.len();
    ws.iter()
        .enumerate()
        .map(|(k, w)| {
            let misc = if k + 1 < n && !space[k] { "SpaceAfter=No".to_string() } else { "_".to_string() };
            [(k + 1).to_string(), or_(&w.form), or_(&w.lemma), w.upos.name().to_string(), w.tag.name().to_string(), w.feats.to_string(), w.head.to_string(), w.rel.name().to_string(), "_".into(), misc]
        })
        .collect()
}

fn row_line(f: &[String; 10]) -> String {
    format!("{}\n", f.join("\t"))
}

/// The bits of one field patch.
fn field_bits(field: usize, canon: &str, val: &str, n: usize) -> f64 {
    match field {
        2 => lemma_bits(val),
        3 => 5.0,
        4 => 6.0,
        5 => {
            let (a, ua) = Feats::parse(canon);
            let (b, ub) = Feats::parse(val);
            if ua + ub > 0 {
                return lit_bits(val);
            }
            7.0 * (a.0 ^ b.0).count_ones() as f64
        }
        6 => pos_bits(n),
        7 => 6.0,
        9 if canon.replace("SpaceAfter=No", "_") == val.replace("SpaceAfter=No", "_") => 1.0,
        _ => lit_bits(val),
    }
}

/// The prose block (or span of one) that holds a sentence.
fn find_sentence(v: &Volume, s: &str) -> SRef {
    if let Some(b) = v.blocks.iter().position(|b| b.text == s) {
        return SRef::Block(b);
    }
    if let Some(p) = v.pieces.iter().position(|p| matches!(p, Piece::Lit(x, Class::Given | Class::Unparsed) if x.trim_end_matches('\n') == s)) {
        return SRef::Given(p);
    }
    if !s.is_empty() {
        if let Some((b, x)) = v.blocks.iter().enumerate().find_map(|(b, blk)| blk.text.find(s).map(|x| (b, x))) {
            return SRef::Span(b, x, x + s.len());
        }
    }
    SRef::Lit(s.to_string())
}

fn encode_conllu(a: &Annotator, v: &Volume, lit: &str, cp: &mut Copier, t: &mut Tally) -> Conllu {
    let lines: Vec<&str> = lit.split_inclusive('\n').collect();
    let res = &mut t.residual;
    let add = |res: &mut crate::tally::Residual, bits: f64, kind: &str, ex: String| res.add_k(Layer::Tree, Class::Conllu, bits, kind.to_string(), || ex);
    let first = lines.first().copied().unwrap_or("");
    let rtl = first.contains("rtl");
    let fence = (first != FENCE).then(|| first.to_string());
    if let Some(f) = &fence {
        add(res, cp.code(f).0, "fence", format!("{f:?}"));
    }
    let has_close = lines.len() > 1 && lines.last().is_some_and(|l| *l == CLOSE);
    let mid = &lines[1.min(lines.len())..if has_close { lines.len() - 1 } else { lines.len() }];
    let close = (!has_close).then(String::new);
    let mut items = Vec::new();
    // a right-to-left tree (Hebrew): from the verse and the lexicon of the training pages when there is one
    if rtl {
        if let Some(lex) = site_heb::HEB_LEX.get() {
            return encode_heb(lex, v, mid, fence, close, cp, res);
        }
        for l in mid {
            add(res, cp.code(l).0, "rtl", format!("{:?}", l.chars().take(50).collect::<String>()));
            items.push(Item::Lit(l.to_string()));
        }
        return Conllu { fence, items, close, hgiven: BTreeMap::new() };
    }
    let mut sref: Option<SRef> = None;
    let mut rows: Vec<&str> = Vec::new();
    for l in mid {
        if let Some(s) = l.strip_prefix("# text = ") {
            let s = s.strip_suffix('\n').unwrap_or(s);
            let r = find_sentence(v, s);
            match &r {
                SRef::Block(_) => add(res, pos_bits(v.blocks.len()), "text-ref", format!("block: {s:?}")),
                SRef::Given(_) => add(res, pos_bits(v.pieces.len()), "text-ref", format!("given: {s:?}")),
                SRef::Span(b, _, _) => {
                    let n = v.blocks[*b].text.len();
                    add(res, pos_bits(v.blocks.len()) + 2.0 * pos_bits(n), "text-ref", format!("span: {s:?}"))
                }
                SRef::Lit(x) => add(res, cp.code(x).0, "text-literal", format!("{x:?}")),
            }
            if sref.is_none() {
                sref = Some(r.clone());
            }
            items.push(Item::Text(r));
        } else if l.starts_with('#') {
            add(res, cp.code(l).0, "comment", format!("{:?}", l.chars().take(60).collect::<String>()));
            items.push(Item::Lit(l.to_string()));
        } else {
            rows.push(l);
            items.push(Item::Lit(l.to_string()));
        }
    }
    // the words of the page against `en`'s annotation of the sentence
    let canon = sref.as_ref().and_then(|r| annotate(a, &sentence(v, r))).map(|(w, s)| canon_rows(&w, &s));
    let words: Vec<&&str> = rows.iter().filter(|l| l.split('\t').next().is_some_and(|id| id.parse::<usize>().is_ok())).collect();
    let fits = canon.as_ref().is_some_and(|c| c.len() == words.len() && words.iter().zip(c).all(|(l, r)| l.split('\t').nth(1) == Some(r[1].as_str()) && l.trim_end_matches('\n').split('\t').count() == 10));
    let n = words.len();
    let mut k = 0;
    for it in items.iter_mut() {
        let Item::Lit(l) = it else { continue };
        if l.starts_with('#') {
            continue;
        }
        let is_word = l.split('\t').next().is_some_and(|id| id.parse::<usize>().is_ok());
        if fits && is_word {
            let c = &canon.as_ref().unwrap()[k];
            let f: Vec<&str> = l.trim_end_matches('\n').split('\t').collect();
            let mut patches = Vec::new();
            for (j, x) in f.iter().enumerate() {
                if *x != c[j] {
                    let bits = field_bits(j, &c[j], x, n);
                    add(res, bits, FIELDS[j], format!("{} {}: {} → {}", f[1], FIELDS[j], c[j], x));
                    patches.push((j, x.to_string()));
                }
            }
            if !patches.is_empty() {
                // which word
                add(res, pos_bits(n), "word", format!("word {} «{}»", k + 1, f[1]));
            }
            *it = Item::Row(k, patches);
            k += 1;
        } else {
            let kind = if fits { "range" } else if canon.is_none() { "rows, no sentence" } else { "rows, other tokens" };
            add(res, cp.code(l).0, kind, format!("{:?}", l.chars().take(60).collect::<String>()));
        }
    }
    Conllu { fence, items, close, hgiven: BTreeMap::new() }
}

/// The words per written token of predicted rows.
fn pred_counts(rows: &[site_heb::Row]) -> Vec<usize> {
    let toks = site_heb::tokens_of(&rows.iter().map(|r| r.to_vec()).collect::<Vec<_>>());
    toks.iter().map(|t| t.2.len()).collect()
}

/// A Hebrew tree: its comment lines stored, its `# text` a reference, its rows those the lexicon writes for the verse
/// with patches (or stored, when the rows do not line up).
fn encode_heb(lex: &site_heb::Lexicon, v: &Volume, mid: &[&str], fence: Option<String>, close: Option<String>, cp: &mut Copier, res: &mut crate::tally::Residual) -> Conllu {
    let add = |res: &mut crate::tally::Residual, bits: f64, kind: &str, ex: String| res.add_k(Layer::Tree, Class::Conllu, bits, kind.to_string(), || ex);
    let mut items = Vec::new();
    let (mut text, mut verse) = (None, String::new());
    let mut rows: Vec<&str> = Vec::new();
    for l in mid {
        if let Some(s) = l.strip_prefix("# text = ") {
            let s = s.strip_suffix('\n').unwrap_or(s);
            let r = find_sentence(v, s);
            match &r {
                SRef::Lit(x) => add(res, cp.code(x).0, "rtl text-literal", format!("{x:?}")),
                SRef::Block(_) => add(res, pos_bits(v.blocks.len()), "rtl text-ref", String::new()),
                _ => add(res, pos_bits(v.pieces.len()), "rtl text-ref", String::new()),
            }
            text.get_or_insert(s.to_string());
            items.push(Item::Text(r));
        } else if l.starts_with('#') {
            if let Some(x) = l.strip_prefix("# verse = ") {
                verse = x.trim_end().to_string();
            }
            add(res, cp.code(l).0, "rtl comment", format!("{:?}", l.chars().take(60).collect::<String>()));
            items.push(Item::Lit(l.to_string()));
        } else {
            rows.push(l);
            items.push(Item::Lit(l.to_string()));
        }
    }
    // the page's written tokens against the lexicon's: a token cut into another number of words is given whole
    let lit: Vec<Vec<&str>> = rows.iter().map(|l| l.trim_end_matches('\n').split('\t').collect()).collect();
    let lit_rows: Vec<Vec<String>> = lit.iter().map(|r| r.iter().map(|x| x.to_string()).collect()).collect();
    let page_toks = site_heb::tokens_of(&lit_rows);
    let mut given: BTreeMap<usize, site_heb::Analysis> = BTreeMap::new();
    if let Some(t) = &text {
        let lt = site_heb::tokens(lex, t);
        if lt.len() == page_toks.len() {
            let pred0 = site_heb::predict(lex, t, &verse);
            let counts = pred_counts(&pred0);
            for (ti, (form, rm, ws)) in page_toks.iter().enumerate() {
                if counts.get(ti) != Some(&ws.len()) || lt[ti] != *form {
                    let an = site_heb::analysis_of(rm, ws);
                    // `MOVA_RT_GIVEN=1`: each token given whole on stderr (the page's words, the lexicon's count), for study
                    if std::env::var_os("MOVA_RT_GIVEN").is_some() {
                        eprintln!("given\t{form}\t{}\t{}\t{}", lt[ti], ws.len(), counts.get(ti).copied().unwrap_or(0));
                    }
                    add(res, cp.code(&site_heb::analysis_record(&an)).0 + pos_bits(lt.len()), "rtl token given", form.clone());
                    given.insert(ti, an);
                }
            }
        }
    }
    let pred = text.as_deref().map(|t| site_heb::predict_with(lex, t, &verse, &given)).unwrap_or_default();
    let fits = pred.len() == lit.len() && pred.iter().zip(&lit).all(|(p, l)| l.len() == 10 && p[0] == l[0]);
    if !fits {
        // nothing given: the rows are stored
        given.clear();
    }
    let n = pred.iter().filter(|r| !r[0].contains('-')).count();
    let mut k = 0;
    for it in items.iter_mut() {
        let Item::Lit(l) = it else { continue };
        if l.starts_with('#') {
            continue;
        }
        if fits {
            let (ps, bits, kinds) = site_heb::diff(&pred[k], &lit[k], n);
            for (i, kd) in kinds.iter().enumerate() {
                // the bits of a row's patches go to its first kind; the others count items
                add(res, if i == 0 { bits } else { 0.0 }, &format!("rtl {kd}"), format!("{}: {kd}", lit[k][1]));
            }
            *it = Item::HRow(k, ps);
            k += 1;
        } else {
            add(res, cp.code(l).0, "rtl rows, other tokens", format!("{:?}", l.chars().take(60).collect::<String>()));
        }
    }
    Conllu { fence, items, close, hgiven: given }
}

fn decode_conllu(a: &Annotator, v: &Volume, c: &Conllu) -> String {
    let mut s = c.fence.clone().unwrap_or_else(|| FENCE.to_string());
    let mut canon: Option<Vec<[String; 10]>> = None;
    // the rows the Hebrew lexicon writes for the verse, once
    let hpred: Vec<site_heb::Row> = match (site_heb::HEB_LEX.get(), c.items.iter().any(|i| matches!(i, Item::HRow(..)))) {
        (Some(lex), true) => {
            let text = c.items.iter().find_map(|i| if let Item::Text(r) = i { Some(sentence(v, r)) } else { None }).unwrap_or_default();
            let verse = c.items.iter().find_map(|i| if let Item::Lit(l) = i { l.strip_prefix("# verse = ").map(|x| x.trim_end().to_string()) } else { None }).unwrap_or_default();
            site_heb::predict_with(lex, &text, &verse, &c.hgiven)
        }
        _ => Vec::new(),
    };
    for it in &c.items {
        match it {
            Item::Lit(l) => s.push_str(l),
            Item::Text(r) => {
                let x = sentence(v, r);
                if canon.is_none() {
                    canon = annotate(a, &x).map(|(w, sp)| canon_rows(&w, &sp));
                }
                s.push_str(&format!("# text = {x}\n"));
            }
            Item::HRow(k, ps) => {
                let row = hpred.get(*k).map(|p| site_heb::apply(p, ps)).unwrap_or_default();
                s.push_str(&format!("{}\n", row.join("\t")));
            }
            Item::Row(k, patches) => {
                let mut f = canon.as_ref().and_then(|c| c.get(*k)).cloned().unwrap_or_default();
                for (j, x) in patches {
                    f[*j] = x.clone();
                }
                s.push_str(&row_line(&f));
            }
        }
    }
    s.push_str(c.close.as_deref().unwrap_or(CLOSE));
    s
}

/// The words of a decoded CoNLL-U for the frames reader (lemmas lower-cased, as the page builder reads them), if they
/// make a tree: ids 1..n, heads in range, a root, no cycle, every relation known.
fn tree_words(conllu: &str) -> Option<Vec<Word>> {
    let mut ws = Vec::new();
    for l in conllu.lines() {
        if l.starts_with('#') || l.starts_with("```") || l.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 10 || c[0].contains('-') || c[0].contains('.') {
            if c.len() < 10 {
                return None;
            }
            continue;
        }
        let id: usize = c[0].parse().ok()?;
        if id != ws.len() + 1 {
            return None;
        }
        let rel = Rel::parse(c[7]).or_else(|| c[7].split_once(':').and_then(|(b, _)| Rel::parse(b)))?;
        let lemma = if c[2] == "_" && c[1] != "_" { String::new() } else { c[2].to_string() };
        ws.push(Word { form: c[1].to_string(), lemma: lemma.to_lowercase(), upos: UPos::parse(c[3]).unwrap_or(UPos::X), tag: Tag::parse(c[4]).unwrap_or(Tag::X), feats: Feats::parse(c[5]).0, head: c[6].parse().ok()?, rel });
    }
    let n = ws.len();
    if n == 0 || ws.iter().enumerate().any(|(i, w)| w.head > n || w.head == i + 1) || !ws.iter().any(|w| w.head == 0) {
        return None;
    }
    for w in &ws {
        let (mut h, mut steps) = (w.head, 0);
        while h != 0 {
            h = ws[h - 1].head;
            steps += 1;
            if steps > n {
                return None;
            }
        }
    }
    Some(ws)
}

/// The table and the note-less table of some frames.
fn tables(fs: &[Frame]) -> (String, String) {
    let full = prag::frames_md::table(fs);
    let short = match full.find("\n<!-- .note -->\nOne row per clause") {
        Some(k) => format!("{}\n", &full[..k]),
        None => full.clone(),
    };
    (full, short)
}

fn lines_of(s: &str) -> Vec<&str> {
    s.split_inclusive('\n').collect()
}

/// The line script from `a` to `b` (the longest common subsequence of lines kept).
fn script(a: &str, b: &str) -> Script {
    let (x, y) = (lines_of(a), lines_of(b));
    let (n, m) = (x.len(), y.len());
    let mut d = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            d[i][j] = if x[i] == y[j] { d[i + 1][j + 1] + 1 } else { d[i + 1][j].max(d[i][j + 1]) };
        }
    }
    let mut sc = Script::default();
    let (mut i, mut j) = (0, 0);
    while i < n || j < m {
        if i < n && j < m && x[i] == y[j] {
            i += 1;
            j += 1;
        } else if j < m && (i == n || d[i][j + 1] >= d[i + 1][j]) {
            sc.put.push((j, y[j].to_string()));
            j += 1;
        } else {
            sc.drop.insert(i);
            i += 1;
        }
    }
    sc
}

fn apply(a: &str, sc: &Script) -> String {
    let kept: Vec<&str> = lines_of(a).into_iter().enumerate().filter(|(i, _)| !sc.drop.contains(i)).map(|x| x.1).collect();
    let mut out: Vec<String> = Vec::with_capacity(kept.len() + sc.put.len());
    let mut it = kept.into_iter();
    let mut puts = sc.put.iter().peekable();
    loop {
        if let Some((p, l)) = puts.peek() {
            if *p == out.len() {
                out.push(l.clone());
                puts.next();
                continue;
            }
        }
        match it.next() {
            Some(l) => out.push(l.to_string()),
            None => break,
        }
    }
    for (_, l) in puts {
        out.push(l.clone());
    }
    out.concat()
}

fn script_bits(rebuilt: &str, sc: &Script, lit: &str) -> f64 {
    let (n, m) = (lines_of(rebuilt).len(), lines_of(lit).len());
    sc.drop.len() as f64 * pos_bits(n) + sc.put.iter().map(|(_, l)| lit_bits(l) + pos_bits(m)).sum::<f64>()
}

/// What a stored table line differs in: the column of the first cell that differs from the rebuilt line at the
/// same place, or the kind of line.
fn line_kind(l: &str, rebuilt: &[&str], at: usize) -> String {
    const COLS: [&str; 10] = ["#", "verb", "tense, mood", "who", "what, whom", "to whom", "where", "when", "other", "roles by"];
    if !l.starts_with('|') {
        return "note".into();
    }
    let cells = |s: &str| s.trim_end().split(" | ").map(str::to_string).collect::<Vec<_>>();
    let a = cells(l);
    match rebuilt.get(at).filter(|r| r.starts_with('|')) {
        Some(r) => {
            let b = cells(r);
            if a.len() != b.len() {
                return "row, other cells".into();
            }
            a.iter().zip(&b).position(|(x, y)| x != y).map(|k| format!("col: {}", COLS.get(k).unwrap_or(&"?"))).unwrap_or_else(|| "row".into())
        }
        None => "row, added".into(),
    }
}

/// The reading of a tree by the frames reader from a discourse state: (frames, state after).
fn read(ws: &[Word], ctx: &Ctx) -> (Vec<Frame>, Ctx) {
    let mut c = ctx.clone();
    let (fs, _, _) = frames_of_tree(&Tree::of_words(ws.to_vec()), &mut c, false);
    c.new_line();
    (fs, c)
}

/// The state of the frames chain: the discourse carried through the block's trees.
#[derive(Default)]
struct Chain {
    ctx: Ctx,
    /// the last right-to-left tree (for the Hebrew verbs' table)
    rtl: Option<String>,
    /// the last English tree's reading: carried on, and from a fresh state
    last: Option<((Vec<Frame>, Ctx), (Vec<Frame>, Ctx))>,
}

impl Chain {
    fn tree(&mut self, conllu: &str) {
        // a tree the next table did not take continues the state
        if let Some((cont, _)) = self.last.take() {
            self.ctx = cont.1;
        }
        let rtl = conllu.lines().next().is_some_and(|l| l.contains("rtl"));
        if rtl {
            self.rtl = Some(conllu.to_string());
            return;
        }
        match tree_words(conllu) {
            Some(ws) => self.last = Some((read(&ws, &self.ctx), read(&ws, &Ctx::default()))),
            None => self.last = Some(((Vec::new(), self.ctx.clone()), (Vec::new(), Ctx::default()))),
        }
    }
    /// The frames for a table, from the state carried on or a fresh one; the state moves on.
    fn take(&mut self, reset: bool) -> Vec<Frame> {
        match self.last.take() {
            Some((cont, fresh)) => {
                let (fs, c) = if reset { fresh } else { cont };
                self.ctx = c;
                fs
            }
            None => Vec::new(),
        }
    }
    fn peek(&self, reset: bool) -> Vec<Frame> {
        match &self.last {
            Some((cont, fresh)) => (if reset { &fresh.0 } else { &cont.0 }).clone(),
            None => Vec::new(),
        }
    }
}

fn encode_table(lit: &str, chain: &mut Chain, cp: &mut Copier, t: &mut Tally) -> Table {
    let res = &mut t.residual;
    // a table of another kind (the Hebrew verbs' frames): stored
    let english = lit.lines().nth(1).is_some_and(|l| l.starts_with("| # | verb | tense, mood |")) || lit.contains("The frames reader finds no event");
    let hebrew = lit.lines().nth(1).is_some_and(|l| l.starts_with("| verb | form · stem |")) || lit.contains("MACULA marks no verb frame");
    if hebrew {
        // MACULA's verb frames, from the Hebrew tree above it
        chain.take(false);
        let cand = prag::frames_md::hebrew_table(&prag::frames_md::hwords(chain.rtl.as_deref().unwrap_or("")));
        let sc = script(&cand, lit);
        res.add_k(Layer::Frames, Class::FrTable, 1.0, "bits".into(), || "Hebrew".into());
        let rl = lines_of(&cand);
        for (_, l) in &sc.put {
            res.add_k(Layer::Frames, Class::FrTable, cp.code(l).0 + pos_bits(lines_of(lit).len()), "Hebrew line".into(), || format!("{:?}", l.chars().take(90).collect::<String>()));
        }
        for d in &sc.drop {
            res.add_k(Layer::Frames, Class::FrTable, pos_bits(rl.len()), "Hebrew line dropped".into(), || format!("dropped {:?}", rl[*d].chars().take(90).collect::<String>()));
        }
        return Table::Hebrew(sc);
    }
    if !english {
        res.add_k(Layer::Frames, Class::FrTable, cp.code(lit).0, "other table".into(), || format!("{:?}", lit.chars().take(60).collect::<String>()));
        // the tree above it is read all the same
        chain.take(false);
        return Table::Lit(lit.to_string());
    }
    let mut best: Option<(f64, bool, bool, Script, String)> = None;
    for reset in [false, true] {
        let (full, short) = tables(&chain.peek(reset));
        for (sh, cand) in [(false, &full), (true, &short)] {
            let sc = script(cand, lit);
            let bits = script_bits(cand, &sc, lit);
            if best.as_ref().is_none_or(|b| bits < b.0) {
                best = Some((bits, reset, sh, sc, cand.clone()));
            }
        }
    }
    let (bits, reset, short, sc, cand) = best.unwrap();
    chain.take(reset);
    res.add_k(Layer::Frames, Class::FrTable, 2.0, "bits".into(), || "start afresh, note".into());
    let rl = lines_of(&cand);
    for (p, l) in &sc.put {
        let kind = line_kind(l, &rl, *p);
        res.add_k(Layer::Frames, Class::FrTable, cp.code(l).0 + pos_bits(lines_of(lit).len()), kind, || format!("{:?}", l.chars().take(90).collect::<String>()));
    }
    for d in &sc.drop {
        res.add_k(Layer::Frames, Class::FrTable, pos_bits(rl.len()), "line dropped".into(), || format!("dropped {:?}", rl[*d].chars().take(90).collect::<String>()));
    }
    let _ = bits;
    Table::Rebuilt { reset, short, script: sc }
}

fn decode_table(tb: &Table, chain: &mut Chain) -> String {
    match tb {
        Table::Lit(s) => {
            chain.take(false);
            s.clone()
        }
        Table::Rebuilt { reset, short, script: sc } => {
            let (full, sh) = tables(&chain.take(*reset));
            apply(if *short { &sh } else { &full }, sc)
        }
        Table::Hebrew(sc) => {
            chain.take(false);
            apply(&prag::frames_md::hebrew_table(&prag::frames_md::hwords(chain.rtl.as_deref().unwrap_or(""))), sc)
        }
    }
}

/// The gate's negative control on the derived pieces: the first stored item of a layer is lost. Whether there was
/// one to lose.
fn damage(codes: &mut [Code], l: Layer) -> bool {
    for c in codes.iter_mut() {
        match (c, l) {
            (Code::Conllu(cc), Layer::Tree) => {
                for it in cc.items.iter_mut() {
                    match it {
                        Item::Row(_, p) if !p.is_empty() => {
                            p.remove(0);
                            return true;
                        }
                        Item::HRow(_, p) if !p.is_empty() => {
                            p.remove(0);
                            return true;
                        }
                        Item::Lit(s) if !s.is_empty() => {
                            s.clear();
                            return true;
                        }
                        _ => {}
                    }
                }
            }
            (Code::Table(Table::Rebuilt { script: sc, .. }) | Code::Table(Table::Hebrew(sc)), Layer::Frames) => {
                if let Some((k, _)) = sc.put.first().cloned() {
                    sc.put.retain(|x| x.0 != k);
                    return true;
                }
                if let Some(d) = sc.drop.iter().next().copied() {
                    sc.drop.remove(&d);
                    return true;
                }
            }
            (Code::Table(Table::Lit(s)), Layer::Frames) if !s.is_empty() => {
                s.clear();
                return true;
            }
            _ => {}
        }
    }
    false
}

/// A piece written whole from the layers: every word row rebuilt without a patch (comment lines aside), or the table
/// with an empty line script.
fn free(c: &Code) -> bool {
    match c {
        Code::Conllu(cc) => cc.fence.is_none() && cc.close.is_none() && cc.items.iter().all(|it| match it {
            Item::Text(r) => !matches!(r, SRef::Lit(_)),
            Item::Row(_, p) => p.is_empty(),
            Item::HRow(_, p) => p.is_empty(),
            Item::Lit(l) => l.starts_with('#'),
        }) && cc.items.iter().any(|it| matches!(it, Item::Row(..) | Item::HRow(..))),
        Code::Table(Table::Rebuilt { script, .. }) | Code::Table(Table::Hebrew(script)) => script.put.is_empty() && script.drop.is_empty(),
        Code::Table(Table::Lit(_)) => false,
    }
}

fn kind_of(c: &Code, lit: &str) -> &'static str {
    match c {
        Code::Conllu(_) if lit.lines().next().is_some_and(|l| l.contains("rtl")) => "conllu, right to left",
        Code::Conllu(_) => "conllu",
        Code::Table(Table::Lit(_)) => "frames table, other",
        Code::Table(Table::Hebrew(_)) => "frames table, Hebrew",
        Code::Table(_) => "frames table",
    }
}

/// Encode a volume's derived pieces, decode them from the codes alone, and gate them; the residual and the counts go
/// to the tally. `damage`: the control loses the first stored item of that layer (when the layers' own residual had
/// none to lose).
pub fn run(a: &Annotator, v: &Volume, damage_l: Option<Layer>, cp: &mut Copier, t: &mut Tally) {
    // the markup coded by the copier so far (`vol::residual_with`)
    let m = t.derived.entry("markup, copied".to_string()).or_default();
    m.pieces += cp.pieces;
    m.free += cp.exact;
    m.literal_bits += cp.literal_bits;
    m.code_bits += cp.code_bits;
    let lits: Vec<(&str, Class)> = v.pieces.iter().filter_map(|p| if let Piece::Lit(x, c @ (Class::Conllu | Class::FrTable)) = p { Some((x.as_str(), *c)) } else { None }).collect();
    let mut codes = Vec::with_capacity(lits.len());
    let mut chain = Chain::default();
    for (lit, c) in &lits {
        let before = t.residual.cells.iter().filter(|(k, _)| k.1 == *c).map(|(_, x)| x.bits).sum::<f64>();
        let code = match c {
            Class::Conllu => {
                let cc = encode_conllu(a, v, lit, cp, t);
                chain.tree(lit);
                Code::Conllu(cc)
            }
            _ => Code::Table(encode_table(lit, &mut chain, cp, t)),
        };
        let after = t.residual.cells.iter().filter(|(k, _)| k.1 == *c).map(|(_, x)| x.bits).sum::<f64>();
        let d = t.derived.entry(kind_of(&code, lit).to_string()).or_insert_with(Derived::default);
        d.pieces += 1;
        d.literal_bits += lit_bits(lit);
        d.code_bits += after - before;
        d.free += free(&code) as u64;
        codes.push(code);
    }
    if let Some(l) = damage_l {
        if t.damaged == 0 && damage(&mut codes, l) {
            t.damaged = 1;
        }
    }
    // down: the codes alone, the prose and the model
    let mut chain = Chain::default();
    let mut bad: Option<String> = None;
    for (code, (lit, _)) in codes.iter().zip(&lits) {
        let out = match code {
            Code::Conllu(cc) => {
                let s = decode_conllu(a, v, cc);
                chain.tree(&s);
                s
            }
            Code::Table(tb) => decode_table(tb, &mut chain),
        };
        if out != *lit && bad.is_none() {
            let k = out.bytes().zip(lit.bytes()).take_while(|(x, y)| x == y).count();
            bad = Some(format!("{}: {} differs at byte {k}: «{}» for «{}»", v.name, kind_of(code, lit), out.get(k..).unwrap_or("").chars().take(40).collect::<String>(), lit.get(k..).unwrap_or("").chars().take(40).collect::<String>()));
        }
    }
    if let Some(b) = bad {
        if t.exact_ok > 0 {
            t.exact_ok = 0;
        }
        t.exact_fail.push(b);
    }
}

/// Every word row of the page that the annotation of its sentence would give, without patches: a check for tests.
#[cfg(test)]
pub(crate) fn rows_for(a: &Annotator, s: &str) -> Vec<String> {
    annotate(a, s).map(|(w, sp)| canon_rows(&w, &sp).iter().map(row_line).collect()).unwrap_or_default()
}

/// `en`'s words of a sentence, lemmas lower-cased as the page builder reads them: a check for tests.
#[cfg(test)]
pub(crate) fn words_for(a: &Annotator, s: &str) -> Vec<Word> {
    annotate(a, s).map(|(w, _)| w.into_iter().map(|x| Word { lemma: x.lemma.to_lowercase(), ..x }).collect()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_scripts_give_the_target_back() {
        for (a, b) in [("a\nb\nc\n", "a\nx\nc\n"), ("a\n", "b\nc\nd\n"), ("", "x\n"), ("a\nb\n", ""), ("p\nq\nr\ns\n", "q\ns\nt\n")] {
            let sc = script(a, b);
            assert_eq!(apply(a, &sc), b, "{a:?} → {b:?}");
        }
        assert!(script("a\nb\n", "a\nb\n").put.is_empty());
    }

    #[test]
    fn a_tree_needs_known_relations_and_a_root() {
        let ok = "```conllu\n# text = x\n1\tGod\tGod\tPROPN\tNNP\t_\t2\tnsubj\t_\t_\n2\tsaw\tsee\tVERB\tVBD\t_\t0\troot\t_\t_\n```\n";
        assert_eq!(tree_words(ok).unwrap()[0].lemma, "god");
        assert!(tree_words(&ok.replace("nsubj", "nosuch")).is_none());
        assert!(tree_words(&ok.replace("\t0\troot", "\t1\troot")).is_none());
    }
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn script_fibers_and_reference_precedence() {
        let texts = ["", "a", "a\n", "b\na\n", "a\na\n", "β\n"];
        for a in texts {
            for b in texts {
                let sc = script(a, b);
                assert_eq!(apply(a, &sc), b);
                assert!(script_bits(a, &sc, b).is_finite());
            }
        }
        let mut v = crate::vol::txt_volume("x", "prefix β suffix");
        v.pieces.push(Piece::Lit("β\n".into(), Class::Given));
        assert_eq!(find_sentence(&v, "β"), SRef::Given(1));
        assert_eq!(find_sentence(&v, "prefix β suffix"), SRef::Block(0));
        assert_eq!(find_sentence(&v, "suffix"), SRef::Span(0, 10, 16));
        assert_eq!(find_sentence(&v, ""), SRef::Lit(String::new()));
        let mut codes = vec![Code::Table(Table::Rebuilt {
            reset: false,
            short: false,
            script: Script {
                drop: [0].into(),
                put: vec![(0, "x".into())],
            },
        })];
        assert!(!damage(&mut codes, Layer::Tree));
        assert!(damage(&mut codes, Layer::Frames));
        assert!(damage(&mut codes, Layer::Frames));
        assert!(!damage(&mut codes, Layer::Frames));
        assert!(free(&codes[0]));
    }
}
