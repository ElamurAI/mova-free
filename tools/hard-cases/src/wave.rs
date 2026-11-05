//! The Fish priority lane: a **wave** turns open hard cases into source-bound Fish annotation
//! tasks, in their documents' context, laid out exactly as the bounded Fish pager
//! (`mova-fish-pages`) consumes them: pinned intents, prepared task queues, ready manifests, one
//! plan and one pager configuration per lane pager. Nothing here calls a teacher; the pagers do.
//!
//! A wave:
//! 1. selects the cases open for gparse that the policy admits (kinds, track), not already in the
//!    lane's ledger, not held out (exact or whitespace-normalized text), one per normalized text,
//!    hardest first (most judges, lowest witness agreement), up to `fish.max_cases`;
//! 2. gets each document once as a bundle (`mova-hard-bundle-v1`) from its source's command and
//!    pins it in the wave;
//! 3. builds each target's single-target Fish task: up to `fish.context` preceding sentences of its
//!    document as context, two candidate producers (the source's own `en` tree and the deployed
//!    gparse on the same tokens, both with the pragmatics SLM's labels), no open pragmatic witness
//!    (declared absent: the Fish record `missing_witness`); `teaching` for the train and dev
//!    splits, `unknown` for the measure split;
//! 4. packs them with the public Fish packer (`mova-fish-batch`, the 100 000-byte request bound,
//!    up to `fish.max_targets` targets per task) page by page; every target is queued, deferred or
//!    rejected, with its reason;
//! 5. writes `wave.json` last, then appends every selected case to the ledger
//!    (`<lane_root>/queued.jsonl`) so no case is sent twice.
//!
//! Layout of `<lane_root>/wave-NNNN/`: `bundles/`, `tasks/`, `queues/`, `intents/`, `ready/`,
//! `plans/hNN.json`, `hNN-config.json`, `selection.jsonl`, `wave.json`, `work/`.
use crate::config::Config;
use crate::{read_json, sha, write_new};
use annotation_review_worker::annotation::{self, Candidate, ContextLine, Negation, Task, Use, VoiceScope};
use anyhow::{bail, ensure, Context, Result};
use hard_case_list::{Case, GPARSE};
use mova_fish_batch::{PackOptions, PacketInput, SourcePin};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const BUNDLE: &str = "mova-hard-bundle-v1";
/// The declared absence of an open pragmatic witness.
const NO_WITNESS: &str = "sent_id\tnli1_act\tnli2_act\tnli1_act_p\tnli2_act_p\n";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BundleSentence {
    pub id: String,
    pub n: usize,
    pub text: String,
    #[serde(default)]
    pub conllu: String,
    /// The pragmatics SLM's row (header `…\tconf`), or empty.
    #[serde(default)]
    pub prag: String,
}

/// A source's document: its sentences in order, with trees.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Bundle {
    pub schema: String,
    pub source: String,
    pub document: String,
    #[serde(default)]
    pub title: String,
    pub version: String,
    #[serde(default)]
    pub licence: String,
    pub model: String,
    pub sentences: Vec<BundleSentence>,
}

/// One selected case and what became of it in a wave.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Selected {
    pub key: String,
    pub norm_sha256: String,
    pub wave: String,
    /// The lane's target id `hard:<doc16>:<n>`.
    pub target: String,
    pub source: String,
    pub source_id: String,
    pub document: String,
    pub n: usize,
    pub licence: String,
    pub track: String,
    pub split: String,
    pub why: Vec<String>,
    /// `queued`, `deferred` or `rejected`.
    pub status: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub intent: String,
    #[serde(default)]
    pub task: String,
}

/// The ledger: every case a wave ever selected.
pub fn ledger(lane: &Path) -> Result<Vec<Selected>> {
    let p = lane.join("queued.jsonl");
    if !p.exists() {
        return Ok(vec![]);
    }
    std::fs::read_to_string(&p)?.lines().filter(|l| !l.trim().is_empty()).map(|l| Ok(serde_json::from_str(l)?)).collect()
}

