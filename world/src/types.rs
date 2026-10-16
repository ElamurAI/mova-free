//! World types: identities and references to states, position in the text, closed sets, fields and values.
//! A value outside a closed set is a parse error, not a silent string.

use std::fmt;

/// Global registry — first letter of the id: locations, objects, characters, relations, events.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Reg {
    L,
    O,
    C,
    R,
    E,
    /// intentions: promises, plans, agreements, wishes, threats, orders, curses (v2)
    P,
    /// beliefs and knowledge of characters (v2)
    B,
    /// narrative frames (v3): not registry entities, only ids for the change language
    F,
    /// observers' claims about a canonical event (v3)
    K,
}

impl Reg {
    pub fn letter(self) -> char {
        match self {
            Reg::L => 'L',
            Reg::O => 'O',
            Reg::C => 'C',
            Reg::R => 'R',
            Reg::E => 'E',
            Reg::P => 'P',
            Reg::B => 'B',
            Reg::F => 'F',
            Reg::K => 'K',
        }
    }

    /// Registry name for gate messages.
    pub fn uk(self) -> &'static str {
        match self {
            Reg::L => "locations",
            Reg::O => "objects",
            Reg::C => "characters",
            Reg::R => "relations",
            Reg::E => "events",
            Reg::P => "intentions",
            Reg::B => "beliefs",
            Reg::F => "frames",
            Reg::K => "claims",
        }
    }
}

/// Entity identity: `C1`, `L2`, `E7`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct Id {
    pub reg: Reg,
    pub n: u32,
}

impl Id {
    pub fn new(reg: Reg, n: u32) -> Id {
        Id { reg, n }
    }

    pub fn parse(s: &str) -> Option<Id> {
        let mut ch = s.chars();
        let reg = match ch.next()? {
            'L' => Reg::L,
            'O' => Reg::O,
            'C' => Reg::C,
            'R' => Reg::R,
            'E' => Reg::E,
            'P' => Reg::P,
            'B' => Reg::B,
            'F' => Reg::F,
            'K' => Reg::K,
            _ => return None,
        };
        let rest = ch.as_str();
        if rest.is_empty() || rest.len() > 6 || rest.starts_with('0') || !rest.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        Some(Id { reg, n: rest.parse().ok()? })
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.reg.letter(), self.n)
    }
}

/// Reference to a specific immutable state of an entity: `C1@3` (like a commit).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SRef {
    pub id: Id,
    pub ver: u32,
}

impl fmt::Display for SRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.id, self.ver)
    }
}

/// Position in the text: chapter, paragraph, sentence (sentence number in the document, from 1).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct TextPos {
    pub chapter: u16,
    pub para: u16,
    pub sent: u16,
}

impl TextPos {
    /// Full form: `ch.1 para.2 sent.6`.
    pub fn long(&self) -> String {
        format!("ch.{} para.{} sent.{}", self.chapter, self.para, self.sent)
    }
}

impl fmt::Display for TextPos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "^s{}", self.sent)
    }
}

// ── Closed sets ────────────────────────────────────────────────────────────────────────────────

/// A value from a closed set. The field is private: from input, a tag is produced only by `Closed::tag`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Tag(&'static str);

impl Tag {
    pub fn as_str(self) -> &'static str {
        self.0
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.0)
    }
}

/// A closed set of values.
pub struct Closed {
    pub what: &'static str,
    pub all: &'static [&'static str],
}

impl Closed {
    pub fn tag(&'static self, s: &str) -> Result<Tag, String> {
        self.all.iter().find(|x| **x == s).map(|x| Tag(x)).ok_or_else(|| format!("{}={s:?} outside the closed set", self.what))
    }

    /// List for the prompt: `a | b | c`.
    pub fn list(&self) -> String {
        self.all.join(" ")
    }
}

