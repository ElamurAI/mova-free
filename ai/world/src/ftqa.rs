//! FairytaleQA (Xu et al., ACL 2022, `2022.acl-long.34`) — the reference for the five-year-old test. Data copy —
//! Hugging Face `WorkInTheDark/FairytaleQA` (Apache-2.0 per the card). For measurement only, not used in training.
//!
//! The CSV does not contain whole tales: each row is a question and its section (`local`) or several sections joined
//! by a space (`summary`). We reconstruct the tale: `local` sections in order of first appearance; `summary` is split
//! into known sections, and an unmatched chunk is a section without its own questions, placed between its neighbours. Sentences —
//! a heuristic split of the tokenized text (`.`/`!`/`?`; a quote with attribution `" … ? " asked she .`
//! is one sentence). The CSV text is already lowercase and tokenized, as in the paper.

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::world::Text;

/// A FairytaleQA CSV row.
#[derive(Clone, Debug)]
pub struct Row {
    pub story: String,
    pub section: String,
    pub question: String,
    pub a1: String,
    pub a2: String,
    pub local: bool,
    pub attr: String,
    pub ex: String,
    pub ex2: String,
}

/// CSV per RFC 4180: commas, quotes with doubling `""`, line breaks inside quotes.
pub fn csv(text: &str) -> Result<Vec<Vec<String>>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut cur = String::new();
    let mut q = false;
    let mut ch = text.chars().peekable();
    while let Some(c) = ch.next() {
        if q {
            if c == '"' {
                if ch.peek() == Some(&'"') {
                    cur.push('"');
                    ch.next();
                } else {
                    q = false;
                }
            } else {
                cur.push(c);
            }
            continue;
        }
        match c {
            '"' if cur.is_empty() => q = true,
            ',' => row.push(std::mem::take(&mut cur)),
            '\r' => {}
            '\n' => {
                row.push(std::mem::take(&mut cur));
                rows.push(std::mem::take(&mut row));
            }
            _ => cur.push(c),
        }
    }
    if q {
        bail!("CSV: unclosed quotes");
    }
    if !cur.is_empty() || !row.is_empty() {
        row.push(cur);
        rows.push(row);
    }
    Ok(rows)
}

pub fn read_rows(text: &str) -> Result<Vec<Row>> {
    let rows = csv(text)?;
    let head = rows.first().context("CSV is empty")?;
    let want = ["story_name", "story_section", "question", "answer1", "answer2", "local_or_sum", "attribute", "ex_or_im", "ex_or_im2"];
    if head.iter().map(String::as_str).collect::<Vec<_>>() != want {
        bail!("CSV: header {head:?}");
    }
    rows[1..]
        .iter()
        .enumerate()
        .map(|(i, r)| {
            if r.len() != 9 {
                bail!("CSV, row {}: {} columns", i + 2, r.len());
            }
            let local = match r[5].as_str() {
                "local" => true,
                "summary" => false,
                x => bail!("CSV, row {}: local_or_sum {x:?}", i + 2),
            };
            Ok(Row {
                story: r[0].clone(),
                section: r[1].trim().to_string(),
                question: r[2].trim().to_string(),
                a1: r[3].trim().to_string(),
                a2: r[4].trim().to_string(),
                local,
                attr: r[6].trim().to_string(),
                ex: r[7].trim().to_string(),
                ex2: r[8].trim().to_string(),
            })
        })
        .collect()
}

/// A FairytaleQA question with section numbers (from 1).
#[derive(Clone, Debug)]
pub struct Qa {
    pub story: String,
    pub n: usize,
    pub attr: String,
    pub ex: String,
    pub ex2: String,
    pub local: bool,
    pub secs: Vec<u16>,
    pub question: String,
    pub a1: String,
    pub a2: String,
}

/// A reconstructed tale: sections in order; how many sections were found only inside `summary`.
#[derive(Clone, Debug)]
pub struct Story {
    pub name: String,
    pub sections: Vec<String>,
    pub orphans: usize,
}

