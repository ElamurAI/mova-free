//! arXivRaw (OAI-PMH) → catalog TSV.
//!
//! A record: `<header>` (identifier, datestamp, setSpec…, status="deleted") and `<metadata><arXivRaw>`:
//! id, submitter, version[@version]{date,size,source_type}, title, authors, categories (the first is
//! primary), comments, proxy, report-no, msc-class, acm-class, journal-ref, doi, license, abstract.
//! The same id from several harvests (sets overlap) — the record with the later datestamp is kept.
//! License is `License` (an enum, not a string): arXiv URL string → variant, variant → code and class.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::reader::Reader;

/// arXiv licenses (https://info.arxiv.org/help/license/index.html) and legacy variants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum License {
    CcBy4,
    CcBy3,
    CcBySa4,
    Cc0,
    /// The legacy mark `creativecommons.org/licenses/publicdomain/` (pre-CC0).
    PdOld,
    CcByNcSa4,
    CcByNcSa3,
    CcByNcNd4,
    /// arXiv.org perpetual, non-exclusive license 1.0 — the standard one (distribution on arXiv only).
    ArxivNonexclusive,
    /// Papers before ~2004: no field, the arXiv distribution consent applies (like the standard one).
    None,
    Other,
}

/// License class for decisions: what may be used to train a closed model.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    /// CC BY, CC BY-SA, CC0, public domain — training and redistribution under the license terms.
    Open,
    /// NC variants — reading and research only.
    Nc,
    /// The standard arXiv license and papers without the field — reading and research only.
    Arxiv,
    Other,
}

impl License {
    pub fn parse(url: &str) -> License {
        let u = url.trim().trim_end_matches('/').to_ascii_lowercase();
        let u = u.split_once("://").map_or(u.as_str(), |(_, r)| r).trim_start_matches("www.");
        match u {
            "" => License::None,
            "creativecommons.org/licenses/by/4.0" => License::CcBy4,
            "creativecommons.org/licenses/by/3.0" => License::CcBy3,
            "creativecommons.org/licenses/by-sa/4.0" => License::CcBySa4,
            "creativecommons.org/publicdomain/zero/1.0" => License::Cc0,
            "creativecommons.org/licenses/publicdomain" => License::PdOld,
            "creativecommons.org/licenses/by-nc-sa/4.0" => License::CcByNcSa4,
            "creativecommons.org/licenses/by-nc-sa/3.0" => License::CcByNcSa3,
            "creativecommons.org/licenses/by-nc-nd/4.0" => License::CcByNcNd4,
            "arxiv.org/licenses/nonexclusive-distrib/1.0" => License::ArxivNonexclusive,
            _ => License::Other,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            License::CcBy4 => "cc-by-4.0",
            License::CcBy3 => "cc-by-3.0",
            License::CcBySa4 => "cc-by-sa-4.0",
            License::Cc0 => "cc0-1.0",
            License::PdOld => "pd-cc-old",
            License::CcByNcSa4 => "cc-by-nc-sa-4.0",
            License::CcByNcSa3 => "cc-by-nc-sa-3.0",
            License::CcByNcNd4 => "cc-by-nc-nd-4.0",
            License::ArxivNonexclusive => "arxiv-1.0",
            License::None => "none",
            License::Other => "other",
        }
    }

    pub fn class(self) -> Class {
        match self {
            License::CcBy4 | License::CcBy3 | License::CcBySa4 | License::Cc0 | License::PdOld => Class::Open,
            License::CcByNcSa4 | License::CcByNcSa3 | License::CcByNcNd4 => Class::Nc,
            License::ArxivNonexclusive | License::None => Class::Arxiv,
            License::Other => Class::Other,
        }
    }
}

impl Class {
    pub fn code(self) -> &'static str {
        match self {
            Class::Open => "open",
            Class::Nc => "nc",
            Class::Arxiv => "arxiv",
            Class::Other => "other",
        }
    }
}

/// Old math archives before the merge into `math` (1998) → modern category.
fn norm_cat(c: &str) -> &str {
    match c {
        "alg-geom" => "math.AG",
        "dg-ga" => "math.DG",
        "funct-an" => "math.FA",
        "q-alg" => "math.QA",
        _ => c,
    }
}

