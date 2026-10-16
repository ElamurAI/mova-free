//! sources — manifests of open sources (JSONL) → layout in raw/, curl queues per host, verification.
//!
//!   sources plan   <raw-root> <queues-dir> <manifest.jsonl>...
//!   sources verify <raw-root> <manifest.jsonl>...
//!   sources bib    <anthology.bib> <rules.tsv> <out-dir>   (ACL Anthology papers by topic → manifests)
//!   sources bib-ids <anthology.bib> <ids.txt> <out.jsonl>  (ACL Anthology papers from an id list [\t topic [\t priority]] → manifest)
//!
//! Manifest row: {"slug","name","langs","kind","year","status","license","license_url",
//!   "source_page","format","files":[{"url","size","md5","sha1"}],"priority","notes"}.
//!
//! plan: for each resource writes `<raw-root>/<slug>/SOURCES.md` (provenance from the manifest),
//!   and into `<queues-dir>/curl-<host>.cfg` url/output pairs for files not yet present
//!   (or whose size differs from the manifest). One queue per host: sequential within a host,
//!   parallel across hosts. `wiktionary-derived` and non-open statuses are not planned
//!   (only raw sources, no pre-parsed derivatives). A duplicate slug, URL or file name within a slug is an error.
//! verify: every file exists, size and md5/sha1 match the manifest, signature matches the extension
//!   (pdf, djvu, zip, gz, bz2, xz, zst). At least one failure → rc=1.

use sources::bib;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use anyhow::{bail, Context, Result};
use md5::Md5;
use serde::Deserialize;
use serde_json::Value;
use sha1::{Digest, Sha1};

const OPEN: [&str; 4] = ["public-domain", "open", "open-nc", "official-free"];

#[derive(Deserialize)]
struct Resource {
    slug: String,
    name: String,
    #[serde(default, deserialize_with = "strings")]
    langs: Vec<String>,
    #[serde(default, deserialize_with = "text")]
    kind: String,
    #[serde(default)]
    year: Value,
    #[serde(default, deserialize_with = "text")]
    status: String,
    #[serde(default, deserialize_with = "text")]
    license: String,
    #[serde(default, deserialize_with = "text")]
    license_url: String,
    #[serde(default, deserialize_with = "text")]
    source_page: String,
    #[serde(default, deserialize_with = "text")]
    format: String,
    #[serde(default)]
    files: Vec<FileRef>,
    #[serde(default)]
    priority: Value,
    #[serde(default, deserialize_with = "text")]
    notes: String,
}

#[derive(Deserialize)]
struct FileRef {
    url: String,
    /// Local name when it cannot be derived from the URL (`…/content`); the field is `save_as` because agents
    /// write `name` as a file caption.
    #[serde(default, rename = "save_as")]
    name: Option<String>,
    #[serde(default)]
    size: Value,
    #[serde(default)]
    md5: Option<String>,
    #[serde(default)]
    sha1: Option<String>,
}

/// A resource that passed the filter, with file names and the source manifest.
struct Planned {
    res: Resource,
    manifest: String,
    names: Vec<String>,
    /// Names by the automatic rule without `save_as` — for migrating already downloaded files.
    auto: Vec<String>,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("plan") if args.len() >= 4 => plan(Path::new(&args[1]), Path::new(&args[2]), &args[3..]),
        Some("verify") if args.len() >= 3 => verify(Path::new(&args[1]), &args[2..]),
        Some("merge") if args.len() >= 3 => merge(Path::new(&args[1]), &args[2..]),
        Some("bib") if args.len() == 4 => bib::run(Path::new(&args[1]), Path::new(&args[2]), Path::new(&args[3])),
        Some("bib-ids") if args.len() == 4 => bib::run_ids(Path::new(&args[1]), Path::new(&args[2]), Path::new(&args[3])),
        _ => bail!(
            "usage: sources plan <raw-root> <queues-dir> <manifest>... | sources verify <raw-root> <manifest>... | sources merge <out.jsonl> <manifest>... | sources bib <anthology.bib> <rules.tsv> <out-dir> | sources bib-ids <anthology.bib> <ids.txt> <out.jsonl>"
        ),
    }
}

