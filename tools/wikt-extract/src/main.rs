//! wikt-extract — the English entries of the English Wiktionary, from the raw multistream dump, as a Mova add-on
//! dictionary module (CC BY-SA 4.0 and GFDL, distributed apart from Mova's core).
//!
//!   wikt-extract page  DUMP_DIR TITLE…              the rows of some pages (index lookup, one stream each)
//!   wikt-extract build DUMP_DIR OUT_DIR [--threads N] [--version V]
//!                                                   the full pass: OUT_DIR/{wiktionary-en.tsv, report.json,
//!                                                   gaps.tsv, module.json}
//!   wikt-extract check MODULE_DIR                   every file of module.json present with its SHA-256 and size
//!   wikt-extract cover LAYER WORDS                  which words of a list (one per line) have an entry, exact or in
//!                                                   another letter case, with their parts of speech and first meaning
//!
//! The dump is read as wikt-page reads it (`tools/wikt-page`): the index `offset:page_id:title` of each part names
//! the bz2 streams (about 100 pages each); each stream is decompressed on its own, so the pass runs on a fixed
//! number of threads with bounded memory (no decompressed XML on disk). Rows are sorted by word (lower case; the
//! lower-case title before other spellings), title and order in the page, so the output does not depend on
//! threads. Every row carries its page id, revision id, dump file and stream offset.

mod entry;
mod wikitext;

use anyhow::{Context, Result, bail, ensure};
use bzip2::read::{BzDecoder, MultiBzDecoder};
use quick_xml::Reader;
use quick_xml::events::Event;
use quick_xml::name::QName;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

pub const LAYER_FILE: &str = "wiktionary-en.tsv";
pub const LICENCE: &str = "CC-BY-SA-4.0 AND GFDL-1.3-or-later";

struct Part {
    xml: PathBuf,
    file: String,
    offsets: Vec<u64>,
}

/// `X-multistream-index1.txt-pApB.bz2` → `X-multistream1.xml-pApB.bz2` (the rule of `tools/wikt-page`).
fn xml_for_index(index: &Path) -> Result<PathBuf> {
    let name = index.file_name().and_then(|n| n.to_str()).context("index file name")?;
    let xml = index.with_file_name(name.replacen("-multistream-index", "-multistream", 1).replacen(".txt", ".xml", 1));
    ensure!(xml.exists(), "index {name} has no paired {}", xml.display());
    Ok(xml)
}

