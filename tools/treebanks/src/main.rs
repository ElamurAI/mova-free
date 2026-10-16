//! treebanks — UD treebank cards and the papers about them.
//!
//! Design note: a separate treebanks folder, one folder per language and per treebank, holding the
//! seeds for each treebank and the list of scientific papers about it; those papers are always
//! downloaded and processed as usual.
//!
//!   treebanks build <ud-treebanks-vX.tgz> <ud-docs/docs/treebanks> <anthology.bib[.gz]> <papers-dir> <out-dir> <run-dir>
//!
//! - The release is read as a stream: README, stats.xml, size of each `.conllu` per split.
//!   Gate: size = release stats.xml for every split of every treebank, otherwise an error with nothing written.
//!   Gate: every README has metadata (License, Genre, the five layers, Contributors, Data available since).
//! - `<out>/<lang>/<treebank>/README.md` — the card; `papers.md` — the papers (see `refs.rs`);
//!   `<out>/<lang>/README.md` — the language's treebanks; `<out>/INDEX.md` — all of them.
//! - `<run>/missing-ids.txt` — ACL ids from the lists that are not yet in the library (input to `sources bib-ids`);
//!   `<run>/refs.jsonl` — every reference item with its result; `<run>/title-matches.tsv` — title and
//!   bibtex matches for review by eye.
//!
//! Seeds (`seeds/`) and converters (`convert.md`) are written by others — build does not touch them.

mod readme;
mod refs;
mod release;
mod render;

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use serde_json::json;
use sources::bib;

use readme::{Item, Readme, has_year};
use refs::{How, Index, Matcher, Outcome, Src, detex, words};
use render::{Card, Found, Unfound};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("build") if args.len() == 7 => build(&Paths {
            tgz: args[1].as_ref(),
            docs: args[2].as_ref(),
            bib: args[3].as_ref(),
            papers: args[4].as_ref(),
            out: args[5].as_ref(),
            run: args[6].as_ref(),
        }),
        _ => bail!("usage: treebanks build <ud-treebanks-vX.tgz> <ud-docs/docs/treebanks> <anthology.bib[.gz]> <papers-dir> <out-dir> <run-dir>"),
    }
}

struct Paths<'a> {
    tgz: &'a Path,
    docs: &'a Path,
    bib: &'a Path,
    papers: &'a Path,
    out: &'a Path,
    run: &'a Path,
}

const REQUIRED: [&str; 9] = ["License", "Genre", "Lemmas", "UPOS", "XPOS", "Features", "Relations", "Contributors", "Data available since"];

/// The item is a reference to a paper (not just a paragraph): bibtex, an ACL id or DOI, or, in a references
/// section, a line with a year and ≥ 6 words that does not end with a colon.
fn citation_like(it: &Item, o: &Outcome) -> bool {
    it.bib.is_some()
        || !o.acl_ids.is_empty()
        || !o.dois.is_empty()
        || (it.in_refs && has_year(&it.raw) && it.raw.split_whitespace().count() >= 6 && !it.raw.trim_end().ends_with(':'))
}

fn short(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let cut: String = s.chars().take(max).collect();
    format!("{}…", cut.trim_end())
}

/// Excerpt of a long paragraph outside the references section around a DOI or id: ~200 chars before and ~100 after.
fn excerpt(raw: &str, needle: &str) -> String {
    let chars: Vec<char> = raw.chars().collect();
    if chars.len() <= 320 {
        return raw.to_string();
    }
    let low = raw.to_lowercase();
    let Some(pos) = low.find(&needle.to_lowercase()).filter(|_| !needle.is_empty()) else { return short(raw, 320) };
    // position in chars (lowercasing may change the byte length — count via the prefix)
    let at = low[..pos].chars().count().min(chars.len());
    let a = at.saturating_sub(200);
    let b = (at + needle.chars().count() + 100).min(chars.len());
    let mut s: String = chars[a..b].iter().collect();
    if a > 0 {
        s = format!("…{}", s.trim_start());
    }
    if b < chars.len() {
        s = format!("{}…", s.trim_end());
    }
    s
}

/// Duplicate key of an item: explicit ids and DOIs, otherwise the words of the text (bibtex — words of the title).
fn item_key(it: &Item, o: &Outcome) -> String {
    if !o.acl_ids.is_empty() || !o.dois.is_empty() {
        let mut k: Vec<&String> = o.acl_ids.iter().chain(&o.dois).collect();
        k.sort();
        return format!("id:{}", k.iter().map(|x| x.as_str()).collect::<Vec<_>>().join(" "));
    }
    if let Some(e) = &it.bib {
        return format!("bib:{}", words(&detex(e.get("title"))).join(" "));
    }
    format!("txt:{}", words(&it.raw).join(" "))
}

