//! The world-model model: concepts and words are integer ids, relations, properties and layers are enums; strings appear only
//! at input (XML, TSV, query) and output (an answer with a proof).
//!
//! The graph is arrays (CSR): outgoing and incoming edges of each concept are contiguous, no maps, so the snapshot loads
//! with a single file read.

/// Concept id.
pub type Cid = u32;
/// Word (lemma) id — index in the sorted dictionary.
pub type Wid = u32;

/// Part of speech of a concept. `Frame` and `Role` are FrameNet concepts.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Pos {
    Noun,
    Verb,
    Adj,
    /// OEWN satellite adjective (`s`)
    Sat,
    Adv,
    Frame,
    Role,
}

impl Pos {
    pub const ALL: [Pos; 7] = [Pos::Noun, Pos::Verb, Pos::Adj, Pos::Sat, Pos::Adv, Pos::Frame, Pos::Role];
    pub const N: usize = 7;

    pub fn from_wn(s: &str) -> Option<Pos> {
        Some(match s {
            "n" => Pos::Noun,
            "v" => Pos::Verb,
            "a" => Pos::Adj,
            "s" => Pos::Sat,
            "r" => Pos::Adv,
            _ => return None,
        })
    }

    pub fn letter(self) -> char {
        match self {
            Pos::Noun => 'n',
            Pos::Verb => 'v',
            Pos::Adj => 'a',
            Pos::Sat => 's',
            Pos::Adv => 'r',
            Pos::Frame => 'F',
            Pos::Role => 'R',
        }
    }

    pub fn from_u8(b: u8) -> Option<Pos> {
        Pos::ALL.get(b as usize).copied()
    }

    /// Adjective and satellite are one part of speech for word lookup.
    pub fn same_word_class(self, o: Pos) -> bool {
        self == o || matches!((self, o), (Pos::Adj, Pos::Sat) | (Pos::Sat, Pos::Adj))
    }
}

/// Where the concept comes from.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Src {
    /// OEWN synset; key — offset (8-digit id), part of speech is in `Pos`
    Oewn,
    /// FrameNet frame; key — frame ID
    Frame,
    /// frame role (frame element); key — role ID
    Role,
    /// Wikidata item; key — Q number (not loaded in v1, only a trial mapping)
    Wikidata,
}

impl Src {
    pub const ALL: [Src; 4] = [Src::Oewn, Src::Frame, Src::Role, Src::Wikidata];
    pub fn from_u8(b: u8) -> Option<Src> {
        Src::ALL.get(b as usize).copied()
    }
}

/// Typed relation between concepts. Direction is as read: `a IsA b` — «a is a kind of b».
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Rel {
    /// kind → genus (OEWN hypernym)
    IsA,
    /// instance → class (OEWN instance_hypernym; seeds)
    InstanceOf,
    /// part → whole (OEWN holo_part)
    PartOf,
    /// member → group (OEWN holo_member)
    MemberOf,
    /// whole → substance (OEWN mero_substance): bread MadeOf flour
    MadeOf,
    /// verb → what it entails (OEWN entails): snore → sleep
    Entails,
    /// verb → consequence (OEWN causes): kill → die
    Causes,
    /// frame → role
    HasRole,
    /// frame → parent frame (FrameNet Inheritance)
    FrameInherits,
    /// frame → frame it uses (Using)
    FrameUses,
    /// frame → whole frame (Subframe: subframe → complex frame)
    SubframeOf,
    /// frame → neutral frame it is a perspective on (Perspective_on)
    PerspectiveOn,
    /// frame → frame that follows (Precedes)
    Precedes,
    /// causative frame → state frame (Causative_of)
    CausativeOf,
    /// inchoative frame → state frame (Inchoative_of)
    InchoativeOf,
}

impl Rel {
    pub const ALL: [Rel; 15] = [
        Rel::IsA,
        Rel::InstanceOf,
        Rel::PartOf,
        Rel::MemberOf,
        Rel::MadeOf,
        Rel::Entails,
        Rel::Causes,
        Rel::HasRole,
        Rel::FrameInherits,
        Rel::FrameUses,
        Rel::SubframeOf,
        Rel::PerspectiveOn,
        Rel::Precedes,
        Rel::CausativeOf,
        Rel::InchoativeOf,
    ];
    pub const N: usize = 15;

    pub fn from_u8(b: u8) -> Option<Rel> {
        Rel::ALL.get(b as usize).copied()
    }

