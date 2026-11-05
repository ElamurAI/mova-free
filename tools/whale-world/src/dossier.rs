//! The dossier of one document (`mova-whale-dossier-v1`): every text written about it, epoch after
//! epoch, append-only.
//!
//! `<dossier_root>/<doc>/texts.jsonl` holds one row per text and `texts/<epoch>-<kind>-<n>.md` (or
//! `.md.zst`) the text. A row is never rewritten and a file never replaced: adding an existing id is
//! refused, and a file whose hash differs from its row is refused by every reader until its new
//! content is added again as a new text. Rows are selected by epoch and kind without reading any
//! text. Appends hold an exclusive lock (`lock`) and are fsynced.
use crate::config::Config;
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use topic_corpus::licence::{self, Class, Origin};

pub mod import;

pub const SCHEMA: &str = "mova-whale-dossier-v1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// The document's own text.
    Original,
    /// A text Mova wrote.
    Mova,
    /// A consultant's description or draft.
    Sol,
    /// An earlier description of ours on a site.
    SiteOld,
    /// A page found on the web (held by Sea Spider or a topic corpus).
    Web,
}

impl Kind {
    pub const ALL: [Kind; 5] = [Kind::Original, Kind::Mova, Kind::Sol, Kind::SiteOld, Kind::Web];
    pub fn name(self) -> &'static str {
        match self {
            Kind::Original => "original",
            Kind::Mova => "mova",
            Kind::Sol => "sol",
            Kind::SiteOld => "site-old",
            Kind::Web => "web",
        }
    }
    pub fn parse(s: &str) -> Result<Kind> {
        Kind::ALL.into_iter().find(|k| k.name() == s).ok_or_else(|| anyhow!("unknown text kind {s:?} (original, mova, sol, site-old, web)"))
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Citations {
    pub count: usize,
    pub unresolved: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextRow {
    pub id: String,
    pub doc: String,
    pub epoch: u64,
    pub kind: Kind,
    pub source: String,
    pub licence: String,
    pub licence_class: Class,
    pub public_ok: bool,
    pub sha256: String,
    pub words: usize,
    pub created_unix: u64,
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(default)]
    pub citations: Citations,
    /// The text's file, relative to the dossier directory.
    pub file: String,
}

/// A text to add.
pub struct NewText<'a> {
    pub epoch: u64,
    pub kind: Kind,
    pub text: &'a str,
    pub source: &'a str,
    /// The licence as stated; `None`: ours, internal.
    pub licence: Option<&'a str>,
    /// Library originals only: whether the text we hold is local-only.
    pub local_only: bool,
    pub inputs: Vec<String>,
    /// `None`: now.
    pub created_unix: Option<u64>,
}

/// Whether a text may feed a public output: only a library original under CC BY, CC0 or public
/// domain that is not local-only, and only when the deployment allows public originals. Mova and
/// consultant drafts, earlier descriptions and web pages never, whatever licence they state.
pub fn public_ok(kind: Kind, class: Class, local_only: bool, public_originals: bool) -> bool {
    public_originals && kind == Kind::Original && licence::public_admissible(Origin::Library, class, local_only)
}

/// A document id is a plain lowercase slug (a library id).
pub fn check_doc(doc: &str) -> Result<()> {
    if doc.is_empty() || doc.len() > 200 || doc.starts_with(['-', '.']) || !doc.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_' || c == '.') {
        bail!("document id must be a lowercase slug: {doc:?}");
    }
    Ok(())
}

pub fn sha(b: &[u8]) -> String {
    topic_corpus::corpus::sha(b)
}

