//! The fourth layer, frames ↔ world.
//!
//! The world is the story's state as `prag`'s discourse and `roles::world_docs` keep it: a registry of entities by
//! id (number, whether a name, every modifier ever said of it) and the line of what happened and what holds, one
//! event per action, utterance or general statement, with its roles as entity ids, its attributes and obliques, and
//! the links «what was said» and «because» between events. What the world does not keep: which pronoun named an
//! entity at a mention, which modifier was said at which mention, the number of one mention, whether speech was
//! direct, the explicit «next» links (the line itself is the order), sentence boundaries,
//! and a general statement said again (the state does not change). A phrase with no predicate (a heading, a title,
//! a noun phrase: a bare frame) is kept in the line as a mention of its entities (ROUNDTRIP_OFF=world-bare drops it).
//!
//! Down: one frame per event, entities by name or by the pronoun the referring-expression generator chooses
//! (`refer`), the modifiers at the first mention; a new sentence starts at an
//! event not linked to the events of the current one. The residual: the frames the world dropped, field patches of
//! the others, and the sentence boundaries that differ.

use std::collections::{BTreeMap, BTreeSet};

use prag::frames::Kind;

use crate::class::{Class, Layer, lemma_bits, pos_bits};
use crate::frames::{E, F};
use crate::tally::Residual;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WEnt {
    pub plural: bool,
    pub named: bool,
    pub attrs: BTreeSet<String>,
    /// the personal pronoun the text uses for it most (he, she, it, they): its gender and number as the story knows them
    pub pro: Option<&'static str>,
}

/// A mention of an entity in an event: its id, and what this mention says beside the registry (coordinated heads,
/// possessor, prepositional modifiers, appositions, one-word modifiers).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WRef {
    pub id: String,
    pub and: Vec<String>,
    pub m: E,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WEv {
    pub kind: Kind,
    pub verb: String,
    pub neg: bool,
    pub agent: Option<WRef>,
    pub patient: Option<WRef>,
    pub recipient: Option<WRef>,
    pub place: Option<WRef>,
    pub attrs: Vec<String>,
    pub obls: Vec<(String, WRef)>,
    /// world-line indices
    pub quote: Vec<usize>,
    pub cause: Option<usize>,
    pub cond: bool,
    pub unsaid: Vec<prag::frames::Slot>,
    pub manner: Vec<String>,
    pub mark: Vec<(String, en::gram::Tag)>,
    pub cc: Option<String>,
    /// world-line index of the event above, the clause's relation, the noun it belongs to
    pub link: Option<(usize, en::gram::Rel, Option<String>)>,
    pub words: Vec<(en::gram::Rel, String, en::gram::Tag)>,
    pub nps: Vec<(en::gram::Rel, WRef)>,
    pub tam: prag::frames::Tam,
    /// the event a noun names
    pub nominal: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct World {
    pub ents: BTreeMap<String, WEnt>,
    pub line: Vec<WEv>,
}

/// The residual of a volume at the world layer.
#[derive(Clone, Debug, Default)]
pub struct R4 {
    /// frames the world does not keep, at their place in the volume's frame sequence (links global)
    pub inserts: BTreeMap<usize, F>,
    /// kept frames whose fields differ from the decoded ones (links global)
    pub patches: BTreeMap<usize, F>,
    /// frame positions where a sentence boundary is toggled against the decoder's
    pub toggles: BTreeSet<usize>,
    /// the sentence lengths (frames) where the frames are none: (sentence index, —)
    pub empties: Vec<usize>,
}

fn wref(e: &E) -> WRef {
    WRef { id: e.id.clone(), and: e.and.clone(), m: E { id: String::new(), pron: None, plural: false, named: false, attrs: Vec::new(), and: Vec::new(), ..e.clone() } }
}

fn globalize(f: &F, off: usize) -> F {
    let mut g = f.clone();
    g.quote = f.quote.iter().map(|q| q + off).collect();
    g.cause = f.cause.map(|c| c + off);
    g.next = f.next.map(|c| c + off);
    g.link = f.link.clone().map(|(c, r, w)| (c + off, r, w));
    g
}

fn localize(f: &F, off: usize) -> F {
    let mut g = f.clone();
    g.quote = f.quote.iter().map(|q| q.wrapping_sub(off)).collect();
    g.cause = f.cause.map(|c| c.wrapping_sub(off));
    g.next = f.next.map(|c| c.wrapping_sub(off));
    g.link = f.link.clone().map(|(c, r, w)| (c.wrapping_sub(off), r, w));
    g
}

fn fact_key(f: &F) -> String {
    let id = |e: &Option<E>| e.as_ref().map(|x| x.id.clone()).unwrap_or_default();
    format!("{:?}|{}|{}|{}|{}|{}|{}|{:?}|{:?}", f.kind, f.verb, f.neg, id(&f.agent), id(&f.patient), id(&f.recipient), id(&f.place), f.attrs, f.obls.iter().map(|(p, e)| (p, &e.id)).collect::<Vec<_>>())
}

/// The class of a third-person personal pronoun: he, she, it or they.
fn pro_class(p: &str) -> Option<&'static str> {
    match p.to_lowercase().as_str() {
        "he" | "him" | "his" | "himself" => Some("he"),
        "she" | "her" | "hers" | "herself" => Some("she"),
        "it" | "its" | "itself" => Some("it"),
        "they" | "them" | "their" | "theirs" | "themselves" => Some("they"),
        _ => None,
    }
}

