//! `mova fetch`: download a release's files (the trained model, the corpus, the dictionaries, add-on modules) from
//! the release manifest in the repository (`releases/<release>-files.json`, compiled in for 0.5), over HTTPS, from
//! the .torrent or from the magnet link, check every byte, unpack and install.
//!
//!   mova fetch [RELEASE] [ID|FILE…|all] [--via https|torrent|magnet] [--magnet LINK] [--manifest FILE]
//!              [--base URL] [--keep-archive]
//!   mova fetch [RELEASE] --page [--manifest FILE]   # print the release-files page (releases/<release>-files.md)
//!
//! Without names it fetches the required files (the model). The three ways end in the same checks: the archive's size
//! and SHA-256 must equal the manifest's, or it is refused and nothing is unpacked.
//!
//! - `https`: the file's URL, through curl.
//! - `torrent`: the .torrent over HTTPS; its info hash must equal the manifest's `btih`; the archive is then taken from
//!   the torrent's web seed (BEP 19 `url-list`) and every piece is checked against the torrent's SHA-1 before the
//!   SHA-256.
//! - `magnet`: the magnet link (the manifest's or `--magnet`); its `btih` must be one of the manifest's files; the
//!   .torrent comes from its `xs` source over HTTPS and must hash to that `btih`; then as `torrent`.
//!
//! What is implemented of BitTorrent: bencode, single-file v1 metainfo, info hash, magnet links (`xt` btih, `dn`, `xl`,
//! `tr`, `ws`, `xs`), piece verification and BEP 19 web seeds. Not implemented: the peer wire protocol, trackers, DHT
//! and metadata exchange (BEP 9); to download from peers, give the .torrent or magnet link to a BitTorrent client and
//! install from the archive it writes with `--archive FILE` (the same checks apply).
//!
//! Install: the archive (one top folder) is unpacked under `$MOVA_DATA/<unpack_to>/<top>`, then each `install` entry
//! is copied to `$MOVA_DATA/<to>`. The unpacked folder must hold exactly the manifest's `contents` (sizes, SHA-256, no other
//! files); a cached unpacked folder is reused only after the same inventory check, otherwise the verified archive is
//! unpacked again. An existing different file or folder there is kept, renamed `<to>.before-<release>`;
//! nothing is deleted. A module archive is unpacked into the module root and verified against its module.json.

use crate::torrent::{self, Magnet};
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};

/// A release file with more files inside is rendered per top folder; the JSON manifest keeps every file.
const RENDER_CONTENTS_MAX: usize = 200;

/// The manifest of release 0.5, as shipped in the repository.
pub const MANIFEST_0_5: &str = include_str!("../../../releases/0.5-files.json");

