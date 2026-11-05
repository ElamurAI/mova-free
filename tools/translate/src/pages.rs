//! The parallel Tanakh as site pages, both ways (`--pages <md root>`). A chapter of the corpus pages is its Hebrew
//! verse pages (`corpora/<part>/<BOOK>.<c>.<v>/`: the verse, its MACULA tree, its pragmatics, the Targum, the links
//! to its English) on one side, and the JPS sentence pages that begin in it (`corpora/<part>/jps/<BOOK>.<n>/`: the
//! sentence, its tree and frames) on the other. Blocks are the runs of whole chapters `roundtrip blocks` cuts the
//! JPS into (`blocks::jps` + `blocks::group`, 2 500 words), the Hebrew side cut the same (`blocks::like`).
//!
//! A target page block costs two things:
//! - **the translation** of its texts: each target verse (Hebrew) or unit (the JPS sentences that begin in a verse)
//!   from the source through the meta-world, with the split code (reader, encoder, stack): Hebrew → English by the
//!   Hebrew reader and the JPS encoder (the `wlc → jps` system), English → Hebrew by the English reader and the
//!   Hebrew encoder (`heb_enc`);
//! - **the page** around them: the target page block through `roundtrip blocks`' site decoder with the translated
//!   texts and the source's own texts given (`site_md::give`): markup, the other prose, the trees and tables it
//!   rebuilds, the Targum.
//! Bits per block, per 1 000 words of the target texts and per verse, by stage and class group.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use en::annotate::Annotator;
use roundtrip::blocks::{self, Block, Chapter, site_md};
use roundtrip::class::{Class, LAYERS};
use roundtrip::pipeline::{self, Opts};
use roundtrip::tally::{Residual, Tally};

use crate::bible::{self, VKey, vkey};
use crate::corpus::{Corpus, par_map};
use crate::heb_enc::{self, HebEncoder};
use crate::hebrew::{HVerse, Reader};
use crate::meta::Lexicon;

const PARTS: [&str; 3] = ["torah", "neviim", "ketuvim"];

/// The page chapters of some books: (chapter id `GEN.1`, Hebrew pages, English pages, Hebrew verses, English ids).
pub struct PageCh {
    pub id: String,
    /// the Tanakh's part (`torah`, `neviim`, `ketuvim`): the site section of its pages
    pub part: &'static str,
    pub he: Chapter,
    pub en: Chapter,
    pub verses: Vec<VKey>,
}

fn read_page(root: &Path, id: &str) -> Result<Chapter> {
    let p = root.join(id).join("index.md");
    let md = std::fs::read_to_string(&p).with_context(|| p.display().to_string())?;
    Ok(site_md::chapter(id, &md))
}

fn concat(id: &str, pages: Vec<Chapter>) -> Chapter {
    let v = blocks::concat(id, &pages);
    Chapter { id: id.to_string(), group: id.split('.').next().unwrap_or("").to_string(), vol: v, para_cut: false }
}

