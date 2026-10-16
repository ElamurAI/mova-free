//! Word problems from the old arithmetic primers (public domain, archive.org OCR, gathered in
//! `$MOVA_DATA/research/raw/corpus-arith`): the problems whose answer is printed after them («… how many oranges can you buy
//! for 354 cents? Ans. 59.»), as a book of problems in SVAMP's form (Body, Question, Answer) for the school.
//!
//! Only word problems with a whole-number answer are kept: a question mark, enough words, no bare sums
//! («195 X 3. Ans. 585.» is drill, not a story). OCR slips of «Ans.» («Am.», «Aiis.») are read too.
//!
//!   world primers extract <out.json> [book dir …]   — the problems of the books with answers in line

use anyhow::Result;

/// The books whose answers stand in line after each problem.
pub const IN_LINE: [&str; 2] = ["brooks-1869-normal-elementary-arithmetic", "ray-1880-rays-new-higher-arithmetic"];

pub fn corpus() -> std::path::PathBuf {
    std::path::PathBuf::from(format!("{}/research/raw/corpus-arith", std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))))
}

/// The answer marker and the number after it: (where the marker starts, the number).
fn answer_of(chunk: &str) -> Option<(usize, i64)> {
    for m in ["Ans.", "Ans ", "Am.", "Aiis.", "Ans,", "Aus."] {
        if let Some(k) = chunk.rfind(m) {
            let rest = chunk[k + m.len()..].trim_start().trim_start_matches('$').trim_start();
            let digits: String = rest.chars().take_while(|c| c.is_ascii_digit() || *c == ',').collect();
            let after = rest[digits.len()..].chars().next();
            // a fraction or a decimal after the number is not a whole answer
            if digits.is_empty() || matches!(after, Some('.') if rest[digits.len() + 1..].starts_with(|c: char| c.is_ascii_digit())) || matches!(after, Some('/' | '£' | '^')) {
                return None;
            }
            return digits.replace(',', "").parse().ok().map(|v| (k, v));
        }
    }
    None
}

