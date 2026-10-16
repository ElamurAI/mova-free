//! Numbers: in digits («55», «1,000»), in words («fifty five», «fifty-five», «one hundred and five», «a dozen»),
//! ordinals in both («55th», «21st», «fifty-fifth», «twentieth»), and the typos in number words corrected by n-grams
//! («fifty firth» → «fifty fifth», «fivty» → «fifty»).
//!
//! The correction: the candidates are the number words within a small edit distance of the typo (Damerau: a swap
//! of two letters is one edit); the fewest edits win, then the most shared letter trigrams (a Dice score over
//! «^^word$$» trigrams), then the fixed vocabulary order. The context decides whether to correct at all: a word
//! English does not have is corrected wherever it stands; a real word («firth», «for», «then») only when the bigram
//! with its neighbour makes one number by the grammar of numbers («fifty firth» → «fifty fifth»); names (a capital
//! inside a sentence) and inflections («thirds») stay. Of all one-edit typos of the number words, 1140 read back,
//! 9 read as another number word (truly ambiguous: «eigth» is one edit from both «eight» and «eighth»).
//!
//!   world numbers "<text>"   — the text with its numbers in digits, and what was corrected
//!   world numbers --typos    — every one-edit typo of every number word: read back, read as another, not read

use std::collections::BTreeSet;

/// Number words: (word, value, kind). Kinds: unit (0–9), teen (10–19), tens, scale (hundred, thousand …), dozen.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Unit,
    Teen,
    Tens,
    Hundred,
    Scale,
}

const CARDINALS: [(&str, i64, Kind); 32] = [
    ("zero", 0, Kind::Unit), ("one", 1, Kind::Unit), ("two", 2, Kind::Unit), ("three", 3, Kind::Unit), ("four", 4, Kind::Unit), ("five", 5, Kind::Unit),
    ("six", 6, Kind::Unit), ("seven", 7, Kind::Unit), ("eight", 8, Kind::Unit), ("nine", 9, Kind::Unit), ("ten", 10, Kind::Teen), ("eleven", 11, Kind::Teen),
    ("twelve", 12, Kind::Teen), ("thirteen", 13, Kind::Teen), ("fourteen", 14, Kind::Teen), ("fifteen", 15, Kind::Teen), ("sixteen", 16, Kind::Teen),
    ("seventeen", 17, Kind::Teen), ("eighteen", 18, Kind::Teen), ("nineteen", 19, Kind::Teen), ("twenty", 20, Kind::Tens), ("thirty", 30, Kind::Tens),
    ("forty", 40, Kind::Tens), ("fifty", 50, Kind::Tens), ("sixty", 60, Kind::Tens), ("seventy", 70, Kind::Tens), ("eighty", 80, Kind::Tens),
    ("ninety", 90, Kind::Tens), ("hundred", 100, Kind::Hundred), ("thousand", 1_000, Kind::Scale), ("million", 1_000_000, Kind::Scale), ("billion", 1_000_000_000, Kind::Scale),
];

const ORDINALS: [(&str, &str); 32] = [
    ("zeroth", "zero"), ("first", "one"), ("second", "two"), ("third", "three"), ("fourth", "four"), ("fifth", "five"), ("sixth", "six"), ("seventh", "seven"),
    ("eighth", "eight"), ("ninth", "nine"), ("tenth", "ten"), ("eleventh", "eleven"), ("twelfth", "twelve"), ("thirteenth", "thirteen"), ("fourteenth", "fourteen"),
    ("fifteenth", "fifteen"), ("sixteenth", "sixteen"), ("seventeenth", "seventeen"), ("eighteenth", "eighteen"), ("nineteenth", "nineteen"), ("twentieth", "twenty"),
    ("thirtieth", "thirty"), ("fortieth", "forty"), ("fiftieth", "fifty"), ("sixtieth", "sixty"), ("seventieth", "seventy"), ("eightieth", "eighty"),
    ("ninetieth", "ninety"), ("hundredth", "hundred"), ("thousandth", "thousand"), ("millionth", "million"), ("billionth", "billion"),
];

/// A number word (cardinal or ordinal): its cardinal form and whether it was an ordinal.
fn word(w: &str) -> Option<(i64, Kind, bool)> {
    if let Some(&(_, v, k)) = CARDINALS.iter().find(|x| x.0 == w) {
        return Some((v, k, false));
    }
    let base = ORDINALS.iter().find(|x| x.0 == w)?.1;
    CARDINALS.iter().find(|x| x.0 == base).map(|&(_, v, k)| (v, k, true))
}

