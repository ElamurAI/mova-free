//! school-sents — the sentences of all school lessons, one per line, for the MMM and the annotator.
//!
//!   school-sents <school-dir> <out.tsv>
//!
//! Reads every `<course>/*.lesson` under <school-dir> and takes the values of the `explain:` and `fact:`
//! lines; a value is cut into sentences at `.`, `!`, `?` that are followed by a space and a capital,
//! an opening quote or «. Fragments under three words are dropped. Output (TSV):
//! course, lesson file, index in the lesson, sentence.

use std::fs;
use std::path::Path;

fn sentences(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        cur.push(c);
        if matches!(c, '.' | '!' | '?') && i + 1 < chars.len() && chars[i + 1] == ' ' {
            // the next word starts with a capital, an opening quote or «
            let next = chars[i + 2..].iter().find(|x| **x != ' ').copied();
            let boundary = matches!(next, Some(n) if n.is_uppercase() || n == '«' || n == '"');
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

fn main() {
    let args: Vec<String> = std::env::args().collect();
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
                for s in sentences(value) {
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