/// Texts of the held-out sets: exact SHA-256 and whitespace-normalized SHA-256 of every `# text`.
pub fn heldout(cfg: &Config) -> Result<(BTreeSet<String>, BTreeSet<String>)> {
    let (mut exact, mut norm) = (BTreeSet::new(), BTreeSet::new());
    for p in &cfg.heldout {
        for l in std::fs::read_to_string(p.check()?)?.lines() {
            if let Some(t) = l.strip_prefix("# text = ") {
                exact.insert(sha(t));
                norm.insert(crate::norm_sha(t));
            }
        }
    }
    Ok((exact, norm))
}

/// The cases a wave would take, in order, with their track and split; and how many were left out, by reason.
pub fn select<'a>(cfg: &Config, st: &'a BTreeMap<String, Case>, done: &[Selected], held: &(BTreeSet<String>, BTreeSet<String>), max: usize) -> (Vec<(&'a Case, String, &'static str)>, BTreeMap<&'static str, usize>) {
    let keys: BTreeSet<&str> = done.iter().map(|s| s.key.as_str()).collect();
    let mut norms: BTreeSet<String> = done.iter().map(|s| s.norm_sha256.clone()).collect();
    let mut out = vec![];
    let mut skipped: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut cand: Vec<&Case> = vec![];
    for c in st.values() {
        let why = if !c.open_for(GPARSE) {
            "not open for gparse"
        } else if keys.contains(c.key.as_str()) {
            "already in the lane"
        } else if !cfg.sources.contains_key(&c.found.source) {
            "source not configured"
        } else if !cfg.policy.admits(&c.found.why) {
            "kinds outside the lane"
        } else if held.0.contains(&sha(c.found.text.trim())) || held.1.contains(&crate::norm_sha(&c.found.text)) {
            "held out"
        } else if cfg.policy.skip_markup && crate::policy::markup(&c.found.text) {
            "markup"
        } else {
            cand.push(c);
            continue;
        };
        *skipped.entry(why).or_default() += 1;
    }
    let score = |c: &Case| c.found.why.iter().filter(|w| cfg.policy.kinds.contains(w)).count();
    cand.sort_by(|a, b| {
        score(b).cmp(&score(a)).then(a.found.witness_uas.unwrap_or(1.0).total_cmp(&b.found.witness_uas.unwrap_or(1.0))).then(a.key.cmp(&b.key))
    });
    for c in cand {
        if out.len() >= max {
            break;
        }
        let track = cfg.policy.track(&c.found.licence).to_string();
        if track == "measure" && !cfg.policy.annotate_measure_track {
            *skipped.entry("licence: measure only").or_default() += 1;
            continue;
        }
        if !norms.insert(crate::norm_sha(&c.found.text)) {
            *skipped.entry("same text as another case").or_default() += 1;
            continue;
        }
        let split = cfg.policy.split(&c.found.source, &c.found.document.id);
        out.push((c, track, split));
    }
    (out, skipped)
}

/// Runs a source's bundle command for a case's document. Returns the exact stdout.
pub fn fetch(cfg: &Config, c: &Case) -> Result<Vec<u8>> {
    let s = cfg.sources.get(&c.found.source).with_context(|| format!("no source command for {}", c.found.source))?;
    ensure!(!s.command.is_empty(), "{}: empty command", c.found.source);
    let mut cmd = Command::new(&s.command[0]);
    cmd.args(&s.command[1..]);
    for f in &s.pointer {
        let v = &c.found.at[f.as_str()];
        let a = match v {
            Value::String(x) => x.clone(),
            Value::Number(x) => x.to_string(),
            _ => bail!("case {}: `where.{f}` missing", c.key),
        };
        cmd.arg(a);
    }
    cmd.envs(&s.env);
    let out = cmd.output().with_context(|| format!("{:?}", s.command))?;
    ensure!(out.status.success(), "bundle command for {} failed: {}", c.found.document.id, String::from_utf8_lossy(&out.stderr).lines().last().unwrap_or(""));
    Ok(out.stdout)
}

/// The pragmatics labeler, loaded once on first use.
pub struct Labels<'a> {
    cfg: &'a Config,
    m: Option<(en::annotate::Annotator, prag::eval::Labeler)>,
}

