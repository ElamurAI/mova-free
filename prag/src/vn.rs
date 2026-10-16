//! VerbNet and WordNet in the reader: the linguists' open rules from syntax to roles (tables made by
//! `prag lexicon convert` / `prag lexicon nominals`, `crate::vnconv`; compiled in). The reader (`crate::frames`)
//! describes a clause — its subject, objects, prepositional phrases, clause complement, voice — and asks for the
//! best-matching frame of the verb's VerbNet classes; the frame names the role of every argument. Among a verb's
//! classes the frame match decides (arguments placed, selectional restrictions); a near tie goes to the class the
//! verb's clauses in our lessons match best most often (`prag/data/vnfreq.tsv`, counted by `world frames vnfreq`).
//! WordNet's derivational links give the verb of a noun that names its event («the arrival of the Queen» → arrive)
//! and whether the verb takes an object (`prag/data/nominals.tsv`).
//!
//! Only permissive data is built in (VerbNet 3.0 License, WordNet 3.0 License, our own lessons). PropBank (CC BY-SA)
//! and NomBank (research-only) are tests only: converted outside the repo, never read here (docs/pragmatics.md).
//!
//! Deterministic: ordered maps, integer scores, ties to the first class and frame in file order.

use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

const VERBNET: &str = include_str!("../data/verbnet.tsv");
const NOMINALS: &str = include_str!("../data/nominals.tsv");
const VNFREQ: &str = include_str!("../data/vnfreq.tsv");

/// An element of a VerbNet frame's syntax.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum El {
    /// a noun phrase with its thematic role and restrictions (`+animate`, `+that_comp` …)
    Np { role: String, restr: String },
    V,
    /// a preposition: the words it can be (a restriction type `+src` is spelt out)
    Prep(Vec<String>),
    Lex(String),
    Adj,
    Adv,
}

#[derive(Clone, Debug)]
pub struct VFrame {
    pub primary: String,
    pub syntax: Vec<El>,
    pub sem: String,
    pub example: String,
}

#[derive(Clone, Debug)]
pub struct Class {
    pub id: String,
    pub parent: Option<usize>,
    pub members: Vec<String>,
    /// role → its selectional restrictions
    pub roles: Vec<(String, String)>,
    pub frames: Vec<VFrame>,
}

/// A noun that names a verb's event (WordNet's derivational link of an act / event / state sense): its verb, the
/// noun's lexicographer file, and whether the linked verb sense takes an object (WordNet's verb frames).
#[derive(Clone, Debug, Default)]
pub struct Nom {
    pub noun: String,
    pub verb: String,
    /// act, event, process, state, communication, cognition, feeling, phenomenon, possession
    pub file: String,
    pub transitive: bool,
}

#[derive(Default)]
pub struct Lex {
    pub classes: Vec<Class>,
    by_id: BTreeMap<String, usize>,
    by_member: BTreeMap<String, Vec<usize>>,
    pub noms: BTreeMap<String, Nom>,
    /// verb → (class, how many of the verb's lesson clauses that class matched best), most first
    freq: BTreeMap<String, Vec<(usize, usize)>>,
}
/// The prepositions a VerbNet restriction type stands for (VerbNet's own definitions of the spatial types).
fn prep_type(t: &str) -> Vec<&'static str> {
    const LOC: [&str; 33] = ["about", "above", "against", "along", "alongside", "amid", "among", "amongst", "around", "astride", "at", "before", "behind", "below", "beneath", "beside", "between", "beyond", "by", "in", "inside", "near", "next", "on", "opposite", "out", "outside", "over", "through", "throughout", "under", "underneath", "upon"];
    const DIR: [&str; 14] = ["across", "along", "around", "down", "into", "onto", "out", "over", "past", "round", "through", "toward", "towards", "up"];
    const SRC: [&str; 4] = ["from", "out", "off", "of"];
    const DEST_CONF: [&str; 2] = ["into", "onto"];
    // (VerbNet lists «for» too — «head for the hills» — but on our texts «for» is a purpose or a beneficiary far more often)
    const DEST_DIR: [&str; 4] = ["into", "onto", "to", "towards"];
    let t = t.trim_start_matches('+');
    let mut v: Vec<&'static str> = match t {
        "loc" => LOC.to_vec(),
        "dir" => DIR.to_vec(),
        "src" | "src_conf" => SRC.to_vec(),
        "dest_conf" => DEST_CONF.to_vec(),
        "dest_dir" | "gol" => DEST_DIR.to_vec(),
        "dest" => [DEST_CONF.as_slice(), DEST_DIR.as_slice()].concat(),
        "path" => [SRC.as_slice(), DIR.as_slice(), DEST_DIR.as_slice()].concat(),
        "spatial" => [LOC.as_slice(), SRC.as_slice(), DIR.as_slice(), DEST_DIR.as_slice()].concat(),
        "at" => vec!["at"],
        _ => Vec::new(),
    };
    v.sort_unstable();
    v.dedup();
    v
}