/// The form of a pronoun class in a role: the subject's nominative, the accusative elsewhere.
fn pro_form(c: &str, subject: bool) -> &'static str {
    match (c, subject) {
        ("he", true) => "he",
        ("he", false) => "him",
        ("she", true) => "she",
        ("she", false) => "her",
        ("they", true) => "they",
        ("they", false) => "them",
        _ => "it",
    }
}

/// The referring-expression generator: which mentions of the decoded frames are pronouns. A mention is its
/// entity's pronoun (the registry's class, in the form of its role, capitalized as the first subject of a sentence)
/// when the entity was mentioned in this sentence or the one before, it is no other role's entity of the same frame,
/// has no modifier to say, and the reader's own resolver (`prag::frames::Ctx`, salience by role, halved at every
/// sentence) would take the pronoun back to it. An entity that is a pronoun itself (an unresolved «it») stays one.
pub(crate) fn refer(w: &World, fs: &mut [F]) {
    use prag::frames::{Ctx, Ent};
    let st = starts(fs);
    let mut ctx = Ctx::default();
    let mut last: BTreeMap<String, usize> = BTreeMap::new();
    let mut si = 0usize;
    for (k, f) in fs.iter_mut().enumerate() {
        if k > 0 && st.contains(&k) {
            ctx.new_line();
            si += 1;
        }
        let first = st.contains(&k);
        let ids: Vec<String> = f.ents().iter().map(|e| e.id.clone()).collect();
        let agent = f.agent.as_ref().map(|a| a.id.clone());
        let mut slots: Vec<(&mut E, u8, bool)> = Vec::new();
        if let Some(a) = f.agent.as_mut() {
            slots.push((a, 3, true));
        }
        if let Some(p) = f.patient.as_mut() {
            slots.push((p, 2, false));
        }
        if let Some(r) = f.recipient.as_mut() {
            slots.push((r, 2, false));
        }
        if let Some(p) = f.place.as_mut() {
            slots.push((p, 1, false));
        }
        for (_, e) in f.obls.iter_mut() {
            slots.push((e, 1, false));
        }
        for (e, role, subject) in slots {
            let we = w.ents.get(&e.id);
            let own = pro_class(&e.id).filter(|c| *c == e.id);
            let class = own.or(we.and_then(|x| x.pro));
            if let Some(c) = class.filter(|_| e.and.is_empty()) {
                let form = pro_form(c, subject);
                let use_it = own.is_some()
                    || e.attrs.is_empty()
                        && last.get(&e.id).is_some_and(|&s| s + 1 >= si)
                        && ids.iter().filter(|x| **x == e.id).count() == 1
                        && ctx.resolve_in(form, if subject { None } else { agent.as_deref() }, role).is_some_and(|x| x.id == e.id);
                if use_it {
                    e.pron = Some(if subject && first { en::text::title(form) } else { form.to_string() });
                }
            }
            let ent = Ent { id: e.id.clone(), text: e.id.clone(), pron: class.map(str::to_string), plural: e.plural, named: e.named, attrs: Vec::new(), and: e.and.clone(), ..Default::default() };
            ctx.mention_as(&ent, subject, role);
            last.insert(e.id.clone(), si);
        }
    }
}

