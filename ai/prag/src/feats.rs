//! Pragmatics features — a fixed set of slots with symbolic values (as in TiMBL), the same for
//! all MMM models: the perceptron sees them as "slot=value" indicators, IGTree and k-NN as a vector.
//!
//! Utterance: if the sentence contains direct speech, the labels describe it (that is how the LLM annotates), so the words and UD tree
//! are taken from the quoted text, while the narrator's frame ("said Hans") only yields frame features. Quotes
//! are tracked across the sentences of a paragraph: speech carried over from the previous sentence is direct too.
//! Tree — `en::annotate` (MMM model `ud-ewt-eslspok.bin`) on the utterance tokens.

use std::collections::HashMap;

use en::annotate::{Annotator, Word};
use en::gram::{Feat, Rel, Tag, UPos};
use serde::{Deserialize, Serialize};

use crate::data::{Item, Src};

/// Feature slots (order = slot number).
pub const SLOTS: &[&str] = &[
    "w1", "w2", "w3", "wl", "u1", "end", "qm", "excl", "len", // words and punctuation
    "rupos", "rform", "rlem", "tense", "aux1", "inv", "wh", "modal", "neg", "subj", "sperson", "you", "p1", "voc", // UD tree
    "please", "hedge", "eval", "cue", // cues from the list
    "quote", "sayv", "ppos", "prev", "genre", // context
];

/// Slot number by name.
pub fn slot(name: &str) -> usize {
    SLOTS.iter().position(|s| *s == name).unwrap_or_else(|| panic!("no slot {name}"))
}

/// Slot for the previous sentence's act: in training from silver, in prediction the model's prediction.
pub fn prev_slot() -> usize {
    slot("prev")
}

/// Cues from `data/cues.tsv`: phrase (words) → class.
pub struct Cues {
    phrases: Vec<(Vec<String>, String)>,
}

impl Cues {
    pub fn load() -> Cues {
        let mut phrases: Vec<(Vec<String>, String)> = include_str!("../data/cues.tsv")
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| l.split_once('\t'))
            .map(|(p, c)| (p.split(' ').map(norm_word).collect(), c.trim().to_string()))
            .collect();
        // longer phrases first ("thank you" before "thank")
        phrases.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
        Cues { phrases }
    }

    /// Cue classes in the words (with repeats, in text order).
    pub fn find(&self, words: &[String]) -> Vec<(usize, &str)> {
        let mut out = Vec::new();
        let mut i = 0;
        while i < words.len() {
            let mut hit = false;
            for (p, c) in &self.phrases {
                if i + p.len() <= words.len() && words[i..i + p.len()] == p[..] {
                    out.push((i, c.as_str()));
                    i += p.len();
                    hit = true;
                    break;
                }
            }
            if !hit {
                i += 1;
            }
        }
        out
    }
}

/// Word for cue matching: lowercase, "’" → "'", contractions expanded (n't → not, 'll → will, ca → can…).
pub fn norm_word(w: &str) -> String {
    let w = w.to_lowercase().replace('’', "'");
    match w.as_str() {
        "n't" => "not".into(),
        "'ll" => "will".into(),
        "'m" => "am".into(),
        "'re" => "are".into(),
        "'ve" => "have".into(),
        "'d" => "would".into(),
        "ca" => "can".into(),
        "wo" => "will".into(),
        "sha" => "shall".into(),
        _ => w,
    }
}

// ── Quotes ──────────────────────────────────────────────────────────────────────────────────────

