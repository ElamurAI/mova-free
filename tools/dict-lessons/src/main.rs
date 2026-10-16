//! dict-lessons — one lesson per dictionary word, in two steps.
//!
//!   dict-lessons requests <silver.conllu> <dictionary.tsv> <example.lesson> <n> <out-dir>
//!       writes out-dir/batch-requests.jsonl: one Batches request per word (the first n words that have
//!       school sentences with trees); the sentences come from the silver corpus by `sent_id`.
//!   dict-lessons post <out-dir>
//!       reads out-dir/batch-results.jsonl and writes out-dir/lessons/NNN-<word>.lesson. Lesson number =
//!       position of the word (1-based); the options of every question are shuffled with a seed from the
//!       lesson number; the right option keeps its leading `*`, and each question must have exactly one.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use serde_json::{json, Value};

const RULES: &str = "You write one lesson for a primary school course (ages 9-10, a prodigy), from one dictionary word.
Output ONLY the lesson, in the exact line format of the example below: lines `course:`, `lesson:`, `title:`, `level:`, `stars:`, then `explain:` (2-4 lines), `fact:` (3-5 lines), `exercise:` (3 lines, each with the question then ` | ` and 3-4 options; put a leading * (asterisk, no space) before the right option; the right option may be in any position), `test:` (2 lines, same form), `exam:` (1 line, same form). No markdown, no code fences, no commentary.
Use the word in plain English a child understands. Use the given sentences as the context examples, quoting them with «» where you use them. Never invent facts that contradict the dictionary entry.";

const QUESTION_KEYS: [&str; 3] = ["exercise:", "test:", "exam:"];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("requests") if args.len() == 7 => requests(&args[2..]),
        Some("post") if args.len() == 3 => post(Path::new(&args[2])),
        Some("subset") if args.len() == 5 => subset(&args[2], &args[3], &args[4]),
        Some("requests-empty") if args.len() == 6 => requests_empty(&args[2..]),
        _ => Err("usage: dict-lessons requests <silver.conllu> <dictionary.tsv> <example.lesson> <n> <out-dir> | post <out-dir> | subset <requests.jsonl> <ids.txt> <out-dir> | requests-empty <dictionary.tsv> <example.lesson> <offset> <out-dir>".into()),
    };
    if let Err(e) = code {
        eprintln!("dict-lessons: {e}");
        std::process::exit(1);
    }
}

fn requests(a: &[String]) -> Result<(), String> {
    let texts = sent_texts(&a[0])?;
    let example: Vec<String> = fs::read_to_string(&a[2]).map_err(|e| e.to_string())?.lines().take(30).map(String::from).collect();
    let n: usize = a[3].parse().map_err(|_| "n must be a number")?;
    let dict = fs::read_to_string(&a[1]).map_err(|e| e.to_string())?;
    let out_dir = Path::new(&a[4]);
    fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let mut lines = Vec::new();
    for row in dict.lines().filter(|l| !l.starts_with('#')) {
        let cols: Vec<&str> = row.split('\t').collect();
        if cols.len() < 7 || cols[6].is_empty() {
            continue;
        }
        let (word, pos, meaning, sense) = (cols[0], cols[1], cols[2], cols[3]);
        let sents: Vec<&str> = cols[6].split(',').take(5).filter_map(|s| texts.get(s).map(String::as_str)).collect();
        let mut body = format!("{RULES}\n\nEXAMPLE LESSON:\n{}\n\nWORD: {word}\nPART OF SPEECH: {pos}\nMEANING FOR A PUPIL: {meaning}\nSENSE IN THE LESSONS: {sense}\nEXAMPLE SENTENCES:\n", example.join("\n"));
        for s in sents {
            body.push_str(&format!("- {s}\n"));
        }
        body.push_str("\nWrite the lesson now.");
        let req = json!({
            "custom_id": lines.len().to_string(),
            "params": {
                "model": "claude-haiku-5-5",
                "max_tokens": 4000,
                "messages": [{"role": "user", "content": [{"type": "text", "text": body}]}],
                "output_config": {"effort": "medium"}
            }
        });
        lines.push(req.to_string());
        if lines.len() == n {
            break;
        }
    }
    fs::write(out_dir.join("batch-requests.jsonl"), lines.join("\n") + "\n").map_err(|e| e.to_string())?;
    println!("requests: {} → {}", lines.len(), out_dir.display());
    Ok(())
}

