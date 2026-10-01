//! Document context for multi-pass parsing (, "Multi-pass
//! parsing"): genre, sentence type (current and previous), paragraph and speaker, token entity.
//! Gold standard: GUM CoNLL-U comments and MISC: `# meta::genre`, `# s_type`, `# newpar`, `# speaker`,
//! `Entity=` (mention type and information status new/giv/acc).
//!
//! Context features are minimal: conjoined with the existing tagger (`ptag`) and parser
//! (`parse`) templates. Without context (`None` or an empty `Ctx`) there are no features, so the model is the same as before,
//! bit for bit. Experiments only via `en ud-eval` (`EN_CTX`, `EN_CTX_EVAL`, `EN_CTX_GROUPS`, `EN_SEED`).

use anyhow::{Result, bail};

use crate::conllu::{Doc, Sentence};
use crate::gram::{Rel, Tag, UPos};
use crate::hash::{FastMap, FastSet};

/// Enum with names as in the treebank: `ALL`, `name()`, `parse()`, `code()` (1.., 0 = "none").
macro_rules! named {
    ($(#[$m:meta])* $name:ident { $($v:ident = $s:literal),+ $(,)? }) => {
        $(#[$m])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
        #[repr(u8)]
        pub enum $name { $($v),+ }

        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$v),+];
            pub fn name(self) -> &'static str {
                match self { $($name::$v => $s),+ }
            }
            pub fn parse(s: &str) -> Option<$name> {
                match s { $($s => Some($name::$v),)+ _ => None }
            }
            /// Code in features: 1.. (0 = "none").
            #[inline]
            pub fn code(self) -> u64 {
                self as u64 + 1
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(self.name())
            }
        }
    };
}

named! {
    /// Document genre: GUM (`# meta::genre`) and EWT (`sent_id` prefix).
    Genre {
        Academic = "academic", Bio = "bio", Conversation = "conversation", Court = "court", Essay = "essay",
        Fiction = "fiction", Interview = "interview", Letter = "letter", News = "news", Podcast = "podcast",
        Speech = "speech", Textbook = "textbook", Vlog = "vlog", Voyage = "voyage", Whow = "whow", Reddit = "reddit",
        Weblog = "weblog", Email = "email", Answers = "answers", Reviews = "reviews", Newsgroup = "newsgroup",
    }
}

named! {
    /// GUM sentence type (`# s_type`): 11 classes.
    SType {
        Decl = "decl", Frag = "frag", Imp = "imp", Sub = "sub", Intj = "intj", Q = "q", Multiple = "multiple",
        Other = "other", Wh = "wh", Inf = "inf", Ger = "ger",
    }
}

named! {
    /// GUM entity type (second field of `Entity=`); unknown is `Other`.
    EType {
        Abstract = "abstract", Animal = "animal", Event = "event", Object = "object", Organization = "organization",
        Person = "person", Place = "place", Plant = "plant", Substance = "substance", Time = "time", Other = "other",
    }
}

named! {
    /// Information status of a mention (third field of `Entity=`): new; giv:act, giv:inact → giv;
    /// acc:inf, acc:com, acc:aggr → acc; anything else (auto) → other.
    Info { New = "new", Giv = "giv", Acc = "acc", Other = "other" }
}

impl Info {
    fn of(s: &str) -> Info {
        match s.split(':').next().unwrap_or("") {
            "new" => Info::New,
            "giv" => Info::Giv,
            "acc" => Info::Acc,
            _ => Info::Other,
        }
    }
}

named! {
    /// Sentence speaker versus the previous one: no speaker annotation, same, changed.
    Turn { NoSpeaker = "none", Same = "same", Changed = "changed" }
}

/// Context feature groups (bits): `EN_CTX_GROUPS=genre,stype,doc,entity`.
pub mod group {
    pub const GENRE: u8 = 1;
    pub const STYPE: u8 = 2;
    pub const DOC: u8 = 4;
    pub const ENTITY: u8 = 8;
    pub const ALL: u8 = 15;
    pub const NAMES: [(&str, u8); 4] = [("genre", GENRE), ("stype", STYPE), ("doc", DOC), ("entity", ENTITY)];