fn indexes(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| format!("dir {}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.contains("-multistream-index") && n.ends_with(".bz2")))
        .collect();
    v.sort();
    ensure!(!v.is_empty(), "no *-multistream-index*.bz2 in {}", dir.display());
    Ok(v)
}

/// Every part with its distinct stream offsets, in file order; `want` (titles) restricts to the streams holding them.
fn parts(dir: &Path, want: Option<&[String]>) -> Result<Vec<Part>> {
    let mut out = Vec::new();
    for index in indexes(dir)? {
        let xml = xml_for_index(&index)?;
        let mut offsets: Vec<u64> = Vec::new();
        let r = BufReader::with_capacity(1 << 20, MultiBzDecoder::new(File::open(&index)?));
        for line in r.lines() {
            let line = line?;
            let mut it = line.splitn(3, ':');
            let (Some(off), Some(_), Some(title)) = (it.next(), it.next(), it.next()) else { bail!("broken index line in {}: {line}", index.display()) };
            if want.is_some_and(|w| !w.iter().any(|t| t == title)) {
                continue;
            }
            let off: u64 = off.parse()?;
            if offsets.last() != Some(&off) {
                offsets.push(off);
            }
        }
        offsets.dedup();
        let file = xml.file_name().unwrap_or_default().to_string_lossy().to_string();
        out.push(Part { xml, file, offsets });
    }
    Ok(out)
}

struct Page {
    title: String,
    ns: String,
    id: u64,
    rev: u64,
    text: String,
}

/// The pages of one decompressed stream (a rootless fragment of `<page>` elements).
fn pages(chunk: &str) -> Result<Vec<Page>> {
    let mut reader = Reader::from_str(chunk);
    reader.config_mut().check_end_names = false;
    reader.config_mut().allow_unmatched_ends = true;
    let mut out = Vec::new();
    let mut in_rev = false;
    let mut p = Page { title: String::new(), ns: String::new(), id: 0, rev: 0, text: String::new() };
    let un = |s: &str| -> Result<String> { Ok(quick_xml::escape::unescape(s)?.into_owned()) };
    loop {
        match reader.read_event()? {
            Event::Start(e) => match e.name().as_ref() {
                "page" => {
                    in_rev = false;
                    p = Page { title: String::new(), ns: String::new(), id: 0, rev: 0, text: String::new() };
                }
                "revision" => in_rev = true,
                "title" => p.title = un(&reader.read_text(QName("title"))?)?,
                "ns" => p.ns = un(&reader.read_text(QName("ns"))?)?,
                "id" if !in_rev && p.id == 0 => p.id = reader.read_text(QName("id"))?.trim().parse()?,
                "id" if in_rev && p.rev == 0 => p.rev = reader.read_text(QName("id"))?.trim().parse()?,
                "text" => p.text = un(&reader.read_text(QName("text"))?)?,
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == "page" => {
                out.push(std::mem::replace(&mut p, Page { title: String::new(), ns: String::new(), id: 0, rev: 0, text: String::new() }));
            }
            Event::Eof => return Ok(out),
            _ => {}
        }
    }
}

fn read_stream(xml: &Path, offset: u64) -> Result<String> {
    let mut f = File::open(xml)?;
    f.seek(SeekFrom::Start(offset))?;
    let mut s = String::new();
    BzDecoder::new(BufReader::new(f)).read_to_string(&mut s)?;
    Ok(s)
}

/// Counts of a pass (report.json).
#[derive(Default, Debug, serde::Serialize)]
struct Stats {
    streams: u64,
    pages: u64,
    ns0_pages: u64,
    english_pages: u64,
    entries: u64,
    senses: u64,
    subsenses: u64,
    examples: u64,
    labelled_senses: u64,
    form_of_senses: u64,
    synonyms: u64,
    antonyms: u64,
    entries_with_forms: u64,
    quotations_counted_not_taken: u64,
    entries_with_gaps: u64,
    by_pos: BTreeMap<String, u64>,
    gaps: BTreeMap<String, u64>,
    dropped: BTreeMap<String, u64>,
}

impl Stats {
    fn add(&mut self, o: Stats) {
        for (a, b) in [
            (&mut self.streams, o.streams), (&mut self.pages, o.pages), (&mut self.ns0_pages, o.ns0_pages), (&mut self.english_pages, o.english_pages),
            (&mut self.entries, o.entries), (&mut self.senses, o.senses), (&mut self.subsenses, o.subsenses), (&mut self.examples, o.examples),
            (&mut self.labelled_senses, o.labelled_senses), (&mut self.form_of_senses, o.form_of_senses), (&mut self.synonyms, o.synonyms),
            (&mut self.antonyms, o.antonyms), (&mut self.entries_with_forms, o.entries_with_forms), (&mut self.quotations_counted_not_taken, o.quotations_counted_not_taken),
            (&mut self.entries_with_gaps, o.entries_with_gaps),
        ] {
            *a += b;
        }
        for (m, n) in [(&mut self.by_pos, o.by_pos), (&mut self.gaps, o.gaps), (&mut self.dropped, o.dropped)] {
            for (k, v) in n {
                *m.entry(k).or_default() += v;
            }
        }
    }

    fn count(&mut self, e: &entry::Entry) {
        self.entries += 1;
        *self.by_pos.entry(e.pos.clone()).or_default() += 1;
        for s in &e.senses {
            self.senses += 1;
            self.subsenses += (s.depth > 1) as u64;
            self.examples += s.examples.len() as u64;
            self.labelled_senses += (!s.labels.is_empty()) as u64;
            self.form_of_senses += s.form_of.is_some() as u64;
            self.synonyms += s.nyms.iter().filter(|x| x.0 == "syn").count() as u64;
            self.antonyms += s.nyms.iter().filter(|x| x.0 == "ant").count() as u64;
        }
        self.synonyms += e.nyms.iter().filter(|x| x.0 == "syn").count() as u64;
        self.antonyms += e.nyms.iter().filter(|x| x.0 == "ant").count() as u64;
        self.entries_with_forms += !e.forms.is_empty() as u64;
        self.quotations_counted_not_taken += e.quotes as u64;
        self.entries_with_gaps += !e.gaps.is_empty() as u64;
        for (k, v) in &e.gaps {
            *self.gaps.entry(k.clone()).or_default() += *v as u64;
        }
        for (k, v) in &e.dropped {
            *self.dropped.entry(k.clone()).or_default() += *v as u64;
        }
    }
}

/// The sort key of a row: word in lower case, the lower-case spelling first, title, order in the page.
type Key = (String, bool, String, u32);

fn sort_key(title: &str, k: u32) -> Key {
    let low = title.to_lowercase();
    let other = low != title;
    (low, other, title.to_string(), k)
}

/// Extract the rows of one stream's pages.
fn stream_rows(part: &Part, offset: u64, rows: &mut Vec<(Key, String)>, st: &mut Stats, only: Option<&[String]>) -> Result<()> {
    let chunk = read_stream(&part.xml, offset).with_context(|| format!("{}@{offset}", part.file))?;
    st.streams += 1;
    for p in pages(&chunk).with_context(|| format!("{}@{offset}", part.file))? {
        st.pages += 1;
        if p.ns != "0" || only.is_some_and(|w| !w.contains(&p.title)) {
            continue;
        }
        st.ns0_pages += 1;
        let Some(sec) = entry::english(&p.text) else { continue };
        st.english_pages += 1;
        let prov = entry::Prov { page_id: p.id, rev: p.rev, file: part.file.clone(), offset };
        for (k, e) in entry::entries(&p.title, sec).iter().enumerate() {
            st.count(e);
            rows.push((sort_key(&p.title, k as u32), entry::row(e, &prov)));
        }
    }
    Ok(())
}

/// Rows of all streams on `threads` threads, sorted.
fn run(parts: &[Part], threads: usize, only: Option<&[String]>) -> Result<(Vec<(Key, String)>, Stats)> {
    let jobs: Vec<(usize, u64)> = parts.iter().enumerate().flat_map(|(i, p)| p.offsets.iter().map(move |o| (i, *o))).collect();
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let results: Mutex<Vec<(Vec<(Key, String)>, Stats)>> = Mutex::new(Vec::new());
    let errors: Mutex<Vec<String>> = Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                let (mut rows, mut st) = (Vec::new(), Stats::default());
                loop {
                    let j = next.fetch_add(1, Ordering::Relaxed);
                    let Some(&(pi, off)) = jobs.get(j) else { break };
                    if let Err(e) = stream_rows(&parts[pi], off, &mut rows, &mut st, only) {
                        errors.lock().unwrap().push(format!("{e:#}"));
                        // fail-fast: stop taking streams
                        next.store(jobs.len(), Ordering::Relaxed);
                        break;
                    }
                    let d = done.fetch_add(1, Ordering::Relaxed) + 1;
                    if d % 10_000 == 0 {
                        eprintln!("{d}/{} streams", jobs.len());
                    }
                }
                results.lock().unwrap().push((rows, st));
            });
        }
    });
    let errors = errors.into_inner().unwrap();
    ensure!(errors.is_empty(), "the pass stopped: {}", errors.join("; "));
    let mut all = Vec::new();
    let mut stats = Stats::default();
    for (r, st) in results.into_inner().unwrap() {
        all.extend(r);
        stats.add(st);
    }
    all.sort_by(|a, b| a.0.cmp(&b.0));
    Ok((all, stats))
}

