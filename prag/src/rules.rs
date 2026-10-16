//! Learned rules of the pragmatic reader: transformations applied to the frames after `frames::read_with`, each a
//! sentence a person can read («if the predicate is the conj of the word above it and has no agent, its agent = the
//! agent of the frame above»). The SLM learns them itself, error-driven (after Brill 1995, as `world::induce` learns
//! tree repairs), from the frame bench's dev gold and the teacher's verdicts (`world frames rules learn`); a checker
//! reads the queue and marks each rule accepted or rejected (docs/pragmatics.md).
//!
//! A rule = a conjunction of 1–3 atoms (features of a site: from the UD tree, the frame, the level-1 lists — verbs of
//! saying and knowing, control, global's verb classes and animacy) + one action. Sites: a frame, a pronoun in a role,
//! a participle on a noun that has no frame.
//!
//! The rules file (`prag/data/rules.tsv`, compiled in; `PRAG_RULES_FILE` reads another) has a status per rule:
//! proposed / accepted / rejected. `PRAG_RULES` unset → only accepted rules; `proposed` → accepted and proposed (for
//! measuring); `off` → none. Rules apply in file order. Deterministic: ordered maps, word order.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex, OnceLock};

use crate::frames::{CONTENT, CONTROL, Ent, FEM, Frame, Kind, MASC, OBJ_CONTROL, PERSONAL, PTag, Read, Rel, Tree, UNRESOLVED, UPos, is_pred, is_say, lex_animate};
use crate::schema::Act;

/// A feature of a site.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Feat {
    /// the predicate's relation to the word above it
    Rel,
    /// the predicate word's tag
    Tag,
    /// the frame's kind (event, fact, say, bare)
    Kind,
    /// the kind of the frame above (none)
    HeadKind,
    /// the verb above: say / know / control / objcontrol / other / none
    HeadCls,
    /// the frame's own verb: say / know / control / objcontrol / other
    Cls,
    /// global's verb class of the verb (level 1)
    GCls,
    /// the verb lemma
    Lemma,
    HasAgent,
    HasPatient,
    HasPlace,
    HasTo,
    /// the predicate has its own subject in the tree
    HasNsubj,
    /// the predicate has its own object in the tree
    HasObj,
    /// what the agent is: none / pron / personal / name / noun / unresolved
    Agent,
    /// the agent's animacy: animate / thing / unknown
    AgentAnim,
    Patient,
    PatientAnim,
    /// a marker word of the clause (mark: because, if, to, when …)
    Mark,
    /// a linking adverb or conjunction of the clause (then, so, and, but …)
    Adv,
    /// the predicate comes before the word above it: yes / no / top
    Before,
    Quoted,
    Cond,
    Neg,
    /// the preposition of an oblique role
    Obl,
    /// the word above is a noun (a participle or clause on a noun)
    OnNoun,
    /// the first frame of its sentence
    First,
    /// the kind of the frame just before in the line
    PrevKind,
    HasCause,
    HasNext,
    /// the frame above has an agent
    HeadHasAgent,
    /// the frame is in what a Say frame carries
    Carried,
    // ---- a pronoun in a role
    Pron,
    Slot,
    /// what the pronoun is resolved to now: unresolved / name / noun / person
    Cur,
    CurAnim,
    CurGender,
    /// the candidate the action brings: none / same (as now) / name / noun
    Cand,
    CandAnim,
    CandGender,
    /// the candidate is this frame's own agent
    CandIsAgent,
    /// the VerbNet class of the verb's matched frame (its top class: «give-13.1»), or none
    VnClass,
    /// the matched VerbNet frame's pattern («NP V NP PP.recipient»)
    VnFrame,
    /// what assigned the roles: vn / pb / nb / heuristic
    RolesBy,
    /// the lexicon's role of the agent (Agent, Theme, Experiencer …)
    VnAgent,
    // ---- a participle on a noun
    NounAnim,
}

pub const FEATS: [Feat; 46] = [
    Feat::Rel, Feat::Tag, Feat::Kind, Feat::HeadKind, Feat::HeadCls, Feat::Cls, Feat::GCls, Feat::Lemma, Feat::HasAgent, Feat::HasPatient, Feat::HasPlace, Feat::HasTo, Feat::HasNsubj, Feat::HasObj, Feat::Agent, Feat::AgentAnim, Feat::Patient, Feat::PatientAnim, Feat::Mark, Feat::Adv, Feat::Before, Feat::Quoted, Feat::Cond, Feat::Neg, Feat::Obl, Feat::OnNoun, Feat::First, Feat::PrevKind, Feat::HasCause, Feat::HasNext, Feat::HeadHasAgent, Feat::Carried, Feat::Pron, Feat::Slot, Feat::Cur, Feat::CurAnim, Feat::CurGender, Feat::Cand, Feat::CandAnim, Feat::CandGender, Feat::CandIsAgent, Feat::NounAnim, Feat::VnClass, Feat::VnFrame, Feat::RolesBy, Feat::VnAgent,
];