    /// Group names, comma-separated.
    pub fn names(g: u8) -> String {
        let v: Vec<&str> = NAMES.iter().filter(|(_, b)| g & b != 0).map(|(n, _)| *n).collect();
        if v.is_empty() { "—".into() } else { v.join(",") }
    }
}

/// Token entity: type and status of the deepest mention containing it.
pub type Ent = (EType, Info);

/// Sentence context. `None` in a field means the layer is absent (the treebank does not annotate it or the group is off).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ctx {
    pub genre: Option<Genre>,
    pub stype: Option<SType>,
    /// type of the previous sentence in the document; `Some(None)` means the sentence is the first in the document
    pub prev: Option<Option<SType>>,
    /// (paragraph start, speaker versus the previous sentence)
    pub doc: Option<(bool, Turn)>,
    /// entity of each token (0-based indices); `Some(v)`, v[i] = None means the token is outside mentions
    pub ent: Option<Vec<Option<Ent>>>,
}

/// Group numbers in feature keys (the first number after the template number).
const G_GENRE: u64 = 1;
const G_STYPE: u64 = 2;
const G_PREV: u64 = 3;
const G_DOC: u64 = 4;
pub const G_ENT: u64 = 5;

impl Ctx {
    pub fn is_empty(&self) -> bool {
        self.genre.is_none() && self.stype.is_none() && self.prev.is_none() && self.doc.is_none() && self.ent.is_none()
    }

    /// Only the enabled groups.
    pub fn masked(mut self, groups: u8) -> Ctx {
        if groups & group::GENRE == 0 {
            self.genre = None;
        }
        if groups & group::STYPE == 0 {
            self.stype = None;
            self.prev = None;
        }
        if groups & group::DOC == 0 {
            self.doc = None;
        }
        if groups & group::ENTITY == 0 {
            self.ent = None;
        }
        self
    }

    /// Sentence-level features: (group, code ≠ 0).
    pub fn sent_codes(&self) -> Vec<(u64, u64)> {
        let mut v = Vec::with_capacity(4);
        if let Some(g) = self.genre {
            v.push((G_GENRE, g.code()));
        }
        if let Some(s) = self.stype {
            v.push((G_STYPE, s.code()));
        }
        if let Some(p) = self.prev {
            v.push((G_PREV, p.map_or(100, SType::code)));
        }
        if let Some((par, turn)) = self.doc {
            v.push((G_DOC, 1 + par as u64 + 2 * turn as u64));
        }
        v
    }

    /// Entity code of token `i` (0-based): 0 = no layer, 1 = outside mentions, then type × status.
    /// Entity ablation (`EN_CTX_ENT`): full = type × status (default); span = only "in a
    /// mention"; type = type only; info = status new/giv/acc only.
    pub fn ent_code(&self, i: usize) -> u64 {
        static PART: std::sync::OnceLock<u8> = std::sync::OnceLock::new();
        let part = *PART.get_or_init(|| match std::env::var("EN_CTX_ENT").as_deref() {
            Ok("span") => 1,
            Ok("type") => 2,
            Ok("info") => 3,
            _ => 0,
        });
        match &self.ent {
            None => 0,
            Some(v) => match v.get(i).copied().flatten() {
                Some((t, f)) => match part {
                    1 => 2,
                    2 => 2 + t as u64,
                    3 => 2 + f as u64,
                    _ => 2 + t as u64 * 4 + f as u64,
                },
                None => 1,
            },
        }
    }
}

/// Context features for the tagger and parser, once per sentence: sentence-level and entity codes.
#[derive(Clone, Debug, Default)]
pub struct Codes {
    pub sent: Vec<(u64, u64)>,
    /// entity code of each token (0-based indices); empty means no layer
    pub ent: Vec<u64>,
    /// tagger: also conjoin with the word itself (`EN_CTX_WORD=1`)
    pub word: bool,
}

impl Codes {
    pub fn of(c: &Ctx, n: usize) -> Codes {
        static WORD: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
        let word = *WORD.get_or_init(|| std::env::var("EN_CTX_WORD").is_ok_and(|v| v == "1"));
        Codes { sent: c.sent_codes(), ent: if c.ent.is_some() { (0..n).map(|i| c.ent_code(i)).collect() } else { Vec::new() }, word }
    }
    pub fn is_empty(&self) -> bool {
        self.sent.is_empty() && self.ent.is_empty()
    }
    /// Entity code of token `i` (0-based), 0 = none.
    #[inline]
    pub fn ent(&self, i: usize) -> u64 {
        self.ent.get(i).copied().unwrap_or(0)
    }
}

