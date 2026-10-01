//! English tokenizer — a table-driven finite automaton (after a table-driven html_parser.c):
//! byte/char → class, (state × class) → (command, new state). One lookup per char.
//! On top of the automaton — two passes with dictionaries learned from UD: abbreviations with a period ("Mr.",
//! "U.S.") and hyphenated compounds that UD keeps as one token ("e-mail"); clitics
//! ("do|n't", "John|'s") are split off by rule. Each token carries a "space after" bit.

use std::collections::HashSet;

/// Character classes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum C {
    Nul = 0, // end of text
    Sp,      // space
    Le,      // letter
    Di,      // digit
    Ap,      // apostrophe ' ’
    Hy,      // hyphen -
    Pe,      // period .
    Co,      // comma ,
    Cl,      // colon :
    Sl,      // slash /
    At,      // @
    Pu,      // other punctuation (separate token)
    Sy,      // symbols $ % & + = …
}
const CLASSES: usize = 13;

fn class(c: char) -> C {
    match c {
        '\0' => C::Nul,
        c if c.is_whitespace() => C::Sp,
        '\'' | '’' => C::Ap,
        '-' => C::Hy,
        '.' => C::Pe,
        ',' => C::Co,
        ':' => C::Cl,
        '/' => C::Sl,
        '@' => C::At,
        c if c.is_ascii_digit() => C::Di,
        c if c.is_alphabetic() || matches!(c, '\u{0300}'..='\u{036F}') => C::Le,
        '$' | '%' | '&' | '+' | '=' | '<' | '>' | '^' | '~' | '|' | '\\' | '°' | '€' | '£' | '#' | '*' => C::Sy,
        _ => C::Pu,
    }
}

/// Automaton states.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum S {
    St = 0, // between tokens
    Wo,     // in a word
    Nu,     // in a number
    Wj,     // in a word after a joiner (' - / :) — the next char decides
    Wd,     // word + period: kept together; abbreviations ("Mr.", "U.S.") are decided by the dictionary after the automaton
    Nj,     // in a number after a joiner (. , : /)
    Pp,     // repeated periods/hyphens ("...", "--")
    Ur,     // URL/email: after @ or "://" — up to a space
}
const STATES: usize = 8;

/// Commands.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum K {
    App,  // append the char to the current token
    New,  // close the current one, start a new one with the char
    One,  // close the current one, the char is a separate token, state St
    Sep,  // close the current one (space after), skip the char
    End,  // close the current one, end
    Back, // joiner not confirmed: cut it off as a separate token, the char is handled as from St
}

use K::*;
use S::*;

/// Table (state × class) → (command, state). Rows are states, columns are classes in enum C order.
#[rustfmt::skip]
const TABLE: [[(K, S); CLASSES]; STATES] = [
    //        Nul        Sp         Le         Di         Ap         Hy         Pe         Co         Cl         Sl         At         Pu         Sy
    /* St */ [(End, St), (Sep, St), (New, Wo), (New, Nu), (New, Wo), (New, Pp), (New, Pp), (One, St), (One, St), (One, St), (One, St), (One, St), (One, St)],
    /* Wo */ [(End, St), (Sep, St), (App, Wo), (App, Wo), (App, Wj), (App, Wj), (App, Wd), (One, St), (App, Wj), (App, Wj), (App, Ur), (One, St), (One, St)],
    /* Nu */ [(End, St), (Sep, St), (App, Wo), (App, Nu), (App, Wj), (App, Wj), (App, Nj), (App, Nj), (App, Nj), (App, Nj), (One, St), (One, St), (One, St)],
    /* Wj */ [(Back,St), (Back,St), (App, Wo), (App, Wo), (Back,St), (Back,St), (Back,St), (Back,St), (Back,St), (App, Wj), (Back,St), (Back,St), (Back,St)],
    /* Wd */ [(End, St), (Sep, St), (App, Wo), (App, Wo), (One, St), (One, St), (Back,St), (One, St), (One, St), (One, St), (One, St), (One, St), (One, St)],
    /* Nj */ [(Back,St), (Back,St), (Back,St), (App, Nu), (Back,St), (Back,St), (Back,St), (Back,St), (Back,St), (Back,St), (Back,St), (Back,St), (Back,St)],
    /* Pp */ [(End, St), (Sep, St), (New, Wo), (New, Nu), (New, Wo), (App, Pp), (App, Pp), (One, St), (One, St), (One, St), (One, St), (One, St), (One, St)],
    /* Ur */ [(End, St), (Sep, St), (App, Ur), (App, Ur), (App, Ur), (App, Ur), (App, Ur), (App, Ur), (App, Ur), (App, Ur), (App, Ur), (App, Ur), (App, Ur)],
];