/// Direct speech in a sentence: quoted text spans (byte bounds) and whether quotes remain open at the
/// end. `open` — quotes left open by the previous sentence of the paragraph.
pub fn quote_spans(text: &str, open: Option<char>) -> (Vec<(usize, usize)>, Option<char>) {
    let cs: Vec<(usize, char)> = text.char_indices().collect();
    let mut segs = Vec::new();
    let mut cur = open;
    let mut start = 0usize;
    let alnum = |k: usize| cs.get(k).is_some_and(|x| x.1.is_alphanumeric());
    for (k, &(b, c)) in cs.iter().enumerate() {
        let after = b + c.len_utf8();
        let prev_word = k > 0 && alnum(k - 1);
        match (cur, c) {
            (None, '“') | (None, '"') => {
                cur = Some(c);
                start = after;
            }
            (None, '‘') | (None, '\'') if !prev_word && alnum(k + 1) => {
                cur = Some(c);
                start = after;
            }
            (Some('“'), '”') | (Some('"'), '"') | (Some('“'), '"') | (Some('"'), '”') => {
                segs.push((start, b));
                cur = None;
            }
            // "’" or "'" between letters is an apostrophe (don’t), not a quote
            (Some('‘'), '’') | (Some('\''), '\'') | (Some('‘'), '\'') | (Some('\''), '’') if !(prev_word && alnum(k + 1)) => {
                segs.push((start, b));
                cur = None;
            }
            _ => {}
        }
    }
    if cur.is_some() {
        segs.push((start, text.len()));
    }
    (segs.into_iter().filter(|(a, b)| b > a && text[*a..*b].chars().any(char::is_alphanumeric)).collect(), cur)
}

// ── Extraction ──────────────────────────────────────────────────────────────────────────────────

/// Features of one sentence (value rows by `SLOTS`).
pub type Row = Vec<String>;

fn bucket(n: usize) -> &'static str {
    match n {
        0..=1 => "1",
        2..=3 => "2-3",
        4..=6 => "4-6",
        7..=12 => "7-12",
        13..=20 => "13-20",
        _ => "21+",
    }
}

fn yes(b: bool) -> String {
    if b { "yes".into() } else { "-".into() }
}

fn is_punct(w: &Word) -> bool {
    w.upos == UPos::PUNCT || w.form.chars().all(|c| !c.is_alphanumeric())
}

/// Features of all sentences in `items` order (quotes carry over between adjacent sentences of a paragraph). Slot `prev`
/// is "start": it is set by whoever trains or predicts (`set_prev`).
pub fn extract(a: &Annotator, cues: &Cues, items: &[Item]) -> Vec<Row> {
    let mut out = Vec::with_capacity(items.len());
    let mut open: Option<char> = None;
    for (i, it) in items.iter().enumerate() {
        let continues = i > 0 && {
            let p = &items[i - 1];
            p.src == it.src && p.doc == it.doc && p.para == it.para && p.ord + 1 == it.ord && it.src != Src::Tatoeba
        };
        if !continues {
            open = None;
        }
        let carried = open;
        let (segs, left) = quote_spans(&it.text, open);
        open = left;
        out.push(row(a, cues, it, &segs, carried.is_some()));
    }
    out
}

/// Text outside the spans — the narrator's frame.
fn frame_of(text: &str, segs: &[(usize, usize)]) -> String {
    let mut f = String::new();
    let mut last = 0;
    for &(x, y) in segs {
        f.push_str(&text[last..x]);
        f.push(' ');
        last = y;
    }
    f.push_str(&text[last..]);
    f
}

fn frame_words(frame: &str) -> Vec<String> {
    frame.split(|c: char| !c.is_alphanumeric() && c != '\'' && c != '’').filter(|x| !x.is_empty()).map(norm_word).collect()
}

