//! Stage 4: web pages. Author pages (the scholarly profiles' homepages of the paper's authors), pages
//! named for claims and any page given explicitly are fetched only by Sea Spider (robots.txt, one
//! request per IP every 2 s, its own robot identity): this module writes Sea Spider's queue and asks
//! it in English over its socket (as caller `whale`), and later imports what Sea Spider stored into
//! the closed corpus. Each page keeps the licence the page states (a `rel="license"` link, a Creative
//! Commons link, a rights meta tag or a copyright line) and the site's terms page when it links one.
//! Every page stays internal: the operator's rule of 2026-11-04 keeps Sea Spider's downloads and
//! their trees out of every public corpus, whatever licence the site states (`corpus::assert_public`).
use crate::corpus::{Corpus, Doc};
use crate::graph::Neighbourhood;
use crate::licence::{self, Origin};
use crate::missing::shadow;
use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::path::Path;

/// The caller name the Owls declare to Sea Spider (they are Whales).
pub const CALLER: &str = "whale";
/// Sea Spider takes at most 16 URLs per fetch request.
pub const BATCH: usize = 16;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Target {
    pub url: String,
    /// `author_page`, `claim`, `given`.
    pub why: String,
    /// The author, claim or note it is for.
    pub about: String,
}

fn web_url(u: &str) -> bool {
    (u.starts_with("https://") || u.starts_with("http://")) && !u.contains(char::is_whitespace) && !shadow(u)
}

/// Pages to fetch for a neighbourhood: the authors' homepages, then the given URLs; each once,
/// shadow-library hosts never.
pub fn plan(g: &Neighbourhood, extra: &[Target]) -> Vec<Target> {
    let mut seen = BTreeSet::new();
    let mut out = vec![];
    for a in &g.authors {
        if let Some(h) = a.homepage.as_deref().map(str::trim).filter(|h| web_url(h)) {
            if seen.insert(h.to_string()) {
                out.push(Target { url: h.into(), why: "author_page".into(), about: a.name.clone() });
            }
        }
    }
    for t in extra {
        if web_url(&t.url) && seen.insert(t.url.clone()) {
            out.push(t.clone());
        }
    }
    out
}

/// Sea Spider's queue file: one JSON object per line with `url`.
pub fn queue(targets: &[Target]) -> String {
    targets.iter().map(|t| json!({"url": t.url, "why": t.why, "about": t.about}).to_string() + "\n").collect()
}

/// The English requests for Sea Spider, at most 16 URLs each.
pub fn prompts(targets: &[Target]) -> Vec<String> {
    targets
        .chunks(BATCH)
        .map(|c| {
            let urls: Vec<&str> = c.iter().map(|t| t.url.as_str()).collect();
            if urls.len() == 1 {
                format!("Please fetch {}.", urls[0])
            } else {
                format!("Please fetch these pages: {}.", urls.join(", "))
            }
        })
        .collect()
}

/// Ask Sea Spider to fetch the targets (queued at once; it answers without waiting for the fetch).
pub fn ask(socket: &Path, id: &str, targets: &[Target]) -> Result<Vec<Value>> {
    let mut out = vec![];
    for (i, p) in prompts(targets).into_iter().enumerate() {
        let r = mova_octopus::relay::ask_animal(socket, CALLER, &format!("{id}-{i}"), &p, false)?;
        out.push(json!({"prompt": p, "ok": r.ok, "answer": r.answer}));
    }
    Ok(out)
}

fn lower_find(h: &str, from: usize, pat: &str) -> Option<usize> {
    h.get(from..)?.to_ascii_lowercase().find(pat).map(|i| i + from)
}

/// Drop whole elements (`<script …>…</script>` and the like).
fn drop_elements(h: &str, tags: &[&str]) -> String {
    let mut s = h.to_string();
    for t in tags {
        loop {
            let Some(a) = lower_find(&s, 0, &format!("<{t}")) else { break };
            let after = s.as_bytes().get(a + t.len() + 1).copied().unwrap_or(b'>');
            if !(after == b'>' || after.is_ascii_whitespace() || after == b'/') {
                // Another tag sharing the prefix (`<navbar>` for `nav`): neutralize this one.
                s.replace_range(a..a + 1, "&lt;");
                continue;
            }
            let end = lower_find(&s, a, &format!("</{t}>")).map(|e| e + t.len() + 3).unwrap_or(s.len());
            s.replace_range(a..end, "\n");
        }
    }
    s
}

