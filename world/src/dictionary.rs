//! Mova's explanatory dictionary: every word of her lexicon explained.
//!
//! Each entry: the word, its parts of speech, its senses with their definitions and examples and the kind each sense
//! is («a kettle: a metal pot for stewing or boiling; a kind of pot»), then what Mova's own worlds say about it
//! («a kettle is used for boiling water»). The definitions come from Open English WordNet (CC BY 4.0, the source named
//! on every page); the facts from the central worlds. A word with neither is listed as a gap — to be explained later.
//! Deterministic: ordered maps, the senses in WordNet's order.
//!
//!   world dictionary build <out dir>     — dictionary.tsv, gaps.txt and one page per letter
//!   world dictionary show <word>         — one entry
//!   world dictionary merge|gaps …        — validate and merge records, the primary gaps (schooltools.rs)

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

fn oewn_dir() -> PathBuf {
    PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("raw/en-oewn/json")
}

/// A sense: part of speech, definition, examples, the words of its kind (hypernym), its synonyms.
#[derive(Clone, Debug)]
pub struct Sense {
    pub pos: String,
    pub definition: String,
    pub examples: Vec<String>,
    pub kind: Vec<String>,
    pub synonyms: Vec<String>,
}

pub struct WordNet {
    /// lemma (lower case) → senses in order
    pub senses: BTreeMap<String, Vec<Sense>>,
}

fn pos_name(p: &str) -> &'static str {
    match p {
        "n" => "noun",
        "v" => "verb",
        "a" | "s" => "adjective",
        "r" => "adverb",
        _ => "word",
    }
}

impl WordNet {
    pub fn load(dir: &Path) -> Result<WordNet> {
        let mut files: Vec<PathBuf> = std::fs::read_dir(dir).with_context(|| format!("{}", dir.display()))?.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "json")).collect();
        files.sort();
        // synsets: id → (pos, definition, examples, hypernyms, members)
        let mut syn: BTreeMap<String, (String, String, Vec<String>, Vec<String>, Vec<String>)> = BTreeMap::new();
        let mut entries: Vec<(String, String, Vec<String>)> = Vec::new();
        let strs = |v: &serde_json::Value| -> Vec<String> { v.as_array().map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default() };
        for f in &files {
            let name = f.file_name().unwrap().to_string_lossy().to_string();
            let v: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(f)?)?;
            let Some(obj) = v.as_object() else { continue };
            if name.starts_with("entries-") {
                for (lemma, by_pos) in obj {
                    for (pos, e) in by_pos.as_object().into_iter().flatten() {
                        let ids: Vec<String> = e["sense"].as_array().map(|a| a.iter().filter_map(|s| s["synset"].as_str().map(String::from)).collect()).unwrap_or_default();
                        entries.push((lemma.to_lowercase(), pos.clone(), ids));
                    }
                }
            } else if name != "frames.json" {
                for (id, s) in obj {
                    syn.insert(id.clone(), (s["partOfSpeech"].as_str().unwrap_or("").to_string(), strs(&s["definition"]).join("; "), strs(&s["example"]), strs(&s["hypernym"]), strs(&s["members"])));
                }
            }
        }
        let mut senses: BTreeMap<String, Vec<Sense>> = BTreeMap::new();
        for (lemma, pos, ids) in entries {
            for id in ids {
                let Some((p, def, ex, hyp, members)) = syn.get(&id) else { continue };
                let kind: Vec<String> = hyp.iter().filter_map(|h| syn.get(h)).flat_map(|x| x.4.iter().take(2).cloned()).take(3).collect();
                let synonyms: Vec<String> = members.iter().filter(|m| m.to_lowercase() != lemma).take(5).cloned().collect();
                senses.entry(lemma.clone()).or_default().push(Sense { pos: pos_name(if p.is_empty() { &pos } else { p }).into(), definition: def.clone(), examples: ex.iter().take(2).cloned().collect(), kind, synonyms });
            }
        }
        Ok(WordNet { senses })
    }
}

/// The WordNet, loaded once (empty when its files are missing).
pub fn wordnet() -> &'static WordNet {
    static W: std::sync::OnceLock<WordNet> = std::sync::OnceLock::new();
    W.get_or_init(|| WordNet::load(&oewn_dir()).unwrap_or(WordNet { senses: BTreeMap::new() }))
}

/// A word's kind in WordNet — the first hypernym of its first sense of that part of speech («ate» → consume,
/// «peaches» → edible fruit): the plain form found by taking off the endings, irregular forms by the mentions' stems.
pub fn kind_of(word: &str, pos: &str) -> Option<String> {
    let wn = wordnet();
    let w = word.to_lowercase();
    let irregular = crate::mentions::stem(&w);
    let mut tries = vec![w.clone()];
    for (suf, add) in [("ies", "y"), ("es", ""), ("s", ""), ("ed", ""), ("ed", "e"), ("d", ""), ("ing", ""), ("ing", "e")] {
        if let Some(b) = w.strip_suffix(suf) {
            tries.push(format!("{b}{add}"));
        }
    }
    tries.push(irregular);
    tries.into_iter().find_map(|t| wn.senses.get(&t).and_then(|ss| ss.iter().find(|s| s.pos == pos)).and_then(|s| s.kind.first().cloned()))
}

