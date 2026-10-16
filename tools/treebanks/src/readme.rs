//! UD treebank README: machine-readable metadata, headings (ATX `#`, setext `===`/`---`, ALL-CAPS lines),
//! Summary/Introduction sections, references sections and reference items (paragraphs, list items, bibtex).

use sources::bib;

/// Heading: line, how many lines it spans (setext — 2), level, text.
#[derive(Debug, Clone)]
pub struct Heading {
    pub line: usize,
    pub span: usize,
    pub level: u8,
    pub title: String,
}

pub struct Readme {
    pub lines: Vec<String>,
    /// "key: value" pairs from the metadata block, in file order (first occurrence of a key).
    pub meta: Vec<(String, String)>,
    /// Lines of the metadata block `[start, end)` — including the heading and the closing line.
    pub meta_range: Option<(usize, usize)>,
    pub headings: Vec<Heading>,
}

/// Reference item: raw text (lines joined), lines `[from, to)`, whether in a references section, bibtex.
pub struct Item {
    pub raw: String,
    /// the same text with line breaks (for title boundaries: "Title⏎Authors⏎Venue")
    pub text: String,
    pub lines: (usize, usize),
    pub in_refs: bool,
    pub bib: Option<bib::Entry>,
}

fn is_meta_header(line: &str) -> bool {
    let l = line.trim().to_lowercase();
    (l.starts_with('=') || l.starts_with('-')) && (l.contains("machine-readable metadata") || l.contains("machine readable metadata"))
}

fn is_meta_end(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("===") || t.starts_with("---") || t.starts_with("```") || t.starts_with("<pre") || t.starts_with("</pre")
}

/// Metadata key: 1–4 words of letters (`Data available since`, `UPOS`, `Includes text`).
fn meta_pair(line: &str) -> Option<(String, String)> {
    let (k, v) = line.split_once(':')?;
    let k = k.trim();
    let words = k.split_whitespace().count();
    if k.is_empty() || words > 4 || !k.chars().all(|c| c.is_alphabetic() || c == ' ') {
        return None;
    }
    Some((k.to_string(), v.trim().to_string()))
}

/// ATX heading (`# Summary`), but not a CoNLL-U comment (`# text = …`, `# newpar`).
fn atx(line: &str) -> Option<(u8, String)> {
    let indent = line.len() - line.trim_start_matches(' ').len();
    if indent > 3 {
        return None;
    }
    let t = &line[indent..];
    let hashes = t.len() - t.trim_start_matches('#').len();
    if hashes == 0 || hashes > 6 {
        return None;
    }
    let rest = &t[hashes..];
    if !(rest.is_empty() || rest.starts_with(' ') || rest.starts_with('\t')) {
        return None;
    }
    let title = rest.trim().trim_end_matches('#').trim().to_string();
    let low = title.to_lowercase();
    if title.is_empty() || title.contains('=') || ["sent_id", "text", "newdoc", "newpar", "global.", "meta::"].iter().any(|p| low.starts_with(p)) {
        return None;
    }
    Some((hashes as u8, title))
}

fn is_underline(line: &str, ch: char) -> bool {
    let t = line.trim();
    t.len() >= 3 && t.chars().all(|c| c == ch)
}

/// An ALL-CAPS line as a heading (`REFERENCES`, `DATA SPLITS`, `README FROM GOOGLE`).
fn caps(line: &str) -> bool {
    let t = line.trim();
    let letters = t.chars().filter(|c| c.is_alphabetic()).count();
    (4..=40).contains(&t.chars().count())
        && letters >= 4
        && t.chars().all(|c| (c.is_alphabetic() && !c.is_lowercase()) || " &/-".contains(c))
}

