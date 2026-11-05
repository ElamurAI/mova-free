//! Queries for writers over `world(epoch)`. Every answer is deterministic (ties break by document,
//! text and sentence) and pages with `offset`/`k`; nothing in the store is dropped to answer.
//!
//! - `entity` — an entity's kinds, forms, mentions per document and definitions;
//! - `explain` — the best defining or explaining sentences of a term across documents;
//! - `related` — documents ranked by graph strength (cites, cited by, similar, same author, shared
//!   entities weighted by idf);
//! - `neighbourhood` — a document's nearest documents, nearest first, with their texts; its
//!   content is every sentence of those texts (`neighbourhood_content`);
//! - `sentences` — diverse example sentences of a word;
//! - `context` — a writer's pack from the neighbourhood: the document's entities with explanations
//!   from other documents, the related documents and every statement of the neighbourhood's texts,
//!   all with citations `{doc, text, s}`.
use crate::world::{entity_key, word_key, World, WSent};
use anyhow::{bail, Result};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

/// Rank of a text kind for explanations and ordering: the document's own text and neighbours'
/// originals first, then earlier descriptions, pages, consultant and Mova drafts.
pub fn kind_rank(kind: &str) -> u8 {
    match kind {
        "original" | "neighbour" => 0,
        "site-old" => 1,
        "web" => 2,
        "sol" => 3,
        "mova" => 4,
        _ => 5,
    }
}

fn kind_bonus(kind: &str) -> f64 {
    [0.5, 0.3, 0.2, 0.1, 0.0, 0.0][kind_rank(kind) as usize]
}

fn cite(w: &World, t: u32, s: u32) -> Value {
    let x = w.text(t);
    json!({"doc": x.doc, "text": x.id, "s": s})
}

fn norm_text(s: &str) -> String {
    s.split_whitespace().map(|w| w.to_lowercase()).collect::<Vec<_>>().join(" ")
}

/// An entity of `world(epoch)`.
pub fn entity(w: &mut World, term: &str, epoch: u64) -> Result<Value> {
    let key = entity_key(term);
    let Some(rec) = w.entity(&key)? else { return Ok(json!({"term": term, "key": key, "found": false})) };
    let posts: Vec<(u32, u32)> = rec.postings.iter().copied().filter(|(t, _)| w.visible(*t, epoch)).collect();
    let mut by_doc: BTreeMap<String, usize> = BTreeMap::new();
    for (t, _) in &posts {
        *by_doc.entry(w.text(*t).doc.clone()).or_default() += 1;
    }
    let mut docs: Vec<(String, usize)> = by_doc.into_iter().collect();
    docs.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    // Expansions grouped case-insensitively, the most frequent first, each with its first citation.
    let mut groups: BTreeMap<String, (usize, Value)> = BTreeMap::new();
    for (t, s, x) in rec.defs.iter().filter(|d| w.visible(d.0, epoch)) {
        let g = groups.entry(x.to_lowercase()).or_insert_with(|| {
            let mut c = cite(w, *t, *s);
            c["expansion"] = json!(x);
            (0, c)
        });
        g.0 += 1;
    }
    let mut defs: Vec<(usize, String, Value)> = groups.into_iter().map(|(k, (n, mut c))| {
        c["count"] = json!(n);
        (n, k, c)
    }).collect();
    defs.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    let defs: Vec<Value> = defs.into_iter().map(|x| x.2).collect();
    Ok(json!({"term": term, "key": key, "found": true, "kind": rec.kind(), "kinds": rec.kinds, "forms": rec.forms, "mentions": posts.len(), "docs": docs.len(),
              "definitions": defs, "by_doc": docs.iter().map(|(d, n)| json!({"doc": d, "mentions": n})).collect::<Vec<_>>()}))
}

/// Candidate sentences of a term: its entity postings and the postings of its rarest word.
fn candidates(w: &mut World, term: &str, epoch: u64) -> Result<Vec<(u32, u32)>> {
    let mut set: BTreeSet<(u32, u32)> = BTreeSet::new();
    if let Some(r) = w.entity(&entity_key(term))? {
        set.extend(r.postings);
    }
    let mut best: Option<Vec<(u32, u32)>> = None;
    for word in term.split_whitespace().filter_map(word_key) {
        let p = w.word(&word)?;
        if best.as_ref().is_none_or(|b| p.len() < b.len()) {
            best = Some(p);
        }
    }
    set.extend(best.unwrap_or_default());
    Ok(set.into_iter().filter(|(t, _)| w.visible(*t, epoch)).collect())
}

