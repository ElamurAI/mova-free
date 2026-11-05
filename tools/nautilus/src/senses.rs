//! Senses: which of the pupils' dictionary's numbered senses a layer's examples show, natively first.
//!
//! The pupils' dictionary (`world/data/dictionary-improved.tsv`) gives most words numbered senses in its meaning
//! column (`1) … 2) …`). A layer question asks the teacher which sense number(s) the examples show, and a new sense
//! text only when none fits; Mova scores the senses first and asks nothing when it is confident.
//!
//! - **Words in examples** are matched on Mova's own lemmas (the built-in dictionary of `en`: a token's form and every
//!   lemma its analyses give, without comparatives and superlatives outside UD («bier» is no «bi», «number» no
//!   «numb»); the Early Modern English
//!   verb endings), never on prefixes: «aged» is not an example of «ag».
//! - **Junk** is closed natively and never asked: a word under three letters, an abbreviation of the scans
//!   (`ABBREVIATIONS`), and examples that are scan or licence boilerplate (`BOILERPLATE`) do not count as examples.
//! - **Native picks** (`native_pick`): the content lemmas the examples share with each sense's text (a quoted phrase of
//!   a sense found in an example counts three). Confident when the word has one sense and the examples share a lemma
//!   with it, or when the best sense shares at least `PICK_MIN` lemmas and `PICK_MARGIN` more than the next.
//! - **Restatements** (`restates`): a written sense whose content lemmas mostly repeat one numbered sense (Dice
//!   coefficient at least `RESTATE_DICE`, at least `RESTATE_SHARED` shared lemmas) is that sense, not a new one.
//! - **Absent** (`absent`): an answer that says the word is not in the examples, or that they cannot establish a
//!   sense, is no answer.
//! - **The sense store** (`Store`): one JSONL file shared by every layer and instance, `word → (pick, sense, layer,
//!   by, evidence)`. A word's first record fixes its general sense; a later layer is asked only whether it uses the
//!   word differently, and when Mova's own scoring does not contradict the fixed sense nothing is asked.
//!
//! Deterministic: sets and maps are `BTreeSet`/`BTreeMap`; ties are broken by the sense number.
use anyhow::Result;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Characters of an example kept around the word in a question.
pub const EXCERPT: usize = 160;
/// Examples per word in a question.
pub const EXAMPLES: usize = 2;
/// Shortest word asked (letters).
pub const MIN_LETTERS: usize = 3;
/// A native pick needs at least this many shared lemmas (several senses) ...
pub const PICK_MIN: usize = 2;
/// ... and this many more than the next sense.
pub const PICK_MARGIN: usize = 2;
/// A written sense restates a numbered one at this Dice coefficient of content lemmas ...
pub const RESTATE_DICE: f64 = 0.5;
/// ... with at least this many shared lemmas.
pub const RESTATE_SHARED: usize = 3;
/// Characters of a sense shown in a question (its head: up to its first example after a colon).
pub const SENSE_CLIP: usize = 70;

/// Abbreviations and markers of scanned books that are never dictionary words of a layer.
pub const ABBREVIATIONS: [&str; 12] = ["ibid", "viz", "etc", "isbn", "http", "https", "www", "html", "htm", "pdf", "ocr", "etext"];
/// An example holding any of these (lower case) is scan or licence boilerplate, not a use of a word.
pub const BOILERPLATE: [&str; 18] = [
    "http://", "https://", "www.", ".org", ".com", ".htm", "license:", "licence:", "versiontitle", "versionsource", "scanned at",
    "notice of attribution", "non-commercial", "etext", "sacred-texts", "internet sacred text archive", "source: ", "public domain",
];
/// Phrases of an answer that says the examples show no sense.
pub const ABSENT: [&str; 22] = [
    "not in the examples", "not in these examples", "not in the example", "does not appear", "do not appear", "doesn't appear",
    "don't appear", "does not contain", "do not contain", "doesn't contain", "don't contain", "cannot establish", "can't establish",
    "cannot be established", "do not establish", "examples do not show", "examples don't show", "not a separate word",
    "not used as a word", "not a word", "no example", "unclear",
];

