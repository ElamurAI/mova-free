//! Frames — the pragmatic structure of any English text, part of Mova's skeleton next to the UD tree: who did
//! what to whom, where, why, and what came next. One general reader for every client (the knowledge courses' student,
//! the tales reader, the maths reader, the brain); docs/pragmatics.md.
//!
//! ## Contract
//!
//! ```text
//! read_with(text, &mut ctx, question, parse) -> Read { frames, asked, aliases, roots }
//! ```
//! - `parse`: a sentence → annotated UD words (lemmas lower case); the caller owns it (world wraps the `en`
//!   annotator with its matrix repairs: `world::frames::read`).
//! - `ctx: Ctx` — the discourse: entities met so far with salience; carry it across the sentences of one text
//!   (a lesson, a story), start a new one for an unrelated text. Pronouns resolve against it.
//! - Every predicate of a tree (a verb, a word with a copula) is one `Frame`: `kind` (Event, Fact(Link),
//!   Say(Act), Bare for a phrase without a predicate), `agent`, `verb` (lemma + particle; «be» for a copula), `neg`,
//!   `patient`, `recipient`, `place`, `attrs`, `obls` (other prepositional roles), `quote` (the frames a Say frame
//!   carries — its patient), `cause` and `next` (indices into `frames`), `cond` (an «if» clause), `irrealis` (not
//!   said to have happened: a wish, a plan, an attempt, an «if» clause; set by the implication signatures of
//!   `data/signatures.tsv`, which also flip `neg` for «failed to», «did not manage to»), `quoted`.
//! - Links: «because / since / as», a purpose «to …» → `cause`; «so / therefore» → the previous clause is the
//!   cause; «and / then / after / before / when / until», consecutive sentences → `next` (the maxim of manner:
//!   told in order = happened in order); ccomp of «say / think / know …» and quoted speech → `quote`.
//! - Entities (`Ent`) are compared by head lemma (`Ent::same`): «the Cheshire Cat» = «the Cat» = cat; pronouns
//!   carry what they were resolved from. Unresolved «it / they» match nothing.
//! - Questions (`question = true`): the first wh-word gives `Asked { frame, what: Ask }` — Cause (why), Next
//!   (what happened next), Role(slot) (who → agent, what → patient, what … do → verb, where → place, how → attr),
//!   Frame (yes/no, when, imperatives). Do-support is put in statement order before parsing (`statement_order`).
//! - `tam`: tense, aspect, voice and modality of the verb group (`tam_of`; `PRAG_OFF=tam` leaves it unread).
//! - What the tree says beside the roles, kept so the frames can be said back (not read by the student): on a frame
//!   `link` (the frame above, the clause's relation, the noun of a relative clause), `unsaid` (roles filled from
//!   elsewhere), `manner`, `words` (other adverbs, discourse words), `nps` (role-less noun phrases), `mark`, `cc`; on
//!   an entity `poss`, `mods`, `appos`, `words`, `conj`, `det`, `case`, `cc`, `rel_word` (docs/pragmatics.md).
//! - Deterministic: word order, ordered maps, no hashing; the same text and ctx give the same frames.
//!
//! Not yet: a coref model (the `coref` crate's sieve works on whole documents; here a salience heuristic, babi-style),
//! time order beyond `next`, quantities (world's mentions); the student reads modality only as `cond`, `irrealis` and Fact(Can).

use std::collections::{BTreeMap, BTreeSet};

use en::annotate::Word;
pub use en::gram::{Rel, Tag as PTag, UPos};

use crate::schema::Act;

/// How a general statement relates its subject to its object (the links of world's know.rs, which reads the
/// same statements into facts).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
pub enum Link {
    IsA,
    Is,
    Has,
    Can,
    Does,
    UsedFor,
    MadeOf,
    LivesIn,
}

impl Link {
    pub fn name(self) -> &'static str {
        match self {
            Link::IsA => "is a",
            Link::Is => "is",
            Link::Has => "has",
            Link::Can => "can",
            Link::Does => "does",
            Link::UsedFor => "used for",
            Link::MadeOf => "made of",
            Link::LivesIn => "lives in",
        }
    }
}

/// A UD tree over annotated words (heads 1-based), with each node's children.
#[derive(Clone, Debug, Default)]
pub struct Tree {
    pub w: Vec<Word>,
    kids: Vec<Vec<usize>>,
}

impl Tree {
    pub fn of_words(w: Vec<Word>) -> Tree {
        let mut kids = vec![Vec::new(); w.len()];
        for (i, x) in w.iter().enumerate() {
            if x.head > 0 && x.head <= kids.len() {
                kids[x.head - 1].push(i);
            }
        }
        Tree { w, kids }
    }
    pub fn len(&self) -> usize {
        self.w.len()
    }
    pub fn is_empty(&self) -> bool {
        self.w.is_empty()
    }
    pub fn head(&self, i: usize) -> Option<usize> {
        let h = self.w[i].head;
        (h > 0 && h <= self.w.len()).then(|| h - 1)
    }
    pub fn root(&self) -> Option<usize> {
        self.w.iter().position(|x| x.head == 0)
    }
    pub fn base(&self, i: usize) -> Rel {
        self.w[i].rel.base()
    }
    pub fn kids(&self, i: usize) -> &[usize] {
        &self.kids[i]
    }
    pub fn kids_of(&self, i: usize, r: Rel) -> Vec<usize> {
        self.kids[i].iter().copied().filter(|k| self.base(*k) == r).collect()
    }
    pub fn form(&self, i: usize) -> &str {
        &self.w[i].form
    }
    pub fn lemma(&self, i: usize) -> &str {
        &self.w[i].lemma
    }
    pub fn upos(&self, i: usize) -> UPos {
        self.w[i].upos
    }
    pub fn tag(&self, i: usize) -> PTag {
        self.w[i].tag
    }
    pub fn show(&self) -> String {
        self.w.iter().enumerate().map(|(i, x)| format!("{}:{}/{}/{}>{}", i + 1, x.form, x.upos, x.rel, x.head)).collect::<Vec<_>>().join(" ")
    }
}

/// The speech act a verb of speech performs (prag::schema): «ask» asks, «promise» promises, «tell» asserts.
pub fn act_of(verb: &str) -> Act {
    match verb {
        "ask" | "wonder" => Act::Ask,
        "beg" => Act::Request,
        "promise" => Act::Promise,
        "warn" => Act::Warn,
        "thank" => Act::Thank,
        "apologize" | "apologise" => Act::Apologize,
        "yell" => Act::Express,
        "greet" => Act::Greet,
        "agree" => Act::Agree,
        "correct" | "protest" | "argue" => Act::Disagree,
        "suggest" => Act::Suggest,
        "demand" | "plead" | "order" => Act::Request,
        "shout" | "cry" | "scream" | "exclaim" | "sing" => Act::Express,
        "say" | "tell" | "answer" | "reply" | "explain" | "add" | "remark" | "mutter" | "whisper" | "call" => Act::Assert,
        _ => Act::Other,
    }
}

/// Sentence split on . ! ? followed by space and an upper-case letter or a quote (as world's babi reader).
pub fn split_sentences(t: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let cs: Vec<char> = t.chars().collect();
    for i in 0..cs.len() {
        cur.push(cs[i]);
        if matches!(cs[i], '.' | '!' | '?') && cs.get(i + 1) == Some(&' ') && cs.get(i + 2).is_some_and(|c| c.is_uppercase() || matches!(c, '"' | '\u{201c}' | '\u{2018}')) {
            out.push(std::mem::take(&mut cur).trim().to_string());
        }
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

/// What a frame is: an action, a general statement (with its know.rs link), speech or thought, or a bare phrase
/// (an answer option without a predicate: «A frying-pan», «Mad»).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum Kind {
    Event,
    Fact(Link),
    /// speech or thought, with the speech act its verb performs (prag::schema::Act)
    Say(Act),
    Bare,
}

/// The roles a frame can fill — and the slots a question can ask about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum Slot {
    Verb,
    Agent,
    Patient,
    Recipient,
    Place,
    Attr,
    Obl,
}

pub const SLOTS: [Slot; 7] = [Slot::Verb, Slot::Agent, Slot::Patient, Slot::Recipient, Slot::Place, Slot::Attr, Slot::Obl];

/// An entity: its id is the head lemma («the Cheshire Cat» → cat); a pronoun keeps what it was resolved from.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Ent {
    pub id: String,
    /// the phrase as written (for showing)
    pub text: String,
    /// the pronoun it was resolved from («it», «she»)
    pub pron: Option<String>,
    pub plural: bool,
    /// a name: a proper noun without a determiner (Alice), not «the Cat»
    pub named: bool,
    /// adjectives and compounds of the phrase («long claws» → long)
    pub attrs: Vec<String>,
    /// the heads coordinated with it («a Hatter and a March Hare»)
    pub and: Vec<String>,
    /// the possessor of this mention («his nose», «Alice's cat»); kept with the mention, not compared by `same`
    pub poss: Option<Box<Ent>>,
    /// prepositional modifiers of the noun with their preposition («a cup of tea», «the man in the hat»)
    pub mods: Vec<(String, Ent)>,
    /// appositions («Bill, the lizard»)
    pub appos: Vec<Ent>,
    /// other one-word modifiers with their relation and tag («only the dog», «Peter Rabbit»'s Peter as a compound)
    pub words: Vec<(Rel, String, PTag)>,
    /// the coordinated phrases in full, as `and` names their heads («the big dog and the small grey cat»)
    pub conj: Vec<Ent>,
    /// the determiner of this mention with its tag («the», «a», «this», «no»); none said when empty
    pub det: Option<(String, PTag)>,
    /// the preposition of this mention («in» the wood, «to» him; «because of»)
    pub case: Option<String>,
    /// the conjunction before this mention when it is a conjunct («or» a cat)
    pub cc: Option<String>,
    /// a relative word that stands for this entity in its clause, with its tag («who», «which», «that»)
    pub rel_word: Option<(String, PTag)>,
}

impl Ent {
    pub fn show(&self) -> String {
        let mut s = self.id.clone();
        if !self.and.is_empty() {
            s += &format!("+{}", self.and.join("+"));
        }
        if let Some(p) = &self.pron {
            if p != &self.id {
                s += &format!("(«{p}»)");
            }
        }
        s
    }

    /// The same entity: the same head, or one of its coordinated heads.
    pub fn same(&self, o: &Ent) -> bool {
        !UNRESOLVED.contains(&self.id.as_str()) && !UNRESOLVED.contains(&o.id.as_str()) && (self.id == o.id || self.and.contains(&o.id) || o.and.contains(&self.id))
    }
}

/// Pronouns left unresolved: they match nothing («it» with no antecedent says nothing about which «it»).
pub(crate) const UNRESOLVED: [&str; 6] = ["it", "they", "he", "she", "this", "that"];
/// The same list, for readers outside the crate (the student's numbered frames).
pub const UNRESOLVED_PUB: [&str; 6] = UNRESOLVED;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Frame {
    pub kind: Kind,
    pub agent: Option<Ent>,
    /// the lemma, with its particle («put down»); «be» for a copula
    pub verb: String,
    pub neg: bool,
    pub patient: Option<Ent>,
    pub recipient: Option<Ent>,
    pub place: Option<Ent>,
    /// properties: a copula's adjective, an adjective complement («get tiny»), a manner adverb («slowly»)
    pub attrs: Vec<String>,
    /// the attributes that are manner adverbs in the text («ran fast»)
    pub manner: Vec<String>,
    /// other obliques with their preposition («with the Queen», «after her»)
    pub obls: Vec<(String, Ent)>,
    /// for Say (and thinking, asking): the frames of what was said — its patient
    pub quote: Vec<usize>,
    /// inside quotation marks
    pub quoted: bool,
    /// the frame that is its cause (or purpose)
    pub cause: Option<usize>,
    /// what happened next
    pub next: Option<usize>,
    /// a condition («if …»): not a fact of the story
    pub cond: bool,
    /// not said to have happened: the «to» complement of a wish, a plan or an attempt («wanted to eat»), set by the
    /// implication signatures (`signature`); an implicative that says it did not happen flips `neg` instead
    pub irrealis: bool,
    /// (lesson number, line index in the lesson), set by the reader
    pub src: (usize, usize),
    /// what assigned the roles: a VerbNet class and frame, a noun naming an event (WordNet), or the tree heuristics
    pub how: How,
    /// roles filled but not said in this clause: an agent shared with the clause above, an imperative's «you», the
    /// noun a relative or infinitive clause belongs to
    pub unsaid: Vec<Slot>,
    /// the clause's other one-word dependents with their relation and tag: adverbs that are no manner (then, also,
    /// there, again), discourse words (oh, well)
    pub words: Vec<(Rel, String, PTag)>,
    /// noun phrases on the clause in no role, with their relation: time and measure without a preposition («last
    /// week», «every morning»), vocatives, dislocated phrases
    pub nps: Vec<(Rel, Ent)>,
    /// the clause's markers with their tags («that», «if», «because», «to»)
    pub mark: Vec<(String, PTag)>,
    /// the conjunction that joins the clause to the one before («but», «or»)
    pub cc: Option<String>,
    /// where the clause hangs in the sentence: the frame above it, the clause's own relation there (conj, advcl,
    /// xcomp, ccomp, parataxis, acl:relcl …), and the noun it belongs to when that is not the predicate above (a
    /// relative clause: «the man who came»)
    pub link: Option<(usize, Rel, Option<String>)>,
    /// tense, aspect, voice and modality, read from the verb's auxiliaries and tags (`tam_of`)
    pub tam: Tam,
}

impl Frame {
    pub(crate) fn new(kind: Kind, verb: String) -> Frame {
        Frame { kind, agent: None, verb, neg: false, patient: None, recipient: None, place: None, attrs: Vec::new(), manner: Vec::new(), obls: Vec::new(), quote: Vec::new(), quoted: false, cause: None, next: None, cond: false, src: (0, 0), how: How::default(), unsaid: Vec::new(), words: Vec::new(), nps: Vec::new(), mark: Vec::new(), cc: None, link: None, tam: Tam::default(), irrealis: false }
    }

    /// A role filled from elsewhere, not said in the clause (PRAG_OFF=unsaid keeps no such mark).
    pub(crate) fn unsay(&mut self, s: Slot) {
        if !off("unsaid") && !self.unsaid.contains(&s) {
            self.unsaid.push(s);
        }
    }

    pub fn slot(&self, s: Slot) -> Option<&Ent> {
        match s {
            Slot::Agent => self.agent.as_ref(),
            Slot::Patient => self.patient.as_ref(),
            Slot::Recipient => self.recipient.as_ref(),
            Slot::Place => self.place.as_ref(),
            _ => None,
        }
    }

    pub fn slot_mut(&mut self, s: Slot) -> Option<&mut Option<Ent>> {
        match s {
            Slot::Agent => Some(&mut self.agent),
            Slot::Patient => Some(&mut self.patient),
            Slot::Recipient => Some(&mut self.recipient),
            Slot::Place => Some(&mut self.place),
            _ => None,
        }
    }

    /// The place is no place but where something ends up — what VerbNet calls a Goal outside the classes of putting,
    /// sending and motion, or a Result («sorted the garbage into paper», «turned into a frog»): that role's name.
    pub fn goal(&self) -> Option<&'static str> {
        let p = self.place.as_ref()?;
        let key = format!("{} {}", p.case.as_deref()?, p.id);
        self.how.roles.iter().find(|(k, _)| *k == key).and_then(|(_, r)| match r.as_str() {
            "Result" => Some("result"),
            "Goal" if !spatial_class(&self.how.class) => Some("goal"),
            _ => None,
        })
    }

    /// Every entity of the frame with its slot.
    pub fn ents(&self) -> Vec<(Slot, &Ent)> {
        let mut v: Vec<(Slot, &Ent)> = [Slot::Agent, Slot::Patient, Slot::Recipient, Slot::Place].iter().filter_map(|&s| self.slot(s).map(|e| (s, e))).collect();
        v.extend(self.obls.iter().map(|(_, e)| (Slot::Obl, e)));
        v
    }

    /// `show` with what assigned the roles (`  ⟨vn give-13.1 «NP V NP PP.recipient»: …⟩`), for traces; the tree
    /// heuristics alone add nothing.
    pub fn show_how(&self, base: usize) -> String {
        let s = self.show(base);
        if self.how.by == By::Heuristic { s } else { format!("{s}  ⟨{}⟩", self.how.show()) }
    }

    /// One line: `Event alice put down patient=baby(«it») place=wood | cause→2 next→4`; indices are shown with the
    /// given offset (the frames' place in the memory or in the line).
    pub fn show(&self, base: usize) -> String {
        let kind = match self.kind {
            Kind::Event => "Event".to_string(),
            Kind::Fact(l) => format!("Fact({})", l.name()),
            Kind::Say(a) => format!("Say({})", a.name()),
            Kind::Bare => "Bare".to_string(),
        };
        let mut s = format!("{kind} {}{}{}", self.agent.as_ref().map(Ent::show).unwrap_or_else(|| "_".into()), if self.neg { " NOT" } else { "" }, if self.verb.is_empty() { String::new() } else { format!(" {}", self.verb) });
        for (n, e) in [("patient", &self.patient), ("to", &self.recipient), ("place", &self.place)] {
            if let Some(e) = e {
                s += &format!(" {n}={}", e.show());
            }
        }
        for (p, e) in &self.obls {
            s += &format!(" {p}={}", e.show());
        }
        if !self.attrs.is_empty() {
            s += &format!(" attrs={}", self.attrs.join(","));
        }
        let mut links = Vec::new();
        if !self.quote.is_empty() {
            links.push(format!("says→{}", self.quote.iter().map(|q| format!("#{}", q + base)).collect::<Vec<_>>().join(",")));
        }
        if let Some(c) = self.cause {
            links.push(format!("cause→#{}", c + base));
        }
        if let Some(n) = self.next {
            links.push(format!("next→#{}", n + base));
        }
        if self.quoted {
            links.push("quoted".into());
        }
        if self.cond {
            links.push("if".into());
        }
        if self.irrealis {
            links.push("unreal".into());
        }
        if !links.is_empty() {
            s += &format!(" | {}", links.join(" "));
        }
        s
    }
}

/// The tense of a clause, from its finite word: none for a clause without one (an infinitive, a gerund, an
/// imperative) or a frame not read from a verb.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum Tense {
    #[default]
    None,
    Pres,
    Past,
    /// «will», «'ll», «wo(n't)»
    Fut,
}

/// The form of a clause's first verb word when it is not finite.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum VForm {
    #[default]
    Fin,
    /// a bare infinitive («let him go») or an imperative
    Inf,
    /// «to go»
    ToInf,
    /// «going», «having gone»
    Ger,
    /// «gone», «taken away»
    Part,
}

/// A modal auxiliary (future «will» is `Tense::Fut`, not a modal).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum Modal {
    Can,
    Could,
    May,
    Might,
    Must,
    Shall,
    Should,
    Would,
    Ought,
    Need,
    Dare,
}

impl Modal {
    pub const ALL: [Modal; 11] = [Modal::Can, Modal::Could, Modal::May, Modal::Might, Modal::Must, Modal::Shall, Modal::Should, Modal::Would, Modal::Ought, Modal::Need, Modal::Dare];

    pub fn lemma(self) -> &'static str {
        match self {
            Modal::Can => "can",
            Modal::Could => "could",
            Modal::May => "may",
            Modal::Might => "might",
            Modal::Must => "must",
            Modal::Shall => "shall",
            Modal::Should => "should",
            Modal::Would => "would",
            Modal::Ought => "ought",
            Modal::Need => "need",
            Modal::Dare => "dare",
        }
    }

    pub fn of(lemma: &str) -> Option<Modal> {
        let l = lemma.to_lowercase();
        Modal::ALL.into_iter().find(|m| m.lemma() == l)
    }
}

/// Tense, aspect, voice and modality of a frame (the verb's auxiliary chain), so that the clause can be said again
/// with the same verb group. `read` is false for a frame not read from a verb (a nominal event, a bare phrase) and
/// with the step switched off (`PRAG_OFF=tam`): its readers then fall back on the frame's kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct Tam {
    pub read: bool,
    pub tense: Tense,
    pub form: VForm,
    pub modal: Option<Modal>,
    /// «have» + participle
    pub perfect: bool,
    /// «be» + -ing
    pub progressive: bool,
    /// «be» (or «get») + participle with the patient as subject
    pub passive: bool,
    /// do-support («did not go», «does he know»)
    pub dosupport: bool,
    /// an imperative: the base form with no subject («Come here!»)
    pub imperative: bool,
}

impl Tam {
    /// A short tag for traces: `past.perf.prog.pass`, `will`, `can`, `imp`, `to-inf`.
    pub fn show(&self) -> String {
        let mut v: Vec<&str> = Vec::new();
        if let Some(m) = self.modal {
            v.push(m.lemma());
        }
        match self.tense {
            Tense::Pres => v.push("pres"),
            Tense::Past => v.push("past"),
            Tense::Fut => v.push("fut"),
            Tense::None => {}
        }
        match self.form {
            VForm::Inf if self.imperative => v.push("imp"),
            VForm::Inf => v.push("inf"),
            VForm::ToInf => v.push("to-inf"),
            VForm::Ger => v.push("ger"),
            VForm::Part => v.push("part"),
            VForm::Fin => {}
        }
        for (b, n) in [(self.perfect, "perf"), (self.progressive, "prog"), (self.passive, "pass"), (self.dosupport, "do")] {
            if b {
                v.push(n);
            }
        }
        v.join(".")
    }
}

/// Where a frame's roles came from (`crate::vn`): shown by `world frames` and `trace`, an atom of the learned rules.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum By {
    /// the reader's tree heuristics alone (no VerbNet class, or no frame matched)
    #[default]
    Heuristic,
    /// a VerbNet class and frame
    VerbNet,
    /// a noun naming the event of a verb (WordNet's derivational links): «the arrival of the Queen»
    Nominal,
}

impl By {
    pub fn name(self) -> &'static str {
        match self {
            By::Heuristic => "heuristic",
            By::VerbNet => "vn",
            By::Nominal => "nom",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct How {
    pub by: By,
    /// the VerbNet class («give-13.1-1») or the noun («arrival»)
    pub class: String,
    /// the VerbNet frame's pattern («NP V NP PP.recipient»)
    pub frame: String,
    /// the lexicon's role of each argument: `agent=Agent patient=Theme to=Recipient with=Instrument`
    pub roles: Vec<(String, String)>,
    /// the clause's syntactic frame as the VerbNet matcher saw it, without its fillers (`vn::Clause::sig`)
    pub sig: String,
}

impl How {
    pub fn show(&self) -> String {
        let mut s = self.by.name().to_string();
        if !self.class.is_empty() {
            s += &format!(" {}", self.class);
        }
        if !self.frame.is_empty() {
            s += &format!(" «{}»", self.frame);
        }
        if !self.roles.is_empty() {
            s += &format!(": {}", self.roles.iter().map(|(a, b)| format!("{a}={b}")).collect::<Vec<_>>().join(" "));
        }
        s
    }
}

/// What a lexicon role means for the frame's slots.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RoleTo {
    Agent,
    Patient,
    Recipient,
    Place,
    Obl,
}

/// VerbNet's thematic roles → the frame's slots (the brief: Agent/Experiencer/Causer → agent; Theme/Patient/Topic/
/// Stimulus → patient; Recipient/Beneficiary/Goal of a person → recipient; Destination/Location/Source → place).
fn role_to(role: &str, animate: Option<bool>) -> RoleTo {
    match role {
        "Agent" | "Experiencer" | "Causer" | "Pivot" | "Affector" => RoleTo::Agent,
        "Theme" | "Patient" | "Topic" | "Stimulus" | "Product" | "Eventuality" => RoleTo::Patient,
        "Recipient" | "Maleficiary" => RoleTo::Recipient,
        "Beneficiary" => RoleTo::Recipient,
        "Goal" | "Destination" if animate == Some(true) => RoleTo::Recipient,
        "Destination" | "Location" | "Source" | "Initial_Location" | "Trajectory" | "Goal" | "Path" => RoleTo::Place,
        _ => RoleTo::Obl,
    }
}

/// A VerbNet class of putting, removing, sending and carrying (9–11), or of existence, appearance and motion (47–51),
/// or «attend» (107.4): where a Goal is a place.
fn spatial_class(id: &str) -> bool {
    let Some(d) = id.find(|c: char| c.is_ascii_digit()) else { return false };
    let num = &id[d..];
    let major: u32 = num.split(['.', '-']).next().and_then(|x| x.parse().ok()).unwrap_or(0);
    matches!(major, 9..=11 | 47..=51) || num.starts_with("107.4")
}

/// Roles of a prepositional phrase that make it no place even when its preposition is spatial («look at the cat»,
/// «sting with tentacles»); the rest (Asset, Value, Extent …) are too often an adjunct matched by chance.
const NOT_PLACE_ROLES: [&str; 13] = ["theme", "patient", "topic", "stimulus", "instrument", "co-agent", "co-theme", "co-patient", "experiencer", "agent", "ppt", "pag", "attribute"];

// ---- the discourse: who «it» and «she» are ------------------------------------------------------------------------

/// The entities met so far, with salience (raised by every mention by its role, halved at every new sentence).
/// Where a frame sits in its sentence's tree (`Ctx::anchors`): its predicate token, the predicate of the frame it
/// hangs on with the clause's relation, and the head token of each role it says in this clause.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Anchor {
    pub pred: Option<usize>,
    pub link: Option<(usize, Rel)>,
    pub roles: Vec<(Slot, usize)>,
}

#[derive(Clone, Debug, Default)]
pub struct Ctx {
    /// the anchors of the last sentence's frames, one per frame in their order (PRAG_OFF=anchors: none kept). Only
    /// recorded: no step of the reader reads them, so the frames are the same with or without them.
    pub anchors: Vec<Anchor>,
    /// the entities read in the sentence being read: head token, id, phrase (for the anchors)
    ent_log: Vec<(usize, String, String)>,
    seen: BTreeMap<String, Seen>,
    time: usize,
    /// the sentences read so far (for the coreference sieve over the whole discourse)
    sents: Vec<Vec<Word>>,
    /// who spoke last (a quote with no verb of saying is theirs)
    last_speaker: Option<Ent>,
}

#[derive(Clone, Debug)]
struct Seen {
    ent: Ent,
    salience: u32,
    last: usize,
    /// the role of its last mention: 3 subject, 2 object, 1 other
    role: u8,
    /// the weight it got in the sentence being read (mentions in one sentence do not stack: the best role counts)
    this_sent: u32,
    /// it did something (an agent of an action): may be «he» or «she»
    animate: bool,
    /// it spoke or thought, or was called «he» or «she»: a person, never «it»
    person: bool,
    /// called «it» before: not «he» or «she»
    thing: bool,
    /// 1 masculine, 2 feminine (a gendered noun, or called «he» / «she» before)
    gender: u8,
}

/// Nouns whose gender is in the word.
pub(crate) const FEM: [&str; 30] = ["woman", "girl", "lady", "mother", "mum", "mom", "sister", "daughter", "aunt", "queen", "duchess", "princess", "wife", "grandmother", "granny", "grandma", "niece", "madam", "ma'am", "miss", "mrs", "nun", "witch", "bride", "hen", "cow", "mare", "goddess", "empress", "waitress"];
pub(crate) const MASC: [&str; 30] = ["man", "boy", "gentleman", "father", "dad", "brother", "son", "uncle", "king", "duke", "prince", "husband", "grandfather", "grandpa", "grandad", "nephew", "sir", "mr", "monk", "wizard", "groom", "bull", "stallion", "god", "emperor", "lord", "knight", "waiter", "fellow", "cock"];

