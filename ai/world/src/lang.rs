//! World change language: one command per line, `^sN` anchors at the end.
//! Specification commands — `cursor loc obj char move have say event`; added `set` (attribute with
//! a cause), `rel` (relationship — a separate class), `give` (handing over an object). Parsing is strict: an unknown
//! command, key, value outside the closed set, duplicate key, unclosed quotes — a format error.
//!
//! v2 (26.09.2026): intentions `plan` (promise, plan, deal… with state open/kept/broken/dropped and a fulfilment
//! event `by=`), beliefs `believe` (claim, about what, from where, truth), `done`/`unset`
//! (closing a goal or attribute with a cause), `feel` (several feelings: `+a -b`), `detach`/`attach`
//! (part — whole), `mention` (a mention without moving the cursor); joint agents `agent=C1+C2`, several acts
//! `act=request+promise`, `neg=1` (an event that did not happen), event `cause=`, `about=` (about the past),
//! `when=` (story time), `before=` (flashback), `num=`, `part=`, `loc … here=0`.
//!
//! v3 (26.09.2026, non-linear narrative): narrative frames `frame open|close` (who narrates, level, own
//! world and time), observer claims `claim` about a canonical event (believes, doubts, is mistaken,
//! knows), story-time relations `time E1 BEFORE|AFTER|DURING|SAME E2` and `time E at="…"` (absolute
//! time only from the text), reality branch `branch E dream|plan|lie|…`, knowledge `know who= fact= from=^sN`,
//! revelation `reveal E …` (a change of knowledge, not a new event); event placement at creation `after=`,
//! `during=`, `same=` (in addition to v2's `before=`). An existing event is mentioned by reference: `claim`, `know`,
//! `reveal`, `time` — no new event is written for the same thing.

use prag::schema::Act;

use crate::types::*;

/// Event participant role.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Agent,
    Patient,
    To,
    At,
    Instr,
}

impl Role {
    pub fn name(self) -> &'static str {
        match self {
            Role::Agent => "agent",
            Role::Patient => "patient",
            Role::To => "to",
            Role::At => "at",
            Role::Instr => "instr",
        }
    }

    /// Allowed registries for the participant.
    pub fn regs(self) -> &'static [Reg] {
        match self {
            Role::Agent => &[Reg::C],
            Role::Patient => &[Reg::C, Reg::O, Reg::L],
            Role::To => &[Reg::C, Reg::L],
            Role::At => &[Reg::L],
            Role::Instr => &[Reg::O],
        }
    }
}

/// Who speaks: a character (several — in chorus, v2) or the narrator (comment, moral).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Speaker {
    Chars(Vec<Id>),
    Narrator,
}

/// Observer (v3): a character, the reader or the narrator.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Obs {
    C(Id),
    Reader,
    Narrator,
}

impl std::fmt::Display for Obs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Obs::C(i) => write!(f, "{i}"),
            Obs::Reader => f.write_str("reader"),
            Obs::Narrator => f.write_str("narrator"),
        }
    }
}

/// Story-time relation (v3): a partial order on event endpoints (Allen intervals, as in TimeML).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TRel {
    /// end of A < start of B
    Before,
    /// end of B < start of A
    After,
    /// A inside B
    During,
    /// ≈ simultaneous: the intervals overlap
    Same,
}

impl TRel {
    pub fn parse(s: &str) -> Option<TRel> {
        Some(match s {
            "BEFORE" => TRel::Before,
            "AFTER" => TRel::After,
            "DURING" => TRel::During,
            "SAME" => TRel::Same,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            TRel::Before => "BEFORE",
            TRel::After => "AFTER",
            TRel::During => "DURING",
            TRel::Same => "SAME",
        }
    }
}

/// v2 event details: negation, cause event, about what, story time, flashback.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Ev2 {
    /// `neg=1`: the text says this did **not** happen ("did not stop")
    pub neg: bool,
    /// `cause=E`: the event is a consequence of another
    pub cause: Option<Id>,
    /// `about=X`: about a past event, intention, belief or entity
    pub about: Option<Id>,
    /// `when=`: story time as a word (morning, next_day, long_ago), separate from the position in the text
    pub when: Option<String>,
    /// `before=E`: happened before E, though told later (a memory)
    pub before: Option<Id>,
    /// v3: `after=E` — happened after E, though told earlier or off the line (a flash-forward)
    pub after: Option<Id>,
    /// v3: `during=E` — happened during E
    pub during: Option<Id>,
    /// v3: `same=E` — ≈ simultaneous with E
    pub same: Option<Id>,
}

impl Ev2 {
    /// Explicit placement on the story axis (v3): the event does not go onto the frame's narrative line.
    pub fn placement(&self) -> Option<(TRel, Id)> {
        if let Some(e) = self.before {
            return Some((TRel::Before, e));
        }
        if let Some(e) = self.after {
            return Some((TRel::After, e));
        }
        if let Some(e) = self.during {
            return Some((TRel::During, e));
        }
        self.same.map(|e| (TRel::Same, e))
    }
}

