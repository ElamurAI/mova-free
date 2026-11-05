//! What the site's training blocks teach (`roundtrip site-learn`): the train split of the site's Markdown, sampled as
//! the runs sample it, gives
//! - **the markup dictionary** (`markup.tsv`): per section, the markup literals the train pages repeat (at least
//!   `MIN_COUNT` times), the most bytes saved first, at most `MAX_DICT`; the copier (`vol::Copier`) starts every
//!   block of that section with them, so a block's first page copies its front matter, links and markers too;
//! - **the Hebrew trees' lexicon** (`hebrew.tsv`, `site_heb`): MACULA's written tokens and their words as the train
//!   pages' right-to-left trees have them, so a Hebrew tree is written from its verse;
//! - **the English of the train pages** (`english.tsv`, every fourth prose sentence): training text for
//!   tools/translate's standard encoder (`translate eval --site-english`);
//! - **the punctuation table** (`punct.tsv`, `en::lin::PunctTable`): the marks `en` hangs on each kind of word of
//!   the train pages, for the frames' realizer (`--site-punct`);
//! - **the order generator adapted to the site** (`en-lin.bin`): `en`'s model with `lin.rs` trained again on its own
//!   training trees and on `en`'s parses of the train pages' prose sentences (`Model::adapt`, the sentences repeated
//!   `weight` times): the order the tree decoder writes is learned from the text it has to give back.
//!
//!   roundtrip site-learn --site-md <root> --out <dir> [--model <en.bin>] [--site-target N] [--threads N]
//!                        [--lin-weight W] [--lin-every K] [--no-lin] [--lin-shuffle]
//!
//! `--shuffle-tables` is the negative control of the tables: each section gets the next section's markup dictionary
//! and every Hebrew token another token's analysis. `--lin-shuffle` is the negative control: every sentence's words shuffled before it is parsed and learned from, so
//! the order learned is no order; the dev residual of the order class must rise with it.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use anyhow::{Context, Result};
use en::annotate::Annotator;

use super::site_md::{self, Sample};
use crate::class::Class;
use crate::vol::{self, Piece, Volume};

pub const MIN_COUNT: u32 = 3;
pub const MAX_DICT: usize = 4096;

/// The markup dictionary, by corpus (`site/<section>`), when one is loaded (`--site-dict`).
pub static SITE_DICT: OnceLock<BTreeMap<String, Vec<String>>> = OnceLock::new();

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\t', "\\t").replace('\n', "\\n")
}

fn unesc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    let mut it = s.chars();
    while let Some(c) = it.next() {
        if c == '\\' {
            match it.next() {
                Some('t') => o.push('\t'),
                Some('n') => o.push('\n'),
                Some(x) => o.push(x),
                None => o.push('\\'),
            }
        } else {
            o.push(c);
        }
    }
    o
}

/// The markup literals of a volume, in the order the copier codes them (pieces, then the blocks' inline marks).
pub fn markup_of(v: &Volume) -> Vec<String> {
    let mut out: Vec<String> = v.pieces.iter().filter_map(|p| if let Piece::Lit(x, Class::Markup) = p { Some(x.clone()) } else { None }).collect();
    for b in &v.blocks {
        out.extend(b.marks.iter().map(|m| m.1.clone()));
    }
    out
}

/// Load a markup dictionary (`section<TAB>literal`, escaped) for the runs.
pub fn load_dict(path: &Path) -> Result<()> {
    let t = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    let mut d: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for l in t.lines() {
        if let Some((s, x)) = l.split_once('\t') {
            d.entry(s.to_string()).or_default().push(unesc(x));
        }
    }
    let _ = SITE_DICT.set(d);
    Ok(())
}

/// The copier a volume starts with: its corpus's dictionary, if one is loaded.
pub fn copier_for(v: &Volume) -> vol::Copier {
    let corpus = v.name.split(':').next().unwrap_or("");
    match SITE_DICT.get().and_then(|d| d.get(corpus)) {
        Some(xs) => vol::Copier::seeded(xs),
        None => vol::Copier::default(),
    }
}