/// Reads manifests, filters out non-open resources, checks uniqueness of slugs/URLs/names.
fn load(manifests: &[String]) -> Result<Vec<Planned>> {
    let mut out = Vec::new();
    let (mut slugs, mut urls) = (HashSet::new(), HashMap::new());
    let mut errors = Vec::new();
    for m in manifests {
        let reader = BufReader::new(File::open(m).with_context(|| format!("manifest {m}"))?);
        for (n, line) in reader.lines().enumerate() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let res: Resource =
                serde_json::from_str(&line).with_context(|| format!("{m}:{}: not a resource JSON", n + 1))?;
            if res.kind == "wiktionary-derived" || !OPEN.contains(&res.status.as_str()) {
                println!("SKIP {} — kind={} status={}", res.slug, res.kind, res.status);
                continue;
            }
            if !slugs.insert(res.slug.clone()) {
                errors.push(format!("duplicate slug {} ({m}:{})", res.slug, n + 1));
            }
            if res.files.is_empty() {
                println!("MANUAL {} — no files in the manifest (license requires registration, broken host, etc.)", res.slug);
                continue;
            }
            for f in &res.files {
                if let Some(prev) = urls.insert(f.url.clone(), res.slug.clone()) {
                    errors.push(format!("duplicate URL {} in {prev} and {}", f.url, res.slug));
                }
            }
            let names = derive_names(&res, true);
            let auto = derive_names(&res, false);
            for (i, name) in names.iter().enumerate() {
                if name.is_empty() || name.contains('/') || name.starts_with('.') {
                    errors.push(format!("{}: cannot derive a file name from {}", res.slug, res.files[i].url));
                } else if names[..i].contains(name) {
                    errors.push(format!("{}: two files named {name}", res.slug));
                }
            }
            out.push(Planned { res, manifest: m.clone(), names, auto });
        }
    }
    if !errors.is_empty() {
        for e in &errors {
            eprintln!("ERR {e}");
        }
        bail!("{} errors in manifests — fix and retry", errors.len());
    }
    Ok(out)
}