/// Webster's Revised Unabridged Dictionary (1913, public domain): headword → its definitions («Defn:» paragraphs and
/// numbered senses), the first few.
pub fn webster() -> BTreeMap<String, Vec<String>> {
    let path = PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("raw/en-webster-1913/pg29765.txt");
    let text = std::fs::read_to_string(path).unwrap_or_default();
    let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut head: Option<String> = None;
    let mut para = String::new();
    let mut flush = |head: &Option<String>, para: &mut String, out: &mut BTreeMap<String, Vec<String>>| {
        let p = para.trim().to_string();
        para.clear();
        let Some(h) = head else { return };
        let def = p.strip_prefix("Defn:").map(str::trim).map(String::from).or_else(|| {
            let mut c = p.chars();
            (c.next().is_some_and(|x| x.is_ascii_digit()) && p.contains(". ")).then(|| p.split_once(". ").map(|x| x.1.trim().to_string()).unwrap_or_default())
        });
        if let Some(d) = def.filter(|d| d.len() > 3) {
            let d = d.split(" -- ").next().unwrap_or(&d).replace("  ", " ");
            let v = out.entry(h.clone()).or_default();
            if v.len() < 3 {
                v.push(d);
            }
        }
    };
    for l in text.lines() {
        let t = l.trim();
        if !t.is_empty() && t.len() < 40 && t.chars().all(|c| c.is_ascii_uppercase() || c == '-' || c == ' ' || c == ';' || c == '\'') && t.chars().any(|c| c.is_ascii_uppercase()) {
            flush(&head, &mut para, &mut out);
            head = Some(t.split(';').next().unwrap_or(t).trim().to_lowercase());
            continue;
        }
        if t.is_empty() {
            flush(&head, &mut para, &mut out);
        } else {
            para.push(' ');
            para.push_str(t);
        }
    }
    flush(&head, &mut para, &mut out);
    out
}

/// A derived word explained from its base: (the explanation, the base).
fn by_morphology(w: &str, known: &dyn Fn(&str) -> Option<String>) -> Option<(String, String)> {
    const PRE: [(&str, &str); 14] = [("over", "too much"), ("under", "too little"), ("un", "not"), ("non", "not"), ("re", "again"), ("mis", "wrongly"), ("pre", "before"), ("post", "after"), ("anti", "against"), ("out", "more than; beyond"), ("sub", "below"), ("super", "above; beyond"), ("semi", "half"), ("self", "of oneself")];
    const SUF: [(&str, &str); 14] = [("ness", "the quality of being"), ("less", "without"), ("ful", "full of"), ("ship", "the state or office of"), ("hood", "the state of being"), ("ism", "the doctrine or practice of"), ("ist", "a person who practises"), ("ity", "the quality of being"), ("ment", "the act or result of"), ("ation", "the act of"), ("able", "able to be"), ("ly", "in a way that is"), ("er", "one who or what"), ("ing", "the act of")];
    for (p, m) in PRE {
        if let Some(base) = w.strip_prefix(p).filter(|b| b.len() >= 3) {
            let base = base.trim_start_matches('-');
            if let Some(d) = known(base) {
                return Some((format!("{m} {base} ({base}: {d})"), base.to_string()));
            }
        }
    }
    for (sx, m) in SUF {
        if let Some(stem) = w.strip_suffix(sx).filter(|b| b.len() >= 3) {
            for base in [stem.to_string(), format!("{stem}e"), stem.trim_end_matches('i').to_string() + "y"] {
                if let Some(d) = known(&base) {
                    return Some((format!("{m} {base} ({base}: {d})"), base));
                }
            }
        }
    }
    None
}

/// The words used as proper names in Mova's lexicon (tagged NNP).
fn proper_names() -> BTreeSet<String> {
    include_str!("../../en/data/forms.tsv").lines().filter_map(|l| {
        let c: Vec<&str> = l.split('\t').collect();
        (c.len() > 1 && c[1] == "NNP").then(|| c[0].to_lowercase())
    }).collect()
}

/// The words of Mova's lexicon: lower-case words of letters (and inner hyphens).
fn lexicon() -> Vec<String> {
    let t = include_str!("../../en/data/lemmas.tsv");
    let mut v: BTreeSet<String> = BTreeSet::new();
    for l in t.lines() {
        let w = l.split('\t').next().unwrap_or("").to_lowercase();
        if w.len() >= 2 && w.chars().all(|c| c.is_ascii_lowercase() || c == '-') && !w.starts_with('-') && !w.ends_with('-') {
            v.insert(w);
        }
    }
    v.into_iter().collect()
}

