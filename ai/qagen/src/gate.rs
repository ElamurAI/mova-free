//! Gates for the LLM's answer to a batch of paragraphs. A line is `p type ex anchors question answer`, TAB-separated.
//!
//! Batch (format; only this rejection allows one retry of the call):
//! - no question lines at all;
//! - a paragraph number outside 1..N — alignment is in doubt;
//! - junk (lines without a number) above 20 %;
//! - more than a quarter of paragraphs without any line — the output is truncated.
//!
//! Pair (rejects only this pair), first reason wins:
//! - not 6 columns;
//! - type outside the closed FairytaleQA set, explicitness outside explicit/implicit;
//! - anchors: not numbers, empty, repeated, or a sentence outside the paragraph (1..n);
//! - empty question or answer;
//! - an implicit question whose answer appears verbatim (consecutive words, ignoring case and punctuation) in one sentence;
//! - rule v2: an explicit answer that is not a span of an anchor sentence (`span`): after normalization (case,
//!   quotes, spaces, punctuation at the edges) it must occur in an anchor sentence as a substring on word boundaries;
//! - a repeated question in the paragraph; the seventh and later questions of a paragraph.
//!
//! Paragraph: no lines at all — the paragraph is rejected; fewer than 3 questions, no implicit one, or all of one type —
//! a note in the log (the pairs stay). The "doubtful implicit" flag (lemmas from `en`) is `lemma::doubt`,
//! log level `doubt`.

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::schema::{ExIm, QType, Rule};
use crate::select::Para;

pub const MIN_Q: usize = 3;
pub const MAX_Q: usize = 6;

/// An accepted pair.
#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    /// paragraph in the batch (from 1)
    pub p: usize,
    /// question number in the paragraph after the gates (from 1)
    pub k: usize,
    pub qtype: QType,
    pub exim: ExIm,
    /// paragraph sentences (from 1), ascending
    pub anchors: Vec<usize>,
    pub question: String,
    pub answer: String,
}

/// Level of a log record.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Level {
    /// the whole batch rejected (format)
    Batch,
    /// paragraph rejected (no lines)
    Para,
    /// pair rejected
    Pair,
    /// a note about the paragraph, the pairs stay
    Note,
    /// a pair flagged "doubtful implicit" (lemmas of a single anchor), the pair stays
    Doubt,
}

impl Level {
    pub fn name(self) -> &'static str {
        match self {
            Level::Batch => "batch",
            Level::Para => "para",
            Level::Pair => "pair",
            Level::Note => "note",
            Level::Doubt => "doubt",
        }
    }
}

/// A rejection log record.
#[derive(Clone, Debug)]
pub struct Reject {
    pub p: Option<usize>,
    pub level: Level,
    pub reason: String,
    /// raw output line (for a pair)
    pub line: String,
}

pub struct Gated {
    pub ok: Vec<Pair>,
    pub rej: Vec<Reject>,
    /// lines without a number
    pub junk: usize,
    /// batch format rejection — grounds for one retry
    pub format: Option<String>,
}