/// Token entities from the sentence's MISC columns: the deepest open mention at each token.
/// `(7-person-new-…)6)5)` opens and closes 7, closes 6 and 5; `3)(5-abstract-…` closes 3,
/// opens 5. Description fields: id-type-status-… (`# global.Entity`).
pub fn entities<S: AsRef<str>>(misc: &[S]) -> Vec<Option<Ent>> {
    entities_checked(misc).0
}

/// Same, plus bracket checking: how many mentions remain open at the end of the sentence and how many
/// closings found no matching opening (in GUM mentions do not cross sentence boundaries, so both are zero).
pub fn entities_checked<S: AsRef<str>>(misc: &[S]) -> (Vec<Option<Ent>>, usize, usize) {
    let mut unmatched = 0usize;
    let mut stack: Vec<(String, Ent)> = Vec::new();
    let mut out = Vec::with_capacity(misc.len());
    let id_of = |s: &str| s.split('[').next().unwrap_or("").to_string();
    for m in misc {
        let (mut opens, mut closes): (Vec<(String, Ent)>, Vec<String>) = (Vec::new(), Vec::new());
        if let Some(e) = m.as_ref().split('|').find_map(|x| x.strip_prefix("Entity=")) {
            let b = e.as_bytes();
            let mut i = 0;
            while i < b.len() {
                if b[i] == b'(' {
                    let j = e[i + 1..].find(['(', ')']).map_or(e.len(), |k| i + 1 + k);
                    let parts: Vec<&str> = e[i + 1..j].split('-').collect();
                    let id = id_of(parts[0]);
                    let t = parts.get(1).and_then(|x| EType::parse(x)).unwrap_or(EType::Other);
                    let f = parts.get(2).map_or(Info::Other, |x| Info::of(x));
                    opens.push((id.clone(), (t, f)));
                    if j < b.len() && b[j] == b')' {
                        closes.push(id);
                        i = j + 1;
                    } else {
                        i = j;
                    }
                } else {
                    let j = e[i..].find(')').map_or(e.len(), |k| i + k);
                    closes.push(id_of(&e[i..j]));
                    i = j + 1;
                }
            }
        }
        stack.extend(opens);
        out.push(stack.last().map(|x| x.1));
        for c in closes {
            match stack.iter().rposition(|x| x.0 == c) {
                Some(p) => {
                    stack.remove(p);
                }
                None => unmatched += 1,
            }
        }
    }
    (out, stack.len(), unmatched)
}

/// EWT genre from the `sent_id` prefix (weblog-…, email-…, answers-…, reviews-…, newsgroup-…).
pub fn ewt_genre(sent_id: &str) -> Option<Genre> {
    let p = sent_id.split('-').next()?;
    Genre::parse(p).filter(|g| matches!(g, Genre::Weblog | Genre::Email | Genre::Answers | Genre::Reviews | Genre::Newsgroup))
}

