//! Derived words: a form the dictionary does not know, read as a known word with productive affixes
//! (*unsupervised* = un- + supervised, *tokenised* = British spelling of *tokenized* = token + -ize + -ed,
//! *incrementally* = increment + -al + -ly). The rules are data: ```` ```derive ```` blocks in the word-formation seeds
//! (`en/seeds/morph/wf-*.md`, `adv-ly.md`), compiled into the binary; nothing is read from disk at run time.
//!
//! Conservative by construction:
//! - only a form the dictionary does not know is analysed: dictionary words keep their analysis (*size* is no -ize
//!   verb, *table* no -able adjective);
//! - every analysis ends in a dictionary word of an open class (noun, verb, adjective, adverb) of at least three letters;
//!   after a prefix at least the rule's shortest stem (4 or 5 letters) remains;
//! - each affix names the classes of stem it attaches to (-ness to adjectives, -ize to nouns and adjectives);
//! - inflections (-s, -ed, -ing) apply only to a word derived here: the forms of dictionary words are in the dictionary;
//! - at most `DEPTH` steps.
//!
//! Every analysis can say why (`Derivation::why`): «unsupervised = un- (not; reverse the action) + supervised».
//!
//! Reverse direction (`compose`): from a stem and the steps, the form is rebuilt by the same spelling rules; an
//! analysis is kept only when it composes back to the form (the round trip is a test).

use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::dict;
use crate::gram::Tag;

/// The seeds with ```` ```derive ```` blocks, compiled in.
const SEEDS: [(&str, &str); 7] = [
    ("wf-negative-prefixes.md", include_str!("../seeds/morph/wf-negative-prefixes.md")),
    ("wf-prefixes-neutral.md", include_str!("../seeds/morph/wf-prefixes-neutral.md")),
    ("wf-adj-suffixes.md", include_str!("../seeds/morph/wf-adj-suffixes.md")),
    ("adv-ly.md", include_str!("../seeds/morph/adv-ly.md")),
    ("wf-noun-suffixes.md", include_str!("../seeds/morph/wf-noun-suffixes.md")),
    ("wf-verb-suffixes.md", include_str!("../seeds/morph/wf-verb-suffixes.md")),
    ("wf-spelling-variants.md", include_str!("../seeds/morph/wf-spelling-variants.md")),
];

/// Most steps in one analysis (un- + token + -ize + -ed is three).
pub const DEPTH: usize = 4;
/// Shortest dictionary word at the bottom of an analysis.
pub const MIN_BASE: usize = 4;
/// Shortest bottom word known only from AGID, not from the UD treebanks: rare AGID words (*cive, poss, synd, mage,
/// pone*) under a misspelling or a name would otherwise make it a derivation (*token* has five letters).
pub const MIN_RARE_BASE: usize = 5;

/// A form the UD treebanks have.
fn in_ud(lower: &str) -> bool {
    dict::form(lower).is_some_and(|f| dict::analyses(f).any(|r| r.src & dict::UD != 0))
}

/// Coarse word classes as bits: noun, verb, adjective, adverb.
pub const N: u8 = 1;
pub const V: u8 = 2;
pub const A: u8 = 4;
pub const R: u8 = 8;

