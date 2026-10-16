//! School tooling: the checks and safe edits of the lesson files and the pupils' dictionary that used to be done
//! with one-off scripts. Agents use these commands, never their own scripts (docs/school-tools.md).
//! Deterministic (ordered maps, sorted files); every length is counted in Unicode characters, never bytes.
//!
//! An option line of a lesson: `exercise:|test:|exam: question | right | wrong | wrong | wrong`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

pub const KNOWLEDGE_USAGE: &str = "\
usage: world knowledge balance [course|dir|file...] [--band L_lo,L_hi,S_lo,S_hi] [--list out.tsv] [--chunk N --prefix p]
         how often the right answer is strictly the longest (L) or strictly the shortest (S) option, per lesson and in all;
         the lessons outside the band (default 1/6,1/3,1/10,1/3, and S > 0); --list writes every lesson, --chunk splits
         the lessons outside the band into files p001.tsv, p002.tsv, … of N lessons each
       world knowledge lint [course|dir|file...] [--fix-spaces]
         option lines with ≠5 fields, empty fields, spacing around |, duplicate options (case-only ones apart), a wrong
         option equal to the right one, glued «X and Y»/«X; Y» wrong options (heuristic), tabs; --fix-spaces only
         normalises the spaces around | and the field ends
       world knowledge edit <edits.tsv> [--dry-run]
         rows `file<TAB>line<TAB>old option<TAB>new option`; a file with any bad row is left untouched
       world knowledge snapshot <course|dir|file...> <dir>
         copies the lesson files into <dir>/<course>/<file>
       world knowledge restore-lines <dir> <file> <line|from-to...>
         puts back only the given lines of one file from its snapshot in <dir>
  (a course is a folder of data/school; with no course: every primary course)
";

pub const DICTIONARY_USAGE: &str = "\
usage: world dictionary merge <records.tsv...> [--check]
         validates records (6 TAB fields, UTF-8, word lower-case, no dates), skips the words already in
         data/dictionary-improved.tsv and appends the new ones; --check only validates
       world dictionary gaps [--split N --prefix p]
         the words of the primary lessons without a record (from the records on disk), split into p001.txt, … of N words
";

pub fn school_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data").join("school")
}

fn dictionary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data").join("dictionary-improved.tsv")
}

fn chars(s: &str) -> usize {
    s.chars().count()
}

const KINDS: [&str; 3] = ["exercise", "test", "exam"];

/// An option line: its key and its fields (trimmed, split by `|`); None for any other line.
pub fn option_line(line: &str) -> Option<(&str, Vec<&str>)> {
    let t = line.trim();
    if t.starts_with('#') {
        return None;
    }
    let (k, v) = t.split_once(':')?;
    let k = k.trim();
    KINDS.contains(&k).then(|| (k, v.split('|').map(str::trim).collect()))
}

/// An option line rebuilt from its fields: the indent kept, one space around every `|`.
fn rebuild(line: &str, key: &str, fields: &[&str]) -> String {
    let indent = &line[..line.len() - line.trim_start().len()];
    format!("{indent}{key}: {}", fields.join(" | "))
}

/// Split a text into lines and join them back exactly (the last newline kept).
fn lines_of(text: &str) -> Vec<String> {
    text.split('\n').map(String::from).collect()
}

fn lessons_in(dir: &Path) -> Vec<PathBuf> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir).map(|r| r.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "lesson")).collect()).unwrap_or_default();
    v.sort();
    v
}

/// The lesson files named by the arguments: a course (a folder of data/school), a folder, or a file; none — every
/// primary course (`knowledge::course_names`: no philosophy, no programming, no maths `lessons`).
pub fn lesson_files(args: &[String]) -> Result<Vec<PathBuf>> {
    let names: Vec<String> = if args.is_empty() { crate::knowledge::course_names().into_iter().map(String::from).collect() } else { args.to_vec() };
    let mut v = Vec::new();
    for a in &names {
        let p = Path::new(a);
        let r = school_root().join(a);
        if p.is_file() {
            v.push(p.to_path_buf());
        } else if p.is_dir() {
            v.extend(lessons_in(p));
        } else if r.is_dir() {
            v.extend(lessons_in(&r));
        } else if r.is_file() {
            v.push(r);
        } else {
            bail!("«{a}» is neither a course of data/school, nor a folder, nor a file");
        }
    }
    Ok(v)
}

/// A file's name for the reports: relative to data/school when it lies there.
fn show(p: &Path) -> String {
    p.strip_prefix(school_root()).unwrap_or(p).display().to_string()
}

/// One lesson file named in an edit or a restore: a path, a path under data/school, or a bare file name found in
/// exactly one course.
pub fn resolve_file(s: &str) -> Result<PathBuf> {
    let p = Path::new(s);
    if p.is_file() {
        return Ok(p.to_path_buf());
    }
    let r = school_root().join(s);
    if r.is_file() {
        return Ok(r);
    }
    if !s.contains('/') {
        let mut found = Vec::new();
        for d in std::fs::read_dir(school_root())?.flatten().map(|e| e.path()).filter(|p| p.is_dir()) {
            let f = d.join(s);
            if f.is_file() {
                found.push(f);
            }
        }
        found.sort();
        if found.len() == 1 {
            return Ok(found.remove(0));
        }
        if found.len() > 1 {
            bail!("«{s}» is in several courses: {}", found.iter().map(|f| show(f)).collect::<Vec<_>>().join(", "));
        }
    }
    bail!("no lesson file «{s}»")
}

// ---- balance ----------------------------------------------------------------------------------------------------

/// How often the right answer is strictly the longest (l) or strictly the shortest (s) of its options; n questions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub n: usize,
    pub l: usize,
    pub s: usize,
}