pub fn entities(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(i) = rest.find('&') {
        out.push_str(&rest[..i]);
        rest = &rest[i..];
        let end = rest.find(';').filter(|&e| e <= 10);
        let Some(e) = end else {
            out.push('&');
            rest = &rest[1..];
            continue;
        };
        let name = &rest[1..e];
        let c = match name {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" | "#39" => Some('\''),
            "nbsp" => Some(' '),
            "mdash" => Some('—'),
            "ndash" => Some('–'),
            "hellip" => Some('…'),
            "copy" => Some('©'),
            n if n.starts_with("#x") || n.starts_with("#X") => u32::from_str_radix(&n[2..], 16).ok().and_then(char::from_u32),
            n if n.starts_with('#') => n[1..].parse().ok().and_then(char::from_u32),
            _ => None,
        };
        match c {
            Some(c) => {
                out.push(c);
                rest = &rest[e + 1..];
            }
            None => {
                out.push('&');
                rest = &rest[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn attr(tag: &str, name: &str) -> Option<String> {
    let l = tag.to_ascii_lowercase();
    let mut from = 0;
    while let Some(i) = l[from..].find(name).map(|i| i + from) {
        let before = l[..i].chars().last();
        let rest = l[i + name.len()..].trim_start();
        if before.is_some_and(|c| c.is_whitespace()) && rest.starts_with('=') {
            let off = tag.len() - rest.len() + 1;
            let v = tag[off..].trim_start();
            let (q, body) = match v.chars().next() {
                Some(q @ ('"' | '\'')) => (Some(q), &v[1..]),
                _ => (None, v),
            };
            let end = match q {
                Some(q) => body.find(q),
                None => body.find(|c: char| c.is_whitespace() || c == '>'),
            }
            .unwrap_or(body.len());
            return Some(entities(&body[..end]));
        }
        from = i + name.len();
    }
    None
}

/// Every start tag of the given name, as raw text `<a …>`.
fn tags<'a>(h: &'a str, name: &str) -> Vec<&'a str> {
    let l = h.to_ascii_lowercase();
    let mut out = vec![];
    let mut from = 0;
    let pat = format!("<{name}");
    while let Some(i) = l[from..].find(&pat).map(|i| i + from) {
        let after = l.as_bytes().get(i + pat.len()).copied().unwrap_or(b'>');
        let end = l[i..].find('>').map(|e| e + i + 1).unwrap_or(l.len());
        if after == b'>' || after.is_ascii_whitespace() || after == b'/' {
            out.push(&h[i..end]);
        }
        from = end;
    }
    out
}

fn absolute(base: &str, href: &str) -> String {
    if href.starts_with("http://") || href.starts_with("https://") {
        return href.into();
    }
    let origin: String = base.splitn(4, '/').take(3).collect::<Vec<_>>().join("/");
    if href.starts_with("//") {
        return format!("{}{href}", base.split("//").next().unwrap_or("https:"));
    }
    if href.starts_with('/') {
        return format!("{origin}{href}");
    }
    let dir = base.rsplit_once('/').map(|(d, _)| d).unwrap_or(base);
    format!("{dir}/{href}")
}

/// The licence a page states: `(statement, url)`, else None.
pub fn page_licence(html: &str, base: &str) -> Option<(String, Option<String>)> {
    for t in tags(html, "a").into_iter().chain(tags(html, "link")) {
        let rel = attr(t, "rel").unwrap_or_default().to_lowercase();
        let href = attr(t, "href").unwrap_or_default();
        if rel.split_whitespace().any(|r| r == "license") && !href.is_empty() {
            let u = absolute(base, &href);
            let name = if u.contains("creativecommons.org") { u.clone() } else { format!("licence page {u}") };
            return Some((name, Some(u)));
        }
    }
    for t in tags(html, "a") {
        let href = attr(t, "href").unwrap_or_default();
        if href.contains("creativecommons.org/licenses/") || href.contains("creativecommons.org/publicdomain/") {
            return Some((href.clone(), Some(href)));
        }
    }
    for t in tags(html, "meta") {
        let name = attr(t, "name").or_else(|| attr(t, "property")).unwrap_or_default().to_lowercase();
        if matches!(name.as_str(), "dc.rights" | "dcterms.rights" | "license" | "copyright" | "dc.license") {
            if let Some(c) = attr(t, "content").filter(|c| !c.trim().is_empty()) {
                return Some((c.trim().to_string(), None));
            }
        }
    }
    let text = entities(&strip_tags(html)).to_lowercase();
    if text.contains("all rights reserved") || text.contains('©') {
        return Some(("all rights reserved (stated on the page)".into(), None));
    }
    None
}

/// A link to the site's terms, licence or copyright page.
pub fn terms_url(html: &str, base: &str) -> Option<String> {
    tags(html, "a").into_iter().filter_map(|t| attr(t, "href")).find(|h| {
        let l = h.to_lowercase();
        if l.contains("creativecommons.org") {
            return false;
        }
        ["/terms", "/tos", "/legal", "/copyright", "/license", "/licence"].iter().any(|p| l.contains(p))
    }).map(|h| absolute(base, &h))
}

fn strip_tags(h: &str) -> String {
    let mut out = String::with_capacity(h.len());
    let mut inside = false;
    for c in h.chars() {
        match c {
            '<' => inside = true,
            '>' if inside => inside = false,
            c if !inside => out.push(c),
            _ => {}
        }
    }
    out
}

/// A page as Markdown for the reader: `# title`, `## Page`, then paragraphs; headings become `##`.
pub fn html_to_markdown(html: &str) -> (String, String) {
    let title = tags(html, "title")
        .first()
        .and_then(|t| {
            let l = html.to_ascii_lowercase();
            let a = l.find(&t.to_ascii_lowercase())? + t.len();
            let e = l[a..].find("</title>")? + a;
            Some(entities(&strip_tags(&html[a..e])).split_whitespace().collect::<Vec<_>>().join(" "))
        })
        .unwrap_or_default();
    let body = drop_elements(html, &["head", "script", "style", "noscript", "svg", "nav", "header", "footer", "form", "aside", "template"]);
    let mut s = String::with_capacity(body.len());
    let mut rest = body.as_str();
    while let Some(i) = rest.find('<') {
        s.push_str(&rest[..i]);
        let end = rest[i..].find('>').map(|e| e + i + 1).unwrap_or(rest.len());
        let tag = rest[i..end].to_ascii_lowercase();
        let name: String = tag.trim_start_matches(['<', '/']).chars().take_while(|c| c.is_ascii_alphanumeric()).collect();
        let closing = tag.starts_with("</");
        match name.as_str() {
            "h1" | "h2" | "h3" | "h4" => s.push_str(if closing { "\n\n" } else { "\n\n## " }),
            "p" | "div" | "li" | "ul" | "ol" | "tr" | "table" | "blockquote" | "article" | "section" | "main" | "pre" | "dd" | "dt" => s.push_str("\n\n"),
            "br" => s.push('\n'),
            "td" | "th" => s.push(' '),
            _ => {}
        }
        rest = &rest[end..];
    }
    s.push_str(rest);
    let text = entities(&s);
    let mut paras: Vec<String> = vec![];
    let mut cur: Vec<String> = vec![];
    for line in text.lines() {
        let l = line.split_whitespace().collect::<Vec<_>>().join(" ");
        if l.is_empty() {
            if !cur.is_empty() {
                paras.push(cur.join(" "));
                cur.clear();
            }
        } else if l.starts_with("## ") {
            if !cur.is_empty() {
                paras.push(cur.join(" "));
                cur.clear();
            }
            if l.len() > 3 {
                paras.push(l);
            }
        } else {
            cur.push(l);
        }
    }
    if !cur.is_empty() {
        paras.push(cur.join(" "));
    }
    let t = if title.is_empty() { "Untitled page".to_string() } else { title.clone() };
    let mut md = format!("# {t}\n\n## Page\n\n");
    for p in paras {
        md.push_str(&p);
        md.push_str("\n\n");
    }
    (title, md)
}

/// The id of a page in the corpus.
pub fn doc_id(url: &str) -> String {
    use sha2::{Digest, Sha256};
    let h: String = Sha256::digest(url.as_bytes()).iter().take(8).map(|x| format!("{x:02x}")).collect();
    format!("web:{h}")
}

/// A stored page as Markdown for the reader: `(title, markdown)`.
pub fn page_markdown(rec: &seaspider::store::Record, body: &[u8], url: &str) -> (String, String) {
    let ct = rec.content_type.clone().unwrap_or_default().to_lowercase();
    let text = String::from_utf8_lossy(body).to_string();
    if ct.contains("html") {
        html_to_markdown(&text)
    } else {
        (url.to_string(), format!("# {url}\n\n## Page\n\n{text}\n"))
    }
}

/// Register the targets Sea Spider has stored (HTML or plain text, HTTP 2xx) in the corpus. The
/// page stays in Sea Spider's compressed store; the corpus keeps a reference and parses it on request.
pub fn import(c: &mut Corpus, store: &Path, targets: &[Target]) -> Result<Value> {
    let r = seaspider::store::Reader::open(store).map_err(|e| anyhow!("{e}"))?;
    let mut added = vec![];
    let mut missing = vec![];
    for t in targets {
        let Ok((rec, body)) = r.get(&t.url) else {
            missing.push(json!({"url": t.url, "why": "not stored by Sea Spider (yet)"}));
            continue;
        };
        let ct = rec.content_type.clone().unwrap_or_default().to_lowercase();
        if !rec.status.is_some_and(|s| (200..300).contains(&s)) || !(ct.contains("html") || ct.starts_with("text/plain")) {
            missing.push(json!({"url": t.url, "why": format!("status {:?}, type {ct}", rec.status)}));
            continue;
        }
        let html = String::from_utf8_lossy(&body).to_string();
        let base = rec.final_url.clone().unwrap_or_else(|| t.url.clone());
        let (title, md) = page_markdown(&rec, &body, &t.url);
        let (lic, from) = match page_licence(&html, &base) {
            Some((l, _)) => (l, "page"),
            None => (String::new(), "none"),
        };
        let mut d = Doc::new(&doc_id(&t.url), "web", Origin::Seaspider);
        d.title = if title.is_empty() { t.url.clone() } else { title };
        d.source_kind = "seaspider".into();
        d.source = t.url.clone();
        d.store = Some(store.display().to_string());
        d.roles = vec![t.why.clone()];
        d.licence_class = licence::classify(&lic);
        d.licence = lic;
        d.licence_from = from.into();
        d.terms_url = terms_url(&html, &base);
        d.cite = json!({"url": t.url, "final_url": rec.final_url, "fetched_unix": rec.fetched_unix, "body_sha256": rec.sha256, "about": t.about});
        let id = d.id.clone();
        c.register(d, &md)?;
        added.push(id);
    }
    Ok(json!({"imported": added, "missing": missing}))
}

pub fn load_targets(p: &Path) -> Result<Vec<Target>> {
    let s = std::fs::read_to_string(p)?;
    let mut out = vec![];
    for l in s.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        if l.starts_with('{') {
            let v: Value = serde_json::from_str(l)?;
            let url = v["url"].as_str().ok_or_else(|| anyhow!("queue line without url"))?;
            out.push(Target { url: url.into(), why: v["why"].as_str().unwrap_or("given").into(), about: v["about"].as_str().unwrap_or("").into() });
        } else {
            let url = l.split('\t').next().unwrap_or(l);
            out.push(Target { url: url.into(), why: "given".into(), about: String::new() });
        }
    }
    if out.iter().any(|t| !web_url(&t.url)) {
        bail!("every target must be an http(s) URL outside shadow libraries");
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Limits;
    use crate::corpus::tests::{built, model};
    use crate::graph;
    use crate::library::tests::{fixture, tmp};
    use crate::library::Library;

    const PAGE: &str = r#"<html><head><title>Ann's blog &amp; notes</title><script>var x = "<p>no</p>";</script></head>
<body><nav><a href="/">Home</a></nav><h1>On error types</h1><p>The ERRANT toolkit classifies
edits by their part of speech.</p><p>Error types are useful for feedback to learners.</p>
<footer><a rel="license" href="https://creativecommons.org/licenses/by/4.0/">CC BY</a> <a href="/terms">Terms</a></footer></body></html>"#;

    #[test]
    fn html_licence_terms_and_prompts() {
        let (t, md) = html_to_markdown(PAGE);
        assert_eq!(t, "Ann's blog & notes");
        assert!(md.starts_with("# Ann's blog & notes\n\n## Page\n\n## On error types"));
        assert!(md.contains("The ERRANT toolkit classifies edits by their part of speech."));
        assert!(!md.contains("no</p>") && !md.contains("Home"));
        let (l, u) = page_licence(PAGE, "https://ann.example.org/post").unwrap();
        assert_eq!(licence::classify(&l), licence::Class::By);
        assert_eq!(u.as_deref(), Some("https://creativecommons.org/licenses/by/4.0/"));
        assert_eq!(terms_url(PAGE, "https://ann.example.org/post").as_deref(), Some("https://ann.example.org/terms"));
        assert_eq!(page_licence("<p>© 2024 Ann</p>", "https://a.example/").unwrap().0, "all rights reserved (stated on the page)");
        assert!(page_licence("<p>plain</p>", "https://a.example/").is_none());
        let ts: Vec<Target> = (0..17).map(|i| Target { url: format!("https://a.example/{i}"), why: "given".into(), about: String::new() }).collect();
        let p = prompts(&ts);
        assert_eq!(p.len(), 2);
        assert!(p[0].starts_with("Please fetch these pages: https://a.example/0, "));
        assert_eq!(p[1], "Please fetch https://a.example/16.");
        let d = tmp("webplan");
        let lib = Library::load(&fixture(&d)).unwrap();
        let g = graph::build(&lib, "pa-2017-error-types", &Limits::default()).unwrap();
        let extra = vec![Target { url: "https://sci-hub.example/x".into(), why: "given".into(), about: String::new() }, Target { url: "https://ann.example.org/".into(), why: "given".into(), about: String::new() }];
        let pl = plan(&g, &extra);
        assert_eq!(pl, vec![Target { url: "https://ann.example.org/".into(), why: "author_page".into(), about: "Ann Pa".into() }]);
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn imports_stored_pages_as_internal_documents() {
        if !model() {
            return;
        }
        let (d, mut c) = built("webimport");
        let store = d.join("spider");
        let mut st = seaspider::store::Store::open(&store, 1 << 20, 3).unwrap();
        let (loc, sha) = st.put_body(PAGE.as_bytes()).unwrap();
        let rec = seaspider::store::Record { v: 2, kind: "page".into(), url: "https://ann.example.org/post".into(), outcome: "fetched".into(), status: Some(200), content_type: Some("text/html; charset=utf-8".into()), fetched_unix: 1_700_000_000, seg: Some(loc.seg), off: Some(loc.off), len: Some(loc.len), body_bytes: Some(loc.body_bytes), sha256: Some(sha), ..Default::default() };
        st.append(&rec).unwrap();
        drop(st);
        let ts = vec![Target { url: "https://ann.example.org/post".into(), why: "author_page".into(), about: "Ann Pa".into() }, Target { url: "https://ann.example.org/none".into(), why: "given".into(), about: String::new() }];
        let r = import(&mut c, &store, &ts).unwrap();
        assert_eq!(r["imported"].as_array().unwrap().len(), 1, "{r}");
        assert_eq!(r["missing"].as_array().unwrap().len(), 1);
        let w = c.doc(&doc_id("https://ann.example.org/post")).unwrap().clone();
        assert_eq!(w.licence_class, licence::Class::By);
        assert_eq!(w.terms_url.as_deref(), Some("https://ann.example.org/terms"));
        // CC BY on the page, still internal: Sea Spider's pages never go public.
        assert!(w.internal_only && !w.public_admissible);
        assert!(crate::corpus::assert_public(&w).is_err());
        assert!(!c.cached(&w.id));
        assert!(c.parsed(&w.id).unwrap().iter().any(|s| s.text.contains("ERRANT toolkit")));
        let _ = std::fs::remove_dir_all(d);
    }
}