impl<'a> Labels<'a> {
    pub fn new(cfg: &'a Config) -> Self {
        Labels { cfg, m: None }
    }
    /// Rows (header `…\tconf`, no header line) for (id, text) pairs in document order.
    pub fn label(&mut self, sents: &[(String, String)]) -> Result<BTreeMap<String, String>> {
        if self.m.is_none() {
            let a = en::annotate::Annotator::load(&self.cfg.en_model)?;
            let l = prag::eval::Labeler::train(&self.cfg.prag_silver, &a)?;
            self.m = Some((a, l));
        }
        let (a, l) = self.m.as_ref().unwrap();
        Ok(l.label(a, sents).lines().filter_map(|r| Some((r.split('\t').next()?.to_string(), r.to_string()))).collect())
    }
}

/// gparse's trees for the given blocks (same ids, same tokens), on the CPU; MISC is kept from the
/// input so the two candidates differ only where the parsers do.
pub fn gparse_annotate(cfg: &Config, blocks: &[String], work: &Path) -> Result<BTreeMap<String, String>> {
    std::fs::create_dir_all(work)?;
    let inp = work.join("gparse-in.conllu");
    let out = work.join("gparse-out.conllu");
    std::fs::write(&inp, blocks.iter().map(|b| format!("{b}\n")).collect::<String>())?;
    let mut cmd = Command::new(&cfg.gparse.executable);
    cmd.arg("annotate").arg(&cfg.gparse.model_dir).arg(&inp).arg(&out).env("GP_CPU", "1");
    if let Some(lex) = &cfg.gparse.lex {
        cmd.env("GP_LEX", lex);
    }
    let r = cmd.output()?;
    ensure!(r.status.success(), "gparse annotate failed: {}", String::from_utf8_lossy(&r.stderr).lines().last().unwrap_or(""));
    let input: BTreeMap<String, &String> = blocks.iter().filter_map(|b| Some((crate::block_id(b)?.to_string(), b))).collect();
    let mut m = BTreeMap::new();
    for b in crate::blocks(&std::fs::read_to_string(&out)?) {
        let Some(id) = crate::block_id(&b).map(String::from) else { continue };
        let Some(src) = input.get(&id) else { continue };
        let misc: Vec<String> = crate::rows(src).iter().map(|c| c[9].to_string()).collect();
        let mut k = 0;
        let mut s = String::new();
        for l in b.lines() {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() == 10 && c[0].parse::<usize>().is_ok() {
                let mut c: Vec<String> = c.iter().map(|x| x.to_string()).collect();
                if let Some(x) = misc.get(k) {
                    c[9] = x.clone();
                }
                k += 1;
                s += &(c.join("\t") + "\n");
            } else {
                s += &format!("{l}\n");
            }
        }
        m.insert(id, s);
    }
    Ok(m)
}

/// The deployed gparse model's identity: SHA-256 of its weights.
pub fn gparse_revision(cfg: &Config) -> Result<String> {
    crate::file_sha(&cfg.gparse.model_dir.join("model.safetensors"))
}

/// A target ready to pack, or why not.
struct Target {
    sel: Selected,
    packet: Option<PacketInput>,
}

fn next_wave(lane: &Path) -> Result<(String, PathBuf)> {
    let mut last = 0;
    for e in std::fs::read_dir(lane)? {
        let e = e?;
        let name = e.file_name().to_string_lossy().into_owned();
        if let Some(n) = name.strip_prefix("wave-").and_then(|x| x.parse::<u32>().ok()) {
            ensure!(e.path().join("wave.json").exists(), "{}: incomplete wave (no wave.json); inspect it before building another", e.path().display());
            last = last.max(n);
        }
    }
    let id = format!("wave-{:04}", last + 1);
    Ok((id.clone(), lane.join(id)))
}

fn reason(s: &str) -> String {
    s.chars().take(2000).collect()
}