/// Animacy of a noun from the level-1 categories (global, as world's sense.rs): Some(true) animate, Some(false) not.
/// The noun lexicon learned from Mova's own parses (`coref::lexicon`): animate or not, and 1 «he» / 2 «she» / 0, for
/// a common noun the closed lists say nothing of. Names are never looked up. Opt-in (PRAG_VN=gender-table): measured
/// 2026-11-04 the school did not gain (24 courses 12 306 → 12 305, books 7 339 → 7 339 with odd −17 / even +17, exam
/// 2 195 → 2 179), while the frame bench's dev pronouns rose (coref F1 0.471 → 0.489).
pub fn lex_table(e: &Ent) -> Option<(Option<bool>, u8)> {
    if e.named || !vn_opt("gender-table") {
        return None;
    }
    let f = coref::lexicon::noun_feats(&e.id)?;
    use coref::mention::{ANIM, FEM, INAN, MASC};
    let anim = if f.animacy == ANIM { Some(true) } else if f.animacy == INAN { Some(false) } else { None };
    let g = if f.gender == MASC { 1 } else if f.gender == FEM { 2 } else { 0 };
    Some((anim, g))
}

pub fn lex_animate(lemma: &str) -> Option<bool> {
    let cats = global::categories_of(lemma);
    let in_list = |l: &str, c: &&str| global::selection(l).contains(c);
    let an = !global::selection("not_animate").contains(&lemma) && cats.iter().any(|c| in_list("animate_cats", c));
    let inan = !global::selection("not_inanimate").contains(&lemma) && cats.iter().any(|c| in_list("inanimate_cats", c));
    match (an, inan) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

impl Ctx {
    /// The temporary world in a few words (for the teacher's prompt: summaries of the pages, not the pages): the `n` most salient entities with what is known of them, and who spoke last.
    pub fn summary(&self, n: usize) -> String {
        let mut seen: Vec<&Seen> = self.seen.values().filter(|s| s.salience > 0).collect();
        seen.sort_by(|a, b| b.salience.cmp(&a.salience).then(b.last.cmp(&a.last)));
        let ents: Vec<String> = seen
            .iter()
            .take(n)
            .map(|s| {
                let mut x = s.ent.show();
                let kind = if s.person { "person" } else if s.animate { "animate" } else if s.thing { "thing" } else { "" };
                let g = match s.gender {
                    1 => "he",
                    2 => "she",
                    _ => "",
                };
                let tags: Vec<&str> = [kind, g].into_iter().filter(|t| !t.is_empty()).collect();
                if !tags.is_empty() {
                    x += &format!(" [{}]", tags.join(", "));
                }
                x
            })
            .collect();
        let mut out = format!("entities: {}", if ents.is_empty() { "none yet".to_string() } else { ents.join("; ") });
        if let Some(sp) = &self.last_speaker {
            out += &format!(" | last speaker: {}", sp.show());
        }
        out
    }

    pub fn new_line(&mut self) {
        for s in self.seen.values_mut() {
            s.salience /= 2;
            s.this_sent = 0;
        }
    }

    /// A mention of `e` in a role (3 subject, 2 object, 1 other).
    pub fn mention_as(&mut self, e: &Ent, animate: bool, role: u8) {
        if UNRESOLVED.contains(&e.id.as_str()) || PERSONAL.contains(&e.id.as_str()) {
            return;
        }
        self.time += 1;
        let t = self.time;
        let g = if FEM.contains(&e.id.as_str()) { 2 } else if MASC.contains(&e.id.as_str()) { 1 } else { 0 };
        let s = self.seen.entry(e.id.clone()).or_insert(Seen { ent: Ent { pron: None, poss: None, mods: Vec::new(), appos: Vec::new(), words: Vec::new(), conj: Vec::new(), det: None, case: None, cc: None, rel_word: None, ..e.clone() }, salience: 0, last: 0, role: 1, this_sent: 0, animate: false, person: false, thing: false, gender: g });
        if coref_mode() >= 3 {
            let w = 4 + 2 * role as u32;
            if w > s.this_sent {
                s.salience += w - s.this_sent;
                s.this_sent = w;
            }
        } else {
            s.salience += 2 * role as u32;
        }
        s.last = t;
        s.role = role;
        s.animate |= animate || e.named;
        match e.pron.as_deref() {
            Some("he" | "him" | "his" | "himself") => {
                s.person = true;
                s.gender = if s.gender == 0 { 1 } else { s.gender };
            }
            Some("she" | "her" | "hers" | "herself") => {
                s.person = true;
                s.gender = if s.gender == 0 { 2 } else { s.gender };
            }
            Some("it" | "its" | "itself") => s.thing = true,
            _ => {}
        }
        s.person |= e.named;
    }

    pub fn mention(&mut self, e: &Ent, animate: bool) {
        self.mention_as(e, animate, 1)
    }

    /// An entity spoke or thought: a person.
    pub fn person(&mut self, id: &str) {
        if let Some(s) = self.seen.get_mut(id) {
            s.person = true;
            s.animate = true;
        }
    }

    /// The most salient entity fitting the pronoun (ties: the most recent).
    pub fn resolve(&self, pron: &str) -> Option<Ent> {
        self.resolve_not(pron, None)
    }

    /// `resolve`, but never to `not` (a plain pronoun object is not its own clause's subject: «the cook throws a
    /// pan after her» — her is not the cook; reflexives are).
    pub fn resolve_not(&self, pron: &str, not: Option<&str>) -> Option<Ent> {
        self.resolve_in(pron, not, 1)
    }

    /// The resolver: agreement (number; he/she → a person or an animate being of the right gender, never a thing
    /// called «it»; it → never a person), then salience by the roles of the mentions, a bonus for the same role as the
    /// pronoun's (parallelism), ties to the most recent. Numbers and times are never antecedents.
    pub fn resolve_in(&self, pron: &str, not: Option<&str>, role: u8) -> Option<Ent> {
        if coref_mode() == 0 {
            return self.resolve_old(pron, not);
        }
        let fits = |s: &Seen| {
            let id = s.ent.id.as_str();
            if id.chars().next().is_some_and(|c| c.is_ascii_digit()) || time_ent(&s.ent) {
                return false;
            }
            let plural = s.ent.plural || !s.ent.and.is_empty();
            match pron {
                "he" | "him" | "his" | "himself" | "she" | "her" | "hers" | "herself" => {
                    let want = if pron.starts_with('h') && pron != "her" && pron != "hers" && pron != "herself" { 1 } else { 2 };
                    let tab = lex_table(&s.ent);
                    let lex = lex_animate(id).or(tab.and_then(|t| t.0));
                    let gender = if s.gender == 0 { tab.map_or(0, |t| t.1) } else { s.gender };
                    !plural && !s.thing && (gender == 0 || gender == want) && (s.person || s.ent.named || lex == Some(true) || s.animate && lex != Some(false))
                }
                "it" | "its" | "itself" => !plural && !s.ent.named && !s.person && s.gender == 0 && lex_table(&s.ent).is_none_or(|t| t.0 != Some(true)),
                "they" | "them" | "their" | "theirs" | "themselves" => plural,
                _ => false,
            }
        };
        let score = |s: &Seen| s.salience + if s.role == role { 2 } else { 0 };
        if std::env::var("PRAG_DEBUG").is_ok() {
            eprintln!("resolve {pron} (role {role}, not {not:?}): {}", self.seen.values().filter(|s| fits(s)).map(|s| format!("{}={}", s.ent.id, score(s))).collect::<Vec<_>>().join(" "));
        }
        self.seen.values().filter(|s| fits(s) && not.is_none_or(|n| n != s.ent.id)).max_by(|a, b| score(a).cmp(&score(b)).then(a.last.cmp(&b.last))).map(|s| s.ent.clone())
    }

    fn resolve_old(&self, pron: &str, not: Option<&str>) -> Option<Ent> {
        let fits = |s: &Seen| match pron {
            "he" | "him" | "his" | "himself" | "she" | "her" | "hers" | "herself" => s.animate && !s.ent.plural,
            "it" | "its" | "itself" => !s.ent.plural && !s.ent.named && !s.person,
            "they" | "them" | "their" | "theirs" | "themselves" => s.ent.plural || !s.ent.and.is_empty(),
            _ => false,
        };
        self.seen.values().filter(|s| fits(s) && not.is_none_or(|n| n != s.ent.id)).max_by(|a, b| a.salience.cmp(&b.salience).then(a.last.cmp(&b.last))).map(|s| s.ent.clone())
    }
}

/// The pronoun as the resolver reads it (its own case form: «her» stays «her», which is feminine).
fn lemma_pron(l: &str) -> String {
    l.to_lowercase()
}

/// The speaker and the listener: kept as they are («you» in the everyday lessons is everyone).
pub(crate) const PERSONAL: [&str; 6] = ["i", "you", "we", "me", "us", "one"];

const SAY: [&str; 21] = ["say", "tell", "ask", "answer", "reply", "shout", "cry", "call", "whisper", "think", "wonder", "explain", "promise", "sing", "scream", "exclaim", "add", "remark", "mutter", "beg", "warn"];

/// More verbs of speech, with their act (`act_of`).
const SAY_MORE: [&str; 22] = ["greet", "correct", "admit", "suggest", "insist", "complain", "announce", "declare", "protest", "demand", "claim", "argue", "agree", "apologise", "apologize", "thank", "respond", "murmur", "yell", "plead", "boast", "grumble"];

/// A verb of speech or thought.
pub(crate) fn is_say(l: &str) -> bool {
    SAY.contains(&l) || !off("say-more") && SAY_MORE.contains(&l) || !off("vn-say") && crate::vn::speech_verb(l)
}

/// Verbs of speech whose person object is the one spoken to («tell her», «ask the Cat»).
const TELL: [&str; 12] = ["tell", "ask", "answer", "warn", "remind", "thank", "greet", "correct", "promise", "beg", "order", "teach"];

/// Verbs whose clause complement is their content (what is known, feared, seen), like speech.
pub(crate) const CONTENT: [&str; 22] = ["know", "see", "hear", "fear", "hope", "believe", "wish", "notice", "realise", "realize", "decide", "find", "learn", "guess", "suppose", "remember", "forget", "show", "mean", "agree", "understand", "dream"];

const PARTICLES: [&str; 8] = ["down", "up", "off", "out", "away", "back", "over", "around"];

/// Words that can be the particle of a phrasal verb (checked against the expression base).
const PHRASAL: [&str; 8] = ["off", "out", "up", "down", "away", "back", "forward", "apart"];

/// Verbs whose object is the doer of their infinitive or bare-verb complement.
pub(crate) const OBJ_CONTROL: [&str; 34] = ["make", "let", "help", "have", "see", "watch", "hear", "feel", "ask", "tell", "want", "allow", "force", "teach", "get", "cause", "order", "persuade", "encourage", "expect", "keep", "leave", "find", "need", "beg", "invite", "remind", "warn", "advise", "send", "lead", "enable", "train", "notice"];

/// Prepositions that are particles when they stand alone after the verb («pass on», «hop about», «go round»).
const PHRASAL_BARE: [&str; 12] = ["on", "in", "about", "round", "around", "over", "through", "along", "together", "behind", "by", "past"];

const WH: [&str; 9] = ["who", "whom", "whose", "what", "which", "where", "when", "why", "how"];

/// Adverbs that are not a manner: they say when, how much or link clauses.
const NOT_MANNER: [&str; 30] = ["not", "never", "so", "then", "also", "too", "very", "just", "only", "even", "quite", "there", "here", "again", "still", "now", "when", "how", "why", "where", "back", "rather", "much", "more", "most", "as", "ever", "already", "yet", "really"];

pub(crate) const PLACE_PREPS: [&str; 18] = ["in", "on", "at", "under", "near", "inside", "behind", "above", "below", "across", "through", "over", "onto", "into", "from", "beside", "around", "out"];

/// An entity that can be a place: not a person or an animal (pronouns of persons, names met as persons, animate
/// nouns), not a time or a measure («at the end», «in a second», «on the way» stay prepositional roles).
fn placeable(e: &Ent) -> bool {
    if off("place-filter") {
        return true;
    }
    let id = e.id.as_str();
    !(e.pron.as_deref().is_some_and(|p| !matches!(p, "it" | "its" | "there" | "here")) && !matches!(id, "it")
        || lex_animate(id) == Some(true)
        || FEM.contains(&id)
        || MASC.contains(&id)
        || PERSONAL.contains(&id)
        || global::selection("measure_time").contains(&id) && !matches!(id, "way")
        || time_ent(e))
}

/// A noun phrase that can be spoken to: a personal pronoun, a name, an animate noun.
fn person_like(t: &Tree, k: usize) -> bool {
    match t.upos(k) {
        UPos::PRON => matches!(t.lemma(k), "he" | "she" | "him" | "her" | "they" | "them" | "i" | "me" | "you" | "we" | "us" | "it"),
        UPos::PROPN => true,
        UPos::NOUN => lex_animate(t.lemma(k)) == Some(true) || FEM.contains(&t.lemma(k)) || MASC.contains(&t.lemma(k)),
        _ => false,
    }
}

/// Verbs whose subject «it» is a dummy.
const DUMMY_IT: [&str; 8] = ["depend", "seem", "matter", "rain", "snow", "appear", "take", "happen"];

/// Verbs whose subject «it» stands for a clause when they have a clause of their own («it means I'm in trouble»).
const CLAUSE_IT: [&str; 6] = ["mean", "look", "sound", "feel", "turn", "show"];

/// Words of time: when something happened, not a role filler («in a day or two», «today», «the last time»).
const TIME: [&str; 22] = ["day", "time", "today", "tonight", "tomorrow", "yesterday", "morning", "evening", "afternoon", "night", "minute", "hour", "second", "week", "month", "year", "moment", "while", "spring", "summer", "autumn", "winter"];

/// More nouns of time than `TIME`: spans and parts of the day (PRAG_OFF=time-more: the reader keeps to `TIME`).
const TIME_MORE: [&str; 11] = ["season", "weekend", "fortnight", "decade", "century", "midnight", "noon", "dawn", "dusk", "sunrise", "sunset"];
/// The days and the months: times when they are names («on Tuesdays», «in May»), not «their march».
const TIME_NAMES: [&str; 19] = ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday", "january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december"];

/// The event of a gerund noun the nominal table does not list («the opening of the stadium», «the building of the
/// bridge»): the verb whose -ing form it is (`en::dict`), only with an «of» or «by» argument; it takes an object when
/// one of the verb's VerbNet classes has a frame «NP V NP» (PRAG_OFF=vn-gerund).
fn gerund_nom(t: &Tree, n: usize) -> Option<crate::vn::Nom> {
    if off("vn-gerund") {
        return None;
    }
    let form = t.form(n).to_lowercase();
    if form.len() < 6 || !form.ends_with("ing") {
        return None;
    }
    let with_arg = t.kids(n).iter().any(|&k| t.base(k) == Rel::Nmod && t.kids_of(k, Rel::Case).first().is_some_and(|&c| matches!(t.lemma(c).to_lowercase().as_str(), "of" | "by")));
    if !with_arg {
        return None;
    }
    let f = en::dict::form(&form)?;
    let verb = en::dict::analyses(f).find(|r| r.tag == PTag::VBG).map(|r| en::dict::text(r.lemma()).to_lowercase()).filter(|v| !v.is_empty() && !matches!(v.as_str(), "be" | "have" | "do" | "get" | "go"))?;
    let lx = crate::vn::lex();
    let cs = lx.classes_of(&verb);
    if cs.is_empty() {
        return None;
    }
    let transitive = cs.iter().any(|&c| lx.chain(c).iter().any(|&k| lx.classes[k].frames.iter().any(|fr| fr.primary.starts_with("NP V NP"))));
    Some(crate::vn::Nom { noun: form, verb, file: "gerund".into(), transitive })
}

/// A word of time (lemma, lower case; `proper`: a name, a day or a month counts only as one): when, not where or what.
pub fn time_word(lemma: &str, proper: bool) -> bool {
    TIME.contains(&lemma) || TIME_MORE.contains(&lemma) || proper && TIME_NAMES.contains(&lemma)
}

/// The reader's test of a token of time: a noun of `TIME`, and with `time-more` on the other nouns and names of time.
fn time_tok(t: &Tree, k: usize) -> bool {
    let l = t.lemma(k);
    match t.upos(k) {
        UPos::NOUN => TIME.contains(&l) || !off("time-more") && TIME_MORE.contains(&l),
        UPos::PROPN => !off("time-more") && TIME_NAMES.contains(&l),
        _ => false,
    }
}

/// The months and the seasons that a time value is made of («March 2022», «Fall 2019»).
const TIME_MONTHS: [&str; 17] = ["january", "february", "march", "april", "may", "june", "july", "august", "september", "october", "november", "december", "spring", "summer", "autumn", "fall", "winter"];
/// Prepositions that put an event noun as a time («during the war», «after the merger»).
const TIME_PREPS: [&str; 6] = ["during", "after", "before", "since", "until", "throughout"];

/// The time value of the phrase headed by `k` (PRAG_OFF=wh-time): its months, seasons, years (three or four digits up to
/// 2999, or a decade such as «1990s»), days (one or two digits beside a month) and era marks, lower-cased in word
/// order: «on November 29, 2023» → «november 29 2023», «in 1999» → «1999». None without a month, a season or a year:
/// «on page 29» has no time. A month or a season counts only as a name or a bare noun («in May», «in the fall of
/// 2019»), not «their march». The value of the year alone comes second when the value holds more.
fn time_value(t: &Tree, k: usize) -> Vec<String> {
    let mut sub = vec![k];
    let mut i = 0;
    while i < sub.len() {
        sub.extend(t.kids(sub[i]).iter().copied().filter(|&j| !matches!(t.base(j), Rel::Case | Rel::Det | Rel::Punct | Rel::Cc | Rel::Acl | Rel::Conj)));
        i += 1;
    }
    sub.sort_unstable();
    let low: Vec<String> = sub.iter().map(|&j| t.form(j).to_lowercase()).collect();
    let digits = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit());
    let year = |s: &str| digits(s) && (3..=4).contains(&s.len()) && s.parse::<u32>().is_ok_and(|y| y <= 2999) || s.len() == 5 && s.ends_with('s') && digits(&s[..4]);
    let month = |j: usize, s: &str| TIME_MONTHS.contains(&s) && (matches!(t.upos(j), UPos::PROPN) || matches!(s, "spring" | "summer" | "autumn" | "fall" | "winter"));
    let has_month = sub.iter().zip(&low).any(|(&j, s)| month(j, s));
    let mut v: Vec<&str> = Vec::new();
    let mut y: Option<&str> = None;
    for (&j, s) in sub.iter().zip(&low) {
        let s = s.as_str();
        if month(j, s) {
            v.push(s);
        } else if year(s) && (t.upos(j) == UPos::NUM || s.ends_with('s')) {
            v.push(s);
            y = y.or(Some(s));
        } else if has_month && digits(s) && s.len() <= 2 && s.parse::<u32>().is_ok_and(|d| (1..=31).contains(&d)) {
            v.push(s);
        } else if !v.is_empty() && matches!(s, "bc" | "bce" | "ad" | "ce") {
            v.push(s);
        }
    }
    if !has_month && y.is_none() {
        return Vec::new();
    }
    let all = v.join(" ");
    let mut out = vec![all.clone()];
    if let Some(y) = y.filter(|y| *y != all) {
        out.push(y.to_string());
    }
    out
}

impl Ent {
    /// The names this entity carries besides its head (the reader's name role, used by «which» questions): its noun
    /// compounds («the Lansdale/Doylestown Line» when the parser makes it a noun), its proper-noun compound and flat
    /// words, its appositions' heads («Bill, the lizard») and its «of» modifiers that are names («the city of Paris»).
    pub fn names(&self) -> Vec<&str> {
        let mut v: Vec<&str> = self.attrs.iter().map(String::as_str).collect();
        v.extend(self.words.iter().filter(|(r, _, _)| matches!(r.base(), Rel::Compound | Rel::Flat)).map(|(_, w, _)| w.as_str()));
        v.extend(self.appos.iter().map(|a| a.id.as_str()));
        v.extend(self.mods.iter().filter(|(c, m)| c == "of" && m.named).map(|(_, m)| m.id.as_str()));
        v
    }
}

/// The same test on an entity (a name for the days and months).
fn time_ent(e: &Ent) -> bool {
    let id = e.id.as_str();
    TIME.contains(&id) || !off("time-more") && (TIME_MORE.contains(&id) || e.named && TIME_NAMES.contains(&id))
}

const PURPOSE_HEADS: [&str; 8] = ["go", "come", "run", "hurry", "rush", "leave", "stop", "use"];

/// Verbs whose «to …» complement is what they aim at or feel, not a purpose (subject control and raising).
pub(crate) const CONTROL: [&str; 40] = ["want", "try", "begin", "start", "like", "love", "hate", "prefer", "need", "have", "seem", "appear", "decide", "hope", "plan", "learn", "forget", "remember", "agree", "refuse", "promise", "manage", "fail", "continue", "wish", "expect", "intend", "mean", "offer", "pretend", "tend", "used", "ought", "be", "get", "dare", "attempt", "afford", "choose", "deserve"];

pub(crate) fn is_quote_mark(f: &str) -> bool {
    matches!(f, "\"" | "“" | "”" | "«" | "»" | "``" | "''")
}

/// A predicate: a verb, a word with a copula, a noun the parser took for a verb (it has a subject), or a word
/// coordinated with a copula predicate («is old and grey»).
pub(crate) fn is_pred(t: &Tree, i: usize) -> bool {
    let has_cop = |k: usize| !t.kids_of(k, Rel::Cop).is_empty();
    match t.upos(i) {
        UPos::VERB => !hyphen_adj(t, i),
        UPos::ADJ | UPos::NOUN | UPos::PROPN | UPos::ADV | UPos::NUM | UPos::PRON => {
            has_cop(i)
                || (matches!(t.upos(i), UPos::NOUN) && t.w[i].rel != Rel::Root && t.kids(i).iter().any(|&k| t.w[k].rel == Rel::Nsubj) && t.kids_of(i, Rel::Case).is_empty())
                || (t.base(i) == Rel::Conj && matches!(t.upos(i), UPos::ADJ) && t.head(i).is_some_and(has_cop))
        }
        _ => false,
    }
}

/// The second part of a hyphenated compound adjective the parser tagged as a verb modifier («a short-eared owl»:
/// short - eared/VERB/amod; «a fast-moving river»): a property of the noun, not a predicate of its own, so no frame
/// (PRAG_OFF=hyphen-adj). The tree keeps it; the noun keeps it among its attributes.
fn hyphen_adj(t: &Tree, i: usize) -> bool {
    i >= 2 && t.base(i) == Rel::Amod && matches!(t.form(i - 1), "-" | "‐" | "‑") && t.base(i - 1) == Rel::Punct && !off("hyphen-adj")
}

/// A prepositional phrase the parser coordinated with a noun that has no preposition («8 eyes, usually in two or
/// three rows»: rows/conj>eyes with its own «in»): no coordinated entity, a phrase of the verb, as the parser's
/// misplaced modifiers of the object are (PRAG_OFF=conj-pp).
fn pp_conj(t: &Tree, c: usize) -> bool {
    t.base(c) == Rel::Conj && !t.kids_of(c, Rel::Case).is_empty() && t.head(c).is_some_and(|h| t.kids_of(h, Rel::Case).is_empty()) && !off("conj-pp")
}

/// Words of measure and sort that stand for what they measure («lots of seawater» is seawater).
const QUANT_OF: [&str; 22] = ["lot", "lots", "plenty", "bit", "deal", "load", "loads", "ton", "tons", "bunch", "handful", "number", "amount", "group", "kind", "sort", "type", "piece", "variety", "majority", "flock", "herd"];

fn negated(t: &Tree, v: usize) -> bool {
    t.kids(v).iter().any(|&k| matches!(t.lemma(k), "not" | "n't" | "never" | "no") && matches!(t.base(k), Rel::Advmod | Rel::Aux))
        || t.kids(v).iter().any(|&k| t.base(k) == Rel::Aux && (t.form(k).ends_with("n't") || t.form(k) == "cannot"))
}

/// Tense, aspect, voice and modality of predicate `p` from its auxiliaries, its copula and the tags: the first of
/// them in the text is the finite word (its tag gives the tense, «will» the future, a modal the modality); «have»
/// before a participle is the perfect, «be» before an -ing form the progressive, «be» or «get» as aux:pass (or «be»
/// before a participle) the passive, «do» do-support. Without an auxiliary the verb's own tag gives the tense, or
/// the form of a clause that has none: «to» before a base form, an -ing form, a participle; a base form at the root
/// with no subject is an imperative.
pub fn tam_of(t: &Tree, p: usize) -> Tam {
    let mut m = Tam { read: true, ..Tam::default() };
    let mut chain: Vec<usize> = t.kids(p).iter().copied().filter(|&k| matches!(t.base(k), Rel::Aux | Rel::Cop) && !matches!(t.lemma(k), "not" | "n't")).collect();
    chain.sort();
    let ptag = t.tag(p);
    for &k in &chain {
        let l = t.lemma(k).to_lowercase();
        let rel = t.w[k].rel;
        if rel == Rel::Cop {
            continue;
        }
        if matches!(l.as_str(), "will" | "'ll" | "wo") {
            m.tense = Tense::Fut;
        } else if let Some(md) = Modal::of(&l).filter(|_| t.tag(k) == PTag::MD || !matches!(l.as_str(), "need" | "dare")) {
            m.modal = Some(md);
        } else if l == "do" {
            m.dosupport = true;
        } else if rel == Rel::AuxPass {
            m.passive = true;
        } else if l == "have" {
            // the perfect: the next verb word is a participle
            let next = chain.iter().copied().find(|&j| j > k).unwrap_or(p);
            if t.tag(next) == PTag::VBN || next == p && t.kids_of(p, Rel::Cop).is_empty() && ptag == PTag::VBN {
                m.perfect = true;
            }
        } else if l == "be" {
            let next = chain.iter().copied().find(|&j| j > k).unwrap_or(p);
            match t.tag(next) {
                PTag::VBG => m.progressive = true,
                PTag::VBN if next == p => m.passive = true,
                _ => {}
            }
        }
    }
    // the copula's own aspect: «is being», «has been»
    if let Some(&c) = chain.iter().find(|&&k| t.w[k].rel == Rel::Cop) {
        if t.tag(c) == PTag::VBG && chain.iter().any(|&k| k < c && t.lemma(k).eq_ignore_ascii_case("be")) {
            m.progressive = true;
        }
    }
    if t.kids(p).iter().any(|&k| t.w[k].rel == Rel::NsubjPass) {
        m.passive = true;
    }
    // the finite word: the first of the chain, else the verb itself
    let first = chain.first().copied().unwrap_or(p);
    let ftag = t.tag(first);
    match ftag {
        PTag::VBD => m.tense = Tense::Past,
        PTag::VBZ | PTag::VBP => m.tense = Tense::Pres,
        PTag::MD => {}
        PTag::VBG => m.form = VForm::Ger,
        PTag::VBN => m.form = VForm::Part,
        PTag::VB => {
            let to = t.kids(p).iter().any(|&k| t.base(k) == Rel::Mark && t.lemma(k).eq_ignore_ascii_case("to"));
            m.form = if to { VForm::ToInf } else { VForm::Inf };
            let subj = t.kids(p).iter().any(|&k| matches!(t.base(k), Rel::Nsubj | Rel::Expl));
            if !to && !subj && t.w[p].rel == Rel::Root && (first == p || imperative_do(t, p)) {
                m.imperative = true;
            }
        }
        _ => {}
    }
    m
}

