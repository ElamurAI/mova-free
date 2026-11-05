//! The pragmatic reading of Biblical Hebrew sentences, by rules on MACULA's words (17.10: «trees and pragmatics for
//! Ancient Hebrew»), in the columns of the parse database's `prag.tsv` (form, act, indirect, hedge, polarity, voice),
//! and its agreement with the English reading a teacher gave the same verses (JPS 1917).
//!
//! A sentence is first cut into its speech parts: the narrator's frame («And God said») and the quote after it
//! («Let there be light»). The quote is read as a character's speech, the frame as narration; the sentence's own row
//! follows the English convention (a sentence that holds a quote takes the quote's form and act, voice «character»)
//! and keeps the frame's reading in a column of its own. Speech is carried across sentences: a frame that ends a
//! verse («And the LORD spoke unto Moses, saying:») opens the next one, and a quote goes on until a narrator's
//! wayyiqtol takes the story up again. The sources of each rule are in the README.

use crate::{consonants, SAYING};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;

/// The rules, each of which can be left out (`--without a,b`) or put in (`--with a,b`) to measure what it gives.
pub const RULES: [&str; 17] = ["lemor", "ccomp", "amar", "carry", "close-qatal", "propose", "resolve", "promise", "rhetorical", "infabs", "first-form", "ask-first", "neg-polarity", "evaluate", "close-offline", "law3", "inner-frame"];
/// Words that judge, by their consonants: good, bless, rejoice, peace, kindness, love, save, favour …
const EVAL_POS: [&str; 10] = ["טוב", "ברך", "שמח", "שלום", "חסד", "אהב", "ישע", "חן", "רצון", "שמחה"];
/// … and evil, curse, sin, iniquity, wickedness, plague, anger, weeping, affliction, abomination.
const EVAL_NEG: [&str; 12] = ["רע", "רעה", "ארר", "קלל", "חטא", "עון", "רשע", "נגף", "חרה", "בכה", "עני", "תועבה"];

#[derive(Clone, Debug)]
pub struct Rules(BTreeSet<&'static str>);

/// Rules measured and not kept: off unless asked for (the README says why).
pub const OFF: [&str; 9] = ["amar", "inner-frame", "propose", "resolve", "rhetorical", "neg-polarity", "evaluate", "close-offline", "law3"];

impl Rules {
    /// The rules in use: all but those in `OFF`, with `with` put in and `without` left out.
    pub fn new(with: &[&str], without: &[&str]) -> Self {
        Rules(RULES.iter().copied().filter(|r| (!OFF.contains(r) || with.contains(r)) && !without.contains(r)).collect())
    }
    fn on(&self, r: &str) -> bool {
        self.0.contains(r)
    }
}

/// One reading: a sentence's, or one of its parts'.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reading {
    pub form: &'static str,
    pub act: &'static str,
    pub indirect: bool,
    pub hedge: bool,
    pub neg: bool,
    /// the speaker's evaluation, the English scheme's «polarity»: pos, neg or none
    pub pol: &'static str,
    pub voice: &'static str,
}

impl Reading {
    fn cols(&self) -> String {
        format!("{}\t{}\t{}\t{}\t{}\t{}", self.form, self.act, self.indirect as u8, self.hedge as u8, self.pol, self.voice)
    }
    fn short(&self) -> String {
        format!("{}/{}/{}", self.form, self.act, if self.neg { "neg" } else { "none" })
    }
}

/// What a sentence holds of speech.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speech {
    /// narration only
    None,
    /// a frame whose quote is in the next sentence («… saying:»)
    Frame,
    /// a frame and its quote
    Quote,
    /// inside a quote begun in an earlier sentence
    Cont,
    /// speech with no frame and none open before it (a verb in the first or second person, a volitive)
    Bare,
}

impl Speech {
    fn name(self) -> &'static str {
        match self {
            Speech::None => "-",
            Speech::Frame => "frame",
            Speech::Quote => "quote",
            Speech::Cont => "cont",
            Speech::Bare => "bare",
        }
    }
}

/// The words of one sentence, with what the rules look at.
struct Words<'a> {
    rows: &'a [Vec<&'a str>],
    kind: Vec<String>,
    lemma: Vec<String>,
    head: Vec<Option<usize>>,
}

impl<'a> Words<'a> {
    fn new(rows: &'a [Vec<&'a str>]) -> Self {
        let misc = |c: &Vec<&str>, k: &str| c[9].split('|').find_map(|kv| kv.strip_prefix(k)).unwrap_or("").to_string();
        Words {
            rows,
            kind: rows.iter().map(|c| misc(c, "Type=")).collect(),
            lemma: rows.iter().map(|c| consonants(c[2])).collect(),
            head: rows.iter().map(|c| c[6].parse::<usize>().ok().filter(|&h| h > 0).map(|h| h - 1)).collect(),
        }
    }
    fn len(&self) -> usize {
        self.rows.len()
    }
    fn person(&self, i: usize, p: &str) -> bool {
        self.rows[i][5].contains(&format!("Person={p}"))
    }
    fn rel(&self, i: usize) -> &str {
        self.rows[i][7]
    }
    fn kids(&self, i: usize) -> Vec<usize> {
        (0..self.len()).filter(|&k| self.head[k] == Some(i)).collect()
    }
    fn subtree(&self, i: usize) -> BTreeSet<usize> {
        let mut s = BTreeSet::from([i]);
        let mut grew = true;
        while grew {
            grew = false;
            for k in 0..self.len() {
                if !s.contains(&k) && self.head[k].is_some_and(|h| s.contains(&h)) {
                    s.insert(k);
                    grew = true;
                }
            }
        }
        s
    }
    /// An argument of a verb of saying that names whom it is said to: «אֵלָיו», «לוֹ», «עָלָיו» — an obl whose
    /// preposition is אֶל, לְ or עַל.
    fn addressee(&self, k: usize) -> bool {
        self.rel(k) == "obl" && self.kids(k).iter().any(|&c| self.rel(c) == "case" && matches!(self.lemma[c].as_str(), "אל" | "ל" | "על"))
    }
    fn root(&self) -> usize {
        self.head.iter().position(|h| h.is_none()).unwrap_or(0)
    }
    fn volitive(&self, i: usize) -> bool {
        matches!(self.kind[i].as_str(), "imperative" | "jussive" | "cohortative")
    }
    /// A narrator's clause: a wayyiqtol in the third person with nothing of the first or second person in its own
    /// clause — the mainline of a story (Cook 2008; Andrason 2011 on wayyiqtol as the narrative foreground).
    fn narrative(&self, i: usize) -> bool {
        self.kind[i] == "wayyiqtol" && !self.person(i, "1") && !self.person(i, "2") && !self.kids(i).iter().any(|&k| self.person(k, "1") || self.person(k, "2"))
    }
}