    pub fn name(self) -> &'static str {
        match self {
            Rel::IsA => "IsA",
            Rel::InstanceOf => "InstanceOf",
            Rel::PartOf => "PartOf",
            Rel::MemberOf => "MemberOf",
            Rel::MadeOf => "MadeOf",
            Rel::Entails => "Entails",
            Rel::Causes => "Causes",
            Rel::HasRole => "HasRole",
            Rel::FrameInherits => "Inherits",
            Rel::FrameUses => "Uses",
            Rel::SubframeOf => "SubframeOf",
            Rel::PerspectiveOn => "PerspectiveOn",
            Rel::Precedes => "Precedes",
            Rel::CausativeOf => "CausativeOf",
            Rel::InchoativeOf => "InchoativeOf",
        }
    }

    /// «Upward» relation in the hierarchy (kind → genus) — for inheritance.
    pub fn is_taxonomic(self) -> bool {
        matches!(self, Rel::IsA | Rel::InstanceOf)
    }
}

/// FrameNet role type (coreType).
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Core {
    Core,
    CoreUnexpressed,
    Peripheral,
    ExtraThematic,
}

impl Core {
    pub const ALL: [Core; 4] = [Core::Core, Core::CoreUnexpressed, Core::Peripheral, Core::ExtraThematic];
    pub fn parse(s: &str) -> Option<Core> {
        Some(match s {
            "Core" => Core::Core,
            "Core-Unexpressed" => Core::CoreUnexpressed,
            "Peripheral" => Core::Peripheral,
            "Extra-Thematic" => Core::ExtraThematic,
            _ => return None,
        })
    }
    pub fn name(self) -> &'static str {
        match self {
            Core::Core => "Core",
            Core::CoreUnexpressed => "Core-Unexpressed",
            Core::Peripheral => "Peripheral",
            Core::ExtraThematic => "Extra-Thematic",
        }
    }
}

/// OEWN lexicographer files — coarse semantic classes (supersenses). The index in the table is what
/// is stored in the concept. For frames and roles — `-`.
pub const LEXFILES: &[&str] = &[
    "-",
    "adj.all",
    "adj.pert",
    "adj.ppl",
    "adv.all",
    "noun.Tops",
    "noun.act",
    "noun.animal",
    "noun.artifact",
    "noun.attribute",
    "noun.body",
    "noun.cognition",
    "noun.communication",
    "noun.event",
    "noun.feeling",
    "noun.food",
    "noun.group",
    "noun.location",
    "noun.motive",
    "noun.object",
    "noun.person",
    "noun.phenomenon",
    "noun.plant",
    "noun.possession",
    "noun.process",
    "noun.quantity",
    "noun.relation",
    "noun.shape",
    "noun.state",
    "noun.substance",
    "noun.time",
    "verb.body",
    "verb.change",
    "verb.cognition",
    "verb.communication",
    "verb.competition",
    "verb.consumption",
    "verb.contact",
    "verb.creation",
    "verb.emotion",
    "verb.motion",
    "verb.perception",
    "verb.possession",
    "verb.social",
    "verb.stative",
    "verb.weather",
];

pub fn lexfile_id(s: &str) -> Option<u8> {
    LEXFILES.iter().position(|x| *x == s).map(|i| i as u8)
}

/// Edge: relation and target.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Edge {
    pub rel: Rel,
    pub to: Cid,
}

/// FrameNet lexical unit: word with part of speech → frame.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Lu {
    pub word: Wid,
    pub pos: Pos,
    pub frame: Cid,
    /// FrameNet lexical unit ID — for the proof
    pub id: u32,
}

/// The whole graph: concepts, words, senses, relations, lexical units. Built from OEWN and FrameNet
/// (`build.rs`), stored as a snapshot (`store.rs`).
#[derive(Default, Debug, PartialEq)]
pub struct Graph {
    // concepts (structure of arrays)
    pub src: Vec<Src>,
    pub key: Vec<u32>,
    pub pos: Vec<Pos>,
    pub lexfile: Vec<u8>,
    /// bit 0 — only in OEWN `-plus`; for a role — the `Core` number in bits 1–2
    pub flags: Vec<u8>,
    /// OEWN ILI (`i43203` → 43203), 0 — none
    pub ili: Vec<u32>,
    /// gloss: offsets into `gloss`
    pub gloss_off: Vec<u32>,
    pub gloss: String,
    /// frame or role name: offsets into `names` (empty for synsets)
    pub name_off: Vec<u32>,
    pub names: String,
    /// synset members (lemmas) — CSR
    pub mem_off: Vec<u32>,
    pub mem: Vec<Wid>,
    // words: sorted strings
    pub word_off: Vec<u32>,
    pub words: String,
    /// word senses in frequency order (all parts of speech) — CSR by word
    pub sense_off: Vec<u32>,
    pub senses: Vec<Cid>,
    // edges
    pub out_off: Vec<u32>,
    pub out: Vec<Edge>,
    pub in_off: Vec<u32>,
    pub inn: Vec<Edge>,
    /// lexical units sorted by word
    pub lus: Vec<Lu>,
}

