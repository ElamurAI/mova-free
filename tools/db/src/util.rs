//! Small helpers: sha1, time without third-party crates, paths with "~" and a simple glob with "*".

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use sha1::{Digest, Sha1};

/// Treebank cards (`treebanks/<language>/<code>/README.md`): the generator (`tools/treebanks`) writes English
/// labels, older committed cards still have Ukrainian ones (written here with `\u{…}` escapes: "Size", "total",
/// "UD name", "License", "Genres", "Authors", "Treebank code"). Both are accepted; values go to the database
/// under the English keys.
pub const CARD_SIZE_HEADINGS: [&str; 2] = ["## Size", "## \u{41e}\u{431}\u{441}\u{44f}\u{433}"];
/// Row labels of the size table meaning the whole treebank → part name "total".
pub const CARD_TOTALS: [&str; 2] = ["total", "\u{440}\u{430}\u{437}\u{43e}\u{43c}"];
/// Card field label → key in `runs.params.fields`.
pub const CARD_FIELDS: [(&str, &str); 10] = [
    ("UD name", "ud_name"),
    ("License", "license"),
    ("Genres", "genres"),
    ("Authors", "authors"),
    ("Treebank code", "code"),
    ("\u{41d}\u{430}\u{437}\u{432}\u{430} UD", "ud_name"),
    ("\u{41b}\u{456}\u{446}\u{435}\u{43d}\u{437}\u{456}\u{44f}", "license"),
    ("\u{416}\u{430}\u{43d}\u{440}\u{438}", "genres"),
    ("\u{410}\u{432}\u{442}\u{43e}\u{440}\u{438}", "authors"),
    ("\u{41a}\u{43e}\u{434} \u{442}\u{440}\u{438}\u{431}\u{430}\u{43d}\u{43a}\u{443}", "code"),
];

/// Part name of a size-table row: "total" for either total label, otherwise as written.
pub fn card_part(label: &str) -> &str {
    if CARD_TOTALS.contains(&label) { "total" } else { label }
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