/// Citation markers in a text: numeric (`[3]`, `[2, 5-7]`) and author–year (`(Smith et al., 2016)`,
/// `Smith (2016)`). A numeric marker is unresolved without a line starting `[n]` (a reference
/// entry); an author–year marker without a reference line (after a References/Bibliography heading,
/// or a line starting `[n]`) holding the same surname and year.
pub fn citations(text: &str) -> Citations {
    let lines: Vec<&str> = text.lines().collect();
    let mut entries: std::collections::BTreeSet<u64> = Default::default();
    let mut ref_lines: Vec<String> = vec![];
    let mut in_refs = false;
    for l in &lines {
        let t = l.trim();
        if t.starts_with('#') {
            let h = t.to_lowercase();
            in_refs = h.contains("reference") || h.contains("bibliograph") || h.contains("works cited");
            continue;
        }
        if let Some(rest) = t.strip_prefix('[') {
            if let Some((n, _)) = rest.split_once(']') {
                if let Ok(n) = n.trim().parse::<u64>() {
                    entries.insert(n);
                    ref_lines.push(t.to_string());
                    continue;
                }
            }
        }
        if in_refs && !t.is_empty() {
            ref_lines.push(t.to_string());
        }
    }
    let mut c = Citations::default();
    for l in &lines {
        let t = l.trim();
        if t.starts_with('#') || ref_lines.iter().any(|r| r == t) {
            continue;
        }
        let b = t.as_bytes();
        let mut i = 0;
        while i < b.len() {
            if b[i] == b'[' {
                if let Some(j) = t[i + 1..].find(']') {
                    let inner = &t[i + 1..i + 1 + j];
                    if !inner.is_empty() && inner.chars().all(|c| c.is_ascii_digit() || matches!(c, ',' | '-' | ' ' | '–')) {
                        for part in inner.split(',') {
                            let nums: Vec<u64> = part.split(['-', '–']).filter_map(|x| x.trim().parse().ok()).collect();
                            let range: Vec<u64> = match nums.as_slice() {
                                [a] => vec![*a],
                                [a, z] if z >= a && z - a < 100 => (*a..=*z).collect(),
                                _ => vec![],
                            };
                            for n in range {
                                c.count += 1;
                                if !entries.contains(&n) {
                                    c.unresolved += 1;
                                }
                            }
                        }
                    }
                    i += j + 1;
                    continue;
                }
            }
            if b[i] == b'(' {
                if let Some(j) = t[i + 1..].find(')') {
                    let inner = &t[i + 1..i + 1 + j];
                    for piece in inner.split(';') {
                        if let Some((name, year)) = author_year(piece, &t[..i]) {
                            c.count += 1;
                            if !ref_lines.iter().any(|r| r.contains(&name) && r.contains(&year)) {
                                c.unresolved += 1;
                            }
                        }
                    }
                    i += j + 1;
                    continue;
                }
            }
            i += 1;
        }
    }
    c
}

/// `Smith et al., 2016` / `Smith and Jones 2016` inside parentheses, or a bare `2016` after
/// `Smith et al.` before the parenthesis: the surname and the year.
fn author_year(piece: &str, before: &str) -> Option<(String, String)> {
    let p = piece.trim();
    let year: String = p.split(|c: char| !c.is_ascii_digit()).filter(|x| x.len() == 4).find(|y| y.starts_with("19") || y.starts_with("20"))?.to_string();
    let words: Vec<&str> = p.split(|c: char| c.is_whitespace() || c == ',').filter(|w| !w.is_empty()).collect();
    let name = words.iter().find(|w| w.chars().next().is_some_and(char::is_uppercase) && w.chars().count() >= 2 && w.chars().all(|c| c.is_alphabetic() || c == '-' || c == '\'')).map(|w| w.to_string());
    let name = match name {
        Some(n) => n,
        None if p.trim_end_matches(|c: char| c.is_ascii_lowercase()) == year => {
            let prev: Vec<&str> = before.split_whitespace().rev().take(3).collect();
            prev.into_iter().find(|w| w.chars().next().is_some_and(char::is_uppercase) && w.chars().all(|c| c.is_alphabetic() || c == '-')).map(str::to_string)?
        }
        None => return None,
    };
    Some((name, year))
}

pub struct Dossier {
    pub dir: PathBuf,
    pub doc: String,
    public_originals: bool,
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

struct Lock(std::fs::File);
impl Lock {
    fn take(p: &Path) -> Result<Lock> {
        let f = std::fs::OpenOptions::new().create(true).truncate(false).write(true).mode(0o600).open(p)?;
        if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX) } != 0 {
            bail!("lock {}: {}", p.display(), std::io::Error::last_os_error());
        }
        Ok(Lock(f))
    }
}
impl Drop for Lock {
    fn drop(&mut self) {
        unsafe { libc::flock(self.0.as_raw_fd(), libc::LOCK_UN) };
    }
}

impl Dossier {
    /// The dossier directory of `doc` (created, private).
    pub fn open(cfg: &Config, doc: &str) -> Result<Dossier> {
        check_doc(doc)?;
        let dir = cfg.dossier_root.join(doc);
        for d in [cfg.dossier_root.clone(), dir.clone(), dir.join("texts")] {
            mova_whale::util::private_dir(&d).map_err(|e| anyhow!("{e}"))?;
        }
        Ok(Dossier { dir, doc: doc.into(), public_originals: cfg.public_originals })
    }