/// The parts of a sentence: the frame's verb of saying, if any, and the words of its quote (empty when the quote is
/// in the next sentence).
fn quote(w: &Words, rules: &Rules) -> Option<(usize, BTreeSet<usize>)> {
    let n = w.len();
    for i in 0..n {
        if w.rows[i][3] != "VERB" {
            continue;
        }
        // לֵאמֹר always opens a quote, whatever the verb before it (Miller-Naudé & Naudé: «a grammaticalized
        // infinitive that always introduces the quote»; the finite verb can be one of hearing, writing, coming)
        if rules.on("lemor") && w.kind[i] == "infinitive_construct" && w.lemma[i] == "אמר" && i > 0 && w.lemma[i - 1] == "ל" {
            // «לֵאמֹר לוֹ» with nothing more: the quote is in the next sentence
            let own = w.subtree(i);
            let q: BTreeSet<usize> = (i + 1..n).filter(|k| !own.contains(k)).collect();
            return Some((i, q));
        }
        // קָרָא with a name as its object names, it does not speak («and he called his name Enosh»)
        let naming = w.lemma[i] == "קרא" && w.kids(i).iter().any(|&k| w.rel(k) == "obj" && w.lemma[k] == "שם");
        if !SAYING.contains(&w.lemma[i].as_str()) || naming {
            continue;
        }
        let kids = w.kids(i);
        // the clause our trees hang on a verb of saying as its ccomp, and the clauses after it on the same verb —
        // except a narrator's wayyiqtol that takes the story up again («וַיְהִי אוֹר», «וַיְהִי כֵן»)
        // (לֵאמֹר is itself the ccomp of its verb when the quote is in the next sentence: the ל heads, אמר is fixed)
        let lemor_l = |k: usize| w.lemma[k] == "ל" && k + 1 < n && w.lemma[k + 1] == "אמר" && w.kind[k + 1] == "infinitive_construct";
        let start = if rules.on("ccomp") { kids.iter().copied().find(|&k| w.rel(k) == "ccomp" && !lemor_l(k)) } else { None };
        // after a finite אמר, what follows its own arguments is said: a quote can open with a conjunction (כִּי, וְעַתָּה),
        // a vocative or a nominal clause, which the ccomp rule of the trees does not take; a לֵאמֹר after it opens the
        // quote itself
        let lemor = |k: usize| w.kind[k] == "infinitive_construct" && w.lemma[k] == "אמר";
        let start = start.or_else(|| {
            (rules.on("amar") && w.lemma[i] == "אמר" && w.kind[i] != "infinitive_construct")
                .then(|| kids.iter().copied().find(|&k| k > i && !w.narrative(k) && !w.addressee(k) && !lemor(k) && !matches!(w.rel(k), "nsubj" | "obj" | "iobj" | "cc" | "case" | "punct")))
                .flatten()
        });
        let Some(c) = start else {
            // a finite אמר of the main line (the root, or a clause joined to it) that ends the sentence with only its own
            // arguments after it («וַיֹּאמֶר אֵלָיו»): its quote is in the next sentence
            let sub = w.subtree(i);
            let main_line = w.head[i].is_none() || w.rel(i) == "conj" && w.head[i] == Some(w.root());
            if rules.on("amar") && main_line && w.lemma[i] == "אמר" && w.kind[i] != "infinitive_construct" && (i + 1..n).all(|k| sub.contains(&k)) && !kids.iter().any(|&k| k > i && (w.narrative(k) || lemor(k))) {
                return Some((i, BTreeSet::new()));
            }
            // a verb of saying with no quote of its own: a later verb may frame it (וַיַּעַן … וַיֹּאמֶר, Miller-Naudé & Naudé)
            continue;
        };
        let mut q = BTreeSet::new();
        for &k in kids.iter().filter(|&&k| k >= c) {
            if k != c && (w.narrative(k) || matches!(w.rel(k), "cc" | "punct")) {
                if w.narrative(k) {
                    break;
                }
                continue;
            }
            q.extend(w.subtree(k));
        }
        // a conjunction of a quoted clause stays with it
        for k in 0..n {
            if w.rel(k) == "cc" && w.head[k].is_some_and(|h| q.contains(&h)) {
                q.insert(k);
            }
        }
        return Some((i, q));
    }
    None
}

