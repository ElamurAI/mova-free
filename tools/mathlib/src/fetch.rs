//! Polite download from a list → `<raw-root>/<slug>/<file>` and a manifest.
//!
//! The list is a TSV with a header: `slug key url save_as license license_url title source_page`.
//! - `save_as` ending in `.*` — the extension comes from the response (Content-Disposition or Content-Type):
//!   arXiv `/src/ID` returns `.tar.gz`, `.gz` (a single .tex) or `.pdf` (PDF only).
//! - license is a code from the catalog; only allowed ones (`allowed`) are downloaded, others are a list error.
//!
//! Rules: one connection, a `delay` pause between requests; free space < `min_free` — stop;
//! total of files > `max_bytes` — stop; file > `file_max` — skip; 403/429 — host closed until the end
//! of the run (route fail-fast), 404 and others — record and continue, no retries; `expect_pdf` — a non-PDF
//! (placeholder page) is deleted. The first request to each host is its robots.txt (`robots.rs`):
//! disallowed paths are not downloaded, Crawl-delay is the host pause.
//!
//! Outputs: `<manifest>.jsonl` in the `tools/sources` format (one line per slug; size and md5 for
//! `sources verify`, plus sha256, license and key of every file), `<manifest>.tsv` — one line per file
//! (for DuckDB). An existing file is not downloaded again — only its hashes are recomputed.

use std::collections::{BTreeMap, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result, bail};
use md5::Md5;
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::http::{Polite, free_bytes, hex, utc_now};

pub struct Opts {
    pub delay: Duration,
    pub max_bytes: u64,
    pub file_max: u64,
    pub min_free: u64,
    pub expect_pdf: bool,
    /// arXiv sources `.tar.gz` → only text files (tex, bbl, bib, sty…) in `<stem>.tex.tar.gz`,
    /// the original with figures is deleted (goal: compact LaTeX sources and compressed text).
    pub compact: bool,
    pub kind: String,
}

struct Item {
    slug: String,
    key: String,
    url: String,
    save_as: String,
    license: String,
    license_url: String,
    title: String,
    source_page: String,
}

/// License codes allowed for download (arXiv and textbooks).
fn allowed(code: &str) -> bool {
    matches!(
        code,
        "cc-by-4.0" | "cc-by-3.0" | "cc-by-sa-4.0" | "cc0-1.0" | "pd-cc-old" | "cc-by" | "cc-by-sa" | "cc0" | "public-domain" | "gfdl"
            // tests3 (26.09): software test data — permissive code licenses (Boost, numpy/SciPy, NIST)
            | "bsl-1.0" | "bsd-3-clause" | "mit"
            // Octave tests (%!test) are GPL: only in raw as an external benchmark, never in the repo (tests3 brief)
            | "gpl-3.0-or-later"
    )
}

fn status(code: &str) -> &'static str {
    match code {
        "cc0-1.0" | "cc0" | "pd-cc-old" | "public-domain" => "public-domain",
        _ => "open",
    }
}

fn read_list(path: &Path) -> Result<Vec<Item>> {
    let mut out = Vec::new();
    let mut lines = BufReader::new(File::open(path).with_context(|| format!("{}", path.display()))?).lines();
    let head = lines.next().context("empty list")??;
    let cols: Vec<&str> = head.split('\t').collect();
    let idx = |name: &str| cols.iter().position(|c| *c == name).with_context(|| format!("header lacks {name}"));
    let (i_slug, i_key, i_url, i_save, i_lic, i_lurl, i_title, i_page) =
        (idx("slug")?, idx("key")?, idx("url")?, idx("save_as")?, idx("license")?, idx("license_url")?, idx("title")?, idx("source_page")?);
    for (n, line) in lines.enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = line.split('\t').collect();
        let get = |i: usize| c.get(i).map_or("", |s| s.trim()).to_string();
        let it = Item {
            slug: get(i_slug),
            key: get(i_key),
            url: get(i_url),
            save_as: get(i_save),
            license: get(i_lic),
            license_url: get(i_lurl),
            title: get(i_title),
            source_page: get(i_page),
        };
        if it.slug.is_empty() || it.url.is_empty() || it.save_as.is_empty() || it.save_as.contains('/') {
            bail!("{}:{}: empty slug/url/save_as or '/' in the name", path.display(), n + 2);
        }
        if !allowed(&it.license) {
            bail!("{}:{}: license \"{}\" is not allowed — not downloading this", path.display(), n + 2, it.license);
        }
        out.push(it);
    }
    Ok(out)
}

fn host(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    rest.split(['/', '?', '#']).next().unwrap_or("").to_ascii_lowercase()
}