fn list_marker(t: &str) -> Option<usize> {
    for m in ["- ", "* ", "+ ", "• "] {
        if t.starts_with(m) {
            return Some(m.len());
        }
    }
    let digits = t.chars().take_while(|c| c.is_ascii_digit()).count();
    if (1..=3).contains(&digits) && (t[digits..].starts_with(". ") || t[digits..].starts_with(") ")) {
        return Some(digits + 2);
    }
    if t.starts_with('[') {
        let d = t[1..].chars().take_while(|c| c.is_ascii_digit()).count();
        if (1..=3).contains(&d) && t[1 + d..].starts_with(']') {
            return Some(d + 2);
        }
    }
    None
}

/// Normalized heading words (ASCII letters and digits, lowercase).
fn title_words(t: &str) -> Vec<String> {
    t.to_lowercase().split(|c: char| !c.is_ascii_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_string).collect()
}

/// Heading of a references section: References, Citation, How to cite, BibTeX, Bibliography, Publications…
pub fn is_ref_title(t: &str) -> bool {
    let w = title_words(t);
    let hit = w.iter().any(|x| matches!(x.as_str(), "references" | "reference" | "citation" | "citations" | "cite" | "citing" | "bibtex" | "bibliography" | "publications"));
    let bad = w.iter().any(|x| matches!(x.as_str(), "coreference" | "cross" | "sentence"));
    hit && !bad
}

pub fn has_year(s: &str) -> bool {
    s.split(|c: char| !c.is_ascii_alphanumeric()).any(|w| year_of(w).is_some())
}

/// `2019`, `2019a` → 2019 (1950–2035).
pub fn year_of(w: &str) -> Option<u32> {
    let d: String = w.chars().take_while(|c| c.is_ascii_digit()).collect();
    let rest = &w[d.len()..];
    if d.len() != 4 || rest.chars().count() > 1 || !rest.chars().all(|c| c.is_ascii_lowercase()) {
        return None;
    }
    let y: u32 = d.parse().ok()?;
    (1950..=2035).contains(&y).then_some(y)
}

impl Readme {
    pub fn parse(text: &str) -> Readme {
        let lines: Vec<String> = text.lines().map(|l| l.trim_end_matches('\r').to_string()).collect();
        let n = lines.len();
        // code blocks
        let mut fenced = vec![false; n];
        let mut open = false;
        for (i, l) in lines.iter().enumerate() {
            let t = l.trim_start();
            if t.starts_with("```") || t.starts_with("~~~") {
                fenced[i] = true;
                open = !open;
                continue;
            }
            fenced[i] = open;
        }
        // metadata
        let (mut meta, mut meta_range) = (Vec::<(String, String)>::new(), None);
        if let Some(h) = (0..n).find(|&i| is_meta_header(&lines[i])) {
            let mut end = n;
            for (j, l) in lines.iter().enumerate().skip(h + 1) {
                // a fence right after the heading (```, <pre>) opens the block rather than closing it
                if is_meta_end(l) && meta.is_empty() {
                    continue;
                }
                if is_meta_end(l) {
                    end = if l.trim_start().starts_with("===") || l.trim_start().starts_with("---") { j + 1 } else { j };
                    break;
                }
                if let Some((k, v)) = meta_pair(l) {
                    if !meta.iter().any(|(x, _)| x.eq_ignore_ascii_case(&k)) {
                        meta.push((k, v));
                    }
                }
            }
            meta_range = Some((h, end));
        }
        let in_meta = |i: usize| meta_range.is_some_and(|(a, b)| i >= a && i < b);
        // headings
        let mut headings = Vec::new();
        let mut i = 0;
        while i < n {
            if fenced[i] || in_meta(i) {
                i += 1;
                continue;
            }
            let l = &lines[i];
            let prev_blank = i == 0 || lines[i - 1].trim().is_empty() || headings.last().is_some_and(|h: &Heading| h.line + h.span == i);
            if let Some((level, title)) = atx(l) {
                headings.push(Heading { line: i, span: 1, level, title });
                i += 1;
                continue;
            }
            let t = l.trim();
            let texty = !t.is_empty() && !"#|>-*=+<".contains(t.chars().next().unwrap_or(' ')) && list_marker(t).is_none();
            if texty && prev_blank && i + 1 < n && !fenced[i + 1] && !in_meta(i + 1) {
                let level = if is_underline(&lines[i + 1], '=') {
                    Some(1)
                } else if is_underline(&lines[i + 1], '-') {
                    Some(2)
                } else {
                    None
                };
                if let Some(level) = level {
                    // "Changelog" in UD READMEs is always a top-level section, however it is underlined
                    let level = if title_words(t).first().is_some_and(|w| w.starts_with("changelog")) { 1 } else { level };
                    headings.push(Heading { line: i, span: 2, level, title: t.to_string() });
                    i += 2;
                    continue;
                }
            }
            // "Changelog" as a bare line (no # and no underline) is also a top-level section
            let bare_changelog = matches!(t.trim_end_matches(':').to_lowercase().as_str(), "changelog" | "change log" | "changes");
            if prev_blank && (caps(l) || bare_changelog) {
                // ALL-CAPS lines are subsections (ORIGIN, ALIGNMENTS inside Introduction), except names of top-level sections
                let top = bare_changelog || title_words(t).first().is_some_and(|w| matches!(w.as_str(), "references" | "reference" | "changelog" | "summary" | "introduction" | "acknowledgments" | "acknowledgements"));
                headings.push(Heading { line: i, span: 1, level: if top { 1 } else { 2 }, title: t.to_string() });
            }
            i += 1;
        }
        Readme { lines, meta, meta_range, headings }
    }