    /// Whether a dossier exists for `doc` (nothing is created).
    pub fn exists(cfg: &Config, doc: &str) -> bool {
        check_doc(doc).is_ok() && cfg.dossier_root.join(doc).join("texts.jsonl").is_file()
    }

    /// The documents with a dossier, sorted.
    pub fn list(cfg: &Config) -> Result<Vec<String>> {
        let mut v = vec![];
        if cfg.dossier_root.is_dir() {
            for e in std::fs::read_dir(&cfg.dossier_root)? {
                let n = e?.file_name().to_string_lossy().to_string();
                if Self::exists(cfg, &n) {
                    v.push(n);
                }
            }
        }
        v.sort();
        Ok(v)
    }

    /// Every row, in append order.
    pub fn rows(&self) -> Result<Vec<TextRow>> {
        let p = self.dir.join("texts.jsonl");
        if !p.exists() {
            return Ok(vec![]);
        }
        std::fs::read_to_string(&p)?.lines().filter(|l| !l.trim().is_empty()).map(|l| serde_json::from_str(l).with_context(|| format!("{}", p.display()))).collect()
    }

    /// Rows of one epoch and/or kind (no text is read).
    pub fn select(&self, epoch: Option<u64>, kind: Option<Kind>) -> Result<Vec<TextRow>> {
        Ok(self.rows()?.into_iter().filter(|r| epoch.is_none_or(|e| r.epoch == e) && kind.is_none_or(|k| r.kind == k)).collect())
    }

    /// The text of a row, hash-checked against the row.
    pub fn read(&self, r: &TextRow) -> Result<String> {
        let p = self.dir.join(&r.file);
        let b = std::fs::read(&p).with_context(|| format!("{}", p.display()))?;
        let b = if r.file.ends_with(".zst") { zstd::decode_all(&b[..])? } else { b };
        if sha(&b) != r.sha256 {
            bail!("{}: the file changed since it was registered (sha256 differs); add the new content as a new text", r.id);
        }
        Ok(String::from_utf8(b)?)
    }

    /// Check every file against its row; the ids of changed or missing files.
    pub fn verify(&self) -> Result<Vec<String>> {
        Ok(self.rows()?.iter().filter(|r| self.read(r).is_err()).map(|r| r.id.clone()).collect())
    }