#[derive(Clone, Debug, PartialEq)]
pub struct Tok {
    pub form: String,
    pub space_after: bool,
}

/// Dictionaries learned from UD train: abbreviations with a period, hyphenated words (whole or split),
/// words starting with an apostrophe ("'s", "'em", "'90s"), suffixes that stick to numbers ("1990s", "2nd").
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Lexicon {
    pub abbrev: HashSet<String>,
    pub hyphen_words: HashSet<String>,
    pub hyphen_split: HashSet<String>,
    /// Majority in train: hyphenated words are more often whole.
    pub hyphen_keep_default: bool,
    pub apos_initial: HashSet<String>,
    pub num_suffix: HashSet<String>,
    /// Multi-char non-alphabetic tokens from train ("...", "!!!", ":)", ":D", "--").
    pub multi_punct: HashSet<String>,
    /// Prefix of a hyphenated word → (whole, split) counts in train: "anti-", "non-", "e-" are mostly whole.
    pub hyphen_prefix: std::collections::HashMap<String, (u32, u32)>,
}

impl Lexicon {
    pub fn learn(sents: &[crate::conllu::Sentence]) -> Lexicon {
        let mut lx = Lexicon::default();
        let (mut kept, mut split) = (0usize, 0usize);
        for s in sents {
            let t = &s.tokens;
            for (i, tok) in t.iter().enumerate() {
                let f = tok.form.as_str();
                let chars: Vec<char> = f.chars().collect();
                if chars.len() > 1 && f.ends_with('.') && chars.iter().any(|c| c.is_alphabetic()) {
                    lx.abbrev.insert(f.to_lowercase());
                }
                if chars.len() > 2 && f.contains('-') && chars.iter().filter(|c| c.is_alphanumeric()).count() >= 2 && !f.starts_with('-') && !f.ends_with('-') {
                    lx.hyphen_words.insert(f.to_lowercase());
                    kept += 1;
                    let pre = f.split('-').next().unwrap_or("").to_lowercase();
                    lx.hyphen_prefix.entry(pre).or_default().0 += 1;
                }
                // split: word · - · word without spaces
                if f == "-" && i > 0 && i + 1 < t.len() && !t[i - 1].space_after && !tok.space_after
                    && t[i - 1].form.chars().any(char::is_alphanumeric) && t[i + 1].form.chars().any(char::is_alphanumeric)
                {
                    lx.hyphen_split.insert(format!("{}-{}", t[i - 1].form, t[i + 1].form).to_lowercase());
                    split += 1;
                    lx.hyphen_prefix.entry(t[i - 1].form.to_lowercase()).or_default().1 += 1;
                }
                if chars.len() > 1 && !chars[0].is_alphanumeric() && chars.iter().filter(|c| c.is_alphanumeric()).count() <= 1 {
                    lx.multi_punct.insert(f.to_string());
                }
                if (f.starts_with('\'') || f.starts_with('’')) && chars.len() > 1 {
                    lx.apos_initial.insert(f.replace('’', "'").to_lowercase());
                }
                let digits: usize = chars.iter().take_while(|c| c.is_ascii_digit()).count();
                if digits > 0 && digits < chars.len() && chars[digits..].iter().all(|c| c.is_alphabetic()) {
                    lx.num_suffix.insert(chars[digits..].iter().collect::<String>().to_lowercase());
                }
            }
        }
        lx.hyphen_keep_default = kept >= split;
        lx
    }
}

/// Automaton: text → primary tokens.
fn fsa(text: &str) -> Vec<Tok> {
    let chars: Vec<char> = text.chars().chain(std::iter::once('\0')).collect();
    let mut out: Vec<Tok> = Vec::new();
    let mut cur = String::new();
    let mut state = St;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let (cmd, next) = TABLE[state as usize][class(c) as usize];
        match cmd {
            App => cur.push(c),
            New => {
                flush(&mut out, &mut cur, false);
                cur.push(c);
            }
            One => {
                flush(&mut out, &mut cur, false);
                out.push(Tok { form: c.to_string(), space_after: false });
            }
            Sep => {
                flush(&mut out, &mut cur, true);
                if let Some(last) = out.last_mut() {
                    last.space_after = true;
                }
            }
            End => flush(&mut out, &mut cur, false),
            Back => {
                // the last char of the current token is an unconfirmed joiner ("word." "10," "it'")
                let j = cur.pop();
                flush(&mut out, &mut cur, false);
                if let Some(j) = j {
                    cur.push(j);
                    // the joiner becomes a token itself; the current char is reprocessed from state Pp/St
                    state = if matches!(j, '.' | '-') { Pp } else { St };
                    if state == St {
                        flush(&mut out, &mut cur, false);
                    }
                }
                continue; // the same char again, from the new state
            }
        }
        state = next;
        i += 1;
    }
    out
}

