//! Queries to the world model with proof: word → sense (with a hook for context), IsA, properties on
//! layers, parts, frames and roles, verb entailment and causation. An unknown word gets "unknown", not a guess.
//!
//! **Word sense choice (v1).** There is no sentence context yet, so the query itself gives the context:
//! - `isa X Y`: we try the senses of X in the lexicographer file of Y's main sense ("whale" with "fish" —
//!   the animal) and the first sense of X; Y — any of its senses;
//! - property: the property's selectional preference (`Prop::prefers`: body abilities — animal, edibility —
//!   food), otherwise the first sense;
//! - bridge to the story world: the object's type (`type=animal` → `noun.animal`).
//!
//! Every answer says which sense was taken and why.

use crate::kb::*;
use crate::model::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verdict {
    Yes,
    No,
    Unknown,
    Undetermined,
}

impl Verdict {
    pub fn uk(self) -> &'static str {
        match self {
            Verdict::Yes => "yes",
            Verdict::No => "no",
            Verdict::Unknown => "unknown",
            Verdict::Undetermined => "undetermined",
        }
    }
    pub fn code(self) -> &'static str {
        match self {
            Verdict::Yes => "yes",
            Verdict::No => "no",
            Verdict::Unknown => "unknown",
            Verdict::Undetermined => "undetermined",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Answer {
    pub verdict: Verdict,
    /// values of the answer when it is not yes/no: the word's frames
    pub values: Vec<String>,
    pub lines: Vec<String>,
}

impl Answer {
    fn new(verdict: Verdict, lines: Vec<String>) -> Answer {
        Answer { verdict, values: Vec::new(), lines }
    }
    fn unknown(msg: String) -> Answer {
        Answer::new(Verdict::Unknown, vec![msg])
    }
    pub fn text(&self) -> String {
        let mut s = self.verdict.uk().to_string();
        if !self.values.is_empty() {
            s.push_str(": ");
            s.push_str(&self.values.join(", "));
        }
        for l in &self.lines {
            s.push_str("\n  ");
            s.push_str(l);
        }
        s
    }
}

/// A found word: in which form, and its senses in frequency order.
pub struct Found {
    pub form: String,
    pub senses: Vec<Cid>,
    pub how: &'static str,
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// Word → senses of the needed part of speech. Forms: as is (underscores are spaces), lower case, upper
/// case; for nouns — the singular of a simple plural. Senses of different forms are merged, the exact form first.
pub fn lookup(g: &Graph, w: &str, pos: Pos) -> Option<Found> {
    // explicit sense: a synset id (`oewn-01913388-n`) — a hook for a context that has already chosen the sense
    if let Some(c) = g.find_synset(w) {
        return g.pos[c as usize].same_word_class(pos).then(|| Found { form: w.to_string(), senses: vec![c], how: " (synset id)" });
    }
    let base = w.replace('_', " ");
    let lower = base.to_lowercase();
    let mut forms = vec![base.clone(), lower.clone(), capitalize(&lower)];
    forms.dedup();
    let collect = |forms: &[String]| -> (Vec<Cid>, String) {
        let mut v: Vec<Cid> = Vec::new();
        let mut form = String::new();
        for f in forms {
            if let Some(wid) = g.word_id(f) {
                for &c in g.senses_of(wid) {
                    if g.pos[c as usize].same_word_class(pos) && !v.contains(&c) {
                        if form.is_empty() {
                            form = f.clone();
                        }
                        v.push(c);
                    }
                }
            }
        }
        (v, form)
    };
    let (v, form) = collect(&forms);
    if !v.is_empty() {
        return Some(Found { form, senses: v, how: "" });
    }
    if pos == Pos::Noun {
        let mut sing = Vec::new();
        if let Some(s) = lower.strip_suffix("ies") {
            sing.push(format!("{s}y"));
        }
        if let Some(s) = lower.strip_suffix("ves") {
            sing.push(format!("{s}f"));
            sing.push(format!("{s}fe"));
        }
        if let Some(s) = lower.strip_suffix("es") {
            sing.push(s.to_string());
        }
        if let Some(s) = lower.strip_suffix('s') {
            sing.push(s.to_string());
        }
        for s in sing {
            let (v, form) = collect(&[s]);
            if !v.is_empty() {
                return Some(Found { form, senses: v, how: " (from plural)" });
            }
        }
    }
    None
}

fn ordinal(i: usize) -> String {
    format!("{}{}", i + 1, match (i + 1) % 100 { 11..=13 => "th", n => match n % 10 { 1 => "st", 2 => "nd", 3 => "rd", _ => "th" } })
}

/// Description of the chosen sense: `crow → oewn-01581669-n «crow» (1st of 3, noun.animal; …)`.
fn sense_line(g: &Graph, word: &str, f: &Found, c: Cid, why: &str) -> String {
    let i = f.senses.iter().position(|x| *x == c).unwrap_or(0);
    format!("sense: {word}{} → {} ({} of {}, {}; {}) — {}", f.how, g.show(c), ordinal(i), f.senses.len(), g.lexfile_name(c), why, short(g.gloss_of(c), 90))
}

fn short(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        format!("«{s}»")
    } else {
        format!("«{}…»", s.chars().take(n).collect::<String>())
    }
}

/// The chain in human language: `«crow» —IsA→ «corvine bird» —IsA→ … «bird»`.
pub fn chain_str(kb: &Kb, ch: &Chain) -> String {
    let g = &kb.g;
    let mut s = g.show(ch.start);
    for st in &ch.steps {
        let via = match st.extra {
            Some(i) => format!(" —{}[seed {}]→ ", st.rel.name(), kb.extra[i as usize].id),
            None => format!(" —{}→ ", st.rel.name()),
        };
        s.push_str(&via);
        s.push_str(&g.show(st.to));
    }
    s
}

/// Senses of X for a query with a lexfile context: all senses in that file (in frequency order) and
/// the very first one. The blind round (v1, 26.09) took only the first sense in the file — "oak" then became wood
/// (OEWN puts it in `noun.plant`) rather than the tree; the second round takes any sense in the category's domain.
fn candidates(g: &Graph, f: &Found, lexfile: Option<u8>) -> Vec<(Cid, &'static str)> {
    let dom = f.senses[0];
    let mut v = Vec::new();
    if let Some(lf) = lexfile {
        for &c in f.senses.iter().filter(|&&c| g.lexfile[c as usize] == lf) {
            v.push((c, "by context"));
        }
    }
    if !v.iter().any(|(c, _)| *c == dom) {
        v.push((dom, "first sense"));
    }
    // v3: the core first — a core concept sense is a candidate too
    for &c in &f.senses {
        if crate::core::of_concept(g, c).is_some() && !v.iter().any(|(x, _)| *x == c) {
            v.push((c, "core concept"));
        }
    }
    rank_candidates(g, v, lexfile)
}

/// v3: candidates by a sentence-free WSD score — core (0.5) + preferred sense from the corpus (1.5 × share) + the query's context
/// lexfile (1); the best and those close to it (≤ 0.3) remain: a clearly weaker sense (dolphinfish next to
/// the core dolphin-mammal) does not turn a trap question into "yes".
pub fn rank_candidates(g: &Graph, v: Vec<(Cid, &'static str)>, lexfile: Option<u8>) -> Vec<(Cid, &'static str)> {
    if v.len() < 2 {
        return v;
    }
    let prior = crate::wsd::global_prior();
    let score = |c: Cid| -> f64 {
        let core = if crate::core::of_concept(g, c).is_some() { crate::wsd::W_CORE } else { 0.0 };
        let pr = crate::wsd::W_PRIOR * prior.share.get(&g.cid_str(c)).copied().unwrap_or(0.0);
        let lf = if lexfile == Some(g.lexfile[c as usize]) { crate::wsd::W_DOMAIN } else { 0.0 };
        core + pr + lf
    };
    let mut sc: Vec<(f64, usize, Cid, &'static str)> = v.iter().enumerate().map(|(i, (c, why))| (score(*c), i, *c, *why)).collect();
    sc.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let best = sc[0].0;
    sc.into_iter().filter(|x| x.0 >= best - 0.3).map(|x| (x.2, if x.3 == "core concept" || (x.0 == best && x.1 > 0) { "WSD score: core, corpus, lexfile" } else { x.3 })).collect()
}

fn unknown_word(w: &str) -> Answer {
    Answer::unknown(format!("the word «{w}» is not in the world model (OEWN 2025+) — not guessing"))
}

/// `isa X Y`: is X a kind or instance of Y?
pub fn isa(kb: &Kb, x: &str, y: &str) -> Answer {
    let g = &kb.g;
    let mut pair = None;
    for pos in [Pos::Noun, Pos::Verb] {
        if let (Some(fx), Some(fy)) = (lookup(g, x, pos), lookup(g, y, pos)) {
            pair = Some((fx, fy));
            break;
        }
    }
    let Some((fx, fy)) = pair else {
        return if lookup(g, x, Pos::Noun).or_else(|| lookup(g, x, Pos::Verb)).is_none() { unknown_word(x) } else { unknown_word(y) };
    };
    let y0 = fy.senses[0];
    let cands = candidates(g, &fx, Some(g.lexfile[y0 as usize]));
    for &(xc, why) in &cands {
        let anc = kb.ancestors(xc);
        for &yc in &fy.senses {
            if let Some(ch) = anc.chain(yc) {
                let why = if why == "by context" { format!("file of the main sense of «{y}»") } else { why.to_string() };
                return Answer::new(
                    Verdict::Yes,
                    vec![
                        sense_line(g, x, &fx, xc, &why),
                        format!("{y} → {} (sense {} of {})", g.show(yc), fy.senses.iter().position(|c| *c == yc).unwrap_or(0) + 1, fy.senses.len()),
                        format!("chain: {}", chain_str(kb, &ch)),
                    ],
                );
            }
        }
    }
    // no: where the chosen sense of X and the main sense of Y diverge
    let xc = cands[0].0;
    let (ax, ay) = (kb.ancestors(xc), kb.ancestors(y0));
    let lca = ax.map.keys().filter(|c| ay.contains(**c)).min_by_key(|c| (ax.depth(**c).unwrap_or(0) + ay.depth(**c).unwrap_or(0), **c)).copied();
    let mut lines = vec![sense_line(g, x, &fx, xc, if cands[0].1 == "by context" { "file of the main sense of Y" } else { cands[0].1 })];
    lines.push(format!("{y} → {} (main of {}); no sense of «{y}» is an ancestor of the chosen senses of «{x}»", g.show(y0), fy.senses.len()));
    match lca {
        Some(l) => {
            lines.push(format!("{x}: {}", chain_str(kb, &ax.chain(l).expect("ancestor"))));
            lines.push(format!("{y}: {}", chain_str(kb, &ay.chain(l).expect("ancestor"))));
            lines.push(format!("diverge under {}", g.show(l)));
        }
        None => lines.push("no common ancestor".into()),
    }
    Answer::new(Verdict::No, lines)
}

/// A word's sense for a property: the property's preference, otherwise the first one.
pub fn prop_sense(g: &Graph, f: &Found, p: Prop) -> (Cid, String) {
    if let Some(lf) = p.prefers().and_then(lexfile_id) {
        // v3: in the preference lexfile — core concepts first (hedgehog — the hedgehog, not a porcupine)
        if let Some(&c) = f.senses.iter().find(|&&c| g.lexfile[c as usize] == lf && crate::core::of_concept(g, c).is_some()) {
            let why = if c == f.senses[0] { "first sense".to_string() } else { format!("{} — preference of property {}, core concept", LEXFILES[lf as usize], p.name()) };
            return (c, why);
        }
        if let Some(&c) = f.senses.iter().find(|&&c| g.lexfile[c as usize] == lf) {
            let why = if c == f.senses[0] { "first sense".to_string() } else { format!("{} — preference of property {}", LEXFILES[lf as usize], p.name()) };
            return (c, why);
        }
    }
    (f.senses[0], "first sense".into())
}

fn assert_str(kb: &Kb, i: usize) -> String {
    let a = &kb.asserts[i];
    let l = &kb.layers[a.layer as usize];
    format!(
        "seed {} [{} — {}, {}]: {} {}={} — source: {}",
        a.id,
        l.name,
        l.kind.uk(),
        if a.strength == Strength::Strict { "Strict" } else { "Default" },
        kb.g.show(a.concept),
        a.prop.name(),
        if a.value { "yes" } else { "no" },
        a.source
    )
}

/// A concept's property on a layer — with proof.
pub fn prop_of(kb: &Kb, c: Cid, p: Prop, layer: LayerId, mut lines: Vec<String>) -> Answer {
    let lname = &kb.layers[layer as usize].name;
    match kb.prop(c, p, layer) {
        PropAnswer::Known { value, win, chain, beaten, layer: l } => {
            if !chain.steps.is_empty() {
                lines.push(format!("inherited: {}", chain_str(kb, &chain)));
            }
            lines.push(assert_str(kb, win));
            if l != layer {
                lines.push(format!("query on layer {lname}; the most specific layer with a seed decides — {}", kb.layers[l as usize].name));
            } else if l != 0 {
                lines.push(format!("exception of layer {} (acts only on it and below)", kb.layers[l as usize].name));
            }
            for b in beaten {
                lines.push(format!("overridden (the more specific concept wins): {}", assert_str(kb, b)));
            }
            Answer::new(if value { Verdict::Yes } else { Verdict::No }, lines)
        }
        PropAnswer::Undetermined { cands, layer: l } => {
            lines.push(format!("conflicting inheritance on layer {} — both proofs:", kb.layers[l as usize].name));
            for (i, ch) in cands {
                lines.push(format!("  {} ⇐ {}", assert_str(kb, i), chain_str(kb, &ch)));
            }
            Answer::new(Verdict::Undetermined, lines)
        }
        PropAnswer::Unknown => {
            lines.push(format!("no {} seed applies to the concept or its ancestors on layer {lname} and above", p.name()));
            Answer::new(Verdict::Unknown, lines)
        }
    }
}

/// `prop X P [--layer L]`
pub fn prop(kb: &Kb, x: &str, p: Prop, layer: LayerId) -> Answer {
    let g = &kb.g;
    let Some(f) = lookup(g, x, Pos::Noun) else { return unknown_word(x) };
    let (c, why) = prop_sense(g, &f, p);
    prop_of(kb, c, p, layer, vec![sense_line(g, x, &f, c, &why)])
}

/// Parts (PartOf), members (MemberOf) and substances (MadeOf) of a concept together with those inherited from ancestors;
/// parts — transitively (a part of a part), up to depth 3. Returns (part, relation, chain to the
/// owning ancestor, path of parts).
pub fn parts_of(kb: &Kb, c: Cid) -> Vec<(Cid, Rel, Chain, Vec<Cid>)> {
    let g = &kb.g;
    let anc = kb.ancestors(c);
    let mut owners: Vec<(Cid, u16)> = anc.map.iter().map(|(k, v)| (*k, v.0)).collect();
    owners.sort_by_key(|(k, d)| (*d, *k));
    let mut out: Vec<(Cid, Rel, Chain, Vec<Cid>)> = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for (o, _) in owners {
        let chain = anc.chain(o).expect("ancestor");
        // parts and members — incoming PartOf/MemberOf edges; substance — outgoing MadeOf
        let mut frontier: Vec<(Cid, Vec<Cid>)> = vec![(o, vec![])];
        for depth in 0..3 {
            let mut next = Vec::new();
            for (w, path) in &frontier {
                for e in g.in_edges(*w) {
                    if e.rel == Rel::PartOf || (depth == 0 && e.rel == Rel::MemberOf) {
                        if seen.insert((e.to, e.rel)) {
                            let mut p = path.clone();
                            p.push(e.to);
                            out.push((e.to, e.rel, chain.clone(), p.clone()));
                            if e.rel == Rel::PartOf {
                                next.push((e.to, p));
                            }
                        }
                    }
                }
                if depth == 0 {
                    for e in g.out_edges(*w).iter().filter(|e| e.rel == Rel::MadeOf) {
                        if seen.insert((e.to, e.rel)) {
                            out.push((e.to, Rel::MadeOf, chain.clone(), vec![e.to]));
                        }
                    }
                }
            }
            frontier = next;
        }
    }
    out
}

/// `haspart X Y` (rel = PartOf), `memberof Y X` (rel = MemberOf: X is the group), `madeof X Y` (MadeOf).
pub fn has_part(kb: &Kb, whole: &str, part: &str, rel: Rel) -> Answer {
    let g = &kb.g;
    let Some(fw) = lookup(g, whole, Pos::Noun) else { return unknown_word(whole) };
    let Some(fp) = lookup(g, part, Pos::Noun) else { return unknown_word(part) };
    let cands = candidates(g, &fw, Some(g.lexfile[fp.senses[0] as usize]));
    for &(wc, why) in &cands {
        for (p, r, owner, path) in parts_of(kb, wc) {
            if r != rel {
                continue;
            }
            let pa = kb.ancestors(p);
            for &yc in &fp.senses {
                if let Some(ch) = pa.chain(yc) {
                    let mut lines = vec![sense_line(g, whole, &fw, wc, why)];
                    if !owner.steps.is_empty() {
                        lines.push(format!("inherited from an ancestor: {}", chain_str(kb, &owner)));
                    }
                    let via: Vec<String> = path.iter().map(|c| g.show(*c)).collect();
                    lines.push(format!("{} {}: {}", g.show(owner.end()), match rel {
                        Rel::PartOf => "has part",
                        Rel::MemberOf => "has member",
                        _ => "made of",
                    }, via.join(" → has part → ")));
                    if !ch.steps.is_empty() {
                        lines.push(format!("{part}: {}", chain_str(kb, &ch)));
                    }
                    return Answer::new(Verdict::Yes, lines);
                }
            }
        }
    }
    Answer::new(
        Verdict::Unknown,
        vec![
            sense_line(g, whole, &fw, cands[0].0, cands[0].1),
            format!("«{part}» not found among {} (including inherited); OEWN meronymy is incomplete — not saying \"no\"", match rel {
                Rel::PartOf => "parts",
                Rel::MemberOf => "members",
                _ => "substances",
            }),
        ],
    )
}

/// `parts X` — a list.
pub fn parts_list(kb: &Kb, x: &str) -> Answer {
    let g = &kb.g;
    let Some(f) = lookup(g, x, Pos::Noun) else { return unknown_word(x) };
    let c = f.senses[0];
    let all = parts_of(kb, c);
    let deep = all.iter().filter(|x| x.3.len() > 1).count();
    // the list shows direct parts of the concept and its ancestors; parts of parts are only counted (`haspart` has them)
    let ps: Vec<_> = all.into_iter().filter(|x| x.3.len() == 1).collect();
    let mut lines = vec![sense_line(g, x, &f, c, "first sense")];
    if ps.is_empty() {
        lines.push("no parts, members or substances in OEWN".into());
        return Answer::new(Verdict::Unknown, lines);
    }
    lines.push(format!("{} direct (including inherited from ancestors), {deep} more — parts of parts", ps.len()));
    for (p, r, owner, path) in &ps {
        let from = if owner.steps.is_empty() { String::new() } else { format!(" (from {})", g.show(owner.end())) };
        let depth = if path.len() > 1 { format!(" [part of a part: {}]", path.iter().map(|c| g.label(*c)).collect::<Vec<_>>().join(" → ")) } else { String::new() };
        lines.push(format!("{} {}{from}{depth}", r.name(), g.show(*p)));
    }
    let mut a = Answer::new(Verdict::Yes, lines);
    a.values = ps.iter().filter(|x| x.3.len() == 1).map(|(p, _, _, _)| g.label(*p)).collect();
    a
}

/// Frames of a verb (or of another part of speech if there is no verb) — in order of the number of annotated sentences.
pub fn frames_of(g: &Graph, w: &str) -> Vec<(Cid, Pos, u32)> {
    let lower = w.replace('_', " ").to_lowercase();
    let Some(wid) = g.word_id(&lower) else { return vec![] };
    let lus = g.lus_of(wid);
    let verbs: Vec<_> = lus.iter().filter(|l| l.pos == Pos::Verb).map(|l| (l.frame, l.pos, l.id)).collect();
    if !verbs.is_empty() {
        return verbs;
    }
    lus.iter().map(|l| (l.frame, l.pos, l.id)).collect()
}

fn roles(g: &Graph, f: Cid) -> Vec<(Cid, Core)> {
    g.out_edges(f).iter().filter(|e| e.rel == Rel::HasRole).map(|e| (e.to, g.core_of(e.to).unwrap_or(Core::Peripheral))).collect()
}

fn role_short(g: &Graph, r: Cid) -> String {
    g.name_of(r).rsplit_once('.').map(|x| x.1.to_string()).unwrap_or_default()
}

/// `frame V`
pub fn frame(kb: &Kb, w: &str) -> Answer {
    let g = &kb.g;
    let fs = frames_of(g, w);
    if fs.is_empty() {
        return Answer::unknown(format!("«{w}» is not among the FrameNet 1.7 lexical units — not guessing"));
    }
    let mut lines = Vec::new();
    for (f, pos, id) in &fs {
        let rs = roles(g, *f);
        let core: Vec<String> = rs.iter().filter(|r| r.1 == Core::Core).map(|r| role_short(g, r.0)).collect();
        let peri: Vec<String> = rs.iter().filter(|r| r.1 != Core::Core).map(|r| role_short(g, r.0)).collect();
        lines.push(format!("{} ← lexical unit {w}.{} (LU {id}); Core roles: {}; others: {}", g.cid_str(*f), pos.letter(), core.join(", "), peri.join(", ")));
        let up: Vec<String> = g.out_edges(*f).iter().filter(|e| e.rel != Rel::HasRole).map(|e| format!("{} {}", e.rel.name(), g.name_of(e.to))).collect();
        if !up.is_empty() {
            lines.push(format!("  {}", up.join("; ")));
        }
    }
    let mut a = Answer::new(Verdict::Yes, lines);
    a.values = fs.iter().map(|(f, _, _)| g.name_of(*f).to_string()).collect();
    a
}

/// `role V R`: whether some frame of the verb has role R.
pub fn role(kb: &Kb, w: &str, r: &str) -> Answer {
    let g = &kb.g;
    let fs = frames_of(g, w);
    if fs.is_empty() {
        return Answer::unknown(format!("«{w}» is not among the FrameNet 1.7 lexical units — not guessing"));
    }
    for (f, _, id) in &fs {
        for (rc, core) in roles(g, *f) {
            if role_short(g, rc).eq_ignore_ascii_case(r) {
                return Answer::new(Verdict::Yes, vec![format!("{w} → {} (LU {id}) —HasRole→ {} [{}] — {}", g.cid_str(*f), g.cid_str(rc), core.name(), short(g.gloss_of(rc), 90))]);
            }
        }
    }
    let names: Vec<&str> = fs.iter().map(|(f, _, _)| g.name_of(*f)).collect();
    Answer::new(Verdict::No, vec![format!("no frame of «{w}» ({}) has role {r}", names.join(", "))])
}

fn frame_by_name(g: &Graph, name: &str) -> Option<Cid> {
    (0..g.n_concepts() as Cid).find(|&c| g.src[c as usize] == Src::Frame && g.name_of(c).eq_ignore_ascii_case(name))
}

/// `inherits F G`: F inherits from G (transitively).
pub fn inherits(kb: &Kb, a: &str, b: &str) -> Answer {
    let g = &kb.g;
    let (Some(fa), Some(fb)) = (frame_by_name(g, a), frame_by_name(g, b)) else {
        return Answer::unknown(format!("frame «{}» is not in FrameNet 1.7", if frame_by_name(g, a).is_none() { a } else { b }));
    };
    let mut prev: std::collections::HashMap<Cid, Cid> = Default::default();
    let mut q = std::collections::VecDeque::from([fa]);
    prev.insert(fa, fa);
    while let Some(x) = q.pop_front() {
        if x == fb {
            let mut path = vec![x];
            let mut cur = x;
            while cur != fa {
                cur = prev[&cur];
                path.push(cur);
            }
            path.reverse();
            return Answer::new(Verdict::Yes, vec![path.iter().map(|c| g.cid_str(*c)).collect::<Vec<_>>().join(" —Inherits→ ")]);
        }
        for e in g.out_edges(x).iter().filter(|e| e.rel == Rel::FrameInherits) {
            if !prev.contains_key(&e.to) {
                prev.insert(e.to, x);
                q.push_back(e.to);
            }
        }
    }
    Answer::new(Verdict::No, vec![format!("{} does not inherit from {} (FrameNet Inheritance, transitively)", g.cid_str(fa), g.cid_str(fb))])
}

/// `entails X Y` / `causes X Y`: verb X entails or causes Y (with inheritance from the hypernyms of X and
/// to the hypernyms of Y).
pub fn verb_rel(kb: &Kb, x: &str, y: &str, rel: Rel) -> Answer {
    let g = &kb.g;
    let Some(fx) = lookup(g, x, Pos::Verb) else { return unknown_word(x) };
    let Some(fy) = lookup(g, y, Pos::Verb) else { return unknown_word(y) };
    for &xc in &fx.senses {
        let anc = kb.ancestors(xc);
        let mut owners: Vec<(Cid, u16)> = anc.map.iter().map(|(k, v)| (*k, v.0)).collect();
        owners.sort_by_key(|(k, d)| (*d, *k));
        for (o, _) in owners {
            for e in g.out_edges(o).iter().filter(|e| e.rel == rel) {
                let ta = kb.ancestors(e.to);
                for &yc in &fy.senses {
                    if let Some(ch) = ta.chain(yc) {
                        let mut lines = vec![sense_line(g, x, &fx, xc, "first sense with the relation")];
                        let oc = anc.chain(o).expect("ancestor");
                        if !oc.steps.is_empty() {
                            lines.push(format!("inherited: {}", chain_str(kb, &oc)));
                        }
                        lines.push(format!("{} —{}→ {}", g.show(o), rel.name(), g.show(e.to)));
                        if !ch.steps.is_empty() {
                            lines.push(format!("{y}: {}", chain_str(kb, &ch)));
                        }
                        return Answer::new(Verdict::Yes, lines);
                    }
                }
            }
        }
    }
    Answer::new(Verdict::No, vec![format!("no sense of «{x}» (with hypernyms) has {} to the senses of «{y}»", rel.name())])
}

/// List of a word's senses with glosses.
pub fn word(kb: &Kb, w: &str) -> Answer {
    let g = &kb.g;
    let mut lines = Vec::new();
    for pos in [Pos::Noun, Pos::Verb, Pos::Adj, Pos::Adv] {
        if let Some(f) = lookup(g, w, pos) {
            for (i, c) in f.senses.iter().enumerate() {
                if pos == Pos::Adj && g.pos[*c as usize] == Pos::Sat && i > 0 && f.senses[..i].contains(c) {
                    continue;
                }
                let plus = if g.flags[*c as usize] & FLAG_PLUS != 0 { " [plus]" } else { "" };
                lines.push(format!("{}.{} {} {}{plus} — {}", f.form, i + 1, g.show(*c), g.lexfile_name(*c), short(g.gloss_of(*c), 100)));
            }
        }
    }
    let fs = frames_of(g, w);
    for (f, pos, id) in fs {
        lines.push(format!("FrameNet: {w}.{} → {} (LU {id})", pos.letter(), g.cid_str(f)));
    }
    if lines.is_empty() {
        return unknown_word(w);
    }
    Answer::new(Verdict::Yes, lines)
}

/// `up X` — ancestors: a synset (`oewn-…`) or the first sense of a word; all paths as a BFS tree.
pub fn up(kb: &Kb, x: &str) -> Answer {
    let g = &kb.g;
    let c = match g.find_synset(x) {
        Some(c) => c,
        None => match lookup(g, x, Pos::Noun).or_else(|| lookup(g, x, Pos::Verb)) {
            Some(f) => f.senses[0],
            None => return unknown_word(x),
        },
    };
    let anc = kb.ancestors(c);
    let mut v: Vec<(u16, Cid)> = anc.map.iter().map(|(k, d)| (d.0, *k)).collect();
    v.sort();
    let lines = v.iter().map(|(d, k)| format!("{}{} — {}", "  ".repeat(*d as usize), g.show(*k), short(g.gloss_of(*k), 70))).collect();
    Answer::new(Verdict::Yes, lines)
}

/// Query parsing: `isa X Y`, `can X V`, `prop X P`, `haspart X Y`, `partof X Y`, `memberof X Y`,
/// `madeof X Y`, `parts X`, `frame V`, `role V R`, `inherits F G`, `entails X Y`, `causes X Y`, `word W`;
/// `--layer L` — world layer (default `real`).
pub fn ask(kb: &Kb, q: &str) -> anyhow::Result<Answer> {
    let mut toks: Vec<&str> = q.split_whitespace().collect();
    let mut layer = 0 as LayerId;
    if let Some(i) = toks.iter().position(|t| *t == "--layer") {
        let name = toks.get(i + 1).ok_or_else(|| anyhow::anyhow!("--layer without a name"))?;
        layer = kb.layer_id(name).ok_or_else(|| anyhow::anyhow!("unknown layer «{name}»; available: {}", kb.layers.iter().map(|l| l.name.as_str()).collect::<Vec<_>>().join(" ")))?;
        toks.drain(i..i + 2);
    }
    let arg = |i: usize| -> anyhow::Result<&str> { toks.get(i).copied().ok_or_else(|| anyhow::anyhow!("query «{q}»: missing argument")) };
    Ok(match arg(0)? {
        "isa" => isa(kb, arg(1)?, arg(2)?),
        "can" | "prop" => {
            let p = Prop::parse(arg(2)?).ok_or_else(|| anyhow::anyhow!("unknown property «{}»; available: {}", arg(2).unwrap_or(""), Prop::ALL.map(|p| p.name()).join(" ")))?;
            prop(kb, arg(1)?, p, layer)
        }
        "haspart" => has_part(kb, arg(1)?, arg(2)?, Rel::PartOf),
        "partof" => has_part(kb, arg(2)?, arg(1)?, Rel::PartOf),
        "memberof" => has_part(kb, arg(2)?, arg(1)?, Rel::MemberOf),
        "madeof" => has_part(kb, arg(1)?, arg(2)?, Rel::MadeOf),
        "parts" => parts_list(kb, arg(1)?),
        "frame" => frame(kb, arg(1)?),
        "role" => role(kb, arg(1)?, arg(2)?),
        "inherits" => inherits(kb, arg(1)?, arg(2)?),
        "entails" => verb_rel(kb, arg(1)?, arg(2)?, Rel::Entails),
        "causes" => verb_rel(kb, arg(1)?, arg(2)?, Rel::Causes),
        "word" => word(kb, arg(1)?),
        "up" => up(kb, arg(1)?),
        other => anyhow::bail!("unknown query «{other}»"),
    })
}