/// tools/translate's dev and test books (`bible::split_of`: places 3, 5 and 8 of every ten).
const HELD_OUT: [&str; 12] = ["NUM", "1SA", "2CH", "PSA", "JER", "JOL", "NAM", "MAL", "JOS", "NEH", "EZK", "ZEP"];

/// A page of the Tanakh corpus (`corpora/<part>/[jps/]<BOOK>.…`) of a held-out book.
pub fn held_out_book(id: &str) -> bool {
    let p: Vec<&str> = id.split('/').collect();
    p.first() == Some(&"corpora") && p.get(1).is_some_and(|x| matches!(*x, "torah" | "neviim" | "ketuvim")) && p.last().and_then(|n| n.split('.').next()).is_some_and(|b| HELD_OUT.contains(&b))
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned()
}

pub fn main(args: &[String], model: &Path) -> Result<()> {
    let root = PathBuf::from(flag(args, "--site-md").context("--site-md <root>")?);
    let out = PathBuf::from(flag(args, "--out").context("--out <dir>")?);
    let threads: usize = flag(args, "--threads").map(|x| x.parse()).transpose()?.unwrap_or(8);
    std::fs::create_dir_all(&out)?;
    let mut s = Sample { split: Some("train".into()), ..Default::default() };
    if let Some(n) = flag(args, "--site-target") {
        s.target = n.parse()?;
    }
    let t0 = std::time::Instant::now();
    let mut sets = site_md::blocks(&root, &s, threads)?;
    // the Tanakh's held-out books (tools/translate's dev and test) teach nothing, whatever their pages' split
    for (_, bs) in sets.iter_mut() {
        bs.retain(|b| !b.chapters.iter().any(|id| held_out_book(id)));
    }
    // the markup dictionary
    let mut dict = String::new();
    let mut n_lit = 0usize;
    for (corpus, bs) in &sets {
        let mut count: BTreeMap<&str, u32> = BTreeMap::new();
        let lits: Vec<Vec<String>> = bs.iter().map(|b| markup_of(&b.vol)).collect();
        for l in lits.iter().flatten() {
            *count.entry(l.as_str()).or_default() += 1;
        }
        let mut v: Vec<(&str, u32)> = count.into_iter().filter(|(x, n)| *n >= MIN_COUNT && x.len() >= 4).collect();
        v.sort_by(|a, b| (b.1 as usize * b.0.len()).cmp(&(a.1 as usize * a.0.len())).then(a.0.cmp(b.0)));
        v.truncate(MAX_DICT);
        n_lit += v.len();
        for (x, _) in v {
            dict.push_str(&format!("{corpus}\t{}\n", esc(x)));
        }
    }
    // the negative control: each section gets the next section's dictionary
    let dict = if args.iter().any(|a| a == "--shuffle-tables") {
        let secs: Vec<&str> = sets.iter().map(|x| x.0.as_str()).collect();
        dict.lines()
            .filter_map(|l| l.split_once('\t'))
            .map(|(sec, x)| {
                let i = secs.iter().position(|y| *y == sec).unwrap_or(0);
                format!("{}\t{x}\n", secs[(i + 1) % secs.len()])
            })
            .collect()
    } else {
        dict
    };
    std::fs::write(out.join("markup.tsv"), dict)?;
    // the Hebrew trees' lexicon
    let mut hl = super::site_heb::Learn::default();
    let mut nh = 0usize;
    for (_, bs) in &sets {
        for b in bs {
            for p in &b.vol.pieces {
                if let Piece::Lit(x, Class::Conllu) = p {
                    if x.lines().next().is_some_and(|l| l.contains("rtl")) {
                        hl.add(x);
                        nh += 1;
                    }
                }
            }
        }
    }
    let mut lex = hl.lexicon();
    // the negative control: every token gets another token's analysis
    if args.iter().any(|a| a == "--shuffle-tables") {
        for m in [&mut lex.tokens, &mut lex.bare] {
            let keys: Vec<String> = m.keys().cloned().collect();
            let vals: Vec<super::site_heb::Analysis> = m.values().cloned().collect();
            let n = vals.len().max(1);
            *m = keys.into_iter().enumerate().map(|(i, k)| (k, vals[(i + 7919) % n].clone())).collect();
        }
    }
    eprintln!("Hebrew lexicon: {nh} trees, {} tokens, {} without accents, {} cuts", lex.tokens.len(), lex.bare.len(), lex.cuts.len());
    std::fs::write(out.join("hebrew.tsv"), super::site_heb::write(&lex))?;
    eprintln!("markup dictionary: {n_lit} literals over {} sections ({:.1}s)", sets.len(), t0.elapsed().as_secs_f64());
    if args.iter().any(|a| a == "--no-lin") {
        return Ok(());
    }
    // the order generator, adapted on the train pages' prose sentences
    let weight: usize = flag(args, "--lin-weight").map(|x| x.parse()).transpose()?.unwrap_or(1);
    let every: usize = flag(args, "--lin-every").map(|x| x.parse()).transpose()?.unwrap_or(1);
    let mut a = Annotator::load(model)?;
    let mut sents: Vec<String> = Vec::new();
    // the punctuation table: which marks `en` hangs on which words of the train pages' sentences
    let mut punct = en::lin::PunctTable::default();
    for (_, bs) in &sets {
        for b in bs.iter().step_by(every.max(1)) {
            for (bi, blk) in b.vol.blocks.iter().enumerate() {
                for st in vol::segment(&a, &blk.text, bi, blk.whole, None)?.sents {
                    if !st.forms.is_empty() {
                        let ws = a.annotate(&st.forms);
                        let forms: Vec<&str> = ws.iter().map(|w| w.form.as_str()).collect();
                        let (tags, rels, heads): (Vec<_>, Vec<_>, Vec<_>) = (ws.iter().map(|w| w.tag).collect(), ws.iter().map(|w| w.rel).collect(), ws.iter().map(|w| w.head).collect());
                        punct.learn(&forms, &tags, &rels, &heads);
                    }
                    if st.forms.len() >= 3 {
                        if args.iter().any(|x| x == "--lin-shuffle") {
                            let mut f = st.forms.clone();
                            langseg::Rng(0x11e5 ^ langseg::fnv(&st.original())).shuffle(&mut f);
                            sents.push(f.join(" "));
                        } else {
                            sents.push(st.original());
                        }
                    }
                }
            }
        }
    }
    std::fs::write(out.join("punct.tsv"), punct.write())?;
    eprintln!("punctuation table: written {}", out.join("punct.tsv").display());
    if args.iter().any(|a| a == "--punct-only") {
        return Ok(());
    }
    eprintln!("order generator: {} prose sentences of the train pages, weight {weight} ({:.1}s)", sents.len(), t0.elapsed().as_secs_f64());
    // every 4th of them, for tools/translate's standard encoder (`--site-english`)
    let pool: String = sents.iter().enumerate().filter(|(i, _)| i % 4 == 0).map(|(i, x)| format!("site:{i}\t{}\n", x.replace(['\t', '\n'], " "))).collect();
    std::fs::write(out.join("english.tsv"), pool)?;
    a.model.adapt(&sents, weight);
    a.save(&out.join("en-lin.bin"))?;
    eprintln!("written {} ({:.1}s)", out.join("en-lin.bin").display(), t0.elapsed().as_secs_f64());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escaped_literals_come_back() {
        for s in ["a\tb\nc\\d", "](https://x/)", "\\", ""] {
            assert_eq!(unesc(&esc(s)), s);
        }
    }
}
