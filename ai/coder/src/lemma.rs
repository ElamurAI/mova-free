//! Parsing the task description: words → our lemmas (`en::dict`: analysis of the form with the highest UD frequency), numbers, literals
//! in quotes and brackets, printf formats, code in backticks.

use std::collections::BTreeMap;

pub fn lemma(word: &str) -> String {
    let lower = word.to_lowercase();
    if let Some(f) = en::dict::form(&lower) {
        if let Some(r) = en::dict::analyses(f).max_by_key(|r| r.count) {
            let l = en::dict::text(r.lemma());
            if !l.is_empty() {
                return l.to_lowercase();
            }
        }
    }
    lower
}

const STOP: &[&str] = &[
    "the", "a", "an", "of", "to", "and", "or", "in", "on", "with", "for", "is", "be", "it", "its", "that", "this", "as", "by",
    "at", "from", "then", "use", "using", "each", "print", "printf", "program", "write", "result", "value", "given", "which",
    "are", "all", "into", "one", "line", "own", "format", "exactly", "their", "them", "your", "not", "no",
];

/// Lemmas of content words (no stop words; "each", "print" etc. stay in `cues`).
pub fn lemmas(text: &str) -> Vec<String> {
    words(text).iter().map(|w| lemma(w)).filter(|l| l.len() > 1 && !STOP.contains(&l.as_str())).collect()
}

/// All lemmas, with stop words (for the cues "each", "first", "how many").
pub fn cues(text: &str) -> Vec<String> {
    words(text).iter().map(|w| lemma(w)).collect()
}

pub fn words(text: &str) -> Vec<String> {
    let stripped = strip_literals(text);
    stripped
        .split(|c: char| !(c.is_alphabetic() || c == '\''))
        .map(|w| w.trim_matches('\''))
        .filter(|w| !w.is_empty() && w.chars().any(char::is_alphabetic))
        .map(str::to_string)
        .collect()
}

/// Text without literals in quotes, brackets and backticks (they are data, not words).
pub fn strip_literals(text: &str) -> String {
    let mut out = String::new();
    let mut depth = 0i32;
    let mut in_q = false;
    let mut in_bt = false;
    let cs: Vec<char> = text.chars().collect();
    for (i, &c) in cs.iter().enumerate() {
        if in_bt {
            if c == '`' {
                in_bt = false;
            }
            continue;
        }
        if c == '`' {
            in_bt = true;
            out.push(' ');
            continue;
        }
        if in_q {
            if c == '\'' {
                in_q = false;
            }
            continue;
        }
        // apostrophe as a quote: no letter before it (it's stays a word)
        if c == '\'' && (i == 0 || !cs[i - 1].is_alphanumeric()) {
            in_q = true;
            out.push(' ');
            continue;
        }
        if c == '[' {
            depth += 1;
            continue;
        }
        if c == ']' {
            depth -= 1;
            out.push(' ');
            continue;
        }
        if depth > 0 {
            continue;
        }
        out.push(c);
    }
    out
}

/// Literals of the description.
#[derive(Debug, Default, Clone)]
pub struct Literals {
    /// strings in single quotes without "%"
    pub strings: Vec<String>,
    /// printf formats (in quotes, with "%")
    pub formats: Vec<String>,
    /// vectors and matrices in square brackets (as written)
    pub brackets: Vec<String>,
    /// numbers outside quotes and brackets, with the preceding word
    pub numbers: Vec<(f64, String, String)>,
    /// code in backticks
    pub code: Vec<String>,
    /// printf/fprintf/disp calls written in the description
    pub prints: Vec<String>,
    /// assignments "name = literal" from the description
    pub named: BTreeMap<String, String>,
}

pub fn literals(text: &str) -> Literals {
    let mut lit = Literals::default();
    let cs: Vec<char> = text.chars().collect();
    let n = cs.len();
    let mut i = 0;
    while i < n {
        let c = cs[i];
        if c == '`' {
            let j = (i + 1..n).find(|&j| cs[j] == '`').unwrap_or(n);
            lit.code.push(cs[i + 1..j].iter().collect());
            i = j + 1;
            continue;
        }
        if c == '\'' && (i == 0 || !cs[i - 1].is_alphanumeric()) {
            let j = (i + 1..n).find(|&j| cs[j] == '\'' && !(j + 1 < n && cs[j + 1] == '\'')).unwrap_or(n);
            let s: String = cs[i + 1..j.min(n)].iter().collect();
            if s.contains('%') {
                lit.formats.push(s);
            } else {
                lit.strings.push(s);
            }
            i = j + 1;
            continue;
        }
        if c == '[' {
            let mut d = 0;
            let mut j = i;
            while j < n {
                if cs[j] == '[' {
                    d += 1;
                }
                if cs[j] == ']' {
                    d -= 1;
                    if d == 0 {
                        break;
                    }
                }
                j += 1;
            }
            lit.brackets.push(cs[i..(j + 1).min(n)].iter().collect());
            i = j + 1;
            continue;
        }
        if c.is_ascii_digit() || (c == '-' && i + 1 < n && cs[i + 1].is_ascii_digit() && (i == 0 || cs[i - 1] == ' ')) {
            // a number, unless part of a word
            if i > 0 && (cs[i - 1].is_alphabetic() || cs[i - 1] == '_') {
                i += 1;
                continue;
            }
            let mut j = i + 1;
            while j < n && (cs[j].is_ascii_digit() || (cs[j] == '.' && j + 1 < n && cs[j + 1].is_ascii_digit()) || cs[j] == 'e' && j + 1 < n && cs[j + 1].is_ascii_digit()) {
                j += 1;
            }
            let s: String = cs[i..j].iter().collect();
            if let Ok(v) = s.parse::<f64>() {
                let before: String = cs[..i].iter().collect();
                let prev = before.split_whitespace().last().unwrap_or("").trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
                lit.numbers.push((v, s, prev));
            }
            i = j;
            continue;
        }
        i += 1;
    }
    // print calls and assignments in the description
    for key in ["printf(", "fprintf(", "disp("] {
        let mut from = 0;
        while let Some(p) = text[from..].find(key) {
            let s = from + p;
            if s > 0 && text[..s].ends_with(|c: char| c.is_alphanumeric()) {
                from = s + key.len();
                continue;
            }
            let mut d = 0;
            let mut end = s;
            let mut inq = false;
            for (k, ch) in text[s..].char_indices() {
                if ch == '\'' {
                    inq = !inq;
                }
                if inq {
                    continue;
                }
                if ch == '(' {
                    d += 1;
                }
                if ch == ')' {
                    d -= 1;
                    if d == 0 {
                        end = s + k + 1;
                        break;
                    }
                }
            }
            if end > s {
                lit.prints.push(text[s..end].to_string());
            }
            from = s + key.len();
        }
    }
    // "x = [..]", "s = '..'", "n = 10" in the description text
    if LITERAL_REPAIR.load(std::sync::atomic::Ordering::Relaxed) {
        named_balanced(&strip_code(text), &mut lit.named);
    } else {
        named_simple(&strip_code(text), &mut lit.named);
    }
    lit
}