/// The licence identifier of Mova's code (the Elamur Cosmic License); no data file of a release carries it.
pub const OURS: &str = "LicenseRef-Elamur-Cosmic";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Install {
    /// a path inside the archive's top folder
    pub from: String,
    /// a path under the data directory
    pub to: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Content {
    pub path: String,
    pub bytes: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseFile {
    /// a short name for the command line: model, corpus, dictionaries, school, or a module's name
    pub id: String,
    pub name: String,
    /// model, corpus, dictionaries, school or module
    pub kind: String,
    pub required: bool,
    pub what: String,
    pub bytes: u64,
    /// the bytes of the files inside, unpacked
    pub raw_bytes: u64,
    pub sha256: String,
    pub licence: String,
    /// the licence text, a file of the download folder
    pub licence_file: String,
    pub url: String,
    pub torrent_url: String,
    pub btih: String,
    pub magnet: String,
    /// the archive's one top folder (a module's `<name>/<version>`)
    pub top: String,
    /// where under the data directory the top folder is unpacked
    pub unpack_to: String,
    #[serde(default)]
    pub install: Vec<Install>,
    /// the files inside the archive, under `top`
    #[serde(default)]
    pub contents: Vec<Content>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseFiles {
    pub schema: u32,
    pub release: String,
    pub date: String,
    /// draft until the release is published
    pub status: String,
    pub base_url: String,
    pub page: String,
    pub trackers: Vec<String>,
    pub files: Vec<ReleaseFile>,
}

fn is_hex(s: &str, n: usize) -> bool {
    s.len() == n && s.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}

impl ReleaseFiles {
    pub fn parse(s: &str) -> Result<ReleaseFiles> {
        let r: ReleaseFiles = serde_json::from_str(s).context("reading the release manifest")?;
        r.check()?;
        Ok(r)
    }

    /// The manifest's own consistency and the data policy (licence/DATA.md, the policy on dictionaries and add-on
    /// modules): the model, the corpus and the shipped dictionaries are CC BY-SA 4.0 (data, not code: the code
    /// licence covers only code); nothing Wikipedia- or Wiktionary-derived outside a module; a module is never required
    /// and goes to the module root; only the model is required. URLs and magnet links follow from the base URL.
    pub fn check(&self) -> Result<()> {
        ensure!(self.schema == 1, "schema {} (this build reads 1)", self.schema);
        ensure!(self.base_url.starts_with("https://") && self.base_url.ends_with('/'), "base_url is an https:// folder: {}", self.base_url);
        let mut seen = std::collections::BTreeSet::new();
        for f in &self.files {
            let at = &f.name;
            ensure!(seen.insert(f.id.clone()) && seen.insert(f.name.clone()), "{at}: a duplicate id or name");
            ensure!(crate::safe_rel(&f.name) && !f.name.contains('/') && f.name.ends_with(".tar.gz"), "{at}: a release file is a .tar.gz name");
            ensure!(is_hex(&f.sha256, 64) && is_hex(&f.btih, 40), "{at}: sha256 or btih malformed");
            ensure!(f.url == format!("{}{}", self.base_url, f.name), "{at}: url is not base_url + name");
            ensure!(f.torrent_url == format!("{}.torrent", f.url), "{at}: torrent_url is not url + .torrent");
            let m = Magnet { btih: f.btih.clone(), dn: f.name.clone(), xl: Some(f.bytes), tr: self.trackers.clone(), ws: vec![f.url.clone()], xs: vec![f.torrent_url.clone()] };
            ensure!(f.magnet == m.link(), "{at}: the magnet link does not carry btih, dn, tr, ws, xs and xl of this manifest");
            ensure!(crate::safe_rel(&f.top) && crate::safe_rel(&f.unpack_to), "{at}: top or unpack_to is not a safe relative path");
            ensure!(!f.licence_file.is_empty() && !f.licence_file.contains('/'), "{at}: licence_file names a file of the download folder");
            for i in &f.install {
                ensure!(crate::safe_rel(&i.from) && crate::safe_rel(&i.to), "{at}: an install path is not safe: {} → {}", i.from, i.to);
                ensure!(f.contents.iter().any(|c| c.path == i.from || c.path.starts_with(&format!("{}/", i.from))), "{at}: installs {} which it does not contain", i.from);
            }
            for c in &f.contents {
                ensure!(crate::safe_rel(&c.path) && is_hex(&c.sha256, 64), "{at}: content {} malformed", c.path);
            }
            ensure!(f.contents.iter().map(|c| c.bytes).sum::<u64>() == f.raw_bytes, "{at}: raw_bytes is not the sum of its contents");
            let wiki = |s: &str| { let l = s.to_ascii_lowercase(); l.contains("wiki") || l.contains("wikt") };
            match f.kind.as_str() {
                "model" | "corpus" | "dictionaries" | "school" => ensure!(f.licence == "CC-BY-SA-4.0", "{at}: a {} is CC-BY-SA-4.0, not {}", f.kind, f.licence),
                "module" => {
                    ensure!(f.licence != OURS && !f.required, "{at}: a module keeps its own licence and is never required");
                    ensure!(f.unpack_to == "modules" && f.install.is_empty(), "{at}: a module unpacks into the module root");
                }
                k => bail!("{at}: unknown kind {k}"),
            }
            if f.kind != "module" {
                ensure!(!wiki(&f.name) && !f.contents.iter().any(|c| wiki(&c.path)), "{at}: Wikipedia or Wiktionary data only as a module");
            }
            ensure!(f.required == (f.kind == "model"), "{at}: only the model is required");
        }
        Ok(())
    }

    pub fn select(&self, names: &[String]) -> Result<Vec<&ReleaseFile>> {
        if names.is_empty() {
            return Ok(self.files.iter().filter(|f| f.required).collect());
        }
        if names.iter().any(|n| n == "all") {
            return Ok(self.files.iter().collect());
        }
        names.iter().map(|n| self.files.iter().find(|f| &f.id == n || &f.name == n).with_context(|| format!("release {} has no file {n} (ids: {})", self.release, self.files.iter().map(|f| f.id.as_str()).collect::<Vec<_>>().join(", ")))).collect()
    }
}

fn human(b: u64) -> String {
    match b {
        b if b >= 1 << 30 => format!("{:.2} GiB", b as f64 / (1u64 << 30) as f64),
        b if b >= 1 << 20 => format!("{:.1} MiB", b as f64 / (1u64 << 20) as f64),
        b if b >= 1 << 10 => format!("{:.1} KiB", b as f64 / 1024.0),
        b => format!("{b} B"),
    }
}

/// The public Markdown page about the release's files (`releases/<release>-files.md`), made from the manifest, so
/// the two always say the same.
pub fn render_bone(r: &ReleaseFiles) -> String {
    let mut s = format!("# Mova {} — the release files\n\n", r.release);
    if r.status == "draft" {
        s.push_str("> **Draft.** These files are published for the coming release and may still change until it is released.\n\n");
    }
    s.push_str(&format!(
        "The files that come with Mova {} beside the code, as of {}. Each is one `.tar.gz` archive with one top folder, \
         served over HTTPS from <{}> and by BitTorrent (a `.torrent` file and a magnet link for each). Every torrent has \
         the HTTPS file as its web seed (BEP 19), so it completes even when no peer is online. Trackers: {}; the DHT \
         too. The release page: <{}>. The same data, for programs: [{}-files.json]({}-files.json).\n\n",
        r.release,
        r.date,
        r.base_url,
        r.trackers.iter().map(|t| format!("`{t}`")).collect::<Vec<_>>().join(", "),
        r.page,
        r.release,
        r.release
    ));
    s.push_str(&format!(
        "Mova fetches them itself: `mova` is the binary of `tools/modules`, which `install.sh` builds as \
         `target/release/mova` and runs with the first line. From the repository root:\n\n```sh\n\
         target/release/mova fetch {v}                       # the required files (the model), over HTTPS\n\
         target/release/mova fetch {v} all --via torrent     # every file, from its .torrent (web seed, piece check)\n\
         target/release/mova fetch {v} corpus --via magnet   # from the magnet link (the .torrent from its xs source)\n```\n\n\
         To call it as plain `mova`, put it on the PATH (default target folder; with `CARGO_TARGET_DIR` set, use \
         `$CARGO_TARGET_DIR/release` instead): `export PATH=\"$PWD/target/release:$PATH\"`.\n\n\
         Whichever way, the archive's SHA-256 must equal the one below, or it is refused and nothing is unpacked. \
         `mova fetch` does not talk to peers; to download from peers, give the .torrent or the magnet link to a \
         BitTorrent client and then run `target/release/mova fetch {v} ID --archive FILE`.\n\n",
        v = r.release
    ));
    s.push_str("| file | id | needed | what it is | size (unpacked) | licence |\n|---|---|---|---|---|---|\n");
    for f in &r.files {
        s.push_str(&format!(
            "| [`{}`]({}) | `{}` | {} | {} | {} ({}) | {} ([text]({}{})) |\n",
            f.name,
            f.url,
            f.id,
            if f.required { "required" } else { "optional" },
            f.what,
            human(f.bytes),
            human(f.raw_bytes),
            f.licence,
            r.base_url,
            f.licence_file
        ));
    }
    for f in &r.files {
        s.push_str(&format!("\n## {}\n\n{}\n\n", f.name, f.what));
        s.push_str(&format!("- needed: {}\n", if f.required { "required" } else { "optional" }));
        s.push_str(&format!("- size: {} bytes ({}); unpacked {} bytes ({})\n", f.bytes, human(f.bytes), f.raw_bytes, human(f.raw_bytes)));
        s.push_str(&format!("- SHA-256: `{}`\n", f.sha256));
        s.push_str(&format!("- licence: {}, text: <{}{}>\n", f.licence, r.base_url, f.licence_file));
        s.push_str(&format!("- HTTPS: <{}>\n", f.url));
        s.push_str(&format!("- torrent: <{}> (info hash v1 `{}`)\n", f.torrent_url, f.btih));
        s.push_str(&format!("- magnet:\n\n  ```\n  {}\n  ```\n\n", f.magnet));
        s.push_str(&format!("- unpacks to: `$MOVA_DATA/{}/{}/`\n", f.unpack_to, f.top));
        for i in &f.install {
            s.push_str(&format!("- installs: `{}` → `$MOVA_DATA/{}`\n", i.from, i.to));
        }
        if f.contents.len() <= RENDER_CONTENTS_MAX {
            s.push_str("\n| inside | bytes | SHA-256 |\n|---|---|---|\n");
            for c in &f.contents {
                s.push_str(&format!("| `{}` | {} | `{}` |\n", c.path, c.bytes, c.sha256));
            }
        } else {
            // a large archive (the school's lessons): per top folder; every file with its SHA-256 is in the JSON manifest
            let mut by: std::collections::BTreeMap<&str, (u64, u64)> = std::collections::BTreeMap::new();
            for c in &f.contents {
                let e = by.entry(c.path.split('/').next().unwrap_or("")).or_default();
                e.0 += 1;
                e.1 += c.bytes;
            }
            s.push_str(&format!("\n{} files inside; each with its size and SHA-256 is listed in the JSON manifest.\n\n| inside | files | bytes |\n|---|---|---|\n", f.contents.len()));
            for (top, (n, b)) in by {
                s.push_str(&format!("| `{top}` | {n} | {b} |\n"));
            }
        }
    }
    s
}

/// Fetch a URL into a file: `https://` through curl, `file://` by copying; anything else is refused.
pub fn download_to(url: &str, dest: &Path) -> Result<()> {
    if let Some(p) = url.strip_prefix("file://") {
        std::fs::copy(p, dest).with_context(|| format!("copying {p}"))?;
        return Ok(());
    }
    ensure!(url.starts_with("https://"), "only https:// (or file://) URLs: {url}");
    let st = std::process::Command::new("curl").args(["-fsSL", "--proto", "=https", "--proto-redir", "=https", "--retry", "2", "-o"]).arg(dest).arg(url).status().context("running curl (needed for https downloads)")?;
    ensure!(st.success(), "downloading {url} failed");
    Ok(())
}

pub fn sha256_file(p: &Path) -> Result<String> {
    let mut f = std::io::BufReader::new(std::fs::File::open(p).with_context(|| format!("reading {}", p.display()))?);
    let (mut h, mut buf) = (Sha256::new(), vec![0u8; 1 << 20]);
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Via {
    Https,
    Torrent,
    Magnet,
}

#[derive(Default)]
pub struct FetchOptions {
    pub via: Option<Via>,
    pub magnet: Option<String>,
    /// another server folder (a mirror, or `file://` for tests) in place of the manifest's base URL
    pub base: Option<String>,
    /// an archive already downloaded (by a BitTorrent client): checked and installed, nothing is downloaded
    pub archive: Option<PathBuf>,
    pub keep_archive: bool,
}

fn rebase(url: &str, r: &ReleaseFiles, o: &FetchOptions) -> String {
    match (&o.base, url.strip_prefix(&r.base_url)) {
        (Some(b), Some(rest)) => format!("{}/{rest}", b.trim_end_matches('/')),
        _ => url.to_string(),
    }
}

/// Download one release file into `dest` by the chosen way; returns how (for the report).
fn get(r: &ReleaseFiles, f: &ReleaseFile, o: &FetchOptions, dest: &Path) -> Result<String> {
    let via = if o.magnet.is_some() { Via::Magnet } else { o.via.unwrap_or(Via::Https) };
    if via == Via::Https {
        download_to(&rebase(&f.url, r, o), dest)?;
        return Ok(format!("https {}", rebase(&f.url, r, o)));
    }
    let (torrent_url, how) = if via == Via::Magnet {
        let link = o.magnet.clone().unwrap_or_else(|| f.magnet.clone());
        let m = Magnet::parse(&link)?;
        ensure!(m.btih == f.btih, "the magnet link's info hash {} is not {}'s ({})", m.btih, f.name, f.btih);
        let xs = m.xs.iter().find(|x| x.starts_with("https://") || x.starts_with("file://")).context("the magnet link has no https:// xs source for its .torrent; give it to a BitTorrent client and use --archive")?;
        (rebase(xs, r, o), "magnet → xs")
    } else {
        (rebase(&f.torrent_url, r, o), "torrent")
    };
    let tbytes = crate::fetch(&torrent_url)?;
    let meta = torrent::read_metainfo(&tbytes)?;
    ensure!(meta.info_hash == f.btih, "the .torrent at {torrent_url} has info hash {}, the manifest says {}", meta.info_hash, f.btih);
    ensure!(meta.length == f.bytes && meta.name == f.name, "the .torrent describes {} of {} bytes, not {} of {}", meta.name, meta.length, f.name, f.bytes);
    let seed = meta.url_list.first().context("the torrent has no web seed; give it to a BitTorrent client and use --archive")?;
    let seed = rebase(seed, r, o);
    download_to(&seed, dest)?;
    torrent::verify_file(dest, &meta).context("the web seed's file failed the torrent's piece check")?;
    Ok(format!("{how} {torrent_url}, web seed {seed}, {} pieces checked", meta.pieces.len() / 20))
}

/// List the archive and check it holds exactly one top folder `top`, with no absolute or `..` paths.
fn check_members(archive: &Path, top: &str) -> Result<()> {
    let out = std::process::Command::new("tar").arg("-tzf").arg(archive).output().context("running tar (needed to unpack)")?;
    ensure!(out.status.success(), "tar cannot list {}: {}", archive.display(), String::from_utf8_lossy(&out.stderr).trim());
    for l in String::from_utf8_lossy(&out.stdout).lines() {
        let p = l.trim_end_matches('/');
        ensure!(!p.starts_with('/') && !p.split('/').any(|s| s == ".."), "unsafe path in the archive: {l}");
        ensure!(p == top || p.starts_with(&format!("{top}/")) || top.starts_with(&format!("{p}/")), "{l} is outside the top folder {top}");
    }
    Ok(())
}

fn tree_sha(p: &Path) -> Result<String> {
    if p.is_file() {
        return sha256_file(p);
    }
    let mut all = String::new();
    let mut stack = vec![p.to_path_buf()];
    let mut files = Vec::new();
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d)? {
            let e = e?.path();
            if e.is_dir() { stack.push(e) } else { files.push(e) }
        }
    }
    files.sort();
    for f in files {
        all.push_str(&format!("{}\t{}\n", f.strip_prefix(p)?.display(), sha256_file(&f)?));
    }
    Ok(crate::sha256_hex(all.as_bytes()))
}

fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    if from.is_file() {
        std::fs::copy(from, to)?;
        return Ok(());
    }
    std::fs::create_dir_all(to)?;
    for e in std::fs::read_dir(from)? {
        let e = e?;
        copy_tree(&e.path(), &to.join(e.file_name()))?;
    }
    Ok(())
}

/// Move an existing path aside to `<path>.before-<release>` (or `-2`, `-3`, …); returns the new name.
pub(crate) fn set_aside(p: &Path, release: &str) -> Result<PathBuf> {
    let mut n = 1;
    loop {
        let q = PathBuf::from(format!("{}.before-{release}{}", p.display(), if n == 1 { String::new() } else { format!("-{n}") }));
        if !q.exists() {
            std::fs::rename(p, &q)?;
            return Ok(q);
        }
        n += 1;
    }
}

/// Check that the folder `dir` holds exactly the manifest's `contents`: the same regular files (no others, no links,
/// no special files), each with its manifest size and SHA-256. Used on a fresh extraction and before a cached
/// extraction is reused, so an installed byte always comes from a folder that equals the manifest.
fn check_inventory(dir: &Path, f: &ReleaseFile) -> Result<()> {
    let mut found = std::collections::BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).with_context(|| format!("reading {}", d.display()))? {
            let p = e?.path();
            let m = std::fs::symlink_metadata(&p)?;
            let rel = p.strip_prefix(dir)?.to_string_lossy().into_owned();
            if m.is_dir() {
                stack.push(p);
            } else {
                ensure!(m.is_file(), "{rel} in {} is not a regular file", dir.display());
                found.insert(rel, (m.len(), p));
            }
        }
    }
    for c in &f.contents {
        let (bytes, p) = found.remove(&c.path).with_context(|| format!("{} is missing from {}", c.path, dir.display()))?;
        ensure!(bytes == c.bytes && sha256_file(p.as_path())? == c.sha256, "{} in {} does not match the manifest of {}", c.path, dir.display(), f.name);
    }
    ensure!(found.is_empty(), "{} holds files the manifest of {} does not list: {}", dir.display(), f.name, found.keys().cloned().collect::<Vec<_>>().join(", "));
    Ok(())
}

