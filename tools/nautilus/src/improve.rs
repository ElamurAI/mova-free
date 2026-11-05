//! The `layer` source: improve an existing dictionary layer in place, as review-only proposals.
//!
//! Nautilus reads the current rows of a layer file (read-only; it never writes it), finds natively the entries whose
//! meaning, sense or examples are missing or weak, and asks its teacher only for those, ten at a time. The answer
//! becomes proposal lines in the format the dictionary builder reads (`target`), so that admission — a separate,
//! reviewed step — can merge them without conversion.
//!
//! Formats (the columns of the files in `dictionaries/` of the school data and the pupils' records):
//!
//! | format | columns |
//! |---|---|
//! | `records` | word, part of speech, meaning for a pupil, sense in the lessons, examples, note (and any extra columns, kept) |
//! | `judaism-senses` | word, part of speech, meaning, sense in the Tanakh and Jewish tradition, interpretation, examples |
//! | `emode-senses` | word, kind, modern equivalent, part of speech, meaning for a pupil, how it differs from today, examples, forms, tests |
//! | `mystic-senses` | word, part of speech, meaning for a pupil, sense in the mystic school |
//!
//! A row is kept as its exact columns, so a row read and rendered again is the same bytes. Part of speech and meaning
//! of a `judaism-senses` or `mystic-senses` proposal stay empty when the pupils' dictionary has the word (the builder
//! then uses the base record), as in the builder's own files.
//!
//! **Weak** (natively, per format): `records` — a meaning under 40 characters, a sense under 80, or fewer than three
//! examples; `judaism-senses` — a word without a primary record and without its own part of speech or meaning (the
//! builder leaves such a word out of the layer), or a word with two or more corpus examples and neither a sense nor an
//! interpretation; `emode-senses` — an empty modern equivalent, meaning or difference; `mystic-senses` — a sense
//! under 80 characters, or a word without a primary record and without its own meaning. Rows with fewer than two
//! corpus examples (`judaism-senses`, `emode-senses`) are counted as thin (`thin`) but not asked: their examples are
//! quotations of the corpora, which a teacher does not write.
//! A weak entry is asked only with evidence: at least one example, or a sense to start from. An entry is asked once
//! (the item keys of the journal or of the job's ledger hold its word).
//!
//! **Teacher fields** (`-` keeps the current value): `records` part of speech, meaning, sense, new example sentences
//! (` | `-separated, written for a pupil, appended to the current ones); `judaism-senses` part of speech, meaning,
//! sense, interpretation; `emode-senses` part of speech, modern equivalent, meaning, difference; `mystic-senses` part
//! of speech, meaning, sense. Everything else of a proposal line (examples quoted from the corpus, kind, forms, tests,
//! note) is the current row's, never the teacher's.
use crate::layer::Base;
use crate::work::{Item, Job, Native};
use std::collections::{BTreeMap, BTreeSet};

pub const FORMATS: [&str; 4] = ["records", "judaism-senses", "emode-senses", "mystic-senses"];
/// Below this many characters a meaning is weak.
pub const WEAK_CHARS: usize = 40;
/// Below this many characters a sense is weak.
pub const WEAK_SENSE: usize = 80;
/// A field written by the teacher: 2..=1200 characters (a part of speech may be short).
const FIELD_MAX: usize = 1200;

/// The columns a format's rows have (at least, for `records`).
pub fn columns(format: &str) -> Option<usize> {
    match format {
        "records" => Some(6),
        "judaism-senses" => Some(6),
        "emode-senses" => Some(9),
        "mystic-senses" => Some(4),
        _ => None,
    }
}

/// One row as its exact columns.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Raw(pub Vec<String>);

impl Raw {
    pub fn word(&self) -> &str {
        self.0.first().map_or("", |s| s.as_str())
    }
    fn col(&self, i: usize) -> &str {
        self.0.get(i).map_or("", |s| s.as_str())
    }
    pub fn line(&self) -> String {
        self.0.join("\t")
    }
}

/// The rows of a layer file: every non-`#`, non-empty line split at tabs, with its exact columns.
pub fn parse(text: &str) -> Vec<Raw> {
    text.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).map(|l| Raw(l.split('\t').map(String::from).collect())).collect()
}