/// Display of a bibtex item: "Authors (year). Title. Venue."
fn bib_line(e: &bib::Entry) -> String {
    let venue = [e.get("booktitle"), e.get("journal"), e.get("publisher")].into_iter().find(|v| !v.is_empty()).unwrap_or("");
    let mut s = format!("{} ({}). {}.", e.get("author").replace(" and ", "; "), e.get("year"), detex(e.get("title")));
    if !venue.is_empty() {
        let _ = write!(s, " {venue}.");
    }
    for k in ["doi", "url"] {
        if !e.get(k).is_empty() {
            let _ = write!(s, " {}", e.get(k));
        }
    }
    let _ = write!(s, " (bibtex `{}`)", e.key);
    s
}

#[derive(Default)]
struct Tally {
    items: usize,
    items_lib: usize,
    items_acl: usize,
    items_none: usize,
    how: BTreeMap<String, usize>,
    unknown_ids: usize,
}

struct Refs {
    found: Vec<Found>,
    unfound: Vec<Unfound>,
}

fn collect(m: &Matcher, tb_dir: &str, docs: &[(&str, &Readme)], audit: &mut String, review: &mut String, tally: &mut Tally) -> Refs {
    let mut found: Vec<Found> = Vec::new();
    let mut unfound: Vec<Unfound> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    for (origin, r) in docs {
        for it in r.items() {
            let o = m.item(&it.text, it.bib.as_ref());
            if o.hits.is_empty() && !citation_like(&it, &o) {
                continue;
            }
            if !seen.insert(item_key(&it, &o)) {
                continue;
            }
            tally.items += 1;
            tally.unknown_ids += o.unknown_ids.len();
            if o.hits.iter().any(|h| h.src == Src::Lib) {
                tally.items_lib += 1;
            } else if !o.hits.is_empty() {
                tally.items_acl += 1;
            } else {
                tally.items_none += 1;
            }
            let hits_json: Vec<_> = o
                .hits
                .iter()
                .map(|h| {
                    let p = m.paper(h.src, h.idx);
                    json!({"src": format!("{:?}", h.src), "how": format!("{:?}", h.how), "anthology": p.anthology, "folder": p.folder, "year": p.year, "title": p.title})
                })
                .collect();
            let near_json: Vec<_> = o
                .near
                .iter()
                .map(|n| {
                    let p = m.paper(n.src, n.idx);
                    json!({"src": format!("{:?}", n.src), "why": n.why, "anthology": p.anthology, "folder": p.folder, "year": p.year, "title": p.title})
                })
                .collect();
            let _ = writeln!(
                audit,
                "{}",
                json!({"treebank": tb_dir, "origin": origin, "in_refs": it.in_refs, "lines": [it.lines.0 + 1, it.lines.1], "bibtex": it.bib.is_some(), "raw": it.raw, "hits": hits_json, "acl_ids": o.acl_ids, "dois": o.dois, "unknown_ids": o.unknown_ids, "near": near_json})
            );
            for h in &o.hits {
                let label = if h.fuzzy_year { format!("{}, year ±1", h.how.label()) } else { h.how.label().to_string() };
                *tally.how.entry(label.clone()).or_default() += 1;
                let p = m.paper(h.src, h.idx);
                if matches!(h.how, How::Title | How::Bibtex) {
                    let _ = writeln!(
                        review,
                        "{tb_dir}\t{label}\t{:?}\t{}\t{}\t{}\t{}",
                        h.src,
                        if p.anthology.is_empty() { &p.folder } else { &p.anthology },
                        p.year,
                        p.title,
                        short(&it.raw, 300).replace('\t', " ")
                    );
                }
                match found.iter_mut().find(|f| f.src == h.src && f.idx == h.idx) {
                    Some(f) => {
                        // a more precise method and an exact year from any item win
                        if (h.how, h.fuzzy_year) < (f.how, f.fuzzy_year) {
                            f.how = h.how;
                            f.fuzzy_year = h.fuzzy_year;
                        }
                    }
                    None => found.push(Found { src: h.src, idx: h.idx, how: h.how, fuzzy_year: h.fuzzy_year, alts: h.alts.clone() }),
                }
            }
            let mut notes: Vec<String> = Vec::new();
            if !o.unknown_ids.is_empty() {
                notes.push(format!("ACL id {} is not in the ACL Anthology bibliography", o.unknown_ids.iter().map(|x| format!("`{x}`")).collect::<Vec<_>>().join(", ")));
            }
            if o.hits.is_empty() {
                for n in o.near.iter().take(2) {
                    let p = m.paper(n.src, n.idx);
                    let where_ = match n.src {
                        Src::Lib => format!("library `{}`", p.folder),
                        Src::Acl => format!("ACL `{}`", p.anthology),
                    };
                    let year = if p.year == 0 { "no year".to_string() } else { p.year.to_string() };
                    notes.push(format!("similar paper: \"{}\" ({year}, {where_}) — not confirmed: {}", p.title, n.why));
                }
                let raw = match &it.bib {
                    Some(e) => bib_line(e),
                    None if !it.in_refs => excerpt(&it.raw, o.dois.first().or(o.acl_ids.first()).map_or("", String::as_str)),
                    None => short(&it.raw, 600),
                };
                unfound.push(Unfound { raw, note: notes.join("; ") });
            } else if !o.unknown_ids.is_empty() {
                unfound.push(Unfound { raw: short(&it.raw, 600), note: notes.join("; ") });
            }
        }
    }
    Refs { found, unfound }
}

