//! Compressed raw page store: zstd frames appended to segments and a JSONL journal.
//!
//! Layout of a store directory:
//! - `seg/<k>.seg` — raw bodies as zstd frames (with the frame checksum) one after another, only
//!   appended; a new segment starts once the current one has reached the size cap.
//! - `pages.jsonl` — the journal, one JSON record per line, only appended (`Record`).
//! - `links.jsonl` — outgoing links of fetched HTML pages, one line per page (`LinksLine`).
//!
//! The layout follows the family Spider's store v2 (`spider/src/store.rs`); its per-domain
//! dictionaries, deltas, block store and dump are not carried over. Identical bodies are stored
//! once: a record whose sha256 is already stored points at the existing frame.
//!
//! Crash safety (fsync policy): a body frame is written and its segment `fsync`ed before the
//! journal line that points at it is appended; every journal line is `fsync`ed before the next
//! request; directories are `fsync`ed when a file is created. A crash therefore leaves at most
//! unreferenced bytes at a segment end (harmless) and a torn last journal line, which the writer
//! cuts on open. A fetch's intent line precedes its request: an intent with no later outcome is an
//! uncertain fetch and is recorded as such when the store is next opened for writing.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

/// Journal record version.
pub const VERSION: u32 = 1;

/// One redirect hop.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Hop {
    pub url: String,
    pub status: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// The robots.txt decision behind a record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RobotsNote {
    pub robots_url: String,
    /// "parsed", "allow_all (HTTP 404)", "unreachable (...)".
    pub source: String,
    /// "own", "*" or "none".
    pub group: String,
    pub allowed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crawl_delay: Option<f64>,
}

/// One journal line. `kind`: "page", "robots", "intent", "uncertain".
/// `outcome` of a page: "fetched", "disallowed", "deferred", "error".
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct Record {
    pub v: u32,
    pub kind: String,
    pub url: String,
    pub outcome: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub final_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub chain: Vec<Hop>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    pub fetched_unix: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seg: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub len: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub truncated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub robots: Option<RobotsNote>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub depth: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub user_agent: String,
}

impl Record {
    pub fn has_body(&self) -> bool {
        self.seg.is_some() && self.off.is_some() && self.len.is_some() && self.sha256.is_some()
    }

    /// A page record that ends a fetch attempt (an intent is closed by it).
    pub fn is_completion(&self) -> bool {
        self.kind == "page"
    }
}

/// Outgoing links of one fetched page.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LinksLine {
    pub url: String,
    pub final_url: String,
    pub sha256: String,
    /// (absolute URL, nofollow)
    pub links: Vec<(String, bool)>,
}

/// Where a body lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Loc {
    pub seg: u32,
    pub off: u64,
    pub len: u64,
    pub body_bytes: u64,
}

/// Records in journal order plus indexes by URL and by sha256.
#[derive(Default)]
pub struct Index {
    pub recs: Vec<Record>,
    by_url: BTreeMap<String, Vec<usize>>,
    by_sha: BTreeMap<String, usize>,
    /// Complete lines that did not parse.
    pub bad_lines: u64,
}

impl Index {
    fn add(&mut self, r: Record) {
        let i = self.recs.len();
        self.by_url.entry(r.url.clone()).or_default().push(i);
        if let Some(f) = &r.final_url {
            if f != &r.url {
                self.by_url.entry(f.clone()).or_default().push(i);
            }
        }
        if r.has_body() {
            self.by_sha.entry(r.sha256.clone().unwrap_or_default()).or_insert(i);
        }
        self.recs.push(r);
    }

    fn load(text: &str) -> Index {
        let mut ix = Index::default();
        for line in text.split_inclusive('\n') {
            // a torn last line (no newline) is not a record
            if !line.ends_with('\n') {
                break;
            }
            match serde_json::from_str::<Record>(line.trim_end()) {
                Ok(r) => ix.add(r),
                Err(_) => ix.bad_lines += 1,
            }
        }
        ix
    }

    /// The latest record with a body for a URL (requested or final), journal order.
    pub fn latest_body(&self, url: &str) -> Option<&Record> {
        self.by_url.get(url)?.iter().rev().map(|&i| &self.recs[i]).find(|r| r.has_body())
    }

    pub fn by_sha(&self, sha: &str) -> Option<&Record> {
        self.by_sha.get(sha).map(|&i| &self.recs[i])
    }

    /// The latest page record of a requested URL.
    pub fn latest_page(&self, url: &str) -> Option<&Record> {
        self.by_url.get(url)?.iter().rev().map(|&i| &self.recs[i]).find(|r| r.kind == "page" && r.url == url)
    }

