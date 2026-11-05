//! school-sents — the sentences of all school lessons, one per line, for the SLM and the annotator.
//!
//!   school-sents <school-dir> <out.tsv>
//!
//! Reads every `<course>/*.lesson` under <school-dir> and takes the values of the `explain:` and `fact:`
//! lines; a value is cut into sentences at `.`, `!`, `?` that are followed by a space and a capital,
//! an opening quote or «. Fragments under three words are dropped. Output (TSV):
//! course, lesson file, index in the lesson, sentence.
//!
//!   school-sents corpus <corpus-dir> <pool.tsv>
//!
//! The rest of an open corpus (corpus/open, release 0.4 layout) as a cycle pool (`id<TAB>sentence`): the books
//! (`books/*.txt`, Project Gutenberg header and footer cut, paragraphs joined; `book:<file>:<n>`, one document per
//! book, in reading order), the pupils' dictionary (`dictionary/dictionary.tsv`: meaning, sense in the lessons,
//! examples, note; `dict:<word>:<n>`, one document per word) and the questions of the maths scripts
//! (`math-scripts/scripts.jsonl`, train splits only; `math:<id>:<n>`). The contrast sets are tests and are not read.
//!
//!   school-sents books <books-dir> <pool.tsv> [--skip <file.txt>]… [--exclude <test.conllu>]…
//!
//! Only books (`*.txt`, as above). `--skip` leaves a book out (already done elsewhere); `--exclude` drops every
//! sentence whose text is a sentence of a held-out test set (a test sentence must never reach the teacher). A
//! «sentence» of more than 200 words (a table of contents, a list run together) is left out too.
//!
//!   school-sents usfm <dir> <pool.tsv> <prefix> [BOOK,…|-] [--hebrew-verses]
//!
//! Scripture in USFM (`*.usfm` or packed `*.usfm.zst`, one book a file; only the listed books, e.g. `GEN,EXO`, if given; `-` for all):
//! footnotes and cross references are dropped, character marks taken off with their words kept; each verse is cut
//! into sentences (a verse too short to cut is kept whole). `<prefix>:<BOOK>:<n>`, one document per book, n counting
//! its sentences in order; `<pool.tsv>.verses` maps each id to its verse (`GEN.1.1`), so translations line up verse
//! by verse. eBible's JPS 1917 numbers verses the English way and writes the Hebrew number where it differs,
//! «(3-2)», in front of a verse or inside it (the titles of the Psalms). The marks are always taken off before the
//! verse is cut, so the ids do not depend on the flag. With `--hebrew-verses` each sentence maps to the Hebrew verse
//! it begins in, so it lines up with the Hebrew text (the Leningrad Codex, as MACULA numbers it); a mark inside a
//! sentence does not cut it. Without it the English numbering stays. The marks of the Ten Commandments (EXO 20,
//! DEU 5) follow the upper accents and are not followed.
//!
//!   school-sents notes <dir> <pool.tsv> <prefix>
//!
//! Markdown notes (`*.md` but README.md) as a pool: headings, list marks, quote marks, emphasis, code marks, links
//! (their text kept) and table rows are taken off; each paragraph or list item is cut into sentences;
//! `<prefix>:<file stem>:<n>`, one document per note. Sentences with Hebrew or Aramaic script are counted.
//!
//!   school-sents addressed <pool.tsv> <prefix>=<in.tsv>…
//!
//! Texts by address (`address<TAB>text`, e.g. the Yerushalmi's English by `Berakhot.1.1.1`) as a pool: each text is
//! cut into sentences (a text too short to cut is kept whole), `<prefix>:<n>` with n counting the sentences of that
//! input in order (one document per input); `<pool.tsv>.verses` maps each id to its address.
//!
//!   school-sents scans <dir> <pool.tsv> <prefix>
//!
//! The English prose of scanned books (`*.txt`, Internet Archive OCR or plain text): running heads and page numbers
//! off, line-end hyphens joined, page breaks inside sentences closed; OCR noise (under 70% known English forms) and
//! «sentences» over 200 words left out; Hebrew or Aramaic script kept in its sentence and counted.
//! `<prefix>:<file stem>:<n>`; `<pool.tsv>.counts` per book.

use std::fs;
use std::path::Path;

/// `prose`: the corpus mode's extra rules (titles like «Mr.», numbered senses); the lessons keep the old cut,
/// since the parse database's ids count their sentences.
fn sentences(text: &str, prose: bool) -> Vec<String> {
    cut(text, prose, false)
}

