//! The Hebrew trees of the site's Tanakh pages (MACULA's, right to left) written from the verse and a lexicon
//! learned on the training pages (`site-learn`, `hebrew.tsv`), instead of stored:
//! - **written words**: each space-separated chunk of the verse (`# text`) becomes MACULA's written tokens by the
//!   commonest cut the training pages give it, else by rule (after a maqaf; ׃, ׀ and a final פ or ס apart);
//! - **words**: each written token is cut into its words with the commonest analysis the training pages give that
//!   token (each word's form, lemma, UPOS, XPOS, FEATS, relation, head inside the token, MACULA's Gloss, Strong and
//!   Type, the token line's MISC), else the analysis of its letters without accents, cut at the same letters, else one
//!   word of unknown lemma;
//! - **heads** outside the token: the commonest offset of its UPOS and relation in the training pages;
//! - **MISC**: Gloss, Strong, Type and SpaceAfter from the analysis, Ref and MId from the verse and the token's number
//!   (MId's book is its place among the 39 books); Frame is stored.
//! What still differs is a patch per field (per FEATS pair, per MISC key), with its bits, as for English trees.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use crate::class::{lit_bits, pos_bits};

/// One word of an analysis.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AWord {
    pub form: String,
    pub lemma: String,
    pub upos: String,
    pub xpos: String,
    pub feats: String,
    pub rel: String,
    /// the head inside the token (1-based within the token), or none
    pub inner: Option<usize>,
    pub gloss: String,
    pub strong: String,
    pub typ: String,
    pub space_no: bool,
}

/// A written token's analysis: its words and its range line's MISC.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Analysis {
    pub words: Vec<AWord>,
    pub range_misc: String,
}

#[derive(Clone, Debug, Default)]
pub struct Lexicon {
    /// chunk of the text → written tokens
    pub cuts: BTreeMap<String, Vec<String>>,
    /// written token → analysis; its letters without accents → analysis
    pub tokens: BTreeMap<String, Analysis>,
    pub bare: BTreeMap<String, Analysis>,
    /// (UPOS, relation) → offset of the head in words
    pub offsets: BTreeMap<(String, String), i64>,
    /// (UPOS, relation, the next word's UPOS) → offset of the head, where the training pages saw it at least 3 times
    pub offsets_next: BTreeMap<(String, String, String), i64>,
    /// a one-letter word that opens tokens (ו, ה, ב, ל, כ, מ, ש): its letter → the word as the training pages give it
    pub prefixes: BTreeMap<char, AWord>,
    /// a token's first letter with its vowels («וּ», «הַ») → (tokens it opened as a word of its own, tokens it opened)
    pub prefix_odds: BTreeMap<String, (u32, u32)>,
    /// a pronoun written at a token's end («ם», «הוּ»), letters and vowels → its word, and (tokens it ended as a word,
    /// tokens it ended)
    pub suffixes: BTreeMap<String, (AWord, u32, u32)>,
    /// a first letter with its vowel that hides the article («בַּ» = בְּ + הַ): the article's word, and in how many
    /// tokens it did
    pub hidden_det: BTreeMap<String, (AWord, u32)>,
}

/// The letters of a form.
fn n_letters(s: &str) -> usize {
    s.chars().filter(|c| is_letter(*c)).count()
}

/// The first letter of a form with the vowels after it (accents off), and the rest.
fn first_group(b: &str) -> Option<(String, String)> {
    let cs: Vec<char> = b.chars().collect();
    let f = cs.iter().position(|c| is_letter(*c))?;
    let mut k = f + 1;
    while k < cs.len() && !is_letter(cs[k]) {
        k += 1;
    }
    Some((cs[..k].iter().collect(), cs[k..].iter().collect()))
}

/// How often a first letter opens a token as a word of its own, and the pronouns written at tokens' ends.
fn affixes_of(bare_lex: &BTreeMap<String, Analysis>) -> (BTreeMap<String, (u32, u32)>, BTreeMap<String, (AWord, u32, u32)>, BTreeMap<String, (AWord, u32)>) {
    let mut pre: BTreeMap<String, (u32, u32)> = BTreeMap::new();
    let mut hidden: BTreeMap<String, (AWord, u32)> = BTreeMap::new();
    let mut suf: BTreeMap<String, (AWord, u32)> = BTreeMap::new();
    for (b, a) in bare_lex {
        let Some((g, _)) = first_group(b) else { continue };
        let e = pre.entry(g.clone()).or_default();
        e.1 += 1;
        if a.words.len() >= 2 && bare(&a.words[0].form) == g {
            e.0 += 1;
            // the article the vowel hides: a second word whose letter the token does not write
            let written = n_letters(b);
            let all: usize = a.words.iter().map(|w| n_letters(&w.form)).sum();
            if a.words.len() >= 3 && a.words[1].upos == "DET" && all > written {
                hidden.entry(g.clone()).or_insert((a.words[1].clone(), 0)).1 += 1;
            }
        }
        if a.words.len() >= 2 {
            let w = a.words.last().unwrap();
            let f = bare(&w.form);
            if w.upos == "PRON" && (1..=3).contains(&f.chars().filter(|c| is_letter(*c)).count()) && b.ends_with(&f) {
                suf.entry(f).or_insert((w.clone(), 0)).1 += 1;
            }
        }
    }
    // how many tokens end with each pronoun at all
    let suffixes = suf
        .into_iter()
        .map(|(f, (w, yes))| {
            let total = bare_lex.keys().filter(|k| k.ends_with(&f) && *k != &f).count() as u32;
            (f, (w, yes, total.max(yes)))
        })
        .collect();
    (pre, suffixes, hidden)
}