/// A change-language command.
#[derive(Clone, PartialEq, Debug)]
pub enum Cmd {
    /// where the narrative moved: position in the text (for checking), location, story time
    Cursor { chapter: Option<u16>, para: Option<u16>, sent: Option<u16>, at: Option<Id>, time: Option<String> },
    /// `here=0` (v2): the place is only mentioned, the narrative cursor does not move
    Loc { id: Id, ty: Tag, class: Option<String>, name: Option<String>, parent: Option<Id>, here: bool },
    /// `part=` (v2): a part of a whole (body, thing), not an object in hand
    Obj { id: Id, ty: Tag, class: Option<String>, name: Option<String>, at: Option<Id>, part: Option<Id>, num: Option<V> },
    Char { id: Id, ty: Tag, class: Option<String>, name: Option<String>, at: Option<Id>, attrs: Vec<(Field, V)> },
    Move { who: Id, to: Id, cause: Option<Id> },
    Have { who: Id, what: Id, cause: Option<Id> },
    Give { who: Id, what: Id, to: Id, cause: Option<Id> },
    Set { who: Id, attrs: Vec<(Field, V)>, cause: Option<Id> },
    Rel { id: Id, kind: Option<Tag>, a: Option<Id>, b: Option<Id>, cause: Option<Id>, fact: Option<Id> },
    Event { id: Id, verb: String, roles: Vec<(Role, Id)>, why: Option<String>, how: Option<String>, x: Ev2 },
    Say {
        id: Option<Id>,
        who: Speaker,
        to: Vec<Id>,
        /// the first is the main act; the rest are further acts of the same utterance (v2)
        acts: Vec<Act>,
        indirect: bool,
        sincere: Option<bool>,
        means: Option<String>,
        why: Option<String>,
        x: Ev2,
    },
    // ── v2 ──
    /// intention: creation (`kind=`) or closing (`status=` + `by=E`)
    Plan { id: Id, kind: Option<Tag>, who: Vec<Id>, to: Option<Id>, what: Option<String>, src: Option<Id>, status: Option<Tag>, by: Option<Id> },
    /// belief: creation (`who=`, `claim=`) or change (`status=`/`true=` + `by=E`)
    Believe { id: Id, who: Option<Id>, claim: Option<String>, about: Option<Id>, from: Option<Id>, truth: Option<bool>, status: Option<Tag>, by: Option<Id> },
    /// goal achieved
    Done { who: Id, cause: Option<Id> },
    /// the attribute no longer holds (goal abandoned, feeling passed), with a cause
    Unset { who: Id, field: Field, cause: Option<Id>, why: Option<String> },
    /// feelings: add and remove, the rest stays
    Feel { who: Id, add: Vec<Tag>, del: Vec<Tag>, cause: Option<Id> },
    /// a part is detached from the whole: lies in L or in C's hands
    Detach { what: Id, to: Id, cause: Option<Id> },
    /// the object became part of C or O
    Attach { what: Id, to: Id, cause: Option<Id> },
    /// a mention of existing entities without changes and without moving the cursor
    Mention { ids: Vec<Id> },
    // ── v3 ──
    /// narrative frame: open (or resume an interrupted one) or close
    Frame {
        id: Id,
        open: bool,
        narrator: Option<Obs>,
        to: Vec<Obs>,
        level: Option<u8>,
        /// `same` (testimony, memory — the same world) or `new` (a made-up story within the story)
        world: Option<Tag>,
        /// `past` (everything in the frame happened earlier) or `now` (a parallel line, the same axis)
        time: Option<Tag>,
        /// a past frame — before this event of the parent
        before: Option<Id>,
        /// the narrating utterance that opens the frame
        src: Option<Id>,
        /// branch frame: dream, lie, plan, "what if"
        branch: Option<Tag>,
    },
    /// an observer's claim about a canonical event: creation (`who`, `about`, `status`) or status change
    Claim { id: Id, who: Option<Obs>, about: Option<Id>, status: Tag, interp: Option<String>, from: Option<Id>, by: Option<Id> },
    /// story-time relation between events or absolute time from the text
    Time { a: Id, rel: Option<(TRel, Id)>, at: Option<String> },
    /// reality branch of an event
    Branch { ev: Id, branch: Tag, by: Option<Id> },
    /// the observer knows a fact from sentence N
    Know { who: Obs, fact: Id, from: u16, how: Option<Tag>, by: Option<Id> },
    /// revelation: a change of knowledge about an existing event (role, cause, time, false and true claims)
    Reveal {
        ev: Id,
        to: Vec<Obs>,
        by: Option<Id>,
        interp: Option<String>,
        wrong: Vec<Id>,
        right: Vec<Id>,
        roles: Vec<(Role, Id)>,
        cause: Option<Id>,
        rel: Option<(TRel, Id)>,
    },
}

