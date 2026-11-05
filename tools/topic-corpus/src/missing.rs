//! Stage 2: more papers. The works a paper's neighbourhood cites that the library lacks are looked
//! up in the local OpenAlex snapshot (the library's `papers openalex lookup`, no network), ranked by
//! citations, given a legal open copy (ACL Anthology PDF, `arxiv.org/pdf/<id>`, or OpenAlex's best
//! open-access PDF on a host that is not a shadow library) and written as a new request run
//! directory for the library's own stream driver: one manifest chunk in the `tools/sources` format,
//! the list-only works (no legal copy: nothing is fetched), the full ranking, and a `run.sh` with the
//! robots.txt gate, host skip and PDF drop. Nothing is started here; the driver keeps its own pacing
//! (one connection per host, its delays and Crawl-delay), its disk guards and its claims.
use crate::config::Downloads;
use crate::graph::Neighbourhood;
use crate::library::{acl_of_doi, bare_doi, title_key, Library, RefEntry};
use crate::licence::{self, Class};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

pub const SCHEMA: &str = "mova-topic-request-v1";

/// Hosts that are never a source (shadow libraries and their mirrors).
pub const SHADOW: [&str; 14] = ["sci-hub", "scihub", "libgen", "library.lol", "z-lib", "zlibrary", "z-library", "annas-archive", "booksc", "1lib", "b-ok", "pdfdrive", "bookfi", "libgen.rs"];

/// Free space the raw root must keep (GB), as the stream driver's guard.
pub const MIN_FREE_GB: u64 = 300;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Candidate {
    /// `acl:ID`, `arxiv:ID` or `doi:DOI`.
    pub key: String,
    pub title: String,
    pub year: Option<i64>,
    pub first_author: String,
    /// How many works of the neighbourhood cite it (the paper counts once).
    pub cited_in_neighbourhood: usize,
    /// The paper itself cites it.
    pub cited_by_paper: bool,
    /// OpenAlex citations in the local snapshot (0 when not found there).
    pub cited_by: i64,
    pub openalex: Option<String>,
    /// `acl`, `arxiv`, `oa`, `list_only` (no legal open copy), `in_library` (found by OpenAlex id).
    pub route: String,
    pub url: Option<String>,
    pub licence: String,
    pub licence_url: Option<String>,
    pub why: String,
}

/// The download key of an entry: `acl:ID`, `arxiv:ID` or `doi:DOI`.
pub fn key_of(e: &RefEntry) -> Option<String> {
    if let Some(a) = e.acl.as_deref().filter(|a| !a.is_empty()) {
        return Some(format!("acl:{a}"));
    }
    if let Some(a) = e.doi.as_deref().and_then(acl_of_doi) {
        return Some(format!("acl:{a}"));
    }
    if let Some(a) = e.arxiv.as_deref().filter(|a| !a.is_empty()) {
        return Some(format!("arxiv:{}", a.to_lowercase()));
    }
    let d = e.doi.as_deref().map(bare_doi).filter(|d| d.starts_with("10."))?;
    if let Some(a) = d.strip_prefix("10.48550/arxiv.") {
        return Some(format!("arxiv:{a}"));
    }
    Some(format!("doi:{d}"))
}

pub fn shadow(url: &str) -> bool {
    let host = url.split("://").nth(1).unwrap_or(url).split('/').next().unwrap_or("").to_lowercase();
    SHADOW.iter().any(|s| host.contains(s))
}

/// The lookup key of an entry without an id: its normalized title (at least four words).
pub fn title_lookup_key(e: &RefEntry) -> Option<String> {
    let t = title_key(e.title.as_deref()?);
    (t.split(' ').count() >= 4).then(|| format!("title:{t}"))
}