/// The numbered problems of a text: «12. …» up to the next number at a line's start.
fn problems(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for line in text.lines() {
        let t = line.trim_start();
        let starts = t.split_once(". ").is_some_and(|(n, _)| !n.is_empty() && n.len() <= 3 && n.chars().all(|c| c.is_ascii_digit()));
        if starts && !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
        if !cur.is_empty() {
            cur.push(' ');
        }
        cur += t;
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// A word problem with its answer, split into the story and the question.
fn word_problem(chunk: &str) -> Option<(String, String, i64)> {
    let (k, ans) = answer_of(chunk)?;
    story_and_question(&chunk[..k]).map(|(s, q)| (s, q, ans))
}

/// OCR's marks of a garbled fraction or sign: a problem with them is left out.
fn garbled(t: &str) -> bool {
    t.chars().any(|c| matches!(c, '§' | '|' | '^' | '£' | '°' | '½' | '¼' | '¾' | '/' | '"' | '%'))
}

/// A numbered problem's text as a story and its question, if it is a word problem.
fn story_and_question(text: &str) -> Option<(String, String)> {
    let body = text.trim();
    // drop the problem's number
    let body = body.split_once(". ").filter(|(n, _)| n.chars().all(|c| c.is_ascii_digit())).map(|x| x.1).unwrap_or(body).trim();
    let words = body.split_whitespace().filter(|w| w.chars().filter(|c| c.is_alphabetic()).count() >= 2).count();
    let drill = body.contains(" X ") || body.contains('×') || body.chars().any(|c| matches!(c, '+' | '=' | '÷'));
    if !body.contains('?') || words < 6 || drill || body.contains("Note") || garbled(body) {
        return None;
    }
    let body = body.replace(" ?", "?").replace(" ;", ";").replace(" ,", ",");
    let q_end = body.rfind('?')?;
    let before = &body[..q_end];
    let q_start = before.rfind(['.', ';']).map(|i| i + 1).unwrap_or(0);
    let (story, question) = (body[..q_start].trim().to_string(), body[q_start..=q_end].trim().to_string());
    let question = question.chars().next().map(|c| c.to_uppercase().collect::<String>() + &question[c.len_utf8()..]).unwrap_or(question);
    Some((story, question))
}

/// The answers printed at the end of a book, keyed by page: «Page 91. — 26. 40 mi. 27. 57.» → (91, 26) → 40.
/// An answer is kept when it starts with a whole number (a unit may follow) and has no garbled fraction.
pub fn page_answers(text: &str) -> std::collections::BTreeMap<(i64, i64), i64> {
    let mut out = std::collections::BTreeMap::new();
    let mut page = -1;
    let toks: Vec<&str> = text.split_whitespace().collect();
    let mut i = 0;
    let item = |t: &str| -> Option<(i64, String)> {
        // «26.» or «2.3625» (glued): the number of the item, and what is glued after its dot
        let (n, rest) = t.split_once('.')?;
        (!n.is_empty() && n.len() <= 3 && n.chars().all(|c| c.is_ascii_digit())).then(|| (n.parse().unwrap_or(0), rest.to_string()))
    };
    while i < toks.len() {
        let t = toks[i];
        if let Some(p) = t.strip_prefix("Page") {
            let digits: String = p.chars().chain(toks.get(i + 1).filter(|_| p.is_empty() || !p.chars().any(|c| c.is_ascii_digit())).map(|x| x.chars()).into_iter().flatten()).filter(|c| c.is_ascii_digit()).collect();
            if let Ok(v) = digits.parse() {
                page = v;
            }
            i += 1;
            continue;
        }
        if page >= 0 {
            if let Some((k, glued)) = item(t) {
                // the answer: the glued part, or the next token(s) up to the next item
                let mut ans = glued;
                let mut j = i + 1;
                while j < toks.len() && item(toks[j]).is_none() && !toks[j].starts_with("Page") {
                    if !ans.is_empty() {
                        ans.push(' ');
                    }
                    ans += toks[j];
                    j += 1;
                }
                let a = ans.trim().trim_start_matches(['$', '.', ' ']);
                let digits: String = a.chars().take_while(|c| c.is_ascii_digit() || *c == ',').collect();
                let next = a[digits.len()..].chars().next();
                if !digits.is_empty() && !garbled(&ans) && !matches!(next, Some('.' | '/')) || (next == Some('.') && a[digits.len() + 1..].trim_start().chars().next().is_none_or(|c| !c.is_ascii_digit())) {
                    if let Ok(v) = digits.replace(',', "").parse::<i64>() {
                        if !digits.is_empty() && !garbled(&ans) {
                            out.insert((page, k), v);
                        }
                    }
                }
                i = j;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// The word problems of a book whose answers are printed at its end, keyed by page: each numbered problem is found in
/// the sections whose printed pages hold that page (a page either side, the OCR's page numbers are approximate).
pub fn extract_paged(book: &str) -> Vec<(String, String, String, i64)> {
    let dir = corpus().join(book);
    let answers = page_answers(&std::fs::read_to_string(dir.join("answers.txt")).unwrap_or_default());
    // the sections with their pages
    let mut sections: Vec<(i64, i64, Vec<String>)> = Vec::new();
    for l in std::fs::read_to_string(dir.join("metadata.jsonl")).unwrap_or_default().lines() {
        let Ok(v) = serde_json::from_str::<serde_json::Value>(l) else { continue };
        let (Some(a), Some(b)) = (v["printed_pages"][0].as_i64(), v["printed_pages"][1].as_i64()) else { continue };
        let Some(path) = v["path"].as_str() else { continue };
        let text = std::fs::read_to_string(corpus().join(path)).unwrap_or_default();
        sections.push((a, b, problems(&text)));
    }
    let mut v = Vec::new();
    let mut taken = std::collections::BTreeSet::new();
    for ((page, k), ans) in answers {
        let found = sections.iter().enumerate().filter(|(_, s)| s.0 - 1 <= page && page <= s.1 + 1).flat_map(|(si, s)| s.2.iter().enumerate().map(move |(pi, c)| (si, pi, c))).find(|(_, _, c)| c.split_once(". ").is_some_and(|(n, _)| n.trim() == k.to_string()));
        if let Some((si, pi, c)) = found {
            if taken.insert((si, pi)) {
                if let Some((s, q)) = story_and_question(c) {
                    v.push((book.to_string(), s, q, ans));
                }
            }
        }
    }
    v
}

/// OpenStax Prealgebra (CC BY 4.0): the section exercises numbered through each chapter, the answers of the odd ones
/// in the Answer Key at the end, by chapter («Chapter 2» … «Section Exercises» … «Review Exercises»). An answer that
/// is a sentence («The total cost was $1,875.») gives its last whole number; one with a decimal or a fraction is left.
pub fn extract_openstax(book: &str) -> Vec<(String, String, String, i64)> {
    let dir = corpus().join(book);
    let key = std::fs::read_to_string(dir.join("answers.txt")).unwrap_or_default();
    let mut answers: std::collections::BTreeMap<(i64, i64), i64> = std::collections::BTreeMap::new();
    let (mut chapter, mut section) = (1i64, false);
    let toks: Vec<&str> = key.split_whitespace().collect();
    let item = |t: &str| -> Option<i64> { t.strip_suffix('.').filter(|n| !n.is_empty() && n.len() <= 3 && n.chars().all(|c| c.is_ascii_digit())).and_then(|n| n.parse().ok()) };
    let mut i = 0;
    while i < toks.len() {
        let t = toks[i];
        if t == "Chapter" {
            if let Some(c) = toks.get(i + 1).and_then(|x| x.parse().ok()) {
                chapter = c;
                section = false;
            }
        } else if t == "Section" && toks.get(i + 1) == Some(&"Exercises") {
            section = true;
        } else if (t == "Review" && toks.get(i + 1) == Some(&"Exercises")) || (t == "Practice" && toks.get(i + 1) == Some(&"Test")) {
            section = false;
        } else if let Some(k) = item(t) {
            // chapter 1 has no «Section Exercises» line: its exercises follow the «Try It» answers (numbered 1.1, 1.2 …)
            if section || (chapter == 1 && k >= 1) {
                let mut ans = String::new();
                let mut j = i + 1;
                while j < toks.len() && item(toks[j]).is_none() && !matches!(toks[j], "Chapter" | "Review" | "Practice" | "Section") {
                    ans.push(' ');
                    ans += toks[j];
                    j += 1;
                }
                let nums: Vec<&str> = ans.split(|c: char| !(c.is_ascii_digit() || c == ',' || c == '.')).map(|x| x.trim_matches(['.', ','])).filter(|x| !x.is_empty()).collect();
                if let Some(last) = nums.last() {
                    if !last.contains('.') && !garbled(&ans) && !ans.contains('=') {
                        if let Ok(v) = last.replace(',', "").parse::<i64>() {
                            answers.insert((chapter, k), v);
                        }
                    }
                }
                i = j;
                continue;
            }
        }
        i += 1;
    }
    let mut v = Vec::new();
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir).map(|r| r.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "txt")).collect()).unwrap_or_default();
    files.sort();
    for f in files {
        // «007-1-3-subtract-whole-numbers.txt»: chapter 1, a section of it
        let name = f.file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
        let parts: Vec<&str> = name.split('-').collect();
        let (Some(c), Some(_)) = (parts.get(1).and_then(|x| x.parse::<i64>().ok()), parts.get(2).and_then(|x| x.parse::<i64>().ok())) else { continue };
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        for chunk in problems(&text) {
            let Some((n, _)) = chunk.split_once(". ") else { continue };
            let Ok(k) = n.trim().parse::<i64>() else { continue };
            if let Some(&a) = answers.get(&(c, k)) {
                if let Some((s, q)) = story_and_question(&chunk) {
                    v.push((book.to_string(), s, q, a));
                }
            }
        }
    }
    v
}

/// OpenStax Prealgebra (1st edition, CC BY 4.0).
pub const OPENSTAX: &str = "marecek-2015-prealgebra";

/// The books whose answers are printed at their end, by page.
pub const PAGED: [&str; 1] = ["milne-1893-elements-arithmetic-primary-intermediate"];

/// The word problems of the books, in the books' order.
pub fn extract(books: &[&str]) -> Vec<(String, String, String, i64)> {
    let mut v = Vec::new();
    for b in books {
        let dir = corpus().join(b);
        let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&dir).map(|r| r.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "txt") && !p.ends_with("answers.txt")).collect()).unwrap_or_default();
        files.sort();
        for f in files {
            let text = std::fs::read_to_string(&f).unwrap_or_default();
            for c in problems(&text) {
                if let Some((s, q, a)) = word_problem(&c) {
                    v.push((b.to_string(), s, q, a));
                }
            }
        }
    }
    v
}