/// A token the training pages never gave, cut by its affixes (unknown Hebrew words resolved from what a lexicon
/// knows: adler-2008 «unsupervised lexicon-based resolution of unknown words»; prefixes and pronominal suffixes as
/// morphemes: adler-2006, zeldes-2018): one-letter words in front where that letter with
/// that vowel mostly stands alone (at most two), a pronoun at the end where it mostly does, the stem between as a
/// word of unknown lemma; the affixes hang on the stem. None when it has no affix.
fn segment(lex: &Lexicon, t: &str) -> Option<Analysis> {
    let mut rest = bare(t);
    let mut front: Vec<AWord> = Vec::new();
    while front.len() < 2 {
        let Some((g, r)) = first_group(&rest) else { break };
        let letter = g.chars().find(|c| is_letter(*c))?;
        let (Some(p), Some(&(yes, total))) = (lex.prefixes.get(&letter), lex.prefix_odds.get(&g)) else { break };
        if yes * 2 <= total || total < 5 || r.chars().filter(|c| is_letter(*c)).count() < 2 {
            break;
        }
        front.push(AWord { form: g.clone(), ..p.clone() });
        if lex.hidden_det.get(&g).is_some_and(|h| h.1 * 2 > yes) {
            let mut d = lex.hidden_det[&g].0.clone();
            d.form = "ה".into();
            front.push(d);
        }
        rest = r;
    }
    let mut back: Option<AWord> = None;
    let mut best: Option<(&String, &(AWord, u32, u32))> = None;
    for (f, v) in &lex.suffixes {
        let longer = best.map_or(true, |b| f.chars().count() > b.0.chars().count());
        if rest.ends_with(f.as_str()) && v.1 * 2 > v.2 && v.2 >= 5 && longer && rest.chars().filter(|c| is_letter(*c)).count() >= f.chars().filter(|c| is_letter(*c)).count() + 2 {
            best = Some((f, v));
        }
    }
    if let Some((f, v)) = best {
        rest = rest[..rest.len() - f.len()].to_string();
        back = Some(AWord { form: f.clone(), ..v.0.clone() });
    }
    if front.is_empty() && back.is_none() {
        return None;
    }
    let stem = front.len() + 1;
    let mut words: Vec<AWord> = front.into_iter().map(|w| AWord { inner: Some(stem), space_no: false, ..w }).collect();
    // the stem's lemma guessed as its letters and vowels (`MOVA_RT_LEMMA=0`: unknown, for the A/B)
    let lemma = if std::env::var("MOVA_RT_LEMMA").as_deref() == Ok("0") { "_".to_string() } else { rest.chars().filter(|c| *c != '\u{05BD}').collect() };
    words.push(AWord { form: rest, lemma, upos: "NOUN".into(), xpos: "_".into(), feats: "_".into(), rel: "dep".into(), ..Default::default() });
    if let Some(w) = back {
        words.push(AWord { inner: Some(stem), ..w });
    }
    // the forms as written, accents with their letters; a hidden article keeps its «ה», writing no letter of the token
    let hid = |w: &AWord| w.upos == "DET" && w.form == "ה";
    let shown = Analysis { words: words.iter().filter(|w| !hid(w)).cloned().collect(), range_misc: String::new() };
    let mut forms: Vec<String> = cut_like(t, &shown);
    // a pronoun takes the vowel before its letters as MACULA writes it («ָם»): the end of the token whose letters and
    // vowels are the pronoun's, the stem the rest
    if let (Some(sf), true) = (words.last().filter(|w| w.upos == "PRON").map(|w| w.form.clone()), forms.len() >= 2) {
        let n = forms.len();
        let joined: String = forms[n - 2..].concat();
        let cs: Vec<char> = joined.chars().collect();
        if let Some(k) = (0..cs.len()).rev().find(|&k| bare(&cs[k..].iter().collect::<String>()) == sf) {
            forms[n - 2] = cs[..k].iter().collect();
            forms[n - 1] = cs[k..].iter().collect();
        }
    }
    let mut forms = forms.into_iter();
    for w in words.iter_mut().filter(|w| !hid(w)) {
        w.form = forms.next().unwrap_or_default();
    }
    Some(Analysis { words, range_misc: "_".into() })
}

/// The one-letter words that open the training tokens of two words or more, by letter: the commonest analysis.
fn prefixes_of(bare: &BTreeMap<String, Analysis>) -> BTreeMap<char, AWord> {
    let mut by: BTreeMap<char, BTreeMap<String, (u32, AWord)>> = BTreeMap::new();
    for a in bare.values().filter(|a| a.words.len() >= 2) {
        let w = &a.words[0];
        let letters: Vec<char> = w.form.chars().filter(|c| is_letter(*c)).collect();
        if letters.len() == 1 {
            let k = format!("{}|{}|{}|{}|{}", w.lemma, w.upos, w.xpos, w.feats, w.rel);
            by.entry(letters[0]).or_default().entry(k).or_insert((0, w.clone())).0 += 1;
        }
    }
    by.into_iter().filter_map(|(c, m)| m.into_values().fold(None::<(u32, AWord)>, |acc, x| match acc { Some(a) if a.0 >= x.0 => Some(a), _ => Some(x) }).map(|(_, w)| (c, w))).collect()
}