/// The `#` header lines of a file declare CC BY-SA (`# License: CC BY-SA …` or any header naming it).
pub fn declares_cc_by_sa(text: &str) -> bool {
    text.lines().take_while(|l| l.starts_with('#')).any(|l| {
        let u = l.to_uppercase();
        u.contains("CC BY-SA") || u.contains("CC-BY-SA")
    })
}

/// A row's examples (` | `-separated) in its format's examples column; none for `mystic-senses`.
pub fn examples_of(format: &str, r: &Raw) -> Vec<String> {
    let c = match format {
        "records" => 4,
        "judaism-senses" => 5,
        "emode-senses" => 6,
        _ => return vec![],
    };
    r.col(c).split(" | ").map(str::trim).filter(|e| !e.is_empty()).map(String::from).collect()
}

fn short(s: &str) -> bool {
    s.trim().chars().count() < WEAK_CHARS
}
fn short_sense(s: &str) -> bool {
    s.trim().chars().count() < WEAK_SENSE
}

/// A row whose corpus examples are fewer than two (quotations a teacher does not write): reported, not asked.
pub fn thin(format: &str, r: &Raw) -> bool {
    matches!(format, "judaism-senses" | "emode-senses") && examples_of(format, r).len() < 2
}

/// Why a row is weak, natively; `None` when it is not.
pub fn weakness(format: &str, r: &Raw, base: &Base) -> Option<String> {
    let has_base = base.contains_key(&r.word().to_lowercase());
    let mut why = Vec::new();
    match format {
        "records" => {
            if short(r.col(2)) {
                why.push("meaning");
            }
            if short_sense(r.col(3)) {
                why.push("sense");
            }
            if examples_of(format, r).len() < 3 {
                why.push("examples");
            }
        }
        "judaism-senses" => {
            if !has_base && (r.col(1).trim().is_empty() || r.col(2).trim().is_empty()) {
                why.push("no primary record and no meaning of its own");
            }
            if r.col(3).trim().is_empty() && r.col(4).trim().is_empty() && examples_of(format, r).len() >= 2 {
                why.push("neither a sense nor an interpretation");
            }
        }
        "emode-senses" => {
            for (i, n) in [(2, "modern equivalent"), (4, "meaning"), (5, "difference")] {
                if r.col(i).trim().is_empty() {
                    why.push(n);
                }
            }
        }
        "mystic-senses" => {
            if short_sense(r.col(3)) {
                why.push("sense");
            }
            if !has_base && r.col(2).trim().is_empty() {
                why.push("no primary record and no meaning of its own");
            }
        }
        _ => {}
    }
    (!why.is_empty()).then(|| why.join(", "))
}

/// How many of a row's examples must use its word (Mova's lemmas) before it is asked: two corpus quotations for
/// `judaism-senses` and `emode-senses`, none for the others.
pub fn min_uses(format: &str) -> usize {
    if matches!(format, "judaism-senses" | "emode-senses") { 2 } else { 0 }
}

/// A weak row with something to start from — an example that uses the word, or a sense — that is not junk
/// (`senses::junk`) and has the uses its format needs (`min_uses`; scan boilerplate and examples that only share a
/// prefix with the word do not count).
pub fn askable(format: &str, r: &Raw, base: &Base) -> bool {
    if weakness(format, r, base).is_none() || crate::senses::junk(r.word()).is_some() {
        return false;
    }
    let uses = crate::senses::uses(r.word(), &examples_of(format, r)).len();
    uses >= min_uses(format) && (uses > 0 || (matches!(format, "mystic-senses" | "records") && !r.col(3).trim().is_empty()))
}

/// The teacher fields asked for a row (indexes into `teacher_fields`): only the weak ones — a part of speech and
/// meaning only for a word without a primary record (`judaism-senses`, `mystic-senses`) or an empty part of speech
/// (`records`); never a field the row already fills well.
pub fn asked_fields(format: &str, r: &Raw, base: &Base) -> Vec<usize> {
    let why = weakness(format, r, base).unwrap_or_default();
    let has = |w: &str| why.split(", ").any(|x| x == w);
    let mut v = Vec::new();
    match format {
        "records" => {
            if r.col(1).trim().is_empty() {
                v.push(0);
            }
            for (i, w) in [(1, "meaning"), (2, "sense"), (3, "examples")] {
                if has(w) {
                    v.push(i);
                }
            }
        }
        "judaism-senses" => {
            if has("no primary record and no meaning of its own") {
                v.extend([0, 1]);
            }
            if has("neither a sense nor an interpretation") {
                v.extend([2, 3]);
            }
        }
        "emode-senses" => {
            for (i, w) in [(1, "modern equivalent"), (2, "meaning"), (3, "difference")] {
                if has(w) {
                    v.push(i);
                }
            }
        }
        "mystic-senses" => {
            if has("no primary record and no meaning of its own") {
                v.extend([0, 1]);
            }
            if has("sense") {
                v.push(2);
            }
        }
        _ => {}
    }
    v
}