/// Decoded frames of the world line (links as world-line indices) — one frame per event, modifiers at the first
/// mention, pronouns by the referring-expression generator (`refer`; `ROUNDTRIP_OFF=refgen`: none).
pub fn decode(w: &World) -> Vec<F> {
    let mut fs = decode_plain(w);
    if !crate::tree::off("refgen") {
        refer(w, &mut fs);
    }
    fs
}

pub(crate) fn decode_plain(w: &World) -> Vec<F> {
    let mut first: BTreeSet<String> = BTreeSet::new();
    let mut ent = |r: &WRef| -> E {
        let we = w.ents.get(&r.id);
        let attrs = if first.insert(r.id.clone()) { we.map(|x| x.attrs.iter().cloned().collect()).unwrap_or_default() } else { Vec::new() };
        E { id: r.id.clone(), pron: None, plural: we.is_some_and(|x| x.plural), named: we.is_some_and(|x| x.named), attrs, and: r.and.clone(), ..r.m.clone() }
    };
    w.line
        .iter()
        .map(|ev| {
            let agent = ev.agent.as_ref().map(&mut ent);
            let patient = ev.patient.as_ref().map(&mut ent);
            let recipient = ev.recipient.as_ref().map(&mut ent);
            let place = ev.place.as_ref().map(&mut ent);
            let obls = ev.obls.iter().map(|(p, r)| (p.clone(), ent(r))).collect();
            let nps = ev.nps.iter().map(|(x, r)| (*x, ent(r))).collect();
            F { unsaid: ev.unsaid.clone(), manner: ev.manner.clone(), mark: ev.mark.clone(), cc: ev.cc.clone(), link: ev.link.clone(), words: ev.words.clone(), nps, kind: ev.kind, verb: ev.verb.clone(), neg: ev.neg, agent, patient, recipient, place, attrs: ev.attrs.clone(), obls, quote: ev.quote.clone(), quoted: false, cause: ev.cause, next: None, cond: ev.cond, tam: ev.tam, nominal: ev.nominal }
        })
        .collect()
}

/// The decoder's sentence starts over the world line: an event not linked to the current sentence's events starts one.
pub fn starts(fs: &[F]) -> BTreeSet<usize> {
    let mut out = BTreeSet::new();
    let mut group: BTreeSet<usize> = BTreeSet::new();
    for (k, f) in fs.iter().enumerate() {
        let up = |x: &F| x.link.as_ref().map(|l| l.0);
        let linked = group.iter().any(|&j| fs[j].quote.contains(&k) || fs[j].cause == Some(k) || f.quote.contains(&j) || f.cause == Some(j) || up(f) == Some(j) || up(&fs[j]) == Some(k));
        if k == 0 || !linked {
            out.insert(k);
            group.clear();
        }
        group.insert(k);
    }
    out
}

/// Bits of a whole frame (an insert): its verb, its roles and links.
fn frame_bits(f: &F) -> f64 {
    lemma_bits(&f.verb) + 4.0 + f.ents().iter().map(|e| lemma_bits(&e.id) + 3.0).sum::<f64>() + f.attrs.iter().map(|a| lemma_bits(a)).sum::<f64>()
}

/// The world of a volume's frames (per sentence, links local), and the residual that gives them back.
pub fn encode(sents: &[Vec<F>], res: &mut Residual) -> (World, R4) {
    encode_by(sents, &vec![0; sents.len()], std::slice::from_mut(res))
}