/// Names and descriptions of character classes (in enum `C` order) — for explanations.
pub const CLASS_INFO: [(&str, &str); CLASSES] = [
    ("Nul", "end of text"),
    ("Sp", "space"),
    ("Le", "letter"),
    ("Di", "digit"),
    ("Ap", "apostrophe"),
    ("Hy", "hyphen"),
    ("Pe", "period"),
    ("Co", "comma"),
    ("Cl", "colon"),
    ("Sl", "slash"),
    ("At", "@"),
    ("Pu", "other punctuation"),
    ("Sy", "symbol ($ % & + = …)"),
];

/// Names and descriptions of states (in enum `S` order).
pub const STATE_INFO: [(&str, &str); STATES] = [
    ("St", "between tokens"),
    ("Wo", "in a word"),
    ("Nu", "in a number"),
    ("Wj", "in a word after a joiner (' - / :) — the next char decides"),
    ("Wd", "word + period (the dictionary decides abbreviations)"),
    ("Nj", "in a number after a joiner (. , : /)"),
    ("Pp", "repeated periods or hyphens"),
    ("Ur", "URL or email — up to a space"),
];

/// Names and descriptions of commands (in enum `K` order).
pub const CMD_INFO: [(&str, &str); 6] = [
    ("App", "append the char to the current token"),
    ("New", "close the current one, start a new one with the char"),
    ("One", "close the current one, the char is a separate token"),
    ("Sep", "space: close the current one"),
    ("End", "end of text: close the current one"),
    ("Back", "joiner not confirmed: cut it off"),
];

/// Automaton table as numbers: [state][class] → (command, new state).
pub fn fsa_table() -> Vec<Vec<(usize, usize)>> {
    TABLE.iter().map(|row| row.iter().map(|&(k, s)| (k as usize, s as usize)).collect()).collect()
}

/// Automaton step: char, its class, state before, command, state after (numbers from the tables above).
#[derive(Clone, Copy, Debug)]
pub struct FsaStep {
    pub ch: char,
    pub class: usize,
    pub state: usize,
    pub cmd: usize,
    pub next: usize,
}

/// Automaton trace for explanations.
pub fn trace(text: &str) -> Vec<FsaStep> {
    let mut out = Vec::new();
    let mut state = St;
    for ch in text.chars() {
        let c = class(ch);
        let (cmd, next) = TABLE[state as usize][c as usize];
        out.push(FsaStep { ch, class: c as usize, state: state as usize, cmd: cmd as usize, next: next as usize });
        // "Back" repeats the char from the new state — for the trace it is enough to show the decision
        state = if cmd == Back { St } else { next };
    }
    out
}

fn flush(out: &mut Vec<Tok>, cur: &mut String, _space: bool) {
    if !cur.is_empty() {
        out.push(Tok { form: std::mem::take(cur), space_after: false });
    }
}

const CLITICS: [&str; 7] = ["n't", "'s", "'re", "'ve", "'ll", "'d", "'m"];

/// Full tokenizer: automaton + abbreviations + clitics + hyphens by dictionary.
pub fn tokenize(text: &str, lx: &Lexicon) -> Vec<Tok> {
    let mut out = Vec::new();
    for t in fsa(text) {
        let lower = t.form.to_lowercase();
        // period at the end of a word: keep abbreviations, otherwise cut it off
        if t.form.len() > 1 && t.form.ends_with('.') && !t.form.ends_with("..") && !lx.abbrev.contains(&lower) {
            let stem = &t.form[..t.form.len() - 1];
            push_word(&mut out, stem, false, lx);
            out.push(Tok { form: ".".into(), space_after: t.space_after });
            continue;
        }
        push_word(&mut out, &t.form, t.space_after, lx);
    }
    merge_punct(out, lx)
}

/// Merges adjacent tokens with no space between them if together they form a multi-char token known from train
/// (":)", "...", ":D") or a repeat of the same sign ("!!!", "??").
fn merge_punct(toks: Vec<Tok>, lx: &Lexicon) -> Vec<Tok> {
    let mut out: Vec<Tok> = Vec::with_capacity(toks.len());
    for t in toks {
        if let Some(last) = out.last_mut() {
            if !last.space_after {
                let joined = format!("{}{}", last.form, t.form);
                let repeat = t.form.chars().count() == 1
                    && matches!(t.form.chars().next(), Some('!' | '?'))
                    && last.form.chars().all(|c| matches!(c, '!' | '?'));
                if repeat || (!last.form.chars().any(char::is_alphanumeric) && lx.multi_punct.contains(&joined)) {
                    last.form = joined;
                    last.space_after = t.space_after;
                    continue;
                }
            }
        }
        out.push(t);
    }
    out
}