impl Feat {
    pub fn name(self) -> &'static str {
        match self {
            Feat::Rel => "rel",
            Feat::Tag => "tag",
            Feat::Kind => "kind",
            Feat::HeadKind => "head-kind",
            Feat::HeadCls => "head-cls",
            Feat::Cls => "cls",
            Feat::GCls => "gcls",
            Feat::Lemma => "lemma",
            Feat::HasAgent => "has-agent",
            Feat::HasPatient => "has-patient",
            Feat::HasPlace => "has-place",
            Feat::HasTo => "has-to",
            Feat::HasNsubj => "has-nsubj",
            Feat::HasObj => "has-obj",
            Feat::Agent => "agent",
            Feat::AgentAnim => "agent-anim",
            Feat::Patient => "patient",
            Feat::PatientAnim => "patient-anim",
            Feat::Mark => "mark",
            Feat::Adv => "adv",
            Feat::Before => "before",
            Feat::Quoted => "quoted",
            Feat::Cond => "cond",
            Feat::Neg => "neg",
            Feat::Obl => "obl",
            Feat::OnNoun => "on-noun",
            Feat::First => "first",
            Feat::PrevKind => "prev-kind",
            Feat::HasCause => "has-cause",
            Feat::HasNext => "has-next",
            Feat::HeadHasAgent => "head-has-agent",
            Feat::Carried => "carried",
            Feat::Pron => "pron",
            Feat::Slot => "slot",
            Feat::Cur => "cur",
            Feat::CurAnim => "cur-anim",
            Feat::CurGender => "cur-gender",
            Feat::Cand => "cand",
            Feat::CandAnim => "cand-anim",
            Feat::CandGender => "cand-gender",
            Feat::CandIsAgent => "cand-is-agent",
            Feat::NounAnim => "noun-anim",
            Feat::VnClass => "vn-class",
            Feat::VnFrame => "vn-frame",
            Feat::RolesBy => "roles-by",
            Feat::VnAgent => "vn-agent",
        }
    }

    pub fn by_name(s: &str) -> Option<Feat> {
        FEATS.iter().copied().find(|f| f.name() == s)
    }

    /// The atom as a phrase of the rule's sentence.
    pub fn say(self, v: &str) -> String {
        let yes = v == "yes";
        let has = |what: &str| if yes { format!("it has {what}") } else { format!("it has no {what}") };
        match self {
            Feat::Rel => format!("the predicate is the {v} of the word above it"),
            Feat::Tag => format!("the predicate word is tagged {v}"),
            Feat::Kind => format!("the frame is a {v} frame"),
            Feat::HeadKind => if v == "none" { "there is no frame above".into() } else { format!("the frame above is a {v} frame") },
            Feat::HeadCls => format!("the verb above is {}", cls_say(v)),
            Feat::Cls => format!("the verb is {}", cls_say(v)),
            Feat::GCls => format!("the verb is of the class «{v}»"),
            Feat::Lemma => format!("the verb is «{v}»"),
            Feat::HasAgent => has("agent"),
            Feat::HasPatient => has("patient"),
            Feat::HasPlace => has("place"),
            Feat::HasTo => has("recipient"),
            Feat::HasNsubj => if yes { "the predicate has its own subject in the tree".into() } else { "the predicate has no subject of its own in the tree".into() },
            Feat::HasObj => if yes { "the predicate has its own object in the tree".into() } else { "the predicate has no object in the tree".into() },
            Feat::Agent => format!("the agent is {}", ent_say(v)),
            Feat::AgentAnim => format!("the agent is {}", anim_say(v)),
            Feat::Patient => format!("the patient is {}", ent_say(v)),
            Feat::PatientAnim => format!("the patient is {}", anim_say(v)),
            Feat::Mark => format!("the clause is marked by «{v}»"),
            Feat::Adv => format!("the clause has «{v}»"),
            Feat::Before => match v {
                "yes" => "the predicate comes before the word above it".into(),
                "no" => "the predicate comes after the word above it".into(),
                _ => "the predicate is the top of its sentence".into(),
            },
            Feat::Quoted => if yes { "it is inside quotation marks".into() } else { "it is not inside quotation marks".into() },
            Feat::Cond => if yes { "it is inside an «if» clause".into() } else { "it is not a condition".into() },
            Feat::Neg => if yes { "it is negated".into() } else { "it is not negated".into() },
            Feat::Obl => format!("it has an oblique role with «{v}»"),
            Feat::OnNoun => if yes { "the word above is a noun".into() } else { "the word above is not a noun".into() },
            Feat::First => if yes { "it is the first frame of its sentence".into() } else { "it is not the first frame of its sentence".into() },
            Feat::PrevKind => if v == "none" { "no frame comes before it in the line".into() } else { format!("the frame just before it is a {v} frame") },
            Feat::HasCause => has("cause"),
            Feat::HasNext => has("«next»"),
            Feat::HeadHasAgent => if yes { "the frame above has an agent".into() } else { "the frame above has no agent".into() },
            Feat::Carried => if yes { "it is carried by a Say frame (what was said or thought)".into() } else { "no Say frame carries it".into() },
            Feat::Pron => format!("the pronoun is «{v}»"),
            Feat::Slot => format!("the pronoun is the {v}"),
            Feat::Cur => format!("the pronoun is now resolved to {}", ent_say(v)),
            Feat::CurAnim => format!("what it is resolved to now is {}", anim_say(v)),
            Feat::CurGender => format!("what it is resolved to now is {}", gender_say(v)),
            Feat::Cand => match v {
                "none" => "the candidate does not exist".into(),
                "same" => "the candidate is what it is resolved to now".into(),
                _ => format!("the candidate is {}", ent_say(v)),
            },
            Feat::CandAnim => format!("the candidate is {}", anim_say(v)),
            Feat::CandGender => format!("the candidate is {}", gender_say(v)),
            Feat::CandIsAgent => if yes { "the candidate is this frame's own agent".into() } else { "the candidate is not this frame's own agent".into() },
            Feat::NounAnim => format!("the noun is {}", anim_say(v)),
            Feat::VnClass => if v == "none" { "VerbNet has no frame for it".into() } else { format!("its VerbNet class is {v}") },
            Feat::VnFrame => format!("its VerbNet frame is «{v}»"),
            Feat::RolesBy => format!("its roles were assigned by {}", match v { "vn" => "VerbNet", "nom" => "a noun naming an event (WordNet)", _ => "the tree heuristics" }),
            Feat::VnAgent => format!("VerbNet calls its agent the {v}"),
        }
    }
}

fn cls_say(v: &str) -> String {
    match v {
        "say" => "a verb of saying or thinking".into(),
        "know" => "a verb of knowing or perceiving".into(),
        "control" => "a control verb (want, try, begin …)".into(),
        "objcontrol" => "an object-control verb (make, let, tell …)".into(),
        "none" => "missing".into(),
        _ => "an ordinary verb".into(),
    }
}

fn ent_say(v: &str) -> String {
    match v {
        "none" => "missing".into(),
        "pron" => "a resolved pronoun".into(),
        "personal" => "I / you / we".into(),
        "name" => "a name".into(),
        "noun" => "a common noun".into(),
        "unresolved" => "an unresolved pronoun".into(),
        "person" => "a person word".into(),
        x => x.into(),
    }
}

fn anim_say(v: &str) -> String {
    match v {
        "animate" => "animate (level 1)".into(),
        "thing" => "a thing (level 1)".into(),
        _ => "of unknown animacy".into(),
    }
}

fn gender_say(v: &str) -> String {
    match v {
        "m" => "masculine".into(),
        "f" => "feminine".into(),
        _ => "of no known gender".into(),
    }
}

/// A feature with its value: `rel:conj`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Atom {
    pub f: Feat,
    pub v: String,
}

impl Atom {
    fn new(f: Feat, v: impl Into<String>) -> Atom {
        Atom { f, v: v.into() }
    }
    pub fn code(&self) -> String {
        format!("{}:{}", self.f.name(), self.v)
    }
    pub fn parse(s: &str) -> Option<Atom> {
        let (f, v) = s.split_once(':')?;
        Some(Atom { f: Feat::by_name(f)?, v: v.to_string() })
    }
}