/// The page chapters of `books`, in their order; `verse_of`: a JPS sentence id → its verse (Hebrew numbers).
pub fn chapters(root: &Path, books: &[&str], verse_of: &BTreeMap<String, String>) -> Result<Vec<PageCh>> {
    let mut out = Vec::new();
    for part in PARTS {
        let dir = root.join("corpora").join(part);
        let Ok(rd) = std::fs::read_dir(&dir) else { continue };
        let names: Vec<String> = rd.flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
        let jdir = dir.join("jps");
        let jnames: Vec<String> = std::fs::read_dir(&jdir).map(|r| r.flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect()).unwrap_or_default();
        for b in books {
            // verse pages BOOK.c.v by chapter
            let mut by: BTreeMap<u16, Vec<(u16, String)>> = BTreeMap::new();
            for n in &names {
                if let Some(k) = vkey(n).filter(|k| bible::BOOKS[k.0 as usize] == *b) {
                    by.entry(k.1).or_default().push((k.2, n.clone()));
                }
            }
            if by.is_empty() {
                continue;
            }
            // English sentence pages BOOK.n by the chapter of their verse
            let mut en: BTreeMap<u16, Vec<(u32, String)>> = BTreeMap::new();
            for n in &jnames {
                let Some((bb, num)) = n.split_once('.') else { continue };
                if bb != *b {
                    continue;
                }
                let Ok(num) = num.parse::<u32>() else { continue };
                let Some(k) = verse_of.get(&format!("jps:{b}:{num}")).and_then(|v| vkey(v)) else { continue };
                en.entry(k.1).or_default().push((num, n.clone()));
            }
            for (c, mut vs) in by {
                vs.sort();
                let mut es = en.remove(&c).unwrap_or_default();
                es.sort();
                let id = format!("{b}.{c}");
                let he: Vec<Chapter> = vs.iter().map(|(_, n)| read_page(root, &format!("corpora/{part}/{n}"))).collect::<Result<_>>()?;
                let enp: Vec<Chapter> = es.iter().map(|(_, n)| read_page(root, &format!("corpora/{part}/jps/{n}"))).collect::<Result<_>>()?;
                let verses = vs.iter().filter_map(|(_, n)| vkey(n)).collect();
                out.push(PageCh { he: concat(&id, he), en: concat(&id, enp), id, part, verses });
            }
        }
    }
    // the books' order
    out.sort_by_key(|c| (bible::book_of(c.id.split('.').next().unwrap_or("")).unwrap_or(99), c.id.split('.').nth(1).and_then(|x| x.parse::<u16>().ok()).unwrap_or(0)));
    Ok(out)
}

/// One direction over the blocks.
#[derive(Default)]
pub struct Dir {
    pub name: String,
    pub blocks: u64,
    pub verses: u64,
    pub words: u64,
    pub gate_ok: u64,
    pub page_ok: u64,
    pub trans: [Residual; 3],
    pub page: Tally,
    pub chrf: f64,
    pub units: u64,
    pub fails: Vec<String>,
}

fn bits(r: &Residual) -> f64 {
    r.cells.values().map(|c| c.bits).sum::<f64>() + 0.0
}

fn page_bits(t: &Tally) -> f64 {
    t.residual.cells.iter().filter(|(k, _)| LAYERS.contains(&k.0) && k.1 != Class::Meta).map(|(_, c)| c.bits).sum::<f64>() + 0.0
}

impl Dir {
    pub fn trans_bits(&self) -> f64 {
        self.trans.iter().map(bits).sum()
    }
    pub fn total(&self) -> f64 {
        self.trans_bits() + page_bits(&self.page)
    }
}

/// What the run gives this module.
pub struct Ctx<'a> {
    pub a: &'a Annotator,
    pub root: PathBuf,
    pub judaism: PathBuf,
    pub books: Vec<&'static str>,
    pub verse_of: BTreeMap<String, String>,
    pub lex: &'a Lexicon,
    pub hreader: &'a Reader,
    pub heb: &'a BTreeMap<VKey, HVerse>,
    pub jpsh: &'a Corpus,
    /// the JPS sentences (the English pages' texts) by the verse they begin in
    pub sents: BTreeMap<VKey, Vec<String>>,
    /// the `wlc → jps` system's per-unit stages
    pub wlc_jps: BTreeMap<VKey, ([Residual; 3], bool, f64)>,
    pub henc: &'a HebEncoder,
    pub henc_shuf: &'a HebEncoder,
    pub lookup: &'a roundtrip::mixed::heb::Lookup,
    pub threads: usize,
}