/// Explaining sentences of `term` in `world(epoch)`: scored by `topic_corpus::corpus::explain_score`
/// plus a bonus by text kind; at most two per document, near-duplicates dropped; `within` limits the
/// documents, `exclude` drops one. Page `offset..offset+k`.
pub fn explain_in(w: &mut World, term: &str, k: usize, offset: usize, epoch: u64, within: Option<&BTreeSet<String>>, exclude: Option<&str>) -> Result<Vec<Value>> {
    let mut scored: Vec<(f64, String, String, u32, u32)> = vec![];
    for (t, s) in candidates(w, term, epoch)? {
        let x = w.text(t).clone();
        if exclude == Some(x.doc.as_str()) || within.is_some_and(|set| !set.contains(&x.doc)) {
            continue;
        }
        let sent = w.sentence(t, s)?;
        let forms: Vec<String> = sent.words.iter().map(|w| w.0.clone()).collect();
        let mut sc = topic_corpus::corpus::explain_score(&forms, term, &x.roles);
        if sc.is_none() {
            let lower: Vec<String> = forms.iter().map(|f| f.to_lowercase()).collect();
            sc = topic_corpus::corpus::explain_score(&lower, &term.to_lowercase(), &x.roles);
        }
        if let Some(sc) = sc {
            scored.push(((sc + kind_bonus(&x.kind)) * 1000.0, x.doc.clone(), x.id.clone(), t, s));
        }
    }
    scored.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(&b.1)).then_with(|| a.2.cmp(&b.2)).then(a.4.cmp(&b.4)));
    let mut per_doc: BTreeMap<String, usize> = BTreeMap::new();
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut out = vec![];
    for (sc, doc, _, t, s) in scored {
        let n = per_doc.entry(doc).or_default();
        if *n >= 2 {
            continue;
        }
        let sent = w.sentence(t, s)?;
        if !seen.insert(norm_text(&sent.text)) {
            continue;
        }
        *n += 1;
        if out.len() >= offset + k {
            break;
        }
        let x = w.text(t);
        out.push(json!({"doc": x.doc, "text": x.id, "s": s, "kind": x.kind, "epoch": x.epoch, "sentence": sent.text, "score": sc / 1000.0}));
    }
    Ok(out.into_iter().skip(offset).collect())
}

pub fn explain(w: &mut World, term: &str, k: usize, offset: usize, epoch: u64) -> Result<Value> {
    Ok(json!({"term": term, "epoch": epoch, "explanations": explain_in(w, term, k, offset, epoch, None, None)?}))
}