/// Which candidate a pronoun is given.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cand {
    /// the agent of the nearest earlier frame of the line with a real agent
    PrevAgent,
    /// the patient of the nearest earlier frame of the line with a real patient
    PrevPatient,
    /// the agent of the line's first frame
    LineAgent,
    /// the speaker: the agent of the nearest earlier Say frame of the line, else the discourse's last speaker
    Speaker,
    /// the agent of the frame above
    HeadAgent,
    /// the most salient entity before this line that agrees in number
    Before,
    /// the most salient person (animate or named) before this line
    BeforePerson,
    /// nothing: the pronoun stays itself
    Unresolve,
}

pub const CANDS: [Cand; 8] = [Cand::PrevAgent, Cand::PrevPatient, Cand::LineAgent, Cand::Speaker, Cand::HeadAgent, Cand::Before, Cand::BeforePerson, Cand::Unresolve];

impl Cand {
    fn name(self) -> &'static str {
        match self {
            Cand::PrevAgent => "prev-agent",
            Cand::PrevPatient => "prev-patient",
            Cand::LineAgent => "line-agent",
            Cand::Speaker => "speaker",
            Cand::HeadAgent => "head-agent",
            Cand::Before => "before",
            Cand::BeforePerson => "before-person",
            Cand::Unresolve => "unresolve",
        }
    }
    fn say(self) -> &'static str {
        match self {
            Cand::PrevAgent => "the agent of the nearest frame before it in the line",
            Cand::PrevPatient => "the patient of the nearest frame before it in the line",
            Cand::LineAgent => "the agent of the line's first frame",
            Cand::Speaker => "the speaker (the nearest Say frame before it, else the last speaker)",
            Cand::HeadAgent => "the agent of the frame above",
            Cand::Before => "the most salient entity of the lines before that agrees in number",
            Cand::BeforePerson => "the most salient person of the lines before",
            Cand::Unresolve => "nothing (the pronoun stays unresolved)",
        }
    }
}

/// What a rule does at its site.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Action {
    AgentFromHead,
    AgentFromHeadPatient,
    AgentFromNoun,
    AgentFromPatient,
    AgentNone,
    PatientNone,
    PlaceNone,
    ToNone,
    PatientToTo,
    ToToPatient,
    OblToPlace,
    OblToPatient,
    PlaceToPatient,
    Swap,
    Drop,
    KindEvent,
    KindSay(Act),
    CauseHeadThis,
    CauseThisHead,
    NextHeadThis,
    NextThisHead,
    NextPrevThis,
    CauseThisPrev,
    ClearCause,
    ClearNext,
    SaysHeadThis,
    SaysRemove,
    // a pronoun in a role
    Resolve(Cand),
    // a participle on a noun with no frame
    // moves that add structure (10.10, the checker: prefer them to deletions)
    AgentFromBy,
    ToFromObl,
    AddNounAgent,
    AddNounPatient,
}

pub const SAY_ACTS: [Act; 4] = [Act::Assert, Act::Ask, Act::Express, Act::Request];

pub fn frame_actions() -> Vec<Action> {
    let mut v = vec![
        Action::AgentFromHead, Action::AgentFromHeadPatient, Action::AgentFromNoun, Action::AgentFromPatient, Action::AgentNone, Action::PatientNone, Action::PlaceNone, Action::ToNone, Action::PatientToTo, Action::ToToPatient, Action::OblToPlace, Action::OblToPatient, Action::PlaceToPatient, Action::Swap, Action::Drop, Action::KindEvent, Action::AgentFromBy, Action::ToFromObl,
    ];
    v.extend(SAY_ACTS.iter().map(|a| Action::KindSay(*a)));
    v.extend([Action::CauseHeadThis, Action::CauseThisHead, Action::NextHeadThis, Action::NextThisHead, Action::NextPrevThis, Action::CauseThisPrev, Action::ClearCause, Action::ClearNext, Action::SaysHeadThis, Action::SaysRemove]);
    v
}

pub fn ent_actions() -> Vec<Action> {
    CANDS.iter().map(|c| Action::Resolve(*c)).collect()
}

pub const TOKEN_ACTIONS: [Action; 2] = [Action::AddNounAgent, Action::AddNounPatient];

impl Action {
    /// An action that only removes structure (a role, a link, a frame, speech).
    pub fn deletion(self) -> bool {
        matches!(self, Action::AgentNone | Action::PatientNone | Action::PlaceNone | Action::ToNone | Action::Drop | Action::KindEvent | Action::ClearCause | Action::ClearNext | Action::SaysRemove)
    }
    pub fn code(self) -> String {
        match self {
            Action::KindSay(a) => format!("kind-say-{}", a.name()),
            Action::Resolve(c) => format!("resolve-{}", c.name()),
            a => format!("{a:?}").chars().fold(String::new(), |mut s, c| {
                if c.is_uppercase() && !s.is_empty() {
                    s.push('-');
                }
                s.push(c.to_ascii_lowercase());
                s
            }),
        }
    }
    pub fn parse(s: &str) -> Option<Action> {
        frame_actions().into_iter().chain(ent_actions()).chain(TOKEN_ACTIONS).find(|a| a.code() == s)
    }
    pub fn say(self) -> String {
        match self {
            Action::AgentFromHead => "its agent = the agent of the frame above".into(),
            Action::AgentFromHeadPatient => "its agent = the patient of the frame above".into(),
            Action::AgentFromNoun => "its agent = the noun it stands on".into(),
            Action::AgentFromPatient => "its patient becomes its agent".into(),
            Action::AgentNone => "it has no agent".into(),
            Action::PatientNone => "it has no patient".into(),
            Action::PlaceNone => "it has no place".into(),
            Action::ToNone => "it has no recipient".into(),
            Action::PatientToTo => "its patient is the recipient".into(),
            Action::ToToPatient => "its recipient is the patient".into(),
            Action::OblToPlace => "its first oblique role is the place".into(),
            Action::OblToPatient => "its first oblique role is the patient".into(),
            Action::PlaceToPatient => "its place is the patient".into(),
            Action::AgentFromBy => "its «by» phrase is the agent".into(),
            Action::ToFromObl => "its first oblique role is the recipient".into(),
            Action::Swap => "agent and patient swap".into(),
            Action::Drop => "it is not a frame".into(),
            Action::KindEvent => "it is an Event, not speech".into(),
            Action::KindSay(a) => format!("it is speech: Say({})", a.name()),
            Action::CauseHeadThis => "it is the cause of the frame above".into(),
            Action::CauseThisHead => "the frame above is its cause".into(),
            Action::NextHeadThis => "it happens next after the frame above".into(),
            Action::NextThisHead => "the frame above happens next after it".into(),
            Action::NextPrevThis => "it happens next after the frame just before it".into(),
            Action::CauseThisPrev => "the frame just before it is its cause".into(),
            Action::ClearCause => "it has no cause".into(),
            Action::ClearNext => "it has no «next»".into(),
            Action::SaysHeadThis => "it is what the frame above says".into(),
            Action::SaysRemove => "it is not what any frame says".into(),
            Action::Resolve(c) => format!("the pronoun is {}", c.say()),
            Action::AddNounAgent => "it is an event of its own: the noun does it".into(),
            Action::AddNounPatient => "it is an event of its own: done to the noun".into(),
        }
    }
}