/// The external works of a neighbourhood: entries in the paper's bibliography and in the
/// bibliographies of its library references and similar works that the library does not resolve,
/// keyed by their id, else by their normalized title (looked up in the snapshot by title).
pub fn external(lib: &Library, g: &Neighbourhood) -> Result<BTreeMap<String, (RefEntry, usize, bool)>> {
    let mut out: BTreeMap<String, (RefEntry, usize, bool)> = BTreeMap::new();
    let mut add = |e: &RefEntry, by_paper: bool, seen: &mut BTreeSet<String>| {
        if lib.resolve(e).is_some() {
            return;
        }
        let Some(k) = key_of(e).or_else(|| title_lookup_key(e)) else { return };
        if !seen.insert(k.clone()) {
            return;
        }
        let x = out.entry(k).or_insert_with(|| (e.clone(), 0, false));
        x.1 += 1;
        x.2 |= by_paper;
        if x.0.title.is_none() {
            x.0.title = e.title.clone();
        }
    };
    let mut seen = BTreeSet::new();
    for r in &g.references {
        add(&r.entry, true, &mut seen);
    }
    let mut members: Vec<&str> = g.references.iter().filter_map(|r| r.work.as_ref()).map(|w| w.id.as_str()).collect();
    members.extend(g.similar.iter().map(|n| n.id.as_str()));
    let members: BTreeSet<&str> = members.into_iter().collect();
    for id in members {
        let Some(w) = lib.get(id) else { continue };
        let mut seen = BTreeSet::new();
        for e in lib.bibliography(w)? {
            add(&e, false, &mut seen);
        }
    }
    Ok(out)
}

/// OpenAlex records of the keys, read from the local snapshot by the library's `papers` binary.
pub fn lookup(papers_bin: &Path, keys: &[String]) -> Result<Vec<Value>> {
    let mut args: Vec<String> = vec!["openalex".into(), "lookup".into()];
    for k in keys {
        if let Some(d) = k.strip_prefix("doi:") {
            args.push(format!("doi:{d}"));
        } else if let Some(a) = k.strip_prefix("arxiv:") {
            args.push(format!("arxiv:{a}"));
        } else if let Some(a) = k.strip_prefix("acl:") {
            args.push(format!("doi:10.18653/v1/{}", a.to_lowercase()));
        } else if let Some(t) = k.strip_prefix("title:") {
            args.push(format!("title:{t}"));
        }
    }
    if args.len() == 2 {
        return Ok(vec![]);
    }
    let out = std::process::Command::new(papers_bin).args(&args).output().with_context(|| format!("run {}", papers_bin.display()))?;
    if !out.status.success() {
        bail!("{} openalex lookup failed: {}", papers_bin.display(), String::from_utf8_lossy(&out.stderr).trim());
    }
    Ok(String::from_utf8_lossy(&out.stdout).lines().filter(|l| l.trim_start().starts_with('{')).filter_map(|l| serde_json::from_str(l).ok()).collect())
}

fn oa_key(v: &Value) -> Vec<String> {
    let mut k = vec![];
    if let Some(t) = v["title"].as_str() {
        k.push(format!("title:{}", title_key(t)));
    }
    if let Some(d) = v["ids"]["doi"].as_str().or(v["doi"].as_str()) {
        let d = bare_doi(d);
        if let Some(a) = acl_of_doi(&d) {
            k.push(format!("acl:{a}"));
        }
        if let Some(a) = d.strip_prefix("10.48550/arxiv.") {
            k.push(format!("arxiv:{a}"));
        }
        k.push(format!("doi:{d}"));
    }
    if let Some(a) = v["ids"]["arxiv"].as_str() {
        k.push(format!("arxiv:{}", a.to_lowercase()));
    }
    k
}

fn licence_name(oa: &str) -> String {
    match oa {
        "cc-by" => "CC BY".into(),
        "cc0" => "CC0".into(),
        "public-domain" => "Public domain".into(),
        x if x.starts_with("cc-") => format!("CC {}", x[3..].to_uppercase()),
        x => x.to_string(),
    }
}