#[derive(Default)]
struct Rec {
    id: String,
    datestamp: String,
    sets: Vec<String>,
    deleted: bool,
    submitter: String,
    versions: Vec<(String, String, u64, String)>,
    title: String,
    authors: String,
    categories: String,
    comments: String,
    msc: String,
    acm: String,
    journal_ref: String,
    doi: String,
    license: String,
    abstract_: String,
}

/// Whitespace and newlines → a single space; tabs in TSV are impossible.
fn clean(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `16kb` → 16; `1.5mb` → 1536; bytes → KB (rounded up).
fn size_kb(s: &str) -> u64 {
    let s = s.trim().to_ascii_lowercase();
    let (num, mult) = if let Some(n) = s.strip_suffix("kb") {
        (n, 1.0)
    } else if let Some(n) = s.strip_suffix("mb") {
        (n, 1024.0)
    } else if let Some(n) = s.strip_suffix("gb") {
        (n, 1024.0 * 1024.0)
    } else if let Some(n) = s.strip_suffix('b') {
        (n, 1.0 / 1024.0)
    } else {
        (s.as_str(), 1.0)
    };
    num.trim().parse::<f64>().map_or(0, |v| (v * mult).ceil() as u64)
}

/// `Wed, 01 Oct 1997 11:48:56 GMT` → `1997-10-01`.
fn iso_date(s: &str) -> String {
    let p: Vec<&str> = s.split_whitespace().collect();
    if p.len() < 4 {
        return String::new();
    }
    let m = match p[2] {
        "Jan" => 1,
        "Feb" => 2,
        "Mar" => 3,
        "Apr" => 4,
        "May" => 5,
        "Jun" => 6,
        "Jul" => 7,
        "Aug" => 8,
        "Sep" => 9,
        "Oct" => 10,
        "Nov" => 11,
        "Dec" => 12,
        _ => 0,
    };
    match (p[3].parse::<u32>(), p[1].parse::<u32>()) {
        (Ok(y), Ok(d)) if m > 0 => format!("{y:04}-{m:02}-{d:02}"),
        _ => String::new(),
    }
}

/// Page count from the comment: "14 pages", "12pp", "25 pages, 3 figures".
fn pages(comments: &str, re: &regex::Regex) -> Option<u32> {
    re.captures(comments).and_then(|c| c[1].parse().ok()).filter(|&n| n > 0 && n < 5000)
}

fn parse_page(xml: &str, out: &mut Vec<Rec>) -> Result<()> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().check_end_names = false;
    let mut rec = Rec::default();
    let mut in_rec = false;
    loop {
        match reader.read_event()? {
            Event::Start(e) => {
                let name = e.local_name();
                let name = name.as_ref();
                match name {
                    "record" => {
                        rec = Rec::default();
                        in_rec = true;
                    }
                    "header" => {
                        if let Some(a) = e.try_get_attribute("status")? {
                            rec.deleted = a.value.as_ref() == "deleted";
                        }
                    }
                    "version" => {
                        let v = e.try_get_attribute("version")?.map(|a| a.value.to_string()).unwrap_or_default();
                        rec.versions.push((v, String::new(), 0, String::new()));
                    }
                    _ if in_rec => {
                        let q = e.name().as_ref().to_string();
                        let leaf = matches!(
                            name,
                            "identifier" | "datestamp" | "setSpec" | "id" | "submitter" | "date" | "size" | "source_type"
                                | "title" | "authors" | "categories" | "comments" | "msc-class" | "acm-class"
                                | "journal-ref" | "doi" | "license" | "abstract" | "proxy" | "report-no"
                        );
                        if leaf {
                            let raw = reader.read_text(QName(&q))?;
                            let text = quick_xml::escape::unescape(&raw).map(|c| c.into_owned()).unwrap_or_else(|_| raw.to_string());
                            match name {
                                "datestamp" => rec.datestamp = text.trim().to_string(),
                                "setSpec" => rec.sets.push(text.trim().to_string()),
                                "id" => rec.id = text.trim().to_string(),
                                "submitter" => rec.submitter = clean(&text),
                                "date" => {
                                    if let Some(v) = rec.versions.last_mut() {
                                        v.1 = iso_date(&text);
                                    }
                                }
                                "size" => {
                                    if let Some(v) = rec.versions.last_mut() {
                                        v.2 = size_kb(&text);
                                    }
                                }
                                "source_type" => {
                                    if let Some(v) = rec.versions.last_mut() {
                                        v.3 = text.trim().to_string();
                                    }
                                }
                                "title" => rec.title = clean(&text),
                                "authors" => rec.authors = clean(&text),
                                "categories" => rec.categories = clean(&text),
                                "comments" => rec.comments = clean(&text),
                                "msc-class" => rec.msc = clean(&text),
                                "acm-class" => rec.acm = clean(&text),
                                "journal-ref" => rec.journal_ref = clean(&text),
                                "doi" => rec.doi = clean(&text),
                                "license" => rec.license = text.trim().to_string(),
                                "abstract" => rec.abstract_ = clean(&text),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            Event::End(e) if e.local_name().as_ref() == "record" => {
                in_rec = false;
                if rec.id.is_empty() {
                    // deleted record: id only in the header
                    // (oai:arXiv.org:ID)
                }
                out.push(std::mem::take(&mut rec));
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(())
}

fn page_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut v: Vec<PathBuf> = fs::read_dir(dir)
        .with_context(|| format!("{}", dir.display()))?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with('p') && n.ends_with(".xml.gz")))
        .collect();
    v.sort();
    Ok(v)
}

pub const HEADER: &str = "id\tprimary_cat\tcats\tn_cats\tlicense\tlic_class\tlicense_url\tyear\tfirst_date\tlast_date\tversions\tsize_kb\tsize_kb_all\tpages\tmsc\tacm\tdoi\tjournal_ref\ttitle\tauthors\tabstract\tcomments\tsubmitter\tsource_type\tdatestamp\tsets\tharvest";

pub fn run(out: &Path, dirs: &[&str]) -> Result<()> {
    let re = regex::Regex::new(r"(?i)\b(\d{1,4})\s*(?:pages?|pp\.?|pgs?)\b")?;
    let mut best: HashMap<String, (Rec, String)> = HashMap::new();
    let (mut seen, mut deleted, mut pages_read) = (0u64, 0u64, 0u64);
    for d in dirs {
        let dir = Path::new(d);
        let tag = dir.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
        for f in page_files(dir)? {
            let mut s = String::new();
            GzDecoder::new(File::open(&f)?).read_to_string(&mut s).with_context(|| format!("{}", f.display()))?;
            let mut recs = Vec::new();
            parse_page(&s, &mut recs).with_context(|| format!("{}", f.display()))?;
            pages_read += 1;
            for r in recs {
                seen += 1;
                if r.deleted || r.id.is_empty() {
                    deleted += 1;
                    continue;
                }
                match best.get_mut(&r.id) {
                    Some((old, tags)) => {
                        if !tags.split(',').any(|t| t == tag) {
                            tags.push(',');
                            tags.push_str(&tag);
                        }
                        if r.datestamp > old.datestamp {
                            *old = r;
                        }
                    }
                    None => {
                        best.insert(r.id.clone(), (r, tag.clone()));
                    }
                }
            }
        }
    }
    let mut ids: Vec<&String> = best.keys().collect();
    ids.sort();
    let mut w = BufWriter::new(File::create(out)?);
    writeln!(w, "{HEADER}")?;
    let mut by_lic: HashMap<&'static str, u64> = HashMap::new();
    for id in ids {
        let (r, tags) = &best[id];
        let cats: Vec<&str> = r.categories.split_whitespace().collect();
        let primary = cats.first().map_or("", |c| norm_cat(c));
        let lic = License::parse(&r.license);
        *by_lic.entry(lic.code()).or_default() += 1;
        let first = r.versions.first().map_or("", |v| v.1.as_str());
        let last = r.versions.last().map_or("", |v| v.1.as_str());
        let size_last = r.versions.last().map_or(0, |v| v.2);
        let size_all: u64 = r.versions.iter().map(|v| v.2).sum();
        let st = r.versions.last().map_or("", |v| v.3.as_str());
        let pg = pages(&r.comments, &re).map_or(String::new(), |n| n.to_string());
        writeln!(
            w,
            "{}\t{primary}\t{}\t{}\t{}\t{}\t{}\t{}\t{first}\t{last}\t{}\t{size_last}\t{size_all}\t{pg}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{st}\t{}\t{}\t{tags}",
            r.id,
            cats.iter().map(|c| norm_cat(c)).collect::<Vec<_>>().join(" "),
            cats.len(),
            lic.code(),
            lic.class().code(),
            r.license,
            first.get(..4).unwrap_or(""),
            r.versions.len(),
            r.msc,
            r.acm,
            r.doi,
            r.journal_ref,
            r.title,
            r.authors,
            r.abstract_,
            r.comments,
            r.submitter,
            r.datestamp,
            r.sets.join(" "),
        )?;
    }
    w.flush()?;
    let mut lic: Vec<_> = by_lic.into_iter().collect();
    lic.sort_by(|a, b| b.1.cmp(&a.1));
    println!(
        "PARSE {pages_read} pages, {seen} records, {deleted} deleted, {} unique → {}",
        best.len(),
        out.display()
    );
    for (l, n) in lic {
        println!("  {l}\t{n}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn licenses_sizes_dates() {
        assert_eq!(License::parse("http://creativecommons.org/licenses/by/4.0/"), License::CcBy4);
        assert_eq!(License::parse("http://creativecommons.org/licenses/by-sa/4.0/"), License::CcBySa4);
        assert_eq!(License::parse("http://creativecommons.org/publicdomain/zero/1.0/"), License::Cc0);
        assert_eq!(License::parse("http://arxiv.org/licenses/nonexclusive-distrib/1.0/"), License::ArxivNonexclusive);
        assert_eq!(License::parse(""), License::None);
        assert_eq!(License::parse("http://creativecommons.org/licenses/by-nc-sa/4.0/").class(), Class::Nc);
        // negative control: an unknown license does not become open
        assert_eq!(License::parse("http://example.org/licenses/by/4.0/").class(), Class::Other);
        assert_eq!(License::parse("http://arxiv.org/licenses/nonexclusive-distrib/1.0/").class(), Class::Arxiv);
        assert_eq!(size_kb("16kb"), 16);
        assert_eq!(size_kb("1.5mb"), 1536);
        assert_eq!(size_kb("2048b"), 2);
        assert_eq!(iso_date("Wed, 01 Oct 1997 11:48:56 GMT"), "1997-10-01");
        assert_eq!(iso_date("garbage"), "");
        let re = regex::Regex::new(r"(?i)\b(\d{1,4})\s*(?:pages?|pp\.?|pgs?)\b").unwrap();
        assert_eq!(pages("LaTeX2e. 14 pages", &re), Some(14));
        assert_eq!(pages("12pp, 3 figures", &re), Some(12));
        assert_eq!(pages("no count", &re), None);
    }

    #[test]
    fn parses_record() {
        let xml = r#"<OAI-PMH><ListRecords><record><header><identifier>oai:arXiv.org:2101.00001</identifier><datestamp>2021-01-05</datestamp><setSpec>math:math:HO</setSpec></header>
<metadata><arXivRaw xmlns="http://arxiv.org/OAI/arXivRaw/"><id>2101.00001</id><submitter>A B</submitter>
<version version="v1"><date>Fri, 01 Jan 2021 10:00:00 GMT</date><size>120kb</size></version>
<version version="v2"><date>Mon, 04 Jan 2021 10:00:00 GMT</date><size>130kb</size></version>
<title>On   the &#34;integral&#34; &amp; more</title><authors>A B</authors><categories>math.HO math.CA</categories>
<license>http://creativecommons.org/licenses/by/4.0/</license><abstract>  x  &lt; y </abstract></arXivRaw></metadata></record>
<record><header status="deleted"><identifier>oai:arXiv.org:2101.00002</identifier><datestamp>2021-01-06</datestamp></header></record>
</ListRecords></OAI-PMH>"#;
        let mut v = Vec::new();
        parse_page(xml, &mut v).unwrap();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].id, "2101.00001");
        assert_eq!(v[0].title, "On the \"integral\" & more");
        assert_eq!(v[0].abstract_, "x < y");
        assert_eq!(v[0].versions.len(), 2);
        assert_eq!(v[0].versions[1].2, 130);
        assert_eq!(v[0].versions[0].1, "2021-01-01");
        assert_eq!(v[0].sets, vec!["math:math:HO"]);
        assert!(v[1].deleted);
    }
}