/// Word → clitics and hyphenated parts.
fn push_word(out: &mut Vec<Tok>, w: &str, space_after: bool, lx: &Lexicon) {
    let norm: String = w.replace('’', "'");
    let lower = norm.to_lowercase();
    // leading apostrophe: "'yuk" — a quote mark if train does not know such a word ("'s", "'em", "'90s")
    if (w.starts_with('\'') || w.starts_with('’')) && w.chars().count() > 1 && !lx.apos_initial.contains(&lower)
        && !CLITICS.contains(&lower.as_str())
    {
        let first = w.chars().next().map_or(1, char::len_utf8);
        out.push(Tok { form: w[..first].to_string(), space_after: false });
        push_word(out, &w[first..], space_after, lx);
        return;
    }
    // number with a letter suffix: "221bn" → 221 · bn, but "1990s", "2nd" stay whole (suffixes from train)
    let digits = w.chars().take_while(|c| c.is_ascii_digit()).count();
    if digits > 0 && digits < w.chars().count() && w.chars().skip(digits).all(char::is_alphabetic) {
        let suffix = lower[digits..].to_string();
        if !lx.num_suffix.contains(&suffix) {
            out.push(Tok { form: w[..digits].to_string(), space_after: false });
            out.push(Tok { form: w[digits..].to_string(), space_after });
            return;
        }
    }
    for cl in CLITICS {
        if lower.len() > cl.len() && lower.ends_with(cl) {
            let cut = w.len() - w.chars().rev().take(cl.chars().count()).map(char::len_utf8).sum::<usize>();
            let (stem, tail) = w.split_at(cut);
            if stem.chars().any(|c| c.is_alphabetic()) {
                push_word(out, stem, false, lx);
                out.push(Tok { form: tail.to_string(), space_after });
                return;
            }
        }
    }
    // "word's"-like without a known clitic, or "rock'n'roll" — keep whole
    let split_hyphen = if lx.hyphen_words.contains(&lower) {
        false
    } else if lx.hyphen_split.contains(&lower) {
        true
    } else if let Some((k, sp)) = lx.hyphen_prefix.get(lower.split('-').next().unwrap_or("")) {
        sp > k
    } else {
        !lx.hyphen_keep_default
    };
    if w.contains('-') && split_hyphen && w.chars().any(|c| c.is_alphabetic()) {
        let parts: Vec<&str> = w.split('-').collect();
        if parts.iter().all(|p| !p.is_empty()) {
            for (k, p) in parts.iter().enumerate() {
                out.push(Tok { form: (*p).to_string(), space_after: false });
                if k + 1 < parts.len() {
                    out.push(Tok { form: "-".into(), space_after: false });
                }
            }
            if let Some(last) = out.last_mut() {
                last.space_after = space_after;
            }
            return;
        }
    }
    out.push(Tok { form: w.to_string(), space_after });
}

/// Text back from tokens (for the reverse path).
pub fn detokenize(toks: &[Tok]) -> String {
    let mut s = String::new();
    for t in toks {
        s.push_str(&t.form);
        if t.space_after {
            s.push(' ');
        }
    }
    s.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn forms(text: &str, lx: &Lexicon) -> Vec<String> {
        tokenize(text, lx).into_iter().map(|t| t.form).collect()
    }

    #[test]
    fn basics() {
        let mut lx = Lexicon::default();
        lx.abbrev.insert("mr.".into());
        lx.abbrev.insert("u.s.".into());
        assert_eq!(forms("Mr. Smith didn't go to the U.S. today.", &lx), ["Mr.", "Smith", "did", "n't", "go", "to", "the", "U.S.", "today", "."]);
        assert_eq!(forms("It costs $3.50, i.e. 10% more...", &lx), ["It", "costs", "$", "3.50", ",", "i.e", ".", "10", "%", "more", "..."]);
        assert_eq!(forms("John's search-engine (and e-mail) wares?", &lx), ["John", "'s", "search", "-", "engine", "(", "and", "e", "-", "mail", ")", "wares", "?"]);
        lx.hyphen_words.insert("e-mail".into());
        assert_eq!(forms("my e-mail at a@b.com now", &lx), ["my", "e-mail", "at", "a@b.com", "now"]);
        assert_eq!(detokenize(&tokenize("Hello, world! It's 10:30.", &lx)), "Hello, world! It's 10:30.");
    }

    #[test]
    fn combining_marks_stay_in_word() {
        // stress mark U+0301 and other combining marks are part of the word (the GPT-2 regex breaks abugidas and stress marks)
        let lx = Lexicon::default();
        assert_eq!(forms("Mama\u{301} kno\u{301}ws café\u{301}.", &lx), ["Mama\u{301}", "kno\u{301}ws", "café\u{301}", "."]);
    }
}