/// Requests for dictionary words that have no school sentence: the entry alone is the context.
/// custom_id = offset + position among those words, so lesson numbers continue after the others.
fn requests_empty(a: &[String]) -> Result<(), String> {
    let example: Vec<String> = fs::read_to_string(&a[1]).map_err(|e| e.to_string())?.lines().take(30).map(String::from).collect();
    let offset: usize = a[2].parse().map_err(|_| "offset must be a number")?;
    let out_dir = Path::new(&a[3]);
    fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let dict = fs::read_to_string(&a[0]).map_err(|e| e.to_string())?;
    let mut lines = Vec::new();
    for row in dict.lines().filter(|l| !l.starts_with('#')) {
        let cols: Vec<&str> = row.split('\t').collect();
        if cols.len() < 4 || (cols.len() >= 7 && !cols[6].is_empty()) {
            continue;
        }
        let (word, pos, meaning, sense) = (cols[0], cols[1], cols[2], cols[3]);
        let body = format!("{RULES}\n\nEXAMPLE LESSON:\n{}\n\nWORD: {word}\nPART OF SPEECH: {pos}\nMEANING FOR A PUPIL: {meaning}\nSENSE IN THE LESSONS: {sense}\nEXAMPLE SENTENCES:\n(none: write the examples yourself, simple and clearly about this word)\n\nWrite the lesson now.", example.join("\n"));
        let req = json!({
            "custom_id": (offset + lines.len()).to_string(),
            "params": {
                "model": "claude-haiku-5-5",
                "max_tokens": 4000,
                "messages": [{"role": "user", "content": [{"type": "text", "text": body}]}],
                "output_config": {"effort": "medium"}
            }
        });
        lines.push(req.to_string());
    }
    fs::write(out_dir.join("batch-requests.jsonl"), lines.join("\n") + "\n").map_err(|e| e.to_string())?;
    println!("requests without school sentences: {} (custom_id from {offset}) → {}", lines.len(), out_dir.display());
    Ok(())
}

/// Keeps the requests whose lesson (custom_id + 1) is listed in ids.txt (lines like `025-absorb`).
fn subset(requests: &str, ids: &str, out_dir: &str) -> Result<(), String> {
    let wanted: std::collections::HashSet<String> = fs::read_to_string(ids)
        .map_err(|e| e.to_string())?
        .lines()
        .filter_map(|l| l.split('-').next()?.parse::<usize>().ok())
        .map(|n| (n - 1).to_string())
        .collect();
    let text = fs::read_to_string(requests).map_err(|e| e.to_string())?;
    let kept: Vec<&str> = text
        .lines()
        .filter(|l| serde_json::from_str::<Value>(l).ok().and_then(|v| v["custom_id"].as_str().map(|c| wanted.contains(c))).unwrap_or(false))
        .collect();
    fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    fs::write(Path::new(out_dir).join("batch-requests.jsonl"), kept.join("\n") + "\n").map_err(|e| e.to_string())?;
    println!("subset: {} of {} requests → {out_dir}", kept.len(), wanted.len());
    Ok(())
}

