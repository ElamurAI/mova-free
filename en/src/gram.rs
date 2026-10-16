//! Grammatical types of the graph — Rust structures instead of strings. PTB tag and UD relation are single-byte
//! enums with a closed list: an unknown relation in a treebank is a read error, not a silent string.
//! Form case is an enum. A lemma is a number (`Sym`) in the lemma heap of the built-in dictionary (`dict`). Strings remain only at the boundary:
//! reading text or a treebank, and printing.

/// Single-byte enum with a name table: `ALL`, `N`, `name()`, `parse()`, `idx()`.
macro_rules! symbols {
    ($(#[$m:meta])* $name:ident { $($v:ident = $s:literal),+ $(,)? }) => {
        $(#[$m])*
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
        #[repr(u8)]
        pub enum $name { $($v),+ }

        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$v),+];
            pub const N: usize = $name::ALL.len();
            pub fn name(self) -> &'static str {
                match self { $($name::$v => $s),+ }
            }
            pub fn parse(s: &str) -> Option<$name> {
                match s { $($s => Some($name::$v),)+ _ => None }
            }
            #[inline]
            pub fn idx(self) -> usize {
                self as usize
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str(self.name())
            }
        }

        /// In the model file — one byte, the index in `ALL`.
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                s.serialize_u8(*self as u8)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let i = u8::deserialize(d)?;
                $name::ALL.get(i as usize).copied().ok_or_else(|| serde::de::Error::custom(format!("{} #{i} out of range", stringify!($name))))
            }
        }
    };
}

symbols! {
    /// Penn Treebank part-of-speech tag (XPOS in UD English) with EWT extensions.
    Tag {
        CC = "CC", CD = "CD", DT = "DT", EX = "EX", FW = "FW", IN = "IN", JJ = "JJ", JJR = "JJR", JJS = "JJS",
        LS = "LS", MD = "MD", NN = "NN", NNS = "NNS", NNP = "NNP", NNPS = "NNPS", PDT = "PDT", POS = "POS",
        PRP = "PRP", PRPS = "PRP$", RB = "RB", RBR = "RBR", RBS = "RBS", RP = "RP", SYM = "SYM", TO = "TO",
        UH = "UH", VB = "VB", VBD = "VBD", VBG = "VBG", VBN = "VBN", VBP = "VBP", VBZ = "VBZ", WDT = "WDT",
        WP = "WP", WPS = "WP$", WRB = "WRB",
        Comma = ",", Stop = ".", Colon = ":", LRB = "-LRB-", RRB = "-RRB-", LQuote = "``", RQuote = "''",
        Pound = "#", Dollar = "$", HYPH = "HYPH", NFP = "NFP", ADD = "ADD", AFX = "AFX", GW = "GW", XX = "XX",
        DQuote = "\"", X = "X",
    }
}

impl Tag {
    pub fn is_verb(self) -> bool {
        matches!(self, Tag::VB | Tag::VBD | Tag::VBG | Tag::VBN | Tag::VBP | Tag::VBZ)
    }
    pub fn is_noun(self) -> bool {
        matches!(self, Tag::NN | Tag::NNS | Tag::NNP | Tag::NNPS)
    }
    pub fn is_proper(self) -> bool {
        matches!(self, Tag::NNP | Tag::NNPS)
    }
    /// Interrogative and relative words: what, which, who, whose, where, when…
    pub fn is_wh(self) -> bool {
        matches!(self, Tag::WDT | Tag::WP | Tag::WPS | Tag::WRB)
    }
    /// Closed classes: in the order generator's keys together with the lemma ("maybe" ≠ "too").
    pub fn is_closed(self) -> bool {
        matches!(
            self,
            Tag::RB | Tag::IN | Tag::DT | Tag::MD | Tag::CC | Tag::TO | Tag::WRB | Tag::WDT | Tag::WP | Tag::PRP | Tag::EX | Tag::RP
                | Tag::UH | Tag::PDT | Tag::POS
        )
    }
}