/// Builds one wave. With `dry`, selects and reports without writing anything.
pub fn build(cfg: &Config, dry: bool) -> Result<Value> {
    let fish = cfg.fish.as_ref().context("the configuration has no fish lane")?;
    let profile_path = fish.profile.check()?.to_path_buf();
    fish.executable.check()?;
    let profile: annotation::Profile = serde_json::from_slice(&std::fs::read(&profile_path)?)?;
    std::fs::create_dir_all(&cfg.lane_root)?;
    let lane = cfg.lane_root.canonicalize()?;
    let done = ledger(&lane)?;
    let held = heldout(cfg)?;
    let st = hard_case_list::state(&cfg.list)?;
    let (chosen, skipped) = select(cfg, &st, &done, &held, fish.max_cases);
    let mut by: BTreeMap<String, usize> = BTreeMap::new();
    for (_, t, s) in &chosen {
        *by.entry(format!("{t}/{s}")).or_default() += 1;
    }
    let summary = json!({"open_for_gparse": st.values().filter(|c| c.open_for(GPARSE)).count(), "in_ledger": done.len(), "selected": chosen.len(), "by_track_split": by, "left_out": skipped});
    if dry || chosen.is_empty() {
        return Ok(summary);
    }
    let (wave, dir) = next_wave(&lane)?;
    std::fs::create_dir(&dir)?;
    // 2. Bundles, one per document.
    let mut docs: BTreeMap<(String, String), Vec<usize>> = BTreeMap::new();
    for (i, (c, _, _)) in chosen.iter().enumerate() {
        docs.entry((c.found.source.clone(), c.found.document.id.clone())).or_default().push(i);
    }
    let mut bundles: BTreeMap<(String, String), std::result::Result<(Bundle, SourcePin), String>> = BTreeMap::new();
    let mut labels = Labels::new(cfg);
    for ((src, doc), idx) in &docs {
        let did = crate::doc_id(src, doc);
        let r = (|| -> Result<(Bundle, SourcePin)> {
            let bytes = fetch(cfg, chosen[idx[0]].0)?;
            let mut b: Bundle = serde_json::from_slice(&bytes).context("bundle JSON")?;
            ensure!(b.schema == BUNDLE, "bundle schema {}", b.schema);
            ensure!(&b.source == src && &b.document == doc, "bundle names {}:{}", b.source, b.document);
            b.sentences.sort_by_key(|s| s.n);
            let p = dir.join("bundles").join(format!("{}.json", &did[5..]));
            write_new(&p, &bytes)?;
            // Sentences without labels get the pragmatics SLM's, read in document order.
            if b.sentences.iter().any(|s| s.prag.is_empty()) {
                let pairs: Vec<(String, String)> = b.sentences.iter().map(|s| (format!("{did}:{}", s.n), s.text.clone())).collect();
                let rows = labels.label(&pairs)?;
                for s in b.sentences.iter_mut().filter(|s| s.prag.is_empty()) {
                    s.prag = rows.get(&format!("{did}:{}", s.n)).cloned().unwrap_or_default();
                }
            }
            Ok((b, SourcePin { path: p.to_string_lossy().into_owned(), sha256: crate::file_sha(&p)? }))
        })();
        bundles.insert((src.clone(), doc.clone()), r.map_err(|e| format!("{e:#}")));
    }
    // 3. One target per case.
    let agreement = annotation::bindings()["prag_agree"].as_str().context("prag_agree binding")?.to_string();
    let gp_rev = gparse_revision(cfg)?;
    let mut targets: Vec<Target> = vec![];
    let mut a_trees: Vec<String> = vec![];
    for (c, track, split) in &chosen {
        let did = crate::doc_id(&c.found.source, &c.found.document.id);
        let mut sel = Selected {
            key: c.key.clone(),
            norm_sha256: crate::norm_sha(&c.found.text),
            wave: wave.clone(),
            target: format!("{did}:{}", c.found.document.n),
            source: c.found.source.clone(),
            source_id: c.found.id.clone(),
            document: c.found.document.id.clone(),
            n: c.found.document.n,
            licence: c.found.licence.clone(),
            track: track.clone(),
            split: split.to_string(),
            why: c.found.why.clone(),
            status: "deferred".into(),
            reason: String::new(),
            intent: String::new(),
            task: String::new(),
        };
        match &bundles[&(c.found.source.clone(), c.found.document.id.clone())] {
            Err(e) => sel.reason = reason(&format!("bundle: {e}")),
            Ok((b, _)) => {
                let text = c.found.text.trim();
                // the sentence at its place, or (the document changed since) the same text elsewhere
                let s = b.sentences.iter().find(|s| s.n == c.found.document.n && s.text.trim() == text).or_else(|| b.sentences.iter().find(|s| s.text.trim() == text));
                match s {
                    None => sel.reason = "the sentence is not in its document's current bundle".into(),
                    Some(s) if s.conllu.trim().is_empty() => sel.reason = "the source has no tree for the sentence".into(),
                    Some(s) if s.prag.trim().is_empty() => sel.reason = "no pragmatics row for the sentence".into(),
                    Some(s) if s.text.len() > 4096 || s.text.contains(['\r', '\n', '\0']) => sel.reason = "sentence text over the Fish context bound".into(),
                    Some(s) => {
                        sel.n = s.n;
                        sel.target = format!("{did}:{}", s.n);
                        sel.status = "candidate".into();
                        a_trees.push(crate::with_id(crate::blocks(&s.conllu).first().map_or("", String::as_str), &sel.target));
                    }
                }
            }
        }
        targets.push(Target { sel, packet: None });
    }
    // The second producer: the deployed gparse on the same tokens.
    let b_trees = if a_trees.is_empty() { BTreeMap::new() } else { gparse_annotate(cfg, &a_trees, &dir.join("work"))? };
    let a_by: BTreeMap<String, String> = a_trees.iter().filter_map(|b| Some((crate::block_id(b)?.to_string(), b.clone()))).collect();
    for (t, (c, _, _)) in targets.iter_mut().zip(&chosen) {
        if t.sel.status != "candidate" {
            continue;
        }
        let (b, pin) = bundles[&(c.found.source.clone(), c.found.document.id.clone())].as_ref().unwrap();
        let did = crate::doc_id(&c.found.source, &c.found.document.id);
        let (Some(a), Some(g)) = (a_by.get(&t.sel.target), b_trees.get(&t.sel.target)) else {
            t.sel.status = "deferred".into();
            t.sel.reason = "gparse gave no tree for the sentence".into();
            continue;
        };
        let pos = b.sentences.iter().position(|s| s.n == t.sel.n).unwrap();
        let lo = pos.saturating_sub(fish.context);
        let window: Vec<&BundleSentence> = b.sentences[lo..=pos].iter().filter(|s| !s.text.trim().is_empty() && s.text.len() <= 4096 && !s.text.contains(['\r', '\n', '\0'])).collect();
        let context: Vec<ContextLine> = window
            .iter()
            .map(|s| ContextLine { id: format!("{did}:{}", s.n), text: s.text.clone(), speaker: None, referents: BTreeMap::new(), negation: Negation::Unknown, observation_scope: VoiceScope::Unknown, unresolved: vec![] })
            .collect();
        let ordinals: BTreeMap<String, u64> = window.iter().map(|s| (format!("{did}:{}", s.n), s.n as u64)).collect();
        let row = &b.sentences[pos].prag;
        let mut f: Vec<&str> = row.split('\t').collect();
        f[0] = &t.sel.target;
        let prag = format!("{}{}\n", prag::eval::Labeler::HEADER, f.join("\t"));
        let doc_sha = sha(b.sentences.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join("\n"));
        let task = Task {
            schema: 1,
            id: format!("t{}", &sha(&t.sel.target)[..32]),
            document: did.clone(),
            document_sha256: doc_sha,
            source_version: format!("v-{}", &sha(&b.version)[..32]),
            context_sha256: sha(serde_json::to_vec(&context)?),
            context,
            target_ids: vec![t.sel.target.clone()],
            dataset_use: if t.sel.split == "measure" { Use::Unknown } else { Use::Teaching },
            candidates: vec![
                Candidate { producer: "mova-en".into(), revision: sha(&b.model), ast_sha256: sha(a), ast: a.clone(), pragmatics_sha256: sha(&prag), pragmatics: prag.clone() },
                Candidate { producer: "mova-gparse".into(), revision: gp_rev.clone(), ast_sha256: sha(g), ast: g.clone(), pragmatics_sha256: sha(&prag), pragmatics: prag.clone() },
            ],
            witnesses_sha256: sha(NO_WITNESS),
            witnesses: NO_WITNESS.into(),
            witnesses_revision: sha("none: no open pragmatic witness run for hard cases"),
            agreement_revision: agreement.clone(),
        };
        t.packet = Some(PacketInput {
            schema: 1,
            ordinal: t.sel.n as u64,
            order_scope: did.clone(),
            task,
            context_ordinals: ordinals,
            source_pins: vec![pin.clone()],
            provenance: json!({"kind": "hard-case-lane", "wave": wave, "key": t.sel.key, "source": t.sel.source, "source_id": t.sel.source_id, "track": t.sel.track, "split": t.sel.split, "why": t.sel.why, "licence": t.sel.licence, "review_only": true, "training_authorized": false}),
        });
    }
    // 4. Pages: targets grouped by document, in order; packed page by page.
    targets.sort_by(|a, b| (a.sel.target.rsplit_once(':').map(|x| x.0), a.sel.n).cmp(&(b.sel.target.rsplit_once(':').map(|x| x.0), b.sel.n)));
    // two cases can name the same sentence (one text met twice); keep the first
    let mut seen = BTreeSet::new();
    for t in targets.iter_mut() {
        if !seen.insert(t.sel.target.clone()) {
            t.sel.status = "deferred".into();
            t.sel.reason = "another case of the wave names the same sentence".into();
            t.sel.target = format!("{}-dup-{}", t.sel.target, &t.sel.key[..8]);
            t.packet = None;
        }
    }
    let opts = PackOptions { max_targets: fish.max_targets, max_preceding_context: fish.context, allow_mosaic_artifacts: false, excluded_ids: BTreeSet::new(), excluded_text_sha256: held.1.clone() };
    let profile_pin = json!({"path": profile_path, "sha256": fish.profile.sha256});
    let mut pages: Vec<(String, PathBuf)> = vec![];
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for (k, chunk) in targets.chunks_mut(fish.page_size).enumerate() {
        let intent = format!("h{}-p{k:04}", &wave[5..]);
        let packets: Vec<PacketInput> = chunk.iter().filter_map(|t| t.packet.clone()).collect();
        let report = if packets.is_empty() { None } else { Some(mova_fish_batch::pack(packets, &profile, &opts)?) };
        let mut queue = String::new();
        let mut rejected: BTreeMap<String, String> = BTreeMap::new();
        if let Some(r) = &report {
            for rj in &r.rejections {
                for id in &rj.target_ids {
                    rejected.insert(id.clone(), reason(&rj.reason));
                }
            }
            for b in &r.batches {
                let tp = dir.join("tasks").join(format!("{}.json", b.task.id));
                write_new(&tp, &serde_json::to_vec(&b.task)?)?;
                let line = json!({"schema": 1, "id": b.task.id, "task": {"path": tp, "sha256": crate::file_sha(&tp)?}, "profile": profile_pin, "source_pins": b.source_pins, "provenance": {"kind": "hard-case-lane", "wave": wave, "intent": intent, "targets": b.target_ids, "pack": b.provenance}});
                queue += &(line.to_string() + "\n");
                for id in &b.target_ids {
                    if let Some(t) = chunk.iter_mut().find(|t| &t.sel.target == id) {
                        t.sel.status = "queued".into();
                        t.sel.task = b.task.id.clone();
                    }
                }
            }
        }
        let (mut deferred, mut rejected_rows) = (vec![], vec![]);
        for t in chunk.iter_mut() {
            t.sel.intent = intent.clone();
            if let Some(r) = rejected.get(&t.sel.target) {
                t.sel.status = "rejected".into();
                t.sel.reason = r.clone();
            }
            match t.sel.status.as_str() {
                "queued" => {}
                "rejected" => rejected_rows.push(json!({"source_id": t.sel.target, "reason": t.sel.reason})),
                _ => {
                    t.sel.status = "deferred".into();
                    if t.sel.reason.is_empty() {
                        t.sel.reason = "not packed".into();
                    }
                    deferred.push(json!({"source_id": t.sel.target, "reason": t.sel.reason}));
                }
            }
            *counts.entry(t.sel.status.clone()).or_default() += 1;
        }
        let ip = dir.join("intents").join(format!("{intent}.json"));
        write_new(&ip, &serde_json::to_vec(&json!({"schema": 1, "id": intent, "sources": chunk.iter().map(|t| t.sel.target.clone()).collect::<Vec<_>>()}))?)?;
        let qp = dir.join("queues").join(format!("{intent}.jsonl"));
        write_new(&qp, queue.as_bytes())?;
        let ready = json!({"schema": 1, "intent": {"path": ip, "sha256": crate::file_sha(&ip)?}, "queue": {"path": qp, "sha256": crate::file_sha(&qp)?}, "deferred": deferred, "rejected": rejected_rows});
        write_new(&dir.join("ready").join(format!("{intent}.json")), &serde_json::to_vec(&ready)?)?;
        pages.push((intent, ip));
    }
    // Plans and pager configurations, pages dealt round-robin.
    let pagers = fish.pagers.min(pages.len()).max(1);
    let mut configs = vec![];
    for p in 0..pagers {
        let name = format!("h{p:02}");
        let mine: Vec<Value> = pages.iter().enumerate().filter(|(i, _)| i % pagers == p).map(|(_, (_, ip))| Ok(json!({"path": ip, "sha256": crate::file_sha(ip)?}))).collect::<Result<_>>()?;
        let pp = dir.join("plans").join(format!("{name}.json"));
        write_new(&pp, &serde_json::to_vec(&json!({"schema": 1, "pages": mine}))?)?;
        let cp = dir.join(format!("{name}-config.json"));
        let pc = json!({"schema": 1, "root": dir.join("workers").join(&name), "plan": {"path": pp, "sha256": crate::file_sha(&pp)?}, "ready_root": dir.join("ready"), "fish_executable": {"path": fish.executable.path, "sha256": fish.executable.sha256}, "not_after_unix": fish.not_after_unix, "timeout_seconds": fish.timeout_seconds, "poll_seconds": fish.poll_seconds});
        write_new(&cp, &serde_json::to_vec_pretty(&pc)?)?;
        configs.push(cp);
    }
    let sel: String = targets.iter().map(|t| serde_json::to_string(&t.sel).unwrap() + "\n").collect();
    write_new(&dir.join("selection.jsonl"), sel.as_bytes())?;
    let w = json!({
        "schema": "mova-hard-lane-wave-v1", "wave": wave, "list": cfg.list, "list_sha256": crate::file_sha(&cfg.list)?,
        "profile": fish.profile, "fish_executable": fish.executable, "gparse_model_sha256": gp_rev, "pages": pages.len(), "pagers": configs,
        "targets": targets.len(), "counts": counts, "selection": summary, "bindings": mova_fish_batch::bindings(),
        "review_only": true, "training_authorized": false,
    });
    write_new(&dir.join("wave.json"), &serde_json::to_vec_pretty(&w)?)?;
    // The ledger last: a wave that failed above leaves its cases selectable again.
    hard_case_list::append(&lane.join("queued.jsonl"), &targets.iter().map(|t| serde_json::to_value(&t.sel)).collect::<std::result::Result<Vec<_>, _>>()?)?;
    Ok(w)
}