/// Gold context of the file's sentences, in the order of sentences with tokens (like `conllu::read`), and the
/// document number of each sentence. Context is taken only from files that annotate it (having
/// `# meta::genre` or `# s_type`); for EWT only the genre from `sent_id`, if `ewt` (for the EWT experiment).
pub fn gold(doc: &Doc, ewt: bool) -> (Vec<Ctx>, Vec<usize>) {
    let rich = doc.sents.iter().any(|s| s.comments.iter().any(|c| c.starts_with("# meta::genre") || c.starts_with("# s_type")));
    let has_ent = rich && doc.sents.iter().any(|s| s.rows.iter().any(|r| r.0[9].contains("Entity=")));
    let (mut out, mut docs) = (Vec::new(), Vec::new());
    let (mut d, mut genre, mut prev, mut speaker, mut started) = (0usize, None, None, None::<String>, false);
    for s in &doc.sents {
        let (mut newdoc, mut newpar, mut stype, mut spk) = (false, false, None, None);
        for c in &s.comments {
            let body = c.trim_start_matches('#').trim();
            let (k, v) = body.split_once(" = ").map_or((body, ""), |(k, v)| (k.trim(), v.trim()));
            match k {
                "newdoc" | "newdoc id" => newdoc = true,
                "newpar" | "newpar id" => newpar = true,
                "meta::genre" => genre = Genre::parse(v),
                "s_type" => stype = SType::parse(v),
                "speaker" => spk = Some(v.to_string()),
                _ => {}
            }
        }
        if newdoc {
            if started {
                d += 1;
            }
            prev = None;
            speaker = None;
        }
        started = true;
        if s.sent.tokens.is_empty() {
            continue;
        }
        let mut c = Ctx::default();
        if rich {
            c.genre = genre;
            c.stype = stype;
            c.prev = Some(prev);
            let turn = match (&spk, &speaker) {
                (None, _) => Turn::NoSpeaker,
                (Some(a), Some(b)) if a == b => Turn::Same,
                _ => Turn::Changed,
            };
            c.doc = Some((newpar || newdoc, turn));
            if has_ent {
                let misc: Vec<&str> = s.words.iter().map(|&k| s.rows[k].0[9].as_str()).collect();
                c.ent = Some(entities(&misc));
            }
        } else if ewt {
            c.genre = ewt_genre(&s.sent.id);
        }
        prev = stype;
        if spk.is_some() {
            speaker = spk;
        }
        out.push(c);
        docs.push(d);
    }
    (out, docs)
}

/// Sentence type from the tree: a rule over the root and its children (stage 2: the first-pass tree).
/// Order of checks:
/// - '?' in the sentence → wh if one of the first three words is interrogative (WDT, WP, WP$, WRB), otherwise q;
/// - root is an interjection (INTJ) → intj;
/// - root is not a verb and has no copula (cop) → frag;
/// - modal auxiliary of the root would, could, should, might, may, must, and also "have to",
///   "supposed to" → sub (in GUM sub is the conditional mood and modality: "Her mom would know.";
///   can and will are mostly decl there);
/// - root VB with "to" (mark) and no subject → inf;
/// - root VBG with no subject and no auxiliaries → ger;
/// - root VB with no subject and no modal auxiliary → imp (don't, let's as well);
/// - otherwise decl. The rule never yields `multiple` or `other`.
pub fn stype_rule(s: &Sentence) -> SType {
    let t = &s.tokens;
    let n = t.len();
    let Some(r) = t.iter().position(|x| x.head == 0) else { return SType::Other };
    let kids: Vec<usize> = (0..n).filter(|&d| t[d].head == r + 1).collect();
    let has = |rel: &[Rel]| kids.iter().any(|&d| rel.contains(&t[d].rel));
    let content: Vec<usize> = (0..n).filter(|&i| t[i].upos != Some(UPos::PUNCT)).collect();
    let tag = |i: usize| t[i].tag.unwrap_or(Tag::X);
    if t.iter().any(|x| x.form.contains('?')) {
        let wh = content.iter().take(3).any(|&i| tag(i).is_wh());
        return if wh { SType::Wh } else { SType::Q };
    }
    if t[r].upos == Some(UPos::INTJ) {
        return SType::Intj;
    }
    let verbal = matches!(t[r].upos, Some(UPos::VERB | UPos::AUX)) || has(&[Rel::Cop]);
    if !verbal {
        return SType::Frag;
    }
    let subj = has(&[Rel::Nsubj, Rel::NsubjPass, Rel::NsubjOuter, Rel::Csubj, Rel::CsubjPass, Rel::Expl]);
    // sub in GUM is the conditional mood and modality: a modal auxiliary (MD) of the root, "have to", "supposed to"
    let md = kids.iter().any(|&d| {
        matches!(t[d].rel, Rel::Aux | Rel::AuxPass) && tag(d) == Tag::MD && matches!(t[d].lemma.to_lowercase().as_str(), "would" | "could" | "should" | "might" | "may" | "must")
    });
    let have_to = matches!(t[r].lemma.to_lowercase().as_str(), "have" | "suppose")
        && kids.iter().any(|&x| t[x].rel == Rel::Xcomp && (0..n).any(|m| t[m].head == x + 1 && t[m].rel == Rel::Mark && tag(m) == Tag::TO));
    if md || have_to {
        return SType::Sub;
    }
    let to = kids.iter().any(|&d| t[d].rel == Rel::Mark && tag(d) == Tag::TO);
    let modal = kids.iter().any(|&d| matches!(t[d].rel, Rel::Aux | Rel::AuxPass) && !t[d].lemma.eq_ignore_ascii_case("do"));
    match tag(r) {
        Tag::VB if to && !subj => SType::Inf,
        Tag::VBG if !subj && !has(&[Rel::Aux, Rel::AuxPass]) => SType::Ger,
        Tag::VB if !subj && !modal && !to => SType::Imp,
        _ => SType::Decl,
    }
}