/// The fields a sense pick can settle (a sense of the word, not a meaning, an example or a part of speech).
pub fn sense_like(format: &str) -> &'static [usize] {
    match format {
        "records" => &[2],
        "judaism-senses" => &[2, 3],
        "mystic-senses" => &[2],
        _ => &[],
    }
}

/// A row's native plan (`senses::plan`): the pupils' numbered senses when the dictionary has the word, the store's
/// fixed sense, and a native pick only when every asked field is one a pick settles.
pub fn plan_row(format: &str, r: &Raw, base: &Base, fixed: &BTreeMap<String, crate::senses::Fixed>) -> crate::senses::WordPlan {
    let w = r.word().to_lowercase();
    let senses = base.get(&w).map(|b| crate::senses::parse(&b.1)).unwrap_or_default();
    let native_ok = asked_fields(format, r, base).iter().all(|i| sense_like(format).contains(i));
    crate::senses::plan(r.word(), &examples_of(format, r), min_uses(format), senses, fixed.get(&w), native_ok)
}

/// The teacher's fields of a format, in answer order (after the word).
pub fn teacher_fields(format: &str) -> &'static [&'static str] {
    match format {
        "records" => &["part of speech", "meaning for a pupil", "sense in the lessons", "new examples"],
        "judaism-senses" => &["part of speech", "meaning", "sense in the Tanakh and Jewish tradition", "interpretation"],
        "emode-senses" => &["part of speech", "modern equivalent", "meaning for a pupil", "how it differs from today"],
        "mystic-senses" => &["part of speech", "meaning for a pupil", "sense in the mystic school"],
        _ => &[],
    }
}

/// The current value of each teacher field of a row.
fn current(format: &str, r: &Raw) -> Vec<String> {
    let idx: &[usize] = match format {
        "records" => &[1, 2, 3],
        "judaism-senses" => &[1, 2, 3, 4],
        "emode-senses" => &[3, 2, 4, 5],
        "mystic-senses" => &[1, 2, 3],
        _ => &[],
    };
    let mut v: Vec<String> = idx.iter().map(|i| r.col(*i).to_string()).collect();
    if format == "records" {
        v.push(String::new());
    }
    v
}

fn clip(s: &str, n: usize) -> String {
    if s.chars().count() <= n { s.to_string() } else { format!("{}…", s.chars().take(n).collect::<String>()) }
}

/// The teacher question for a layer's asked rows, bounded by `teacher::TASK_MAX`; returns it and the rows it holds.
/// Each entry shows why it is weak, the current value of each asked field that has one, the pupils' numbered senses,
/// the store's fixed sense, two examples cut around the word, and the fields to answer.
pub fn task(name: &str, format: &str, rows: &[(Raw, crate::senses::WordPlan)], base: &Base) -> (String, usize) {
    let fields = teacher_fields(format);
    let mut t = format!(
        "You are improving entries of {name}, one of Mova's dictionaries, for a clever pupil of 9-10. Each entry below is \
weak (the reason is given) and lists the pupils' dictionary's numbered senses when it has the word. For EACH entry \
write exactly one line: the word, then TAB-separated the sense pick and the fields named in its \"answer:\" line, in that \
order. The sense pick is the number of the pupils' sense the examples show (2, or 2,5 when they show both), new when \
none fits, same when the entry's fixed sense fits, - when no senses are listed. Write a field only with what the \
numbered senses do not already say, and - for a field you leave as it is. Use the examples given; do not invent facts \
or quotations. State a tradition's reading as its reading, not as fact. No tabs or line breaks inside a field, no \
dates, no extra lines, no numbering; keep the word exactly as given.{}\n\n",
        match format {
            "records" => " New examples are short sentences you write for a pupil, separated by \" | \".",
            _ => "",
        }
    );
    let mut n = 0;
    for (r, p) in rows {
        let cur = current(format, r);
        let asked = asked_fields(format, r, base);
        let mut item = format!("word: {}\nweak: {}\n", r.word(), weakness(format, r, base).unwrap_or_default());
        if matches!(format, "judaism-senses" | "mystic-senses") {
            item.push_str(if base.contains_key(&r.word().to_lowercase()) { "primary record: yes\n" } else { "primary record: none\n" });
        }
        for i in &asked {
            if let Some(v) = cur.get(*i).filter(|v| !v.trim().is_empty()) {
                item.push_str(&format!("current {}: {}\n", fields[*i], clip(v, 300)));
            }
        }
        item.push_str(&p.block());
        item.push_str(&format!("answer: sense pick{}\n\n", asked.iter().map(|i| format!(", {}", fields[*i])).collect::<String>()));
        if t.len() + item.len() > crate::teacher::TASK_MAX {
            break;
        }
        t.push_str(&item);
        n += 1;
    }
    (t, n)
}