fn post(out_dir: &Path) -> Result<(), String> {
    let words: HashMap<String, String> = fs::read_to_string(out_dir.join("batch-requests.jsonl"))
        .map_err(|e| e.to_string())?
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .map(|v| {
            let body = v["params"]["messages"][0]["content"][0]["text"].as_str().unwrap_or("");
            let word = body.lines().find_map(|l| l.strip_prefix("WORD: ")).unwrap_or("word").to_string();
            (v["custom_id"].as_str().unwrap_or("").to_string(), word)
        })
        .collect();
    let lessons_dir = out_dir.join("lessons");
    fs::create_dir_all(&lessons_dir).map_err(|e| e.to_string())?;
    let mut problems: BTreeMap<&str, usize> = BTreeMap::new();
    let mut position: BTreeMap<usize, usize> = BTreeMap::new();
    let mut written = 0;
    for line in fs::read_to_string(out_dir.join("batch-results.jsonl")).map_err(|e| e.to_string())?.lines() {
        let r: Value = serde_json::from_str(line).map_err(|e| e.to_string())?;
        let id = r["custom_id"].as_str().unwrap_or("").to_string();
        let n: usize = id.parse::<usize>().map_err(|_| "custom_id is not a number")? + 1;
        if r["result"]["type"] != "succeeded" {
            *problems.entry("failed request").or_default() += 1;
            continue;
        }
        let text: String = r["result"]["message"]["content"].as_array().map(|c| c.iter().filter_map(|b| b["text"].as_str()).collect()).unwrap_or_default();
        let mut rng = Rng::new(n as u64);
        let mut out_lines = Vec::new();
        for l in text.lines() {
            let mut l = if l.starts_with("lesson:") { format!("lesson: {n}") } else { l.to_string() };
            if QUESTION_KEYS.iter().any(|k| l.starts_with(k)) && l.contains(" | ") {
                let parts: Vec<&str> = l.split(" | ").collect();
                let (question, opts) = (parts[0], &parts[1..]);
                let marked: Vec<usize> = opts.iter().enumerate().filter(|(_, o)| o.starts_with('*')).map(|(i, _)| i).collect();
                if marked.len() != 1 {
                    *problems.entry("not exactly one *").or_default() += 1;
                    out_lines.push(std::iter::once(question.to_string()).chain(opts.iter().map(|o| o.trim_start_matches('*').to_string())).collect::<Vec<_>>().join(" | "));
                    continue;
                }
                let right = opts[marked[0]].trim_start_matches('*').to_string();
                let mut shuffled: Vec<String> = opts.iter().map(|o| o.trim_start_matches('*').to_string()).collect();
                rng.shuffle(&mut shuffled);
                let at = shuffled.iter().position(|o| *o == right).unwrap_or(0);
                *position.entry(at).or_default() += 1;
                let rebuilt: Vec<String> = shuffled.into_iter().map(|o| if o == right { format!("*{o}") } else { o }).collect();
                l = std::iter::once(question.to_string()).chain(rebuilt).collect::<Vec<_>>().join(" | ");
            }
            out_lines.push(l);
        }
        let word = words.get(&id).map(String::as_str).unwrap_or("word");
        let safe: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
        fs::write(lessons_dir.join(format!("{n:03}-{safe}.lesson")), out_lines.join("\n") + "\n").map_err(|e| e.to_string())?;
        written += 1;
    }
    println!("lessons written: {written}; problems: {problems:?}");
    println!("right answer position after shuffle (0 = first): {position:?}");
    Ok(())
}

/// sent_id → text, from the silver corpus (block: `# text = ` then `# sent_id = `).
fn sent_texts(path: &str) -> Result<HashMap<String, String>, String> {
    let mut map = HashMap::new();
    let mut text = String::new();
    for l in fs::read_to_string(path).map_err(|e| e.to_string())?.lines() {
        if let Some(t) = l.strip_prefix("# text = ") {
            text = t.to_string();
        } else if let Some(s) = l.strip_prefix("# sent_id = ") {
            map.insert(s.to_string(), text.clone());
        }
    }
    Ok(map)
}

/// Deterministic xorshift: the same lesson number always gives the same order.
struct Rng(u64);
impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = (self.next() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
    }
}