// v1 sets (fables) remain; v2 appends general fairy-tale vocabulary at the end — the list was compiled before
// the FairytaleQA run, without looking at the reference answers.
pub static LOC_TYPE: Closed = Closed {
    what: "loc type",
    all: &[
        "field", "meadow", "forest", "tree", "ground", "road", "path", "wayside", "course", "finish", "river", "lake", "sea", "shore",
        "mountain", "hill", "cave", "den", "nest", "house", "farm", "village", "town", "palace", "garden", "sky", "place",
        // v2
        "room", "hall", "kitchen", "castle", "mill", "church", "well", "bridge", "island", "ship", "boat", "country", "kingdom", "city",
        "underworld", "barn", "stable", "yard", "market", "inn", "shop", "grave", "land", "water", "hut",
    ],
};

pub static OBJ_TYPE: Closed = Closed {
    what: "obj type",
    all: &[
        "food", "drink", "plant", "tree", "tool", "rope", "net", "weapon", "container", "treasure", "clothing", "body_part", "trap", "stone",
        "water", "vehicle", "thing",
        // v2
        "money", "jewel", "ring", "key", "letter", "book", "instrument", "furniture", "fire", "animal", "magic", "medicine", "building",
    ],
};

pub static CHAR_TYPE: Closed = Closed { what: "char type", all: &["human", "animal", "magical", "deity", "personified"] };

/// Relation kinds. Symmetric (v1 + `spouse`) and directed: "a is <kind> for b" (`benefactor`: a helped
/// b, b owes a; `master`: a is b's master; `parent`: a is b's father or mother …).
pub static REL_KIND: Closed = Closed {
    what: "rel kind",
    all: &[
        "kin", "romantic", "friend", "ally", "acquaintance", "rival", "adversary", "stranger", // v1
        "spouse", "benefactor", "master", "parent", "ruler", "captor", "suitor", "mentor", // v2
    ],
};

/// Directed relation kinds (v2): in queries they read as "a → b".
pub const REL_DIRECTED: &[&str] = &["benefactor", "master", "parent", "ruler", "captor", "suitor", "mentor"];

pub static GENDER: Closed = Closed { what: "gender", all: &["male", "female"] };

pub static AGE: Closed = Closed { what: "age", all: &["young", "adult", "old"] };

pub static FEELING: Closed = Closed {
    what: "feeling",
    all: &[
        "happy", "sad", "angry", "afraid", "proud", "ashamed", "grateful", "amused", "scornful", "eager", "offended", "calm", "tired",
        "surprised", "hopeful", "content", "confident",
        // v2: general emotion vocabulary
        "worried", "jealous", "curious", "embarrassed", "disappointed", "relieved", "excited", "lonely", "upset", "shocked", "horrified",
        "disgusted", "guilty", "sorry", "desperate", "impatient", "annoyed", "loving", "pitying", "admiring", "suspicious", "greedy",
        "brave", "helpless", "sympathetic", "puzzled",
    ],
};

/// Intention kind (v2).
pub static PLAN_KIND: Closed = Closed {
    what: "plan kind",
    all: &["promise", "plan", "agreement", "wish", "threat", "order", "task", "curse", "prophecy", "bargain", "bet"],
};

/// Intention state: open → kept / broken / dropped (v2).
pub static PLAN_STATUS: Closed = Closed { what: "plan status", all: &["open", "kept", "broken", "dropped"] };

/// Belief state: held → dropped (v2).
pub static BELIEF_STATUS: Closed = Closed { what: "belief status", all: &["held", "dropped"] };

/// Quantity in words (v2); a number is written in digits.
pub static NUM_WORD: Closed = Closed { what: "num", all: &["some", "few", "several", "many", "all", "pair"] };

pub static SLEEP: Closed = Closed { what: "sleep", all: &["awake", "asleep"] };

pub static FREEDOM: Closed = Closed { what: "freedom", all: &["free", "captive"] };

pub static LIFE: Closed = Closed { what: "life", all: &["alive", "dead"] };

// ── v3: non-linear narrative ─────────────────────────────────────────────────────────────────────────

/// Reality branch of an event (v3): happened, dream, plan, intention, lie, "what if" hypothesis, retelling of others' words,
/// a character's belief. Only events of the `real` branch change the canonical world.
pub static BRANCH: Closed = Closed { what: "branch", all: &["real", "dream", "plan", "intent", "lie", "if", "report", "belief"] };