/// A date anywhere (ISO `2026-10-10` or dotted `10.10.2026`): the pupils' records refuse dates.
fn has_date(s: &str) -> bool {
    let b = s.as_bytes();
    let d = |i: usize| b.get(i).is_some_and(u8::is_ascii_digit);
    (0..b.len()).any(|i| {
        (d(i) && d(i + 1) && d(i + 2) && d(i + 3) && b.get(i + 4) == Some(&b'-') && d(i + 5) && d(i + 6) && b.get(i + 7) == Some(&b'-') && d(i + 8))
            || (d(i) && d(i + 1) && b.get(i + 2) == Some(&b'.') && d(i + 3) && d(i + 4) && b.get(i + 5) == Some(&b'.') && d(i + 6) && d(i + 7))
    })
}

fn usable(f: &str) -> bool {
    let n = f.chars().count();
    (2..=FIELD_MAX).contains(&n) && !has_date(f) && !f.chars().any(|c| c.is_control())
}

/// One proposal line in the target format from a row and the teacher's fields (`None` keeps the current value).
/// Returns `None` when nothing changes.
pub fn propose(format: &str, r: &Raw, fields: &[Option<String>], base: &Base) -> Option<String> {
    let cur = current(format, r);
    let get = |i: usize| fields.get(i).cloned().flatten().unwrap_or_else(|| cur[i].clone());
    if fields.iter().all(Option::is_none) {
        return None;
    }
    let has_base = base.contains_key(&r.word().to_lowercase());
    let w = r.word().to_string();
    let cols: Vec<String> = match format {
        "records" => {
            let mut ex: Vec<String> = examples_of(format, r);
            if let Some(Some(new)) = fields.get(3) {
                ex.extend(new.split(" | ").map(str::trim).filter(|e| !e.is_empty()).map(String::from));
            }
            vec![w.to_lowercase(), get(0), get(1), get(2), ex.join(" | "), r.col(5).to_string()]
        }
        "judaism-senses" => {
            let (pos, meaning) = if has_base { (String::new(), String::new()) } else { (get(0), get(1)) };
            vec![w, pos, meaning, get(2), get(3), r.col(5).to_string()]
        }
        "emode-senses" => vec![w, r.col(1).into(), get(1), get(0), get(2), get(3), r.col(6).into(), r.col(7).into(), r.col(8).into()],
        "mystic-senses" => {
            let (pos, meaning) = if has_base { (String::new(), String::new()) } else { (get(0), get(1)) };
            vec![w, pos, meaning, get(2)]
        }
        _ => return None,
    };
    let line = cols.join("\t");
    (line != r.line()).then_some(line)
}

/// The teacher's answer → proposal lines for the asked rows (without the sense store). See `validate_planned`.
pub fn validate(format: &str, asked: &[Raw], answer: &str, base: &Base) -> Result<String, String> {
    let rows: Vec<(Raw, crate::senses::WordPlan)> = asked.iter().map(|r| (r.clone(), plan_row(format, r, base, &BTreeMap::new()))).collect();
    validate_planned(format, &rows, answer, base)
}