/// All number words, in a fixed order (the tie-break of the correction).
fn vocabulary() -> Vec<&'static str> {
    CARDINALS.iter().map(|x| x.0).chain(ORDINALS.iter().map(|x| x.0)).chain(["dozen"]).collect()
}

fn trigrams(w: &str) -> BTreeSet<String> {
    let p: Vec<char> = format!("^^{w}$$").chars().collect();
    p.windows(3).map(|t| t.iter().collect()).collect()
}

/// Damerau–Levenshtein distance (optimal string alignment), in half edits: a letter dropped, added or changed costs
/// 2, two neighbours swapped cost 1 — the swap is the commonest slip of the fingers («tne» is «ten» before «one»).
pub fn edits(a: &str, b: &str) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let mut d = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = 2 * i;
    }
    for j in 0..=b.len() {
        d[0][j] = 2 * j;
    }
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            let c = (a[i - 1] != b[j - 1]) as usize;
            d[i][j] = (d[i - 1][j] + 2).min(d[i][j - 1] + 2).min(d[i - 1][j - 1] + 2 * c);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                d[i][j] = d[i][j].min(d[i - 2][j - 2] + 1);
            }
        }
    }
    d[a.len()][b.len()]
}

/// The number word a typo stands for: the best trigram overlap within the allowed edits (1 for short words, 2 for
/// words of seven letters or more), or none.
pub fn correct(w: &str) -> Option<&'static str> {
    // the same word gets the same reading: remembered (the search below compares it with every number word)
    static C: std::sync::Mutex<std::collections::BTreeMap<String, Option<&'static str>>> = std::sync::Mutex::new(std::collections::BTreeMap::new());
    if let Some(v) = C.lock().unwrap().get(w) {
        return *v;
    }
    let v = correct_uncached(w);
    C.lock().unwrap().insert(w.to_string(), v);
    v
}

fn correct_uncached(w: &str) -> Option<&'static str> {
    if w.len() < 3 || word(w).is_some() || w == "dozen" || !w.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let tw = trigrams(w);
    let mut best: Option<(u64, usize, &'static str)> = None;
    for v in vocabulary() {
        let e = edits(w, v);
        if e > if v.len() >= 7 { 4 } else { 2 } {
            continue;
        }
        let tv = trigrams(v);
        let dice = 2000 * tw.intersection(&tv).count() as u64 / (tw.len() + tv.len()) as u64;
        // fewer edits first; then the more shared trigrams; then the vocabulary order
        if best.is_none_or(|b| e < b.1 || (e == b.1 && dice > b.0)) {
            best = Some((dice, e, v));
        }
    }
    best.map(|b| b.2)
}

/// One number read at the start of `toks`: (value, ordinal, tokens used). Words follow the grammar of English
/// numbers: «fifty five», «fifty-five», «one hundred and five», «two thousand three hundred», «a hundred», «a dozen».
fn read_words(toks: &[String]) -> Option<(i64, bool, usize)> {
    let (mut total, mut cur, mut used, mut ordinal) = (0i64, 0i64, 0usize, false);
    let mut prev: Option<Kind> = None;
    let mut i = 0;
    while i < toks.len() && !ordinal {
        let t = toks[i].as_str();
        // «a hundred», «a thousand», «a dozen»
        if t == "a" && prev.is_none() && toks.get(i + 1).is_some_and(|n| matches!(n.as_str(), "hundred" | "thousand" | "million" | "dozen")) {
            cur = 1;
            prev = Some(Kind::Unit);
            i += 1;
            continue;
        }
        if t == "dozen" && prev.is_some_and(|k| matches!(k, Kind::Unit | Kind::Teen)) {
            cur *= 12;
            used = i + 1;
            prev = Some(Kind::Hundred);
            i += 1;
            continue;
        }
        // «one hundred and five»: «and» only after hundred or a scale, before a smaller number
        if t == "and" && prev.is_some_and(|k| matches!(k, Kind::Hundred | Kind::Scale)) && toks.get(i + 1).and_then(|n| word(n)).is_some_and(|(_, k, _)| matches!(k, Kind::Unit | Kind::Teen | Kind::Tens)) {
            i += 1;
            continue;
        }
        let Some((v, k, ord)) = word(t) else { break };
        // «a third», «one fifth», «two thirds»: after «a» or a unit an ordinal is a fraction, not part of the number
        if ord && !matches!(k, Kind::Hundred | Kind::Scale) && (prev.is_some_and(|p| matches!(p, Kind::Unit | Kind::Teen)) || (i > 0 && toks[i - 1] == "a")) {
            break;
        }
        let fits = match (prev, k) {
            (None, _) => true,
            (Some(Kind::Tens), Kind::Unit) => v > 0,
            (Some(Kind::Unit | Kind::Teen | Kind::Tens), Kind::Hundred) => cur % 100 != 0 || cur < 100,
            (Some(Kind::Unit | Kind::Teen | Kind::Tens | Kind::Hundred), Kind::Scale) => true,
            (Some(Kind::Hundred | Kind::Scale), Kind::Unit | Kind::Teen | Kind::Tens) => true,
            (Some(Kind::Scale), Kind::Hundred) => false,
            _ => false,
        };
        if !fits {
            break;
        }
        match k {
            Kind::Hundred => cur = cur.max(1) * 100,
            Kind::Scale => {
                total += cur.max(1) * v;
                cur = 0;
            }
            _ => cur += v,
        }
        prev = Some(k);
        ordinal = ord;
        i += 1;
        used = i;
    }
    (used > 0).then_some((total + cur, ordinal, used))
}