fn licence_url(name: &str) -> Option<String> {
    let l = name.to_lowercase();
    let code = ["by-nc-nd", "by-nc-sa", "by-nc", "by-nd", "by-sa", "by"].into_iter().find(|c| l.replace(' ', "-").contains(&format!("cc-{c}")))?;
    Some(format!("https://creativecommons.org/licenses/{code}/4.0/"))
}

/// Rank and route the external works.
pub fn candidates(lib: &Library, g: &Neighbourhood, papers_bin: &Path) -> Result<Vec<Candidate>> {
    candidates_with(lib, g, papers_bin, &BTreeMap::new())
}

/// Rank and route the external works and `extra` keys (e.g. the sources a consultant's draft cites).
pub fn candidates_with(lib: &Library, g: &Neighbourhood, papers_bin: &Path, extra: &BTreeMap<String, RefEntry>) -> Result<Vec<Candidate>> {
    let mut ext = external(lib, g)?;
    for (k, e) in extra {
        if lib.resolve(e).is_none() {
            ext.entry(k.clone()).or_insert_with(|| (e.clone(), 0, false));
        }
    }
    let keys: Vec<String> = ext.keys().cloned().collect();
    let mut by_key: BTreeMap<String, Value> = BTreeMap::new();
    for v in lookup(papers_bin, &keys)? {
        for k in oa_key(&v) {
            // The most cited record of a key wins (a preprint and its publication may share a key).
            let c = v["cited_by_count"].as_i64().unwrap_or(0);
            if by_key.get(&k).is_none_or(|o| o["cited_by_count"].as_i64().unwrap_or(0) < c) {
                by_key.insert(k, v.clone());
            }
        }
    }
    let have_oa: BTreeSet<&str> = lib.works.iter().filter_map(|w| w.openalex.as_deref()).collect();
    let mut out = vec![];
    let mut merged: BTreeMap<String, (RefEntry, usize, bool)> = BTreeMap::new();
    for (k, (e, n, by_paper)) in ext {
        // A title-keyed work found in the snapshot is routed by its ids (arXiv, ACL DOI, DOI).
        let k = match (k.strip_prefix("title:"), by_key.get(&k)) {
            (Some(_), Some(v)) => {
                let year_ok = match (e.year, v["publication_year"].as_i64()) {
                    (Some(a), Some(b)) => (a - b).abs() <= 1,
                    _ => true,
                };
                let ids = oa_key(v);
                let id = ids.iter().find(|x| x.starts_with("acl:")).or_else(|| ids.iter().find(|x| x.starts_with("arxiv:"))).or_else(|| ids.iter().find(|x| x.starts_with("doi:"))).cloned();
                match id {
                    Some(id) if year_ok => {
                        if by_key.get(&id).is_none() {
                            by_key.insert(id.clone(), v.clone());
                        }
                        id
                    }
                    _ => k,
                }
            }
            _ => k,
        };
        let x = merged.entry(k).or_insert_with(|| (e.clone(), 0, false));
        x.1 += n;
        x.2 |= by_paper;
    }
    for (k, (e, n, by_paper)) in merged {
        let v = by_key.get(&k).cloned().unwrap_or(Value::Null);
        let v = if k.starts_with("title:") {
            // A title match needs the year within one.
            match (e.year, v["publication_year"].as_i64()) {
                (Some(a), Some(b)) if (a - b).abs() > 1 => Value::Null,
                _ => v,
            }
        } else {
            v
        };
        let year = v["publication_year"].as_i64().or(e.year);
        let title = v["title"].as_str().map(str::to_string).or(e.title.clone()).unwrap_or_else(|| e.raw.chars().take(160).collect());
        let first = v["authorships"][0]["author"]["display_name"].as_str().map(str::to_string).or(e.author.clone()).unwrap_or_default();
        let openalex = v["id"].as_str().map(|s| s.trim_start_matches("https://openalex.org/").to_string());
        let mut c = Candidate { key: k.clone(), title, year, first_author: first, cited_in_neighbourhood: n, cited_by_paper: by_paper, cited_by: v["cited_by_count"].as_i64().unwrap_or(0), openalex: openalex.clone(), route: "list_only".into(), url: None, licence: String::new(), licence_url: None, why: String::new() };
        let oa_lic = v["best_oa_location"]["license"].as_str().unwrap_or("");
        let oa_pdf = v["best_oa_location"]["pdf_url"].as_str().unwrap_or("");
        if openalex.as_deref().is_some_and(|o| have_oa.contains(o) || have_oa.contains(format!("https://openalex.org/{o}").as_str())) {
            c.route = "in_library".into();
            c.why = "the library has this OpenAlex work under another key".into();
        } else if let Some(a) = k.strip_prefix("acl:") {
            c.route = "acl".into();
            c.url = Some(format!("https://aclanthology.org/{a}.pdf"));
            // The Anthology's licence: CC BY 4.0 from 2016, CC BY-NC-SA 3.0 before.
            if year.unwrap_or(0) >= 2016 {
                c.licence = "CC BY 4.0 (ACL Anthology)".into();
                c.licence_url = Some("https://creativecommons.org/licenses/by/4.0/".into());
            } else {
                c.licence = "CC BY-NC-SA 3.0 (ACL Anthology, before 2016)".into();
                c.licence_url = Some("https://creativecommons.org/licenses/by-nc-sa/3.0/".into());
            }
            c.why = "ACL Anthology PDF".into();
        } else if let Some(a) = k.strip_prefix("arxiv:") {
            c.route = "arxiv".into();
            c.url = Some(format!("https://arxiv.org/pdf/{a}"));
            if oa_lic.starts_with("cc") {
                c.licence = format!("{} (arXiv)", licence_name(oa_lic));
                c.licence_url = licence_url(&c.licence);
            } else {
                c.licence = "arXiv — non-exclusive license to distribute".into();
                c.licence_url = Some("http://arxiv.org/licenses/nonexclusive-distrib/1.0/".into());
            }
            c.why = "arXiv PDF (never /e-print)".into();
        } else if !oa_pdf.is_empty() && (oa_pdf.starts_with("https://") || oa_pdf.starts_with("http://")) && !shadow(oa_pdf) {
            c.route = "oa".into();
            c.url = Some(oa_pdf.to_string());
            c.licence = if oa_lic.is_empty() { "free to read, no licence stated (OpenAlex best open-access location)".into() } else { format!("{} (OpenAlex best open-access location)", licence_name(oa_lic)) };
            c.licence_url = licence_url(&c.licence);
            c.why = "OpenAlex best open-access PDF".into();
        } else if shadow(oa_pdf) {
            c.why = "only a shadow-library copy: never fetched".into();
        } else if v.is_null() {
            c.why = "not in the local OpenAlex snapshot and no arXiv/ACL copy".into();
        } else {
            c.why = "no open PDF location".into();
        }
        out.push(c);
    }
    out.sort_by(|a, b| b.cited_by.cmp(&a.cited_by).then(b.cited_in_neighbourhood.cmp(&a.cited_in_neighbourhood)).then(b.cited_by_paper.cmp(&a.cited_by_paper)).then_with(|| a.key.cmp(&b.key)));
    Ok(out)
}

