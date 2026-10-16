//! UD release (`ud-treebanks-v2.NN.tgz`) as a stream: README, stats.xml and the size of each `.conllu`
//! per train/dev/test split. The archive is not unpacked to disk.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

use anyhow::{Context, Result, bail};

/// Size of a split: sentences, surface tokens, syntactic words, multiword tokens (ranges `1-2`),
/// empty nodes (`1.1`). Tokens = words − words under ranges + ranges (as in stats.xml).
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Counts {
    pub sentences: u64,
    pub tokens: u64,
    pub words: u64,
    pub fused: u64,
    pub empty: u64,
}

impl Counts {
    pub fn add(&mut self, o: &Counts) {
        self.sentences += o.sentences;
        self.tokens += o.tokens;
        self.words += o.words;
        self.fused += o.fused;
        self.empty += o.empty;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Split {
    Train,
    Dev,
    Test,
}

pub const SPLITS: [Split; 3] = [Split::Train, Split::Dev, Split::Test];

impl Split {
    pub fn name(self) -> &'static str {
        match self {
            Split::Train => "train",
            Split::Dev => "dev",
            Split::Test => "test",
        }
    }

    fn parse(s: &str) -> Option<Split> {
        SPLITS.into_iter().find(|x| x.name() == s)
    }
}

/// A release treebank: folder (`UD_French-GSD`), code from file names (`fr_gsd`), README, stats.xml, size.
#[derive(Default)]
pub struct Treebank {
    pub dir: String,
    pub code: String,
    pub readme_name: String,
    pub readme: String,
    pub stats_xml: String,
    pub splits: BTreeMap<Split, Counts>,
    pub files: Vec<String>,
}

impl Treebank {
    pub fn total(&self) -> Counts {
        let mut t = Counts::default();
        for c in self.splits.values() {
            t.add(c);
        }
        t
    }