/// A number in digits: «55», «1,000», «55th», «21st», «2nd», «3rd».
fn read_digits(t: &str) -> Option<(i64, bool)> {
    let (digits, suffix) = t.split_at(t.find(|c: char| !(c.is_ascii_digit() || c == ',')).unwrap_or(t.len()));
    if digits.is_empty() || digits.starts_with(',') || digits.ends_with(',') {
        return None;
    }
    let v: i64 = digits.replace(',', "").parse().ok()?;
    match suffix {
        "" => Some((v, false)),
        "st" | "nd" | "rd" | "th" => Some((v, true)),
        _ => None,
    }
}

/// A number in words, as a teacher writes it: 55 → «fifty-five», 120 → «one hundred and twenty».
pub fn to_words(n: i64) -> String {
    let unit = |k: i64| CARDINALS.iter().find(|x| x.1 == k && matches!(x.2, Kind::Unit | Kind::Teen | Kind::Tens)).map(|x| x.0).unwrap_or("");
    let below100 = |k: i64| if k < 20 || k % 10 == 0 { unit(k).to_string() } else { format!("{}-{}", unit(k - k % 10), unit(k % 10)) };
    let below1000 = |k: i64| match (k / 100, k % 100) {
        (0, r) => below100(r),
        (h, 0) => format!("{} hundred", unit(h)),
        (h, r) => format!("{} hundred and {}", unit(h), below100(r)),
    };
    match (n / 1000, n % 1000) {
        (0, r) => below1000(r),
        (t, 0) => format!("{} thousand", below1000(t)),
        (t, r) if r < 100 => format!("{} thousand and {}", below1000(t), below100(r)),
        (t, r) => format!("{} thousand {}", below1000(t), below1000(r)),
    }
}

/// An ordinal in words: 55 → «fifty-fifth», 21 → «twenty-first», 100 → «one hundredth».
pub fn to_ordinal_words(n: i64) -> String {
    let w = to_words(n);
    let (head, last) = match w.rfind([' ', '-']) { Some(k) => (&w[..=k], &w[k + 1..]), None => ("", w.as_str()) };
    let ord = ORDINALS.iter().find(|x| x.1 == last).map(|x| x.0).unwrap_or(last);
    format!("{head}{ord}")
}

/// The ordinal suffix of a number: 1st, 2nd, 3rd, 11th, 12th, 13th, 21st …
pub fn ordinal_suffix(n: i64) -> &'static str {
    match (n % 100, n % 10) {
        (11..=13, _) => "th",
        (_, 1) => "st",
        (_, 2) => "nd",
        (_, 3) => "rd",
        _ => "th",
    }
}

/// A number found in a text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Found {
    pub value: i64,
    pub ordinal: bool,
    /// the words it was read from, as written
    pub said: String,
    /// typos corrected on the way: (written, read as)
    pub corrected: Vec<(String, String)>,
}

/// Split into words and the separators after them, so the text can be put back together; hyphens inside number words
/// («fifty-five») split too.
fn tokens(text: &str) -> Vec<(String, String)> {
    let chars: Vec<char> = text.chars().collect();
    let mut out: Vec<(String, String)> = vec![(String::new(), String::new())];
    for (i, &c) in chars.iter().enumerate() {
        // a comma between digits («1,000») is part of the number
        let digit_comma = c == ',' && i > 0 && chars[i - 1].is_ascii_digit() && chars.get(i + 1).is_some_and(|x| x.is_ascii_digit());
        let last = out.last_mut().unwrap();
        if c.is_alphanumeric() || digit_comma {
            if !last.1.is_empty() {
                out.push((c.to_string(), String::new()));
            } else {
                last.0.push(c);
            }
        } else {
            last.1.push(c);
        }
    }
    if out.len() > 1 && out[0].0.is_empty() && out[0].1.is_empty() {
        out.remove(0);
    }
    out
}