/// A token the training pages never gave, as a known one-letter word before a known token («וּבְ» + «בַּיִת»), at
/// most two such letters deep: the words of both, the inner heads moved; the letter's word hangs on the rest's head.
fn compose(lex: &Lexicon, t: &str, depth: usize) -> Option<Analysis> {
    if depth == 0 {
        return None;
    }
    let chars: Vec<char> = t.chars().collect();
    let first = chars.iter().position(|c| is_letter(*c))?;
    let p = lex.prefixes.get(&chars[first])?;
    // the letter with its vowels and accents, then the rest
    let mut k = first + 1;
    while k < chars.len() && !is_letter(chars[k]) {
        k += 1;
    }
    let (head, rest): (String, String) = (chars[..k].iter().collect(), chars[k..].iter().collect());
    if !rest.chars().any(is_letter) {
        return None;
    }
    let a = match lex.tokens.get(&rest) {
        Some(a) => a.clone(),
        None => match lex.bare.get(&bare(&rest)) {
            Some(a) => {
                let forms = cut_like(&rest, a);
                let mut a = a.clone();
                for (w, f) in a.words.iter_mut().zip(forms) {
                    w.form = f;
                }
                a
            }
            None => compose(lex, &rest, depth - 1)?,
        },
    };
    let root = a.words.iter().position(|w| w.inner.is_none()).unwrap_or(0) + 2;
    let mut words = vec![AWord { form: head, inner: Some(root), space_no: false, ..p.clone() }];
    words.extend(a.words.into_iter().map(|mut w| {
        w.inner = w.inner.map(|x| x + 1);
        w
    }));
    Some(Analysis { words, range_misc: a.range_misc })
}

pub static HEB_LEX: OnceLock<Lexicon> = OnceLock::new();

/// The books in the order MACULA numbers them (MId).
const BOOKS: [&str; 39] = [
    "GEN", "EXO", "LEV", "NUM", "DEU", "JOS", "JDG", "RUT", "1SA", "2SA", "1KI", "2KI", "1CH", "2CH", "EZR", "NEH", "EST", "JOB", "PSA", "PRO", "ECC", "SNG", "ISA", "JER", "LAM", "EZK", "DAN", "HOS", "JOL", "AMO",
    "OBA", "JON", "MIC", "NAM", "HAB", "ZEP", "HAG", "ZEC", "MAL",
];

fn is_letter(c: char) -> bool {
    ('\u{05D0}'..='\u{05EA}').contains(&c)
}

fn is_accent(c: char) -> bool {
    matches!(c, '\u{0591}'..='\u{05AF}' | '\u{05BD}' | '\u{05BF}' | '\u{05C4}' | '\u{05C5}')
}

/// The letters and vowels of a form, accents off.
pub fn bare(s: &str) -> String {
    s.chars().filter(|c| !is_accent(*c)).collect()
}

