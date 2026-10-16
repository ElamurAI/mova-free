//! Meanings read from the pupils' dictionary (`data/dictionary-improved.tsv`): what a word does to a quantity, and the
//! numbers it carries — so the small one takes a word's meaning from its explanation instead of learning every word
//! anew from problems. Two readings, both plain and deterministic:
//!
//! - the operation a word points to, by the cue words of its meaning for a pupil and its sense in the lessons («The
//!   total is the whole amount you get when you add everything up» → +; «To share is to give parts … so each person has
//!   some» → ÷); a word whose cues are split between operations points to none;
//! - the counted facts it states («A week is seven days» → 7 day; «An hour … has sixty minutes» → 60 minute; «(12
//!   inches)» → 12 inch): a number after «is», «has», «of» or an opening bracket, and the noun after it.
//!
//!   world meanings <word …>   — what the dictionary says each word does

use std::collections::BTreeMap;

/// The cue words of each operation, as they stand in explanations.
const CUES: [(char, &[&str]); 4] = [
    ('+', &["add", "adds", "added", "adding", "together", "total", "sum", "plus", "altogether", "join", "joins", "gather", "gets", "receive", "receives", "gain", "more"]),
    ('-', &["subtract", "subtracts", "subtracting", "taking", "difference", "minus", "less", "fewer", "left", "remain", "remains", "lose", "loses", "lost", "away", "spend", "spends", "longer", "gone", "smaller", "bigger", "than"]),
    ('*', &["times", "multiply", "multiplies", "multiplying", "multiplied", "copies", "rows", "per", "rate", "groups", "twice", "double"]),
    ('/', &["share", "shares", "shared", "sharing", "divide", "divides", "dividing", "divided", "split", "equally", "among", "parts", "half", "third", "quarter"]),
];

/// The cues that name an operation outright.
const STRONG: [&str; 26] = ["add", "adds", "added", "adding", "total", "sum", "plus", "subtract", "subtracts", "subtracting", "minus", "difference", "multiply", "multiplies", "multiplying", "multiplied", "times", "share", "shares", "shared", "sharing", "divide", "divides", "dividing", "divided", "equally"];

/// The single words of the register of euphemisms (`data/school/euphemisms/register.tsv`): their records came for the
/// euphemism and must not hide the meaning of the word they are a form of.
fn euphemisms() -> &'static std::collections::BTreeSet<String> {
    static E: std::sync::OnceLock<std::collections::BTreeSet<String>> = std::sync::OnceLock::new();
    E.get_or_init(|| include_str!("../data/school/euphemisms/register.tsv").lines().skip(1).filter_map(|l| l.split('\t').next()).map(|w| w.trim().to_lowercase()).filter(|w| !w.contains(' ')).collect())
}

fn records() -> &'static BTreeMap<String, [String; 6]> {
    static R: std::sync::OnceLock<BTreeMap<String, [String; 6]>> = std::sync::OnceLock::new();
    R.get_or_init(crate::dictionary::improved)
}

/// The plain form of a word as the dictionary keeps it («cookies» → cookie, «shared» → share, «hours» → hour).
pub fn entry_word(word: &str) -> Option<String> {
    let w = word.to_lowercase();
    let r = records();
    if r.contains_key(&w) {
        return Some(w);
    }
    for (suf, add) in [("ies", "y"), ("es", ""), ("s", ""), ("ed", ""), ("ed", "e"), ("d", ""), ("ing", ""), ("ing", "e")] {
        if let Some(b) = w.strip_suffix(suf) {
            let c = format!("{b}{add}");
            // a base of two letters is another word («has» is not «ha», a hectare)
            if c.len() > 2 && r.contains_key(&c) {
                return Some(c);
            }
        }
    }
    None
}