fn plan(raw: &Path, queues: &Path, manifests: &[String]) -> Result<()> {
    let planned = load(manifests)?;
    fs::create_dir_all(queues)?;
    // files already stored under other slugs (a slug may have changed, e.g. after decoding LaTeX accents
    // in names): file name → path, only for names unique across all of raw
    let mut elsewhere: HashMap<String, Vec<std::path::PathBuf>> = HashMap::new();
    for d in fs::read_dir(raw).into_iter().flatten().flatten().filter(|e| e.path().is_dir()) {
        for f in fs::read_dir(d.path()).into_iter().flatten().flatten().filter(|e| e.path().is_file()) {
            let n = f.file_name().to_string_lossy().into_owned();
            if n != "SOURCES.md" {
                elsewhere.entry(n).or_default().push(f.path());
            }
        }
    }
    let mut by_host: BTreeMap<String, (String, usize, u64)> = BTreeMap::new();
    let (mut have, mut unknown_size, mut total) = (0usize, 0usize, 0u64);
    let mut by_status: BTreeMap<String, (usize, u64)> = BTreeMap::new();
    for p in &planned {
        let dir = raw.join(&p.res.slug);
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("SOURCES.md"), sources_md(p))?;
        for (f, name) in p.res.files.iter().zip(&p.names) {
            let size = parse_size(&f.size);
            let dest = dir.join(name);
            let entry = by_status.entry(p.res.status.clone()).or_default();
            entry.0 += 1;
            entry.1 += size.unwrap_or(0);
            total += size.unwrap_or(0);
            if size.is_none() {
                unknown_size += 1;
            }
            // name migration: a file downloaded under the old naming rule is renamed, not downloaded again
            if !dest.exists() {
                let i = p.names.iter().position(|n| n == name).unwrap_or(0);
                for old in [p.auto[i].clone(), file_name(&f.url), path_name(&f.url)] {
                    let old = dir.join(&old);
                    if old != dest && fs::metadata(&old).is_ok_and(|m| size.is_none_or(|s| s == m.len())) {
                        fs::rename(&old, &dest)?;
                        println!("RENAME {} → {}", old.display(), dest.display());
                        break;
                    }
                }
            }
            // slug migration: the same file (name unique across all of raw) lies in another slug's folder
            if !dest.exists() {
                if let Some([only]) = elsewhere.get(name).map(Vec::as_slice) {
                    if only.parent() != Some(dir.as_path()) && fs::metadata(only).is_ok_and(|m| size.is_none_or(|s| s == m.len())) {
                        fs::rename(only, &dest)?;
                        println!("MOVE {} → {}", only.display(), dest.display());
                        for extra in ["txt"] {
                            let side = only.with_extension(extra);
                            if side.is_file() {
                                fs::rename(&side, dest.with_extension(extra))?;
                            }
                        }
                    }
                }
            }
            if let Ok(meta) = fs::metadata(&dest) {
                if size.is_none_or(|s| s == meta.len()) {
                    have += 1;
                    continue;
                }
            }
            let q = by_host.entry(host(&f.url).to_string()).or_default();
            writeln!(q.0, "url = \"{}\"\noutput = \"{}\"", quote(&f.url), quote(&dest.to_string_lossy()))?;
            q.1 += 1;
            q.2 += size.unwrap_or(0);
        }
    }
    for old in fs::read_dir(queues)? {
        let old = old?.path();
        if old.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with("curl-") && n.ends_with(".cfg")) {
            fs::remove_file(old)?;
        }
    }
    for (h, (cfg, n, bytes)) in &by_host {
        fs::write(queues.join(format!("curl-{h}.cfg")), cfg)?;
        println!("QUEUE {h}: {n} files, {:.2} GB (known)", *bytes as f64 / 1e9);
    }
    for (s, (n, bytes)) in &by_status {
        println!("STATUS {s}: {n} files, {:.2} GB", *bytes as f64 / 1e9);
    }
    println!(
        "PLAN {} resources, {} files, {:.2} GB known; size unknown for {unknown_size}; already on disk {have}",
        planned.len(),
        planned.iter().map(|p| p.names.len()).sum::<usize>(),
        total as f64 / 1e9
    );
    Ok(())
}

fn verify(raw: &Path, manifests: &[String]) -> Result<()> {
    let planned = load(manifests)?;
    let (mut ok, mut bad) = (0usize, 0usize);
    for p in &planned {
        for (f, name) in p.res.files.iter().zip(&p.names) {
            let path = raw.join(&p.res.slug).join(name);
            match check(&path, f) {
                Ok(()) => ok += 1,
                Err(e) => {
                    bad += 1;
                    println!("FAIL {}/{name}: {e}", p.res.slug);
                }
            }
        }
    }
    println!("VERIFY ok={ok} fail={bad}");
    if bad > 0 {
        std::process::exit(1);
    }
    Ok(())
}