/// Extension from the response: the name from Content-Disposition, otherwise the type.
fn ext_from(disposition: &str, content_type: &str) -> Option<&'static str> {
    let d = disposition.to_ascii_lowercase();
    if d.contains(".tar.gz") || d.contains(".tgz") {
        return Some("tar.gz");
    }
    if d.contains(".pdf") {
        return Some("pdf");
    }
    if d.contains(".gz") {
        return Some("gz");
    }
    let t = content_type.to_ascii_lowercase();
    if t.contains("pdf") {
        Some("pdf")
    } else if t.contains("gzip") {
        Some("gz")
    } else if t.contains("x-tar") {
        Some("tar")
    } else {
        None
    }
}

fn magic_ok(path: &Path) -> Result<bool> {
    let mut head = [0u8; 8];
    let n = File::open(path)?.read(&mut head)?;
    let name = path.to_string_lossy().to_ascii_lowercase();
    Ok(if name.ends_with(".pdf") {
        head[..n].starts_with(b"%PDF")
    } else if name.ends_with(".gz") {
        head[..n].starts_with(b"\x1f\x8b")
    } else {
        true
    })
}

fn hash_file(path: &Path) -> Result<(u64, String, String)> {
    let mut f = File::open(path)?;
    let (mut sha, mut md5) = (Sha256::new(), Md5::new());
    let mut buf = vec![0u8; 1 << 20];
    let mut n_all = 0u64;
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        n_all += n as u64;
        sha.update(&buf[..n]);
        md5.update(&buf[..n]);
    }
    Ok((n_all, hex(&sha.finalize()), hex(&md5.finalize())))
}

/// An existing file for `save_as` (with `.*` — any extension except `.part`).
fn existing(dir: &Path, save_as: &str) -> Option<PathBuf> {
    match save_as.strip_suffix(".*") {
        Some(stem) => fs::read_dir(dir).ok()?.flatten().map(|e| e.path()).find(|p| {
            p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with(&format!("{stem}.")) && !n.ends_with(".part") && !n.ends_with(".dl"))
        }),
        None => Some(dir.join(save_as)).filter(|p| p.exists()),
    }
}

struct Done {
    item_idx: usize,
    file: String,
    bytes: u64,
    sha256: String,
    md5: String,
    content_type: String,
    final_url: String,
    at: String,
    /// Compression: (original bytes, original sha256, files kept, files dropped).
    orig: Option<(u64, String, usize, usize)>,
}

/// Source text files that are kept after compression.
const TEXT_EXT: [&str; 14] = ["tex", "ltx", "latex", "bbl", "bib", "sty", "cls", "bst", "def", "cfg", "clo", "txt", "md", "dtx"];

/// `.tar.gz` → `.tex.tar.gz` with text files only (up to 5 MB each). `None` — no text
/// (the original is kept). Archive paths are never extracted to disk — only repacked.
fn compact_tar_gz(src: &Path, dst: &Path) -> Result<Option<(usize, usize)>> {
    use flate2::{Compression, read::GzDecoder, write::GzEncoder};
    let mut ar = tar::Archive::new(GzDecoder::new(BufReader::new(File::open(src)?)));
    let tmp = dst.with_extension("part");
    let mut b = tar::Builder::new(GzEncoder::new(File::create(&tmp)?, Compression::best()));
    let (mut kept, mut dropped) = (0usize, 0usize);
    for e in ar.entries()? {
        let mut e = e?;
        if !e.header().entry_type().is_file() {
            continue;
        }
        let path = e.path()?.into_owned();
        let ext = path.extension().and_then(|x| x.to_str()).unwrap_or("").to_ascii_lowercase();
        let size = e.header().size()?;
        if TEXT_EXT.contains(&ext.as_str()) && size <= 5 << 20 {
            let mut h = tar::Header::new_gnu();
            h.set_size(size);
            h.set_mode(0o644);
            h.set_mtime(e.header().mtime().unwrap_or(0));
            h.set_entry_type(tar::EntryType::Regular);
            b.append_data(&mut h, &path, &mut e)?;
            kept += 1;
        } else {
            dropped += 1;
        }
    }
    b.into_inner()?.finish()?.sync_all()?;
    if kept == 0 {
        fs::remove_file(&tmp)?;
        return Ok(None);
    }
    fs::rename(&tmp, dst)?;
    Ok(Some((kept, dropped)))
}