/// The coarse class of a PTB tag (0 for closed classes and proper nouns).
pub fn class(t: Tag) -> u8 {
    match t {
        Tag::NN | Tag::NNS => N,
        Tag::VB | Tag::VBD | Tag::VBG | Tag::VBN | Tag::VBP | Tag::VBZ => V,
        Tag::JJ | Tag::JJR | Tag::JJS => A,
        Tag::RB | Tag::RBR | Tag::RBS => R,
        _ => 0,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Kind {
    Prefix,
    Suffix,
    Inflect,
    Variant,
}

/// One rule from a seed.
#[derive(Clone, Debug)]
pub struct Rule {
    pub kind: Kind,
    /// The affix (for a variant: the British ending).
    pub affix: String,
    /// Prefix, suffix, inflection: the stem classes it attaches to.
    pub stem: u8,
    /// Suffix, inflection: the tags of the result (a prefix keeps the stem's tags).
    pub tags: Vec<Tag>,
    /// Prefix: shortest stem after it.
    pub min: usize,
    /// Optional `after=`: endings the stem must have (`after=ate,ct`; empty: any).
    pub after: Vec<String>,
    /// Optional `except=`: endings the stem must not have (in- does not take an adverb in -ly: *in- + visibly*).
    pub except: Vec<String>,
    /// Optional `block=`: rival affixes; when one of them with the same stem makes a dictionary word, this rule is
    /// blocked (*impossible* blocks *unpossible*, *readable* blocks *readible*; Aronoff 1976, blocking).
    pub block: Vec<String>,
    /// Prefix, optional `start=` / `nostart=`: letters the stem must (must not) begin with (im- before b, m, p;
    /// in- not before b, m, p, l, r: the prefix assimilates).
    pub start: Vec<String>,
    pub nostart: Vec<String>,
    /// Prefix, optional `part=<tag>`: the tag a participle stem (VBN, VBD, VBG) takes under this prefix (un- +
    /// participle is an adjective: *unchanged, unwilling* are JJ in UD EWT 2.18 train, 55 of 61 un…ed forms).
    pub part: Option<Tag>,
    /// Variant: the American ending.
    pub to: String,
    pub gloss: String,
    /// Seed file and line.
    pub source: String,
}

fn classes(s: &str) -> Result<u8, String> {
    s.split(',').try_fold(0u8, |m, c| {
        Ok(m | match c {
            "N" => N,
            "V" => V,
            "A" => A,
            "R" => R,
            _ => return Err(format!("unknown class '{c}'")),
        })
    })
}

fn tag_list(s: &str) -> Result<Vec<Tag>, String> {
    s.split(',').map(|t| Tag::ALL.iter().copied().find(|x| x.name() == t).ok_or_else(|| format!("unknown tag '{t}'"))).collect()
}

/// Rules of one seed text: lines of its ```` ```derive ```` blocks (`#` starts a comment).
pub fn parse(text: &str, file: &str) -> Result<Vec<Rule>, String> {
    let mut out = Vec::new();
    for block in crate::expert::blocks(text, "derive", true) {
        for (k, line) in block.iter().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let err = |e: String| format!("{file}: derive line {}: {e}: «{line}»", k + 1);
            let (head, gloss) = line.split_once('|').map_or((line, ""), |(h, g)| (h.trim(), g.trim()));
            let f: Vec<&str> = head.split_whitespace().collect();
            let source = format!("{file}:{}", k + 1);
            // mandatory fields, then optional key=value fields
            let n = f.iter().position(|x| x.contains('=')).unwrap_or(f.len());
            let mut opt: BTreeMap<&str, Vec<String>> = BTreeMap::new();
            for x in &f[n..] {
                let (k, v) = x.split_once('=').unwrap_or((x, ""));
                if !matches!(k, "after" | "except" | "block" | "start" | "nostart" | "part") || v.is_empty() || opt.insert(k, v.split(',').map(str::to_string).collect()).is_some() {
                    return Err(err(format!("unknown, empty or repeated field '{x}' (after=, except=, block=, start=, nostart=, part=)")));
                }
            }
            let take = |k: &str| opt.get(k).cloned().unwrap_or_default();
            let mut rule = match (f.first().copied(), n) {
                (Some("prefix"), 4) => Rule {
                    kind: Kind::Prefix,
                    affix: f[1].into(),
                    stem: classes(f[2]).map_err(err)?,
                    tags: vec![],
                    min: f[3].parse().map_err(|_| err("shortest stem must be a number".into()))?,
                    after: vec![],
                    except: vec![],
                    block: vec![],
                    start: vec![],
                    nostart: vec![],
                    part: None,
                    to: String::new(),
                    gloss: gloss.into(),
                    source,
                },
                (Some(k @ ("suffix" | "inflect")), 4) => Rule {
                    kind: if k == "suffix" { Kind::Suffix } else { Kind::Inflect },
                    affix: f[1].into(),
                    stem: classes(f[2]).map_err(err)?,
                    tags: tag_list(f[3]).map_err(err)?,
                    min: 0,
                    after: vec![],
                    except: vec![],
                    block: vec![],
                    start: vec![],
                    nostart: vec![],
                    part: None,
                    to: String::new(),
                    gloss: gloss.into(),
                    source,
                },
                (Some("variant"), 3) => Rule {
                    kind: Kind::Variant,
                    affix: f[1].into(),
                    stem: 0,
                    tags: vec![],
                    min: 0,
                    after: vec![],
                    except: vec![],
                    block: vec![],
                    start: vec![],
                    nostart: vec![],
                    part: None,
                    to: f[2].into(),
                    gloss: format!("British spelling of -{}", f[2]),
                    source,
                },
                _ => return Err(err("expected 'prefix A C N', 'suffix A C T', 'inflect A C T' or 'variant B A'".into())),
            };
            if rule.kind == Kind::Variant && !opt.is_empty() || rule.kind != Kind::Prefix && ["start", "nostart", "part"].iter().any(|k| opt.contains_key(k)) {
                return Err(err("start=, nostart= and part= are for prefixes; a variant takes no options".into()));
            }
            if let Some(p) = opt.get("part") {
                rule.part = Some(tag_list(&p.join(",")).map_err(err)?.first().copied().ok_or_else(|| err("part= needs a tag".into()))?);
            }
            (rule.after, rule.except, rule.block) = (take("after"), take("except"), take("block"));
            (rule.start, rule.nostart) = (take("start"), take("nostart"));
            if (rule.kind == Kind::Suffix || rule.kind == Kind::Prefix) && rule.gloss.is_empty() {
                return Err(err("a gloss after '|' is required".into()));
            }
            out.push(rule);
        }
    }
    Ok(out)
}

/// All rules of the compiled seeds, in seed order. A bad line is a build defect: the test `rules_parse` keeps it out.
pub fn rules() -> &'static [Rule] {
    static R: OnceLock<Vec<Rule>> = OnceLock::new();
    R.get_or_init(|| {
        let mut v = Vec::new();
        for (f, t) in SEEDS {
            v.extend(parse(t, f).unwrap_or_else(|e| panic!("{e}")));
        }
        v
    })
}