fn check(path: &Path, f: &FileRef) -> Result<()> {
    let meta = fs::metadata(path).context("file missing")?;
    if let Some(s) = parse_size(&f.size) {
        if s != meta.len() {
            bail!("size {} ≠ {s} from the manifest", meta.len());
        }
    }
    let mut file = File::open(path)?;
    let (mut md5, mut sha1) = (Md5::new(), Sha1::new());
    let mut buf = vec![0u8; 1 << 20];
    let mut head = Vec::new();
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        if head.len() < 16 {
            head.extend_from_slice(&buf[..n.min(16)]);
        }
        md5.update(&buf[..n]);
        sha1.update(&buf[..n]);
    }
    // an "md5" with a hyphen (`…-4`) is an S3 multipart-upload ETag (OAPEN serves these), not the file's md5
    if let Some(want) = f.md5.as_deref().filter(|s| !s.is_empty() && !s.contains('-')) {
        let got = hex(&md5.finalize());
        if !got.eq_ignore_ascii_case(want) {
            bail!("md5 {got} ≠ {want}");
        }
    }
    if let Some(want) = f.sha1.as_deref().filter(|s| !s.is_empty()) {
        let got = hex(&sha1.finalize());
        if !got.eq_ignore_ascii_case(want) {
            bail!("sha1 {got} ≠ {want}");
        }
    }
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    let magic: &[u8] = match ext.as_str() {
        "pdf" => b"%PDF",
        "djvu" => b"AT&TFORM",
        "zip" => b"PK\x03\x04",
        "gz" | "tgz" => b"\x1f\x8b",
        "bz2" => b"BZh",
        "xz" => b"\xfd7zXZ\x00",
        "zst" => b"\x28\xb5\x2f\xfd",
        _ => b"",
    };
    if !head.starts_with(magic) {
        bail!("signature is not {ext} (first bytes {:?})", String::from_utf8_lossy(&head[..head.len().min(8)]));
    }
    Ok(())
}

fn sources_md(p: &Planned) -> String {
    let r = &p.res;
    let year = match &r.year {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        v => v.to_string(),
    };
    let mut s = format!("# {}\n\n| Field | Value |\n|---|---|\n", r.name);
    for (k, v) in [
        ("slug", format!("`{}`", r.slug)),
        ("Languages", r.langs.join(", ")),
        ("Type", r.kind.clone()),
        ("Year", year),
        ("Status", r.status.clone()),
        ("License", r.license.clone()),
        ("License text", r.license_url.clone()),
        ("Source page", r.source_page.clone()),
        ("Format", r.format.clone()),
        ("Priority", r.priority.to_string()),
    ] {
        let _ = writeln!(s, "| {k} | {} |", v.replace('|', "\\|"));
    }
    if !r.notes.is_empty() {
        let _ = write!(s, "\n**Notes:** {}\n", r.notes);
    }
    s.push_str("\n## Files\n\n| File | Size, B | md5 | sha1 | URL |\n|---|---:|---|---|---|\n");
    for (f, name) in r.files.iter().zip(&p.names) {
        let _ = writeln!(
            s,
            "| `{name}` | {} | {} | {} | {} |",
            parse_size(&f.size).map_or("?".into(), |v| v.to_string()),
            f.md5.as_deref().unwrap_or(""),
            f.sha1.as_deref().unwrap_or(""),
            f.url
        );
    }
    let _ = write!(
        s,
        "\n_Generated by `sources plan` from `{}`. Verification: `sources verify`._\n",
        Path::new(&p.manifest).file_name().unwrap_or_default().to_string_lossy()
    );
    s
}

fn parse_size(v: &Value) -> Option<u64> {
    match v {
        Value::Number(n) => n.as_u64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

fn host(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    rest.split(['/', '?', '#']).next().unwrap_or(rest)
}

/// Last segment of the URL path, percent-decoded.
fn file_name(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let path = rest.split(['?', '#']).next().unwrap_or(rest);
    let seg = path.rsplit('/').next().unwrap_or("");
    let bytes = seg.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let pair = std::str::from_utf8(&bytes[i + 1..i + 3]).ok();
            if let Some(b) = pair.and_then(|p| u8::from_str_radix(p, 16).ok()) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn quote(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_hosts() {
        assert_eq!(file_name("https://archive.org/download/x/Gr%C3%BCn%CC%88.pdf"), "Grün\u{308}.pdf");
        assert_eq!(file_name("https://h/a/b.zip?download=1"), "b.zip");
        assert_eq!(file_name("https://h/a/100%25.txt"), "100%.txt");
        assert_eq!(file_name("https://h/a/bad%2"), "bad%2");
        assert_eq!(host("https://zenodo.org/records/1/files/a.zip"), "zenodo.org");
    }
}

/// String manifest field: `null` and absence → empty (agents write both).
fn text<'de, D: serde::Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Null => String::new(),
        Value::String(s) => s,
        v => v.to_string(),
    })
}