/// The same, each residual item in the residual of the owner of its sentence (`owner[sentence]` indexes `res`): a
/// mixed volume keeps one residual per language.
pub fn encode_by(sents: &[Vec<F>], owner: &[usize], res: &mut [Residual]) -> (World, R4) {
    let mut fo: Vec<usize> = Vec::new();
    let mut flat: Vec<F> = Vec::new();
    let mut r4 = R4::default();
    let mut orig_starts = BTreeSet::new();
    for (si, fs) in sents.iter().enumerate() {
        if fs.is_empty() {
            r4.empties.push(si);
            continue;
        }
        orig_starts.insert(flat.len());
        let off = flat.len();
        for f in fs {
            flat.push(globalize(f, off));
            fo.push(owner[si]);
        }
    }
    let nf = flat.len();
    for &si in &r4.empties {
        res[owner[si]].add(Layer::World, Class::Sentence, pos_bits(sents.len()), || "a sentence with no frame".into());
    }
    // what the world keeps
    let mut world = World::default();
    let mut kept: Vec<Option<usize>> = vec![None; nf];
    let mut facts: BTreeMap<String, usize> = BTreeMap::new();
    for g in 0..nf {
        let f = &flat[g];
        if f.kind == Kind::Bare && crate::tree::off("world-bare") {
            let s = f.show();
            res[fo[g]].add(Layer::World, Class::Dropped, frame_bits(f) + pos_bits(nf), || format!("no predicate: {s}"));
            r4.inserts.insert(g, f.clone());
            continue;
        }
        if matches!(f.kind, Kind::Fact(_)) {
            let k = fact_key(f);
            if let Some(&prev) = facts.get(&k) {
                let s = f.show();
                res[fo[g]].add(Layer::World, Class::Repetition, pos_bits(nf) + pos_bits(prev), || format!("said again: {s}"));
                r4.inserts.insert(g, f.clone());
                continue;
            }
            facts.insert(k, g);
        }
        kept[g] = Some(world.line.len());
        for e in f.ents() {
            let we = world.ents.entry(e.id.clone()).or_insert(WEnt { plural: e.plural, named: e.named, attrs: BTreeSet::new(), pro: None });
            we.attrs.extend(e.attrs.iter().cloned());
        }
        world.line.push(WEv {
            kind: f.kind,
            verb: f.verb.clone(),
            neg: f.neg,
            agent: f.agent.as_ref().map(wref),
            patient: f.patient.as_ref().map(wref),
            recipient: f.recipient.as_ref().map(wref),
            place: f.place.as_ref().map(wref),
            attrs: f.attrs.clone(),
            obls: f.obls.iter().map(|(p, e)| (p.clone(), wref(e))).collect(),
            quote: Vec::new(),
            cause: None,
            cond: f.cond,
            unsaid: f.unsaid.clone(),
            words: f.words.clone(),
            manner: f.manner.clone(),
            mark: f.mark.clone(),
            cc: f.cc.clone(),
            link: None,
            nps: f.nps.iter().map(|(r, e)| (*r, wref(e))).collect(),
            tam: f.tam,
            nominal: f.nominal,
        });
    }
    // links between kept events
    for g in 0..nf {
        if let Some(k) = kept[g] {
            let f = &flat[g];
            world.line[k].quote = f.quote.iter().filter_map(|&q| kept.get(q).copied().flatten()).collect();
            world.line[k].cause = f.cause.and_then(|c| kept.get(c).copied().flatten());
            world.line[k].link = f.link.clone().and_then(|(c, r, w)| kept.get(c).copied().flatten().map(|x| (x, r, w)));
        }
    }
    // the pronoun class of every entity: the one its pronoun mentions use most (ties: the first met)
    let mut pro_n: BTreeMap<String, BTreeMap<&'static str, (usize, usize)>> = BTreeMap::new();
    let mut order = 0;
    for g in (0..nf).filter(|&g| kept[g].is_some()) {
        for e in flat[g].ents() {
            if let Some(c) = e.pron.as_deref().and_then(pro_class) {
                order += 1;
                let x = pro_n.entry(e.id.clone()).or_default().entry(c).or_insert((0, order));
                x.0 += 1;
            }
        }
    }
    for (id, m) in pro_n {
        if let Some(we) = world.ents.get_mut(&id) {
            we.pro = m.into_iter().max_by(|a, b| a.1.0.cmp(&b.1.0).then(b.1.1.cmp(&a.1.1))).map(|x| x.0);
        }
    }
    // decode and compare
    let mut pron_kinds: BTreeMap<(usize, String), u64> = BTreeMap::new();
    let dec = decode(&world);
    let g_of: Vec<usize> = (0..nf).filter(|&g| kept[g].is_some()).collect();
    for (k, d) in dec.iter().enumerate() {
        let g = g_of[k];
        let mut d = d.clone();
        d.quote = d.quote.iter().map(|&q| g_of[q]).collect();
        d.cause = d.cause.map(|c| g_of[c]);
        d.link = d.link.map(|(c, r, w)| (g_of[c], r, w));
        let o = &flat[g];
        if *o == d {
            continue;
        }
        let og = fo[g];
        let mut add = |c: Class, bits: f64, what: String| res[og].add(Layer::World, c, bits, || what);
        let slots = [(&o.agent, &d.agent, "agent"), (&o.patient, &d.patient, "patient"), (&o.recipient, &d.recipient, "to"), (&o.place, &d.place, "place")];
        let mut pairs: Vec<(Option<&E>, Option<&E>, String)> = slots.iter().map(|(a, b, n)| (a.as_ref(), b.as_ref(), n.to_string())).collect();
        pairs.extend(o.obls.iter().zip(d.obls.iter()).map(|((p, a), (_, b))| (Some(a), Some(b), p.clone())));
        for (a, b, n) in pairs {
            let (Some(a), Some(b)) = (a, b) else { continue };
            if a.pron != b.pron {
                add(Class::PronName, a.pron.as_deref().map_or(1.0, |p| 1.0 + 8.0 * p.len() as f64), match (&a.pron, &b.pron) {
                    (None, Some(p)) => format!("{n} {}: the name, where the generator would say «{p}» (1 bit: a flag)", a.id),
                    (Some(p), _) => format!("{n} {}: the pronoun «{p}» ({} bits: a flag and its bytes)", a.id, 1 + 8 * p.len()),
                    (None, None) => format!("{n} {}", a.id),
                });
                *pron_kinds.entry((og, a.pron.clone().unwrap_or_default().to_lowercase())).or_default() += 1;
            }
            if a.plural != b.plural {
                add(Class::Quantity, 1.0, format!("{n} {}: plural {} at this mention, {} in the registry", a.id, a.plural, b.plural));
            }
            if a.named != b.named {
                add(Class::Function, 1.0, format!("{n} {}: name {} at this mention", a.id, a.named));
            }
            if a.attrs != b.attrs {
                add(Class::Dropped, a.attrs.iter().map(|x| lemma_bits(x)).sum::<f64>() + 1.0, format!("{n} {}: modifiers here {:?}, decoded {:?}", a.id, a.attrs, b.attrs));
            }
            if a.id != b.id || a.and != b.and {
                add(Class::Lexical, lemma_bits(&a.id), format!("{n}: {} vs {}", a.id, b.id));
            }
        }
        if o.quote != d.quote || o.cause != d.cause || o.next != d.next || o.link != d.link {
            add(Class::Links, pos_bits(nf), format!("{}: links said {:?} cause {:?} next {:?}", o.verb, o.quote, o.cause, o.next));
        }
        if o.quoted != d.quoted {
            add(Class::Quote, 1.0, format!("{}: inside quotation marks", o.verb));
        }
        if o.kind != d.kind || o.verb != d.verb || o.neg != d.neg || o.attrs != d.attrs || o.cond != d.cond || o.tam != d.tam || o.obls.len() != d.obls.len() || o.obls.iter().zip(&d.obls).any(|(a, b)| a.0 != b.0) {
            add(Class::Lexical, frame_bits(o), format!("{}: frame fields", o.verb));
        }
        r4.patches.insert(g, o.clone());
    }
    for ((o, k), v) in pron_kinds {
        if let Some(cell) = res[o].cells.get_mut(&(Layer::World, Class::PronName)) {
            *cell.kinds.entry(k).or_default() += v;
        }
    }
    // sentence boundaries over the frame sequence
    let dec_starts: BTreeSet<usize> = starts(&dec).into_iter().map(|k| g_of[k]).collect();
    let dec_starts = with_inserts(&dec_starts, &r4.inserts, nf);
    for &g in orig_starts.symmetric_difference(&dec_starts) {
        let s = flat[g].show();
        res[fo[g]].add(Layer::World, Class::Sentence, pos_bits(nf), || format!("boundary before «{s}» {}", if orig_starts.contains(&g) { "missing" } else { "extra" }));
        r4.toggles.insert(g);
    }
    (world, r4)
}

