//! Fairy tales from Project Gutenberg for the five-year-old test (`docs/context.md`): plain text → tales →
//! paragraphs → sentences and tokens (`en::tok`) → UD by the Rust annotator (`en::annotate`, as bronze) → CoNLL-U
//! → database. Each tale is a separate document; title, author, translator, license go to `# meta::*` of the first
//! sentence (as in GUM), where the loader takes them from.
//!
//! How to split a book is given by the sources manifest (`tales` in the resource row):
//! - `contents: true` — tales by the book's table of contents: contents items with the smallest indent; a tale heading in
//!   the text is a separate line between blank lines that equals the item (ignoring case and quotes);
//!   `ignore` — contents lines that are not headings; `skip` — sections that are not tales (preface, index);
//! - without contents — one tale `title` from the line `start`;
//! - `end` — the line that ends the last tale (afterword, "THE END").
//! The Gutenberg header and footer (`*** START/END OF THE PROJECT GUTENBERG EBOOK`) are always cut off, and so are
//! `[Illustration]` lines. A heading, marker or contents not found is an error (fail-fast).

use std::collections::HashSet;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use en::annotate::{Annotator, rows};
use en::tok::{Tok, detokenize};
use serde_json::{Value, json};

use crate::duck::Duck;
use crate::load::{self, Docs, Match, Opts, RunInfo};
use crate::util;

/// How to split a book and what to write about it.
#[derive(Clone, Debug, Default)]
pub struct Spec {
    pub book: String,
    pub author: String,
    pub translator: Option<String>,
    pub genre: String,
    pub year: String,
    pub contents: bool,
    pub ignore: Vec<String>,
    pub skip: Vec<String>,
    pub start: Option<String>,
    pub end: Option<String>,
    pub title: Option<String>,
}

/// A book from the manifest.
#[derive(Clone, Debug)]
pub struct Book {
    pub slug: String,
    pub pg: String,
    pub license: String,
    pub pd_scope: String,
    pub source_page: String,
    pub file: PathBuf,
    pub spec: Spec,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Para {
    pub text: String,
    /// head — a heading inside a tale, lg — indented lines (verse), p — an ordinary paragraph.
    pub block: &'static str,
}

#[derive(Clone, Debug)]
pub struct Tale {
    pub heading: String,
    pub title: String,
    pub paras: Vec<Para>,
}

fn strs(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(str::to_string)).collect()).unwrap_or_default()
}

fn opt(v: &Value) -> Option<String> {
    v.as_str().map(str::to_string)
}

/// Books from the manifest: rows with the `tales` field; file — `<raw>/<slug>/<name from URL>`.
pub fn books(manifest: &Path, raw: &Path) -> Result<Vec<Book>> {
    let mut out = Vec::new();
    for (n, line) in std::fs::read_to_string(manifest).with_context(|| format!("{}", manifest.display()))?.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let v: Value = serde_json::from_str(line).with_context(|| format!("{}:{}", manifest.display(), n + 1))?;
        let t = &v["tales"];
        if t.is_null() {
            continue;
        }
        let slug = v["slug"].as_str().context("slug")?.to_string();
        let url = v["files"][0]["url"].as_str().context("files[0].url")?;
        let fname = url.rsplit('/').next().unwrap_or(url);
        let pg = v["source_page"].as_str().and_then(|p| p.rsplit('/').next()).unwrap_or("").to_string();
        out.push(Book {
            file: raw.join(&slug).join(fname),
            slug,
            pg,
            license: v["license"].as_str().unwrap_or("").to_string(),
            pd_scope: v["pd_scope"].as_str().unwrap_or("").to_string(),
            source_page: v["source_page"].as_str().unwrap_or("").to_string(),
            spec: Spec {
                book: t["book"].as_str().context("tales.book")?.to_string(),
                author: t["author"].as_str().context("tales.author")?.to_string(),
                translator: opt(&t["translator"]),
                genre: t["genre"].as_str().unwrap_or("").to_string(),
                year: t["year"].as_str().unwrap_or("").to_string(),
                contents: t["contents"].as_bool().unwrap_or(false),
                ignore: strs(&t["ignore"]),
                skip: strs(&t["skip"]),
                start: opt(&t["start"]),
                end: opt(&t["end"]),
                title: opt(&t["title"]),
            },
        });
    }
    Ok(out)
}