/// A rule: atoms → action.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rule {
    pub id: String,
    pub status: String,
    pub atoms: Vec<Atom>,
    pub action: Action,
    /// the checker's reason (or the learner's note)
    pub reason: String,
}

impl Rule {
    pub fn say(&self) -> String {
        let site = match self.action {
            Action::Resolve(_) => "a pronoun in a role",
            Action::AddNounAgent | Action::AddNounPatient => "a participle on a noun without a frame",
            _ => "a frame",
        };
        let cond = self.atoms.iter().map(|a| a.f.say(&a.v)).collect::<Vec<_>>().join(" and ");
        format!("For {site}: if {cond}, then {}.", self.action.say())
    }
    pub fn atoms_code(&self) -> String {
        self.atoms.iter().map(Atom::code).collect::<Vec<_>>().join("&")
    }
}

// ---- the rules file ------------------------------------------------------------------------------------------------

pub const FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/rules.tsv");
const EMBEDDED: &str = include_str!("../data/rules.tsv");
pub const HEADER: &str = "id\tstatus\taction\tatoms\trule\ttrain_fixed\ttrain_broken\treason";

/// Parse a rules file (the header and unreadable lines are skipped).
pub fn parse_rules(text: &str) -> Vec<Rule> {
    let mut out = Vec::new();
    for l in text.lines().skip(1) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() < 4 {
            continue;
        }
        let Some(action) = Action::parse(c[2]) else { continue };
        let atoms: Option<Vec<Atom>> = c[3].split('&').filter(|x| !x.is_empty()).map(Atom::parse).collect();
        let Some(atoms) = atoms else { continue };
        out.push(Rule { id: c[0].into(), status: c[1].into(), atoms, action, reason: c.get(7).map(|x| x.to_string()).unwrap_or_default() });
    }
    out
}

static OVERRIDE: Mutex<Option<Arc<Vec<Rule>>>> = Mutex::new(None);
static LOADED: OnceLock<Arc<Vec<Rule>>> = OnceLock::new();

/// Use these rules instead of the file's (the learner; None = back to the file).
pub fn set_override(rules: Option<Vec<Rule>>) {
    *OVERRIDE.lock().unwrap() = rules.map(Arc::new);
}

/// The rules the reader applies: accepted ones (PRAG_RULES=proposed adds the proposed, off none).
pub fn active() -> Arc<Vec<Rule>> {
    if let Some(r) = OVERRIDE.lock().unwrap().clone() {
        return r;
    }
    LOADED.get_or_init(|| {
        let mode = std::env::var("PRAG_RULES").unwrap_or_default();
        if mode == "off" {
            return Arc::new(Vec::new());
        }
        let text = std::env::var("PRAG_RULES_FILE").ok().and_then(|p| std::fs::read_to_string(p).ok()).unwrap_or_else(|| EMBEDDED.to_string());
        Arc::new(parse_rules(&text).into_iter().filter(|r| r.status == "accepted" || mode == "proposed" && r.status == "proposed").collect())
    }).clone()
}

// ---- sites and their atoms -----------------------------------------------------------------------------------------

/// Where a rule acts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Site {
    Frame(usize),
    /// a pronoun: frame, role (0 agent, 1 patient, 2 recipient, 3 place, 4+k the k-th oblique)
    Ent(usize, usize),
    /// a participle on a noun: (sentence, token)
    Token(usize, usize),
}

fn site_kind(a: Action) -> u8 {
    match a {
        Action::Resolve(_) => 1,
        Action::AddNounAgent | Action::AddNounPatient => 2,
        _ => 0,
    }
}

const COREF_PRONS: [&str; 13] = ["he", "him", "his", "himself", "she", "her", "herself", "it", "its", "they", "them", "their", "themselves"];

fn real(e: &Option<Ent>) -> Option<&Ent> {
    e.as_ref().filter(|e| !UNRESOLVED.contains(&e.id.as_str()) && !PERSONAL.contains(&e.id.as_str()))
}

fn kind_name(k: Kind) -> &'static str {
    match k {
        Kind::Event => "event",
        Kind::Fact(_) => "fact",
        Kind::Say(_) => "say",
        Kind::Bare => "bare",
    }
}

fn cls(l: &str) -> &'static str {
    if is_say(l) {
        "say"
    } else if CONTENT.contains(&l) {
        "know"
    } else if OBJ_CONTROL.contains(&l) {
        "objcontrol"
    } else if CONTROL.contains(&l) {
        "control"
    } else {
        "other"
    }
}

fn anim(id: &str) -> &'static str {
    match lex_animate(id) {
        Some(true) => "animate",
        Some(false) => "thing",
        None => {
            if FEM.contains(&id) || MASC.contains(&id) {
                "animate"
            } else {
                "unknown"
            }
        }
    }
}

fn gender(e: &Ent) -> &'static str {
    let id = e.id.as_str();
    if FEM.contains(&id) || matches!(e.pron.as_deref(), Some("she" | "her" | "herself")) {
        "f"
    } else if MASC.contains(&id) || matches!(e.pron.as_deref(), Some("he" | "him" | "his" | "himself")) {
        "m"
    } else {
        "none"
    }
}

fn ent_form(e: &Option<Ent>) -> &'static str {
    match e {
        None => "none",
        Some(e) if PERSONAL.contains(&e.id.as_str()) => "personal",
        Some(e) if UNRESOLVED.contains(&e.id.as_str()) => "unresolved",
        Some(e) if e.pron.is_some() => "pron",
        Some(e) if e.named => "name",
        Some(_) => "noun",
    }
}