pub const FLAG_PLUS: u8 = 1;

impl Graph {
    pub fn n_concepts(&self) -> usize {
        self.src.len()
    }
    pub fn n_words(&self) -> usize {
        self.word_off.len().saturating_sub(1)
    }

    fn slice<'a>(pool: &'a str, off: &[u32], i: usize) -> &'a str {
        &pool[off[i] as usize..off[i + 1] as usize]
    }

    pub fn word(&self, w: Wid) -> &str {
        Self::slice(&self.words, &self.word_off, w as usize)
    }
    pub fn gloss_of(&self, c: Cid) -> &str {
        Self::slice(&self.gloss, &self.gloss_off, c as usize)
    }
    pub fn name_of(&self, c: Cid) -> &str {
        Self::slice(&self.names, &self.name_off, c as usize)
    }
    pub fn members(&self, c: Cid) -> &[Wid] {
        &self.mem[self.mem_off[c as usize] as usize..self.mem_off[c as usize + 1] as usize]
    }
    pub fn out_edges(&self, c: Cid) -> &[Edge] {
        &self.out[self.out_off[c as usize] as usize..self.out_off[c as usize + 1] as usize]
    }
    pub fn in_edges(&self, c: Cid) -> &[Edge] {
        &self.inn[self.in_off[c as usize] as usize..self.in_off[c as usize + 1] as usize]
    }
    pub fn senses_of(&self, w: Wid) -> &[Cid] {
        &self.senses[self.sense_off[w as usize] as usize..self.sense_off[w as usize + 1] as usize]
    }

    /// Word by string — binary search in the sorted dictionary.
    pub fn word_id(&self, s: &str) -> Option<Wid> {
        let (mut lo, mut hi) = (0usize, self.n_words());
        while lo < hi {
            let mid = (lo + hi) / 2;
            match self.word(mid as Wid).cmp(s) {
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
                std::cmp::Ordering::Equal => return Some(mid as Wid),
            }
        }
        None
    }

    /// Lexical units of a word.
    pub fn lus_of(&self, w: Wid) -> &[Lu] {
        let lo = self.lus.partition_point(|l| l.word < w);
        let hi = self.lus.partition_point(|l| l.word <= w);
        &self.lus[lo..hi]
    }

    /// Human-readable concept id: `oewn-01505702-n`, `fn:Commerce_buy`, `fe:Commerce_buy.Buyer`.
    pub fn cid_str(&self, c: Cid) -> String {
        let i = c as usize;
        match self.src[i] {
            Src::Oewn => format!("oewn-{:08}-{}", self.key[i], self.pos[i].letter()),
            Src::Frame => format!("fn:{}", self.name_of(c)),
            Src::Role => format!("fe:{}", self.name_of(c)),
            Src::Wikidata => format!("Q{}", self.key[i]),
        }
    }

    /// Short concept name: the first synset lemma or the frame name.
    pub fn label(&self, c: Cid) -> String {
        match self.src[c as usize] {
            Src::Oewn => self.members(c).first().map(|w| self.word(*w).to_string()).unwrap_or_default(),
            _ => self.name_of(c).to_string(),
        }
    }

    /// `oewn-01505702-n «bird»`
    pub fn show(&self, c: Cid) -> String {
        match self.src[c as usize] {
            Src::Oewn => format!("{} «{}»", self.cid_str(c), self.label(c)),
            _ => self.cid_str(c),
        }
    }

    pub fn lexfile_name(&self, c: Cid) -> &'static str {
        LEXFILES.get(self.lexfile[c as usize] as usize).copied().unwrap_or("?")
    }

    pub fn core_of(&self, c: Cid) -> Option<Core> {
        (self.src[c as usize] == Src::Role).then(|| Core::ALL[((self.flags[c as usize] >> 1) & 3) as usize])
    }

    /// Concept by human-readable id (`oewn-01505702-n`) — linear search only for synsets by key.
    pub fn find_synset(&self, id: &str) -> Option<Cid> {
        let rest = id.strip_prefix("oewn-")?;
        let (num, p) = rest.split_once('-')?;
        let key: u32 = num.parse().ok()?;
        let pos = Pos::from_wn(p)?;
        (0..self.n_concepts()).find(|&i| self.src[i] == Src::Oewn && self.key[i] == key && self.pos[i] == pos).map(|i| i as Cid)
    }
}
