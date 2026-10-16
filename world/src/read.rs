//! Reading fairy tales. Design note: reading a book or a fairy tale, it can write a summary of each paragraph for
//! the LLM to check; at the end also answer questions about the paragraphs and the book.
//!
//! v1: the snake on its own, without the LLM, writes a paragraph summary — the skeleton of each sentence from the UD tree (`en`): agent, action,
//! object, recipient, place, negation; direct speech — «who says: …». The LLM only checks the summary as a separate step
//! (`read-check`): faithfulness and completeness 0–2. Questions about paragraphs and the book are the next step.

use anyhow::Result;
use en::gram::{Rel, UPos};

use crate::tree::Tree;

/// Gutenberg book → stories (title, paragraphs): headings in capitals between blank lines; the table of contents
/// (headings with no text between them) is skipped; the Gutenberg header and footer are dropped.
pub fn split_book(text: &str) -> Vec<(String, Vec<String>)> {
    let body = text.split("*** START").nth(1).map(|s| s.split_once('\n').map(|x| x.1).unwrap_or(s)).unwrap_or(text);
    let body = body.split("*** END").next().unwrap_or(body);
    let lines: Vec<&str> = body.lines().map(|l| l.trim_end_matches('\r')).collect();
    let is_title = |i: usize| {
        let l = lines[i].trim();
        let letters: Vec<char> = l.chars().filter(|c| c.is_alphabetic()).collect();
        let blank_before = i == 0 || lines[i - 1].trim().is_empty();
        let caps = letters.len() >= 5 && letters.iter().all(|c| c.is_uppercase()) && l.len() < 70;
        // centered heading in normal case (Lang's books, «The Thousand and One Nights»)
        let indented = lines[i].starts_with("   ") && l.chars().next().is_some_and(|c| c.is_uppercase()) && l.split_whitespace().count() <= 10 && !l.ends_with(['.', ',', ';', ':']) && letters.len() >= 5 && i + 1 < lines.len() && lines[i + 1].trim().is_empty();
        blank_before && (caps || indented)
    };
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    let mut cur: Option<(String, Vec<String>, String)> = None;
    let flush = |cur: &mut Option<(String, Vec<String>, String)>, out: &mut Vec<(String, Vec<String>)>| {
        if let Some((t, mut ps, buf)) = cur.take() {
            if !buf.trim().is_empty() {
                ps.push(buf.split_whitespace().collect::<Vec<_>>().join(" "));
            }
            let total: usize = ps.iter().map(|p| p.len()).sum();
            if total >= 200 {
                out.push((t, ps));
            }
        }
    };
    for i in 0..lines.len() {
        if is_title(i) {
            flush(&mut cur, &mut out);
            cur = Some((lines[i].trim().to_string(), Vec::new(), String::new()));
            continue;
        }
        if let Some((_, ps, buf)) = cur.as_mut() {
            if lines[i].trim().is_empty() {
                if !buf.trim().is_empty() {
                    ps.push(buf.split_whitespace().collect::<Vec<_>>().join(" "));
                    buf.clear();
                }
            } else {
                buf.push(' ');
                buf.push_str(lines[i].trim());
            }
        }
    }
    flush(&mut cur, &mut out);
    // illustration paragraphs and service lines: «[Illustration]» etc.
    for (_, ps) in out.iter_mut() {
        ps.retain(|p| !p.starts_with('[') && p.split_whitespace().count() >= 4);
    }
    out.retain(|(t, ps)| !ps.is_empty() && !["INTRODUCTION", "PREFACE", "CONTENTS", "LIST OF ILLUSTRATIONS", "ILLUSTRATIONS", "NOTE", "NOTES", "INDEX", "Preface", "Contents", "Introduction"].contains(&t.as_str()) && !t.ends_with("Fairy Book"));
    out
}

fn phrase(t: &Tree, i: usize, orig: &[String]) -> String {
    // noun phrase: possessive, adjective, compound + head word (form with the original casing)
    let mut parts: Vec<(usize, String)> = Vec::new();
    for &k in t.kids(i) {
        if t.w[k].rel == Rel::NmodPoss || matches!(t.base(k), Rel::Amod | Rel::Compound) {
            parts.push((k, orig.get(k).cloned().unwrap_or_default()));
        }
    }
    parts.push((i, orig.get(i).cloned().unwrap_or_default()));
    parts.sort_by_key(|p| p.0);
    parts.into_iter().map(|p| p.1).collect::<Vec<_>>().join(" ")
}

const SAY: &[&str] = &["say", "tell", "ask", "cry", "reply", "answer", "shout", "exclaim", "call", "whisper", "think"];

/// Sentence skeleton: for the root and coordinated verbs — «agent action object [recipient] [place]»; speech — «who says: …».
pub fn skeleton(t: &Tree, orig: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let Some(root) = t.root() else { return out };
    // verb nodes: root, coordinated, adverbial clauses (advcl) and complement clauses (ccomp)
    let mut verbs = vec![root];
    let mut k = 0;
    while k < verbs.len() {
        let v = verbs[k];
        for &c in t.kids(v) {
            if matches!(t.base(c), Rel::Conj | Rel::Advcl | Rel::Ccomp | Rel::Parataxis) && matches!(t.w[c].upos, UPos::VERB | UPos::AUX | UPos::ADJ | UPos::NOUN) && !verbs.contains(&c) {
                verbs.push(c);
            }
        }
        k += 1;
    }
    verbs.sort();
    let mut last_subj = String::new();
    for &v in &verbs {
        let subj = t.kids(v).iter().copied().find(|&c| matches!(t.w[c].rel, Rel::Nsubj | Rel::NsubjPass)).map(|c| phrase(t, c, orig)).unwrap_or_else(|| last_subj.clone());
        if !subj.is_empty() {
            last_subj = subj.clone();
        }
        let neg = t.kids(v).iter().any(|&c| matches!(t.lemma(c), "not" | "n't" | "never" | "no") || t.form(c) == "n't");
        // modal verb (could, would, must…) is part of the content: «could not stand»
        let modal = t.kids_of(v, Rel::Aux).into_iter().find(|&c| matches!(t.lemma(c), "can" | "could" | "will" | "would" | "must" | "should" | "may" | "might")).map(|c| orig.get(c).cloned().unwrap_or_default());
        if !matches!(t.w[v].upos, UPos::VERB) {
            // copular sentence: «X was in vain», «the Fox was hungry»
            if let Some(cop) = t.kids_of(v, Rel::Cop).first() {
                let pred = phrase(t, v, orig);
                let case = t.kids_of(v, Rel::Case).first().map(|&c| orig.get(c).cloned().unwrap_or_default()).unwrap_or_default();
                out.push(format!("{} {}{} {} {}", if subj.is_empty() { "it" } else { &subj }, orig.get(*cop).cloned().unwrap_or_default(), if neg { " not" } else { "" }, case, pred).split_whitespace().collect::<Vec<_>>().join(" "));
            }
            continue;
        }
        let lemma = t.lemma(v).to_string();
        let form = orig.get(v).cloned().unwrap_or_else(|| lemma.clone());
        if SAY.contains(&lemma.as_str()) {
            out.push(format!("{} {}{}", if subj.is_empty() { "someone" } else { &subj }, if neg { "not " } else { "" }, form));
            continue;
        }
        let obj = t.kids_of(v, Rel::Obj).first().map(|&o| phrase(t, o, orig));
        let iobj = t.kids_of(v, Rel::Iobj).first().map(|&o| phrase(t, o, orig));
        let obl = t.kids_of(v, Rel::Obl).first().map(|&o| {
            let case = t.kids_of(o, Rel::Case).first().map(|&c| t.form(c).to_string()).unwrap_or_default();
            format!("{case} {}", phrase(t, o, orig)).trim().to_string()
        });
        let xc = t.kids_of(v, Rel::Xcomp).first().map(|&x| orig.get(x).cloned().unwrap_or_default());
        let prt = t.kids(v).iter().find(|&&c| t.w[c].rel == Rel::CompoundPrt).map(|&c| orig.get(c).cloned().unwrap_or_default()).unwrap_or_default();
        let mut s = format!("{}{} {} {} {prt}", if subj.is_empty() { String::new() } else { format!("{subj} ") }, modal.unwrap_or_default(), if neg { "not" } else { "" }, form);
        if let Some(x) = xc { s.push_str(&format!(" {x}")); }
        if let Some(i) = iobj { s.push_str(&format!(" {i}")); }
        if let Some(o) = obj { s.push_str(&format!(" {o}")); }
        if let Some(o) = obl { s.push_str(&format!(" {o}")); }
        out.push(s.split_whitespace().collect::<Vec<_>>().join(" "));
    }
    out
}

/// Paragraph summary: sentence skeletons, no repeats, direct speech without its content (only who says it).
pub fn summarize(par: &str) -> Result<String> {
    let a = crate::tree::annotator()?;
    // cut out direct speech — keep «who says»
    let mut plain = String::new();
    let mut inq = false;
    for c in par.chars() {
        if c == '"' || c == '“' || c == '”' {
            inq = !inq;
            if inq { plain.push_str(" \u{2026} "); }
            continue;
        }
        if !inq {
            plain.push(c);
        }
    }
    let mut parts: Vec<String> = Vec::new();
    for s in crate::quant::sentences(&plain) {
        let orig: Vec<String> = a.tokenize(&s).into_iter().map(|t| t.form).collect();
        let t = Tree::parse(a, &s);
        for sk in skeleton(&t, &orig) {
            if !parts.contains(&sk) {
                parts.push(sk);
            }
        }
    }
    Ok(parts.join("; "))
}

/// Read a book: stories → paragraphs → the snake's summaries (JSONL in `dir/summaries.jsonl`).
pub fn read_book(path: &std::path::Path, dir: &std::path::Path, limit: usize) -> Result<usize> {
    std::fs::create_dir_all(dir)?;
    let text = std::fs::read_to_string(path)?;
    let book = path.file_stem().and_then(|s| s.to_str()).unwrap_or("book").to_string();
    let mut out = String::new();
    let mut n = 0;
    for (si, (title, ps)) in split_book(&text).into_iter().take(limit).enumerate() {
        for (pi, p) in ps.iter().enumerate() {
            let sum = summarize(p)?;
            out.push_str(&serde_json::json!({"book": book, "story": si, "title": title, "par": pi, "text": p, "summary": sum}).to_string());
            out.push('\n');
            n += 1;
        }
    }
    std::fs::write(dir.join("summaries.jsonl"), out)?;
    Ok(n)
}

