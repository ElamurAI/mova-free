//! ocrfix: OCR of public-domain printed Hebrew and Aramaic, deterministic post-correction, and measurement.
//!
//! Subcommands:
//!   ocrfix lexicon --out LEX.tsv [--conllu DIR]... [--verses DIR]...
//!       word forms (consonants only) and their counts from CoNLL-U `# text =` lines and `ref<TAB>text` files
//!   ocrfix ocr --pdf PDF --out DIR [--first N] [--last N] [--lang heb] [--mask heb+eng] [--dpi N] [--crop x,y,w,h] [--jobs N]
//!       renders each page with pdftoppm (at the scan's native resolution unless --dpi), reads it with Tesseract's
//!       Hebrew model, and keeps the Hebrew-script lines and words as told by a second reading with a Latin model
//!       alongside (--mask "" keeps everything); one output line per physical line
//!   ocrfix correct --lex LEX.tsv[:weight]... [--self W] [--edit-cost C] [--oov C] [--lead] [--explain] IN... --out DIR
//!       noisy-channel correction of each word; --self adds the inputs' own word counts with weight W
//!   ocrfix cer --gt GT.txt --hyp HYP.txt [--points] [--confusions N]
//!       character error rate over the Hebrew-script letters (word gaps count as one space)
//!   ocrfix vote --a DIR --b DIR --out DIR --lex LEX.tsv[:weight]... [--self W] [--explain]
//!       two readings of the same pages (Tesseract in A, Kraken in B): A's words are kept, except where the aligned
//!       word of B differs a little and is the better attested form
//!   ocrfix lines --dir DIR --edition ID [--min-share F]
//!       the Hebrew-script lines of every page as `ID:pNNNN:lNN<TAB>kind<TAB>text`, kind `text` (a Hebrew-script
//!       line) or `cited` (Hebrew words quoted inside a Latin-script line)
//!
//! Tools: `pdftoppm`/`pdfimages` (poppler) and `tesseract` from PATH, or MOVA_TESSERACT / MOVA_TESSDATA.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use langseg::{is_heb_letter, is_heb_point};
use unicode_normalization::UnicodeNormalization;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let r = match args.first().map(String::as_str) {
        Some("lexicon") => cmd_lexicon(&args[1..]),
        Some("ocr") => cmd_ocr(&args[1..]),
        Some("correct") => cmd_correct(&args[1..]),
        Some("cer") => cmd_cer(&args[1..]),
        Some("lines") => cmd_lines(&args[1..]),
        Some("vote") => cmd_vote(&args[1..]),
        _ => Err("usage: ocrfix lexicon|ocr|correct|cer|lines ... (see src/main.rs)".into()),
    };
    if let Err(e) = r {
        eprintln!("ocrfix: {e}");
        std::process::exit(1);
    }
}

type R<T> = Result<T, String>;

// ---------------------------------------------------------------- arguments

struct Args {
    flags: BTreeMap<String, Vec<String>>,
    free: Vec<String>,
}

fn parse(args: &[String], switches: &[&str]) -> Args {
    let mut flags: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut free = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if let Some(k) = a.strip_prefix("--") {
            if switches.contains(&k) {
                flags.entry(k.into()).or_default().push(String::new());
            } else {
                let v = args.get(i + 1).cloned().unwrap_or_default();
                flags.entry(k.into()).or_default().push(v);
                i += 1;
            }
        } else {
            free.push(a.clone());
        }
        i += 1;
    }
    Args { flags, free }
}

impl Args {
    fn one(&self, k: &str) -> Option<&str> {
        self.flags.get(k).and_then(|v| v.last()).map(String::as_str)
    }
    fn need(&self, k: &str) -> R<&str> {
        self.one(k).ok_or_else(|| format!("--{k} is required"))
    }
    fn all(&self, k: &str) -> Vec<String> {
        self.flags.get(k).cloned().unwrap_or_default()
    }
    fn has(&self, k: &str) -> bool {
        self.flags.contains_key(k)
    }
    fn num<T: std::str::FromStr>(&self, k: &str, d: T) -> R<T> {
        match self.one(k) {
            None => Ok(d),
            Some(s) => s.parse().map_err(|_| format!("--{k}: bad number {s}")),
        }
    }
}

fn read(p: &Path) -> R<String> {
    fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))
}
fn write(p: &Path, s: &str) -> R<()> {
    fs::write(p, s).map_err(|e| format!("{}: {e}", p.display()))
}

// ---------------------------------------------------------------- characters

/// NFD, with the alef-lamed ligature (which has no decomposition) spelled out.
fn nfd(s: &str) -> String {
    s.replace('\u{FB4F}', "אל").nfd().collect()
}

/// A text as a stream of Hebrew letters (and points, if asked), every gap between words reduced to one space.
fn letter_stream(text: &str, points: bool) -> Vec<char> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.starts_with('#') {
            continue;
        }
        for c in nfd(line).chars() {
            if is_heb_letter(c) || (points && is_heb_point(c)) {
                out.push(c);
            } else if is_heb_point(c) || ('\u{0591}'..='\u{05AF}').contains(&c) || is_editorial(c) {
                // marks and an editor's brackets inside a word are not gaps
            } else if out.last().is_some_and(|&l| l != ' ') {
                out.push(' ');
            }
        }
        if out.last().is_some_and(|&l| l != ' ') {
            out.push(' ');
        }
    }
    while out.last() == Some(&' ') {
        out.pop();
    }
    out
}