fn read_bib(path: &Path) -> Result<String> {
    if path.extension().is_some_and(|e| e == "gz") {
        let mut s = String::new();
        flate2::read::GzDecoder::new(fs::File::open(path).with_context(|| path.display().to_string())?).read_to_string(&mut s)?;
        Ok(s)
    } else {
        fs::read_to_string(path).with_context(|| path.display().to_string())
    }
}

fn build(p: &Paths) -> Result<()> {
    let t0 = Instant::now();
    let (root, tbs) = release::read(p.tgz)?;
    let release = root.trim_start_matches("ud-treebanks-v").to_string();
    println!("release {root}: {} treebanks; {:.0} s", tbs.len(), t0.elapsed().as_secs_f64());
    // gate: size = stats.xml
    let errs: Vec<String> = tbs.iter().flat_map(release::check_stats).collect();
    if !errs.is_empty() {
        for e in &errs {
            eprintln!("STATS {e}");
        }
        bail!("{} size mismatches with stats.xml — nothing written", errs.len());
    }
    // gate: metadata
    let readmes: Vec<Readme> = tbs.iter().map(|t| Readme::parse(&readme::strip_jekyll(&t.readme))).collect();
    let mut bad = Vec::new();
    for (t, r) in tbs.iter().zip(&readmes) {
        let miss: Vec<&str> = REQUIRED.iter().copied().filter(|k| r.meta(k).is_empty()).collect();
        if t.readme.is_empty() || !miss.is_empty() {
            bad.push(format!("{} ({}): missing {}", t.dir, if t.readme_name.is_empty() { "no README" } else { &t.readme_name }, miss.join(", ")));
        }
    }
    if !bad.is_empty() {
        for b in &bad {
            eprintln!("META {b}");
        }
        bail!("{} READMEs without complete metadata — nothing written", bad.len());
    }
    // indexes
    let t1 = Instant::now();
    let entries = bib::parse(&read_bib(p.bib)?);
    let acl = Index::acl(&entries);
    drop(entries);
    let lib = Index::library(p.papers)?;
    println!("ACL Anthology: {} papers with ids; library: {} papers ({} with ACL ids); {:.0} s", acl.papers.len(), lib.papers.len(), lib.by_id.len(), t1.elapsed().as_secs_f64());
    let m = Matcher::new(&lib, &acl);
    // treebanks
    let (mut audit, mut review) = (String::new(), String::from("treebank\thow\tsrc\tid\tyear\ttitle\traw\n"));
    let mut tally = Tally::default();
    let mut cards: Vec<Card> = Vec::new();
    for (t, r) in tbs.iter().zip(&readmes) {
        let (lang_code, name) = t.lang_and_name();
        let lang = t.dir.trim_start_matches("UD_").split_once('-').map_or(t.dir.as_str(), |x| x.0).replace('_', " ");
        let docs_path = p.docs.join(&t.code).join("index.md");
        let docs_text = fs::read_to_string(&docs_path).ok();
        let (docs_body, family) = docs_text.as_deref().map(readme::docs_page).unwrap_or_default();
        let docs_readme = Readme::parse(&docs_body);
        let mut sources_: Vec<(&str, &Readme)> = vec![("readme", r)];
        if docs_text.is_some() {
            sources_.push(("docs", &docs_readme));
        }
        let refs = collect(&m, &t.dir, &sources_, &mut audit, &mut review, &mut tally);
        cards.push(Card {
            tb: t,
            readme: r,
            lang,
            lang_code: lang_code.to_string(),
            name: name.to_string(),
            family,
            has_docs: docs_text.is_some(),
            found: refs.found,
            unfound: refs.unfound,
        });
    }
    // languages: one code — one name
    let mut by_lang: BTreeMap<&str, Vec<&Card>> = BTreeMap::new();
    for c in &cards {
        by_lang.entry(c.lang_code.as_str()).or_default().push(c);
    }
    for (code, cs) in &by_lang {
        let names: BTreeSet<&str> = cs.iter().map(|c| c.lang.as_str()).collect();
        if names.len() > 1 {
            println!("WARNING: code {code} has several language names: {names:?} — the language index uses the first");
        }
    }
    // write
    let papers_link = format!("../../../{}/", p.papers.file_name().map(|x| x.to_string_lossy().into_owned()).unwrap_or_else(|| "papers".into()));
    for c in &cards {
        let dir = p.out.join(&c.lang_code).join(&c.name);
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("README.md"), c.readme_md(&release))?;
        fs::write(dir.join("papers.md"), c.papers_md(&m, &papers_link, &release))?;
    }
    for (code, cs) in &by_lang {
        fs::write(p.out.join(code).join("README.md"), render::lang_md(cs, &release))?;
    }
    // paper summary
    let mut uniq: BTreeSet<(Src, usize)> = BTreeSet::new();
    let (mut pairs_lib, mut pairs_acl, mut unfound) = (0usize, 0usize, 0usize);
    for c in &cards {
        for f in &c.found {
            uniq.insert((f.src, f.idx));
            if f.src == Src::Lib {
                pairs_lib += 1;
            } else {
                pairs_acl += 1;
            }
        }
        unfound += c.unfound.len();
    }
    let missing: BTreeSet<&str> = uniq.iter().filter(|x| x.0 == Src::Acl).map(|x| m.paper(x.0, x.1).anthology.as_str()).collect();
    let uniq_lib = uniq.iter().filter(|x| x.0 == Src::Lib).count();
    let stats = format!(
        "Papers in the lists: {} distinct — {uniq_lib} in the library, {} in the ACL Anthology outside the library; {unfound} references outside the ACL Anthology.",
        uniq.len(),
        missing.len()
    );
    let archive = p.tgz.file_name().map(|x| x.to_string_lossy().into_owned()).unwrap_or_default();
    fs::write(p.out.join("INDEX.md"), render::index_md(&cards, &release, &archive, &stats))?;
    // run files
    fs::create_dir_all(p.run)?;
    fs::write(p.run.join("missing-ids.txt"), missing.iter().map(|x| format!("{x}\n")).collect::<String>())?;
    fs::write(p.run.join("refs.jsonl"), audit)?;
    fs::write(p.run.join("title-matches.tsv"), review)?;
    println!(
        "cards: {} treebanks, {} languages → {}\nreference items: {} — library {}, ACL only {}, no match {}; unknown ACL ids {}\nmethods: {:?}\ntreebank–paper pairs: library {pairs_lib}, ACL outside the library {pairs_acl}; distinct papers {} (library {uniq_lib}, ACL {}); in the \"outside ACL\" lists {unfound}\n{:.0} s",
        cards.len(),
        by_lang.len(),
        p.out.display(),
        tally.items,
        tally.items_lib,
        tally.items_acl,
        tally.items_none,
        tally.unknown_ids,
        tally.how,
        uniq.len(),
        missing.len(),
        t0.elapsed().as_secs_f64()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excerpts_and_citation_filter() {
        let long = format!("{} see https://doi.org/10.1000/xyz for details. {}", "a ".repeat(300), "b ".repeat(200));
        let e = excerpt(&long, "10.1000/xyz");
        assert!(e.starts_with('…') && e.ends_with('…') && e.contains("10.1000/xyz") && e.chars().count() < 330, "{e}");
        assert_eq!(excerpt("short text", "x"), "short text");
        // an item in the references section with a year is a reference; a description without a year or ending with a colon is not
        let r = Readme::parse("# References\n\nSmith, J. (2010). A title about treebanks and parsing. Journal.\n\nPlease cite the following paper from 2010 as follows:\n\nThis treebank is great and has many sentences in it.\n");
        let items = r.items();
        let o = Outcome::default();
        let flags: Vec<bool> = items.iter().map(|it| citation_like(it, &o)).collect();
        assert_eq!(flags, vec![true, false, false]);
    }
}