/// Words for comparison: lowercase, quote-apostrophes unified, punctuation acts as word boundaries; an apostrophe
/// inside a word stays (don't, king's).
pub fn words(s: &str) -> Vec<String> {
    let cs: Vec<char> = s.chars().map(|c| if matches!(c, '’' | '‘' | 'ʼ') { '\'' } else { c }).collect();
    let mut out = Vec::new();
    let mut cur = String::new();
    for (i, &c) in cs.iter().enumerate() {
        if c.is_alphanumeric() {
            cur.extend(c.to_lowercase());
        } else if c == '\'' && !cur.is_empty() && cs.get(i + 1).is_some_and(|x| x.is_alphanumeric()) {
            cur.push('\'');
        } else if !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Number of the sentence (from 1) where the answer appears verbatim — consecutive words.
pub fn verbatim(answer: &str, sents: &[Vec<String>]) -> Option<usize> {
    let a = words(answer);
    if a.is_empty() {
        return None;
    }
    sents.iter().position(|s| s.windows(a.len()).any(|w| w == a.as_slice())).map(|i| i + 1)
}

fn squote(c: char) -> bool {
    matches!(c, '\'' | '‘' | '’' | 'ʼ' | '‛' | '`' | '´' | '′' | '‹' | '›')
}

fn dquote(c: char) -> bool {
    matches!(c, '"' | '“' | '”' | '„' | '‟' | '«' | '»' | '″')
}

/// Normalization for the span gate: lowercase; quotes removed (a single quote between letters is an
/// apostrophe and stays as `'`: king's, don't); whitespace collapsed to one space. Other punctuation is kept as is.
pub fn span_norm(s: &str) -> String {
    let cs: Vec<char> = s.chars().collect();
    let mut out = String::with_capacity(s.len());
    let mut space = false;
    for (i, &c) in cs.iter().enumerate() {
        let c = if dquote(c) {
            continue;
        } else if squote(c) {
            let inner = i > 0 && cs[i - 1].is_alphanumeric() && cs.get(i + 1).is_some_and(|x| x.is_alphanumeric());
            if !inner {
                continue;
            }
            '\''
        } else {
            c
        };
        if c.is_whitespace() {
            space = true;
            continue;
        }
        if space && !out.is_empty() {
            out.push(' ');
        }
        space = false;
        out.extend(c.to_lowercase());
    }
    out
}

/// Normalized answer without punctuation at the edges.
pub fn span_answer(answer: &str) -> String {
    span_norm(answer).trim_matches(|c: char| !c.is_alphanumeric()).to_string()
}

/// `a` (from `span_answer`) is a substring of the normalized sentence `s` on word boundaries.
pub fn is_span(a: &str, s: &str) -> bool {
    if a.is_empty() {
        return false;
    }
    let mut from = 0;
    while let Some(i) = s[from..].find(a) {
        let at = from + i;
        let end = at + a.len();
        let left = s[..at].chars().next_back().is_none_or(|c| !c.is_alphanumeric());
        let right = s[end..].chars().next().is_none_or(|c| !c.is_alphanumeric());
        if left && right {
            return true;
        }
        from = at + a.chars().next().map_or(1, char::len_utf8);
    }
    false
}

/// Span gate (rule v2): `None` — the explicit answer occurs as a substring of an anchor sentence; otherwise the reason.
/// `norm` — the paragraph's sentences after `span_norm`, `sents` — their words (`words`), `anchors` — from 1.
pub fn span_reason(answer: &str, norm: &[String], sents: &[Vec<String>], anchors: &[usize]) -> Option<String> {
    let a = span_answer(answer);
    if anchors.iter().any(|&k| is_span(&a, &norm[k - 1])) {
        return None;
    }
    if let Some(k) = (1..=norm.len()).find(|k| !anchors.contains(k) && is_span(&a, &norm[k - 1])) {
        return Some(format!("explicit: span of sentence {k}, which is not an anchor"));
    }
    let w = words(answer);
    if let Some(&k) = anchors.iter().find(|&&k| !w.is_empty() && sents[k - 1].windows(w.len()).any(|x| x == w.as_slice())) {
        return Some(format!("explicit: consecutive words in sentence {k}, but different punctuation"));
    }
    Some("explicit: not a span of an anchor sentence".into())
}

/// Anchors: comma-separated numbers, each in 1..=n, no repeats, at least one.
pub fn anchors(s: &str, n: usize) -> Result<Vec<usize>, String> {
    let mut out = BTreeSet::new();
    for part in s.split(',') {
        let t = part.trim();
        if t.is_empty() {
            continue;
        }
        let k: usize = t.parse().map_err(|_| format!("anchor «{t}» is not a number"))?;
        if k == 0 || k > n {
            return Err(format!("anchor {k} outside the paragraph ({n} sentences)"));
        }
        if !out.insert(k) {
            return Err(format!("anchor {k} twice"));
        }
    }
    if out.is_empty() {
        return Err("no anchors".into());
    }
    Ok(out.into_iter().collect())
}

/// Paragraph sentences for the gates: words (`words`) and normalized text (`span_norm`).
pub struct Sents {
    pub words: Vec<Vec<String>>,
    pub norm: Vec<String>,
}

impl Sents {
    pub fn of(p: &Para) -> Sents {
        Sents { words: p.sents.iter().map(|s| words(&s.text)).collect(), norm: p.sents.iter().map(|s| span_norm(&s.text)).collect() }
    }
}

/// Gates for one pair: (type, explicitness, anchors, question, answer) or the rejection reason.
fn check(cols: &[&str], s: &Sents, rule: Rule) -> Result<(QType, ExIm, Vec<usize>, String, String), String> {
    let sents = &s.words;
    if cols.len() != 6 {
        return Err(format!("{} columns instead of 6", cols.len()));
    }
    let qtype = QType::parse(cols[1]).ok_or_else(|| format!("type «{}» not in the set", cols[1].trim()))?;
    let exim = ExIm::parse(cols[2]).ok_or_else(|| format!("explicitness «{}» not in the set", cols[2].trim()))?;
    let anchors = anchors(cols[3], sents.len())?;
    let q = cols[4].trim();
    if words(q).is_empty() {
        return Err("empty question".into());
    }
    let a = cols[5].trim();
    if words(a).is_empty() {
        return Err("empty answer".into());
    }
    if exim == ExIm::Implicit
        && let Some(k) = verbatim(a, sents)
    {
        return Err(format!("implicit, but the answer is verbatim in sentence {k}"));
    }
    if rule >= Rule::V2
        && exim == ExIm::Explicit
        && let Some(why) = span_reason(a, &s.norm, sents, &anchors)
    {
        return Err(why);
    }
    Ok((qtype, exim, anchors, q.to_string(), a.to_string()))
}

/// Gates for the answer to the batch of paragraphs `paras` (numbered from 1 in batch order) under rule `rule`.
pub fn gate(text: &str, paras: &[&Para], rule: Rule) -> Gated {
    let n = paras.len();
    let mut by_p: BTreeMap<usize, Vec<(String, Vec<String>)>> = BTreeMap::new();
    let (mut junk, mut total) = (0usize, 0usize);
    let mut outside: Vec<usize> = Vec::new();
    for line in text.lines() {
        let l = line.trim_end_matches('\r');
        let t = l.trim();
        if t.is_empty() || t.starts_with("```") {
            continue;
        }
        let cols: Vec<String> = l.split('\t').map(str::to_string).collect();
        if cols[0].trim().eq_ignore_ascii_case("p") {
            continue; // header
        }
        // "P3" instead of "3" is the same unambiguous paragraph label
        let Ok(p) = cols[0].trim().trim_start_matches(['P', 'p']).parse::<usize>() else {
            junk += 1;
            continue;
        };
        if p == 0 || p > n {
            outside.push(p);
            continue;
        }
        total += 1;
        by_p.entry(p).or_default().push((l.to_string(), cols));
    }
    let missing = (1..=n).filter(|p| !by_p.contains_key(p)).count();
    let format = if total == 0 {
        Some("no question lines at all".to_string())
    } else if !outside.is_empty() {
        Some(format!("paragraph numbers outside 1..{n}: {outside:?} — alignment in doubt"))
    } else if junk * 5 > total + junk {
        Some(format!("lines without a number: {junk} of {}", total + junk))
    } else if missing * 4 > n {
        Some(format!("paragraphs without questions: {missing} of {n} — truncated output?"))
    } else {
        None
    };
    if let Some(f) = format {
        let rej = vec![Reject { p: None, level: Level::Batch, reason: f.clone(), line: String::new() }];
        return Gated { ok: Vec::new(), rej, junk, format: Some(f) };
    }
    let mut ok = Vec::new();
    let mut rej = Vec::new();
    for (p, para) in paras.iter().enumerate().map(|(i, a)| (i + 1, a)) {
        let Some(lines) = by_p.get(&p) else {
            rej.push(Reject { p: Some(p), level: Level::Para, reason: "no questions".into(), line: String::new() });
            continue;
        };
        let sents = Sents::of(para);
        let mut acc: Vec<Pair> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();
        for (line, cols) in lines {
            let cols: Vec<&str> = cols.iter().map(String::as_str).collect();
            let r = check(&cols, &sents, rule).and_then(|(qtype, exim, anchors, question, answer)| {
                if !seen.insert(words(&question).join(" ")) {
                    return Err("question repeated".into());
                }
                if acc.len() >= MAX_Q {
                    return Err(format!("more than {MAX_Q} questions per paragraph"));
                }
                Ok(Pair { p, k: acc.len() + 1, qtype, exim, anchors, question, answer })
            });
            match r {
                Ok(pair) => acc.push(pair),
                Err(reason) => rej.push(Reject { p: Some(p), level: Level::Pair, reason, line: line.clone() }),
            }
        }
        let note = |reason: String| Reject { p: Some(p), level: Level::Note, reason, line: String::new() };
        if acc.len() < MIN_Q {
            rej.push(note(format!("accepted questions {} < {MIN_Q}", acc.len())));
        }
        if !acc.is_empty() && !acc.iter().any(|x| x.exim == ExIm::Implicit) {
            rej.push(note("no implicit questions".into()));
        }
        if acc.len() > 1 && acc.iter().map(|x| x.qtype).collect::<BTreeSet<_>>().len() == 1 {
            rej.push(note("all questions of one type".into()));
        }
        ok.extend(acc);
    }
    Gated { ok, rej, junk, format: None }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::select::Sent;

    fn para(texts: &[&str]) -> Para {
        Para {
            doc: "g:1:t".into(),
            book: "g:1".into(),
            title: "T".into(),
            author: String::new(),
            para: 1,
            block: "p".into(),
            sents: texts.iter().enumerate().map(|(i, t)| Sent { sent_id: format!("g:1:t:{}", i + 1), text: t.to_string() }).collect(),
            dialogue: false,
            words: 0,
            ctx: vec![],
            batch: 0,
        }
    }

    fn a() -> Para {
        para(&["A certain king had a beautiful garden with a tree of golden apples.", "Every night one of the apples was gone.", "The king was very angry and ordered the gardener to watch."])
    }

    fn b() -> Para {
        para(&["‘I will catch the thief,’ said the eldest son.", "At midnight he fell asleep."])
    }

    const GOOD: &str = "1\tcharacter\texplicit\t1\tWho owned the garden?\ta certain king\n\
1\tcausal relationship\timplicit\t2,3\tWhy was the king angry?\tsomeone kept stealing his golden apples\n\
1\taction\texplicit\t3\tWhat did the king order the gardener to do?\tto watch\n\
2\tprediction\timplicit\t1\tWhat will the eldest son try to do?\tstay up and catch whoever takes the apples\n\
2\toutcome resolution\texplicit\t2\tWhat happened to the eldest son at midnight?\the fell asleep\n\
2\tfeeling\timplicit\t1\tHow did the eldest son feel about the watch?\tconfident\n";

    #[test]
    fn words_and_verbatim() {
        assert_eq!(words("‘I will catch the King’s thief,’ said he."), ["i", "will", "catch", "the", "king's", "thief", "said", "he"]);
        let s = vec![words("The king was very angry."), words("He fell asleep.")];
        assert_eq!(verbatim("very angry", &s), Some(1));
        assert_eq!(verbatim("He fell asleep", &s), Some(2));
        assert_eq!(verbatim("angry and sad", &s), None);
        assert_eq!(verbatim("king angry", &s), None); // words not consecutive
    }

    #[test]
    fn good_batch_passes() {
        let (a, b) = (a(), b());
        let g = gate(GOOD, &[&a, &b], Rule::V1);
        assert!(g.format.is_none());
        assert_eq!(g.ok.len(), 6);
        assert!(g.rej.is_empty(), "{:?}", g.rej);
        assert_eq!(g.ok[1], Pair { p: 1, k: 2, qtype: QType::Causal, exim: ExIm::Implicit, anchors: vec![2, 3], question: "Why was the king angry?".into(), answer: "someone kept stealing his golden apples".into() });
        // header, code fence and CRLF are not junk
        let g = gate(&format!("```\np\ttype\tex\tanchors\tquestion\tanswer\n{}```\n", GOOD.replace('\n', "\r\n")), &[&a, &b], Rule::V1);
        assert_eq!((g.ok.len(), g.junk), (6, 0));
        let g = gate(&GOOD.replace("\n1\t", "\nP1\t"), &[&a, &b], Rule::V1);
        assert_eq!(g.ok.len(), 6);
    }

    #[test]
    fn pair_gates_reject_only_the_pair() {
        let (a, b) = (a(), b());
        let bad = [
            ("1\tcausal\timplicit\t2\tWhy?\tbecause\n", "type «causal» not in the set"),
            ("1\taction\tinferred\t2\tWhat happened?\tapples vanished\n", "explicitness «inferred» not in the set"),
            ("1\taction\texplicit\t4\tWhat did the king do?\tordered a watch\n", "anchor 4 outside the paragraph (3 sentences)"),
            ("1\taction\texplicit\t0\tWhat did the king do?\tordered a watch\n", "anchor 0 outside the paragraph (3 sentences)"),
            ("1\taction\texplicit\t2,2\tWhat did the king do?\tordered a watch\n", "anchor 2 twice"),
            ("1\taction\texplicit\t\tWhat did the king do?\tordered a watch\n", "no anchors"),
            ("1\taction\texplicit\ttwo\tWhat did the king do?\tordered a watch\n", "anchor «two» is not a number"),
            ("1\taction\texplicit\t3\tWhat did the king do?\t \n", "empty answer"),
            ("1\taction\texplicit\t3\tWhat did the king do?\t-\n", "empty answer"),
            ("1\taction\texplicit\t3\t?\tordered a watch\n", "empty question"),
            ("1\tfeeling\timplicit\t3\tHow did the king feel?\tVery angry.\n", "implicit, but the answer is verbatim in sentence 3"),
            ("1\taction\texplicit\t3\tWhat did the king do?\n", "5 columns instead of 6"),
            ("1\tcharacter\texplicit\t1\tWho owned the garden?\tthe king\n", "question repeated"),
        ];
        for (line, why) in bad {
            let g = gate(&format!("{GOOD}{line}"), &[&a, &b], Rule::V1);
            assert!(g.format.is_none(), "{line}");
            assert_eq!(g.ok.len(), 6, "{line}");
            let r: Vec<&Reject> = g.rej.iter().filter(|r| r.level == Level::Pair).collect();
            assert_eq!(r.len(), 1, "{line}: {:?}", g.rej);
            assert_eq!(r[0].reason, why, "{line}");
            assert_eq!(r[0].line, line.trim_end_matches('\n'));
        }
        // the same answer as explicit passes
        let g = gate(&format!("{GOOD}1\tfeeling\texplicit\t3\tHow did the king feel?\tVery angry.\n"), &[&a, &b], Rule::V1);
        assert_eq!(g.ok.len(), 7);
        // the seventh question of a paragraph is rejected
        let extra = "1\tsetting\texplicit\t1\tWhere was the tree?\tin the garden\n1\tfeeling\timplicit\t3\tHow did the gardener feel?\tworried\n1\taction\texplicit\t2\tWhat was gone every night?\tone of the apples\n1\tcharacter\texplicit\t3\tWho had to guard the tree?\tthe gardener\n";
        let g = gate(&format!("{GOOD}{extra}"), &[&a, &b], Rule::V1);
        assert_eq!(g.ok.iter().filter(|x| x.p == 1).count(), MAX_Q);
        assert!(g.rej.iter().any(|r| r.reason == "more than 6 questions per paragraph"));
    }

    #[test]
    fn batch_and_paragraph_gates() {
        let (a, b) = (a(), b());
        // an extra paragraph number is a format error (alignment)
        let g = gate(&format!("{GOOD}3\taction\texplicit\t1\tWho?\tking\n"), &[&a, &b], Rule::V1);
        assert!(g.format.is_some() && g.ok.is_empty() && g.rej[0].level == Level::Batch);
        // empty or only explanations
        assert!(gate("", &[&a, &b], Rule::V1).format.is_some());
        assert!(gate("Here are the questions:\nSure!\n", &[&a, &b], Rule::V1).format.is_some());
        // lots of junk
        assert!(gate(&format!("{GOOD}a\nb\nc\nd\n"), &[&a, &b], Rule::V1).format.is_some());
        // truncated output: half of the paragraphs missing
        let only1: String = GOOD.lines().filter(|l| l.starts_with("1\t")).map(|l| format!("{l}\n")).collect();
        assert!(gate(&only1, &[&a, &b], Rule::V1).format.is_some());
        // paragraph: fewer than 3, no implicit, one type — notes, the pairs stay; no lines — the paragraph is rejected
        let (c, d, e, f, h) = (a.clone(), a.clone(), a.clone(), a.clone(), a.clone());
        let text = format!("{GOOD}3\taction\texplicit\t3\tWhat did the king order?\tto watch the tree\n3\taction\texplicit\t2\tWhat was gone?\tone of the apples\n4\tcharacter\texplicit\t1\tWho had a garden?\ta certain king\n4\tsetting\texplicit\t1\tWhat grew in the garden?\ta tree of golden apples\n4\tcausal relationship\timplicit\t2,3\tWhy did the king need a watch?\tapples vanished at night\n5\tcharacter\texplicit\t1\tWho had a garden?\ta certain king\n5\tsetting\texplicit\t1\tWhere was the tree?\tin a beautiful garden\n5\taction\texplicit\t3\tWhat did the king order?\tto watch the tree\n6\tfeeling\timplicit\t3\tHow did the king feel?\tfurious\n6\tcharacter\texplicit\t1\tWho had the garden?\ta certain king\n6\taction\texplicit\t3\tWhat did the king do?\tordered a watch\n");
        let g = gate(&text, &[&a, &b, &c, &d, &e, &f, &h], Rule::V1);
        assert!(g.format.is_none(), "{:?}", g.format);
        let notes: Vec<(usize, &str)> = g.rej.iter().filter(|r| r.level != Level::Pair).map(|r| (r.p.unwrap(), r.reason.as_str())).collect();
        assert_eq!(notes, [(3, "accepted questions 2 < 3"), (3, "no implicit questions"), (3, "all questions of one type"), (5, "no implicit questions"), (7, "no questions")]);
        assert_eq!(g.ok.len(), 6 + 2 + 3 + 3 + 3);
    }

    #[test]
    fn span_norm_keeps_apostrophes_drops_quotes() {
        assert_eq!(span_norm("‘I will catch the King’s  thief,’ said he."), "i will catch the king's thief, said he.");
        assert_eq!(span_norm(" “Yes,”\tsaid the kings’ horse "), "yes, said the kings horse");
        assert_eq!(span_answer("“(A certain KING.)”"), "a certain king");
        assert!(is_span("the king", "it was the king's garden."));
        assert!(!is_span("the kin", "it was the king's garden."));
        assert!(!is_span("he king", "it was the king's garden."));
        assert!(!is_span("", "it was the king's garden."));
        // the first occurrence is not on a word boundary, the second is
        assert!(is_span("ring", "a string and a ring"));
    }

    /// Rule v2: an explicit answer is a span of an anchor sentence. Normalization ignores case, quotes,
    /// spaces and punctuation at the edges; negative controls: skipped and reordered words, part of a word,
    /// a span not from an anchor, different punctuation inside, two sentences. Under v1 the same pairs pass.
    #[test]
    fn span_gate_v2() {
        let (a, b) = (a(), b());
        let g = gate(GOOD, &[&a, &b], Rule::V2);
        assert_eq!(g.ok.len(), 6, "{:?}", g.rej);
        assert!(g.rej.is_empty(), "{:?}", g.rej);
        let row = |p: usize, anchors: &str, ans: &str| format!("{GOOD}{p}\taction\texplicit\t{anchors}\tWhat is it about?\t{ans}\n");
        let good = [
            (1, "1", "A CERTAIN KING."),
            (1, "1", "“a certain  king”"),
            (1, "1", " (a certain king) "),
            (1, "2,1", "certain king"),
            (1, "1", "a beautiful garden with a tree of golden apples"),
            (2, "1", "I will catch the thief"),
            (2, "1", "'I will catch the thief'"),
            (2, "1", "catch the thief, said the eldest son"),
            (2, "1", "‘I will catch the thief,’ said the eldest son."),
        ];
        for (p, anchors, ans) in good {
            let g = gate(&row(p, anchors, ans), &[&a, &b], Rule::V2);
            assert_eq!(g.ok.len(), 7, "{ans}: {:?}", g.rej);
        }
        let bad = [
            (1, "1", "a king", "explicit: not a span of an anchor sentence"),
            (1, "1", "king certain a", "explicit: not a span of an anchor sentence"),
            (1, "1", "a certain kin", "explicit: not a span of an anchor sentence"),
            (1, "1", "ertain king", "explicit: not a span of an anchor sentence"),
            (1, "3", "the king was angry", "explicit: not a span of an anchor sentence"),
            (1, "2,3", "was gone. The king was very angry", "explicit: not a span of an anchor sentence"),
            (1, "1", "one of the apples was gone", "explicit: span of sentence 2, which is not an anchor"),
            (1, "1", "a certain king had a beautiful garden, with a tree", "explicit: consecutive words in sentence 1, but different punctuation"),
            (2, "1", "the eldest son said I will catch the thief", "explicit: not a span of an anchor sentence"),
            (2, "1", "catching the thief", "explicit: not a span of an anchor sentence"),
        ];
        for (p, anchors, ans, why) in bad {
            let text = row(p, anchors, ans);
            let g = gate(&text, &[&a, &b], Rule::V2);
            let r: Vec<&Reject> = g.rej.iter().filter(|r| r.level == Level::Pair).collect();
            assert_eq!((g.ok.len(), r.len()), (6, 1), "{ans}: {:?}", g.rej);
            assert_eq!(r[0].reason, why, "{ans}");
            // rule v1 (pilot) does not require a span
            assert_eq!(gate(&text, &[&a, &b], Rule::V1).ok.len(), 7, "{ans}");
        }
        // implicit answers need no span; a verbatim implicit answer is rejected as before
        let g = gate(&format!("{GOOD}1\tcharacter\timplicit\t1\tWhat kind of man was the king?\trich and proud\n"), &[&a, &b], Rule::V2);
        assert_eq!(g.ok.len(), 7);
        let g = gate(&format!("{GOOD}1\tfeeling\timplicit\t3\tHow did the king feel?\tvery angry\n"), &[&a, &b], Rule::V2);
        assert_eq!(g.ok.len(), 6);
    }
}