/// What Mova's worlds say about a word: a few facts, the plainest kinds first.
fn facts_about(k: &crate::know::Knowledge, w: &str) -> Vec<String> {
    use crate::know::Link;
    let Some(fs) = k.by_subj.get(w) else { return Vec::new() };
    let rank = |l: Link| match l { Link::IsA => 0, Link::UsedFor => 1, Link::MadeOf => 2, Link::Has => 3, Link::Can => 4, Link::LivesIn => 5, _ => 9 };
    let mut v: Vec<&crate::know::Fact> = fs.iter().filter(|f| rank(f.link) < 9 && !f.source.starts_with("derived")).collect();
    v.sort_by(|a, b| rank(a.link).cmp(&rank(b.link)).then(a.source.len().cmp(&b.source.len())).then(a.source.cmp(&b.source)));
    let mut seen = BTreeSet::new();
    v.into_iter().filter(|f| seen.insert(f.show())).take(6).map(|f| f.source.clone()).collect()
}

/// One entry in plain text.
pub fn entry(wn: &WordNet, k: &crate::know::Knowledge, w: &str) -> Option<String> {
    entry_with(wn, k, &BTreeMap::new(), &BTreeSet::new(), w)
}

/// The records improved by the large model for the words of the school lessons (public, `data/dictionary-improved.tsv`):
/// word → [date (closed build only, from its list), part of speech, meaning for a pupil, sense in the lessons, examples, note].
pub fn improved() -> BTreeMap<String, [String; 6]> {
    improved_from(include_str!("../data/dictionary-improved.tsv"))
}

/// The improved records parsed from a text in the format of `data/dictionary-improved.tsv`.
pub fn improved_from(text: &str) -> BTreeMap<String, [String; 6]> {
    let dates = improved_dates();
    text.lines().filter(|l| !l.starts_with('#')).filter_map(|l| {
        let c: Vec<&str> = l.split('\t').collect();
        let w = c.first()?.to_lowercase();
        (c.len() >= 6).then(|| (w.clone(), [dates.get(&w).cloned().unwrap_or_default(), c[1].into(), c[2].into(), c[3].into(), c[4].into(), c[5].into()]))
    }).collect()
}

/// The list of improved words with the dates of their improvement is kept apart, outside the release; the records
/// never carry a date.
fn improved_dates() -> BTreeMap<String, String> {
    BTreeMap::new()
}

/// The improved record of a word, as the first block of its entry.
fn improved_block(w: &str) -> Option<String> {
    static M: std::sync::OnceLock<BTreeMap<String, [String; 6]>> = std::sync::OnceLock::new();
    let r = M.get_or_init(improved).get(w)?;
    // the date of the improvement lives only in its own list (outside the release), never in the record
    Some(format!("  for a pupil ({}): {}\n    in the lessons: {}\n    «{}»\n    note: {}\n", r[1], r[2], r[3], r[4].replace(" | ", "» «"), r[5]))
}

/// One entry, with the fallbacks for a word WordNet does not know: Webster 1913, the base of a derived word, a name.
pub fn entry_with(wn: &WordNet, k: &crate::know::Knowledge, web: &BTreeMap<String, Vec<String>>, names: &BTreeSet<String>, w: &str) -> Option<String> {
    let senses = wn.senses.get(w);
    let facts = facts_about(k, w);
    let better = improved_block(w).unwrap_or_default();
    if senses.is_none() {
        let mut s = format!("{w}\n{better}");
        let known = |b: &str| wn.senses.get(b).and_then(|v| v.first()).map(|x| x.definition.clone()).or_else(|| web.get(b).and_then(|v| v.first()).cloned());
        if let Some(defs) = web.get(w) {
            writeln!(s, "  (Webster's Revised Unabridged Dictionary, 1913)").ok();
            for (i, d) in defs.iter().enumerate() {
                writeln!(s, "    {}. {}", i + 1, d).ok();
            }
        } else if let Some((m, _)) = by_morphology(w, &known) {
            writeln!(s, "  (from its parts)\n    1. {m}").ok();
        } else if names.contains(w) {
            writeln!(s, "  proper name\n    1. a name of a person, a place or a thing, written with a capital letter").ok();
        } else if facts.is_empty() {
            return None;
        }
        if !facts.is_empty() {
            writeln!(s, "  in my worlds:").ok();
            for f in facts {
                writeln!(s, "    · {f}").ok();
            }
        }
        return Some(s);
    }
    let mut s = format!("{w}\n{better}");
    let mut last_pos = String::new();
    for (i, sn) in senses.into_iter().flatten().enumerate().take(12) {
        if sn.pos != last_pos {
            writeln!(s, "  {}", sn.pos).ok();
            last_pos = sn.pos.clone();
        }
        write!(s, "    {}. {}", i + 1, sn.definition).ok();
        if !sn.kind.is_empty() {
            write!(s, " — a kind of {}", sn.kind.join(", ")).ok();
        }
        if !sn.synonyms.is_empty() {
            write!(s, "; also: {}", sn.synonyms.join(", ")).ok();
        }
        s.push('\n');
        for e in &sn.examples {
            writeln!(s, "       «{e}»").ok();
        }
    }
    if !facts.is_empty() {
        writeln!(s, "  in my worlds:").ok();
        for f in facts {
            writeln!(s, "    · {f}").ok();
        }
    }
    Some(s)
}