/// Function words and the words every sense text uses; never content.
const STOP: &[&str] = &[
    "a", "an", "the", "and", "or", "but", "if", "of", "to", "in", "on", "at", "by", "for", "with", "from", "into", "onto", "out", "up",
    "down", "off", "over", "under", "about", "as", "than", "then", "so", "too", "very", "not", "no", "nor", "yes", "be", "is", "are",
    "was", "were", "been", "being", "am", "have", "has", "had", "do", "does", "did", "done", "can", "could", "will", "would", "shall",
    "should", "may", "might", "must", "it", "its", "this", "that", "these", "those", "there", "here", "he", "she", "they", "we",
    "you", "i", "me", "him", "her", "them", "us", "my", "your", "his", "their", "our", "who", "whom", "whose", "what", "which",
    "when", "where", "why", "how", "all", "any", "some", "each", "every", "one", "ones", "also", "often", "usually", "sometimes",
    "just", "only", "more", "most", "much", "many", "such", "other", "another", "same", "own", "mean", "means", "meaning", "word",
    "words", "use", "used", "say", "says", "call", "called", "like", "way", "kind", "thing", "something", "someone", "somebody",
    "casual", "formal", "rude", "slang", "euphemism", "informal",
];

/// The lower-case letter words of a text with their byte spans (apostrophes and inner hyphens kept).
pub fn tokens(text: &str) -> Vec<(usize, usize, String)> {
    let mut out = Vec::new();
    let mut start: Option<usize> = None;
    let b: Vec<(usize, char)> = text.char_indices().collect();
    for (k, &(i, c)) in b.iter().enumerate() {
        let inner = (c == '\'' || c == '\u{2019}' || c == '-')
            && start.is_some()
            && b.get(k + 1).is_some_and(|(_, n)| n.is_alphabetic());
        if c.is_alphabetic() || inner {
            start.get_or_insert(i);
        } else if let Some(s) = start.take() {
            out.push((s, i, text[s..i].to_lowercase().replace('\u{2019}', "'")));
        }
    }
    if let Some(s) = start {
        out.push((s, text.len(), text[s..].to_lowercase().replace('\u{2019}', "'")));
    }
    out
}

/// Every lemma Mova's dictionary gives a lower-case form (the form itself included).
pub fn lemmas(lower: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::from([lower.to_string()]);
    if let Some(f) = en::dict::form(lower) {
        for r in en::dict::analyses(f) {
            let comparative = matches!(r.tag, en::gram::Tag::JJS | en::gram::Tag::JJR | en::gram::Tag::RBS | en::gram::Tag::RBR);
            if r.src & en::dict::UD != 0 || !comparative {
                let l = en::dict::text(r.lemma()).to_lowercase();
                if !l.is_empty() {
                    out.insert(l);
                }
            }
        }
    }
    if let Some(l) = en::morph::archaic(lower, en::gram::Tag::VB) {
        out.insert(l.to_lowercase());
    }
    out
}

/// A token's one lemma for overlaps: the dictionary's first analysis (UD first), else the form.
pub fn lemma(lower: &str) -> String {
    en::dict::form(lower)
        .and_then(|f| en::dict::analyses(f).map(|r| en::dict::text(r.lemma()).to_lowercase()).find(|l| !l.is_empty()))
        .unwrap_or_else(|| lower.to_string())
}

/// The text of an example without its trailing source id (`text «id»`).
pub fn example_text(e: &str) -> &str {
    match e.rfind(" «") {
        Some(i) if e.trim_end().ends_with('»') => &e[..i],
        _ => e,
    }
}

/// Where `word` (a lemma, or a phrase of several) is used in `text` as a word: the byte span of the first use.
pub fn find(text: &str, word: &str) -> Option<(usize, usize)> {
    let w = word.to_lowercase();
    let parts: Vec<String> = tokens(&w).into_iter().map(|t| t.2).collect();
    let toks = tokens(text);
    match parts.as_slice() {
        [] => None,
        [one] => toks.iter().find(|t| t.2 == *one || lemmas(&t.2).contains(one)).map(|t| (t.0, t.1)),
        many => toks.windows(many.len()).find(|win| {
            win.iter().zip(many).enumerate().all(|(i, (t, p))| t.2 == *p || (i + 1 == many.len() && lemmas(&t.2).contains(p)))
        }).map(|win| (win[0].0, win[many.len() - 1].1)),
    }
}