/// Context derived from annotation (stage 2), for the sentences of `doc` (in `gold` order): genre, paragraph and
/// speaker come from the document header (comments); sentence type (and the previous one) from `stype_rule` over
/// the `ann` tree; entities are only noun status: a NOUN/PROPN lemma already seen in the document → giv,
/// otherwise new (type other); other tokens are outside mentions. `ann` is the gold standard (training) or the
/// first-pass parse (evaluation).
pub fn derived(doc: &Doc, ann: &[Sentence]) -> Result<(Vec<Ctx>, Vec<usize>)> {
    let (mut ctx, docs) = gold(doc, false);
    if ann.len() != ctx.len() {
        bail!("annotation for context: {} sentences, but the document has {}", ann.len(), ctx.len());
    }
    let (mut seen, mut prev, mut cur_doc) = (FastSet::<String>::default(), None, usize::MAX);
    for ((c, s), &d) in ctx.iter_mut().zip(ann).zip(&docs) {
        if d != cur_doc {
            seen.clear();
            prev = None;
            cur_doc = d;
        }
        if c.is_empty() {
            continue;
        }
        let st = stype_rule(s);
        c.stype = Some(st);
        c.prev = Some(prev);
        prev = Some(st);
        let mut ent = Vec::with_capacity(s.tokens.len());
        for x in &s.tokens {
            if matches!(x.upos, Some(UPos::NOUN | UPos::PROPN)) {
                let l = x.lemma.to_lowercase();
                ent.push(Some((EType::Other, if seen.contains(&l) { Info::Giv } else { Info::New })));
                seen.insert(l);
            } else {
                ent.push(None);
            }
        }
        c.ent = Some(ent);
    }
    Ok((ctx, docs))
}

/// Deterministic generator (xorshift) for shuffling.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(splitmix(seed) | 1)
    }
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }
}

fn splitmix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// Training seed: `EN_SEED` (0 or unset = as before, bit for bit). A different seed gives a different shuffling order
/// of sentences in the tagger and parser.
pub fn seed() -> u64 {
    std::env::var("EN_SEED").ok().and_then(|x| x.parse().ok()).unwrap_or(0)
}

/// Initial training shuffle state: `base` without `EN_SEED`, otherwise mixed with the seed.
pub fn seeded(base: u64) -> u64 {
    match seed() {
        0 => base,
        s => (base ^ splitmix(s)) | 1,
    }
}

/// Negative control: each sentence's context comes from a random sentence of another document
/// (of the same length, if any), together with its document's genre. Entities by position:
/// extra ones are dropped, missing ones are "outside mentions". Sentences without context stay without it.
pub fn shuffled(ctx: &[Ctx], docs: &[usize], lens: &[usize], seed: u64) -> Vec<Ctx> {
    let mut rng = Rng::new(seed ^ 0x5348_5546);
    let pool: Vec<usize> = (0..ctx.len()).filter(|&i| !ctx[i].is_empty()).collect();
    let mut by_len: FastMap<usize, Vec<usize>> = FastMap::default();
    for &i in &pool {
        by_len.entry(lens[i]).or_default().push(i);
    }
    let many_docs = pool.iter().any(|&i| docs[i] != docs[pool[0]]);
    ctx.iter()
        .enumerate()
        .map(|(i, c)| {
            if c.is_empty() || !many_docs {
                return c.clone();
            }
            let same: Vec<usize> = by_len.get(&lens[i]).map_or(Vec::new(), |v| v.iter().copied().filter(|&j| docs[j] != docs[i]).collect());
            let j = if same.is_empty() {
                loop {
                    let j = pool[rng.below(pool.len())];
                    if docs[j] != docs[i] {
                        break j;
                    }
                }
            } else {
                same[rng.below(same.len())]
            };
            let mut d = ctx[j].clone();
            if let Some(v) = &mut d.ent {
                v.resize(lens[i], None);
            }
            d
        })
        .collect()
}