pub fn run(list: &Path, raw: &Path, manifest: &Path, o: &Opts) -> Result<()> {
    let items = read_list(list)?;
    let free = free_bytes(raw.parent().unwrap_or(raw))?;
    println!("{} START {} rows; free {:.1} GiB (as df -h); cap {:.2} GB", utc_now(), items.len(), free as f64 / (1u64 << 30) as f64, o.max_bytes as f64 / 1e9);
    if free < o.min_free {
        bail!("free {:.1} GiB < {:.0} GiB — stopping before download (free-space rule)", free as f64 / (1u64 << 30) as f64, o.min_free as f64 / (1u64 << 30) as f64);
    }
    let mut http = Polite::new(o.delay);
    let mut gate = crate::robots::Gate::new(o.delay);
    let mut blocked: HashSet<String> = HashSet::new();
    let mut done: Vec<Done> = Vec::new();
    let mut total = 0u64;
    let (mut miss, mut fresh) = (0usize, 0usize);
    let log_path = manifest.with_extension("log");
    let fresh_log = !log_path.exists();
    let mut log = BufWriter::new(std::fs::OpenOptions::new().create(true).append(true).open(&log_path)?);
    if fresh_log {
        writeln!(log, "utc\tslug\tkey\tresult\tdetail\turl")?;
    }
    let stop = manifest.with_file_name("STOP");
    for (i, it) in items.iter().enumerate() {
        let dir = raw.join(&it.slug);
        fs::create_dir_all(&dir)?;
        if let Some(p) = existing(&dir, &it.save_as) {
            let (bytes, sha256, md5) = hash_file(&p)?;
            total += bytes;
            done.push(Done {
                item_idx: i,
                file: p.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                bytes,
                sha256,
                md5,
                content_type: String::new(),
                final_url: String::new(),
                at: utc_now(),
                orig: None,
            });
            continue;
        }
        if stop.exists() {
            println!("{} PAUSE STOP file {} — stopping; to resume: remove STOP and run again", utc_now(), stop.display());
            break;
        }
        if fresh > 0 && fresh % 100 == 0 {
            log.flush()?;
            write_manifest(&items, &done, manifest, o)?;
        }
        if total >= o.max_bytes {
            println!("{} CAP total {:.2} GB ≥ cap — not downloading further", utc_now(), total as f64 / 1e9);
            writeln!(log, "{}\t{}\t{}\tcap\t\t{}", utc_now(), it.slug, it.key, it.url)?;
            break;
        }
        let h = host(&it.url);
        if blocked.contains(&h) {
            writeln!(log, "{}\t{}\t{}\tskip-blocked-host\t{h}\t{}", utc_now(), it.slug, it.key, it.url)?;
            miss += 1;
            continue;
        }
        if fresh > 0 && fresh % 10 == 0 {
            let free = free_bytes(raw)?;
            if free < o.min_free {
                println!("{} STOP free {:.1} GB < threshold", utc_now(), free as f64 / 1e9);
                writeln!(log, "{}\t{}\t{}\tstop-disk\t{free}\t{}", utc_now(), it.slug, it.key, it.url)?;
                break;
            }
        }
        // robots.txt and host pacing — from the first request
        if let Err(why) = gate.check(&mut http, &it.url)? {
            miss += 1;
            println!("{} ROBOTS-SKIP {}/{} {why}", utc_now(), it.slug, it.key);
            writeln!(log, "{}\t{}\t{}\trobots-disallow\t{why}\t{}", utc_now(), it.slug, it.key, it.url)?;
            continue;
        }
        let stem = it.save_as.strip_suffix(".*").unwrap_or(&it.save_as).to_string();
        let part = dir.join(format!("{stem}.dl"));
        let r = http.download(&it.url, &part, o.file_max.min(o.max_bytes.saturating_sub(total).max(1)));
        fresh += 1;
        let f = match r {
            Ok(Ok(f)) => f,
            Ok(Err(st)) => {
                miss += 1;
                let detail = format!("HTTP {} {}", st.code, st.body.chars().take(120).collect::<String>().replace(['\t', '\n'], " "));
                println!("{} MISS {}/{} {detail}", utc_now(), it.slug, it.key);
                writeln!(log, "{}\t{}\t{}\tmiss\t{detail}\t{}", utc_now(), it.slug, it.key, it.url)?;
                if st.code == 403 || st.code == 429 {
                    println!("{} BLOCK host {h} — HTTP {}: route stopped until the end of the run", utc_now(), st.code);
                    blocked.insert(h);
                } else if st.code == 503 {
                    // the server asks to slow down: wait Retry-After (without retrying this row —
                    // the next run will pick it up; fetch does not re-download existing files)
                    let wait = st.retry_after.unwrap_or(30).clamp(5, 600);
                    println!("{} 503 Retry-After {:?} — waiting {wait} s", utc_now(), st.retry_after);
                    std::thread::sleep(Duration::from_secs(wait));
                }
                continue;
            }
            Err(e) => {
                miss += 1;
                let detail = format!("{e:#}").replace(['\t', '\n'], " ");
                println!("{} ERR {}/{} {detail}", utc_now(), it.slug, it.key);
                writeln!(log, "{}\t{}\t{}\terror\t{detail}\t{}", utc_now(), it.slug, it.key, it.url)?;
                continue;
            }
        };
        let ext = if it.save_as.ends_with(".*") {
            match ext_from(&f.disposition, &f.content_type) {
                Some(e) => e.to_string(),
                None => {
                    fs::remove_file(&part)?;
                    miss += 1;
                    writeln!(log, "{}\t{}\t{}\tunknown-type\t{} {}\t{}", utc_now(), it.slug, it.key, f.content_type, f.disposition, it.url)?;
                    continue;
                }
            }
        } else {
            String::new()
        };
        let name = if ext.is_empty() { it.save_as.clone() } else { format!("{stem}.{ext}") };
        let dest = dir.join(&name);
        fs::rename(&part, &dest)?;
        let is_pdf = name.to_ascii_lowercase().ends_with(".pdf");
        if !magic_ok(&dest)? || (o.expect_pdf && !is_pdf) {
            fs::remove_file(&dest)?;
            miss += 1;
            println!("{} NOT {}/{} — wrong signature ({})", utc_now(), it.slug, it.key, f.content_type);
            writeln!(log, "{}\t{}\t{}\tbad-signature\t{}\t{}", utc_now(), it.slug, it.key, f.content_type, it.url)?;
            continue;
        }
        let (mut name, mut bytes, mut sha256, mut md5, mut orig) = (name, f.bytes, f.sha256, f.md5, None);
        if o.compact && name.ends_with(".tar.gz") && !name.ends_with(".tex.tar.gz") {
            let small = dir.join(format!("{stem}.tex.tar.gz"));
            match compact_tar_gz(&dest, &small) {
                Ok(Some((kept, dropped))) => {
                    let (b2, s2, m2) = hash_file(&small)?;
                    fs::remove_file(&dest)?;
                    orig = Some((bytes, sha256.clone(), kept, dropped));
                    name = format!("{stem}.tex.tar.gz");
                    (bytes, sha256, md5) = (b2, s2, m2);
                }
                Ok(None) => println!("{} KEEP {}/{name} — no text in the archive, keeping as is", utc_now(), it.slug),
                Err(e) => println!("{} KEEP {}/{name} — not a tar ({e:#}), keeping as is", utc_now(), it.slug),
            }
        }
        total += bytes;
        let orig_s = orig.as_ref().map_or(String::new(), |(b, s, k, d)| format!(" orig={b} orig_sha256={s} kept={k} dropped={d}"));
        println!("{} OK {}/{name} {bytes} B {}{orig_s} (total {:.3} GB)", utc_now(), it.slug, f.content_type, total as f64 / 1e9);
        writeln!(log, "{}\t{}\t{}\tok\t{bytes} {}{orig_s}\t{}", utc_now(), it.slug, it.key, f.content_type, it.url)?;
        done.push(Done {
            item_idx: i,
            file: name,
            bytes,
            sha256,
            md5,
            content_type: f.content_type,
            final_url: f.final_url,
            at: utc_now(),
            orig,
        });
    }
    log.flush()?;
    write_manifest(&items, &done, manifest, o)?;
    println!(
        "{} END files {} ({} new), misses {miss}, {:.3} GB; manifest {}",
        utc_now(),
        done.len(),
        fresh.saturating_sub(miss),
        total as f64 / 1e9,
        manifest.display()
    );
    Ok(())
}