fn esc(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Build the whole dictionary: a tsv, the gaps, a page per letter.
pub fn build(out: &Path) -> Result<String> {
    let wn = WordNet::load(&oewn_dir())?;
    let k = crate::know::knowledge();
    let web = webster();
    let names = proper_names();
    let words = lexicon();
    std::fs::create_dir_all(out)?;
    let mut tsv = String::from("# word\tpart of speech\tsense\tdefinition\texamples\tkind\tsynonyms (Open English WordNet 2025, CC BY 4.0)\n");
    let mut gaps = String::new();
    let mut pages: BTreeMap<char, String> = BTreeMap::new();
    let (mut explained, mut by_facts_only, mut senses_n) = (0, 0, 0);
    for w in &words {
        let Some(e) = entry_with(&wn, &k, &web, &names, w) else {
            gaps.push_str(w);
            gaps.push('\n');
            continue;
        };
        explained += 1;
        if !wn.senses.contains_key(w) {
            by_facts_only += 1;
        }
        for (i, sn) in wn.senses.get(w).into_iter().flatten().enumerate() {
            senses_n += 1;
            writeln!(tsv, "{w}\t{}\t{}\t{}\t{}\t{}\t{}", sn.pos, i + 1, sn.definition.replace('\t', " "), sn.examples.join(" | ").replace('\t', " "), sn.kind.join(", "), sn.synonyms.join(", "))?;
        }
        let letter = w.chars().next().unwrap_or('_');
        let p = pages.entry(letter).or_default();
        let mut lines = e.lines();
        let head = lines.next().unwrap_or(w);
        write!(p, "<div class=\"e\" id=\"{}\"><b>{}</b><pre>{}</pre></div>", esc(head), esc(head), esc(&lines.collect::<Vec<_>>().join("\n")))?;
    }
    std::fs::write(out.join("dictionary.tsv"), &tsv)?;
    std::fs::write(out.join("gaps.txt"), &gaps)?;
    let css = ":root{--bg:#f7f5f0;--ink:#1c1d21;--muted:#62646c;--line:#e3dfd6;--accent:#0f766e;--panel:#fff}@media (prefers-color-scheme:dark){:root{--bg:#141518;--ink:#ebeae6;--muted:#a1a1aa;--line:#2e3036;--accent:#2dd4bf;--panel:#1c1d21}}body{margin:0;background:var(--bg);color:var(--ink);font:16px/1.5 system-ui,sans-serif}.wrap{max-width:900px;margin:0 auto;padding:30px 16px}a{color:var(--accent)}.e{background:var(--panel);border:1px solid var(--line);border-radius:10px;padding:8px 12px;margin:8px 0}.e pre{margin:4px 0 0;white-space:pre-wrap;font:14px/1.45 system-ui,sans-serif;color:var(--ink)}.letters a{display:inline-block;margin:3px 6px;font-size:1.2em}p.m{color:var(--muted)}";
    let head = |t: &str| format!("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><meta name=\"robots\" content=\"noindex, nofollow\"><title>{t} — Mova's dictionary</title><style>{css}</style></head><body><div class=\"wrap\">");
    let letters: String = pages.keys().map(|c| format!("<a href=\"{c}.html\">{c}</a>")).collect();
    for (c, body) in &pages {
        std::fs::write(out.join(format!("{c}.html")), format!("{}<p><a href=\"./\">← the dictionary</a></p><h1>{c}</h1><div class=\"letters\">{letters}</div>{body}<p class=\"m\">Definitions: Open English WordNet 2025 (CC BY 4.0); Webster's Revised Unabridged Dictionary 1913 (public domain); derived words explained from their parts. «In my worlds»: sentences of Mova's central worlds.</p></div></body></html>", head(&c.to_string())))?;
    }
    // the records improved for the lessons (closed build only): a page of their own, with their dates
    let imp = improved();
    let improved_link = if imp.is_empty() {
        String::new()
    } else {
        let mut rows = String::new();
        for (w, r) in &imp {
            let c = w.chars().next().unwrap_or('_');
            write!(rows, "<div class=\"e\"><b><a href=\"{c}.html#{}\">{}</a></b> <span class=\"m\">{}</span><pre>{}\nin the lessons: {}\nnote: {}</pre></div>", esc(w), esc(w), esc(&r[1]), esc(&r[2]), esc(&r[3]), esc(&r[5]))?;
        }
        std::fs::write(out.join("improved.html"), format!("{}<p><a href=\"./\">← the dictionary</a></p><h1>Records improved for the lessons</h1><p class=\"m\">{} words of the school lessons.</p>{rows}</div></body></html>", head("Improved records"), imp.len()))?;
        format!("<p><a href=\"improved.html\">Records improved for the lessons</a> — {} words.</p>", imp.len())
    };
    let summary = format!("{} words of the lexicon: {explained} explained ({} by WordNet with {senses_n} senses; {by_facts_only} by Webster 1913, their parts, a name or the worlds alone), {} gaps", words.len(), explained - by_facts_only, words.len() - explained);
    std::fs::write(out.join("index.html"), format!("{}<p><a href=\"../\">← Mova</a></p><h1>Mova's explanatory dictionary</h1><p class=\"m\">Every word of Mova's lexicon with its senses, definitions, examples and kinds, and what her own worlds say about it. {summary}.</p><p><a href=\"schools/primary-school/\"><b>The primary school dictionary</b></a> — only the words of the primary lessons and the words their records need, by letters.</p>{improved_link}<div class=\"letters\">{letters}</div></div></body></html>", head("Mova's explanatory dictionary")))?;
    Ok(summary)
}

/// `world dictionary build <out dir> | show <word>`.
/// Every text of the courses: the lessons as data (their explanations and a draw of their exercises and tests), and
/// the knowledge courses (explanations, facts, questions and every answer).
fn course_texts() -> Vec<String> {
    let mut v = Vec::new();
    for l in crate::lessons::all().unwrap_or_default() {
        v.push(l.title.clone());
        v.extend(l.explain.iter().cloned());
        v.extend(l.exercises(20).into_iter().map(|d| d.text));
        v.extend(l.test(20).into_iter().map(|d| d.text));
        v.extend(l.exam(8).into_iter().map(|d| d.text));
    }
    for c in crate::knowledge::course_names() {
        for l in crate::knowledge::course(c).unwrap_or_default() {
            v.push(l.title.clone());
            v.extend(l.explain.iter().cloned());
            v.extend(l.facts.iter().cloned());
            for (q, r, w) in l.exercises.iter().chain(&l.tests).chain(&l.exams) {
                v.push(q.clone());
                v.push(r.clone());
                v.extend(w.iter().cloned());
            }
        }
    }
    v
}

/// The plain form of a word for the pupils' dictionary: an irregular form's base, else the form itself if WordNet or
/// the improved records know it, else the form with its ending taken off that they know.
fn plain_form(wn: &WordNet, improved: &BTreeMap<String, [String; 6]>, w: &str) -> Option<String> {
    const IRREGULAR: [(&str, &str); 60] = [("has", "have"), ("had", "have"), ("is", "be"), ("are", "be"), ("was", "be"), ("were", "be"), ("am", "be"), ("did", "do"), ("does", "do"), ("done", "do"), ("found", "find"), ("lost", "lose"), ("ate", "eat"), ("eaten", "eat"), ("bought", "buy"), ("gave", "give"), ("given", "give"), ("met", "meet"), ("told", "tell"), ("sold", "sell"), ("got", "get"), ("made", "make"), ("took", "take"), ("taken", "take"), ("went", "go"), ("gone", "go"), ("came", "come"), ("ran", "run"), ("saw", "see"), ("seen", "see"), ("kept", "keep"), ("spent", "spend"), ("paid", "pay"), ("brought", "bring"), ("caught", "catch"), ("thought", "think"), ("won", "win"), ("grew", "grow"), ("drew", "draw"), ("wrote", "write"), ("held", "hold"), ("fell", "fall"), ("began", "begin"), ("children", "child"), ("feet", "foot"), ("teeth", "tooth"), ("men", "man"), ("women", "woman"), ("people", "person"), ("mice", "mouse"), ("her", "she"), ("him", "he"), ("his", "he"), ("them", "they"), ("their", "they"), ("us", "we"), ("sat", "sit"), ("stood", "stand"), ("sent", "send"), ("built", "build")];
    if let Some(b) = IRREGULAR.iter().find(|x| x.0 == w).map(|x| x.1) {
        return Some(b.to_string());
    }
    let known = |x: &str| improved.contains_key(x) || wn.senses.contains_key(x);
    if improved.contains_key(w) {
        return Some(w.to_string());
    }
    for (suf, add) in [("ies", "y"), ("ves", "f"), ("s", ""), ("es", ""), ("ed", ""), ("ed", "e"), ("ing", ""), ("ing", "e"), ("er", ""), ("est", "")] {
        if let Some(b) = w.strip_suffix(suf) {
            let c = format!("{b}{add}");
            if c.len() > 1 && improved.contains_key(&c) {
                return Some(c);
            }
        }
    }
    if known(w) {
        return Some(w.to_string());
    }
    for (suf, add) in [("ies", "y"), ("ves", "f"), ("s", ""), ("es", ""), ("ed", ""), ("ed", "e"), ("ing", ""), ("ing", "e")] {
        if let Some(b) = w.strip_suffix(suf) {
            let c = format!("{b}{add}");
            if c.len() > 1 && known(&c) {
                return Some(c);
            }
            // a doubled consonant: «stopped» → stop
            let mut d = b.to_string();
            if d.len() > 2 && d.chars().last() == d.chars().rev().nth(1) {
                d.pop();
                if known(&d) {
                    return Some(d);
                }
            }
        }
    }
    None
}

/// The words of the courses that have no improved record yet, the most used first: (word, how often).
pub fn missing_words() -> Result<Vec<(String, usize)>> {
    let wn = WordNet::load(&oewn_dir())?;
    let improved = improved();
    let names: BTreeSet<&str> = ["tom", "anna", "sam", "lily", "max", "mia", "ben", "zoe", "jack", "emma", "leo", "ruby", "noah", "ella", "omar", "ivy"].into();
    let mut count: BTreeMap<String, usize> = BTreeMap::new();
    for t in course_texts() {
        for w in t.split(|c: char| !c.is_alphabetic()).filter(|w| w.len() > 1) {
            // a capital inside a text is a name or a place; the lower-case words are the dictionary's
            if w.chars().next().is_some_and(char::is_uppercase) && !wn.senses.contains_key(&w.to_lowercase()) {
                continue;
            }
            let lw = w.to_lowercase();
            if names.contains(lw.as_str()) {
                continue;
            }
            if let Some(b) = plain_form(&wn, &improved, &lw) {
                if !improved.contains_key(&b) {
                    *count.entry(b).or_default() += 1;
                }
            }
        }
    }
    let mut v: Vec<(String, usize)> = count.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    Ok(v)
}

/// The texts of each lesson of a course, in the course's order: («maths», n, texts), («world», n, texts) …
fn lesson_texts() -> Vec<(&'static str, usize, Vec<String>)> {
    let mut v = Vec::new();
    for l in crate::lessons::all().unwrap_or_default() {
        let mut t = vec![l.title.clone()];
        t.extend(l.explain.iter().cloned());
        t.extend(l.exercises(120).into_iter().map(|d| d.text));
        t.extend(l.test(60).into_iter().map(|d| d.text));
        t.extend(l.exam(30).into_iter().map(|d| d.text));
        v.push(("maths", l.n, t));
    }
    for c in crate::knowledge::course_names() {
        for l in crate::knowledge::course(c).unwrap_or_default() {
            let mut t = vec![l.title.clone()];
            t.extend(l.explain.iter().chain(&l.facts).cloned());
            for (q, r, w) in l.exercises.iter().chain(&l.tests).chain(&l.exams) {
                t.push(q.clone());
                t.push(r.clone());
                t.extend(w.iter().cloned());
            }
            v.push((c, l.n, t));
        }
    }
    v
}

/// `world dictionary saturation`: how near the courses are to saturation — the new words each block of ten lessons
/// brings (fewer and fewer as the language fills), and how often the words are met (the common ones hundreds and
/// thousands of times).
pub fn saturation() -> String {
    let words = |t: &str| -> Vec<String> { t.split(|c: char| !c.is_alphabetic() && c != '\'').map(|w| w.trim_matches('\'').to_lowercase()).filter(|w| w.len() > 1).collect() };
    let mut out = String::new();
    let mut freq: BTreeMap<String, usize> = BTreeMap::new();
    let mut seen_all: BTreeSet<String> = BTreeSet::new();
    let texts = lesson_texts();
    for course in std::iter::once("maths").chain(crate::knowledge::course_names()) {
        let ls: Vec<&(&str, usize, Vec<String>)> = texts.iter().filter(|x| x.0 == course).collect();
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut line = String::new();
        for (i, block) in ls.chunks(10).enumerate() {
            let before = seen.len();
            for (_, _, t) in block {
                for x in t {
                    for w in words(x) {
                        *freq.entry(w.clone()).or_default() += 1;
                        seen_all.insert(w.clone());
                        seen.insert(w);
                    }
                }
            }
            line += &format!("{}{}", if i == 0 { "" } else { " " }, seen.len() - before);
        }
        out += &format!("«{course}»: {} lessons, {} words; new words per ten lessons: {line}\n", ls.len(), seen.len());
    }
    let n = |k: usize| freq.values().filter(|&&c| c >= k).count();
    out += &format!("all courses: {} words; met ≥ 10 times: {}, ≥ 100: {}, ≥ 1000: {}, ≥ 10000: {}; met once: {}\n", seen_all.len(), n(10), n(100), n(1000), n(10000), freq.values().filter(|&&c| c == 1).count());
    let mut top: Vec<(&String, &usize)> = freq.iter().collect();
    top.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    // MOVA_TOPWORDS=N: every word met at least N times, one per line, most met first
    if let Some(k) = std::env::var("MOVA_TOPWORDS").ok().and_then(|x| x.parse::<usize>().ok()) {
        return top.iter().filter(|(_, c)| **c >= k).map(|(w, c)| format!("{w}\t{c}\n")).collect();
    }
    out += &format!("the most met: {}\n", top.iter().take(25).map(|(w, c)| format!("{w} {c}")).collect::<Vec<_>>().join(", "));
    out
}

/// The texts of the courses of one school (`knowledge::grade`); the maths lessons belong to the primary school.
fn school_texts(school: &str) -> Vec<String> {
    let mut v = Vec::new();
    if school == crate::knowledge::PRIMARY {
        for l in crate::lessons::all().unwrap_or_default() {
            v.push(l.title.clone());
            v.extend(l.explain.iter().cloned());
            v.extend(l.exercises(20).into_iter().map(|d| d.text));
            v.extend(l.test(20).into_iter().map(|d| d.text));
            v.extend(l.exam(8).into_iter().map(|d| d.text));
        }
    }
    for &(c, ..) in crate::knowledge::COURSES {
        if c == "dictionary" || crate::knowledge::grade(c) != school {
            continue;
        }
        for l in crate::knowledge::course(c).unwrap_or_default() {
            v.push(l.title.clone());
            v.extend(l.explain.iter().chain(&l.facts).cloned());
            for (q, r, w) in l.exercises.iter().chain(&l.tests).chain(&l.exams) {
                v.push(q.clone());
                v.push(r.clone());
                v.extend(w.iter().cloned());
            }
        }
    }
    v
}

/// The schools in order; each school's dictionary holds the words of its own and every earlier school (cumulative).
/// Only the primary school is taught for now; the later schools are placeholders and get no dictionary.
const SCHOOLS: [&str; 1] = [crate::knowledge::PRIMARY];

/// The words of a school's dictionary: the improved records of every word its lessons use, and — so that every
/// record can be read — of every word those records use in turn, until nothing new comes in (enough, and no more).
/// Returns (the words, the words of the lessons that have no improved record yet).
fn school_words(wn: &WordNet, improved: &BTreeMap<String, [String; 6]>, texts: &[String], start: &BTreeSet<String>) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut have = start.clone();
    let mut gaps = BTreeSet::new();
    let mut queue: Vec<String> = Vec::new();
    let mut take = |t: &str, gaps: &mut BTreeSet<String>, queue: &mut Vec<String>, have: &BTreeSet<String>, from_lessons: bool| {
        for w in t.split(|c: char| !c.is_alphabetic()).filter(|w| w.len() > 1) {
            if w.chars().next().is_some_and(char::is_uppercase) && !wn.senses.contains_key(&w.to_lowercase()) && !improved.contains_key(&w.to_lowercase()) {
                continue;
            }
            let Some(b) = plain_form(wn, improved, &w.to_lowercase()) else { continue };
            if improved.contains_key(&b) {
                if !have.contains(&b) {
                    queue.push(b);
                }
            } else if from_lessons {
                gaps.insert(b);
            }
        }
    };
    for t in texts {
        take(t, &mut gaps, &mut queue, &have, true);
    }
    while let Some(w) = queue.pop() {
        if !have.insert(w.clone()) {
            continue;
        }
        let r = &improved[&w];
        let body = format!("{} {} {} {}", r[2], r[3], r[4], r[5]);
        take(&body, &mut gaps, &mut queue, &have, false);
    }
    (have, gaps)
}