/// A relative word on the clause of `p` or inside one of its phrases («who», «which», «that» as a pronoun, «whose»,
/// «where», «when»): it stands for the noun the clause belongs to.
fn relative_word(t: &Tree, p: usize) -> bool {
    let rel = |k: usize| matches!(t.tag(k), PTag::WDT | PTag::WP | PTag::WPS | PTag::WRB) && k < p;
    t.kids(p).iter().any(|&k| rel(k) || t.kids(k).iter().any(|&j| rel(j)))
}

/// An imperative whose verb group starts with «do» or the copula: «Don't throw it away», «Do come in», «Be quiet»,
/// «Don't be silly» — the base form first, every auxiliary that «do», no subject, marker or expletive
/// (PRAG_OFF=imp-do; the plain «Come here!» is read without it).
fn imperative_do(t: &Tree, p: usize) -> bool {
    if off("imp-do") || t.kids(p).iter().any(|&k| matches!(t.base(k), Rel::Nsubj | Rel::Mark | Rel::Expl)) {
        return false;
    }
    let do_aux = |k: usize| t.base(k) == Rel::Aux && t.lemma(k).eq_ignore_ascii_case("do");
    let be_cop = |k: usize| t.w[k].rel == Rel::Cop && t.lemma(k).eq_ignore_ascii_case("be");
    let chain: Vec<usize> = t.kids(p).iter().copied().filter(|&k| matches!(t.base(k), Rel::Aux | Rel::Cop) && !matches!(t.lemma(k), "not" | "n't")).collect();
    let Some(&first) = chain.first() else { return false };
    first < p && t.tag(first) == PTag::VB && (do_aux(first) || be_cop(first)) && chain.iter().all(|&k| do_aux(k) || be_cop(k)) && (t.upos(p) != UPos::VERB || t.tag(p) == PTag::VB)
}

/// The reader of one text: its tree, the frames made so far, the discourse.
struct Reader<'a> {
    t: &'a Tree,
    ctx: &'a mut Ctx,
    /// the token's quotation parity
    inq: Vec<bool>,
    /// interrogative words found (token index)
    wh: BTreeSet<usize>,
    /// the subject of the clause being read (its plain pronoun objects are someone else)
    not: Option<String>,
    depth: usize,
    /// pronouns the coreference sieve resolved: token → the head lemma of the antecedent
    forced: BTreeMap<usize, String>,
    /// the sentence is a question
    question: bool,
}

impl Reader<'_> {
    /// The entity of a noun phrase headed by `n` (a relative «whose» can lead back to a phrase being read: depth-bound).
    fn ent(&mut self, n: usize, animate: bool) -> Option<Ent> {
        if self.depth > 6 {
            return None;
        }
        self.depth += 1;
        let mut e = self.ent_at(n, animate);
        self.depth -= 1;
        if let Some(x) = e.as_ref().filter(|_| !off("anchors")) {
            self.ctx.ent_log.push((n, x.id.clone(), x.text.clone()));
        }
        // the function words of the mention: its preposition (with a fixed second word, «because of»), its conjunction
        if let Some(e) = e.as_mut().filter(|_| !off("ent-case")) {
            let t = self.t;
            if e.case.is_none() {
                e.case = t.kids_of(n, Rel::Case).into_iter().find(|&c| t.tag(c) != PTag::POS).map(|c| {
                    let mut w = vec![t.lemma(c).to_lowercase()];
                    w.extend(t.kids_of(c, Rel::Fixed).into_iter().map(|f| t.lemma(f).to_lowercase()));
                    w.join(" ")
                });
            }
            if e.cc.is_none() && t.base(n) == Rel::Conj {
                e.cc = t.kids_of(n, Rel::Cc).first().map(|&c| t.lemma(c).to_lowercase());
            }
        }
        e
    }

    fn ent_at(&mut self, n: usize, animate: bool) -> Option<Ent> {
        let t = self.t;
        let lemma = t.lemma(n).to_string();
        // «lots of seawater», «a group of wolves», «a kind of bird»: a word of measure or sort is the thing it measures,
        // kept as a property («pair» and «cup» stay themselves: the student counts pairs). Off by default, on with
        // PRAG_VN=quant-of: measured 2026-10-23 on the 25 courses, 19 420 → 19 385 (biology −38, both halves lower)
        if QUANT_OF.contains(&lemma.to_lowercase().as_str()) && vn_opt("quant-of") {
            let of = t.kids(n).iter().copied().find(|&k| t.base(k) == Rel::Nmod && matches!(t.upos(k), UPos::NOUN | UPos::PROPN) && t.kids_of(k, Rel::Case).iter().any(|&c| t.lemma(c) == "of"));
            if let Some(mut e) = of.and_then(|k| self.ent(k, animate)) {
                e.case = None;
                e.attrs.push(lemma.to_lowercase());
                return Some(e);
            }
        }
        if self.wh.contains(&n) {
            return None;
        }
        if t.upos(n) == UPos::PRON || t.upos(n) == UPos::DET && t.base(n) != Rel::Det {
            if WH.contains(&lemma.as_str()) || matches!(lemma.as_str(), "that" | "which") && t.head(n).is_some_and(|h| t.base(h) == Rel::Acl) {
                // a relative pronoun: the noun its clause belongs to
                let m = t.head(n).and_then(|h| t.head(h));
                let mut e = m.filter(|&m| matches!(t.upos(m), UPos::NOUN | UPos::PROPN)).and_then(|m| self.ent(m, animate));
                if let Some(e) = e.as_mut().filter(|_| !off("rel-word")) {
                    // said as the relative word here, not as the noun's phrase
                    e.rel_word = Some((lemma.to_lowercase(), t.tag(n)));
                    e.case = None;
                }
                return e;
            }
            let norm = match lemma.as_str() {
                "him" | "his" | "himself" => "he",
                "her" | "hers" | "herself" => "she",
                "its" | "itself" => "it",
                "them" | "their" | "theirs" | "themselves" => "they",
                "me" | "my" | "mine" | "myself" => "i",
                "your" | "yours" | "yourself" | "yourselves" => "you",
                "us" | "our" | "ours" | "ourselves" => "we",
                x => x,
            };
            if PERSONAL.contains(&norm) {
                return Some(Ent { id: norm.into(), text: t.form(n).into(), ..Default::default() });
            }
            // a dummy «it» («it depends», «it is unkind to …», «it seems»): no one
            if norm == "it" && t.base(n) == Rel::Nsubj && !off("clause-it") && t.head(n).is_some_and(|h| CLAUSE_IT.contains(&t.lemma(h)) && t.kids(h).iter().any(|&k| matches!(t.base(k), Rel::Ccomp | Rel::Advcl) && t.kids(k).iter().any(|&j| t.base(j) == Rel::Nsubj || t.base(j) == Rel::Mark))) {
                return Some(Ent { id: "it".into(), text: t.form(n).into(), pron: Some(t.form(n).into()), ..Default::default() });
            }
            if norm == "it" && t.base(n) == Rel::Nsubj && t.head(n).is_some_and(|h| DUMMY_IT.contains(&t.lemma(h)) || t.kids(h).iter().any(|&k| matches!(t.base(k), Rel::Csubj | Rel::Ccomp) || t.base(k) == Rel::Xcomp && t.kids_of(k, Rel::Mark).iter().any(|&m| t.lemma(m) == "to")) && !t.kids_of(h, Rel::Cop).is_empty() || t.head(n).is_some_and(|h| DUMMY_IT.contains(&t.lemma(h)))) {
                return Some(Ent { id: "it".into(), text: t.form(n).into(), pron: Some(t.form(n).into()), ..Default::default() });
            }
            if matches!(norm, "it" | "he" | "she" | "they") {
                let reflexive = lemma.ends_with("self") || lemma.ends_with("selves");
                if let Some(id) = self.forced.get(&n).cloned() {
                    let mut e = self.ctx.seen.get(&id).map(|s| s.ent.clone()).unwrap_or(Ent { id: id.clone(), text: id.clone(), ..Default::default() });
                    e.pron = Some(t.form(n).to_string());
                    self.ctx.mention(&e, animate);
                    return Some(e);
                }
                if coref_mode() == 1 || coref_mode() >= 3 {
                    return Some(Ent { id: norm.into(), text: t.form(n).into(), pron: Some(t.form(n).into()), plural: norm == "they", ..Default::default() });
                }
                let prole = match t.base(n) {
                    Rel::Nsubj => 3,
                    Rel::Obj | Rel::Iobj => 2,
                    _ => 1,
                };
                let r = self.ctx.resolve_in(&lemma_pron(&lemma), if reflexive || t.base(n) == Rel::Nsubj || t.w[n].rel == Rel::NmodPoss { None } else { self.not.as_deref() }, prole);
                return Some(match r {
                    Some(mut e) => {
                        e.pron = Some(t.form(n).to_string());
                        self.ctx.mention_as(&e, animate, prole);
                        e
                    }
                    None => Ent { id: norm.into(), text: t.form(n).into(), pron: Some(t.form(n).into()), plural: norm == "they", ..Default::default() },
                });
            }
            // «something», «everyone», «this»: an entity by its word (with what the phrase says beside it: ent-rest)
            let mut e = Ent { id: norm.into(), text: t.form(n).into(), ..Default::default() };
            if !off("ent-rest") && !off("ent-mods") {
                self.ent_more(n, &mut e);
            }
            return Some(e);
        }
        if !matches!(t.upos(n), UPos::NOUN | UPos::PROPN | UPos::NUM | UPos::ADJ | UPos::X) {
            return None;
        }
        let det = t.kids_of(n, Rel::Det).first().map(|&d| t.lemma(d).to_string());
        let named = t.upos(n) == UPos::PROPN && det.is_none();
        let plural = matches!(t.tag(n), PTag::NNS | PTag::NNPS) || det.as_deref().is_some_and(|d| matches!(d, "these" | "those" | "both" | "all" | "many"));
        let mut attrs: Vec<String> = t.kids(n).iter().filter(|&&k| matches!(t.base(k), Rel::Amod) || (t.base(k) == Rel::Compound && t.upos(k) == UPos::NOUN)).map(|&k| t.lemma(k).to_string()).collect();
        attrs.sort();
        let mut ids: Vec<usize> = t.kids(n).iter().copied().filter(|&k| k < n && matches!(t.base(k), Rel::Compound | Rel::Amod | Rel::Flat | Rel::Nummod)).collect();
        ids.push(n);
        ids.sort_unstable();
        let text = ids.iter().map(|&i| t.form(i)).collect::<Vec<_>>().join(" ");
        let and: Vec<String> = t.kids_of(n, Rel::Conj).into_iter().filter(|&c| matches!(t.upos(c), UPos::NOUN | UPos::PROPN) && !pp_conj(t, c)).map(|c| t.lemma(c).to_string()).collect();
        let id = if t.upos(n) == UPos::NUM { t.form(n).to_string() } else { lemma };
        let mut e = Ent { id, text, pron: None, plural: plural || !and.is_empty(), named, attrs, and, ..Default::default() };
        if !off("ent-det") {
            let dets = t.kids_of(n, Rel::Det);
            e.det = dets.first().map(|&d| (t.lemma(d).to_lowercase(), t.tag(d)));
            // a predeterminer, a second determiner («all the», «such a»)
            for k in t.kids(n).iter().copied().filter(|&k| t.w[k].rel == Rel::DetPredet || t.base(k) == Rel::Det && Some(&k) != dets.first()) {
                e.words.push((t.w[k].rel, t.lemma(k).to_lowercase(), t.tag(k)));
            }
        }
        // the possessor is read first («its nose»: «its» is resolved before the nose is met, the baby stays salient)
        for k in t.kids(n).to_vec() {
            if t.w[k].rel == Rel::NmodPoss {
                let p = self.ent(k, false);
                if e.poss.is_none() && !off("ent-poss") {
                    e.poss = p.map(Box::new);
                }
            }
        }
        if !off("ent-mods") {
            self.ent_more(n, &mut e);
        }
        let role = match t.base(n) {
            Rel::Nsubj => 3,
            Rel::Obj | Rel::Iobj => 2,
            _ => 1,
        };
        self.ctx.mention_as(&e, animate, role);
        let mut e = e;
        for c in t.kids_of(n, Rel::Conj) {
            if matches!(t.upos(c), UPos::NOUN | UPos::PROPN) && !pp_conj(t, c) {
                let x = self.ent(c, animate);
                if let Some(x) = x.filter(|_| !off("ent-conj")) {
                    e.conj.push(x);
                }
            }
        }
        Some(e)
    }

    /// What else the noun phrase of `n` says, read without touching the discourse: its prepositional modifiers (not
    /// the ones the frame takes as roles: those of a predicate noun, and those the parser hung on an object, which the
    /// frame reads as the clause's), its appositions, its other one-word modifiers.
    fn ent_more(&mut self, n: usize, e: &mut Ent) {
        let t = self.t;
        if is_pred(t, n) {
            return;
        }
        let obj = t.base(n) == Rel::Obj;
        for k in t.kids(n).to_vec() {
            let r = t.w[k].rel;
            match r.base() {
                Rel::Nmod if r != Rel::NmodPoss && !is_pred(t, k) => {
                    let case = t.kids_of(k, Rel::Case).first().map(|&c| t.lemma(c).to_lowercase()).unwrap_or_default();
                    if obj && !case.is_empty() && case != "of" {
                        continue;
                    }
                    if let Some(x) = self.ent_quiet(k) {
                        e.mods.push((case, x));
                    }
                }
                Rel::Appos if !is_pred(t, k) => {
                    if let Some(x) = self.ent_quiet(k) {
                        e.appos.push(x);
                    }
                }
                Rel::Advmod | Rel::Discourse => e.words.push((r, t.lemma(k).to_string(), t.tag(k))),
                Rel::Compound | Rel::Flat if t.upos(k) != UPos::NOUN => e.words.push((r, t.lemma(k).to_string(), t.tag(k))),
                _ => {}
            }
            if off("ent-rest") {
                continue;
            }
            // (ent-rest) what else the phrase says: the modifiers of its modifiers, flattened onto it («very big»,
            // «small and transparent»); its numbers; a noun's flat part; and any other dependent — a noun phrase as
            // a modifier with its preposition, a word with its relation
            if is_pred(t, k) || r.base() == Rel::Punct {
                continue;
            }
            // (a pronoun's phrase reads no coordinated phrases of its own: `ent_at`)
            let noun_conj = r.base() == Rel::Conj && matches!(t.upos(k), UPos::NOUN | UPos::PROPN) && !pp_conj(t, k) && !matches!(t.upos(n), UPos::PRON | UPos::DET);
            let kept = match r.base() {
                Rel::Det | Rel::Case | Rel::Appos | Rel::Advmod | Rel::Discourse | Rel::Amod | Rel::Nmod | Rel::Compound => true,
                Rel::Flat => t.upos(k) != UPos::NOUN,
                Rel::Conj => noun_conj,
                Rel::Cc => t.base(n) == Rel::Conj,
                _ => false,
            };
            if !kept {
                if matches!(t.upos(k), UPos::NOUN | UPos::PROPN | UPos::PRON) && r.base() != Rel::Flat {
                    if let Some(x) = self.ent_quiet(k) {
                        let case = x.case.clone().unwrap_or_default();
                        e.mods.push((case, x));
                        continue;
                    }
                }
                e.words.push((r, t.lemma(k).to_string(), t.tag(k)));
            }
            // the words under a modifier the phrase keeps as one word (not under a noun phrase it reads whole)
            if !(matches!(r.base(), Rel::Nmod | Rel::Appos | Rel::Det | Rel::Case) || noun_conj) {
                let mut ws = Vec::new();
                self.flat_words(k, &mut ws);
                e.words.extend(ws);
            }
        }
    }

    /// The words under `k` (not punctuation, not a clause), in tree order, as one-word dependents with their
    /// relation and tag: the modifiers of a modifier («very» of «very big», «transparent» and its «and» of «small and
    /// transparent», «Universal» of «Universal Dependencies» in a compound), flattened onto the phrase that keeps `k`.
    fn flat_words(&self, k: usize, out: &mut Vec<(Rel, String, PTag)>) {
        let t = self.t;
        for &j in t.kids(k) {
            if t.base(j) == Rel::Punct || is_pred(t, j) {
                continue;
            }
            out.push((t.w[j].rel, t.lemma(j).to_string(), t.tag(j)));
            self.flat_words(j, out);
        }
    }

    /// Animacy of an argument for the lexicon's restrictions: persons and animals, things, unknown.
    fn filler(&self, k: usize) -> crate::vn::Filler {
        let t = self.t;
        let animate = match t.upos(k) {
            UPos::PRON => match t.lemma(k) {
                "he" | "she" | "him" | "her" | "i" | "me" | "we" | "us" | "you" | "who" | "whom" | "someone" | "somebody" | "everyone" | "everybody" | "anyone" | "anybody" | "nobody" => Some(true),
                "it" | "something" | "everything" | "anything" | "nothing" | "what" => Some(false),
                _ => None,
            },
            UPos::PROPN => Some(true),
            UPos::NOUN => {
                let l = t.lemma(k);
                if FEM.contains(&l) || MASC.contains(&l) { Some(true) } else { lex_animate(l) }
            }
            _ => None,
        };
        crate::vn::Filler { animate }
    }

    /// The entity token `k` was read into, without touching the discourse (the reader has mentioned it already).
    fn ent_quiet(&mut self, k: usize) -> Option<Ent> {
        // only the entities and the clock change when a phrase is read (not the sentences of the discourse)
        let (seen, time) = (self.ctx.seen.clone(), self.ctx.time);
        let e = self.ent(k, false);
        self.ctx.seen = seen;
        self.ctx.time = time;
        e
    }

    /// The lexicon's roles for the clause of `p` (VerbNet's best frame), applied to the
    /// frame the tree heuristics made: an object VerbNet calls a Recipient is the one given or told to; a
    /// prepositional phrase it calls a Destination, Location or Source is the place, a Recipient (or a Goal that is
    /// a person) the recipient, anything else (a Theme «at» or «about», an Instrument, an Attribute) not a place.
    fn lexicon_roles(&mut self, p: usize, f: &mut Frame, subj: Option<usize>, speaker_obj: Option<usize>, phrasal_obj: Option<usize>, passive: bool) {
        use crate::vn::{Clause, Comp};
        let t = self.t;
        let kids: Vec<usize> = t.kids(p).to_vec();
        let mut cl = Clause { passive, ..Default::default() };
        if f.verb.contains(' ') {
            cl.verbs.push(f.verb.clone());
        }
        cl.verbs.push(t.lemma(p).to_string());
        cl.subj = subj.map(|s| self.filler(s));
        let mut by = kids.iter().copied().find(|&k| t.w[k].rel == Rel::OblAgent);
        // a passive's «by» phrase the parser left a plain oblique («flung from steamers by them»): the doer, when it
        // can do things (a person, an animal, a pronoun) — not a means («by angle», «by hand»)
        if passive && by.is_none() && f.agent.is_none() && !off("vn-by") {
            if let Some(k) = kids.iter().copied().find(|&k| t.base(k) == Rel::Obl && t.kids_of(k, Rel::Case).first().is_some_and(|&c| t.lemma(c) == "by") && self.filler(k).animate != Some(false) && matches!(t.upos(k), UPos::PRON | UPos::PROPN | UPos::NOUN)) {
                if let Some(e) = self.ent_quiet(k) {
                    if let Some(i) = f.obls.iter().position(|(c, x)| c == "by" && x.text == e.text) {
                        f.agent = Some(f.obls.remove(i).1);
                        by = Some(k);
                    }
                }
            }
        }
        cl.by = by.map(|k| self.filler(k));
        let time = |k: usize| time_tok(t, k);
        let mut obj_toks: Vec<usize> = kids.iter().copied().filter(|&k| t.base(k) == Rel::Iobj && !time(k)).collect();
        obj_toks.extend(kids.iter().copied().filter(|&k| t.base(k) == Rel::Obj && Some(k) != speaker_obj && !time(k) && !is_pred(t, k)));
        if let Some(k) = phrasal_obj {
            obj_toks.push(k);
        }
        cl.objs = obj_toks.iter().map(|&k| self.filler(k)).collect();
        // (vn-gap) a relative clause with a subject and no object nor relative word: the noun it belongs to is the
        // object it lacks («things you throw away»), which the reader makes its patient — the matcher sees it too
        let to_inf = t.kids_of(p, Rel::Mark).iter().any(|&k| t.lemma(k) == "to");
        let gap = (!off("vn-gap") && t.base(p) == Rel::Acl && !passive && subj.is_some() && obj_toks.is_empty() && !to_inf && !relative_word(t, p)).then(|| t.head(p)).flatten().filter(|&m| matches!(t.upos(m), UPos::NOUN | UPos::PROPN));
        if let Some(m) = gap {
            cl.objs.push(self.filler(m));
        }
        // prepositional phrases on the verb, and the ones the parser hung on its object («throws a pan after her»)
        let mut pp_toks: Vec<(usize, String)> = Vec::new();
        let mut pp_of = |k: usize, pp_toks: &mut Vec<(usize, String)>| {
            let r = t.w[k].rel;
            if Some(k) == phrasal_obj || Some(k) == by || is_pred(t, k) || time(k) || r == Rel::OblAgent || r == Rel::NmodPoss || r == Rel::NmodTmod || r == Rel::OblTmod {
                return;
            }
            if let Some(&c) = t.kids_of(k, Rel::Case).first() {
                pp_toks.push((k, t.lemma(c).to_lowercase()));
            }
        };
        for &k in &kids {
            if matches!(t.base(k), Rel::Obl | Rel::Nmod) {
                pp_of(k, &mut pp_toks);
            }
        }
        if let Some(&o) = obj_toks.iter().find(|&&o| t.base(o) == Rel::Obj) {
            for j in t.kids(o).to_vec() {
                if t.w[j].rel == Rel::Nmod && t.kids_of(j, Rel::Case).first().is_some_and(|&c| t.lemma(c) != "of") || pp_conj(t, j) {
                    pp_of(j, &mut pp_toks);
                }
            }
        }
        cl.pps = pp_toks.iter().map(|(k, c)| (c.clone(), self.filler(*k))).collect();
        // a clause complement: what kind
        for &k in &kids {
            match t.base(k) {
                Rel::Ccomp | Rel::Xcomp if is_pred(t, k) && t.upos(k) == UPos::VERB => {
                    let marks: Vec<&str> = t.kids(k).iter().filter(|&&j| t.base(j) == Rel::Mark).map(|&j| t.lemma(j)).collect();
                    cl.comp = Some(if self.inq[k] && !self.inq[p] {
                        Comp::Quote
                    } else if marks.contains(&"to") {
                        Comp::To
                    } else if marks.iter().any(|m| matches!(*m, "whether" | "if")) || t.kids(k).iter().any(|&j| j < k && WH.contains(&t.lemma(j))) {
                        Comp::Wh
                    } else if t.tag(k) == PTag::VBG {
                        Comp::Ing
                    } else {
                        Comp::That
                    });
                }
                Rel::Parataxis if self.inq[k] != self.inq[p] && cl.comp.is_none() => cl.comp = Some(Comp::Quote),
                Rel::Xcomp if t.upos(k) == UPos::ADJ => cl.adj = true,
                _ => {}
            }
        }
        // VerbNet's best frame (a verb VerbNet lacks keeps the heuristics' roles)
        let (roles_obj, roles_pp): (Vec<Option<String>>, Vec<Option<(RoleTo, String)>>);
        let mut how = How::default();
        if let Some(m) = crate::vn::best(&cl).filter(|_| !off("vn-verbnet")) {
            how.by = By::VerbNet;
            how.sig = cl.sig();
            how.class = m.class_id().to_string();
            how.frame = m.vframe().primary.clone();
            if let Some(r) = if passive { &m.by } else { &m.subj } {
                how.roles.push(("agent".into(), r.clone()));
            }
            if passive {
                if let Some(r) = &m.subj {
                    how.roles.push(("subject".into(), r.clone()));
                }
            }
            for (k, r) in obj_toks.iter().chain(gap.iter()).zip(&m.objs) {
                if let Some(r) = r {
                    how.roles.push((t.lemma(*k).to_string(), r.clone()));
                }
            }
            for ((k, c), r) in pp_toks.iter().zip(&m.pps) {
                if let Some(r) = r {
                    how.roles.push((format!("{c} {}", t.lemma(*k)), r.clone()));
                }
            }
            if let Some(r) = &m.comp {
                how.roles.push(("clause".into(), r.clone()));
            }
            roles_obj = m.objs.clone();
            // (PRAG_VN=goal-obl) a Goal is a place only in the classes of putting, sending and motion; elsewhere it is
            // where something ends up in no space — «sort into paper and plastic», «adapt to the situation» — and an
            // oblique (2026-10-19: −24 tests in biology, the student's weights lean on place matches; not on by default:
            // `Frame::goal` tells such a place apart instead)
            let spatial = !vn_opt("goal-obl") || spatial_class(m.class_id());
            roles_pp = m.pps.iter().zip(&cl.pps).map(|(r, (_, fl))| r.as_ref().map(|r| (if r == "Goal" && !spatial && fl.animate != Some(true) { RoleTo::Obl } else { role_to(r, fl.animate) }, r.clone()))).collect();
        } else {
            return;
        }
        // objects: a Recipient is the one given or told to; a Theme the heuristics took for the one told is the patient
        if !off("vn-obj") {
            for (&k, r) in obj_toks.iter().zip(&roles_obj) {
                let Some(r) = r else { continue };
                let an = self.filler(k).animate;
                let to = role_to(r, an);
                let Some(e) = self.ent_quiet(k) else { continue };
                let same = |x: &Option<Ent>| x.as_ref().is_some_and(|x| x.text == e.text);
                // only a person or an animal is told or given to; a thing the heuristics took for one is the patient
                // (an object with a clause complement is the doer of the complement — «help her find», object control —
                // and stays where the control reads it)
                if to == RoleTo::Recipient && an == Some(true) && f.recipient.is_none() && same(&f.patient) && cl.comp.is_none() {
                    f.recipient = f.patient.take();
                } else if to == RoleTo::Patient && an == Some(false) && f.patient.is_none() && same(&f.recipient) {
                    f.patient = f.recipient.take();
                } else if to == RoleTo::Patient && f.patient.is_none() && !same(&f.recipient) && obj_toks.len() == 2 && !off("vn-dative") {
                    // «gave the Mole a cake» with two objects: the second, which the heuristics left, is the theme
                    f.patient = Some(e);
                }
            }
        }
        // prepositional phrases: the place, the recipient, or another role
        for ((k, c), to) in pp_toks.iter().zip(&roles_pp) {
            let Some((to, rname)) = to else { continue };
            let Some(e) = self.ent_quiet(*k) else { continue };
            let same = |x: &Option<Ent>| x.as_ref().is_some_and(|x| x.text == e.text);
            match to {
                RoleTo::Place if !off("vn-place") && c == "to" && vn_opt("to-recipient") => {
                    if f.recipient.is_none() {
                        if same(&f.place) {
                            f.recipient = f.place.take();
                        } else if let Some(i) = f.obls.iter().position(|(p, x)| p == c && x.text == e.text) {
                            f.recipient = Some(f.obls.remove(i).1);
                        }
                    }
                }
                RoleTo::Place if !off("vn-place") => {
                    // «by», «with» and the like are no place whatever the frame says («hang by the tail», «live with warthogs»)
                    let ok_prep = !matches!(c.as_str(), "by" | "with" | "of" | "for" | "about" | "like" | "as" | "than") || vn_opt("place-any-prep");
                    let ok_to = c != "to" || !vn_opt("noto-place");
                    if f.place.is_none() && placeable(&e) && ok_prep && ok_to {
                        if same(&f.recipient) {
                            f.place = f.recipient.take();
                        } else if let Some(i) = f.obls.iter().position(|(p, x)| p == c && x.text == e.text) {
                            f.place = Some(f.obls.remove(i).1);
                        }
                    }
                }
                RoleTo::Recipient if !off("vn-recipient") => {
                    if f.recipient.is_none() && (c == "to" || vn_opt("recipient-any")) {
                        if same(&f.place) {
                            f.recipient = f.place.take();
                        } else if let Some(i) = f.obls.iter().position(|(p, x)| p == c && x.text == e.text) {
                            f.recipient = Some(f.obls.remove(i).1);
                        }
                    }
                }
                RoleTo::Patient | RoleTo::Obl | RoleTo::Agent if !off("vn-notplace") && (NOT_PLACE_ROLES.contains(&rname.to_lowercase().as_str()) || vn_opt("goal-obl") && matches!(rname.as_str(), "Goal" | "Result")) => {
                    if same(&f.place) {
                        let x = f.place.take().unwrap();
                        f.obls.push((c.clone(), x));
                    }
                }
                _ => {}
            }
        }
        f.how = how;
    }

    /// The event a noun names (WordNet's derivational links): «the arrival of the Queen» → the Queen arrives, «the
    /// destruction of the city by the army» → the army destroys the city, «Alice's answer» → Alice answers. Only with
    /// an argument. The positions by general rules: «by X» is the agent; «of X» is the patient when the verb sense
    /// takes an object (WordNet's frames), else the agent; «X's» is the agent, but the patient of a verb with an
    /// object when a «by» agent is there or X is a thing and no «of» patient is («the city's destruction»), and
    /// nothing when an «of» agent is there.
    fn nominal(&mut self, n: usize) -> Option<Frame> {
        let t = self.t;
        let gerund;
        let nom = match crate::vn::nom(t.lemma(n)) {
            Some(x) => x,
            None => {
                gerund = gerund_nom(t, n)?;
                &gerund
            }
        };
        let transitive = nom.transitive;
        let mut f = Frame::new(Kind::Event, nom.verb.clone());
        f.how = How { by: By::Nominal, class: nom.noun.clone(), frame: format!("{} {}", nom.file, if transitive { "transitive" } else { "intransitive" }), ..How::default() };
        let mut args: Vec<(String, Option<bool>, Ent)> = Vec::new();
        for k in t.kids(n).to_vec() {
            if is_pred(t, k) || !matches!(t.upos(k), UPos::NOUN | UPos::PROPN | UPos::PRON) {
                continue;
            }
            let poss = t.w[k].rel == Rel::NmodPoss;
            if !poss && t.base(k) != Rel::Nmod {
                continue;
            }
            let case = if poss { "poss".to_string() } else { t.kids_of(k, Rel::Case).first().map(|&c| t.lemma(c).to_lowercase()).unwrap_or_default() };
            if case.is_empty() {
                continue;
            }
            let an = self.filler(k).animate;
            let Some(e) = self.ent_quiet(k) else { continue };
            args.push((case, an, e));
        }
        let has = |c: &str| args.iter().any(|a| a.0 == c);
        let (by, of) = (has("by"), has("of"));
        for (case, an, e) in args {
            let slot = match case.as_str() {
                "by" => "agent",
                "of" if transitive => "patient",
                "of" => "agent",
                "poss" if transitive && (by || an == Some(false) && !of) => "patient",
                "poss" if !transitive && of => "",
                "poss" => "agent",
                c if PLACE_PREPS.contains(&c) && placeable(&e) => "place",
                _ => "obl",
            };
            match slot {
                "agent" if f.agent.is_none() => f.agent = Some(e),
                "patient" if f.patient.is_none() => f.patient = Some(e),
                "place" if f.place.is_none() => f.place = Some(e),
                "obl" => f.obls.push((case.clone(), e)),
                _ => continue,
            }
            f.how.roles.push((case, slot.to_string()));
        }
        (f.agent.is_some() || f.patient.is_some()).then_some(f)
    }

    /// The frame of predicate `p` (roles only; links later).
    fn frame(&mut self, p: usize) -> Frame {
        let t = self.t;
        let kids: Vec<usize> = t.kids(p).to_vec();
        let has_cop = !t.kids_of(p, Rel::Cop).is_empty();
        let passive = kids.iter().any(|&k| t.w[k].rel == Rel::AuxPass || t.w[k].rel == Rel::NsubjPass);
        let mut verb = if has_cop || t.upos(p) != UPos::VERB && t.upos(p) != UPos::NOUN { "be".to_string() } else { t.lemma(p).to_string() };
        // the particle: «put down», also when the parser took it for an adverb right after the verb or its object
        let mut prt_tok: Option<usize> = None;
        if let Some(&prt) = kids.iter().find(|&&k| t.w[k].rel == Rel::CompoundPrt || k > p && k <= p + 3 && matches!(t.upos(k), UPos::ADV | UPos::ADP) && PARTICLES.contains(&t.lemma(k)) && matches!(t.base(k), Rel::Advmod | Rel::Xcomp) && t.kids(k).is_empty()) {
            if !has_cop {
                verb += &format!(" {}", t.lemma(prt));
                prt_tok = Some(prt);
            }
        }
        // a phrasal verb the expression base knows («turn on the fan», «break into a snigger»): the word right after
        // the verb, which the parser hung as the case of the next noun, is the particle and the noun is the object
        let mut phrasal_obj: Option<usize> = None;
        if !has_cop && !verb.contains(' ') && t.upos(p) == UPos::VERB && !off("phrasal") {
            // the adverb particles anywhere; the prepositional ones only when they head no noun (an adverb, «pass on»)
            let bare_prt = |k: usize| !off("phrasal-bare") && PHRASAL_BARE.contains(&t.lemma(k)) && t.base(k) != Rel::Case && t.kids(k).is_empty();
            if let Some(k) = (p + 1 < t.len()).then_some(p + 1).filter(|&k| matches!(t.upos(k), UPos::ADP | UPos::ADV) && (PHRASAL.contains(&t.lemma(k)) || bare_prt(k))) {
                let pv = format!("{} {}", t.lemma(p), t.lemma(k));
                if global::expression(&pv).iter().any(|e| e.pos == "Verb") {
                    verb = pv;
                    if t.base(k) != Rel::Case {
                        prt_tok = Some(k);
                    }
                    phrasal_obj = t.head(k).filter(|&n| t.base(k) == Rel::Case && t.head(n) == Some(p) && matches!(t.base(n), Rel::Obl | Rel::Nmod));
                }
            }
        }
        let mut f = Frame::new(Kind::Event, verb.clone());
        f.neg = negated(t, p);
        if !off("tam") {
            f.tam = tam_of(t, p);
        }
        // the subject first: it is the first mention, and the agent of an action is animate
        let mut subj = kids.iter().copied().find(|&k| t.base(k) == Rel::Nsubj);
        // a speech verb after its quote, its speaker after it («"…" cries the Badger», «"…," says Mum»): the speaker
        // is the noun after the verb outside the quotes, not a word of the quote
        let mut speaker_obj: Option<usize> = None;
        // (a quoted subject that is only a noun phrase — «"a long yellow ferret with a gun" says …» — is the speaker)
        let quoted_clause = |s: usize| {
            let (mut a, mut b) = (s, s);
            while a > 0 && self.inq[a - 1] {
                a -= 1;
            }
            while b + 1 < t.len() && self.inq[b + 1] {
                b += 1;
            }
            // (a copula as a clause too — «"Where is the lamb?" said Isaac», speaker Isaac, not the lamb — was tried
            // 17.10: −1 on the courses, the frame bench unchanged; not kept)
            (a..=b).any(|i| t.upos(i) == UPos::VERB)
        };
        if is_say(t.lemma(p)) && !self.inq[p] && (0..p).any(|k| self.inq[k]) && subj.is_none_or(|s| self.inq[s] && quoted_clause(s)) && !off("say-inverted") {
            if let Some(k) = kids.iter().copied().find(|&k| k > p && !self.inq[k] && matches!(t.base(k), Rel::Obj | Rel::Nsubj) && matches!(t.upos(k), UPos::NOUN | UPos::PROPN)) {
                speaker_obj = Some(k);
                subj = Some(k);
            } else if subj.is_some() {
                subj = None;
            }
        }
        if let Some(s) = subj {
            let e = self.ent(s, !has_cop);
            if passive { f.patient = e } else { f.agent = e }
        } else if let Some(&g) = kids.iter().find(|&&k| t.base(k) == Rel::Csubj && t.tag(k) == PTag::VBG).filter(|_| !off("gerund-subject")) {
            // «Holding the handrail lets you …», «Waiting your turn is fair»: the activity is the subject
            f.agent = Some(Ent { id: t.form(g).to_lowercase(), text: t.form(g).into(), ..Default::default() });
        }
        // an imperative: the root verb in its base form with no subject and no auxiliary — said to «you»
        if subj.is_none() && !off("imperative") && t.w[p].rel == Rel::Root && (t.tag(p) == PTag::VB && !kids.iter().any(|&k| matches!(t.base(k), Rel::Aux | Rel::Mark | Rel::Expl)) && !has_cop || imperative_do(t, p)) {
            f.agent = Some(Ent { id: "you".into(), text: "you".into(), ..Default::default() });
            f.unsay(Slot::Agent);
        }
        self.not = f.agent.as_ref().map(|a| a.id.clone());
        if passive {
            if let Some(&a) = kids.iter().find(|&&k| t.w[k].rel == Rel::OblAgent) {
                f.agent = self.ent(a, true);
            }
        }
        if has_cop {
            match t.upos(p) {
                UPos::NOUN | UPos::PROPN | UPos::PRON | UPos::NUM => {
                    f.kind = Kind::Fact(Link::IsA);
                    if t.kids_of(p, Rel::Case).is_empty() {
                        f.patient = self.ent(p, false);
                    } else {
                        // «is in the wood», «was like a star-fish»
                        let c = t.lemma(t.kids_of(p, Rel::Case)[0]).to_string();
                        let e = self.ent(p, false);
                        f.kind = Kind::Fact(Link::Is);
                        if let Some(e) = e {
                            if PLACE_PREPS.contains(&c.as_str()) && placeable(&e) { f.place = Some(e) } else { f.obls.push((c, e)) }
                        }
                    }
                }
                _ => {
                    f.kind = Kind::Fact(Link::Is);
                    f.attrs.push(t.lemma(p).to_string());
                }
            }
        }
        for &k in &kids {
            let r = t.w[k].rel;
            if is_pred(t, k) && k != p {
                continue;
            }
            // (PRAG_OFF=wh-time) a time value is kept whole as an attribute, «In March 2022» → when:march 2022 (and
            // when:2022), «in 1999» → when:1999, and an event a time preposition puts as a time («during the war») as
            // when:war; the rest of the reading is unchanged
            if matches!(r.base(), Rel::Obj | Rel::Obl | Rel::Nmod) && !off("wh-time") {
                let mut vs = time_value(t, k);
                if vs.is_empty() && r.base() != Rel::Obj && matches!(t.upos(k), UPos::NOUN | UPos::PROPN) && t.kids_of(k, Rel::Case).first().is_some_and(|&c| TIME_PREPS.contains(&t.lemma(c).to_lowercase().as_str())) {
                    vs.push(t.lemma(k).to_lowercase());
                }
                for v in vs {
                    let a = format!("when:{v}");
                    if !f.attrs.contains(&a) {
                        f.attrs.push(a);
                    }
                }
            }
            // a time is not a role: it goes to the attributes as «when:…»
            if matches!(r.base(), Rel::Obj | Rel::Obl | Rel::Nmod) && time_tok(t, k) {
                f.attrs.push(format!("when:{}", t.lemma(k)));
                continue;
            }
            match r.base() {
                Rel::Obj if speaker_obj == Some(k) => {}
                Rel::Obj if !has_cop && TELL.contains(&t.lemma(p)) && f.recipient.is_none() && person_like(t, k) && !off("tell-to") => f.recipient = self.ent(k, false),
                Rel::Obj if !has_cop => {
                    if f.patient.is_none() {
                        f.patient = self.ent(k, false);
                        // a prepositional phrase the parser hung on the object («throws a pan after her»)
                        for j in t.kids(k).to_vec() {
                            if (t.w[j].rel == Rel::Nmod || pp_conj(t, j)) && !is_pred(t, j) {
                                let case = t.kids_of(j, Rel::Case).first().map(|&c| t.lemma(c).to_string()).unwrap_or_default();
                                if case.is_empty() || case == "of" {
                                    continue;
                                }
                                if let Some(e) = self.ent(j, false) {
                                    if PLACE_PREPS.contains(&case.as_str()) && f.place.is_none() && placeable(&e) { f.place = Some(e) } else { f.obls.push((case, e)) }
                                }
                            }
                        }
                    }
                }
                Rel::Iobj => f.recipient = self.ent(k, false),
                Rel::Expl if t.lemma(k) == "it" && t.upos(p) == UPos::VERB && f.patient.is_none() => f.patient = self.ent(k, false),
                Rel::Obl | Rel::Nmod if phrasal_obj == Some(k) && f.patient.is_none() => f.patient = self.ent(k, false),
                Rel::Obl | Rel::Nmod if r != Rel::OblAgent && r != Rel::NmodPoss && r != Rel::NmodTmod && r != Rel::OblTmod => {
                    let case = t.kids_of(k, Rel::Case).first().map(|&c| t.lemma(c).to_string()).unwrap_or_default();
                    let Some(e) = self.ent(k, false) else { continue };
                    if case == "to" && (e.named || PERSONAL.contains(&e.id.as_str()) || e.pron.is_some()) && f.recipient.is_none() {
                        f.recipient = Some(e);
                    } else if PLACE_PREPS.contains(&case.as_str()) && f.place.is_none() && placeable(&e) {
                        f.place = Some(e);
                    } else if !case.is_empty() {
                        f.obls.push((case, e));
                    } else if f.patient.is_none() && !has_cop {
                        f.patient = Some(e);
                    } else if !off("frame-nps") {
                        f.nps.push((r, e));
                    }
                }
                Rel::Obl | Rel::Nmod | Rel::Vocative | Rel::Dislocated if matches!(r, Rel::OblTmod | Rel::NmodTmod | Rel::Vocative | Rel::Dislocated) && !off("frame-nps") => {
                    if let Some(e) = self.ent_quiet(k) {
                        f.nps.push((r, e));
                    }
                }
                Rel::Xcomp if matches!(t.upos(k), UPos::ADJ) => f.attrs.push(t.lemma(k).to_string()),
                Rel::Xcomp if matches!(t.upos(k), UPos::NOUN) && f.patient.is_none() => f.patient = self.ent(k, false),
                Rel::Advmod if t.upos(k) == UPos::ADV && !NOT_MANNER.contains(&t.lemma(k)) && t.lemma(k).len() > 2 => {
                    f.attrs.push(t.lemma(k).to_string());
                    if !off("frame-manner") {
                        f.manner.push(t.lemma(k).to_string());
                    }
                }
                // the other adverbs and discourse words, but not negation (`neg`), a particle (in `verb`) or the asked word
                Rel::Advmod | Rel::Discourse if !off("frame-words") && Some(k) != prt_tok && !self.wh.contains(&k) && !matches!(t.lemma(k), "not" | "n't" | "never") && t.kids(k).iter().all(|&j| t.base(j) != Rel::Case) => {
                    f.words.push((r, t.lemma(k).to_string(), t.tag(k)))
                }
                _ => {}
            }
        }
        // the linguists' rules: the verb's VerbNet frame names each argument's role
        if !has_cop && t.upos(p) == UPos::VERB && !off("vn") && !(self.question && vn_opt("noq")) {
            self.lexicon_roles(p, &mut f, subj, speaker_obj, phrasal_obj, passive);
        }
        // (frame-rest) the clause's other noun phrases, which no role took — a second object, a noun in parataxis or
        // coordinated with the verb, a noun compound on the verb — with their relation; and the one-word modifiers of
        // its words and manner adverbs («when and where», «very slowly»), flattened onto the clause
        if !off("frame-rest") {
            let nominal = matches!(t.upos(p), UPos::NOUN | UPos::PROPN | UPos::PRON | UPos::NUM | UPos::ADJ);
            // the words a punctuation mark heads (the parser hangs a word on a «·» or a «:» now and then) are the clause's
            let mut kids = kids.clone();
            for &k in t.kids(p) {
                if t.base(k) == Rel::Punct {
                    kids.extend(t.kids(k).iter().copied().filter(|&j| t.base(j) != Rel::Punct));
                }
            }
            for &k in &kids {
                // (a predicate noun's adjectives and compounds are its entity's attributes: their own modifiers here)
                if nominal && matches!(t.base(k), Rel::Amod | Rel::Compound | Rel::Flat | Rel::Nummod) {
                    if matches!(t.base(k), Rel::Nummod | Rel::Flat) || t.base(k) == Rel::Compound && t.upos(k) != UPos::NOUN {
                        f.words.push((t.w[k].rel, t.lemma(k).to_string(), t.tag(k)));
                    }
                    self.flat_words(k, &mut f.words);
                    continue;
                }
                if k == p || is_pred(t, k) || time_tok(t, k) || !matches!(t.upos(k), UPos::NOUN | UPos::PROPN | UPos::PRON | UPos::NUM) || matches!(t.base(k), Rel::Punct | Rel::Det | Rel::Case | Rel::Mark | Rel::Cc | Rel::Aux | Rel::Cop | Rel::Expl) {
                    continue;
                }
                // (wh-free) a wh-word of no relative clause on a noun — «who acts», «what happens», «I know what you
                // did» — fills no role (the student reads none there) and is kept here by its word, as «something» is
                let e = match self.ent_quiet(k) {
                    Some(e) => e,
                    None if WH.contains(&t.lemma(k)) && !off("wh-free") => {
                        let mut e = Ent { id: t.lemma(k).to_lowercase(), text: t.form(k).into(), ..Default::default() };
                        if !off("ent-mods") {
                            self.ent_more(k, &mut e);
                        }
                        e
                    }
                    None => continue,
                };
                // (a phrase a role or an oblique took is said there)
                let has = |x: &Ent| x.text == e.text && (x.id == e.id || x.pron.is_some());
                let roles = [&f.agent, &f.patient, &f.recipient, &f.place];
                if roles.iter().any(|r| r.as_ref().is_some_and(has)) || f.obls.iter().any(|(_, x)| has(x)) || f.nps.iter().any(|(_, x)| has(x)) {
                    continue;
                }
                f.nps.push((t.w[k].rel, e));
            }
            for &k in &kids {
                let said = t.base(k) == Rel::Advmod && (f.attrs.iter().any(|a| a == t.lemma(k)) || f.words.iter().any(|w| w.1 == t.lemma(k))) || t.base(k) == Rel::Discourse;
                if said && Some(k) != prt_tok {
                    self.flat_words(k, &mut f.words);
                }
            }
        }
        // the kind of a statement (know.rs links) — an action otherwise
        if f.kind == Kind::Event {
            let aux: Vec<&str> = kids.iter().filter(|&&k| t.base(k) == Rel::Aux).map(|&k| t.lemma(k)).collect();
            let lemma = t.lemma(p);
            let case_of = |e: &str| f.obls.iter().any(|(c, _)| c == e);
            let naming = lemma == "call" && !off("call-naming") && !verb.contains(' ') && !kids.iter().any(|&k| matches!(t.base(k), Rel::Ccomp | Rel::Parataxis));
            f.kind = if is_say(lemma) && !naming {
                Kind::Say(act_of(lemma))
            } else if aux.iter().any(|a| matches!(*a, "can" | "could" | "cannot")) {
                Kind::Fact(Link::Can)
            } else if lemma == "have" && !passive {
                Kind::Fact(Link::Has)
            } else if lemma == "live" && f.place.is_some() {
                Kind::Fact(Link::LivesIn)
            } else if lemma == "make" && passive && case_of("of") {
                Kind::Fact(Link::MadeOf)
            } else if lemma == "use" && passive && case_of("for") {
                Kind::Fact(Link::UsedFor)
            } else if matches!(t.tag(p), PTag::VBZ | PTag::VBP) && f.agent.as_ref().is_some_and(|a| a.plural && !a.named || a.id == "you") {
                Kind::Fact(Link::Does)
            } else {
                Kind::Event
            };
        }
        if f.kind == Kind::Say(Act::Assert) && f.attrs.iter().any(|a| a == "sorry") && !off("say-sorry") {
            f.kind = Kind::Say(Act::Apologize);
        }
        if !off("frame-mark") {
            f.mark = kids.iter().filter(|&&k| t.base(k) == Rel::Mark).map(|&k| (t.lemma(k).to_lowercase(), t.tag(k))).collect();
            f.cc = kids.iter().find(|&&k| t.base(k) == Rel::Cc).map(|&k| t.lemma(k).to_lowercase());
            for &k in &kids {
                if t.base(k) == Rel::Expl && t.lemma(k).eq_ignore_ascii_case("there") {
                    f.words.push((t.w[k].rel, "there".into(), t.tag(k)));
                }
            }
        }
        f.attrs.sort();
        f.attrs.dedup();
        self.not = None;
        f
    }
}