/// A token's cut by rule: after a maqaf, and the marks ׃ ׀ apart, a final פ or ס apart.
fn rule_cut(chunk: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    for c in chunk.chars() {
        match c {
            '\u{05BE}' => {
                cur.push(c);
                out.push(std::mem::take(&mut cur));
            }
            '׃' | '׀' => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
                out.push(c.to_string());
            }
            _ => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    // a final פ or ס after ׃ is a token of its own
    if let Some(last) = out.last().cloned() {
        if (last == "פ" || last == "ס") && out.len() == 1 {
            return out;
        }
    }
    out
}

/// The forms of a token cut into words at the same letters as an analysis: vowels stay with their letter, accents
/// after a word's last letter go to the next word.
fn cut_like(token: &str, a: &Analysis) -> Vec<String> {
    let counts: Vec<usize> = a.words.iter().map(|w| w.form.chars().filter(|c| is_letter(*c)).count()).collect();
    let chars: Vec<char> = token.chars().collect();
    let mut out = Vec::new();
    let mut i = 0;
    for (k, &n) in counts.iter().enumerate() {
        let mut s = String::new();
        let mut seen = 0;
        while i < chars.len() {
            let c = chars[i];
            if is_letter(c) {
                if seen == n && k + 1 < counts.len() {
                    break;
                }
                seen += 1;
            } else if seen == n && k + 1 < counts.len() && is_accent(c) {
                break;
            }
            s.push(c);
            i += 1;
        }
        out.push(s);
    }
    out
}

/// A predicted row of the tree: its ten columns (ID and HEAD as numbers in the sentence).
pub type Row = [String; 10];

/// The rows the lexicon writes for a verse (`# text`) and its id (`GEN.1.1`): range lines and word lines.
pub fn predict(lex: &Lexicon, text: &str, verse: &str) -> Vec<Row> {
    predict_with(lex, text, verse, &BTreeMap::new())
}

/// The written tokens the lexicon cuts a verse into.
pub fn tokens(lex: &Lexicon, text: &str) -> Vec<String> {
    text.split_whitespace().flat_map(|c| lex.cuts.get(c).cloned().unwrap_or_else(|| rule_cut(c))).collect()
}

/// The same, with the analysis of some tokens given (by the token's number): a token the lexicon cuts into another
/// number of words than the page.
pub fn predict_with(lex: &Lexicon, text: &str, verse: &str, given: &BTreeMap<usize, Analysis>) -> Vec<Row> {
    let toks = tokens(lex, text);
    let mut vs = verse.split('.');
    let (b, c, v) = (vs.next().unwrap_or(""), vs.next().unwrap_or("0"), vs.next().unwrap_or("0"));
    let bn = BOOKS.iter().position(|x| *x == b).map_or(0, |i| i + 1);
    let (cn, vn): (u32, u32) = (c.parse().unwrap_or(0), v.parse().unwrap_or(0));
    // the words of every token
    let mut words: Vec<(usize, AWord)> = Vec::new();
    let mut ranges: Vec<(usize, usize, String, String)> = Vec::new();
    for (ti, t) in toks.iter().enumerate() {
        let an = match given.get(&ti).or_else(|| lex.tokens.get(t)) {
            Some(a) => a.clone(),
            None => match lex.bare.get(&bare(t)) {
                Some(a) => {
                    let forms = cut_like(t, a);
                    let mut a = a.clone();
                    for (w, f) in a.words.iter_mut().zip(forms) {
                        w.form = f;
                    }
                    a
                }
                None => compose(lex, t, 2).or_else(|| segment(lex, t)).unwrap_or_else(|| Analysis { words: vec![AWord { form: t.clone(), lemma: "_".into(), upos: if t.chars().any(is_letter) { "NOUN".into() } else { "PUNCT".into() }, xpos: "_".into(), feats: "_".into(), rel: "dep".into(), ..Default::default() }], range_misc: "_".into() }),
            },
        };
        let start = words.len() + 1;
        if an.words.len() > 1 {
            ranges.push((start, start + an.words.len() - 1, t.clone(), an.range_misc.clone()));
        }
        for w in an.words {
            words.push((ti, w));
        }
    }
    // the token numbers of Ref and MId: punctuation takes the number before it
    let mut num = vec![0u32; toks.len()];
    let mut n = 0;
    for ti in 0..toks.len() {
        let punct = words.iter().filter(|(t, _)| *t == ti).all(|(_, w)| w.upos == "PUNCT");
        if !punct {
            n += 1;
        }
        num[ti] = n.max(1);
    }
    let nw = words.len();
    let mut out: Vec<Row> = Vec::new();
    let mut ri = 0;
    let mut sub = 0u32;
    let mut last_t = usize::MAX;
    for (i, (ti, w)) in words.iter().enumerate() {
        let id = i + 1;
        if ri < ranges.len() && ranges[ri].0 == id {
            let r = &ranges[ri];
            out.push([format!("{}-{}", r.0, r.1), r.2.clone(), "_".into(), "_".into(), "_".into(), "_".into(), "_".into(), "_".into(), "_".into(), r.3.clone()]);
            ri += 1;
        }
        if *ti != last_t {
            sub = 0;
            last_t = *ti;
        }
        sub += 1;
        let first = words.iter().position(|(t, _)| t == ti).unwrap_or(i);
        let head = match w.inner {
            Some(k) => first + k,
            None => {
                let nx = words.get(i + 1).map(|(_, x)| x.upos.clone()).unwrap_or_else(|| "END".into());
                let off = lex.offsets_next.get(&(w.upos.clone(), w.rel.clone(), nx)).or_else(|| lex.offsets.get(&(w.upos.clone(), w.rel.clone()))).copied().unwrap_or(0);
                if off == 0 { 0 } else { (id as i64 + off).clamp(0, nw as i64) as usize }
            }
        };
        let mut misc: Vec<String> = Vec::new();
        if !w.gloss.is_empty() || w.upos != "PUNCT" {
            misc.push(format!("Gloss={}", w.gloss));
        }
        misc.push(format!("Ref={b}_{cn}:{vn}!{}", num[*ti]));
        if w.upos != "PUNCT" {
            misc.push(format!("MId=o{bn:02}{cn:03}{vn:03}{:03}{sub}", num[*ti]));
        }
        if !w.strong.is_empty() {
            misc.push(format!("Strong={}", w.strong));
        }
        if !w.typ.is_empty() {
            misc.push(format!("Type={}", w.typ));
        }
        if w.space_no {
            misc.push("SpaceAfter=No".into());
        }
        out.push([id.to_string(), w.form.clone(), w.lemma.clone(), w.upos.clone(), w.xpos.clone(), w.feats.clone(), head.to_string(), w.rel.clone(), "_".into(), misc.join("|")]);
    }
    out
}

/// The keys of MACULA's MISC in the order it writes them.
const MISC_ORDER: [&str; 7] = ["Gloss", "Ref", "MId", "Strong", "Frame", "Type", "SpaceAfter"];

fn kv(s: &str) -> Vec<(String, String)> {
    if s == "_" || s.is_empty() {
        return Vec::new();
    }
    // a value keeps its «=» (a key may stand alone, or have an empty value)
    s.split('|').map(|x| x.split_once('=').map(|(a, b)| (a.to_string(), format!("={b}"))).unwrap_or((x.to_string(), String::new()))).collect()
}

fn canonical(m: &[(String, String)]) -> bool {
    let ix: Vec<usize> = m.iter().map(|(k, _)| MISC_ORDER.iter().position(|x| x == k).unwrap_or(99)).collect();
    ix.windows(2).all(|w| w[0] < w[1]) && !ix.contains(&99)
}

/// A patch of one field: the new value (FEATS and MISC: per pair, `None` removes).
#[derive(Clone, Debug, PartialEq)]
pub enum Patch {
    Field(usize, String),
    Pairs(usize, Vec<(String, Option<String>)>),
}

/// The patches from a predicted row to the page's, and their bits (n: the words of the verse).
pub fn diff(pred: &Row, lit: &[&str], n: usize) -> (Vec<Patch>, f64, Vec<&'static str>) {
    let mut out = Vec::new();
    let mut bits = 0.0;
    let mut kinds = Vec::new();
    for j in 0..10 {
        let (a, b) = (pred[j].as_str(), lit[j]);
        if a == b {
            continue;
        }
        match j {
            5 | 9 => {
                let (pa, pb) = if j == 5 { (feats_kv(a), feats_kv(b)) } else { (kv(a), kv(b)) };
                let ok_order = if j == 9 { canonical(&pb) } else { pb.windows(2).all(|w| w[0].0.to_lowercase() <= w[1].0.to_lowercase()) };
                if !ok_order {
                    out.push(Patch::Field(j, b.to_string()));
                    bits += lit_bits(b);
                    kinds.push(if j == 5 { "feats" } else { "misc" });
                    continue;
                }
                let ma: BTreeMap<&str, &str> = pa.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                let mb: BTreeMap<&str, &str> = pb.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
                let mut ops = Vec::new();
                for (k, v) in &mb {
                    if ma.get(k) != Some(v) {
                        ops.push((k.to_string(), Some(v.to_string())));
                        bits += if j == 5 { 7.0 } else { 3.0 + lit_bits(v) };
                        kinds.push(if j == 5 { "feats" } else { misc_kind(k) });
                    }
                }
                for k in ma.keys() {
                    if !mb.contains_key(k) {
                        ops.push((k.to_string(), None));
                        bits += if j == 5 { 7.0 } else { 3.0 };
                        kinds.push(if j == 5 { "feats" } else { misc_kind(k) });
                    }
                }
                out.push(Patch::Pairs(j, ops));
            }
            _ => {
                out.push(Patch::Field(j, b.to_string()));
                bits += match j {
                    3 => 5.0,
                    4 | 7 => 6.0,
                    6 => pos_bits(n),
                    _ => lit_bits(b),
                };
                kinds.push(["id", "form", "lemma", "upos", "xpos", "feats", "head", "deprel", "deps", "misc"][j]);
            }
        }
    }
    if !out.is_empty() {
        bits += pos_bits(n);
    }
    (out, bits, kinds)
}

fn misc_kind(k: &str) -> &'static str {
    match k {
        "Gloss" => "misc Gloss",
        "Ref" => "misc Ref",
        "MId" => "misc MId",
        "Strong" => "misc Strong",
        "Frame" => "misc Frame",
        "Type" => "misc Type",
        "SpaceAfter" => "misc SpaceAfter",
        _ => "misc other",
    }
}