impl Tally {
    fn add(&mut self, o: Tally) {
        self.n += o.n;
        self.l += o.l;
        self.s += o.s;
    }
    fn pct(k: usize, n: usize) -> String {
        if n == 0 { "-".into() } else { format!("{:.1}%", 100.0 * k as f64 / n as f64) }
    }
    pub fn line(&self) -> String {
        format!("n {}, L {} ({}), S {} ({})", self.n, self.l, Tally::pct(self.l, self.n), self.s, Tally::pct(self.s, self.n))
    }
}

/// The tally of a lesson's text: every option line with a right and at least one wrong answer.
pub fn tally(text: &str) -> Tally {
    let mut t = Tally::default();
    for line in text.lines() {
        let Some((_, f)) = option_line(line) else { continue };
        if f.len() < 3 {
            continue;
        }
        let r = chars(f[1]);
        let ws: Vec<usize> = f[2..].iter().map(|w| chars(w)).collect();
        t.n += 1;
        t.l += usize::from(ws.iter().all(|&w| r > w));
        t.s += usize::from(ws.iter().all(|&w| r < w));
    }
    t
}

/// A fraction, exact: «1/6» or a decimal «0.25» (to a millionth).
#[derive(Clone, Copy, Debug)]
pub struct Frac(u64, u64);

impl Frac {
    pub fn parse(s: &str) -> Result<Frac> {
        let s = s.trim();
        if let Some((a, b)) = s.split_once('/') {
            let (a, b): (u64, u64) = (a.trim().parse()?, b.trim().parse()?);
            if b == 0 {
                bail!("«{s}»: a zero denominator");
            }
            return Ok(Frac(a, b));
        }
        let x: f64 = s.parse().with_context(|| format!("«{s}» is not a fraction"))?;
        if !(0.0..=1.0).contains(&x) {
            bail!("«{s}» is not between 0 and 1");
        }
        Ok(Frac((x * 1e6).round() as u64, 1_000_000))
    }
    /// k/n < self
    fn above(&self, k: usize, n: usize) -> bool {
        (k as u128) * (self.1 as u128) < (self.0 as u128) * (n as u128)
    }
    /// k/n > self
    fn below(&self, k: usize, n: usize) -> bool {
        (k as u128) * (self.1 as u128) > (self.0 as u128) * (n as u128)
    }
    fn show(&self) -> String {
        if self.1 == 1_000_000 { format!("{}", self.0 as f64 / 1e6) } else { format!("{}/{}", self.0, self.1) }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Band {
    pub l_lo: Frac,
    pub l_hi: Frac,
    pub s_lo: Frac,
    pub s_hi: Frac,
}

impl Default for Band {
    fn default() -> Self {
        Band { l_lo: Frac(1, 6), l_hi: Frac(1, 3), s_lo: Frac(1, 10), s_hi: Frac(1, 3) }
    }
}

impl Band {
    pub fn parse(s: &str) -> Result<Band> {
        let v: Vec<Frac> = s.split(',').map(Frac::parse).collect::<Result<_>>()?;
        if v.len() != 4 {
            bail!("--band needs four fractions: L_lo,L_hi,S_lo,S_hi");
        }
        Ok(Band { l_lo: v[0], l_hi: v[1], s_lo: v[2], s_hi: v[3] })
    }
    /// What is off in a lesson (empty: inside the band).
    pub fn off(&self, t: &Tally) -> Vec<&'static str> {
        let mut v = Vec::new();
        if t.n == 0 {
            return v;
        }
        if t.s == 0 {
            v.push("right never shortest");
        } else if self.s_lo.above(t.s, t.n) {
            v.push("right-shortest too rare");
        } else if self.s_hi.below(t.s, t.n) {
            v.push("right-shortest too often");
        }
        if self.l_lo.above(t.l, t.n) {
            v.push("right-longest too rare");
        } else if self.l_hi.below(t.l, t.n) {
            v.push("right-longest too often");
        }
        v
    }
    fn show(&self) -> String {
        format!("L in [{}, {}], S in [{}, {}], S > 0", self.l_lo.show(), self.l_hi.show(), self.s_lo.show(), self.s_hi.show())
    }
}

/// The balance of the given files: the report, the rows of every lesson (file, L, S, n, off) and the rows outside.
pub fn balance(files: &[PathBuf], band: &Band) -> Result<(String, Vec<String>, Vec<String>)> {
    let mut by_course: BTreeMap<String, Tally> = BTreeMap::new();
    let mut all = Tally::default();
    let mut rows = Vec::new();
    let mut off_rows = Vec::new();
    let mut off_lines = String::new();
    for f in files {
        let t = tally(&std::fs::read_to_string(f).with_context(|| format!("reading {}", f.display()))?);
        let course = f.parent().and_then(|p| p.file_name()).map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
        by_course.entry(course).or_default().add(t);
        all.add(t);
        let off = band.off(&t);
        let row = format!("{}\t{}\t{}\t{}\t{}", show(f), t.l, t.s, t.n, off.join("; "));
        if !off.is_empty() {
            off_lines += &format!("  {}: {} — {}\n", show(f), t.line(), off.join(", "));
            off_rows.push(row.clone());
        }
        rows.push(row);
    }
    let mut out = String::new();
    for (c, t) in &by_course {
        out += &format!("{c}: {}\n", t.line());
    }
    out += &format!("in all: {} lessons, {} questions; right strictly longest {} ({}), strictly shortest {} ({})\n", files.len(), all.n, all.l, Tally::pct(all.l, all.n), all.s, Tally::pct(all.s, all.n));
    out += &format!("band {}: {} lessons outside\n{off_lines}", band.show(), off_rows.len());
    Ok((out, rows, off_rows))
}

/// Write rows into chunk files `<prefix>001<ext>`, … of n rows each; returns the files.
pub fn write_chunks(rows: &[String], n: usize, prefix: &str, ext: &str) -> Result<Vec<PathBuf>> {
    if n == 0 {
        bail!("--chunk/--split needs a number above 0");
    }
    let mut v = Vec::new();
    for (k, c) in rows.chunks(n).enumerate() {
        let p = PathBuf::from(format!("{prefix}{:03}{ext}", k + 1));
        if let Some(d) = p.parent().filter(|d| !d.as_os_str().is_empty()) {
            std::fs::create_dir_all(d)?;
        }
        std::fs::write(&p, c.iter().map(|r| format!("{r}\n")).collect::<String>())?;
        v.push(p);
    }
    Ok(v)
}

// ---- lint -------------------------------------------------------------------------------------------------------

/// A finding of the lint: line number (1-based), kind, detail.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub line: usize,
    pub kind: &'static str,
    pub detail: String,
}