/// `quotes`: a stop may be followed by closing quotes or brackets (`?” He said`, `.) The`) before the space, and
/// the next sentence may open with “ or ‘; they stay with the sentence they close (the `addressed` pools only: the
/// other pools keep their ids).
fn cut(text: &str, prose: bool, quotes: bool) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        cur.push(c);
        // «Mr. Rabbit», «Dr. Foster»: a title's stop is not a boundary
        let title = prose && c == '.' && ["Mr", "Mrs", "Dr", "St", "Mt"].iter().any(|a| cur.ends_with(&format!(" {a}.")) || cur == format!("{a}."));
        if quotes && !title && matches!(c, '.' | '!' | '?') {
            // the closing marks after the stop belong to this sentence
            let mut j = i + 1;
            while j < chars.len() && matches!(chars[j], '”' | '’' | '"' | '\'' | ')' | ']' | '»') {
                j += 1;
            }
            if j > i + 1 && j < chars.len() && chars[j] == ' ' {
                let next = chars[j + 1..].iter().find(|x| **x != ' ').copied();
                if matches!(next, Some(n) if n.is_uppercase() || matches!(n, '“' | '‘' | '"' | '«')) {
                    cur.extend(&chars[i + 1..j]);
                    out.push(std::mem::take(&mut cur).trim().to_string());
                    i = j;
                    continue;
                }
            }
        }
        if !title && matches!(c, '.' | '!' | '?') && i + 1 < chars.len() && chars[i + 1] == ' ' {
            // the next word starts with a capital, an opening quote or «
            let next = chars[i + 2..].iter().find(|x| **x != ' ').copied();
            // … or a numbered sense («… dinosaurs. 2) About can …»)
            let after: String = chars[i + 2..].iter().skip_while(|x| **x == ' ').take(3).collect();
            let numbered = prose && !quotes && after.chars().next().is_some_and(|d| d.is_ascii_digit()) && after.chars().skip_while(|d| d.is_ascii_digit()).next() == Some(')');
            let boundary = numbered || matches!(next, Some(n) if n.is_uppercase() || n == '«' || n == '"' || (quotes && matches!(n, '“' | '‘')));
            // a closing quote or bracket right after the stop belongs to this sentence
            if boundary || i + 1 == chars.len() {
                out.push(std::mem::take(&mut cur).trim().to_string());
            }
        }
        i += 1;
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out.into_iter().filter(|s| s.split_whitespace().count() >= 3).collect()
}

/// The text of a Project Gutenberg book between its START and END markers, as paragraphs (lines joined, `_` of
/// italics dropped).
fn book_paragraphs(text: &str) -> Vec<String> {
    let body = text.split_once("*** START").map_or(text, |x| x.1);
    let body = body.split_once('\n').map_or(body, |x| x.1);
    let body = body.split_once("*** END").map_or(body, |x| x.0);
    body.replace('\r', "").split("\n\n").map(|p| p.split_whitespace().collect::<Vec<_>>().join(" ").replace('_', "")).filter(|p| !p.is_empty()).collect()
}

/// A word as one id field: no colons, tabs or spaces.
fn key(w: &str) -> String {
    w.trim().chars().map(|c| if c == ':' || c.is_whitespace() { '_' } else { c }).collect()
}

/// The sentences of the books in a folder, in reading order: (book name, sentence).
fn book_sentences(dir: &Path, skip: &[String]) -> Vec<(String, String)> {
    let mut books: Vec<_> = fs::read_dir(dir).expect("books dir").flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "txt")).collect();
    books.sort();
    let mut v = Vec::new();
    for b in books {
        let file = b.file_name().unwrap().to_string_lossy().to_string();
        if skip.contains(&file) {
            continue;
        }
        let name = b.file_stem().unwrap().to_string_lossy().to_string();
        for p in book_paragraphs(&fs::read_to_string(&b).expect("book")) {
            for s in sentences(&p, true) {
                v.push((name.clone(), s));
            }
        }
    }
    v
}

/// The prose of a markdown note, one paragraph or list item per entry.
fn md_paragraphs(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let flush = |cur: &mut String, out: &mut Vec<String>| {
        let p = cur.split_whitespace().collect::<Vec<_>>().join(" ");
        if !p.is_empty() {
            out.push(p);
        }
        cur.clear();
    };
    for line in text.lines() {
        let l = line.trim();
        // a blank line, a rule or a table row ends the paragraph; tables are not prose
        if l.is_empty() || l.starts_with("---") || l.starts_with('|') {
            flush(&mut cur, &mut out);
            continue;
        }
        let mut l = l.trim_start_matches('#').trim_start_matches('>').trim_start().to_string();
        // a list item is a paragraph of its own
        let item = l.starts_with("- ") || l.starts_with("* ") || l.split_once(". ").is_some_and(|(n, _)| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()));
        if item {
            flush(&mut cur, &mut out);
            l = l.split_once(' ').map(|x| x.1.to_string()).unwrap_or_default();
        }
        if line.trim_start().starts_with('#') {
            // a heading stands alone
            flush(&mut cur, &mut out);
            cur.push_str(&l);
            flush(&mut cur, &mut out);
            continue;
        }
        cur.push(' ');
        cur.push_str(&l);
        if item {
            flush(&mut cur, &mut out);
        }
    }
    flush(&mut cur, &mut out);
    // links keep their text; emphasis and code marks go
    out.into_iter()
        .map(|p| {
            let mut s = String::new();
            let mut rest = p.as_str();
            while let Some(i) = rest.find("](") {
                let (a, b) = rest.split_at(i);
                s.push_str(a);
                rest = b.find(')').map(|j| &b[j + 1..]).unwrap_or("");
            }
            s.push_str(rest);
            s.replace(['[', '*', '`', '_'], "")
        })
        .collect()
}