fn yn(b: bool) -> &'static str {
    if b { "yes" } else { "no" }
}

/// The frame index of each predicate (sentence, token).
fn frame_at(r: &Read) -> BTreeMap<(usize, usize), usize> {
    r.pred.iter().enumerate().filter(|(_, p)| p.1 != usize::MAX).map(|(i, p)| (*p, i)).collect()
}

/// The frame above frame `fi`: the frame of its nearest predicate ancestor in the tree.
fn head_frame(r: &Read, at: &BTreeMap<(usize, usize), usize>, fi: usize) -> Option<usize> {
    let (s, p) = *r.pred.get(fi)?;
    let t = r.trees.get(s)?;
    if p == usize::MAX || p >= t.len() {
        return None;
    }
    let mut x = t.head(p)?;
    for _ in 0..t.len() {
        if let Some(&f) = at.get(&(s, x)) {
            return Some(f);
        }
        x = t.head(x)?;
    }
    None
}

fn tree_of(r: &Read, fi: usize) -> Option<(&Tree, usize)> {
    let (s, p) = *r.pred.get(fi)?;
    let t = r.trees.get(s)?;
    (p < t.len()).then_some((t, p))
}

/// All sites of a read, in order: frames, pronouns in roles, participles on nouns.
pub fn sites(r: &Read) -> Vec<Site> {
    let mut v: Vec<Site> = (0..r.frames.len()).map(Site::Frame).collect();
    for (fi, f) in r.frames.iter().enumerate() {
        let slots = [&f.agent, &f.patient, &f.recipient, &f.place];
        for (k, e) in slots.iter().enumerate() {
            if e.as_ref().is_some_and(|e| e.pron.as_deref().is_some_and(|p| COREF_PRONS.contains(&p.to_lowercase().as_str()))) {
                v.push(Site::Ent(fi, k));
            }
        }
        for (k, (_, e)) in f.obls.iter().enumerate() {
            if e.pron.as_deref().is_some_and(|p| COREF_PRONS.contains(&p.to_lowercase().as_str())) {
                v.push(Site::Ent(fi, 4 + k));
            }
        }
    }
    let at = frame_at(r);
    for (s, t) in r.trees.iter().enumerate() {
        for i in 0..t.len() {
            if at.contains_key(&(s, i)) || !matches!(t.tag(i), PTag::VBG | PTag::VBN) || !matches!(t.base(i), Rel::Amod | Rel::Acl) {
                continue;
            }
            if t.head(i).is_some_and(|h| matches!(t.upos(h), UPos::NOUN | UPos::PROPN)) {
                v.push(Site::Token(s, i));
            }
        }
    }
    v
}

fn slot_ent(f: &Frame, k: usize) -> Option<&Ent> {
    match k {
        0 => f.agent.as_ref(),
        1 => f.patient.as_ref(),
        2 => f.recipient.as_ref(),
        3 => f.place.as_ref(),
        _ => f.obls.get(k - 4).map(|x| &x.1),
    }
}

fn slot_name(k: usize) -> &'static str {
    match k {
        0 => "agent",
        1 => "patient",
        2 => "recipient",
        3 => "place",
        _ => "oblique",
    }
}

/// The atoms of a frame site.
fn frame_atoms(r: &Read, at: &BTreeMap<(usize, usize), usize>, fi: usize) -> Vec<Atom> {
    let f = &r.frames[fi];
    let mut v = vec![Atom::new(Feat::Kind, kind_name(f.kind))];
    let h = head_frame(r, at, fi);
    v.push(Atom::new(Feat::HeadKind, h.map(|h| kind_name(r.frames[h].kind)).unwrap_or("none")));
    v.push(Atom::new(Feat::HasAgent, yn(f.agent.is_some())));
    v.push(Atom::new(Feat::HasPatient, yn(f.patient.is_some())));
    v.push(Atom::new(Feat::HasPlace, yn(f.place.is_some())));
    v.push(Atom::new(Feat::HasTo, yn(f.recipient.is_some())));
    v.push(Atom::new(Feat::Agent, ent_form(&f.agent)));
    if let Some(a) = &f.agent {
        v.push(Atom::new(Feat::AgentAnim, anim(&a.id)));
    }
    v.push(Atom::new(Feat::Patient, ent_form(&f.patient)));
    if let Some(a) = &f.patient {
        v.push(Atom::new(Feat::PatientAnim, anim(&a.id)));
    }
    v.push(Atom::new(Feat::Quoted, yn(f.quoted)));
    v.push(Atom::new(Feat::Cond, yn(f.cond)));
    v.push(Atom::new(Feat::Neg, yn(f.neg)));
    if let Some((p, _)) = f.obls.first() {
        if !p.is_empty() {
            v.push(Atom::new(Feat::Obl, p.clone()));
        }
    }
    v.push(Atom::new(Feat::HasCause, yn(f.cause.is_some())));
    v.push(Atom::new(Feat::HasNext, yn(f.next.is_some())));
    v.push(Atom::new(Feat::PrevKind, if fi == 0 { "none" } else { kind_name(r.frames[fi - 1].kind) }));
    v.push(Atom::new(Feat::Carried, yn(r.frames.iter().any(|g| g.quote.contains(&fi)))));
    if let Some(h) = h {
        v.push(Atom::new(Feat::HeadHasAgent, yn(r.frames[h].agent.is_some())));
    }
    // what the lexicon said: the class (its top class), the frame pattern, the source, the agent's role
    v.push(Atom::new(Feat::RolesBy, f.how.by.name()));
    if f.how.by == crate::frames::By::VerbNet {
        let top: Vec<&str> = f.how.class.splitn(3, '-').take(2).collect();
        v.push(Atom::new(Feat::VnClass, top.join("-")));
        v.push(Atom::new(Feat::VnFrame, f.how.frame.replace(' ', "_")));
        if let Some((_, r)) = f.how.roles.iter().find(|(s, _)| s == "agent") {
            v.push(Atom::new(Feat::VnAgent, r.clone()));
        }
    } else {
        v.push(Atom::new(Feat::VnClass, "none"));
    }
    let head = f.verb.split_whitespace().next().unwrap_or("");
    if !head.is_empty() {
        v.push(Atom::new(Feat::Lemma, head));
        v.push(Atom::new(Feat::Cls, cls(head)));
        if let Some((c, _)) = global::verb_class(head) {
            v.push(Atom::new(Feat::GCls, c));
        }
    }
    if let Some((t, p)) = tree_of(r, fi) {
        v.push(Atom::new(Feat::Rel, format!("{:?}", t.base(p)).to_lowercase()));
        v.push(Atom::new(Feat::Tag, format!("{:?}", t.tag(p))));
        v.push(Atom::new(Feat::HasNsubj, yn(t.kids(p).iter().any(|&k| t.base(k) == Rel::Nsubj))));
        v.push(Atom::new(Feat::HasObj, yn(t.kids(p).iter().any(|&k| matches!(t.base(k), Rel::Obj | Rel::Iobj)))));
        let mut marks: BTreeSet<String> = BTreeSet::new();
        let mut advs: BTreeSet<String> = BTreeSet::new();
        for &k in t.kids(p) {
            match t.base(k) {
                Rel::Mark => {
                    marks.insert(t.lemma(k).to_lowercase());
                }
                Rel::Advmod | Rel::Cc if LINK_WORDS.contains(&t.lemma(k).to_lowercase().as_str()) => {
                    advs.insert(t.lemma(k).to_lowercase());
                }
                _ => {}
            }
        }
        v.extend(marks.into_iter().map(|m| Atom::new(Feat::Mark, m)));
        v.extend(advs.into_iter().map(|m| Atom::new(Feat::Adv, m)));
        let above = t.head(p);
        v.push(Atom::new(Feat::Before, match above {
            None => "top",
            Some(x) if p < x => "yes",
            _ => "no",
        }));
        v.push(Atom::new(Feat::OnNoun, yn(above.is_some_and(|x| matches!(t.upos(x), UPos::NOUN | UPos::PROPN | UPos::PRON)))));
        let s = r.pred[fi].0;
        v.push(Atom::new(Feat::First, yn(!(0..fi).any(|j| r.pred[j].0 == s))));
        let hl = h.and_then(|h| tree_of(r, h)).map(|(t, q)| t.lemma(q).to_lowercase());
        v.push(Atom::new(Feat::HeadCls, hl.as_deref().map(cls).unwrap_or("none")));
    }
    v
}