/// One element of the syntax column: `Agent[+animate]`, `V`, `{to|into}`, `{+src}`, `'there'`, `ADJ`, `ADV`.
fn element(tok: &str) -> El {
    if tok == "V" {
        return El::V;
    }
    if tok == "ADJ" {
        return El::Adj;
    }
    if tok == "ADV" {
        return El::Adv;
    }
    if let Some(w) = tok.strip_prefix('\'') {
        return El::Lex(w.trim_end_matches('\'').to_lowercase());
    }
    if let Some(b) = tok.strip_prefix('{') {
        let (inner, rest) = b.split_once('}').unwrap_or((b, ""));
        let mut ps: Vec<String> = Vec::new();
        for p in inner.split(['|', '/']) {
            if p.starts_with('+') {
                ps.extend(prep_type(p).into_iter().map(String::from));
            } else if !p.is_empty() {
                ps.push(p.to_lowercase());
            }
        }
        // a restriction after the words: «{to}[+dest]» keeps the words
        if ps.is_empty() {
            for t in rest.trim_matches(|c| c == '[' || c == ']').split(['|', '&', '(', ')']) {
                ps.extend(prep_type(t).into_iter().map(String::from));
            }
        }
        ps.sort();
        ps.dedup();
        return El::Prep(ps);
    }
    let (role, restr) = match tok.split_once('[') {
        Some((r, x)) => (r.to_string(), x.trim_end_matches(']').to_string()),
        None => (tok.to_string(), String::new()),
    };
    El::Np { role, restr }
}

/// The syntax column split into elements (restrictions in brackets may hold spaces only inside parentheses).
fn syntax(s: &str) -> Vec<El> {
    s.split_whitespace().map(element).collect()
}

impl Lex {
    fn load() -> Lex {
        let mut x = Lex::default();
        for l in VERBNET.lines().filter(|l| !l.starts_with('#')) {
            let c: Vec<&str> = l.split('\t').collect();
            match c[0] {
                "class" if c.len() >= 5 => {
                    let parent = x.by_id.get(c[2]).copied();
                    let roles = c[4].split_whitespace().map(|r| match r.split_once('[') {
                        Some((a, b)) => (a.to_string(), b.trim_end_matches(']').to_string()),
                        None => (r.to_string(), String::new()),
                    }).collect();
                    let members: Vec<String> = c[3].split(',').filter(|m| !m.is_empty()).map(String::from).collect();
                    let i = x.classes.len();
                    for m in &members {
                        x.by_member.entry(m.clone()).or_default().push(i);
                    }
                    x.by_id.insert(c[1].to_string(), i);
                    x.classes.push(Class { id: c[1].to_string(), parent, members, roles, frames: Vec::new() });
                }
                "frame" if c.len() >= 6 => {
                    if let Some(&i) = x.by_id.get(c[1]) {
                        x.classes[i].frames.push(VFrame { primary: c[2].to_string(), syntax: syntax(c[3]), sem: c[4].to_string(), example: c[5].to_string() });
                    }
                }
                _ => {}
            }
        }
        for l in NOMINALS.lines().filter(|l| !l.starts_with('#')) {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() < 4 {
                continue;
            }
            x.noms.entry(c[0].to_string()).or_insert(Nom { noun: c[0].into(), verb: c[1].into(), file: c[2].into(), transitive: c[3] == "t" });
        }
        for l in VNFREQ.lines().filter(|l| !l.starts_with('#')) {
            let c: Vec<&str> = l.split('\t').collect();
            let (Some(v), Some(&k), Some(n)) = (c.first(), c.get(1).and_then(|id| x.by_id.get(*id)), c.get(2).and_then(|n| n.parse::<usize>().ok())) else { continue };
            x.freq.entry(v.to_string()).or_default().push((k, n));
        }
        for v in x.freq.values_mut() {
            v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        }
        x
    }