/// The frames of one parsed text, with the discourse carried on. `wh` collects the interrogative words' tokens.
pub fn frames_of_tree(t: &Tree, ctx: &mut Ctx, question: bool) -> (Vec<Frame>, Vec<Asked>, Vec<(String, String)>) {
    let (fs, asked, aliases, _) = frames_of_tree_with(t, ctx, question, BTreeMap::new());
    (fs, asked, aliases)
}

/// The implication signature of a verb with a «to» complement (`data/signatures.tsv`): did the complement happen when
/// the verb is affirmed, and when negated (b'+', b'-', b'o').
pub fn signature(lemma: &str) -> Option<(u8, u8)> {
    static S: std::sync::OnceLock<BTreeMap<&'static str, (u8, u8)>> = std::sync::OnceLock::new();
    S.get_or_init(|| {
        include_str!("../data/signatures.tsv")
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            .filter_map(|l| {
                let c: Vec<&str> = l.split('\t').collect();
                Some((*c.first()?, (*c.get(1)?.as_bytes().first()?, *c.get(2)?.as_bytes().first()?)))
            })
            .collect()
    })
    .get(lemma)
    .copied()
}

/// A step of the reader switched off (PRAG_OFF=name,name — for measuring what each step brings).
pub fn off(name: &str) -> bool {
    std::env::var("PRAG_OFF").is_ok_and(|v| v.split(',').any(|x| x == name))
}

/// An experimental step of the lexicon roles switched on (PRAG_VN=name,name — for measuring).
pub fn vn_opt(name: &str) -> bool {
    std::env::var("PRAG_VN").is_ok_and(|v| v.split(',').any(|x| x == name))
}

/// Experiment switch (PRAG_COREF): 0 — the old salience, 1 — the coref sieve only, 2 — the sieve, then the
/// resolver, 3 — the resolver in word order, 4 — the sieve, then the resolver in word order.
pub fn coref_mode() -> u8 {
    std::env::var("PRAG_COREF").ok().and_then(|x| x.parse().ok()).unwrap_or(3)
}

/// The coref crate's sieve over the discourse so far (`ctx.sents`, the last one being read): each pronoun of the
/// last sentence in a chain → the head lemma of the chain's first nominal mention before it.
fn coref_last(ctx: &Ctx) -> BTreeMap<usize, String> {
    let mut out = BTreeMap::new();
    let Some(last) = ctx.sents.len().checked_sub(1) else { return out };
    let doc = coref::doc::from_words("d", &ctx.sents);
    let sys = coref::eval::resolve(&doc, &coref::sieve::Config::default());
    for c in &sys.clusters {
        for &m in c {
            let x = &sys.ms[m];
            if x.span.sent != last || x.kind != coref::mention::Kind::Pronoun || x.coord {
                continue;
            }
            let ante = c.iter().copied().filter(|&a| sys.ms[a].kind != coref::mention::Kind::Pronoun && !sys.ms[a].coord && (sys.ms[a].span.sent, sys.ms[a].head) < (x.span.sent, x.head)).min_by_key(|&a| (sys.ms[a].span.sent, sys.ms[a].head));
            if let Some(a) = ante {
                let s = &doc.sents[sys.ms[a].span.sent];
                out.insert(x.head, s.lem(sys.ms[a].head));
            }
        }
    }
    out
}

/// The resolver's pass over a sentence in word order (a pronoun sees only what came before it, as a reader does):
/// nouns are mentioned with their role, each personal pronoun is resolved (`Ctx::resolve_in`) and mentioned as what
/// it refers to. `forced` (the sieve's answers) wins where it has one. Token → the antecedent's head lemma.
fn linear_pass(t: &Tree, ctx: &mut Ctx, forced: &BTreeMap<usize, String>) -> BTreeMap<usize, String> {
    let mut out: BTreeMap<usize, String> = BTreeMap::new();
    let role_of = |i: usize| match t.base(i) {
        Rel::Nsubj => 3u8,
        Rel::Obj | Rel::Iobj => 2,
        _ => 1,
    };
    for i in 0..t.len() {
        let l = t.lemma(i).to_lowercase();
        let animate = t.base(i) == Rel::Nsubj && t.w[i].rel != Rel::NsubjPass && t.head(i).is_some_and(|h| t.kids_of(h, Rel::Cop).is_empty());
        if matches!(t.upos(i), UPos::NOUN | UPos::PROPN) && !matches!(t.base(i), Rel::Compound | Rel::Flat) {
            if TIME.contains(&l.as_str()) || time_tok(t, i) {
                continue;
            }
            let det = t.kids_of(i, Rel::Det).first().map(|&d| t.lemma(d).to_string());
            let named = t.upos(i) == UPos::PROPN && det.is_none();
            let and: Vec<String> = t.kids_of(i, Rel::Conj).into_iter().filter(|&c| matches!(t.upos(c), UPos::NOUN | UPos::PROPN) && !pp_conj(t, c)).map(|c| t.lemma(c).to_string()).collect();
            let plural = matches!(t.tag(i), PTag::NNS | PTag::NNPS) || !and.is_empty();
            let e = Ent { id: l.clone(), text: t.form(i).into(), named, plural, and, ..Default::default() };
            ctx.mention_as(&e, animate, role_of(i));
            continue;
        }
        if t.upos(i) != UPos::PRON {
            continue;
        }
        let f = t.form(i).to_lowercase();
        let norm = match f.as_str() {
            "he" | "him" | "his" | "himself" => "he",
            "she" | "her" | "hers" | "herself" => "she",
            "it" | "its" | "itself" => "it",
            "they" | "them" | "their" | "theirs" | "themselves" => "they",
            _ => continue,
        };
        // a dummy «it» is left to the reader
        if norm == "it" && t.base(i) == Rel::Nsubj && t.head(i).is_some_and(|h| DUMMY_IT.contains(&t.lemma(h)) || !off("clause-it") && CLAUSE_IT.contains(&t.lemma(h)) && t.kids(h).iter().any(|&k| matches!(t.base(k), Rel::Ccomp | Rel::Advcl) && t.kids(k).iter().any(|&j| t.base(j) == Rel::Nsubj || t.base(j) == Rel::Mark))) {
            continue;
        }
        let id = if let Some(x) = forced.get(&i) {
            Some(x.clone())
        } else {
            let reflexive = f.ends_with("self") || f.ends_with("selves");
            // a plain object pronoun is not its own clause's subject
            let not: Option<String> = if reflexive || t.base(i) == Rel::Nsubj || t.w[i].rel == Rel::NmodPoss {
                None
            } else {
                let mut h = t.head(i);
                while let Some(x) = h {
                    if is_pred(t, x) {
                        break;
                    }
                    h = t.head(x);
                }
                h.and_then(|x| t.kids_of(x, Rel::Nsubj).first().copied()).map(|s| out.get(&s).cloned().unwrap_or_else(|| t.lemma(s).to_lowercase()))
            };
            ctx.resolve_in(&f, not.as_deref(), role_of(i)).map(|e| e.id)
        };
        if let Some(id) = id {
            let mut e = ctx.seen.get(&id).map(|s| s.ent.clone()).unwrap_or(Ent { id: id.clone(), text: id.clone(), ..Default::default() });
            e.pron = Some(f.clone());
            ctx.mention_as(&e, animate, role_of(i));
            out.insert(i, id);
        }
    }
    out
}