fn notes(args: &[String]) {
    let (dir, out_path, prefix) = (Path::new(&args[0]), &args[1], &args[2]);
    let mut files: Vec<_> = fs::read_dir(dir).expect("notes dir").flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "md") && p.file_name().is_some_and(|n| n != "README.md")).collect();
    files.sort();
    let mut out = String::new();
    let mut total = 0usize;
    let mut heb = (0usize, 0usize);
    for f in &files {
        let stem = f.file_stem().unwrap().to_string_lossy().to_string();
        let mut k = 0usize;
        for p in md_paragraphs(&fs::read_to_string(f).expect("note")) {
            // a heading or a short line («Not a cheerleader.») is kept whole; the cut keeps fragments under three words out
            let ss = sentences(&p, true);
            let ss = if ss.is_empty() && p.split_whitespace().count() >= 2 { vec![p.clone()] } else { ss };
            for s in ss {
                let h = semitic_segments(&s);
                if h > 0 {
                    heb.0 += 1;
                    heb.1 += h;
                }
                out.push_str(&format!("{prefix}:{stem}:{k}\t{s}\n"));
                k += 1;
            }
        }
        total += k;
    }
    fs::write(out_path, out).expect("write pool");
    println!("pool → {out_path}: {total} sentences of {} notes; {} carry Hebrew or Aramaic script ({} passages)", files.len(), heb.0, heb.1);
}

/// The verses of one USFM book: (book code, chapter, verse, text); eBible's Hebrew verse marks are left in the text.
fn usfm_verses(text: &str) -> Vec<(String, u32, u32, String)> {
    // footnotes \f … \f* and cross references \x … \x* go whole
    let mut clean = String::new();
    let mut rest = text;
    while let Some(i) = rest.find('\\') {
        clean.push_str(&rest[..i]);
        let tail = &rest[i..];
        let end = |m: &str| tail.find(m).map(|j| j + m.len());
        if tail.starts_with("\\f ") || tail.starts_with("\\x ") || tail.starts_with("\\fe ") {
            let m = &tail[..tail.find(' ').unwrap()];
            rest = &tail[end(&format!("{m}*")).unwrap_or(tail.len())..];
        } else {
            clean.push('\\');
            rest = &tail[1..];
        }
    }
    clean.push_str(rest);
    let (mut book, mut c, mut out) = (String::new(), 0u32, Vec::<(String, u32, u32, String)>::new());
    for line in clean.lines() {
        let l = line.trim();
        let (mark, body) = l.split_once(' ').unwrap_or((l, ""));
        match mark {
            "\\id" => book = body.split_whitespace().next().unwrap_or("").to_string(),
            "\\c" => c = body.trim().parse().unwrap_or(c),
            "\\v" => {
                let (n, txt) = body.trim().split_once(' ').unwrap_or((body.trim(), ""));
                out.push((book.clone(), c, n.parse().unwrap_or(0), txt.to_string()));
            }
            // headings and titles are not verses; a paragraph or poetry mark goes on with the verse it is in
            m if m.starts_with("\\h") || m.starts_with("\\toc") || m.starts_with("\\mt") || m.starts_with("\\s") || m.starts_with("\\ms") || m.starts_with("\\r") || m.starts_with("\\d") => {}
            _ => {
                if let Some(v) = out.last_mut() {
                    let more = if mark.starts_with('\\') { body } else { l };
                    v.3.push(' ');
                    v.3.push_str(more);
                }
            }
        }
    }
    // character marks (\\w word|lemma\\w*, \\add, \\nd …) off, their words kept
    for v in &mut out {
        v.3 = strip_char_marks(&v.3);
    }
    out
}