/// The teacher's answer → proposal lines for the asked rows: one line per word, the first well-formed one wins; a
/// line is well formed with the word, a sense pick (`senses::Pick`) and exactly the row's asked fields, each `-` or
/// usable, none saying the word is absent from the examples. At least half of the asked words must get one. The
/// proposal holds the builder lines that change their entry, and a `# pick` line per word with a pick: a sense field
/// that restates a numbered sense is dropped from the builder line and marked "restates sense N". None of either is
/// a rejection.
pub fn validate_planned(format: &str, asked: &[(Raw, crate::senses::WordPlan)], answer: &str, base: &Base) -> Result<String, String> {
    use crate::senses::{Pick, pick_line, restates, text_of};
    let by: BTreeMap<&str, &(Raw, crate::senses::WordPlan)> = asked.iter().map(|x| (x.0.word(), x)).collect();
    let names = teacher_fields(format);
    let mut formed: BTreeSet<String> = BTreeSet::new();
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    let mut picks: BTreeMap<String, String> = BTreeMap::new();
    let mut absent = 0;
    for l in answer.lines() {
        let c: Vec<&str> = l.split('\t').map(str::trim).collect();
        let Some((r, p)) = by.get(c[0]).map(|x| (&x.0, &x.1)) else { continue };
        let want = asked_fields(format, r, base);
        if c.len() != want.len() + 2 || formed.contains(c[0]) {
            continue;
        }
        if c[1..].iter().any(|f| crate::senses::absent(f)) {
            absent += 1;
            continue;
        }
        let Some(pick) = Pick::parse(c[1], p.senses.len()) else { continue };
        if pick == Pick::Same && p.fixed.is_none() {
            continue;
        }
        if !c[2..].iter().all(|f| *f == "-" || usable(f)) {
            continue;
        }
        formed.insert(c[0].to_string());
        let mut fields: Vec<Option<String>> = vec![None; names.len()];
        let mut restated = None;
        for (i, f) in want.iter().zip(&c[2..]) {
            if *f == "-" {
                continue;
            }
            if sense_like(format).contains(i) && !p.senses.is_empty() {
                if let Some(n) = restates(r.word(), f, &p.senses) {
                    restated = Some(n);
                    continue;
                }
            }
            fields[*i] = Some(f.to_string());
        }
        if let Some(line) = propose(format, r, &fields, base) {
            out.insert(c[0].to_string(), line);
        }
        let new_text = sense_like(format).iter().find_map(|i| fields[*i].clone()).unwrap_or_default();
        let (label, sense, evidence) = match (&pick, restated) {
            (Pick::Senses(v), Some(n)) => (pick.label(), text_of(v, &p.senses), format!("the teacher's pick; its sense field restates sense {n}")),
            (_, Some(n)) => (n.to_string(), text_of(&[n], &p.senses), format!("restates sense {n}")),
            (Pick::Senses(v), None) => (pick.label(), text_of(v, &p.senses), "the teacher's pick".to_string()),
            (Pick::New(t), None) => ("new".into(), if t.is_empty() { new_text } else { t.clone() }, "a sense none of the numbered ones gives".into()),
            (Pick::Same, None) => {
                let f = p.fixed.as_ref().expect("checked above");
                ("same".into(), f.sense.clone(), format!("as the sense {} fixed by {}", f.pick, f.layer))
            }
            (Pick::Keep, None) => continue,
        };
        picks.insert(c[0].to_string(), pick_line(c[0], &label, &sense, "teacher", &evidence));
    }
    if formed.is_empty() || formed.len() * 2 < asked.len() {
        return Err(format!("{} of {} asked words got a well-formed line ({absent} said the word is absent)", formed.len(), asked.len()));
    }
    if out.is_empty() && picks.is_empty() {
        return Err(format!("the teacher kept all {} entries as they are", formed.len()));
    }
    Ok(out.values().chain(picks.values()).map(|l| format!("{l}\n")).collect())
}

/// Nautilus's job on an existing layer: its weak, askable rows (the part `part` of `parts`, by the word's hash), `batch`
/// asked rows to a question; rows Mova settles natively (`plan_row`) ride along in the same item and are recorded as
/// native picks. The item key is `<layer>:<word>+<word>…`. The file is only read.
pub struct LayerJob {
    pub layer: String,
    pub name: String,
    pub file: std::path::PathBuf,
    pub format: String,
    pub base: Base,
    pub part: u32,
    pub parts: u32,
    /// rows asked per question
    pub batch: usize,
    /// the shared sense store
    pub store: crate::senses::Store,
}