/// Unpack a checked archive and install its entries; returns report lines.
pub fn unpack_and_install(r: &ReleaseFiles, f: &ReleaseFile, archive: &Path, data: &Path) -> Result<Vec<String>> {
    let mut rep = Vec::new();
    check_members(archive, &f.top)?;
    let base = data.join(&f.unpack_to);
    let dest = base.join(&f.top);
    let stamp = PathBuf::from(format!("{}.fetched", dest.display()));
    // A cached extraction is reused only when its stamp names this archive AND its inventory equals the manifest's
    // contents (the stamp alone proves nothing about the bytes beside it); otherwise it is set aside and the verified
    // archive is unpacked again.
    let stamped = dest.is_dir() && std::fs::read_to_string(&stamp).is_ok_and(|s| s.trim() == f.sha256);
    let cached = stamped && match check_inventory(&dest, f) {
        Ok(()) => true,
        Err(e) => {
            rep.push(format!("the cached {} fails its inventory check ({e:#}); unpacking the verified archive again", dest.display()));
            false
        }
    };
    if cached {
        rep.push(format!("{} already unpacked at {} (inventory checked)", f.name, dest.display()));
    } else {
        std::fs::create_dir_all(&base)?;
        let staging = base.join(format!(".fetch-{}", std::process::id()));
        std::fs::create_dir_all(&staging)?;
        let unpacked = (|| -> Result<()> {
            let st = std::process::Command::new("tar").arg("-xzf").arg(archive).arg("--no-same-owner").arg("-C").arg(&staging).status().context("running tar")?;
            ensure!(st.success(), "tar could not unpack {}", archive.display());
            check_inventory(&staging.join(&f.top), f).with_context(|| format!("{} refused after unpacking", f.name))
        })();
        if let Err(e) = unpacked {
            let _ = std::fs::remove_dir_all(&staging);
            return Err(e);
        }
        if dest.exists() {
            rep.push(format!("kept the earlier {} as {}", dest.display(), set_aside(&dest, &r.release)?.display()));
        }
        std::fs::create_dir_all(dest.parent().context("a top folder")?)?;
        std::fs::rename(staging.join(&f.top), &dest)?;
        std::fs::remove_dir_all(&staging)?;
        std::fs::write(&stamp, format!("{}\n", f.sha256))?;
        rep.push(format!("unpacked {} into {}", f.name, dest.display()));
    }
    if f.kind == "module" {
        let mbytes = std::fs::read(dest.join("module.json")).context("the module has no module.json")?;
        let m = crate::Manifest::parse(&mbytes)?;
        crate::verify_dir(&dest, &m)?;
        ensure!(m.licence == f.licence, "the module's licence {} differs from the release manifest's {}", m.licence, f.licence);
        rep.push(format!("module {}@{} verified ({}); load it with MOVA_MODULES={}", m.name, m.version, m.licence, m.name));
    }
    for i in &f.install {
        let (from, to) = (dest.join(&i.from), data.join(&i.to));
        let want = tree_sha(&from)?;
        if to.exists() && tree_sha(&to)? == want {
            rep.push(format!("$MOVA_DATA/{} is up to date", i.to));
            continue;
        }
        std::fs::create_dir_all(to.parent().context("an install path")?)?;
        let part = PathBuf::from(format!("{}.part-fetch", to.display()));
        ensure!(!part.exists(), "{} is in the way (an interrupted fetch?)", part.display());
        copy_tree(&from, &part)?;
        if to.exists() {
            rep.push(format!("kept the earlier $MOVA_DATA/{} as {}", i.to, set_aside(&to, &r.release)?.display()));
        }
        std::fs::rename(&part, &to)?;
        rep.push(format!("installed $MOVA_DATA/{}", i.to));
    }
    Ok(rep)
}

