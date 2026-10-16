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
//!   carries — its patient), `cause` and `next` (indices into `frames`), `cond` (an «if» clause), `quoted`.
//! - Links: «because / since / as», a purpose «to …» → `cause`; «so / therefore» → the previous clause is the
//!   cause; «and / then / after / before / when / until», consecutive sentences → `next` (the maxim of manner:
//!   told in order = happened in order); ccomp of «say / think / know …» and quoted speech → `quote`.
//! - Entities (`Ent`) are compared by head lemma (`Ent::same`): «the Cheshire Cat» = «the Cat» = cat; pronouns
//!   carry what they were resolved from. Unresolved «it / they» match nothing.
//! - Questions (`question = true`): the first wh-word gives `Asked { frame, what: Ask }` — Cause (why), Next
//!   (what happened next), Role(slot) (who → agent, what → patient, what … do → verb, where → place, how → attr),
//!   Frame (yes/no, when, imperatives). Do-support is put in statement order before parsing (`statement_order`).
//! - Deterministic: word order, ordered maps, no hashing; the same text and ctx give the same frames.
//!
//! Not yet: a coref model (the `coref` crate's sieve works on whole documents; here a salience heuristic, babi-style),
//! tense and time, quantities (world's mentions), modality beyond `cond` and Fact(Can).

use std::collections::{BTreeMap, BTreeSet};

use en::annotate::Word;
pub use en::gram::{Rel, Tag as PTag, UPos};

use crate::schema::Act;

/// How a general statement relates its subject to its object (the links of world's know.rs, which reads the
/// same statements into facts).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Event,
    Fact(Link),
    /// speech or thought, with the speech act its verb performs (prag::schema::Act)
    Say(Act),
    Bare,
}

/// The roles a frame can fill — and the slots a question can ask about.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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
        !UNRESOLVED.contains(&self.id.as_str()) && (self.id == o.id || self.and.contains(&o.id) || o.and.contains(&self.id))
    }
}

/// Pronouns left unresolved: they match nothing («it» with no antecedent says nothing about which «it»).
pub(crate) const UNRESOLVED: [&str; 6] = ["it", "they", "he", "she", "this", "that"];
/// The same list, for readers outside the crate (the student's numbered frames).
pub const UNRESOLVED_PUB: [&str; 6] = UNRESOLVED;

#[derive(Clone, Debug, PartialEq, Eq)]
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
    /// (lesson number, line index in the lesson), set by the reader
    pub src: (usize, usize),
    /// what assigned the roles: a VerbNet class and frame, a noun naming an event (WordNet), or the tree heuristics
    pub how: How,
}

impl Frame {
    pub(crate) fn new(kind: Kind, verb: String) -> Frame {
        Frame { kind, agent: None, verb, neg: false, patient: None, recipient: None, place: None, attrs: Vec::new(), obls: Vec::new(), quote: Vec::new(), quoted: false, cause: None, next: None, cond: false, src: (0, 0), how: How::default() }
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
        if !links.is_empty() {
            s += &format!(" | {}", links.join(" "));
        }
        s
    }
}

/// Where a frame's roles came from (`crate::vn`): shown by `world frames` and `trace`, an atom of the learned rules.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct How {
    pub by: By,
    /// the VerbNet class («give-13.1-1») or the noun («arrival»)
    pub class: String,
    /// the VerbNet frame's pattern («NP V NP PP.recipient»)
    pub frame: String,
    /// the lexicon's role of each argument: `agent=Agent patient=Theme to=Recipient with=Instrument`
    pub roles: Vec<(String, String)>,
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

/// Roles of a prepositional phrase that make it no place even when its preposition is spatial («look at the cat»,
/// «sting with tentacles»); the rest (Asset, Value, Extent …) are too often an adjunct matched by chance.
const NOT_PLACE_ROLES: [&str; 13] = ["theme", "patient", "topic", "stimulus", "instrument", "co-agent", "co-theme", "co-patient", "experiencer", "agent", "ppt", "pag", "attribute"];

// ---- the discourse: who «it» and «she» are ------------------------------------------------------------------------

