//! Building the graph from sources: Open English WordNet 2025 (WN-LMF XML, `-plus`) and FrameNet 1.7 (zip).
//!
//! Build gates (a violation is a refusal, not a silent graph):
//! - inverse OEWN pairs agree: every `hypernym` S→T has a matching `hyponym` T→S, likewise
//!   `instance_*`, `holo_*`/`mero_*`, `entails`/`is_entailed_by`, `causes`/`is_caused_by`;
//! - the edge target exists;
//! - IsA/InstanceOf without cycles (`infer::isa_cycle`);
//! - the frame referenced by a frame relation exists.

use std::collections::{HashMap, HashSet};
use std::io::BufReader;
use std::path::Path;

use anyhow::{bail, Context, Result};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Reader, XmlVersion};

use crate::model::*;
use crate::zip::Zip;

/// Concept during the build.
#[derive(Default, Clone)]
struct CB {
    src: Option<Src>,
    key: u32,
    pos: Option<Pos>,
    lexfile: u8,
    flags: u8,
    ili: u32,
    gloss: String,
    name: String,
    members: Vec<String>,
}

#[derive(Default)]
pub struct Builder {
    cs: Vec<CB>,
    edges: Vec<(Cid, Rel, Cid)>,
    /// word → senses in frequency order (order of entries and senses in the XML)
    senses: Vec<(String, Vec<Cid>)>,
    /// (word, part of speech, frame, id, annotated sentences)
    lus: Vec<(String, Pos, Cid, u32, u32)>,
    pub report: Vec<String>,
}

fn attr(e: &BytesStart, name: &str) -> Result<Option<String>> {
    for a in e.attributes() {
        let a = a?;
        if a.key.as_ref() == name {
            return Ok(Some(a.normalized_value(XmlVersion::Implicit1_0)?.into_owned()));
        }
    }
    Ok(None)
}

fn entity(name: &str) -> Option<char> {
    Some(match name {
        "lt" => '<',
        "gt" => '>',
        "amp" => '&',
        "apos" => '\'',
        "quot" => '"',
        _ => {
            let n = name.strip_prefix('#')?;
            let v = match n.strip_prefix('x') {
                Some(h) => u32::from_str_radix(h, 16).ok()?,
                None => n.parse().ok()?,
            };
            char::from_u32(v)?
        }
    })
}

/// OEWN synset id `oewn-01505702-n` → (offset, part of speech).
fn synset_key(id: &str) -> Result<(u32, Pos)> {
    let rest = id.strip_prefix("oewn-").with_context(|| format!("synset id without oewn-: {id}"))?;
    let (num, p) = rest.rsplit_once('-').with_context(|| format!("synset id: {id}"))?;
    Ok((num.parse().with_context(|| format!("synset offset: {id}"))?, Pos::from_wn(p).with_context(|| format!("part of speech: {id}"))?))
}