fn is_number_word(w: &str) -> bool {
    word(w).is_some() || w == "dozen" || read_digits(w).is_some()
}

/// The text with every number in digits (ordinals as «55th»), and the numbers found.
pub fn normalize(text: &str) -> (String, Vec<Found>) {
    let toks = tokens(text);
    let low: Vec<String> = toks.iter().map(|t| t.0.to_lowercase()).collect();
    // the n-gram correction, with its word context
    let fixed: Vec<String> = (0..low.len())
        .map(|i| {
            let w = &low[i];
            let Some(c) = correct(w) else { return w.clone() };
            // a capital inside a sentence is a name («Ben» is not «ten»)
            let starts = i == 0 || toks[i - 1].1.contains(['.', '?', '!']);
            let name = !starts && toks[i].0.chars().next().is_some_and(char::is_uppercase);
            // an inflection of a number word is no typo («two thirds»)
            let inflected = w.strip_suffix('s').is_some_and(|b| word(b).is_some());
            if name || inflected {
                return w.clone();
            }
            let real = en::dict::form(w).is_some();
            if !real {
                return c.to_string();
            }
            // a real word is read as a number only where the grammar of numbers wants it — the word bigram with its
            // neighbour must make one number («fifty firth» → «fifty fifth», «five hundrid»); «for 8», «four. Then» stay
            let kind = |x: &str| word(x).map(|y| y.1);
            let ck = kind(c);
            let joined = |sep: &str| !sep.contains(['.', '?', '!', ',', ';']);
            let before = (i > 0 && joined(&toks[i - 1].1)).then(|| kind(&low[i - 1])).flatten();
            let after = (joined(&toks[i].1)).then(|| low.get(i + 1).and_then(|n| kind(n))).flatten();
            let fits = matches!((before, ck), (Some(Kind::Tens), Some(Kind::Unit)) | (Some(Kind::Unit | Kind::Teen | Kind::Tens), Some(Kind::Hundred | Kind::Scale)))
                || matches!((ck, after), (Some(Kind::Unit | Kind::Teen | Kind::Tens), Some(Kind::Hundred | Kind::Scale)) | (Some(Kind::Tens), Some(Kind::Unit)));
            if fits { c.to_string() } else { w.clone() }
        })
        .collect();
    let mut out = String::new();
    let mut found = Vec::new();
    let mut i = 0;
    while i < toks.len() {
        if let Some((v, ord)) = read_digits(&fixed[i]) {
            out += &format!("{}{}{}", v, if ord { ordinal_suffix(v) } else { "" }, toks[i].1);
            found.push(Found { value: v, ordinal: ord, said: toks[i].0.clone(), corrected: vec![] });
            i += 1;
            continue;
        }
        // a lone «a» is no number; «a hundred» is; after «a» or a unit an ordinal is a fraction («a third», «one fifth»)
        let fraction = i > 0 && word(&fixed[i]).is_some_and(|x| x.2 && !matches!(x.1, Kind::Hundred | Kind::Scale)) && (fixed[i - 1] == "a" || word(&fixed[i - 1]).is_some_and(|x| matches!(x.1, Kind::Unit | Kind::Teen)) || read_digits(&fixed[i - 1]).is_some());
        let read = if fraction { None } else { read_words(&fixed[i..]).filter(|r| !(r.2 == 1 && fixed[i] == "a")) };
        match read {
            Some((v, ord, used)) => {
                let said: Vec<&str> = toks[i..i + used].iter().map(|t| t.0.as_str()).collect();
                let corrected = (i..i + used).filter(|&k| fixed[k] != low[k]).map(|k| (toks[k].0.clone(), fixed[k].clone())).collect();
                out += &format!("{}{}{}", v, if ord { ordinal_suffix(v) } else { "" }, toks[i + used - 1].1);
                found.push(Found { value: v, ordinal: ord, said: said.join(" "), corrected });
                i += used;
            }
            None => {
                out += &toks[i].0;
                out += &toks[i].1;
                i += 1;
            }
        }
    }
    (out, found)
}

