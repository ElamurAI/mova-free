//! Catalog of open math textbooks: OpenStax (CMS API), Open Textbook Library
//! (JSON of the mathematics subject), AIM Open Textbook Initiative (list and book pages).
//!
//! `books-fetch` only stores raw pages in `<catalog-dir>` (a rerun does not re-download existing ones):
//! - OpenStax — one API request;
//! - AIM — the list and ~70 book pages, 3 s pause;
//! - OTL — pages via `links.next`, 120 s pause (`Crawl-delay: 120` in their robots.txt).
//! `books` — offline: raw pages → catalog TSV (license as an enum, class, whether downloadable,
//! topic and level from the title, direct PDF where known; GitHub is flagged and left alone).

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use regex::Regex;
use serde_json::Value;

use crate::http::{Polite, utc_now};

const OPENSTAX: &str = "https://openstax.org/apps/cms/api/v2/pages/?type=books.Book&fields=title,book_state,book_subjects,license_name,license_url,high_resolution_pdf_url,publish_date,authors&limit=250";
const AIM_LIST: &str = "https://textbooks.aimath.org/textbooks/approved-textbooks/";
const OTL_FIRST: &str = "https://open.umn.edu/opentextbooks/subjects/mathematics.json";

fn save(http: &mut Polite, url: &str, path: &Path) -> Result<bool> {
    if path.exists() {
        return Ok(false);
    }
    match http.text(url)? {
        Ok(body) => {
            fs::write(path, body)?;
            println!("{} GET {url} → {}", utc_now(), path.display());
            Ok(true)
        }
        Err(st) if st.code == 403 || st.code == 429 => bail!("HTTP {} on {url} — stopping the route", st.code),
        Err(st) => {
            println!("{} MISS HTTP {} {url}", utc_now(), st.code);
            Ok(false)
        }
    }
}

pub fn fetch(dir: &Path, which: &str) -> Result<()> {
    fs::create_dir_all(dir)?;
    if which == "all" || which == "openstax" {
        let mut http = Polite::new(Duration::from_secs(3));
        save(&mut http, OPENSTAX, &dir.join("openstax-books.json"))?;
    }
    if which == "all" || which == "aim" {
        let mut http = Polite::new(Duration::from_secs(3));
        let list = dir.join("aim-approved.html");
        save(&mut http, AIM_LIST, &list)?;
        let html = fs::read_to_string(&list)?;
        let aim_dir = dir.join("aim");
        fs::create_dir_all(&aim_dir)?;
        for (_, slug, _) in aim_list(&html)? {
            let path = aim_dir.join(format!("{slug}.html"));
            save(&mut http, &format!("{AIM_LIST}{slug}/"), &path)?;
        }
    }
    // `otl` — the mathematics subject; `otl:<subject>` — another one (physics, chemistry…)
    let otl_subject = match which {
        "all" | "otl" => Some("mathematics"),
        w => w.strip_prefix("otl:"),
    };
    if let Some(subject) = otl_subject {
        let mut http = Polite::new(Duration::from_secs(120));
        let mut n = 1;
        let mut url = OTL_FIRST.replace("mathematics", subject);
        loop {
            let path = dir.join(format!("otl-{subject}-p{n}.json"));
            save(&mut http, &url, &path)?;
            if !path.exists() {
                bail!("OTL: page {n} failed to download — stopping");
            }
            let v: Value = serde_json::from_str(&fs::read_to_string(&path)?).with_context(|| format!("{}", path.display()))?;
            match v["links"]["next"].as_str() {
                Some(next) if !next.is_empty() => {
                    url = next.to_string();
                    n += 1;
                }
                _ => break,
            }
        }
        println!("{} OTL: {n} pages", utc_now());
    }
    Ok(())
}