fn row(a: &Annotator, cues: &Cues, it: &Item, segs: &[(usize, usize)], carried: bool) -> Row {
    let text = &it.text;
    // quotes are direct speech, not a title or a quoted word ("the title of “Gentlemen Weavers.”"): a span
    // carried over from the previous sentence, of 4+ words, at the start of the sentence, with a speech verb in the frame,
    // or ending with ! or ?
    let say = cues.find(&frame_words(&frame_of(text, segs))).iter().any(|x| x.1 == "say");
    let segs: Vec<(usize, usize)> = segs
        .iter()
        .copied()
        .filter(|&(x, y)| {
            let seg = &text[x..y];
            (carried && x == 0) || seg.split_whitespace().count() >= 4 || !text[..x].chars().any(char::is_alphanumeric) || say || seg.trim_end().ends_with(['!', '?'])
        })
        .collect();
    let segs = &segs[..];
    // utterance: the quoted text, otherwise the whole sentence; the frame is the rest
    let (utt, frame): (String, String) = if segs.is_empty() {
        (text.clone(), String::new())
    } else {
        let u = segs.iter().map(|&(x, y)| text[x..y].trim()).collect::<Vec<_>>().join(" ");
        (u, frame_of(text, segs))
    };
    let frame_has_words = frame.chars().any(char::is_alphabetic);
    let quote = match (carried, segs.is_empty(), frame_has_words) {
        (true, _, _) => "cont",
        (false, true, _) => "none",
        (false, false, false) => "whole",
        (false, false, true) => "framed",
    };
    let toks = a.tokenize(&utt);
    let forms: Vec<&str> = toks.iter().map(|t| t.form.as_str()).collect();
    let words = if forms.is_empty() { Vec::new() } else { a.annotate(&forms) };
    let lw: Vec<String> = words.iter().map(|w| norm_word(&w.form)).collect();
    // words without punctuation
    let content: Vec<usize> = (0..words.len()).filter(|&k| !is_punct(&words[k])).collect();
    let wd = |n: usize| content.get(n).map(|&k| lw[k].clone()).unwrap_or_else(|| "-".into());
    let wl = content.last().map(|&k| lw[k].clone()).unwrap_or_else(|| "-".into());
    let u1 = content.first().map(|&k| words[k].upos.name().to_string()).unwrap_or_else(|| "-".into());
    // punctuation at the end of the utterance
    let tail: String = utt.trim_end().chars().rev().take_while(|c| !c.is_alphanumeric()).collect::<Vec<_>>().into_iter().rev().collect();
    let end = if tail.contains('?') {
        "?"
    } else if tail.contains('!') {
        "!"
    } else if tail.contains('…') || tail.contains("...") {
        "…"
    } else if tail.contains('.') {
        "."
    } else if tail.contains('-') || tail.contains('—') {
        "dash"
    } else if tail.contains([',', ';', ':']) {
        ","
    } else {
        "none"
    };
    let qm = utt.contains('?');
    let excl = end == "!" && (wd(0) == "how" || (wd(0) == "what" && matches!(wd(1).as_str(), "a" | "an")));
    // tree
    let root = words.iter().position(|w| w.head == 0);
    let kids = |h: usize| words.iter().enumerate().filter(move |(_, w)| w.head == h + 1).map(|(k, _)| k);
    let (rupos, rform, rlem, tense) = match root {
        Some(r) => {
            let w = &words[r];
            let f = w.feats;
            let form = if f.has(Feat::MoodImp) {
                "Imp"
            } else if f.has(Feat::VerbFormFin) {
                "Fin"
            } else if f.has(Feat::VerbFormInf) {
                "Inf"
            } else if f.has(Feat::VerbFormGer) {
                "Ger"
            } else if f.has(Feat::VerbFormPart) {
                "Part"
            } else {
                "-"
            };
            let t_of = |x: &Word| if x.feats.has(Feat::TensePast) { Some("Past") } else if x.feats.has(Feat::TensePres) { Some("Pres") } else { None };
            let tense = t_of(w).or_else(|| kids(r).filter(|&k| matches!(words[k].rel, Rel::Aux | Rel::AuxPass | Rel::Cop)).find_map(|k| t_of(&words[k]))).unwrap_or("-");
            (w.upos.name().to_string(), form.to_string(), w.lemma.to_lowercase(), tense.to_string())
        }
        None => ("-".into(), "-".into(), "-".into(), "-".into()),
    };
    let first = content.first().copied();
    let aux1 = first.filter(|&k| words[k].upos == UPos::AUX).map(|k| words[k].lemma.to_lowercase()).unwrap_or_else(|| "-".into());
    let subj = root.and_then(|r| kids(r).find(|&k| matches!(words[k].rel, Rel::Nsubj | Rel::NsubjPass | Rel::Expl | Rel::Csubj)));
    let inv = match (root, subj) {
        (Some(r), Some(s)) => kids(r).any(|k| k < s && words[k].upos == UPos::AUX),
        _ => false,
    };
    let wh = content.iter().take(3).map(|&k| &words[k]).find(|w| w.tag.is_wh() || w.feats.has(Feat::PronTypeInt)).map(|w| norm_word(&w.form)).unwrap_or_else(|| "-".into());
    let modal = words.iter().find(|w| w.tag == Tag::MD).map(|w| norm_word(&w.form)).unwrap_or_else(|| "-".into());
    let negw = |w: &Word| w.feats.has(Feat::PolarityNeg) || matches!(norm_word(&w.form).as_str(), "not" | "never" | "nothing" | "nobody" | "none" | "neither" | "nor" | "no");
    let neg = match root {
        Some(r) if negw(&words[r]) || kids(r).any(|k| negw(&words[k])) => "root",
        _ if words.iter().any(negw) => "other",
        _ => "-",
    };
    let (subj_v, sperson) = match subj {
        Some(s) => {
            let w = &words[s];
            let v = if w.upos == UPos::PRON || w.rel == Rel::Expl { norm_word(&w.form) } else { w.upos.name().to_string() };
            let p = if w.feats.has(Feat::Person1) {
                "1"
            } else if w.feats.has(Feat::Person2) {
                "2"
            } else if w.feats.has(Feat::Person3) || matches!(w.upos, UPos::NOUN | UPos::PROPN) {
                "3"
            } else {
                "-"
            };
            (v, p.to_string())
        }
        None => ("-".into(), "-".into()),
    };
    let you = lw.iter().any(|w| matches!(w.as_str(), "you" | "your" | "yours" | "yourself" | "yourselves" | "thee" | "thou" | "thy" | "thine" | "ye"));
    let p1 = lw.iter().any(|w| matches!(w.as_str(), "i" | "me" | "my" | "mine" | "myself" | "we" | "us" | "our" | "ours" | "ourselves"));
    let voc = words.iter().any(|w| w.rel == Rel::Vocative) || (content.len() > 1 && matches!(first.map(|k| words[k].upos), Some(UPos::PROPN | UPos::NOUN)) && lw.get(first.unwrap() + 1).is_some_and(|x| x == ","));
    // cues (utterance words, "let 's" → "let us")
    let mut cw: Vec<String> = lw.clone();
    for k in 1..cw.len() {
        if cw[k] == "'s" && cw[k - 1] == "let" {
            cw[k] = "us".into();
        }
    }
    let found = cues.find(&cw);
    let first_of = |cls: &[&str]| found.iter().find(|(_, c)| cls.contains(c)).map(|(k, c)| (*k, *c));
    let hedge = first_of(&["hedge"]).map(|(k, _)| cw[k].clone()).unwrap_or_else(|| "-".into());
    let (pos, negc) = (found.iter().filter(|x| x.1 == "pos").count(), found.iter().filter(|x| x.1 == "neg").count());
    let eval = match (pos > 0, negc > 0) {
        (true, true) => "mix",
        (true, false) => "pos",
        (false, true) => "neg",
        _ => "-",
    };
    let cue = first_of(&["thank", "apol", "greet", "promise", "warn", "suggest", "offer", "yes", "no", "intj"]).map(|(_, c)| c.to_string()).unwrap_or_else(|| "-".into());
    let please = cw.iter().any(|w| w == "please");
    // frame: speech verb
    let fw = frame_words(&frame);
    let sayv = cues.find(&fw).into_iter().find(|x| x.1 == "say").map(|(k, _)| fw[k].clone()).unwrap_or_else(|| "-".into());
    let ppos = if it.plen <= 1 {
        "only"
    } else if it.pos == 0 {
        "first"
    } else if it.pos + 1 == it.plen {
        "last"
    } else {
        "mid"
    };
    let genre = match it.src {
        Src::Tale => "tale",
        Src::Tatoeba => "tatoeba",
        Src::Gum => "other",
    };
    let r: Row = vec![
        wd(0),
        wd(1),
        wd(2),
        wl,
        u1,
        end.into(),
        yes(qm),
        yes(excl),
        bucket(content.len()).into(),
        rupos,
        rform,
        rlem,
        tense,
        aux1,
        yes(inv),
        wh,
        modal,
        neg.into(),
        subj_v,
        sperson,
        yes(you),
        yes(p1),
        yes(voc),
        yes(please),
        hedge,
        eval.into(),
        cue,
        quote.into(),
        sayv,
        ppos.into(),
        "start".into(),
        genre.into(),
    ];
    debug_assert_eq!(r.len(), SLOTS.len());
    r
}