/// Character marks off a verse's text, their words and the punctuation around them kept: an opening mark (`\w `,
/// `\+w `, `\add `, `\nd ` …) goes with the one space after it, a closing mark (`\w*`) goes alone, and a word's
/// attributes (`|strong="H7225"`) go up to the closing mark. «\w earth|strong="H0776"\w*.» is «earth.», and
/// «“\w Let|…\w*» is «“Let». Whitespace runs become one space.
fn strip_char_marks(text: &str) -> String {
    let mut s = String::with_capacity(text.len());
    let mut it = text.chars().peekable();
    while let Some(c) = it.next() {
        match c {
            '|' => {
                while it.peek().is_some_and(|&x| x != '\\') {
                    it.next();
                }
            }
            '\\' => {
                let mut closing = false;
                while let Some(&x) = it.peek() {
                    if x.is_ascii_alphanumeric() || x == '+' {
                        it.next();
                    } else {
                        if x == '*' {
                            it.next();
                            closing = true;
                        }
                        break;
                    }
                }
                if !closing && it.peek() == Some(&' ') {
                    it.next();
                }
            }
            c => s.push(c),
        }
    }
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A Hebrew verse number as eBible's JPS writes it: «(3-2)» → (3, 2).
fn hebrew_mark(w: &str) -> Option<(u32, u32)> {
    let (c, v) = w.strip_prefix('(')?.strip_suffix(')')?.split_once('-')?;
    Some((c.parse().ok()?, v.parse().ok()?))
}

/// The sentences of one verse with the verse each maps to. eBible's JPS writes the Hebrew number where it differs:
/// «(20-13) Thou shalt not …», and inside a verse where a Hebrew verse begins («A Psalm of David. (3-2) LORD, how
/// many …»). The marks are always taken off and the sentences cut from the text left, so the ids are the same with
/// and without `hebrew` (the Torah's pool of 17.10 keeps its ids). With `hebrew` each sentence goes to the Hebrew
/// verse it begins in; a mark inside a sentence («after the plague, (26-1) that the LORD spoke») does not cut it.
/// In the Ten Commandments (EXO 20, DEU 5) eBible numbers by the upper accents, which join four commandments into one
/// verse; the Leningrad Codex numbers them as the English do, so there the marks are not followed.
fn verse_sentences(b: &str, c: u32, v: u32, text: &str, hebrew: bool) -> Vec<(u32, u32, String)> {
    let follow = hebrew && !matches!((b, c), ("EXO", 20) | ("DEU", 5));
    // the text without marks, and where each mark stood in it
    let (mut clean, mut marks) = (String::new(), Vec::new());
    for w in text.split_whitespace() {
        match hebrew_mark(w) {
            Some(h) => marks.push((clean.len(), h)),
            None => {
                if !clean.is_empty() {
                    clean.push(' ');
                }
                clean.push_str(w);
            }
        }
    }
    let ss = sentences(&clean, true);
    let ss = if ss.is_empty() && !clean.is_empty() { vec![clean.clone()] } else { ss };
    let mut at = 0;
    ss.into_iter()
        .map(|s| {
            let start = clean[at..].find(s.as_str()).map_or(at, |i| at + i);
            at = start + s.len();
            // a mark stands before the space in front of its word: it belongs to a sentence starting just after it
            let (c, v) = marks.iter().rev().find(|m| follow && m.0 <= start + 1).map_or((c, v), |m| m.1);
            (c, v, s)
        })
        .collect()
}

/// A USFM book as text: plain, or packed as seekable zstd (`.usfm.zst`, whole-line frames and a skippable seek table;
/// the stream decoder gives the original back byte for byte, as `zstdcat` does).
fn read_usfm(p: &Path) -> String {
    if p.extension().is_some_and(|x| x == "zst") {
        let f = fs::File::open(p).expect("usfm.zst");
        String::from_utf8(zstd::stream::decode_all(f).expect("zstd")).expect("utf-8")
    } else {
        fs::read_to_string(p).expect("usfm")
    }
}

fn usfm(args: &[String]) {
    let (dir, out_path, prefix) = (Path::new(&args[0]), &args[1], &args[2]);
    let hebrew = args.iter().any(|a| a == "--hebrew-verses");
    let only: Vec<String> = args.get(3).filter(|x| !x.starts_with("--") && *x != "-").map(|x| x.split(',').map(String::from).collect()).unwrap_or_default();
    let mut files: Vec<_> = fs::read_dir(dir).expect("usfm dir").flatten().map(|e| e.path()).filter(|p| p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.ends_with(".usfm") || n.ends_with(".usfm.zst"))).collect();
    // by name, a packed book (`.usfm.zst`) where its plain file would stand
    files.sort_by_key(|p| p.file_name().and_then(|n| n.to_str()).map(|n| n.trim_end_matches(".zst").to_string()));
    let (mut out, mut map) = (String::new(), String::new());
    let mut count = std::collections::BTreeMap::<String, (usize, usize)>::new();
    for f in files {
        let verses = usfm_verses(&read_usfm(&f));
        let Some(book) = verses.first().map(|v| v.0.clone()) else { continue };
        if !only.is_empty() && !only.contains(&book) {
            continue;
        }
        let mut n = 0usize;
        for (b, c, v, text) in &verses {
            for (c, v, s) in verse_sentences(b, *c, *v, text, hebrew) {
                out.push_str(&format!("{prefix}:{b}:{n}\t{s}\n"));
                map.push_str(&format!("{prefix}:{b}:{n}\t{b}.{c}.{v}\n"));
                n += 1;
            }
        }
        count.insert(book, (verses.len(), n));
    }
    fs::write(out_path, out).expect("write pool");
    fs::write(format!("{out_path}.verses"), map).expect("write verses");
    println!("pool → {out_path}: (verses, sentences) by book {count:?}");
}