symbols! {
    /// UD syntactic relation: 37 universal ones and English subtypes (UD 2.18).
    Rel {
        Acl = "acl", AclRelcl = "acl:relcl", Advcl = "advcl", AdvclRelcl = "advcl:relcl", Advmod = "advmod",
        Amod = "amod", Appos = "appos", Aux = "aux", AuxPass = "aux:pass", Case = "case", Cc = "cc",
        CcPreconj = "cc:preconj", Ccomp = "ccomp", Clf = "clf", Compound = "compound", CompoundPrt = "compound:prt",
        Conj = "conj", Cop = "cop", Csubj = "csubj", CsubjOuter = "csubj:outer", CsubjPass = "csubj:pass",
        Dep = "dep", Det = "det", DetPredet = "det:predet", Discourse = "discourse", Dislocated = "dislocated",
        Expl = "expl", Fixed = "fixed", Flat = "flat", FlatForeign = "flat:foreign", Goeswith = "goeswith",
        Iobj = "iobj", List = "list", Mark = "mark", Nmod = "nmod", NmodDesc = "nmod:desc", NmodNpmod = "nmod:npmod",
        NmodPoss = "nmod:poss", NmodTmod = "nmod:tmod", NmodUnmarked = "nmod:unmarked", Nsubj = "nsubj",
        NsubjOuter = "nsubj:outer", NsubjPass = "nsubj:pass", Nummod = "nummod", Obj = "obj", Obl = "obl",
        OblAgent = "obl:agent", OblNpmod = "obl:npmod", OblTmod = "obl:tmod", OblUnmarked = "obl:unmarked",
        Orphan = "orphan", Parataxis = "parataxis", Punct = "punct", Reparandum = "reparandum", Root = "root",
        Vocative = "vocative", Xcomp = "xcomp",
    }
}

impl Rel {
    /// Universal relation without subtype (aux:pass → aux).
    pub fn base(self) -> Rel {
        use Rel::*;
        match self {
            AclRelcl => Acl,
            AdvclRelcl => Advcl,
            AuxPass => Aux,
            CcPreconj => Cc,
            CompoundPrt => Compound,
            CsubjOuter | CsubjPass => Csubj,
            DetPredet => Det,
            FlatForeign => Flat,
            NmodDesc | NmodNpmod | NmodPoss | NmodTmod | NmodUnmarked => Nmod,
            NsubjOuter | NsubjPass => Nsubj,
            OblAgent | OblNpmod | OblTmod | OblUnmarked => Obl,
            r => r,
        }
    }
}

/// How to reproduce a form from the lemma: as is, capitalized, all caps; `Raw` — escape, the graph holds the form itself.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[repr(u8)]
pub enum Case {
    AsIs,
    Title,
    Upper,
    Raw,
}

/// String number: in the model's shared dictionary or, with the high bit set, in the graph's local heap
/// (new words the model does not know).
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, serde::Serialize, serde::Deserialize)]
pub struct Sym(u32);

impl Sym {
    const LOCAL: u32 = 1 << 31;
    /// "None": key for an unknown lemma (the dictionary never has such a number).
    pub const NONE: Sym = Sym(Self::LOCAL - 1);