/// Documents related to `doc` in `world(epoch)` with their graph strength: cites or cited by 3,
/// same author 2, similar 4 × score, shared entities 5 × (idf of shared entities / idf of the
/// document's entities). Only documents with visible texts. Sorted strongest first, then by id.
pub fn related_all(w: &mut World, doc: &str, epoch: u64) -> Result<Vec<(String, f64, Vec<Value>)>> {
    if !w.docs.contains_key(doc) {
        bail!("{doc} is not in the world");
    }
    let present: BTreeSet<String> = w.texts.iter().filter(|t| t.epoch < epoch).map(|t| t.doc.clone()).collect();
    let mut score: BTreeMap<String, (f64, Vec<Value>)> = BTreeMap::new();
    let mut add = |d: &str, kind: &str, wgt: f64, sc: f64| {
        if d != doc && present.contains(d) {
            let e = score.entry(d.to_string()).or_default();
            e.0 += sc;
            e.1.push(json!({"kind": kind, "weight": wgt}));
        }
    };
    for e in w.edges.clone() {
        match (e.kind.as_str(), e.from == doc, e.to == doc) {
            ("cites", true, _) => add(&e.to, "cites", e.weight, 3.0),
            ("cites", _, true) => add(&e.from, "cited_by", e.weight, 3.0),
            ("similar", true, _) => add(&e.to, "similar", e.weight, 4.0 * e.weight.max(0.1)),
            ("similar", _, true) => add(&e.from, "similar", e.weight, 4.0 * e.weight.max(0.1)),
            ("same_author", true, _) => add(&e.to, "same_author", e.weight, 2.0),
            ("same_author", _, true) => add(&e.from, "same_author", e.weight, 2.0),
            _ => {}
        }
    }
    // Shared entities, idf-weighted.
    let n_docs = present.len().max(1) as f64;
    let mine = w.doc_entities_at(doc, epoch)?;
    let mut total = 0.0;
    let mut shared: BTreeMap<String, f64> = BTreeMap::new();
    for key in mine.keys() {
        let Some(rec) = w.entity(key)? else { continue };
        let ds: BTreeSet<String> = rec.postings.iter().filter(|(t, _)| w.visible(*t, epoch)).map(|(t, _)| w.text(*t).doc.clone()).collect();
        let idf = (n_docs / ds.len().max(1) as f64).ln();
        total += idf;
        for d in ds {
            if d != doc {
                *shared.entry(d).or_default() += idf;
            }
        }
    }
    for (d, v) in shared {
        if v > 0.0 && total > 0.0 {
            let r = (v / total * 1e6).round() / 1e6;
            add(&d, "shares_entity", r, 5.0 * r);
        }
    }
    let mut v: Vec<(String, f64, Vec<Value>)> = score.into_iter().map(|(d, (s, e))| (d, (s * 1e6).round() / 1e6, e)).collect();
    v.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    Ok(v)
}

pub fn related(w: &mut World, doc: &str, k: usize, offset: usize, epoch: u64) -> Result<Value> {
    let v = related_all(w, doc, epoch)?;
    let total = v.len();
    let rows: Vec<Value> = v.into_iter().skip(offset).take(k).map(|(d, s, e)| json!({"doc": d, "title": w.docs.get(&d).map(|x| x.title.clone()), "score": s, "edges": e})).collect();
    Ok(json!({"doc": doc, "epoch": epoch, "total": total, "related": rows}))
}

/// Defaults of `neighbourhood`: radius 1, at most 20 documents besides the document itself.
pub const RADIUS: usize = 1;
pub const MAX_DOCS: usize = 20;

#[derive(Clone, Debug)]
pub struct Near {
    pub doc: String,
    pub distance: usize,
    pub strength: f64,
    pub via: Option<String>,
}

/// The nearest documents of `doc` in `world(epoch)`: breadth-first over `related_all` up to
/// `radius`; order (distance, strength descending, id). The strength of a document at distance 2
/// or more is the best over its parents of min(parent strength, edge strength). The document
/// itself comes first (distance 0).
pub fn nearest(w: &mut World, doc: &str, radius: usize, max_docs: usize, epoch: u64) -> Result<Vec<Near>> {
    let mut out = vec![Near { doc: doc.into(), distance: 0, strength: f64::INFINITY, via: None }];
    let mut seen: BTreeSet<String> = [doc.to_string()].into();
    let mut frontier = vec![(doc.to_string(), f64::INFINITY)];
    for dist in 1..=radius {
        let mut best: BTreeMap<String, (f64, String)> = BTreeMap::new();
        for (p, ps) in &frontier {
            for (d, s, _) in related_all(w, p, epoch)? {
                if seen.contains(&d) {
                    continue;
                }
                let st = s.min(*ps);
                let e = best.entry(d).or_insert((f64::NEG_INFINITY, p.clone()));
                if st > e.0 || (st == e.0 && p < &e.1) {
                    *e = (st, p.clone());
                }
            }
        }
        let mut layer: Vec<(String, f64, String)> = best.into_iter().map(|(d, (s, p))| (d, s, p)).collect();
        layer.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        frontier.clear();
        for (d, s, p) in layer {
            if out.len() > max_docs {
                break;
            }
            seen.insert(d.clone());
            frontier.push((d.clone(), s));
            out.push(Near { doc: d, distance: dist, strength: s, via: Some(p) });
        }
        if out.len() > max_docs || frontier.is_empty() {
            break;
        }
    }
    Ok(out)
}