fn frames_of_tree_with(t: &Tree, ctx: &mut Ctx, question: bool, forced: BTreeMap<usize, String>) -> (Vec<Frame>, Vec<Asked>, Vec<(String, String)>, Vec<usize>) {
    let forced = if coref_mode() >= 3 { linear_pass(t, ctx, &forced) } else { forced };
    let n = t.len();
    let mut inq = vec![false; n];
    let mut open = false;
    for (i, q) in inq.iter_mut().enumerate() {
        if is_quote_mark(t.form(i)) {
            open = !open;
            continue;
        }
        *q = open;
    }
    let mut wh = BTreeSet::new();
    if question {
        // the first wh-word is the question's; a later one opens a clause («what does a dog do when it is pleased»)
        if let Some(i) = (0..n).find(|&i| WH.contains(&t.lemma(i)) && !matches!(t.base(i), Rel::Mark) && !(t.head(i).is_some_and(|h| t.base(h) == Rel::Acl) && i > 0 && matches!(t.upos(i - 1), UPos::NOUN | UPos::PROPN))) {
            wh.insert(i);
        }
    }
    ctx.ent_log.clear();
    ctx.anchors.clear();
    let mut r = Reader { t, ctx, inq, wh, not: None, depth: 0, forced, question };
    let preds: Vec<usize> = (0..n).filter(|&i| is_pred(t, i) && !matches!(t.base(i), Rel::Aux | Rel::Cop)).collect();
    let mut at: BTreeMap<usize, usize> = BTreeMap::new();
    let mut fs: Vec<Frame> = Vec::new();
    for &p in &preds {
        let mut f = r.frame(p);
        f.quoted = r.inq[p];
        if is_say(t.lemma(p)) {
            if let Some(a) = &f.agent {
                r.ctx.person(&a.id);
            }
        }
        at.insert(p, fs.len());
        fs.push(f);
    }
    // the predicate above a node (its nearest predicate ancestor)
    let up = |i: usize| -> Option<usize> {
        let mut x = t.head(i)?;
        for _ in 0..n {
            if at.contains_key(&x) {
                return Some(x);
            }
            x = t.head(x)?;
        }
        None
    };
    for &p in &preds {
        let fi = at[&p];
        // relative clauses: the noun they belong to fills the missing role
        let to_inf = t.kids_of(p, Rel::Mark).iter().any(|&k| t.lemma(k) == "to");
        if t.base(p) == Rel::Acl && to_inf && !off("acl-to") {
            // «snow to eat lichen», «food to eat»: an infinitive on a noun is a purpose of the clause above — its doer
            // is that clause's agent; the noun is what it acts on when it has no object of its own
            if let Some(m) = t.head(p).filter(|&m| matches!(t.upos(m), UPos::NOUN | UPos::PROPN | UPos::PRON)) {
                let above = up(m).map(|h| at[&h]);
                let ag = above.and_then(|h| fs[h].agent.clone()).filter(|a| !(a.id == "it" && a.pron.is_some()));
                let e = r.ent(m, false);
                let f = &mut fs[fi];
                if f.agent.is_none() && ag.is_some() {
                    f.agent = ag;
                    f.unsay(Slot::Agent);
                }
                if f.patient.is_none() && above.is_some_and(|h| fs[h].patient.as_ref().is_some_and(|x| e.as_ref().is_some_and(|e| x.id == e.id))) {
                    fs[fi].patient = e;
                    fs[fi].unsay(Slot::Patient);
                }
            }
        } else if t.base(p) == Rel::Acl {
            if let Some(m) = t.head(p).filter(|&m| matches!(t.upos(m), UPos::NOUN | UPos::PROPN)) {
                let e = r.ent(m, false);
                // (acl-own) a relative word in the clause stands for the noun — «things that are untrue», «the house
                // which I build», «the castle in whose hall», «the place where» —, so the noun fills no other role;
                // nor does it become the agent of a clause with a subject of its own
                let own = !off("acl-own");
                let rel_word = own && relative_word(t, p);
                let own_subj = own && t.kids(p).iter().any(|&k| t.base(k) == Rel::Nsubj && t.w[k].rel != Rel::NsubjPass);
                let f = &mut fs[fi];
                if rel_word {
                } else if (f.agent.is_none() && f.patient.is_some() || f.agent.is_none() && t.w[p].tag != PTag::VBN) && !own_subj {
                    f.agent = e;
                    f.unsay(Slot::Agent);
                } else if f.patient.is_none() {
                    f.patient = e;
                    f.unsay(Slot::Patient);
                }
            }
        }
        let Some(h) = up(p) else { continue };
        let hi = at[&h];
        let rel = t.base(p);
        if !off("frame-link") {
            // the noun the clause belongs to, when the predicate above is not its head
            let of = t.head(p).filter(|&x| x != h && matches!(t.upos(x), UPos::NOUN | UPos::PROPN | UPos::PRON)).map(|x| t.lemma(x).to_string());
            fs[fi].link = Some((hi, t.w[p].rel, of));
        }
        let marks: Vec<String> = t.kids(p).iter().filter(|&&k| t.base(k) == Rel::Mark).map(|&k| t.lemma(k).to_string()).collect();
        let advs: Vec<String> = t.kids(p).iter().filter(|&&k| matches!(t.base(k), Rel::Advmod | Rel::Cc)).map(|&k| t.lemma(k).to_string()).collect();
        let m = |w: &str| marks.iter().any(|x| x == w);
        let a = |w: &str| advs.iter().any(|x| x == w);
        // a clause without its own subject shares the head's (coordinated predicates, purposes, control)
        if fs[fi].agent.is_none() && matches!(rel, Rel::Conj | Rel::Xcomp | Rel::Advcl | Rel::Ccomp) && !t.kids(p).iter().any(|&k| t.base(k) == Rel::Nsubj) && !(rel == Rel::Ccomp && (is_say(t.lemma(h)) || CONTENT.contains(&t.lemma(h)))) {
            // object control: «make the mould grow», «tell Alice to go» — the doer is the head's object
            let ag = if rel == Rel::Xcomp && OBJ_CONTROL.contains(&t.lemma(h)) && fs[hi].patient.is_some() && !off("obj-control") { fs[hi].patient.clone() } else { fs[hi].agent.clone() };
            let ag = ag.filter(|a| !(a.id == "it" && a.pron.is_some()));
            if ag.is_some() {
                fs[fi].unsay(Slot::Agent);
            }
            fs[fi].agent = ag;
        }
        // «you never walk, swim or drive through flood water»: a predicate joined to a negated one, with no subject,
        // auxiliary or «but» of its own, is negated with it (PRAG_OFF=conj-neg)
        if rel == Rel::Conj && fs[hi].neg && !fs[fi].neg && !off("conj-neg")
            && !t.kids(p).iter().any(|&k| matches!(t.base(k), Rel::Nsubj | Rel::Aux | Rel::Cop) || t.base(k) == Rel::Cc && t.lemma(k) == "but")
            // «she doesn't sing and dances»: after «not» the shared verb is in its base form; a finite one stands alone
            && !(matches!(t.tag(p), PTag::VBZ | PTag::VBD) && t.kids(h).iter().any(|&k| matches!(t.lemma(k), "not" | "n't")))
        {
            fs[fi].neg = true;
        }
        let set_cause = |fs: &mut Vec<Frame>, of: usize, c: usize| {
            if fs[of].cause.is_none() && of != c {
                fs[of].cause = Some(c);
            }
        };
        let set_next = |fs: &mut Vec<Frame>, of: usize, nx: usize| {
            if fs[of].next.is_none() && of != nx {
                fs[of].next = Some(nx);
            }
        };
        if ["because", "since", "cos", "'cos", "cause", "'cause", "cuz"].iter().any(|x| m(x)) || m("as") && rel == Rel::Advcl {
            set_cause(&mut fs, hi, fi);
        } else if (m("so") || a("so") || a("therefore") || a("thus")) && !m("that") {
            // «…, so Alice puts it down»: the clause just before is the cause (the nearest predicate before «so»)
            let prev = if !off("so-nearest") {
                preds.iter().copied().filter(|&q| q < p && !fs[at[&q]].quoted && !matches!(t.base(q), Rel::Amod | Rel::Acl) && q != p).max().unwrap_or(h)
            } else if matches!(rel, Rel::Conj | Rel::Parataxis) {
                t.kids(h).iter().copied().filter(|&k| k < p && matches!(t.base(k), Rel::Conj | Rel::Parataxis | Rel::Obl | Rel::Advcl) && at.contains_key(&k)).max().unwrap_or(h)
            } else {
                h
            };
            let pi = at[&prev];
            set_cause(&mut fs, fi, pi);
            set_next(&mut fs, pi, fi);
        } else if m("that") && !off("so-that") && t.kids(h).iter().any(|&k| matches!(t.lemma(k), "so" | "such") && k < p) || m("that") && !off("so-that") && t.kids(h).iter().any(|&k| t.kids(k).iter().any(|&j| matches!(t.lemma(j), "so" | "such"))) {
            // «so hungry that they ate rats»: the result's cause is the clause with «so»
            set_cause(&mut fs, fi, hi);
        } else if m("if") || m("unless") {
            fs[fi].cond = true;
        } else if m("to") && (rel == Rel::Advcl || rel == Rel::Xcomp && PURPOSE_HEADS.contains(&t.lemma(h)) || !off("purpose-wide") && rel == Rel::Xcomp && !CONTROL.contains(&t.lemma(h)) && !OBJ_CONTROL.contains(&t.lemma(h)) && !is_say(t.lemma(h)) || !off("purpose-acl") && rel == Rel::Acl) {
            // a purpose is the reason why
            set_cause(&mut fs, hi, fi);
        } else if !off("by-means") && rel == Rel::Advcl && (m("by") || t.kids(p).iter().any(|&k| t.lemma(k) == "by" && t.base(k) == Rel::Case)) {
            // «re-zip them by preening»: the means is the reason it works
            set_cause(&mut fs, hi, fi);
        } else if m("after") || m("once") || m("when") || m("whenever") {
            set_next(&mut fs, fi, hi);
        } else if m("before") || m("until") || m("till") {
            set_next(&mut fs, hi, fi);
        } else if matches!(rel, Rel::Ccomp | Rel::Csubj) && (is_say(t.lemma(h)) || CONTENT.contains(&t.lemma(h)) || fs[fi].quoted) || (rel == Rel::Parataxis && (fs[fi].quoted != fs[hi].quoted)) {
            // what was said or thought
            fs[hi].quote.push(fi);
            if is_say(t.lemma(h)) {
                fs[hi].kind = Kind::Say(act_of(t.lemma(h)));
            }
        } else if matches!(rel, Rel::Conj | Rel::Parataxis) || a("then") {
            // «and» = in order (the maxim of manner): the previous coordinated predicate comes first
            let prev = t.kids(h).iter().copied().filter(|&k| k < p && t.base(k) == Rel::Conj && at.contains_key(&k)).max().unwrap_or(h);
            let pi = at[&prev];
            set_next(&mut fs, pi, fi);
        }
    }
    // factuality (rudinger-2018 I1, white-2016 I1): a «to» complement takes its status from the verb above it and that
    // verb's polarity — «managed to eat» ate, «did not manage to eat» did not (neg flips), «wanted to eat» is not said
    // to have happened (irrealis). Top-down, so «did not try to manage to eat» composes. PRAG_OFF=factuality.
    if !off("factuality") {
        let depth = |mut i: usize| {
            let mut d = 0;
            while let Some(h) = t.head(i) {
                d += 1;
                i = h;
                if d > n {
                    break;
                }
            }
            d
        };
        let mut order = preds.clone();
        order.sort_by_key(|&p| (depth(p), p));
        for &p in &order {
            let Some(h) = up(p) else { continue };
            let to = t.kids_of(p, Rel::Mark).iter().any(|&k| t.lemma(k) == "to");
            if !(t.base(p) == Rel::Xcomp || t.base(p) == Rel::Ccomp && to) || t.head(p) != Some(h) {
                continue;
            }
            let Some((aff, neg)) = signature(t.lemma(h)) else { continue };
            let (hi, fi) = (at[&h], at[&p]);
            if fs[hi].irrealis {
                fs[fi].irrealis = true;
                continue;
            }
            match if fs[hi].neg { neg } else { aff } {
                b'+' => {}
                b'-' => fs[fi].neg = !fs[fi].neg,
                _ => fs[fi].irrealis = true,
            }
        }
    }
    // an «if» clause is not said to have happened either (ning-2020 I3); PRAG_OFF=cond-irrealis
    if !off("cond-irrealis") {
        for f in fs.iter_mut() {
            if f.cond {
                f.irrealis = true;
            }
        }
    }
    // quoted frames not yet carried: the nearest unquoted say-frame above them, else in the sentence
    let carried: BTreeSet<usize> = fs.iter().flat_map(|f| f.quote.iter().copied()).collect();
    for &p in &preds {
        let fi = at[&p];
        if !fs[fi].quoted || carried.contains(&fi) {
            continue;
        }
        // only the top of a quoted clause: its predicate parent is not quoted, or it has none
        if up(p).is_some_and(|h| fs[at[&h]].quoted) {
            continue;
        }
        // a quotation of the book's words inside the narration («the glass shone "a hot ruby"») is not speech: only a
        // verb of saying or knowing carries a quote
        let speechy = |h: &usize| off("quote-say-only") || is_say(t.lemma(*h)) || CONTENT.contains(&t.lemma(*h));
        let sayer = up(p).filter(|h| !fs[at[h]].quoted && speechy(h)).or_else(|| preds.iter().copied().filter(|&q| !fs[at[&q]].quoted && is_say(t.lemma(q))).min_by_key(|&q| q.abs_diff(p)));
        if let Some(s) = sayer {
            let si = at[&s];
            fs[si].quote.push(fi);
            if is_say(t.lemma(s)) {
                fs[si].kind = Kind::Say(act_of(t.lemma(s)));
            }
        }
    }
    // quoted speech: «I» is the speaker, «you» the one spoken to (the Say frame that carries a frame of the quote)
    if !off("quote-person") {
        let mut span = vec![0usize; n];
        let (mut open, mut k) = (false, 0);
        for i in 0..n {
            if is_quote_mark(t.form(i)) {
                open = !open;
                if open {
                    k += 1;
                }
                continue;
            }
            span[i] = if open { k } else { 0 };
        }
        let pred_of: BTreeMap<usize, usize> = at.iter().map(|(p, f)| (*f, *p)).collect();
        for q in 1..=k {
            let inside: Vec<usize> = (0..fs.len()).filter(|f| pred_of.get(f).is_some_and(|&p| span[p] == q)).collect();
            let real = |a: &Option<Ent>| a.clone().filter(|a| !UNRESOLVED.contains(&a.id.as_str()) && !PERSONAL.contains(&a.id.as_str()));
            let sayer = (0..fs.len()).find(|&si| matches!(fs[si].kind, Kind::Say(_)) && fs[si].quote.iter().any(|x| inside.contains(x)));
            let (who, to) = if let Some(sayer) = sayer {
                // the act by what is said: a question asks, an exclamation expresses (for the plain verbs of saying)
                if !off("act-by-quote") && fs[sayer].kind == Kind::Say(Act::Assert) && matches!(fs[sayer].verb.as_str(), "say" | "answer" | "reply" | "add" | "call" | "call out" | "whisper" | "mutter" | "remark") {
                    let toks: Vec<&str> = (0..n).filter(|&i| span[i] == q).map(|i| t.form(i)).collect();
                    if toks.contains(&"?") {
                        fs[sayer].kind = Kind::Say(Act::Ask);
                    } else if toks.contains(&"!") {
                        fs[sayer].kind = Kind::Say(Act::Express);
                    }
                }
                (real(&fs[sayer].agent), real(&fs[sayer].recipient))
            } else {
                // no verb of saying carries it: the last speaker of the discourse goes on (a noun before a colon as the
                // speaker was tried: no change on the bench)
                if !off("quote-last-speaker") {
                    (r.ctx.last_speaker.clone(), None)
                } else {
                    (None, None)
                }
            };
            for &fi in &inside {
                let f = &mut fs[fi];
                let swap = |e: &mut Ent| {
                    let (src, form) = match e.id.as_str() {
                        "i" => (&who, e.text.to_lowercase()),
                        "you" => (&to, e.text.to_lowercase()),
                        _ => return,
                    };
                    if let Some(x) = src {
                        *e = Ent { pron: Some(form), ..x.clone() };
                    }
                };
                for s in [&mut f.agent, &mut f.patient, &mut f.recipient, &mut f.place] {
                    if let Some(e) = s.as_mut() {
                        swap(e);
                    }
                }
                for (_, e) in f.obls.iter_mut() {
                    swap(e);
                }
            }
        }
    }
    for f in &fs {
        if matches!(f.kind, Kind::Say(_)) && !f.quoted {
            if let Some(a) = f.agent.clone().filter(|a| !UNRESOLVED.contains(&a.id.as_str()) && !PERSONAL.contains(&a.id.as_str())) {
                r.ctx.last_speaker = Some(Ent { pron: None, ..a });
            }
        }
    }
    // nominalisations with an argument: the events nouns name (WordNet's derivational links; PRAG_OFF=vn-nom)
    let mut noms: Vec<usize> = Vec::new();
    if !off("vn-nom") {
        for k in 0..n {
            if t.upos(k) == UPos::NOUN && !at.contains_key(&k) {
                if let Some(f) = r.nominal(k) {
                    let mut f = f;
                    f.quoted = r.inq[k];
                    fs.push(f);
                    noms.push(k);
                }
            }
        }
    }
    // a root that is no predicate (a heading, a title, a noun phrase): its own bare frame, after the predicates'
    // frames and before the nouns' events; the clauses under it that hang on no predicate link to it
    if let Some(root) = t.root().filter(|r| !at.contains_key(r) && !off("frame-fragment")) {
        if let Some(mut f) = r.fragment(root) {
            let fi = preds.len();
            f.quoted = r.inq[root];
            for &p in &preds {
                if up(p).is_none() && !off("frame-link") {
                    let of = t.head(p).filter(|&x| x != root && matches!(t.upos(x), UPos::NOUN | UPos::PROPN | UPos::PRON)).map(|x| t.lemma(x).to_string());
                    fs[at[&p]].link = Some((fi, t.w[p].rel, of));
                }
            }
            fs.insert(fi, f);
            noms.insert(0, root);
        }
    }
    let mut asked: Vec<Asked> = Vec::new();
    for &w in &r.wh {
        let Some(p) = (if at.contains_key(&w) { Some(w) } else { up(w) }) else { continue };
        let mut a = Asked { word: t.lemma(w).to_string(), frame: at[&p], what: ask_of(t, w, p) };
        if let Some(root) = t.root().filter(|r| at.contains_key(r)) {
            // «What does a bee do when …»: «what» with the main verb «do» asks the action, whatever the parser
            // hung «what» on
            if !off("q-what-do") && t.lemma(w) == "what" && t.lemma(root) == "do" && root != p && fs[at[&root]].patient.is_none() {
                a = Asked { word: a.word, frame: at[&root], what: Ask::Role(Slot::Verb) };
            }
            // «What happens when / would happen if X»: what follows X
            if !off("q-happen") && t.lemma(a.frame_pred(&at)) == "happen" && matches!(a.what, Ask::Frame | Ask::Next) {
                let hf = a.frame;
                let pred_of = |j: usize| at.iter().find(|(_, f)| **f == j).map(|(p, _)| *p);
                let clause = |j: usize| {
                    let Some(pj) = pred_of(j) else { return false };
                    up(pj).is_some_and(|h| at[&h] == hf) && matches!(t.base(pj), Rel::Advcl | Rel::Ccomp) && t.kids(pj).iter().any(|&k| matches!(t.lemma(k), "when" | "if" | "whenever" | "once"))
                };
                if let Some(x) = (0..fs.len()).find(|&j| j != hf && (fs[j].next == Some(hf) || clause(j))) {
                    if fs[x].next == Some(hf) {
                        fs[x].next = None;
                    }
                    a = Asked { word: a.word, frame: x, what: Ask::Next };
                }
            }
        }
        // (PRAG_VN=goal-obl) «what does a tadpole turn into?»: a Goal outside space or a Result is asked as the oblique the
        // statements keep it as, not as a place
        if a.what == Ask::Role(Slot::Place) && vn_opt("goal-obl") {
            let f = &fs[a.frame];
            let key = t.kids_of(w, Rel::Case).first().map(|&c| format!("{} {}", t.lemma(c).to_lowercase(), t.lemma(w)));
            if key.is_some_and(|k| f.how.roles.iter().any(|(x, r)| *x == k && (r == "Result" || r == "Goal" && !spatial_class(&f.how.class)))) {
                a.what = Ask::Role(Slot::Obl);
            }
        }
        asked.push(a);
    }
    // a yes/no question: an auxiliary first, no wh-word
    if question && asked.is_empty() {
        if let (Some(r0), Some(first)) = (t.root(), (0..n).find(|&i| t.upos(i) != UPos::PUNCT)) {
            if matches!(t.upos(first), UPos::AUX) || matches!(t.lemma(first), "do" | "be" | "have" | "can" | "will" | "would" | "should" | "could") {
                if let Some(&fi) = at.get(&r0) {
                    asked.push(Asked { word: t.lemma(first).to_string(), frame: fi, what: Ask::Frame });
                }
            }
        }
    }
    // «When did the founding of X take place?», «Where did the opening of X happen?»: an event predicate says only that
    // its subject's event took place; when the subject is a noun the reader read as an event (its nominal frame), the
    // time or place asked is that event's (PRAG_OFF=wh-event-noun)
    if question && !off("wh-event-noun") {
        let toks: Vec<usize> = preds.iter().chain(noms.iter()).copied().collect();
        for a in asked.iter_mut() {
            if !matches!(a.what, Ask::Role(Slot::Attr) | Ask::Role(Slot::Place)) {
                continue;
            }
            let Some(&p) = toks.get(a.frame) else { continue };
            if p >= n {
                continue;
            }
            let event_pred = matches!(t.lemma(p), "happen" | "occur") || t.lemma(p) == "take" && t.kids(p).iter().any(|&k| t.base(k) == Rel::Obj && t.lemma(k) == "place" && t.kids_of(k, Rel::Det).is_empty());
            if !event_pred {
                continue;
            }
            let Some(s) = t.kids(p).iter().copied().find(|&k| t.base(k) == Rel::Nsubj) else { continue };
            if let Some(fi) = noms.iter().position(|&k| k == s).map(|j| preds.len() + j).filter(|&fi| fi < fs.len() && fs[fi].how.by == By::Nominal) {
                a.frame = fi;
            }
        }
    }
    // names given in apposition («a bough (a branch)»): the same thing
    // («Isaac said to his father, "Where is the lamb?"»: a noun of the quote is not another name of one outside it)
    let aliases: Vec<(String, String)> = (0..n).filter(|&i| t.base(i) == Rel::Appos && matches!(t.upos(i), UPos::NOUN | UPos::PROPN)).filter_map(|i| t.head(i).filter(|&h| matches!(t.upos(h), UPos::NOUN | UPos::PROPN) && (r.inq[h] == r.inq[i] || off("appos-quote"))).map(|h| (t.lemma(h).to_string(), t.lemma(i).to_string()))).collect();
    if !off("anchors") {
        let toks: Vec<usize> = preds.iter().chain(noms.iter()).copied().collect();
        let log = std::mem::take(&mut r.ctx.ent_log);
        r.ctx.anchors = fs.iter().enumerate().map(|(i, f)| anchor_of(t, f, toks.get(i).copied(), &toks, &log)).collect();
    }
    (fs, asked, aliases, noms)
}

/// The anchor of one frame: its predicate token, the frame above (by `link`), and for each role said in the clause
/// the head token of the mention read with the same id and phrase, the nearest to the predicate in the tree.
fn anchor_of(t: &Tree, f: &Frame, pred: Option<usize>, toks: &[usize], log: &[(usize, String, String)]) -> Anchor {
    let mut a = Anchor { pred, link: f.link.as_ref().and_then(|(hi, rel, _)| toks.get(*hi).map(|&h| (h, *rel))), roles: Vec::new() };
    let Some(p) = pred else { return a };
    let depth_to = |mut x: usize| -> usize {
        for d in 0..t.len() {
            if x == p {
                return d;
            }
            match t.head(x) {
                Some(h) => x = h,
                None => break,
            }
        }
        usize::MAX
    };
    let mut slots: Vec<(Slot, &Ent)> = Vec::new();
    for (s, e) in [(Slot::Agent, &f.agent), (Slot::Patient, &f.patient), (Slot::Recipient, &f.recipient), (Slot::Place, &f.place)] {
        if let Some(e) = e {
            if !f.unsaid.contains(&s) {
                slots.push((s, e));
            }
        }
    }
    slots.extend(f.obls.iter().map(|(_, e)| (Slot::Obl, e)));
    for (s, e) in slots {
        let best = log.iter().filter(|(n, id, text)| *id == e.id && *text == e.text && *n != p).min_by_key(|(n, _, _)| (depth_to(*n), n.abs_diff(p), *n));
        if let Some((n, _, _)) = best {
            a.roles.push((s, *n));
        }
    }
    a
}

/// What a question asks: the cause, what came next, a role of its frame, or the frame itself (yes/no, «put him
/// right», «what happened when …»).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ask {
    Cause,
    Next,
    Role(Slot),
    Frame,
}

impl Ask {
    pub fn name(self) -> String {
        match self {
            Ask::Cause => "why → the cause".into(),
            Ask::Next => "what next → the next frame".into(),
            Ask::Role(s) => format!("the {} of the frame", format!("{s:?}").to_lowercase()),
            Ask::Frame => "the frame itself".into(),
        }
    }
}