/// Lower-case letters and digits, one space between words.
fn norm(s: &str) -> String {
    s.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).collect::<Vec<_>>().join(" ")
}

fn books(args: &[String]) {
    let (dir, out_path) = (Path::new(&args[0]), &args[1]);
    let (mut skip, mut exclude) = (Vec::new(), std::collections::BTreeSet::new());
    let mut i = 2;
    while i + 1 < args.len() {
        match args[i].as_str() {
            "--skip" => skip.push(args[i + 1].clone()),
            "--exclude" => exclude.extend(fs::read_to_string(&args[i + 1]).expect("test set").lines().filter_map(|l| l.strip_prefix("# text = ")).map(norm).filter(|s| s.len() >= 12)),
            o => panic!("unknown option {o}"),
        }
        i += 2;
    }
    let mut out = String::new();
    let (mut kept, mut dropped, mut long) = (0usize, 0usize, 0usize);
    let mut k = std::collections::BTreeMap::<String, usize>::new();
    for (name, s) in book_sentences(dir, &skip) {
        // the index counts every sentence of the book, so ids stay put whatever is excluded
        let n = k.entry(name.clone()).or_default();
        let id = format!("book:{name}:{n}");
        *n += 1;
        // compared as plain letters and digits (quotes, case and spacing differ between editions), and either way
        // round: a test sentence glued to its neighbour in our cut is still a test sentence
        if s.split_whitespace().count() > 200 {
            long += 1;
            continue;
        }
        let ns = norm(&s);
        if exclude.iter().any(|x| ns.contains(x.as_str()) || (ns.len() >= 12 && x.contains(ns.as_str()))) {
            dropped += 1;
            continue;
        }
        out.push_str(&format!("{id}\t{s}\n"));
        kept += 1;
    }
    fs::write(out_path, out).expect("write pool");
    println!("pool → {out_path}: {kept} sentences of {} books; {dropped} test sentences and {long} over 200 words left out", k.len());
}

fn corpus(root: &Path, out_path: &str) {
    let mut out = String::new();
    let mut count = std::collections::BTreeMap::<&str, usize>::new();
    let mut books: Vec<_> = fs::read_dir(root.join("books")).expect("books dir").flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "txt")).collect();
    books.sort();
    for b in books {
        let name = b.file_stem().unwrap().to_string_lossy().to_string();
        let mut k = 0usize;
        for p in book_paragraphs(&fs::read_to_string(&b).expect("book")) {
            for s in sentences(&p, true) {
                out.push_str(&format!("book:{name}:{k}\t{s}\n"));
                k += 1;
            }
        }
        *count.entry("books").or_default() += k;
    }
    let dict = fs::read_to_string(root.join("dictionary/dictionary.tsv")).expect("dictionary");
    for line in dict.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = line.split('\t').collect();
        let Some(word) = f.first() else { continue };
        let mut k = 0usize;
        for v in f.iter().skip(2) {
            for s in sentences(v, true) {
                out.push_str(&format!("dict:{}:{k}\t{s}\n", key(word)));
                k += 1;
            }
        }
        *count.entry("dictionary").or_default() += k;
    }
    for line in fs::read_to_string(root.join("math-scripts/scripts.jsonl")).expect("math scripts").lines() {
        let field = |name: &str| line.split_once(&format!("\"{name}\": \"")).and_then(|x| x.1.split_once('"')).map(|x| x.0.to_string()).unwrap_or_default();
        let (id, set, q) = (field("id"), field("set"), field("question"));
        // only the training splits; a test problem never goes to the teacher
        if !set.ends_with("-train") {
            continue;
        }
        for (k, s) in sentences(&q.replace("\\\"", "\""), true).into_iter().enumerate() {
            out.push_str(&format!("math:{}:{k}\t{s}\n", key(&id)));
            *count.entry("math").or_default() += 1;
        }
    }
    fs::write(out_path, out).expect("write pool");
    println!("pool → {out_path}: {count:?}");
}

fn addressed(args: &[String]) {
    let out_path = &args[0];
    let (mut out, mut map) = (String::new(), String::new());
    for spec in &args[1..] {
        let (prefix, input) = spec.split_once('=').expect("<prefix>=<in.tsv>");
        let (mut n, mut texts) = (0usize, 0usize);
        for line in fs::read_to_string(input).expect("addressed input").lines() {
            let Some((addr, text)) = line.split_once('\t') else { continue };
            texts += 1;
            let ss = cut(text, true, true);
            let ss = if ss.is_empty() && !text.trim().is_empty() { vec![text.trim().to_string()] } else { ss };
            for s in ss {
                out.push_str(&format!("{prefix}:{n}\t{s}\n"));
                map.push_str(&format!("{prefix}:{n}\t{addr}\n"));
                n += 1;
            }
        }
        println!("{prefix}: {texts} addresses → {n} sentences");
    }
    fs::write(out_path, out).expect("write pool");
    fs::write(format!("{out_path}.verses"), map).expect("write verses");
}