/// Visible texts of a document, in reading order: kind rank, then newest epoch first, then index.
pub fn texts_of(w: &World, doc: &str, epoch: u64) -> Vec<u32> {
    let mut v: Vec<u32> = w.docs.get(doc).map(|d| d.texts.iter().copied().filter(|t| w.visible(*t, epoch)).collect()).unwrap_or_default();
    v.sort_by(|a, b| {
        let (x, y) = (w.text(*a), w.text(*b));
        kind_rank(&x.kind).cmp(&kind_rank(&y.kind)).then(y.epoch.cmp(&x.epoch)).then(a.cmp(b))
    });
    v
}

pub fn neighbourhood(w: &mut World, doc: &str, radius: usize, max_docs: usize, epoch: u64) -> Result<Value> {
    let near = nearest(w, doc, radius, max_docs, epoch)?;
    let mut rows = vec![];
    for n in &near {
        let ts = texts_of(w, &n.doc, epoch);
        let sentences: usize = ts.iter().map(|t| w.text(*t).sentences).sum();
        let statements: usize = ts.iter().map(|t| w.text(*t).statements).sum();
        rows.push(json!({"doc": n.doc, "title": w.docs.get(&n.doc).map(|d| d.title.clone()), "distance": n.distance,
            "strength": if n.strength.is_finite() { json!(n.strength) } else { json!(null) }, "via": n.via,
            "texts": ts.iter().map(|t| { let x = w.text(*t); json!({"text": x.id, "kind": x.kind, "epoch": x.epoch, "sentences": x.sentences, "statements": x.statements}) }).collect::<Vec<_>>(),
            "sentences": sentences, "statements": statements}));
    }
    Ok(json!({"doc": doc, "epoch": epoch, "radius": radius, "max_docs": max_docs, "docs": rows}))
}

/// Every sentence of every visible text of the neighbourhood, nearest document first, each as one
/// JSON line `{doc, text, s, kind, statement, sentence, entities}` passed to `emit`.
pub fn neighbourhood_content(w: &mut World, doc: &str, radius: usize, max_docs: usize, epoch: u64, emit: &mut dyn FnMut(Value) -> Result<()>) -> Result<usize> {
    let near = nearest(w, doc, radius, max_docs, epoch)?;
    let mut n = 0;
    for x in near {
        for t in texts_of(w, &x.doc, epoch) {
            let sh = w.shard(t)?;
            let tx = w.text(t).clone();
            for s in sh.iter() {
                emit(json!({"doc": tx.doc, "text": tx.id, "s": s.s, "kind": tx.kind, "statement": s.kind, "sentence": s.text, "entities": s.ents}))?;
                n += 1;
            }
        }
    }
    Ok(n)
}

fn jaccard(a: &BTreeSet<String>, b: &BTreeSet<String>) -> f64 {
    let i = a.intersection(b).count() as f64;
    let u = a.union(b).count() as f64;
    if u == 0.0 {
        1.0
    } else {
        i / u
    }
}

/// Example sentences of a word in `world(epoch)`: round-robin over documents (sorted by id), within a
/// document by text order (`texts_of`); sentences of 6–40 words first, then the others; a sentence
/// whose word set overlaps a chosen one by 0.8 or more (Jaccard) is dropped.
pub fn sentences(w: &mut World, word: &str, k: usize, offset: usize, epoch: u64) -> Result<Value> {
    let key = word_key(word).unwrap_or_default();
    let posts = w.word(&key)?;
    let mut by_doc: BTreeMap<String, Vec<(u8, std::cmp::Reverse<u64>, u32, u32)>> = BTreeMap::new();
    for (t, s) in posts.into_iter().filter(|(t, _)| w.visible(*t, epoch)) {
        let x = w.text(t);
        by_doc.entry(x.doc.clone()).or_default().push((kind_rank(&x.kind), std::cmp::Reverse(x.epoch), t, s));
    }
    for v in by_doc.values_mut() {
        v.sort();
    }
    let mut chosen: Vec<(u32, u32, WSent)> = vec![];
    let mut sets: Vec<BTreeSet<String>> = vec![];
    for pass in 0..2 {
        let mut cursors: BTreeMap<String, usize> = by_doc.keys().map(|d| (d.clone(), 0)).collect();
        loop {
            let mut progressed = false;
            for (d, v) in &by_doc {
                let c = cursors.get_mut(d).unwrap();
                while *c < v.len() {
                    let (_, _, t, s) = v[*c];
                    *c += 1;
                    let sent = w.sentence(t, s)?;
                    let n = sent.words.len();
                    let good = (6..=40).contains(&n);
                    if (pass == 0) != good || chosen.iter().any(|x| x.0 == t && x.1 == s) {
                        continue;
                    }
                    let set: BTreeSet<String> = sent.words.iter().map(|w| w.0.to_lowercase()).collect();
                    if sets.iter().any(|x| jaccard(x, &set) >= 0.8) {
                        continue;
                    }
                    sets.push(set);
                    chosen.push((t, s, sent));
                    progressed = true;
                    break;
                }
                if chosen.len() >= offset + k {
                    break;
                }
            }
            if !progressed || chosen.len() >= offset + k {
                break;
            }
        }
        if chosen.len() >= offset + k {
            break;
        }
    }
    let rows: Vec<Value> = chosen.into_iter().skip(offset).take(k).map(|(t, s, x)| {
        let mut c = cite(w, t, s);
        c["sentence"] = json!(x.text);
        c
    }).collect();
    Ok(json!({"word": word, "epoch": epoch, "sentences": rows}))
}