fn feats_kv(s: &str) -> Vec<(String, String)> {
    kv(s)
}

/// Apply patches to a predicted row.
pub fn apply(pred: &Row, ps: &[Patch]) -> Row {
    let mut r = pred.clone();
    for p in ps {
        match p {
            Patch::Field(j, v) => r[*j] = v.clone(),
            Patch::Pairs(j, ops) => {
                let mut m: BTreeMap<String, String> = kv(&r[*j]).into_iter().collect();
                for (k, v) in ops {
                    match v {
                        Some(v) => {
                            m.insert(k.clone(), v.clone());
                        }
                        None => {
                            m.remove(k);
                        }
                    }
                }
                let mut v: Vec<(String, String)> = m.into_iter().collect();
                if *j == 9 {
                    v.sort_by_key(|(k, _)| MISC_ORDER.iter().position(|x| x == k).unwrap_or(99));
                } else {
                    v.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()).then(a.0.cmp(&b.0)));
                }
                r[*j] = if v.is_empty() { "_".into() } else { v.iter().map(|(k, x)| format!("{k}{x}")).collect::<Vec<_>>().join("|") };
            }
        }
    }
    r
}

// ---------------------------------------------------------------- learning

/// The analysis of a written token from its rows (its range line's MISC and its word rows).
pub fn analysis_of(range_misc: &str, ws: &[Vec<String>]) -> Analysis {
    let first: usize = ws.first().and_then(|w| w[0].parse().ok()).unwrap_or(1);
    let words = ws
        .iter()
        .map(|w| {
            let m: BTreeMap<String, String> = kv(&w[9]).into_iter().map(|(k, v)| (k, v.strip_prefix('=').unwrap_or(&v).to_string())).collect();
            let head: usize = w[6].parse().unwrap_or(0);
            let inner = (head >= first && head < first + ws.len()).then(|| head + 1 - first);
            AWord { form: w[1].clone(), lemma: w[2].clone(), upos: w[3].clone(), xpos: w[4].clone(), feats: w[5].clone(), rel: w[7].clone(), inner, gloss: m.get("Gloss").cloned().unwrap_or_default(), strong: m.get("Strong").cloned().unwrap_or_default(), typ: m.get("Type").cloned().unwrap_or_default(), space_no: m.get("SpaceAfter").is_some_and(|v| v == "No") }
        })
        .collect();
    Analysis { words, range_misc: range_misc.to_string() }
}