    pub const fn global(i: u32) -> Sym {
        Sym(i)
    }
    pub fn local(i: usize) -> Sym {
        Sym(Self::LOCAL | i as u32)
    }
    pub fn is_local(self) -> bool {
        self.0 & Self::LOCAL != 0
    }
    pub fn index(self) -> usize {
        (self.0 & !Self::LOCAL) as usize
    }
    pub fn raw(self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip() {
        for &t in Tag::ALL {
            assert_eq!(Tag::parse(t.name()), Some(t));
        }
        for &r in Rel::ALL {
            assert_eq!(Rel::parse(r.name()), Some(r));
        }
        assert_eq!(Rel::parse("aux:pass").map(Rel::base), Some(Rel::Aux));
        assert!(Tag::N < 64 && Rel::N < 64);
    }
}

symbols! {
    /// UD universal part of speech (UPOS).
    UPos {
        ADJ = "ADJ", ADP = "ADP", ADV = "ADV", AUX = "AUX", CCONJ = "CCONJ", DET = "DET", INTJ = "INTJ",
        NOUN = "NOUN", NUM = "NUM", PART = "PART", PRON = "PRON", PROPN = "PROPN", PUNCT = "PUNCT",
        SCONJ = "SCONJ", SYM = "SYM", VERB = "VERB", X = "X",
    }
}

symbols! {
    /// A "feature=value" pair from FEATS of English UD 2.18 (all treebanks). Order — as in CoNLL-U:
    /// by feature name, case-insensitive (Number < NumForm < NumType, Person < Polarity < Poss).
    Feat {
        AbbrYes = "Abbr=Yes",
        CaseAcc = "Case=Acc", CaseGen = "Case=Gen", CaseNom = "Case=Nom",
        DefiniteDef = "Definite=Def", DefiniteInd = "Definite=Ind",
        DegreeCmp = "Degree=Cmp", DegreePos = "Degree=Pos", DegreeSup = "Degree=Sup",
        ExtPosAdp = "ExtPos=ADP", ExtPosAdv = "ExtPos=ADV", ExtPosCconj = "ExtPos=CCONJ", ExtPosNoun = "ExtPos=NOUN",
        ExtPosPron = "ExtPos=PRON", ExtPosPropn = "ExtPos=PROPN", ExtPosSconj = "ExtPos=SCONJ",
        ForeignYes = "Foreign=Yes",
        GenderFem = "Gender=Fem", GenderFemMasc = "Gender=Fem,Masc", GenderMasc = "Gender=Masc", GenderNeut = "Gender=Neut",
        MoodImp = "Mood=Imp", MoodInd = "Mood=Ind", MoodSub = "Mood=Sub",
        NumberPlur = "Number=Plur", NumberPtan = "Number=Ptan", NumberSing = "Number=Sing",
        NumFormCombi = "NumForm=Combi", NumFormDigit = "NumForm=Digit", NumFormRoman = "NumForm=Roman", NumFormWord = "NumForm=Word",
        NumTypeCard = "NumType=Card", NumTypeFrac = "NumType=Frac", NumTypeMult = "NumType=Mult", NumTypeOrd = "NumType=Ord",
        Person1 = "Person=1", Person2 = "Person=2", Person3 = "Person=3",
        PolarityNeg = "Polarity=Neg", PolarityPos = "Polarity=Pos",
        PossYes = "Poss=Yes",
        PronTypeArt = "PronType=Art", PronTypeDem = "PronType=Dem", PronTypeEmp = "PronType=Emp", PronTypeInd = "PronType=Ind",
        PronTypeInt = "PronType=Int", PronTypeIntRel = "PronType=Int,Rel", PronTypeNeg = "PronType=Neg", PronTypePrs = "PronType=Prs",
        PronTypeRcp = "PronType=Rcp", PronTypeRel = "PronType=Rel", PronTypeTot = "PronType=Tot",
        ReflexYes = "Reflex=Yes",
        StyleArch = "Style=Arch", StyleColl = "Style=Coll", StyleExpr = "Style=Expr", StyleSlng = "Style=Slng", StyleVrnc = "Style=Vrnc",
        TensePast = "Tense=Past", TensePres = "Tense=Pres",
        TypoYes = "Typo=Yes",
        VerbFormFin = "VerbForm=Fin", VerbFormGer = "VerbForm=Ger", VerbFormInf = "VerbForm=Inf", VerbFormPart = "VerbForm=Part",
        VoicePass = "Voice=Pass",
    }
}

impl Feat {
    /// Feature name (before "=").
    pub fn key(self) -> &'static str {
        self.name().split_once('=').map_or("", |x| x.0)
    }
}

/// UD features of 2018 measured by UFeats (CoNLL 2018): the rest (NumForm, ExtPos, Style, Typo) are language-specific.
const UNIVERSAL: &[&str] = &[
    "PronType", "NumType", "Poss", "Reflex", "Foreign", "Abbr", "Gender", "Animacy", "Number", "Case", "Definite", "Degree",
    "VerbForm", "Mood", "Tense", "Aspect", "Voice", "Evident", "Polarity", "Person", "Polite",
];

/// Morphological features of a token — bits of `Feat` pairs (70 pairs of English UD fit into u128).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Feats(pub u128);

impl Feats {
    /// Parsing the FEATS column; the second value — how many pairs are outside `Feat` (they are dropped).
    pub fn parse(s: &str) -> (Feats, usize) {
        let mut f = Feats::default();
        let mut unknown = 0;
        if s != "_" {
            for p in s.split('|') {
                match Feat::parse(p) {
                    Some(x) => f.0 |= 1 << x.idx(),
                    None => unknown += 1,
                }
            }
        }
        (f, unknown)
    }
    pub fn has(self, x: Feat) -> bool {
        self.0 >> x.idx() & 1 == 1
    }
    pub fn iter(self) -> impl Iterator<Item = Feat> {
        Feat::ALL.iter().copied().filter(move |&x| self.has(x))
    }
    /// Universal features only (for UFeats).
    pub fn universal(self) -> Feats {
        Feats(self.iter().filter(|x| UNIVERSAL.contains(&x.key())).fold(0, |a, x| a | 1 << x.idx()))
    }
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for Feats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_empty() {
            return f.write_str("_");
        }
        let v: Vec<&str> = self.iter().map(Feat::name).collect();
        f.write_str(&v.join("|"))
    }
}