/// The waves and their pagers' progress (from the pagers' status snapshots, not liveness).
pub fn status(cfg: &Config) -> Result<Value> {
    let mut waves = vec![];
    if cfg.lane_root.exists() {
        let mut names: Vec<_> = std::fs::read_dir(&cfg.lane_root)?.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).filter(|n| n.starts_with("wave-")).collect();
        names.sort();
        for n in names {
            let d = cfg.lane_root.join(&n);
            let w = read_json(&d.join("wave.json")).unwrap_or(json!({"incomplete": true}));
            let mut pagers = vec![];
            for p in w["pagers"].as_array().cloned().unwrap_or_default() {
                let c = read_json(Path::new(p.as_str().unwrap_or("")))?;
                let s = read_json(&Path::new(c["root"].as_str().unwrap_or("")).join("status.json")).unwrap_or(json!({"state": "not started"}));
                pagers.push(json!({"config": p, "state": s["state"], "counts": s["counts"], "cursor": s["cursor"], "review_pending_targets": s["review_pending_targets"].as_array().map(Vec::len)}));
            }
            waves.push(json!({"wave": n, "targets": w["targets"], "counts": w["counts"], "pagers": pagers}));
        }
    }
    Ok(json!({"lane_root": cfg.lane_root, "waves": waves}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hard_case_list::{Document, Found, EN};

    fn cfg() -> Config {
        serde_json::from_value(json!({"schema": "mova-hard-cases-v1", "list": "/x/list", "lane_root": "/x/lane", "sources": {"wikipedia": {"command": ["/bin/true"], "pointer": ["article"]}}, "en_model": "/x/en.bin", "prag_silver": "/x/silver", "gparse": {"executable": "/x/gparse", "model_dir": "/x/m"}, "fish": null, "witness": null})).unwrap()
    }

    fn case(text: &str, why: &[&str], uas: Option<f64>, licence: &str, source: &str) -> Case {
        let found = Found { text: text.into(), source: source.into(), id: "wiki:A:1".into(), document: Document { id: format!("wiki:{text}"), n: 1 }, at: json!({"article": "A"}), why: why.iter().map(|x| x.to_string()).collect(), licence: licence.into(), model: "en.bin:x".into(), reader: "r".into(), draft_conllu: None, witness_uas: uas };
        Case { key: hard_case_list::key(text), found, views: BTreeMap::from([(EN.to_string(), hard_case_list::View { status: "open".into(), model: String::new() })]), times_found: 1 }
    }

    #[test]
    fn selection_takes_the_hardest_admissible_cases_once() {
        let c = cfg();
        let cases = [
            case("Easy one.", &["witness"], Some(0.79), "CC BY-SA 4.0", "wikipedia"),
            case("Hard one.", &["witness", "grammar"], Some(0.2), "CC BY-SA 4.0", "wikipedia"),
            case("Low witness.", &["witness"], Some(0.1), "CC BY-SA 4.0", "wikipedia"),
            case("Low  witness.", &["witness"], Some(0.1), "CC BY-SA 4.0", "wikipedia"),
            case("|- | rowspan=2 | x", &["witness"], Some(0.0), "CC BY-SA 4.0", "wikipedia"),
            case("A table cell", &["no_skeleton"], None, "CC BY 4.0", "wikipedia"),
            case("Non-commercial text.", &["grammar"], None, "CC BY-NC-SA 3.0", "wikipedia"),
            case("Elsewhere.", &["grammar"], None, "CC BY 4.0", "other-reader"),
            case("Held out.", &["grammar"], None, "CC BY 4.0", "wikipedia"),
            case("Already queued.", &["grammar"], None, "CC BY 4.0", "wikipedia"),
        ];
        let st: BTreeMap<String, Case> = cases.iter().map(|c| (c.key.clone(), c.clone())).collect();
        let held = (BTreeSet::from([sha("Held out.")]), BTreeSet::new());
        let done = vec![Selected { key: hard_case_list::key("Already queued."), norm_sha256: crate::norm_sha("Already queued."), wave: "wave-0001".into(), target: "hard:x:1".into(), source: "wikipedia".into(), source_id: "s".into(), document: "d".into(), n: 1, licence: String::new(), track: "shipped".into(), split: "train".into(), why: vec![], status: "queued".into(), reason: String::new(), intent: String::new(), task: String::new() }];
        let (sel, left) = select(&c, &st, &done, &held, 10);
        let texts: Vec<&str> = sel.iter().map(|x| x.0.found.text.as_str()).collect();
        // two judges first, then the lowest witness agreement; the whitespace twin once
        assert_eq!(texts.len(), 3);
        assert_eq!((texts[0], &texts[1][..3], texts[2]), ("Hard one.", "Low", "Easy one."));
        assert!(sel.iter().all(|x| x.1 == "internal"));
        for (k, n) in [("markup", 1), ("kinds outside the lane", 1), ("licence: measure only", 1), ("source not configured", 1), ("held out", 1), ("already in the lane", 1), ("same text as another case", 1)] {
            assert_eq!(left.get(k), Some(&n), "{k}: {left:?}");
        }
        // a cap takes the hardest
        let (sel, _) = select(&c, &st, &done, &held, 1);
        assert_eq!(sel[0].0.found.text, "Hard one.");
    }
}