/// An interrogative word: what it asks and the frame it sits in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asked {
    pub word: String,
    pub frame: usize,
    pub what: Ask,
}

impl Asked {
    /// The predicate token of the asked frame (`at`: predicate token → frame).
    fn frame_pred(&self, at: &BTreeMap<usize, usize>) -> usize {
        at.iter().find(|(_, f)| **f == self.frame).map(|(p, _)| *p).unwrap_or(0)
    }
}

/// The role a wh-word asks, from its place in the tree (`w` the word, `p` its predicate).
fn ask_of(t: &Tree, w: usize, p: usize) -> Ask {
    let l = t.lemma(w);
    let after: Vec<&str> = (w + 1..t.len()).map(|i| t.lemma(i)).collect();
    let role_of = |n: usize| -> Slot {
        match t.base(n) {
            Rel::Nsubj if t.w[n].rel == Rel::NsubjPass => Slot::Patient,
            Rel::Nsubj => Slot::Agent,
            Rel::Obj if TELL.contains(&t.lemma(p)) && matches!(t.lemma(n), "who" | "whom") && !off("tell-to") => Slot::Recipient,
            Rel::Obj => Slot::Patient,
            Rel::Iobj => Slot::Recipient,
            Rel::Obl | Rel::Nmod => {
                let c = t.kids_of(n, Rel::Case).first().map(|&c| t.lemma(c).to_string()).unwrap_or_default();
                if c == "to" { Slot::Recipient } else if PLACE_PREPS.contains(&c.as_str()) { Slot::Place } else { Slot::Obl }
            }
            _ => Slot::Patient,
        }
    };
    match l {
        "why" => Ask::Cause,
        "where" => Ask::Role(Slot::Place),
        // (PRAG_OFF=wh-time) «when» asks the time, which the reader keeps among the attributes as «when:…»
        "when" if !off("wh-time") => Ask::Role(Slot::Attr),
        "when" => Ask::Frame,
        "how" => if matches!(after.first(), Some(&"many") | Some(&"much")) { Ask::Role(Slot::Patient) } else { Ask::Role(Slot::Attr) },
        _ => {
            if t.lemma(p) == "happen" {
                return if after.iter().any(|x| matches!(*x, "next" | "then" | "after" | "afterwards" | "later")) { Ask::Next } else { Ask::Frame };
            }
            if t.base(w) == Rel::Det {
                return t.head(w).map(|n| if n == p { Ask::Role(Slot::Patient) } else { Ask::Role(role_of(n)) }).unwrap_or(Ask::Frame);
            }
            if w == p {
                // «what is a bough?»: the wh-word is the predicate
                return Ask::Role(Slot::Patient);
            }
            match t.base(w) {
                Rel::Obj if t.lemma(p) == "do" => Ask::Role(Slot::Verb),
                Rel::Nsubj if !t.kids_of(p, Rel::Cop).is_empty() => Ask::Frame,
                _ => Ask::Role(role_of(w)),
            }
        }
    }
}

/// A bare phrase (an option with no predicate): its head entity, or its adjectives and adverbs.
pub fn bare(t: &Tree, ctx: &mut Ctx) -> Option<Frame> {
    let root = t.root()?;
    let mut r = Reader { t, ctx, inq: vec![false; t.len()], wh: BTreeSet::new(), not: None, depth: 0, forced: BTreeMap::new(), question: false };
    r.bare_at(root)
}

impl Reader<'_> {
    /// The bare frame of the phrase headed by `root` (`bare`).
    fn bare_at(&mut self, root: usize) -> Option<Frame> {
        let t = self.t;
        let r = self;
        let mut f = Frame::new(Kind::Bare, String::new());
        match t.upos(root) {
            UPos::ADJ | UPos::ADV => {
                f.attrs = (0..t.len()).filter(|&i| matches!(t.upos(i), UPos::ADJ | UPos::ADV) && !NOT_MANNER.contains(&t.lemma(i))).map(|i| t.lemma(i).to_string()).collect();
                if f.attrs.is_empty() {
                    f.attrs.push(t.lemma(root).to_string());
                }
                // «Slowly, tail first»: the nouns too
                for i in 0..t.len() {
                    if i != root && matches!(t.upos(i), UPos::NOUN) && t.head(i) == Some(root) {
                        if let Some(e) = r.ent(i, false) {
                            f.obls.push((String::new(), e));
                        }
                    }
                }
            }
            UPos::VERB => {
                // a gerund or a bare verb with no frame made («Purring», «Growls»)
                f.verb = t.lemma(root).to_string();
            }
            _ => {
                let e = r.ent(root, false)?;
                let case = t.kids_of(root, Rel::Case).first().map(|&c| t.lemma(c).to_string()).unwrap_or_default();
                if PLACE_PREPS.contains(&case.as_str()) && placeable(&e) {
                    f.place = Some(e);
                } else if !case.is_empty() {
                    f.obls.push((case.clone(), e));
                } else {
                    f.patient = Some(e);
                }
                // «A Hatter and a March Hare», «the chimneys shaped like ears»
                f.attrs = (0..t.len()).filter(|&i| t.upos(i) == UPos::ADJ && t.head(i) == Some(root)).map(|i| t.lemma(i).to_string()).collect();
                // (PRAG_OFF=wh-time) a time as a phrase: its whole value («March 2022» → when:march 2022), or the event a
                // time preposition puts («during the war» → when:war)
                if !off("wh-time") {
                    if let Some(v) = time_value(t, root).into_iter().next() {
                        f.attrs.push(format!("when:{v}"));
                    } else if TIME_PREPS.contains(&case.as_str()) {
                        f.attrs.push(format!("when:{}", t.lemma(root).to_lowercase()));
                    }
                }
            }
        }
        f.neg = (0..t.len()).any(|i| matches!(t.lemma(i), "not" | "never" | "n't" | "no"));
        Some(f)
    }

    /// A sentence whose root is no predicate — a heading, a title, a list item, a noun phrase («The English
    /// dictionary», «Mova · this page in HTML»): one bare frame for the root and what hangs on it. Its role fields are
    /// `bare`'s (the root's entity as the patient, the place or an oblique; for an adjective or an adverb the
    /// attributes); besides, a root that is no entity is the frame's word with its relation (an adjective, an adverb,
    /// an interjection, a symbol), and the root's dependents the entity does not carry are the frame's words (one
    /// word) and noun phrases. The clauses under the root link to it (`frames_of_tree_with`). PRAG_OFF=frame-fragment.
    fn fragment(&mut self, root: usize) -> Option<Frame> {
        let t = self.t;
        if t.upos(root) == UPos::PUNCT || self.wh.contains(&root) {
            return None;
        }
        let adj = matches!(t.upos(root), UPos::ADJ | UPos::ADV);
        let mut f = self.bare_at(root).unwrap_or_else(|| Frame::new(Kind::Bare, String::new()));
        let ent = !adj && (f.patient.is_some() || f.place.is_some() || !f.obls.is_empty());
        if !ent {
            f.words.push((t.w[root].rel, t.lemma(root).to_string(), t.tag(root)));
        }
        for k in t.kids(root).to_vec() {
            let r = t.w[k].rel;
            if is_pred(t, k) || r.base() == Rel::Punct || self.wh.contains(&k) {
                continue;
            }
            // what the root's entity says itself (`ent_at`, `ent_more`)
            let carried = ent
                && (!off("ent-rest") && !off("ent-mods") || match r.base() {
                    Rel::Det | Rel::Amod | Rel::Case | Rel::Appos | Rel::Advmod | Rel::Discourse => true,
                    Rel::Nmod => true,
                    Rel::Compound => true,
                    Rel::Flat => t.upos(k) != UPos::NOUN,
                    Rel::Conj => matches!(t.upos(k), UPos::NOUN | UPos::PROPN),
                    _ => false,
                });
            // (the nouns of an adjective or adverb root are `bare`'s obliques)
            if carried || adj && t.upos(k) == UPos::NOUN {
                continue;
            }
            if matches!(t.upos(k), UPos::NOUN | UPos::PROPN | UPos::PRON | UPos::NUM) && !t.kids(k).is_empty() || matches!(t.upos(k), UPos::NOUN | UPos::PROPN) {
                if let Some(e) = self.ent_quiet(k) {
                    f.nps.push((r, e));
                    continue;
                }
            }
            if t.kids(k).iter().all(|&j| t.base(j) == Rel::Punct) || !off("ent-rest") {
                f.words.push((r, t.lemma(k).to_string(), t.tag(k)));
                if !off("ent-rest") {
                    let mut ws = Vec::new();
                    self.flat_words(k, &mut ws);
                    f.words.extend(ws);
                }
            }
        }
        Some(f)
    }
}

/// «What did the cook throw?»: the tagger, trained on statements, takes «did» for the verb and «cook throw» for a
/// compound noun. When «do» heads an object noun that can be a verb and has a noun compound before it, the tree is
/// set right: the noun is the verb, its compound the subject, «do» its auxiliary, the wh-word its object.
pub fn repair_do_question(w: &mut [en::annotate::Word]) -> bool {
    let n = w.len();
    let can_be_verb = |f: &str| en::dict::form(f).is_some_and(|x| en::dict::analyses(x).any(|r| r.tag == PTag::VB));
    let Some(d) = (0..n).find(|&i| w[i].lemma == "do" && w[i].upos == UPos::VERB) else { return false };
    let Some(v) = (d + 1..n).find(|&k| w[k].head == d + 1 && w[k].rel == Rel::Obj && w[k].upos == UPos::NOUN && can_be_verb(&w[k].form)) else { return false };
    let comps: Vec<usize> = (0..n).filter(|&k| w[k].head == v + 1 && w[k].rel == Rel::Compound && k < v).collect();
    let Some(&subj) = comps.last() else { return false };
    let (dh, dr) = (w[d].head, w[d].rel);
    w[v].upos = UPos::VERB;
    w[v].tag = PTag::VB;
    w[v].lemma = w[v].form.clone();
    w[v].head = dh;
    w[v].rel = dr;
    w[d].upos = UPos::AUX;
    w[d].rel = Rel::Aux;
    w[d].head = v + 1;
    w[subj].rel = if matches!(w[subj].upos, UPos::PROPN) || w[subj].upos == UPos::NOUN { Rel::Nsubj } else { Rel::Nsubj };
    w[subj].head = v + 1;
    for k in 0..n {
        if k == subj || k == v || k == d {
            continue;
        }
        if w[k].head == v + 1 && k < subj {
            // the determiner and the first words of the subject («the March Hare»)
            w[k].head = subj + 1;
        } else if w[k].head == v + 1 && w[k].rel == Rel::Compound {
            w[k].head = subj + 1;
        } else if w[k].head == d + 1 {
            w[k].head = v + 1;
            if WH.contains(&w[k].lemma.as_str()) && matches!(w[k].lemma.as_str(), "what" | "who" | "whom" | "which") && w[k].rel == Rel::Nsubj {
                w[k].rel = Rel::Obj;
            }
        }
    }
    true
}

/// «Pollution is harmful material put into air»: the parser, expecting a passive, takes «is» for its auxiliary and
/// gives the participle two subjects. Two subjects on one verb with a «be» between them is the copula reading: the
/// second noun is the predicate, «be» its copula, the first noun its subject, the participle describes it.
pub fn repair_double_subject(w: &mut [en::annotate::Word]) -> bool {
    let n = w.len();
    let subj = |r: Rel| r.base() == Rel::Nsubj;
    for v in 0..n {
        if w[v].upos != UPos::VERB {
            continue;
        }
        let ss: Vec<usize> = (0..v).filter(|&k| w[k].head == v + 1 && subj(w[k].rel)).collect();
        if ss.len() != 2 {
            continue;
        }
        let (a, n2) = (ss[0], ss[1]);
        let Some(b) = (a + 1..n2).find(|&k| w[k].head == v + 1 && w[k].lemma == "be") else { continue };
        if !matches!(w[n2].upos, UPos::NOUN | UPos::PROPN) {
            continue;
        }
        let (vh, vr) = (w[v].head, w[v].rel);
        w[n2].head = vh;
        w[n2].rel = vr;
        w[b].head = n2 + 1;
        w[b].rel = Rel::Cop;
        w[b].upos = UPos::AUX;
        w[a].head = n2 + 1;
        w[a].rel = Rel::Nsubj;
        w[v].head = n2 + 1;
        w[v].rel = Rel::Acl;
        // the sentence's punctuation and whatever hung on the verb before the noun go to the new predicate
        for k in 0..n {
            if k != v && w[k].head == v + 1 && (k < n2 || w[k].upos == UPos::PUNCT && k == n - 1) {
                w[k].head = n2 + 1;
            }
            if vh == 0 && w[k].head == 0 && k != n2 {
                w[k].head = n2 + 1;
            }
        }
        return true;
    }
    false
}

/// «Isaac asked, "Where is the lamb?"»: inside a longer sentence the parser takes the noun for the predicate, with
/// «where» its adverb and no subject. A noun with a copula, a «where» / «how» before the copula and no subject is set
/// right as the question alone reads: the wh-word is the predicate, the noun its subject.
pub fn repair_wh_copula(w: &mut [en::annotate::Word]) -> bool {
    let n = w.len();
    for p in 0..n {
        if !matches!(w[p].upos, UPos::NOUN | UPos::PROPN) {
            continue;
        }
        let kid = |k: usize| w[k].head == p + 1;
        let Some(c) = (0..p).find(|&k| kid(k) && w[k].rel == Rel::Cop) else { continue };
        let Some(h) = (0..c).find(|&k| kid(k) && w[k].rel.base() == Rel::Advmod && matches!(w[k].lemma.as_str(), "where" | "how")) else { continue };
        if (0..n).any(|k| kid(k) && w[k].rel.base() == Rel::Nsubj) {
            continue;
        }
        let (ph, pr) = (w[p].head, w[p].rel);
        w[h].head = ph;
        w[h].rel = pr;
        w[c].head = h + 1;
        w[p].head = h + 1;
        w[p].rel = Rel::Nsubj;
        return true;
    }
    false
}

/// «When was the college founded by John Smith?»: the parser takes «when» for the predicate, «was» for its copula and
/// the participle for a modifier of the subject, so the reader asks the time of «the college is». A when / where
/// predicate with a copula «be», whose subject has a past participle after it, is the passive question it reads as:
/// the participle is the predicate, «be» its passive auxiliary, the noun its passive subject, the wh-word its adverb
/// (questions only; PRAG_OFF=wh-passive).
pub fn repair_wh_passive(w: &mut [en::annotate::Word]) -> bool {
    let n = w.len();
    for h in 0..n {
        if !matches!(w[h].lemma.as_str(), "when" | "where") || w[h].head != 0 {
            continue;
        }
        let kid = |k: usize, x: usize| w[k].head == x + 1;
        let Some(c) = (h + 1..n).find(|&k| kid(k, h) && w[k].rel == Rel::Cop && w[k].lemma == "be") else { continue };
        let Some(s) = (c + 1..n).find(|&k| kid(k, h) && w[k].rel.base() == Rel::Nsubj && matches!(w[k].upos, UPos::NOUN | UPos::PROPN)) else { continue };
        let Some(v) = (s + 1..n).find(|&k| kid(k, s) && w[k].rel.base() == Rel::Acl && w[k].tag == PTag::VBN) else { continue };
        let (hh, hr) = (w[h].head, w[h].rel);
        let others: Vec<usize> = (0..n).filter(|&k| k != c && k != s && kid(k, h)).collect();
        for k in others {
            w[k].head = v + 1;
        }
        w[v].head = hh;
        w[v].rel = hr;
        w[c].head = v + 1;
        w[c].rel = Rel::AuxPass;
        w[c].upos = UPos::AUX;
        w[s].head = v + 1;
        w[s].rel = Rel::NsubjPass;
        w[h].head = v + 1;
        w[h].rel = Rel::Advmod;
        return true;
    }
    // «When was the college founded?» read as «the college» with a copula and «when», itself the passive subject of
    // «founded»: the copula is the participle's passive auxiliary, the wh-word its adverb
    for s in 0..n {
        let v = w[s].head.wrapping_sub(1);
        if !matches!(w[s].upos, UPos::NOUN | UPos::PROPN) || w[s].rel.base() != Rel::Nsubj || v >= n || w[v].tag != PTag::VBN {
            continue;
        }
        let Some(c) = (0..s).find(|&k| w[k].head == s + 1 && w[k].rel == Rel::Cop && w[k].lemma == "be") else { continue };
        let Some(h) = (0..c).find(|&k| w[k].head == s + 1 && matches!(w[k].lemma.as_str(), "when" | "where")) else { continue };
        w[c].head = v + 1;
        w[c].rel = Rel::AuxPass;
        w[c].upos = UPos::AUX;
        w[h].head = v + 1;
        w[h].rel = Rel::Advmod;
        w[s].rel = Rel::NsubjPass;
        return true;
    }
    false
}

/// A verb form with the tense of the auxiliary «do» it loses («did … throw» → threw, «does … shine» → shines).
fn inflect(base: &str, aux: &str) -> String {
    let want = match aux {
        "did" => PTag::VBD,
        "does" => PTag::VBZ,
        _ => return base.to_string(),
    };
    en::dict::lemma(base).and_then(|l| en::dict::paradigm(l).find(|r| r.tag == want).map(|r| en::dict::form_text(r.form()).to_string())).unwrap_or_else(|| if want == PTag::VBZ { format!("{base}s") } else { format!("{base}ed") })
}

/// «What did the cook throw after the Duchess?» → «the cook threw what after the Duchess?»: a question with
/// do-support put in statement order, which the parser (trained mostly on statements) reads right. The wh-word of
/// the object goes after the verb; «why / how / where / when» stay in front. None when the question has no such
/// shape.
pub fn statement_order(sentence: &str) -> Option<String> {
    let toks: Vec<String> = sentence.trim_end_matches(['?', ' ']).split_whitespace().map(String::from).collect();
    let low: Vec<String> = toks.iter().map(|t| t.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_string()).collect();
    // the wh-phrase at the start: «what», «which way», «how many cakes»
    let w0 = low.iter().position(|t| WH.contains(&t.as_str()))?;
    if w0 > 3 {
        return None;
    }
    if !off("q-strand") {
        if let Some(s) = stranded_order(&toks, &low) {
            return Some(s);
        }
    }
    // «What should you do if …»: a modal first, «do» is the main verb → «you should do what if …»
    const MODALS: [&str; 9] = ["should", "would", "could", "can", "will", "must", "might", "may", "shall"];
    if !off("q-modal") {
        if let Some(m) = (w0 + 1..low.len().min(w0 + 4)).find(|&i| MODALS.contains(&low[i].as_str())) {
            if let Some(d) = (m + 2..low.len().min(m + 6)).find(|&i| low[i] == "do") {
                let subj = &toks[m + 1..d];
                if !subj.is_empty() && !matches!(low[w0].as_str(), "why" | "how" | "where" | "when") && !subj.iter().any(|x| x.ends_with(',')) {
                    let mut out: Vec<String> = toks[..w0].to_vec();
                    out.extend(subj.iter().cloned());
                    out.push(toks[m].to_lowercase());
                    out.push("do".into());
                    out.extend(toks[w0..m].iter().cloned());
                    out.extend(toks[d + 1..].iter().cloned());
                    return Some(format!("{}?", out.join(" ")));
                }
            }
            return None;
        }
    }
    let d = (w0 + 1..low.len().min(w0 + 5)).find(|&i| matches!(low[i].as_str(), "do" | "does" | "did"))?;
    // the words between the wh-word and «do» belong to the wh-phrase («which way», «how many cakes»)
    if (w0 + 1..d).any(|i| toks[i].ends_with(',')) {
        return None;
    }
    const DETS: [&str; 13] = ["the", "a", "an", "this", "that", "his", "her", "its", "their", "my", "your", "our", "these"];
    let can_be_verb = |f: &str| en::dict::form(f).is_some_and(|x| en::dict::analyses(x).any(|r| r.tag == PTag::VB));
    let free = |i: usize| can_be_verb(&low[i]) && !DETS.contains(&low[i - 1].as_str()) && !toks[i].chars().next().is_some_and(char::is_uppercase) && !low[i - 1].ends_with("'s");
    // a preposition is never the main verb, whatever form the dictionary also lists for it («What did the population
    // of the valley grow?»: not «of»; PRAG_OFF=q-do-prep)
    const PREPS: [&str; 27] = ["of", "in", "on", "at", "by", "for", "from", "with", "into", "onto", "upon", "about", "over", "under", "after", "before", "through", "during", "within", "without", "between", "among", "across", "against", "toward", "towards", "near"];
    let prep = |i: usize| !off("q-do-prep") && PREPS.contains(&low[i].as_str());
    let v = (d + 2..low.len()).find(|&i| free(i) && !prep(i))
        .or_else(|| (d + 1 < low.len() && matches!(low[d + 1].as_str(), "i" | "you" | "we" | "they" | "he" | "she" | "it") && d + 2 < low.len() && can_be_verb(&low[d + 2])).then_some(d + 2))?;
    if v <= d + 1 {
        return None;
    }
    let neg = low.get(d + 1).is_some_and(|x| x == "not");
    let subj: Vec<&str> = toks[d + 1 + neg as usize..v].iter().map(String::as_str).collect();
    if subj.is_empty() {
        return None;
    }
    let verb = inflect(&low[v], &low[d]);
    let wh: Vec<&str> = toks[..d].iter().map(String::as_str).collect();
    let adverbial = matches!(low[w0].as_str(), "why" | "how" | "where" | "when");
    let mut out: Vec<String> = Vec::new();
    if adverbial {
        out.extend(wh.iter().map(|x| x.to_string()));
    } else {
        out.extend(toks[..w0].iter().cloned());
    }
    out.extend(subj.iter().map(|x| x.to_string()));
    if neg {
        out.push(format!("{} not", toks[d]));
        out.push(low[v].clone());
    } else {
        out.push(verb);
    }
    if !adverbial {
        out.extend(toks[w0..d].iter().cloned());
    }
    out.extend(toks[v + 1..].iter().cloned());
    Some(format!("{}?", out.join(" ")))
}

/// The prepositions a wh-word leaves at the end of a question («Who is the serpent talking to?»).
const STRANDED: [&str; 9] = ["to", "with", "for", "from", "about", "at", "on", "in", "of"];

/// A wh-word that is the object of a preposition, the preposition left at the end («Who is the serpent talking to?»,
/// «What did he break the ice with?») or in front («To whom did Dick give the cat?»), put in statement order with the
/// wh-word after its preposition: «the serpent is talking to whom?», «he broke the ice with what?», «Dick gave the cat
/// to whom?». The parser then hangs the wh-word on its preposition, and the asked role is the preposition's (to → the
/// recipient, a place preposition → the place, else an oblique).
fn stranded_order(toks: &[String], low: &[String]) -> Option<String> {
    let n = low.len();
    if n < 4 {
        return None;
    }
    // «To whom did …»: the preposition goes to the end
    if STRANDED.contains(&low[0].as_str()) && matches!(low[1].as_str(), "whom" | "who" | "what") {
        let mut t: Vec<String> = toks[1..].to_vec();
        t.push(low[0].clone());
        let l: Vec<String> = t.iter().map(|x| x.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_string()).collect();
        return stranded_order(&t, &l);
    }
    if !matches!(low[0].as_str(), "who" | "whom" | "what") || !STRANDED.contains(&low[n - 1].as_str()) || toks.iter().any(|x| x.ends_with(',')) {
        return None;
    }
    let wh = if low[0] == "what" { "what" } else { "whom" };
    let prep = n - 1;
    const DETS: [&str; 13] = ["the", "a", "an", "this", "that", "his", "her", "its", "their", "my", "your", "our", "these"];
    let has = |f: &str, tags: &[PTag]| en::dict::form(f).is_some_and(|x| en::dict::analyses(x).any(|r| tags.contains(&r.tag)));
    let free = |i: usize| !DETS.contains(&low[i - 1].as_str()) && !toks[i].chars().next().is_some_and(char::is_uppercase) && !low[i - 1].ends_with("'s");
    let aux = low[1].as_str();
    let mut out: Vec<String> = Vec::new();
    if matches!(aux, "do" | "does" | "did") {
        let neg = low[2] == "not";
        let s0 = 2 + neg as usize;
        let v = (s0 + 1..prep).find(|&i| has(&low[i], &[PTag::VB]) && free(i))
            .or_else(|| (matches!(low[s0].as_str(), "i" | "you" | "we" | "they" | "he" | "she" | "it") && s0 + 1 < prep && has(&low[s0 + 1], &[PTag::VB])).then_some(s0 + 1))?;
        out.extend(toks[s0..v].iter().cloned());
        if neg {
            out.push(format!("{} not", toks[1].to_lowercase()));
            out.push(low[v].clone());
        } else {
            out.push(inflect(&low[v], aux));
        }
        out.extend(toks[v + 1..].iter().cloned());
    } else if matches!(aux, "is" | "are" | "was" | "were" | "will" | "would" | "can" | "could" | "should" | "must" | "might" | "may" | "has" | "have" | "had") {
        // the subject ends at the first verb form (or adjective) after it: «the serpent | talking to», «the zoo | famous for»
        let modal = !matches!(aux, "is" | "are" | "was" | "were");
        let tags: &[PTag] = if modal { &[PTag::VB, PTag::VBN] } else { &[PTag::VBG, PTag::VBN, PTag::JJ] };
        let v = (3..prep).find(|&i| has(&low[i], tags) && free(i)).unwrap_or(prep);
        out.extend(toks[2..v].iter().cloned());
        out.push(low[1].clone());
        out.extend(toks[v..].iter().cloned());
    } else {
        return None;
    }
    if out.is_empty() {
        return None;
    }
    out.push(wh.to_string());
    Some(format!("{}?", out.join(" ")))
}

/// The units the reader parses: the sentences of a text, a question in statement order.
pub fn units(text: &str, question: bool) -> Vec<(String, bool)> {
    sentences(text).into_iter().map(|s| {
        let q = question && s.trim_end().ends_with('?');
        let u = if q { statement_order(&s).unwrap_or(s) } else { s };
        (u, q)
    }).collect()
}

/// Slang and spellings the parser does not know, put plain before parsing («'cos» → because).
pub fn plain(text: &str) -> String {
    // «…» and “…” are quotation marks the tokenizer does not split well: plain double quotes
    let text: String = text.chars().map(|c| if matches!(c, '«' | '»' | '“' | '”') { '"' } else { c }).collect();
    let mut s = format!(" {text} ");
    for (a, b) in [(" 'cos ", " because "), (" ’cos ", " because "), (" cos ", " because "), (" cuz ", " because "), (" coz ", " because "), (" 'cause ", " because "), (" gonna ", " going to "), (" wanna ", " want to "), (" gotta ", " have to "), (" ain't ", " is not ")] {
        s = s.replace(a, b);
    }
    let s = s.trim().to_string();
    if off("speaker-label") { s } else { speaker_label(&s) }
}