    /// `fr_gsd` → (`fr`, `gsd`).
    pub fn lang_and_name(&self) -> (&str, &str) {
        self.code.split_once('_').unwrap_or((self.code.as_str(), ""))
    }
}

/// `fr_gsd-ud-train.conllu` → (`fr_gsd`, train).
fn conllu_name(name: &str) -> Option<(String, Split)> {
    let stem = name.strip_suffix(".conllu")?;
    let (code, split) = stem.rsplit_once("-ud-")?;
    Some((code.to_string(), Split::parse(split)?))
}

/// Size of one CoNLL-U as a stream (bytes; lines `\n` or `\r\n`).
pub fn count(r: impl BufRead) -> Result<Counts> {
    let mut r = r;
    let mut c = Counts::default();
    let (mut in_sent, mut under_ranges) = (false, 0u64);
    let mut line = Vec::with_capacity(4096);
    loop {
        line.clear();
        if r.read_until(b'\n', &mut line)? == 0 {
            break;
        }
        while matches!(line.last(), Some(b'\n' | b'\r')) {
            line.pop();
        }
        if line.is_empty() {
            if in_sent {
                c.sentences += 1;
                in_sent = false;
            }
            continue;
        }
        if line[0] == b'#' {
            continue;
        }
        in_sent = true;
        let id = &line[..line.iter().position(|&b| b == b'\t').unwrap_or(line.len())];
        if let Some(dash) = id.iter().position(|&b| b == b'-') {
            let num = |s: &[u8]| std::str::from_utf8(s).ok().and_then(|x| x.parse::<u64>().ok());
            let (Some(a), Some(b)) = (num(&id[..dash]), num(&id[dash + 1..])) else {
                bail!("id range is not numeric: {}", String::from_utf8_lossy(id));
            };
            if b < a {
                bail!("id range is reversed: {}", String::from_utf8_lossy(id));
            }
            c.fused += 1;
            under_ranges += b - a + 1;
        } else if id.contains(&b'.') {
            c.empty += 1;
        } else {
            c.words += 1;
        }
    }
    if in_sent {
        c.sentences += 1;
    }
    c.tokens = (c.words + c.fused).checked_sub(under_ranges).context("more ranges than words")?;
    Ok(c)
}

/// Release archive stream → (release folder, treebanks by folder name).
pub fn read(tgz: &Path) -> Result<(String, Vec<Treebank>)> {
    let gz = flate2::read::GzDecoder::new(BufReader::with_capacity(1 << 20, File::open(tgz).with_context(|| tgz.display().to_string())?));
    let mut ar = tar::Archive::new(gz);
    let mut by_dir: BTreeMap<String, Treebank> = BTreeMap::new();
    let mut root = String::new();
    for entry in ar.entries()? {
        let mut e = entry?;
        if !e.header().entry_type().is_file() {
            continue;
        }
        let path = e.path()?.to_string_lossy().into_owned();
        let parts: Vec<&str> = path.split('/').filter(|p| !p.is_empty() && *p != ".").collect();
        if parts.len() != 3 || !parts[1].starts_with("UD_") {
            continue;
        }
        if root.is_empty() {
            root = parts[0].to_string();
        }
        let (dir, name) = (parts[1].to_string(), parts[2].to_string());
        let tb = by_dir.entry(dir.clone()).or_insert_with(|| Treebank { dir, ..Default::default() });
        tb.files.push(name.clone());
        let lower = name.to_lowercase();
        if lower == "readme.md" || (lower == "readme.txt" && tb.readme_name.is_empty()) || (lower == "readme" && tb.readme_name.is_empty()) {
            let mut b = Vec::new();
            e.read_to_end(&mut b)?;
            tb.readme = String::from_utf8_lossy(&b).into_owned();
            tb.readme_name = name;
        } else if lower == "stats.xml" {
            let mut b = Vec::new();
            e.read_to_end(&mut b)?;
            tb.stats_xml = String::from_utf8_lossy(&b).into_owned();
        } else if let Some((code, split)) = conllu_name(&name) {
            if !tb.code.is_empty() && tb.code != code {
                bail!("{}: different codes in .conllu names — {} and {code}", tb.dir, tb.code);
            }
            tb.code = code;
            let c = count(BufReader::with_capacity(1 << 20, &mut e)).with_context(|| path.clone())?;
            if tb.splits.insert(split, c).is_some() {
                bail!("{}: two {} splits", tb.dir, split.name());
            }
        }
    }
    let mut out: Vec<Treebank> = by_dir.into_values().collect();
    let bad: Vec<&str> = out.iter().filter(|t| t.code.is_empty()).map(|t| t.dir.as_str()).collect();
    if !bad.is_empty() {
        bail!("treebanks without .conllu: {}", bad.join(", "));
    }
    for t in &mut out {
        t.files.sort();
    }
    Ok((root, out))
}

/// `<size>` from stats.xml: `total`, `train`, `dev`, `test` blocks with sentences, tokens, words, fused
/// (words and fused may be absent when there are no multiword tokens). A split absent from the release is written as zeros in stats.xml.
pub fn parse_stats(xml: &str) -> BTreeMap<&'static str, Counts> {
    let mut out = BTreeMap::new();
    let Some(size) = between(xml, "<size>", "</size>") else { return out };
    for name in ["total", "train", "dev", "test"] {
        let Some(block) = between(size, &format!("<{name}>"), &format!("</{name}>")) else { continue };
        let num = |tag: &str| between(block, &format!("<{tag}>"), &format!("</{tag}>")).and_then(|v| v.trim().parse::<u64>().ok());
        let tokens = num("tokens").unwrap_or(0);
        out.insert(name, Counts { sentences: num("sentences").unwrap_or(0), tokens, words: num("words").unwrap_or(tokens), fused: num("fused").unwrap_or(0), empty: 0 });
    }
    out
}

fn between<'a>(s: &'a str, a: &str, b: &str) -> Option<&'a str> {
    let i = s.find(a)? + a.len();
    let j = s[i..].find(b)? + i;
    Some(&s[i..j])
}

fn key(c: &Counts) -> (u64, u64, u64, u64) {
    (c.sentences, c.tokens, c.words, c.fused)
}