/// The reading of a part: its heads are the words whose head is outside it, its own clause the heads and what hangs
/// on them directly.
fn read(w: &Words, scope: &BTreeSet<usize>, speech: bool, cont: bool, rules: &Rules) -> Reading {
    let heads: Vec<usize> = scope.iter().copied().filter(|&k| w.head[k].is_none_or(|h| !scope.contains(&h)) && !matches!(w.rel(k), "cc" | "punct" | "case")).collect();
    let own: BTreeSet<usize> = scope.iter().copied().filter(|&k| heads.contains(&k) || w.head[k].is_some_and(|h| heads.contains(&h))).collect();
    // the main clause: the first head that is a verb, else the first head
    let main = heads.iter().copied().find(|&k| w.rows[k][3] == "VERB").or(heads.first().copied()).unwrap_or(0);
    let mk = w.kind[main].as_str();
    let neg_word = |k: usize| w.kind[k] == "negative" || matches!(w.lemma[k].as_str(), "לא" | "אל" | "אין" | "בלי" | "בלת");
    let wh = own.iter().any(|&k| w.kind[k] == "interrogative" && !matches!(w.lemma[k].as_str(), "ה"));
    let q = !wh && own.iter().any(|&k| w.kind[k] == "interrogative");
    // הֲלֹא: a question that asserts («is it not …?» = «it is»)
    let rhetorical = rules.on("rhetorical") && q && own.iter().any(|&k| w.kind[k] == "interrogative" && w.lemma[k] == "ה" && k + 1 < w.len() && neg_word(k + 1));
    let please = own.iter().any(|&k| w.lemma[k] == "נא" && matches!(w.kind[k].as_str(), "exhortation" | "interjection"));
    let mut neg = own.iter().any(|&k| neg_word(k) && w.kind[k] != "interrogative");
    // the English scheme's polarity is the speaker's evaluation (docs/context.md), not negation: words that judge
    // (good, bless, rejoice … / evil, curse, sin …), the sign flipped by a negation of the same clause; with
    // «neg-polarity» a negation alone makes it «neg» (the first versions)
    let pol = {
        let (mut p, mut m) = (0, 0);
        if rules.on("evaluate") {
            for &k in scope {
                if EVAL_POS.contains(&w.lemma[k].as_str()) {
                    p += 1;
                } else if EVAL_NEG.contains(&w.lemma[k].as_str()) {
                    m += 1;
                }
            }
            if neg {
                std::mem::swap(&mut p, &mut m);
            }
        }
        if rules.on("neg-polarity") && neg || m > p {
            "neg"
        } else if p > m {
            "pos"
        } else {
            "none"
        }
    };
    if !speech {
        let act = if mk == "wayyiqtol" { "narrate" } else { "assert" };
        return Reading { form: "decl", act, indirect: false, hedge: please, neg, pol, voice: "narrator" };
    }
    // an infinitive absolute heading a clause of speech gives a command (זָכוֹר «remember», שָׁמוֹר «keep»)
    let infabs = |k: usize| rules.on("infabs") && w.kind[k] == "infinitive_absolute";
    let volitive = heads.iter().any(|&k| w.volitive(k) || infabs(k)) || own.iter().any(|&k| w.volitive(k) && w.rel(k) == "conj");
    let second = |k: usize| matches!(w.kind[k].as_str(), "yiqtol" | "weqatal") && w.person(k, "2");
    // in a speech carried on from an earlier sentence (the law), a yiqtol or weqatal in the third person at the head
    // prescribes («he shall put off his garments»), when the speaker does not speak of himself (rule «law3»)
    let law3 = rules.on("law3") && cont && matches!(mk, "yiqtol" | "weqatal") && w.person(main, "3") && !scope.iter().any(|&k| w.person(k, "1"));
    // a command of the law: «thou shalt (not) …» — a yiqtol or weqatal in the second person (Cook 2008: weqatal a
    // modal form; Racher 2017: לֹא + yiqtol a general prohibition; אַל + jussive, a specific one, is a volitive)
    let command = heads.iter().any(|&k| second(k)) || law3;
    let imperative = heads.iter().any(|&k| w.kind[k] == "imperative") || own.iter().any(|&k| w.kind[k] == "imperative");
    let cohortative = w.kind[main] == "cohortative" && !imperative && !command;
    // a cohortative in the plural (or a jussive with נָא), no imperative: a proposal, «let us …» (Racher 2017 after
    // Christiansen 2009: נָא with a cohortative or jussive is propositive); in the singular the speaker's resolve,
    // «I will go» — he binds himself (rule «resolve»)
    let plural = w.rows[main][5].contains("Number=Plur");
    let propose = rules.on("propose") && (cohortative && (plural || !rules.on("resolve")) || please && w.kind[main] == "jussive" && !imperative);
    let resolve = rules.on("resolve") && cohortative && !plural;
    // the speaker binds himself: a yiqtol or weqatal in the first person at the head of the clause, not a question
    let promise = rules.on("promise") && !volitive && !command && matches!(mk, "yiqtol" | "weqatal") && w.person(main, "1") || resolve;
    if rhetorical {
        neg = false;
    }
    // the form is the first clause's (the main one), as the English reading gives it; the act any clause's
    let main_volitive = w.volitive(main) || infabs(main);
    let form = if rules.on("first-form") {
        if main_volitive { "imp" } else if wh { "wh" } else if q { "q" } else { "decl" }
    } else if volitive && !wh && !q {
        "imp"
    } else if wh {
        "wh"
    } else if q {
        "q"
    } else {
        "decl"
    };
    // a question asks, though its verb be in the second person («wherefore smitest thou …?»)
    let ask_first = rules.on("ask-first") && (wh || q) && !main_volitive;
    let act = if rhetorical {
        "assert"
    } else if ask_first {
        "ask"
    } else if propose {
        "suggest"
    } else if resolve {
        "promise"
    } else if volitive || command {
        "request"
    } else if wh || q {
        "ask"
    } else if promise {
        "promise"
    } else {
        "assert"
    };
    let pol = if rhetorical && pol == "neg" && !rules.on("evaluate") { "none" } else { pol };
    Reading { form, act, indirect: command && !volitive, hedge: please, neg, pol, voice: "character" }
}