/// Linking words kept as atoms (closed class).
const LINK_WORDS: [&str; 16] = ["then", "so", "therefore", "thus", "and", "but", "or", "also", "when", "after", "before", "because", "while", "still", "again", "first"];

/// The candidate a pronoun site gets.
fn candidate(r: &Read, at: &BTreeMap<(usize, usize), usize>, fi: usize, k: usize, c: Cand) -> Option<Ent> {
    let cur = slot_ent(&r.frames[fi], k)?;
    let pron = cur.pron.clone().unwrap_or_default().to_lowercase();
    let plural = matches!(pron.as_str(), "they" | "them" | "their" | "themselves");
    let person = matches!(pron.as_str(), "he" | "him" | "his" | "himself" | "she" | "her" | "herself");
    // earlier frames: those before fi, and fi's own earlier roles do not count
    match c {
        Cand::PrevAgent => (0..fi).rev().find_map(|j| real(&r.frames[j].agent).cloned()),
        Cand::PrevPatient => (0..fi).rev().find_map(|j| real(&r.frames[j].patient).cloned()),
        Cand::LineAgent => (fi > 0).then(|| real(&r.frames[0].agent).cloned()).flatten(),
        Cand::Speaker => (0..fi).rev().find_map(|j| matches!(r.frames[j].kind, Kind::Say(_)).then(|| real(&r.frames[j].agent).cloned()).flatten()).or_else(|| r.before_speaker.clone()),
        Cand::HeadAgent => head_frame(r, at, fi).and_then(|h| real(&r.frames[h].agent).cloned()),
        Cand::Before => r.before.iter().find(|e| (e.plural || !e.and.is_empty()) == plural).cloned(),
        Cand::BeforePerson => r.before.iter().find(|e| !e.plural && (e.named || anim(&e.id) == "animate")).cloned().filter(|_| person),
        Cand::Unresolve => Some(Ent { id: norm_pron(&pron).into(), text: pron.clone(), pron: Some(pron.clone()), plural, ..Default::default() }),
    }
}

fn norm_pron(p: &str) -> &'static str {
    match p {
        "he" | "him" | "his" | "himself" => "he",
        "she" | "her" | "herself" => "she",
        "they" | "them" | "their" | "themselves" => "they",
        _ => "it",
    }
}

/// The atoms of a site for an action (a pronoun site adds the features of the action's candidate).
pub fn atoms(r: &Read, site: Site, action: Action) -> Vec<Atom> {
    let at = frame_at(r);
    match site {
        Site::Frame(fi) => frame_atoms(r, &at, fi),
        Site::Ent(fi, k) => {
            let f = &r.frames[fi];
            let Some(cur) = slot_ent(f, k) else { return Vec::new() };
            let mut v = vec![Atom::new(Feat::Pron, cur.pron.clone().unwrap_or_default().to_lowercase()), Atom::new(Feat::Slot, slot_name(k)), Atom::new(Feat::Kind, kind_name(f.kind)), Atom::new(Feat::Quoted, yn(f.quoted))];
            let cf = if UNRESOLVED.contains(&cur.id.as_str()) { "unresolved" } else if cur.named { "name" } else if FEM.contains(&cur.id.as_str()) || MASC.contains(&cur.id.as_str()) { "person" } else { "noun" };
            v.push(Atom::new(Feat::Cur, cf));
            if cf != "unresolved" {
                v.push(Atom::new(Feat::CurAnim, anim(&cur.id)));
                v.push(Atom::new(Feat::CurGender, gender(&Ent { pron: None, ..cur.clone() })));
            }
            let head = f.verb.split_whitespace().next().unwrap_or("");
            if !head.is_empty() {
                v.push(Atom::new(Feat::Cls, cls(head)));
                if let Some((c, _)) = global::verb_class(head) {
                    v.push(Atom::new(Feat::GCls, c));
                }
            }
            if let Some((t, p)) = tree_of(r, fi) {
                v.push(Atom::new(Feat::Rel, format!("{:?}", t.base(p)).to_lowercase()));
            }
            if let Action::Resolve(c) = action {
                match candidate(r, &at, fi, k, c) {
                    None => v.push(Atom::new(Feat::Cand, "none")),
                    Some(e) => {
                        let same = e.id == cur.id;
                        v.push(Atom::new(Feat::Cand, if same { "same" } else if e.named { "name" } else if UNRESOLVED.contains(&e.id.as_str()) { "unresolved" } else { "noun" }));
                        if c != Cand::Unresolve {
                            v.push(Atom::new(Feat::CandAnim, anim(&e.id)));
                            v.push(Atom::new(Feat::CandGender, gender(&Ent { pron: None, ..e.clone() })));
                            v.push(Atom::new(Feat::CandIsAgent, yn(k != 0 && f.agent.as_ref().is_some_and(|a| a.id == e.id))));
                        }
                    }
                }
            }
            v
        }
        Site::Token(s, i) => {
            let t = &r.trees[s];
            let mut v = vec![Atom::new(Feat::Tag, format!("{:?}", t.tag(i))), Atom::new(Feat::Rel, format!("{:?}", t.base(i)).to_lowercase()), Atom::new(Feat::Cls, cls(t.lemma(i)))];
            if let Some(h) = t.head(i) {
                v.push(Atom::new(Feat::NounAnim, anim(&t.lemma(h).to_lowercase())));
            }
            if let Some((c, _)) = global::verb_class(t.lemma(i)) {
                v.push(Atom::new(Feat::GCls, c));
            }
            v.push(Atom::new(Feat::Before, yn(t.head(i).is_some_and(|h| i < h))));
            v
        }
    }
}