fn sha256_file(p: &Path) -> Result<(String, u64)> {
    let mut f = File::open(p).with_context(|| format!("reading {}", p.display()))?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 20];
    let mut n = 0u64;
    loop {
        let k = f.read(&mut buf)?;
        if k == 0 {
            break;
        }
        n += k as u64;
        h.update(&buf[..k]);
    }
    Ok((format!("{:x}", h.finalize()), n))
}

/// The layer's header: the licence line (dictstore's `shippable` and the release export refuse a CC BY-SA header)
/// and the column names.
pub fn header(dump: &str) -> String {
    format!(
        "# License: CC BY-SA 4.0 and GFDL 1.3 or later. Wiktionary contributors, https://en.wiktionary.org/ — dump {dump}; changes: extracted and reformatted by Mova (tools/wikt-extract). An add-on module distributed separately under these terms; never part of Mova's core.\n# {}\n",
        entry::COLUMNS.join("\t")
    )
}

/// The dump's name from its files: `enwiktionary-20260901-…` → `enwiktionary 20260901`.
fn dump_name(parts: &[Part]) -> String {
    let f = &parts[0].file;
    let mut it = f.split('-');
    format!("{} {}", it.next().unwrap_or(""), it.next().unwrap_or(""))
}

fn peak_rss_kb() -> u64 {
    std::fs::read_to_string("/proc/self/status").ok().and_then(|s| s.lines().find(|l| l.starts_with("VmHWM:")).and_then(|l| l.split_whitespace().nth(1)?.parse().ok())).unwrap_or(0)
}