/// The target page blocks through the site decoder with their given texts; each tally's `derived["given"]` counts
/// the pieces given (and the texts there were to give).
fn run_pages(a: &Annotator, bs: &[Block], given: &(dyn Fn(&Block) -> BTreeSet<String> + Sync), part: &(dyn Fn(&Block) -> &'static str + Sync), threads: usize) -> Vec<Tally> {
    par_map(bs, threads, &|b: &Block| {
        let g = given(b);
        let mut v = site_md::repeat(&site_md::give(&b.vol, &g));
        // named as the site's blocks are, so the copier starts with the section's markup dictionary
        v.name = format!("site/corpora/{}:{}", part(b), b.id);
        let mut t = pipeline::run(a, &v, &Opts { exact: true, ..Default::default() }).unwrap_or_else(|e| Tally { volumes: 1, exact_fail: vec![format!("{}: {e:#}", b.id)], ..Default::default() });
        let d = t.derived.entry("given".into()).or_default();
        d.pieces += v.pieces.iter().filter(|p| matches!(p, roundtrip::vol::Piece::Lit(_, Class::Given))).count() as u64;
        d.free += g.len() as u64;
        t
    })
}

/// Both directions over the page blocks of the split's books; the report section and the rows.
pub fn run(c: &Ctx) -> Result<(String, String)> {
    let chs = chapters(&c.root, &c.books, &c.verse_of)?;
    let jps = blocks::jps(&c.judaism, &c.books)?;
    let runs = blocks::blocks("tanakh", &jps, blocks::MAX_WORDS);
    let he_ch: Vec<Chapter> = chs.iter().map(|x| x.he.clone()).collect();
    let en_ch: Vec<Chapter> = chs.iter().map(|x| x.en.clone()).collect();
    let he_b = blocks::like("tanakh-he-pages", &he_ch, &runs);
    let en_b = blocks::like("tanakh-en-pages", &en_ch, &runs);
    let verses_of: BTreeMap<&str, &Vec<VKey>> = chs.iter().map(|x| (x.id.as_str(), &x.verses)).collect();
    let part_of: BTreeMap<&str, &'static str> = chs.iter().map(|x| (x.id.as_str(), x.part)).collect();
    let part = |b: &Block| -> &'static str { b.chapters.first().and_then(|c| part_of.get(c.as_str()).copied()).unwrap_or("torah") };
    let block_verses = |b: &Block| -> Vec<VKey> { b.chapters.iter().flat_map(|id| verses_of.get(id.as_str()).map(|v| v.to_vec()).unwrap_or_default()).collect() };
    let en_texts = |b: &Block| -> BTreeSet<String> {
        block_verses(b).iter().filter_map(|k| c.sents.get(k)).flatten().cloned().collect()
    };
    let he_texts = |b: &Block| -> BTreeSet<String> { block_verses(b).iter().filter_map(|k| c.heb.get(k)).map(|v| v.text.clone()).collect() };
    let mut out = String::new();
    let mut rows = String::from("direction\tblock\tverses\twords\tgate\tpage_exact\treader\tencoder\tstack\tpage\ttotal\n");
    let mut dirs: Vec<Dir> = Vec::new();
    // Hebrew → English: the English pages, the units from the wlc → jps system
    {
        let tallies = run_pages(c.a, &en_b, &|b| {
            let mut g = en_texts(b);
            g.extend(he_texts(b));
            g
        }, &part, c.threads);
        let mut d = Dir { name: "wlc → jps (English pages)".into(), ..Default::default() };
        for (b, t) in en_b.iter().zip(&tallies) {
            let mut st: [Residual; 3] = Default::default();
            let (mut ok, mut n, mut w) = (true, 0u64, 0u64);
            for k in block_verses(b) {
                let Some((r, g, chrf)) = c.wlc_jps.get(&k) else { continue };
                for s in 0..3 {
                    st[s].merge(&r[s]);
                }
                ok &= *g;
                n += 1;
                d.chrf += chrf;
                d.units += 1;
                w += c.jpsh.verses.get(&k).map_or(0, |v| v.text.split_whitespace().count() as u64);
            }
            add_block(&mut d, &mut rows, b, st, ok, n, w, t);
        }
        dirs.push(d);
    }
    // English → Hebrew: the Hebrew pages, each verse by the Hebrew encoder (and its control)
    for (name, enc) in [("jps → wlc (Hebrew pages)", c.henc), ("jps → wlc: Hebrew encoder learned on shuffled pairs", c.henc_shuf)] {
        let control = enc as *const _ != c.henc as *const _;
        let tallies = if control { Vec::new() } else { run_pages(c.a, &he_b, &|b| {
            let mut g = he_texts(b);
            g.extend(en_texts(b));
            g
        }, &part, c.threads) };
        let mut d = Dir { name: name.into(), ..Default::default() };
        for (bi, b) in he_b.iter().enumerate() {
            let ks: Vec<VKey> = block_verses(b).into_iter().filter(|k| c.heb.contains_key(k)).collect();
            let outs = par_map(&ks, c.threads, &|k: &VKey| {
                let hv = &c.heb[k];
                let src = c.jpsh.verses.get(k).map(|v| c.lex.read(&v.meta_words())).unwrap_or_default();
                let (tm, _) = c.hreader.read(c.lex, hv);
                heb_enc::measure(enc, c.lookup, &src, &tm, hv)
            });
            let mut st: [Residual; 3] = Default::default();
            let (mut ok, mut w) = (true, 0u64);
            for (k, o) in ks.iter().zip(&outs) {
                for s in 0..3 {
                    st[s].merge(&o.res[s]);
                }
                if !o.gate_ok && d.fails.len() < 4 {
                    d.fails.push(format!("{}: {}", bible::show(*k), o.fail.clone().unwrap_or_default()));
                }
                ok &= o.gate_ok;
                d.chrf += o.chrf;
                d.units += 1;
                w += c.heb[k].text.split_whitespace().count() as u64;
            }
            let t = tallies.get(bi).cloned().unwrap_or_default();
            add_block(&mut d, &mut rows, b, st, ok, ks.len() as u64, w, &t);
        }
        dirs.push(d);
    }
    // the report
    let _ = writeln!(out, "## The parallel Tanakh as pages, both ways\n\nBlocks: the JPS's chapter runs of `roundtrip blocks` (2 500 words) over the corpus pages of the split's books ({}): {} blocks, {} Hebrew verse pages and {} English sentence pages.\n", c.books.join(", "), he_b.len(), he_b.iter().map(|b| b.vol.text().matches("\n<!-- .vhe ").count()).sum::<usize>(), en_b.iter().map(|b| b.vol.text().matches("<!-- md -->").count()).sum::<usize>());
    let _ = writeln!(out, "| direction | blocks | verses | target words | gate | page exact | translation, per block | page, per block | total per block | per 1 000 words | per verse (translation) | chrF per verse |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for d in &dirs {
        let nb = d.blocks.max(1) as f64;
        let pg = page_bits(&d.page);
        let page_cells = if d.page.volumes == 0 { ("—".to_string(), "—".to_string()) } else { (format!("{:.0}", pg / nb), format!("{}/{}", d.page_ok, d.blocks)) };
        let _ = writeln!(out, "| {} | {} | {} | {} | {}/{} | {} | {:.0} | {} | {:.0} | {:.0} | {:.1} | {:.1} |", d.name, d.blocks, d.verses, d.words, d.gate_ok, d.blocks, page_cells.1, d.trans_bits() / nb, page_cells.0, d.total() / nb, 1000.0 * d.total() / d.words.max(1) as f64, d.trans_bits() / d.verses.max(1) as f64, 100.0 * d.chrf / d.units.max(1) as f64);
    }
    for d in dirs.iter().filter(|d| d.page.volumes > 0) {
        let g = d.page.derived.get("given").cloned().unwrap_or_default();
        let _ = writeln!(out, "\n{}: {} pieces of the pages given (translated texts and the source's own), of {} distinct texts there were to give.", d.name, g.pieces, g.free);
    }
    let _ = writeln!(out, "\nThe translation by stage and class group (bits per verse):\n\n| direction | reader | encoder | stack | lexical | construction | order | function | punct/case | morphology |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for d in &dirs {
        let nv = d.verses.max(1) as f64;
        let mut g = [0.0f64; 6];
        for r in &d.trans {
            for ((_, cl), cell) in &r.cells {
                g[crate::report::group(*cl)] += cell.bits;
            }
        }
        let _ = writeln!(out, "| {} | {:.1} | {:.1} | {:.1} | {} |", d.name, bits(&d.trans[0]) / nv, bits(&d.trans[1]) / nv, bits(&d.trans[2]) / nv, g.iter().map(|x| format!("{:.1}", x / nv)).collect::<Vec<_>>().join(" | "));
    }
    let _ = writeln!(out, "\nThe page around the texts (bits per block, by class; the translated and the source's texts given):\n\n| direction | markup | conllu | frames-table | unparsed | prose layers |\n|---|---:|---:|---:|---:|---:|");
    for d in dirs.iter().filter(|d| d.page.volumes > 0) {
        let nb = d.blocks.max(1) as f64;
        let cl = |x: Class| d.page.residual.cells.iter().filter(|(k, _)| k.1 == x).map(|(_, c)| c.bits).sum::<f64>() / nb;
        let (m, cn, f, u) = (cl(Class::Markup), cl(Class::Conllu), cl(Class::FrTable), cl(Class::Unparsed));
        let _ = writeln!(out, "| {} | {m:.0} | {cn:.0} | {f:.0} | {u:.0} | {:.0} |", d.name, page_bits(&d.page) / nb - m - cn - f - u);
    }
    let _ = writeln!(out, "\nThe translation's classes that cost most (bits per verse):\n");
    for d in &dirs {
        let mut cells: BTreeMap<(usize, String, String), f64> = BTreeMap::new();
        for (s, r) in d.trans.iter().enumerate() {
            for ((l, cl), cell) in &r.cells {
                *cells.entry((s, l.boundary().to_string(), cl.name().to_string())).or_default() += cell.bits;
            }
        }
        let mut v: Vec<_> = cells.into_iter().collect();
        v.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        let top: Vec<String> = v.iter().take(8).map(|((s, _, cl), b)| format!("{} {cl} {:.1}", crate::measure::STAGES[*s], b / d.verses.max(1) as f64)).collect();
        let _ = writeln!(out, "- **{}**: {}", d.name, top.join(", "));
        if !d.fails.is_empty() {
            let _ = writeln!(out, "  - gate failures: {}", d.fails.join("; "));
        }
        if let Some(f) = d.page.exact_fail.first() {
            let _ = writeln!(out, "  - page gate failure: {f}");
        }
    }
    Ok((out, rows))
}

#[allow(clippy::too_many_arguments)]
fn add_block(d: &mut Dir, rows: &mut String, b: &Block, st: [Residual; 3], ok: bool, n: u64, w: u64, t: &Tally) {
    d.blocks += 1;
    d.verses += n;
    d.words += w;
    d.gate_ok += ok as u64;
    d.page_ok += t.exact_ok;
    if !ok && d.fails.len() < 4 {
        d.fails.push(b.id.clone());
    }
    let _ = writeln!(rows, "{}\t{}\t{n}\t{w}\t{}\t{}\t{:.1}\t{:.1}\t{:.1}\t{:.1}\t{:.1}", d.name, b.id, ok as u8, t.exact_ok, bits(&st[0]), bits(&st[1]), bits(&st[2]), page_bits(t), st.iter().map(bits).sum::<f64>() + page_bits(t));
    for s in 0..3 {
        d.trans[s].merge(&st[s]);
    }
    d.page.merge(t);
}


#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn translation_and_page_costs() {
        use roundtrip::class::Layer;
        let mut d = Dir::default();
        d.trans[0].add(Layer::World, Class::Lexical, 3.0, String::new);
        d.page
            .residual
            .add(Layer::Tokens, Class::Layout, 2.0, String::new);
        d.page
            .residual
            .add(Layer::Tokens, Class::Meta, 7.0, String::new);
        assert_eq!(d.trans_bits(), 3.0);
        assert_eq!(page_bits(&d.page), 2.0);
        assert_eq!(d.total(), 5.0);
    }
}