fn ascii_words(s: &str) -> Vec<String> {
    title_key(s).split(' ').filter(|w| !w.is_empty() && w.is_ascii()).map(str::to_string).collect()
}

/// A new slug `tc-<year>-<surname>-<title words>`, unique against the library ids, the raw root and
/// the slugs already taken.
pub fn slug(c: &Candidate, lib: &Library, raw_root: &Path, taken: &mut BTreeSet<String>) -> String {
    let sur = ascii_words(&c.first_author).last().cloned().unwrap_or_else(|| "anon".into());
    let words: Vec<String> = crate::library::words(&c.title).into_iter().filter(|w| w.is_ascii()).take(4).collect();
    let base = format!("tc-{}-{}-{}", c.year.map(|y| y.to_string()).unwrap_or_else(|| "nd".into()), sur, words.join("-")).trim_end_matches('-').to_string();
    let mut s = base.clone();
    let mut k = 2;
    while taken.contains(&s) || lib.get(&s).is_some() || raw_root.join(&s).exists() {
        s = format!("{base}-{k}");
        k += 1;
    }
    taken.insert(s.clone());
    s
}

/// The manifest row (`tools/sources` format, as the topoa stream writes it).
pub fn manifest_row(c: &Candidate, slug: &str, paper: &str) -> Value {
    let class = licence::classify(&c.licence);
    let status = if matches!(class, Class::By | Class::PublicDomain | Class::ShareAlike) || c.route == "arxiv" { "open" } else { "open-nc" };
    let mut r = json!({
        "authors": c.first_author, "files": [{"md5": null, "save_as": "paper.pdf", "sha1": null, "size": null, "url": c.url}],
        "format": "pdf", "kind": "paper", "langs": [], "license": c.licence, "license_url": c.licence_url, "name": c.title,
        "notes": format!("topic-corpus request for {paper} ({}; cited_by {} in the local OpenAlex snapshot; cited by {} work(s) of the neighbourhood; route {})", c.openalex.as_deref().unwrap_or("not in the snapshot"), c.cited_by, c.cited_in_neighbourhood, c.route),
        "priority": 2, "slug": slug, "status": status, "topics": ["topic-corpus", format!("topic-corpus-{paper}")], "year": c.year,
    });
    let src = match (c.key.split_once(':'), c.route.as_str()) {
        (Some(("acl", a)), _) => format!("https://aclanthology.org/{a}/"),
        (Some(("arxiv", a)), _) => format!("https://arxiv.org/abs/{a}"),
        (Some(("doi", d)), _) => format!("https://doi.org/{d}"),
        _ => String::new(),
    };
    r["source_page"] = json!(src);
    match c.key.split_once(':') {
        Some(("doi", d)) => r["doi"] = json!(d),
        Some(("arxiv", a)) => {
            r["arxiv"] = json!(a);
            r["doi"] = json!(format!("10.48550/arxiv.{a}"));
        }
        Some(("acl", a)) => r["acl"] = json!(a),
        _ => {}
    }
    r
}