/// One sentence read: its row (by the English convention), what it holds of speech, the frame's own reading, and
/// whether a quote is open after it.
pub struct Read {
    pub row: Reading,
    pub speech: Speech,
    pub frame: Option<Reading>,
    pub open: bool,
    /// the sentence shows speech on its own, read alone: a word in the first or second person, a volitive, a
    /// question word, נָא — what a reader of the one sentence, with no context, can see
    pub marked: bool,
    /// a negation anywhere in the sentence (לֹא, אַל, אַיִן, בְּלִי …), in the frame or the quote, a main clause or not
    pub negation: bool,
}

pub fn read_sentence(rows: &[Vec<&str>], open_before: bool, rules: &Rules) -> Read {
    let w = Words::new(rows);
    let all: BTreeSet<usize> = (0..w.len()).collect();
    let root = w.root();
    let marks = |k: usize| w.person(k, "1") || w.person(k, "2") || w.volitive(k) || w.kind[k] == "interrogative" || w.lemma[k] == "נא" && w.kind[k] == "exhortation";
    let speaks_root = marks(root) || (0..w.len()).any(|k| w.head[k] == Some(root) && marks(k));
    let marked = (0..w.len()).any(marks);
    let negation = (0..w.len()).any(|k| w.kind[k] == "negative");
    let at = |row: Reading, speech: Speech, frame: Option<Reading>, open: bool| Read { row, speech, frame, open, marked, negation };
    if let Some((f, q)) = quote(&w, rules) {
        let rest: BTreeSet<usize> = all.difference(&q).copied().collect();
        // a frame inside speech: its verb is in the first or second person («וָאֹמַר», «וְאָמַרְתָּ»), or a quote is open
        // and the verb is not a narrator's wayyiqtol («lest thou say: …», «and the LORD said unto me: …»)
        let inner = rules.on("inner-frame") && (w.person(f, "1") || w.person(f, "2") || open_before && rules.on("carry") && !w.narrative(f));
        let frame = read(&w, &rest, inner, false, rules);
        if q.is_empty() {
            return at(frame.clone(), if inner { Speech::Cont } else { Speech::Frame }, Some(frame), true);
        }
        let mut row = read(&w, &q, true, false, rules);
        row.neg |= frame.neg;
        // the quote reaches the end of the sentence: it may go on in the next one
        let open = rules.on("carry") && q.contains(&(w.len() - 1));
        return at(row, Speech::Quote, Some(frame), open);
    }
    // a narrator takes the story up again with a wayyiqtol (or, when «close-qatal», a qatal) in the third person
    let closes = w.narrative(root) || rules.on("close-qatal") && w.kind[root] == "qatal" && !marked || rules.on("close-offline") && !marked && !matches!(w.kind[root].as_str(), "yiqtol" | "weqatal" | "imperative" | "jussive" | "cohortative" | "infinitive_absolute");
    if open_before && rules.on("carry") && !closes {
        return at(read(&w, &all, true, true, rules), Speech::Cont, None, true);
    }
    if speaks_root {
        return at(read(&w, &all, true, false, rules), Speech::Bare, None, rules.on("carry"));
    }
    at(read(&w, &all, false, false, rules), Speech::None, None, false)
}

/// The trees with their words only: multiword-token lines and marks of punctuation left out, words numbered anew
/// (the rules read words and their heads, not the written tokens).
fn words_only(text: &str) -> String {
    let mut out = String::new();
    for block in text.split("\n\n") {
        let rows: Vec<Vec<&str>> = block.lines().filter(|l| !l.starts_with('#')).map(|l| l.split('\t').collect::<Vec<_>>()).filter(|c| c.len() == 10 && !c[0].contains('-') && !c[0].contains('.')).collect();
        let keep: BTreeMap<&str, usize> = rows.iter().filter(|c| c[3] != "PUNCT").enumerate().map(|(k, c)| (c[0], k + 1)).collect();
        for l in block.lines().filter(|l| l.starts_with('#')) {
            out.push_str(l);
            out.push('\n');
        }
        for c in rows.iter().filter(|c| c[3] != "PUNCT") {
            let head = if c[6] == "0" { "0".to_string() } else { keep.get(c[6]).map_or("0".to_string(), |h| h.to_string()) };
            out.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\t{head}\t{}\t{}\t{}\n", keep[c[0]], c[1], c[2], c[3], c[4], c[5], c[7], c[8], c[9]));
        }
        out.push('\n');
    }
    out
}

fn sentences(text: &str) -> Vec<(String, Vec<Vec<&str>>)> {
    text.split("\n\n")
        .filter_map(|block| {
            let id = block.lines().find_map(|l| l.strip_prefix("# sent_id = "))?;
            let rows: Vec<Vec<&str>> = block.lines().filter(|l| !l.starts_with('#')).map(|l| l.split('\t').collect::<Vec<_>>()).filter(|c| c.len() == 10 && !c[0].contains('-')).collect();
            Some((id.to_string(), rows))
        })
        .collect()
}