/// A dictionary word: the same test as `morph::known_form` before derivation (AGID's false superlatives excluded).
pub fn in_dictionary(lower: &str) -> bool {
    dict::form(lower).is_some_and(|f| {
        dict::analyses(f).any(|r| r.src & (dict::UD | dict::REV | dict::BASE) != 0 || !matches!(r.tag, Tag::JJS | Tag::JJR | Tag::RBS | Tag::RBR))
    })
}

/// Open-class tags the dictionary gives a form (proper nouns, closed classes and false superlatives left out), sorted.
fn dictionary_tags(lower: &str) -> Vec<Tag> {
    let Some(f) = dict::form(lower) else { return vec![] };
    let mut v: Vec<Tag> = dict::analyses(f)
        .filter(|r| class(r.tag) != 0 && (r.src & (dict::UD | dict::REV | dict::BASE) != 0 || !matches!(r.tag, Tag::JJS | Tag::JJR | Tag::RBS | Tag::RBR)))
        .map(|r| r.tag)
        .collect();
    v.sort_by_key(|t| t.idx());
    v.dedup();
    v
}

/// One step of an analysis: the rule and the word it made.
#[derive(Clone, Debug)]
pub struct Step {
    pub rule: &'static Rule,
    /// The word before this step (the stem it attached to, in its dictionary or derived spelling).
    pub stem: String,
    /// The word after it.
    pub word: String,
}

/// An analysis of a derived form.
#[derive(Clone, Debug)]
pub struct Derivation {
    /// The dictionary word at the bottom.
    pub base: String,
    /// Its open-class tags in the dictionary.
    pub base_tags: Vec<Tag>,
    /// Steps from the base up to the form.
    pub steps: Vec<Step>,
    /// Tags of the form by this analysis.
    pub tags: Vec<Tag>,
}

impl Derivation {
    pub fn form(&self) -> &str {
        self.steps.last().map_or(self.base.as_str(), |s| s.word.as_str())
    }

    /// Human-readable: «unsupervised: supervised [dictionary VBD VBN] → un- + supervised = unsupervised (not; reverse
    /// the action) ⇒ VBD VBN».
    pub fn why(&self) -> String {
        let names = |t: &[Tag]| t.iter().map(|t| t.name()).collect::<Vec<_>>().join(" ");
        let mut s = format!("{}: {} [dictionary {}]", self.form(), self.base, names(&self.base_tags));
        for st in &self.steps {
            let r = st.rule;
            let gloss = r.gloss.replace("<stem>", &st.stem);
            s += &match r.kind {
                Kind::Prefix => format!(" → {}- + {} = {} ({gloss})", r.affix.trim_end_matches('-'), st.stem, st.word),
                Kind::Suffix => format!(" → {} + -{} = {} ({gloss})", st.stem, r.affix, st.word),
                Kind::Inflect => format!(" → {} + -{} = {} (inflection)", st.stem, r.affix, st.word),
                Kind::Variant => format!(" → {} = {} ({gloss})", st.stem, st.word),
            };
        }
        format!("{s} ⇒ {}", names(&self.tags))
    }
}