/// PDF search on the sites of books without a direct PDF (allowed license, not GitHub): one page per
/// site, 3 s pause; candidates go to `<catalog>/sites-pdf.tsv` (all, with rank), raw pages to `sites/`.
pub fn find_pdfs(dir: &Path, catalog_tsv: &Path) -> Result<()> {
    let text_all = fs::read_to_string(catalog_tsv)?;
    let mut lines = text_all.lines();
    let head: Vec<&str> = lines.next().context("empty catalog")?.split('\t').collect();
    let col = |n: &str| head.iter().position(|c| *c == n).with_context(|| format!("no column {n}"));
    let (i_src, i_key, i_title, i_allowed, i_site, i_pdf, i_gh) =
        (col("source")?, col("key")?, col("title")?, col("allowed")?, col("site")?, col("pdf_url")?, col("github")?);
    let sites = dir.join("sites");
    fs::create_dir_all(&sites)?;
    let mut http = Polite::new(Duration::from_secs(3));
    let href = Regex::new(r#"(?i)href\s*=\s*["']([^"'#]+?\.pdf)(?:[?#][^"']*)?["']"#)?;
    let dc = Regex::new(r#"(?i)href\s*=\s*["']([^"']*viewcontent\.cgi\?[^"']+)["']"#)?;
    let bad = Regex::new(r"(?i)solution|answer|errat|exercis|chapter|/ch\d|sample|preface|toc\b|cover|syllab|slides|manual|guide|worksheet|homework|quiz|exam|review|appendix|index\.pdf|flyer|poster")?;
    let mut out = String::from("source\tkey\ttitle\trank\tpdf_url\tsite\n");
    let mut seen_site: HashMap<String, String> = HashMap::new();
    // 403/429 — host closed until the end of the run (route fail-fast)
    let mut blocked: std::collections::HashSet<String> = std::collections::HashSet::new();
    for line in lines {
        let c: Vec<&str> = line.split('\t').collect();
        let g = |i: usize| c.get(i).copied().unwrap_or("");
        // where to look: a catalog "PDF" that is really a page (not .pdf), and the book's site
        let pdf = g(i_pdf);
        if g(i_allowed) != "true" || looks_pdf(pdf) || g(i_gh) == "true" {
            continue;
        }
        let (src, key, title) = (g(i_src), g(i_key), g(i_title));
        let mut targets: Vec<(String, &str)> = Vec::new();
        if !pdf.is_empty() && !looks_pdf(pdf) {
            targets.push((format!("{src}-{}.html", key.replace(['/', ' '], "_")), pdf));
        }
        let site_url = g(i_site);
        if !site_url.is_empty() && site_url != pdf {
            let name = if targets.is_empty() { format!("{src}-{}.html", key.replace(['/', ' '], "_")) } else { format!("{src}-{}-site.html", key.replace(['/', ' '], "_")) };
            targets.push((name, site_url));
        }
        for (cache_name, site) in targets {
            // open.umn.edu — Crawl-delay 120 s, their pages give the same links as the JSON
            if !site.starts_with("http") || is_github(site) || host(site) == "open.umn.edu" || blocked.contains(&host(site)) || crate::robots::github_host(&host(site)) {
                continue;
            }
            let cache = sites.join(&cache_name);
            let (body, base) = if let Some(b) = seen_site.get(site) {
                (b.clone(), site.to_string())
            } else if cache.exists() {
                (fs::read_to_string(&cache).unwrap_or_default(), site.to_string())
            } else {
                match http.text_url(site) {
                    Ok(Ok((b, fin))) => {
                        fs::write(&cache, &b)?;
                        println!("{} SITE {site} → {} KB", utc_now(), b.len() / 1024);
                        (b, fin)
                    }
                    Ok(Err(st)) => {
                        println!("{} MISS HTTP {} {site}", utc_now(), st.code);
                        if st.code == 403 || st.code == 429 {
                            blocked.insert(host(site));
                        }
                        continue;
                    }
                    Err(e) => {
                        println!("{} ERR {site}: {e:#}", utc_now());
                        continue;
                    }
                }
            };
            seen_site.insert(site.to_string(), body.clone());
            let mut rank = 0;
            let mut uniq = Vec::new();
            for cap in href.captures_iter(&body) {
                let u = resolve(&base, &cap[1]);
                if uniq.contains(&u) {
                    continue;
                }
                uniq.push(u.clone());
                let r = if bad.is_match(&u) { 90 + rank } else { rank };
                rank += 1;
                if is_github(&u) {
                    continue;
                }
                out.push_str(&format!("{src}\t{key}\t{title}\t{r}\t{u}\t{site}\n"));
            }
            // Digital Commons (scholarworks…): the file is served by viewcontent.cgi, not .pdf — lower priority
            for cap in dc.captures_iter(&body) {
                let u = resolve(&base, &cap[1]);
                if !uniq.contains(&u) {
                    uniq.push(u.clone());
                    out.push_str(&format!("{src}\t{key}\t{title}\t{}\t{u}\t{site}\n", 50 + rank));
                    rank += 1;
                }
            }
        }
    }
    // merge with the previous run: rows whose PDF was already found are skipped by the second run
    let path = dir.join("sites-pdf.tsv");
    let mut lines: Vec<String> = out.lines().map(str::to_string).collect();
    if let Ok(old) = fs::read_to_string(&path) {
        for l in old.lines().skip(1) {
            if !lines.iter().any(|x| x == l) {
                lines.push(l.to_string());
            }
        }
    }
    fs::write(&path, lines.join("\n") + "\n")?;
    println!("{} SITES-PDF → {}", utc_now(), dir.join("sites-pdf.tsv").display());
    Ok(())
}

/// The URL leads to a PDF (by path, ignoring parameters; Digital Commons `viewcontent.cgi` also serves the file).
fn looks_pdf(url: &str) -> bool {
    let path = url.split(['?', '#']).next().unwrap_or(url).to_ascii_lowercase();
    path.ends_with(".pdf") || path.ends_with("viewcontent.cgi")
}

/// Relative link → absolute with respect to `base`.
fn resolve(base: &str, href: &str) -> String {
    let href = href.trim().replace("&amp;", "&");
    if href.starts_with("http://") || href.starts_with("https://") {
        return href;
    }
    let (scheme, rest) = base.split_once("://").unwrap_or(("https", base));
    let host = rest.split('/').next().unwrap_or("");
    if let Some(h) = href.strip_prefix("//") {
        return format!("{scheme}://{h}");
    }
    if href.starts_with('/') {
        return format!("{scheme}://{host}{href}");
    }
    let path = &rest[host.len()..];
    let dir = match path.rfind('/') {
        Some(i) => &path[..=i],
        None => "/",
    };
    let mut parts: Vec<&str> = dir.split('/').filter(|p| !p.is_empty()).collect();
    for seg in href.split('/') {
        match seg {
            "." | "" => {}
            ".." => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    format!("{scheme}://{host}/{}", parts.join("/"))
}

/// AIM list: (course, slug, title) from `<summary>` and `<a href="slug/">` in `p.approved-book`.
fn aim_list(html: &str) -> Result<Vec<(String, String, String)>> {
    let tok = Regex::new(r#"(?s)<summary class="summary">(.*?)</summary>|<a href="([a-z0-9-]+)/?">(.*?)</a>"#)?;
    let mut course = String::new();
    let mut out: Vec<(String, String, String)> = Vec::new();
    let start = html.find("approved-book").unwrap_or(0);
    let end = html.find("</article>").unwrap_or(html.len());
    for c in tok.captures_iter(&html[start.saturating_sub(4000)..end]) {
        if let Some(s) = c.get(1) {
            course = text(s.as_str());
        } else if let (Some(slug), Some(t)) = (c.get(2), c.get(3)) {
            let slug = slug.as_str().to_string();
            if !out.iter().any(|(_, s, _)| *s == slug) && !course.is_empty() {
                out.push((course.clone(), slug, text(t.as_str())));
            }
        }
    }
    Ok(out)
}

/// HTML fragment → text: drop tags, decode entities, collapse whitespace.
fn text(s: &str) -> String {
    let no_tags = Regex::new(r"<[^>]*>").map(|r| r.replace_all(s, " ").into_owned()).unwrap_or_default();
    let un = no_tags
        .replace("&#8217;", "’")
        .replace("&#8220;", "“")
        .replace("&#8221;", "”")
        .replace("&#8211;", "–")
        .replace("&#8212;", "—")
        .replace("&#038;", "&")
        .replace("&nbsp;", " ");
    let un = quick_xml::escape::unescape(&un).map(|c| c.into_owned()).unwrap_or(un);
    un.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Textbook licenses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BookLicense {
    Pd,
    Cc0,
    CcBy,
    CcBySa,
    Gfdl,
    CcByNc,
    CcByNcSa,
    CcByNd,
    CcByNcNd,
    Other,
}

impl BookLicense {
    /// The most permissive of those listed (dual licenses: "CC BY-SA or CC BY-NC-SA" → CC BY-SA).
    pub fn parse(raw: &str) -> BookLicense {
        let r = raw.to_ascii_lowercase();
        let parts = Regex::new(r"\bor\b|\band\b|;|/ |\bdual\b").map(|re| re.split(&r).map(str::to_string).collect::<Vec<_>>()).unwrap_or_default();
        parts.iter().map(|p| Self::one(p)).min().unwrap_or(BookLicense::Other)
    }

    fn one(p: &str) -> BookLicense {
        let p = p.replace(['_', '–'], "-");
        let has = |k: &str| p.contains(k);
        if has("public domain") || has("publicdomain") {
            return BookLicense::Pd;
        }
        if has("cc0") || has("zero") {
            return BookLicense::Cc0;
        }
        if has("free documentation") || has("gfdl") || has("fdl") {
            return BookLicense::Gfdl;
        }
        // "Creative Commons License" without a variant — unknown which (NC? ND?) → Other, check manually
        let cc = has("attribution") || has("cc by") || has("cc-by") || has("cc-by") || has("by-sa") || has("by-nc");
        if !cc {
            return BookLicense::Other;
        }
        let nc = has("noncommercial") || has("non-commercial") || has("-nc") || has(" nc");
        let nd = has("noderiv") || has("no deriv") || has("no-deriv") || has("-nd") || has(" nd");
        let sa = has("sharealike") || has("share-alike") || has("share alike") || has("-sa") || has(" sa");
        match (nc, nd, sa) {
            (false, false, false) => BookLicense::CcBy,
            (false, false, true) => BookLicense::CcBySa,
            (true, false, false) => BookLicense::CcByNc,
            (true, false, true) => BookLicense::CcByNcSa,
            (false, true, _) => BookLicense::CcByNd,
            (true, true, _) => BookLicense::CcByNcNd,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            BookLicense::Pd => "public-domain",
            BookLicense::Cc0 => "cc0",
            BookLicense::CcBy => "cc-by",
            BookLicense::CcBySa => "cc-by-sa",
            BookLicense::Gfdl => "gfdl",
            BookLicense::CcByNc => "cc-by-nc",
            BookLicense::CcByNcSa => "cc-by-nc-sa",
            BookLicense::CcByNd => "cc-by-nd",
            BookLicense::CcByNcNd => "cc-by-nc-nd",
            BookLicense::Other => "other",
        }
    }

    /// May be downloaded and used to train a closed model: CC BY, CC BY-SA, CC0, public domain, GFDL.
    pub fn allowed(self) -> bool {
        matches!(self, BookLicense::Pd | BookLicense::Cc0 | BookLicense::CcBy | BookLicense::CcBySa | BookLicense::Gfdl)
    }
}

/// Topic from the title (and the AIM course / OTL subjects).
/// Science of a book for the science library: first by title, then by the course or OTL subject
/// (`physics: …`, `chemistry: …`); OpenStax books from the Science subject without clues — `other`.
fn science(title: &str, course: &str) -> &'static str {
    let pick = |s: &str| -> Option<&'static str> {
        let s = s.to_ascii_lowercase();
        let has = |k: &str| s.contains(k);
        if has("chemi") {
            Some("chemistry")
        } else if has("physic") || has("thermodynam") || has("electromagnet") || has("relativity") || has("astronom") || has("optics") || has("quantum") {
            Some("physics")
        } else if has("statist") || has("probabilit") || has("data science") || has("regression") || has("econometric") {
            Some("statistics")
        } else {
            None
        }
    };
    let c = course.to_ascii_lowercase();
    pick(title).or_else(|| pick(&c)).unwrap_or(if c.contains("science") && !c.contains("math") { "other" } else { "math" })
}

fn topic(s: &str) -> &'static str {
    let low = s.to_ascii_lowercase();
    if low.contains("physics") || low.contains("software engineering") || low.contains("chemistry") {
        return "not-math";
    }
    // word boundaries: "ring" not inside "Engineering", "group" not inside "subgroups…" (that is algebra anyway)
    let s = format!(" {} ", s.to_ascii_lowercase().replace(|c: char| !c.is_alphanumeric() && c != '-', " "));
    let has = |k: &str| if k.len() <= 6 { s.contains(&format!(" {k} ")) || s.contains(&format!(" {k}s ")) } else { s.contains(k) };
    if has("precalc") || has("pre-calc") || has("college algebra") || has("trigonometr") {
        "precalc"
    } else if has("prealgebra") || has("elementary algebra") || has("intermediate algebra") || has("arithmetic") || has("beginning algebra") || has("basic math") || has("algebra 1") || has("algebra i ") || has("algebra 2") {
        "pre-university"
    } else if has("linear algebra") || has("matrix") || has("matrices") || has("vector space") {
        "linear-algebra"
    } else if has("differential equation") || has("diffy") || has("ordinary differential") {
        "diff-eq"
    } else if has("calculus") {
        "calculus"
    } else if has("probabilit") || has("statist") || has("stochastic") || has("data science") {
        "prob-stat"
    } else if has("discrete") || has("combinator") || has("graph theory") || has("counting") {
        "discrete"
    } else if has("proof") || has("reasoning") || has("transition") || has("logic") || has("set theory") || has("book of") {
        "proofs-logic"
    } else if has("abstract algebra") || has("group") || has("ring") || has("galois") || has("algebraic") || has("modern algebra") {
        "algebra"
    } else if has("numerical") || has("optimization") || has("operations research") || has("modeling") || has("modelling") || has("computational") {
        "applied"
    } else if has("analysis") || has("measure") || has("complex") {
        "analysis"
    } else if has("number theory") || has("theory of numbers") || has("cryptograph") {
        "number-theory"
    } else if has("geometr") || has("topolog") {
        "geometry-topology"
    } else if has("society") || has("liberal") || has("teachers") || has("contemporary") || has("quantitative") || has("business") || has("finance") {
        "general"
    } else if has("algebra") {
        "algebra"
    } else {
        "other"
    }
}

/// Level from the topic and hints in the text (AIM writes "Sophomore level…", "graduate…").
fn level(t: &str, hint: &str) -> &'static str {
    let h = hint.to_ascii_lowercase();
    if h.contains("graduate") && !h.contains("undergraduate") {
        return "grad";
    }
    match t {
        "pre-university" | "general" => "pre",
        "precalc" => "intro",
        "calculus" | "linear-algebra" | "diff-eq" | "prob-stat" | "discrete" | "proofs-logic" => "lower",
        "algebra" | "analysis" | "number-theory" | "geometry-topology" | "applied" => "upper",
        _ => "?",
    }
}

fn host(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    rest.split(['/', '?', '#']).next().unwrap_or("").to_ascii_lowercase()
}

fn is_github(url: &str) -> bool {
    let h = host(url);
    h == "github.com" || h.ends_with(".github.io") || h.ends_with("githubusercontent.com") || h == "codeload.github.com"
}

fn norm_title(t: &str) -> String {
    t.to_ascii_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect()
}

/// Surname of the first author: "Carl Stitz and Jeff Zeager" → `stitz`. Titles like "Calculus"
/// or "Elementary Algebra" are not unique, so AIM and OTL books are matched by title and surname.
fn author_key(authors: &str) -> String {
    let first = authors.split([',', ';']).next().unwrap_or("").split(" and ").next().unwrap_or("");
    first.split_whitespace().last().unwrap_or("").to_ascii_lowercase().chars().filter(|c| c.is_alphanumeric()).collect()
}

struct Row {
    source: &'static str,
    key: String,
    title: String,
    authors: String,
    year: String,
    course: String,
    lic_raw: String,
    lic_url: String,
    /// The book's page in the catalog (OTL, AIM, OpenStax).
    url: String,
    /// The author's or publisher's site (where we look for a PDF if the catalog gave none).
    site: String,
    pdf: String,
    state: String,
    hint: String,
    lang: String,
}

pub const HEADER: &str = "source\tkey\takey\ttitle\tauthors\tyear\tlang\tcourse\ttopic\tlevel\tlicense\tallowed\tlicense_raw\tlicense_url\turl\tsite\tpdf_url\tpdf_host\tgithub\tstate\tin_aim\tin_otl\tnotes\tscience";

pub fn build(dir: &Path, out: &Path) -> Result<()> {
    let all_subjects = std::env::var("BOOKS_ALL").is_ok_and(|v| v == "1");
    let mut rows: Vec<Row> = Vec::new();
    // OpenStax
    let os: Value = serde_json::from_str(&fs::read_to_string(dir.join("openstax-books.json"))?)?;
    for b in os["items"].as_array().context("openstax: no items")? {
        let subj: Vec<String> =
            b["book_subjects"].as_array().map(|a| a.iter().filter_map(|s| s["subject_name"].as_str().map(str::to_string)).collect()).unwrap_or_default();
        // math as before; for other sciences (`BOOKS_ALL=1`) — all Math and Science books
        let want = |s: &String| {
            let s = s.to_ascii_lowercase();
            s.contains("math") || (all_subjects && s.contains("science"))
        };
        if !subj.iter().any(want) {
            continue;
        }
        let authors = b["authors"]
            .as_array()
            .map(|a| a.iter().filter(|x| x["senior_author"].as_bool().unwrap_or(false)).filter_map(|x| x["name"].as_str()).collect::<Vec<_>>().join(", "))
            .unwrap_or_default();
        rows.push(Row {
            source: "openstax",
            key: b["meta"]["slug"].as_str().unwrap_or("").to_string(),
            title: b["title"].as_str().unwrap_or("").to_string(),
            authors,
            year: b["publish_date"].as_str().unwrap_or("").get(..4).unwrap_or("").to_string(),
            course: subj.join("; "),
            lic_raw: b["license_name"].as_str().unwrap_or("").to_string(),
            lic_url: b["license_url"].as_str().unwrap_or("").to_string(),
            url: b["meta"]["html_url"].as_str().unwrap_or("").to_string(),
            site: b["meta"]["html_url"].as_str().unwrap_or("").to_string(),
            pdf: b["high_resolution_pdf_url"].as_str().unwrap_or("").to_string(),
            state: b["book_state"].as_str().unwrap_or("").to_string(),
            hint: String::new(),
            lang: b["meta"]["locale"].as_str().unwrap_or("").to_string(),
        });
    }
    // OTL: all subjects present in the folder (`otl-<subject>-pN.json`); a book in several
    // subjects — one row (the first in subject order)
    let otl_re = Regex::new(r"^otl-([a-z-]+)-p(\d+)\.json$")?;
    let mut otl_pages: Vec<(String, u32, std::path::PathBuf)> = fs::read_dir(dir)?
        .flatten()
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            otl_re.captures(&name).map(|c| (c[1].to_string(), c[2].parse().unwrap_or(0), e.path()))
        })
        .collect();
    otl_pages.sort();
    let mut otl_seen: std::collections::HashSet<i64> = std::collections::HashSet::new();
    for (subject, _, p) in &otl_pages {
        let v: Value = serde_json::from_str(&fs::read_to_string(p)?)?;
        for b in v["data"].as_array().context("otl: no data")? {
            if !otl_seen.insert(b["id"].as_i64().unwrap_or(-1)) {
                continue;
            }
            let authors = b["contributors"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter(|c| c["contribution"].as_str() == Some("Author"))
                        .map(|c| format!("{} {}", c["first_name"].as_str().unwrap_or(""), c["last_name"].as_str().unwrap_or("")).trim().to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default();
            let subj: Vec<String> = b["subjects"].as_array().map(|a| a.iter().filter_map(|s| s["name"].as_str().map(str::to_string)).collect()).unwrap_or_default();
            let pdf = b["formats"]
                .as_array()
                .and_then(|a| a.iter().find(|f| f["type"].as_str().is_some_and(|t| t.eq_ignore_ascii_case("pdf"))))
                .and_then(|f| f["url"].as_str())
                .unwrap_or("")
                .to_string();
            let site = b["publishers"].as_array().and_then(|a| a.first()).and_then(|p| p["url"].as_str()).unwrap_or("").to_string();
            let desc = b["description"].as_str().unwrap_or("");
            rows.push(Row {
                source: "otl",
                key: b["id"].as_i64().map_or(String::new(), |i| i.to_string()),
                title: b["title"].as_str().unwrap_or("").to_string(),
                authors,
                year: b["copyright_year"].as_i64().map_or(String::new(), |y| y.to_string()),
                course: if subject == "mathematics" { subj.join("; ") } else { format!("{subject}: {}", subj.join("; ")) },
                lic_raw: b["license"].as_str().unwrap_or("").to_string(),
                lic_url: String::new(),
                url: b["url"].as_str().unwrap_or("").to_string(),
                site,
                pdf,
                state: String::new(),
                hint: desc.chars().take(400).collect(),
                lang: b["language"].as_str().unwrap_or("").to_string(),
            });
        }
    }
    // AIM (math only; absent from the folders of other sciences)
    let list = fs::read_to_string(dir.join("aim-approved.html")).unwrap_or_default();
    for (course, slug, title) in aim_list(&list)? {
        let slug_for_url = slug.clone();
        let p = dir.join("aim").join(format!("{slug}.html"));
        let (mut authors, mut lic, mut site, mut hint) = (String::new(), String::new(), String::new(), String::new());
        if let Ok(h) = fs::read_to_string(&p) {
            let body = &h[h.find("entry-content").unwrap_or(0)..h.find("</article>").unwrap_or(h.len())];
            if let Some(c) = Regex::new(r"(?s)<h4>(.*?)</h4>")?.captures(body) {
                authors = text(&c[1]);
            }
            for c in Regex::new(r"(?s)<tr>\s*<td>(.*?)</td>\s*<td>(.*?)</td>")?.captures_iter(body) {
                if text(&c[1]).to_ascii_lowercase().starts_with("license") {
                    lic = text(&c[2]);
                }
            }
            let lis: Vec<String> = Regex::new(r"(?s)<li>(.*?)(?:</li>|<li>|</ul>)")?.captures_iter(body).map(|c| text(&c[1])).collect();
            hint = lis.iter().take(3).cloned().collect::<Vec<_>>().join("; ");
            if let Some(c) = Regex::new(r#"<a href="([^"]+)"[^>]*>[^<]*(?:download|more information|website)"#)?.captures(body) {
                site = c[1].to_string();
            } else if let Some(c) = Regex::new(r#"<a href="(https?://[^"]+)""#)?.captures(body) {
                site = c[1].to_string();
            }
        }
        rows.push(Row {
            source: "aim",
            key: slug,
            title,
            authors,
            year: String::new(),
            course,
            lic_raw: lic,
            lic_url: String::new(),
            url: format!("{AIM_LIST}{}/", slug_for_url),
            site,
            pdf: String::new(),
            state: String::new(),
            hint,
            lang: "eng".into(),
        });
    }
    // cross-references: an AIM book is also in OTL (by title) — take the PDF from OTL
    let mut otl_by_title: HashMap<(String, String), usize> = HashMap::new();
    let mut aim_titles: HashMap<(String, String), usize> = HashMap::new();
    for (i, r) in rows.iter().enumerate() {
        let k = (norm_title(&r.title), author_key(&r.authors));
        match r.source {
            "otl" => {
                otl_by_title.entry(k).or_insert(i);
            }
            "aim" => {
                aim_titles.entry(k).or_insert(i);
            }
            _ => {}
        }
    }
    // PDFs found by `books-pdf` on sites: lowest rank (the first "non-suspicious" in page order)
    let mut site_pdf: HashMap<(String, String), String> = HashMap::new();
    // extra "not a book" cases: lab manuals, workbooks, companions, instructor notes, debug builds
    let not_book = Regex::new(r"(?i)/lab\.pdf|labs?\.pdf|workbook|activit|companion|teachingnotes|act(oer)?\.pdf|debug|latex-tutorial|review|jhanswer")?;
    if let Ok(t) = fs::read_to_string(dir.join("sites-pdf.tsv")) {
        let mut best: HashMap<(String, String), (u32, String)> = HashMap::new();
        for l in t.lines().skip(1) {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() < 5 {
                continue;
            }
            let rank: u32 = c[3].parse().unwrap_or(999);
            if rank >= 90 || not_book.is_match(c[4]) {
                continue;
            }
            let k = (c[0].to_string(), c[1].to_string());
            if best.get(&k).is_none_or(|(r, _)| rank < *r) {
                best.insert(k, (rank, c[4].to_string()));
            }
        }
        site_pdf = best.into_iter().map(|(k, (_, u))| (k, u)).collect();
    }
    // License conflict: the same book (title + surname) where one source explicitly states closed terms
    // ("Copyright…", "for commercial use contact…") and another an open license → do not download.
    let mut explicit_closed: std::collections::HashSet<(String, String)> = std::collections::HashSet::new();
    for r in &rows {
        let raw = r.lic_raw.to_ascii_lowercase();
        if BookLicense::parse(&r.lic_raw) == BookLicense::Other && !raw.trim().is_empty() && !raw.contains("creative commons") && raw != "none" {
            explicit_closed.insert((norm_title(&r.title), author_key(&r.authors)));
        }
    }
    let openstax_lic: HashMap<String, BookLicense> =
        rows.iter().filter(|r| r.source == "openstax" && !r.pdf.is_empty()).map(|r| (r.pdf.clone(), BookLicense::parse(&r.lic_raw))).collect();
    let openstax_by_title: HashMap<String, BookLicense> =
        rows.iter().filter(|r| r.source == "openstax").map(|r| (norm_title(&r.title), BookLicense::parse(&r.lic_raw))).collect();
    let mut gh_cache: HashMap<String, bool> = HashMap::new();
    let mut w = BufWriter::new(File::create(out)?);
    writeln!(w, "{HEADER}")?;
    let (mut total, mut allowed_n) = (0, 0);
    let mut by_lic: HashMap<&'static str, usize> = HashMap::new();
    for r in &rows {
        let mut lic = BookLicense::parse(&r.lic_raw);
        let mut lic_note = "";
        // AIM writes "Creative Commons License" without a variant — take the more precise license of the same book from OTL
        if r.source == "aim" && lic == BookLicense::Other && r.lic_raw.to_ascii_lowercase().contains("creative commons") {
            if let Some(&i) = otl_by_title.get(&(norm_title(&r.title), author_key(&r.authors))) {
                let o = BookLicense::parse(&rows[i].lic_raw);
                if o != BookLicense::Other {
                    lic = o;
                    lic_note = "license from OTL (no variant in AIM); ";
                }
            }
        }
        // PDF from assets.openstax.org: the current license comes from the OpenStax API (2e editions moved to
        // CC BY-NC-SA, while OTL remembers CC BY from when they were added)
        if r.source != "openstax" {
            let by_title = if host(&r.pdf) == "assets.openstax.org" { openstax_by_title.get(&norm_title(&r.title)) } else { None };
            if let Some(o) = openstax_lic.get(&r.pdf).or(by_title) {
                if *o != lic {
                    lic = *o;
                    lic_note = "license from the OpenStax API (current; the catalog gave another); ";
                }
            }
        }
        if lic.allowed() && explicit_closed.contains(&(norm_title(&r.title), author_key(&r.authors))) {
            lic = BookLicense::Other;
            lic_note = "license conflict: another source states closed terms — not downloading; ";
        }
        let t = match topic(&r.title) {
            "other" => topic(&format!("{} {}", r.title, r.course)),
            t => t,
        };
        let lv = level(t, &r.hint);
        let nt = (norm_title(&r.title), author_key(&r.authors));
        let in_aim = aim_titles.contains_key(&nt);
        let in_otl = otl_by_title.get(&nt).map(|&i| rows[i].url.clone()).unwrap_or_default();
        let mut pdf = r.pdf.clone();
        let mut notes = String::from(lic_note);
        if r.source == "aim" && pdf.is_empty() {
            if let Some(&i) = otl_by_title.get(&nt) {
                pdf = rows[i].pdf.clone();
                notes.push_str("PDF from OTL; ");
            }
        }
        if !looks_pdf(&pdf) {
            if let Some(u) = site_pdf.get(&(r.source.to_string(), r.key.clone())) {
                pdf = u.clone();
                notes.push_str("PDF found on the book's site; ");
            }
        }
        // GitHub — both by name and by address (GitHub Pages under a custom domain), DNS cached
        let mut dns_gh = |u: &str| -> bool {
            let h = host(u);
            !h.is_empty() && *gh_cache.entry(h.clone()).or_insert_with(|| crate::robots::github_host(&h))
        };
        let gh = is_github(&pdf) || (!looks_pdf(&pdf) && is_github(&r.site)) || (!pdf.is_empty() && dns_gh(&pdf)) || (!looks_pdf(&pdf) && !r.site.is_empty() && dns_gh(&r.site));
        if gh {
            notes.push_str("GitHub — left alone; ");
        }
        notes.push_str(&r.hint.replace(['\t', '\n'], " "));
        total += 1;
        if lic.allowed() {
            allowed_n += 1;
        }
        *by_lic.entry(lic.code()).or_default() += 1;
        writeln!(
            w,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{t}\t{lv}\t{}\t{}\t{}\t{}\t{}\t{}\t{pdf}\t{}\t{gh}\t{}\t{in_aim}\t{in_otl}\t{}\t{}",
            r.source,
            r.key,
            author_key(&r.authors),
            r.title.replace('\t', " "),
            r.authors.replace('\t', " "),
            r.year,
            r.lang,
            r.course.replace('\t', " "),
            lic.code(),
            lic.allowed(),
            r.lic_raw.replace('\t', " "),
            r.lic_url,
            r.url,
            r.site,
            host(&pdf),
            r.state,
            notes.trim().chars().take(500).collect::<String>(),
            science(&r.title, &r.course),
        )?;
    }
    w.flush()?;
    let mut v: Vec<_> = by_lic.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    println!("BOOKS {total} rows (OpenStax+OTL+AIM), {allowed_n} with allowed licenses → {}", out.display());
    for (l, n) in v {
        println!("  {l}\t{n}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book_licenses() {
        assert_eq!(BookLicense::parse("Free Documentation License (GNU)"), BookLicense::Gfdl);
        assert_eq!(BookLicense::parse("Attribution-NonCommercial-ShareAlike"), BookLicense::CcByNcSa);
        assert_eq!(BookLicense::parse("Creative Commons Attribution License"), BookLicense::CcBy);
        assert_eq!(BookLicense::parse("Attribution-ShareAlike"), BookLicense::CcBySa);
        assert_eq!(BookLicense::parse("CC BY-SA 4.0 or CC BY-NC-SA 4.0"), BookLicense::CcBySa);
        assert_eq!(BookLicense::parse("Public Domain"), BookLicense::Pd);
        // negative controls: NC, ND and unknown are not allowed
        assert!(!BookLicense::parse("Attribution-NonCommercial").allowed());
        assert!(!BookLicense::parse("Creative Commons BY-NC-ND").allowed());
        assert!(!BookLicense::parse("Attribution-NoDerivs").allowed());
        assert!(!BookLicense::parse("All rights reserved").allowed());
        assert!(!BookLicense::parse("").allowed());
        assert!(!BookLicense::parse("Creative Commons License").allowed());
        assert_eq!(BookLicense::parse("CC-BY-SA and CC-BY-NC-SA"), BookLicense::CcBySa);
        assert_eq!(BookLicense::parse("Creative Commons-Attribution"), BookLicense::CcBy);
    }

    #[test]
    fn topics_and_hosts() {
        assert_eq!(topic("A First Course in Linear Algebra"), "linear-algebra");
        assert_eq!(topic("Precalculus"), "precalc");
        assert_eq!(topic("Calculus Volume 1"), "calculus");
        assert_eq!(topic("Notes on Diffy Qs"), "diff-eq");
        assert_eq!(topic("Abstract Algebra: Theory and Applications"), "algebra");
        // negative control for word boundaries: "Engineering" does not make a book algebra
        assert_ne!(topic("Evidence-based Software Engineering"), "algebra");
        assert_eq!(topic("An Introduction to the Theory of Numbers"), "number-theory");
        assert_eq!(topic("Matrix Theory and Linear Algebra"), "linear-algebra");
        assert_eq!(topic("Introduction to Probability"), "prob-stat");
        assert_eq!(topic("Book of Proof"), "proofs-logic");
        assert_eq!(topic("Calculus-Based Physics I"), "not-math");
        assert_eq!(science("Calculus-Based Physics I", ""), "physics");
        assert_eq!(science("Chemistry: Atoms First", "Science"), "chemistry");
        assert_eq!(science("Introductory Statistics", "Math"), "statistics");
        assert_eq!(science("Biology 2e", "Science"), "other");
        assert_eq!(science("Linear Algebra", "Algebra; Pure"), "math");
        assert_eq!(science("Mechanics", "physics: Physics"), "physics");
        assert_eq!(author_key("Carl Stitz and Jeff Zeager"), "stitz");
        assert_eq!(author_key("Robert A. Beezer"), "beezer");
        assert_eq!(author_key("David H. Collingwood, K. David Prince"), "collingwood");
        assert!(is_github("https://davidaustinm.github.io/ula/ula.pdf"));
        assert!(is_github("https://github.com/x/y/raw/main/book.pdf"));
        assert!(!is_github("https://linear.ups.edu/download/fcla.pdf"));
        assert_eq!(resolve("https://www.jirka.org/diffyqs/", "diffyqs.pdf"), "https://www.jirka.org/diffyqs/diffyqs.pdf");
        assert_eq!(resolve("https://a.org/x/index.html", "../b/c.pdf"), "https://a.org/b/c.pdf");
        assert_eq!(resolve("https://a.org/x/", "/d.pdf"), "https://a.org/d.pdf");
    }

    #[test]
    fn aim_list_parses() {
        let h = r#"<p>x</p><details><summary class="summary">Calculus</summary><p class="approved-book"><i><a href="guichard/">Calculus<br />
</a></i>David Guichard</p><p class="approved-book"><i><a href="boelkins/">Active Calculus</a></i></p></details></article>"#;
        let v = aim_list(h).unwrap();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], ("Calculus".to_string(), "guichard".to_string(), "Calculus".to_string()));
    }
}