// ---- actions -------------------------------------------------------------------------------------------------------

/// A noun of the tree as an entity (for the frames the rules make).
fn simple_ent(t: &Tree, k: usize) -> Option<Ent> {
    if !matches!(t.upos(k), UPos::NOUN | UPos::PROPN) {
        return None;
    }
    let det = t.kids_of(k, Rel::Det).first().map(|&d| t.lemma(d).to_string());
    Some(Ent { id: t.lemma(k).to_lowercase(), text: t.form(k).into(), named: t.upos(k) == UPos::PROPN && det.is_none(), plural: matches!(t.tag(k), PTag::NNS | PTag::NNPS), ..Default::default() })
}

/// Do the action at the site; false when it changes nothing (or cannot be done). `Drop` is not done here (see
/// `apply_rule`, which drops at the end so that frame indices stay put).
fn act(r: &mut Read, at: &BTreeMap<(usize, usize), usize>, site: Site, a: Action) -> bool {
    match site {
        Site::Frame(fi) => act_frame(r, at, fi, a),
        Site::Ent(fi, k) => {
            let Action::Resolve(c) = a else { return false };
            let Some(mut e) = candidate(r, at, fi, k, c) else { return false };
            let cur = slot_ent(&r.frames[fi], k).cloned().unwrap();
            if e.id == cur.id {
                return false;
            }
            e.pron = cur.pron.clone();
            let f = &mut r.frames[fi];
            match k {
                0 => f.agent = Some(e),
                1 => f.patient = Some(e),
                2 => f.recipient = Some(e),
                3 => f.place = Some(e),
                _ => f.obls[k - 4].1 = e,
            }
            true
        }
        Site::Token(s, i) => {
            let t = &r.trees[s];
            let Some(n) = t.head(i).and_then(|h| simple_ent(t, h)) else { return false };
            let mut f = Frame::new(Kind::Event, t.lemma(i).to_lowercase());
            match a {
                Action::AddNounAgent => f.agent = Some(n),
                Action::AddNounPatient => f.patient = Some(n),
                _ => return false,
            }
            r.frames.push(f);
            r.pred.push((s, i));
            true
        }
    }
}

fn act_frame(r: &mut Read, at: &BTreeMap<(usize, usize), usize>, fi: usize, a: Action) -> bool {
    let h = head_frame(r, at, fi);
    let prev = fi.checked_sub(1).filter(|&j| r.pred.get(j).map(|p| p.0) == r.pred.get(fi).map(|p| p.0));
    let set_ent = |slot: &mut Option<Ent>, v: Option<Ent>| -> bool {
        if *slot == v {
            return false;
        }
        *slot = v;
        true
    };
    match a {
        Action::AgentFromHead | Action::AgentFromHeadPatient => {
            let Some(h) = h else { return false };
            let v = if a == Action::AgentFromHead { r.frames[h].agent.clone() } else { r.frames[h].patient.clone() };
            if v.is_none() {
                return false;
            }
            set_ent(&mut r.frames[fi].agent, v)
        }
        Action::AgentFromNoun => {
            let Some((t, p)) = tree_of(r, fi) else { return false };
            let v = t.head(p).and_then(|x| simple_ent(t, x));
            if v.is_none() {
                return false;
            }
            set_ent(&mut r.frames[fi].agent, v)
        }
        Action::AgentFromPatient => {
            let f = &mut r.frames[fi];
            if f.patient.is_none() {
                return false;
            }
            f.agent = f.patient.take();
            true
        }
        Action::AgentNone => r.frames[fi].agent.take().is_some(),
        Action::PatientNone => r.frames[fi].patient.take().is_some(),
        Action::PlaceNone => r.frames[fi].place.take().is_some(),
        Action::ToNone => r.frames[fi].recipient.take().is_some(),
        Action::PatientToTo => {
            let f = &mut r.frames[fi];
            if f.patient.is_none() || f.recipient.is_some() {
                return false;
            }
            f.recipient = f.patient.take();
            true
        }
        Action::ToToPatient => {
            let f = &mut r.frames[fi];
            if f.recipient.is_none() || f.patient.is_some() {
                return false;
            }
            f.patient = f.recipient.take();
            true
        }
        Action::OblToPlace | Action::OblToPatient => {
            let f = &mut r.frames[fi];
            let slot = if a == Action::OblToPlace { &mut f.place } else { &mut f.patient };
            if slot.is_some() || f.obls.is_empty() {
                return false;
            }
            *slot = Some(f.obls.remove(0).1);
            true
        }
        Action::AgentFromBy => {
            let f = &mut r.frames[fi];
            let Some(i) = f.obls.iter().position(|(p, _)| p == "by") else { return false };
            if f.agent.is_some() {
                return false;
            }
            f.agent = Some(f.obls.remove(i).1);
            true
        }
        Action::ToFromObl => {
            let f = &mut r.frames[fi];
            if f.recipient.is_some() || f.obls.is_empty() {
                return false;
            }
            f.recipient = Some(f.obls.remove(0).1);
            true
        }
        Action::PlaceToPatient => {
            let f = &mut r.frames[fi];
            if f.place.is_none() || f.patient.is_some() {
                return false;
            }
            f.patient = f.place.take();
            true
        }
        Action::Swap => {
            let f = &mut r.frames[fi];
            if f.agent.is_none() && f.patient.is_none() || f.agent == f.patient {
                return false;
            }
            std::mem::swap(&mut f.agent, &mut f.patient);
            true
        }
        Action::Drop => false,
        Action::KindEvent => {
            let f = &mut r.frames[fi];
            if !matches!(f.kind, Kind::Say(_)) {
                return false;
            }
            f.kind = Kind::Event;
            true
        }
        Action::KindSay(act) => {
            let f = &mut r.frames[fi];
            if f.kind == Kind::Say(act) || f.kind == Kind::Bare {
                return false;
            }
            f.kind = Kind::Say(act);
            true
        }
        Action::CauseHeadThis => {
            let Some(h) = h else { return false };
            if r.frames[h].cause == Some(fi) {
                return false;
            }
            r.frames[h].cause = Some(fi);
            true
        }
        Action::CauseThisHead => {
            let Some(h) = h else { return false };
            if r.frames[fi].cause == Some(h) {
                return false;
            }
            r.frames[fi].cause = Some(h);
            true
        }
        Action::NextHeadThis => {
            let Some(h) = h else { return false };
            if r.frames[h].next == Some(fi) {
                return false;
            }
            r.frames[h].next = Some(fi);
            true
        }
        Action::NextThisHead => {
            let Some(h) = h else { return false };
            if r.frames[fi].next == Some(h) {
                return false;
            }
            r.frames[fi].next = Some(h);
            true
        }
        Action::NextPrevThis => {
            let Some(p) = prev else { return false };
            if r.frames[p].next == Some(fi) {
                return false;
            }
            r.frames[p].next = Some(fi);
            true
        }
        Action::CauseThisPrev => {
            let Some(p) = prev else { return false };
            if r.frames[fi].cause == Some(p) {
                return false;
            }
            r.frames[fi].cause = Some(p);
            true
        }
        Action::ClearCause => r.frames[fi].cause.take().is_some(),
        Action::ClearNext => r.frames[fi].next.take().is_some(),
        Action::SaysHeadThis => {
            let Some(h) = h else { return false };
            if r.frames[h].quote.contains(&fi) {
                return false;
            }
            r.frames[h].quote.push(fi);
            true
        }
        Action::SaysRemove => {
            let mut any = false;
            for f in r.frames.iter_mut() {
                let n = f.quote.len();
                f.quote.retain(|&q| q != fi);
                any |= f.quote.len() != n;
            }
            any
        }
        Action::Resolve(_) | Action::AddNounAgent | Action::AddNounPatient => false,
    }
}