fn vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

/// Ends consonant–vowel–consonant (not w, x, y): the last consonant may double before a vowel suffix (fit → fitting,
/// forget → forgettable; stress decides, and it is not known here, so both spellings are allowed).
fn cvc(s: &str) -> bool {
    let c: Vec<char> = s.chars().collect();
    let n = c.len();
    n >= 3 && !vowel(c[n - 1]) && !matches!(c[n - 1], 'w' | 'x' | 'y') && vowel(c[n - 2]) && !vowel(c[n - 3])
}

/// The forward spelling: the forms a stem and a suffix (or inflection) make, by the rules of English spelling
/// (seed `adv-ly`, `wf-adj-suffixes`; Jespersen MEG VI, spelling of derivatives):
/// - a final «-ue» loses its «e» (argue → argument, true → truly);
/// - a final «-e» is dropped before a vowel (use → usable, tokenize → tokenization, -e + -ed → -ed), kept after «c»/«g»
///   before «a»/«o» (notice → noticeable, courage → courageous);
/// - consonant + «y» becomes «i» before anything but «i» (happy → happiness, happily); before -ic, -ical, -ist, -ism,
///   -ize, -ify the «y» is dropped (morphology → morphological, economy → economist);
/// - a consonant–vowel–consonant end may double before a vowel (fit → fitting);
/// - -able/-ible + -ity → -ability/-ibility;
/// - -ly: -le → -ly (able → ably), -ll → -lly (full → fully), never after -ic (that is -ally);
/// - -s after s, x, z, ch, sh or consonant + y is -es (-ies).
///
/// A form is analysed only when it is one of these spellings: *happyness*, *adoreable*, *noticable*, *arguement*,
/// *basicly* are misspellings, not derivations.
pub fn join(stem: &str, a: &str) -> Vec<String> {
    let first = a.chars().next().unwrap_or(' ');
    let v = vowel(first) || first == 'y';
    let last = stem.chars().last().unwrap_or(' ');
    let before_last = stem.chars().rev().nth(1).unwrap_or(' ');
    if a == "s" || a == "es" {
        let sibilant = stem.ends_with(['s', 'x', 'z']) || stem.ends_with("ch") || stem.ends_with("sh");
        let cons_y = last == 'y' && !vowel(before_last);
        return match (a, sibilant, cons_y) {
            ("s", false, false) => vec![format!("{stem}s")],
            ("es", true, _) => vec![format!("{stem}es")],
            ("es", false, true) => vec![format!("{}ies", &stem[..stem.len() - 1])],
            _ => vec![],
        };
    }
    if a == "ly" {
        if stem.ends_with("ic") {
            return vec![];
        }
        if let Some(s) = stem.strip_suffix("le").filter(|s| !s.is_empty() && !s.ends_with(|c: char| vowel(c))) {
            return vec![format!("{s}ly")];
        }
        if stem.ends_with("ll") {
            return vec![format!("{stem}y")];
        }
    }
    if a == "ity" {
        for (x, y) in [("able", "abil"), ("ible", "ibil")] {
            if let Some(s) = stem.strip_suffix(x) {
                return vec![format!("{s}{y}ity")];
            }
        }
    }
    if let Some(s) = stem.strip_suffix("ue") {
        return vec![format!("{s}u{a}")];
    }
    if last == 'e' && v {
        if (stem.ends_with("ce") || stem.ends_with("ge")) && matches!(first, 'a' | 'o') {
            return vec![format!("{stem}{a}")];
        }
        if stem.ends_with("ee") && first != 'e' {
            return vec![format!("{stem}{a}")];
        }
        return vec![format!("{}{a}", &stem[..stem.len() - 1])];
    }
    if last == 'y' && !vowel(before_last) {
        let s = &stem[..stem.len() - 1];
        return if matches!(a, "ic" | "ical" | "ist" | "ism" | "ize" | "ify") {
            vec![format!("{s}{a}")]
        } else if first == 'i' {
            vec![format!("{stem}{a}")]
        } else {
            vec![format!("{s}i{a}")]
        };
    }
    let mut out = vec![format!("{stem}{a}")];
    if v && cvc(stem) {
        out.push(format!("{stem}{last}{a}"));
    }
    out
}