/// `macula prag <trees-dir> <out.tsv> [--without rule,…]`: one row per sentence, in the order of the books' files;
/// speech state is carried from sentence to sentence within a book.
pub fn prag(trees: &str, out: &str, rules: &Rules) {
    let mut files: Vec<_> = fs::read_dir(trees).expect("trees").flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "conllu")).collect();
    files.sort();
    let mut tsv = String::from("sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tteacher\tspeech\tmarked\tnegation\tframe\n");
    let mut n = 0;
    let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
    for f in files {
        let text = words_only(&fs::read_to_string(f).expect("ours"));
        let mut open = false;
        for (id, rows) in sentences(&text) {
            let r = read_sentence(&rows, open, rules);
            open = r.open;
            *kinds.entry(r.speech.name()).or_default() += 1;
            let frame = r.frame.as_ref().map(Reading::short).unwrap_or_else(|| "-".into());
            tsv.push_str(&format!("{id}\t{}\trules\t{}\t{}\t{}\t{frame}\n", r.row.cols(), r.speech.name(), r.marked as u8, r.negation as u8));
            n += 1;
        }
    }
    fs::write(out, tsv).expect("write prag");
    println!("{n} Hebrew sentences read → {out}; speech: {}", kinds.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
}

/// JPS 1917 marks direct speech with single quotes: for each English sentence (in the book's order), whether it is
/// all or partly inside a quote, and all or partly outside one. An opening quote follows a space or the start and
/// comes before a letter; a closing one comes before a space, punctuation or the end — «'s» and «s' » (possessives)
/// are not quotes. Quotes are not nested: a quote opened while one is open (a new paragraph of the same speech, or a
/// quote inside a quote) keeps it open, and the next closing quote closes it — counting depth drifts at the
/// paragraphs JPS re-opens without closing.
pub fn jps_quoted(text: &str, open: &mut bool) -> (bool, bool) {
    let c: Vec<char> = text.chars().collect();
    let (mut inside, mut outside) = (false, false);
    for i in 0..c.len() {
        let letter = |j: usize| c.get(j).is_some_and(|x| x.is_alphanumeric());
        if c[i] != '\'' {
            if c[i].is_alphabetic() {
                if *open {
                    inside = true;
                } else {
                    outside = true;
                }
            }
            continue;
        }
        let prev_space = i == 0 || c[i - 1].is_whitespace() || c[i - 1] == '(' || c[i - 1] == '—';
        if prev_space && letter(i + 1) {
            *open = true;
        } else if !letter(i + 1) && !(i > 0 && c[i - 1] == 's' && c.get(i + 1).is_some_and(|x| x.is_whitespace())) {
            *open = false;
        }
    }
    (inside, outside)
}

/// Searle's classes of the teacher's acts: directive (request, suggest, warn), question (ask), commissive (promise,
/// offer), expressive (express, thank), and statement for the rest (assert, narrate, evaluate, agree, disagree, other).
pub fn act_class(a: &str) -> &'static str {
    match a {
        "request" | "suggest" | "warn" => "directive",
        "ask" => "question",
        "promise" | "offer" => "commissive",
        "express" | "thank" => "expressive",
        _ => "statement",
    }
}