/// Square brackets of restored letters: part of the word they stand in.
fn is_editorial(c: char) -> bool {
    matches!(c, '[' | ']')
}

/// The consonants of a word.
fn skeleton(w: &str) -> String {
    nfd(w).chars().filter(|&c| is_heb_letter(c)).collect()
}

/// Hebrew-script words of a text (maximal runs of letters and marks), as consonant skeletons.
fn words(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for c in nfd(text).chars() {
        if is_heb_letter(c) {
            cur.push(c);
        } else if is_heb_point(c) || ('\u{0591}'..='\u{05AF}').contains(&c) || is_editorial(c) {
        } else if !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

// ---------------------------------------------------------------- lexicon

fn cmd_lexicon(args: &[String]) -> R<()> {
    let a = parse(args, &[]);
    let out = PathBuf::from(a.need("out")?);
    let mut counts: BTreeMap<String, u64> = BTreeMap::new();
    let mut add = |text: &str| {
        for w in words(text) {
            *counts.entry(w).or_default() += 1;
        }
    };
    for d in a.all("conllu") {
        for f in sorted_files(Path::new(&d), "conllu")? {
            for line in read(&f)?.lines() {
                if let Some(t) = line.strip_prefix("# text = ") {
                    add(t);
                }
            }
        }
    }
    for d in a.all("verses") {
        for f in sorted_files(Path::new(&d), "tsv")? {
            for line in read(&f)?.lines() {
                if let Some((_, t)) = line.split_once('\t') {
                    add(t);
                }
            }
        }
    }
    let total: u64 = counts.values().sum();
    let mut s = format!("# form\tcount (consonants only); {} forms, {} tokens\n", counts.len(), total);
    for (w, c) in &counts {
        s.push_str(&format!("{w}\t{c}\n"));
    }
    write(&out, &s)?;
    eprintln!("lexicon: {} forms, {} tokens -> {}", counts.len(), total, out.display());
    Ok(())
}

fn sorted_files(dir: &Path, ext: &str) -> R<Vec<PathBuf>> {
    let mut v: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == ext))
        .collect();
    v.sort();
    Ok(v)
}

fn load_lexicon(path: &str, weight: f64, into: &mut BTreeMap<String, f64>) -> R<()> {
    for line in read(Path::new(path))?.lines() {
        if line.starts_with('#') {
            continue;
        }
        if let Some((w, c)) = line.split_once('\t') {
            let c: f64 = c.trim().parse().map_err(|_| format!("{path}: bad count in {line}"))?;
            *into.entry(w.to_string()).or_default() += weight * c;
        }
    }
    Ok(())
}

// ---------------------------------------------------------------- the channel

struct Channel {
    sub: BTreeMap<char, Vec<(char, f64)>>,
    indel: Vec<(char, f64)>,
}

impl Channel {
    fn load(base: f64) -> R<Channel> {
        let mut sub: BTreeMap<char, Vec<(char, f64)>> = BTreeMap::new();
        let mut indel = Vec::new();
        for line in include_str!("../data/confusions.tsv").lines().skip_while(|l| l.starts_with('#')).skip(1) {
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 3 {
                continue;
            }
            let b = f[1].chars().next().ok_or("confusions: empty letter")?;
            let cost: f64 = base + f[2].parse::<f64>().map_err(|_| format!("confusions: bad cost {line}"))?;
            if f[0] == "+" {
                indel.push((b, cost));
            } else {
                let a = f[0].chars().next().ok_or("confusions: empty letter")?;
                sub.entry(a).or_default().push((b, cost));
                sub.entry(b).or_default().push((a, cost));
            }
        }
        Ok(Channel { sub, indel })
    }

    /// Every string reachable from `w` with total channel cost <= `max`, with its cost (cheapest path).
    fn candidates(&self, w: &str, max: f64, depth: usize) -> BTreeMap<String, f64> {
        let mut best: BTreeMap<String, f64> = BTreeMap::new();
        let mut frontier: Vec<(Vec<char>, f64)> = vec![(w.chars().collect(), 0.0)];
        for _ in 0..depth {
            let mut next = Vec::new();
            for (cs, c0) in &frontier {
                let mut push = |v: Vec<char>, c: f64| {
                    if c > max + 1e-9 {
                        return;
                    }
                    let s: String = v.iter().collect();
                    let e = best.entry(s).or_insert(f64::INFINITY);
                    if c < *e {
                        *e = c;
                        next.push((v, c));
                    }
                };
                for i in 0..cs.len() {
                    if let Some(alts) = self.sub.get(&cs[i]) {
                        for &(b, cost) in alts {
                            let mut v = cs.clone();
                            v[i] = b;
                            push(v, c0 + cost);
                        }
                    }
                    for &(b, cost) in &self.indel {
                        if cs[i] == b && cs.len() > 1 {
                            let mut v = cs.clone();
                            v.remove(i);
                            push(v, c0 + cost);
                        }
                    }
                }
                for i in 0..=cs.len() {
                    for &(b, cost) in &self.indel {
                        let mut v = cs.clone();
                        v.insert(i, b);
                        push(v, c0 + cost);
                    }
                }
            }
            frontier = next;
        }
        best.remove(w);
        best
    }
}