impl Cmd {
    pub fn name(&self) -> &'static str {
        match self {
            Cmd::Cursor { .. } => "cursor",
            Cmd::Loc { .. } => "loc",
            Cmd::Obj { .. } => "obj",
            Cmd::Char { .. } => "char",
            Cmd::Move { .. } => "move",
            Cmd::Have { .. } => "have",
            Cmd::Give { .. } => "give",
            Cmd::Set { .. } => "set",
            Cmd::Rel { .. } => "rel",
            Cmd::Event { .. } => "event",
            Cmd::Say { .. } => "say",
            Cmd::Plan { .. } => "plan",
            Cmd::Believe { .. } => "believe",
            Cmd::Done { .. } => "done",
            Cmd::Unset { .. } => "unset",
            Cmd::Feel { .. } => "feel",
            Cmd::Detach { .. } => "detach",
            Cmd::Attach { .. } => "attach",
            Cmd::Mention { .. } => "mention",
            Cmd::Frame { .. } => "frame",
            Cmd::Claim { .. } => "claim",
            Cmd::Time { .. } => "time",
            Cmd::Branch { .. } => "branch",
            Cmd::Know { .. } => "know",
            Cmd::Reveal { .. } => "reveal",
        }
    }

    /// A v3 command (handled by the narrative layer, not by the frame's world executor).
    pub fn is_v3(&self) -> bool {
        matches!(self, Cmd::Frame { .. } | Cmd::Claim { .. } | Cmd::Time { .. } | Cmd::Branch { .. } | Cmd::Know { .. } | Cmd::Reveal { .. })
    }
}

/// Parsed line: number in the LLM output, raw text, command, anchors.
#[derive(Clone, Debug)]
pub struct Line {
    pub no: usize,
    pub raw: String,
    pub cmd: Cmd,
    pub anchors: Vec<u16>,
}

/// Format rejection.
#[derive(Clone, Debug)]
pub struct FormatErr {
    pub no: usize,
    pub raw: String,
    pub msg: String,
}