    /// Intents with no later completion or uncertain line for the same URL, journal order.
    pub fn dangling_intents(&self) -> Vec<&Record> {
        let mut open: BTreeMap<&str, usize> = BTreeMap::new();
        for (i, r) in self.recs.iter().enumerate() {
            match r.kind.as_str() {
                "intent" => {
                    open.insert(&r.url, i);
                }
                "page" | "uncertain" => {
                    open.remove(r.url.as_str());
                }
                _ => {}
            }
        }
        let mut v: Vec<usize> = open.into_values().collect();
        v.sort_unstable();
        v.into_iter().map(|i| &self.recs[i]).collect()
    }
}

pub fn sha256_hex(b: &[u8]) -> String {
    let d = Sha256::digest(b);
    d.iter().map(|x| format!("{x:02x}")).collect()
}

fn seg_path(dir: &Path, k: u32) -> PathBuf {
    dir.join("seg").join(format!("{k}.seg"))
}

fn sync_dir(p: &Path) -> std::io::Result<()> {
    File::open(p)?.sync_all()
}

/// Read and decompress a body; the sha256 must match the record.
pub fn read_body(dir: &Path, r: &Record) -> Result<Vec<u8>, String> {
    let (Some(seg), Some(off), Some(len), Some(sha)) = (r.seg, r.off, r.len, r.sha256.as_ref()) else {
        return Err("record has no body".into());
    };
    let mut f = File::open(seg_path(dir, seg)).map_err(|e| format!("segment {seg}: {e}"))?;
    f.seek(SeekFrom::Start(off)).map_err(|e| e.to_string())?;
    let mut z = vec![0; len as usize];
    f.read_exact(&mut z).map_err(|e| format!("segment {seg} at {off}+{len}: {e}"))?;
    let cap = r.body_bytes.unwrap_or(0) as usize + 1;
    let body = zstd::bulk::Decompressor::new().and_then(|mut d| d.decompress(&z, cap)).map_err(|e| format!("zstd: {e}"))?;
    if &sha256_hex(&body) != sha {
        return Err("sha256 mismatch".into());
    }
    Ok(body)
}

/// Read-only view: `get`, `stats`.
pub struct Reader {
    pub dir: PathBuf,
    pub index: Index,
}

impl Reader {
    pub fn open(dir: &Path) -> Result<Reader, String> {
        let text = std::fs::read_to_string(dir.join("pages.jsonl")).map_err(|e| format!("{}: {e}", dir.join("pages.jsonl").display()))?;
        Ok(Reader { dir: dir.to_path_buf(), index: Index::load(&text) })
    }

    /// Body by URL (latest with a body; requested or final URL) or by `sha256:<hex>`.
    pub fn get(&self, key: &str) -> Result<(Record, Vec<u8>), String> {
        let rec = match key.strip_prefix("sha256:") {
            Some(h) => self.index.by_sha(&h.to_ascii_lowercase()),
            None => self.index.latest_body(key),
        }
        .ok_or_else(|| format!("no stored body for {key}"))?;
        Ok((rec.clone(), read_body(&self.dir, rec)?))
    }
}

/// The writer: exactly one per store directory, enforced by an exclusive lock on `lock`
/// (held for the writer's lifetime), so a second crawler on the same store refuses to start.
pub struct Store {
    _lock: File,
    dir: PathBuf,
    seg: u32,
    seg_len: u64,
    seg_file: Option<File>,
    seg_max: u64,
    level: i32,
    journal: File,
    links: Option<File>,
    pub index: Index,
    /// Bytes cut from a torn journal tail on open.
    pub repaired_bytes: u64,
    /// Uncertain lines written on open for intents of an interrupted run.
    pub uncertain_on_open: u64,
}