/// Neighbourhood for the previous sentence's act: the previous sentence of the same document (order − 1).
pub fn prev_of(items: &[Item]) -> Vec<Option<usize>> {
    let mut at: HashMap<(&str, i64), usize> = HashMap::new();
    for (i, it) in items.iter().enumerate() {
        if it.src != Src::Tatoeba && it.ord > 0 {
            at.insert((it.doc.as_str(), it.ord), i);
        }
    }
    items.iter().map(|it| if it.src == Src::Tatoeba || it.ord <= 1 { None } else { at.get(&(it.doc.as_str(), it.ord - 1)).copied() }).collect()
}

// ── Value dictionary ────────────────────────────────────────────────────────────────────────────

/// Slot values as numbers (a separate dictionary per slot). An unknown value is `UNSEEN`: it matches nothing
/// in memory and has no weight.
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Dict {
    pub values: Vec<Vec<String>>,
    #[serde(skip)]
    index: Vec<HashMap<String, u32>>,
}

pub const UNSEEN: u32 = u32::MAX;

impl Dict {
    /// Dictionary from (training) rows.
    pub fn build(rows: &[Row]) -> Dict {
        let mut d = Dict { values: vec![Vec::new(); SLOTS.len()], index: vec![HashMap::new(); SLOTS.len()] };
        // all acts for the prev slot, so that a predicted "request" has a number even if it never occurred in training
        for a in crate::schema::Act::ALL.iter().map(|x| x.name()).chain(["start"]) {
            d.add(prev_slot(), a);
        }
        for r in rows {
            for (s, v) in r.iter().enumerate() {
                d.add(s, v);
            }
        }
        d
    }