/// Lines between the Gutenberg header and footer, without CR.
pub fn body(text: &str) -> Result<Vec<String>> {
    let lines: Vec<&str> = text.split('\n').map(|l| l.strip_suffix('\r').unwrap_or(l)).collect();
    let start = lines.iter().position(|l| l.starts_with("*** START OF THE PROJECT GUTENBERG EBOOK")).context("missing \"*** START OF THE PROJECT GUTENBERG EBOOK\"")?;
    let end = lines.iter().position(|l| l.starts_with("*** END OF THE PROJECT GUTENBERG EBOOK")).context("missing \"*** END OF THE PROJECT GUTENBERG EBOOK\"")?;
    if end <= start {
        bail!("Gutenberg footer comes before the header");
    }
    Ok(lines[start + 1..end].iter().map(|l| l.to_string()).collect())
}

/// For comparing headings: lowercase, straight quotes, whitespace collapsed.
fn norm(s: &str) -> String {
    s.replace(['’', '‘'], "'").to_lowercase().split_whitespace().collect::<Vec<_>>().join(" ")
}

fn blank(l: &str) -> bool {
    l.trim().is_empty()
}

/// An all-caps heading → "Hansel and Gretel"; mixed case is kept.
pub fn title_case(s: &str) -> String {
    if s.chars().any(char::is_lowercase) {
        return s.to_string();
    }
    const SMALL: [&str; 16] = ["a", "an", "and", "as", "at", "by", "for", "from", "in", "of", "on", "or", "the", "to", "who", "with"];
    let mut out = String::new();
    for (i, w) in s.split(' ').enumerate() {
        if i > 0 {
            out.push(' ');
        }
        let lw = w.to_lowercase();
        if i > 0 && SMALL.contains(&lw.as_str()) {
            out.push_str(&lw);
            continue;
        }
        let mut up = true;
        for c in lw.chars() {
            if up && c.is_alphabetic() {
                out.extend(c.to_uppercase());
                up = false;
            } else {
                out.push(c);
            }
            if c == '-' || c == '[' || c == '(' {
                up = true;
            }
        }
    }
    out
}

