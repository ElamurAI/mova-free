//! Links from HTML: a one-pass byte scanner with no tree, no copies and no error state; any input
//! leads somewhere. The contents of `<script>`, `<style>`, `<noscript>`, `<template>` and comments
//! are skipped. Yields `href` of `<a>`, `<area>`, `<link rel=canonical|alternate|next|prev>` and
//! `<base href>`, each with a nofollow mark: `rel` containing `nofollow`/`ugc`/`sponsored`, links
//! inside `<noindex>`, and every link of a page with `<meta name=robots content=…nofollow…>`.
//!
//! Taken from the family Spider (`spider/src/links.rs`) with comments in English; behaviour
//! unchanged except that `in_domain` is not carried (Sea Spider follows the same host only).

use reqwest::Url;

/// Raw links of a page: (base, hrefs).
pub fn extract(html: &[u8]) -> (Option<String>, Vec<String>) {
    let (base, v) = extract_rel(html);
    (base, v.into_iter().map(|x| x.0).collect())
}

/// Raw links of a page with a nofollow mark: (base, [(href, nofollow)]).
pub fn extract_rel(html: &[u8]) -> (Option<String>, Vec<(String, bool)>) {
    let mut out: Vec<(String, bool)> = Vec::new();
    let mut base = None;
    let mut noindex = 0u32;
    let mut page_nofollow = false;
    let n = html.len();
    let mut i = 0;
    while i < n {
        if html[i] != b'<' {
            i += 1;
            continue;
        }
        if html[i..].starts_with(b"<!--") {
            i = find(html, i + 4, b"-->").map_or(n, |j| j + 3);
            continue;
        }
        if html.get(i + 1) == Some(&b'/') {
            // a closing `</noindex>`
            if tag_name(html, i + 2).0 == "noindex" {
                noindex = noindex.saturating_sub(1);
            }
            i += 2;
            continue;
        }
        let (name, mut j) = tag_name(html, i + 1);
        if name.is_empty() {
            i += 1;
            continue;
        }
        let mut href = None;
        let mut rel = String::new();
        let (mut meta_name, mut content) = (String::new(), String::new());
        // attributes up to `>`
        loop {
            while j < n && html[j].is_ascii_whitespace() {
                j += 1;
            }
            if j >= n || html[j] == b'>' || (html[j] == b'/' && html.get(j + 1) == Some(&b'>')) {
                break;
            }
            let ks = j;
            while j < n && !html[j].is_ascii_whitespace() && !matches!(html[j], b'=' | b'>' | b'/') {
                j += 1;
            }
            let key = String::from_utf8_lossy(&html[ks..j]).to_ascii_lowercase();
            while j < n && html[j].is_ascii_whitespace() {
                j += 1;
            }
            let mut val = None;
            if j < n && html[j] == b'=' {
                j += 1;
                while j < n && html[j].is_ascii_whitespace() {
                    j += 1;
                }
                if j < n && (html[j] == b'"' || html[j] == b'\'') {
                    let q = html[j];
                    let vs = j + 1;
                    j = html[vs..].iter().position(|&c| c == q).map_or(n, |p| vs + p);
                    val = Some(&html[vs..j.min(n)]);
                    j += 1;
                } else {
                    let vs = j;
                    while j < n && !html[j].is_ascii_whitespace() && html[j] != b'>' {
                        j += 1;
                    }
                    val = Some(&html[vs..j]);
                }
            } else if ks == j {
                j += 1; // a junk character: step forward, no error state
            }
            if let Some(v) = val {
                match key.as_str() {
                    "href" => href = Some(entities(&String::from_utf8_lossy(v))),
                    "rel" => rel = String::from_utf8_lossy(v).to_ascii_lowercase(),
                    "name" if name == "meta" => meta_name = String::from_utf8_lossy(v).to_ascii_lowercase(),
                    "content" if name == "meta" => content = String::from_utf8_lossy(v).to_ascii_lowercase(),
                    _ => {}
                }
            }
        }
        let nf = page_nofollow || noindex > 0 || rel.split_whitespace().any(|r| matches!(r, "nofollow" | "ugc" | "sponsored"));
        match name.as_str() {
            "a" | "area" => out.extend(href.map(|h| (h, nf))),
            "link" if rel.split_whitespace().any(|r| matches!(r, "canonical" | "alternate" | "next" | "prev")) => out.extend(href.map(|h| (h, nf))),
            "noindex" => noindex += 1,
            "meta" if meta_name == "robots" && content.contains("nofollow") => {
                page_nofollow = true;
                out.iter_mut().for_each(|x| x.1 = true);
            }
            "base" if base.is_none() => base = href,
            "script" | "style" | "noscript" | "template" => {
                let close = format!("</{name}");
                j = find_ci(html, j, close.as_bytes()).unwrap_or(n);
            }
            _ => {}
        }
        i = j + 1;
    }
    (base, out)
}

fn tag_name(h: &[u8], mut i: usize) -> (String, usize) {
    let s = i;
    while i < h.len() && h[i].is_ascii_alphanumeric() {
        i += 1;
    }
    (String::from_utf8_lossy(&h[s..i]).to_ascii_lowercase(), i)
}

fn find(h: &[u8], from: usize, pat: &[u8]) -> Option<usize> {
    h.get(from..)?.windows(pat.len()).position(|w| w == pat).map(|p| from + p)
}

fn find_ci(h: &[u8], from: usize, pat: &[u8]) -> Option<usize> {
    h.get(from..)?.windows(pat.len()).position(|w| w.eq_ignore_ascii_case(pat)).map(|p| from + p)
}

/// The most common entities in URLs.
fn entities(s: &str) -> String {
    s.trim().replace("&amp;", "&").replace("&#38;", "&").replace("&#x2F;", "/").replace("&#47;", "/")
}