fn build(dump: &Path, out: &Path, threads: usize, version: &str) -> Result<()> {
    let t0 = std::time::Instant::now();
    ensure!(!out.join("module.json").exists(), "{} already holds a module: a version is written once", out.display());
    ensure!(!inside_git(out), "{} is inside a Git work tree: a CC BY-SA module never enters Mova's repository", out.display());
    std::fs::create_dir_all(out)?;
    let parts = parts(dump, None)?;
    let name = dump_name(&parts);
    let (rows, st) = run(&parts, threads, None)?;
    let secs = t0.elapsed().as_secs_f64();
    let layer = out.join(LAYER_FILE);
    let mut w = std::io::BufWriter::new(File::create(&layer)?);
    w.write_all(header(&name).as_bytes())?;
    for (_, r) in &rows {
        w.write_all(r.as_bytes())?;
        w.write_all(b"\n")?;
    }
    w.flush()?;
    drop(w);
    let mut gaps: Vec<(&String, &u64)> = st.gaps.iter().collect();
    gaps.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let mut g = String::from("# templates with no handler in the English entries' definition, example and head lines: occurrences (their text is left out of the rows; each row names its own in the gaps column)\n");
    for (k, v) in &gaps {
        g.push_str(&format!("{k}\t{v}\n"));
    }
    std::fs::write(out.join("gaps.tsv"), g)?;
    let report = serde_json::json!({
        "dump": name, "dump_dir": dump.file_name().map(|x| x.to_string_lossy().to_string()), "files": parts.iter().map(|p| p.file.clone()).collect::<Vec<_>>(),
        "rows": rows.len(), "threads": threads, "seconds": (secs * 10.0).round() / 10.0, "peak_rss_mb": peak_rss_kb() / 1024, "stats": st,
    });
    std::fs::write(out.join("report.json"), serde_json::to_string_pretty(&report)? + "\n")?;
    let mut files = Vec::new();
    for f in [LAYER_FILE, "gaps.tsv", "report.json"] {
        let (h, n) = sha256_file(&out.join(f))?;
        files.push(serde_json::json!({"path": f, "sha256": h, "bytes": n}));
    }
    let dumpdate = name.split(' ').nth(1).unwrap_or("");
    let module = serde_json::json!({
        "schema": 1, "name": "wiktionary-en", "version": version, "kind": "dictionary-layer", "licence": LICENCE, "licence_class": "cc-by-sa",
        "attribution": format!("Wiktionary contributors, https://en.wiktionary.org/ — dump enwiktionary {dumpdate}; changes: extracted and reformatted by Mova"),
        "files": files, "provides": [{"kind": "dictionary-layer", "layer": "wiktionary-en", "path": LAYER_FILE}], "requires_mova": ">=0.5",
    });
    std::fs::write(out.join("module.json"), serde_json::to_string_pretty(&module)? + "\n")?;
    eprintln!("{} rows, {:.1} s, peak RSS {} MB → {}", rows.len(), secs, peak_rss_kb() / 1024, out.display());
    Ok(())
}

/// Coverage of a word list (one word per line, first tab field) by a layer: per word whether a row has exactly
/// that title, else one whose title matches in lower case, the parts of speech and the first meaning; then the
/// totals. Streams the layer; holds only the list.
fn cover(layer: &Path, words: &Path) -> Result<()> {
    let list: Vec<String> = std::fs::read_to_string(words)?.lines().filter_map(|l| l.split('\t').next()).map(|w| w.trim().to_string()).filter(|w| !w.is_empty()).collect();
    let low: BTreeMap<String, Vec<usize>> = list.iter().enumerate().fold(BTreeMap::new(), |mut m, (i, w)| {
        m.entry(w.to_lowercase()).or_insert_with(Vec::new).push(i);
        m
    });
    // per word: (exact rows, case-folded rows, parts of speech, first meaning)
    let mut hit: Vec<(u32, u32, Vec<String>, String)> = vec![(0, 0, Vec::new(), String::new()); list.len()];
    let r = BufReader::with_capacity(1 << 20, File::open(layer)?);
    for line in r.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }
        let mut c = line.split('\t');
        let (Some(t), Some(pos), Some(m)) = (c.next(), c.next(), c.next()) else { continue };
        let Some(ix) = low.get(&t.to_lowercase()) else { continue };
        for &i in ix {
            let h = &mut hit[i];
            if t == list[i] { h.0 += 1 } else { h.1 += 1 }
            if !h.2.iter().any(|p| p == pos) {
                h.2.push(pos.to_string());
            }
            if h.3.is_empty() || t == list[i] && h.0 == 1 {
                h.3 = m.split(" 2) ").next().unwrap_or("").chars().take(160).collect();
            }
        }
    }
    let (mut exact, mut folded) = (0, 0);
    for (w, h) in list.iter().zip(&hit) {
        let k = if h.0 > 0 { exact += 1; "exact" } else if h.1 > 0 { folded += 1; "case" } else { "none" };
        println!("{w}\t{k}\t{}\t{}", h.2.join(","), h.3);
    }
    println!("# {} words: {exact} with an entry of that exact spelling, {folded} more in another letter case, {} none", list.len(), list.len() - exact - folded);
    Ok(())
}