/// Scan or licence boilerplate rather than a use of a word.
pub fn boilerplate(example: &str) -> bool {
    let low = example.to_lowercase();
    BOILERPLATE.iter().any(|b| low.contains(b))
}

/// Why a word is junk for a teacher (never asked), natively.
pub fn junk(word: &str) -> Option<&'static str> {
    let letters = word.chars().filter(|c| c.is_alphabetic()).count();
    if letters < MIN_LETTERS {
        return Some("shorter than three letters");
    }
    if ABBREVIATIONS.contains(&word.to_lowercase().as_str()) {
        return Some("an abbreviation of scanned books");
    }
    None
}

/// The examples that use the word (Mova's lemmas) and are not boilerplate, in their order.
pub fn uses(word: &str, examples: &[String]) -> Vec<String> {
    examples.iter().filter(|e| !boilerplate(e) && find(example_text(e), word).is_some()).cloned().collect()
}

/// An example cut to about `EXCERPT` characters around the word; its source id stays in the row, not in the question.
pub fn excerpt(e: &str, word: &str) -> String {
    let text = example_text(e).trim();
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let cut = if chars.len() <= EXCERPT {
        text.to_string()
    } else {
        let (s, _) = find(text, word).unwrap_or((0, 0));
        let at = chars.iter().position(|(i, _)| *i >= s).unwrap_or(0);
        let mut from = at.saturating_sub(EXCERPT / 2);
        let mut to = (from + EXCERPT).min(chars.len());
        from = to.saturating_sub(EXCERPT);
        // to word boundaries
        while from > 0 && !chars[from - 1].1.is_whitespace() {
            from += 1;
        }
        while to < chars.len() && !chars[to].1.is_whitespace() && to > from {
            to -= 1;
        }
        let byte = |k: usize| chars.get(k).map_or(text.len(), |c| c.0);
        format!("{}{}{}", if from > 0 { "…" } else { "" }, text[byte(from)..byte(to)].trim(), if to < chars.len() { "…" } else { "" })
    };
    cut
}

/// The numbered senses of a meaning (`1) … 2) …`); a meaning without numbers is one sense; empty: none.
pub fn parse(meaning: &str) -> Vec<(u32, String)> {
    let m = meaning.trim();
    if m.is_empty() {
        return vec![];
    }
    // starts of `N) ` at the start or after a space
    let b = m.as_bytes();
    let mut starts: Vec<(usize, usize, u32)> = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_digit() && (i == 0 || b[i - 1] == b' ') {
            let mut j = i;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            if j + 1 < b.len() && b[j] == b')' && b[j + 1] == b' ' {
                let n: u32 = m[i..j].parse().unwrap_or(0);
                if n == starts.len() as u32 + 1 {
                    starts.push((i, j + 2, n));
                }
            }
            i = j.max(i + 1);
        } else {
            i += 1;
        }
    }
    if starts.is_empty() {
        return vec![(1, m.to_string())];
    }
    starts
        .iter()
        .enumerate()
        .map(|(k, (_, body, n))| (*n, m[*body..starts.get(k + 1).map_or(m.len(), |s| s.0)].trim().to_string()))
        .filter(|(_, t)| !t.is_empty())
        .collect()
}

/// Content lemmas of a text: not a function word, not the headword, at least three letters.
pub fn content(text: &str, word: &str) -> BTreeSet<String> {
    let head = lemmas(&word.to_lowercase());
    tokens(text)
        .into_iter()
        .map(|t| lemma(&t.2))
        .filter(|l| l.chars().count() >= 3 && !STOP.contains(&l.as_str()) && !head.contains(l))
        .collect()
}

/// The quoted phrases of a sense («…») of two or more words.
fn phrases(sense: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = sense;
    while let Some(a) = rest.find('«') {
        let after = &rest[a + '«'.len_utf8()..];
        let Some(b) = after.find('»') else { break };
        let p = after[..b].trim().to_lowercase();
        if p.split_whitespace().count() >= 2 {
            out.push(p);
        }
        rest = &after[b..];
    }
    out
}

/// One sense's score against the examples.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Scored {
    pub sense: u32,
    pub score: usize,
    pub shared: Vec<String>,
}