    /// Metadata value by key (case-insensitive); empty if absent.
    pub fn meta(&self, key: &str) -> &str {
        self.meta.iter().find(|(k, _)| k.eq_ignore_ascii_case(key)).map_or("", |(_, v)| v.as_str())
    }

    /// Section body `[from, to)`: up to the next heading of the same or higher level, or the metadata.
    pub fn body(&self, h: usize) -> (usize, usize) {
        let head = &self.headings[h];
        let start = head.line + head.span;
        let mut end = self.headings[h + 1..].iter().find(|x| x.level <= head.level).map_or(self.lines.len(), |x| x.line);
        if let Some((a, _)) = self.meta_range {
            if a >= start && a < end {
                end = a;
            }
        }
        (start, end)
    }

    /// Text of the first section whose heading satisfies `pred` (blank lines at the edges trimmed).
    pub fn section(&self, pred: impl Fn(&[String]) -> bool) -> String {
        let Some(h) = self.headings.iter().position(|x| pred(&title_words(&x.title))) else { return String::new() };
        let (a, b) = self.body(h);
        self.lines[a..b].join("\n").trim_matches('\n').trim_end().to_string()
    }

    pub fn summary(&self) -> String {
        self.section(|w| w.len() == 1 && w[0] == "summary")
    }

    pub fn introduction(&self) -> String {
        self.section(|w| w.first().is_some_and(|x| x == "introduction"))
    }

    /// Lines of references sections.
    pub fn ref_ranges(&self) -> Vec<(usize, usize)> {
        (0..self.headings.len()).filter(|&h| is_ref_title(&self.headings[h].title)).map(|h| self.body(h)).collect()
    }