/// Gate: our streaming count against the release stats.xml (each split and the total). Empty means a match.
pub fn check_stats(t: &Treebank) -> Vec<String> {
    let stats = parse_stats(&t.stats_xml);
    let mut errs = Vec::new();
    if !stats.contains_key("total") {
        errs.push(format!("{}: stats.xml has no <size><total>", t.dir));
        return errs;
    }
    let mut check = |name: &str, ours: Counts, theirs: Option<&Counts>| {
        let theirs = theirs.copied().unwrap_or_default();
        if key(&ours) != key(&theirs) {
            let (a, b) = (key(&ours), key(&theirs));
            errs.push(format!("{} {name}: ours sentences/tokens/words/fused {}/{}/{}/{} ≠ stats.xml {}/{}/{}/{}", t.dir, a.0, a.1, a.2, a.3, b.0, b.1, b.2, b.3));
        }
    };
    for s in SPLITS {
        check(s.name(), t.splits.get(&s).copied().unwrap_or_default(), stats.get(s.name()));
    }
    check("total", t.total(), stats.get("total"));
    errs
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONLLU: &str = "# sent_id = 1\n# text = Vámonos al mar.\n1-2\tVámonos\t_\t_\t_\t_\t_\t_\t_\t_\n1\tVamos\tir\tVERB\t_\t_\t0\troot\t_\t_\n2\tnos\tnosotros\tPRON\t_\t_\t1\tobj\t_\t_\n3-4\tal\t_\t_\t_\t_\t_\t_\t_\t_\n3\ta\ta\tADP\t_\t_\t5\tcase\t_\t_\n4\tel\tel\tDET\t_\t_\t5\tdet\t_\t_\n5\tmar\tmar\tNOUN\t_\t_\t1\tobl\t_\t_\n5.1\tva\tir\tVERB\t_\t_\t_\t_\t1:conj\t_\n6\t.\t.\tPUNCT\t_\t_\t1\tpunct\t_\t_\n\r\n# sent_id = 2\n1\tSí\tsí\tINTJ\t_\t_\t0\troot\t_\t_\n";

    #[test]
    fn counts_multiword_and_empty_nodes() {
        let c = count(CONLLU.as_bytes()).unwrap();
        // 6 words in the first sentence + 1 in the second; 2 ranges cover 4 words → tokens 7 − 4 + 2 = 5
        assert_eq!(c, Counts { sentences: 2, tokens: 5, words: 7, fused: 2, empty: 1 });
        assert_eq!(conllu_name("fr_gsd-ud-train.conllu"), Some(("fr_gsd".into(), Split::Train)));
        assert_eq!(conllu_name("qtd_sagt-ud-test.conllu"), Some(("qtd_sagt".into(), Split::Test)));
        assert_eq!(conllu_name("fr_gsd-ud-train.txt"), None);
    }

    #[test]
    fn broken_range_is_red() {
        assert!(count("3-x\tal\n".as_bytes()).is_err());
        assert!(count("4-3\tal\n".as_bytes()).is_err());
    }

    #[test]
    fn stats_gate_shows_red() {
        // a split that is absent (dev) is written as zeros in stats.xml — that is a match
        let block = "<sentences>2</sentences><tokens>5</tokens><words>7</words><fused>2</fused>";
        let xml = format!("<treebank><size><total>{block}</total><train>{block}</train><dev><sentences>0</sentences><tokens>0</tokens><words>0</words><fused>0</fused></dev></size></treebank>");
        let mut t = Treebank { dir: "UD_X-Y".into(), code: "x_y".into(), stats_xml: xml.clone(), ..Default::default() };
        t.splits.insert(Split::Train, count(CONLLU.as_bytes()).unwrap());
        assert!(check_stats(&t).is_empty(), "{:?}", check_stats(&t));
        // negative controls: a different number (in a split and in the total), an extra split in the data, no <size>
        t.stats_xml = xml.replacen("<tokens>5</tokens>", "<tokens>6</tokens>", 2);
        assert_eq!(check_stats(&t).len(), 2);
        t.stats_xml = xml.clone();
        t.splits.insert(Split::Test, Counts { sentences: 1, tokens: 1, words: 1, fused: 0, empty: 0 });
        assert_eq!(check_stats(&t).len(), 2, "test is in the data but not in stats.xml; the total also differs");
        t.stats_xml = "<treebank/>".into();
        assert_eq!(check_stats(&t).len(), 1);
        // no words and fused in stats.xml → words = tokens, fused 0
        let s = parse_stats("<size><dev><sentences>1</sentences><tokens>5</tokens></dev></size>");
        assert_eq!(s["dev"], Counts { sentences: 1, tokens: 5, words: 5, fused: 0, empty: 0 });
    }
}