/// Context mode: none, gold, shuffled (negative control), derived from the
/// first pass.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mode {
    None,
    Gold,
    Shuffle,
    Pred,
}

impl Mode {
    pub fn parse(s: &str) -> Result<Mode> {
        Ok(match s.trim() {
            "none" | "" => Mode::None,
            "gold" => Mode::Gold,
            "shuffle" => Mode::Shuffle,
            "pred" => Mode::Pred,
            x => bail!("EN_CTX: '{x}': expected none|gold|shuffle|pred"),
        })
    }
    pub fn name(self) -> &'static str {
        match self {
            Mode::None => "none",
            Mode::Gold => "gold",
            Mode::Shuffle => "shuffle",
            Mode::Pred => "pred",
        }
    }
}

/// Experiment settings from environment variables:
/// - `EN_CTX`: training context: none (default: everything as before) | gold | shuffle;
/// - `EN_CTX_EVAL`: evaluation contexts, comma-separated (default: the same as in training);
/// - `EN_CTX_GROUPS`: feature groups (default: all): genre,stype,doc,entity;
/// - `EN_CTX_EWT=1`: EWT genre from the `sent_id` prefix.
#[derive(Clone, Debug)]
pub struct Setup {
    pub train: Mode,
    pub eval: Vec<Mode>,
    pub groups: u8,
    pub ewt: bool,
    /// stage 2 (`EN_CTX_SRC=rule`): sentence type and noun status come from rules over the annotation
    /// (`derived`), not from the gold standard; `EN_PASS1_DIR` is the first-pass annotation for `pred`
    pub rule: bool,
    pub pass1: Option<std::path::PathBuf>,
    /// stage 2: first-pass annotation of the training sentences (jackknife), `EN_CTX_TRAIN_PASS1`
    /// comma-separated; without it the training context is rules over the gold standard
    pub train_pass1: Vec<std::path::PathBuf>,
}