    fn add(&mut self, s: usize, v: &str) -> u32 {
        if let Some(&i) = self.index[s].get(v) {
            return i;
        }
        let i = self.values[s].len() as u32;
        self.values[s].push(v.to_string());
        self.index[s].insert(v.to_string(), i);
        i
    }

    /// After reading from file — rebuild the lookup index.
    pub fn reindex(&mut self) {
        self.index = self.values.iter().map(|vs| vs.iter().enumerate().map(|(i, v)| (v.clone(), i as u32)).collect()).collect();
    }

    pub fn id(&self, s: usize, v: &str) -> u32 {
        self.index[s].get(v).copied().unwrap_or(UNSEEN)
    }

    pub fn encode(&self, r: &Row) -> Vec<u32> {
        r.iter().enumerate().map(|(s, v)| self.id(s, v)).collect()
    }

    /// "slot=value" for explanations.
    pub fn show(&self, s: usize, v: u32) -> String {
        let val = if v == UNSEEN { "?" } else { self.values[s][v as usize].as_str() };
        format!("{}={val}", SLOTS[s])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quotes_across_sentences() {
        let t = "At last he said, ‘Master, my time is up; I must go home.’";
        let (s, open) = quote_spans(t, None);
        assert_eq!((s.len(), open), (1, None));
        assert!(t[s[0].0..s[0].1].starts_with("Master"));
        // an apostrophe inside a word is not a quote; an unclosed quote carries over to the next sentence
        let (s, open) = quote_spans("‘Don’t go, dear.", None);
        assert_eq!((s.len(), open), (1, Some('‘')));
        let (s, open) = quote_spans("I will come.’ And he went.", open);
        assert_eq!(open, None);
        assert_eq!(s.len(), 1);
        // no quotes — no spans
        assert!(quote_spans("The dogs’ bones lay there.", None).0.is_empty());
        let (s, _) = quote_spans("He said, \"I'm hungry.\"", None);
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn cues_and_norm() {
        let c = Cues::load();
        let w: Vec<String> = ["i", "think", "thank", "you", "let", "us", "go"].iter().map(|x| x.to_string()).collect();
        let f: Vec<&str> = c.find(&w).into_iter().map(|x| x.1).collect();
        assert_eq!(f, ["hedge", "thank", "suggest"]);
        assert_eq!(norm_word("n’t"), "not");
        assert_eq!(norm_word("Ca"), "can");
    }
}