/// The LLM checks the snake's summaries (a separate tool, not training): faithfulness and completeness 0–2 per paragraph.
pub fn check(dir: &std::path::Path, limit: usize, batch: usize) -> Result<()> {
    use prag::opus::{Opus, append_call};
    let rows: Vec<serde_json::Value> = std::fs::read_to_string(dir.join("summaries.jsonl"))?.lines().filter_map(|l| serde_json::from_str(l).ok()).take(limit).collect();
    let opus = Opus::from_env(dir.join("cwd"));
    let mut out = String::new();
    let (mut f_sum, mut c_sum, mut n) = (0.0f64, 0.0f64, 0.0f64);
    for (bi, ch) in rows.chunks(batch).enumerate() {
        let mut p = String::from("A small rule-based model wrote a skeleton summary of each paragraph of a fable (who did what to whom; speech is reduced to \"X says\"). Rate each summary against its paragraph.\nfaithful: 2 = nothing false, 1 = minor distortion, 0 = wrong actor/event.\ncoverage: 2 = all main events present, 1 = some missing, 0 = main event missing.\nAnswer ONLY lines `<n> faithful=<0-2> coverage=<0-2> missing=<few words or ->`.\n\n");
        for (i, r) in ch.iter().enumerate() {
            p.push_str(&format!("[{i}] PARAGRAPH: {}\nSUMMARY: {}\n\n", r["text"].as_str().unwrap_or(""), r["summary"].as_str().unwrap_or("")));
        }
        let (text, call) = opus.ask("read-check", bi, ch.len(), &p)?;
        append_call(&dir.join("calls.jsonl"), &call)?;
        for l in text.lines() {
            let l = l.trim();
            let Some((idx, rest)) = l.split_once(' ') else { continue };
            let Ok(i) = idx.trim_matches(|c| c == '[' || c == ']').parse::<usize>() else { continue };
            let get = |k: &str| rest.split_whitespace().find_map(|w| w.strip_prefix(&format!("{k}=")).and_then(|v| v.parse::<f64>().ok()));
            if let (Some(f), Some(c), Some(r)) = (get("faithful"), get("coverage"), ch.get(i)) {
                f_sum += f;
                c_sum += c;
                n += 1.0;
                let missing = rest.split("missing=").nth(1).unwrap_or("").to_string();
                out.push_str(&serde_json::json!({"title": r["title"], "par": r["par"], "faithful": f, "coverage": c, "missing": missing}).to_string());
                out.push('\n');
            }
        }
    }
    std::fs::write(dir.join("check.jsonl"), out)?;
    println!("checked {n} paragraphs: faithfulness {:.2}/2, completeness {:.2}/2", f_sum / n.max(1.0), c_sum / n.max(1.0));
    Ok(())
}

// ---------- questions about what was read (the snake on its own) ----------

const QSTOP: &[&str] = &["be", "do", "have", "what", "who", "whom", "whose", "where", "when", "why", "how", "which", "the", "a", "an", "of", "to", "in", "on", "at", "for", "with", "did", "does", "could", "would", "will", "can", "that", "this", "it", "he", "she", "they", "his", "her", "their", "him", "them", "and", "or", "not", "feel", "happen", "kind", "after", "before", "because"];

fn content(t: &Tree) -> Vec<String> {
    (0..t.len()).filter(|&i| matches!(t.w[i].upos, UPos::NOUN | UPos::PROPN | UPos::VERB | UPos::ADJ | UPos::NUM | UPos::ADV) && !QSTOP.contains(&t.lemma(i))).map(|i| t.lemma(i).to_string()).collect()
}

/// Subtree of a node as text (in word order).
fn subtree(t: &Tree, i: usize) -> String {
    let mut idx = vec![i];
    let mut k = 0;
    while k < idx.len() {
        let v = idx[k];
        idx.extend(t.kids(v).iter().copied().filter(|&c| t.base(c) != Rel::Punct));
        k += 1;
    }
    idx.sort();
    idx.iter().map(|&j| t.form(j)).collect::<Vec<_>>().join(" ")
}

/// The snake's answer to a question about a fairy tale: the sentence with the largest weight of shared content lemmas (rare ones weigh
/// more), then the sentence constituent by question type. Returns (sentence, answer, how it was found).
pub fn answer(sents: &[String], question: &str) -> Result<(String, String, String)> {
    let a = crate::tree::annotator()?;
    let trees: Vec<Tree> = sents.iter().map(|s| Tree::parse(a, s)).collect();
    let qt = Tree::parse(a, question);
    let qc = content(&qt);
    let mut df: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for t in &trees {
        let mut seen = std::collections::HashSet::new();
        for l in content(t) {
            if seen.insert(l.clone()) {
                *df.entry(l).or_default() += 1;
            }
        }
    }
    let n = trees.len().max(1) as f64;
    let sc: Vec<f64> = trees.iter().map(|t| {
        let c = content(t);
        qc.iter().filter(|w| c.contains(w)).map(|w| (n / (1.0 + *df.get(w).unwrap_or(&0) as f64)).ln() + 1.0).sum()
    }).collect();
    // neighbours add weight: answer and question are often spread over two sentences (READ_NB — neighbour weight)
    let nb: f64 = std::env::var("READ_NB").ok().and_then(|x| x.parse().ok()).unwrap_or(0.3);
    let (mut best, _) = (0..trees.len()).map(|i| {
        let s = sc[i] + nb * (if i > 0 { sc[i - 1] } else { 0.0 } + sc.get(i + 1).copied().unwrap_or(0.0));
        (i, s)
    }).fold((0, f64::MIN), |acc, x| if x.1 > acc.1 { x } else { acc });
    // «what happened / what next / how did it end» — the answer is in the next sentence if it has no question words
    let ql0 = question.to_lowercase();
    if std::env::var("READ_NEXT").as_deref() != Ok("0") && (ql0.contains("happen") || ql0.contains(" after ") || ql0.contains("then") || ql0.contains("next") || ql0.contains("result")) && best + 1 < trees.len() && sc[best + 1] < sc[best] * 0.5 {
        best += 1;
    }
    let (ans, how) = extract(&trees[best], &qt, &qc, question);
    Ok((sents[best].clone(), ans, how))
}


/// Sentence constituent by question type (who → agent, where → place, why → reason, what did X do → dependent…).
fn extract(t: &Tree, qt: &Tree, qc: &[String], question: &str) -> (String, String) {
    let ql = question.to_lowercase();
    let first = ql.split_whitespace().next().unwrap_or("");
    // the sentence verb that matches the question verb (or the root)
    let qverbs: Vec<&str> = (0..qt.len()).filter(|&i| qt.w[i].upos == UPos::VERB).map(|i| qt.lemma(i)).collect();
    let v = (0..t.len()).find(|&i| t.w[i].upos == UPos::VERB && qverbs.contains(&t.lemma(i))).or_else(|| t.root()).unwrap_or(0);
    let pick = |rels: &[Rel]| t.kids(v).iter().copied().find(|&c| rels.contains(&t.base(c)));
    let (ans, how) = match first {
        "who" | "whom" => {
            let qsubj_is_wh = (0..qt.len()).any(|i| matches!(qt.lemma(i), "who") && matches!(qt.base(i), Rel::Nsubj));
            let r = if qsubj_is_wh || first == "who" { pick(&[Rel::Nsubj]) } else { pick(&[Rel::Obj, Rel::Iobj]) };
            (r.map(|i| subtree(t, i)), "who → agent")
        }
        "where" => {
            let r = t.kids(v).iter().copied().find(|&c| t.base(c) == Rel::Obl && t.kids_of(c, Rel::Case).iter().any(|&k| matches!(t.lemma(k), "in" | "at" | "on" | "to" | "into" | "under" | "near" | "from" | "through" | "over" | "by")));
            (r.map(|i| subtree(t, i)), "where → place")
        }
        "when" => {
            let r = t.kids(v).iter().copied().find(|&c| t.base(c) == Rel::Advcl || (t.base(c) == Rel::Obl && t.w[c].rel == Rel::OblTmod));
            (r.map(|i| subtree(t, i)), "when → time")
        }
        "why" => {
            let r = t.kids(v).iter().copied().find(|&c| t.base(c) == Rel::Advcl).or_else(|| (0..t.len()).find(|&i| matches!(t.lemma(i), "because" | "for" | "so") && t.base(i) == Rel::Mark).and_then(|m| t.head(m)));
            (r.map(|i| subtree(t, i)), "why → reason or purpose")
        }
        "how" if ql.starts_with("how many") || ql.starts_with("how much") => {
            let r = (0..t.len()).find(|&i| t.base(i) == Rel::Nummod).map(|i| t.head(i).map(|h| subtree(t, h)).unwrap_or_else(|| t.form(i).to_string()));
            (r, "how many → number")
        }
        "what" if qverbs.contains(&"do") || qverbs.contains(&"happen") || ql.contains(" do ") || ql.contains(" do?") || ql.contains("happen") => {
            // what did they do / what happened — the verb's dependent (below)
            (None, "what did X do → dependent")
        }
        "what" => {
            let r = pick(&[Rel::Obj]).or_else(|| pick(&[Rel::Nsubj])).filter(|&i| !qc.contains(&t.lemma(i).to_string()));
            (r.map(|i| subtree(t, i)), "what → object")
        }
        _ => (None, ""),
    };
    // a short single word — expand to its subtree; for actions and consequences — the verb's dependent without question words
    let clause = {
        let mut idx = vec![v];
        let mut k = 0;
        while k < idx.len() {
            let x = idx[k];
            idx.extend(t.kids(x).iter().copied().filter(|&c| t.base(c) != Rel::Punct && !(x == v && matches!(t.base(c), Rel::Conj | Rel::Cc))));
            k += 1;
        }
        idx.sort();
        idx.iter().copied().filter(|&j| !qc.contains(&t.lemma(j).to_string()) || matches!(t.w[j].upos, UPos::VERB)).map(|j| t.form(j)).collect::<Vec<_>>().join(" ")
    };
    let ans = match (ans, first) {
        (Some(x), _) if x.split_whitespace().count() <= 1 && !matches!(first, "who" | "whom") => Some(clause.clone()),
        (None, "what") | (None, "why") | (None, "how") => Some(clause.clone()),
        (a, _) => a,
    };
    // fallback: sentence words that are not in the question
    let (ans, how) = match ans {
        Some(x) if !x.trim().is_empty() => (x, how.to_string()),
        _ => {
            let rest: Vec<&str> = (0..t.len()).filter(|&i| t.base(i) != Rel::Punct && !qc.contains(&t.lemma(i).to_string()) && !QSTOP.contains(&t.lemma(i))).map(|i| t.form(i)).collect();
            (rest.join(" "), "rest of sentence".to_string())
        }
    };
    (ans, how)
}