/// The operations of words as the skeleton holds them (`data/skeleton-ops.tsv`, made by `world meanings freeze` from
/// the dictionary at a checked state): the dictionary keeps growing, the operations change only when frozen anew and
/// checked (MOVA_LIVEOPS=1 reads them from the dictionary as it is now).
fn skeleton_ops() -> &'static BTreeMap<String, char> {
    static S: std::sync::OnceLock<BTreeMap<String, char>> = std::sync::OnceLock::new();
    S.get_or_init(|| include_str!("../data/skeleton-ops.tsv").lines().filter_map(|l| { let (w, o) = l.split_once('\t')?; Some((w.to_string(), o.chars().next()?)) }).collect())
}

/// The operation a word points to by its explanation, if one does clearly.
pub fn op_of(word: &str) -> Option<char> {
    if std::env::var("MOVA_LIVEOPS").is_err() && !skeleton_ops().is_empty() {
        let w = word.to_lowercase();
        // «0»: the word has a record and points to no operation
        let found = skeleton_ops().get(&w).copied().or_else(|| {
            [("ies", "y"), ("es", ""), ("s", ""), ("ed", ""), ("ed", "e"), ("d", ""), ("ing", ""), ("ing", "e")].iter().find_map(|(suf, add)| w.strip_suffix(suf).map(|b| format!("{b}{add}")).filter(|c| c.len() > 2).and_then(|c| skeleton_ops().get(&c).copied()))
        });
        return found.filter(|o| *o != '0');
    }
    live_op(word)
}

/// What the dictionary says now (see `op_of`).
fn live_op(word: &str) -> Option<char> {
    static C: std::sync::Mutex<BTreeMap<String, Option<char>>> = std::sync::Mutex::new(BTreeMap::new());
    if let Some(v) = C.lock().unwrap().get(word) {
        return *v;
    }
    // a euphemism's record of an inflected form («harvested» = killed by hunters) must not hide the base's meaning; any
    // other record of the form stands for itself
    let euphemism = |w: &String| euphemisms().contains(w.as_str());
    let v = match entry_word(word) {
        Some(w) if euphemism(&w) => op_in(Some(w)).or_else(|| op_in(base_word(word))),
        other => op_in(other),
    };
    C.lock().unwrap().insert(word.to_string(), v);
    v
}

/// The plain base of an inflected word that has a record of its own («harvested» → harvest).
fn base_word(word: &str) -> Option<String> {
    let w = word.to_lowercase();
    let r = records();
    for (suf, add) in [("ies", "y"), ("es", ""), ("s", ""), ("ed", ""), ("ed", "e"), ("d", ""), ("ing", ""), ("ing", "e")] {
        if let Some(b) = w.strip_suffix(suf) {
            let c = format!("{b}{add}");
            if c.len() > 2 && r.contains_key(&c) {
                return Some(c);
            }
        }
    }
    None
}

fn op_in(entry: Option<String>) -> Option<char> {
    let v = entry.and_then(|w| {
        let rec = &records()[&w];
        // the first sense for a pupil and the opening of the sense in the lessons say what the word does in a problem
        // («the operation + ; also, …»); the rest of a rich entry (other senses, everyday uses) names operations by the way
        let first = |t: &str| t.split(" 2) ").next().unwrap_or("").split([';', '.']).next().unwrap_or("").to_lowercase();
        let toks = |t: &str| t.split(|c: char| !c.is_alphanumeric()).filter(|x| !x.is_empty() && *x != w).map(String::from).collect::<Vec<_>>();
        // all the cues of the opening of the sense in the lessons, and only the operations named outright in the
        // pupil's first sense (its loose words — more, less, left, parts — belong to everyday senses too)
        let (sense, pupil) = (toks(&first(&rec[3])), toks(&first(&rec[2])));
        let mut counts: Vec<(char, usize)> = CUES.iter().map(|(op, cues)| (*op, sense.iter().filter(|x| cues.contains(&x.as_str())).count() + pupil.iter().filter(|x| cues.contains(&x.as_str()) && STRONG.contains(&x.as_str())).count())).collect();
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        (counts[0].1 > 0 && counts[0].1 > counts[1].1).then_some(counts[0].0)
    });
    v
}