/// The written tokens of a tree's rows: (form, range line's MISC, word rows).
pub fn tokens_of(rows: &[Vec<String>]) -> Vec<(String, String, Vec<Vec<String>>)> {
    let mut toks = Vec::new();
    let mut i = 0;
    while i < rows.len() {
        let r = &rows[i];
        if let Some((a, b)) = r[0].split_once('-') {
            let (a, b): (usize, usize) = (a.parse().unwrap_or(0), b.parse().unwrap_or(0));
            let n = (b + 1).saturating_sub(a);
            toks.push((r[1].clone(), r[9].clone(), rows[i + 1..(i + 1 + n).min(rows.len())].to_vec()));
            i += 1 + n;
        } else {
            toks.push((r[1].clone(), "_".into(), vec![r.clone()]));
            i += 1;
        }
    }
    toks
}

/// An analysis as a stored string (for its bits) — the record of `write`.
pub fn analysis_record(a: &Analysis) -> String {
    let ws: Vec<String> = a.words.iter().map(|w| [w.form.clone(), w.lemma.clone(), w.upos.clone(), w.xpos.clone(), w.feats.clone(), w.rel.clone(), w.inner.map_or("-".into(), |x| x.to_string()), w.gloss.clone(), w.strong.clone(), w.typ.clone(), (w.space_no as u8).to_string()].join("\x1e")).collect();
    format!("{}\t{}", a.range_misc, ws.join("\x1f"))
}

/// Counts of the training pages' trees.
#[derive(Default)]
pub struct Learn {
    cuts: BTreeMap<String, BTreeMap<Vec<String>, u32>>,
    tokens: BTreeMap<String, BTreeMap<String, (u32, Analysis)>>,
    bare: BTreeMap<String, BTreeMap<String, (u32, Analysis)>>,
    offsets: BTreeMap<(String, String), BTreeMap<i64, u32>>,
    offsets_next: BTreeMap<(String, String, String), BTreeMap<i64, u32>>,
}

fn key_of(a: &Analysis) -> String {
    format!("{a:?}")
}

impl Learn {
    /// One page's right-to-left CoNLL-U (the fenced block's lines).
    pub fn add(&mut self, conllu: &str) {
        let mut text = String::new();
        let mut rows: Vec<Vec<String>> = Vec::new();
        for l in conllu.lines() {
            if let Some(t) = l.strip_prefix("# text = ") {
                text = t.to_string();
            } else if !l.starts_with('#') && !l.starts_with("```") && !l.trim().is_empty() {
                let c: Vec<String> = l.split('\t').map(String::from).collect();
                if c.len() == 10 {
                    rows.push(c);
                }
            }
        }
        let toks = tokens_of(&rows);
        // the UPOS of the word after each word (by id), for the heads that depend on what follows
        let word_rows: Vec<&Vec<String>> = rows.iter().filter(|r| r[0].parse::<usize>().is_ok()).collect();
        let next_upos = |id: i64| -> String { word_rows.iter().find(|r| r[0].parse::<i64>().ok() == Some(id + 1)).map(|r| r[3].clone()).unwrap_or_else(|| "END".into()) };
        for (form, rm, ws) in &toks {
            let a = analysis_of(rm, ws);
            for (w, row) in a.words.iter().zip(ws) {
                if w.inner.is_none() {
                    let (id, head): (i64, i64) = (row[0].parse().unwrap_or(0), row[6].parse().unwrap_or(0));
                    let off = if head == 0 { 0 } else { head - id };
                    *self.offsets.entry((w.upos.clone(), w.rel.clone())).or_default().entry(off).or_default() += 1;
                    *self.offsets_next.entry((w.upos.clone(), w.rel.clone(), next_upos(id))).or_default().entry(off).or_default() += 1;
                }
            }
            let k = key_of(&a);
            let e = self.tokens.entry(form.clone()).or_default().entry(k.clone()).or_insert((0, a.clone()));
            e.0 += 1;
            let e = self.bare.entry(bare(form)).or_default().entry(k).or_insert((0, a));
            e.0 += 1;
        }
        // the cut of every chunk of the text into the tokens
        let forms: Vec<&str> = toks.iter().map(|t| t.0.as_str()).collect();
        let mut k = 0;
        for chunk in text.split_whitespace() {
            let mut acc = String::new();
            let mut got = Vec::new();
            while k < forms.len() && acc.len() < chunk.len() {
                acc.push_str(forms[k]);
                got.push(forms[k].to_string());
                k += 1;
            }
            if acc != chunk {
                return;
            }
            *self.cuts.entry(chunk.to_string()).or_default().entry(got).or_default() += 1;
        }
    }

    pub fn lexicon(self) -> Lexicon {
        let top = |m: BTreeMap<String, (u32, Analysis)>| m.into_values().fold((0u32, Analysis::default()), |acc, x| if x.0 > acc.0 { x } else { acc }).1;
        Lexicon {
            cuts: self.cuts.into_iter().map(|(k, m)| (k, m.into_iter().fold((Vec::new(), 0u32), |acc, (c, n)| if n > acc.1 { (c, n) } else { acc }).0)).filter(|(k, v)| *v != rule_cut(k)).collect(),
            tokens: self.tokens.into_iter().map(|(k, m)| (k, top(m))).collect(),
            bare: self.bare.into_iter().map(|(k, m)| (k, top(m))).collect(),
            offsets: self.offsets.into_iter().map(|(k, m)| (k, m.into_iter().fold((0i64, 0u32), |acc, (o, n)| if n > acc.1 { (o, n) } else { acc }).0)).collect(),
            prefix_odds: BTreeMap::new(),
            suffixes: BTreeMap::new(),
            hidden_det: BTreeMap::new(),
            offsets_next: self
                .offsets_next
                .into_iter()
                .filter_map(|(k, m)| {
                    let total: u32 = m.values().sum();
                    let (o, n) = m.into_iter().fold((0i64, 0u32), |acc, (o, n)| if n > acc.1 { (o, n) } else { acc });
                    (total >= 3 && n * 2 > total).then_some((k, o))
                })
                .collect(),
            prefixes: BTreeMap::new(),
        }
        .with_prefixes()
    }
}