/// Final letters only stand at the end of a word, and their plain forms not there; a candidate that breaks this is out.
fn finals_ok(w: &str) -> bool {
    let cs: Vec<char> = w.chars().collect();
    for (i, &c) in cs.iter().enumerate() {
        let last = i + 1 == cs.len();
        if matches!(c, 'ך' | 'ם' | 'ן' | 'ף' | 'ץ') && !last {
            return false;
        }
    }
    true
}

struct Corrector {
    lex: BTreeMap<String, f64>,
    total: f64,
    oov: f64,
    channel: Channel,
    max_cost: f64,
    short_cost: f64,
    min_len: usize,
    lead_cost: f64,
}

impl Corrector {
    fn count(&self, w: &str) -> f64 {
        self.lex.get(w).copied().unwrap_or(0.0)
    }
    /// The best reading of a consonant skeleton and its explanation.
    fn best(&self, w: &str) -> (String, String) {
        let ln_n = self.total.ln();
        let c0 = self.count(w).max(self.oov);
        let mut best = (c0.ln() - ln_n, w.to_string(), 0.0);
        let len = w.chars().count();
        // one- and two-letter words have too many attested neighbours to be told apart by frequency
        if len < self.min_len {
            return (w.to_string(), String::new());
        }
        let max = if len <= 3 { self.max_cost.min(self.short_cost) } else { self.max_cost };
        let mut cands = self.channel.candidates(w, max, 2);
        // (--lead) a superscript verse number read as a letter glued to the word: drop a first letter that is no
        // prefix. Off by default: it also cuts real verb prefixes (תועל -> ועל)
        if let Some(first) = w.chars().next() {
            if self.lead_cost.is_finite() && !"ובהכלמש".contains(first) && len >= 4 {
                let rest: String = w.chars().skip(1).collect();
                let e = cands.entry(rest).or_insert(f64::INFINITY);
                *e = e.min(self.lead_cost);
            }
        }
        for (cand, cost) in cands {
            let c = self.count(&cand);
            // short words only take substitutions, not a letter more or less
            if c <= 0.0 || !finals_ok(&cand) || (len <= 3 && cand.chars().count() != len) {
                continue;
            }
            let score = c.ln() - ln_n - cost;
            if score > best.0 + 1e-12 {
                best = (score, cand, cost);
            }
        }
        let why = if best.1 == w {
            String::new()
        } else {
            format!("{w}->{} (count {:.1} vs {:.1}, channel {:.1})", best.1, self.count(&best.1), self.count(w), best.2)
        };
        (best.1, why)
    }

    /// Correct one line, keeping every non-word character; words with brackets or mixed scripts stay as they are.
    fn line(&self, line: &str, log: &mut Vec<String>) -> String {
        let mut out = String::new();
        for (i, tok) in join_brackets(line).split(' ').enumerate() {
            if i > 0 {
                out.push(' ');
            }
            out.push_str(&self.token(tok, log));
        }
        out
    }

    fn token(&self, tok: &str, log: &mut Vec<String>) -> String {
        let t = nfd(tok);
        let is_word = |c: char| is_heb_letter(c) || is_heb_point(c) || ('\u{0591}'..='\u{05AF}').contains(&c);
        let start = t.char_indices().find(|&(_, c)| is_word(c)).map(|(i, _)| i);
        let Some(start) = start else { return tok.to_string() };
        let end = t.char_indices().filter(|&(_, c)| is_word(c)).map(|(i, c)| i + c.len_utf8()).last().unwrap_or(start);
        let core = &t[start..end];
        if core.chars().any(|c| !is_word(c)) {
            return tok.to_string();
        }
        let sk = skeleton(core);
        let (fixed, why) = self.best(&sk);
        if fixed == sk {
            return tok.to_string();
        }
        log.push(why);
        // points cannot follow a changed skeleton, so a corrected word is written unpointed
        format!("{}{}{}", &t[..start], fixed, &t[end..]).nfc().collect()
    }
}

fn cmd_correct(args: &[String]) -> R<()> {
    let a = parse(args, &["explain", "lead"]);
    let out = PathBuf::from(a.need("out")?);
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let mut lex = BTreeMap::new();
    for spec in a.all("lex") {
        let (p, w) = match spec.rsplit_once(':') {
            Some((p, w)) if w.parse::<f64>().is_ok() => (p.to_string(), w.parse().unwrap()),
            _ => (spec.clone(), 1.0),
        };
        load_lexicon(&p, w, &mut lex)?;
    }
    let inputs: Vec<PathBuf> = a.free.iter().map(PathBuf::from).collect();
    let texts: Vec<String> = inputs.iter().map(|p| read(p)).collect::<R<_>>()?;
    let self_w: f64 = a.num("self", 0.0)?;
    if self_w > 0.0 {
        for t in &texts {
            for w in words(t) {
                *lex.entry(w).or_default() += self_w;
            }
        }
    }
    let total: f64 = lex.values().sum();
    let base: f64 = a.num("edit-cost", 4.0)?;
    let c = Corrector {
        lex,
        total,
        oov: a.num("oov", 0.5)?,
        channel: Channel::load(base)?,
        max_cost: a.num("max-cost", 2.0 * base + 4.0)?,
        short_cost: base + 2.0,
        min_len: a.num("min-len", 3)?,
        lead_cost: if a.has("lead") { base + 1.0 } else { f64::INFINITY },
    };
    let mut changed = 0usize;
    for (p, t) in inputs.iter().zip(&texts) {
        let mut log = Vec::new();
        let fixed: Vec<String> = t.lines().map(|l| c.line(l, &mut log)).collect();
        changed += log.len();
        let name = p.file_name().ok_or("bad input name")?;
        write(&out.join(name), &(fixed.join("\n") + "\n"))?;
        if a.has("explain") {
            for l in &log {
                println!("{}\t{l}", p.display());
            }
        }
    }
    eprintln!("correct: {} files, {changed} words changed", inputs.len());
    Ok(())
}