/// Status of an observer's claim (v3): believes, doubts, is mistaken, knows.
pub static CLAIM_STATUS: Closed = Closed { what: "claim status", all: &["believes", "doubts", "wrong", "knows"] };

/// Source of knowledge (v3).
pub static KNOW_HOW: Closed = Closed { what: "know how", all: &["saw", "heard", "told", "inferred", "revealed", "guessed"] };

/// World of a frame (v3): the same one (testimony, memory) or a new one (a story within a story).
pub static FRAME_WORLD: Closed = Closed { what: "frame world", all: &["same", "new"] };

/// Time of a frame relative to the parent frame (v3): past (memory, testimony) or present (a parallel line).
pub static FRAME_TIME: Closed = Closed { what: "frame time", all: &["past", "now"] };

// ── Fields and values ───────────────────────────────────────────────────────────────────────────────

/// Single-valued state field. `Type`/`Class` are identity (written at creation), `At`/`Parent`/
/// `A`/`B` are links, the rest are simple attributes.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Field {
    Type,
    Class,
    Name,
    At,
    Parent,
    A,
    B,
    Kind,
    Gender,
    Age,
    Mother,
    Father,
    Trait,
    Feeling,
    Sleep,
    Freedom,
    Life,
    Goal,
    Belief,
    Status,
    // v2
    /// quantity (group character, item)
    Num,
    /// part of a whole: O.partof = C|O
    PartOf,
    /// intention: text ("what"); belief: the claim
    What,
    /// intention: to whom (to=C)
    Target,
    /// intention: source utterance; belief: where from (from=E)
    Src,
    /// intention state (open/kept/broken/dropped) or belief state (held/dropped)
    Stage,
    /// event that closed the intention or changed the belief
    By,
    /// belief: about what (E/P/C/O/L)
    About,
    /// belief: true in the tale's world (1/0)
    Truth,
    // v3
    /// claim: who (observer — a character, the reader or the narrator)
    Who,
}

impl Field {
    pub fn name(self) -> &'static str {
        match self {
            Field::Type => "type",
            Field::Class => "class",
            Field::Name => "name",
            Field::At => "at",
            Field::Parent => "in",
            Field::A => "a",
            Field::B => "b",
            Field::Kind => "kind",
            Field::Gender => "gender",
            Field::Age => "age",
            Field::Mother => "mother",
            Field::Father => "father",
            Field::Trait => "trait",
            Field::Feeling => "feeling",
            Field::Sleep => "sleep",
            Field::Freedom => "freedom",
            Field::Life => "life",
            Field::Goal => "goal",
            Field::Belief => "belief",
            Field::Status => "status",
            Field::Num => "num",
            Field::PartOf => "part_of",
            Field::What => "what",
            Field::Target => "to",
            Field::Src => "src",
            Field::Stage => "status",
            Field::By => "by",
            Field::About => "about",
            Field::Truth => "true",
            Field::Who => "who",
        }
    }
}

/// Set field (a set of references).
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum SetF {
    Chars,
    Objects,
    Children,
    Items,
    Relations,
    Facts,
    // v2
    /// parts of a whole (C or O): `X.parts ∋ O ⇔ O.part_of = X`
    Parts,
    /// a character's intentions: `C.plans ∋ P ⇔ P.owners ∋ C`
    Plans,
    /// a character's beliefs: `C.beliefs ∋ B ⇔ B.owners ∋ C`
    Beliefs,
    /// owners of an intention or belief
    Owners,
}

impl SetF {
    pub fn name(self) -> &'static str {
        match self {
            SetF::Chars => "chars",
            SetF::Objects => "objects",
            SetF::Children => "places",
            SetF::Items => "items",
            SetF::Relations => "relations",
            SetF::Facts => "facts",
            SetF::Parts => "parts",
            SetF::Plans => "plans",
            SetF::Beliefs => "beliefs",
            SetF::Owners => "owners",
        }
    }
}