    /// Reference items: bibtex records, list items and paragraphs (outside metadata and headings).
    /// In a references section, a line with a year and ≥ 6 words after a line ending with a period starts a new item.
    pub fn items(&self) -> Vec<Item> {
        let refs = self.ref_ranges();
        let in_refs = |i: usize| refs.iter().any(|&(a, b)| i >= a && i < b);
        let in_meta = |i: usize| self.meta_range.is_some_and(|(a, b)| i >= a && i < b);
        let heading_line = |i: usize| self.headings.iter().any(|h| i >= h.line && i < h.line + h.span);
        let n = self.lines.len();
        let mut out: Vec<Item> = Vec::new();
        let mut cur: Option<(usize, Vec<String>)> = None;
        let mut cur_marked = false;
        let flush = |cur: &mut Option<(usize, Vec<String>)>, end: usize, out: &mut Vec<Item>| {
            if let Some((start, parts)) = cur.take() {
                let raw = parts.join(" ").split_whitespace().collect::<Vec<_>>().join(" ");
                if !raw.is_empty() {
                    out.push(Item { raw, text: parts.join("\n"), lines: (start, end), in_refs: in_refs(start), bib: None });
                }
            }
        };
        let mut i = 0;
        while i < n {
            let l = &self.lines[i];
            let t = l.trim();
            if in_meta(i) || heading_line(i) || t.is_empty() || t.starts_with("```") || t.starts_with("~~~") {
                flush(&mut cur, i, &mut out);
                i += 1;
                continue;
            }
            // bibtex: from `@type{` until the braces close
            if t.starts_with('@') && t[1..].trim_start().chars().take_while(|c| c.is_ascii_alphabetic()).count() > 0 && t.contains('{') {
                flush(&mut cur, i, &mut out);
                let mut depth = 0i32;
                let mut j = i;
                let mut text = String::new();
                while j < n && j < i + 80 {
                    let lj = &self.lines[j];
                    if j > i && (lj.trim_start().starts_with("```") || (lj.trim_start().starts_with('@') && depth <= 0)) {
                        break;
                    }
                    for c in lj.chars() {
                        match c {
                            '{' => depth += 1,
                            '}' => depth -= 1,
                            _ => {}
                        }
                    }
                    text.push_str(lj);
                    text.push('\n');
                    j += 1;
                    if depth <= 0 {
                        break;
                    }
                }
                let entry = bib::parse(&text).into_iter().next().filter(|e| !e.get("title").is_empty());
                let raw = text.split_whitespace().collect::<Vec<_>>().join(" ");
                out.push(Item { raw, text: text.clone(), lines: (i, j), in_refs: in_refs(i), bib: entry });
                i = j.max(i + 1);
                continue;
            }
            let marker = list_marker(t);
            let body = marker.map_or(t, |m| t[m..].trim_start());
            // a new item without a marker: only in a references section, in a paragraph without a list marker, from a line without
            // indentation that does not continue the previous reference ("In: Proceedings…", "pp.…")
            const CONT: [&str; 17] = ["In ", "In:", "in ", "Proc", "pp", "Pages", "pages", "Vol", "vol", "Journal", "ISBN", "DOI", "doi", "http", "URL", "(", "["];
            let starts_citation = in_refs(i)
                && !cur_marked
                && !l.starts_with(char::is_whitespace)
                && !CONT.iter().any(|c| t.starts_with(c))
                && cur.as_ref().is_some_and(|(_, p)| p.last().is_some_and(|x| x.trim_end().ends_with('.')))
                && has_year(t)
                && t.split_whitespace().count() >= 6
                && t.chars().next().is_some_and(char::is_uppercase);
            if marker.is_some() || starts_citation {
                flush(&mut cur, i, &mut out);
                cur_marked = marker.is_some();
            }
            if cur.is_none() {
                cur_marked = marker.is_some();
            }
            match &mut cur {
                Some((_, parts)) => parts.push(body.to_string()),
                None => cur = Some((i, vec![body.to_string()])),
            }
            i += 1;
        }
        flush(&mut cur, n, &mut out);
        out
    }
}

/// Jekyll escaping (`{% raw %}`, `{% endraw %}`) in READMEs and docs pages is removed: it breaks titles and bibtex.
pub fn strip_jekyll(s: &str) -> String {
    s.replace("{% raw %}", "").replace("{% endraw %}", "")
}

/// Treebank page from the docs repo: without the Jekyll front matter and without statistics (`# Statistics of …`);
/// the language family from the `Family: …` line.
pub fn docs_page(text: &str) -> (String, String) {
    let text = strip_jekyll(text);
    let mut body = text.as_str();
    if let Some(rest) = body.strip_prefix("---\n") {
        if let Some(end) = rest.find("\n---\n") {
            body = &rest[end + 5..];
        }
    }
    if let Some(stat) = body.find("\n# Statistics of") {
        body = &body[..stat];
    }
    let family = body
        .lines()
        .find_map(|l| l.strip_prefix("Family:"))
        .map(|f| f.trim().trim_end_matches("<br/>").trim_end_matches("<br />").trim().to_string())
        .unwrap_or_default();
    (body.to_string(), family)
}