/// Every sense scored: the content lemmas it shares with the examples, a quoted phrase found in an example three.
pub fn score(word: &str, examples: &[String], senses: &[(u32, String)]) -> Vec<Scored> {
    let ex: BTreeSet<String> = examples.iter().flat_map(|e| content(example_text(e), word)).collect();
    let low: Vec<String> = examples.iter().map(|e| example_text(e).to_lowercase()).collect();
    senses
        .iter()
        .map(|(n, t)| {
            let shared: Vec<String> = content(t, word).intersection(&ex).cloned().collect();
            let ph = phrases(t).iter().filter(|p| low.iter().any(|e| find(e, p).is_some())).count();
            Scored { sense: *n, score: shared.len() + 3 * ph, shared }
        })
        .collect()
}

/// A confident native pick: the sense and its evidence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NativePick {
    pub sense: u32,
    pub evidence: String,
}

/// Mova's own pick of a sense, when confident (see the module's rule); `None` leaves the word to the teacher.
pub fn native_pick(word: &str, examples: &[String], senses: &[(u32, String)]) -> Option<NativePick> {
    let mut s = score(word, examples, senses);
    s.sort_by(|a, b| b.score.cmp(&a.score).then(a.sense.cmp(&b.sense)));
    let best = s.first()?;
    let next = s.get(1).map_or(0, |x| x.score);
    let sure = (senses.len() == 1 && best.score >= 1) || (best.score >= PICK_MIN && best.score >= next + PICK_MARGIN);
    sure.then(|| NativePick {
        sense: best.sense,
        evidence: format!(
            "Mova's overlap: the examples share {} with sense {} (score {}, next {}{})",
            best.shared.iter().map(|w| format!("«{w}»")).collect::<Vec<_>>().join(", "),
            best.sense,
            best.score,
            next,
            if senses.len() == 1 { ", the only sense" } else { "" }
        ),
    })
}

/// The numbered sense a written text restates, if it is a near copy of one.
pub fn restates(word: &str, text: &str, senses: &[(u32, String)]) -> Option<u32> {
    let t = content(text, word);
    let mut best: Option<(f64, u32)> = None;
    for (n, s) in senses {
        let c = content(s, word);
        let shared = t.intersection(&c).count();
        if shared < RESTATE_SHARED || t.is_empty() || c.is_empty() {
            continue;
        }
        let dice = 2.0 * shared as f64 / (t.len() + c.len()) as f64;
        if dice >= RESTATE_DICE && best.is_none_or(|(d, _)| dice > d) {
            best = Some((dice, *n));
        }
    }
    best.map(|b| b.1)
}

/// An answer that says the word is not in the examples, or that they show no sense.
pub fn absent(text: &str) -> bool {
    let low = text.to_lowercase();
    ABSENT.iter().any(|a| low.contains(a))
}

/// A teacher's sense pick.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pick {
    /// numbered senses of the pupils' dictionary
    Senses(Vec<u32>),
    /// none fits: a new sense (its text, empty when another field holds it)
    New(String),
    /// as the sense the store fixed for the word
    Same,
    /// `-`: no pick (the word has no numbered senses)
    Keep,
}

impl Pick {
    /// `2`, `2,5`, `new: text`, `new`, `same`, `-`; numbers within `1..=n`.
    pub fn parse(s: &str, n: usize) -> Option<Pick> {
        let s = s.trim();
        let low = s.to_lowercase();
        if s == "-" {
            return Some(Pick::Keep);
        }
        if low == "same" {
            return Some(Pick::Same);
        }
        if low == "new" {
            return Some(Pick::New(String::new()));
        }
        if let Some(t) = low.strip_prefix("new:").map(|_| s[4..].trim()) {
            return Some(Pick::New(t.to_string()));
        }
        let nums: Option<Vec<u32>> = s.split([',', ' ']).filter(|x| !x.is_empty()).map(|x| x.parse().ok()).collect();
        let nums = nums.filter(|v| !v.is_empty() && v.iter().all(|k| *k >= 1 && (*k as usize) <= n))?;
        let set: BTreeSet<u32> = nums.into_iter().collect();
        Some(Pick::Senses(set.into_iter().collect()))
    }
    /// How a pick is written in proposals and the store: `2,5`, `new`, `same`, `-`.
    pub fn label(&self) -> String {
        match self {
            Pick::Senses(v) => v.iter().map(u32::to_string).collect::<Vec<_>>().join(","),
            Pick::New(_) => "new".into(),
            Pick::Same => "same".into(),
            Pick::Keep => "-".into(),
        }
    }
}