impl LayerJob {
    /// The askable rows of this job's part, in file order.
    pub fn askable(&self) -> anyhow::Result<Vec<Raw>> {
        let text = std::fs::read_to_string(&self.file)?;
        Ok(parse(&text)
            .into_iter()
            .filter(|r| crate::work::short_hash(r.word()) % self.parts.max(1) == self.part)
            .filter(|r| askable(&self.format, r, &self.base))
            .collect())
    }
    /// The words already in item keys of this layer.
    pub fn done(&self, seen: &BTreeSet<String>) -> BTreeSet<String> {
        let p = format!("{}:", self.layer);
        seen.iter().filter_map(|k| k.strip_prefix(&p)).flat_map(|w| w.split('+').map(String::from)).collect()
    }
    fn planned(&self, item: &Item) -> Vec<(Raw, crate::senses::WordPlan)> {
        let fixed = self.store.fixed();
        parse(&item.body).into_iter().map(|r| {
            let p = plan_row(&self.format, &r, &self.base, &fixed);
            (r, p)
        }).collect()
    }
    fn asked(&self, item: &Item) -> Vec<(Raw, crate::senses::WordPlan)> {
        self.planned(item).into_iter().filter(|x| x.1.fate == crate::senses::Fate::Ask).collect()
    }
    /// The open rows and the items `next` would make of them: (rows per item).
    fn items(&self, seen: &BTreeSet<String>) -> anyhow::Result<Vec<Vec<Raw>>> {
        let done = self.done(seen);
        let fixed = self.store.fixed();
        let b = self.batch.max(1);
        let mut out: Vec<Vec<Raw>> = vec![];
        let (mut cur, mut asked) = (Vec::new(), 0);
        for r in self.askable()?.into_iter().filter(|r| !done.contains(r.word())) {
            if asked >= b || cur.len() >= 4 * b {
                out.push(std::mem::take(&mut cur));
                asked = 0;
            }
            asked += usize::from(plan_row(&self.format, &r, &self.base, &fixed).fate == crate::senses::Fate::Ask);
            cur.push(r);
        }
        if !cur.is_empty() {
            out.push(cur);
        }
        Ok(out)
    }
}