/// A writer's context pack for `doc` from its neighbourhood in `world(epoch)`: the document's
/// entities (most mentioned first) with up to two explanations each from other documents of the
/// neighbourhood, the related documents, and every statement of the neighbourhood's texts
/// (nearest document first). With `max_bytes`, statements and then entities are cut from the end
/// until the pack fits; `cut` says how many.
pub fn context(w: &mut World, doc: &str, radius: usize, max_docs: usize, epoch: u64, max_bytes: Option<usize>) -> Result<Value> {
    let near = nearest(w, doc, radius, max_docs, epoch)?;
    let within: BTreeSet<String> = near.iter().map(|n| n.doc.clone()).collect();
    let mut ents: Vec<(String, String, u64)> = w.doc_entities_at(doc, epoch)?.into_iter().map(|(k, (kind, n))| (k, kind, n)).collect();
    ents.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0)));
    let mut entities = vec![];
    for (k, kind, n) in ents {
        let ex = if n >= 2 && within.len() > 1 { explain_in(w, &k, 2, 0, epoch, Some(&within), Some(doc))? } else { vec![] };
        entities.push(json!({"entity": k, "kind": kind, "mentions": n, "explanations": ex}));
    }
    let related: Vec<Value> = near.iter().skip(1).map(|n| json!({"doc": n.doc, "title": w.docs.get(&n.doc).map(|d| d.title.clone()), "distance": n.distance, "strength": n.strength, "via": n.via})).collect();
    let mut statements = vec![];
    for n in &near {
        for t in texts_of(w, &n.doc, epoch) {
            let sh = w.shard(t)?;
            let tx = w.text(t).clone();
            for s in sh.iter().filter(|s| !s.kind.is_empty()) {
                statements.push(json!({"doc": tx.doc, "text": tx.id, "s": s.s, "kind": s.kind, "sentence": s.text}));
            }
        }
    }
    let title = w.docs.get(doc).map(|d| d.title.clone());
    let mut cut = json!({"statements": 0, "entities": 0});
    let mut pack = json!({"doc": doc, "title": title, "epoch": epoch, "entities": entities, "related": related, "statements": statements, "cut": cut});
    if let Some(max) = max_bytes {
        let size = |p: &Value| serde_json::to_vec(p).map(|v| v.len()).unwrap_or(usize::MAX);
        // Cut an eighth of the larger list (in bytes) from its far end until the pack fits.
        while size(&pack) > max {
            let (se, ss) = (size(&pack["entities"]), size(&pack["statements"]));
            let field = if ss >= se { "statements" } else { "entities" };
            let arr = pack[field].as_array_mut().unwrap();
            if arr.is_empty() {
                break;
            }
            let drop = (arr.len() / 8).max(1);
            arr.truncate(arr.len() - drop);
            cut[field] = json!(cut[field].as_u64().unwrap() + drop as u64);
            pack["cut"] = cut.clone();
        }
    }
    let bytes = serde_json::to_vec(&pack)?.len();
    pack["bytes"] = json!(bytes);
    Ok(pack)
}