/// Words that show a label before a colon is a clause, not a speaker: verbs of saying and thinking, copulas and
/// auxiliaries, subordinators («Mum is telling a story when Dad cuts in: "…"» already has its verb).
const LABEL_CLAUSE: [&str; 64] = [
    "say", "says", "said", "ask", "asks", "asked", "tell", "tells", "told", "shout", "shouts", "shouted", "yell", "yells",
    "yelled", "reply", "replies", "replied", "whisper", "whispers", "whispered", "write", "writes", "wrote", "reckon",
    "reckons", "reckoned", "think", "thinks", "thought", "mouth", "mouths", "mouthed", "add", "adds", "added", "answer",
    "answers", "answered", "warn", "warns", "warned", "is", "are", "was", "were", "be", "has", "have", "had", "do",
    "does", "did", "will", "would", "can", "could", "when", "while", "as", "because", "if", "that", "like",
];

/// A speaker label: «Dad: "pass me the salt"», «Mum, at the door: "have you seen my keys?"» — a report of speech whose
/// verb of saying is left out. The parser reads the label as the subject of the quoted clause (the speaker passing the
/// salt); with the verb given back («Dad says: …», «Mum says, at the door: …») the quote is read as said: its «me» is
/// the speaker, its «you» and its vocative the one spoken to (PRAG_OFF=speaker-label). A label is short (its first
/// part, before a comma, at most six words, the whole at most twelve), begins a sentence with a capital letter, holds
/// no digit, no quotation mark and no word of `LABEL_CLAUSE`, and no word that can only be a finite verb.
pub fn speaker_label(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 16);
    let mut start = 0; // the start of the current sentence in `text`
    let mut quotes = 0usize;
    let mut last = 0; // copied up to here
    for (i, c) in text.char_indices() {
        match c {
            '"' => {
                quotes += 1;
                if quotes % 2 == 0 {
                    start = i + 1;
                }
            }
            '.' | '!' | '?' | ';' if quotes % 2 == 0 => start = i + 1,
            ':' if quotes % 2 == 0 && text[i + 1..].trim_start().starts_with('"') => {
                let label = &text[start..i];
                if let Some(at) = label_insert(label) {
                    let at = start + at;
                    out.push_str(&text[last..at]);
                    out.push_str(" says");
                    last = at;
                }
                start = i + 1;
            }
            _ => {}
        }
    }
    out.push_str(&text[last..]);
    out
}

/// Where «says» goes in a speaker label (a byte offset into it), or None when it is no label.
fn label_insert(label: &str) -> Option<usize> {
    let lead = label.len() - label.trim_start().len();
    let l = label.trim();
    if !l.chars().next().is_some_and(char::is_uppercase) || l.chars().any(|c| c.is_ascii_digit() || c == '"') {
        return None;
    }
    let head = l.split(',').next().unwrap_or(l).trim_end();
    let words = |s: &str| s.split_whitespace().map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'').to_lowercase()).filter(|w| !w.is_empty()).collect::<Vec<_>>();
    let (hw, all) = (words(head), words(l));
    if hw.is_empty() || hw.len() > 6 || all.len() > 12 || all.iter().any(|w| LABEL_CLAUSE.contains(&w.as_str())) {
        return None;
    }
    // a word that can only be a verb («Remember:», «Polite requests wrap …:»), or after the first word one that can be a
    // past or third-person verb («Your mate fell off the wall:», «Dad turns to you:»), makes it a clause
    let tags = |w: &str| -> Vec<PTag> { en::dict::form(w).map(|f| en::dict::analyses(f).map(|r| r.tag).collect()).unwrap_or_default() };
    let verb_only = |w: &str| {
        let t = tags(w);
        !t.is_empty() && t.iter().all(|t| matches!(t, PTag::VBZ | PTag::VBD | PTag::VBP | PTag::VB | PTag::MD))
    };
    let finite = |w: &str| tags(w).iter().any(|t| matches!(t, PTag::VBZ | PTag::VBD));
    if hw.iter().any(|w| verb_only(w)) || hw.iter().skip(1).any(|w| finite(w)) {
        return None;
    }
    Some(lead + head.len())
}

/// A text read into frames.
#[derive(Clone, Debug, Default)]
pub struct Read {
    pub frames: Vec<Frame>,
    /// the interrogative words (a question's gap)
    pub asked: Vec<Asked>,
    /// names given in apposition
    pub aliases: Vec<(String, String)>,
    /// the root frame of each sentence
    pub roots: Vec<usize>,
    /// the trees of the sentences read (for the learned rules, `crate::rules`)
    pub trees: Vec<Tree>,
    /// each frame's predicate: (sentence, token); token `usize::MAX` for a bare phrase
    pub pred: Vec<(usize, usize)>,
    /// the discourse before this text: its entities by salience (the most salient first), and the last speaker
    pub before: Vec<Ent>,
    pub before_speaker: Option<Ent>,
}

/// A rule a sentence states, condition ⇒ conclusion, as indices into the frames of one reading (`Read::frames` of a
/// line, whose `link`s it follows). «If someone asks "can you pass the salt?", they want you to do it» → the
/// condition is the quoted «can you pass the salt», the conclusion «they want» and its complement «you do it», and
/// «do it» stands for the condition's action. «"Don't forget X" means remember to do X» → the quoted subject of
/// «mean» ⇒ its complement «remember», «do X».
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Implication {
    /// what joins them: the condition's marker (if, when, whenever, once), «mean» / «ask» / «tell» (a quoted pattern
    /// and what it means), or «because» (a cause link)
    pub conn: String,
    /// the condition's frames: a clause, a speech condition's quoted frames, the quoted subject of «mean», and the
    /// frames coordinated with them («"Can you…?" or "could you…?"»)
    pub cond: Vec<usize>,
    /// the conclusion: its main clause and that clause's complements (xcomp, ccomp), outermost first
    pub concl: Vec<usize>,
    /// the conclusion's frames that stand for the condition's action: «do» with «it», «that», «this» or «so»
    pub pro: Vec<usize>,
}

/// Whether frame `f` of a reading is asked, not stated: an auxiliary before its subject («can you pass the salt?»,
/// «could you grab …», not «you can use the steps»).
pub fn interrogative(r: &Read, f: usize) -> bool {
    let Some(&(s, p)) = r.pred.get(f) else { return false };
    let Some(t) = r.trees.get(s) else { return false };
    if p >= t.len() {
        return false;
    }
    let subj = t.kids_of(p, Rel::Nsubj);
    let aux = t.kids_of(p, Rel::Aux);
    subj.first().is_some_and(|&n| aux.iter().any(|&a| a < n))
}

/// The markers of a condition clause.
const COND_MARKS: [&str; 4] = ["if", "when", "whenever", "once"];
/// The verbs that say what a quoted pattern means («"Have you got a pen?" means "lend me a pen"», «"Can you…?" asks you
/// to do it»).
const MEANS: [&str; 3] = ["mean", "ask", "tell"];

/// The rules a reading states (`Implication`), in the order of their frames.
pub fn implications(fs: &[Frame]) -> Vec<Implication> {
    let kids = |p: usize, rels: &[Rel]| -> Vec<usize> { (0..fs.len()).filter(|&i| i != p && fs[i].link.as_ref().is_some_and(|(h, r, _)| *h == p && rels.contains(&r.base()))).collect() };
    // a frame and its complements, recursively, outermost first
    let chain = |k: usize| -> Vec<usize> {
        let mut out = vec![k];
        let mut i = 0;
        while i < out.len() && out.len() < 8 {
            for c in kids(out[i], &[Rel::Xcomp, Rel::Ccomp]) {
                if !out.contains(&c) {
                    out.push(c);
                }
            }
            i += 1;
        }
        out
    };
    // a frame and the frames coordinated with it
    let with_conj = |v: Vec<usize>| -> Vec<usize> {
        let mut out = v;
        let mut i = 0;
        while i < out.len() && out.len() < 8 {
            for c in kids(out[i], &[Rel::Conj]) {
                if !out.contains(&c) {
                    out.push(c);
                }
            }
            i += 1;
        }
        out
    };
    let pro = |concl: &[usize]| -> Vec<usize> {
        concl.iter().copied().filter(|&i| {
            let f = &fs[i];
            f.verb == "do" && (f.patient.as_ref().is_some_and(|p| ["it", "that", "this", "so"].iter().any(|w| p.id == *w || p.pron.as_deref() == Some(*w)))
                || f.patient.is_none() && f.words.iter().any(|(_, w, _)| w == "so"))
        }).collect()
    };
    let mut out = Vec::new();
    let mut push = |conn: &str, cond: Vec<usize>, concl: Vec<usize>| {
        let cond: Vec<usize> = cond.into_iter().filter(|c| !concl.contains(c)).collect();
        if !cond.is_empty() && !concl.is_empty() {
            let pro = pro(&concl);
            out.push(Implication { conn: conn.to_string(), cond, concl, pro });
        }
    };
    for (c, f) in fs.iter().enumerate() {
        // «if / when … , K»: the condition clause hangs on its main clause as advcl
        if let Some((k, r, _)) = &f.link {
            if r.base() == Rel::Advcl && *k != c && *k < fs.len() {
                if let Some(m) = f.mark.iter().map(|m| m.0.to_lowercase()).find(|m| COND_MARKS.contains(&m.as_str())) {
                    // a speech condition: what is said is the pattern («if someone asks "can you X?"»)
                    let cond = if matches!(f.kind, Kind::Say(_)) && !f.quote.is_empty() { f.quote.clone() } else { vec![c] };
                    push(&m, with_conj(cond), chain(*k));
                }
            }
        }
        // «"P" means Q», «"P" asks you to Q»: the quoted subject (or what the verb quotes, when it has no subject of
        // its own) ⇒ the complement
        if MEANS.contains(&f.verb.as_str()) && !f.neg {
            let comps: Vec<usize> = kids(c, &[Rel::Xcomp, Rel::Ccomp]);
            let mut subj: Vec<usize> = kids(c, &[Rel::Csubj, Rel::Nsubj]);
            if subj.is_empty() && f.agent.is_none() {
                subj = f.quote.iter().copied().filter(|q| *q < fs.len() && !comps.contains(q)).collect();
            }
            if !subj.is_empty() {
                let mut concl = Vec::new();
                for k in comps {
                    for x in chain(k) {
                        if !concl.contains(&x) {
                            concl.push(x);
                        }
                    }
                }
                push(&f.verb, with_conj(subj), concl);
            }
        }
        // «K because C», «C, so K»: the reader's cause link
        if let Some(cz) = f.cause.filter(|&cz| cz != c && cz < fs.len()) {
            push("because", vec![cz], chain(c));
        }
    }
    out
}

/// The sentences of a text as the reader parses them (for caching trees).
pub fn sentences(text: &str) -> Vec<String> {
    split_sentences(&plain(text))
}

/// Read a text into frames with the given parser (a sentence → annotated words, lemmas lower case). The caller
/// owns the parser (world: the `en` annotator with its matrix repairs) and the discourse `ctx`.
pub fn read_with(text: &str, ctx: &mut Ctx, question: bool, parse: &dyn Fn(&str) -> Vec<Word>) -> Read {
    let mut out = Read::default();
    {
        let mut seen: Vec<&Seen> = ctx.seen.values().filter(|s| s.salience > 0).collect();
        seen.sort_by(|a, b| b.salience.cmp(&a.salience).then(b.last.cmp(&a.last)));
        out.before = seen.into_iter().take(8).map(|s| s.ent.clone()).collect();
        out.before_speaker = ctx.last_speaker.clone();
    }
    let mut last_root: Option<usize> = None;
    for (s, q) in units(text, question) {
        let mut t = Tree::of_words(parse(&s));
        if t.is_empty() {
            continue;
        }
        if q {
            let mut w = t.w.clone();
            if repair_do_question(&mut w) {
                t = Tree::of_words(w);
            }
        }
        if !off("double-subject") {
            let mut w = t.w.clone();
            if repair_double_subject(&mut w) {
                t = Tree::of_words(w);
            }
        }
        if !off("wh-copula") {
            let mut w = t.w.clone();
            if repair_wh_copula(&mut w) {
                t = Tree::of_words(w);
            }
        }
        if q && !off("wh-passive") {
            let mut w = t.w.clone();
            if repair_wh_passive(&mut w) {
                t = Tree::of_words(w);
            }
        }
        let base = out.frames.len();
        let forced = if matches!(coref_mode(), 1 | 2 | 4) {
            ctx.sents.push(t.w.clone());
            coref_last(ctx)
        } else {
            BTreeMap::new()
        };
        let (mut fs, asked, aliases, noms) = frames_of_tree_with(&t, ctx, q, forced);
        let mut root = t.root().and_then(|r| {
            let preds: Vec<usize> = (0..t.len()).filter(|&i| is_pred(&t, i) && !matches!(t.base(i), Rel::Aux | Rel::Cop)).collect();
            preds.iter().position(|&p| p == r).map(|k| k + base)
        });
        let preds: Vec<usize> = (0..t.len()).filter(|&i| is_pred(&t, i) && !matches!(t.base(i), Rel::Aux | Rel::Cop)).collect();
        let si = out.trees.len();
        out.pred.extend(preds.iter().map(|&p| (si, p)));
        out.pred.extend(noms.iter().map(|&k| (si, k)));
        if fs.is_empty() {
            if let Some(b) = bare(&t, ctx) {
                fs.push(b);
                out.pred.push((si, usize::MAX));
                root = Some(base);
            }
        }
        out.trees.push(t.clone());
        if root.is_none() && !fs.is_empty() {
            root = Some(base);
        }
        for f in &mut fs {
            for x in f.quote.iter_mut() {
                *x += base;
            }
            f.cause = f.cause.map(|c| c + base);
            f.next = f.next.map(|c| c + base);
            if let Some(l) = f.link.as_mut() {
                l.0 += base;
            }
        }
        out.asked.extend(asked.into_iter().map(|a| Asked { frame: a.frame + base, ..a }));
        out.aliases.extend(aliases);
        out.frames.extend(fs);
        if let (Some(lr), Some(r)) = (last_root, root) {
            if !off("next-sentences") && out.frames[lr].next.is_none() && !out.frames[r].quoted && !out.frames[lr].quoted && lr != r {
                out.frames[lr].next = Some(r);
            }
        }
        if let Some(r) = root {
            out.roots.push(r);
            last_root = Some(r);
        }
        ctx.new_line();
    }
    // «Your friend says: "…" Put her right.» — the claim said is what the question is about
    let low = text.to_lowercase();
    if question && out.asked.is_empty() && !off("q-claim") && ["put him right", "put her right", "put them right", "put me right", "correct him", "correct her", "correct them"].iter().any(|x| low.contains(x)) {
        let claim = out.frames.iter().position(|f| f.quoted).or_else(|| out.frames.iter().find_map(|f| f.quote.first().copied())).or_else(|| out.roots.first().copied());
        if let Some(c) = claim {
            out.asked.push(Asked { word: "claim".into(), frame: c, what: Ask::Frame });
        }
    }
    if !off("next-events") {
        // «next» is the order of happenings: between actions and speech, never from or to a state or a general fact
        let ev = |f: &Frame| matches!(f.kind, Kind::Event | Kind::Say(_)) && !f.cond;
        for i in 0..out.frames.len() {
            if let Some(n) = out.frames[i].next {
                if !(ev(&out.frames[i]) && ev(&out.frames[n])) {
                    out.frames[i].next = None;
                }
            }
        }
    }
    // the rules the reader learned and the checker accepted (`crate::rules`, PRAG_RULES)
    let rules = crate::rules::active();
    for rule in rules.iter() {
        crate::rules::apply_rule(&mut out, rule);
    }
    out
}

/// The frames a scorer compares with the frame bench's gold: the events of nouns (WordNet) left out — the teacher's
/// gold writes frames for predicates only, so a noun's event can be neither right nor wrong there — with the links
/// remapped. PRAG_VN=score-nouns keeps them (they then count as made and unmatched).
pub fn scored(frames: &[Frame]) -> Vec<Frame> {
    if vn_opt("score-nouns") {
        return frames.to_vec();
    }
    let keep: Vec<usize> = (0..frames.len()).filter(|&i| frames[i].how.by != By::Nominal).collect();
    let map: BTreeMap<usize, usize> = keep.iter().enumerate().map(|(k, &i)| (i, k)).collect();
    keep.iter().map(|&i| {
        let mut f = frames[i].clone();
        f.quote = f.quote.iter().filter_map(|x| map.get(x).copied()).collect();
        f.cause = f.cause.and_then(|x| map.get(&x).copied());
        f.next = f.next.and_then(|x| map.get(&x).copied());
        f.link = f.link.take().and_then(|(x, r, w)| map.get(&x).map(|&y| (y, r, w)));
        f
    }).collect()
}

#[cfg(test)]
mod order_tests {
    use super::*;

    #[test]
    fn a_stranded_preposition_takes_its_wh_word() {
        assert_eq!(statement_order("Who is the serpent talking to?").as_deref(), Some("the serpent is talking to whom?"));
        assert_eq!(statement_order("Who did Abraham give the land to?").as_deref(), Some("Abraham gave the land to whom?"));
        assert_eq!(statement_order("To whom did Dick give the cat?").as_deref(), Some("Dick gave the cat to whom?"));
        assert_eq!(statement_order("What did he break the ice with?").as_deref(), Some("he broke the ice with what?"));
        // no stranded preposition: the old order
        assert_eq!(statement_order("What did the cook throw?").as_deref(), Some("the cook threw What?"));
    }
}

/// The frames keep what the tree says beside the roles; each test has a control tree without that part, where the
/// field must stay empty (so a field filled by mistake shows red).
#[cfg(test)]
mod complete_tests {
    use super::*;

    /// A tree from rows (form, lemma, upos, tag, head, rel).
    fn tree(rows: &[(&str, &str, UPos, PTag, usize, Rel)]) -> Tree {
        Tree::of_words(rows.iter().map(|&(form, lemma, upos, tag, head, rel)| Word { form: form.into(), lemma: lemma.into(), upos, tag, feats: Default::default(), head, rel }).collect())
    }

    /// «Then Alice's dog of the queen barked and ran.» and its controls: without «then» and «of the queen», and
    /// with «and it ran».
    fn dog(full: bool, with_it: bool) -> Tree {
        use {PTag as T, Rel as R, UPos as U};
        if !full {
            return tree(&[("Alice", "Alice", U::PROPN, T::NNP, 3, R::NmodPoss), ("'s", "'s", U::PART, T::POS, 1, R::Case), ("dog", "dog", U::NOUN, T::NN, 4, R::Nsubj), ("barked", "bark", U::VERB, T::VBD, 0, R::Root), ("and", "and", U::CCONJ, T::CC, 6, R::Cc), ("ran", "run", U::VERB, T::VBD, 4, R::Conj), (".", ".", U::PUNCT, T::Stop, 4, R::Punct)]);
        }
        let mut rows = vec![("Then", "then", U::ADV, T::RB, 8, R::Advmod), ("Alice", "Alice", U::PROPN, T::NNP, 4, R::NmodPoss), ("'s", "'s", U::PART, T::POS, 2, R::Case), ("dog", "dog", U::NOUN, T::NN, 8, R::Nsubj), ("of", "of", U::ADP, T::IN, 7, R::Case), ("the", "the", U::DET, T::DT, 7, R::Det), ("queen", "queen", U::NOUN, T::NN, 4, R::Nmod), ("barked", "bark", U::VERB, T::VBD, 0, R::Root)];
        if with_it {
            rows.extend([("and", "and", U::CCONJ, T::CC, 11, R::Cc), ("it", "it", U::PRON, T::PRP, 11, R::Nsubj), ("ran", "run", U::VERB, T::VBD, 8, R::Conj), (".", ".", U::PUNCT, T::Stop, 8, R::Punct)]);
        } else {
            rows.extend([("and", "and", U::CCONJ, T::CC, 10, R::Cc), ("ran", "run", U::VERB, T::VBD, 8, R::Conj), (".", ".", U::PUNCT, T::Stop, 8, R::Punct)]);
        }
        tree(&rows)
    }

    #[test]
    fn possessor_modifiers_adverbs_and_the_unsaid_subject() {
        let (fs, _, _) = frames_of_tree(&dog(true, false), &mut Ctx::default(), false);
        assert_eq!(fs.len(), 2, "{fs:?}");
        let a = fs[0].agent.as_ref().unwrap();
        assert_eq!(a.id, "dog");
        assert_eq!(a.poss.as_ref().map(|p| p.id.as_str()), Some("Alice"));
        assert_eq!(a.mods.iter().map(|(c, e)| (c.as_str(), e.id.as_str())).collect::<Vec<_>>(), vec![("of", "queen")]);
        assert!(fs[0].words.iter().any(|w| w.1 == "then"));
        assert_eq!(fs[1].unsaid, vec![Slot::Agent], "the second verb shares the subject");
        // controls: without the parts, the fields stay empty
        let (fs, _, _) = frames_of_tree(&dog(false, false), &mut Ctx::default(), false);
        let a = fs[0].agent.as_ref().unwrap();
        assert!(a.mods.is_empty() && fs[0].words.is_empty());
        let (fs, _, _) = frames_of_tree(&dog(true, true), &mut Ctx::default(), false);
        assert!(fs[1].unsaid.is_empty(), "«and it ran» says its subject: {:?}", fs[1]);
    }

    /// «But the dog sat in a box.» and the control «Dog sat in box.»
    #[test]
    fn determiners_prepositions_and_conjunctions() {
        use {PTag as T, Rel as R, UPos as U};
        let full = tree(&[("But", "but", U::CCONJ, T::CC, 4, R::Cc), ("the", "the", U::DET, T::DT, 3, R::Det), ("dog", "dog", U::NOUN, T::NN, 4, R::Nsubj), ("sat", "sit", U::VERB, T::VBD, 0, R::Root), ("in", "in", U::ADP, T::IN, 7, R::Case), ("a", "a", U::DET, T::DT, 7, R::Det), ("box", "box", U::NOUN, T::NN, 4, R::Obl), (".", ".", U::PUNCT, T::Stop, 4, R::Punct)]);
        let (fs, _, _) = frames_of_tree(&full, &mut Ctx::default(), false);
        assert_eq!(fs[0].cc.as_deref(), Some("but"));
        assert_eq!(fs[0].agent.as_ref().unwrap().det, Some(("the".to_string(), T::DT)));
        let pl = fs[0].place.as_ref().expect("a place");
        assert_eq!((pl.case.as_deref(), pl.det.as_ref().map(|d| d.0.as_str())), (Some("in"), Some("a")));
        // control: no determiners, no conjunction in the text, none in the frame (the preposition stays)
        let bare = tree(&[("Dog", "dog", U::NOUN, T::NN, 2, R::Nsubj), ("sat", "sit", U::VERB, T::VBD, 0, R::Root), ("in", "in", U::ADP, T::IN, 4, R::Case), ("box", "box", U::NOUN, T::NN, 2, R::Obl), (".", ".", U::PUNCT, T::Stop, 2, R::Punct)]);
        let (fs, _, _) = frames_of_tree(&bare, &mut Ctx::default(), false);
        assert!(fs[0].cc.is_none() && fs[0].agent.as_ref().unwrap().det.is_none() && fs[0].place.as_ref().unwrap().det.is_none());
        assert_eq!(fs[0].place.as_ref().unwrap().case.as_deref(), Some("in"));
    }

    /// «The man who came sat.»: the relative clause hangs on «man» and «who» stands for the man; the control «The man
    /// came and sat.» is a conjunct of the first verb with no noun.
    #[test]
    fn anchors_name_the_tokens_of_predicates_roles_and_links() {
        use {PTag as T, Rel as R, UPos as U};
        let t = tree(&[("The", "the", U::DET, T::DT, 2, R::Det), ("dog", "dog", U::NOUN, T::NN, 3, R::Nsubj), ("saw", "see", U::VERB, T::VBD, 0, R::Root), ("a", "a", U::DET, T::DT, 5, R::Det), ("cat", "cat", U::NOUN, T::NN, 3, R::Obj), ("and", "and", U::CCONJ, T::CC, 7, R::Cc), ("ran", "run", U::VERB, T::VBD, 3, R::Conj), (".", ".", U::PUNCT, T::Stop, 3, R::Punct)]);
        let mut ctx = Ctx::default();
        let (fs, _, _) = frames_of_tree(&t, &mut ctx, false);
        assert_eq!(ctx.anchors.len(), fs.len());
        let see = fs.iter().position(|f| f.verb == "see").unwrap();
        let run = fs.iter().position(|f| f.verb == "run").unwrap();
        assert_eq!(ctx.anchors[see].pred, Some(2));
        assert!(ctx.anchors[see].roles.contains(&(Slot::Agent, 1)), "{:?}", ctx.anchors[see]);
        assert!(ctx.anchors[see].roles.contains(&(Slot::Patient, 4)), "{:?}", ctx.anchors[see]);
        assert_eq!(ctx.anchors[run].pred, Some(6));
        assert_eq!(ctx.anchors[run].link, Some((2, R::Conj)));
        // the shared subject is not said in the second clause: no token
        assert!(!ctx.anchors[run].roles.iter().any(|r| r.0 == Slot::Agent), "{:?}", ctx.anchors[run]);
        // the frames are the same without anchors
        let (fs2, _, _) = frames_of_tree(&t, &mut Ctx::default(), false);
        assert_eq!(fs, fs2);
    }

    #[test]
    fn clause_links_and_relative_words() {
        use {PTag as T, Rel as R, UPos as U};
        let rel = tree(&[("The", "the", U::DET, T::DT, 2, R::Det), ("man", "man", U::NOUN, T::NN, 5, R::Nsubj), ("who", "who", U::PRON, T::WP, 4, R::Nsubj), ("came", "come", U::VERB, T::VBD, 2, R::AclRelcl), ("sat", "sit", U::VERB, T::VBD, 0, R::Root), (".", ".", U::PUNCT, T::Stop, 5, R::Punct)]);
        let (fs, _, _) = frames_of_tree(&rel, &mut Ctx::default(), false);
        let came = fs.iter().position(|f| f.verb == "come").unwrap();
        let sat = fs.iter().position(|f| f.verb == "sit").unwrap();
        assert_eq!(fs[came].link, Some((sat, R::AclRelcl, Some("man".into()))));
        assert_eq!(fs[came].agent.as_ref().unwrap().rel_word, Some(("who".into(), T::WP)));
        assert_eq!(fs[sat].link, None);
        let conj = tree(&[("The", "the", U::DET, T::DT, 2, R::Det), ("man", "man", U::NOUN, T::NN, 3, R::Nsubj), ("came", "come", U::VERB, T::VBD, 0, R::Root), ("and", "and", U::CCONJ, T::CC, 5, R::Cc), ("sat", "sit", U::VERB, T::VBD, 3, R::Conj), (".", ".", U::PUNCT, T::Stop, 3, R::Punct)]);
        let (fs, _, _) = frames_of_tree(&conj, &mut Ctx::default(), false);
        assert_eq!(fs[1].link, Some((0, R::Conj, None)));
        assert!(fs[0].agent.as_ref().unwrap().rel_word.is_none());
    }