    /// The classes a verb belongs to (its own members; «put down» before «put»).
    pub fn classes_of(&self, verb: &str) -> &[usize] {
        self.by_member.get(verb).map(Vec::as_slice).unwrap_or(&[])
    }

    /// The preference for a class of a verb from our lessons (`vnfreq.tsv`): the class its clauses match best most
    /// often +2, another class that ever matched best +1 — a tie-breaker in half points, below one argument (+4).
    pub fn lesson_pref(&self, verb: &str, c: usize) -> i32 {
        let Some(v) = self.freq.get(verb) else { return 0 };
        match v.iter().position(|(k, _)| *k == c) {
            Some(0) if v.get(1).is_none_or(|x| x.1 < v[0].1) => 2,
            Some(_) => 1,
            None => 0,
        }
    }

    /// The class and its ancestors (a subclass inherits its parent's frames and roles).
    pub fn chain(&self, c: usize) -> Vec<usize> {
        let mut v = vec![c];
        let mut x = self.classes[c].parent;
        while let Some(p) = x {
            v.push(p);
            x = self.classes[p].parent;
        }
        v
    }

    /// The restrictions of a role in a class (the nearest class of the chain that names it).
    pub fn restr(&self, c: usize, role: &str) -> String {
        for k in self.chain(c) {
            if let Some((_, r)) = self.classes[k].roles.iter().find(|(x, _)| x == role) {
                return r.clone();
            }
        }
        String::new()
    }

    /// A class of verbs of communication (VerbNet 37.x: say, tell, talk, manner of speaking, inquire, complain …).
    pub fn communication(&self, c: usize) -> bool {
        let id = &self.classes[c].id;
        id.find(|ch: char| ch.is_ascii_digit()).is_some_and(|p| id[p..].starts_with("37."))
    }
}

pub fn lex() -> &'static Lex {
    static L: OnceLock<Lex> = OnceLock::new();
    L.get_or_init(Lex::load)
}

// ---- matching a clause against the frames ----------------------------------------------------------------------

/// What fills an argument position, as far as the restrictions care.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Filler {
    /// Some(true) a person or an animal, Some(false) a thing, None unknown
    pub animate: Option<bool>,
}

/// A clause complement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comp {
    /// a finite clause («that …», a bare clause)
    That,
    /// an infinitive («to …»)
    To,
    /// a gerund («-ing»)
    Ing,
    /// an indirect question («what / how / whether …»)
    Wh,
    /// quoted speech
    Quote,
}

/// A clause as the matcher sees it: the arguments in the order the tree has them.
#[derive(Clone, Debug, Default)]
pub struct Clause {
    /// the verbs to look up, the most specific first («put down», «put»)
    pub verbs: Vec<String>,
    pub subj: Option<Filler>,
    pub passive: bool,
    /// the passive's «by» phrase
    pub by: Option<Filler>,
    /// objects: the indirect object first («give the dog a bone»)
    pub objs: Vec<Filler>,
    /// prepositional phrases: (preposition, filler)
    pub pps: Vec<(String, Filler)>,
    pub comp: Option<Comp>,
    /// an adjective complement («get tiny»)
    pub adj: bool,
}