#[cfg(test)]
mod tests {
    use super::*;

    const README: &str = "# Summary\n\nFrench treebank.\n\n# Introduction\n\nConverted from UD 2.0.\n\n```\n# text = Il pleut.\n# Not a heading\n```\n\n## Text sources\n\nNews.\n\n# Acknowledgments\n\nThanks.\n\n## References\n\n* Guillaume, B., de Marneffe, M.-C., Perrier, G. (2019). Conversion et améliorations de corpus du français annotés en Universal Dependencies. TAL 60(2).\n* Nivre, J. et al. (2016). Universal Dependencies v1: A Multilingual Treebank Collection. In Proceedings of LREC 2016.\n\n```\n@inproceedings{nivre-etal-2016-universal,\n    title = \"{U}niversal {D}ependencies v1: A Multilingual Treebank Collection\",\n    author = \"Nivre, Joakim\",\n    year = \"2016\",\n    url = \"https://aclanthology.org/L16-1262\",\n}\n```\n\n# Changelog\n\n* 2019-05-15 v2.4\n  * Fixed lemmas\n\n<pre>\n=== Machine-readable metadata (DO NOT REMOVE!) ================================\nData available since: UD v1.0\nLicense: CC BY-SA 4.0\nGenre: blog news\nLemmas: \tautomatic with corrections\nContributors: de Marneffe, Marie-Catherine; Guillaume, Bruno\n===============================================================================\n</pre>\n";

    #[test]
    fn metadata_and_sections() {
        let r = Readme::parse(README);
        assert_eq!(r.meta("license"), "CC BY-SA 4.0");
        assert_eq!(r.meta("Lemmas"), "automatic with corrections");
        assert_eq!(r.meta("Data available since"), "UD v1.0");
        assert_eq!(r.meta("XPOS"), "", "missing key — empty");
        assert_eq!(r.summary(), "French treebank.");
        let intro = r.introduction();
        assert!(intro.starts_with("Converted from UD 2.0.") && intro.contains("# text = Il pleut.") && intro.ends_with("News."), "{intro}");
        assert!(!r.headings.iter().any(|h| h.title.contains("Not a heading")), "a heading in a code block is not a heading");
    }

    #[test]
    fn metadata_variants() {
        let fin = "Intro\n\n--- Machine readable metadata ---\nLicense: CC BY-SA 4.0\nUPOS: converted from manual\n";
        assert_eq!(Readme::parse(fin).meta("UPOS"), "converted from manual");
        let cop = "=== Machine readable metadata ==============\n\nDocumentation status: complete\nLicense: CC BY 4.0\n\n============================================";
        let r = Readme::parse(cop);
        assert_eq!(r.meta("License"), "CC BY 4.0");
        assert_eq!(r.meta("Documentation status"), "complete");
        let fenced = "# Metadata\n\n=== Machine-readable metadata =====\n\n```\nLicense: CC BY-NC-SA 4.0\nGenre: legal news\n```\n\n# After\n\nText: not metadata\n";
        let r = Readme::parse(fenced);
        assert_eq!((r.meta("License"), r.meta("Genre"), r.meta("Text")), ("CC BY-NC-SA 4.0", "legal news", ""));
        // negative control: without a metadata block, "License: …" in the text is not read as metadata
        let none = "# Summary\n\nLicense: this text is free.\n";
        assert!(Readme::parse(none).meta.is_empty());
    }