/// Fetch, check and install one release file into the data directory.
pub fn fetch_one(r: &ReleaseFiles, f: &ReleaseFile, o: &FetchOptions, data: &Path) -> Result<Vec<String>> {
    let dir = data.join("release").join(&r.release);
    std::fs::create_dir_all(&dir)?;
    let (archive, how, ours) = match &o.archive {
        Some(a) => (a.clone(), format!("given archive {}", a.display()), false),
        None => {
            let part = dir.join(format!("{}.part", f.name));
            let how = get(r, f, o, &part).inspect_err(|_| {
                let _ = std::fs::remove_file(&part);
            })?;
            (part, how, true)
        }
    };
    let size = std::fs::metadata(&archive)?.len();
    let sum = sha256_file(&archive)?;
    if size != f.bytes || sum != f.sha256 {
        if ours {
            let _ = std::fs::remove_file(&archive);
        }
        bail!("{} refused: {size} bytes, SHA-256 {sum}; the manifest says {} bytes, {}", f.name, f.bytes, f.sha256);
    }
    let mut rep = vec![format!("{}: {how}; {size} bytes, SHA-256 ok", f.name)];
    rep.extend(unpack_and_install(r, f, &archive, data)?);
    if ours {
        if o.keep_archive {
            let kept = dir.join(&f.name);
            std::fs::rename(&archive, &kept)?;
            rep.push(format!("archive kept at {}", kept.display()));
        } else {
            std::fs::remove_file(&archive)?;
        }
    }
    Ok(rep)
}