impl Store {
    pub fn open(dir: &Path, seg_max: u64, level: i32) -> Result<Store, String> {
        let io = |e: std::io::Error| format!("{}: {e}", dir.display());
        std::fs::create_dir_all(dir.join("seg")).map_err(io)?;
        let lock = OpenOptions::new().create(true).truncate(false).write(true).open(dir.join("lock")).map_err(io)?;
        match lock.try_lock() {
            Ok(()) => {}
            Err(std::fs::TryLockError::WouldBlock) => {
                return Err(format!("{}: another Sea Spider is writing this store (one crawler at a time)", dir.display()))
            }
            Err(std::fs::TryLockError::Error(e)) => return Err(io(e)),
        }
        let jpath = dir.join("pages.jsonl");
        let created = !jpath.exists();
        let mut journal = OpenOptions::new().create(true).read(true).append(true).open(&jpath).map_err(io)?;
        if created {
            sync_dir(dir).map_err(io)?;
            sync_dir(&dir.join("seg")).map_err(io)?;
        }
        let mut text = String::new();
        journal.read_to_string(&mut text).map_err(|e| format!("{}: {e}", jpath.display()))?;
        // cut a torn last line so the next append starts on a fresh line
        let keep = text.rfind('\n').map_or(0, |p| p + 1);
        let repaired_bytes = (text.len() - keep) as u64;
        if repaired_bytes > 0 {
            journal.set_len(keep as u64).map_err(io)?;
            journal.sync_all().map_err(io)?;
            text.truncate(keep);
        }
        let index = Index::load(&text);
        let max = std::fs::read_dir(dir.join("seg"))
            .map_err(io)?
            .filter_map(|e| e.ok()?.file_name().into_string().ok()?.strip_suffix(".seg")?.parse::<u32>().ok())
            .max();
        let (seg, seg_len) = match max {
            Some(k) => (k, std::fs::metadata(seg_path(dir, k)).map_err(io)?.len()),
            None => (1, 0),
        };
        let mut s = Store { _lock: lock, dir: dir.to_path_buf(), seg, seg_len, seg_file: None, seg_max, level, journal, links: None, index, repaired_bytes, uncertain_on_open: 0 };
        // uncertain fetches of an interrupted run: recorded once, then eligible again
        let dangling: Vec<Record> = s.index.dangling_intents().into_iter().cloned().collect();
        for i in dangling {
            let r = Record { v: VERSION, kind: "uncertain".into(), url: i.url.clone(), outcome: "uncertain".into(), fetched_unix: i.fetched_unix, depth: i.depth, parent: i.parent.clone(), error: Some("intent without outcome: the request may have reached the server".into()), ..Default::default() };
            s.append(&r)?;
            s.uncertain_on_open += 1;
        }
        Ok(s)
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Store a body (deduplicated by sha256): returns its location and hash.
    pub fn put_body(&mut self, body: &[u8]) -> Result<(Loc, String), String> {
        let sha = sha256_hex(body);
        if let Some(r) = self.index.by_sha(&sha) {
            if let (Some(seg), Some(off), Some(len)) = (r.seg, r.off, r.len) {
                return Ok((Loc { seg, off, len, body_bytes: body.len() as u64 }, sha));
            }
        }
        let mut c = zstd::bulk::Compressor::new(self.level).map_err(|e| e.to_string())?;
        c.include_checksum(true).map_err(|e| e.to_string())?;
        let z = c.compress(body).map_err(|e| e.to_string())?;
        if self.seg_len >= self.seg_max {
            self.seg += 1;
            self.seg_len = 0;
            self.seg_file = None;
        }
        if self.seg_file.is_none() {
            let p = seg_path(&self.dir, self.seg);
            let created = !p.exists();
            self.seg_file = Some(OpenOptions::new().create(true).append(true).open(&p).map_err(|e| format!("{}: {e}", p.display()))?);
            if created {
                sync_dir(&self.dir.join("seg")).map_err(|e| e.to_string())?;
            }
        }
        let f = self.seg_file.as_mut().expect("segment open");
        f.write_all(&z).map_err(|e| e.to_string())?;
        f.sync_data().map_err(|e| e.to_string())?;
        let loc = Loc { seg: self.seg, off: self.seg_len, len: z.len() as u64, body_bytes: body.len() as u64 };
        self.seg_len += z.len() as u64;
        Ok((loc, sha))
    }

    /// Append one journal line and fsync it.
    pub fn append(&mut self, r: &Record) -> Result<(), String> {
        let mut line = serde_json::to_string(r).map_err(|e| e.to_string())?;
        line.push('\n');
        self.journal.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
        self.journal.sync_data().map_err(|e| e.to_string())?;
        self.index.add(r.clone());
        Ok(())
    }

    /// Append one links line and fsync it.
    pub fn append_links(&mut self, l: &LinksLine) -> Result<(), String> {
        if self.links.is_none() {
            let p = self.dir.join("links.jsonl");
            let created = !p.exists();
            self.links = Some(OpenOptions::new().create(true).append(true).open(&p).map_err(|e| format!("{}: {e}", p.display()))?);
            if created {
                sync_dir(&self.dir).map_err(|e| e.to_string())?;
            }
        }
        let f = self.links.as_mut().expect("links open");
        let mut line = serde_json::to_string(l).map_err(|e| e.to_string())?;
        line.push('\n');
        f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
        f.sync_data().map_err(|e| e.to_string())
    }

    /// Is the URL done: a page record other than an error, younger than `refetch_after`?
    pub fn done(&self, url: &str, now_unix: u64, refetch_after: Option<u64>) -> bool {
        match self.index.latest_page(url) {
            Some(r) if r.outcome != "error" => refetch_after.is_none_or(|a| now_unix.saturating_sub(r.fetched_unix) < a),
            _ => false,
        }
    }
}

/// Counts for `seaspider stats`.
pub fn stats(r: &Reader) -> serde_json::Value {
    let mut kinds: BTreeMap<String, u64> = BTreeMap::new();
    let mut outcomes: BTreeMap<String, u64> = BTreeMap::new();
    let mut statuses: BTreeMap<String, u64> = BTreeMap::new();
    let mut hosts: BTreeMap<String, u64> = BTreeMap::new();
    let (mut raw, mut stored) = (0u64, 0u64);
    let mut frames: BTreeMap<(u32, u64), ()> = BTreeMap::new();
    for rec in &r.index.recs {
        *kinds.entry(rec.kind.clone()).or_default() += 1;
        if rec.kind == "page" {
            *outcomes.entry(rec.outcome.clone()).or_default() += 1;
            if let Some(s) = rec.status {
                *statuses.entry(s.to_string()).or_default() += 1;
            }
            if let Some(h) = reqwest::Url::parse(&rec.url).ok().and_then(|u| u.host_str().map(str::to_string)) {
                *hosts.entry(h).or_default() += 1;
            }
        }
        if rec.has_body() {
            raw += rec.body_bytes.unwrap_or(0);
            if frames.insert((rec.seg.unwrap_or(0), rec.off.unwrap_or(0)), ()).is_none() {
                stored += rec.len.unwrap_or(0);
            }
        }
    }
    let segs: u64 = std::fs::read_dir(r.dir.join("seg")).map(|d| d.filter_map(|e| e.ok()).filter_map(|e| e.metadata().ok()).map(|m| m.len()).sum()).unwrap_or(0);
    serde_json::json!({
        "records": r.index.recs.len(),
        "bad_lines": r.index.bad_lines,
        "kinds": kinds,
        "page_outcomes": outcomes,
        "page_statuses": statuses,
        "hosts": hosts.len(),
        "raw_body_bytes": raw,
        "unique_frames": frames.len(),
        "frame_bytes": stored,
        "segment_bytes": segs,
        "uncertain_open": r.index.dangling_intents().len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("seaspider-store-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        p
    }

    fn page(url: &str, loc: Loc, sha: &str) -> Record {
        Record { v: VERSION, kind: "page".into(), url: url.into(), outcome: "fetched".into(), status: Some(200), fetched_unix: 100, seg: Some(loc.seg), off: Some(loc.off), len: Some(loc.len), body_bytes: Some(loc.body_bytes), sha256: Some(sha.into()), ..Default::default() }
    }

    #[test]
    fn round_trip_dedup_and_reopen() {
        let d = tmp("rt");
        let mut s = Store::open(&d, 1 << 20, 3).unwrap();
        let (a, sa) = s.put_body(b"<html>one</html>").unwrap();
        s.append(&page("http://h/1", a, &sa)).unwrap();
        let (b, sb) = s.put_body(b"<html>one</html>").unwrap();
        assert_eq!((a, &sa), (b, &sb), "identical body stored once");
        s.append(&page("http://h/2", b, &sb)).unwrap();
        drop(s);
        let r = Reader::open(&d).unwrap();
        assert_eq!(r.get("http://h/2").unwrap().1, b"<html>one</html>");
        assert_eq!(r.get(&format!("sha256:{sa}")).unwrap().1, b"<html>one</html>");
        assert!(r.get("http://h/3").is_err());
        let st = stats(&r);
        assert_eq!(st["unique_frames"], 1);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn segments_roll_over_at_cap() {
        let d = tmp("roll");
        let mut s = Store::open(&d, 1 << 20, 1).unwrap();
        // incompressible-ish bodies of ~600 KiB each: the second frame crosses the 1 MiB cap
        let mut x: u64 = 1;
        let mut segs = Vec::new();
        for k in 0..3u8 {
            let body: Vec<u8> = (0..600_000)
                .map(|_| {
                    x = x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407 + k as u64);
                    (x >> 33) as u8
                })
                .collect();
            let (l, sha) = s.put_body(&body).unwrap();
            s.append(&page(&format!("http://h/{k}"), l, &sha)).unwrap();
            segs.push(l.seg);
        }
        assert_eq!(segs, vec![1, 1, 2]);
        drop(s);
        let r = Reader::open(&d).unwrap();
        assert_eq!(r.get("http://h/2").unwrap().1.len(), 600_000);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn torn_tail_is_cut_and_appends_stay_parseable() {
        let d = tmp("torn");
        let mut s = Store::open(&d, 1 << 20, 3).unwrap();
        let (a, sa) = s.put_body(b"body").unwrap();
        s.append(&page("http://h/1", a, &sa)).unwrap();
        drop(s);
        // a crash mid-line: half a record, no newline; plus orphan bytes in the segment
        let mut j = OpenOptions::new().append(true).open(d.join("pages.jsonl")).unwrap();
        j.write_all(br#"{"v":1,"kind":"page","url":"http://h/2","outc"#).unwrap();
        let mut g = OpenOptions::new().append(true).open(d.join("seg/1.seg")).unwrap();
        g.write_all(b"orphan").unwrap();
        // negative control: the torn line is not a record and not a "bad line"
        assert_eq!(Reader::open(&d).unwrap().index.recs.len(), 1);
        let mut s = Store::open(&d, 1 << 20, 3).unwrap();
        assert!(s.repaired_bytes > 0);
        let (b, sb) = s.put_body(b"body two").unwrap();
        assert_eq!(b.off, a.len + 6, "new frame after the orphan bytes");
        s.append(&page("http://h/2", b, &sb)).unwrap();
        drop(s);
        let r = Reader::open(&d).unwrap();
        assert_eq!(r.index.bad_lines, 0);
        assert_eq!(r.get("http://h/1").unwrap().1, b"body");
        assert_eq!(r.get("http://h/2").unwrap().1, b"body two");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn corrupted_frame_detected() {
        let d = tmp("corrupt");
        let mut s = Store::open(&d, 1 << 20, 3).unwrap();
        let body = b"<p>abc</p>".repeat(200);
        let (a, sa) = s.put_body(&body).unwrap();
        s.append(&page("http://h/1", a, &sa)).unwrap();
        drop(s);
        let p = d.join("seg/1.seg");
        let mut bytes = std::fs::read(&p).unwrap();
        let i = bytes.len() / 2;
        bytes[i] ^= 0x55;
        std::fs::write(&p, bytes).unwrap();
        assert!(Reader::open(&d).unwrap().get("http://h/1").is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn dangling_intent_becomes_uncertain_once() {
        let d = tmp("intent");
        let mut s = Store::open(&d, 1 << 20, 3).unwrap();
        let intent = |u: &str| Record { v: VERSION, kind: "intent".into(), url: u.into(), outcome: "intent".into(), fetched_unix: 5, ..Default::default() };
        s.append(&intent("http://h/a")).unwrap();
        s.append(&intent("http://h/b")).unwrap();
        let (l, sha) = s.put_body(b"b").unwrap();
        s.append(&page("http://h/b", l, &sha)).unwrap();
        drop(s);
        let s = Store::open(&d, 1 << 20, 3).unwrap();
        let unc: Vec<&Record> = s.index.recs.iter().filter(|r| r.kind == "uncertain").collect();
        assert_eq!(unc.len(), 1);
        assert_eq!(unc[0].url, "http://h/a");
        assert!(!s.done("http://h/a", 10, None), "uncertain is not done");
        assert!(s.done("http://h/b", 10, None));
        assert!(!s.done("http://h/b", 1000, Some(500)), "refetch age passed");
        drop(s);
        let s = Store::open(&d, 1 << 20, 3).unwrap();
        assert_eq!(s.index.recs.iter().filter(|r| r.kind == "uncertain").count(), 1, "recorded once");
        let _ = std::fs::remove_dir_all(&d);
    }
}

#[cfg(test)]
mod one_writer_tests {
    use super::*;
    #[test]
    fn a_second_writer_on_the_same_store_is_refused_until_the_first_closes() {
        let dir = std::env::temp_dir().join(format!("seaspider-lock-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let first = Store::open(&dir, 1 << 20, 3).unwrap();
        let second = Store::open(&dir, 1 << 20, 3);
        assert!(second.is_err(), "one crawler at a time");
        assert!(second.err().unwrap().contains("another Sea Spider"));
        drop(first);
        assert!(Store::open(&dir, 1 << 20, 3).is_ok());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