// ---------------------------------------------------------------- measurement

/// Levenshtein alignment: (substitutions, insertions, deletions, substitution pairs gt->hyp).
fn align(gt: &[char], hyp: &[char]) -> (usize, usize, usize, BTreeMap<(char, char), usize>) {
    let (n, m) = (gt.len(), hyp.len());
    let mut d = vec![vec![0u32; m + 1]; n + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i as u32;
    }
    for j in 0..=m {
        d[0][j] = j as u32;
    }
    for i in 1..=n {
        for j in 1..=m {
            let s = d[i - 1][j - 1] + u32::from(gt[i - 1] != hyp[j - 1]);
            d[i][j] = s.min(d[i - 1][j] + 1).min(d[i][j - 1] + 1);
        }
    }
    let (mut i, mut j) = (n, m);
    let (mut s, mut ins, mut del) = (0, 0, 0);
    let mut pairs = BTreeMap::new();
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && d[i][j] == d[i - 1][j - 1] + u32::from(gt[i - 1] != hyp[j - 1]) {
            if gt[i - 1] != hyp[j - 1] {
                s += 1;
                *pairs.entry((gt[i - 1], hyp[j - 1])).or_insert(0) += 1;
            }
            i -= 1;
            j -= 1;
        } else if i > 0 && d[i][j] == d[i - 1][j] + 1 {
            del += 1;
            i -= 1;
        } else {
            ins += 1;
            j -= 1;
        }
    }
    (s, ins, del, pairs)
}

fn cmd_cer(args: &[String]) -> R<()> {
    let a = parse(args, &["points"]);
    let points = a.has("points");
    let gt = letter_stream(&read(Path::new(a.need("gt")?))?, points);
    let hyp = letter_stream(&read(Path::new(a.need("hyp")?))?, points);
    let (s, i, d, pairs) = align(&gt, &hyp);
    let e = s + i + d;
    let cer = e as f64 / gt.len().max(1) as f64;
    println!("chars\t{}\terrors\t{e}\tsub\t{s}\tins\t{i}\tdel\t{d}\tcer\t{:.4}", gt.len(), cer);
    let n: usize = a.num("confusions", 0)?;
    if n > 0 {
        let mut v: Vec<_> = pairs.into_iter().collect();
        v.sort_by(|x, y| y.1.cmp(&x.1).then(x.0.cmp(&y.0)));
        for ((g, h), c) in v.into_iter().take(n) {
            println!("  {g}->{h}\t{c}");
        }
    }
    Ok(())
}

// ---------------------------------------------------------------- OCR

fn tool(env: &str, default: &str) -> String {
    std::env::var(env).unwrap_or_else(|_| default.to_string())
}

fn page_count(pdf: &str) -> R<usize> {
    let o = Command::new("pdfinfo").arg(pdf).output().map_err(|e| format!("pdfinfo: {e}"))?;
    let s = String::from_utf8_lossy(&o.stdout);
    s.lines()
        .find_map(|l| l.strip_prefix("Pages:").map(|v| v.trim().parse().ok()))
        .flatten()
        .ok_or_else(|| format!("{pdf}: no page count"))
}

/// The highest image resolution on a page (the scan's own), or 400.
fn native_dpi(pdf: &str, page: usize) -> u32 {
    let o = Command::new("pdfimages")
        .args(["-list", "-f", &page.to_string(), "-l", &page.to_string(), pdf])
        .output();
    let Ok(o) = o else { return 400 };
    let s = String::from_utf8_lossy(&o.stdout);
    s.lines()
        .skip(2)
        .filter_map(|l| l.split_whitespace().nth(12).and_then(|v| v.parse::<u32>().ok()))
        .max()
        .filter(|&d| (150..=1200).contains(&d))
        .unwrap_or(400)
}

/// One word of Tesseract's TSV output.
#[derive(Clone, Debug)]
struct TWord {
    line: (u32, u32, u32),
    bbox: (i32, i32, i32, i32),
    text: String,
}

fn parse_tsv(tsv: &str) -> Vec<TWord> {
    let mut v = Vec::new();
    for l in tsv.lines().skip(1) {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() < 12 || f[0] != "5" || f[11].trim().is_empty() {
            continue;
        }
        let n = |i: usize| f[i].parse::<i32>().unwrap_or(0);
        v.push(TWord {
            line: (n(2) as u32, n(3) as u32, n(4) as u32),
            bbox: (n(6), n(7), n(6) + n(8), n(7) + n(9)),
            text: f[11].to_string(),
        });
    }
    v
}