/// Whether a directory (existing or not: its nearest existing ancestor, canonicalized) lies in a tree holding `.git`.
fn inside_git(dir: &Path) -> bool {
    let abs = std::env::current_dir().map(|c| c.join(dir)).unwrap_or_else(|_| dir.to_path_buf());
    let Some(d) = abs.ancestors().find_map(|a| a.canonicalize().ok()) else { return false };
    d.ancestors().any(|a| a.join(".git").exists())
}

fn check(dir: &Path) -> Result<()> {
    let m: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(dir.join("module.json"))?)?;
    for f in m["files"].as_array().context("module.json: files")? {
        let p = f["path"].as_str().context("a file path")?;
        let (h, n) = sha256_file(&dir.join(p))?;
        ensure!(Some(h.as_str()) == f["sha256"].as_str() && Some(n) == f["bytes"].as_u64(), "{p}: SHA-256 or size differs from module.json");
        println!("ok\t{p}\t{n}\t{h}");
    }
    Ok(())
}

fn main() -> Result<()> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let flag = |k: &str| a.iter().position(|x| x == k).and_then(|i| a.get(i + 1)).cloned();
    match a.first().map(String::as_str) {
        Some("page") if a.len() >= 3 => {
            let titles = &a[2..];
            let ps = parts(Path::new(&a[1]), Some(titles))?;
            let (rows, _) = run(&ps, 1, Some(titles))?;
            for (_, r) in rows {
                println!("{r}");
            }
        }
        Some("build") if a.len() >= 3 => {
            let threads = flag("--threads").map(|t| t.parse()).transpose()?.unwrap_or(6);
            let version = flag("--version").unwrap_or_else(|| "20260901-1".into());
            build(Path::new(&a[1]), Path::new(&a[2]), threads, &version)?;
        }
        Some("check") if a.len() >= 2 => check(Path::new(&a[1]))?,
        Some("cover") if a.len() >= 3 => cover(Path::new(&a[1]), Path::new(&a[2]))?,
        _ => bail!("usage: wikt-extract page DUMP_DIR TITLE… | build DUMP_DIR OUT_DIR [--threads N] [--version V] | check MODULE_DIR | cover LAYER WORDS"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pages_of_a_stream() {
        let xml = "<page><title>a&amp;b</title><ns>0</ns><id>7</id><revision><id>8</id><contributor><id>9</id></contributor><text>==English==\n===Noun===\n# A thing.</text></revision></page><page><title>T</title><ns>10</ns><id>1</id><revision><id>2</id><text/></revision></page></mediawiki>";
        let ps = pages(xml).unwrap();
        assert_eq!(ps.len(), 2);
        assert_eq!((ps[0].title.as_str(), ps[0].id, ps[0].rev), ("a&b", 7, 8));
        assert_eq!(ps[1].text, "");
    }

    #[test]
    fn rows_sort_lower_case_first_and_header_is_cc_by_sa() {
        let mut v = vec![sort_key("Cat", 0), sort_key("cat", 1), sort_key("cat", 0), sort_key("CAT", 0)];
        v.sort();
        assert_eq!(v.iter().map(|k| (k.2.as_str(), k.3)).collect::<Vec<_>>(), vec![("cat", 0), ("cat", 1), ("CAT", 0), ("Cat", 0)]);
        let h = header("enwiktionary 20260901");
        assert!(h.lines().next().unwrap().to_lowercase().contains("cc by-sa"));
        assert_eq!(h.lines().nth(1).unwrap().split('\t').count(), entry::COLUMNS.len());
        // the repository guard and its negative control
        assert!(inside_git(Path::new(env!("CARGO_MANIFEST_DIR"))));
        assert!(!inside_git(Path::new("/")));
        assert!(inside_git(&Path::new(env!("CARGO_MANIFEST_DIR")).join("not-yet/made")));
    }
}