/// Tales of a book.
pub fn split(lines: &[String], spec: &Spec) -> Result<Vec<Tale>> {
    let is_illustration = |l: &str| {
        let t = l.trim();
        t.starts_with("[Illustration") && t.ends_with(']')
    };
    let lines: Vec<String> = lines.iter().filter(|l| !is_illustration(l)).cloned().collect();
    let mut sections: Vec<(String, usize, usize)> = Vec::new();
    if spec.contents {
        let h = lines
            .iter()
            .position(|l| {
                let u = l.trim().to_uppercase();
                u.starts_with("CONTENTS") || u.starts_with("TABLE OF CONTENTS")
            })
            .context("table of contents not found")?;
        // items: after the heading and blank lines — until two blank lines in a row
        let mut i = h + 1;
        while i < lines.len() && blank(&lines[i]) {
            i += 1;
        }
        let mut items: Vec<(usize, String)> = Vec::new();
        while i < lines.len() && !(blank(&lines[i]) && lines.get(i + 1).is_none_or(|l| blank(l))) {
            if !blank(&lines[i]) {
                let indent = lines[i].len() - lines[i].trim_start().len();
                items.push((indent, lines[i].trim().to_string()));
            }
            i += 1;
        }
        let min = items.iter().map(|x| x.0).min().context("table of contents is empty")?;
        let ignore: HashSet<String> = spec.ignore.iter().map(|s| norm(s)).collect();
        let items: Vec<String> = items.into_iter().filter(|(ind, t)| *ind == min && !ignore.contains(&norm(t))).map(|x| x.1).collect();
        // headings in the text in contents order
        let mut heads: Vec<(String, usize)> = Vec::new();
        let mut from = i;
        for it in &items {
            let want = norm(it);
            let at = (from..lines.len())
                .find(|&k| norm(&lines[k]) == want && k > 0 && blank(&lines[k - 1]) && lines.get(k + 1).is_none_or(|l| blank(l)))
                .with_context(|| format!("heading \"{it}\" not found after line {from}"))?;
            heads.push((it.clone(), at));
            from = at + 1;
        }
        let last = heads.last().map(|x| x.1).context("no tales")?;
        let end = match &spec.end {
            Some(e) => (last + 1..lines.len()).find(|&k| lines[k].trim() == e).with_context(|| format!("end \"{e}\" not found"))?,
            None => lines.len(),
        };
        for (k, (t, at)) in heads.iter().enumerate() {
            let stop = heads.get(k + 1).map_or(end, |x| x.1);
            sections.push((t.clone(), at + 1, stop));
        }
    } else {
        let title = spec.title.clone().context("book without contents: tales.title is required")?;
        let start = match &spec.start {
            Some(s) => lines.iter().position(|l| l.trim_start().starts_with(s.as_str())).with_context(|| format!("start \"{s}\" not found"))?,
            None => 0,
        };
        let end = match &spec.end {
            Some(e) => (start..lines.len()).find(|&k| lines[k].trim() == e).with_context(|| format!("end \"{e}\" not found"))?,
            None => lines.len(),
        };
        sections.push((title, start, end));
    }
    let skip: HashSet<String> = spec.skip.iter().map(|s| norm(s)).collect();
    let mut out = Vec::new();
    for (heading, a, b) in sections {
        if skip.contains(&norm(&heading)) {
            continue;
        }
        let paras = paragraphs(&lines[a..b]);
        if paras.is_empty() {
            bail!("tale \"{heading}\" is empty");
        }
        out.push(Tale { title: title_case(&heading), heading, paras });
    }
    Ok(out)
}

/// Paragraphs — between blank lines; lines joined with one space, whitespace collapsed.
pub fn paragraphs(lines: &[String]) -> Vec<Para> {
    let mut out = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    let flush = |cur: &mut Vec<&str>, out: &mut Vec<Para>| {
        if cur.is_empty() {
            return;
        }
        let text = cur.iter().flat_map(|l| l.split_whitespace()).collect::<Vec<_>>().join(" ");
        let block = if cur.len() == 1 && !text.chars().any(char::is_lowercase) && text.chars().any(char::is_alphabetic) {
            "head"
        } else if cur.len() > 1 && cur.iter().all(|l| l.starts_with([' ', '\t'])) {
            "lg"
        } else {
            "p"
        };
        out.push(Para { text, block });
        cur.clear();
    };
    for l in lines {
        if blank(l) {
            flush(&mut cur, &mut out);
        } else {
            cur.push(l);
        }
    }
    flush(&mut cur, &mut out);
    out
}

fn terminal(f: &str) -> bool {
    !f.is_empty() && f.chars().all(|c| matches!(c, '.' | '!' | '?' | '…'))
}

fn closer(f: &str) -> bool {
    matches!(f, "”" | "’" | "\"" | "'" | "''" | ")" | "]" | "»")
}

