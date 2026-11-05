//! Output: one fact row per treebank (`cards.tsv`) and the index `INDEX.md` rendered from those rows.
//! No README prose is carried: only metadata values, sizes, links and paper match ids.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::readme::Readme;
use crate::refs::{How, Matcher, Src};
use crate::release::{Counts, SPLITS, Treebank};

/// A paper from a treebank's list.
pub struct Found {
    pub src: Src,
    pub idx: usize,
    pub how: How,
    /// the year in the reference differs by 1 from the paper's year
    pub fuzzy_year: bool,
    /// ACL ids of other records with the same title, year and author
    pub alts: Vec<String>,
}

/// A reference without a match: raw line and a note (kept in the run files only).
pub struct Unfound {
    pub raw: String,
    pub note: String,
}

/// Everything about a treebank for output.
pub struct Card<'a> {
    pub tb: &'a Treebank,
    pub readme: &'a Readme,
    pub lang: String,
    pub lang_code: String,
    pub name: String,
    pub family: String,
    pub has_docs: bool,
    pub found: Vec<Found>,
    pub unfound: Vec<Unfound>,
}

/// Columns of `cards.tsv`, in order. Sizes (`train`, `dev`, `test`, `total`) are five numbers separated by
/// spaces: sentences, tokens, words, multiword tokens, empty nodes; an absent split is empty.
pub const COLUMNS: [&str; 26] = [
    "ud_name", "lang", "language", "family", "treebank", "ud_code", "release", "license", "genres", "includes_text", "parallel", "since",
    "authors", "lemmas", "upos", "xpos", "features", "relations", "train", "dev", "test", "total", "ud_page", "readme", "papers", "unmatched_refs",
];

/// One treebank: a row of `cards.tsv`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub f: BTreeMap<&'static str, String>,
}

pub fn num(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(' ');
        }
        out.push(c);
    }
    out
}

fn cell(s: &str) -> String {
    let s = s.trim();
    if s.is_empty() { "—".into() } else { s.replace('|', "\\|") }
}