/// Attribute value domain.
#[derive(Clone, Copy)]
pub enum Dom {
    /// quoted text (or a single word)
    Text,
    /// a word or comma-separated words
    Words,
    /// reference to a character
    Char,
    /// place (L or C) — only so that the teleport gate says "no" to `set … at=`
    Place,
    Tags(&'static Closed),
    /// several closed-set values separated by commas (feelings, v2); a single value — as for `Tags`
    TagSet(&'static Closed),
    /// a number in digits or a word from `NUM_WORD` (v2)
    Num,
}

/// Attribute keys for `char … key=v` and `set X key=v`.
pub fn key(s: &str) -> Option<(Field, Dom)> {
    Some(match s {
        "name" => (Field::Name, Dom::Text),
        "gender" => (Field::Gender, Dom::Tags(&GENDER)),
        "age" => (Field::Age, Dom::Tags(&AGE)),
        "mother" => (Field::Mother, Dom::Char),
        "father" => (Field::Father, Dom::Char),
        "trait" => (Field::Trait, Dom::Words),
        "feeling" => (Field::Feeling, Dom::TagSet(&FEELING)),
        "sleep" => (Field::Sleep, Dom::Tags(&SLEEP)),
        "freedom" => (Field::Freedom, Dom::Tags(&FREEDOM)),
        "life" => (Field::Life, Dom::Tags(&LIFE)),
        "goal" => (Field::Goal, Dom::Text),
        "belief" => (Field::Belief, Dom::Text),
        "status" => (Field::Status, Dom::Words),
        "num" => (Field::Num, Dom::Num),
        "at" | "to" | "in" => (Field::At, Dom::Place),
        _ => return None,
    })
}

/// Which attributes a registry entity has: character — all, object — name, traits, status, number, location —
/// name, traits, status.
pub fn key_ok(reg: Reg, f: Field) -> bool {
    match reg {
        Reg::C => true,
        Reg::O => matches!(f, Field::Name | Field::Trait | Field::Status | Field::At | Field::Num),
        Reg::L => matches!(f, Field::Name | Field::Trait | Field::Status | Field::At),
        Reg::R | Reg::E | Reg::P | Reg::B | Reg::F | Reg::K => false,
    }
}

/// Field value.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum V {
    Id(Id),
    Tag(Tag),
    Words(Vec<String>),
    Text(String),
    /// several tags (feelings, v2); always ≥ 2 and sorted — a single value is written as `Tag`
    Tags(Vec<Tag>),
}

impl V {
    /// Tag set from several: one gives `Tag` (as in v1), several give `Tags` (sorted, without duplicates).
    pub fn tags(mut v: Vec<Tag>) -> Option<V> {
        v.sort();
        v.dedup();
        match v.len() {
            0 => None,
            1 => Some(V::Tag(v[0])),
            _ => Some(V::Tags(v)),
        }
    }

    /// Tags of the value (one for `Tag`).
    pub fn tag_list(&self) -> Vec<Tag> {
        match self {
            V::Tag(t) => vec![*t],
            V::Tags(v) => v.clone(),
            _ => Vec::new(),
        }
    }
}

impl fmt::Display for V {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            V::Id(i) => write!(f, "{i}"),
            V::Tag(t) => write!(f, "{t}"),
            V::Words(w) => f.write_str(&w.join(",")),
            V::Text(t) => write!(f, "«{t}»"),
            V::Tags(v) => f.write_str(&v.iter().map(|t| t.as_str()).collect::<Vec<_>>().join(",")),
        }
    }
}

/// Value that is unknown by default.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub enum Val {
    #[default]
    Unknown,
    Known(V),
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Val::Unknown => f.write_str("?"),
            Val::Known(v) => write!(f, "{v}"),
        }
    }
}

/// Word: lowercase Latin letters, digits, `_`, `-`; up to 32 characters.
pub fn is_word(s: &str) -> bool {
    let b = s.as_bytes();
    !b.is_empty() && b.len() <= 32 && b[0].is_ascii_lowercase() && b.iter().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'_' || *c == b'-')
}

/// Maximum number of words in a text value (why, means, goal, belief).
pub const TEXT_MAX_WORDS: usize = 25;