pub const CASE_ONLY: &str = "case-only duplicate (spelling question?)";
pub const GLUED: &str = "glued (heuristic)";

/// The spacing around `|` in an option line's value is right: exactly one space on each side of every bar.
fn spacing_ok(line: &str) -> bool {
    let t = line.trim();
    let Some((_, v)) = t.split_once(':') else { return true };
    let segs: Vec<&str> = v.split('|').collect();
    let k = segs.len();
    segs.iter().enumerate().all(|(i, s)| {
        let start_ok = i == 0 || (s.starts_with(' ') && !s.starts_with("  "));
        let end_ok = i + 1 == k || (s.ends_with(' ') && !s.ends_with("  "));
        start_ok && end_ok
    }) && line.trim_end() == line.trim_end_matches(['\r'])
}

/// The lint of one lesson's text.
pub fn lint_text(text: &str) -> Vec<Finding> {
    let mut v = Vec::new();
    for (i, line) in text.lines().enumerate() {
        let n = i + 1;
        if line.contains('\t') {
            v.push(Finding { line: n, kind: "tab", detail: "a stray tab".into() });
        }
        let Some((_, f)) = option_line(line) else { continue };
        if f.len() != 5 {
            v.push(Finding { line: n, kind: "fields", detail: format!("{} fields, not 5 (question, right, three wrong)", f.len()) });
        }
        if f.iter().any(|x| x.is_empty()) {
            v.push(Finding { line: n, kind: "empty field", detail: "an empty field".into() });
        }
        if !spacing_ok(line) {
            v.push(Finding { line: n, kind: "spacing", detail: "not exactly one space on each side of a |".into() });
        }
        let opts = &f[1.min(f.len())..];
        for a in 0..opts.len() {
            for b in a + 1..opts.len() {
                if opts[a].is_empty() {
                    continue;
                }
                if opts[a] == opts[b] {
                    let kind = if a == 0 { "wrong equals right" } else { "duplicate option" };
                    v.push(Finding { line: n, kind, detail: format!("«{}»", opts[b]) });
                } else if opts[a].to_lowercase() == opts[b].to_lowercase() {
                    v.push(Finding { line: n, kind: CASE_ONLY, detail: format!("«{}» / «{}»", opts[a], opts[b]) });
                }
            }
        }
        // a wrong option glued from other options with «and» or «;» («X and Y», «X; Y»), every part another option
        // (a part alone, such as «9» beside «9 and a half», is an honest distractor and is not reported)
        let wrongs: Vec<&str> = f.iter().skip(2).copied().collect();
        for w in &wrongs {
            let parts: Vec<String> = w.split("; ").flat_map(|p| p.split(" and ")).map(|p| p.trim().trim_end_matches(['.', ',']).to_lowercase()).collect();
            let others: Vec<String> = f.iter().skip(1).filter(|o| *o != w && !o.is_empty()).map(|o| o.to_lowercase()).collect();
            if parts.len() >= 2 && parts.iter().all(|p| others.contains(p)) {
                v.push(Finding { line: n, kind: GLUED, detail: format!("«{w}» is glued from the other options") });
            }
        }
    }
    v
}