/// The lexicon as a file, and back (one record a line, fields by tabs; an analysis as its words joined by «\x1f»).
pub fn write(l: &Lexicon) -> String {
    let mut s = String::new();
    let enc = |a: &Analysis| -> String {
        let ws: Vec<String> = a.words.iter().map(|w| [w.form.clone(), w.lemma.clone(), w.upos.clone(), w.xpos.clone(), w.feats.clone(), w.rel.clone(), w.inner.map_or("-".into(), |x| x.to_string()), w.gloss.clone(), w.strong.clone(), w.typ.clone(), (w.space_no as u8).to_string()].join("\x1e")).collect();
        format!("{}\t{}", a.range_misc, ws.join("\x1f"))
    };
    for (k, v) in &l.cuts {
        s += &format!("cut\t{k}\t{}\n", v.join("\x1f"));
    }
    for (k, a) in &l.tokens {
        s += &format!("tok\t{k}\t{}\n", enc(a));
    }
    for (k, a) in &l.bare {
        s += &format!("bare\t{k}\t{}\n", enc(a));
    }
    for ((u, r), o) in &l.offsets {
        s += &format!("off\t{u}\t{r}\t{o}\n");
    }
    for ((u, r, nx), o) in &l.offsets_next {
        s += &format!("offn\t{u}\t{r}\t{nx}\t{o}\n");
    }
    s
}

pub fn read(text: &str) -> Lexicon {
    let mut l = Lexicon::default();
    let dec = |rm: &str, ws: &str| -> Analysis {
        let words = ws
            .split('\x1f')
            .map(|w| {
                let f: Vec<&str> = w.split('\x1e').collect();
                let g = |i: usize| f.get(i).copied().unwrap_or("").to_string();
                AWord { form: g(0), lemma: g(1), upos: g(2), xpos: g(3), feats: g(4), rel: g(5), inner: g(6).parse().ok(), gloss: g(7), strong: g(8), typ: g(9), space_no: g(10) == "1" }
            })
            .collect();
        Analysis { words, range_misc: rm.to_string() }
    };
    for line in text.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        match f.as_slice() {
            ["cut", k, v] => {
                l.cuts.insert(k.to_string(), v.split('\x1f').map(String::from).collect());
            }
            ["tok", k, rm, ws] => {
                l.tokens.insert(k.to_string(), dec(rm, ws));
            }
            ["bare", k, rm, ws] => {
                l.bare.insert(k.to_string(), dec(rm, ws));
            }
            ["off", u, r, o] => {
                l.offsets.insert((u.to_string(), r.to_string()), o.parse().unwrap_or(0));
            }
            ["offn", u, r, nx, o] => {
                l.offsets_next.insert((u.to_string(), r.to_string(), nx.to_string()), o.parse().unwrap_or(0));
            }
            _ => {}
        }
    }
    l.with_prefixes()
}

impl Lexicon {
    fn with_prefixes(mut self) -> Lexicon {
        self.prefixes = prefixes_of(&self.bare);
        let (p, s, h) = affixes_of(&self.bare);
        self.prefix_odds = p;
        self.suffixes = s;
        self.hidden_det = h;
        self
    }
}

#[allow(dead_code)]
fn _unused(_: BTreeSet<()>) {}

#[cfg(test)]
mod tests {
    use super::*;

    const PAGE: &str = "```conllu rtl gloss\n# sent_id = wlc:1SA:73\n# text = וַיָּבֹ֤א יְהוָה֙ עַבְדֶּֽךָ׃\n# verse = 1SA.3.10\n1-2\tוַיָּבֹ֤א\t_\t_\t_\t_\t_\t_\t_\t_\n1\tוַ\tוְ\tCCONJ\t_\t_\t2\tcc\t_\tGloss=and|Ref=1SA_3:10!1|MId=o090030100011|Strong=2050b\n2\tיָּבֹ֤א\tבּוֹא\tVERB\t_\tGender=Masc|Number=Sing\t0\troot\t_\tGloss=he.came|Ref=1SA_3:10!1|MId=o090030100012|Strong=0935|Frame=A0:090030100021;|Type=wayyiqtol\n3\tיְהוָה֙\tיהוה\tPROPN\t_\t_\t2\tnsubj\t_\tGloss=Yahweh|Ref=1SA_3:10!2|MId=o090030100021|Strong=3068\n4-5\tעַבְדֶּֽךָ\t_\t_\t_\t_\t_\t_\t_\t_\n4\tעַבְדּ\tעֶבֶד\tNOUN\t_\tGender=Masc|Number=Sing\t2\tobj\t_\tGloss=servant|Ref=1SA_3:10!3|MId=o090030100031|Strong=5650\n5\tֶֽךָ\tהוּא\tPRON\t_\tPerson=2\t4\tnmod:poss\t_\tGloss=your|Ref=1SA_3:10!3|MId=o090030100032|Strong=3509b\n6\t׃\t׃\tPUNCT\t_\t_\t5\tpunct\t_\tRef=1SA_3:10!3|SpaceAfter=No\n```\n";