/// Measurement on FairytaleQA (Apache-2.0): ROUGE-L of the snake's answer and of the «whole found sentence» baseline.
pub fn eval_ftqa(dir: &std::path::Path, show: usize) -> Result<()> {
    let mut stories: Vec<std::path::PathBuf> = std::fs::read_dir(dir.join("stories"))?.flatten().map(|e| e.path()).collect();
    stories.sort();
    let (mut r_ans, mut r_sent, mut n) = (0.0f64, 0.0f64, 0usize);
    let mut by: std::collections::BTreeMap<String, (f64, f64, usize)> = std::collections::BTreeMap::new();
    let mut shown = 0;
    for (doc, sp) in stories.iter().enumerate() {
        let text = crate::ftqa::read_story(sp, doc as i64)?;
        let sents: Vec<String> = text.sents.iter().map(|x| x.1.clone()).collect();
        let qp = dir.join("questions").join(sp.file_name().unwrap());
        let Ok(qs) = crate::ftqa::read_qa(&qp) else { continue };
        for q in &qs {
            let (sent, ans, how) = answer(&sents, &q.question)?;
            let ra = crate::ans::rouge_q(&ans, q);
            let rs = crate::ans::rouge_q(&sent, q);
            r_ans += ra;
            r_sent += rs;
            n += 1;
            let e = by.entry(q.attr.clone()).or_default();
            e.0 += ra;
            e.1 += rs;
            e.2 += 1;
            if shown < show {
                shown += 1;
                println!("? {}\n  snake ({how}): {ans}\n  human: {}   ROUGE-L {ra:.2} (sentence {rs:.2})", q.question, q.a1);
            }
        }
    }
    println!("FairytaleQA, {n} questions: snake ROUGE-L {:.3}; whole sentence {:.3}", r_ans / n.max(1) as f64, r_sent / n.max(1) as f64);
    for (k, (a, s, c)) in &by {
        println!("  {k:>22}: snake {:.3}, sentence {:.3} ({c})", a / *c as f64, s / *c as f64);
    }
    Ok(())
}

// ---------- context graph of a story ----------
//
// Design note: the context storage structure should hold paragraph links better; the answer should come not
// from walking over neighbours but via the context graph, triggers and associations — the apple gets nervous about the knife next to it;
// objects are linked by the graph not only through place, but through interaction of different characters, or as property;
// «you also have to train graph walks, connectivity, typical shortcuts».
//
// Nodes — entities (nouns and names by lemma; pronoun → the last subject), sentences. Links: interaction
// (subject and object of one action), place (entity — place adverbial), property (possessive, «have»), shared
// sentence. Search: from the question's entities activation spreads two steps with type weights; a sentence gets
// the weight of its activated entities. Type weights are learned on the valid split (grid search), test — separately.

pub const KINDS_N: usize = 4; // interact, place, own, co

pub struct Story {
    pub sents: Vec<String>,
    trees: Vec<Tree>,
    ents: Vec<Vec<String>>,
    /// entity → (neighbour, type, count)
    adj: std::collections::HashMap<String, Vec<(String, usize, f64)>>,
    df: std::collections::HashMap<String, usize>,
    content: Vec<Vec<String>>,
}

fn ent_of(t: &Tree, i: usize, last_subj: &str) -> Option<String> {
    match t.w[i].upos {
        UPos::NOUN | UPos::PROPN => Some(t.lemma(i).to_string()),
        UPos::PRON if matches!(t.lemma(i), "he" | "she" | "it" | "they" | "him" | "her" | "them") && !last_subj.is_empty() => Some(last_subj.to_string()),
        _ => None,
    }
}

pub fn prepare_story(sents: Vec<String>) -> Result<Story> {
    let a = crate::tree::annotator()?;
    let trees: Vec<Tree> = sents.iter().map(|s| Tree::parse(a, s)).collect();
    let mut ents = Vec::new();
    let mut adj: std::collections::HashMap<String, Vec<(String, usize, f64)>> = std::collections::HashMap::new();
    let mut add = |a: &str, b: &str, k: usize, adj: &mut std::collections::HashMap<String, Vec<(String, usize, f64)>>| {
        if a == b { return; }
        for (x, y) in [(a, b), (b, a)] {
            let v = adj.entry(x.to_string()).or_default();
            match v.iter_mut().find(|e| e.0 == y && e.1 == k) {
                Some(e) => e.2 += 1.0,
                None => v.push((y.to_string(), k, 1.0)),
            }
        }
    };
    let mut last_subj = String::new();
    for t in &trees {
        let mut es: Vec<String> = Vec::new();
        let mut subj_here = String::new();
        for i in 0..t.len() {
            if let Some(e) = ent_of(t, i, &last_subj) {
                if !es.contains(&e) { es.push(e.clone()); }
                if matches!(t.w[i].rel, Rel::Nsubj | Rel::NsubjPass) && subj_here.is_empty() && matches!(t.w[i].upos, UPos::NOUN | UPos::PROPN) {
                    subj_here = e;
                }
            }
        }
        for v in 0..t.len() {
            if !matches!(t.w[v].upos, UPos::VERB) { continue; }
            let subj: Vec<String> = t.kids(v).iter().filter(|&&c| matches!(t.w[c].rel, Rel::Nsubj | Rel::NsubjPass)).filter_map(|&c| ent_of(t, c, &last_subj)).collect();
            let objs: Vec<String> = t.kids(v).iter().filter(|&&c| matches!(t.base(c), Rel::Obj | Rel::Iobj)).filter_map(|&c| ent_of(t, c, &last_subj)).collect();
            let places: Vec<String> = t.kids(v).iter().filter(|&&c| t.base(c) == Rel::Obl && t.kids_of(c, Rel::Case).iter().any(|&k| matches!(t.lemma(k), "in" | "at" | "on" | "under" | "near" | "into" | "beside" | "by"))).filter_map(|&c| ent_of(t, c, &last_subj)).collect();
            for s in &subj {
                for o in &objs { add(s, o, 0, &mut adj); }
                if t.lemma(v) == "have" || t.lemma(v) == "own" { for o in &objs { add(s, o, 2, &mut adj); } }
            }
            for p in &places {
                for x in subj.iter().chain(objs.iter()) { add(x, p, 1, &mut adj); }
            }
        }
        for i in 0..t.len() {
            if t.w[i].rel == Rel::NmodPoss {
                if let (Some(owner), Some(h)) = (ent_of(t, i, &last_subj), t.head(i).and_then(|h| ent_of(t, h, &last_subj))) {
                    add(&owner, &h, 2, &mut adj);
                }
            }
        }
        for x in 0..es.len() {
            for y in x + 1..es.len() { add(&es[x].clone(), &es[y].clone(), 3, &mut adj); }
        }
        if !subj_here.is_empty() { last_subj = subj_here; }
        ents.push(es);
    }
    let content_v: Vec<Vec<String>> = trees.iter().map(content).collect();
    let mut df = std::collections::HashMap::new();
    for c in &content_v {
        let mut seen = std::collections::HashSet::new();
        for l in c { if seen.insert(l.clone()) { *df.entry(l.clone()).or_default() += 1; } }
    }
    Ok(Story { sents, trees, ents, adj, df, content: content_v })
}

#[derive(Clone, Copy, Debug)]
pub struct GraphParams {
    pub w: [f64; KINDS_N],
    pub lambda: f64,
}

/// Answer via the graph: lexical weight + activation of the sentence's entities from the question (two steps).
pub fn answer_graph(st: &Story, question: &str, gp: GraphParams) -> Result<(usize, String, String)> {
    let a = crate::tree::annotator()?;
    let qt = Tree::parse(a, question);
    let qc = content(&qt);
    let n = st.trees.len().max(1) as f64;
    let mut act: std::collections::HashMap<String, f64> = std::collections::HashMap::new();
    let q_ents: Vec<String> = (0..qt.len()).filter_map(|i| ent_of(&qt, i, "")).collect();
    for e in &q_ents { act.insert(e.clone(), 1.0); }
    for hop in 0..2 {
        let decay = if hop == 0 { 1.0 } else { 0.5 };
        let cur: Vec<(String, f64)> = act.iter().map(|(k, v)| (k.clone(), *v)).collect();
        for (e, v) in cur {
            if let Some(ns) = st.adj.get(&e) {
                let tot: f64 = ns.iter().map(|x| x.2).sum::<f64>().max(1.0);
                for (m, k, c) in ns {
                    *act.entry(m.clone()).or_insert(0.0) += decay * v * gp.w[*k] * c / tot;
                }
            }
        }
    }
    let mut best = (0usize, f64::MIN);
    for i in 0..st.trees.len() {
        let lex: f64 = qc.iter().filter(|w| st.content[i].contains(w)).map(|w| (n / (1.0 + *st.df.get(w).unwrap_or(&0) as f64)).ln() + 1.0).sum();
        let g: f64 = st.ents[i].iter().map(|e| act.get(e).copied().unwrap_or(0.0)).sum::<f64>() / (st.ents[i].len() as f64).sqrt().max(1.0);
        let s = lex + gp.lambda * g;
        if s > best.1 { best = (i, s); }
    }
    let (ans, how) = extract(&st.trees[best.0], &qt, &qc, question);
    Ok((best.0, ans, how))
}

type Split = Vec<(Story, Vec<crate::ftqa::Qa>)>;

pub fn load_split(dir: &std::path::Path) -> Result<Split> {
    let mut stories: Vec<std::path::PathBuf> = std::fs::read_dir(dir.join("stories"))?.flatten().map(|e| e.path()).collect();
    stories.sort();
    let mut out = Vec::new();
    for (doc, sp) in stories.iter().enumerate() {
        let text = crate::ftqa::read_story(sp, doc as i64)?;
        let Ok(qs) = crate::ftqa::read_qa(&dir.join("questions").join(sp.file_name().unwrap())) else { continue };
        out.push((prepare_story(text.sents.iter().map(|x| x.1.clone()).collect())?, qs));
    }
    Ok(out)
}

pub fn score_split(sp: &Split, gp: GraphParams) -> Result<(f64, f64)> {
    let (mut ra, mut rs, mut n) = (0.0, 0.0, 0.0);
    for (st, qs) in sp {
        for q in qs {
            let (i, ans, _) = answer_graph(st, &q.question, gp)?;
            ra += crate::ans::rouge_q(&ans, q);
            rs += crate::ans::rouge_q(&st.sents[i], q);
            n += 1.0;
        }
    }
    Ok((ra / n, rs / n))
}

/// Training graph walks: grid search over link-type weights and λ on valid; the best ones are measured on test.
pub fn train_graph(valid: &std::path::Path, test: &std::path::Path) -> Result<()> {
    let v = load_split(valid)?;
    let t = load_split(test)?;
    let base = GraphParams { w: [0.0; KINDS_N], lambda: 0.0 };
    let (bv, _) = score_split(&v, base)?;
    let (bt, bts) = score_split(&t, base)?;
    println!("without graph: valid {bv:.3}; test {bt:.3} (sentence {bts:.3})");
    let grid = [0.0, 0.5, 1.0];
    let mut best = (base, bv);
    for &l in &[0.5, 1.0, 2.0, 4.0] {
        for &a in &grid { for &b in &grid { for &c in &grid { for &d in &grid {
            if a + b + c + d == 0.0 { continue; }
            let gp = GraphParams { w: [a, b, c, d], lambda: l };
            let (sv, _) = score_split(&v, gp)?;
            if sv > best.1 { best = (gp, sv); }
        }}}}
    }
    let (st, sts) = score_split(&t, best.0)?;
    println!("best weights on valid: interaction {}, place {}, property {}, shared sentence {}, λ {} → valid {:.3}", best.0.w[0], best.0.w[1], best.0.w[2], best.0.w[3], best.0.lambda, best.1);
    println!("test with these weights: snake {st:.3} (sentence {sts:.3}); without graph {bt:.3}");
    Ok(())
}