/// A letter of the Hebrew square script (Hebrew and Aramaic, points and cantillation included) or of Syriac.
fn semitic(c: char) -> bool {
    matches!(c, '\u{0590}'..='\u{05FF}' | '\u{FB1D}'..='\u{FB4F}' | '\u{0700}'..='\u{074F}')
}

/// The Hebrew or Aramaic passages of a sentence: maximal runs of words written in the square script (or Syriac),
/// punctuation between them not breaking a run.
fn semitic_segments(s: &str) -> usize {
    let (mut n, mut inside) = (0, false);
    for w in s.split_whitespace() {
        if w.chars().any(semitic) {
            if !inside {
                n += 1;
            }
            inside = true;
        } else if w.chars().any(char::is_alphanumeric) {
            inside = false;
        }
    }
    n
}

/// The pages of a scanned book's OCR text (Internet Archive `_djvu.txt`, or any plain text) as running paragraphs:
/// running heads and page numbers off (a short line without lower-case letters, or one mostly in capitals with a
/// number at an end), a word hyphenated at a line end joined again, and a page break inside a sentence closed up
/// (a block that does not end in a stop goes on into the next when that one starts in lower case).
fn scan_paragraphs(text: &str) -> Vec<String> {
    let head = |l: &str| {
        let words = l.split_whitespace().count();
        let lower = l.chars().filter(|c| c.is_lowercase()).count();
        let upper = l.chars().filter(|c| c.is_uppercase()).count();
        let num_end = l.split_whitespace().next().is_some_and(|w| w.chars().all(|c| c.is_ascii_digit()))
            || l.split_whitespace().last().is_some_and(|w| w.chars().all(|c| c.is_ascii_digit()));
        (words <= 4 && lower == 0) || (words <= 10 && num_end && upper >= 2 * lower.max(1))
    };
    // blocks of lines between blank lines, heads dropped
    let mut blocks: Vec<Vec<String>> = vec![Vec::new()];
    for line in text.replace('\r', "").lines() {
        let l = line.trim();
        if l.is_empty() {
            if !blocks.last().unwrap().is_empty() {
                blocks.push(Vec::new());
            }
        } else if !head(l) {
            blocks.last_mut().unwrap().push(l.to_string());
        }
    }
    let join = |lines: &[String]| {
        let mut s = String::new();
        for l in lines {
            // «gram-» + «mar» → «grammar»; «well-» + «Known» stays apart
            if s.ends_with('-') && s.chars().rev().nth(1).is_some_and(char::is_alphabetic) && l.chars().next().is_some_and(char::is_lowercase) {
                s.pop();
            } else if !s.is_empty() {
                s.push(' ');
            }
            s.push_str(l);
        }
        s.split_whitespace().collect::<Vec<_>>().join(" ")
    };
    let mut out: Vec<String> = Vec::new();
    let mut open = false;
    for b in blocks.iter().filter(|b| !b.is_empty()) {
        let p = join(b);
        let starts_lower = p.chars().next().is_some_and(char::is_lowercase);
        match out.last_mut() {
            Some(last) if open && starts_lower => {
                if last.ends_with('-') && last.chars().rev().nth(1).is_some_and(char::is_alphabetic) {
                    last.pop();
                } else {
                    last.push(' ');
                }
                last.push_str(&p);
            }
            _ => out.push(p),
        }
        let last = out.last().unwrap();
        open = !last.ends_with(['.', '!', '?', ':', '"', '”', ')']);
    }
    out
}

/// The word forms an OCR'd sentence is checked against (AGID + EWT forms, `en/data/forms.tsv`), lower case.
fn lexicon() -> std::collections::BTreeSet<String> {
    let p = concat!(env!("CARGO_MANIFEST_DIR"), "/../../en/data/forms.tsv");
    fs::read_to_string(p).expect("en/data/forms.tsv").lines().filter_map(|l| l.split('\t').next()).map(str::to_lowercase).collect()
}

/// Whether a sentence of OCR text reads as English: at least three known lower-case words, and at least 70% of its
/// words known forms or capitalised (names, abbreviations of books). OCR'd Hebrew or tables («Itol rfyfo», «yaqtuliJwa»)
/// fail it; a sentence with Hebrew script in it is judged by its other words.
fn english_enough(s: &str, lex: &std::collections::BTreeSet<String>) -> bool {
    let words: Vec<String> = s
        .split_whitespace()
        .filter(|w| !w.chars().any(semitic))
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|w| !w.is_empty() && w.chars().all(char::is_alphabetic))
        .collect();
    if words.len() < 3 {
        return false;
    }
    // a single letter is a known word only as «a» or «I» (OCR leaves «m. c. s.» of tables and references)
    let form = |w: &String| (w.chars().count() > 1 || w == "a" || w == "I") && lex.contains(&w.to_lowercase());
    let plain = words.iter().filter(|w| w.chars().all(char::is_lowercase) && form(w)).count();
    let known = words.iter().filter(|w| form(w) || (w.chars().count() > 1 && w.chars().next().is_some_and(char::is_uppercase) && w.chars().skip(1).all(char::is_lowercase))).count();
    // and at least three ordinary lower-case words: «Verg. ii. i Si.», «Demosthenes and Aeschines.» are references
    plain >= 3 && known * 10 >= words.len() * 7
}

