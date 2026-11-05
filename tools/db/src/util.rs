//! Small helpers: sha1, time without third-party crates, paths with "~" and a simple glob with "*".

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use sha1::{Digest, Sha1};

/// Size columns of a treebank fact row: part → five numbers (sentences, tokens, words, multiword, empty).
pub const CARD_PARTS: [&str; 4] = ["train", "dev", "test", "total"];

/// Treebank facts `treebanks/cards.tsv` (written by `tools/treebanks`): a header of column names, one row per
/// treebank. Returns the first row satisfying `pred`, column → value.
pub fn card_row(tsv: &Path, pred: impl Fn(&BTreeMap<String, String>) -> bool) -> Result<BTreeMap<String, String>> {
    let text = std::fs::read_to_string(tsv).with_context(|| tsv.display().to_string())?;
    let mut lines = text.lines();
    let head: Vec<&str> = lines.next().unwrap_or("").split('\t').collect();
    if !head.contains(&"ud_name") || CARD_PARTS.iter().any(|p| !head.contains(p)) {
        bail!("{}: not a treebank cards table (no ud_name or size columns)", tsv.display());
    }
    for l in lines {
        let row: BTreeMap<String, String> = head.iter().zip(l.split('\t')).map(|(k, v)| (k.to_string(), v.to_string())).collect();
        if pred(&row) {
            return Ok(row);
        }
    }
    bail!("{}: no matching treebank row", tsv.display())
}

/// Sizes of a fact row: part → [sentences, tokens, words, multiword, empty]. An absent split is empty and left
/// out; a value that is not five numbers is an error, so a broken table cannot pass a size check.
pub fn card_volume(row: &BTreeMap<String, String>) -> Result<serde_json::Map<String, serde_json::Value>> {
    let mut out = serde_json::Map::new();
    for p in CARD_PARTS {
        let v = row.get(p).map_or("", String::as_str);
        if v.is_empty() {
            continue;
        }
        let nums: Vec<i64> = v.split_whitespace().map(str::parse).collect::<std::result::Result<_, _>>().with_context(|| format!("size {p} = {v:?}"))?;
        if nums.len() != 5 {
            bail!("size {p} = {v:?}: expected five numbers");
        }
        out.insert(p.to_string(), serde_json::json!(nums));
    }
    Ok(out)
}

/// A card reference stored in the database: `<path to cards.tsv>#<UD name>`. A reference to a per-treebank
/// `treebanks/<lang>/<code>/README.md` is looked up in the `cards.tsv` of that treebanks folder by language and code.
pub fn card_lookup(card: &str) -> Result<BTreeMap<String, String>> {
    if let Some((tsv, dir)) = card.rsplit_once('#') {
        return card_row(Path::new(tsv), |r| r.get("ud_name").is_some_and(|x| x == dir)).with_context(|| format!("treebank {dir}"));
    }
    let p = Path::new(card);
    let code = p.parent().and_then(Path::file_name).map(|x| x.to_string_lossy().into_owned()).unwrap_or_default();
    let lang_dir = p.parent().and_then(Path::parent);
    let lang = lang_dir.and_then(Path::file_name).map(|x| x.to_string_lossy().into_owned()).unwrap_or_default();
    let Some(root) = lang_dir.and_then(Path::parent).filter(|_| p.file_name().is_some_and(|n| n == "README.md")) else {
        bail!("{card}: not a treebank card reference");
    };
    card_row(&root.join("cards.tsv"), |r| r.get("lang") == Some(&lang) && r.get("treebank") == Some(&code))
}

pub fn sha1_hex(bytes: &[u8]) -> String {
    Sha1::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

/// UNIX seconds → "YYYY-MM-DD HH:MM:SS" (UTC), civil_from_days algorithm (Howard Hinnant).
pub fn ts(secs: i64) -> String {
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + (m <= 2) as i64;
    format!("{y:04}-{m:02}-{d:02} {:02}:{:02}:{:02}", rem / 3600, rem % 3600 / 60, rem % 60)
}

pub fn now_ts() -> String {
    ts(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0))
}