/// The words of the primary lessons without an improved record — the list `dictionary schools` writes to gaps.txt —
/// computed from the records on disk (not the ones built into the binary), so a fresh merge counts at once.
pub fn primary_gaps(records: &Path) -> Result<Vec<String>> {
    let wn = WordNet::load(&oewn_dir())?;
    let improved = improved_from(&std::fs::read_to_string(records).with_context(|| format!("reading {}", records.display()))?);
    let texts = school_texts(crate::knowledge::PRIMARY);
    Ok(school_words(&wn, &improved, &texts, &BTreeSet::new()).1.into_iter().collect())
}

/// `world dictionary schools <out dir>`: the explanatory dictionary by school, cumulative — the primary school's holds
/// only the words its lessons use and the words their records need; each page a letter, or two first letters when a
/// letter has more than 500 records.
pub fn schools(out: &Path) -> Result<String> {
    let wn = WordNet::load(&oewn_dir())?;
    let improved = improved();
    let css = ":root{--bg:#f7f5f0;--ink:#1c1d21;--muted:#62646c;--line:#e3dfd6;--accent:#0f766e;--panel:#fff}@media (prefers-color-scheme:dark){:root{--bg:#141518;--ink:#ebeae6;--muted:#a1a1aa;--line:#2e3036;--accent:#2dd4bf;--panel:#1c1d21}}body{margin:0;background:var(--bg);color:var(--ink);font:16px/1.5 system-ui,sans-serif}.wrap{max-width:900px;margin:0 auto;padding:30px 16px}a{color:var(--accent)}.e{background:var(--panel);border:1px solid var(--line);border-radius:10px;padding:8px 12px;margin:8px 0}.e pre{margin:4px 0 0;white-space:pre-wrap;font:14px/1.45 system-ui,sans-serif}.letters a{display:inline-block;margin:3px 6px;font-size:1.1em}p.m,span.m{color:var(--muted)}";
    let head = |t: &str| format!("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><meta name=\"robots\" content=\"noindex, nofollow\"><title>{t} — Mova's dictionary</title><style>{css}</style></head><body><div class=\"wrap\">");
    let mut summary = String::new();
    let mut cards = String::new();
    let mut so_far: BTreeSet<String> = BTreeSet::new();
    for school in SCHOOLS {
        let texts = school_texts(school);
        if texts.is_empty() && school != crate::knowledge::PRIMARY {
            continue;
        }
        let before = so_far.clone();
        let (words, gaps) = school_words(&wn, &improved, &texts, &so_far);
        so_far = words.clone();
        let slug = school.replace(' ', "-");
        let d = out.join(&slug);
        std::fs::create_dir_all(&d)?;
        // the pages: a letter, or its two first letters when the letter has more than 500 records
        let mut by_letter: BTreeMap<String, Vec<&String>> = BTreeMap::new();
        for w in &words {
            by_letter.entry(w.chars().take(1).collect()).or_default().push(w);
        }
        let mut pages: BTreeMap<String, Vec<&String>> = BTreeMap::new();
        for (l, ws) in by_letter {
            if ws.len() > 500 {
                for w in ws {
                    let two: String = w.chars().take(2).collect();
                    pages.entry(two).or_default().push(w);
                }
            } else {
                pages.insert(l, ws);
            }
        }
        let index: String = pages.iter().map(|(k, ws)| format!("<a href=\"{k}.html\">{k}</a><span class=\"m\">{}</span> ", ws.len())).collect();
        for (k, ws) in &pages {
            let mut body = String::new();
            for w in ws {
                let r = &improved[*w];
                let new = if before.contains(*w) { "" } else { " <span class=\"m\">new at this school</span>" };
                write!(body, "<div class=\"e\" id=\"{}\"><b>{}</b> <span class=\"m\">{}</span>{new}<pre>{}\nin the lessons: {}\n«{}»\nnote: {}</pre></div>", esc(w), esc(w), esc(&r[1]), esc(&r[2]), esc(&r[3]), esc(&r[4].replace(" | ", "» «")), esc(&r[5]))?;
            }
            std::fs::write(d.join(format!("{k}.html")), format!("{}<p><a href=\"./\">← the {school} dictionary</a></p><h1>{k}</h1><div class=\"letters\">{index}</div>{body}</div></body></html>", head(&format!("{school}: {k}"))))?;
        }
        let list: String = words.iter().map(|w| format!("{w}\n")).collect();
        // the school's own dictionary, kept in the repo: each school keeps its records; a later school will have its
        // own, bigger and better ones, which may replace the less important records of the schools before it
        let data = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data").join("dictionary-schools");
        std::fs::create_dir_all(&data)?;
        let mut tsv = String::from("# word\tpart of speech\tmeaning for a pupil\tsense in the lessons\texamples\tnote\n");
        for w in &words {
            let r = &improved[w];
            writeln!(tsv, "{w}\t{}\t{}\t{}\t{}\t{}", r[1], r[2], r[3], r[4], r[5])?;
        }
        std::fs::write(data.join(format!("{slug}.tsv")), &tsv)?;
        std::fs::write(d.join("words.txt"), &list)?;
        std::fs::write(d.join("gaps.txt"), gaps.iter().map(|w| format!("{w}\n")).collect::<String>())?;
        let line = format!("{school}: {} words ({} new at this school), {} words of its lessons still without a record", words.len(), words.len() - before.len(), gaps.len());
        std::fs::write(d.join("index.html"), format!("{}<p><a href=\"../\">← the dictionaries by school</a></p><h1>The {school} dictionary</h1><p class=\"m\">{line}. Every word its lessons use, and every word those records need to be read — enough, and no more. The dictionaries grow: each school holds the words of the schools before it.</p><div class=\"letters\">{index}</div></div></body></html>", head(school)))?;
        cards += &format!("<p><a href=\"{slug}/\"><b>The {school}</b></a> — {} words</p>", words.len());
        summary += &line;
        summary.push('\n');
    }
    std::fs::write(out.join("index.html"), format!("{}<p><a href=\"../\">← Mova</a></p><h1>Mova's explanatory dictionary, by school</h1><p class=\"m\">The small one is taught the primary school for now.</p>{cards}</div></body></html>", head("By school")))?;
    Ok(summary)
}