/// Language list: an array of strings, a single string or `null`.
fn strings<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Array(a) => a.into_iter().map(|v| v.as_str().map_or_else(|| v.to_string(), str::to_string)).collect(),
        Value::String(s) => vec![s],
        _ => Vec::new(),
    })
}

/// Longer name from the URL path when the last segment is not unique within the resource.
/// GitHub archives → `<repo>-<ref>.<ext>`; the rest → path segments joined by `_`.
fn path_name(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let rest = rest.split(['?', '#']).next().unwrap_or(rest);
    let mut parts = rest.split('/');
    let host = parts.next().unwrap_or("");
    let segs: Vec<String> = parts.filter(|s| !s.is_empty()).map(file_name).collect();
    let last = segs.last().cloned().unwrap_or_default();
    match host {
        "github.com" if segs.len() >= 4 && segs[2] == "archive" => format!("{}-{last}", segs[1]),
        "codeload.github.com" if segs.len() >= 4 => {
            let ext = if segs[2].contains("zip") { "zip" } else { "tar.gz" };
            format!("{}-{last}.{ext}", segs[1])
        }
        _ => {
            let mut name = segs.join("_");
            while name.len() > 180 {
                name.remove(0);
            }
            name
        }
    }
}

#[cfg(test)]
mod path_tests {
    use super::*;

    #[test]
    fn disambiguated_names() {
        assert_eq!(path_name("https://github.com/apertium/apertium-eng/archive/refs/heads/master.tar.gz"), "apertium-eng-master.tar.gz");
        assert_eq!(path_name("https://codeload.github.com/apertium/apertium-ukr/tar.gz/refs/heads/main"), "apertium-ukr-main.tar.gz");
        assert_eq!(path_name("https://lindat.example/api/core/bitstreams/ab-12/content"), "api_core_bitstreams_ab-12_content");
    }
}