/// Strips markup from FrameNet definition text: `<def-root>…<fen>Buyer</fen>…` → `…Buyer…`.
fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut depth = 0;
    for ch in s.chars() {
        match ch {
            '<' => depth += 1,
            '>' if depth > 0 => depth -= 1,
            _ if depth == 0 => out.push(ch),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn first_sentence(s: &str, max: usize) -> String {
    let cut = s.find(". ").map(|i| i + 1).unwrap_or(s.len());
    let t = &s[..cut];
    if t.chars().count() <= max {
        t.to_string()
    } else {
        t.chars().take(max).collect::<String>() + "…"
    }
}

impl Builder {
    fn add(&mut self, c: CB) -> Cid {
        self.cs.push(c);
        (self.cs.len() - 1) as Cid
    }

    /// Only synset ids of the base OEWN — to mark concepts absent from the base edition.
    pub fn base_ids(path: &Path) -> Result<HashSet<String>> {
        let f = std::fs::File::open(path).with_context(|| format!("cannot open {}", path.display()))?;
        let mut rd = Reader::from_reader(BufReader::new(flate2::read::GzDecoder::new(BufReader::new(f))));
        let mut buf = Vec::new();
        let mut ids = HashSet::new();
        loop {
            match rd.read_event_into(&mut buf)? {
                Event::Start(e) | Event::Empty(e) if e.name().as_ref() == "Synset" => {
                    if let Some(id) = attr(&e, "id")? {
                        ids.insert(id);
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        Ok(ids)
    }

    /// OEWN WN-LMF XML (gz): entries (lemma + senses in frequency order) and synsets with relations.
    pub fn oewn(&mut self, path: &Path, base: Option<&HashSet<String>>) -> Result<()> {
        let f = std::fs::File::open(path).with_context(|| format!("cannot open {}", path.display()))?;
        let mut rd = Reader::from_reader(BufReader::with_capacity(1 << 20, flate2::read::GzDecoder::new(BufReader::new(f))));
        let mut buf = Vec::new();
        // entry: id → lemma; the entry's senses are synset ids
        let mut entry_lemma: HashMap<String, String> = HashMap::new();
        let mut entries: Vec<(String, Vec<String>)> = Vec::new();
        let (mut cur_entry, mut cur_lemma, mut cur_senses) = (String::new(), String::new(), Vec::<String>::new());
        // synsets
        let mut syn_id: HashMap<String, Cid> = HashMap::new();
        let mut rels: Vec<(Cid, String, String)> = Vec::new();
        let mut cur_syn: Option<Cid> = None;
        let mut in_def = false;
        let mut def = String::new();
        loop {
            let ev = rd.read_event_into(&mut buf).with_context(|| format!("OEWN XML, position {}", rd.buffer_position()))?;
            match ev {
                Event::Start(e) | Event::Empty(e) => match e.name().as_ref() {
                    "LexicalEntry" => {
                        cur_entry = attr(&e, "id")?.context("LexicalEntry without id")?;
                        cur_senses.clear();
                    }
                    "Lemma" => {
                        cur_lemma = attr(&e, "writtenForm")?.context("Lemma without writtenForm")?;
                        entry_lemma.insert(cur_entry.clone(), cur_lemma.clone());
                    }
                    "Sense" => cur_senses.push(attr(&e, "synset")?.context("Sense without synset")?),
                    "Synset" => {
                        let id = attr(&e, "id")?.context("Synset without id")?;
                        let (key, pos) = synset_key(&id)?;
                        let lexfile = attr(&e, "lexfile")?.and_then(|l| lexfile_id(&l)).with_context(|| format!("{id}: unknown lexfile"))?;
                        let ili = attr(&e, "ili")?.and_then(|s| s.strip_prefix('i').and_then(|n| n.parse().ok())).unwrap_or(0);
                        let members = attr(&e, "members")?.unwrap_or_default().split_whitespace().map(|m| m.to_string()).collect();
                        let plus = base.map(|b| !b.contains(&id)).unwrap_or(false);
                        let c = self.add(CB {
                            src: Some(Src::Oewn),
                            key,
                            pos: Some(pos),
                            lexfile,
                            flags: if plus { FLAG_PLUS } else { 0 },
                            ili,
                            members,
                            ..Default::default()
                        });
                        if syn_id.insert(id.clone(), c).is_some() {
                            bail!("synset {id} twice");
                        }
                        cur_syn = Some(c);
                        def.clear();
                    }
                    "Definition" => {
                        in_def = true;
                    }
                    "SynsetRelation" => {
                        let s = cur_syn.context("SynsetRelation outside Synset")?;
                        rels.push((s, attr(&e, "relType")?.context("relType")?, attr(&e, "target")?.context("target")?));
                    }
                    _ => {}
                },
                Event::Text(t) if in_def => def.push_str(&t.xml10_content()),
                Event::GeneralRef(r) if in_def => {
                    if let Some(ch) = entity(&r.into_inner()) {
                        def.push(ch);
                    }
                }
                Event::End(e) => match e.name().as_ref() {
                    "LexicalEntry" => entries.push((std::mem::take(&mut cur_lemma), std::mem::take(&mut cur_senses))),
                    "Definition" => {
                        in_def = false;
                        if let Some(c) = cur_syn {
                            let cb = &mut self.cs[c as usize];
                            if cb.gloss.is_empty() {
                                cb.gloss = def.trim().to_string();
                            }
                        }
                        def.clear();
                    }
                    "Synset" => cur_syn = None,
                    _ => {}
                },
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }
        // synset members: entry ids → lemmas
        for c in self.cs.iter_mut().filter(|c| c.src == Some(Src::Oewn)) {
            for m in c.members.iter_mut() {
                *m = entry_lemma.get(m.as_str()).with_context(|| format!("synset member {m} without an entry"))?.clone();
            }
        }
        // word senses in frequency order
        let mut n_senses = 0;
        for (lemma, ss) in entries {
            let mut v = Vec::with_capacity(ss.len());
            for s in ss {
                v.push(*syn_id.get(&s).with_context(|| format!("sense {lemma} → no synset {s}"))?);
            }
            n_senses += v.len();
            self.senses.push((lemma, v));
        }
        // relations + inverse-pair gate
        let direct: &[(&str, &str, Rel, bool)] = &[
            // (type, inverse type, relation, flip direction)
            ("hypernym", "hyponym", Rel::IsA, false),
            ("instance_hypernym", "instance_hyponym", Rel::InstanceOf, false),
            ("holo_part", "mero_part", Rel::PartOf, false),
            ("holo_member", "mero_member", Rel::MemberOf, false),
            ("mero_substance", "holo_substance", Rel::MadeOf, false),
            ("entails", "is_entailed_by", Rel::Entails, false),
            ("causes", "is_caused_by", Rel::Causes, false),
        ];
        let mut pairs: HashMap<&str, HashSet<(Cid, Cid)>> = HashMap::new();
        for (s, ty, t) in &rels {
            let tc = *syn_id.get(t).with_context(|| format!("relation {ty} to a missing synset {t}"))?;
            pairs.entry(ty.as_str()).or_default().insert((*s, tc));
        }
        let empty = HashSet::new();
        let mut mismatch = 0usize;
        for (ty, inv, rel, _) in direct {
            let a = pairs.get(ty).unwrap_or(&empty);
            let b = pairs.get(inv).unwrap_or(&empty);
            let miss_b = a.iter().filter(|(s, t)| !b.contains(&(*t, *s))).count();
            let miss_a = b.iter().filter(|(s, t)| !a.contains(&(*t, *s))).count();
            mismatch += miss_a + miss_b;
            self.report.push(format!("OEWN {ty}: {} edges {} (unpaired: {miss_b} / {miss_a})", a.len(), rel.name()));
            let mut v: Vec<_> = a.iter().copied().collect();
            v.sort_unstable();
            for (s, t) in v {
                self.edges.push((s, *rel, t));
            }
        }
        if mismatch > 0 {
            bail!("gate: {mismatch} OEWN edges without an inverse pair");
        }
        self.report.push(format!(
            "OEWN: synsets {}, entries {}, senses {n_senses}, only in -plus {}",
            syn_id.len(),
            self.senses.len(),
            self.cs.iter().filter(|c| c.flags & FLAG_PLUS != 0).count()
        ));
        Ok(())
    }

    /// FrameNet 1.7 from zip: frames, roles (frame elements), lexical units, frame relations.
    pub fn framenet(&mut self, path: &Path) -> Result<()> {
        let zip = Zip::open(path)?;
        let names: Vec<String> = zip.names().filter(|n| n.starts_with("framenet_v17/frame/") && n.ends_with(".xml")).map(|s| s.to_string()).collect();
        if names.is_empty() {
            bail!("{} has no framenet_v17/frame/*.xml", path.display());
        }
        let mut frame_by_name: HashMap<String, Cid> = HashMap::new();
        let mut frels: Vec<(Cid, Rel, String)> = Vec::new();
        let (mut n_fe, mut n_lu) = (0usize, 0usize);
        for n in &names {
            let xml = String::from_utf8(zip.read(n)?).with_context(|| format!("{n}: not UTF-8"))?;
            let mut rd = Reader::from_str(&xml);
            let mut frame: Option<Cid> = None;
            let mut fe: Option<Cid> = None;
            let mut rel: Option<Rel> = None;
            let mut in_rel_frame = false;
            let mut rel_name = String::new();
            let mut in_def = 0u8; // 1 — frame definition, 2 — role definition
            let mut def = String::new();
            let mut cur_lu: Option<(String, Pos, u32)> = None;
            let mut in_lu = false;
            loop {
                match rd.read_event().with_context(|| format!("{n}: XML"))? {
                    ev @ (Event::Start(_) | Event::Empty(_)) => {
                        let (e, is_empty) = match ev {
                            Event::Start(e) => (e, false),
                            Event::Empty(e) => (e, true),
                            _ => unreachable!(),
                        };
                        match e.name().as_ref() {
                            "frame" => {
                                let name = attr(&e, "name")?.context("frame without name")?;
                                let id: u32 = attr(&e, "ID")?.context("frame without ID")?.parse()?;
                                let c = self.add(CB { src: Some(Src::Frame), key: id, pos: Some(Pos::Frame), name: name.clone(), ..Default::default() });
                                if frame_by_name.insert(name.clone(), c).is_some() {
                                    bail!("frame {name} twice");
                                }
                                frame = Some(c);
                            }
                            "definition" if !is_empty => {
                                in_def = if fe.is_some() { 2 } else if in_lu { 0 } else if frame.is_some() { 1 } else { 0 };
                                def.clear();
                            }
                            "FE" => {
                                let f = frame.context("FE outside a frame")?;
                                let name = attr(&e, "name")?.context("FE without name")?;
                                let id: u32 = attr(&e, "ID")?.context("FE without ID")?.parse()?;
                                let core = attr(&e, "coreType")?.and_then(|s| Core::parse(&s)).with_context(|| format!("{name}: coreType"))?;
                                let fname = self.cs[f as usize].name.clone();
                                let c = self.add(CB {
                                    src: Some(Src::Role),
                                    key: id,
                                    pos: Some(Pos::Role),
                                    flags: (core as u8) << 1,
                                    name: format!("{fname}.{name}"),
                                    ..Default::default()
                                });
                                self.edges.push((f, Rel::HasRole, c));
                                n_fe += 1;
                                fe = (!is_empty).then_some(c);
                            }
                            "frameRelation" => {
                                rel = if is_empty { None } else { match attr(&e, "type")?.as_deref() {
                                    Some("Inherits from") => Some(Rel::FrameInherits),
                                    Some("Uses") => Some(Rel::FrameUses),
                                    Some("Subframe of") => Some(Rel::SubframeOf),
                                    Some("Perspective on") => Some(Rel::PerspectiveOn),
                                    Some("Precedes") => Some(Rel::Precedes),
                                    Some("Is Causative of") => Some(Rel::CausativeOf),
                                    Some("Is Inchoative of") => Some(Rel::InchoativeOf),
                                    _ => None,
                                } };
                            }
                            "relatedFrame" if !is_empty => {
                                in_rel_frame = rel.is_some();
                                rel_name.clear();
                            }
                            "lexUnit" => {
                                let name = attr(&e, "name")?.context("lexUnit without name")?;
                                let id: u32 = attr(&e, "ID")?.context("lexUnit without ID")?.parse()?;
                                let pos = match attr(&e, "POS")?.as_deref() {
                                    Some("V") => Some(Pos::Verb),
                                    Some("N") => Some(Pos::Noun),
                                    Some("A") => Some(Pos::Adj),
                                    Some("ADV") => Some(Pos::Adv),
                                    _ => None,
                                };
                                // «purchase [act].n» → «purchase»; «pick up.v» → «pick up»
                                let base = name.rsplit_once('.').map(|(b, _)| b).unwrap_or(&name);
                                let base = base.split(" [").next().unwrap_or(base).split('(').next().unwrap_or(base).trim().to_string();
                                cur_lu = pos.map(|p| (base, p, id));
                                in_lu = !is_empty;
                                if is_empty {
                                    if let (Some((w, p, id)), Some(f)) = (cur_lu.take(), frame) {
                                        self.lus.push((w, p, f, id, 0));
                                        n_lu += 1;
                                    }
                                }
                            }
                            "sentenceCount" => {
                                if let (Some((w, p, id)), Some(f)) = (cur_lu.take(), frame) {
                                    let ann: u32 = attr(&e, "annotated")?.and_then(|s| s.parse().ok()).unwrap_or(0);
                                    self.lus.push((w, p, f, id, ann));
                                    n_lu += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    Event::Text(t) => {
                        if in_def > 0 {
                            def.push_str(&t.xml10_content());
                        } else if in_rel_frame {
                            rel_name.push_str(&t.xml10_content());
                        }
                    }
                    Event::GeneralRef(r) => {
                        if in_def > 0 {
                            if let Some(ch) = entity(&r.into_inner()) {
                                def.push(ch);
                            }
                        }
                    }
                    Event::End(e) => match e.name().as_ref() {
                        "definition" => {
                            let target = if in_def == 2 { fe } else if in_def == 1 { frame } else { None };
                            if let Some(c) = target {
                                self.cs[c as usize].gloss = first_sentence(&strip_tags(&def), 300);
                            }
                            in_def = 0;
                        }
                        "FE" => fe = None,
                        "relatedFrame" => {
                            if in_rel_frame {
                                frels.push((frame.context("relatedFrame outside a frame")?, rel.context("relatedFrame without a type")?, rel_name.trim().to_string()));
                            }
                            in_rel_frame = false;
                        }
                        "frameRelation" => rel = None,
                        "lexUnit" => {
                            in_lu = false;
                            // an LU without sentenceCount is still a lexical unit
                            if let (Some((w, p, id)), Some(f)) = (cur_lu.take(), frame) {
                                self.lus.push((w, p, f, id, 0));
                                n_lu += 1;
                            }
                        }
                        _ => {}
                    },
                    Event::Eof => break,
                    _ => {}
                }
            }
        }
        let mut n_rel = 0;
        for (f, r, name) in frels {
            let t = *frame_by_name.get(&name).with_context(|| format!("gate: relation of frame {} to a missing frame {name}", self.cs[f as usize].name))?;
            self.edges.push((f, r, t));
            n_rel += 1;
        }
        self.report.push(format!("FrameNet: frames {}, roles {n_fe}, lexical units {n_lu}, frame relations {n_rel}", frame_by_name.len()));
        Ok(())
    }

    /// Assembles the graph: dictionary (sorted), senses, CSR of edges in both directions, lexical units.
    pub fn finish(self) -> Result<(Graph, Vec<String>)> {
        let Builder { cs, mut edges, senses, lus, report } = self;
        let n = cs.len();
        // dictionary: sense lemmas, synset members, lexical-unit lemmas
        let mut all: Vec<&str> = senses.iter().map(|(w, _)| w.as_str()).collect();
        all.extend(cs.iter().flat_map(|c| c.members.iter().map(|m| m.as_str())));
        all.extend(lus.iter().map(|l| l.0.as_str()));
        all.sort_unstable();
        all.dedup();
        let mut g = Graph::default();
        g.word_off.push(0);
        for w in &all {
            g.words.push_str(w);
            g.word_off.push(g.words.len() as u32);
        }
        let wid = |s: &str| -> Wid { all.binary_search(&s).expect("word in the dictionary") as Wid };
        // concepts
        g.gloss_off.push(0);
        g.name_off.push(0);
        g.mem_off.push(0);
        for c in &cs {
            g.src.push(c.src.context("concept without a source")?);
            g.key.push(c.key);
            g.pos.push(c.pos.context("concept without a part of speech")?);
            g.lexfile.push(c.lexfile);
            g.flags.push(c.flags);
            g.ili.push(c.ili);
            g.gloss.push_str(&c.gloss);
            g.gloss_off.push(g.gloss.len() as u32);
            g.names.push_str(&c.name);
            g.name_off.push(g.names.len() as u32);
            for m in &c.members {
                g.mem.push(wid(m));
            }
            g.mem_off.push(g.mem.len() as u32);
        }
        // senses: a word may have several entries (parts of speech) — merge in entry order
        let mut per_word: Vec<Vec<Cid>> = vec![Vec::new(); all.len()];
        for (w, ss) in &senses {
            let v = &mut per_word[wid(w) as usize];
            for s in ss {
                if !v.contains(s) {
                    v.push(*s);
                }
            }
        }
        g.sense_off.push(0);
        for v in per_word {
            g.senses.extend(v);
            g.sense_off.push(g.senses.len() as u32);
        }
        // edges: no duplicates, CSR
        edges.sort_unstable_by_key(|&(s, r, t)| (s, r as u8, t));
        edges.dedup();
        for &(s, _, t) in &edges {
            if s as usize >= n || t as usize >= n {
                bail!("gate: edge outside the concepts");
            }
        }
        let csr = |key: &dyn Fn(&(Cid, Rel, Cid)) -> (Cid, Edge)| -> (Vec<u32>, Vec<Edge>) {
            let mut v: Vec<(Cid, Edge)> = edges.iter().map(key).collect();
            v.sort_by_key(|(s, e)| (*s, e.rel as u8, e.to));
            let mut off = vec![0u32; n + 1];
            for (s, _) in &v {
                off[*s as usize + 1] += 1;
            }
            for i in 0..n {
                off[i + 1] += off[i];
            }
            (off, v.into_iter().map(|(_, e)| e).collect())
        };
        (g.out_off, g.out) = csr(&|&(s, r, t)| (s, Edge { rel: r, to: t }));
        (g.in_off, g.inn) = csr(&|&(s, r, t)| (t, Edge { rel: r, to: s }));
        // lexical units: by word, within a word — more annotated sentences first (frequency in the FrameNet corpus)
        let mut lv: Vec<(Wid, Pos, Cid, u32, u32)> = lus.iter().map(|(w, p, f, id, a)| (wid(w), *p, *f, *id, *a)).collect();
        lv.sort_by(|a, b| a.0.cmp(&b.0).then(b.4.cmp(&a.4)).then(a.3.cmp(&b.3)));
        g.lus = lv.into_iter().map(|(w, p, f, id, _)| Lu { word: w, pos: p, frame: f, id }).collect();
        let mut report = report;
        report.push(format!("graph: concepts {n}, words {}, edges {}, lexical units {}", g.n_words(), g.out.len(), g.lus.len()));
        Ok((g, report))
    }
}