fn overlap(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> i64 {
    let w = (a.2.min(b.2) - a.0.max(b.0)).max(0) as i64;
    let h = (a.3.min(b.3) - a.1.max(b.1)).max(0) as i64;
    w * h
}

fn tesseract_tsv(png: &Path, lang: &str, dpi: u32) -> R<String> {
    // the TSV renderer is switched on by variables, not by the `tsv` config file, which a bare tessdata dir lacks
    let base = png.with_extension(format!("{}", lang.replace('+', "_")));
    let mut t = Command::new(tool("MOVA_TESSERACT", "tesseract"));
    t.env("OMP_THREAD_LIMIT", "1").arg(png).arg(&base).args(["-l", lang, "--dpi", &dpi.to_string()]);
    if let Ok(d) = std::env::var("MOVA_TESSDATA") {
        t.args(["--tessdata-dir", &d]);
    }
    t.args(["-c", "tessedit_create_txt=0", "-c", "tessedit_create_tsv=1"]);
    if let Ok(psm) = std::env::var("MOVA_TESS_PSM") {
        t.args(["--psm", &psm]);
    }
    let o = t.output().map_err(|e| format!("tesseract: {e}"))?;
    if !o.status.success() {
        return Err(format!("tesseract {lang} failed: {}", String::from_utf8_lossy(&o.stderr)));
    }
    let f = base.with_extension(format!("{}.tsv", lang.replace('+', "_")));
    let f = if f.exists() { f } else { PathBuf::from(format!("{}.tsv", base.display())) };
    let s = read(&f)?;
    let _ = fs::remove_file(&f);
    Ok(s)
}

/// Merge the two readings of a page. The Hebrew-only reading gives the text; the reading with a Latin model
/// alongside (`mask`) tells which words are in Latin script. A line goes out whole when the mask reads it as mostly
/// Hebrew script; in a mostly Latin line only the words the mask also reads as Hebrew script are kept.
/// Every physical line gives one output line (empty when nothing Hebrew is on it), so line numbers stay physical.
fn merge(heb: &[TWord], mask: &[TWord]) -> String {
    let mut lines: Vec<((u32, u32, u32), Vec<&TWord>)> = Vec::new();
    for w in heb {
        match lines.last_mut() {
            Some((k, v)) if *k == w.line => v.push(w),
            _ => lines.push((w.line, vec![w])),
        }
    }
    let script = |w: &TWord| -> Option<&TWord> {
        mask.iter().filter(|m| overlap(m.bbox, w.bbox) > 0).max_by_key(|m| overlap(m.bbox, w.bbox))
    };
    let heb_share = |t: &str| {
        let h = t.chars().filter(|&c| is_heb_letter(c)).count();
        let o = t.chars().filter(|c| c.is_alphabetic() && !is_heb_letter(*c)).count();
        (h, o)
    };
    let mut out = String::new();
    for (_, ws) in &lines {
        let (mut h, mut o) = (0, 0);
        for w in ws {
            if let Some(m) = script(w) {
                let (a, b) = heb_share(&m.text);
                h += a;
                o += b;
            }
        }
        let text_line = h > 0 && h >= o;
        let keep: Vec<&str> = if text_line {
            ws.iter().map(|w| w.text.as_str()).collect()
        } else {
            ws.iter()
                .filter(|w| script(w).is_some_and(|m| heb_share(&m.text).0 > heb_share(&m.text).1))
                .map(|w| w.text.as_str())
                .collect()
        };
        // numbers (line and verse numbers) and Hebrew letters mixed with Latin or digits are reading debris
        let keep: Vec<&str> = keep.into_iter().filter(|t| !t.chars().any(|c| c.is_ascii_alphanumeric())).collect();
        // Hebrew words cited inside a Latin-script line are marked `~`, so they can be told from the text proper
        if !text_line && !keep.is_empty() {
            out.push_str("~ ");
        }
        out.push_str(&join_brackets(&keep.join(" ")));
        out.push('\n');
    }
    out
}

/// Kraken (`--lang kraken`): baseline segmentation and recognition with the model in MOVA_KRAKEN_MODEL.
/// Its text comes out in visual order; each line is turned back into logical order.
fn kraken(png: &Path) -> R<String> {
    let model = std::env::var("MOVA_KRAKEN_MODEL").map_err(|_| "MOVA_KRAKEN_MODEL is not set".to_string())?;
    let txt = png.with_extension("kraken.txt");
    let o = Command::new(tool("MOVA_KRAKEN", "kraken"))
        .env("CUDA_VISIBLE_DEVICES", "")
        .env("OMP_NUM_THREADS", "1")
        .arg("-i")
        .arg(png)
        .arg(&txt)
        .args(["binarize", "segment", "-bl", "ocr", "-m", &model])
        .output()
        .map_err(|e| format!("kraken: {e}"))?;
    if !o.status.success() {
        return Err(format!("kraken failed: {}", String::from_utf8_lossy(&o.stderr)));
    }
    let s = read(&txt)?;
    let _ = fs::remove_file(&txt);
    Ok(s.lines().map(visual_to_logical).collect::<Vec<_>>().join("\n") + "\n")
}

/// Reverse a right-to-left line given in visual order: marks stay after their letter, paired signs are mirrored.
fn visual_to_logical(line: &str) -> String {
    let mut clusters: Vec<String> = Vec::new();
    for c in line.chars() {
        let mark = is_heb_point(c) || ('\u{0591}'..='\u{05AF}').contains(&c) || ('\u{0300}'..='\u{036F}').contains(&c);
        match clusters.last_mut() {
            Some(last) if mark => last.push(c),
            _ => clusters.push(c.to_string()),
        }
    }
    clusters
        .iter()
        .rev()
        .map(|g| match g.as_str() {
            "(" => ")".to_string(),
            ")" => "(".to_string(),
            "[" => "]".to_string(),
            "]" => "[".to_string(),
            "<" => ">".to_string(),
            ">" => "<".to_string(),
            _ => g.clone(),
        })
        .collect()
}

/// Tesseract splits a word at an editor's bracket inside it (`מ[ לך`, `א ]בוהי`); a bracket glued to a letter on one
/// side and followed (or preceded) by a space and a letter on the other is put back inside the word.
fn join_brackets(line: &str) -> String {
    let cs: Vec<char> = line.chars().collect();
    let mut out = String::new();
    for i in 0..cs.len() {
        if cs[i] == ' ' && i > 0 && i + 1 < cs.len() {
            let (p, n) = (cs[i - 1], cs[i + 1]);
            // an opening bracket that hangs on the letter before it, a closing one that hangs on the letter after it
            let glued_left = p == '[' && i >= 2 && is_heb_letter(cs[i - 2]) && is_heb_letter(n);
            let glued_right = n == ']' && i + 2 < cs.len() && is_heb_letter(cs[i + 2]) && is_heb_letter(p);
            if glued_left || glued_right {
                continue;
            }
        }
        out.push(cs[i]);
    }
    out
}

fn ocr_page(pdf: &str, page: usize, out: &Path, lang: &str, mask: &str, dpi: Option<u32>, crop: Option<&str>) -> R<()> {
    let dpi = dpi.unwrap_or_else(|| native_dpi(pdf, page));
    let stem = out.join(format!("p{page:04}"));
    let png = stem.with_extension("png");
    let mut c = Command::new("pdftoppm");
    c.args(["-r", &dpi.to_string(), "-gray", "-png", "-singlefile", "-f", &page.to_string(), "-l", &page.to_string()]);
    if let Some(cr) = crop {
        let v: Vec<&str> = cr.split(',').collect();
        if v.len() != 4 {
            return Err(format!("--crop wants x,y,w,h, got {cr}"));
        }
        c.args(["-x", v[0], "-y", v[1], "-W", v[2], "-H", v[3]]);
    }
    c.arg(pdf).arg(&stem);
    let st = c.status().map_err(|e| format!("pdftoppm: {e}"))?;
    if !st.success() {
        return Err(format!("pdftoppm failed on page {page}"));
    }
    let r = (|| {
        if lang == "kraken" {
            return write(&stem.with_extension("txt"), &kraken(&png)?);
        }
        let heb = parse_tsv(&tesseract_tsv(&png, lang, dpi)?);
        let text = if mask.is_empty() {
            merge(&heb, &heb)
        } else {
            merge(&heb, &parse_tsv(&tesseract_tsv(&png, mask, dpi)?))
        };
        write(&stem.with_extension("txt"), &text)
    })();
    let _ = fs::remove_file(&png);
    r.map_err(|e| format!("page {page}: {e}"))
}

fn cmd_ocr(args: &[String]) -> R<()> {
    let a = parse(args, &[]);
    let pdf = a.need("pdf")?.to_string();
    let out = PathBuf::from(a.need("out")?);
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let n = page_count(&pdf)?;
    let first: usize = a.num("first", 1)?;
    let last: usize = a.num("last", n)?.min(n);
    let lang = a.one("lang").unwrap_or("heb").to_string();
    let mask = a.one("mask").unwrap_or("heb+eng").to_string();
    let dpi: Option<u32> = a.one("dpi").map(|v| v.parse().map_err(|_| "bad --dpi".to_string())).transpose()?;
    let crop = a.one("crop").map(String::from);
    let jobs: usize = a.num("jobs", 8)?;
    // --only-heb DIR N: only the pages whose earlier reading in DIR has at least N Hebrew letters
    let only = a.one("only-heb").map(String::from);
    let min_heb: usize = a.num("min-heb", 200)?;
    let pages: Vec<usize> = (first..=last)
        .filter(|p| !out.join(format!("p{p:04}.txt")).exists())
        .filter(|p| match &only {
            None => true,
            Some(d) => fs::read_to_string(Path::new(d).join(format!("p{p:04}.txt")))
                .map(|t| t.chars().filter(|&c| is_heb_letter(c)).count() >= min_heb)
                .unwrap_or(false),
        })
        .collect();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let errors = std::sync::Mutex::new(Vec::new());
    std::thread::scope(|s| {
        for _ in 0..jobs.max(1) {
            s.spawn(|| loop {
                let k = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                let Some(&p) = pages.get(k) else { break };
                if let Err(e) = ocr_page(&pdf, p, &out, &lang, &mask, dpi, crop.as_deref()) {
                    errors.lock().unwrap().push(e);
                }
            });
        }
    });
    let errors = errors.into_inner().unwrap();
    eprintln!("ocr: pages {first}-{last}, {} read, {} errors", pages.len() - errors.len(), errors.len());
    match errors.first() {
        Some(e) => Err(e.clone()),
        None => Ok(()),
    }
}

// ---------------------------------------------------------------- two witnesses

/// The words of a second witness, without the debris of verse numbers (one or two letters with a geresh or dot).
fn witness_words(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for tok in text.split_whitespace() {
        for part in tok.split(['\'', '\u{05F3}', '\u{02D9}', '/']) {
            let sk = skeleton(part);
            let marked = tok.contains(['\'', '\u{05F3}', '\u{02D9}']);
            if sk.is_empty() || (marked && sk.chars().count() <= 2 && !part.contains('\u{FB4F}')) {
                continue;
            }
            if part.chars().any(|c| c.is_ascii_alphanumeric()) {
                continue;
            }
            out.push(sk);
        }
    }
    out
}

fn edit_distance(a: &[char], b: &[char]) -> usize {
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for i in 1..=a.len() {
        let mut cur = vec![i; b.len() + 1];
        for j in 1..=b.len() {
            cur[j] = (prev[j - 1] + usize::from(a[i - 1] != b[j - 1])).min(prev[j] + 1).min(cur[j - 1] + 1);
        }
        prev = cur;
    }
    prev[b.len()]
}

/// Word alignment of two sequences; returns for each word of `a` the index of its partner in `b`, if any.
fn align_words(a: &[String], b: &[String]) -> Vec<Option<usize>> {
    let (n, m) = (a.len(), b.len());
    let ac: Vec<Vec<char>> = a.iter().map(|w| w.chars().collect()).collect();
    let bc: Vec<Vec<char>> = b.iter().map(|w| w.chars().collect()).collect();
    let sub = |i: usize, j: usize| -> f64 {
        let d = edit_distance(&ac[i], &bc[j]) as f64;
        (d / ac[i].len().max(bc[j].len()) as f64).min(1.0) * 1.2
    };
    let gap = 0.6;
    let mut d = vec![vec![0.0f64; m + 1]; n + 1];
    for i in 0..=n {
        d[i][0] = i as f64 * gap;
    }
    for j in 0..=m {
        d[0][j] = j as f64 * gap;
    }
    for i in 1..=n {
        for j in 1..=m {
            d[i][j] = (d[i - 1][j - 1] + sub(i - 1, j - 1)).min(d[i - 1][j] + gap).min(d[i][j - 1] + gap);
        }
    }
    let mut pair = vec![None; n];
    let (mut i, mut j) = (n, m);
    while i > 0 && j > 0 {
        if (d[i][j] - (d[i - 1][j - 1] + sub(i - 1, j - 1))).abs() < 1e-9 {
            pair[i - 1] = Some(j - 1);
            i -= 1;
            j -= 1;
        } else if (d[i][j] - (d[i - 1][j] + gap)).abs() < 1e-9 {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    pair
}

fn cmd_vote(args: &[String]) -> R<()> {
    let a = parse(args, &["explain"]);
    let da = PathBuf::from(a.need("a")?);
    let db = PathBuf::from(a.need("b")?);
    let out = PathBuf::from(a.need("out")?);
    fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    let mut lex = BTreeMap::new();
    for spec in a.all("lex") {
        let (p, w) = match spec.rsplit_once(':') {
            Some((p, w)) if w.parse::<f64>().is_ok() => (p.to_string(), w.parse().unwrap()),
            _ => (spec.clone(), 1.0),
        };
        load_lexicon(&p, w, &mut lex)?;
    }
    let files = sorted_files(&da, "txt")?;
    let mut pages = Vec::new();
    for f in &files {
        let name = f.file_name().ok_or("bad name")?.to_owned();
        let ta = read(f)?;
        let tb = fs::read_to_string(db.join(&name)).unwrap_or_default();
        pages.push((name, ta, tb));
    }
    let self_w: f64 = a.num("self", 0.0)?;
    if self_w > 0.0 {
        for (_, ta, tb) in &pages {
            for w in words(ta).into_iter().chain(witness_words(tb)) {
                *lex.entry(w).or_default() += self_w / 2.0;
            }
        }
    }
    let count = |w: &str| lex.get(w).copied().unwrap_or(0.0);
    let (mut taken, mut seen) = (0usize, 0usize);
    for (name, ta, tb) in &pages {
        // the words of A with their place in the text, so they can be replaced in place
        let mut slots: Vec<(usize, usize, String)> = Vec::new();
        let lines: Vec<Vec<String>> = ta.lines().map(|l| l.split(' ').map(String::from).collect()).collect();
        for (li, toks) in lines.iter().enumerate() {
            for (ti, t) in toks.iter().enumerate() {
                let sk = skeleton(t);
                if !sk.is_empty() && !t.contains(['[', ']']) {
                    slots.push((li, ti, sk));
                }
            }
        }
        let wa: Vec<String> = slots.iter().map(|s| s.2.clone()).collect();
        let wb = witness_words(tb);
        let pair = align_words(&wa, &wb);
        let mut lines = lines;
        for (k, (li, ti, sk)) in slots.iter().enumerate() {
            let Some(j) = pair[k] else { continue };
            let other = &wb[j];
            if other == sk {
                continue;
            }
            seen += 1;
            let d = edit_distance(&sk.chars().collect::<Vec<_>>(), &other.chars().collect::<Vec<_>>());
            if d > 2 || d * 3 > sk.chars().count().max(other.chars().count()) {
                continue;
            }
            let (ca, cb) = (count(sk), count(other));
            // B wins when its form is clearly better attested; a well attested form in A is only overturned by a far
            // better attested one (zayin/vav pairs like זאת/ואת are both common words)
            if cb > 2.0 * ca + 1.0 && (ca < 5.0 || cb > 20.0 * ca) {
                taken += 1;
                if a.has("explain") {
                    println!("{}\t{sk}->{other}\t{ca:.1} vs {cb:.1}", name.to_string_lossy());
                }
                let t = &lines[*li][*ti];
                let tn = nfd(t);
                let st = tn.char_indices().find(|&(_, c)| is_heb_letter(c)).map(|(i, _)| i).unwrap_or(0);
                let en = tn
                    .char_indices()
                    .filter(|&(_, c)| is_heb_letter(c) || is_heb_point(c))
                    .map(|(i, c)| i + c.len_utf8())
                    .last()
                    .unwrap_or(tn.len());
                lines[*li][*ti] = format!("{}{}{}", &tn[..st], other, &tn[en..]).nfc().collect();
            }
        }
        let text: Vec<String> = lines.iter().map(|l| l.join(" ")).collect();
        write(&out.join(name), &(text.join("\n") + "\n"))?;
    }
    eprintln!("vote: {} pages, {seen} disagreements, {taken} taken from B", pages.len());
    Ok(())
}

// ---------------------------------------------------------------- edition text

/// Hebrew-script lines of each page, with Latin, digits and stray signs dropped from them.
fn cmd_lines(args: &[String]) -> R<()> {
    let a = parse(args, &[]);
    let dir = PathBuf::from(a.need("dir")?);
    let ed = a.need("edition")?;
    let min_share: f64 = a.num("min-share", 0.6)?;
    let min_letters: usize = a.num("min-letters", 4)?;
    let mut out = String::new();
    let mut kept = 0usize;
    for f in sorted_files(&dir, "txt")? {
        let stem = f.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
        let mut n = 0;
        for line in read(&f)?.lines() {
            n += 1;
            if line.trim().is_empty() {
                continue;
            }
            let heb = line.chars().filter(|&c| is_heb_letter(c)).count();
            let other = line.chars().filter(|c| c.is_alphanumeric() && !is_heb_letter(*c)).count();
            if heb < min_letters || (heb as f64) < min_share * (heb + other) as f64 {
                continue;
            }
            let (kind, line) = match line.strip_prefix("~ ") {
                Some(rest) => ("cited", rest),
                None => ("text", line),
            };
            let toks: Vec<&str> = line
                .split_whitespace()
                .filter(|t| t.chars().any(is_heb_letter) && !t.chars().any(|c| c.is_ascii_alphanumeric()))
                .collect();
            if toks.is_empty() {
                continue;
            }
            out.push_str(&format!("{ed}:{stem}:l{n:02}\t{kind}\t{}\n", toks.join(" ")));
            kept += 1;
        }
    }
    print!("{out}");
    eprintln!("lines: {kept} Hebrew-script lines");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brackets_rejoined() {
        assert_eq!(join_brackets("מ[ לך אתור"), "מ[לך אתור");
        assert_eq!(join_brackets("א ]בוהי [אנה ומן]"), "א]בוהי [אנה ומן]");
    }

    #[test]
    fn stream_and_cer() {
        let gt = letter_stream("אנך משע : 1 בן", false);
        assert_eq!(gt.iter().collect::<String>(), "אנך משע בן");
        let hyp = letter_stream("אנך . מ[ש]ע ! בו", false);
        let (s, i, d, _) = align(&gt, &hyp);
        assert_eq!((s, i, d), (1, 0, 0));
    }

    #[test]
    fn channel_corrects_and_keeps() {
        let mut lex = BTreeMap::new();
        lex.insert("כמש".to_string(), 500.0);
        lex.insert("במה".to_string(), 3.0);
        let c = Corrector { total: 1000.0, lex, oov: 0.5, channel: Channel::load(4.0).unwrap(), max_cost: 12.0, short_cost: 6.0, min_len: 3, lead_cost: 5.0 };
        assert_eq!(c.best("במש").0, "כמש");
        // an unknown word with no attested neighbour stays
        assert_eq!(c.best("שתברזן").0, "שתברזן");
        let mut log = Vec::new();
        assert_eq!(c.line("לכמש . במש ,", &mut log), "לכמש . כמש ,");
    }

    #[test]
    fn negative_control_known_word_stays() {
        // a frequent word next to a rarer one must not be "corrected" into it
        let mut lex = BTreeMap::new();
        lex.insert("בר".to_string(), 100.0);
        lex.insert("כר".to_string(), 2.0);
        let c = Corrector { total: 1000.0, lex, oov: 0.5, channel: Channel::load(4.0).unwrap(), max_cost: 12.0, short_cost: 6.0, min_len: 3, lead_cost: 5.0 };
        assert_eq!(c.best("בר").0, "בר");
    }
}
#[cfg(test)]
mod wave33 {
    use super::*;
    #[test]
    fn channel_depth_alignment_and_masks() {
        let c = Channel {
            sub: BTreeMap::from([('א', vec![('ב', 1.0)])]),
            indel: vec![],
        };
        assert!(c.candidates("א", 2.0, 0).is_empty());
        assert!(c.candidates("א", 0.5, 1).is_empty());
        assert_eq!(
            c.candidates("א", 1.0, 1),
            BTreeMap::from([("ב".into(), 1.0)])
        );
        assert!(!finals_ok("ךא"));
        assert!(finals_ok("כ"));
        assert_eq!(edit_distance(&[], &['א']), 1);
        assert_eq!(align(&[], &['א']).1, 1);
        assert_eq!(align(&['א'], &[]).2, 1);
        assert_eq!(visual_to_logical("(אב)"), "(בא)");
        assert_eq!(overlap((0, 0, 2, 2), (2, 0, 3, 2)), 0);
        let heb = vec![TWord {
            line: (1, 1, 1),
            bbox: (0, 0, 2, 2),
            text: "אב".into(),
        }];
        assert_eq!(merge(&heb, &heb), "אב\n");
        assert_eq!(merge(&heb, &[]), "\n");
    }
}