/// Sentence boundaries over `en::tok` tokens: after . ! ? … (and closing quotes or brackets), unless what follows is
/// a lowercase letter or sentence-internal punctuation. Abbreviations ("Mr.") are kept whole by the tokenizer.
pub fn sentences(toks: &[Tok]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let (mut start, mut i) = (0usize, 0usize);
    while i < toks.len() {
        if terminal(&toks[i].form) {
            let mut j = i + 1;
            while j < toks.len() && (closer(&toks[j].form) || terminal(&toks[j].form)) {
                j += 1;
            }
            let next = toks.get(j).and_then(|t| t.form.chars().next());
            let continues = matches!(next, Some(c) if c.is_lowercase() || matches!(c, ',' | ';' | ':' | '-' | '–' | '—'));
            if !continues {
                out.push((start, j));
                start = j;
            }
            i = j;
        } else {
            i += 1;
        }
    }
    if start < toks.len() {
        out.push((start, toks.len()));
    }
    out
}

fn slug(s: &str) -> String {
    let mut out = String::new();
    for c in s.to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// Book summary.
#[derive(Default, Debug)]
pub struct Sum {
    pub tales: usize,
    pub paras: usize,
    pub sents: usize,
    pub tokens: usize,
    /// paragraphs where the token text (with spaces) differs from the paragraph text
    pub detok_diff: usize,
    /// document keys of the tales, in order
    pub keys: Vec<String>,
}

/// CoNLL-U of a book: tale by tale, document, paragraph and sentence comments.
pub fn conllu(book: &Book, tales: &[Tale], ann: &Annotator, sha1: &str, run_dir: &Path) -> Result<(String, Sum)> {
    let mut out = String::new();
    let mut sum = Sum::default();
    let mut seen: HashSet<String> = HashSet::new();
    let lic = format!("public domain ({})", book.pd_scope);
    let annotation = format!("Mova bronze: Rust only (en), not checked by Opus; sentences and tokens — en::tok; {}", run_dir.display());
    for (ti, t) in tales.iter().enumerate() {
        let mut s = slug(&t.heading);
        let base = s.clone();
        let mut n = 1;
        while !seen.insert(s.clone()) {
            n += 1;
            s = format!("{base}-{n}");
        }
        let key = format!("gutenberg:{}:{s}", book.pg);
        sum.tales += 1;
        sum.keys.push(key.clone());
        let mut k = 0usize;
        for (pi, p) in t.paras.iter().enumerate() {
            let toks = ann.tokenize(&p.text);
            if toks.is_empty() {
                continue;
            }
            if detokenize(&toks) != p.text {
                sum.detok_diff += 1;
            }
            sum.paras += 1;
            let ss = sentences(&toks);
            for (si, &(a, b)) in ss.iter().enumerate() {
                k += 1;
                let st = &toks[a..b];
                let forms: Vec<&str> = st.iter().map(|x| x.form.as_str()).collect();
                let mut space: Vec<bool> = st.iter().map(|x| x.space_after).collect();
                if b == toks.len() {
                    *space.last_mut().expect("token") = true;
                }
                let words = ann.annotate(&forms);
                if k == 1 {
                    let _ = writeln!(out, "# newdoc id = {key}");
                    let _ = writeln!(out, "# meta::title = {}", t.title);
                    let _ = writeln!(out, "# meta::heading = {}", t.heading);
                    let _ = writeln!(out, "# meta::author = {}", book.spec.author);
                    if let Some(tr) = &book.spec.translator {
                        let _ = writeln!(out, "# meta::translator = {tr}");
                    }
                    let _ = writeln!(out, "# meta::book = {}", book.spec.book);
                    let _ = writeln!(out, "# meta::genre = {}", book.spec.genre);
                    let _ = writeln!(out, "# meta::year = {}", book.spec.year);
                    let _ = writeln!(out, "# meta::license = {lic}");
                    let _ = writeln!(out, "# meta::license_note = {}", book.license);
                    let _ = writeln!(out, "# meta::source = Project Gutenberg #{}", book.pg);
                    let _ = writeln!(out, "# meta::sourceURL = {}", book.source_page);
                    let _ = writeln!(out, "# meta::sha1 = {sha1}");
                    let _ = writeln!(out, "# meta::tale = {} / {}", ti + 1, tales.len());
                }
                if si == 0 {
                    let _ = writeln!(out, "# newpar id = {key}:p{}", pi + 1);
                    let _ = writeln!(out, "# newpar_block = {} ({} s)", p.block, ss.len());
                }
                let _ = writeln!(out, "# sent_id = {key}:{k}");
                let _ = writeln!(out, "# text = {}", detokenize(st));
                let _ = writeln!(out, "# source = Project Gutenberg #{}, {lic}", book.pg);
                let _ = writeln!(out, "# annotation = {annotation}");
                for r in rows(&words, &space) {
                    out.push_str(&r.join("\t"));
                    out.push('\n');
                }
                out.push('\n');
                sum.sents += 1;
                sum.tokens += b - a;
            }
        }
    }
    Ok((out, sum))
}

/// `db tales`: books from the manifest → `<out>/conllu/<slug>.conllu` → database (bronze, run = name of directory `out`).
pub fn run(duck: &Duck, manifest: &Path, raw: &Path, out: &Path, train: &[PathBuf]) -> Result<()> {
    let books = books(manifest, raw)?;
    if books.is_empty() {
        bail!("{}: no resource with a tales field", manifest.display());
    }
    let dir = out.join("conllu");
    std::fs::create_dir_all(&dir)?;
    let out_abs = util::abs(out)?;
    let run_name = util::file_name(&out_abs);
    eprintln!("training the Rust annotator on {} files…", train.len());
    let t0 = std::time::Instant::now();
    let ann = Annotator::train_files(train)?;
    eprintln!("trained in {:.0} s", t0.elapsed().as_secs_f64());
    let mut made = Vec::new();
    let mut index = String::from("book\tpg\ttale\tdoc\ttitle\tparagraphs\n");
    let mut book_params = Vec::new();
    for b in &books {
        let bytes = std::fs::read(&b.file).with_context(|| format!("{}", b.file.display()))?;
        let sha1 = util::sha1_hex(&bytes);
        let text = String::from_utf8(bytes).with_context(|| format!("{}: not UTF-8", b.file.display()))?;
        let tales = split(&body(&text)?, &b.spec).with_context(|| b.slug.clone())?;
        let (conllu, sum) = conllu(b, &tales, &ann, &sha1, &out_abs)?;
        for (i, t) in tales.iter().enumerate() {
            let _ = writeln!(index, "{}\t{}\t{}\t{}\t{}\t{}", b.slug, b.pg, i + 1, sum.keys[i], t.title, t.paras.len());
        }
        let path = dir.join(format!("{}.conllu", b.slug));
        std::fs::write(&path, &conllu)?;
        println!(
            "{}: tales {}, paragraphs {}, sentences {}, tokens {}; paragraphs where tokens do not reproduce the text verbatim: {}",
            b.slug, sum.tales, sum.paras, sum.sents, sum.tokens, sum.detok_diff
        );
        book_params.push(json!({"slug": b.slug, "pg": b.pg, "file": b.file, "sha1": sha1, "tales": sum.tales, "paragraphs": sum.paras, "sentences": sum.sents, "tokens": sum.tokens, "detok_diff": sum.detok_diff}));
        made.push((b.slug.clone(), path));
    }
    std::fs::write(out.join("tales.tsv"), index)?;
    let run = RunInfo {
        name: run_name.clone(),
        kind: "tales".into(),
        tool: "db tales".into(),
        path: out_abs.to_string_lossy().into_owned(),
        params: json!({
            "manifest": util::abs(manifest)?,
            "train": train.iter().map(|p| util::abs(p).unwrap_or_else(|_| p.clone())).collect::<Vec<_>>(),
            "tokenizer": "en::tok",
            "sentences": "db tales: boundary after . ! ? … (with quotes), unless a lowercase letter follows",
            "annotator": "en::annotate (tagger, parser, lemmas, UPOS and FEATS with agreement)",
            "books": book_params,
        }),
        started: Some(util::now_ts()),
        note: Some("Public-domain fairy tales for the five-year-old test: UD bronze, Rust only".into()),
        method: None,
        license: None,
        genre: None,
    };
    let reg = en::udreg::Registry::load();
    for (slug, path) in made {
        let mut o = Opts::new("bronze", "ud-2.18")?;
        o.kind = "tales".into();
        o.docs = Docs::NewDoc;
        o.matching = Match::New;
        o.origin = Some(format!("{run_name}/{slug}"));
        o.run_info = Some(run.clone());
        println!("{}", load::load_file(duck, &reg, &o, &path)?);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(forms: &[&str]) -> Vec<Tok> {
        forms.iter().map(|f| Tok { form: f.to_string(), space_after: true }).collect()
    }

    #[test]
    fn sentence_bounds() {
        // a quote after the final mark goes with the sentence; a lowercase letter next — the same sentence
        let x = t(&["‘", "Now", "run", ".", "I", "am", "out", ".", "’", "Then", "he", "went", "."]);
        assert_eq!(sentences(&x), [(0, 4), (4, 9), (9, 13)]);
        let y = t(&["“", "It", "was", "mercy", "!", "”", "she", "said", "."]);
        assert_eq!(sentences(&y), [(0, 9)]);
        let z = t(&["No", "end"]);
        assert_eq!(sentences(&z), [(0, 2)]);
        assert_eq!(sentences(&t(&["Wait", "...", "and", "see", "?", "!", "Yes", "."])), [(0, 6), (6, 8)]);
    }

    #[test]
    fn split_by_contents() {
        let text = "x\n*** START OF THE PROJECT GUTENBERG EBOOK X ***\r\n\nCONTENTS\n\n     THE ONE\n       1. PART\n     Two Tales\n     junk.\n\n\n\nTHE ONE\n\nOnce there was\na king.\n\n1. PART\n\nHe went.\n\n\nTWO TALES\n\n[Illustration]\n\n    Verse one,\n    verse two.\n\nTHE END\n\nAfterword.\n*** END OF THE PROJECT GUTENBERG EBOOK X ***\nlicense\n";
        let lines = body(text).unwrap();
        let spec = Spec { contents: true, ignore: vec!["junk.".into()], end: Some("THE END".into()), ..Default::default() };
        let tales = split(&lines, &spec).unwrap();
        assert_eq!(tales.len(), 2);
        assert_eq!((tales[0].heading.as_str(), tales[0].title.as_str()), ("THE ONE", "The One"));
        assert_eq!(tales[0].paras, [Para { text: "Once there was a king.".into(), block: "p" }, Para { text: "1. PART".into(), block: "head" }, Para { text: "He went.".into(), block: "p" }]);
        assert_eq!(tales[1].paras, [Para { text: "Verse one, verse two.".into(), block: "lg" }]);
        // a skipped section and a missing heading
        let skip = Spec { skip: vec!["two tales".into()], ..spec.clone() };
        assert_eq!(split(&lines, &skip).unwrap().len(), 1);
        let bad = Spec { ignore: vec![], ..spec };
        assert!(format!("{:#}", split(&lines, &bad).unwrap_err()).contains("junk."));
        assert!(body("no markers").is_err());
        assert_eq!(title_case("LITTLE RED-CAP [LITTLE RED RIDING HOOD]"), "Little Red-Cap [Little Red Riding Hood]");
        assert_eq!(title_case("THE ELEPHANT’S CHILD"), "The Elephant’s Child");
        assert_eq!(title_case("The Fir Tree"), "The Fir Tree");
        assert_eq!(slug("THE STRAW, THE COAL, AND THE BEAN"), "the-straw-the-coal-and-the-bean");
    }
}