fn write_manifest(items: &[Item], done: &[Done], manifest: &Path, o: &Opts) -> Result<()> {
    let mut by_slug: BTreeMap<&str, Vec<&Done>> = BTreeMap::new();
    for d in done {
        by_slug.entry(items[d.item_idx].slug.as_str()).or_default().push(d);
    }
    let mut j = BufWriter::new(File::create(manifest)?);
    for (slug, ds) in &by_slug {
        let first = &items[ds[0].item_idx];
        let lics: HashSet<&str> = ds.iter().map(|d| items[d.item_idx].license.as_str()).collect();
        let (license, license_url, status_s) = if lics.len() == 1 {
            (first.license.clone(), first.license_url.clone(), status(&first.license))
        } else {
            let mut v: Vec<&str> = lics.into_iter().collect();
            v.sort();
            (format!("per file: {}", v.join(", ")), String::new(), "open")
        };
        let files: Vec<_> = ds
            .iter()
            .map(|d| {
                let it = &items[d.item_idx];
                let mut v = json!({"url": it.url, "save_as": d.file, "size": d.bytes, "md5": d.md5, "sha256": d.sha256,
                       "key": it.key, "license": it.license, "license_url": it.license_url,
                       "content_type": d.content_type, "title": it.title, "source_page": it.source_page, "fetched": d.at});
                if let Some((b, s, k, dr)) = &d.orig {
                    v["orig_size"] = json!(b);
                    v["orig_sha256"] = json!(s);
                    v["text_files"] = json!(k);
                    v["dropped_files"] = json!(dr);
                }
                v
            })
            .collect();
        let name = if ds.len() == 1 { first.title.clone() } else { format!("{slug} ({} files)", ds.len()) };
        let fmt = ds.iter().map(|d| d.file.rsplit('.').next().unwrap_or("")).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>().join("/");
        let line = json!({"slug": slug, "name": name, "langs": ["en"], "kind": o.kind, "status": status_s,
                          "license": license, "license_url": license_url, "source_page": first.source_page,
                          "format": fmt, "files": files,
                          "notes": "mathlib fetch: sha256 and license of every file; check with sources verify"});
        writeln!(j, "{}", serde_json::to_string(&line)?)?;
    }
    j.flush()?;
    let mut t = BufWriter::new(File::create(manifest.with_extension("tsv"))?);
    writeln!(t, "slug\tkey\tfile\tbytes\tsha256\tmd5\tlicense\tlicense_url\turl\tfinal_url\tcontent_type\ttitle\tfetched")?;
    for d in done {
        let it = &items[d.item_idx];
        writeln!(
            t,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            it.slug, it.key, d.file, d.bytes, d.sha256, d.md5, it.license, it.license_url, it.url, d.final_url, d.content_type, it.title, d.at
        )?;
    }
    t.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tar_gz(path: &Path, files: &[(&str, &[u8])]) {
        use flate2::{Compression, write::GzEncoder};
        let mut b = tar::Builder::new(GzEncoder::new(File::create(path).unwrap(), Compression::fast()));
        for (name, data) in files {
            let mut h = tar::Header::new_gnu();
            h.set_size(data.len() as u64);
            h.set_mode(0o644);
            h.set_entry_type(tar::EntryType::Regular);
            b.append_data(&mut h, name, *data).unwrap();
        }
        b.into_inner().unwrap().finish().unwrap();
    }

    #[test]
    fn compacts_sources() {
        let dir = std::env::temp_dir().join(format!("mathlib-test-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let src = dir.join("a.tar.gz");
        tar_gz(&src, &[("main.tex", b"\\documentclass{article}"), ("fig.png", &[0u8; 5000]), ("refs.bbl", b"x")]);
        let dst = dir.join("a.tex.tar.gz");
        assert_eq!(compact_tar_gz(&src, &dst).unwrap(), Some((2, 1)));
        let mut names = Vec::new();
        let mut ar = tar::Archive::new(flate2::read::GzDecoder::new(File::open(&dst).unwrap()));
        for e in ar.entries().unwrap() {
            names.push(e.unwrap().path().unwrap().to_string_lossy().into_owned());
        }
        assert_eq!(names, vec!["main.tex", "refs.bbl"]);
        // negative control: the archive holds only a figure — compression refuses, the original is kept
        let only_fig = dir.join("b.tar.gz");
        tar_gz(&only_fig, &[("fig.png", &[1u8; 100])]);
        assert_eq!(compact_tar_gz(&only_fig, &dir.join("b.tex.tar.gz")).unwrap(), None);
        assert!(!dir.join("b.tex.tar.gz").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn ext_and_licenses() {
        assert_eq!(ext_from("attachment; filename=\"arXiv-2508.00001v1.tar.gz\"", "application/gzip"), Some("tar.gz"));
        assert_eq!(ext_from("attachment; filename=\"arXiv-2508.00002v1.gz\"", "application/gzip"), Some("gz"));
        assert_eq!(ext_from("", "application/pdf"), Some("pdf"));
        assert_eq!(ext_from("", "text/html; charset=utf-8"), None);
        assert!(allowed("cc-by-4.0") && allowed("gfdl") && allowed("cc0-1.0"));
        // negative controls: NC, standard arXiv and unknown are not downloaded
        assert!(!allowed("cc-by-nc-sa-4.0") && !allowed("arxiv-1.0") && !allowed("none") && !allowed("cc-by-nd"));
    }
}