impl Job for LayerJob {
    fn purpose(&self) -> &'static str {
        "dictionary"
    }
    fn next(&self, seen: &BTreeSet<String>) -> anyhow::Result<Option<Item>> {
        let Some(rows) = self.items(seen)?.into_iter().next() else { return Ok(None) };
        let key = format!("{}:{}", self.layer, rows.iter().map(Raw::word).collect::<Vec<_>>().join("+"));
        Ok(Some(Item { key, body: rows.iter().map(Raw::line).collect::<Vec<_>>().join("\n") }))
    }
    fn native(&self, item: &Item) -> Native {
        let asked = self.asked(item);
        let (t, n) = task(&self.name, &self.format, &asked, &self.base);
        if n == 0 {
            return Native::Done(format!("{} rows, none asked: settled by Mova or not askable", parse(&item.body).len()));
        }
        Native::Unsure(t)
    }
    fn validate(&self, item: &Item, answer: &str) -> Result<String, String> {
        let asked = self.asked(item);
        let (_, n) = task(&self.name, &self.format, &asked, &self.base);
        validate_planned(&self.format, &asked[..n], answer, &self.base)
    }
    fn open(&self, seen: &BTreeSet<String>) -> anyhow::Result<usize> {
        Ok(self.items(seen)?.len())
    }
    fn describe(&self) -> serde_json::Value {
        serde_json::json!({"layer":self.layer,"target":self.format,"file":self.file,"part":self.part,"parts":self.parts,"batch":self.batch})
    }
    fn native_proposal(&self, item: &Item) -> Option<String> {
        let lines: Vec<String> = self.planned(item).iter().filter_map(|x| x.1.native_line()).collect();
        (!lines.is_empty()).then(|| lines.iter().map(|l| format!("{l}\n")).collect())
    }
    fn learn(&self, _item: &Item, proposals: &[&str], now: u64) -> anyhow::Result<()> {
        for p in proposals {
            self.store.append(&crate::senses::records(p, &self.layer, now))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn base() -> Base {
        crate::layer::read_base("abide\tverb\tTo stay.\nab\tnoun\tA muscle.\n")
    }
    #[test]
    fn rows_read_back_byte_for_byte() {
        // the real layer files of the school data (outside the repository; absent files are skipped): every row renders
        // back to its own bytes
        let dir = std::env::var_os("MOVA_SCHOOL_DATA").filter(|v| !v.is_empty()).map(std::path::PathBuf::from).unwrap_or_else(|| {
            std::path::PathBuf::from(std::env::var_os("MOVA_DATA").filter(|v| !v.is_empty()).unwrap_or_else(|| std::path::Path::new(&std::env::var_os("HOME").unwrap_or_default()).join("mova-data").into_os_string())).join("school")
        }).join("dictionaries");
        for (f, fmt) in [("emode-senses.tsv", "emode-senses"), ("judaism-senses.tsv", "judaism-senses"), ("mystic-senses.tsv", "mystic-senses"), ("mystic-school.tsv", "records"), ("primary-school.v2.tsv", "records")] {
            let Ok(t) = std::fs::read_to_string(dir.join(f)) else { continue };
            let rows = parse(&t);
            let body: Vec<&str> = t.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).collect();
            assert_eq!(rows.len(), body.len(), "{f}");
            for (r, l) in rows.iter().zip(&body) {
                assert_eq!(r.line(), *l, "{f}");
                assert!(r.0.len() >= columns(fmt).unwrap(), "{f}: {} columns", r.0.len());
            }
        }
    }
    #[test]
    fn weak_rows_and_builder_lines() {
        let b = base();
        // emode: an empty difference is weak; the proposal keeps kind, examples, forms and tests
        let e = Raw("abide\tarchaic sense\tstay\tverb\tTo remain.\t\tAbide with me. «Torah GEN.29.19, JPS 1917»\tabide\trate".split('\t').map(String::from).collect());
        assert_eq!(weakness("emode-senses", &e, &b).as_deref(), Some("difference"));
        let p = validate("emode-senses", std::slice::from_ref(&e), "abide\t-\tToday it means to put up with something; here it means to stay.\n", &b).unwrap();
        assert_eq!(p, "abide\tarchaic sense\tstay\tverb\tTo remain.\tToday it means to put up with something; here it means to stay.\tAbide with me. «Torah GEN.29.19, JPS 1917»\tabide\trate\n");
        assert_eq!(p.trim_end().split('\t').count(), 9);
        // judaism: a word without a primary record and no meaning is weak; with a record, pos and meaning stay empty
        let j = Raw("abacus\t\t\t\t\tIn the abacus. «Mordell 1914»".split('\t').map(String::from).collect());
        assert!(weakness("judaism-senses", &j, &b).is_some());
        let ab = Raw("ab\t\t\tA month.\t\tAb is the fifth. «SY»".split('\t').map(String::from).collect());
        assert!(weakness("judaism-senses", &ab, &b).is_none());
        let p = validate("judaism-senses", std::slice::from_ref(&j), "abacus\t-\tnoun\tA frame with beads for counting.\n", &b).unwrap();
        assert_eq!(p, "abacus\tnoun\tA frame with beads for counting.\t\t\tIn the abacus. «Mordell 1914»\n");
        // records: new examples are appended; six columns, the pupils' record format
        let r = Raw("aah\tinterjection\t1) A sound you make when you feel relief.\tUsed in speech.\tAah, that's better.\tnote".split('\t').map(String::from).collect());
        assert_eq!(weakness("records", &r, &b).as_deref(), Some("sense, examples"));
        let p = validate("records", std::slice::from_ref(&r), "aah\t-\tUsed in speech, comics and stories to show relief.\tEveryone went aah.\n", &b).unwrap();
        assert_eq!(p, "aah\tinterjection\t1) A sound you make when you feel relief.\tUsed in speech, comics and stories to show relief.\tAah, that's better. | Everyone went aah.\tnote\n");
        // red: wrong field count, a date, an unchanged line, a word not asked
        assert!(validate("records", std::slice::from_ref(&r), "aah\t-\tUsed in speech.\n", &b).is_err());
        assert!(validate("records", std::slice::from_ref(&r), "aah\t-\tUsed since 2026-10-10 in speech and comics.\t-\n", &b).is_err());
        assert!(validate("records", std::slice::from_ref(&r), "aah\t-\t-\t-\n", &b).is_err());
        assert!(validate("records", std::slice::from_ref(&r), "ooh\t-\tUsed in speech, comics and stories to show relief.\t-\n", &b).is_err());
    }
    #[test]
    fn senses_picks_restatements_and_absent_lines() {
        let b = crate::layer::read_base("bend\tverb\t1) To bend something is to make it curved, like a bow or a wire. 2) To bend down is to lean your body forward and down, as when you kneel.\n");
        let row = |ex: &str| Raw(format!("bend\t\t\t\t\t{ex}").split('\t').map(String::from).collect());
        // only the weak fields are asked: sense and interpretation, not part of speech and meaning (primary record)
        let j = row("He hath bent His bow, and set me as a mark. «Writings LAM.3.12, JPS 1917» | Mine arms do bend a bow of brass. «Prophets 2SA.22.35, JPS 1917»");
        assert_eq!(asked_fields("judaism-senses", &j, &b), [2, 3]);
        let p = plan_row("judaism-senses", &j, &b, &BTreeMap::new());
        assert_eq!(p.fate, crate::senses::Fate::Ask, "one shared lemma is not enough");
        let (t, n) = task("the Judaism dictionary", "judaism-senses", &[(j.clone(), p)], &b);
        assert_eq!(n, 1);
        assert!(t.contains("senses: 1) To bend something") && t.contains("answer: sense pick, sense in the Tanakh and Jewish tradition, interpretation"), "{t}");
        assert!(!t.contains("part of speech:") && !t.contains("(empty)"), "{t}");
        // a sense field that restates sense 1 is dropped and marked; the interpretation is kept
        let a = "bend\t1\tIn Samuel and Lamentations, to bend a bow is to make it curved, like a wire.\tA drawn bow is ready to shoot.\n";
        let v = validate("judaism-senses", std::slice::from_ref(&j), a, &b).unwrap();
        assert!(v.starts_with("bend\t\t\t\tA drawn bow is ready to shoot.\tHe hath bent"), "{v}");
        assert!(v.contains("# pick\tbend\t1\t1) To bend something is to make it curved, like a bow or a wire.\tteacher\tthe teacher's pick; its sense field restates sense 1"), "{v}");
        // red: a line saying the word is absent is no answer
        let e = validate("judaism-senses", std::slice::from_ref(&j), "bend\t-\tThe examples do not contain bend.\t-\n", &b).unwrap_err();
        assert!(e.contains("1 said the word is absent"), "{e}");
        // examples sharing two lemmas with sense 1 and none with sense 2: Mova picks it, nothing is asked
        let k = row("She bent the wire into a curve. «a:1» | Bend the wire to make a curve. «a:2»");
        let p = plan_row("judaism-senses", &k, &b, &BTreeMap::new());
        assert!(matches!(&p.fate, crate::senses::Fate::Native { pick, .. } if pick == "1"), "{:?}", p.fate);
        // junk and boilerplate are never asked
        let ag = Raw("ag\t\t\t\t\tIs wisdom with aged men? «JOB.12.12» | The aged rose up. «JOB.29.8»".split('\t').map(String::from).collect());
        let ar = Raw("archive\t\t\t\t\tSource: Internet Sacred Text Archive, https://archive.sacred-texts.com «SY» | license: Public Domain | versionSource: https://archive.org «SY»".split('\t').map(String::from).collect());
        assert!(weakness("judaism-senses", &ag, &b).is_some() && !askable("judaism-senses", &ag, &b));
        assert!(weakness("judaism-senses", &ar, &b).is_some() && !askable("judaism-senses", &ar, &b));
        assert!(askable("judaism-senses", &j, &b));
    }
    #[test]
    fn cc_by_sa_headers() {
        assert!(declares_cc_by_sa("# License: CC BY-SA 4.0\nword\tx\n"));
        assert!(!declares_cc_by_sa("# word\tpart of speech\nCC BY-SA in a row is not a header\n"));
    }
}