    #[test]
    fn setext_and_caps_headings() {
        let t = "Summary\n=======\n\nText.\n\nChangelog\n---------\n\n* v2.0\n\nREFERENCES\nSmith, J. 2010. A title of a paper about things. Journal.\n";
        let r = Readme::parse(t);
        let titles: Vec<&str> = r.headings.iter().map(|h| h.title.as_str()).collect();
        assert_eq!(titles, vec!["Summary", "Changelog", "REFERENCES"]);
        assert_eq!(r.summary(), "Text.");
        assert_eq!(r.ref_ranges().len(), 1);
        // an ALL-CAPS line inside Introduction is a subsection; the section is not cut
        let r = Readme::parse("# Introduction\n\nText one.\n\nORIGIN\n\nMore text.\n\n# Changelog\n\n* v1\n");
        assert!(r.introduction().ends_with("More text."), "{}", r.introduction());
    }

    #[test]
    fn reference_items() {
        let r = Readme::parse(README);
        let items = r.items();
        let refs: Vec<&Item> = items.iter().filter(|i| i.in_refs).collect();
        assert_eq!(refs.len(), 3, "{:?}", refs.iter().map(|i| &i.raw).collect::<Vec<_>>());
        assert!(refs[0].raw.starts_with("Guillaume, B."));
        assert!(refs[1].raw.starts_with("Nivre, J."));
        let b = refs[2].bib.as_ref().expect("bibtex");
        assert_eq!(b.get("title"), "Universal Dependencies v1: A Multilingual Treebank Collection");
        // items outside the references section exist too, but are not flagged
        assert!(items.iter().any(|i| !i.in_refs && i.raw == "Thanks."));
        assert!(is_ref_title("References:") && is_ref_title("How to cite") && is_ref_title("BibTeX reference format"));
        // negative controls: not references sections
        assert!(!is_ref_title("Coreference and Entities") && !is_ref_title("Cross-sentence references (interactional relations)") && !is_ref_title("Changelog"));
    }

    #[test]
    fn split_unmarked_citations_in_refs() {
        let t = "# References\n\nSmith, J. (2010). A first title about treebanks. Journal of Things.\nDoe, A. (2012). A second title about parsing. In Proceedings of X.\n";
        let items = Readme::parse(t).items();
        assert_eq!(items.len(), 2);
        // a line break inside a reference does not split it
        let t2 = "# References\n\nSmith, J. (2010). A first title about\ntreebanks and parsing in 2010 of many things. Journal.\n";
        assert_eq!(Readme::parse(t2).items().len(), 1);
        // negative controls: continuation lines of a list item and "In: Proceedings… 2008" do not split the reference
        let t3 = "# References\n\n* Otakar Smrž, Jan Hajič. 2008.\n  Prague Arabic Dependency Treebank: A Word on the Million Words.\n  In: Proceedings of the Workshop on Arabic and Local Languages (LREC 2008), pp. 16–23.\n* Tyers, F. M. (2018). UD Annotatrix. In Proceedings of TLT.\n";
        assert_eq!(Readme::parse(t3).items().len(), 2);
        let t4 = "# References\n\nSmith, J. (2008). A title about treebanks.\nIn: Proceedings of the Workshop on Things (LREC 2008), pp. 16–23.\n";
        assert_eq!(Readme::parse(t4).items().len(), 1);
        // a bare "Changelog" line closes the references section: change entries are not references
        let t5 = "# Reference\n\n[1] Smith, J. (2016). A title about treebanks. LREC.\n\nChangelog\n2021-05-01 v2.8 (UD_Japanese-BCCWJ)\n  * Initial release in Universal Dependencies.\n";
        let items = Readme::parse(t5).items();
        assert_eq!(items.iter().filter(|i| i.in_refs).count(), 1);
        assert_eq!(year_of("2019a"), Some(2019));
        assert_eq!(year_of("20190"), None);
        assert_eq!(year_of("1066"), None);
    }

    #[test]
    fn docs_family() {
        let page = "---\nlayout: base\ntitle:  'UD_French-GSD'\n---\n\n# UD French GSD\n\nLanguage: [French](/fr/index.html) (code: `fr`)<br/>\nFamily: IE\n\nText.\n\n# Statistics of UD French GSD\n\nPOS\n";
        let (body, fam) = docs_page(page);
        assert_eq!(fam, "IE");
        assert!(body.contains("Text.") && !body.contains("POS") && !body.contains("layout"));
    }
}