/// `school-sents scans <dir> <pool.tsv> <prefix>`: the English prose of scanned books (`*.txt`, one book a file)
/// as a pool, `<prefix>:<file stem>:<n>`, n counting every sentence of the book (kept or not), one document per
/// book. Left out: «sentences» of more than 200 words, and OCR noise (`english_enough`). Sentences with Hebrew or
/// Aramaic script stay and are counted. `<pool.tsv>.counts` has a row per book.
fn scans(args: &[String]) {
    let (dir, out_path, prefix) = (Path::new(&args[0]), &args[1], &args[2]);
    let lex = lexicon();
    let mut files: Vec<_> = fs::read_dir(dir).expect("scans dir").flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "txt")).collect();
    files.sort();
    let mut out = String::new();
    let mut counts = String::from("book\tsentences\tkept\tnoise\tlong\twith_hebrew_script\thebrew_segments\n");
    let mut tot = [0usize; 6];
    for f in &files {
        let stem = f.file_stem().unwrap().to_string_lossy().to_string();
        let mut c = [0usize; 6];
        for p in scan_paragraphs(&fs::read_to_string(f).expect("book")) {
            for s in sentences(&p, true) {
                let id = format!("{prefix}:{stem}:{}", c[0]);
                c[0] += 1;
                if s.split_whitespace().count() > 200 {
                    c[3] += 1;
                } else if !english_enough(&s, &lex) {
                    c[2] += 1;
                } else {
                    c[1] += 1;
                    let h = semitic_segments(&s);
                    if h > 0 {
                        c[4] += 1;
                        c[5] += h;
                    }
                    out.push_str(&format!("{id}\t{s}\n"));
                }
            }
        }
        counts.push_str(&format!("{stem}\t{}\n", c.iter().map(|x| x.to_string()).collect::<Vec<_>>().join("\t")));
        for i in 0..6 {
            tot[i] += c[i];
        }
    }
    fs::write(out_path, out).expect("write pool");
    fs::write(format!("{out_path}.counts"), counts).expect("write counts");
    println!(
        "pool → {out_path}: {} sentences of {} books kept; {} OCR noise and {} over 200 words left out; {} kept sentences carry Hebrew or Aramaic script ({} passages)",
        tot[1], files.len(), tot[2], tot[3], tot[4], tot[5]
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 4 && args[1] == "addressed" {
        return addressed(&args[2..]);
    }
    if args.len() == 5 && args[1] == "scans" {
        return scans(&args[2..]);
    }
    if (5..=7).contains(&args.len()) && args[1] == "usfm" {
        return usfm(&args[2..]);
    }
    if args.len() == 5 && args[1] == "notes" {
        return notes(&args[2..]);
    }
    if args.len() >= 4 && args[1] == "books" {
        return books(&args[2..]);
    }
    if args.len() == 4 && args[1] == "corpus" {
        return corpus(Path::new(&args[2]), &args[3]);
    }
    if args.len() != 3 {
        eprintln!("usage: school-sents <school-dir> <out.tsv>");
        std::process::exit(2);
    }
    let root = Path::new(&args[1]);
    let mut courses: Vec<_> = fs::read_dir(root).expect("school dir").flatten().filter(|e| e.path().is_dir()).collect();
    courses.sort_by_key(|e| e.file_name());
    let mut out = String::new();
    let mut total = 0usize;
    for course in courses {
        let cname = course.file_name().to_string_lossy().to_string();
        let mut files: Vec<_> = fs::read_dir(course.path()).expect("course dir").flatten()
            .filter(|e| e.path().extension().is_some_and(|x| x == "lesson"))
            .collect();
        files.sort_by_key(|e| e.file_name());
        for f in files {
            let fname = f.file_name().to_string_lossy().to_string();
            let body = fs::read_to_string(f.path()).unwrap_or_default();
            let mut k = 0usize;
            for line in body.lines() {
                let value = match line.split_once(':') {
                    Some(("explain", v)) | Some(("fact", v)) => v.trim(),
                    _ => continue,
                };
                for s in sentences(value, false) {
                    out.push_str(&format!("{cname}\t{fname}\t{k}\t{s}\n"));
                    k += 1;
                    total += 1;
                }
            }
        }
    }
    fs::write(&args[2], out).expect("write output");
    println!("sentences: {total} → {}", args[2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    const PSALM: &str = "\\id PSA\n\\c 3\n\\p\n\\v 1 (3-1) A Psalm of David. (3-2) LORD, how many are mine adversaries!\n\\v 2 (3-3) Many there are.\n\\c 4\n\\v 1 Answer me when I call.\n";

    fn cut_usfm(usfm: &str, hebrew: bool) -> Vec<(u32, u32, String)> {
        usfm_verses(usfm).iter().flat_map(|(b, c, v, t)| verse_sentences(b, *c, *v, t, hebrew)).collect()
    }

    #[test]
    fn scans_close_pages_and_hyphens() {
        let t = "88 HEBREW SYNTAX § 62.\n\nThe cohortative is used to ex-\npress the will of the\n\nspeaker. A new sentence.\n\nE 2\n";
        let p = scan_paragraphs(t);
        assert_eq!(p, vec!["The cohortative is used to express the will of the speaker. A new sentence.".to_string()]);
        // negative control: a block ending in a stop does not swallow the next one
        assert_eq!(scan_paragraphs("One ends here.\n\nanother starts.").len(), 2);
    }

    #[test]
    fn english_filter_and_hebrew_count() {
        let lex: std::collections::BTreeSet<String> = ["the", "is", "used", "to", "of", "speaker", "will", "express", "and", "as", "in"].iter().map(|s| s.to_string()).collect();
        assert!(english_enough("The cohortative is used to express the will of the speaker.", &lex));
        // negative controls: OCR'd Hebrew, a reference
        assert!(!english_enough("Itol rfyfo yaqtuliJwa n^v ttt:i.", &lex));
        assert!(!english_enough("Verg. ii. i Si.", &lex));
        assert_eq!(semitic_segments("as יָדַ֫עְתִּ ψ 140, and אָמַ֫רְתִּ is"), 2);
        assert_eq!(semitic_segments("no Hebrew here"), 0);
    }


    #[test]
    fn quoted_stops() {
        let t = "They said: But did we not state: “He starts there?” He said: Since you answered. “Come,” he said. It is so (v. 7). And more here.";
        assert_eq!(cut(t, true, true), ["They said: But did we not state: “He starts there?”", "He said: Since you answered.", "“Come,” he said.", "It is so (v. 7).", "And more here."]);
        // negative control: the old cut (other pools' ids) does not split after a closing quote
        assert_eq!(sentences(t, true).len(), 3);
    }

    #[test]
    fn hebrew_verses() {
        let s = |x: &str| x.to_string();
        assert_eq!(cut_usfm(PSALM, true), [(3, 1, s("A Psalm of David.")), (3, 2, s("LORD, how many are mine adversaries!")), (3, 3, s("Many there are.")), (4, 1, s("Answer me when I call."))]);
        // without the flag the same sentences (the same ids), on the English numbering
        assert_eq!(cut_usfm(PSALM, false), [(3, 1, s("A Psalm of David.")), (3, 1, s("LORD, how many are mine adversaries!")), (3, 2, s("Many there are.")), (4, 1, s("Answer me when I call."))]);
        // a mark inside a sentence does not cut it: the sentence stays whole, in the verse it begins in
        let num = "\\id NUM\n\\c 25\n\\v 19 (25-19) And it came to pass after the plague, (26-1) that the LORD spoke unto Moses, saying:\n";
        assert_eq!(cut_usfm(num, true), [(25, 19, s("And it came to pass after the plague, that the LORD spoke unto Moses, saying:"))]);
        // negative control: the Ten Commandments keep the English numbering
        let exo = "\\id EXO\n\\c 20\n\\v 14 (20-13) Thou shalt not commit adultery.\n";
        assert_eq!(cut_usfm(exo, true), [(20, 14, s("Thou shalt not commit adultery."))]);
        assert_eq!(hebrew_mark("(20-13)"), Some((20, 13)));
        assert_eq!(hebrew_mark("(see)"), None);
    }
}
#[cfg(test)]
mod wave33 {
    use super::*;
    #[test]
    fn normalization_and_verse_identity() {
        assert!(sentences("Two words.", true).is_empty());
        assert_eq!(
            sentences("Mr. Rabbit came home. He ate the meal.", true).len(),
            2
        );
        assert_eq!(key(" a:b\tc "), "a_b_c");
        assert_eq!(norm("Élan,  A!"), "élan a");
        assert_eq!(strip_char_marks(r"\w earth|strong=X\w*."), "earth.");
        let t = "(3-2) One little sentence. (3-3) Another little sentence.";
        let a = verse_sentences("PSA", 3, 1, t, false);
        let b = verse_sentences("PSA", 3, 1, t, true);
        assert_eq!(
            a.iter().map(|x| &x.2).collect::<Vec<_>>(),
            b.iter().map(|x| &x.2).collect::<Vec<_>>()
        );
        assert_eq!((a[0].1, b[0].1, b[1].1), (1, 2, 3));
        assert_eq!(book_paragraphs("first\nsecond line"), vec!["second line"]);
    }
}