/// A word's fixed general sense in the store.
#[derive(Clone, Debug, PartialEq)]
pub struct Fixed {
    /// `2`, `2,5` or `new`
    pub pick: String,
    pub sense: String,
    pub layer: String,
    pub by: String,
}

/// The shared sense store (`senses.jsonl`); `None` path: no store.
#[derive(Clone, Debug, Default)]
pub struct Store {
    pub path: Option<PathBuf>,
}

impl Store {
    pub fn at(path: Option<PathBuf>) -> Store {
        Store { path }
    }
    /// Every word's general sense: its first record with a numbered or new pick.
    pub fn fixed(&self) -> BTreeMap<String, Fixed> {
        let mut m = BTreeMap::new();
        let Some(p) = &self.path else { return m };
        let Ok(t) = std::fs::read_to_string(p) else { return m };
        for v in t.lines().filter_map(|l| serde_json::from_str::<Value>(l).ok()) {
            let (Some(w), Some(pick)) = (v["word"].as_str(), v["pick"].as_str()) else { continue };
            if pick == "-" || pick == "same" {
                continue;
            }
            m.entry(w.to_string()).or_insert_with(|| Fixed {
                pick: pick.to_string(),
                sense: v["sense"].as_str().unwrap_or("").to_string(),
                layer: v["layer"].as_str().unwrap_or("").to_string(),
                by: v["by"].as_str().unwrap_or("").to_string(),
            });
        }
        m
    }
    /// Append records (one line each, fsynced).
    pub fn append(&self, records: &[Value]) -> Result<()> {
        let Some(p) = &self.path else { return Ok(()) };
        if records.is_empty() {
            return Ok(());
        }
        if let Some(d) = p.parent() {
            std::fs::create_dir_all(d)?;
        }
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(p)?;
        let text: String = records.iter().map(|r| format!("{r}\n")).collect();
        f.write_all(text.as_bytes())?;
        f.sync_data()?;
        Ok(())
    }
}

/// A store record.
pub fn record(word: &str, pick: &str, sense: &str, layer: &str, by: &str, evidence: &str, now: u64) -> Value {
    json!({"word":word,"pick":pick,"sense":sense,"layer":layer,"by":by,"evidence":evidence,"admitted":false,"unix":now})
}

/// The text of picked senses (`2,5` → their texts joined by ` / `).
pub fn text_of(pick: &[u32], senses: &[(u32, String)]) -> String {
    pick.iter().filter_map(|k| senses.iter().find(|s| s.0 == *k).map(|s| format!("{k}) {}", s.1))).collect::<Vec<_>>().join(" / ")
}

/// A sense's head for a question: the text before its first example (`: `), at most `SENSE_CLIP` characters.
pub fn head(t: &str) -> String {
    let t = match t.find(": ") {
        Some(i) if t[..i].chars().count() >= 15 => &t[..i],
        _ => t,
    };
    if t.chars().count() <= SENSE_CLIP { t.to_string() } else { format!("{}…", t.chars().take(SENSE_CLIP).collect::<String>().trim_end()) }
}

/// Senses for a question: each its head (`head`), ` | `-joined.
pub fn show(senses: &[(u32, String)]) -> String {
    senses.iter().map(|(n, t)| format!("{n}) {}", head(t))).collect::<Vec<_>>().join(" | ")
}

/// The store a configuration names, for a path that may be absent.
pub fn store_at(p: Option<&Path>) -> Store {
    Store::at(p.map(Path::to_path_buf))
}