const USAGE: &str = "usage: mova fetch [RELEASE] [ID|FILE…|all] [--via https|torrent|magnet] [--magnet LINK] [--archive FILE] [--manifest FILE] [--base URL] [--keep-archive] | mova fetch [RELEASE] --page [--manifest FILE]";

/// `mova fetch …` (args after `fetch`).
pub fn main_args(args: &[String]) -> Result<()> {
    let (mut o, mut pos, mut manifest) = (FetchOptions::default(), Vec::new(), None::<PathBuf>);
    let mut page = false;
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        let mut val = || -> Result<String> {
            i += 1;
            args.get(i).cloned().with_context(|| format!("{a} needs a value"))
        };
        match a {
            "--via" => {
                o.via = Some(match val()?.as_str() {
                    "https" => Via::Https,
                    "torrent" => Via::Torrent,
                    "magnet" => Via::Magnet,
                    v => bail!("--via https|torrent|magnet, not {v}"),
                })
            }
            "--magnet" => o.magnet = Some(val()?),
            "--archive" => o.archive = Some(PathBuf::from(val()?)),
            "--manifest" => manifest = Some(PathBuf::from(val()?)),
            "--base" => o.base = Some(val()?),
            "--keep-archive" => o.keep_archive = true,
            "--page" => page = true,
            _ if a.starts_with("--") => bail!("unknown option {a}\n{USAGE}"),
            _ => pos.push(a.to_string()),
        }
        i += 1;
    }
    let release = match pos.first() {
        Some(p) if p.chars().next().is_some_and(|c| c.is_ascii_digit()) => pos.remove(0),
        _ => "0.5".to_string(),
    };
    let text = match &manifest {
        Some(p) => std::fs::read_to_string(p).with_context(|| format!("reading {}", p.display()))?,
        None if release == "0.5" => MANIFEST_0_5.to_string(),
        None => bail!("no manifest of release {release} is built in; give --manifest releases/{release}-files.json"),
    };
    let r = ReleaseFiles::parse(&text)?;
    ensure!(r.release == release, "the manifest is of release {}, not {release}", r.release);
    if page {
        print!("{}", render_bone(&r));
        return Ok(());
    }
    let mut files = r.select(&pos)?;
    if let Some(l) = &o.magnet {
        let m = Magnet::parse(l)?;
        let f = r.files.iter().find(|f| f.btih == m.btih).with_context(|| format!("no file of release {release} has info hash {}", m.btih))?;
        files = vec![f];
    }
    ensure!(o.archive.is_none() || files.len() == 1, "--archive goes with one file");
    let data = crate::data_dir();
    for f in files {
        eprintln!("mova fetch: {} ({}, {})", f.name, f.licence, if f.required { "required" } else { "optional" });
        for l in fetch_one(&r, f, &o, &data)? {
            println!("{l}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small release in a scratch folder: an archive `x-0.5.tar.gz` with `x-0.5/a.txt` and `x-0.5/d/b.txt`, its torrent and the
    /// manifest pointing at `https://example.org/0.5/` (rebased to the folder with `--base file://…`).
    fn release(d: &Path, kind: &str, licence: &str) -> (ReleaseFiles, PathBuf) {
        let srv = d.join("srv");
        std::fs::create_dir_all(srv.join("x-0.5")).unwrap();
        std::fs::write(srv.join("x-0.5/a.txt"), "hello\n").unwrap();
        std::fs::create_dir_all(srv.join("x-0.5/d")).unwrap();
        std::fs::write(srv.join("x-0.5/d/b.txt"), "world\n").unwrap();
        let st = std::process::Command::new("tar").args(["-czf", "x-0.5.tar.gz", "x-0.5"]).current_dir(&srv).status().unwrap();
        assert!(st.success());
        let arc = srv.join("x-0.5.tar.gz");
        let base = "https://example.org/0.5/".to_string();
        let tr = vec!["udp://tracker.example.org:1337/announce".to_string()];
        let (url, turl) = (format!("{base}x-0.5.tar.gz"), format!("{base}x-0.5.tar.gz.torrent"));
        let t = torrent::single_file(&arc, "x-0.5.tar.gz", &tr, std::slice::from_ref(&url), std::slice::from_ref(&turl), "").unwrap();
        std::fs::write(srv.join("x-0.5.tar.gz.torrent"), &t.bytes).unwrap();
        let f = ReleaseFile {
            id: "x".into(),
            name: "x-0.5.tar.gz".into(),
            kind: kind.into(),
            required: kind == "model",
            what: "a test file".into(),
            bytes: t.total,
            raw_bytes: 12,
            sha256: sha256_file(&arc).unwrap(),
            licence: licence.into(),
            licence_file: "LICENSE-x.txt".into(),
            url,
            torrent_url: turl,
            btih: t.info_hash.clone(),
            magnet: t.magnet.clone(),
            top: "x-0.5".into(),
            unpack_to: "release/0.5".into(),
            install: vec![Install { from: "a.txt".into(), to: "x/a.txt".into() }, Install { from: "d".into(), to: "x/d".into() }],
            contents: vec![
                Content { path: "a.txt".into(), bytes: 6, sha256: crate::sha256_hex(b"hello\n") },
                Content { path: "d/b.txt".into(), bytes: 6, sha256: crate::sha256_hex(b"world\n") },
            ],
        };
        let r = ReleaseFiles { schema: 1, release: "0.5".into(), date: "2026-11-05".into(), status: "draft".into(), base_url: base, page: "https://example.org/".into(), trackers: tr, files: vec![f] };
        (r, srv)
    }

    #[test]
    fn the_shipped_manifest_follows_the_policy_and_the_bone_agrees() {
        let r = ReleaseFiles::parse(MANIFEST_0_5).unwrap();
        let bone = include_str!("../../../releases/0.5-files.md");
        assert_eq!(render_bone(&r), bone, "releases/0.5-files.md is not the rendering of releases/0.5-files.json");
        assert!(r.files.iter().any(|f| f.required && f.kind == "model"));
        // the school archive (tens of thousands of lessons) is rendered per top folder; every file stays in the JSON
        let school = r.files.iter().find(|f| f.kind == "school").expect("the school archive is in the 0.5 manifest");
        assert!(school.contents.len() > RENDER_CONTENTS_MAX && bone.contains(&format!("{} files inside", school.contents.len())));
        // files at the archive's top are rows of their own; a file inside a folder is not
        let inner = school.contents.iter().find(|c| c.path.contains('/')).expect("the school has folders");
        assert!(!bone.contains(&format!("| `{}` |", inner.path)), "{} is listed one by one", inner.path);
        let mut bad = r.clone();
        if let Some(f) = bad.files.iter_mut().find(|f| f.kind == "school") {
            f.licence = OURS.into();
        }
        assert!(bad.check().is_err(), "the school under the code licence must be refused");
        // negative controls: a wrong licence row, a model marked optional, a changed sha in the bone
        let mut bad = r.clone();
        if let Some(f) = bad.files.iter_mut().find(|f| f.kind == "model") {
            f.licence = OURS.into();
        }
        assert!(bad.check().is_err(), "a model under our licence must be refused");
        let mut bad = r.clone();
        if let Some(f) = bad.files.iter_mut().find(|f| f.kind == "dictionaries") {
            f.licence = OURS.into();
            assert!(bad.check().is_err(), "dictionaries under the code licence must be refused");
        }
        let mut bad = r.clone();
        if let Some(f) = bad.files.iter_mut().find(|f| f.kind == "model") {
            f.sha256.replace_range(0..1, if f.sha256.starts_with('0') { "1" } else { "0" });
        }
        assert_ne!(render_bone(&bad), bone);
    }

    #[test]
    fn policy_rows_go_red() {
        let d = crate::tests::tmp("fetch-policy");
        let (r, _) = release(&d, "corpus", "CC-BY-SA-4.0");
        let mut r2 = r.clone();
        r2.files[0].required = false;
        r2.check().unwrap();
        for (kind, licence, why) in [("corpus", OURS, "corpus under our licence"), ("dictionaries", OURS, "dictionaries under the code licence"), ("module", OURS, "a module under our licence"), ("model", "CC-BY-SA-4.0", "an optional model")] {
            let mut b = r2.clone();
            b.files[0].kind = kind.into();
            b.files[0].licence = licence.into();
            assert!(b.check().is_err(), "{why} must be refused");
        }
        let mut b = r2.clone();
        b.files[0].contents[0].path = "wiktionary.tsv".into();
        b.files[0].install.clear();
        assert!(b.check().is_err(), "Wiktionary data outside a module");
        let mut b = r2.clone();
        b.files[0].magnet = b.files[0].magnet.replace("&xs=", "&xx=");
        assert!(b.check().is_err(), "a magnet link without its xs source");
    }

    #[test]
    fn fetch_by_each_way_installs_and_a_tampered_byte_is_refused() {
        let d = crate::tests::tmp("fetch");
        let (mut r, srv) = release(&d, "model", "CC-BY-SA-4.0");
        r.check().unwrap();
        let base = format!("file://{}/", srv.display());
        for (n, via) in [Via::Https, Via::Torrent, Via::Magnet].into_iter().enumerate() {
            let data = d.join(format!("data{n}"));
            let o = FetchOptions { via: Some(via), base: Some(base.clone()), ..Default::default() };
            let rep = fetch_one(&r, &r.files[0], &o, &data).unwrap();
            assert_eq!(std::fs::read_to_string(data.join("x/a.txt")).unwrap(), "hello\n", "{via:?}: {rep:?}");
            assert!(!data.join("release/0.5/x-0.5.tar.gz.part").exists());
            // again: nothing to do
            let rep = fetch_one(&r, &r.files[0], &o, &data).unwrap();
            assert!(rep.iter().any(|l| l.contains("up to date")), "{rep:?}");
        }
        // an earlier different file is kept aside, never deleted
        let data = d.join("data-old");
        std::fs::create_dir_all(data.join("x")).unwrap();
        std::fs::write(data.join("x/a.txt"), "old\n").unwrap();
        fetch_one(&r, &r.files[0], &FetchOptions { base: Some(base.clone()), ..Default::default() }, &data).unwrap();
        assert_eq!(std::fs::read_to_string(data.join("x/a.txt.before-0.5")).unwrap(), "old\n");
        // negative controls: one byte of the served archive changed → refused by every way, nothing installed
        let arc = srv.join("x-0.5.tar.gz");
        let good = std::fs::read(&arc).unwrap();
        let mut bad = good.clone();
        let k = bad.len() / 2;
        bad[k] ^= 0x40;
        std::fs::write(&arc, &bad).unwrap();
        for (n, via) in [Via::Https, Via::Torrent, Via::Magnet].into_iter().enumerate() {
            let data = d.join(format!("bad{n}"));
            let e = fetch_one(&r, &r.files[0], &FetchOptions { via: Some(via), base: Some(base.clone()), ..Default::default() }, &data).unwrap_err();
            assert!(!data.join("x/a.txt").exists(), "{via:?}: installed despite {e:#}");
            assert!(!data.join("release/0.5/x-0.5.tar.gz.part").exists());
        }
        std::fs::write(&arc, &good).unwrap();
        // a manifest sha that does not match the (good) archive is refused too
        r.files[0].sha256 = "0".repeat(64);
        assert!(fetch_one(&r, &r.files[0], &FetchOptions { base: Some(base.clone()), ..Default::default() }, &d.join("bad-sum")).is_err());
        // a magnet link of another torrent is refused
        let (r, _) = release(&d.join("again"), "model", "CC-BY-SA-4.0");
        let other = Magnet { btih: "1".repeat(40), ..Magnet::parse(&r.files[0].magnet).unwrap() }.link();
        let o = FetchOptions { magnet: Some(other), base: Some(base), ..Default::default() };
        assert!(fetch_one(&r, &r.files[0], &o, &d.join("bad-magnet")).is_err());
    }

    /// A cached extraction whose stamp still names the archive but whose bytes changed (same size or not), lost a
    /// file or gained one (also inside a copied directory) is not reused: the verified archive is unpacked again and
    /// only manifest bytes are installed. An archive that itself holds an unlisted file is refused.
    #[test]
    fn a_tampered_cached_extraction_is_unpacked_again_never_installed() {
        let d = crate::tests::tmp("fetch-cache");
        let (r, srv) = release(&d, "model", "CC-BY-SA-4.0");
        let o = FetchOptions { base: Some(format!("file://{}/", srv.display())), ..Default::default() };
        let data = d.join("data");
        fetch_one(&r, &r.files[0], &o, &data).unwrap();
        let cache = data.join("release/0.5/x-0.5");
        let rep = fetch_one(&r, &r.files[0], &o, &data).unwrap();
        assert!(rep.iter().any(|l| l.contains("inventory checked")), "an intact cache is reused: {rep:?}");
        type Tamper = fn(&Path);
        let cases: [(&str, Tamper, &str); 5] = [
            ("changed bytes, same size", |c| std::fs::write(c.join("a.txt"), "HELLO\n").unwrap(), "x/a.txt"),
            ("changed size", |c| std::fs::write(c.join("a.txt"), "hello, world\n").unwrap(), "x/a.txt"),
            ("missing file", |c| std::fs::remove_file(c.join("d/b.txt")).unwrap(), "x/d"),
            ("extra file in a copied directory", |c| std::fs::write(c.join("d/evil.txt"), "evil\n").unwrap(), "x/d"),
            ("extra top-level file", |c| std::fs::write(c.join("evil.txt"), "evil\n").unwrap(), "x/a.txt"),
        ];
        for (why, tamper, installed) in cases {
            tamper(&cache);
            // remove the installed copy, so trusting the cache would install its bytes
            let p = data.join(installed);
            if p.is_dir() { std::fs::remove_dir_all(&p).unwrap() } else { std::fs::remove_file(&p).unwrap() }
            let rep = fetch_one(&r, &r.files[0], &o, &data).unwrap();
            assert!(rep.iter().any(|l| l.contains("fails its inventory check")), "{why}: {rep:?}");
            assert_eq!(std::fs::read_to_string(data.join("x/a.txt")).unwrap(), "hello\n", "{why}");
            assert_eq!(std::fs::read_to_string(data.join("x/d/b.txt")).unwrap(), "world\n", "{why}");
            assert!(!data.join("x/d/evil.txt").exists(), "{why}: an unlisted file was installed");
            check_inventory(&cache, &r.files[0]).unwrap();
        }
        // the tampered caches were set aside, never deleted
        assert!(data.join("release/0.5/x-0.5.before-0.5").is_dir());
        // negative control of the check itself: it goes red on each tampering
        for tamper in [|c: &Path| std::fs::write(c.join("a.txt"), "HELLO\n").unwrap(), |c: &Path| std::fs::write(c.join("d/x"), "").unwrap()] {
            let c2 = d.join("probe");
            copy_tree(&cache, &c2).unwrap();
            check_inventory(&c2, &r.files[0]).unwrap();
            tamper(&c2);
            assert!(check_inventory(&c2, &r.files[0]).is_err());
            std::fs::remove_dir_all(&c2).unwrap();
        }
        // an archive holding a file its manifest does not list is refused on a fresh unpack
        let mut short = r.clone();
        short.files[0].contents.retain(|c| c.path == "a.txt");
        short.files[0].install.retain(|i| i.from == "a.txt");
        short.files[0].raw_bytes = 6;
        let bad = d.join("data-short");
        let e = fetch_one(&short, &short.files[0], &o, &bad).unwrap_err();
        assert!(format!("{e:#}").contains("does not list"), "{e:#}");
        assert!(!bad.join("x/a.txt").exists() && !bad.join("release/0.5/x-0.5").exists());
    }
}