/// The entities met so far, with salience (raised by every mention by its role, halved at every new sentence).
#[derive(Clone, Debug, Default)]
pub struct Ctx {
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
        let s = self.seen.entry(e.id.clone()).or_insert(Seen { ent: Ent { pron: None, ..e.clone() }, salience: 0, last: 0, role: 1, this_sent: 0, animate: false, person: false, thing: false, gender: g });
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
            if id.chars().next().is_some_and(|c| c.is_ascii_digit()) || TIME.contains(&id) {
                return false;
            }
            let plural = s.ent.plural || !s.ent.and.is_empty();
            match pron {
                "he" | "him" | "his" | "himself" | "she" | "her" | "hers" | "herself" => {
                    let want = if pron.starts_with('h') && pron != "her" && pron != "hers" && pron != "herself" { 1 } else { 2 };
                    let lex = lex_animate(id);
                    !plural && !s.thing && (s.gender == 0 || s.gender == want) && (s.person || s.ent.named || lex == Some(true) || s.animate && lex != Some(false))
                }
                "it" | "its" | "itself" => !plural && !s.ent.named && !s.person && s.gender == 0,
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
        || TIME.contains(&id))
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
        UPos::VERB => true,
        UPos::ADJ | UPos::NOUN | UPos::PROPN | UPos::ADV | UPos::NUM | UPos::PRON => {
            has_cop(i)
                || (matches!(t.upos(i), UPos::NOUN) && t.w[i].rel != Rel::Root && t.kids(i).iter().any(|&k| t.w[k].rel == Rel::Nsubj) && t.kids_of(i, Rel::Case).is_empty())
                || (t.base(i) == Rel::Conj && matches!(t.upos(i), UPos::ADJ) && t.head(i).is_some_and(has_cop))
        }
        _ => false,
    }
}

