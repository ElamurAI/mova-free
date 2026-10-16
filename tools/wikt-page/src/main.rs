//! wikt-page — raw wikitext of pages by exact title from a Wikimedia multistream dump.
//!
//!   wikt-page $MOVA_DATA/raw/wiktionary/enwiktionary/20260901 language 'Template:en-noun'
//!
//! Looks the title up in `*-multistream-index*.bz2` (line `offset:page_id:title`), then decompresses
//! exactly one bz2 stream (~100 pages) from `offset` in the paired `*-multistream*.xml*.bz2` —
//! the rest of the dump is not touched. Titles are exact, with the namespace prefix as the wiki spells it
//! (`Template:`, `Module:`, or the local name on non-English wikis).
//! Output: `=== <title> | ns=… id=… rev=… | <file>` and the wikitext as is. Titles not found go to stderr, rc=1.

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use bzip2::read::{BzDecoder, MultiBzDecoder};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;

struct Page {
    ns: String,
    id: String,
    rev: String,
    text: String,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        bail!("usage: wikt-page <dump-dir> <title>...");
    }
    let dir = PathBuf::from(&args[0]);
    let titles = &args[1..];

    let mut indexes: Vec<PathBuf> = std::fs::read_dir(&dir)
        .with_context(|| format!("dir {}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.contains("-multistream-index") && n.ends_with(".bz2"))
        })
        .collect();
    indexes.sort();
    if indexes.is_empty() {
        bail!("no *-multistream-index*.bz2 in {}", dir.display());
    }

    // title → (xml file, stream offset); titles are unique within a wiki — stop as soon as all are found
    let mut found: HashMap<&str, Option<(PathBuf, u64)>> =
        titles.iter().map(|t| (t.as_str(), None)).collect();
    let mut left = found.len();
    'scan: for index in &indexes {
        let xml = xml_for_index(index)?;
        let reader = BufReader::with_capacity(1 << 20, MultiBzDecoder::new(File::open(index)?));
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.splitn(3, ':');
            let (Some(offset), Some(_), Some(title)) = (parts.next(), parts.next(), parts.next()) else {
                bail!("broken index line {}: {line}", index.display());
            };
            if let Some(slot @ None) = found.get_mut(title) {
                *slot = Some((xml.clone(), offset.parse()?));
                left -= 1;
                if left == 0 {
                    break 'scan;
                }
            }
        }
    }

    let mut out = io::stdout().lock();
    let mut missing = 0;
    for title in titles {
        let Some((xml, offset)) = &found[title.as_str()] else {
            eprintln!("not found: {title}");
            missing += 1;
            continue;
        };
        let chunk = read_stream(xml, *offset)?;
        let Some(page) = find_page(&chunk, title)? else {
            bail!("index points to {}@{offset}, but page \"{title}\" is not in that stream", xml.display());
        };
        let file = xml.file_name().unwrap_or_default().to_string_lossy();
        writeln!(out, "=== {title} | ns={} id={} rev={} | {file}", page.ns, page.id, page.rev)?;
        writeln!(out, "{}", page.text)?;
    }
    if missing > 0 {
        std::process::exit(1);
    }
    Ok(())
}

/// `X-multistream-index.txt.bz2` → `X-multistream.xml.bz2`;
/// `X-multistream-index1.txt-pApB.bz2` → `X-multistream1.xml-pApB.bz2`.
fn xml_for_index(index: &Path) -> Result<PathBuf> {
    let name = index.file_name().and_then(|n| n.to_str()).context("index file name")?;
    let xml = index.with_file_name(
        name.replacen("-multistream-index", "-multistream", 1).replacen(".txt", ".xml", 1),
    );
    if !xml.exists() {
        bail!("index {name} has no paired {}", xml.display());
    }
    Ok(xml)
}

/// One bz2 stream from `offset`: `BzDecoder` (not Multi) stops at the end of the stream.
fn read_stream(xml: &Path, offset: u64) -> Result<String> {
    let mut file = File::open(xml)?;
    file.seek(SeekFrom::Start(offset))?;
    let mut chunk = String::new();
    BzDecoder::new(BufReader::new(file)).read_to_string(&mut chunk)?;
    Ok(chunk)
}

/// A stream is a rootless fragment: a sequence of `<page>`, the last one also has `</mediawiki>`.
/// The first `<id>` of a page is its id, the first `<id>` inside `<revision>` is the revision (then the author id).
fn find_page(chunk: &str, want: &str) -> Result<Option<Page>> {
    let mut reader = Reader::from_str(chunk);
    reader.config_mut().check_end_names = false;
    let (mut title, mut in_rev) = (String::new(), false);
    let mut page = Page { ns: String::new(), id: String::new(), rev: String::new(), text: String::new() };
    loop {
        match reader.read_event()? {
            Event::Start(e) => match e.name().as_ref() {
                "page" => {
                    title.clear();
                    in_rev = false;
                    page = Page { ns: String::new(), id: String::new(), rev: String::new(), text: String::new() };
                }
                "revision" => in_rev = true,
                "title" => title = unescape(&reader.read_text(QName("title"))?)?,
                "ns" => page.ns = unescape(&reader.read_text(QName("ns"))?)?,
                "id" if !in_rev && page.id.is_empty() => page.id = unescape(&reader.read_text(QName("id"))?)?,
                "id" if in_rev && page.rev.is_empty() => page.rev = unescape(&reader.read_text(QName("id"))?)?,
                "text" => {
                    let text = unescape(&reader.read_text(QName("text"))?)?;
                    if title == want {
                        page.text = text;
                        return Ok(Some(page));
                    }
                }
                _ => {}
            },
            Event::Empty(e) if e.name().as_ref() == "text" && title == want => return Ok(Some(page)),
            Event::Eof => return Ok(None),
            _ => {}
        }
    }
}

fn unescape(raw: &str) -> Result<String> {
    Ok(quick_xml::escape::unescape(raw)?.into_owned())
}