    /// Append a text. Originals are stored zstd-compressed. Refused when the id or file exists.
    pub fn add(&self, t: NewText) -> Result<TextRow> {
        let _l = Lock::take(&self.dir.join("lock"))?;
        let rows = self.rows()?;
        let n = rows.iter().filter(|r| r.epoch == t.epoch && r.kind == t.kind).count() + 1;
        let stem = format!("{}-{}-{n}", t.epoch, t.kind.name());
        let id = format!("{}/{stem}", self.doc);
        if rows.iter().any(|r| r.id == id) {
            bail!("{id} exists: a dossier is append-only");
        }
        for i in &t.inputs {
            if !i.contains('/') {
                bail!("input {i:?} is not a text id (<doc>/<epoch>-<kind>-<n>)");
            }
        }
        let file = if t.kind == Kind::Original { format!("texts/{stem}.md.zst") } else { format!("texts/{stem}.md") };
        let body = if t.kind == Kind::Original { zstd::encode_all(t.text.as_bytes(), 9)? } else { t.text.as_bytes().to_vec() };
        let p = self.dir.join(&file);
        let mut f = std::fs::OpenOptions::new().create_new(true).write(true).mode(0o600).open(&p).with_context(|| format!("{} exists: a dossier is append-only", p.display()))?;
        f.write_all(&body)?;
        f.sync_all()?;
        let (lic, class) = match t.licence {
            Some(l) => (l.to_string(), licence::classify(l)),
            None => ("ours (internal)".to_string(), Class::Unknown),
        };
        let row = TextRow {
            id,
            doc: self.doc.clone(),
            epoch: t.epoch,
            kind: t.kind,
            source: t.source.into(),
            public_ok: public_ok(t.kind, class, t.local_only, self.public_originals),
            licence: lic,
            licence_class: class,
            sha256: sha(t.text.as_bytes()),
            words: t.text.split_whitespace().count(),
            created_unix: t.created_unix.unwrap_or_else(now),
            inputs: t.inputs,
            citations: citations(t.text),
            file,
        };
        let mut idx = std::fs::OpenOptions::new().create(true).append(true).mode(0o600).open(self.dir.join("texts.jsonl"))?;
        idx.write_all((serde_json::to_string(&row)? + "\n").as_bytes())?;
        idx.sync_all()?;
        Ok(row)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub fn tmp(name: &str) -> PathBuf {
        let d = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join(format!("whale-world-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    pub fn cfg(d: &Path) -> Config {
        Config {
            schema: crate::config::SCHEMA.into(),
            dossier_root: d.join("docs"),
            world_root: d.join("world"),
            library: None,
            corpora_root: None,
            site_old: None,
            mova_state_roots: vec![],
            mova_rounds: Default::default(),
            public_originals: true,
            limits: Default::default(),
        }
    }

    fn new<'a>(kind: Kind, text: &'a str, licence: Option<&'a str>) -> NewText<'a> {
        NewText { epoch: 0, kind, text, source: "test", licence, local_only: false, inputs: vec![], created_unix: Some(1) }
    }

    #[test]
    fn append_only_select_and_changed_files() {
        let d = tmp("dossier");
        let c = cfg(&d);
        let x = Dossier::open(&c, "smith-2020-trees").unwrap();
        let a = x.add(new(Kind::Original, "# T\n\nTrees are graphs.\n", Some("CC BY 4.0"))).unwrap();
        assert_eq!(a.id, "smith-2020-trees/0-original-1");
        assert!(a.file.ends_with(".md.zst") && a.public_ok);
        let b = x.add(NewText { epoch: 7, inputs: vec![a.id.clone()], ..new(Kind::Mova, "Mova wrote this.", None) }).unwrap();
        let b2 = x.add(NewText { epoch: 7, ..new(Kind::Mova, "Mova wrote this again.", None) }).unwrap();
        assert_eq!((b.id.as_str(), b2.id.as_str()), ("smith-2020-trees/7-mova-1", "smith-2020-trees/7-mova-2"));
        assert_eq!(x.select(Some(7), Some(Kind::Mova)).unwrap().len(), 2);
        assert_eq!(x.select(Some(0), None).unwrap().len(), 1);
        assert_eq!(x.read(&a).unwrap(), "# T\n\nTrees are graphs.\n");
        // A changed file is refused by every reader.
        std::fs::write(x.dir.join(&b.file), "edited").unwrap();
        assert!(x.read(&b).unwrap_err().to_string().contains("changed"));
        assert_eq!(x.verify().unwrap(), vec![b.id.clone()]);
        // An existing file is never replaced (simulate a stale file at the next id).
        std::fs::write(x.dir.join("texts/7-mova-3.md"), "stale").unwrap();
        assert!(x.add(NewText { epoch: 7, ..new(Kind::Mova, "third", None) }).is_err());
        assert_eq!(x.rows().unwrap().len(), 3);
        assert!(Dossier::open(&c, "Bad Id").is_err());
        assert!(Dossier::open(&c, "../x").is_err());
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn licence_negative_controls() {
        // Only an open-licensed original may be public, and only when allowed.
        for k in [Kind::Web, Kind::Sol, Kind::Mova, Kind::SiteOld] {
            for l in ["CC0 1.0", "CC BY 4.0", "CC BY-SA 4.0", "public domain"] {
                assert!(!public_ok(k, licence::classify(l), false, true), "{k:?} {l}");
            }
        }
        assert!(public_ok(Kind::Original, Class::By, false, true));
        assert!(!public_ok(Kind::Original, Class::By, false, false));
        assert!(!public_ok(Kind::Original, Class::By, true, true));
        assert!(!public_ok(Kind::Original, licence::classify("CC BY-SA 4.0"), false, true));
        assert!(!public_ok(Kind::Original, licence::classify("CC BY-NC 4.0"), false, true));
        // Through the store: a CC BY-SA original and a CC0 web page stay internal.
        let d = tmp("licence");
        let x = Dossier::open(&cfg(&d), "smith-2020-trees").unwrap();
        assert!(!x.add(new(Kind::Original, "sa", Some("CC BY-SA 4.0"))).unwrap().public_ok);
        assert!(!x.add(new(Kind::Web, "page", Some("CC0 1.0"))).unwrap().public_ok);
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn citation_markers() {
        let t = "# Paper\n\nAs shown [1, 3-4], and by Smith et al. (2016) and (Jones, 2019; Brown 2020).\n\n## References\n\n[1] A. 2001.\n[3] B. 2002.\nSmith, J. 2016. Trees.\nJones, K. 2019. Graphs.\n";
        let c = citations(t);
        // [1] [3] [4] Smith 2016, Jones 2019, Brown 2020; unresolved: [4] and Brown 2020.
        assert_eq!(c, Citations { count: 6, unresolved: 2 });
        assert_eq!(citations("No citations here (none at all)."), Citations::default());
    }
}