/// `macula prag-compare <hbo-prag.tsv> <trees>/verses.tsv <en prag.tsv> <en pool.tsv.verses>`: agreement of the
/// Hebrew reading with the English one, verse by verse, for verses that are one Hebrew sentence and have English
/// sentences. A verse's English reading is «request» if any of its sentences is, then «ask», else its first
/// sentence's; voice «character» if any sentence's is; polarity «neg» if any is. Besides the raw agreement it gives
/// the agreement on what the two schemes can both say (polarity «pos» is «none», acts by Searle's classes), the
/// agreement on speech with the quotes of JPS 1917 (`<en pool.tsv>` next to the verses map), and each field by what
/// the Hebrew verse holds of speech.
pub fn compare(hbo: &str, hbo_verses: &str, en: &str, en_verses: &str) {
    let read = |p: &str| -> BTreeMap<String, Vec<String>> { fs::read_to_string(p).expect("prag").lines().skip(1).filter_map(|l| l.split_once('\t').map(|(id, r)| (id.to_string(), r.split('\t').map(String::from).collect()))).collect() };
    let map = |p: &str| -> BTreeMap<String, String> { fs::read_to_string(p).expect("verses").lines().filter_map(|l| l.split_once('\t')).map(|(a, b)| (a.to_string(), b.to_string())).collect() };
    let (h, hv, e, ev) = (read(hbo), map(hbo_verses), read(en), map(en_verses));
    // the English text, for its quotes, in the book's order
    let pool = en_verses.strip_suffix(".verses").unwrap_or(en_verses);
    let text = map(pool);
    let order = |id: &str| -> (String, usize) {
        let mut p = id.rsplitn(2, ':');
        let n = p.next().and_then(|x| x.parse().ok()).unwrap_or(0);
        (p.next().unwrap_or("").to_string(), n)
    };
    let mut ids: Vec<&String> = ev.keys().collect();
    ids.sort_by_key(|id| order(id));
    let mut quoted: BTreeMap<String, (bool, bool)> = BTreeMap::new();
    // verses whose English has a negation word
    let mut en_neg: BTreeSet<String> = BTreeSet::new();
    let (mut open, mut book) = (false, String::new());
    for id in ids {
        let b = order(id).0;
        if b != book {
            open = false;
            book = b;
        }
        let q = jps_quoted(text.get(id).map(String::as_str).unwrap_or(""), &mut open);
        let e = quoted.entry(ev[id].clone()).or_insert((false, false));
        e.0 |= q.0;
        e.1 |= q.1;
        if english_negation(text.get(id).map(String::as_str).unwrap_or("")) {
            en_neg.insert(ev[id].clone());
        }
    }
    let mut en_by_verse: BTreeMap<String, Vec<&Vec<String>>> = BTreeMap::new();
    for (id, v) in &ev {
        if let Some(r) = e.get(id) {
            en_by_verse.entry(v.clone()).or_default().push(r);
        }
    }
    let fields = ["form", "act", "polarity", "voice", "act class", "polarity (pos = none)", "voice read alone"];
    let mut agree = [0usize; 7];
    let mut n = 0usize;
    let mut conf: BTreeMap<(String, String, String), usize> = BTreeMap::new();
    // by what the Hebrew verse holds of speech: (n, form, act class, voice, voice read alone)
    let mut by: BTreeMap<String, [usize; 5]> = BTreeMap::new();
    // speech against the quotes of JPS, by book: [Hebrew speech, the teacher's voice][says speech][JPS quoted]
    let mut sq: BTreeMap<String, [[[usize; 2]; 2]; 2]> = BTreeMap::new();
    // negation: [Hebrew sentence has one][English verse has one]
    let mut ng = [[0usize; 2]; 2];
    // how often each English value is, field by field: the constant answer is the floor a rule must beat
    let mut base: [BTreeMap<String, usize>; 7] = Default::default();
    for (id, verse) in &hv {
        if verse.contains(' ') {
            continue;
        }
        let (Some(hr), Some(ers)) = (h.get(id), en_by_verse.get(verse)) else { continue };
        let pick = |i: usize, first: &[&str]| -> String {
            for f in first {
                if ers.iter().any(|r| r[i] == *f) {
                    return f.to_string();
                }
            }
            ers[0][i].clone()
        };
        // columns: form 0, act 1, indirect 2, hedge 3, polarity 4, voice 5, teacher 6, speech 7, marked 8, negation 9
        let kind = hr.get(7).cloned().unwrap_or_else(|| "?".into());
        // the voice a reader of the one sentence would give: speech it frames or shows on its own
        let alone = if kind == "quote" || kind == "bare" || kind == "cont" && hr.get(8).is_some_and(|m| m == "1") { "character" } else { "narrator" };
        let ea = pick(1, &["request", "ask"]);
        let ep = pick(4, &["neg"]);
        let ev = pick(5, &["character"]);
        let ens = [pick(0, &["imp", "wh", "q"]), ea.clone(), ep.clone(), ev.clone(), act_class(&ea).into(), if ep == "pos" { "none".into() } else { ep }, ev];
        let hes = [hr[0].clone(), hr[1].clone(), hr[4].clone(), hr[5].clone(), act_class(&hr[1]).into(), hr[4].clone(), alone.into()];
        n += 1;
        for i in 0..7 {
            if ens[i] == hes[i] {
                agree[i] += 1;
            } else if i != 5 && i != 6 {
                *conf.entry((fields[i].to_string(), ens[i].clone(), hes[i].clone())).or_default() += 1;
            }
        }
        let b = by.entry(kind).or_default();
        b[0] += 1;
        for (j, f) in [0, 4, 3, 6].iter().enumerate() {
            b[j + 1] += (ens[*f] == hes[*f]) as usize;
        }
        for i in 0..7 {
            *base[i].entry(ens[i].clone()).or_default() += 1;
        }
        if let Some(m) = hr.get(9) {
            ng[(m == "1") as usize][en_neg.contains(verse) as usize] += 1;
        }
        if let Some(&(inside, _)) = quoted.get(verse) {
            let t = sq.entry(verse.split('.').next().unwrap_or("").to_string()).or_default();
            t[0][(hes[3] == "character") as usize][inside as usize] += 1;
            t[1][(ens[3] == "character") as usize][inside as usize] += 1;
        }
    }
    let pc = |x: usize, n: usize| 100.0 * x as f64 / n.max(1) as f64;
    println!("{n} verses: agreement with the English reading — form {:.1}%, act {:.1}%, polarity {:.1}%, voice {:.1}%", pc(agree[0], n), pc(agree[1], n), pc(agree[2], n), pc(agree[3], n));
    println!("  on what both schemes can say — act by Searle's class {:.1}%, polarity with «pos» read as «none» {:.1}%, voice of the sentence read alone {:.1}%", pc(agree[4], n), pc(agree[5], n), pc(agree[6], n));
    let floor: Vec<String> = base.iter().zip(fields).map(|(b, f)| { let (v, k) = b.iter().max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0))).map(|(v, k)| (v.as_str(), *k)).unwrap_or(("-", 0)); format!("{f} «{v}» {:.1}%", pc(k, n)) }).collect();
    println!("  the constant answer, for comparison: {}", floor.join(", "));
    let (tp, fp, fn_, tn) = (ng[1][1], ng[1][0], ng[0][1], ng[0][0]);
    if tp + fp + fn_ + tn > 0 {
        println!("  negation against the English negation words (not, no, nor, neither, never, nothing, none, nought, cannot): agree {:.1}%, precision {:.1}%, recall {:.1}% (both {tp}, only Hebrew {fp}, only English {fn_}, neither {tn})", pc(tp + tn, n), pc(tp, tp + fp), pc(tp, tp + fn_));
    }
    let mut c: Vec<_> = conf.into_iter().collect();
    c.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!("  most frequent disagreements (field: English → Hebrew): {}", c.iter().take(14).map(|((f, a, b), k)| format!("{f}: {a}→{b} {k}")).collect::<Vec<_>>().join(", "));
    for (k, b) in &by {
        println!("  Hebrew speech «{k}»: {} verses — form {:.1}%, act class {:.1}%, voice {:.1}%, voice read alone {:.1}%", b[0], pc(b[1], b[0]), pc(b[2], b[0]), pc(b[3], b[0]), pc(b[4], b[0]));
    }
    println!("  speech against the quotes of JPS 1917 (JPS leaves the law of LEV and Moses' speech in DEU unquoted): agree / precision / recall");
    let line = |t: &[[usize; 2]; 2]| {
        let (tp, fp, fn_, tn) = (t[1][1], t[1][0], t[0][1], t[0][0]);
        format!("{:.1} / {:.1} / {:.1}", pc(tp + tn, tp + fp + fn_ + tn), pc(tp, tp + fp), pc(tp, tp + fn_))
    };
    for (book, t) in &sq {
        println!("    {book}: Hebrew {}, teacher {} ({} verses, {} quoted)", line(&t[0]), line(&t[1]), t[0][0][0] + t[0][0][1] + t[0][1][0] + t[0][1][1], t[0][0][1] + t[0][1][1]);
    }
}