/// The reverse spelling: stems a suffix may have been joined to, from what is left when it is cut off (`rest`); a
/// candidate is kept only when `join` gives the form back.
fn stems(rest: &str, affix: &str) -> Vec<String> {
    let mut v = vec![rest.to_string(), format!("{rest}e"), format!("{rest}y")];
    if let Some(s) = rest.strip_suffix('i') {
        v.push(format!("{s}y"));
    }
    let b = rest.as_bytes();
    if b.len() >= 3 && b[b.len() - 1] == b[b.len() - 2] {
        v.push(rest[..rest.len() - 1].to_string());
    }
    for (a, b) in [("abil", "able"), ("ibil", "ible")] {
        if let Some(s) = rest.strip_suffix(a) {
            v.push(format!("{s}{b}"));
        }
    }
    if affix == "ly" {
        v.push(format!("{rest}le"));
        v.push(format!("{rest}l"));
    }
    if let Some(s) = rest.strip_suffix("ie").filter(|_| affix == "s") {
        v.push(format!("{s}y"));
    }
    v.dedup();
    v
}

fn letters(s: &str) -> usize {
    s.chars().filter(|c| c.is_alphabetic()).count()
}

fn tags_class(t: &[Tag]) -> u8 {
    t.iter().fold(0, |m, &t| m | class(t))
}

/// Classes of the base-form tags only (NN, VB, VBP, JJ, RB): a derivational suffix or an inflection attaches to a
/// base, not to an inflected form («flicking» + -er is no word).
fn base_class(t: &[Tag]) -> u8 {
    t.iter().filter(|t| matches!(t, Tag::NN | Tag::VB | Tag::VBP | Tag::JJ | Tag::RB)).fold(0, |m, &t| m | class(t))
}

/// All analyses of `w` up to `depth` steps; the dictionary word itself is the analysis with no steps.
fn go(w: &str, depth: usize, memo: &mut BTreeMap<(String, usize), Vec<Derivation>>) -> Vec<Derivation> {
    if let Some(v) = memo.get(&(w.to_string(), depth)) {
        return v.clone();
    }
    let mut out = Vec::new();
    // a word of the lexicon extension is a base below the form, never the form itself (`derivations` explains a
    // form the extension has: *modelled* = model + -ed)
    if in_dictionary(w) || (depth < DEPTH && crate::lexext::known(w)) {
        // the dictionary's open-class tags and those the lexicon extension adds (*directional* JJ beside its NN)
        let mut t = dictionary_tags(w);
        t.extend(crate::lexext::tags(w).into_iter().filter(|&t| class(t) != 0));
        t.sort_by_key(|t| t.idx());
        t.dedup();
        if !t.is_empty() && letters(w) >= MIN_BASE && (in_ud(w) || letters(w) >= MIN_RARE_BASE) {
            out.push(Derivation { base: w.to_string(), base_tags: t.clone(), steps: vec![], tags: t });
        }
    } else if depth > 0 && w.chars().all(|c| c.is_ascii_lowercase() || c == '-') {
        for r in rules() {
            match r.kind {
                Kind::Variant => {
                    if let Some(rest) = w.strip_suffix(r.affix.as_str()) {
                        let american = format!("{rest}{}", r.to);
                        for d in go(&american, depth - 1, memo) {
                            let tags = d.tags.clone();
                            out.push(extend(d, r, &american, w, tags));
                        }
                    }
                }
                Kind::Prefix => {
                    let p = r.affix.trim_end_matches('-');
                    let hyphen_only = r.affix.ends_with('-');
                    let Some(rest) = w.strip_prefix(p) else { continue };
                    let rest = match rest.strip_prefix('-') {
                        Some(x) => x,
                        None if hyphen_only => continue,
                        None => rest,
                    };
                    if letters(rest) < r.min
                        || rest.contains('-')
                        || r.except.iter().any(|e| rest.ends_with(e.as_str()))
                        || (!r.start.is_empty() && !r.start.iter().any(|x| rest.starts_with(x.as_str())))
                        || r.nostart.iter().any(|x| rest.starts_with(x.as_str()))
                    {
                        continue;
                    }
                    // blocking: a rival prefix with this stem is a word already (impossible ⇒ no unpossible)
                    if r.block.iter().any(|b| in_dictionary(&format!("{b}{rest}"))) {
                        continue;
                    }
                    for d in go(rest, depth - 1, memo) {
                        // the prefix keeps the stem's tags of its classes (bi- + lateral: not the verb); `part=` turns a
                        // participle into that tag (un- + changed → JJ)
                        let mut tags: Vec<Tag> = d.tags.iter().filter(|&&t| class(t) & r.stem != 0).map(|&t| match (r.part, t) {
                            (Some(p), Tag::VBN | Tag::VBD | Tag::VBG) => p,
                            _ => t,
                        }).collect();
                        tags.sort_by_key(|t| t.idx());
                        tags.dedup();
                        if !tags.is_empty() {
                            out.push(extend(d, r, rest, w, tags));
                        }
                    }
                }
                Kind::Suffix | Kind::Inflect => {
                    let Some(rest) = w.strip_suffix(r.affix.as_str()) else { continue };
                    if rest.contains('-') {
                        continue;
                    }
                    for s in stems(rest, &r.affix) {
                        if letters(&s) < MIN_BASE
                            || (!r.after.is_empty() && !r.after.iter().any(|a| s.ends_with(a.as_str())))
                            || r.except.iter().any(|e| s.ends_with(e.as_str()))
                            || !join(&s, &r.affix).iter().any(|x| x == w)
                            || r.block.iter().any(|b| join(&s, b).iter().any(|x| in_dictionary(x)))
                        {
                            continue;
                        }
                        for d in go(&s, depth - 1, memo) {
                            // an inflection only on a derived word; the stem must have one of the rule's classes
                            if (r.kind == Kind::Inflect && d.steps.is_empty()) || base_class(&d.tags) & r.stem == 0 {
                                continue;
                            }
                            let tags = r.tags.clone();
                            out.push(extend(d, r, &s, w, tags));
                        }
                    }
                }
            }
        }
        // the shortest analyses first; of equal length, one ending in a suffix before one ending in a prefix (*intense
        // + -ly*, not *in- + tensely*); then seed order (stable)
        out.sort_by_key(rank);
    }
    memo.insert((w.to_string(), depth), out.clone());
    out
}