    /// «A short-eared owl hunts.» as the parser gives it (short - eared/VERB/amod): one frame, «hunt», the owl keeps
    /// «ear» among its attributes. Control: «A sleeping owl hunts.» (a participle modifier without a hyphen) keeps
    /// its own frame, so the rule does not swallow every verb modifier.
    #[test]
    fn a_hyphenated_compound_adjective_is_no_predicate() {
        use {PTag as T, Rel as R, UPos as U};
        let hy = tree(&[("A", "a", U::DET, T::DT, 5, R::Det), ("short", "short", U::ADJ, T::JJ, 4, R::Amod), ("-", "-", U::PUNCT, T::HYPH, 2, R::Punct), ("eared", "ear", U::VERB, T::VBN, 5, R::Amod), ("owl", "owl", U::NOUN, T::NN, 6, R::Nsubj), ("hunts", "hunt", U::VERB, T::VBZ, 0, R::Root), (".", ".", U::PUNCT, T::Stop, 6, R::Punct)]);
        let (fs, _, _) = frames_of_tree(&hy, &mut Ctx::default(), false);
        assert_eq!(fs.iter().map(|f| f.verb.as_str()).collect::<Vec<_>>(), vec!["hunt"], "{fs:?}");
        assert!(fs[0].agent.as_ref().is_some_and(|a| a.id == "owl" && a.attrs.iter().any(|x| x == "ear")), "{fs:?}");
        let control = tree(&[("A", "a", U::DET, T::DT, 3, R::Det), ("sleeping", "sleep", U::VERB, T::VBG, 3, R::Amod), ("owl", "owl", U::NOUN, T::NN, 4, R::Nsubj), ("hunts", "hunt", U::VERB, T::VBZ, 0, R::Root), (".", ".", U::PUNCT, T::Stop, 4, R::Punct)]);
        let (fc, _, _) = frames_of_tree(&control, &mut Ctx::default(), false);
        assert!(fc.iter().any(|f| f.verb == "sleep"), "control: a plain participle modifier keeps its frame: {fc:?}");
    }

    /// «Spiders have eyes, in rows.» with the parser's rows/conj>eyes carrying its own «in»: no coordinated entity,
    /// the verb's place. Control: «Spiders have eyes and legs.» keeps the coordination.
    #[test]
    fn a_coordinated_prepositional_phrase_is_the_verbs_phrase() {
        use {PTag as T, Rel as R, UPos as U};
        let pp = tree(&[("Spiders", "spider", U::NOUN, T::NNS, 2, R::Nsubj), ("have", "have", U::VERB, T::VBP, 0, R::Root), ("eyes", "eye", U::NOUN, T::NNS, 2, R::Obj), (",", ",", U::PUNCT, T::Comma, 6, R::Punct), ("in", "in", U::ADP, T::IN, 6, R::Case), ("rows", "row", U::NOUN, T::NNS, 3, R::Conj), (".", ".", U::PUNCT, T::Stop, 2, R::Punct)]);
        let (fs, _, _) = frames_of_tree(&pp, &mut Ctx::default(), false);
        let p = fs[0].patient.as_ref().unwrap();
        assert_eq!((p.id.as_str(), p.and.len(), fs[0].place.as_ref().map(|e| e.id.as_str())), ("eye", 0, Some("row")), "{fs:?}");
        let control = tree(&[("Spiders", "spider", U::NOUN, T::NNS, 2, R::Nsubj), ("have", "have", U::VERB, T::VBP, 0, R::Root), ("eyes", "eye", U::NOUN, T::NNS, 2, R::Obj), ("and", "and", U::CCONJ, T::CC, 5, R::Cc), ("legs", "leg", U::NOUN, T::NNS, 3, R::Conj), (".", ".", U::PUNCT, T::Stop, 2, R::Punct)]);
        let (fc, _, _) = frames_of_tree(&control, &mut Ctx::default(), false);
        assert_eq!(fc[0].patient.as_ref().map(|p| p.and.clone()), Some(vec!["leg".to_string()]), "control: {fc:?}");
    }
}

#[cfg(test)]
mod factuality_tests {
    use super::*;
    use en::gram::{Feats, Tag};

    fn w(form: &str, lemma: &str, upos: UPos, head: usize, rel: Rel) -> Word {
        Word { form: form.into(), lemma: lemma.into(), upos, tag: if upos == UPos::VERB { PTag::VBD } else { Tag::NN }, feats: Feats::default(), head, rel }
    }

    /// «The fox [did not] <verb> to eat the grapes.» → the eat frame.
    fn eat(verb: &str, neg: bool) -> Frame {
        let mut v = vec![w("The", "the", UPos::DET, 2, Rel::Det), w("fox", "fox", UPos::NOUN, 3, Rel::Nsubj)];
        let off = if neg { 2 } else { 0 };
        let root = 3 + off;
        v[1].head = root;
        if neg {
            v.push(w("did", "do", UPos::AUX, root, Rel::Aux));
            v.push(w("not", "not", UPos::PART, root, Rel::Advmod));
        }
        v.push(w(verb, verb, UPos::VERB, 0, Rel::Root));
        v.push(w("to", "to", UPos::PART, root + 2, Rel::Mark));
        v.push(w("eat", "eat", UPos::VERB, root, Rel::Xcomp));
        v.push(w("the", "the", UPos::DET, root + 4, Rel::Det));
        v.push(w("grapes", "grape", UPos::NOUN, root + 2, Rel::Obj));
        let t = Tree::of_words(v);
        let (fs, _, _) = frames_of_tree(&t, &mut Ctx::default(), false);
        fs.into_iter().find(|f| f.verb == "eat").expect("an eat frame")
    }

    /// rudinger-2018 I1: the signatures and negation set the complement's status. The control: a ○/○ verb («want»)
    /// leaves it unsaid whether affirmed or negated, and the sentence with the signatures off reads every «eat» as a
    /// fact.
    #[test]
    fn implication_signatures_set_what_happened() {
        let st = |f: Frame| (f.neg, f.irrealis);
        assert_eq!(st(eat("manage", false)), (false, false), "managed to eat: ate");
        assert_eq!(st(eat("manage", true)), (true, false), "did not manage to eat: did not eat");
        assert_eq!(st(eat("fail", false)), (true, false), "failed to eat: did not eat");
        assert_eq!(st(eat("fail", true)), (false, false), "did not fail to eat: ate");
        assert_eq!(st(eat("want", false)), (false, true), "wanted to eat: not said");
        assert_eq!(st(eat("want", true)), (false, true), "did not want to eat: not said either");
        assert_eq!(signature("eat"), None);
    }
}

#[cfg(test)]
mod wave15_models {
    use super::*;
    fn ent(id: &str) -> Ent {
        Ent {
            id: id.into(),
            text: id.into(),
            ..Ent::default()
        }
    }
    fn word(id: &str, upos: UPos, tag: PTag, head: usize, rel: Rel) -> Word {
        Word {
            form: id.into(),
            lemma: id.into(),
            upos,
            tag,
            head,
            rel,
            feats: Default::default(),
        }
    }
    fn event(subject: &str, verb: &str) -> Vec<Word> {
        vec![
            word(subject, UPos::NOUN, PTag::NN, 2, Rel::Nsubj),
            word(verb, UPos::VERB, PTag::VBD, 0, Rel::Root),
        ]
    }
    #[test]
    fn unresolved_identity_is_symmetric_even_in_coordination() {
        for id in UNRESOLVED_PUB {
            let unknown = ent(id);
            let mut group = ent("dog");
            group.and.push(id.into());
            assert!(!unknown.same(&group));
            assert!(
                !group.same(&unknown),
                "unresolved right operand {id} matched coordination"
            );
        }
    }
    #[test]
    fn head_identity_preserves_coordination_but_not_transitivity() {
        let a = ent("dog");
        let b = ent("cat");
        let mut group = a.clone();
        group.and.push("cat".into());
        assert!(a.same(&group) && group.same(&a));
        assert!(b.same(&group) && group.same(&b));
        assert!(!a.same(&b));
        let mut mention = a.clone();
        mention.text = "the old dog".into();
        mention.pron = Some("it".into());
        assert!(mention.same(&a));
    }
    #[test]
    fn tree_coordinates_are_ordered_and_invalid_heads_are_absent() {
        let mut words = event("dog", "bark");
        words.push(word("cat", UPos::NOUN, PTag::NN, 2, Rel::Obj));
        words.push(word("lost", UPos::NOUN, PTag::NN, 99, Rel::Obl));
        let t = Tree::of_words(words);
        assert_eq!(t.root(), Some(1));
        assert_eq!(t.kids(1), &[0, 2]);
        assert_eq!(t.kids_of(1, Rel::Obj), vec![2]);
        assert_eq!(t.head(0), Some(1));
        assert_eq!(t.head(3), None);
        assert!(Tree::default().root().is_none());
    }
    #[test]
    fn context_resolution_respects_exclusion_and_unresolved_mentions() {
        let mut ctx = Ctx::default();
        ctx.mention(&ent("it"), false);
        assert_eq!(ctx.summary(5), "entities: none yet");
        ctx.mention(&ent("box"), false);
        assert_eq!(ctx.resolve("it").unwrap().id, "box");
        assert!(ctx.resolve_not("it", Some("box")).is_none());
        ctx.new_line();
        assert_eq!(ctx.resolve("it").unwrap().id, "box");
    }
    #[test]
    fn supplied_tree_projects_roles_and_predicate_anchors() {
        let t = Tree::of_words(event("dog", "bark"));
        let mut ctx = Ctx::default();
        let (fs, asked, aliases) = frames_of_tree(&t, &mut ctx, false);
        assert_eq!(fs.len(), 1);
        assert!(asked.is_empty() && aliases.is_empty());
        assert_eq!(fs[0].kind, Kind::Event);
        assert_eq!(fs[0].verb, "bark");
        assert_eq!(fs[0].agent.as_ref().unwrap().id, "dog");
        assert_eq!(ctx.anchors[0].pred, Some(1));
        assert_eq!(ctx.anchors[0].roles, vec![(Slot::Agent, 0)]);
    }
    #[test]
    fn question_gap_and_read_offsets_share_frame_coordinates() {
        let parse = |s: &str| {
            if s.starts_with("Who") {
                vec![
                    word("who", UPos::PRON, PTag::WP, 2, Rel::Nsubj),
                    word("bark", UPos::VERB, PTag::VBD, 0, Rel::Root),
                ]
            } else {
                event("cat", "run")
            }
        };
        let read = read_with("Cat ran. Who barked?", &mut Ctx::default(), true, &parse);
        assert_eq!(read.frames.len(), 2);
        assert_eq!(read.roots, vec![0, 1]);
        assert_eq!(read.pred, vec![(0, 1), (1, 1)]);
        assert!(
            read.asked
                .iter()
                .any(|a| a.frame == 1 && a.what == Ask::Role(Slot::Agent))
        );
    }
    #[test]
    fn read_with_preserves_context_and_sentence_event_order() {
        let mut ctx = Ctx::default();
        ctx.mention(&ent("box"), false);
        let read = read_with("Dog barked. Cat ran.", &mut ctx, false, &|s| {
            if s.starts_with("Dog") {
                event("dog", "bark")
            } else {
                event("cat", "run")
            }
        });
        assert_eq!(read.before[0].id, "box");
        assert_eq!(read.frames[0].next, Some(1));
        assert_eq!(read.frames[1].next, None);
        assert_eq!(read.trees.len(), 2);
        let empty = read_with("", &mut ctx, false, &|_| panic!("empty input parsed"));
        assert!(empty.frames.is_empty() && empty.roots.is_empty());
    }
    #[test]
    fn scored_projection_remaps_every_link_and_drops_nominals() {
        let mut a = Frame::new(Kind::Event, "bark".into());
        let mut nominal = Frame::new(Kind::Event, "arrival".into());
        nominal.how.by = By::Nominal;
        let mut b = Frame::new(Kind::Event, "run".into());
        a.quote = vec![1, 2];
        a.cause = Some(1);
        a.next = Some(2);
        b.link = Some((0, Rel::Conj, Some("dog".into())));
        let out = scored(&[a, nominal, b]);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].quote, vec![1]);
        assert_eq!(out[0].cause, None);
        assert_eq!(out[0].next, Some(1));
        assert_eq!(out[1].link, Some((0, Rel::Conj, Some("dog".into()))));
    }
}

#[cfg(test)]
mod speaker_label_tests {
    use super::*;

    #[test]
    fn a_speaker_label_gets_its_verb_of_saying_back() {
        assert_eq!(speaker_label(r#"Dad: "Leo, pass me the salt.""#), r#"Dad says: "Leo, pass me the salt.""#);
        assert_eq!(speaker_label(r#"Mum, at the door: "have you seen my keys?""#), r#"Mum says, at the door: "have you seen my keys?""#);
        assert_eq!(speaker_label(r#"Teacher by the board: "Right. Pencils. Down." What does she want?"#),
            r#"Teacher by the board says: "Right. Pencils. Down." What does she want?"#);
        // a second label after a sentence and after a closing quote
        assert_eq!(speaker_label(r#"Mum: "tea?" Dad: "yes.""#), r#"Mum says: "tea?" Dad says: "yes.""#);
    }

    #[test]
    fn what_is_no_speaker_label_stays_byte_for_byte() {
        for t in [
            r#"The teacher says: "sit down.""#,            // the verb is there
            r#"Mum is telling a story when Dad cuts in: "sorry, love.""#, // a clause
            r#"Polite requests wrap the same command in softer words: "could you…", "would you…""#, // too long a head
            r#"Remember: "please" is kind."#,                // a word that can only be a verb
            r#"At 10:30 the bell goes."#,                    // a time, no quote
            r#"the list: "a", "b""#,                         // no capital: not the start of a sentence
            r#"She said "go: now" and left."#,               // a colon inside a quote
            r#"Your mate fell off the wall: "don't tell anyone.""#, // a clause with its own verb
            "Dad passes the salt.",
        ] {
            assert_eq!(speaker_label(t), t, "{t}");
        }
    }

    #[test]
    fn the_rewrite_is_idempotent_and_switchable() {
        let t = r#"Gran: "can you give us a hand with these bags?" Your little brother holds out his hand."#;
        let once = speaker_label(t);
        assert_ne!(once, t);
        assert_eq!(speaker_label(&once), once);
        // only « says» is added; the quote is untouched
        assert_eq!(once.replacen(" says", "", 1), t);
    }
}

#[cfg(test)]
mod wh_role_tests {
    use super::*;
    use {PTag as T, Rel as R, UPos as U};

    fn tree(rows: &[(&str, UPos, PTag, usize, Rel)]) -> Tree {
        Tree::of_words(rows.iter().map(|&(form, upos, tag, head, rel)| Word { form: form.into(), lemma: form.to_lowercase(), upos, tag, feats: Default::default(), head, rel }).collect())
    }

    /// The time value of a phrase: a date whole and its year, a year alone; and the negative controls: a number that is
    /// no year («page 29») and a month word that is no name («their march») have no time.
    #[test]
    fn a_time_value_is_a_date_or_a_year() {
        // «on November 29, 2023»
        let t = tree(&[("on", U::ADP, T::IN, 2, R::Case), ("November", U::PROPN, T::NNP, 0, R::Root), ("29", U::NUM, T::CD, 2, R::Nummod), (",", U::PUNCT, T::Comma, 2, R::Punct), ("2023", U::NUM, T::CD, 2, R::Nmod)]);
        assert_eq!(time_value(&t, 1), vec!["november 29 2023".to_string(), "2023".to_string()]);
        // «in 1999»
        let t = tree(&[("in", U::ADP, T::IN, 2, R::Case), ("1999", U::NUM, T::CD, 0, R::Root)]);
        assert_eq!(time_value(&t, 1), vec!["1999".to_string()]);
        // negative controls
        let t = tree(&[("on", U::ADP, T::IN, 2, R::Case), ("page", U::NOUN, T::NN, 0, R::Root), ("29", U::NUM, T::CD, 2, R::Nummod)]);
        assert!(time_value(&t, 1).is_empty(), "page 29 is no time");
        let t = tree(&[("their", U::PRON, T::PRPS, 2, R::NmodPoss), ("march", U::NOUN, T::NN, 0, R::Root)]);
        assert!(time_value(&t, 1).is_empty(), "their march is no time");
    }

    /// The name role: compounds and flat words, appositions, «of» modifiers that are names; the negative control: an
    /// «of» modifier that is no name («a cup of tea») is not a name of the cup.
    #[test]
    fn an_entity_names_its_compounds_appositions_and_of_names() {
        let ent = |id: &str, named: bool| Ent { id: id.into(), text: id.into(), named, ..Default::default() };
        let mut line = ent("line", false);
        line.words.push((R::Compound, "lansdale/doylestown".into(), T::NNP));
        assert_eq!(line.names(), vec!["lansdale/doylestown"]);
        let mut city = ent("city", false);
        city.mods.push(("of".into(), ent("paris", true)));
        city.appos.push(ent("capital", false));
        assert_eq!(city.names(), vec!["capital", "paris"]);
        let mut cup = ent("cup", false);
        cup.mods.push(("of".into(), ent("tea", false)));
        assert!(cup.names().is_empty(), "tea is no name of the cup");
    }
}

#[cfg(test)]
mod implication_tests {
    use super::*;

    fn ent(id: &str) -> Ent {
        Ent { id: id.into(), text: id.into(), ..Default::default() }
    }

    fn frame(kind: Kind, verb: &str, agent: Option<&str>, patient: Option<Ent>, link: Option<(usize, Rel)>, mark: &[&str]) -> Frame {
        let mut f = Frame::new(kind, verb.into());
        f.agent = agent.map(ent);
        f.patient = patient;
        f.link = link.map(|(h, r)| (h, r, None));
        f.mark = mark.iter().map(|m| (m.to_string(), PTag::IN)).collect();
        f
    }

    /// «If someone asks "can you pass the salt?", they want you to do it»: the condition is the quoted request, the
    /// conclusion «want» with its complement, and «do it» stands for the condition's action. «"Don't forget X" means
    /// remember to do X»: the quoted subject of «mean» ⇒ its complement chain, with no pro-verb («do X» names X). The
    /// negative controls: the same clause marked «that» (no condition), and a «mean» without a subject, state no rule.
    #[test]
    fn rule_sentences_give_condition_and_conclusion() {
        let mut ask = frame(Kind::Say(Act::Ask), "ask", Some("someone"), None, Some((2, Rel::Advcl)), &["if"]);
        ask.quote = vec![1];
        let mut can = frame(Kind::Fact(Link::Can), "pass", Some("you"), Some(ent("salt")), Some((0, Rel::Ccomp)), &[]);
        can.quoted = true;
        let want = frame(Kind::Fact(Link::Does), "want", Some("they"), Some(ent("you")), None, &[]);
        let it = Ent { pron: Some("it".into()), ..ent("salt") };
        let doit = frame(Kind::Event, "do", Some("you"), Some(it), Some((2, Rel::Xcomp)), &["to"]);
        let fs = vec![ask, can, want, doit];
        assert_eq!(implications(&fs), vec![Implication { conn: "if".into(), cond: vec![1], concl: vec![2, 3], pro: vec![3] }]);

        let mut forget = frame(Kind::Event, "forget", None, Some(ent("x")), Some((1, Rel::Nsubj)), &[]);
        forget.neg = true;
        forget.quoted = true;
        let mean = frame(Kind::Event, "mean", None, None, None, &[]);
        let remember = frame(Kind::Event, "remember", None, None, Some((1, Rel::Ccomp)), &[]);
        let dox = frame(Kind::Event, "do", None, Some(ent("x")), Some((2, Rel::Xcomp)), &["to"]);
        let ms = vec![forget, mean, remember, dox];
        assert_eq!(implications(&ms), vec![Implication { conn: "mean".into(), cond: vec![0], concl: vec![2, 3], pro: vec![] }]);

        // negative controls
        let mut not_cond = fs.clone();
        not_cond[0].mark = vec![("that".into(), PTag::IN)];
        assert!(implications(&not_cond).is_empty(), "a clause marked «that» states no condition");
        let mut no_subj = ms.clone();
        no_subj[0].link = None;
        no_subj[1].agent = Some(ent("word"));
        assert!(implications(&no_subj).is_empty(), "«mean» with no quoted subject states no rule");
    }

    /// «can you pass the salt» is asked (the auxiliary before the subject); the negative control «you can pass the
    /// salt» is stated.
    #[test]
    fn an_auxiliary_before_the_subject_asks() {
        let w = |form: &str, upos: UPos, tag: PTag, head: usize, rel: Rel| Word { form: form.into(), lemma: form.to_lowercase(), upos, tag, feats: Default::default(), head, rel };
        let asked = Tree::of_words(vec![w("can", UPos::AUX, PTag::MD, 3, Rel::Aux), w("you", UPos::PRON, PTag::PRP, 3, Rel::Nsubj), w("pass", UPos::VERB, PTag::VB, 0, Rel::Root), w("salt", UPos::NOUN, PTag::NN, 3, Rel::Obj)]);
        let said = Tree::of_words(vec![w("you", UPos::PRON, PTag::PRP, 3, Rel::Nsubj), w("can", UPos::AUX, PTag::MD, 3, Rel::Aux), w("pass", UPos::VERB, PTag::VB, 0, Rel::Root), w("salt", UPos::NOUN, PTag::NN, 3, Rel::Obj)]);
        let read = |t: Tree| Read { trees: vec![t], pred: vec![(0, 2)], ..Default::default() };
        assert!(interrogative(&read(asked), 0));
        assert!(!interrogative(&read(said), 0));
    }
}

#[cfg(test)]
mod event_question_tests {
    use super::*;
    use {PTag as T, Rel as R, UPos as U};

    fn tree(rows: &[(&str, &str, UPos, PTag, usize, Rel)]) -> Tree {
        Tree::of_words(rows.iter().map(|&(form, lemma, upos, tag, head, rel)| Word { form: form.into(), lemma: lemma.into(), upos, tag, feats: Default::default(), head, rel }).collect())
    }

    fn asked_verb(t: &Tree) -> (String, Ask) {
        let (fs, asked, _) = frames_of_tree(t, &mut Ctx::default(), true);
        let a = asked.first().expect("asked");
        (fs[a.frame].verb.clone(), a.what)
    }

    /// «When was the college founded?» as the parser gives it (when the predicate, «was» its copula, «founded» a
    /// modifier of the college) is read as the passive it is: the time of «found», not of «be»; and the second shape
    /// (the copula and «when» hung on the noun). Negative controls: unrepaired, the time is asked of «be»; a question
    /// without a participle («Where was the college?») is not touched.
    #[test]
    fn a_passive_when_question_asks_the_time_of_its_verb() {
        let rows = [
            ("When", "when", U::ADV, T::WRB, 0, R::Root),
            ("was", "be", U::AUX, T::VBD, 1, R::Cop),
            ("the", "the", U::DET, T::DT, 4, R::Det),
            ("college", "college", U::NOUN, T::NN, 1, R::Nsubj),
            ("founded", "found", U::VERB, T::VBN, 4, R::Acl),
            ("?", "?", U::PUNCT, T::Stop, 1, R::Punct),
        ];
        let t = tree(&rows);
        assert_eq!(asked_verb(&t), ("be".to_string(), Ask::Role(Slot::Attr)), "the parser's reading asks the time of «be»");
        let mut w = t.w.clone();
        assert!(repair_wh_passive(&mut w));
        let (v, a) = asked_verb(&Tree::of_words(w.clone()));
        assert_eq!((v.as_str(), a), ("found", Ask::Role(Slot::Attr)));
        let (fs, _, _) = frames_of_tree(&Tree::of_words(w), &mut Ctx::default(), true);
        assert!(fs.iter().any(|f| f.verb == "found" && f.patient.as_ref().is_some_and(|p| p.id == "college")));
        // the second shape: «when» and «was» on «college», the passive subject of «founded»
        let second = tree(&[
            ("When", "when", U::ADV, T::WRB, 4, R::Advmod),
            ("was", "be", U::AUX, T::VBD, 4, R::Cop),
            ("the", "the", U::DET, T::DT, 4, R::Det),
            ("college", "college", U::NOUN, T::NN, 5, R::NsubjPass),
            ("founded", "found", U::VERB, T::VBN, 0, R::Root),
            ("?", "?", U::PUNCT, T::Stop, 5, R::Punct),
        ]);
        let mut w = second.w.clone();
        assert!(repair_wh_passive(&mut w));
        assert_eq!(asked_verb(&Tree::of_words(w)).0, "found");
        // negative control: no participle
        let plain = tree(&[("Where", "where", U::ADV, T::WRB, 0, R::Root), ("was", "be", U::AUX, T::VBD, 1, R::Cop), ("the", "the", U::DET, T::DT, 4, R::Det), ("college", "college", U::NOUN, T::NN, 1, R::Nsubj), ("?", "?", U::PUNCT, T::Stop, 1, R::Punct)]);
        let mut w = plain.w.clone();
        assert!(!repair_wh_passive(&mut w));
    }

    /// «When did the founding of the college take place?» asks the time of the founding: the event noun's frame
    /// (WordNet's derivational link: found the college) is the asked one; «the opening of the stadium», a gerund the
    /// nominal table lacks, reads as «open the stadium». Negative controls: «take a place» is no event predicate (the
    /// time stays on «take»), and a gerund without an «of» argument names no event.
    #[test]
    fn an_event_noun_question_asks_the_event() {
        let q = |noun: &str, det_place: bool| {
            let mut rows = vec![
                ("When", "when", U::ADV, T::WRB, 7, R::Advmod),
                ("the", "the", U::DET, T::DT, 3, R::Det),
                (noun, noun, U::NOUN, T::NN, 7, R::Nsubj),
                ("of", "of", U::ADP, T::IN, 6, R::Case),
                ("the", "the", U::DET, T::DT, 6, R::Det),
                ("college", "college", U::NOUN, T::NN, 3, R::Nmod),
                ("took", "take", U::VERB, T::VBD, 0, R::Root),
                ("place", "place", U::NOUN, T::NN, 7, R::Obj),
                ("?", "?", U::PUNCT, T::Stop, 7, R::Punct),
            ];
            if det_place {
                // «a» before «place» (token 9 after it; no head points past token 7)
                rows.insert(7, ("a", "a", U::DET, T::DT, 9, R::Det));
            }
            tree(&rows)
        };
        let (fs, asked, _) = frames_of_tree(&q("founding", false), &mut Ctx::default(), true);
        let f = &fs[asked[0].frame];
        assert_eq!((f.verb.as_str(), f.patient.as_ref().map(|p| p.id.as_str()), asked[0].what), ("found", Some("college"), Ask::Role(Slot::Attr)));
        let (fs, asked, _) = frames_of_tree(&q("opening", false), &mut Ctx::default(), true);
        assert_eq!(fs[asked[0].frame].verb, "open", "a gerund names its verb's event");
        // negative controls
        let (fs, asked, _) = frames_of_tree(&q("founding", true), &mut Ctx::default(), true);
        assert_eq!(fs[asked[0].frame].verb, "take", "«take a place» is not «take place»");
        let alone = tree(&[("the", "the", U::DET, T::DT, 2, R::Det), ("opening", "opening", U::NOUN, T::NN, 3, R::Nsubj), ("happened", "happen", U::VERB, T::VBD, 0, R::Root)]);
        let (fs, _, _) = frames_of_tree(&alone, &mut Ctx::default(), false);
        assert!(fs.iter().all(|f| f.verb != "open"), "no argument, no event");
    }

    /// Do-support put in statement order takes the verb after the subject, never a preposition inside it; the
    /// negative control: without a preposition in the subject the order is the old one.
    #[test]
    fn statement_order_skips_a_preposition() {
        assert_eq!(statement_order("When did the founding of the college take place?").as_deref(), Some("When the founding of the college took place?"));
        assert_eq!(statement_order("What did the population of the valley grow?").as_deref(), Some("the population of the valley grew What?"));
        assert_eq!(statement_order("What did the children like?").as_deref(), Some("the children liked What?"));
    }
}