/// Whether an answer can come from the problem's numbers in one or two operations (with the dozen, the hour and the
/// hundred a book may leave untold): a check against the OCR's slips that tie a problem to another's answer.
pub fn reachable(text: &str, ans: i64) -> bool {
    let norm = crate::numbers::normalize(text).0;
    let mut ns: Vec<i64> = norm.split(|c: char| !(c.is_ascii_digit() || c == ',')).filter_map(|w| w.replace(',', "").parse().ok()).collect();
    ns.sort();
    ns.dedup();
    if ns.contains(&ans) && ns.len() == 1 {
        return true;
    }
    let ops = |a: i64, b: i64| -> Vec<i64> {
        let mut v = vec![a + b, a * b];
        if a >= b { v.push(a - b) } else { v.push(b - a) }
        if b != 0 && a % b == 0 { v.push(a / b) }
        if a != 0 && b % a == 0 { v.push(b / a) }
        v
    };
    let mut one: Vec<i64> = Vec::new();
    let with: Vec<i64> = ns.iter().copied().chain([12, 60, 100, 7, 24]).collect();
    for i in 0..ns.len() {
        for j in 0..with.len() {
            if ns[i] != with[j] || j >= ns.len() {
                one.extend(ops(ns[i], with[j]));
            }
        }
    }
    if one.contains(&ans) {
        return true;
    }
    one.iter().any(|&x| with.iter().any(|&y| ops(x, y).contains(&ans)))
}