/// Split into tokens; `"…"` is one token together with its key (`name="the Fox"`).
fn tokens(s: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    for c in s.chars() {
        if quoted {
            cur.push(c);
            if c == '"' {
                quoted = false;
            }
            continue;
        }
        if c == '"' {
            quoted = true;
            cur.push(c);
        } else if c.is_whitespace() {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
        } else {
            cur.push(c);
        }
    }
    if quoted {
        return Err("unclosed quotes".into());
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    Ok(out)
}

/// Command arguments: positional and `key=value` (a quoted value is marked).
struct Args {
    pos: Vec<String>,
    kv: Vec<(String, String, bool)>,
}

impl Args {
    fn new(toks: &[String]) -> Result<Args, String> {
        let mut pos = Vec::new();
        let mut kv: Vec<(String, String, bool)> = Vec::new();
        for t in toks {
            match t.split_once('=') {
                Some((k, v)) => {
                    if k.is_empty() || !k.bytes().all(|b| b.is_ascii_lowercase()) {
                        return Err(format!("key {k:?}"));
                    }
                    if kv.iter().any(|(x, _, _)| x == k) {
                        return Err(format!("key {k} twice"));
                    }
                    let quoted = v.starts_with('"');
                    let v = if quoted {
                        if v.len() < 2 || !v.ends_with('"') || v[1..v.len() - 1].contains('"') {
                            return Err(format!("{k}: quotes"));
                        }
                        v[1..v.len() - 1].to_string()
                    } else {
                        if v.contains('"') {
                            return Err(format!("{k}: quotes in the middle of a value"));
                        }
                        v.to_string()
                    };
                    kv.push((k.to_string(), v, quoted));
                }
                None => {
                    if !kv.is_empty() {
                        return Err(format!("positional argument {t:?} after keys"));
                    }
                    if t.starts_with('"') {
                        return Err(format!("text {t} without a key"));
                    }
                    pos.push(t.clone());
                }
            }
        }
        Ok(Args { pos, kv })
    }

    fn take(&mut self, k: &str) -> Option<(String, bool)> {
        let i = self.kv.iter().position(|(x, _, _)| x == k)?;
        let (_, v, q) = self.kv.remove(i);
        Some((v, q))
    }

    fn id(&mut self, k: &str, regs: &[Reg]) -> Result<Option<Id>, String> {
        self.take(k).map(|(v, q)| if q { Err(format!("{k}: id without quotes")) } else { id(&v, regs) }).transpose()
    }

    /// Several ids joined by `+` (joint agents, v2): `agent=C1+C2`; empty — no key.
    fn ids(&mut self, k: &str, regs: &[Reg]) -> Result<Vec<Id>, String> {
        match self.take(k) {
            None => Ok(Vec::new()),
            Some((_, true)) => Err(format!("{k}: id without quotes")),
            Some((v, false)) => plus_ids(&v, regs),
        }
    }

    fn word(&mut self, k: &str) -> Result<Option<String>, String> {
        self.take(k).map(|(v, q)| if q || !is_word(&v) { Err(format!("{k}={v:?}: a word is lowercase Latin, _ or -")) } else { Ok(v) }).transpose()
    }

    fn text(&mut self, k: &str) -> Result<Option<String>, String> {
        self.take(k).map(|(v, _)| text(k, &v)).transpose()
    }

    fn tag(&mut self, k: &str, set: &'static Closed) -> Result<Option<Tag>, String> {
        self.take(k).map(|(v, _)| set.tag(&v)).transpose()
    }

    fn bit(&mut self, k: &str) -> Result<Option<bool>, String> {
        self.take(k)
            .map(|(v, _)| match v.as_str() {
                "0" => Ok(false),
                "1" => Ok(true),
                _ => Err(format!("{k}={v:?} outside the set 0|1")),
            })
            .transpose()
    }

    fn num(&mut self, k: &str) -> Result<Option<u16>, String> {
        self.take(k).map(|(v, _)| v.parse::<u16>().map_err(|_| format!("{k}={v:?} — not a number"))).transpose()
    }

    /// The remaining keys are attributes (for `char` and `set`).
    fn attrs(&mut self) -> Result<Vec<(Field, V)>, String> {
        let mut out = Vec::new();
        for (k, v, _) in std::mem::take(&mut self.kv) {
            let (f, dom) = key(&k).ok_or_else(|| format!("unknown key {k}"))?;
            out.push((f, value(&k, &v, dom)?));
        }
        Ok(out)
    }

    fn one_pos(&mut self, what: &str, regs: &[Reg]) -> Result<Id, String> {
        if self.pos.is_empty() {
            return Err(format!("missing {what}"));
        }
        let t = self.pos.remove(0);
        id(&t, regs)
    }

    /// Unused arguments are an error (fail-fast).
    fn done(self) -> Result<(), String> {
        if let Some(p) = self.pos.first() {
            return Err(format!("extra argument {p:?}"));
        }
        if let Some((k, _, _)) = self.kv.first() {
            return Err(format!("unknown key {k}"));
        }
        Ok(())
    }
}

fn id(s: &str, regs: &[Reg]) -> Result<Id, String> {
    let i = Id::parse(s).ok_or_else(|| format!("{s:?} — not an id"))?;
    if !regs.contains(&i.reg) {
        let want: String = regs.iter().map(|r| r.letter()).collect();
        return Err(format!("{s}: only ids from {want} here"));
    }
    Ok(i)
}

/// Maximum participants in one role joined by `+`.
pub const MAX_JOINT: usize = 6;

/// `C1+C2` → [C1, C2]: no repeats, up to `MAX_JOINT`.
fn plus_ids(s: &str, regs: &[Reg]) -> Result<Vec<Id>, String> {
    let mut out: Vec<Id> = Vec::new();
    for p in s.split('+') {
        let i = id(p, regs)?;
        if out.contains(&i) {
            return Err(format!("{s}: {i} twice"));
        }
        out.push(i);
    }
    if out.len() > MAX_JOINT {
        return Err(format!("{s}: more than {MAX_JOINT} participants"));
    }
    Ok(out)
}

/// Registries that `about=` may point to.
const ABOUT: &[Reg] = &[Reg::E, Reg::P, Reg::B, Reg::C, Reg::O, Reg::L];

/// Keys that `unset` may remove: states, not identity (name, gender, parents) and not place.
const UNSET_KEYS: &[Field] = &[Field::Goal, Field::Belief, Field::Feeling, Field::Status, Field::Trait, Field::Sleep, Field::Freedom, Field::Num];

fn text(k: &str, v: &str) -> Result<String, String> {
    let v = v.trim();
    if v.is_empty() {
        return Err(format!("{k}: empty text"));
    }
    let n = v.split_whitespace().count();
    if n > TEXT_MAX_WORDS {
        return Err(format!("{k}: {n} words > {TEXT_MAX_WORDS}"));
    }
    Ok(v.to_string())
}

fn value(k: &str, v: &str, dom: Dom) -> Result<V, String> {
    Ok(match dom {
        Dom::Text => V::Text(text(k, v)?),
        Dom::Words => {
            let w: Vec<String> = v.split(',').map(|x| x.trim().to_string()).collect();
            if let Some(bad) = w.iter().find(|x| !is_word(x)) {
                return Err(format!("{k}: {bad:?} — not a word"));
            }
            V::Words(w)
        }
        Dom::Char => V::Id(id(v, &[Reg::C])?),
        Dom::Place => V::Id(id(v, &[Reg::L, Reg::C])?),
        Dom::Tags(set) => V::Tag(set.tag(v)?),
        Dom::TagSet(set) => {
            let mut tags = Vec::new();
            for x in v.split(',') {
                let t = set.tag(x.trim())?;
                if tags.contains(&t) {
                    return Err(format!("{k}: {t} twice"));
                }
                tags.push(t);
            }
            V::tags(tags).ok_or_else(|| format!("{k}: empty"))?
        }
        Dom::Num => num(k, v)?,
    })
}

/// Quantity: an integer 0–100000 in digits or a word from `NUM_WORD`.
fn num(k: &str, v: &str) -> Result<V, String> {
    if !v.is_empty() && v.len() <= 6 && v.bytes().all(|b| b.is_ascii_digit()) && (v == "0" || !v.starts_with('0')) {
        return Ok(V::Words(vec![v.to_string()]));
    }
    NUM_WORD.tag(v).map(|t| V::Words(vec![t.as_str().to_string()])).map_err(|e| format!("{k}: {e}"))
}

/// Feelings for `feel`: `+a` add, `-b` remove.
fn feel_tokens(pos: &[String]) -> Result<(Vec<Tag>, Vec<Tag>), String> {
    let (mut add, mut del) = (Vec::new(), Vec::new());
    for p in pos {
        let (sign, f) = p.split_at(p.chars().next().map(char::len_utf8).unwrap_or(0));
        let t = FEELING.tag(f)?;
        if add.contains(&t) || del.contains(&t) {
            return Err(format!("feel: {t} twice"));
        }
        match sign {
            "+" => add.push(t),
            "-" => del.push(t),
            _ => return Err(format!("feel: {p:?} — expected +feeling or -feeling")),
        }
    }
    if add.is_empty() && del.is_empty() {
        return Err("feel without feelings".into());
    }
    Ok((add, del))
}

/// Parse a line; empty — `None`.
pub fn parse_line(no: usize, raw: &str) -> Result<Option<Line>, FormatErr> {
    let err = |msg: String| FormatErr { no, raw: raw.to_string(), msg };
    let t = raw.trim();
    if t.is_empty() {
        return Ok(None);
    }
    let mut toks = tokens(t).map_err(err)?;
    // anchors — the tail of the line
    let mut anchors = Vec::new();
    while let Some(last) = toks.last() {
        let Some(n) = last.strip_prefix("^s") else { break };
        let n: u16 = n.parse().map_err(|_| err(format!("anchor {last:?}")))?;
        anchors.push(n);
        toks.pop();
    }
    anchors.reverse();
    if toks.iter().any(|x| x.starts_with('^')) {
        return Err(err("anchor not at the end of the line".into()));
    }
    if toks.is_empty() {
        return Err(err("no command".into()));
    }
    let name = toks.remove(0);
    let mut a = Args::new(&toks).map_err(err)?;
    let cmd = command(&name, &mut a).map_err(err)?;
    a.done().map_err(err)?;
    if anchors.is_empty() {
        match &cmd {
            Cmd::Cursor { sent: Some(s), .. } => anchors.push(*s),
            _ => return Err(err("no ^sN anchor".into())),
        }
    }
    Ok(Some(Line { no, raw: t.to_string(), cmd, anchors }))
}

fn command(name: &str, a: &mut Args) -> Result<Cmd, String> {
    let class = |a: &mut Args| a.word("class");
    Ok(match name {
        "cursor" => Cmd::Cursor {
            chapter: a.num("chapter")?,
            para: a.num("para")?,
            sent: a.num("sent")?,
            at: a.id("at", &[Reg::L])?,
            time: a.word("time")?,
        },
        "loc" => {
            let id = a.one_pos("location id", &[Reg::L])?;
            let ty = a.tag("type", &LOC_TYPE)?.ok_or("missing type")?;
            Cmd::Loc { id, ty, class: class(a)?, name: a.text("name")?, parent: a.id("in", &[Reg::L])?, here: a.bit("here")?.unwrap_or(true) }
        }
        "obj" => {
            let id = a.one_pos("object id", &[Reg::O])?;
            let ty = a.tag("type", &OBJ_TYPE)?.ok_or("missing type")?;
            let (class, name, at, part) = (class(a)?, a.text("name")?, a.id("at", &[Reg::L, Reg::C])?, a.id("part", &[Reg::C, Reg::O])?);
            if at.is_some() && part.is_some() {
                return Err("obj: at= and part= together — a part does not lie separately".into());
            }
            if part == Some(id) {
                return Err(format!("{id}: part of itself"));
            }
            let num = a.take("num").map(|(v, _)| num("num", &v)).transpose()?;
            Cmd::Obj { id, ty, class, name, at, part, num }
        }
        "char" => {
            let id = a.one_pos("character id", &[Reg::C])?;
            let ty = a.tag("type", &CHAR_TYPE)?.ok_or("missing type")?;
            let (class, name, at) = (class(a)?, a.text("name")?, a.id("at", &[Reg::L])?);
            Cmd::Char { id, ty, class, name, at, attrs: a.attrs()? }
        }
        "move" => {
            let who = a.one_pos("who/what", &[Reg::C, Reg::O])?;
            let to = a.id("to", &[Reg::L, Reg::C])?.ok_or("missing to")?;
            Cmd::Move { who, to, cause: a.id("cause", &[Reg::E])? }
        }
        "have" => {
            let who = a.one_pos("character", &[Reg::C])?;
            let what = a.one_pos("object", &[Reg::O])?;
            Cmd::Have { who, what, cause: a.id("cause", &[Reg::E])? }
        }
        "give" => {
            let who = a.one_pos("character", &[Reg::C])?;
            let what = a.one_pos("object", &[Reg::O])?;
            let to = a.id("to", &[Reg::C])?.ok_or("missing to")?;
            Cmd::Give { who, what, to, cause: a.id("cause", &[Reg::E])? }
        }
        "set" => {
            let who = a.one_pos("entity", &[Reg::C, Reg::O, Reg::L])?;
            let cause = a.id("cause", &[Reg::E])?;
            let attrs = a.attrs()?;
            if attrs.is_empty() {
                return Err("set without an attribute".into());
            }
            Cmd::Set { who, attrs, cause }
        }
        "rel" => {
            let id = a.one_pos("relationship id", &[Reg::R])?;
            Cmd::Rel {
                id,
                kind: a.tag("kind", &REL_KIND)?,
                a: a.id("a", &[Reg::C])?,
                b: a.id("b", &[Reg::C])?,
                cause: a.id("cause", &[Reg::E])?,
                fact: a.id("fact", &[Reg::E])?,
            }
        }
        "event" => {
            let id = a.one_pos("event id", &[Reg::E])?;
            let verb = a.word("verb")?.ok_or("missing verb")?;
            let mut roles = Vec::new();
            for r in [Role::Agent, Role::Patient, Role::To] {
                for x in a.ids(r.name(), r.regs())? {
                    roles.push((r, x));
                }
            }
            for r in [Role::At, Role::Instr] {
                if let Some(x) = a.id(r.name(), r.regs())? {
                    roles.push((r, x));
                }
            }
            let (why, how) = (a.text("why")?, a.text("how")?);
            Cmd::Event { id, verb, roles, why, how, x: ev2(a)? }
        }
        "say" => {
            if a.pos.is_empty() {
                return Err("missing speaker".into());
            }
            let who = if a.pos[0] == "narrator" {
                a.pos.remove(0);
                Speaker::Narrator
            } else {
                let p = a.pos.remove(0);
                Speaker::Chars(plus_ids(&p, &[Reg::C])?)
            };
            let act = a.take("act").ok_or("missing act")?.0;
            let mut acts = Vec::new();
            for x in act.split('+') {
                let t = Act::parse(x).ok_or_else(|| format!("act={x:?} outside the closed set"))?;
                if acts.contains(&t) {
                    return Err(format!("act={act}: {t} twice"));
                }
                acts.push(t);
            }
            if acts.len() > 3 {
                return Err(format!("act={act}: more than 3 acts"));
            }
            Cmd::Say {
                id: a.id("id", &[Reg::E])?,
                who,
                to: a.ids("to", &[Reg::C])?,
                acts,
                indirect: a.bit("indirect")?.unwrap_or(false),
                sincere: a.bit("sincere")?,
                means: a.text("means")?,
                why: a.text("why")?,
                x: ev2(a)?,
            }
        }
        "plan" => {
            let id = a.one_pos("intention id", &[Reg::P])?;
            let kind = a.tag("kind", &PLAN_KIND)?;
            let status = a.tag("status", &PLAN_STATUS)?;
            let (who, to, what, src, by) = (a.ids("who", &[Reg::C])?, a.id("to", &[Reg::C])?, a.text("what")?, a.id("src", &[Reg::E])?, a.id("by", &[Reg::E])?);
            match (kind, status) {
                (Some(_), None) => {
                    if who.is_empty() || what.is_none() || by.is_some() {
                        return Err("plan: a new intention — kind, who, what (by= only when closing)".into());
                    }
                }
                (None, Some(s)) => {
                    if !who.is_empty() || to.is_some() || what.is_some() || src.is_some() || s.as_str() == "open" {
                        return Err("plan: closing — only status=kept|broken|dropped and by=E".into());
                    }
                }
                _ => return Err("plan: either kind=… (new) or status=… (closing)".into()),
            }
            Cmd::Plan { id, kind, who, to, what, src, status, by }
        }
        "believe" => {
            let id = a.one_pos("belief id", &[Reg::B])?;
            let who = a.id("who", &[Reg::C])?;
            let claim = a.text("claim")?;
            let about = a.id("about", ABOUT)?;
            let from = a.id("from", &[Reg::E])?;
            let truth = a.bit("true")?;
            let status = a.tag("status", &BELIEF_STATUS)?;
            let by = a.id("by", &[Reg::E])?;
            match (who.is_some(), claim.is_some()) {
                (true, true) => {
                    if status.is_some() || by.is_some() {
                        return Err("believe: a new belief — who, claim; status and by — only on change".into());
                    }
                }
                (false, false) => {
                    if about.is_some() || from.is_some() || (status.is_none() && truth.is_none()) {
                        return Err("believe: change — status=dropped or true=0|1, and by=E".into());
                    }
                }
                _ => return Err("believe: a new belief — both who= and claim=".into()),
            }
            Cmd::Believe { id, who, claim, about, from, truth, status, by }
        }
        "done" => {
            let who = a.one_pos("character", &[Reg::C])?;
            Cmd::Done { who, cause: a.id("cause", &[Reg::E])? }
        }
        "unset" => {
            let who = a.one_pos("character", &[Reg::C])?;
            if a.pos.is_empty() {
                return Err("unset: missing key".into());
            }
            let k = a.pos.remove(0);
            let (field, _) = key(&k).ok_or_else(|| format!("unset: unknown key {k}"))?;
            if !UNSET_KEYS.contains(&field) {
                return Err(format!("unset {k}: only these can be removed: {}", UNSET_KEYS.iter().map(|f| f.name()).collect::<Vec<_>>().join(" ")));
            }
            Cmd::Unset { who, field, cause: a.id("cause", &[Reg::E])?, why: a.text("why")? }
        }
        "feel" => {
            let who = a.one_pos("character", &[Reg::C])?;
            let (add, del) = feel_tokens(&std::mem::take(&mut a.pos))?;
            Cmd::Feel { who, add, del, cause: a.id("cause", &[Reg::E])? }
        }
        "detach" | "attach" => {
            let what = a.one_pos("object", &[Reg::O])?;
            let regs: &[Reg] = if name == "detach" { &[Reg::L, Reg::C] } else { &[Reg::C, Reg::O] };
            let to = a.id("to", regs)?.ok_or("missing to")?;
            if to == what {
                return Err(format!("{name} {what} to={to}"));
            }
            let cause = a.id("cause", &[Reg::E])?;
            if name == "detach" { Cmd::Detach { what, to, cause } } else { Cmd::Attach { what, to, cause } }
        }
        "mention" => {
            let mut ids = Vec::new();
            for p in std::mem::take(&mut a.pos) {
                let i = id(&p, &[Reg::L, Reg::O, Reg::C, Reg::P, Reg::B])?;
                if ids.contains(&i) {
                    return Err(format!("mention: {i} twice"));
                }
                ids.push(i);
            }
            if ids.is_empty() {
                return Err("mention without entities".into());
            }
            Cmd::Mention { ids }
        }
        "frame" | "claim" | "time" | "branch" | "know" | "reveal" => command_v3(name, a)?,
        _ => return Err(format!("unknown command {name:?}")),
    })
}

/// v2 event details (`neg= cause= about= when= before=`) and v3 placement (`after= during= same=`).
fn ev2(a: &mut Args) -> Result<Ev2, String> {
    let x = Ev2 {
        neg: a.bit("neg")?.unwrap_or(false),
        cause: a.id("cause", &[Reg::E])?,
        about: a.id("about", ABOUT)?,
        when: a.word("when")?,
        before: a.id("before", &[Reg::E])?,
        after: a.id("after", &[Reg::E])?,
        during: a.id("during", &[Reg::E])?,
        same: a.id("same", &[Reg::E])?,
    };
    if [x.before, x.after, x.during, x.same].iter().filter(|p| p.is_some()).count() > 1 {
        return Err("before=/after=/during=/same= — only one placement".into());
    }
    Ok(x)
}

/// Observer: `C<n>`, `reader`, `narrator`.
fn obs(s: &str) -> Result<Obs, String> {
    match s {
        "reader" => Ok(Obs::Reader),
        "narrator" => Ok(Obs::Narrator),
        _ => Ok(Obs::C(id(s, &[Reg::C])?)),
    }
}

/// Several observers joined by `+`: `C1+C2+reader`.
fn obs_list(s: &str) -> Result<Vec<Obs>, String> {
    let mut out: Vec<Obs> = Vec::new();
    for p in s.split('+') {
        let o = obs(p)?;
        if out.contains(&o) {
            return Err(format!("{s}: {o} twice"));
        }
        out.push(o);
    }
    if out.len() > MAX_JOINT {
        return Err(format!("{s}: more than {MAX_JOINT}"));
    }
    Ok(out)
}

/// v3 placement in `reveal`: `before=|after=|during=|same=E` — only one.
fn placement(a: &mut Args) -> Result<Option<(TRel, Id)>, String> {
    let mut out = None;
    for (k, r) in [("before", TRel::Before), ("after", TRel::After), ("during", TRel::During), ("same", TRel::Same)] {
        if let Some(e) = a.id(k, &[Reg::E])? {
            if out.is_some() {
                return Err("only one placement in time".into());
            }
            out = Some((r, e));
        }
    }
    Ok(out)
}

/// v3 commands.
fn command_v3(name: &str, a: &mut Args) -> Result<Cmd, String> {
    Ok(match name {
        "frame" => {
            if a.pos.len() != 2 {
                return Err("frame open|close F<n>".into());
            }
            let op = a.pos.remove(0);
            let open = match op.as_str() {
                "open" => true,
                "close" => false,
                _ => return Err(format!("frame {op:?}: only open or close")),
            };
            let id = a.one_pos("frame id", &[Reg::F])?;
            let narrator = a.take("narrator").map(|(v, q)| if q { Err("narrator: without quotes".to_string()) } else { obs(&v) }).transpose()?;
            if narrator == Some(Obs::Reader) {
                return Err("narrator=reader: the reader does not narrate".into());
            }
            let to = match a.take("to") {
                Some((_, true)) => return Err("to: without quotes".into()),
                Some((v, false)) => obs_list(&v)?,
                None => Vec::new(),
            };
            let level = a.num("level")?.map(|x| u8::try_from(x).map_err(|_| format!("level={x}"))).transpose()?;
            let (world, time, before, src) = (a.tag("world", &FRAME_WORLD)?, a.tag("time", &FRAME_TIME)?, a.id("before", &[Reg::E])?, a.id("src", &[Reg::E])?);
            let branch = a.tag("branch", &BRANCH)?;
            if branch.is_some_and(|b| b.as_str() == "real") {
                return Err("frame branch=real — a branch frame is only for the unreal (dream, lie, plan, if …)".into());
            }
            if open {
                if narrator.is_none() || level.is_none() {
                    return Err("frame open: narrator= and level=".into());
                }
                if world.is_some_and(|w| w.as_str() == "new") && (time.is_some() || before.is_some()) {
                    return Err("frame world=new — its own time axis: no time= and before=".into());
                }
            } else if narrator.is_some() || !to.is_empty() || level.is_some() || world.is_some() || time.is_some() || before.is_some() || src.is_some() || branch.is_some() {
                return Err("frame close F<n> — no keys".into());
            }
            Cmd::Frame { id, open, narrator, to, level, world, time, before, src, branch }
        }
        "claim" => {
            let id = a.one_pos("claim id", &[Reg::K])?;
            let who = a.take("who").map(|(v, q)| if q { Err("who: without quotes".to_string()) } else { obs(&v) }).transpose()?;
            let about = a.id("about", &[Reg::E])?;
            let status = a.tag("status", &CLAIM_STATUS)?.ok_or("claim: missing status=")?;
            let (interp, from, by) = (a.text("interp")?, a.id("from", &[Reg::E])?, a.id("by", &[Reg::E])?);
            match (who.is_some(), about.is_some()) {
                (true, true) => {
                    if by.is_some() {
                        return Err("claim: a new claim — who, about, status, interp; by= only on change".into());
                    }
                    if interp.is_none() {
                        return Err("claim: missing interp= (the observer's interpretation)".into());
                    }
                }
                (false, false) => {
                    if interp.is_some() || from.is_some() {
                        return Err("claim: change — only status= and by=E".into());
                    }
                }
                _ => return Err("claim: a new claim — both who= and about=".into()),
            }
            Cmd::Claim { id, who, about, status, interp, from, by }
        }
        "time" => {
            let a1 = a.one_pos("event", &[Reg::E])?;
            let rel = if a.pos.is_empty() {
                None
            } else {
                if a.pos.len() != 2 {
                    return Err("time E1 BEFORE|AFTER|DURING|SAME E2".into());
                }
                let r = a.pos.remove(0);
                let r = TRel::parse(&r).ok_or_else(|| format!("time: {r:?} — only BEFORE AFTER DURING SAME"))?;
                let b = a.one_pos("second event", &[Reg::E])?;
                if b == a1 {
                    return Err(format!("time {a1} {} {a1}: an event with itself", r.name()));
                }
                Some((r, b))
            };
            let at = a.text("at")?;
            if rel.is_none() == at.is_none() {
                return Err("time: either a relation (E1 BEFORE E2) or at=\"…\"".into());
            }
            Cmd::Time { a: a1, rel, at }
        }
        "branch" => {
            let ev = a.one_pos("event", &[Reg::E])?;
            if a.pos.is_empty() {
                return Err("branch E<n> <branch>".into());
            }
            let b = a.pos.remove(0);
            let branch = BRANCH.tag(&b)?;
            Cmd::Branch { ev, branch, by: a.id("by", &[Reg::E])? }
        }
        "know" => {
            let who = a.take("who").map(|(v, q)| if q { Err("who: without quotes".to_string()) } else { obs(&v) }).transpose()?.ok_or("know: missing who=")?;
            if who == Obs::Narrator {
                return Err("know who=narrator: the narrator knows everything they narrate".into());
            }
            let fact = a.id("fact", &[Reg::E, Reg::K, Reg::P, Reg::B, Reg::C, Reg::O, Reg::L])?.ok_or("know: missing fact=")?;
            let from = match a.take("from") {
                Some((v, false)) => v.strip_prefix("^s").and_then(|x| x.parse::<u16>().ok()).filter(|x| *x > 0).ok_or_else(|| format!("from={v:?} — expected ^sN"))?,
                Some((_, true)) => return Err("from: without quotes".into()),
                None => return Err("know: missing from=^sN".into()),
            };
            Cmd::Know { who, fact, from, how: a.tag("how", &KNOW_HOW)?, by: a.id("by", &[Reg::E])? }
        }
        "reveal" => {
            let ev = a.one_pos("event", &[Reg::E])?;
            let to = match a.take("to") {
                Some((_, true)) => return Err("to: without quotes".into()),
                Some((v, false)) => obs_list(&v)?,
                None => Vec::new(),
            };
            if to.contains(&Obs::Narrator) {
                return Err("reveal to=narrator: the narrator already knows".into());
            }
            let (by, interp) = (a.id("by", &[Reg::E])?, a.text("interp")?);
            let wrong = match a.take("wrong") {
                Some((v, false)) => plus_ids(&v, &[Reg::K])?,
                Some((_, true)) => return Err("wrong: without quotes".into()),
                None => Vec::new(),
            };
            let right = match a.take("right") {
                Some((v, false)) => plus_ids(&v, &[Reg::K])?,
                Some((_, true)) => return Err("right: without quotes".into()),
                None => Vec::new(),
            };
            if let Some(k) = wrong.iter().find(|k| right.contains(k)) {
                return Err(format!("reveal: {k} both wrong and right"));
            }
            let mut roles = Vec::new();
            for r in [Role::Agent, Role::Patient, Role::To] {
                for x in a.ids(r.name(), r.regs())? {
                    roles.push((r, x));
                }
            }
            if let Some(x) = a.id("instr", Role::Instr.regs())? {
                roles.push((Role::Instr, x));
            }
            let cause = a.id("cause", &[Reg::E])?;
            let rel = placement(a)?;
            if cause == Some(ev) || rel.is_some_and(|(_, e)| e == ev) {
                return Err(format!("reveal {ev}: reference to itself"));
            }
            if interp.is_none() && wrong.is_empty() && right.is_empty() && roles.is_empty() && cause.is_none() && rel.is_none() {
                return Err("reveal: what was revealed? interp=, wrong=, right=, a role, cause= or time".into());
            }
            Cmd::Reveal { ev, to, by, interp, wrong, right, roles, cause, rel }
        }
        _ => return Err(format!("unknown command {name:?}")),
    })
}

/// Parse the whole LLM output: lines and format rejections (empty lines are skipped).
pub fn parse_all(text: &str) -> (Vec<Line>, Vec<FormatErr>) {
    let mut lines = Vec::new();
    let mut errs = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        match parse_line(i + 1, raw) {
            Ok(Some(l)) => lines.push(l),
            Ok(None) => {}
            Err(e) => errs.push(e),
        }
    }
    (lines, errs)
}