impl Setup {
    pub fn from_env() -> Result<Setup> {
        let train = Mode::parse(&std::env::var("EN_CTX").unwrap_or_default())?;
        if train == Mode::Pred {
            bail!("EN_CTX=pred is only for evaluation (EN_CTX_EVAL); training is gold or shuffle");
        }
        let eval = match std::env::var("EN_CTX_EVAL") {
            Ok(v) if !v.trim().is_empty() => v.split(',').map(Mode::parse).collect::<Result<Vec<_>>>()?,
            _ => vec![train],
        };
        let groups = match std::env::var("EN_CTX_GROUPS") {
            Ok(v) if !v.trim().is_empty() => {
                let mut g = 0u8;
                for x in v.split(',').map(str::trim) {
                    match group::NAMES.iter().find(|(n, _)| *n == x) {
                        Some((_, b)) => g |= b,
                        None if x == "all" => g |= group::ALL,
                        None => bail!("EN_CTX_GROUPS: '{x}': expected genre,stype,doc,entity"),
                    }
                }
                g
            }
            _ => group::ALL,
        };
        let ewt = std::env::var("EN_CTX_EWT").is_ok_and(|v| v == "1");
        let rule = match std::env::var("EN_CTX_SRC").as_deref() {
            Ok("rule") => true,
            Ok("gold") | Err(_) => false,
            Ok(x) => bail!("EN_CTX_SRC: '{x}': expected gold|rule"),
        };
        let pass1 = std::env::var("EN_PASS1_DIR").ok().map(std::path::PathBuf::from);
        if eval.contains(&Mode::Pred) && (!rule || pass1.is_none()) {
            bail!("EN_CTX_EVAL=pred: requires EN_CTX_SRC=rule and EN_PASS1_DIR (first-pass annotation)");
        }
        if train == Mode::None && eval.iter().any(|&m| m != Mode::None) {
            bail!("EN_CTX_EVAL has context, but the model was trained without it (EN_CTX=none): it has no context features");
        }
        let train_pass1: Vec<std::path::PathBuf> = match std::env::var("EN_CTX_TRAIN_PASS1") {
            Ok(v) if !v.trim().is_empty() => v.split(',').map(|x| std::path::PathBuf::from(x.trim())).collect(),
            _ => Vec::new(),
        };
        if !train_pass1.is_empty() && !rule {
            bail!("EN_CTX_TRAIN_PASS1 only with EN_CTX_SRC=rule");
        }
        Ok(Setup { train, eval, groups, ewt, rule, pass1, train_pass1 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entities_nested_and_single() {
        // "a sample of adults": 6 opens at a, 7 at adults (and closes), then 6 and 5 close
        let misc = ["Entity=(5-abstract-new-nnnnn-cf3-1-sgl", "Entity=(6-person-acc:inf-ssnns-cf1-4-coref", "_", "_", "Entity=(7-person-giv:act-ssnns-cf2-1-coref)6)5)", "_"];
        let e = entities(&misc);
        assert_eq!(e[0], Some((EType::Abstract, Info::New)));
        assert_eq!(e[1], Some((EType::Person, Info::Acc)));
        assert_eq!(e[2], Some((EType::Person, Info::Acc)));
        assert_eq!(e[4], Some((EType::Person, Info::Giv)));
        assert_eq!(e[5], None);
        // close and open on one token; description up to the next bracket
        let misc = ["Entity=(1-place-new-x-cf1-1-coref-Durham(2-time-giv:inact-x-cf1-1-sgl)", "Entity=1)", "Entity=(3-event-new-x-cf1-1-sgl)|SpaceAfter=No"];
        let e = entities(&misc);
        assert_eq!(e[0], Some((EType::Time, Info::Giv)));
        assert_eq!(e[1], Some((EType::Place, Info::New)));
        assert_eq!(e[2], Some((EType::Event, Info::New)));
    }



    #[test]
    fn gold_context_from_comments_and_shuffle_control() {
        let row = |i: usize, w: &str, misc: &str| format!("{i}\t{w}\t{w}\tX\tNN\t_\t0\troot\t_\t{misc}\n");
        let text = format!(
            "# newdoc id = d1\n# meta::genre = news\n# newpar\n# sent_id = a\n# s_type = decl\n# speaker = A\n{}\n\
             # sent_id = b\n# s_type = q\n# speaker = B\n{}\n\
             # newdoc id = d2\n# meta::genre = fiction\n# newpar\n# sent_id = c\n# s_type = imp\n{}\n",
            row(1, "Hi", "Entity=(1-person-new-x-cf1-1-sgl)"),
            row(1, "Yes", "_"),
            row(1, "Go", "_")
        );
        let doc = Doc::parse(&text).unwrap();
        let (c, d) = gold(&doc, false);
        assert_eq!(d, [0, 0, 1]);
        assert_eq!((c[0].genre, c[0].stype, c[0].prev, c[0].doc), (Some(Genre::News), Some(SType::Decl), Some(None), Some((true, Turn::Changed))));
        assert_eq!((c[1].stype, c[1].prev, c[1].doc), (Some(SType::Q), Some(Some(SType::Decl)), Some((false, Turn::Changed))));
        assert_eq!((c[2].genre, c[2].prev, c[2].doc), (Some(Genre::Fiction), Some(None), Some((true, Turn::NoSpeaker))));
        assert_eq!(c[0].ent, Some(vec![Some((EType::Person, Info::New))]));
        assert_eq!(c[1].ent_code(0), 1);
        // disabled group: no features; without context: none at all
        assert!(c[0].clone().masked(group::GENRE).ent.is_none());
        assert!(Ctx::default().sent_codes().is_empty() && Ctx::default().ent_code(0) == 0);
        // negative control: the donor is from another document, so the genre is foreign
        let s = shuffled(&c, &d, &[1, 1, 1], 7);
        assert_eq!(s[2].genre, Some(Genre::News));
        assert_eq!(s[0].genre, Some(Genre::Fiction));
    }
}