fn norm(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Split joined sections into known ones; unknown chunks — `Err(text)`.
fn decompose(sum: &str, known: &[String]) -> Vec<Result<usize, String>> {
    let mut out = Vec::new();
    let mut rest = norm(sum);
    while !rest.is_empty() {
        // the longest known section the remainder starts with
        let hit = known.iter().enumerate().filter(|(_, k)| rest.starts_with(k.as_str())).max_by_key(|(_, k)| k.len());
        if let Some((i, k)) = hit {
            out.push(Ok(i));
            rest = rest[k.len()..].trim_start().to_string();
            continue;
        }
        // unknown chunk — up to the start of the nearest known section
        let next = known.iter().filter_map(|k| rest.find(k.as_str())).filter(|p| *p > 0).min();
        let cut = next.unwrap_or(rest.len());
        out.push(Err(rest[..cut].trim().to_string()));
        rest = rest[cut..].trim_start().to_string();
    }
    out
}

/// Reconstruct tales and questions from CSV rows (tale order as in the file).
pub fn reconstruct(rows: &[Row]) -> Result<(Vec<Story>, Vec<Qa>)> {
    let mut names: Vec<String> = Vec::new();
    for r in rows {
        if !names.contains(&r.story) {
            names.push(r.story.clone());
        }
    }
    let mut stories = Vec::new();
    let mut qas = Vec::new();
    for name in names {
        let rs: Vec<&Row> = rows.iter().filter(|r| r.story == name).collect();
        let mut secs: Vec<String> = Vec::new();
        for r in rs.iter().filter(|r| r.local) {
            let s = norm(&r.section);
            if !secs.contains(&s) {
                secs.push(s);
            }
        }
        // sections without their own questions occur only inside summary: insert after the preceding known one
        let mut orphans = 0;
        for r in rs.iter().filter(|r| !r.local) {
            // one insertion at a time, re-splitting after each (section numbers shift)
            loop {
                let parts = decompose(&r.section, &secs);
                let mut prev: Option<usize> = None;
                let mut ins = None;
                for p in parts {
                    match p {
                        Ok(i) => prev = Some(i),
                        Err(t) if t.split_whitespace().count() < 5 || secs.contains(&t) => {}
                        Err(t) => {
                            ins = Some((prev.map(|i| i + 1).unwrap_or(0), t));
                            break;
                        }
                    }
                }
                let Some((at, t)) = ins else { break };
                secs.insert(at, t);
                orphans += 1;
            }
        }
        // question → section numbers
        for (n, r) in rs.iter().enumerate() {
            let parts = decompose(&r.section, &secs);
            let mut idx = Vec::new();
            for p in parts {
                match p {
                    Ok(i) => idx.push(i as u16 + 1),
                    Err(t) if t.split_whitespace().count() < 5 => {}
                    Err(t) => bail!("{name}, question {}: section chunk not found: {:?}", n + 1, t.chars().take(80).collect::<String>()),
                }
            }
            if idx.is_empty() {
                bail!("{name}, question {}: section not found", n + 1);
            }
            if r.local && idx.len() != 1 {
                bail!("{name}, question {}: local, but {} sections", n + 1, idx.len());
            }
            qas.push(Qa {
                story: name.clone(),
                n: n + 1,
                attr: r.attr.clone(),
                ex: r.ex.clone(),
                ex2: r.ex2.clone(),
                local: r.local,
                secs: idx,
                question: r.question.clone(),
                a1: r.a1.clone(),
                a2: r.a2.clone(),
            });
        }
        stories.push(Story { name, sections: secs, orphans });
    }
    Ok((stories, qas))
}

/// Verbs and words of quote attribution: `" … ? " asked the girl .` is one sentence.
const SAID: &[&str] = &[
    "said", "says", "say", "asked", "asks", "cried", "cries", "replied", "answered", "called", "shouted", "exclaimed", "whispered", "thought",
    "continued", "added", "returned", "screamed", "sang", "roared", "laughed", "sobbed", "came", "began", "inquired", "muttered", "murmured",
    "remarked", "repeated", "declared", "begged", "pleaded", "groaned", "grumbled", "growled", "sighed", "wept", "quoth", "spoke", "went",
    "shrieked", "yelled", "howled", "croaked", "chirped", "snapped", "explained", "observed", "suggested", "commanded", "ordered", "told",
    "thundered", "stammered", "gasped", "panted", "moaned", "wailed", "called", "sneered", "retorted", "responded", "demanded", "bawled",
];

/// Sentences of a section (tokens joined by spaces). Boundary — `.`/`!`/`?` (with the quote closing the speech), except
/// a quote with attribution right after the closing quote.
pub fn sentences(section: &str, qmark: &str) -> Vec<String> {
    let toks: Vec<&str> = section.split_whitespace().collect();
    let mut out = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    let mut in_q = false;
    let mut i = 0;
    while i < toks.len() {
        let t = toks[i];
        cur.push(t);
        if t == qmark {
            in_q = !in_q;
        }
        i += 1;
        if !matches!(t, "." | "!" | "?") {
            continue;
        }
        // more consecutive punctuation (`? !`) goes into the same sentence
        while i < toks.len() && matches!(toks[i], "." | "!" | "?") {
            cur.push(toks[i]);
            i += 1;
        }
        let mut closed = false;
        if in_q && i < toks.len() && toks[i] == qmark {
            cur.push(toks[i]);
            in_q = false;
            i += 1;
            closed = true;
        }
        if closed {
            // attribution: one of the next 4 words is a speech verb, before any punctuation
            let attrib = toks[i..].iter().take(4).take_while(|w| w.chars().any(char::is_alphanumeric)).any(|w| SAID.contains(w));
            if attrib {
                continue;
            }
        }
        if in_q {
            // a sentence boundary inside a quote — only if the quote is long (otherwise keep together)
            let words = cur.iter().filter(|w| w.chars().any(char::is_alphanumeric)).count();
            if words < 12 {
                continue;
            }
        }
        out.push(cur.join(" "));
        cur.clear();
    }
    if !cur.is_empty() {
        out.push(cur.join(" "));
    }
    out
}

/// Tale text for the world: paragraph = section number.
pub fn text_of(doc: i64, s: &Story) -> Text {
    // the tale's quote mark: `"` or a standalone `'` — whichever is more frequent
    let count = |m: &str| s.sections.iter().map(|x| x.split_whitespace().filter(|w| *w == m).count()).sum::<usize>();
    let qmark = if count("'") > count("\"") { "'" } else { "\"" };
    let mut sents = Vec::new();
    for (i, sec) in s.sections.iter().enumerate() {
        for x in sentences(sec, qmark) {
            sents.push((i as u16 + 1, x));
        }
    }
    Text { doc, title: s.name.clone(), sents }
}

/// Tale record: `sec<TAB>sent<TAB>text`.
pub fn story_tsv(t: &Text) -> String {
    let mut o = String::from("# sec\tsent\ttext\n");
    for (i, (p, s)) in t.sents.iter().enumerate() {
        let _ = writeln!(o, "{p}\t{}\t{s}", i + 1);
    }
    o
}

/// Read a tale from TSV (`stories/<name>.tsv`).
pub fn read_story(path: &Path, doc: i64) -> Result<Text> {
    let name = path.file_stem().and_then(|x| x.to_str()).unwrap_or_default().to_string();
    let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    let mut sents = Vec::new();
    for (i, l) in text.lines().enumerate() {
        if l.starts_with('#') || l.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = l.splitn(3, '\t').collect();
        if c.len() != 3 {
            bail!("{}: line {} — {} columns", path.display(), i + 1, c.len());
        }
        let n: usize = c[1].parse().with_context(|| format!("{}: line {}", path.display(), i + 1))?;
        if n != sents.len() + 1 {
            bail!("{}: sentence {n} at position {}", path.display(), sents.len() + 1);
        }
        sents.push((c[0].parse::<u16>()?, c[2].to_string()));
    }
    Ok(Text { doc, title: name, sents })
}

const QA_HEAD: &str = "# n\tattr\tex\tex2\tlocal\tsecs\tquestion\tanswer1\tanswer2";

pub fn qa_tsv(qs: &[&Qa]) -> String {
    let mut o = format!("{QA_HEAD}\n");
    for q in qs {
        let secs: Vec<String> = q.secs.iter().map(|s| s.to_string()).collect();
        let _ = writeln!(
            o,
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            q.n,
            q.attr,
            q.ex,
            q.ex2,
            if q.local { "local" } else { "summary" },
            secs.join(","),
            q.question,
            q.a1,
            q.a2
        );
    }
    o
}

pub fn read_qa(path: &Path) -> Result<Vec<Qa>> {
    let story = path.file_stem().and_then(|x| x.to_str()).unwrap_or_default().to_string();
    let text = std::fs::read_to_string(path).with_context(|| path.display().to_string())?;
    let mut out = Vec::new();
    for (i, l) in text.lines().enumerate() {
        if l.starts_with('#') || l.trim().is_empty() {
            continue;
        }
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() != 9 {
            bail!("{}: line {} — {} columns", path.display(), i + 1, c.len());
        }
        out.push(Qa {
            story: story.clone(),
            n: c[0].parse()?,
            attr: c[1].into(),
            ex: c[2].into(),
            ex2: c[3].into(),
            local: c[4] == "local",
            secs: c[5].split(',').map(|x| x.parse::<u16>()).collect::<Result<_, _>>()?,
            question: c[6].into(),
            a1: c[7].into(),
            a2: c[8].into(),
        });
    }
    Ok(out)
}

/// Import: CSV → `stories/<name>.tsv`, `questions/<name>.tsv`, summary (tale, sections, orphans, sentences,
/// words, questions).
pub fn import(csv_path: &Path, out: &Path) -> Result<String> {
    let rows = read_rows(&std::fs::read_to_string(csv_path).with_context(|| csv_path.display().to_string())?)?;
    let (stories, qas) = reconstruct(&rows)?;
    std::fs::create_dir_all(out.join("stories"))?;
    std::fs::create_dir_all(out.join("questions"))?;
    let mut o = String::from("tale\tsections\torphans\tsentences\twords\tquestions\n");
    for (i, s) in stories.iter().enumerate() {
        let t = text_of(i as i64 + 1, s);
        std::fs::write(out.join("stories").join(format!("{}.tsv", s.name)), story_tsv(&t))?;
        let qs: Vec<&Qa> = qas.iter().filter(|q| q.story == s.name).collect();
        std::fs::write(out.join("questions").join(format!("{}.tsv", s.name)), qa_tsv(&qs))?;
        let words: usize = t.sents.iter().map(|(_, x)| x.split_whitespace().filter(|w| w.chars().any(char::is_alphanumeric)).count()).sum();
        let _ = writeln!(o, "{}\t{}\t{}\t{}\t{words}\t{}", s.name, s.sections.len(), s.orphans, t.sents.len(), qs.len());
    }
    Ok(o)
}