/// The lesson text with the spaces around `|` and at the field ends normalised; nothing else changes.
pub fn fix_spaces(text: &str) -> String {
    lines_of(text)
        .iter()
        .map(|line| match option_line(line) {
            Some((k, f)) => {
                let cr = if line.ends_with('\r') { "\r" } else { "" };
                format!("{}{cr}", rebuild(line, k, &f))
            }
            None => line.clone(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn lint(files: &[PathBuf], fix: bool) -> Result<String> {
    // errors; case-only duplicates (often a spelling question on purpose); glued options (heuristic, often honest)
    let mut sections: [String; 3] = Default::default();
    let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
    let mut fixed = 0;
    for f in files {
        let mut text = std::fs::read_to_string(f).with_context(|| format!("reading {}", f.display()))?;
        if fix {
            let t = fix_spaces(&text);
            if t != text {
                std::fs::write(f, &t)?;
                fixed += 1;
                text = t;
            }
        }
        for x in lint_text(&text) {
            *kinds.entry(x.kind).or_default() += 1;
            let k = if x.kind == CASE_ONLY { 1 } else if x.kind == GLUED { 2 } else { 0 };
            sections[k] += &format!("  {}:{}: {} — {}\n", show(f), x.line, x.kind, x.detail);
        }
    }
    let count = |k: &str| kinds.get(k).copied().unwrap_or(0);
    let is_error = |k: &&str| *k != CASE_ONLY && *k != GLUED;
    let errors: usize = kinds.iter().filter(|(k, _)| is_error(k)).map(|x| x.1).sum();
    let mut r = format!("{} lesson files: {errors} errors, {} case-only duplicates, {} glued options (heuristic){}\n", files.len(), count(CASE_ONLY), count(GLUED), if fix { format!("; spaces fixed in {fixed} files") } else { String::new() });
    for (k, n) in kinds.iter().filter(|(k, _)| is_error(k)) {
        r += &format!("  {k}: {n}\n");
    }
    r += &sections[0];
    if !sections[1].is_empty() {
        r += &format!("case-only duplicates (a spelling question? check by hand):\n{}", sections[1]);
    }
    if !sections[2].is_empty() {
        r += &format!("glued options (heuristic, report only — «X and Y» is often an honest distractor):\n{}", sections[2]);
    }
    Ok(r)
}

// ---- edit -------------------------------------------------------------------------------------------------------

/// One edit: line (1-based), the old option text, the new.
#[derive(Clone, Debug)]
pub struct Edit {
    pub line: usize,
    pub old: String,
    pub new: String,
}

/// Apply the edits to a lesson's text: the new text, or every reason it is refused (then nothing is applied).
pub fn apply_edits(text: &str, edits: &[Edit]) -> std::result::Result<String, Vec<String>> {
    let mut lines = lines_of(text);
    let mut bad = Vec::new();
    for e in edits {
        let at = format!("line {}", e.line);
        if e.line == 0 || e.line > lines.len() {
            bad.push(format!("{at}: no such line"));
            continue;
        }
        let line = lines[e.line - 1].clone();
        let Some((key, before)) = option_line(&line) else {
            bad.push(format!("{at}: not an option line"));
            continue;
        };
        let (old, new) = (e.old.trim(), e.new.trim());
        if new.is_empty() || new.contains(['|', '\t', '\n', '\r']) {
            bad.push(format!("{at}: the new text «{new}» is empty or holds |, a tab or a newline"));
            continue;
        }
        let at_pos: Vec<usize> = (1..before.len()).filter(|&i| before[i] == old).collect();
        if at_pos.is_empty() {
            bad.push(format!("{at}: «{old}» is not an option there"));
            continue;
        }
        if at_pos.len() > 1 {
            bad.push(format!("{at}: «{old}» is more than one option there"));
            continue;
        }
        if old == new {
            bad.push(format!("{at}: «{old}» would not change"));
            continue;
        }
        let mut after = before.clone();
        after[at_pos[0]] = new;
        if after.len() != 5 {
            bad.push(format!("{at}: {} wrong options, not 3", after.len().saturating_sub(2)));
            continue;
        }
        if after[0] != before[0] {
            bad.push(format!("{at}: the question would change"));
            continue;
        }
        if at_pos[0] != 1 && after[1] != before[1] {
            bad.push(format!("{at}: the right answer would stop being first"));
            continue;
        }
        let low: Vec<String> = after[1..].iter().map(|x| x.to_lowercase()).collect();
        if low.iter().collect::<BTreeSet<_>>().len() != low.len() {
            bad.push(format!("{at}: «{new}» would duplicate another option"));
            continue;
        }
        let cr = if line.ends_with('\r') { "\r" } else { "" };
        lines[e.line - 1] = format!("{}{cr}", rebuild(&line, key, &after));
    }
    if bad.is_empty() { Ok(lines.join("\n")) } else { Err(bad) }
}

/// Read an edits table: file → its edits, in the table's order.
pub fn read_edits(text: &str) -> Result<BTreeMap<PathBuf, Vec<Edit>>> {
    let mut m: BTreeMap<PathBuf, Vec<Edit>> = BTreeMap::new();
    for (i, l) in text.lines().enumerate() {
        let l = l.trim_end_matches('\r');
        if l.trim().is_empty() || l.starts_with('#') {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 4 {
            bail!("edits row {}: {} fields, not 4 (file, line, old option, new option)", i + 1, c.len());
        }
        let line: usize = c[1].trim().parse().with_context(|| format!("edits row {}: «{}» is not a line number", i + 1, c[1]))?;
        m.entry(resolve_file(c[0].trim())?).or_default().push(Edit { line, old: c[2].into(), new: c[3].into() });
    }
    Ok(m)
}

pub fn edit(table: &Path, dry: bool) -> Result<String> {
    let m = read_edits(&std::fs::read_to_string(table).with_context(|| format!("reading {}", table.display()))?)?;
    let mut out = String::new();
    let (mut done, mut refused) = (0, 0);
    for (f, es) in &m {
        let text = std::fs::read_to_string(f)?;
        match apply_edits(&text, es) {
            Ok(t) => {
                if !dry {
                    std::fs::write(f, &t)?;
                }
                done += 1;
                out += &format!("{} {}: {} edits; before {}; after {}\n", if dry { "would edit" } else { "edited" }, show(f), es.len(), tally(&text).line(), tally(&t).line());
            }
            Err(bad) => {
                refused += 1;
                out += &format!("REFUSED {} (nothing applied):\n{}", show(f), bad.iter().map(|b| format!("  {b}\n")).collect::<String>());
            }
        }
    }
    out += &format!("{done} files {}, {refused} refused\n", if dry { "would be edited (dry run)" } else { "edited" });
    if refused > 0 {
        print!("{out}");
        bail!("{refused} files refused");
    }
    Ok(out)
}

// ---- snapshot and restore ---------------------------------------------------------------------------------------

/// Where a lesson file's copy lies in a snapshot: <dir>/<course>/<file>.
fn snapshot_path(dir: &Path, f: &Path) -> PathBuf {
    let course = f.parent().and_then(|p| p.file_name()).map(PathBuf::from).unwrap_or_default();
    dir.join(course).join(f.file_name().unwrap_or_default())
}

pub fn snapshot(files: &[PathBuf], dir: &Path) -> Result<String> {
    for f in files {
        let to = snapshot_path(dir, f);
        std::fs::create_dir_all(to.parent().unwrap())?;
        std::fs::copy(f, &to).with_context(|| format!("copying {} to {}", f.display(), to.display()))?;
    }
    Ok(format!("{} lesson files saved in {}\n", files.len(), dir.display()))
}

/// Line numbers from «12» and «10-14».
pub fn line_numbers(args: &[String]) -> Result<Vec<usize>> {
    let mut v = Vec::new();
    for a in args {
        if let Some((x, y)) = a.split_once('-') {
            let (x, y): (usize, usize) = (x.parse()?, y.parse()?);
            if x == 0 || y < x {
                bail!("«{a}» is not a range of lines");
            }
            v.extend(x..=y);
        } else {
            let x: usize = a.parse().with_context(|| format!("«{a}» is not a line number"))?;
            if x == 0 {
                bail!("lines count from 1");
            }
            v.push(x);
        }
    }
    v.sort();
    v.dedup();
    Ok(v)
}

/// Put back the given lines of a file from its snapshot copy; the copy must have as many lines as the file.
pub fn restore_lines(dir: &Path, f: &Path, lines: &[usize]) -> Result<String> {
    let copy = snapshot_path(dir, f);
    let saved = lines_of(&std::fs::read_to_string(&copy).with_context(|| format!("no snapshot copy {}", copy.display()))?);
    let mut now = lines_of(&std::fs::read_to_string(f)?);
    if saved.len() != now.len() {
        bail!("{} has {} lines and its snapshot {}: the lines do not match, nothing restored", show(f), now.len(), saved.len());
    }
    if let Some(&x) = lines.iter().find(|&&x| x > now.len()) {
        bail!("{} has no line {x}, nothing restored", show(f));
    }
    let mut out = String::new();
    let mut changed = 0;
    for &x in lines {
        if now[x - 1] != saved[x - 1] {
            out += &format!("  {x}: «{}» → «{}»\n", now[x - 1], saved[x - 1]);
            now[x - 1] = saved[x - 1].clone();
            changed += 1;
        }
    }
    std::fs::write(f, now.join("\n"))?;
    Ok(format!("{}: {changed} of {} lines restored\n{out}", show(f), lines.len()))
}

// ---- the dictionary ---------------------------------------------------------------------------------------------

/// A field that is or starts with a date (2026-10-10, 10.10.2026, 10.10).
fn looks_like_date(f: &str) -> bool {
    let b: Vec<char> = f.trim().chars().collect();
    let d = |i: usize| b.get(i).is_some_and(char::is_ascii_digit);
    let iso = b.len() >= 10 && (0..4).all(d) && b[4] == '-' && d(5) && d(6) && b[7] == '-' && d(8) && d(9);
    let dotted = {
        let parts: Vec<&str> = f.trim().split('.').collect();
        (2..=3).contains(&parts.len()) && parts.iter().all(|p| !p.is_empty() && p.len() <= 4 && p.chars().all(|c| c.is_ascii_digit())) && parts[0].len() <= 2 && parts[1].len() <= 2
    };
    iso || dotted
}

/// Validate a records file: the good records (word, the line) and every problem.
pub fn validate_records(bytes: &[u8], name: &str) -> (Vec<(String, String)>, Vec<String>) {
    let text = match std::str::from_utf8(bytes) {
        Ok(t) => t,
        Err(e) => return (Vec::new(), vec![format!("{name}: not UTF-8 ({e})")]),
    };
    let mut good = Vec::new();
    let mut bad = Vec::new();
    for (i, l) in text.split('\n').enumerate() {
        let at = format!("{name}:{}", i + 1);
        if l.trim().is_empty() || l.starts_with('#') {
            continue;
        }
        if l.contains('\r') {
            bad.push(format!("{at}: a carriage return"));
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 6 {
            bad.push(format!("{at}: {} TAB fields, not 6 (a tab inside a field?)", c.len()));
            continue;
        }
        let w = c[0];
        if w.trim() != w || w.is_empty() {
            bad.push(format!("{at}: the word «{w}» is empty or has spaces at its ends"));
        } else if w.to_lowercase() != w {
            bad.push(format!("{at}: the word «{w}» is not lower-case"));
        } else if c.iter().any(|f| f.trim().is_empty()) {
            bad.push(format!("{at}: «{w}» has an empty field"));
        } else if c.iter().any(|f| looks_like_date(f)) {
            bad.push(format!("{at}: «{w}» carries a date; records never do"));
        } else {
            good.push((w.to_string(), l.to_string()));
        }
    }
    (good, bad)
}

/// Merge records into the dictionary file: the report; refuses everything if a record is bad.
pub fn merge(inputs: &[PathBuf], into: &Path, check: bool) -> Result<String> {
    let have_text = std::fs::read_to_string(into).with_context(|| format!("reading {}", into.display()))?;
    let have: BTreeSet<String> = have_text.lines().filter(|l| !l.starts_with('#')).filter_map(|l| l.split('\t').next()).map(|w| w.to_lowercase()).collect();
    let mut bad = Vec::new();
    let mut new: Vec<(String, String)> = Vec::new();
    let mut skipped = Vec::new();
    let mut repeated = Vec::new();
    let mut seen = BTreeSet::new();
    let mut good_n = 0;
    for p in inputs {
        let (good, b) = validate_records(&std::fs::read(p).with_context(|| format!("reading {}", p.display()))?, &p.display().to_string());
        bad.extend(b);
        good_n += good.len();
        for (w, l) in good {
            if have.contains(&w) {
                skipped.push(w);
            } else if !seen.insert(w.clone()) {
                repeated.push(w);
            } else {
                new.push((w, l));
            }
        }
    }
    let mut out = format!("{good_n} good records, {} bad; {} new, {} already in the dictionary, {} repeated in the input\n", bad.len(), new.len(), skipped.len(), repeated.len());
    if !skipped.is_empty() {
        let more = if skipped.len() > 60 { format!(" … and {} more", skipped.len() - 60) } else { String::new() };
        out += &format!("already in (skipped): {}{more}\n", skipped[..skipped.len().min(60)].join(" "));
    }
    if !repeated.is_empty() {
        out += &format!("repeated (the first kept): {}\n", repeated.join(" "));
    }
    for b in &bad {
        out += &format!("  {b}\n");
    }
    if !bad.is_empty() {
        print!("{out}");
        bail!("{} bad records: nothing merged", bad.len());
    }
    if check {
        return Ok(out + "checked only, nothing merged\n");
    }
    let mut t = have_text;
    if !t.is_empty() && !t.ends_with('\n') {
        t.push('\n');
    }
    for (_, l) in &new {
        t += l;
        t.push('\n');
    }
    std::fs::write(into, t)?;
    Ok(out + &format!("{} records appended to {}\n", new.len(), into.display()))
}

// ---- arguments --------------------------------------------------------------------------------------------------

/// Take `--name value` out of the arguments.
fn take_value(args: &mut Vec<String>, name: &str) -> Result<Option<String>> {
    let Some(i) = args.iter().position(|a| a == name) else { return Ok(None) };
    if i + 1 >= args.len() {
        bail!("{name} needs a value");
    }
    let v = args.remove(i + 1);
    args.remove(i);
    Ok(Some(v))
}

fn take_switch(args: &mut Vec<String>, name: &str) -> bool {
    let k = args.len();
    args.retain(|a| a != name);
    args.len() != k
}

fn chunking(args: &mut Vec<String>, flag: &str) -> Result<Option<(usize, String)>> {
    let n = take_value(args, flag)?;
    let p = take_value(args, "--prefix")?;
    match (n, p) {
        (Some(n), Some(p)) => Ok(Some((n.parse().with_context(|| format!("{flag} «{n}» is not a number"))?, p))),
        (None, None) => Ok(None),
        _ => bail!("{flag} N and --prefix p go together"),
    }
}

/// `world knowledge balance|lint|edit|snapshot|restore-lines …`
pub fn knowledge_args(all: &[String]) -> Result<()> {
    let cmd = all[1].as_str();
    let mut args: Vec<String> = all[2..].to_vec();
    if take_switch(&mut args, "--help") || take_switch(&mut args, "-h") {
        print!("{KNOWLEDGE_USAGE}");
        return Ok(());
    }
    match cmd {
        "balance" => {
            let band = take_value(&mut args, "--band")?.map(|b| Band::parse(&b)).transpose()?.unwrap_or_default();
            let list = take_value(&mut args, "--list")?;
            let chunk = chunking(&mut args, "--chunk")?;
            if let Some(a) = args.iter().find(|a| a.starts_with("--")) {
                bail!("unknown option {a}\n{KNOWLEDGE_USAGE}");
            }
            let (out, rows, off) = balance(&lesson_files(&args)?, &band)?;
            print!("{out}");
            if let Some(l) = list {
                if let Some(d) = Path::new(&l).parent().filter(|d| !d.as_os_str().is_empty()) {
                    std::fs::create_dir_all(d)?;
                }
                std::fs::write(&l, format!("# file\tL\tS\tn\toff\n{}", rows.iter().map(|r| format!("{r}\n")).collect::<String>()))?;
                println!("the list of {} lessons: {l}", rows.len());
            }
            if let Some((n, p)) = chunk {
                let fs = write_chunks(&off, n, &p, ".tsv")?;
                println!("{} lessons outside the band in {} chunk files {p}001.tsv…", off.len(), fs.len());
            }
        }
        "lint" => {
            let fix = take_switch(&mut args, "--fix-spaces");
            if let Some(a) = args.iter().find(|a| a.starts_with("--")) {
                bail!("unknown option {a}\n{KNOWLEDGE_USAGE}");
            }
            print!("{}", lint(&lesson_files(&args)?, fix)?);
        }
        "edit" => {
            let dry = take_switch(&mut args, "--dry-run");
            let [t] = args.as_slice() else { bail!("{KNOWLEDGE_USAGE}") };
            print!("{}", edit(Path::new(t), dry)?);
        }
        "snapshot" => {
            if args.len() < 2 {
                bail!("{KNOWLEDGE_USAGE}");
            }
            let dir = PathBuf::from(args.pop().unwrap());
            print!("{}", snapshot(&lesson_files(&args)?, &dir)?);
        }
        "restore-lines" => {
            if args.len() < 3 {
                bail!("{KNOWLEDGE_USAGE}");
            }
            print!("{}", restore_lines(Path::new(&args[0]), &resolve_file(&args[1])?, &line_numbers(&args[2..])?)?);
        }
        _ => bail!("{KNOWLEDGE_USAGE}"),
    }
    Ok(())
}

/// `world dictionary merge|gaps …`
pub fn dictionary_args(all: &[String]) -> Result<()> {
    let cmd = all[1].as_str();
    let mut args: Vec<String> = all[2..].to_vec();
    if take_switch(&mut args, "--help") || take_switch(&mut args, "-h") {
        print!("{DICTIONARY_USAGE}");
        return Ok(());
    }
    match cmd {
        "merge" => {
            let check = take_switch(&mut args, "--check");
            if args.is_empty() || args.iter().any(|a| a.starts_with("--")) {
                bail!("{DICTIONARY_USAGE}");
            }
            let inputs: Vec<PathBuf> = args.iter().map(PathBuf::from).collect();
            print!("{}", merge(&inputs, &dictionary_path(), check)?);
        }
        "gaps" => {
            let split = chunking(&mut args, "--split")?;
            if !args.is_empty() {
                bail!("{DICTIONARY_USAGE}");
            }
            let gaps = crate::dictionary::primary_gaps(&dictionary_path())?;
            match split {
                Some((n, p)) => {
                    let fs = write_chunks(&gaps, n, &p, ".txt")?;
                    println!("{} words of the primary lessons without a record, in {} files {p}001.txt…", gaps.len(), fs.len());
                }
                None => {
                    for w in &gaps {
                        println!("{w}");
                    }
                    println!("{} words of the primary lessons without a record", gaps.len());
                }
            }
        }
        _ => bail!("{DICTIONARY_USAGE}"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("schooltools-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    const LESSON: &str = "course: t\nlesson: 1\ntitle: T\nfact: f\nexercise: Q one? | right answer here | no | nope | nah\ntest: Q two? | ok | a longer wrong | another long wrong | still longer wrong\nexam: Q three? | mid size | tiny | a much longer one | ab\n";

    #[test]
    fn tally_counts_characters_not_bytes() {
        let t = tally(LESSON);
        assert_eq!(t, Tally { n: 3, l: 1, s: 1 });
        // «ééé» is 3 characters (6 bytes): shorter than «abcd» by characters, though longer by bytes
        let t = tally("exercise: q | ééé | abcd | abcde | abcdef\n");
        assert_eq!(t, Tally { n: 1, l: 0, s: 1 });
        // ties are neither strictly longest nor strictly shortest
        assert_eq!(tally("test: q | abc | abc2 | xyz | a\n"), Tally { n: 1, l: 0, s: 0 });
    }

    #[test]
    fn band_reports_what_is_off() {
        let b = Band::default();
        assert!(b.off(&Tally { n: 12, l: 2, s: 2 }).is_empty()); // 1/6 and 1/6: inside, the edges count
        assert_eq!(b.off(&Tally { n: 12, l: 3, s: 0 }), vec!["right never shortest"]);
        assert_eq!(b.off(&Tally { n: 20, l: 1, s: 1 }), vec!["right-shortest too rare", "right-longest too rare"]);
        assert_eq!(b.off(&Tally { n: 12, l: 5, s: 5 }), vec!["right-shortest too often", "right-longest too often"]);
        let b = Band::parse("0.1,0.5,1/20,0.5").unwrap();
        assert!(b.off(&Tally { n: 20, l: 2, s: 1 }).is_empty());
        assert!(Band::parse("1/6,1/3").is_err());
        assert!(Frac::parse("1/0").is_err());
    }

    #[test]
    fn balance_lists_and_chunks() {
        let d = tmp("balance");
        let c = d.join("course");
        std::fs::create_dir_all(&c).unwrap();
        std::fs::write(c.join("001-a.lesson"), LESSON).unwrap();
        std::fs::write(c.join("002-b.lesson"), "exercise: q | longest one | a | b | c\n").unwrap();
        std::fs::write(c.join("003-c.lesson"), "exercise: q | x | aa | bb | cc\nexercise: q | yyy | a | bbbbbb | cc\n").unwrap();
        let files = lesson_files(&[c.display().to_string()]).unwrap();
        assert_eq!(files.len(), 3);
        let (out, rows, off) = balance(&files, &Band::default()).unwrap();
        assert!(out.contains("in all: 3 lessons, 6 questions"), "{out}");
        assert_eq!(rows.len(), 3);
        assert_eq!(off.len(), 2, "{off:?}"); // 002: never shortest, longest too often; 003: never longest
        let fs = write_chunks(&off, 1, &d.join("chunk-").display().to_string(), ".tsv").unwrap();
        assert_eq!(fs.len(), 2);
        assert!(std::fs::read_to_string(&fs[0]).unwrap().contains("002-b.lesson"));
        assert!(write_chunks(&off, 0, "x", ".tsv").is_err());
    }

    #[test]
    fn lint_finds_each_kind() {
        let text = "exercise: q | a | b | c\n\
                    exercise: q | a |  b | c | d\n\
                    exercise: q | a | b|c | d\n\
                    exercise: q | a | b | b | c\n\
                    exercise: q | a | a | b | c\n\
                    exercise: q | Polish | polish | b | c\n\
                    exercise: q | a | cats and dogs | dogs | cats\n\
                    exercise: q | a | b | c | \n\
                    fact: a\tb\n\
                    exercise: q | right | one | two | three\n";
        let f = lint_text(text);
        let kinds: Vec<(usize, &str)> = f.iter().map(|x| (x.line, x.kind)).collect();
        assert!(kinds.contains(&(1, "fields")));
        assert!(kinds.contains(&(2, "spacing")));
        assert!(kinds.contains(&(3, "spacing")));
        assert!(kinds.contains(&(4, "duplicate option")));
        assert!(kinds.contains(&(5, "wrong equals right")));
        assert!(kinds.contains(&(6, CASE_ONLY)));
        assert!(kinds.contains(&(7, "glued (heuristic)")));
        assert!(kinds.contains(&(8, "empty field")));
        assert!(kinds.contains(&(9, "tab")));
        assert!(!kinds.iter().any(|k| k.0 == 10), "{kinds:?}");
        // negative control: a clean lesson has no findings, an honest «9 and a half» beside «9» is not glued
        assert!(lint_text(LESSON).is_empty());
        assert!(lint_text("exercise: q | 10 | 9 and a half | 9 | 11\n").is_empty());
    }

    #[test]
    fn fix_spaces_changes_spaces_only() {
        let t = "title: x  y\nexercise: q  |  a |b | c | d  \n";
        let f = fix_spaces(t);
        assert_eq!(f, "title: x  y\nexercise: q | a | b | c | d\n");
        assert!(lint_text(&f).is_empty());
        assert_eq!(tally(t), tally(&f));
    }

    #[test]
    fn edit_applies_or_refuses_the_whole_file() {
        let e = |line, old: &str, new: &str| Edit { line, old: old.into(), new: new.into() };
        let t = apply_edits(LESSON, &[e(5, "no", "not at all"), e(5, "right answer here", "right")]).unwrap();
        assert!(t.contains("exercise: Q one? | right | not at all | nope | nah\n"));
        assert_eq!(t.lines().count(), LESSON.lines().count());
        for bad in [
            vec![e(1, "t", "u")],                       // not an option line
            vec![e(5, "missing", "x")],                 // not an option there
            vec![e(5, "no", "nope")],                   // a duplicate
            vec![e(5, "no", "Right Answer Here")],      // equal to the right one
            vec![e(5, "no", "a | b")],                  // would add a field
            vec![e(5, "Q one?", "Q changed?")],         // the question is not an option
            vec![e(99, "no", "x")],                     // no such line
            vec![e(5, "no", "fine"), e(6, "absent", "x")], // one bad edit refuses the file
        ] {
            assert!(apply_edits(LESSON, &bad).is_err(), "{bad:?}");
        }
        assert!(apply_edits("exercise: q | a | b | c\n", &[e(1, "b", "x")]).is_err()); // two wrong options
        assert!(apply_edits("exercise: q | a | b | b | c\n", &[e(1, "b", "x")]).is_err()); // ambiguous
    }

    #[test]
    fn edit_command_and_dry_run() {
        let d = tmp("edit");
        let c = d.join("course");
        std::fs::create_dir_all(&c).unwrap();
        let f = c.join("001-a.lesson");
        std::fs::write(&f, LESSON).unwrap();
        let tsv = d.join("edits.tsv");
        std::fs::write(&tsv, format!("# file\tline\told\tnew\n{}\t5\tno\tnot at all\n", f.display())).unwrap();
        edit(&tsv, true).unwrap();
        assert_eq!(std::fs::read_to_string(&f).unwrap(), LESSON);
        let out = edit(&tsv, false).unwrap();
        assert!(out.contains("before n 3"), "{out}");
        assert!(std::fs::read_to_string(&f).unwrap().contains("| not at all |"));
        std::fs::write(&tsv, format!("{}\t5\tgone\tx\n", f.display())).unwrap();
        let before = std::fs::read_to_string(&f).unwrap();
        assert!(edit(&tsv, false).is_err());
        assert_eq!(std::fs::read_to_string(&f).unwrap(), before);
        std::fs::write(&tsv, format!("{}\t5\tonly three fields\n", f.display())).unwrap();
        assert!(edit(&tsv, false).is_err());
    }

    #[test]
    fn snapshot_and_restore_only_the_given_lines() {
        let d = tmp("snap");
        let c = d.join("course");
        std::fs::create_dir_all(&c).unwrap();
        let (a, b) = (c.join("001-a.lesson"), c.join("002-b.lesson"));
        std::fs::write(&a, "l1\nl2\nl3\nl4\n").unwrap();
        std::fs::write(&b, "b1\n").unwrap();
        let s = d.join("snapshot");
        snapshot(&lesson_files(&[c.display().to_string()]).unwrap(), &s).unwrap();
        assert!(s.join("course").join("002-b.lesson").is_file());
        // my edits to lines 1 and 3, a neighbour's to line 4 and to the other file
        std::fs::write(&a, "mine1\nl2\nmine3\nneighbour4\n").unwrap();
        std::fs::write(&b, "neighbour\n").unwrap();
        let out = restore_lines(&s, &a, &line_numbers(&["1".into(), "3-3".into()]).unwrap()).unwrap();
        assert!(out.contains("2 of 2 lines restored"), "{out}");
        assert_eq!(std::fs::read_to_string(&a).unwrap(), "l1\nl2\nl3\nneighbour4\n");
        assert_eq!(std::fs::read_to_string(&b).unwrap(), "neighbour\n");
        // a file whose lines moved is refused
        std::fs::write(&a, "x\nl1\nl2\nl3\nl4\n").unwrap();
        assert!(restore_lines(&s, &a, &[1]).is_err());
        assert!(line_numbers(&["0".into()]).is_err());
        assert_eq!(line_numbers(&["4-6".into(), "5".into(), "2".into()]).unwrap(), vec![2, 4, 5, 6]);
    }

    #[test]
    fn records_are_validated() {
        let good = "word\tnoun\tmeaning\tsense\texample | example\tnote\n";
        assert_eq!(validate_records(good.as_bytes(), "t").0.len(), 1);
        for bad in [
            "Word\tnoun\tm\ts\te\tn\n",
            "word\tnoun\tm\ts\te\n",
            "word\tnoun\tm\ts\te\tn\t2026-10-10\n",
            "2026-10-10\tword\tnoun\tm\ts\te\n",
            "word\tnoun\tm\t10.10\te\tn\n",
            "word\tnoun\t\ts\te\tn\n",
            "word\tnoun\tm\ts\te\tn\r\n",
        ] {
            let (g, b) = validate_records(bad.as_bytes(), "t");
            assert!(g.is_empty() && b.len() == 1, "{bad:?}: {b:?}");
        }
        assert!(!validate_records(&[0xff, b'\n'], "t").1.is_empty());
        assert!(!looks_like_date("about 10.5 metres") && looks_like_date("10.10.2026"));
    }

    #[test]
    fn the_dictionary_on_disk_validates() {
        let (good, bad) = validate_records(&std::fs::read(dictionary_path()).unwrap(), "dictionary-improved.tsv");
        assert!(bad.is_empty(), "{:?}", &bad[..bad.len().min(5)]);
        assert!(good.len() > 10_000);
    }

    #[test]
    fn merge_skips_known_and_refuses_bad() {
        let d = tmp("merge");
        let into = d.join("dict.tsv");
        std::fs::write(&into, "# header\nold\tn\tm\ts\te\tn").unwrap();
        let a = d.join("a.tsv");
        std::fs::write(&a, "old\tn\tm\ts\te\tn\nnew\tn\tm\ts\te\tn\nnew\tn\tm2\ts\te\tn\n").unwrap();
        merge(&[a.clone()], &into, true).unwrap();
        assert_eq!(std::fs::read_to_string(&into).unwrap().lines().count(), 2);
        let out = merge(&[a.clone()], &into, false).unwrap();
        assert!(out.contains("1 new, 1 already in the dictionary, 1 repeated"), "{out}");
        assert_eq!(std::fs::read_to_string(&into).unwrap(), "# header\nold\tn\tm\ts\te\tn\nnew\tn\tm\ts\te\tn\n");
        let b = d.join("b.tsv");
        std::fs::write(&b, "fresh\tn\tm\ts\te\tn\nBad\tn\tm\ts\te\tn\n").unwrap();
        assert!(merge(&[b], &into, false).is_err());
        assert!(!std::fs::read_to_string(&into).unwrap().contains("fresh"));
    }
}