// ---------- event graph ----------
//
// Node — an event: a verb with participants (subject, object, place) and its own dependent. Links: next /
// previous in the story, cause (a dependent with because / so / for / since, or the next event after «so»),
// shared participant. The walk depends on the question type; which policy to use for which type is learned on valid
// (grid search), test — separately. The policy is transparent: «for why — the cause event», «for what happened — the next one».

#[derive(Clone, Debug)]
pub struct Event {
    pub sent: usize,
    pub verb: usize,
    pub lemma: String,
    pub subj: Vec<String>,
    pub text: String,
    pub content: Vec<String>,
    /// dependent of reason/purpose in this event (because, so that, to …)
    pub cause_text: Option<String>,
}

fn clause_text(t: &Tree, v: usize) -> String {
    let mut idx = vec![v];
    let mut k = 0;
    while k < idx.len() {
        let x = idx[k];
        idx.extend(t.kids(x).iter().copied().filter(|&c| t.base(c) != Rel::Punct && !(matches!(t.base(c), Rel::Conj | Rel::Parataxis) && matches!(t.w[c].upos, UPos::VERB))));
        k += 1;
    }
    idx.sort();
    idx.iter().map(|&j| t.form(j)).collect::<Vec<_>>().join(" ")
}

pub fn events_of(st: &Story) -> Vec<Event> {
    let mut out = Vec::new();
    let mut last_subj = String::new();
    for (si, t) in st.trees.iter().enumerate() {
        for v in 0..t.len() {
            if !matches!(t.w[v].upos, UPos::VERB) { continue; }
            let subj: Vec<String> = t.kids(v).iter().filter(|&&c| matches!(t.w[c].rel, Rel::Nsubj | Rel::NsubjPass)).filter_map(|&c| ent_of(t, c, &last_subj)).collect();
            if let Some(s) = subj.first() { last_subj = s.clone(); }
            let text = clause_text(t, v);
            let tt = Tree::parse(crate::tree::annotator().unwrap(), &text);
            let cause_text = t.kids(v).iter().copied().find(|&c| t.base(c) == Rel::Advcl && t.kids_of(c, Rel::Mark).iter().any(|&m| matches!(t.lemma(m), "because" | "so" | "for" | "since" | "as" | "to"))).map(|c| clause_text(t, c));
            out.push(Event { sent: si, verb: v, lemma: t.lemma(v).to_string(), subj, content: content(&tt), text, cause_text });
        }
    }
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum QType { Who, Where, When, Why, WhatHappen, WhatDo, Feel, What, How, Other }

pub fn qtype(q: &str) -> QType {
    let l = q.to_lowercase();
    let f = l.split_whitespace().next().unwrap_or("");
    if l.contains("feel") { return QType::Feel; }
    match f {
        "who" | "whom" | "whose" => QType::Who,
        "where" => QType::Where,
        "when" => QType::When,
        "why" => QType::Why,
        "how" => QType::How,
        "what" if l.contains("happen") => QType::WhatHappen,
        "what" if l.contains(" do ") || l.contains(" do?") || l.contains(" did ") && l.contains(" do") => QType::WhatDo,
        "what" => QType::What,
        _ => QType::Other,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Hop { Same, Next, Prev, Cause, SentenceOfSame, Extract }

pub const HOPS: [Hop; 6] = [Hop::Same, Hop::Next, Hop::Prev, Hop::Cause, Hop::SentenceOfSame, Hop::Extract];

/// Best event for a question: lexical weight (rare words weigh more) + verb match + subject match.
pub fn best_event(st: &Story, evs: &[Event], q: &str) -> Option<usize> {
    let a = crate::tree::annotator().ok()?;
    let qt = Tree::parse(a, q);
    let qc = content(&qt);
    let qverbs: Vec<String> = (0..qt.len()).filter(|&i| qt.w[i].upos == UPos::VERB).map(|i| qt.lemma(i).to_string()).collect();
    let qents: Vec<String> = (0..qt.len()).filter_map(|i| ent_of(&qt, i, "")).collect();
    let n = st.trees.len().max(1) as f64;
    let idf = |w: &str| (n / (1.0 + *st.df.get(w).unwrap_or(&0) as f64)).ln() + 1.0;
    let mut best = None;
    let mut bs = f64::MIN;
    for (i, e) in evs.iter().enumerate() {
        let mut s: f64 = qc.iter().filter(|w| e.content.contains(w)).map(|w| idf(w)).sum();
        // sentence context — half weight
        s += 0.5 * qc.iter().filter(|w| st.content[e.sent].contains(w) && !e.content.contains(w)).map(|w| idf(w)).sum::<f64>();
        if qverbs.contains(&e.lemma) { s += 1.5; }
        if e.subj.iter().any(|x| qents.contains(x)) { s += 1.0; }
        if s > bs { bs = s; best = Some(i); }
    }
    best
}

pub fn hop_text(st: &Story, evs: &[Event], i: usize, h: Hop, q: &str) -> String {
    match h {
        Hop::Extract => {
            let a = crate::tree::annotator().unwrap();
            let qt = Tree::parse(a, q);
            let qc = content(&qt);
            extract(&st.trees[evs[i].sent], &qt, &qc, q).0
        }
        Hop::Same => evs[i].text.clone(),
        Hop::Next => evs.get(i + 1).map(|e| e.text.clone()).unwrap_or_else(|| evs[i].text.clone()),
        Hop::Prev => if i > 0 { evs[i - 1].text.clone() } else { evs[i].text.clone() },
        Hop::Cause => evs[i].cause_text.clone().or_else(|| evs.get(i + 1).filter(|e| e.sent == evs[i].sent).map(|e| e.text.clone())).unwrap_or_else(|| evs[i].text.clone()),
        Hop::SentenceOfSame => st.sents[evs[i].sent].clone(),
    }
}

/// Training the walk policy: for each question type — the move with the best ROUGE-L on valid; measured on test.
pub fn train_events(valid: &std::path::Path, test: &std::path::Path) -> Result<()> {
    let prep = |p: &std::path::Path| -> Result<Vec<(Story, Vec<Event>, Vec<crate::ftqa::Qa>)>> {
        Ok(load_split(p)?.into_iter().map(|(st, qs)| { let ev = events_of(&st); (st, ev, qs) }).collect())
    };
    let v = prep(valid)?;
    let t = prep(test)?;
    // move scores on valid
    let mut tab: std::collections::BTreeMap<QType, std::collections::HashMap<Hop, (f64, usize)>> = std::collections::BTreeMap::new();
    for (st, evs, qs) in &v {
        for q in qs {
            let Some(i) = best_event(st, evs, &q.question) else { continue };
            for h in HOPS {
                let r = crate::ans::rouge_q(&hop_text(st, evs, i, h, &q.question), q);
                let e = tab.entry(qtype(&q.question)).or_default().entry(h).or_default();
                e.0 += r;
                e.1 += 1;
            }
        }
    }
    let mut policy: std::collections::BTreeMap<QType, Hop> = std::collections::BTreeMap::new();
    println!("walk policy (trained on valid):");
    for (qt, m) in &tab {
        let (h, (r, c)) = m.iter().max_by(|a, b| (a.1.0 / a.1.1 as f64).partial_cmp(&(b.1.0 / b.1.1 as f64)).unwrap()).unwrap();
        policy.insert(*qt, *h);
        println!("  {qt:?} → {h:?} (valid ROUGE-L {:.3}, {c} questions)", r / *c as f64);
    }
    // test: the policy against «whole sentence» and against the old answerer
    let (mut rp, mut rs, mut n) = (0.0, 0.0, 0.0);
    let mut by: std::collections::BTreeMap<QType, (f64, f64, usize)> = std::collections::BTreeMap::new();
    for (st, evs, qs) in &t {
        for q in qs {
            let Some(i) = best_event(st, evs, &q.question) else { continue };
            let qt = qtype(&q.question);
            let h = policy.get(&qt).copied().unwrap_or(Hop::SentenceOfSame);
            // feelings — from level 1 (explicit or inferred), if found
            let txt = if qt == QType::Feel { let (w, _) = feeling(st, evs, i, &q.question); if w.is_empty() { hop_text(st, evs, i, h, &q.question) } else { w } } else { hop_text(st, evs, i, h, &q.question) };
            let a = crate::ans::rouge_q(&txt, q);
            let s = crate::ans::rouge_q(&st.sents[evs[i].sent], q);
            rp += a; rs += s; n += 1.0;
            let e = by.entry(qt).or_default();
            e.0 += a; e.1 += s; e.2 += 1;
        }
    }
    println!("FairytaleQA test: event graph {:.3}; sentence of the best event {:.3} ({n} questions)", rp / n, rs / n);
    for (qt, (a, s, c)) in &by { println!("  {qt:?}: {:.3} / sentence {:.3} ({c})", a / *c as f64, s / *c as f64); }
    Ok(())
}

/// Feelings: an explicit feeling word in the event sentence and its two neighbours (level 1 categories),
/// otherwise — inference from the event via level 1 links (loss → sadness, received → joy…). Returns (word, how).
pub fn feeling(st: &Story, evs: &[Event], i: usize, q: &str) -> (String, String) {
    const EMO: [&str; 7] = ["joy", "sadness", "fear", "anger", "surprise", "gratitude", "shame"];
    let s0 = evs[i].sent;
    for d in [0i64, 1, -1, 2, -2] {
        let k = s0 as i64 + d;
        if k < 0 || k as usize >= st.trees.len() { continue; }
        let t = &st.trees[k as usize];
        for j in 0..t.len() {
            let l = t.lemma(j);
            for e in EMO {
                if let Some(c) = global::concept(e) {
                    if global::CONCEPTS[c].words.contains(&l) {
                        // negation before the word: «not at all pleased»
                        let neg = (j.saturating_sub(3)..j).any(|k| matches!(t.lemma(k), "not" | "n't" | "never" | "no"));
                        return (format!("{}{}", if neg { "not " } else { "" }, t.form(j)), format!("explicit feeling «{l}» ({e}){} in sentence {:+}", if neg { " with negation" } else { "" }, d));
                    }
                }
            }
        }
    }
    // inference: verbs of the event and sentence → level 1 event concepts → feeling
    let t = &st.trees[s0];
    // is the question's character the verb's object (being acted on)? then the feeling of «others» (ATOMIC oReact)
    if let Ok(a) = crate::tree::annotator() {
        let qt = Tree::parse(a, q);
        let qents: Vec<String> = (0..qt.len()).filter_map(|k| ent_of(&qt, k, "")).collect();
        for v in 0..t.len() {
            if t.w[v].upos != UPos::VERB { continue; }
            let subj_is_q = t.kids(v).iter().any(|&c| matches!(t.w[c].rel, Rel::Nsubj | Rel::NsubjPass) && ent_of(t, c, "").is_some_and(|e| qents.contains(&e)));
            // the object is the question's character, or a pronoun (him/her/them) when the subject is not the question's character
            let obj_is_q = t.kids(v).iter().any(|&c| matches!(t.base(c), Rel::Obj | Rel::Iobj) && (ent_of(t, c, "").is_some_and(|e| qents.contains(&e)) || (t.w[c].upos == UPos::PRON && matches!(t.lemma(c), "he" | "she" | "they" | "him" | "her" | "them") && !subj_is_q)));
            if !obj_is_q { continue; }
            let l = t.lemma(v);
            for from in ["atomic_o_gratitude_event", "atomic_o_sadness_event", "atomic_o_anger_event", "atomic_o_fear_event", "atomic_o_surprise_event", "atomic_o_joy_event", "atomic_o_shame_event"] {
                let Some(fc) = global::concept(from) else { continue };
                if !global::CONCEPTS[fc].words.contains(&l) { continue; }
                if let Some(l0) = global::LINKS.iter().find(|x| x.from == fc) {
                    let emo = &global::CONCEPTS[l0.to];
                    return (emo.words[0].to_string(), format!("inferred (character acted on): «{l}» → {from} → {} ({})", emo.name, l0.why));
                }
            }
        }
    }
    for j in 0..t.len() {
        let l = t.lemma(j);
        for from in ["loss_event", "gain_event", "threat_event", "wrong_event", "help_event", "atomic_sadness_event", "atomic_anger_event", "atomic_shame_event", "atomic_fear_event", "atomic_gratitude_event", "atomic_surprise_event"] {
            let Some(fc) = global::concept(from) else { continue };
            if !global::CONCEPTS[fc].words.contains(&l) { continue; }
            if let Some(l0) = global::LINKS.iter().find(|x| x.from == fc) {
                let emo = &global::CONCEPTS[l0.to];
                return (emo.words[0].to_string(), format!("inferred: «{l}» → {from} → {} ({})", emo.name, l0.why));
            }
        }
    }
    (String::new(), "neither an explicit feeling nor a level 1 event — report".into())
}

/// Feelings measurement on FairytaleQA (valid and test): explicit / inferred / nothing.
pub fn eval_feel(valid: &std::path::Path, test: &std::path::Path) -> Result<()> {
    for (name, dir) in [("valid", valid), ("test", test)] {
        let sp = load_split(dir)?;
        let (mut r, mut r_old, mut n, mut expl, mut inf) = (0.0, 0.0, 0.0, 0, 0);
        let mut shown = 0;
        for (st, qs) in &sp {
            let evs = events_of(st);
            for q in qs {
                if qtype(&q.question) != QType::Feel { continue; }
                let Some(i) = best_event(st, &evs, &q.question) else { continue };
                let (w, how) = feeling(st, &evs, i, &q.question);
                if how.starts_with("explicit") { expl += 1; } else if how.starts_with("inferred") { inf += 1; }
                r += crate::ans::rouge_q(&w, q);
                r_old += crate::ans::rouge_q(&st.sents[evs[i].sent], q);
                n += 1.0;
                if name == "test" && shown < 5 {
                    shown += 1;
                    println!("  ? {}\n    snake: {w} — {how}\n    human: {}", q.question, q.a1);
                }
            }
        }
        println!("{name}: feelings {n} questions — snake {:.3} (sentence {:.3}); explicit {expl}, inferred {inf}", r / n, r_old / n);
    }
    Ok(())
}

// ---------- short questions from the LLM for fast training and checking without the LLM ----------
//
// Design note: when training on fairy tales and books, Opus medium can create lots of simple questions for each
// paragraph, and across many paragraphs; the answer must be very short — then checking the answer won't need
// Opus for training and validating the SLM. Wide input (a batch of paragraphs), dense output (lines with an answer ≤ 3 words).
// Gate without the LLM: the answer is ≤ 3 words and appears verbatim in the paragraph text (for multi-paragraph ones — in the batch text).

pub fn qgen(book_dir: &std::path::Path, out: &std::path::Path, stories: usize, per_call: usize) -> Result<()> {
    use prag::opus::{Opus, append_call};
    std::fs::create_dir_all(out)?;
    let rows: Vec<serde_json::Value> = std::fs::read_to_string(book_dir.join("summaries.jsonl"))?.lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
    let mut by_story: Vec<(String, Vec<String>)> = Vec::new();
    for r in &rows {
        let t = r["title"].as_str().unwrap_or("").to_string();
        let p = r["text"].as_str().unwrap_or("").to_string();
        match by_story.last_mut() { Some((tt, ps)) if *tt == t => ps.push(p), _ => by_story.push((t, vec![p])) }
    }
    let opus = Opus::from_env(out.join("cwd"));
    // intermediate dumps: each batch is appended to qa.jsonl immediately; a restart skips stories already present
    let qa_path = out.join("qa.jsonl");
    let done_stories: std::collections::HashSet<String> = std::fs::read_to_string(&qa_path).unwrap_or_default().lines().filter_map(|l| serde_json::from_str::<serde_json::Value>(l).ok()).filter_map(|v| v["story"].as_str().map(String::from)).collect();
    if !done_stories.is_empty() { println!("resuming: questions already exist for {} stories — skipping them", done_stories.len()); }
    let mut kept = String::new();
    let (mut n_all, mut n_ok) = (0, 0);
    let mut batch: Vec<(String, usize, String)> = Vec::new(); // (story, paragraph, text)
    let mut flush = |batch: &mut Vec<(String, usize, String)>, bi: usize, kept: &mut String, n_all: &mut usize, n_ok: &mut usize| -> Result<()> {
        if batch.is_empty() { return Ok(()); }
        let mut p = String::from("Write many SIMPLE reading-comprehension questions for a small model that is learning to read fairy tales.\nFor EACH paragraph write 4-6 questions; then write 3-5 questions that need TWO OR MORE paragraphs of the same story.\nRules: the answer is 1-3 words copied EXACTLY from the text (a name, thing, place, number, verb or short phrase). Questions must be answerable from the text alone.\nAnswer ONLY lines: `P<n> | question | answer` for paragraph n, `M | question | answer` for multi-paragraph.\n\n");
        for (i, (t, _, x)) in batch.iter().enumerate() { p.push_str(&format!("[P{i}] ({t}) {x}\n\n")); }
        let (text, call) = opus.ask("read-qgen", bi, batch.len(), &p)?;
        append_call(&out.join("calls.jsonl"), &call)?;
        let all_text: String = batch.iter().map(|x| x.2.clone()).collect::<Vec<_>>().join(" ").to_lowercase();
        for l in text.lines() {
            let parts: Vec<&str> = l.split('|').map(|x| x.trim()).collect();
            if parts.len() != 3 { continue; }
            *n_all += 1;
            let (tag, q, a) = (parts[0], parts[1], parts[2].trim_end_matches('.'));
            let src = if let Some(n) = tag.strip_prefix('P').and_then(|x| x.parse::<usize>().ok()) { batch.get(n).map(|x| x.2.to_lowercase()) } else if tag == "M" { Some(all_text.clone()) } else { None };
            let Some(src) = src else { continue };
            if a.split_whitespace().count() == 0 || a.split_whitespace().count() > 3 || !src.contains(&a.to_lowercase()) { continue; }
            *n_ok += 1;
            let (story, para) = if let Some(n) = tag.strip_prefix('P').and_then(|x| x.parse::<usize>().ok()) { (batch[n].0.clone(), batch[n].1 as i64) } else { (batch[0].0.clone(), -1) };
            kept.push_str(&serde_json::json!({"story": story, "para": para, "multi": tag == "M", "question": q, "answer": a}).to_string());
            kept.push('\n');
        }
        batch.clear();
        // straight to disk — a crash does not eat finished work
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(out.join("qa.jsonl"))?;
        f.write_all(kept.as_bytes())?;
        kept.clear();
        Ok(())
    };
    let mut bi = 0;
    for (t, ps) in by_story.iter().take(stories).filter(|(t, _)| !done_stories.contains(t)) {
        // a batch stays within one story so multi-paragraph questions make sense
        for (i, p) in ps.iter().enumerate() {
            batch.push((t.clone(), i, p.clone()));
            if batch.len() >= per_call { flush(&mut batch, bi, &mut kept, &mut n_all, &mut n_ok)?; bi += 1; }
        }
        if batch.len() >= per_call / 2 { flush(&mut batch, bi, &mut kept, &mut n_all, &mut n_ok)?; bi += 1; }
    }
    flush(&mut batch, bi, &mut kept, &mut n_all, &mut n_ok)?;
    let cost: f64 = prag::opus::read_calls(&out.join("calls.jsonl")).map(|c| c.iter().map(|x| x.cost_usd).sum()).unwrap_or(0.0);
    println!("questions from the LLM {n_all}, passed the gate (≤ 3 words, verbatim in text) {n_ok}; equiv. ${cost:.2}");
    Ok(())
}

fn norm_ans(s: &str) -> Vec<String> {
    s.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty() && !matches!(*w, "the" | "a" | "an")).map(String::from).collect()
}

/// The snake answers short questions (event graph + choice of sentence constituent); checking without the LLM: exact match and token F1.
pub fn eval_short(book_dir: &std::path::Path, qa_dir: &std::path::Path, show: usize) -> Result<()> {
    let rows: Vec<serde_json::Value> = std::fs::read_to_string(book_dir.join("summaries.jsonl"))?.lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
    let mut stories: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
    for r in &rows { stories.entry(r["title"].as_str().unwrap_or("").to_string()).or_default().push(r["text"].as_str().unwrap_or("").to_string()); }
    let qas: Vec<serde_json::Value> = std::fs::read_to_string(qa_dir.join("qa.jsonl"))?.lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
    let mut prepped: std::collections::HashMap<String, (Story, Vec<Event>)> = std::collections::HashMap::new();
    let (mut em, mut f1s, mut n) = (0.0, 0.0, 0.0);
    let sets_path = qa_dir.join("sets.json");
    let mut sets: std::collections::BTreeMap<String, (Vec<String>, Vec<String>)> = std::fs::read_to_string(&sets_path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    let mut verdicts: std::collections::BTreeMap<&str, usize> = std::collections::BTreeMap::new();
    let mut queue = String::new();
    let mut by: std::collections::BTreeMap<String, (f64, f64, usize)> = std::collections::BTreeMap::new();
    let mut shown = 0;
    for qa in &qas {
        let story = qa["story"].as_str().unwrap_or("");
        let Some(ps) = stories.get(story) else { continue };
        if !prepped.contains_key(story) {
            let sents: Vec<String> = ps.iter().flat_map(|p| crate::quant::sentences(p)).collect();
            let st = prepare_story(sents)?;
            let ev = events_of(&st);
            prepped.insert(story.to_string(), (st, ev));
        }
        let (st, evs) = &prepped[story];
        let q = qa["question"].as_str().unwrap_or("");
        let gold = qa["answer"].as_str().unwrap_or("");
        let Some(i) = best_event(st, evs, q) else { continue };
        let a = crate::tree::annotator()?;
        let qt = Tree::parse(a, q);
        let qc = content(&qt);
        let (ans, _) = extract(&st.trees[evs[i].sent], &qt, &qc, q);
        let ans = shorten(&st.trees[evs[i].sent], evs[i].verb, &qc, q, &ans);
        let (g, h) = (norm_ans(gold), norm_ans(&ans));
        // answer sets: correct / incorrect / unknown → queue for LLM classification
        let key = format!("{story}|{q}");
        let set = sets.entry(key.clone()).or_insert_with(|| (vec![gold.to_string()], Vec::new()));
        let verdict = if set.0.iter().any(|x| norm_ans(x) == h) { "yes" } else if set.1.iter().any(|x| norm_ans(x) == h) { "no" } else { "?" };
        *verdicts.entry(verdict).or_default() += 1;
        if verdict == "?" && !h.is_empty() { queue.push_str(&serde_json::json!({"key": key, "question": q, "gold": gold, "answer": ans}).to_string()); queue.push('\n'); }
        let common = h.iter().filter(|w| g.contains(w)).count() as f64;
        let f1 = if common == 0.0 { 0.0 } else { let p = common / h.len() as f64; let r = common / g.len() as f64; 2.0 * p * r / (p + r) };
        let exact = if g == h { 1.0 } else { 0.0 };
        em += exact; f1s += f1; n += 1.0;
        let key = format!("{:?}{}", qtype(q), if qa["multi"].as_bool() == Some(true) { "·multi" } else { "" });
        let e = by.entry(key).or_default();
        e.0 += exact; e.1 += f1; e.2 += 1;
        if shown < show { shown += 1; println!("  ? {q}\n    snake: {ans}\n    reference: {gold}   F1 {f1:.2}"); }
    }
    println!("short questions: {n} — exact match {:.1}%, F1 {:.3} (without LLM)", 100.0 * em / n, f1s / n);
    println!("by answer sets: yes {}, no {}, unknown {} → queue for LLM classification", verdicts.get("yes").unwrap_or(&0), verdicts.get("no").unwrap_or(&0), verdicts.get("?").unwrap_or(&0));
    std::fs::write(&sets_path, serde_json::to_string(&sets)?)?;
    std::fs::write(qa_dir.join("queue.jsonl"), queue)?;
    for (k, (e, f, c)) in &by { println!("  {k:>18}: EM {:.0}%, F1 {:.3} ({c})", 100.0 * e / *c as f64, f / *c as f64); }
    Ok(())
}


/// Short mode: from the dependent — the verb's noun phrase (object / subject / adverbial) that is not in the question.
fn shorten(t: &Tree, v: usize, qc: &[String], q: &str, ans: &str) -> String {
    if ans.split_whitespace().count() <= 3 { return ans.to_string(); }
    let f = q.to_lowercase();
    let first = f.split_whitespace().next().unwrap_or("");
    let order: &[Rel] = match first { "who" | "whom" => &[Rel::Nsubj, Rel::Obj, Rel::Iobj], "where" => &[Rel::Obl], _ => &[Rel::Obj, Rel::Obl, Rel::Nsubj, Rel::Xcomp] };
    let np = |i: usize| -> String {
        let mut idx = vec![i];
        idx.extend(t.kids(i).iter().copied().filter(|&c| matches!(t.base(c), Rel::Amod | Rel::Compound | Rel::Nummod | Rel::Det) || t.w[c].rel == Rel::NmodPoss));
        idx.sort();
        idx.iter().map(|&j| t.form(j)).collect::<Vec<_>>().join(" ")
    };
    for r in order {
        for &c in t.kids(v) {
            if t.base(c) == *r && !qc.contains(&t.lemma(c).to_string()) {
                return np(c);
            }
        }
    }
    // any noun of the sentence that is not in the question — the one closest to the verb
    (0..t.len()).filter(|&i| matches!(t.w[i].upos, UPos::NOUN | UPos::PROPN | UPos::NUM) && !qc.contains(&t.lemma(i).to_string())).min_by_key(|&i| (i as i64 - v as i64).abs()).map(np).unwrap_or_else(|| ans.to_string())
}

/// The LLM classifies the snake's unknown answers (correct / not) — short output; the answer sets grow.
pub fn classify(qa_dir: &std::path::Path, limit: usize) -> Result<()> {
    use prag::opus::{Opus, append_call};
    let items: Vec<serde_json::Value> = std::fs::read_to_string(qa_dir.join("queue.jsonl"))?.lines().filter_map(|l| serde_json::from_str(l).ok()).take(limit).collect();
    let sets_path = qa_dir.join("sets.json");
    let mut sets: std::collections::BTreeMap<String, (Vec<String>, Vec<String>)> = std::fs::read_to_string(&sets_path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    let opus = Opus::from_env(qa_dir.join("cwd"));
    let (mut yes, mut no) = (0, 0);
    for (bi, ch) in items.chunks(40).enumerate() {
        let mut p = String::from("For each item decide if the SMALL MODEL answer is also a correct answer to the question (same meaning as the reference, may be shorter or longer). Answer ONLY lines `<n> yes` or `<n> no`.\n\n");
        for (i, it) in ch.iter().enumerate() { p.push_str(&format!("{i}. Q: {} | reference: {} | small model: {}\n", it["question"].as_str().unwrap_or(""), it["gold"].as_str().unwrap_or(""), it["answer"].as_str().unwrap_or(""))); }
        let (text, call) = opus.ask("read-classify", bi, ch.len(), &p)?;
        append_call(&qa_dir.join("calls.jsonl"), &call)?;
        for l in text.lines() {
            let mut w = l.split_whitespace();
            let (Some(n), Some(v)) = (w.next().and_then(|x| x.trim_end_matches('.').parse::<usize>().ok()), w.next()) else { continue };
            let Some(it) = ch.get(n) else { continue };
            let key = it["key"].as_str().unwrap_or("").to_string();
            let a = it["answer"].as_str().unwrap_or("").to_string();
            let e = sets.entry(key).or_insert_with(|| (vec![it["gold"].as_str().unwrap_or("").to_string()], Vec::new()));
            if v.starts_with("yes") { e.0.push(a); yes += 1; } else { e.1.push(a); no += 1; }
        }
    }
    std::fs::write(&sets_path, serde_json::to_string(&sets)?)?;
    println!("LLM classified {}: correct alternatives {yes}, incorrect {no}", yes + no);
    Ok(())
}

// ---------- training the snake on short questions ----------

/// Answer candidates in a sentence: noun phrases (with modifiers) and verb phrases; features for each.
fn candidates(t: &Tree, v: usize, qc: &[String], q: &str) -> Vec<(String, Vec<String>)> {
    let qt = qtype(q);
    let np = |i: usize| -> String {
        let mut idx = vec![i];
        idx.extend(t.kids(i).iter().copied().filter(|&c| matches!(t.base(c), Rel::Amod | Rel::Compound | Rel::Nummod | Rel::Det) || t.w[c].rel == Rel::NmodPoss));
        idx.sort();
        idx.iter().map(|&j| t.form(j)).collect::<Vec<_>>().join(" ")
    };
    let mut out = Vec::new();
    for i in 0..t.len() {
        let up = t.w[i].upos;
        if !matches!(up, UPos::NOUN | UPos::PROPN | UPos::NUM | UPos::VERB | UPos::ADJ | UPos::PRON) { continue; }
        let mut texts: Vec<(String, &str)> = Vec::new();
        if up == UPos::VERB {
            texts.push((t.form(i).to_string(), "v"));
            let prt = t.kids(i).iter().find(|&&c| t.w[c].rel == Rel::CompoundPrt).map(|&c| t.form(c).to_string());
            if let Some(p) = &prt { texts.push((format!("{} {p}", t.form(i)), "v+prt")); }
            if let Some(&o) = t.kids(i).iter().find(|&&c| t.base(c) == Rel::Obj) { texts.push((format!("{} {}", t.form(i), np(o)), "v+obj")); }
        } else {
            texts.push((np(i), "np"));
            if np(i) != t.form(i) { texts.push((t.form(i).to_string(), "head")); }
            // «a piece of cheese»: a phrase with «of X»
            if let Some(&n) = t.kids(i).iter().find(|&&c| t.base(c) == Rel::Nmod && t.kids_of(c, Rel::Case).iter().any(|&k| t.lemma(k) == "of")) {
                texts.push((format!("{} of {}", np(i), np(n)), "np+of"));
            }
            // prepositional adverbial: «in the wood»
            if let Some(&c) = t.kids_of(i, Rel::Case).first() { texts.push((format!("{} {}", t.form(c), np(i)), "case+np")); }
        }
        let in_q = qc.contains(&t.lemma(i).to_string());
        let rel = format!("{:?}", t.base(i));
        let dist = (i as i64 - v as i64).abs().min(6);
        let case = t.kids_of(i, Rel::Case).first().map(|&c| t.lemma(c).to_string()).unwrap_or_default();
        let f = vec![
            "bias".to_string(),
            format!("qt_rel:{qt:?}|{rel}"),
            format!("qt_up:{qt:?}|{up:?}"),
            format!("in_q:{in_q}"),
            format!("qt_inq:{qt:?}|{in_q}"),
            format!("head_is_v:{}", t.head(i) == Some(v)),
            format!("qt_headv:{qt:?}|{}", t.head(i) == Some(v)),
            format!("dist:{dist}"),
            format!("qt_case:{qt:?}|{case}"),
        ];
        for (text, kind) in texts {
            let mut g = f.clone();
            g.push(format!("kind:{kind}"));
            g.push(format!("qt_kind:{qt:?}|{kind}"));
            g.push(format!("len:{}", text.split_whitespace().count().min(4)));
            out.push((text, g));
        }
    }
    out
}

#[derive(Default)]
pub struct Extractor { w: std::collections::HashMap<u64, f64> }

fn hq(s: &str) -> u64 { let mut x: u64 = 0xcbf29ce484222325; for b in s.bytes() { x ^= b as u64; x = x.wrapping_mul(0x100000001b3); } x }

impl Extractor {
    fn sc(&self, f: &[String]) -> f64 { f.iter().map(|x| self.w.get(&hq(x)).copied().unwrap_or(0.0)).sum() }
    fn pick<'a>(&self, cs: &'a [(String, Vec<String>)]) -> Option<&'a String> { cs.iter().max_by(|a, b| self.sc(&a.1).partial_cmp(&self.sc(&b.1)).unwrap()).map(|x| &x.0) }
}

/// Training: train/test split by story (every 5th is test); search diagnostics; learned choice vs rules.
pub fn train_short(pairs: &[(std::path::PathBuf, std::path::PathBuf)]) -> Result<()> {
    struct Item { multi: bool, bridge: usize, story: String, q: String, gold: String, cands: Vec<(String, Vec<String>)>, sent_has: bool, rule: String, sfeats: Vec<Vec<(String, f64)>>, spos: Vec<bool>, scands: Vec<Vec<(String, Vec<String>)>> }
    let mut items: Vec<Item> = Vec::new();
    for (book, qa) in pairs {
        let rows: Vec<serde_json::Value> = std::fs::read_to_string(book.join("summaries.jsonl"))?.lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
        let mut stories: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        for r in &rows { stories.entry(r["title"].as_str().unwrap_or("").to_string()).or_default().push(r["text"].as_str().unwrap_or("").to_string()); }
        let Ok(qt) = std::fs::read_to_string(qa.join("qa.jsonl")) else { continue };
        let mut prepped: std::collections::HashMap<String, (Story, Vec<Event>)> = std::collections::HashMap::new();
        for l in qt.lines() {
            let Ok(qa) = serde_json::from_str::<serde_json::Value>(l) else { continue };
            let story = qa["story"].as_str().unwrap_or("").to_string();
            let Some(ps) = stories.get(&story) else { continue };
            if !prepped.contains_key(&story) {
                let st = prepare_story(ps.iter().flat_map(|p| crate::quant::sentences(p)).collect())?;
                let ev = events_of(&st);
                prepped.insert(story.clone(), (st, ev));
            }
            let (st, evs) = &prepped[&story];
            let q = qa["question"].as_str().unwrap_or("").to_string();
            let gold = qa["answer"].as_str().unwrap_or("").to_string();
            let Some(i) = best_event(st, evs, &q) else { continue };
            let a = crate::tree::annotator()?;
            let qtr = Tree::parse(a, &q);
            let qc = content(&qtr);
            let t = &st.trees[evs[i].sent];
            let sent_has = st.sents[evs[i].sent].to_lowercase().contains(&gold.to_lowercase());
            let (r, _) = extract(t, &qtr, &qc, &q);
            let rule = shorten(t, evs[i].verb, &qc, &q, &r);
            let qverbs: Vec<String> = (0..qtr.len()).filter(|&k| qtr.w[k].upos == UPos::VERB).map(|k| qtr.lemma(k).to_string()).collect();
            let qents: Vec<String> = (0..qtr.len()).filter_map(|k| ent_of(&qtr, k, "")).collect();
            let qty = qtype(&q);
            // memory: only the 15 sentences closest by words and the best event's sentence (long tales have hundreds of sentences); no hint from the reference
            let gl = gold.to_lowercase();
            let all_f: Vec<Vec<(String, f64)>> = (0..st.trees.len()).map(|si| sent_feats(st, si, &qc, &qverbs, &qents, qty)).collect();
            let mut order: Vec<usize> = (0..st.trees.len()).collect();
            order.sort_by(|&a, &b| all_f[b][0].1.partial_cmp(&all_f[a][0].1).unwrap());
            let mut keep: Vec<usize> = order.into_iter().take(15).collect();
            if !keep.contains(&evs[i].sent) { keep.push(evs[i].sent); }
            let sfeats: Vec<Vec<(String, f64)>> = keep.iter().map(|&k| all_f[k].clone()).collect();
            drop(all_f);
            let spos: Vec<bool> = keep.iter().map(|&k| st.sents[k].to_lowercase().contains(&gl)).collect();
            let scands: Vec<Vec<(String, Vec<String>)>> = keep.iter().map(|&si| {
                let tt = &st.trees[si];
                let v = (0..tt.len()).find(|&k| tt.w[k].upos == UPos::VERB && qverbs.contains(&tt.lemma(k).to_string())).or_else(|| tt.root()).unwrap_or(0);
                if std::env::var("READ_SPANS").as_deref() == Ok("1") { span_candidates(tt, v, &qc, &q) } else { candidates(tt, v, &qc, &q) }
            }).collect();
            // bridge for multi-paragraph ones: participants of the best sentence extend the question — the second sentence is elsewhere
            let first_k = (0..keep.len()).max_by(|&a, &b| sfeats[a][0].1.partial_cmp(&sfeats[b][0].1).unwrap()).unwrap_or(0);
            let first_i = keep[first_k];
            // bridge via specifics: only rare entities of the first sentence (≤ 3 sentences of the story) — not characters that are everywhere
            let freq = |e: &String| st.ents.iter().filter(|es| es.contains(e)).count();
            let mut qc2 = qc.clone();
            for e in &st.ents[first_i] { if freq(e) <= 3 && !qc2.contains(e) { qc2.push(e.clone()); } }
            let bridge = (0..keep.len()).filter(|&k| k != first_k).max_by(|&a, &b| {
                let sa = sent_feats(st, keep[a], &qc2, &qverbs, &qents, qty)[0].1;
                let sb = sent_feats(st, keep[b], &qc2, &qverbs, &qents, qty)[0].1;
                sa.partial_cmp(&sb).unwrap()
            }).unwrap_or(first_k);
            items.push(Item { multi: qa["multi"].as_bool() == Some(true), bridge, story: format!("{}|{story}", book.display()), q: q.clone(), gold, cands: if std::env::var("READ_SPANS").as_deref() == Ok("1") { span_candidates(t, evs[i].verb, &qc, &q) } else { candidates(t, evs[i].verb, &qc, &q) }, sent_has, rule, sfeats, spos, scands });
        }
    }
    let mut story_ids: Vec<String> = items.iter().map(|x| x.story.clone()).collect();
    story_ids.sort();
    story_ids.dedup();
    let is_test = |s: &str| story_ids.iter().position(|x| x == s).unwrap_or(0) % 5 == 4;
    let matches = |a: &str, g: &str| norm_ans(a) == norm_ans(g);
    let f1 = |a: &str, g: &str| { let (h, gg) = (norm_ans(a), norm_ans(g)); let c = h.iter().filter(|w| gg.contains(w)).count() as f64; if c == 0.0 { 0.0 } else { let p = c / h.len() as f64; let r = c / gg.len() as f64; 2.0 * p * r / (p + r) } };
    let train: Vec<&Item> = items.iter().filter(|x| !is_test(&x.story)).collect();
    let test: Vec<&Item> = items.iter().filter(|x| is_test(&x.story)).collect();
    let hit = |xs: &[&Item]| xs.iter().filter(|x| x.sent_has).count() as f64 / xs.len().max(1) as f64;
    let cand_hit = |xs: &[&Item]| xs.iter().filter(|x| x.cands.iter().any(|c| matches(&c.0, &x.gold))).count() as f64 / xs.len().max(1) as f64;
    println!("questions {} (train {}, test {}); reference in the chosen sentence: {:.0}%; verbatim among candidates: {:.0}%", items.len(), train.len(), test.len(), 100.0 * hit(&test), 100.0 * cand_hit(&test));
    // perceptron: for questions where the correct candidate exists — boost the correct one, weaken the chosen one
    // averaged perceptron (Daumé's trick)
    let mut m = Extractor::default();
    let mut acc: std::collections::HashMap<u64, f64> = std::collections::HashMap::new();
    let mut tt = 0.0;
    for _ in 0..15 {
        for x in &train {
            tt += 1.0;
            let Some(gi) = x.cands.iter().position(|c| matches(&c.0, &x.gold)) else { continue };
            let Some(pi) = x.cands.iter().enumerate().max_by(|a, b| m.sc(&a.1.1).partial_cmp(&m.sc(&b.1.1)).unwrap()).map(|x| x.0) else { continue };
            if pi != gi {
                for (fs, d) in [(&x.cands[gi].1, 1.0), (&x.cands[pi].1, -1.0)] {
                    for f in fs { *m.w.entry(hq(f)).or_default() += d; *acc.entry(hq(f)).or_default() += d * tt; }
                }
            }
        }
    }
    for (k, a) in acc { if let Some(w) = m.w.get_mut(&k) { *w -= a / tt.max(1.0); } }
    let score = |xs: &[&Item], learned: bool| -> (f64, f64) {
        let (mut e, mut f) = (0.0, 0.0);
        for x in xs {
            let a = if learned { m.pick(&x.cands).cloned().unwrap_or_default() } else { x.rule.clone() };
            if matches(&a, &x.gold) { e += 1.0; }
            f += f1(&a, &x.gold);
        }
        (100.0 * e / xs.len().max(1) as f64, f / xs.len().max(1) as f64)
    };
    let (re, rf) = score(&test, false);
    let (le, lf) = score(&test, true);
    println!("test (other tales): rules — EM {re:.1}%, F1 {rf:.3}; learned choice — EM {le:.1}%, F1 {lf:.3}");
    let mut r = Retriever::default();
    let mut racc: std::collections::HashMap<u64, f64> = std::collections::HashMap::new();
    let mut rt = 0.0;
    for _ in 0..10 {
        for x in &train {
            rt += 1.0;
            if !x.spos.iter().any(|&b| b) { continue; }
            let pick = (0..x.sfeats.len()).max_by(|&a, &b| r.sc(&x.sfeats[a]).partial_cmp(&r.sc(&x.sfeats[b])).unwrap()).unwrap_or(0);
            if x.spos[pick] { continue; }
            let gold_i = (0..x.sfeats.len()).filter(|&k| x.spos[k]).max_by(|&a, &b| r.sc(&x.sfeats[a]).partial_cmp(&r.sc(&x.sfeats[b])).unwrap()).unwrap();
            for (fs, d) in [(&x.sfeats[gold_i], 1.0), (&x.sfeats[pick], -1.0)] {
                for (k, v) in fs {
                    let e = r.w.entry(hq(k)).or_insert(if k == "lex" { 1.0 } else { 0.0 });
                    *e += d * v * 0.1;
                    *racc.entry(hq(k)).or_default() += d * v * 0.1 * rt;
                }
            }
        }
    }
    for (k, a) in racc { if let Some(w) = r.w.get_mut(&k) { *w -= a / rt.max(1.0); } }
    let (mut hitn, mut e2, mut f2) = (0.0, 0.0, 0.0);
    for x in &test {
        let pick = (0..x.sfeats.len()).max_by(|&a, &b| r.sc(&x.sfeats[a]).partial_cmp(&r.sc(&x.sfeats[b])).unwrap()).unwrap_or(0);
        if x.spos[pick] { hitn += 1.0; }
        let a = m.pick(&x.scands[pick]).cloned().unwrap_or_default();
        if matches(&a, &x.gold) { e2 += 1.0; }
        f2 += f1(&a, &x.gold);
    }
    // choice over sentence pairs: candidates from the search sentence and the bridge, with a «where from» feature; training on train
    let pair_cands = |x: &Item| -> Vec<(String, Vec<String>)> {
        let pick = (0..x.sfeats.len()).max_by(|&a, &b| r.sc(&x.sfeats[a]).partial_cmp(&r.sc(&x.sfeats[b])).unwrap()).unwrap_or(0);
        let mut out: Vec<(String, Vec<String>)> = x.scands[pick].iter().map(|(t, f)| { let mut g = f.clone(); g.push("src:first".into()); g.push(format!("src_multi:first|{}", x.multi)); (t.clone(), g) }).collect();
        if x.bridge != pick {
            out.extend(x.scands[x.bridge].iter().map(|(t, f)| { let mut g = f.clone(); g.push("src:bridge".into()); g.push(format!("src_multi:bridge|{}", x.multi)); (t.clone(), g) }));
        }
        out
    };
    let mut m2 = Extractor::default();
    let mut acc2: std::collections::HashMap<u64, f64> = std::collections::HashMap::new();
    let mut t2 = 0.0;
    let train_pc: Vec<Vec<(String, Vec<String>)>> = train.iter().map(|x| pair_cands(x)).collect();
    for _ in 0..15 {
        for (x, cs) in train.iter().zip(&train_pc) {
            t2 += 1.0;
            let Some(gi) = cs.iter().position(|c| matches(&c.0, &x.gold)) else { continue };
            let Some(pi) = cs.iter().enumerate().max_by(|a, b| m2.sc(&a.1.1).partial_cmp(&m2.sc(&b.1.1)).unwrap()).map(|x| x.0) else { continue };
            if pi != gi {
                for (fs, d) in [(&cs[gi].1, 1.0), (&cs[pi].1, -1.0)] {
                    for f in fs { *m2.w.entry(hq(f)).or_default() += d; *acc2.entry(hq(f)).or_default() += d * t2; }
                }
            }
        }
    }
    for (k, a) in acc2 { if let Some(w) = m2.w.get_mut(&k) { *w -= a / t2.max(1.0); } }
    let eval_pair = |xs: &[&&Item]| -> (f64, f64) {
        let (mut e, mut f) = (0.0, 0.0);
        for x in xs { let a = m2.pick(&pair_cands(x)).cloned().unwrap_or_default(); if matches(&a, &x.gold) { e += 1.0; } f += f1(&a, &x.gold); }
        (100.0 * e / xs.len().max(1) as f64, f / xs.len().max(1) as f64)
    };
    // pair trigger: pairs only when search is uncertain (small margin between the first and second sentence); threshold — on train
    let margin = |x: &Item| -> f64 {
        let mut v: Vec<f64> = x.sfeats.iter().map(|f| r.sc(f)).collect();
        v.sort_by(|a, b| b.partial_cmp(a).unwrap());
        v.first().copied().unwrap_or(0.0) - v.get(1).copied().unwrap_or(0.0)
    };
    let single = |x: &Item| -> String { let pick = (0..x.sfeats.len()).max_by(|&a, &b| r.sc(&x.sfeats[a]).partial_cmp(&r.sc(&x.sfeats[b])).unwrap()).unwrap_or(0); m.pick(&x.scands[pick]).cloned().unwrap_or_default() };
    let paired = |x: &Item| -> String { m2.pick(&pair_cands(x)).cloned().unwrap_or_default() };
    let mut best_th = f64::NEG_INFINITY;
    let mut best_em = -1.0;
    for th in [f64::NEG_INFINITY, 0.1, 0.25, 0.5, 1.0, 2.0, f64::INFINITY] {
        let em: f64 = train.iter().map(|x| { let a = if margin(x) < th { paired(x) } else { single(x) }; if matches(&a, &x.gold) { 1.0 } else { 0.0 } }).sum();
        if em > best_em { best_em = em; best_th = th; }
    }
    let (mut te, mut tf, mut fired, mut helped, mut harmed) = (0.0, 0.0, 0, 0, 0);
    for x in &test {
        let (s1, p1) = (single(x), paired(x));
        let use_pair = margin(x) < best_th;
        let a = if use_pair { fired += 1; let (o1, o2) = (matches(&s1, &x.gold), matches(&p1, &x.gold)); if o2 && !o1 { helped += 1 } else if o1 && !o2 { harmed += 1 } p1 } else { s1 };
        if matches(&a, &x.gold) { te += 1.0; }
        tf += f1(&a, &x.gold);
    }
    let ntt = test.len().max(1) as f64;
    println!("pair trigger (margin threshold {best_th:.2}, chosen on train): EM {:.1}%, F1 {:.3}; fired {fired} — helpful {helped}, harmful {harmed}", 100.0 * te / ntt, tf / ntt);
    let all_t: Vec<&&Item> = test.iter().collect();
    let mult_t: Vec<&&Item> = test.iter().filter(|x| x.multi).collect();
    let (pe, pf) = eval_pair(&all_t);
    let (me, mf) = eval_pair(&mult_t);
    println!("choice over sentence pairs: all — EM {pe:.1}%, F1 {pf:.3}; multi-paragraph — EM {me:.1}%, F1 {mf:.3}");
    // multi-paragraph separately: the usual chain vs the bridge
    let mt: Vec<&&Item> = test.iter().filter(|x| x.multi).collect();
    if !mt.is_empty() {
        let (mut e1, mut f1a, mut e3, mut f3, mut h1, mut h3) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        for x in &mt {
            let pick = (0..x.sfeats.len()).max_by(|&a, &b| r.sc(&x.sfeats[a]).partial_cmp(&r.sc(&x.sfeats[b])).unwrap()).unwrap_or(0);
            let a1 = m.pick(&x.scands[pick]).cloned().unwrap_or_default();
            // candidates from both sentences — the learned choice decides on its own
            let mut both = x.scands[pick].clone();
            if x.bridge != pick { both.extend(x.scands[x.bridge].iter().cloned()); }
            let a3 = m.pick(&both).cloned().unwrap_or_default();
            if x.spos[pick] { h1 += 1.0; }
            if x.spos[x.bridge] || x.spos[pick] { h3 += 1.0; }
            if matches(&a1, &x.gold) { e1 += 1.0; }
            if matches(&a3, &x.gold) { e3 += 1.0; }
            f1a += f1(&a1, &x.gold);
            f3 += f1(&a3, &x.gold);
        }
        let k = mt.len() as f64;
        println!("multi-paragraph ({}): usual — sentence {:.0}%, EM {:.1}%, F1 {:.3}; first + bridge — in one of the two {:.0}%, EM {:.1}%, F1 {:.3}", mt.len(), 100.0 * h1 / k, 100.0 * e1 / k, f1a / k, 100.0 * h3 / k, 100.0 * e3 / k, f3 / k);
    }
    let nt = test.len().max(1) as f64;
    println!("learned search: reference in the chosen sentence {:.0}% (was {:.0}%); search + choice — EM {:.1}%, F1 {:.3}", 100.0 * hitn / nt, 100.0 * hit(&test), 100.0 * e2 / nt, f2 / nt);
    Ok(())
}


/// Sentence features for the learned search: (name, value). Real-valued — shared-word weight; the rest — flags × question type.
fn sent_feats(st: &Story, si: usize, qc: &[String], qverbs: &[String], qents: &[String], qt: QType) -> Vec<(String, f64)> {
    let n = st.trees.len().max(1) as f64;
    let idf = |w: &str| (n / (1.0 + *st.df.get(w).unwrap_or(&0) as f64)).ln() + 1.0;
    let lex: f64 = qc.iter().filter(|w| st.content[si].contains(w)).map(|w| idf(w)).sum();
    let nb = |k: i64| -> f64 { let j = si as i64 + k; if j < 0 || j as usize >= st.trees.len() { 0.0 } else { qc.iter().filter(|w| st.content[j as usize].contains(w)).map(|w| idf(w)).sum() } };
    let t = &st.trees[si];
    let verb_hit = (0..t.len()).any(|i| t.w[i].upos == UPos::VERB && qverbs.contains(&t.lemma(i).to_string()));
    let ent_hit = st.ents[si].iter().filter(|e| qents.contains(e)).count();
    let has_num = (0..t.len()).any(|i| t.w[i].upos == UPos::NUM);
    let has_loc = (0..t.len()).any(|i| t.base(i) == Rel::Case && matches!(t.lemma(i), "in" | "at" | "on" | "into" | "under" | "near" | "to"));
    let has_quote = st.sents[si].contains('"') || st.sents[si].contains('\u{201c}') || st.sents[si].contains('\'');
    let unmatched = qc.iter().filter(|w| !st.content[si].contains(w)).count() as f64;
    vec![
        ("lex".into(), lex),
        ("lex_prev".into(), nb(-1)),
        ("lex_next".into(), nb(1)),
        ("unmatched".into(), unmatched),
        (format!("verb_hit:{verb_hit}"), 1.0),
        (format!("ents:{}", ent_hit.min(3)), 1.0),
        (format!("qt_num:{qt:?}|{has_num}"), 1.0),
        (format!("qt_loc:{qt:?}|{has_loc}"), 1.0),
        (format!("qt_quote:{qt:?}|{has_quote}"), 1.0),
        (format!("len:{}", (t.len() / 10).min(4)), 1.0),
    ]
}

#[derive(Default)]
pub struct Retriever { w: std::collections::HashMap<u64, f64> }

impl Retriever {
    fn sc(&self, f: &[(String, f64)]) -> f64 { f.iter().map(|(k, v)| self.w.get(&hq(k)).copied().unwrap_or(if k == "lex" { 1.0 } else { 0.0 }) * v).sum() }
}


/// Span candidates: all runs of 1–3 consecutive words (no punctuation); features — whether the phrase is whole, head, role, length, question.
fn span_candidates(t: &Tree, v: usize, qc: &[String], q: &str) -> Vec<(String, Vec<String>)> {
    let qt = qtype(q);
    let mut out = Vec::new();
    let n = t.len();
    for st in 0..n {
        for len in 1..=3usize {
            let en = st + len;
            if en > n { break; }
            if (st..en).any(|k| t.base(k) == Rel::Punct) { break; }
            // span head: the word whose head is outside the span
            let heads: Vec<usize> = (st..en).filter(|&k| t.head(k).is_none_or(|h| h < st || h >= en)).collect();
            let constituent = heads.len() == 1 && {
                // whole phrase: all descendants of the head (no punctuation) are in the span
                let h = heads[0];
                let mut idx = vec![h];
                let mut i = 0;
                while i < idx.len() { let x = idx[i]; idx.extend(t.kids(x).iter().copied().filter(|&c| t.base(c) != Rel::Punct)); i += 1; }
                idx.iter().all(|&k| k >= st && k < en)
            };
            let h = heads.first().copied().unwrap_or(st);
            let text: String = (st..en).map(|k| t.form(k)).collect::<Vec<_>>().join(" ");
            let inq = (st..en).filter(|&k| qc.contains(&t.lemma(k).to_string())).count();
            let rel = format!("{:?}", t.base(h));
            let up = format!("{:?}", t.w[h].upos);
            let first = format!("{:?}", t.w[st].upos);
            let dist = (h as i64 - v as i64).abs().min(6);
            let case = t.kids_of(h, Rel::Case).first().map(|&c| t.lemma(c).to_string()).unwrap_or_default();
            let f = vec![
                "bias".to_string(),
                format!("constituent:{constituent}"),
                format!("heads:{}", heads.len().min(3)),
                format!("qt_rel:{qt:?}|{rel}"),
                format!("qt_up:{qt:?}|{up}"),
                format!("qt_first:{qt:?}|{first}"),
                format!("qt_len:{qt:?}|{len}"),
                format!("qt_const:{qt:?}|{constituent}"),
                format!("inq:{}", inq.min(2)),
                format!("qt_inq:{qt:?}|{}", inq.min(2)),
                format!("dist:{dist}"),
                format!("head_is_v:{}", t.head(h) == Some(v)),
                format!("qt_case:{qt:?}|{case}"),
                format!("rel_len:{rel}|{len}"),
            ];
            out.push((text, f));
        }
    }
    out
}