/// Merges group manifests into one without duplicates. Argument order is priority.
/// - same slug → the first one stays;
/// - a resource's URL set ⊆ another's (equal sets — the first stays) → the resource is dropped;
/// - partial overlap → shared URLs stay in the first, and are cut from the later one.
/// Rows without files (MANUAL) pass through as is. Every decision is printed.
fn merge(out: &Path, manifests: &[String]) -> Result<()> {
    struct Row {
        v: Value,
        slug: String,
        urls: Vec<String>,
    }
    let mut rows: Vec<Row> = Vec::new();
    for m in manifests {
        for (n, line) in BufReader::new(File::open(m).with_context(|| format!("manifest {m}"))?).lines().enumerate() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let v: Value = serde_json::from_str(&line).with_context(|| format!("{m}:{}: not JSON", n + 1))?;
            let slug = v["slug"].as_str().with_context(|| format!("{m}:{}: no slug", n + 1))?.to_string();
            let urls = v["files"]
                .as_array()
                .map(|a| a.iter().filter_map(|f| f["url"].as_str().map(str::to_string)).collect())
                .unwrap_or_default();
            rows.push(Row { v, slug, urls });
        }
    }
    let mut keep = vec![true; rows.len()];
    let mut seen = HashSet::new();
    for (i, r) in rows.iter().enumerate() {
        if !seen.insert(r.slug.clone()) {
            keep[i] = false;
            println!("DROP {} — duplicate slug", r.slug);
        }
    }
    let sets: Vec<HashSet<&str>> = rows.iter().map(|r| r.urls.iter().map(String::as_str).collect()).collect();
    for i in 0..rows.len() {
        if !keep[i] || sets[i].is_empty() {
            continue;
        }
        let cover = (0..rows.len()).find(|&j| {
            j != i && keep[j] && sets[i].is_subset(&sets[j]) && (sets[i].len() < sets[j].len() || j < i)
        });
        if let Some(j) = cover {
            keep[i] = false;
            println!("DROP {} — all {} URLs already in {}", rows[i].slug, sets[i].len(), rows[j].slug);
        }
    }
    let mut owner: HashMap<String, String> = HashMap::new();
    let mut written = 0;
    let mut text = String::new();
    for (i, r) in rows.iter_mut().enumerate() {
        if !keep[i] {
            continue;
        }
        if let Some(files) = r.v.get_mut("files").and_then(Value::as_array_mut) {
            let before = files.len();
            let mut cut = Vec::new();
            files.retain(|f| {
                let url = f["url"].as_str().unwrap_or("");
                match owner.get(url) {
                    Some(o) => {
                        cut.push(o.clone());
                        false
                    }
                    None => true,
                }
            });
            for f in files.iter() {
                if let Some(url) = f["url"].as_str() {
                    owner.insert(url.to_string(), r.slug.clone());
                }
            }
            if !cut.is_empty() {
                cut.dedup();
                println!("TRIM {} — cut {} of {before} URLs (present in {})", r.slug, before - files.len(), cut.join(", "));
                if files.is_empty() {
                    println!("DROP {} — empty after cutting", r.slug);
                    continue;
                }
            }
        }
        text.push_str(&serde_json::to_string(&r.v)?);
        text.push('\n');
        written += 1;
    }
    fs::write(out, text)?;
    println!("MERGE {} rows → {written} in {}", rows.len(), out.display());
    Ok(())
}

/// A name that is the end of a script, not a file: no extension or `.php`/`.aspx`/`.jsp`/`.cgi`/`.asp`.
fn script_like(name: &str) -> bool {
    match name.rsplit_once('.') {
        None => true,
        Some((_, ext)) => matches!(ext.to_ascii_lowercase().as_str(), "php" | "aspx" | "asp" | "jsp" | "cgi"),
    }
}

#[cfg(test)]
mod script_tests {
    use super::*;

    #[test]
    fn script_names() {
        assert!(script_like("index.php"));
        assert!(script_like("8211"));
        assert!(!script_like("ukr.xz"));
        assert!(!script_like("paper.pdf"));
    }
}

/// File names of a resource: explicit `save_as` (if `use_save_as`) > last URL segment; a script-like URL end
/// (`index.php?…`, `…/download/8211`) — name from slug and format; name clash within a slug → longer name from the path.
fn derive_names(res: &Resource, use_save_as: bool) -> Vec<String> {
    let single = res.files.len() == 1;
    let explicit = |f: &FileRef| if use_save_as { f.name.clone() } else { None };
    let mut names: Vec<String> = res
        .files
        .iter()
        .enumerate()
        .map(|(i, f)| {
            explicit(f).unwrap_or_else(|| {
                let n = file_name(&f.url);
                if script_like(&n) {
                    let ext = if res.format.chars().all(|c| c.is_ascii_alphanumeric()) && !res.format.is_empty() {
                        format!(".{}", res.format.to_ascii_lowercase())
                    } else {
                        String::new()
                    };
                    if single { format!("{}{ext}", res.slug) } else { format!("{}-{}{ext}", res.slug, i + 1) }
                } else {
                    n
                }
            })
        })
        .collect();
    for i in 0..names.len() {
        if explicit(&res.files[i]).is_none() && names.iter().filter(|n| **n == names[i]).count() > 1 {
            names[i] = path_name(&res.files[i].url);
        }
    }
    names
}