fn negated(t: &Tree, v: usize) -> bool {
    t.kids(v).iter().any(|&k| matches!(t.lemma(k), "not" | "n't" | "never" | "no") && matches!(t.base(k), Rel::Advmod | Rel::Aux))
        || t.kids(v).iter().any(|&k| t.base(k) == Rel::Aux && (t.form(k).ends_with("n't") || t.form(k) == "cannot"))
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
        let e = self.ent_at(n, animate);
        self.depth -= 1;
        e
    }

    fn ent_at(&mut self, n: usize, animate: bool) -> Option<Ent> {
        let t = self.t;
        let lemma = t.lemma(n).to_string();
        if self.wh.contains(&n) {
            return None;
        }
        if t.upos(n) == UPos::PRON || t.upos(n) == UPos::DET && t.base(n) != Rel::Det {
            if WH.contains(&lemma.as_str()) || matches!(lemma.as_str(), "that" | "which") && t.head(n).is_some_and(|h| t.base(h) == Rel::Acl) {
                // a relative pronoun: the noun its clause belongs to
                let h = t.head(n)?;
                let m = t.head(h)?;
                return (matches!(t.upos(m), UPos::NOUN | UPos::PROPN)).then(|| self.ent(m, animate)).flatten();
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
            // «something», «everyone», «this»: an entity by its word
            return Some(Ent { id: norm.into(), text: t.form(n).into(), ..Default::default() });
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
        let and: Vec<String> = t.kids_of(n, Rel::Conj).into_iter().filter(|&c| matches!(t.upos(c), UPos::NOUN | UPos::PROPN)).map(|c| t.lemma(c).to_string()).collect();
        let id = if t.upos(n) == UPos::NUM { t.form(n).to_string() } else { lemma };
        let e = Ent { id, text, pron: None, plural: plural || !and.is_empty(), named, attrs, and };
        // the possessor is read first («its nose»: «its» is resolved before the nose is met, the baby stays salient)
        for k in t.kids(n).to_vec() {
            if t.w[k].rel == Rel::NmodPoss {
                let _ = self.ent(k, false);
            }
        }
        let role = match t.base(n) {
            Rel::Nsubj => 3,
            Rel::Obj | Rel::Iobj => 2,
            _ => 1,
        };
        self.ctx.mention_as(&e, animate, role);
        for c in t.kids_of(n, Rel::Conj) {
            if matches!(t.upos(c), UPos::NOUN | UPos::PROPN) {
                let _ = self.ent(c, animate);
            }
        }
        Some(e)
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
        let saved = self.ctx.clone();
        let e = self.ent(k, false);
        *self.ctx = saved;
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
        let time = |k: usize| t.upos(k) == UPos::NOUN && TIME.contains(&t.lemma(k));
        let mut obj_toks: Vec<usize> = kids.iter().copied().filter(|&k| t.base(k) == Rel::Iobj && !time(k)).collect();
        obj_toks.extend(kids.iter().copied().filter(|&k| t.base(k) == Rel::Obj && Some(k) != speaker_obj && !time(k) && !is_pred(t, k)));
        if let Some(k) = phrasal_obj {
            obj_toks.push(k);
        }
        cl.objs = obj_toks.iter().map(|&k| self.filler(k)).collect();
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
                if t.w[j].rel == Rel::Nmod && t.kids_of(j, Rel::Case).first().is_some_and(|&c| t.lemma(c) != "of") {
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
            for (k, r) in obj_toks.iter().zip(&m.objs) {
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
            roles_pp = m.pps.iter().zip(&cl.pps).map(|(r, (_, fl))| r.as_ref().map(|r| (role_to(r, fl.animate), r.clone()))).collect();
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
                RoleTo::Patient | RoleTo::Obl | RoleTo::Agent if !off("vn-notplace") && NOT_PLACE_ROLES.contains(&rname.to_lowercase().as_str()) => {
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
        let nom = crate::vn::nom(t.lemma(n))?;
        let transitive = nom.transitive;
        let mut f = Frame::new(Kind::Event, nom.verb.clone());
        f.how = How { by: By::Nominal, class: nom.noun.clone(), frame: format!("{} {}", nom.file, if transitive { "transitive" } else { "intransitive" }), roles: Vec::new() };
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
        if let Some(&prt) = kids.iter().find(|&&k| t.w[k].rel == Rel::CompoundPrt || k > p && k <= p + 3 && matches!(t.upos(k), UPos::ADV | UPos::ADP) && PARTICLES.contains(&t.lemma(k)) && matches!(t.base(k), Rel::Advmod | Rel::Xcomp) && t.kids(k).is_empty()) {
            if !has_cop {
                verb += &format!(" {}", t.lemma(prt));
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
                    phrasal_obj = t.head(k).filter(|&n| t.base(k) == Rel::Case && t.head(n) == Some(p) && matches!(t.base(n), Rel::Obl | Rel::Nmod));
                }
            }
        }
        let mut f = Frame::new(Kind::Event, verb.clone());
        f.neg = negated(t, p);
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
        if subj.is_none() && !off("imperative") && t.w[p].rel == Rel::Root && t.tag(p) == PTag::VB && !kids.iter().any(|&k| matches!(t.base(k), Rel::Aux | Rel::Mark | Rel::Expl)) && !has_cop {
            f.agent = Some(Ent { id: "you".into(), text: "you".into(), ..Default::default() });
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
            // a time is not a role: it goes to the attributes as «when:…»
            if matches!(r.base(), Rel::Obj | Rel::Obl | Rel::Nmod) && t.upos(k) == UPos::NOUN && TIME.contains(&t.lemma(k)) {
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
                            if t.w[j].rel == Rel::Nmod && !is_pred(t, j) {
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
                    }
                }
                Rel::Xcomp if matches!(t.upos(k), UPos::ADJ) => f.attrs.push(t.lemma(k).to_string()),
                Rel::Xcomp if matches!(t.upos(k), UPos::NOUN) && f.patient.is_none() => f.patient = self.ent(k, false),
                Rel::Advmod if t.upos(k) == UPos::ADV && !NOT_MANNER.contains(&t.lemma(k)) && t.lemma(k).len() > 2 => f.attrs.push(t.lemma(k).to_string()),
                _ => {}
            }
        }
        // the linguists' rules: the verb's VerbNet frame names each argument's role
        if !has_cop && t.upos(p) == UPos::VERB && !off("vn") && !(self.question && vn_opt("noq")) {
            self.lexicon_roles(p, &mut f, subj, speaker_obj, phrasal_obj, passive);
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
            if TIME.contains(&l.as_str()) {
                continue;
            }
            let det = t.kids_of(i, Rel::Det).first().map(|&d| t.lemma(d).to_string());
            let named = t.upos(i) == UPos::PROPN && det.is_none();
            let and: Vec<String> = t.kids_of(i, Rel::Conj).into_iter().filter(|&c| matches!(t.upos(c), UPos::NOUN | UPos::PROPN)).map(|c| t.lemma(c).to_string()).collect();
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
                if f.agent.is_none() {
                    f.agent = ag;
                }
                if f.patient.is_none() && above.is_some_and(|h| fs[h].patient.as_ref().is_some_and(|x| e.as_ref().is_some_and(|e| x.id == e.id))) {
                    fs[fi].patient = e;
                }
            }
        } else if t.base(p) == Rel::Acl {
            if let Some(m) = t.head(p).filter(|&m| matches!(t.upos(m), UPos::NOUN | UPos::PROPN)) {
                let e = r.ent(m, false);
                let f = &mut fs[fi];
                if f.agent.is_none() && f.patient.is_some() || f.agent.is_none() && t.w[p].tag != PTag::VBN {
                    f.agent = e;
                } else if f.patient.is_none() {
                    f.patient = e;
                }
            }
        }
        let Some(h) = up(p) else { continue };
        let hi = at[&h];
        let rel = t.base(p);
        let marks: Vec<String> = t.kids(p).iter().filter(|&&k| t.base(k) == Rel::Mark).map(|&k| t.lemma(k).to_string()).collect();
        let advs: Vec<String> = t.kids(p).iter().filter(|&&k| matches!(t.base(k), Rel::Advmod | Rel::Cc)).map(|&k| t.lemma(k).to_string()).collect();
        let m = |w: &str| marks.iter().any(|x| x == w);
        let a = |w: &str| advs.iter().any(|x| x == w);
        // a clause without its own subject shares the head's (coordinated predicates, purposes, control)
        if fs[fi].agent.is_none() && matches!(rel, Rel::Conj | Rel::Xcomp | Rel::Advcl | Rel::Ccomp) && !t.kids(p).iter().any(|&k| t.base(k) == Rel::Nsubj) && !(rel == Rel::Ccomp && (is_say(t.lemma(h)) || CONTENT.contains(&t.lemma(h)))) {
            // object control: «make the mould grow», «tell Alice to go» — the doer is the head's object
            let ag = if rel == Rel::Xcomp && OBJ_CONTROL.contains(&t.lemma(h)) && fs[hi].patient.is_some() && !off("obj-control") { fs[hi].patient.clone() } else { fs[hi].agent.clone() };
            let ag = ag.filter(|a| !(a.id == "it" && a.pron.is_some()));
            fs[fi].agent = ag;
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
    // names given in apposition («a bough (a branch)»): the same thing
    let aliases: Vec<(String, String)> = (0..n).filter(|&i| t.base(i) == Rel::Appos && matches!(t.upos(i), UPos::NOUN | UPos::PROPN)).filter_map(|i| t.head(i).filter(|&h| matches!(t.upos(h), UPos::NOUN | UPos::PROPN)).map(|h| (t.lemma(h).to_string(), t.lemma(i).to_string()))).collect();
    (fs, asked, aliases, noms)
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
                f.obls.push((case, e));
            } else {
                f.patient = Some(e);
            }
            // «A Hatter and a March Hare», «the chimneys shaped like ears»
            f.attrs = (0..t.len()).filter(|&i| t.upos(i) == UPos::ADJ && t.head(i) == Some(root)).map(|i| t.lemma(i).to_string()).collect();
        }
    }
    f.neg = (0..t.len()).any(|i| matches!(t.lemma(i), "not" | "never" | "n't" | "no"));
    Some(f)
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
    let v = (d + 2..low.len()).find(|&i| can_be_verb(&low[i]) && !DETS.contains(&low[i - 1].as_str()) && !toks[i].chars().next().is_some_and(char::is_uppercase) && !low[i - 1].ends_with("'s"))
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
    s.trim().to_string()
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
        f
    }).collect()
}