pub fn free_gb(p: &Path) -> Result<u64> {
    use std::os::unix::ffi::OsStrExt;
    let c = std::ffi::CString::new(p.as_os_str().as_bytes())?;
    let mut s: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(c.as_ptr(), &mut s) } != 0 {
        bail!("statvfs {}: {}", p.display(), std::io::Error::last_os_error());
    }
    Ok(s.f_bavail as u64 * s.f_frsize as u64 / (1 << 30))
}

/// Today's date (UTC) as `YYYY-MM-DD`.
pub fn today() -> String {
    let days = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) / 86_400) as i64;
    // Civil from days (H. Hinnant's algorithm).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Write a request run directory (it must be new). Returns its path and the number of works queued.
pub fn write_request(lib: &Library, g: &Neighbourhood, d: &Downloads, cands: &[Candidate], date: &str, min_free_gb: u64) -> Result<(PathBuf, usize)> {
    let free = free_gb(&d.raw_root)?;
    if free < min_free_gb {
        bail!("the raw root {} has {free} GB free, below the {min_free_gb} GB guard: no request written", d.raw_root.display());
    }
    let dir = d.runs_root.join(format!("topic-{}-{date}", g.paper.id));
    if dir.exists() {
        bail!("{} exists: a request directory is never rewritten (a running driver reads it)", dir.display());
    }
    std::fs::create_dir_all(&dir)?;
    let mut taken = BTreeSet::new();
    let mut chunk = String::new();
    let mut list = String::from("key\tcited_by\ttitle\tyear\twhy\n");
    let mut n = 0;
    let mut ranking = vec![];
    for c in cands {
        let mut v = serde_json::to_value(c)?;
        if c.url.is_some() && n < d.max_requests {
            let s = slug(c, lib, &d.raw_root, &mut taken);
            chunk.push_str(&manifest_row(c, &s, &g.paper.id).to_string());
            chunk.push('\n');
            v["slug"] = json!(s);
            v["queued"] = json!(true);
            n += 1;
        } else {
            v["queued"] = json!(false);
            if c.url.is_none() && c.route == "list_only" {
                list.push_str(&format!("{}\t{}\t{}\t{}\t{}\n", c.key, c.cited_by, c.title.replace(['\t', '\n'], " "), c.year.map(|y| y.to_string()).unwrap_or_default(), c.why));
            }
        }
        ranking.push(v);
    }
    std::fs::write(dir.join("chunk-topic-00.jsonl"), chunk)?;
    std::fs::write(dir.join("listonly.tsv"), list)?;
    std::fs::write(dir.join("request.json"), serde_json::to_vec_pretty(&json!({"schema": SCHEMA, "paper": g.paper.id, "date": date, "queued": n, "max_requests": d.max_requests, "free_gb_at_request": free, "candidates": ranking}))?)?;
    std::fs::copy(&d.driver, dir.join(".driver.sh")).with_context(|| format!("copy {}", d.driver.display()))?;
    std::fs::copy(&d.claim, dir.join(".claim.py")).with_context(|| format!("copy {}", d.claim.display()))?;
    let run = format!(
        "#!/usr/bin/env bash\n# topic-corpus request for {p}: the most-cited open copies its neighbourhood cites that the library lacks\n# (mova-topic-corpus missing). robots.txt gate, host skip, verified PDF deletion after import.\nexport R={r} TAG=topic-corpus DESC=\"topic-corpus works for {p}\"\nexport CHUNK_RE='^chunk-topic-[0-9]+' ROBOTS=1 HOST_SKIP=1 PDFDROP=1\nexec bash \"$R/.driver.sh\"\n",
        p = g.paper.id,
        r = dir.display()
    );
    std::fs::write(dir.join("run.sh"), run)?;
    Ok((dir, n))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Limits;
    use crate::graph;
    use crate::library::tests::{fixture, tmp};
    use std::os::unix::fs::PermissionsExt;

    /// A fake `papers` binary answering `openalex lookup` with fixed snapshot records.
    pub fn fake_papers(dir: &Path, shadow_copy: bool) -> PathBuf {
        let a = json!({"id":"https://openalex.org/W100","doi":"https://doi.org/10.3115/v1/w14-1701","ids":{"doi":"https://doi.org/10.3115/v1/w14-1701"},"title":"The CoNLL-2014 Shared Task on Grammatical Error Correction","publication_year":2014,"cited_by_count":700,
            "best_oa_location":{"pdf_url": if shadow_copy {"https://sci-hub.example/10.3115/v1/w14-1701"} else {"https://repository.example.org/w14-1701.pdf"},"license":"cc-by-nc-sa"},"authorships":[{"author":{"display_name":"Hwee Tou Ng"}}]});
        let b = json!({"id":"https://openalex.org/W200","ids":{"doi":"https://doi.org/10.48550/arxiv.1101.0001","arxiv":"1101.0001"},"title":"A New Dataset and Method for Automatically Grading ESOL Texts","publication_year":2011,"cited_by_count":900,"best_oa_location":null,"authorships":[{"author":{"display_name":"Helen Yannakoudakis"}}]});
        let t = json!({"id":"https://openalex.org/W300","ids":{"doi":"https://doi.org/10.48550/arxiv.1905.00001","arxiv":"1905.00001"},"title":"Neural Models of Learner Text Errors","publication_year":2019,"cited_by_count":50,"best_oa_location":{"pdf_url":"https://arxiv.org/pdf/1905.00001","license":"cc-by"},"authorships":[{"author":{"display_name":"Zoe Zed"}}]});
        let p = dir.join("papers");
        std::fs::write(&p, format!("#!/bin/sh\n[ \"$1 $2\" = \"openalex lookup\" ] || exit 2\ncat <<'EOF'\n{a}\n{b}\n{t}\nEOF\necho 'scan 0.1 s' >&2\n")).unwrap();
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
        p
    }

    #[test]
    fn ranks_routes_and_writes_a_request() {
        let d = tmp("missing");
        let lib = Library::load(&fixture(&d)).unwrap();
        let g = graph::build(&lib, "pa-2017-error-types", &Limits::default()).unwrap();
        let bin = fake_papers(&d, false);
        let c = candidates(&lib, &g, &bin).unwrap();
        // The two external works of the fixture, the most cited first.
        assert_eq!(c.iter().map(|c| c.key.as_str()).collect::<Vec<_>>(), vec!["arxiv:1101.0001", "doi:10.3115/v1/w14-1701", "arxiv:1905.00001"]);
        // A reference without an id, found in the snapshot by its title, is routed by its arXiv id.
        assert_eq!((c[2].route.as_str(), c[2].licence.as_str()), ("arxiv", "CC BY (arXiv)"));
        assert_eq!((c[0].route.as_str(), c[0].url.as_deref()), ("arxiv", Some("https://arxiv.org/pdf/1101.0001")));
        assert_eq!(c[1].route, "oa");
        assert_eq!(licence::classify(&c[1].licence), Class::NonCommercial);
        let runs = d.join("runs");
        let raw = d.join("raw");
        std::fs::create_dir_all(&runs).unwrap();
        std::fs::create_dir_all(&raw).unwrap();
        std::fs::write(d.join("driver.sh"), "#!/bin/sh\n").unwrap();
        std::fs::write(d.join("claim.py"), "").unwrap();
        let dl = Downloads { papers_bin: bin.clone(), runs_root: runs.clone(), raw_root: raw.clone(), driver: d.join("driver.sh"), claim: d.join("claim.py"), max_requests: 1 };
        let (dir, n) = write_request(&lib, &g, &dl, &c, "2026-11-04", 0).unwrap();
        assert_eq!(n, 1);
        let chunk = std::fs::read_to_string(dir.join("chunk-topic-00.jsonl")).unwrap();
        let row: Value = serde_json::from_str(chunk.lines().next().unwrap()).unwrap();
        assert_eq!(row["slug"], "tc-2011-yannakoudakis-dataset-method-automatically-grading");
        assert_eq!(row["status"], "open");
        assert_eq!(row["files"][0]["url"], "https://arxiv.org/pdf/1101.0001");
        assert!(std::fs::read_to_string(dir.join("run.sh")).unwrap().contains("ROBOTS=1 HOST_SKIP=1 PDFDROP=1"));
        // A request directory is never rewritten.
        assert!(write_request(&lib, &g, &dl, &c, "2026-11-04", 0).is_err());
        // The disk guard refuses before anything is written.
        assert!(write_request(&lib, &g, &dl, &c, "2026-11-05", u64::MAX).unwrap_err().to_string().contains("guard"));
        assert!(!runs.join("topic-pa-2017-error-types-2026-11-05").exists());
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn a_shadow_library_copy_is_never_queued() {
        let d = tmp("shadow");
        let lib = Library::load(&fixture(&d)).unwrap();
        let g = graph::build(&lib, "pa-2017-error-types", &Limits::default()).unwrap();
        let c = candidates(&lib, &g, &fake_papers(&d, true)).unwrap();
        let w = c.iter().find(|c| c.key.starts_with("doi:")).unwrap();
        assert_eq!((w.route.as_str(), w.url.as_deref()), ("list_only", None));
        assert!(w.why.contains("shadow"));
        assert!(shadow("https://libgen.rs/x.pdf") && !shadow("https://aclanthology.org/P17-1074.pdf"));
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn date_is_civil() {
        let t = today();
        assert_eq!(t.len(), 10);
        assert!(t.as_str() >= "2026-01-01");
    }
}