/// File modification time (UTC).
pub fn mtime_ts(p: &Path) -> Option<String> {
    let m = std::fs::metadata(p).ok()?.modified().ok()?;
    Some(ts(m.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64))
}

/// Date from the end of a run directory name: "silver-ud-2026-09-25" → "2026-09-25 00:00:00".
pub fn date_suffix(name: &str) -> Option<String> {
    let b = name.as_bytes();
    if b.len() < 10 {
        return None;
    }
    let tail = &name[name.len() - 10..];
    let ok = tail.char_indices().all(|(i, c)| if i == 4 || i == 7 { c == '-' } else { c.is_ascii_digit() });
    ok.then(|| format!("{tail} 00:00:00"))
}

pub fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/".into()))
}

/// «~/x» → «$HOME/x».
pub fn expand(p: &str) -> PathBuf {
    match p.strip_prefix("~/") {
        Some(rest) => home().join(rest),
        None if p == "~" => home(),
        None => PathBuf::from(p),
    }
}

/// Glob with "*" in any part of the path (no "**"); the result is sorted. A path without "*" is taken as is
/// if the file exists.
pub fn glob(pattern: &str) -> Result<Vec<PathBuf>> {
    let path = expand(pattern);
    let mut cur: Vec<PathBuf> = vec![if path.is_absolute() { PathBuf::from("/") } else { PathBuf::from(".") }];
    for comp in path.components() {
        let part = comp.as_os_str().to_string_lossy().into_owned();
        if part == "/" {
            continue;
        }
        let mut next = Vec::new();
        for base in &cur {
            if part.contains('*') {
                let Ok(rd) = std::fs::read_dir(base) else { continue };
                let mut names: Vec<String> = rd.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).filter(|n| wild(&part, n)).collect();
                names.sort();
                next.extend(names.into_iter().map(|n| base.join(n)));
            } else {
                let p = base.join(&part);
                if p.exists() {
                    next.push(p);
                }
            }
        }
        cur = next;
    }
    cur.retain(|p| p.is_file());
    cur.sort();
    Ok(cur)
}

/// Match a name against a pattern where "*" is any string.
fn wild(pat: &str, name: &str) -> bool {
    let parts: Vec<&str> = pat.split('*').collect();
    let mut rest = name;
    for (i, p) in parts.iter().enumerate() {
        if i == 0 {
            match rest.strip_prefix(p) {
                Some(r) => rest = r,
                None => return false,
            }
        } else if i == parts.len() - 1 {
            return rest.ends_with(p) && rest.len() >= p.len();
        } else {
            match rest.find(p) {
                Some(k) => rest = &rest[k + p.len()..],
                None => return false,
            }
        }
    }
    rest.is_empty()
}

/// Name of the directory containing the file.
pub fn parent_name(p: &Path) -> String {
    p.parent().and_then(|d| d.file_name()).map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
}

pub fn stem(p: &Path) -> String {
    p.file_stem().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
}

pub fn file_name(p: &Path) -> String {
    p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
}

pub fn abs(p: &Path) -> Result<PathBuf> {
    std::fs::canonicalize(p).with_context(|| format!("{}", p.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_and_wildcards() {
        assert_eq!(ts(0), "1970-01-01 00:00:00");
        assert_eq!(ts(1_778_096_578), "2026-05-06 19:42:58");
        assert_eq!(date_suffix("silver-ud-2026-09-25").as_deref(), Some("2026-09-25 00:00:00"));
        assert_eq!(date_suffix("calib-2026-09-25-high"), None);
        assert!(wild("calib-*", "calib-ud-2026-09-25"));
        assert!(wild("*.conllu", "a.conllu"));
        assert!(wild("UD_English-*", "UD_English-EWT"));
        assert!(!wild("UD_English-*", "UD_French-GSD"));
        assert!(!wild("a*b", "ab_"));
        assert!(wild("a*b*c", "a1b2c"));
        assert_eq!(sha1_hex(b"abc"), "a9993e364706816aba3e25717850c26c9cd0d89d");
    }
}