    fn rows(s: &str) -> Vec<Vec<String>> {
        s.lines().filter(|l| !l.starts_with('#') && !l.starts_with("```")).map(|l| l.split('\t').map(String::from).collect()).collect()
    }

    #[test]
    fn affixes_cut_an_unknown_token() {
        let adp = AWord { form: "בְּ".into(), lemma: "בְּ".into(), upos: "ADP".into(), rel: "case".into(), ..Default::default() };
        let det = AWord { form: "ה".into(), lemma: "הַ".into(), upos: "DET".into(), rel: "det".into(), ..Default::default() };
        let pron = AWord { form: "ָם".into(), lemma: "הוּא".into(), upos: "PRON".into(), rel: "nmod:poss".into(), ..Default::default() };
        let mut lex = Lexicon::default();
        lex.prefixes.insert('ב', adp.clone());
        lex.prefix_odds.insert("בְּ".into(), (9, 10));
        lex.prefix_odds.insert("בַּ".into(), (9, 10));
        lex.prefix_odds.insert("בֵּ".into(), (1, 10));
        lex.hidden_det.insert("בַּ".into(), (det, 8));
        lex.suffixes.insert("ָם".into(), (pron, 8, 10));
        lex.suffixes.insert("ִים".into(), (AWord::default(), 1, 10));
        let forms = |a: Option<Analysis>| a.map(|a| a.words.iter().map(|w| w.form.clone()).collect::<Vec<_>>());
        // a preposition before an unknown stem; heads on the stem
        let a = segment(&lex, "בְּמִדְבָּר").unwrap();
        assert_eq!(a.words.iter().map(|w| w.form.as_str()).collect::<Vec<_>>(), ["בְּ", "מִדְבָּר"]);
        assert_eq!((a.words[0].inner, a.words[1].inner), (Some(2), None));
        // the vowel that hides the article writes the article's word too, with no letter of its own
        assert_eq!(forms(segment(&lex, "בַּמִּדְבָּר")), Some(vec!["בַּ".into(), "ה".into(), "מִּדְבָּר".into()]));
        // a pronoun at the end
        assert_eq!(forms(segment(&lex, "סוּסָם")), Some(vec!["סוּס".into(), "ָם".into()]));
        // negative controls: a letter whose vowel rarely stands alone (בֵּית), a plural ending, a stem too short
        assert_eq!(segment(&lex, "בֵּיתוֹ"), None);
        assert_eq!(segment(&lex, "סוּסִים"), None);
        assert_eq!(segment(&lex, "בְּךָ"), None);
    }

    #[test]
    fn a_learned_tree_comes_back_with_its_patches() {
        let mut ln = Learn::default();
        ln.add(PAGE);
        let lex = read(&write(&ln.lexicon()));
        let pred = predict(&lex, "וַיָּבֹ֤א יְהוָה֙ עַבְדֶּֽךָ׃", "1SA.3.10");
        let lit = rows(PAGE);
        assert_eq!(pred.len(), lit.len());
        // everything comes from the lexicon and the verse but the Frame and the heads outside the tokens
        let mut patched = 0;
        for (p, l) in pred.iter().zip(&lit) {
            let lr: Vec<&str> = l.iter().map(String::as_str).collect();
            let (ps, _, _) = diff(p, &lr, 6);
            assert_eq!(apply(p, &ps).to_vec(), *l);
            patched += !ps.is_empty() as usize;
        }
        assert!(patched <= 3, "{patched}");
        // negative control: an empty lexicon writes one unknown word a token, and most rows differ
        let none = predict(&Lexicon::default(), "וַיָּבֹ֤א יְהוָה֙ עַבְדֶּֽךָ׃", "1SA.3.10");
        assert_ne!(none.len(), lit.len());
        // a token never seen is cut like its letters without accents
        let a = Analysis { words: vec![AWord { form: "עַבְדּ".into(), ..Default::default() }, AWord { form: "ֶךָ".into(), ..Default::default() }], range_misc: "_".into() };
        assert_eq!(cut_like("עַבְדֶּֽךָ", &a).concat(), "עַבְדֶּֽךָ");
    }
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn patch_fields_and_canonical_pair_fibers() {
        let pred: Row = std::array::from_fn(|_| "_".into());
        for j in 0..10 {
            let mut lit = pred.clone();
            lit[j] = match j {
                5 => "Gender=Masc|Number=Sing",
                9 => "Gloss=x=y|Ref=Q",
                _ => "value",
            }
            .into();
            let refs: Vec<&str> = lit.iter().map(String::as_str).collect();
            let (ps, bits, _) = diff(&pred, &refs, 3);
            assert_eq!(apply(&pred, &ps), lit);
            assert!(bits > 0.0 && bits.is_finite());
        }
        let lit = ["_", "_", "_", "_", "_", "_", "_", "_", "_", "Ref=Q|Gloss=x"];
        let (ps, _, _) = diff(&pred, &lit, 0);
        assert!(matches!(&ps[0], Patch::Field(9, _)));
        assert_eq!(apply(&pred, &ps)[9], lit[9]);
        let p = predict(&Lexicon::default(), "אב׃", "GEN.1.2");
        assert_eq!(p.len(), 2);
        assert_eq!(p[0][0], "1");
        assert_eq!(p[1][3], "PUNCT");
        assert!(p[1][9].contains("Ref=GEN_1:2!1"));
        assert_eq!(bare("אָ֑ב"), "אָב");
    }
}