/// The best frame for a clause and the role it gives each argument.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Match {
    /// the verb's own class (its restrictions), and the class of the chain the frame comes from
    pub class: usize,
    pub fclass: usize,
    pub frame: usize,
    pub score: i32,
    /// the verb looked up that matched («put down» or «put»)
    pub verb: String,
    pub subj: Option<String>,
    pub by: Option<String>,
    pub objs: Vec<Option<String>>,
    pub pps: Vec<Option<String>>,
    pub comp: Option<String>,
}

impl Match {
    pub fn class_id(&self) -> &'static str {
        &lex().classes[self.class].id
    }
    pub fn vframe(&self) -> &'static VFrame {
        &lex().classes[self.fclass].frames[self.frame]
    }
}

/// Synrestr types that make an NP a clause.
const SENTENTIAL: [&str; 27] = ["sentential", "that_comp", "tensed_that", "wh_comp", "what_extract", "how_extract", "wh_inf", "what_inf", "wheth_inf", "sc_ing", "ac_ing", "oc_ing", "be_sc_ing", "sc_to_inf", "ac_to_inf", "oc_to_inf", "rs_to_inf", "np_to_inf", "for_comp", "quotation", "poss_ing", "np_ppart", "np_omit_ing", "small_clause", "to_be", "wh_extract", "wh_ing"];

fn plus(restr: &str, t: &str) -> bool {
    restr.split(['|', '&', '(', ')', '[', ']']).any(|x| x == format!("+{t}"))
}

/// The shape of a frame after its verb: bare objects, clause complements, prepositional phrases, an adjective.
#[derive(Debug, Default)]
struct Shape {
    subj: Option<String>,
    expletive: bool,
    objs: Vec<String>,
    comps: Vec<(String, String)>,
    pps: Vec<(Vec<String>, String)>,
    adj: bool,
}

const LEX_PREPS: [&str; 14] = ["at", "of", "to", "as", "for", "with", "on", "in", "from", "into", "about", "against", "like", "upon"];

fn shape(f: &VFrame) -> Shape {
    let mut s = Shape::default();
    let v = f.syntax.iter().position(|e| *e == El::V).unwrap_or(0);
    for e in &f.syntax[..v] {
        match e {
            El::Np { role, .. } if s.subj.is_none() => s.subj = Some(role.clone()),
            El::Lex(w) if matches!(w.as_str(), "there" | "it") => s.expletive = true,
            _ => {}
        }
    }
    let post = &f.syntax[(v + 1).min(f.syntax.len())..];
    let mut i = 0;
    while i < post.len() {
        match &post[i] {
            El::Prep(ps) => {
                if let Some(El::Np { role, .. }) = post.get(i + 1) {
                    s.pps.push((ps.clone(), role.clone()));
                    i += 1;
                }
            }
            El::Lex(w) if LEX_PREPS.contains(&w.as_str()) => {
                if let Some(El::Np { role, .. }) = post.get(i + 1) {
                    s.pps.push((vec![w.clone()], role.clone()));
                    i += 1;
                }
            }
            El::Np { role, restr } => {
                if SENTENTIAL.iter().any(|t| plus(restr, t)) {
                    s.comps.push((role.clone(), restr.clone()));
                } else {
                    s.objs.push(role.clone());
                }
            }
            El::Adj => s.adj = true,
            _ => {}
        }
        i += 1;
    }
    s
}

/// How well a filler fits a role's restrictions: +1 a fit, −2 a clash, 0 unknown.
fn fits(restr: &str, f: Filler) -> i32 {
    let wants_an = plus(restr, "animate") || plus(restr, "human");
    let wants_inan = restr.split(['|', '&', '(', ')']).any(|x| x == "-animate") || plus(restr, "concrete") && !wants_an;
    match f.animate {
        Some(true) if wants_an => 1,
        Some(true) if wants_inan => -2,
        Some(false) if wants_an && !plus(restr, "organization") && !plus(restr, "machine") => -2,
        Some(false) if wants_inan => 1,
        _ => 0,
    }
}