/// Sentence starts with the dropped frames put back: a frame the world dropped joins the sentence before it (or
/// starts the first).
fn with_inserts(starts: &BTreeSet<usize>, inserts: &BTreeMap<usize, F>, nf: usize) -> BTreeSet<usize> {
    let mut out = starts.clone();
    if nf > 0 && !out.contains(&0) && inserts.contains_key(&0) {
        // the first frame was dropped: the first sentence starts at it, the next kept frame continues it
        if let Some(&k) = (1..nf).find(|g| !inserts.contains_key(g)).as_ref() {
            out.remove(&k);
        }
        out.insert(0);
    }
    out
}

/// Exact decode: the world and its residual → the frames per sentence (links local), as read.
pub fn decode_exact(w: &World, r: &R4, nsents: usize) -> Vec<Vec<F>> {
    let dec = decode(w);
    let nf = dec.len() + r.inserts.len();
    let g_of: Vec<usize> = (0..nf).filter(|g| !r.inserts.contains_key(g)).collect();
    let mut flat: Vec<F> = vec![F::default(); nf];
    for (k, d) in dec.iter().enumerate() {
        let g = g_of[k];
        let mut d = d.clone();
        d.quote = d.quote.iter().map(|&q| g_of[q]).collect();
        d.cause = d.cause.map(|c| g_of[c]);
        d.link = d.link.map(|(c, r, w)| (g_of[c], r, w));
        flat[g] = r.patches.get(&g).cloned().unwrap_or(d);
    }
    for (g, f) in &r.inserts {
        flat[*g] = f.clone();
    }
    let dec_starts: BTreeSet<usize> = starts(&dec).into_iter().map(|k| g_of[k]).collect();
    let dec_starts = with_inserts(&dec_starts, &r.inserts, nf);
    let st: BTreeSet<usize> = dec_starts.symmetric_difference(&r.toggles).copied().collect();
    let mut groups: Vec<Vec<F>> = Vec::new();
    let mut off = 0;
    for g in 0..nf {
        if st.contains(&g) || groups.is_empty() {
            groups.push(Vec::new());
            off = g;
        }
        groups.last_mut().unwrap().push(localize(&flat[g], off));
    }
    let mut out: Vec<Vec<F>> = Vec::with_capacity(nsents);
    let mut it = groups.into_iter();
    for si in 0..nsents {
        if r.empties.contains(&si) {
            out.push(Vec::new());
        } else {
            out.push(it.next().unwrap_or_default());
        }
    }
    out
}

