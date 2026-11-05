//! The frame benchmark's scorer: frames written in the notation `Frame::show` prints (the teacher's gold and the
//! reader's own frames go through the same parser), aligned line by line, scored role by role. Deterministic: no
//! hashing, ties broken by position.
//!
//! Notation (one frame): `Kind agent [NOT] verb [key=ent …] [attrs=a,b] [| says→#i,#j cause→#k next→#k quoted if]`,
//! Kind = Event | Fact(is a|is|has|can|does|used for|made of|lives in) | Say(act) | Bare; an entity is
//! `head+other(«pronoun»)`, `_` = none.
//!
//! Roles scored (precision / recall / F1, micro over the lines): agent, verb (with its particle), patient, recipient
//! (`to=`), place, speaker (a Say frame aligned to a Say frame: the same act and the same speaker), says (the quote
//! links), cause, next (links mapped through the alignment), coref (pronouns in roles: resolved to the same entity).
//! Overall = micro over all of them.

use std::collections::BTreeMap;

/// An entity of the notation: its heads (the first is the head, the rest coordinated) and the pronoun it came from.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GEnt {
    pub ids: Vec<String>,
    pub pron: Option<String>,
}

impl GEnt {
    pub fn head(&self) -> &str {
        self.ids.first().map(String::as_str).unwrap_or("")
    }
    /// The same entity: a shared head (coordination counts either way).
    pub fn same(&self, o: &GEnt) -> bool {
        self.ids.iter().any(|a| o.ids.iter().any(|b| a == b))
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GFrame {
    pub kind: String,
    pub agent: Option<GEnt>,
    pub verb: String,
    pub neg: bool,
    pub patient: Option<GEnt>,
    pub to: Option<GEnt>,
    pub place: Option<GEnt>,
    pub obls: Vec<(String, GEnt)>,
    pub attrs: Vec<String>,
    pub says: Vec<usize>,
    pub cause: Option<usize>,
    pub next: Option<usize>,
    pub quoted: bool,
    pub cond: bool,
}

/// A head lemma as the reader writes it: lower case, the last word of a multi-word name («March Hare» → hare).
pub fn norm_id(s: &str) -> String {
    let s = s.trim().trim_matches(|c: char| c == '"' || c == '\'' || c == '`').to_lowercase();
    let s = s.rsplit([' ', '_']).next().unwrap_or("").to_string();
    s
}

fn ent(s: &str) -> Option<GEnt> {
    let s = s.trim();
    if s.is_empty() || s == "_" {
        return None;
    }
    let (body, pron) = match s.find("(«") {
        Some(i) => (&s[..i], Some(s[i + "(«".len()..].trim_end_matches(')').trim_end_matches('»').to_lowercase())),
        None => (s, None),
    };
    let ids: Vec<String> = body.split('+').map(norm_id).filter(|x| !x.is_empty()).collect();
    if ids.is_empty() {
        return pron.map(|p| GEnt { ids: vec![p.clone()], pron: Some(p) });
    }
    Some(GEnt { ids, pron })
}

fn idx(s: &str) -> Option<usize> {
    s.trim().trim_start_matches('#').parse().ok()
}

/// Parse one frame of the notation; None for an empty or unreadable line.
pub fn parse_frame(line: &str) -> Option<GFrame> {
    let line = line.trim().trim_start_matches(|c: char| c == '#' || c.is_ascii_digit()).trim();
    let line = line.replace(['‑', '–'], "-");
    if line.is_empty() {
        return None;
    }
    let (main, links) = match line.split_once(" | ") {
        Some((a, b)) => (a.to_string(), b.to_string()),
        None => match line.strip_suffix(" |") {
            Some(a) => (a.to_string(), String::new()),
            None => (line.clone(), String::new()),
        },
    };
    let main = main.trim();
    // the kind: «Fact(is a)» has a space inside
    let (kind, rest) = if let Some(close) = main.find(')').filter(|_| main.starts_with("Fact(") || main.starts_with("Say(")) {
        (main[..=close].to_string(), main[close + 1..].trim())
    } else {
        let (k, r) = main.split_once(' ').unwrap_or((main, ""));
        (k.to_string(), r.trim())
    };
    if !(kind == "Event" || kind == "Bare" || kind.starts_with("Fact(") || kind.starts_with("Say(")) {
        return None;
    }
    let toks: Vec<&str> = rest.split_whitespace().collect();
    let mut f = GFrame { kind, ..Default::default() };
    let mut i = 0;
    if let Some(&a) = toks.first() {
        if !a.contains('=') {
            f.agent = ent(a);
            i = 1;
        }
    }
    let mut verb: Vec<&str> = Vec::new();
    while i < toks.len() && !toks[i].contains('=') {
        if toks[i] == "NOT" {
            f.neg = true;
        } else {
            verb.push(toks[i]);
        }
        i += 1;
    }
    f.verb = verb.join(" ").to_lowercase();
    // key=value; a value may run over spaces until the next key («with=queen» stays one token, a stray space joins)
    let mut kv: Vec<(String, String)> = Vec::new();
    while i < toks.len() {
        if let Some((k, v)) = toks[i].split_once('=') {
            kv.push((k.to_string(), v.to_string()));
        } else if let Some(last) = kv.last_mut() {
            last.1 += &format!(" {}", toks[i]);
        }
        i += 1;
    }
    for (k, v) in kv {
        match k.as_str() {
            "patient" => f.patient = ent(&v),
            "to" | "recipient" => f.to = ent(&v),
            "place" => f.place = ent(&v),
            "attrs" => f.attrs = v.split(',').map(|x| x.trim().to_lowercase()).filter(|x| !x.is_empty()).collect(),
            "agent" => f.agent = ent(&v),
            _ => {
                if let Some(e) = ent(&v) {
                    f.obls.push((k.to_lowercase(), e));
                }
            }
        }
    }
    for tok in links.split_whitespace() {
        let tok = tok.replace("->", "→");
        if let Some(v) = tok.strip_prefix("says→") {
            f.says.extend(v.split(',').filter_map(idx));
        } else if let Some(v) = tok.strip_prefix("cause→") {
            f.cause = idx(v);
        } else if let Some(v) = tok.strip_prefix("next→") {
            f.next = idx(v);
        } else if tok == "quoted" {
            f.quoted = true;
        } else if tok == "if" {
            f.cond = true;
        }
    }
    Some(f)
}

/// The roles scored, in report order.
pub const ROLES: [&str; 10] = ["agent", "verb", "patient", "recipient", "place", "speaker", "says", "cause", "next", "coref"];

/// Counts of one role: right, made by the reader, in the gold.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Prf {
    pub tp: usize,
    pub sys: usize,
    pub gold: usize,
}

impl Prf {
    pub fn add(&mut self, o: Prf) {
        self.tp += o.tp;
        self.sys += o.sys;
        self.gold += o.gold;
    }
    pub fn p(&self) -> f64 {
        if self.sys == 0 { 0.0 } else { self.tp as f64 / self.sys as f64 }
    }
    pub fn r(&self) -> f64 {
        if self.gold == 0 { 0.0 } else { self.tp as f64 / self.gold as f64 }
    }
    pub fn f1(&self) -> f64 {
        let (p, r) = (self.p(), self.r());
        if p + r == 0.0 { 0.0 } else { 2.0 * p * r / (p + r) }
    }
}

fn verb_head(v: &str) -> &str {
    v.split_whitespace().next().unwrap_or("")
}

fn eq_ent(a: &Option<GEnt>, b: &Option<GEnt>) -> bool {
    matches!((a, b), (Some(x), Some(y)) if x.same(y))
}

/// The similarity of two frames for the alignment; None when they cannot be the same predicate (neither the verb's
/// head nor two of the roles agree).
fn sim(g: &GFrame, s: &GFrame) -> Option<i32> {
    let vh = !g.verb.is_empty() && verb_head(&g.verb) == verb_head(&s.verb);
    let roles = [eq_ent(&g.agent, &s.agent), eq_ent(&g.patient, &s.patient), eq_ent(&g.to, &s.to), eq_ent(&g.place, &s.place)].iter().filter(|x| **x).count() as i32;
    if !vh && roles < 2 {
        return None;
    }
    Some(vh as i32 * 4 + (g.verb == s.verb) as i32 + roles * 2 + (g.kind == s.kind) as i32)
}

/// Gold frame → reader frame, greedily by similarity (ties: the earlier gold, then the earlier reader frame).
pub fn align(gold: &[GFrame], sys: &[GFrame]) -> BTreeMap<usize, usize> {
    let mut pairs: Vec<(i32, usize, usize)> = Vec::new();
    for (i, g) in gold.iter().enumerate() {
        for (j, s) in sys.iter().enumerate() {
            if let Some(x) = sim(g, s) {
                pairs.push((x, i, j));
            }
        }
    }
    pairs.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
    let mut m = BTreeMap::new();
    let mut used = std::collections::BTreeSet::new();
    for (_, i, j) in pairs {
        if m.contains_key(&i) || used.contains(&j) {
            continue;
        }
        m.insert(i, j);
        used.insert(j);
    }
    m
}

/// The pronouns of a line's frames in roles, in reading order: (pronoun, what it was resolved to).
pub fn pronouns(fs: &[GFrame]) -> Vec<(String, String)> {
    let mut v = Vec::new();
    for f in fs {
        for e in [&f.agent, &f.patient, &f.to, &f.place].into_iter().flatten().chain(f.obls.iter().map(|(_, e)| e)) {
            if let Some(p) = &e.pron {
                if COREF.contains(&p.as_str()) {
                    v.push((p.clone(), e.head().to_string()));
                }
            }
        }
    }
    v
}

/// The pronouns the coreference score counts (the first and second person count inside quotes, where the reader
/// should resolve them to the speaker and the addressee).
pub const COREF: [&str; 23] = ["he", "him", "his", "himself", "she", "her", "hers", "herself", "it", "its", "itself", "they", "them", "their", "themselves", "i", "me", "my", "myself", "you", "your", "we", "us"];

/// One line scored: the counts per role (index as `ROLES`).
pub fn score_line(gold: &[GFrame], sys: &[GFrame]) -> [Prf; 10] {
    let mut out = [Prf::default(); 10];
    let m = align(gold, sys);
    let inv: BTreeMap<usize, usize> = m.iter().map(|(a, b)| (*b, *a)).collect();
    let slot = |f: &GFrame, r: usize| -> Option<GEnt> {
        match r {
            0 => f.agent.clone(),
            2 => f.patient.clone(),
            3 => f.to.clone(),
            4 => f.place.clone(),
            _ => None,
        }
    };
    for r in [0usize, 2, 3, 4] {
        out[r].gold = gold.iter().filter(|f| slot(f, r).is_some()).count();
        out[r].sys = sys.iter().filter(|f| slot(f, r).is_some()).count();
        out[r].tp = m.iter().filter(|(i, j)| eq_ent(&slot(&gold[**i], r), &slot(&sys[**j], r))).count();
    }
    out[1].gold = gold.iter().filter(|f| !f.verb.is_empty()).count();
    out[1].sys = sys.iter().filter(|f| !f.verb.is_empty()).count();
    out[1].tp = m.iter().filter(|(i, j)| !gold[**i].verb.is_empty() && gold[**i].verb == sys[**j].verb).count();
    let is_say = |f: &GFrame| f.kind.starts_with("Say(");
    out[5].gold = gold.iter().filter(|f| is_say(f)).count();
    out[5].sys = sys.iter().filter(|f| is_say(f)).count();
    out[5].tp = m.iter().filter(|(i, j)| is_say(&gold[**i]) && gold[**i].kind == sys[**j].kind && eq_ent(&gold[**i].agent, &sys[**j].agent)).count();
    // links, mapped through the alignment
    type Links = fn(&GFrame) -> Vec<usize>;
    let links: [(usize, Links); 3] = [(6, |f| f.says.clone()), (7, |f| f.cause.into_iter().collect()), (8, |f| f.next.into_iter().collect())];
    for (r, get) in links {
        let gl: Vec<(usize, usize)> = gold.iter().enumerate().flat_map(|(i, f)| get(f).into_iter().map(move |t| (i, t))).filter(|(_, t)| *t < gold.len()).collect();
        let sl: Vec<(usize, usize)> = sys.iter().enumerate().flat_map(|(i, f)| get(f).into_iter().map(move |t| (i, t))).filter(|(_, t)| *t < sys.len()).collect();
        out[r].gold = gl.len();
        out[r].sys = sl.len();
        // Links are occurrences: each gold occurrence can credit at most one prediction.
        // Unique links retain the original behavior; duplicates cannot inflate recall.
        let mut remaining: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        for link in gl { *remaining.entry(link).or_default() += 1; }
        for (a, b) in sl {
            if let (Some(x), Some(y)) = (inv.get(&a), inv.get(&b)) {
                if let Some(left) = remaining.get_mut(&(*x, *y)) {
                    if *left > 0 { *left -= 1; out[r].tp += 1; }
                }
            }
        }
    }
    // coreference: the k-th occurrence of a pronoun in the gold against the k-th in the reader's frames
    let gp = pronouns(gold);
    let sp = pronouns(sys);
    out[9].gold = gp.iter().filter(|(p, h)| p != h).count();
    out[9].sys = sp.iter().filter(|(p, h)| p != h).count();
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    let mut gk: BTreeMap<(String, usize), String> = BTreeMap::new();
    for (p, h) in &gp {
        let k = seen.entry(p.as_str()).or_default();
        gk.insert((p.clone(), *k), h.clone());
        *k += 1;
    }
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for (p, h) in &sp {
        let k = seen.entry(p.as_str()).or_default();
        if p != h && gk.get(&(p.clone(), *k)).is_some_and(|g| g == h) {
            out[9].tp += 1;
        }
        *k += 1;
    }
    out
}

/// The F1 of all roles of one set of counts.
pub fn overall(c: &[Prf; 10]) -> Prf {
    let mut o = Prf::default();
    for x in c {
        o.add(*x);
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notation_and_negative_control() {
        let g: Vec<GFrame> = ["Event duchess fling patient=baby at=alice | next→#1", "Event duchess rush off | cause→#2", "Event duchess get attrs=ready", "Event they kill patient=baby(«it») place=house"].iter().filter_map(|x| parse_frame(x)).collect();
        assert_eq!(g.len(), 4);
        assert_eq!(g[1].verb, "rush off");
        assert_eq!(g[3].patient.as_ref().unwrap().pron.as_deref(), Some("it"));
        let k = parse_frame("Fact(is a) lady be patient=character | quoted").unwrap();
        assert_eq!((k.kind.as_str(), k.verb.as_str(), k.quoted), ("Fact(is a)", "be", true));
        // the gold against itself is perfect; against nothing or a wrong pronoun it is not (a gate that can show red)
        assert_eq!(overall(&score_line(&g, &g)).f1(), 1.0);
        assert_eq!(overall(&score_line(&g, &[])).f1(), 0.0);
        let mut bad = g.clone();
        bad[3].patient = Some(GEnt { ids: vec!["house".into()], pron: Some("it".into()) });
        bad[0].next = None;
        let c = score_line(&g, &bad);
        assert_eq!((c[9].tp, c[9].gold, c[8].tp), (0, 1, 0));
        assert!(overall(&c).f1() < 1.0);
    }
}

#[cfg(test)]
mod wave16_models {
    use super::*;
    fn f(s: &str) -> GFrame { parse_frame(s).unwrap() }
    #[test]
    fn canonical_parser_fields_and_loss() {
        let x = f("#12 Say(ask) Alice+Bob NOT speak up patient=Cat(«IT») recipient=March Hare place=Home with=Tea attrs=Red,LOUD | says->#1,#x,#2 cause→#2 next→#9 quoted if");
        assert_eq!(x.kind, "Say(ask)");
        assert_eq!(x.agent.unwrap().ids, ["alice", "bob"]);
        assert_eq!(x.verb, "speak up");
        assert!(x.neg && x.quoted && x.cond);
        assert_eq!(x.patient.unwrap().pron.as_deref(), Some("it"));
        assert_eq!(x.to.unwrap().head(), "hare");
        assert_eq!(x.attrs, ["red", "loud"]);
        assert_eq!(x.says, [1,2]);
        assert_eq!((x.cause,x.next), (Some(2),Some(9)));
        assert_eq!(x.obls[0].0, "with");
        assert!(parse_frame("nonsense").is_none());
        assert!(parse_frame("").is_none());
        assert_eq!(norm_id("'March_Hare'"), "hare");
    }
    #[test]
    fn alignment_ties_and_threshold() {
        let a = f("Event cat run patient=mouse");
        assert_eq!(align(&[a.clone(),a.clone()], &[a.clone(),a]).into_iter().collect::<Vec<_>>(), [(0,0),(1,1)]);
        let g = f("Event cat eat patient=mouse");
        let s = f("Bare cat chase patient=mouse");
        assert_eq!(align(&[g], &[s]).get(&0), Some(&0));
        assert!(align(&[f("Event cat eat")], &[f("Event cat sleep")]).is_empty());
    }
    #[test]
    fn independent_role_counts_and_mapped_links() {
        let g = vec![f("Say(ask) alice say patient=cat(«it») to=bob place=home | says→#1 cause→#1 next→#1"), f("Event bob run")];
        let s = vec![f("Event bob run"), f("Say(ask) alice say patient=dog(«it») to=bob place=away | says→#0 cause→#0 next→#0")];
        let c = score_line(&g,&s);
        assert_eq!(c.map(|x|(x.tp,x.sys,x.gold)), [(2,2,2),(2,2,2),(0,1,1),(1,1,1),(0,1,1),(1,1,1),(1,1,1),(1,1,1),(1,1,1),(0,1,1)]);
        assert_eq!(overall(&c),Prf{tp:9,sys:12,gold:12});
        assert_eq!(overall(&c).f1(),0.75);
    }
    #[test]
    fn duplicate_links_are_occurrence_bounded() {
        let g = vec![f("Event cat run | says→#1"),f("Event dog sleep")];
        let mut s=g.clone(); s[0].says=vec![1,1,1,99];
        assert_eq!(score_line(&g,&s)[6],Prf{tp:1,sys:3,gold:1});
        assert_eq!(score_line(&s,&g)[6],Prf{tp:1,sys:1,gold:3});
    }
    #[test]
    fn finite_canonical_count_oracle() {
        for agent in ["cat","dog"] { for verb in ["run","sleep"] { for patient in ["_","mouse"] {
            let g=vec![f("Event cat run patient=mouse")];
            let s=vec![f(&format!("Event {agent} {verb} patient={patient}"))];
            let eligible=verb=="run" || (agent=="cat" && patient=="mouse");
            let c=score_line(&g,&s);
            assert_eq!(c[0],Prf{tp:usize::from(eligible && agent=="cat"),sys:1,gold:1});
            assert_eq!(c[1],Prf{tp:usize::from(verb=="run"),sys:1,gold:1});
            assert_eq!(c[2],Prf{tp:usize::from(eligible && patient=="mouse"),sys:usize::from(patient!="_"),gold:1});
            for r in 3..10 {assert_eq!(c[r],Prf::default());}
        }}}
    }
    #[test]
    fn coreference_occurrence_order_and_zero_metrics() {
        let g=vec![f("Event cat(«it») see patient=dog(«it») with=bird(«she»)")];
        let s=vec![f("Event dog(«it») see patient=cat(«it») with=bird(«she»)")];
        assert_eq!(pronouns(&g),vec![("it".into(),"cat".into()),("it".into(),"dog".into()),("she".into(),"bird".into())]);
        assert_eq!(score_line(&g,&s)[9],Prf{tp:1,sys:3,gold:3});
        assert_eq!((Prf::default().p(),Prf::default().r(),Prf::default().f1()),(0.0,0.0,0.0));
        assert_eq!(overall(&score_line(&[],&[])),Prf::default());
    }
}