pub fn main_args(args: &[String]) -> Result<()> {
    match args.get(1).map(String::as_str) {
        Some("merge" | "gaps") => crate::schooltools::dictionary_args(args)?,
        Some("missing") => {
            // `world dictionary missing [out file]`: the words of the courses with no improved record, most used first
            let v = missing_words()?;
            let list: String = v.iter().map(|(w, n)| format!("{w}\t{n}\n")).collect();
            if let Some(out) = args.get(2) {
                std::fs::write(out, &list)?;
            }
            println!("{} words of the courses have no improved record yet; the first: {}", v.len(), v.iter().take(30).map(|x| x.0.as_str()).collect::<Vec<_>>().join(" "));
        }
        Some("saturation") => print!("{}", saturation()),
        Some("schools") => print!("{}", schools(Path::new(args.get(2).context("usage: world dictionary schools <out dir>")?))?),
        Some("build") => println!("{}", build(Path::new(args.get(2).context("usage: world dictionary build <out dir>")?))?),
        Some("show") => {
            let wn = WordNet::load(&oewn_dir())?;
            let k = crate::know::knowledge();
            let w = args.get(2).cloned().unwrap_or_default().to_lowercase();
            println!("{}", entry(&wn, &k, &w).unwrap_or_else(|| format!("{w}: no explanation yet (a gap)")));
        }
        _ => {
            println!("usage: world dictionary build <out dir> | show <word> | missing [out] | saturation | schools <out dir>");
            print!("{}", crate::schooltools::DICTIONARY_USAGE);
        }
    }
    Ok(())
}