/// Text without code in backticks.
fn strip_code(text: &str) -> String {
    let mut out = String::new();
    let mut bt = false;
    for c in text.chars() {
        if c == '`' {
            bt = !bt;
            continue;
        }
        if !bt {
            out.push(c);
        }
    }
    out
}

/// Architect's change b2 (round 2): literals with bracket balancing (matrices with ";"), names with digits and "_".
pub static LITERAL_REPAIR: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// v1: assignments from the description, text split at ";", "," and brackets (matrices with ";" get truncated — a flaw found by the architect).
fn named_simple(flat: &str, named: &mut BTreeMap<String, String>) {
    for part in flat.split([',', ';', '(', ')']) {
        if let Some((l, r)) = part.split_once('=') {
            let name = l.split_whitespace().last().unwrap_or("");
            let rhs = r.trim().trim_end_matches('.').trim();
            if !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') && name.chars().next().is_some_and(char::is_alphabetic) && !rhs.is_empty() {
                let rhs_first: String = if rhs.starts_with('[') {
                    rhs.chars().scan(0i32, |d, c| {
                        if *d < 0 {
                            return None;
                        }
                        if c == '[' {
                            *d += 1;
                        }
                        if c == ']' {
                            *d -= 1;
                            if *d == 0 {
                                *d = -1;
                                return Some(c);
                            }
                        }
                        Some(c)
                    })
                    .collect()
                } else if rhs.starts_with('\'') {
                    let e = rhs[1..].find('\'').map(|e| e + 2).unwrap_or(rhs.len());
                    rhs[..e].to_string()
                } else {
                    rhs.split_whitespace().next().unwrap_or("").trim_end_matches(['.', ',']).to_string()
                };
                if !rhs_first.is_empty() && name.len() <= 12 {
                    named.entry(name.to_string()).or_insert(rhs_first);
                }
            }
        }
    }
}

/// b2: scanning "name = …" with balancing of square brackets and quotes.
fn named_balanced(flat: &str, named: &mut BTreeMap<String, String>) {
    let cs: Vec<char> = flat.chars().collect();
    let n = cs.len();
    for i in 0..n {
        if cs[i] != '=' {
            continue;
        }
        let prev = if i > 0 { cs[i - 1] } else { ' ' };
        let next = if i + 1 < n { cs[i + 1] } else { ' ' };
        if matches!(prev, '=' | '<' | '>' | '~' | '!') || next == '=' {
            continue;
        }
        // name before "="
        let mut j = i;
        while j > 0 && cs[j - 1] == ' ' {
            j -= 1;
        }
        let e = j;
        while j > 0 && (cs[j - 1].is_ascii_alphanumeric() || cs[j - 1] == '_') {
            j -= 1;
        }
        let name: String = cs[j..e].iter().collect();
        if name.is_empty() || !name.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) || name.len() > 16 {
            continue;
        }
        // right-hand side
        let mut k = i + 1;
        while k < n && cs[k] == ' ' {
            k += 1;
        }
        if k >= n {
            continue;
        }
        let rhs: String = if cs[k] == '[' {
            let mut d = 0;
            let mut m = k;
            while m < n {
                if cs[m] == '[' {
                    d += 1;
                }
                if cs[m] == ']' {
                    d -= 1;
                    if d == 0 {
                        break;
                    }
                }
                m += 1;
            }
            if m >= n {
                continue;
            }
            cs[k..=m].iter().collect()
        } else if cs[k] == '\'' {
            match (k + 1..n).find(|&m| cs[m] == '\'' && !(m + 1 < n && cs[m + 1] == '\'')) {
                Some(m) => cs[k..=m].iter().collect(),
                None => continue,
            }
        } else {
            let t: String = cs[k..].iter().take_while(|c| c.is_ascii_digit() || **c == '.' || **c == '-').collect();
            let t = t.trim_end_matches('.').to_string();
            if t.parse::<f64>().is_err() {
                continue;
            }
            t
        };
        named.entry(name).or_insert(rhs);
    }
}