/// The counted facts a word's explanation states: (how many, of what — a plain noun, or empty for a bare number).
pub fn facts_of(word: &str) -> Vec<(i64, String)> {
    static C: std::sync::Mutex<BTreeMap<String, Vec<(i64, String)>>> = std::sync::Mutex::new(BTreeMap::new());
    if let Some(v) = C.lock().unwrap().get(word) {
        return v.clone();
    }
    let v = facts_uncached(word);
    C.lock().unwrap().insert(word.to_string(), v.clone());
    v
}

fn facts_uncached(word: &str) -> Vec<(i64, String)> {
    let Some(w) = entry_word(word) else { return vec![] };
    let mut out = Vec::new();
    // only the sentences that speak of the word itself («A week is seven days»), not the numbers of other things
    for raw in records()[&w][2].split(". ") {
        if !raw.split(|c: char| !c.is_alphanumeric()).any(|t| entry_word(t).as_deref() == Some(w.as_str())) {
            continue;
        }
        let sentence = crate::numbers::normalize(raw).0;
        let toks: Vec<String> = sentence.replace('(', " ( ").split(|c: char| !(c.is_alphanumeric() || c == '(')).filter(|x| !x.is_empty()).map(|x| x.to_lowercase()).collect();
        // the fact is said of the word itself: «a week is 7 days», «an hour has 60 minutes», «a dozen (12)»; the word
        // itself may have become its number («a dozen» → «12»)
        for i in 0..toks.len() {
            let named = |t: &String| entry_word(t).as_deref() == Some(w.as_str());
            let is_w = named(&toks[i]) || (toks[i] == "it" && toks[..i].iter().any(named)) || (toks[i].parse::<i64>().is_ok() && i + 1 < toks.len() && matches!(toks[i + 1].as_str(), "is" | "means" | "equals"));
            if is_w && toks.get(i + 1).is_some_and(|x| matches!(x.as_str(), "is" | "has" | "have" | "equals" | "makes" | "(")) {
                facts_in(&toks[..(i + 4).min(toks.len())], i + 1, &mut out);
            }
        }
    }
    out
}

fn facts_in(toks: &[String], from: usize, out: &mut Vec<(i64, String)>) {
    for i in from..toks.len() {
        let Ok(n) = toks[i].parse::<i64>() else { continue };
        if i == 0 || !matches!(toks[i - 1].as_str(), "is" | "has" | "have" | "of" | "(" | "equals" | "makes") || n <= 1 {
            continue;
        }
        // the noun after the number, made singular; none when the sentence ends («A dozen is twelve.»)
        let unit = toks.get(i + 1).filter(|x| x.parse::<i64>().is_err() && !matches!(x.as_str(), "and" | "or" | "of" | "a" | "the" | "it" | "is" | "equal" | "same" | "parts")).map(|x| crate::mentions::stem(x)).unwrap_or_default();
        out.push((n, unit));
    }
}

pub fn main_args(args: &[String]) -> anyhow::Result<()> {
    // `world meanings freeze <out>`: the operation of every word the dictionary explains, as it reads now
    if args.get(1).map(String::as_str) == Some("freeze") {
        let out = args.get(2).cloned().unwrap_or_else(|| "skeleton-ops.tsv".into());
        // every word with a record, with its operation or «0» — so a form never falls through to another word's
        let t: String = records().keys().filter(|w| !w.contains(' ')).map(|w| format!("{w}\t{}\n", live_op(w).unwrap_or('0'))).collect();
        std::fs::write(&out, &t)?;
        println!("{} operations frozen to {out}", t.lines().count());
        return Ok(());
    }
    for w in args.iter().skip(1) {
        println!("{w}: operation {:?}, facts {:?}", op_of(w), facts_of(w));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_dictionary() {
        assert_eq!(op_of("total"), Some('+'));
        assert_eq!(op_of("share"), Some('/'));
        assert_eq!(facts_of("week").first(), Some(&(7, "day".to_string())));
        assert_eq!(facts_of("dozen").first(), Some(&(12, String::new())));
        assert!(facts_of("hour").contains(&(60, "minut".to_string())));
        // the negative control: a word the dictionary does not explain
        assert_eq!(op_of("zorp"), None);
        assert!(facts_of("zorp").is_empty());
    }
}