/// Normal decode: the world's frames placed in the original sentences (boundaries given), links to frames the world
/// dropped removed.
pub fn decode_in(w: &World, sents: &[Vec<F>]) -> Vec<Vec<F>> {
    // the same keep rule as `encode`, to know which original frame each event is
    let mut facts: BTreeSet<String> = BTreeSet::new();
    let mut place: Vec<(usize, usize)> = Vec::new();
    let mut local_of: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut g = 0;
    for (si, fs) in sents.iter().enumerate() {
        for (li, f) in fs.iter().enumerate() {
            let keep = (f.kind != Kind::Bare || !crate::tree::off("world-bare")) && (!matches!(f.kind, Kind::Fact(_)) || facts.insert(fact_key(f)));
            if keep {
                place.push((si, li));
            }
            local_of.insert(g, (si, li));
            g += 1;
        }
    }
    let dec = decode(w);
    let mut out: Vec<Vec<(usize, F)>> = vec![Vec::new(); sents.len()];
    for (k, d) in dec.into_iter().enumerate() {
        if let Some(&(si, li)) = place.get(k) {
            out[si].push((li, d));
        }
    }
    // world indices → local indices in the sentence (dropped when the target is elsewhere)
    let pos_of: BTreeMap<usize, (usize, usize)> = place.iter().enumerate().map(|(k, p)| (k, *p)).collect();
    out.into_iter()
        .enumerate()
        .map(|(si, v)| {
            let lis: Vec<usize> = v.iter().map(|(li, _)| *li).collect();
            v.into_iter()
                .map(|(_, mut f)| {
                    let tr = |k: usize| pos_of.get(&k).filter(|p| p.0 == si).and_then(|p| lis.iter().position(|&l| l == p.1));
                    f.quote = f.quote.iter().filter_map(|&q| tr(q)).collect();
                    f.cause = f.cause.and_then(tr);
                    f.link = f.link.take().and_then(|(c, r, w)| tr(c).map(|x| (x, r, w)));
                    f
                })
                .collect()
        })
        .collect()
}