/// Order of analyses: fewer steps first; then an analysis whose last step is a suffix, an inflection or a variant
/// before one whose last step is a prefix.
fn rank(d: &Derivation) -> (usize, bool) {
    (d.steps.len(), d.steps.last().is_some_and(|s| s.rule.kind == Kind::Prefix))
}

fn extend(mut d: Derivation, r: &'static Rule, stem: &str, word: &str, tags: Vec<Tag>) -> Derivation {
    d.steps.push(Step { rule: r, stem: stem.to_string(), word: word.to_string() });
    d.tags = tags;
    d
}

/// Rebuild a form from a stem spelling and a rule: what a step's `word` must be (the forward direction of `go`'s cut).
pub fn compose(stem: &str, r: &Rule) -> Vec<String> {
    match r.kind {
        Kind::Prefix => vec![format!("{}{stem}", r.affix), format!("{}-{stem}", r.affix.trim_end_matches('-'))],
        Kind::Variant => stem.strip_suffix(r.to.as_str()).map(|s| vec![format!("{s}{}", r.affix)]).unwrap_or_default(),
        Kind::Suffix | Kind::Inflect => join(stem, &r.affix),
    }
}

/// Analyses of a form neither the dictionary nor the lexicon extension knows, shortest first (empty for a known
/// word or no analysis).
pub fn analyses(lower: &str) -> Vec<Derivation> {
    if crate::lexext::known(lower) {
        return vec![];
    }
    derivations(lower)
}

/// Analyses of a form the dictionary does not know, whether or not the lexicon extension has it (*unsupervised* is in
/// SPECIALIST and is still un- + supervised): what `en derive` explains.
pub fn derivations(lower: &str) -> Vec<Derivation> {
    // an Early Modern English ending on a known verb is `morph::ending_verb`'s («biteth» is bite, not bi- + teth)
    if lower.is_empty() || in_dictionary(lower) || crate::morph::ending_verb(lower).is_some() {
        return vec![];
    }
    let mut memo = BTreeMap::new();
    go(lower, DEPTH, &mut memo)
}

/// The shortest analysis of a form the dictionary does not know.
pub fn analyze(lower: &str) -> Option<Derivation> {
    analyses(lower).into_iter().next()
}