/// Scores (in half points): a matched argument +4, a frame's element left empty −3, a clause's object or
/// complement the frame has no place for −4, a clause's prepositional phrase it has no place for −1 (adjuncts are
/// common), a restriction fit +2, a clash −4; (the lessons' preference, `Lex::lesson_pref`, is added by `best`).
fn score(lx: &Lex, c: usize, f: &VFrame, cl: &Clause) -> Option<(i32, Match)> {
    let sh = shape(f);
    if sh.expletive {
        return None;
    }
    let mut m = Match { class: c, objs: vec![None; cl.objs.len()], pps: vec![None; cl.pps.len()], ..Default::default() };
    let mut sc = 0;
    let restr = |role: &str| lx.restr(c, role);
    // the arguments that stand where a bare object stands: in the passive the subject is the first object
    let mut objs: Vec<(usize, Filler)> = Vec::new();
    let subj_role = sh.subj.clone();
    if cl.passive {
        if let Some(s) = cl.subj {
            objs.push((usize::MAX, s));
        }
    }
    objs.extend(cl.objs.iter().copied().enumerate());
    let mut fobjs = sh.objs.clone();
    // a passive's subject can be the frame's only complement too («it was said that …» is rare; «the bottle was flung»)
    if objs.len() != fobjs.len() {
        sc -= 4 * (objs.len() as i32 - fobjs.len() as i32).abs();
    }
    // a ditransitive with one object in the clause: the object is the frame's theme (the last), not the recipient
    if objs.len() == 1 && fobjs.len() == 2 {
        fobjs.remove(0);
    }
    for (k, (i, fl)) in objs.iter().enumerate() {
        let Some(role) = fobjs.get(k) else { break };
        sc += 4 + 2 * fits(&restr(role), *fl);
        if *i == usize::MAX {
            m.subj = Some(role.clone());
        } else {
            m.objs[*i] = Some(role.clone());
        }
    }
    if let Some(r) = &subj_role {
        if cl.passive {
            if let Some(b) = cl.by {
                sc += 4 + 2 * fits(&restr(r), b);
                m.by = Some(r.clone());
            }
        } else if let Some(s) = cl.subj {
            sc += 2 * fits(&restr(r), s);
            m.subj = Some(r.clone());
        } else {
            m.subj = Some(r.clone());
        }
    }
    match (cl.comp, sh.comps.first()) {
        (Some(k), Some((role, r))) => {
            let ok = match k {
                Comp::Quote => plus(r, "quotation") || plus(r, "sentential") || plus(r, "that_comp"),
                Comp::That => !plus(r, "quotation") || plus(r, "sentential"),
                Comp::To => ["sc_to_inf", "ac_to_inf", "oc_to_inf", "rs_to_inf", "np_to_inf", "wh_inf", "what_inf", "wheth_inf", "for_comp", "to_be", "sentential"].iter().any(|t| plus(r, t)),
                Comp::Ing => ["sc_ing", "ac_ing", "oc_ing", "be_sc_ing", "poss_ing", "np_omit_ing", "wh_ing"].iter().any(|t| plus(r, t)),
                Comp::Wh => ["wh_comp", "what_extract", "how_extract", "wh_inf", "what_inf", "wheth_inf", "wh_extract", "sentential"].iter().any(|t| plus(r, t)),
            };
            sc += if ok { 4 } else { 1 };
            m.comp = Some(role.clone());
        }
        (Some(_), None) => sc -= 4,
        (None, Some(_)) => sc -= 3,
        (None, None) => {}
    }
    let mut used = vec![false; sh.pps.len()];
    for (k, (p, fl)) in cl.pps.iter().enumerate() {
        if let Some(j) = (0..sh.pps.len()).find(|&j| !used[j] && sh.pps[j].0.iter().any(|x| x == p)) {
            used[j] = true;
            let role = &sh.pps[j].1;
            sc += 4 + 2 * fits(&restr(role), *fl);
            m.pps[k] = Some(role.clone());
        } else {
            sc -= 1;
        }
    }
    sc -= 3 * used.iter().filter(|u| !**u).count() as i32;
    match (cl.adj, sh.adj) {
        (true, true) => sc += 2,
        (true, false) => sc -= 2,
        (false, true) => sc -= 3,
        _ => {}
    }
    Some((sc, m))
}