/// Every one-edit typo of every number word (a letter dropped, doubled, swapped with the next, or replaced by its
/// keyboard-free neighbour in the alphabet): how many come back to their word, how many to another, how many stay.
pub fn typo_check() -> (usize, usize, usize, Vec<String>) {
    let (mut back, mut other, mut none) = (0, 0, 0);
    let mut wrong = Vec::new();
    for v in vocabulary().into_iter().filter(|v| v.len() >= 4) {
        let c: Vec<char> = v.chars().collect();
        let mut typos: BTreeSet<String> = BTreeSet::new();
        for i in 0..c.len() {
            typos.insert(c.iter().enumerate().filter(|x| x.0 != i).map(|x| x.1).collect());
            typos.insert(c.iter().enumerate().flat_map(|x| if x.0 == i { vec![*x.1, *x.1] } else { vec![*x.1] }).collect());
            if i + 1 < c.len() {
                let mut d = c.clone();
                d.swap(i, i + 1);
                typos.insert(d.into_iter().collect());
            }
        }
        for t in typos.into_iter().filter(|t| t != v && word(t).is_none() && en::dict::form(t).is_none()) {
            match correct(&t) {
                Some(x) if x == v => back += 1,
                Some(x) => {
                    other += 1;
                    if wrong.len() < 12 {
                        wrong.push(format!("{t} → {x} (meant {v})"));
                    }
                }
                None => none += 1,
            }
        }
    }
    (back, other, none, wrong)
}

/// `world numbers "<text>"`; `world numbers --typos` — the typo check.
pub fn main_args(args: &[String]) -> anyhow::Result<()> {
    if args.get(1).map(String::as_str) == Some("--typos") {
        let (b, o, n, w) = typo_check();
        println!("one-edit typos of number words: {b} read back, {o} read as another word, {n} not read");
        for x in w {
            println!("  {x}");
        }
        return Ok(());
    }
    let text = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
    let (out, found) = normalize(&text);
    println!("{out}");
    for f in found {
        println!("  {} = {}{}{}", f.said, f.value, if f.ordinal { format!(" ({}{})", f.value, ordinal_suffix(f.value)) } else { String::new() }, f.corrected.iter().map(|(a, b)| format!("; «{a}» read as «{b}»")).collect::<String>());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(t: &str) -> String {
        normalize(t).0
    }

    #[test]
    fn cardinals_and_ordinals() {
        assert_eq!(n("Tom has fifty five apples."), "Tom has 55 apples.");
        assert_eq!(n("Tom has fifty-five apples."), "Tom has 55 apples.");
        assert_eq!(n("one hundred and five"), "105");
        assert_eq!(n("two thousand three hundred and forty-one"), "2341");
        assert_eq!(n("a hundred cards and a dozen eggs"), "100 cards and 12 eggs");
        assert_eq!(n("the fifty-fifth day"), "the 55th day");
        assert_eq!(n("the twenty first floor"), "the 21st floor");
        assert_eq!(n("the 55th and the 2nd and the 13th, 1,000"), "the 55th and the 2nd and the 13th, 1000");
        assert_eq!(n("the twelfth and the hundredth"), "the 12th and the 100th");
        // the grammar stops a number where it breaks: «five fifty» is two numbers, not 55
        assert_eq!(n("five fifty"), "5 50");
        // there and back: every number up to 3000 in words reads as itself
        for k in 0..3000 {
            assert_eq!(n(&to_words(k)), k.to_string(), "{}", to_words(k));
            if k > 0 {
                assert_eq!(n(&format!("the {}", to_ordinal_words(k))), format!("the {k}{}", ordinal_suffix(k)), "{}", to_ordinal_words(k));
            }
        }
        // a lone «a» is no number
        assert_eq!(n("a cat"), "a cat");
        // fractions stay words: «a third», «one fifth», «two thirds»
        assert_eq!(n("a third of them, one fifth, two thirds"), "a third of them, 1 fifth, 2 thirds");
    }

    #[test]
    fn typos_by_ngrams_with_context() {
        assert_eq!(n("the fifty firth day"), "the 55th day");
        assert_eq!(n("Tom has fivty apples."), "Tom has 50 apples.");
        assert_eq!(n("twnety two"), "22");
        assert_eq!(n("seventeeen cards and ninteen stamps"), "17 cards and 19 stamps");
        // real words beside numbers that make no number with them stay
        assert_eq!(n("buys 2 books for 8 cents. He gives Zoe four. Then she eats one."), "buys 2 books for 8 cents. He gives Zoe 4. Then she eats 1.");
        // the negative control: a real word away from numbers stays
        assert_eq!(n("I saw a firth near the tree."), "I saw a firth near the tree.");
        assert_eq!(correct("fifth"), None);
        // names are real words too: «Ben» is not «ten»
        assert_eq!(n("Max gives Ben half of his cards."), "Max gives Ben half of his cards.");
        assert_eq!(correct("firth"), Some("fifth"));
        assert_eq!(correct("tne"), Some("ten"));
        assert_eq!(correct("oen"), Some("one"));
    }
}