/// Tags a form may have by its best analyses (those of the first one's rank; empty: a dictionary word or none),
/// sorted by tag index.
pub fn tags(lower: &str) -> Vec<Tag> {
    let all = analyses(lower);
    let Some(best) = all.first().map(rank) else { return vec![] };
    let mut t: Vec<Tag> = all.into_iter().filter(|d| rank(d) == best).flat_map(|d| d.tags).collect();
    t.sort_by_key(|t| t.idx());
    t.dedup();
    t
}

/// Tags for the taggers' dictionary hint on a form the dictionary does not know: `tags`, or nothing when
/// `EN_DERIVE_TAGS=0` (the reference switch: the taggers as before derivation).
pub fn hint(lower: &str) -> Vec<Tag> {
    static ON: OnceLock<bool> = OnceLock::new();
    if !*ON.get_or_init(|| std::env::var("EN_DERIVE_TAGS").map_or(true, |v| v != "0")) {
        return vec![];
    }
    tags(lower)
}

/// A derived word of one of the classes `cls`: «unsupervise» (V) is the lemma of «unsupervised» VBN.
pub fn derived_as(lower: &str, cls: u8) -> bool {
    analyses(lower).iter().any(|d| !d.steps.is_empty() && tags_class(&d.tags) & cls != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_parse() {
        let r = rules();
        assert!(r.iter().filter(|r| r.kind == Kind::Prefix).count() >= 30, "prefixes");
        assert!(r.iter().filter(|r| r.kind == Kind::Suffix).count() >= 20, "suffixes");
        assert!(r.iter().any(|r| r.kind == Kind::Variant && r.affix == "isation"));
        // negative control: a malformed line is an error, never skipped
        for bad in ["prefix un X 4 | not", "suffix ness A QQ | x", "suffix ness A NN", "prefix un A four | not", "infix x A NN"] {
            assert!(parse(&format!("```derive\n{bad}\n```\n"), "t.md").is_err(), "{bad}");
        }
        assert_eq!(parse("```derive\n# comment\n\nprefix un A 4 | not\n```\n", "t.md").unwrap().len(), 1);
    }

    /// The derivation's tags, whether or not the lexicon extension has the form.
    fn has(w: &str, t: Tag) -> bool {
        let all = derivations(w);
        let best = all.first().map(rank);
        all.iter().filter(|d| Some(rank(d)) == best).any(|d| d.tags.contains(&t))
    }

    fn first(w: &str) -> Option<Derivation> {
        derivations(w).into_iter().next()
    }

    #[test]
    fn derived_words_of_papers() {
        if dict::N_LEMMAS == 0 {
            return;
        }
        for (w, t) in [
            ("unsupervised", Tag::JJ),
            ("unchanged", Tag::JJ),
            ("re-wording", Tag::VBG),
            // the dictionary (AGID) has «directional» only as NN; the lexicon extension adds JJ (OEWN, SPECIALIST)
            ("bidirectional", Tag::JJ),
            ("downsampled", Tag::VBN),
            ("overfitting", Tag::VBG),
            ("incremental", Tag::JJ),
            ("incrementally", Tag::RB),
            ("tokenized", Tag::VBN),
            ("tokenised", Tag::VBN),
            ("tokenization", Tag::NN),
            ("tokenisation", Tag::NN),
            ("tokenizers", Tag::NNS),
            ("reannotate", Tag::VB),
            ("rerank", Tag::VB),
            ("pre-trained", Tag::VBN),
            ("multilingual", Tag::JJ),
            ("learnable", Tag::JJ),
            ("scalability", Tag::NN),
            ("explainability", Tag::NN),
            ("modelled", Tag::VBN),
            ("pairwise", Tag::RB),
            ("self-attention", Tag::NN),
        ] {
            assert!(has(w, t), "{w} {t:?}: {:?}", first(w).map(|d| d.why()));
            let d = first(w).unwrap();
            assert!(d.why().contains(&d.base), "{}", d.why());
        }
        assert_eq!(first("unsupervised").unwrap().why(), "unsupervised: supervised [dictionary JJ VBD VBN] → un- + supervised = unsupervised (not; reverse the action) ⇒ JJ");
    }

    /// Negative controls: dictionary words are not analysed; short stems, closed classes, wrong classes and non-words
    /// give nothing.
    #[test]
    fn conservative() {
        if dict::N_LEMMAS == 0 {
            return;
        }
        for w in ["size", "table", "unless", "business", "happiness", "national", "organisation"] {
            assert!(analyses(w).is_empty(), "{w} is a dictionary word");
        }
        for w in [
            "cohen",     // co- + hen: the stem is shorter than co-'s five letters
            "reus",      // too short
            "unthe",     // closed class
            "inman",     // in- takes adjectives only
            "nessness",  // nothing under it
            "xyzable",   // no stem
            "goest",     // EModE ending, not derivation
            "derful",    // stem too short
            "japheth",
            "biteth",    // EModE: bite + -eth (morph::ending_verb), not bi- + teth
            "flickinger", // -er on an inflected form (flicking)
            "kantor",    // -or only after -ate, -ct, -it, -ss
            "hajic",     // stem shorter than four letters
            "baral",
            // misspellings from the spelling lessons: the spelling rules do not make them
            "happyness", "happyly", "adoreable", "noticable", "arguement", "basicly",
            // blocked by a rival prefix in the dictionary (impossible, ...)
            "unpossible", "dispossible", "readible", "foregetting",
            // in- assimilates: im- before b, m, p, il- before l, ir- before r
            "inpossible", "inmature", "inlegal", "imlegal", "inlogical", "inresponsible",
            // -ion only after -ate, -ct, -ute, -ss, -pt; rare AGID bottoms need five letters
            "decition", "discusion", "magision", "recive",
            "unthinkablyx",
        ] {
            // the derivation itself, apart from the lexicon extension (SPECIALIST has *unpossible*)
            assert!(derivations(w).is_empty(), "{w}: {:?}", first(w).map(|d| d.why()));
        }
    }

    /// The reverse direction: each step's word is one of the spellings `compose` rebuilds from its stem.
    #[test]
    fn steps_compose_back() {
        if dict::N_LEMMAS == 0 {
            return;
        }
        for w in ["morphological", "unsupervised", "tokenised", "scalability", "scalably", "forgettable", "unlearnability", "incrementally", "co-training", "tokenizers"] {
            let ds = derivations(w);
            assert!(!ds.is_empty(), "{w}");
            for d in ds {
                for s in &d.steps {
                    assert!(compose(&s.stem, s.rule).contains(&s.word), "{w}: {} from {} by {:?}", s.word, s.stem, s.rule.affix);
                }
            }
        }
    }

    /// The forward spelling rules, with their negative controls.
    #[test]
    fn spelling() {
        for (stem, a, w) in [
            ("argue", "ment", "argument"), ("true", "ly", "truly"), ("use", "able", "usable"), ("notice", "able", "noticeable"),
            ("happy", "ness", "happiness"), ("happy", "ly", "happily"), ("morphology", "ical", "morphological"),
            ("adversary", "al", "adversarial"), ("fit", "ing", "fitting"), ("fit", "ing", "fiting"), ("scalable", "ity", "scalability"),
            ("scalable", "ly", "scalably"), ("full", "ly", "fully"), ("tokenize", "er", "tokenizer"), ("tokenize", "ed", "tokenized"),
            ("agree", "ed", "agreed"), ("agree", "able", "agreeable"), ("summary", "es", "summaries"), ("box", "es", "boxes"),
            ("token", "s", "tokens"), ("play", "ful", "playful"), ("study", "ing", "studying"),
        ] {
            assert!(join(stem, a).iter().any(|x| x == w), "{stem} + {a} → {w}: {:?}", join(stem, a));
        }
        for (stem, a, w) in [
            ("argue", "ment", "arguement"), ("adore", "able", "adoreable"), ("notice", "able", "noticable"), ("happy", "ness", "happyness"),
            ("basic", "ly", "basicly"), ("box", "s", "boxs"), ("token", "es", "tokenes"), ("study", "ing", "studing"),
        ] {
            assert!(!join(stem, a).iter().any(|x| x == w), "{stem} + {a} must not give {w}");
        }
    }

    #[test]
    fn suffix_before_prefix() {
        if dict::N_LEMMAS == 0 {
            return;
        }
        // intense + -ly, not in- + tensely (in- takes no -ly adverb, and a suffix-final analysis ranks first)
        let d = first("intensely").unwrap();
        assert!(d.steps.last().is_some_and(|s| s.rule.kind == Kind::Suffix) && d.tags == vec![Tag::RB], "{}", d.why());
    }

    #[test]
    fn deterministic() {
        if dict::N_LEMMAS == 0 {
            return;
        }
        let a: Vec<String> = analyses("unsupervised").iter().map(Derivation::why).collect();
        let b: Vec<String> = analyses("unsupervised").iter().map(Derivation::why).collect();
        assert_eq!(a, b);
    }
}