/// The best VerbNet frame for a clause, if the verb has a class and some frame matches its core (the objects and
/// the complement): None leaves the roles to the reader's heuristics.
pub fn best(cl: &Clause) -> Option<Match> {
    let lx = lex();
    let rec = RECORD.load(Ordering::Relaxed);
    for v in &cl.verbs {
        let cs = lx.classes_of(v);
        if cs.is_empty() {
            continue;
        }
        let mut best: Option<(i32, Match)> = None;
        // each class's best frame without the lessons' preference (what `world frames vnfreq` counts)
        let mut per_class: Vec<(usize, i32)> = Vec::new();
        for &c in cs {
            let mut cb: Option<i32> = None;
            for k in lx.chain(c) {
                for (fi, f) in lx.classes[k].frames.iter().enumerate() {
                    let Some((s0, mut m)) = score(lx, c, f, cl) else { continue };
                    cb = Some(cb.map_or(s0, |b| b.max(s0)));
                    let s = s0 + if rec { 0 } else { lx.lesson_pref(v, c) };
                    m.class = c;
                    m.frame = fi;
                    m.score = s;
                    m.verb = v.clone();
                    m.fclass = k;
                    if best.as_ref().is_none_or(|(b, _)| s > *b) {
                        best = Some((s, m));
                    }
                }
            }
            if let Some(b) = cb {
                per_class.push((c, b));
            }
        }
        // the core must match: every object and the complement found a place
        let best = best.filter(|(_, m)| m.objs.iter().all(Option::is_some) && (cl.comp.is_none() || m.comp.is_some()) && (!cl.passive || cl.subj.is_none() || m.subj.is_some())).map(|(_, m)| m);
        // recording: a class that alone matched the clause best (the core matched) counts once for the verb
        if rec && best.is_some() && cs.len() > 1 {
            let top = per_class.iter().map(|x| x.1).max();
            let winners: Vec<usize> = per_class.iter().filter(|x| Some(x.1) == top).map(|x| x.0).collect();
            if winners.len() == 1 {
                let mut m = COUNTS.lock().unwrap_or_else(|e| e.into_inner());
                *m.entry((v.clone(), lx.classes[winners[0]].id.clone())).or_default() += 1;
            }
        }
        return best;
    }
    None
}

static RECORD: AtomicBool = AtomicBool::new(false);
static COUNTS: Mutex<BTreeMap<(String, String), usize>> = Mutex::new(BTreeMap::new());

/// Start counting, for every clause `best` reads, the class that alone matches it best (the lessons' preference
/// switched off while counting); `record_take` stops and returns (verb, class) → clauses.
pub fn record_start() {
    COUNTS.lock().unwrap_or_else(|e| e.into_inner()).clear();
    RECORD.store(true, Ordering::Relaxed);
}

pub fn record_take() -> BTreeMap<(String, String), usize> {
    RECORD.store(false, Ordering::Relaxed);
    std::mem::take(&mut *COUNTS.lock().unwrap_or_else(|e| e.into_inner()))
}

/// A noun that names the event of a verb (WordNet: «arrival» → arrive, «destruction» → destroy).
pub fn nom(noun: &str) -> Option<&'static Nom> {
    lex().noms.get(noun)
}

/// A verb of communication by VerbNet: every class it belongs to is a 37.x class (say, tell, talk, chitchat,
/// manner of speaking, inquire, complain, advise, lecture …). Verbs with other senses too («go», «call», «add»)
/// are left to the reader's own lists.
pub fn speech_verb(verb: &str) -> bool {
    let lx = lex();
    let cs = lx.classes_of(verb);
    !cs.is_empty() && cs.iter().all(|&c| lx.communication(c))
}

/// All verbs `speech_verb` holds (for showing).
pub fn speech_verbs() -> Vec<&'static str> {
    lex().by_member.keys().filter(|v| speech_verb(v)).map(String::as_str).collect()
}