/// Extensions that are not fetched as pages.
const SKIP_EXT: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "webp", "svg", "ico", "bmp", "tif", "tiff", "avif", "pdf", "zip", "gz", "rar", "7z", "tar", "exe", "msi",
    "dmg", "apk", "iso", "bin", "mp3", "mp4", "m4a", "m4v", "avi", "mov", "mkv", "webm", "wav", "ogg", "flac", "woff", "woff2", "ttf",
    "otf", "eot", "css", "js", "json", "xml", "rss", "atom", "csv", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
];

/// An absolute normalised URL or None: http(s) only, no fragment, no `utm_*`, no non-page extensions.
pub fn normalize(base: &Url, href: &str) -> Option<Url> {
    let low = href.trim_start().to_ascii_lowercase();
    if low.starts_with("javascript:") || low.starts_with("mailto:") || low.starts_with("tel:") || low.starts_with("data:") || low.starts_with('#') {
        return None;
    }
    let mut u = base.join(href.trim()).ok()?;
    if u.scheme() != "http" && u.scheme() != "https" {
        return None;
    }
    u.set_fragment(None);
    if u.query().is_some() {
        let kept: Vec<(String, String)> = u.query_pairs().filter(|(k, _)| !k.starts_with("utm_")).map(|(k, v)| (k.into_owned(), v.into_owned())).collect();
        if kept.is_empty() {
            u.set_query(None);
        } else {
            u.query_pairs_mut().clear().extend_pairs(kept);
        }
    }
    let last = u.path_segments().and_then(|mut s| s.next_back().map(str::to_ascii_lowercase)).unwrap_or_default();
    if let Some((_, ext)) = last.rsplit_once('.') {
        if SKIP_EXT.contains(&ext) {
            return None;
        }
    }
    Some(u)
}

/// Absolute outgoing links of a page fetched from `page`: deduplicated, first occurrence order;
/// a link is nofollow only if every occurrence is.
pub fn outgoing(page: &Url, html: &[u8]) -> Vec<(Url, bool)> {
    let (base, raw) = extract_rel(html);
    let base = base.and_then(|b| page.join(&b).ok()).unwrap_or_else(|| page.clone());
    let mut out: Vec<(Url, bool)> = Vec::new();
    let mut seen: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for (href, nf) in raw {
        let Some(u) = normalize(&base, &href) else { continue };
        match seen.get(u.as_str()) {
            Some(&k) => out[k].1 &= nf,
            None => {
                seen.insert(u.to_string(), out.len());
                out.push((u, nf));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchors_and_canonical() {
        let h = br#"<html><head><link rel="canonical" href="/c"><link rel=stylesheet href="/s.css"></head>
            <body><a href='/x?a=1&amp;b=2'>x</a><A HREF=/y>y</A><area href="z"></body>"#;
        let (_, l) = extract(h);
        assert_eq!(l, ["/c", "/x?a=1&b=2", "/y", "z"]);
    }

    #[test]
    fn nofollow_marks() {
        let h = br#"<a href="/a">a</a><a rel="nofollow noopener" href="/b">b</a><a rel=ugc href=/c>c</a>
<noindex><a href="/d">d</a></noindex><a href="/e">e</a>"#;
        let (_, v) = extract_rel(h);
        let nf: Vec<(&str, bool)> = v.iter().map(|(x, f)| (x.as_str(), *f)).collect();
        assert_eq!(nf, vec![("/a", false), ("/b", true), ("/c", true), ("/d", true), ("/e", false)]);
        // meta robots nofollow marks every link of the page, also those before the tag
        let (_, v) = extract_rel(br#"<a href="/x">x</a><meta name="robots" content="noindex, nofollow"><a href="/y">y</a>"#);
        assert!(v.iter().all(|x| x.1));
        // negative control: meta robots without nofollow marks nothing
        let (_, v) = extract_rel(br#"<meta name="robots" content="noindex"><a href="/z">z</a>"#);
        assert!(!v[0].1);
    }

    #[test]
    fn script_style_comment_skipped() {
        // negative control: links inside script/style/comments are not links
        let h = br#"<script>var s='<a href="/bad1">';</script><style>a{}</style><!-- <a href="/bad2"> --><a href="/good">g</a>"#;
        assert_eq!(extract(h).1, ["/good"]);
    }

    #[test]
    fn base_and_normalize() {
        let (b, l) = extract(br#"<base href="https://h.com/dir/"><a href="p.html#f">"#);
        let base = Url::parse(b.as_deref().unwrap()).unwrap();
        assert_eq!(normalize(&base, &l[0]).unwrap().as_str(), "https://h.com/dir/p.html");
        assert!(normalize(&base, "/img/a.JPG").is_none());
        assert!(normalize(&base, "mailto:a@b.c").is_none());
        assert_eq!(normalize(&base, "/q?utm_source=x&id=5").unwrap().as_str(), "https://h.com/q?id=5");
    }

    #[test]
    fn outgoing_dedups_in_order() {
        let page = Url::parse("https://h.com/a/").unwrap();
        let v = outgoing(&page, br##"<a href="b" rel=nofollow>1</a><a href="/c">2</a><a href="b">3</a><a href="#x">4</a>"##);
        let got: Vec<(&str, bool)> = v.iter().map(|(u, f)| (u.as_str(), *f)).collect();
        assert_eq!(got, vec![("https://h.com/a/b", false), ("https://h.com/c", false)]);
    }

    #[test]
    fn garbage_does_not_panic() {
        for h in [&b"<"[..], b"<a", b"<a href=", b"<a href=\"", b"<<<>>>", b"<script>", b"<!--"] {
            let _ = extract(h);
        }
    }
}