/// `macula prag-quotes <en pool.tsv>`: each English sentence with whether JPS has it inside a quote (in, out) and the
/// whether a quote is open after it — to see why the speech measure says what it says.
pub fn quotes(pool: &str) {
    let text: BTreeMap<String, String> = fs::read_to_string(pool).expect("pool").lines().filter_map(|l| l.split_once('\t')).map(|(a, b)| (a.to_string(), b.to_string())).collect();
    let order = |id: &str| -> (String, usize) {
        let mut p = id.rsplitn(2, ':');
        let n = p.next().and_then(|x| x.parse().ok()).unwrap_or(0);
        (p.next().unwrap_or("").to_string(), n)
    };
    let mut ids: Vec<&String> = text.keys().collect();
    ids.sort_by_key(|id| order(id));
    let (mut open, mut book) = (false, String::new());
    for id in ids {
        let b = order(id).0;
        if b != book {
            open = false;
            book = b;
        }
        let (i, o) = jps_quoted(&text[id], &mut open);
        println!("{id}\t{}\t{}\t{}\t{}", i as u8, o as u8, open as u8, text[id]);
    }
}

/// Whether an English sentence has a negation word: not, no, nor, neither, never, nothing, none, nought, naught,
/// cannot, nay.
pub fn english_negation(text: &str) -> bool {
    text.split(|c: char| !c.is_alphabetic()).any(|w| matches!(w.to_lowercase().as_str(), "not" | "no" | "nor" | "neither" | "never" | "nothing" | "none" | "nought" | "naught" | "cannot" | "nay"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CoNLL-U with fields split by spaces (no field holds one), as our trees give them: EXO 6:10–12 and GEN 1:3.
    const EXO_6_10: &str = "# sent_id = t:0
1 וַ וְ CCONJ _ _ 2 cc _ _
2 יְדַבֵּר דָבַר VERB _ Number=Sing|Person=3 0 root _ Type=wayyiqtol
3 יְהוָה יהוה PROPN _ _ 2 nsubj _ _
4 אֶל־ אֶל ADP _ _ 5 case _ _
5 מֹשֶׁה מֹשֶׁה PROPN _ _ 2 obl _ _
6 לֵּ לְ ADP _ _ 7 case _ _
7 אמֹר אָמַר VERB _ _ 2 advcl _ Type=infinitive_construct";
    const EXO_6_11: &str = "# sent_id = t:1
1 בֹּא בּוֹא VERB _ Number=Sing|Person=2 0 root _ Type=imperative
2 דַבֵּר דָבַר VERB _ Number=Sing|Person=2 1 conj _ Type=imperative
3 אֶל־ אֶל ADP _ _ 4 case _ _
4 פַּרְעֹה פַּרְעֹה PROPN _ _ 2 obl _ _
5 וִ וְ CCONJ _ _ 6 cc _ _
6 ישַׁלַּח שָׁלַח VERB _ Number=Sing|Person=3 1 conj _ Type=yiqtol
7 אֶת־ אֵת ADP _ _ 8 case _ Type=direct_object_marker
8 בְּנֵי־ בֵּן NOUN _ Number=Plur 6 obj _ _
9 יִשְׂרָאֵל יִשְׂרָאֵל PROPN _ _ 8 compound:smixut _ _";
    const EXO_6_12: &str = "# sent_id = t:2
1 וַ וְ CCONJ _ _ 2 cc _ _
2 יְדַבֵּר דָבַר VERB _ Number=Sing|Person=3 0 root _ Type=wayyiqtol
3 מֹשֶׁה מֹשֶׁה PROPN _ _ 2 nsubj _ _
4 לֵ לְ ADP _ _ 5 case _ _
5 אמֹר אָמַר VERB _ _ 2 advcl _ Type=infinitive_construct
6 הֵן הֵן INTJ _ _ 2 ccomp _ Type=demonstrative
7 בְּנֵי־ בֵּן NOUN _ Number=Plur 10 nsubj _ _
8 יִשְׂרָאֵל יִשְׂרָאֵל PROPN _ _ 7 compound:smixut _ _
9 לֹא־ לֹא ADV _ _ 10 advmod _ Type=negative
10 שָׁמְעוּ שָׁמַע VERB _ Number=Plur|Person=3 6 conj _ Type=qatal
11 וְ וְ CCONJ _ _ 13 cc _ _
12 אֵיךְ אֵיךְ INTJ _ _ 13 advmod _ Type=interrogative
13 יִשְׁמָע שָׁמַע VERB _ Number=Sing|Person=3 2 conj _ Type=yiqtol
14 ֵנִי הוּא PRON _ Number=Sing|Person=1 13 obj _ Type=pronominal
15 פַרְעֹה פַּרְעֹה PROPN _ _ 13 nsubj _ _";
    const GEN_1_3: &str = "# sent_id = t:3
1 וַ וְ CCONJ _ _ 2 cc _ _
2 יֹּאמֶר אָמַר VERB _ Number=Sing|Person=3 0 root _ Type=wayyiqtol
3 אֱלֹהִים אֱלֹהִים NOUN _ Number=Plur 2 nsubj _ _
4 יְהִי הָיָה VERB _ Number=Sing|Person=3 2 ccomp _ Type=jussive
5 אוֹר אוֹר NOUN _ Number=Sing 4 nsubj _ _
6 וַ וְ CCONJ _ _ 7 cc _ _
7 יְהִי־ הָיָה VERB _ Number=Sing|Person=3 2 conj _ Type=wayyiqtol
8 אוֹר אוֹר NOUN _ Number=Sing 7 nsubj _ _";

    fn tabs(s: &str) -> String {
        s.lines().map(|l| if l.starts_with('#') { l.to_string() } else { l.split(' ').collect::<Vec<_>>().join("\t") }).collect::<Vec<_>>().join("\n")
    }

    /// The sentences read in order, speech carried from one to the next: (speech, form, act, voice, frame).
    fn story(texts: &[&str], rules: &Rules) -> Vec<(Speech, &'static str, &'static str, &'static str, Option<String>)> {
        let mut open = false;
        texts
            .iter()
            .map(|t| {
                let t = tabs(t);
                let s = sentences(&t);
                let r = read_sentence(&s[0].1, open, rules);
                open = r.open;
                (r.speech, r.row.form, r.row.act, r.row.voice, r.frame.as_ref().map(Reading::short))
            })
            .collect()
    }

    #[test]
    fn frame_and_quote() {
        let all = Rules::new(&[], &[]);
        let got = story(&[EXO_6_10, EXO_6_11, EXO_6_12, GEN_1_3], &all);
        // «And the LORD spoke unto Moses, saying:» — a frame; its quote is the next verse
        assert_eq!(got[0], (Speech::Frame, "decl", "narrate", "narrator", Some("decl/narrate/none".into())));
        // «Go in, speak unto Pharaoh …» — inside the quote: a command, a character speaks
        assert_eq!(got[1], (Speech::Cont, "imp", "request", "character", None));
        // «And Moses spoke before the LORD, saying: … how then shall Pharaoh hear me?» — the frame narrates, the quote asks
        assert_eq!(got[2], (Speech::Quote, "wh", "ask", "character", Some("decl/narrate/none".into())));
        // «And God said: 'Let there be light.' And there was light.» — the narrator's «and there was light» is no part
        // of the quote
        assert_eq!(got[3], (Speech::Quote, "imp", "request", "character", Some("decl/narrate/none".into())));
        let w = tabs(GEN_1_3);
        let s = sentences(&w);
        let words = Words::new(&s[0].1);
        assert_eq!(quote(&words, &all), Some((1, BTreeSet::from([3, 4]))));
    }

    #[test]
    fn rules_off_show_red() {
        // negative control: with no rule the frame is not cut off and the quote not carried — the same verses read
        // otherwise, so the test above can fail
        let none = Rules::new(&[], &RULES);
        let got = story(&[EXO_6_10, EXO_6_11, EXO_6_12, GEN_1_3], &none);
        assert_eq!(got[0].0, Speech::None);
        assert_ne!(got[2].0, Speech::Quote);
        // the jussive of the quote is seen only as a mark of speech: no frame, no command
        assert_eq!(got[3], (Speech::Bare, "decl", "assert", "character", None));
    }

    #[test]
    fn narrator_closes_quote() {
        // «And Moses went» after the quote: a narrator's wayyiqtol in the third person takes the story up again
        const WENT: &str = "# sent_id = t:4
1 וַ וְ CCONJ _ _ 2 cc _ _
2 יֵּלֶךְ הָלַךְ VERB _ Number=Sing|Person=3 0 root _ Type=wayyiqtol
3 מֹשֶׁה מֹשֶׁה PROPN _ _ 2 nsubj _ _";
        let all = Rules::new(&[], &[]);
        let got = story(&[EXO_6_10, EXO_6_11, WENT], &all);
        assert_eq!(got[1].0, Speech::Cont);
        assert_eq!(got[2], (Speech::None, "decl", "narrate", "narrator", None));
        // without «carry» the quote does not go on into the next verse
        let got = story(&[EXO_6_10, EXO_6_11], &Rules::new(&[], &["carry"]));
        assert_eq!(got[1].0, Speech::Bare);
    }
    #[test]
    fn jps_quotes() {
        let mut open = false;
        assert_eq!(jps_quoted("And God said: 'Let there be light.' And there was light.", &mut open), (true, true));
        assert!(!open);
        // possessives are not quotes
        assert_eq!(jps_quoted("and Aaron's sons, the priests, and his sons' wives", &mut open), (false, true));
        assert!(!open);
        // a quote open at the end of one sentence goes on in the next, and closes there
        assert_eq!(jps_quoted("And the LORD said: 'Go;", &mut open), (true, true));
        assert!(open);
        assert_eq!(jps_quoted("and speak unto Pharaoh.' And he went.", &mut open), (true, true));
        assert!(!open);
        assert_eq!(jps_quoted("Thou shalt not steal.", &mut open), (false, true));
    }

    #[test]
    fn negation_and_acts() {
        assert!(english_negation("Thou shalt not steal."));
        assert!(english_negation("Nay, but thou didst laugh."));
        assert!(!english_negation("And it was so; nothingness"));
        assert_eq!(act_class("suggest"), "directive");
        assert_eq!(act_class("narrate"), "statement");
        assert_eq!(act_class("offer"), "commissive");
    }
}