pub fn main_args(args: &[String]) -> Result<()> {
    match args.get(1).map(String::as_str) {
        Some("extract") => {
            let out = args.get(2).cloned().unwrap_or_else(|| "primers.json".into());
            let books: Vec<&str> = if args.len() > 3 { args[3..].iter().map(String::as_str).collect() } else { IN_LINE.to_vec() };
            let mut v = extract(&books);
            if args.len() <= 3 {
                for b in PAGED {
                    v.extend(extract_paged(b));
                }
                v.extend(extract_openstax(OPENSTAX));
            }
            let books: Vec<&str> = books.into_iter().chain(if args.len() <= 3 { PAGED.iter().copied().chain([OPENSTAX]).collect() } else { vec![] }).collect();
            let all = v.len();
            v.retain(|(_, s, q, a)| reachable(&format!("{s} {q}"), *a));
            println!("{} of {all} have an answer one or two operations on their numbers reach", v.len());
            let json: Vec<serde_json::Value> = v.iter().enumerate().map(|(i, (b, s, q, a))| serde_json::json!({"ID": format!("{b}-{i}"), "Body": s, "Question": q, "Answer": *a as f64, "Type": "", "Source": b})).collect();
            std::fs::write(&out, serde_json::to_string_pretty(&json)?)?;
            println!("{} word problems with answers → {out}", v.len());
            for b in books {
                println!("  {b}: {}", v.iter().filter(|x| x.0 == b).count());
            }
        }
        _ => println!("usage: world primers extract <out.json> [book dir …]"),
    }
    Ok(())
}