/// A TSV value: one line, no tabs, trimmed.
fn flat(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// `de Marneffe, Marie-Catherine; Guillaume, Bruno` → `Marie-Catherine de Marneffe; Bruno Guillaume`.
pub fn contributors(s: &str) -> String {
    s.split(';')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(|a| match a.split_once(',') {
            Some((last, first)) if !first.contains(',') && !first.trim().is_empty() => format!("{} {}", first.trim(), last.trim()),
            _ => a.to_string(),
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn yes_no(v: &str) -> String {
    match v.trim().to_lowercase().as_str() {
        "yes" => "yes".into(),
        "no" => "no".into(),
        _ => v.to_string(),
    }
}

fn counts(c: &Counts) -> String {
    format!("{} {} {} {} {}", c.sentences, c.tokens, c.words, c.fused, c.empty)
}

/// Match method code in `papers`: `url`, `doi`, `bibtex`, `title`; `~y` marks a README year off by one.
fn how_code(how: How, fuzzy_year: bool) -> String {
    let h = match how {
        How::Url => "url",
        How::Doi => "doi",
        How::Bibtex => "bibtex",
        How::Title => "title",
    };
    if fuzzy_year { format!("{h}~y") } else { h.to_string() }
}

impl Card<'_> {
    pub fn ud_url(&self) -> String {
        format!("https://universaldependencies.org/treebanks/{}/index.html", self.tb.code)
    }

    pub fn meta(&self, k: &str) -> &str {
        self.readme.meta(k)
    }

    /// The fact row. Papers are `<ACL id>|<how>` (or `lib:<library folder>|<how>` when the library paper has no ACL
    /// id), sorted by year and title; other records of the same paper (ACL id or library folder) follow as `|=<id>`. `|` keeps the id a
    /// separate token for readers that look for ACL ids in text.
    pub fn row(&self, m: &Matcher, release: &str) -> Row {
        let t = self.tb;
        let mut found: Vec<&Found> = self.found.iter().collect();
        found.sort_by(|a, b| {
            let (pa, pb) = (m.paper(a.src, a.idx), m.paper(b.src, b.idx));
            (pa.year, &pa.title, &pa.anthology, &pa.folder).cmp(&(pb.year, &pb.title, &pb.anthology, &pb.folder))
        });
        let papers: Vec<String> = found
            .iter()
            .map(|f| {
                let p = m.paper(f.src, f.idx);
                let id = if p.anthology.is_empty() { format!("lib:{}", p.folder) } else { p.anthology.clone() };
                let alts: String = f.alts.iter().map(|a| format!("|={a}")).collect();
                format!("{id}|{}{alts}", how_code(f.how, f.fuzzy_year))
            })
            .collect();
        let split = |sp| t.splits.get(&sp).map(counts).unwrap_or_default();
        let vals: [(&'static str, String); 26] = [
            ("ud_name", t.dir.clone()),
            ("lang", self.lang_code.clone()),
            ("language", self.lang.clone()),
            ("family", self.family.clone()),
            ("treebank", self.name.clone()),
            ("ud_code", t.code.clone()),
            ("release", release.to_string()),
            ("license", self.meta("License").to_string()),
            ("genres", self.meta("Genre").split_whitespace().collect::<Vec<_>>().join(", ")),
            ("includes_text", yes_no(self.meta("Includes text"))),
            ("parallel", self.meta("Parallel").to_string()),
            ("since", self.meta("Data available since").to_string()),
            ("authors", contributors(self.meta("Contributors"))),
            ("lemmas", self.meta("Lemmas").to_string()),
            ("upos", self.meta("UPOS").to_string()),
            ("xpos", self.meta("XPOS").to_string()),
            ("features", self.meta("Features").to_string()),
            ("relations", self.meta("Relations").to_string()),
            ("train", split(SPLITS[0])),
            ("dev", split(SPLITS[1])),
            ("test", split(SPLITS[2])),
            ("total", counts(&t.total())),
            ("ud_page", if self.has_docs { self.ud_url() } else { String::new() }),
            ("readme", if t.readme_name.is_empty() { String::new() } else { format!("{}/{}", t.dir, t.readme_name) }),
            ("papers", papers.join(" ")),
            ("unmatched_refs", self.unfound.len().to_string()),
        ];
        Row { f: vals.into_iter().map(|(k, v)| (k, flat(&v))).collect() }
    }
}

impl Row {
    pub fn get(&self, k: &str) -> &str {
        self.f.get(k).map_or("", String::as_str)
    }

    /// Five numbers of a size column; `None` for an absent split.
    pub fn size(&self, k: &str) -> Option<[u64; 5]> {
        let v: Vec<u64> = self.get(k).split_whitespace().filter_map(|x| x.parse().ok()).collect();
        <[u64; 5]>::try_from(v).ok()
    }

    pub fn line(&self) -> String {
        COLUMNS.iter().map(|k| self.get(k)).collect::<Vec<_>>().join("\t")
    }
}

/// `cards.tsv`: header and rows sorted by language code and treebank name.
pub fn tsv(rows: &[Row]) -> String {
    let mut rows: Vec<&Row> = rows.iter().collect();
    rows.sort_by(|a, b| (a.get("lang"), a.get("treebank"), a.get("ud_name")).cmp(&(b.get("lang"), b.get("treebank"), b.get("ud_name"))));
    let mut s = COLUMNS.join("\t") + "\n";
    for r in rows {
        s.push_str(&r.line());
        s.push('\n');
    }
    s
}

/// Parse `cards.tsv`: the header must equal `COLUMNS`; every row must have every column and valid sizes.
pub fn parse_tsv(text: &str) -> Result<Vec<Row>> {
    let mut lines = text.lines();
    let head = lines.next().unwrap_or("");
    if head != COLUMNS.join("\t") {
        bail!("cards.tsv: header is not {:?}", COLUMNS.join(" "));
    }
    let mut out = Vec::new();
    for (i, l) in lines.enumerate() {
        let cells: Vec<&str> = l.split('\t').collect();
        if cells.len() != COLUMNS.len() {
            bail!("cards.tsv line {}: {} columns, expected {}", i + 2, cells.len(), COLUMNS.len());
        }
        let r = Row { f: COLUMNS.iter().copied().zip(cells.iter().map(|c| c.to_string())).collect() };
        for k in ["train", "dev", "test", "total"] {
            if !r.get(k).is_empty() && r.size(k).is_none() {
                bail!("cards.tsv line {}: {k} is not five numbers: {:?}", i + 2, r.get(k));
            }
        }
        if r.size("total").is_none() {
            bail!("cards.tsv line {}: no total", i + 2);
        }
        out.push(r);
    }
    Ok(out)
}

/// The global index from the rows. `dir` is the treebanks folder: a treebank with its own folder there (seeds,
/// converter) gets links to what the folder holds.
pub fn index_md(rows: &[Row], dir: &Path) -> Result<String> {
    let add = |a: &mut [u64; 5], b: [u64; 5]| a.iter_mut().zip(b).for_each(|(x, y)| *x += y);
    let mut total = [0u64; 5];
    for r in rows {
        add(&mut total, r.size("total").with_context(|| format!("{}: no total", r.get("ud_name")))?);
    }
    let langs: BTreeSet<&str> = rows.iter().map(|r| r.get("lang")).collect();
    let releases: BTreeSet<&str> = rows.iter().map(|r| r.get("release")).collect();
    let release = releases.into_iter().collect::<Vec<_>>().join(", ");
    let mut papers: BTreeSet<&str> = BTreeSet::new();
    let (mut pairs, mut unmatched) = (0usize, 0usize);
    for r in rows {
        for p in r.get("papers").split_whitespace() {
            papers.insert(p.split('|').next().unwrap_or(p));
            pairs += 1;
        }
        unmatched += r.get("unmatched_refs").parse::<usize>().unwrap_or(0);
    }
    let mut s = format!("# UD {release} treebanks — index\n\n");
    let _ = writeln!(
        s,
        "Treebanks {}, languages (codes) {}; sentences {}, tokens {}, words {}. Papers matched: {} distinct, {pairs} treebank–paper pairs; {unmatched} references without a match.\n\nRendered by `tools/treebanks` from [cards.tsv](cards.tsv), one fact row per treebank (licence, genres, annotation layers, sizes per split, links, paper match ids). Folder layout: [README.md](README.md).\n",
        rows.len(),
        langs.len(),
        num(total[0]),
        num(total[1]),
        num(total[2]),
        papers.len()
    );
    let mut lic: BTreeMap<String, usize> = BTreeMap::new();
    for r in rows {
        *lic.entry(cell(r.get("license"))).or_default() += 1;
    }
    let mut lic: Vec<(String, usize)> = lic.into_iter().collect();
    lic.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    s.push_str("## Licenses\n\n| License | Treebanks |\n|---|---:|\n");
    for (l, n) in lic {
        let _ = writeln!(s, "| {l} | {n} |");
    }
    s.push_str("\n## Languages and treebanks\n\n| Language | Code | Treebank | Sentences | Tokens | License | Papers | In Mova |\n|---|---|---|---:|---:|---|---:|---|\n");
    let mut order: Vec<&Row> = rows.iter().collect();
    order.sort_by(|a, b| (a.get("language"), a.get("treebank"), a.get("ud_name")).cmp(&(b.get("language"), b.get("treebank"), b.get("ud_name"))));
    for r in order {
        let t = r.size("total").unwrap_or_default();
        let (lang, name) = (r.get("lang"), r.get("treebank"));
        let page = if r.get("ud_page").is_empty() { name.to_string() } else { format!("[{name}]({})", r.get("ud_page")) };
        // an anchor per row, `INDEX.md#en-ewt`, for links from elsewhere
        let tb = format!("<a id=\"{lang}-{name}\"></a>{page}");
        let own: Vec<String> = ["seeds.md", "convert.md"]
            .iter()
            .filter(|f| dir.join(lang).join(name).join(f).is_file())
            .map(|f| format!("[{}]({lang}/{name}/{f})", f.trim_end_matches(".md")))
            .collect();
        let _ = writeln!(
            s,
            "| {} | `{lang}` | {tb} | {} | {} | {} | {} | {} |",
            cell(r.get("language")),
            num(t[0]),
            num(t[1]),
            cell(r.get("license")),
            r.get("papers").split_whitespace().count(),
            if own.is_empty() { "—".to_string() } else { own.join(" · ") }
        );
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(name: &str, total: &str) -> Row {
        let mut f: BTreeMap<&'static str, String> = COLUMNS.iter().map(|k| (*k, String::new())).collect();
        for (k, v) in [("ud_name", name), ("lang", "xx"), ("language", "X"), ("treebank", "zz"), ("release", "2.18"), ("license", "CC BY 4.0"), ("test", total), ("total", total), ("papers", "L16-1262|url lib:a/ab/x|title~y"), ("unmatched_refs", "2")] {
            f.insert(k, v.to_string());
        }
        Row { f }
    }

    #[test]
    fn formatting() {
        assert_eq!(num(1234567), "1 234 567");
        assert_eq!(num(999), "999");
        assert_eq!(contributors("de Marneffe, Marie-Catherine; Guillaume, Bruno; Zeman, Daniel"), "Marie-Catherine de Marneffe; Bruno Guillaume; Daniel Zeman");
        assert_eq!(contributors("Team X"), "Team X");
        assert_eq!(cell("a | b"), "a \\| b");
        assert_eq!(cell("  "), "—");
        assert_eq!(flat(" a\tb\n c "), "a b c");
        assert_eq!(how_code(How::Title, true), "title~y");
    }

    #[test]
    fn tsv_round_trip_and_index() {
        let rows = vec![row("UD_X-ZZ", "1 2 2 0 0")];
        let text = tsv(&rows);
        let back = parse_tsv(&text).unwrap();
        assert_eq!(back, rows);
        assert_eq!(back[0].size("total"), Some([1, 2, 2, 0, 0]));
        assert_eq!(back[0].size("train"), None);
        let md = index_md(&back, Path::new("/nonexistent")).unwrap();
        assert!(md.contains("# UD 2.18 treebanks — index") && md.contains("2 distinct, 2 treebank–paper pairs; 2 references") && md.contains("| X | `xx` | <a id=\"xx-zz\"></a>zz | 1 | 2 | CC BY 4.0 | 2 | — |"), "{md}");
        // negative controls: a wrong header, a short row and a size that is not five numbers are refused
        assert!(parse_tsv(&text.replacen("ud_name", "name", 1)).is_err());
        assert!(parse_tsv(&format!("{}\nUD_Y\n", COLUMNS.join("\t"))).is_err());
        assert!(parse_tsv(&tsv(&[row("UD_X-ZZ", "1 2 2 0")])).is_err());
    }
}