/// What becomes of one word of a layer question, natively.
#[derive(Clone, Debug, PartialEq)]
pub enum Fate {
    /// never asked: the reason
    Junk(&'static str),
    /// fewer examples use the word than the layer needs: not asked
    Thin(usize),
    /// resolved by Mova: the pick (sense numbers), the sense text and the evidence
    Native { pick: String, sense: String, evidence: String },
    /// asked
    Ask,
}

/// One word's native plan: its examples that use it, the pupils' senses, the store's fixed sense and its fate.
#[derive(Clone, Debug, PartialEq)]
pub struct WordPlan {
    pub word: String,
    pub uses: Vec<String>,
    pub senses: Vec<(u32, String)>,
    pub fixed: Option<Fixed>,
    pub fate: Fate,
}

/// Plan one word: junk, too few uses (`min_uses`), a confident native pick (only when `native_ok`: the layer asks
/// nothing a pick cannot settle), or a question.
pub fn plan(word: &str, examples: &[String], min_uses: usize, senses: Vec<(u32, String)>, fixed: Option<&Fixed>, native_ok: bool) -> WordPlan {
    let uses = uses(word, examples);
    let fate = if let Some(r) = junk(word) {
        Fate::Junk(r)
    } else if uses.len() < min_uses {
        Fate::Thin(uses.len())
    } else if !native_ok || senses.is_empty() {
        Fate::Ask
    } else {
        match native_pick(word, &uses, &senses) {
            Some(p) => {
                let agree = match fixed {
                    Some(f) if f.pick == p.sense.to_string() => format!("; agrees with sense {} fixed by {}", f.pick, f.layer),
                    Some(f) => format!("; differs from sense {} fixed by {}", f.pick, f.layer),
                    None => String::new(),
                };
                Fate::Native { pick: p.sense.to_string(), sense: text_of(&[p.sense], &senses), evidence: format!("{}{agree}", p.evidence) }
            }
            None => Fate::Ask,
        }
    };
    WordPlan { word: word.to_string(), uses, senses, fixed: fixed.cloned(), fate }
}

impl WordPlan {
    /// The lines of a question that show this word's senses, fixed sense and examples (two, cut around the word).
    pub fn block(&self) -> String {
        let mut b = String::new();
        if self.senses.is_empty() {
            b.push_str("senses: none listed\n");
        } else {
            b.push_str(&format!("senses: {}\n", show(&self.senses)));
        }
        if let Some(f) = &self.fixed {
            b.push_str(&format!("fixed: {} (by {})\n", f.pick, f.layer));
        }
        if !self.uses.is_empty() {
            b.push_str(&format!("examples: {}\n", self.uses.iter().take(EXAMPLES).map(|e| excerpt(e, &self.word)).collect::<Vec<_>>().join(" | ")));
        }
        b
    }
    /// A `# not asked` line for a junk or thin word, a pick line for a native one; `None` for an asked word.
    pub fn native_line(&self) -> Option<String> {
        match &self.fate {
            Fate::Junk(r) => Some(format!("# not asked\t{}\t{r}", self.word)),
            Fate::Thin(n) => Some(format!("# not asked\t{}\t{n} example(s) use the word", self.word)),
            Fate::Native { pick, sense, evidence } => Some(pick_line(&self.word, pick, sense, "native", evidence)),
            Fate::Ask => None,
        }
    }
}

/// A pick line of a proposal: `# pick<TAB>word<TAB>pick<TAB>sense<TAB>by<TAB>evidence`.
pub fn pick_line(word: &str, pick: &str, sense: &str, by: &str, evidence: &str) -> String {
    let c = |s: &str| s.replace(['\t', '\n', '\r'], " ");
    format!("# pick\t{}\t{}\t{}\t{by}\t{}", c(word), c(pick), c(sense), c(evidence))
}

/// The pick lines of a proposal as store records of `layer`.
pub fn records(proposal: &str, layer: &str, now: u64) -> Vec<Value> {
    proposal
        .lines()
        .filter_map(|l| l.strip_prefix("# pick\t"))
        .filter_map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            (c.len() == 5).then(|| record(c[0], c[1], c[2], layer, c[3], c[4], now))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn words_are_matched_on_lemmas_not_prefixes() {
        assert!(find("the aged rose up and stood", "ag").is_none(), "«aged» is no example of «ag»");
        assert!(find("the aged rose up and stood", "aged").is_some());
        assert!(find("He hath bent His bow", "bend").is_some(), "bent → bend");
        assert!(find("They repay me evil for good", "repay").is_some());
        assert!(find("Children played in the garden", "child").is_some(), "children → child");
        assert!(find("he goeth forth", "go").is_some(), "Early Modern English -eth");
        assert!(find("a piggy bank on the shelf", "piggy bank").is_some());
        assert!(find("the bier of Abner", "bi").is_none() && find("a number of men", "numb").is_none(), "no AGID-only comparatives");
        // red control: a prefix match would take «aged» for «ag»
        assert!("the aged rose".contains("ag"));
    }
    #[test]
    fn junk_and_boilerplate() {
        for w in ["br", "bi", "ag", "cc", "cf", "ibid"] {
            assert!(junk(w).is_some(), "{w}");
        }
        assert!(junk("bend").is_none() && junk("archive").is_none());
        let ex = vec![
            "Source: Internet Sacred Text Archive, https://archive.sacred-texts.com/jud/yetzirah.htm . «Sefer Yetzirah, Westcott 1887»".to_string(),
            "NOTICE OF ATTRIBUTION Scanned at sacred-texts.com, September 2005. «Sefer Yetzirah, Kalisch 1877»".to_string(),
            "Kalisch gives a different attribution ; both are wrong. «Sefer Yetzirah, Westcott 1893»".to_string(),
        ];
        assert_eq!(uses("archive", &ex).len(), 0);
        assert_eq!(uses("attribution", &ex).len(), 1);
    }
    #[test]
    fn senses_scores_picks_and_restatements() {
        let m = "1) Water is the clear liquid in rivers, lakes, rain and the sea; people, animals and plants need it to live. 2) To water plants is to give them water. 3) Your eyes or mouth water when tears or spit come.";
        let s = parse(m);
        assert_eq!(s.len(), 3);
        assert!(s[1].1.starts_with("To water plants"));
        assert_eq!(parse("A small dog."), vec![(1, "A small dog.".to_string())]);
        assert!(parse("").is_empty());
        let ex = vec!["Rain fills the rivers and lakes with fresh water. «a:1»".to_string(), "The sea and the lakes hold most water. «a:2»".to_string()];
        let p = native_pick("water", &ex, &s).expect("confident");
        assert_eq!(p.sense, 1);
        assert!(p.evidence.contains("«lake»") || p.evidence.contains("«river»"), "{}", p.evidence);
        // no overlap: no pick
        assert!(native_pick("water", &["He drank water quickly. «a:3»".to_string()], &s).is_none());
        // a restatement of sense 1 is sense 1; a new sense is not
        assert_eq!(restates("water", "The clear liquid of rivers, lakes and rain that plants and animals need.", &s), Some(1));
        assert_eq!(restates("water", "In the Exodus story, water stands for danger crossed and freedom won.", &s), None);
        assert!(absent("The examples do not contain bi as a separate word.") && !absent("To give plants water."));
    }
    #[test]
    fn picks_parse_and_excerpts_cut_around_the_word() {
        assert_eq!(Pick::parse("2", 3), Some(Pick::Senses(vec![2])));
        assert_eq!(Pick::parse("3, 1", 3), Some(Pick::Senses(vec![1, 3])));
        assert_eq!(Pick::parse("4", 3), None);
        assert_eq!(Pick::parse("new: A tool for it.", 3), Some(Pick::New("A tool for it.".into())));
        assert_eq!(Pick::parse("same", 3), Some(Pick::Same));
        assert_eq!(Pick::parse("-", 0), Some(Pick::Keep));
        let long = format!("{} the river water rose {} «wiki:X:1»", "word ".repeat(60), "more ".repeat(60));
        let e = excerpt(&long, "water");
        assert!(e.contains("water") && !e.contains("«wiki") && e.chars().count() <= EXCERPT + 2, "{e}");
        assert_eq!(excerpt("Short one. «a:1»", "one"), "Short one.");
        assert_eq!(head("To make is to build, cook or create something: make a cake."), "To make is to build, cook or create something");
        assert!(head(&"long ".repeat(40)).chars().count() <= SENSE_CLIP + 1);
    }
    #[test]
    fn the_store_fixes_a_word_once() {
        let d = std::env::temp_dir().join(format!("nautilus-senses-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let st = Store::at(Some(d.join("senses.jsonl")));
        assert!(st.fixed().is_empty());
        st.append(&[record("water", "1", "1) Water is…", "commands", "teacher", "", 1), record("water", "2", "2) To water…", "wikipedia", "native", "", 2)]).unwrap();
        let f = st.fixed();
        assert_eq!((f["water"].pick.as_str(), f["water"].layer.as_str()), ("1", "commands"));
        assert!(Store::default().fixed().is_empty() && Store::default().append(&[json!({})]).is_ok());
    }
}