/// Remove frames, keeping every index (links, quotes, roots, asked) right.
pub fn drop_frames(r: &mut Read, gone: &BTreeSet<usize>) {
    if gone.is_empty() {
        return;
    }
    let n = r.frames.len();
    let mut map: Vec<Option<usize>> = vec![None; n];
    let mut k = 0;
    for (i, m) in map.iter_mut().enumerate() {
        if !gone.contains(&i) {
            *m = Some(k);
            k += 1;
        }
    }
    let frames = std::mem::take(&mut r.frames);
    let pred = std::mem::take(&mut r.pred);
    for (i, (mut f, p)) in frames.into_iter().zip(pred).enumerate() {
        if map[i].is_none() {
            continue;
        }
        f.quote = f.quote.iter().filter_map(|&q| map.get(q).copied().flatten()).collect();
        f.cause = f.cause.and_then(|c| map.get(c).copied().flatten());
        f.next = f.next.and_then(|c| map.get(c).copied().flatten());
        r.frames.push(f);
        r.pred.push(p);
    }
    r.roots = r.roots.iter().filter_map(|&x| map.get(x).copied().flatten()).collect();
    r.asked = std::mem::take(&mut r.asked).into_iter().filter_map(|mut a| {
        a.frame = map.get(a.frame).copied().flatten()?;
        Some(a)
    }).collect();
}

/// Does the site match these atoms (for this action)?
fn matches(r: &Read, site: Site, rule: &Rule) -> bool {
    let kind_ok = match site {
        Site::Frame(_) => site_kind(rule.action) == 0,
        Site::Ent(..) => site_kind(rule.action) == 1,
        Site::Token(..) => site_kind(rule.action) == 2,
    };
    if !kind_ok {
        return false;
    }
    let have = atoms(r, site, rule.action);
    rule.atoms.iter().all(|a| have.contains(a))
}

/// Apply one rule to a read: every matching site (matched on the read as it was before the rule), in order; drops
/// at the end. The number of sites changed.
pub fn apply_rule(r: &mut Read, rule: &Rule) -> usize {
    if r.pred.len() != r.frames.len() {
        return 0;
    }
    let hits: Vec<Site> = sites(r).into_iter().filter(|s| matches(r, *s, rule)).collect();
    let mut n = 0;
    let mut gone = BTreeSet::new();
    let at = frame_at(r);
    for s in hits {
        if rule.action == Action::Drop {
            if let Site::Frame(fi) = s {
                gone.insert(fi);
                n += 1;
            }
            continue;
        }
        if act(r, &at, s, rule.action) {
            n += 1;
        }
    }
    drop_frames(r, &gone);
    n
}

/// The action at one site only (the learner scores sites one by one): the changed read, or None if nothing changed.
pub fn apply_at(r: &Read, site: Site, a: Action) -> Option<Read> {
    let mut x = r.clone();
    if a == Action::Drop {
        let Site::Frame(fi) = site else { return None };
        drop_frames(&mut x, &BTreeSet::from([fi]));
        return Some(x);
    }
    let at = frame_at(&x);
    act(&mut x, &at, site, a).then_some(x)
}

/// The actions that can act at a site.
pub fn actions_for(site: Site) -> Vec<Action> {
    match site {
        Site::Frame(_) => frame_actions(),
        Site::Ent(..) => ent_actions(),
        Site::Token(..) => TOKEN_ACTIONS.to_vec(),
    }
}

/// `is_pred` for callers outside (the learner's sanity checks).
pub fn predicate(t: &Tree, i: usize) -> bool {
    is_pred(t, i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_round_trip() {
        for a in frame_actions().into_iter().chain(ent_actions()).chain(TOKEN_ACTIONS) {
            assert_eq!(Action::parse(&a.code()), Some(a), "{}", a.code());
        }
        for f in FEATS {
            assert_eq!(Feat::by_name(f.name()), Some(f));
        }
        let r = parse_rules(&format!("{HEADER}\nr1\taccepted\tagent-from-head\trel:conj&has-agent:no\tx\t3\t0\t-\n"));
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].action, Action::AgentFromHead);
        assert!(r[0].say().contains("conj"));
    }
}
