//! Epochs of texts (`mova-whale epoch CONFIG`). Per document and epoch N (any increasing number;
//! no epoch is special):
//!
//! 1. **Mova writes** (`compose::compose_epoch`): the description of the original from Mova's own
//!    reading, then what the document's neighbourhood says about its key terms — the document's
//!    own texts of all earlier epochs and the nearest documents' texts by the global world
//!    (`dossier::WorldConfig`), whole, nearest first. Every sentence is cited; the text closes with
//!    its references. It must reach `mova.words.min`; a shorter text is recorded (`mova_short`) and
//!    not registered, and nothing is asked for that document.
//! 2. The text is registered in the document's dossier (`dossier.rs`) as kind `mova`.
//! 3. **The Whale's own consultant writes** through its private terminal: one question whose
//!    prompt (a pinned template) lists the given sources by path — the original, Mova's text, the
//!    document's earlier texts and the nearest documents' key texts — numbered; the answer must
//!    cite only those numbers. Sources beyond the hard limits (`max_inputs`, `max_input_bytes`) are
//!    left out and journaled. A text citing an unknown number, citing nothing, or outside
//!    `sol.words` is refused: kept in the run root (`sol-answer.md`), never registered.
//! 4. The consultant's text is registered as kind `sol` (never public), and its sentences become
//!    claims followed to the given sources (`claims.rs`); optionally also registered in the
//!    paper's closed topic corpus (`mova-topic-corpus sol-import`, `sol-claims`).
//! 5. **Stop rule per document:** an epoch whose consultant text adds no sourced claim that no
//!    earlier epoch had and is not longer than the best earlier one marks the document
//!    `saturated`; later epochs skip it. Saturation is a record, not an error.
//!
//! Resumable at any epoch: `run_root/epoch-N/docs/<doc>/status.json` is written before every
//! step, so a registered Mova text is not written again and a question is never sent twice
//! (`sol_asking` without an answer becomes `uncertain`). Fail-fast: a terminal limit, timeout or
//! failure stops the whole run; nothing is retried. One driver per run root (`epoch.lock`), and
//! the Whale holds its terminal's `holder.lock` for the run, as a batch learner does.
use crate::claims::{self, Given};
use crate::compose::{self, Known, Neighbour};
use crate::dossier::{self, valid_doc, Citations, DossierConfig, NearDoc, NewText, Store, TextRow, WorldConfig};
use crate::gates::{near_duplicate, unquote, Guard, Params};
use crate::knowledge::{self as kn, Item as KItem};
use crate::paper::{open_licence, PaperSource, Words};
use crate::reader::{self, content, Reading};
use crate::terminal::{MovaTerminal, PrivateTerminal, Question, TerminalConfig, TerminalLease};
use crate::util::{error, file_hash, hash, now, private_dir, save, try_lease, write_sync};
use crate::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

pub const SCHEMA: &str = "mova-whale-epoch-v1";

/// Which documents an epoch covers, in relevance order (never alphabetical).
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "from", deny_unknown_fields)]
pub enum Documents {
    /// Explicit papers, in this order.
    #[serde(rename = "list")]
    List { items: Vec<PaperSource> },
    /// The papers a batch run assigned (`batches.jsonl`), in the batches' order and each batch's
    /// assignment order; optionally only those assigned to the given learner labels.
    #[serde(rename = "batches")]
    Batches {
        path: PathBuf,
        batches: Vec<u64>,
        #[serde(default)]
        whales: Vec<String>,
    },
    /// The batch selection over a pinned catalog (`batch::candidates`).
    #[serde(rename = "catalog")]
    Catalog {
        catalog: crate::batch::Catalog,
        #[serde(default)]
        cited_dir: Option<PathBuf>,
        #[serde(default)]
        include_uncited: bool,
        #[serde(default)]
        priority: Vec<String>,
        #[serde(default)]
        exclude: Vec<String>,
        #[serde(default)]
        extraction: crate::reader::ExtractionLimits,
        #[serde(default)]
        selection: crate::batch::Selection,
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MovaStep {
    pub words: Words,
    #[serde(default)]
    pub gates: Params,
    /// A Whale's state root whose retained knowledge explains terms (read only).
    #[serde(default)]
    pub knowledge: Option<PathBuf>,
    /// Neighbour sentences must speak of at least this many of the document's key terms.
    #[serde(default = "d_relevance")]
    pub min_relevance: usize,
}
fn d_relevance() -> usize {
    2
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pin {
    pub path: PathBuf,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SolStep {
    pub terminal: TerminalConfig,
    /// The prompt template: `{{SOURCES}}`, `{{MIN_WORDS}}` and `{{MAX_WORDS}}` exactly once;
    /// `{{TITLE}}`, `{{DOC}}`, `{{EPOCH}}`, `{{WHALE}}` optional.
    pub prompt: Pin,
    pub words: Words,
    /// Copy the given sources into the terminal's root (`inputs/<doc>/e<N>/`), which every
    /// terminal's sandbox may read; otherwise the prompt names the dossier paths.
    #[serde(default = "d_true")]
    pub stage_inputs: bool,
    /// Hard limits on what is handed over; what does not fit is left out and journaled.
    #[serde(default = "d_inputs")]
    pub max_inputs: usize,
    #[serde(default = "d_input_bytes")]
    pub max_input_bytes: u64,
    /// Nearest documents whose key text (latest consultant text, else original) is given.
    #[serde(default = "d_near")]
    pub neighbour_texts: usize,
    /// A claim is `sourced` when a cited source holds a sentence this similar (Jaccard).
    #[serde(default = "d_support")]
    pub min_support: f64,
}
fn d_true() -> bool {
    true
}
fn d_inputs() -> usize {
    64
}
fn d_input_bytes() -> u64 {
    8_000_000
}
fn d_near() -> usize {
    12
}
fn d_support() -> f64 {
    0.2
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EpochConfig {
    pub schema: String,
    pub whale: String,
    pub run_root: PathBuf,
    /// The epoch this run writes (1, 2, …); `--epoch N` on the command line overrides it.
    pub epoch: u64,
    pub documents: Documents,
    #[serde(default = "d_docs")]
    pub max_documents: usize,
    /// The dossier store (`files` or the world tool's `world`).
    pub dossier: DossierConfig,
    pub world: WorldConfig,
    pub mova: MovaStep,
    /// None: Mova only.
    #[serde(default)]
    pub sol: Option<SolStep>,
    /// Also register each consultant text in the paper's closed topic corpus and follow its
    /// claims there (`sol-import`, `sol-claims`).
    #[serde(default)]
    pub topic_corpus: Option<crate::topic::TopicCorpus>,
}
fn d_docs() -> usize {
    5
}

impl EpochConfig {
    pub fn load(p: &Path) -> Result<Self> {
        let c: EpochConfig = serde_json::from_value(crate::util::load(p, 1 << 20)?)?;
        c.validate()?;
        Ok(c)
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA || !crate::util::label(&self.whale) || !self.run_root.is_absolute() || self.epoch == 0 || !(1..=100_000).contains(&self.max_documents) {
            return Err(error("epoch config: schema, whale label, absolute run_root and dossier, epoch >= 1, max_documents 1..100000"));
        }
        let m = &self.mova;
        if m.words.min == 0 || m.words.min > m.words.max || m.words.max > 100_000 || !(1..=20).contains(&m.min_relevance) || m.knowledge.as_ref().is_some_and(|p| !p.is_absolute()) {
            return Err(error("epoch config: 0 < mova.words.min <= mova.words.max <= 100000, min_relevance 1..20, absolute knowledge"));
        }
        m.gates.validate()?;
        self.dossier.validate()?;
        self.world.validate()?;
        match &self.documents {
            Documents::List { items } if items.is_empty() || items.iter().any(|p| !p.path.is_absolute() || !crate::util::valid_hash(&p.sha256) || !dossier::valid_doc(&p.id)) => return Err(error("epoch config: documents.items need valid ids, absolute paths and SHA-256")),
            Documents::Batches { path, batches, .. } if !path.is_absolute() || batches.is_empty() => return Err(error("epoch config: documents.path absolute and at least one batch")),
            Documents::Catalog { catalog, cited_dir, .. } if !catalog.path.is_absolute() || !crate::util::valid_hash(&catalog.sha256) || cited_dir.as_ref().is_some_and(|d| !d.is_absolute()) => return Err(error("epoch config: a pinned absolute catalog")),
            _ => {}
        }
        if let Some(s) = &self.sol {
            if s.words.min == 0 || s.words.min > s.words.max || !(1..=1000).contains(&s.max_inputs) || s.max_input_bytes < 1000 || s.neighbour_texts > 1000 || !(0.0..=1.0).contains(&s.min_support) || !s.prompt.path.is_absolute() || !crate::util::valid_hash(&s.prompt.sha256) {
                return Err(error("epoch config: sol words, max_inputs 1..1000, max_input_bytes >= 1000, neighbour_texts <= 1000, min_support 0..1, a pinned absolute prompt"));
            }
            let p = s.terminal.load()?;
            if p.animal != self.whale {
                return Err(error(format!("epoch config: terminal profile belongs to {}, not {}", p.animal, self.whale)));
            }
        }
        if let Some(t) = &self.topic_corpus {
            t.validate()?;
        }
        Ok(())
    }
    pub fn epoch_dir(&self, epoch: u64) -> PathBuf {
        self.run_root.join(format!("epoch-{epoch}"))
    }
}

/// The documents of the epoch, in order.
pub fn documents(c: &EpochConfig) -> Result<(Vec<PaperSource>, Vec<Value>)> {
    match &c.documents {
        Documents::List { items } => Ok((items.iter().take(c.max_documents).cloned().collect(), vec![])),
        Documents::Batches { path, batches, whales } => {
            let mut out: Vec<PaperSource> = vec![];
            for l in std::fs::read_to_string(path)?.lines().filter(|l| !l.trim().is_empty()) {
                let v: Value = serde_json::from_str(l)?;
                if v["event"] != "opened" || !v["batch"].as_u64().is_some_and(|b| batches.contains(&b)) {
                    continue;
                }
                for a in v["assignments"].as_array().cloned().unwrap_or_default() {
                    let who = crate::util::legacy(&a, "whale").as_str().unwrap_or("");
                    if !whales.is_empty() && !whales.iter().any(|w| w == who) {
                        continue;
                    }
                    let p: PaperSource = serde_json::from_value(a["paper"].clone())?;
                    if !out.iter().any(|x| x.id == p.id) {
                        out.push(p);
                    }
                }
            }
            // The batches in the order they were opened, each in its assignment order.
            out.truncate(c.max_documents);
            Ok((out, vec![]))
        }
        Documents::Catalog { catalog, cited_dir, include_uncited, priority, exclude, extraction, selection } => {
            let pick = crate::batch::Pick { catalog, cited_dir: cited_dir.as_deref(), include_uncited: *include_uncited, priority, exclude, extraction, selection };
            crate::batch::candidates(&pick, &BTreeSet::new(), c.max_documents)
        }
    }
}

struct Journal(std::fs::File);
impl Journal {
    fn open(p: &Path) -> Result<Self> {
        Ok(Self(std::fs::OpenOptions::new().create(true).append(true).mode(0o600).open(p)?))
    }
    fn log(&mut self, v: Value) -> Result<()> {
        self.0.write_all((v.to_string() + "\n").as_bytes())?;
        self.0.sync_data()?;
        Ok(())
    }
}

fn safe(id: &str) -> String {
    id.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '.' { c } else { '_' }).take(160).collect()
}

/// States after which a document's epoch is not touched again.
pub const FINAL: [&str; 9] = ["done", "saturated", "mova_short", "mova_gates", "sol_rejected", "uncertain", "sol_limit", "sol_timeout", "sol_failed"];

fn read_status(p: &Path) -> Result<Option<Value>> {
    Ok(if p.exists() { Some(crate::util::load(p, 1 << 20)?) } else { None })
}

/// A text of the neighbourhood that Mova reads, with how to cite it.
struct Material {
    cite: String,
    title: String,
    relation: String,
    entry: String,
    text: String,
}

/// A text as the reader needs it: a title, then a part heading before the first paragraph (drafts
/// and our own texts open with their paragraphs right under the title).
fn readable(md: &str, title: &str) -> String {
    let mut lines = md.lines().peekable();
    let mut out = String::new();
    match lines.peek() {
        Some(l) if l.starts_with("# ") => {
            out.push_str(l);
            out.push_str("\n\n");
            lines.next();
        }
        _ => out.push_str(&format!("# {title}\n\n")),
    }
    let rest: Vec<&str> = lines.collect();
    if rest.iter().find(|l| !l.trim().is_empty()).is_some_and(|l| !l.starts_with("## ")) {
        out.push_str("## Text\n\n");
    }
    out.push_str(&rest.join("\n"));
    out
}

fn bounded_text(p: &Path) -> Option<String> {
    let b = crate::util::read_bounded(p, 16 << 20).ok()?;
    String::from_utf8(b).ok()
}

/// The document's own earlier texts (newest epoch first; consultant texts, older descriptions,
/// web descriptions, then Mova's own) — as the world gives them (distance 0) or else from the
/// dossier — then the texts of the nearest documents, nearest first: the texts the world names,
/// or, when it names none, those the dossier holds (latest per kind). The original is read
/// separately and is not material.
fn material(d: &Store, doc: &str, epoch: u64, near: &[NearDoc]) -> Result<Vec<Material>> {
    let mut out: Vec<Material> = vec![];
    let rank = |k: &str| ["sol", "site-old", "web", "mova"].iter().position(|x| *x == k).unwrap_or(9);
    let own_title = |kind: &str, e: u64, source: &str| -> String {
        match kind {
            "sol" => format!("the description of epoch {e}"),
            "mova" => format!("my text of epoch {e}"),
            "site-old" => "the older site description".into(),
            "web" if !source.is_empty() => format!("a web description ({source})"),
            k => format!("the {k} text"),
        }
    };
    let me = near.iter().find(|n| n.doc == doc && n.distance == 0);
    let mut own: Vec<(u64, String, String, String)> = vec![];
    match me {
        Some(n) => {
            for t in n.texts.iter().filter(|t| t.kind != "original" && t.epoch < epoch) {
                if let Some(x) = t.text() {
                    own.push((t.epoch, t.kind.clone(), t.id.clone(), x));
                }
            }
        }
        None => {
            for r in d.rows(doc)?.into_iter().filter(|r| r.epoch < epoch && r.kind != "original") {
                let x = d.read(&r)?;
                own.push((r.epoch, r.kind.clone(), r.id.clone(), x));
            }
        }
    }
    own.sort_by(|a, b| b.0.cmp(&a.0).then(rank(&a.1).cmp(&rank(&b.1))).then(a.2.cmp(&b.2)));
    for (e, kind, id, text) in own {
        let title = own_title(&kind, e, "");
        out.push(Material { entry: format!("{title} (dossier text {id})."), cite: id, title, relation: format!("own:{kind}"), text });
    }
    for n in near.iter().filter(|n| n.doc != doc) {
        let title = if n.title.is_empty() { n.doc.clone() } else { n.title.clone() };
        let rel = n.relations.first().cloned().unwrap_or_default();
        let licence = if n.licence.is_empty() { String::new() } else { format!("; licence: {}", n.licence) };
        let mut texts: Vec<(String, String, String)> = n.texts.iter().filter_map(|t| t.text().map(|x| (t.id.clone(), t.kind.clone(), x))).collect();
        if n.texts.is_empty() && valid_doc(&n.doc) {
            let rows = d.rows(&n.doc)?;
            for k in ["original", "sol", "site-old", "web", "mova"] {
                if let Some(r) = rows.iter().filter(|r| r.kind == k).max_by(|a, b| a.epoch.cmp(&b.epoch).then(a.id.cmp(&b.id))) {
                    texts.push((r.id.clone(), r.kind.clone(), d.read(r)?));
                }
            }
        }
        for (id, kind, text) in texts {
            out.push(Material { cite: id, entry: format!("{title} ({kind}; {}{licence}).", n.relations.join(", ")), title: title.clone(), relation: rel.clone(), text });
        }
    }
    Ok(out)
}

/// Mova's text with numbered citations and its references.
struct Cited {
    markdown: String,
    sources: Vec<(usize, String, String)>,
    uncited_items: usize,
    unresolved: usize,
}

fn cite_draft(d: &compose::Draft, title: &str, original: &TextRow, original_entry: &str, entries: &BTreeMap<String, String>, prov: &BTreeMap<String, Value>, whale: &str) -> Cited {
    let mut order: Vec<String> = vec![];
    let mut num = |id: &str| -> usize {
        match order.iter().position(|x| x == id) {
            Some(i) => i + 1,
            None => {
                order.push(id.to_string());
                order.len()
            }
        }
    };
    let mut paras: Vec<(Option<usize>, Vec<String>)> = vec![];
    let mut uncited = 0;
    for it in &d.items {
        let mut ids: Vec<String> = it.cites.clone();
        for k in &it.knowledge {
            let p = prov.get(k).cloned().unwrap_or(Value::Null);
            let id = match p["source"].as_str() {
                Some("topic-corpus") => p["doc"].as_str().map(str::to_string),
                Some("terminal") | Some("reread") => Some(format!("consult:{whale}")),
                _ => Some(original.id.clone()),
            };
            if let Some(id) = id.filter(|i| !ids.contains(i)) {
                ids.push(id);
            }
        }
        if ids.is_empty() || matches!(it.kind, "source" | "reading") {
            if !ids.contains(&original.id) {
                ids.insert(0, original.id.clone());
            }
        }
        if ids.is_empty() {
            uncited += 1;
        }
        let mut ns: Vec<usize> = ids.iter().map(|i| num(i)).collect();
        ns.sort_unstable();
        ns.dedup();
        let marks = ns.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", ");
        let t = it.text.trim();
        let (body, end) = match t.char_indices().last() {
            Some((i, ch)) if matches!(ch, '.' | '?' | '!') => (&t[..i], ch.to_string()),
            _ => (t, ".".to_string()),
        };
        let s = format!("{body} [{marks}]{end}");
        match paras.last_mut() {
            Some((sec, v)) if *sec == it.section => v.push(s),
            _ => paras.push((it.section, vec![s])),
        }
    }
    let mut md = format!("# {title}\n\n");
    for (_, v) in &paras {
        md.push_str(&v.join(" "));
        md.push_str("\n\n");
    }
    md.push_str("## References\n\n");
    let mut sources = vec![];
    let mut unresolved = 0;
    for (i, id) in order.iter().enumerate() {
        let entry = if id == &original.id {
            original_entry.to_string()
        } else if let Some(w) = id.strip_prefix("consult:") {
            format!("Notes from the consultation of {w} (not a published source; terms only).")
        } else if let Some(e) = entries.get(id) {
            e.clone()
        } else {
            unresolved += 1;
            "Unresolved source.".to_string()
        };
        md.push_str(&format!("[{}] {entry} `{id}`\n", i + 1));
        sources.push((i + 1, id.clone(), entry));
    }
    Cited { markdown: md, sources, uncited_items: uncited, unresolved }
}

/// The epoch text's gates: length, verbatim cap (against the paper and every neighbour text),
/// near duplicates, every sentence cited, structure share.
fn gates(d: &compose::Draft, guard: &Guard, words: &Words, uncited: usize, unresolved: usize) -> Value {
    let p = &guard.params;
    let sets: Vec<BTreeSet<String>> = d.items.iter().map(|i| content(&i.text).into_iter().collect()).collect();
    let mut dups = 0;
    for i in 0..sets.len() {
        for j in 0..i {
            if near_duplicate(&sets[i], &sets[j], p.near_duplicate) {
                dups += 1;
            }
        }
    }
    let verbatim_max = d.items.iter().map(|it| guard.verbatim(&if it.kind == "structure" { unquote(&it.text) } else { it.text.clone() })).max().unwrap_or(0);
    let structure: usize = d.items.iter().filter(|i| i.kind == "structure").map(|i| crate::contract::word_count(&i.text)).sum();
    let share = if d.words == 0 { 0.0 } else { structure as f64 / d.words as f64 };
    let checks = [("words", (words.min..=words.max).contains(&d.words)), ("verbatim", verbatim_max <= p.verbatim_max), ("near_duplicates", dups == 0), ("citations", uncited == 0 && unresolved == 0), ("structure_share", share <= p.structure_share_max)];
    let failed: Vec<&str> = checks.iter().filter(|c| !c.1).map(|c| c.0).collect();
    json!({"pass": failed.is_empty(), "failed": failed, "words": d.words, "words_min": words.min, "words_max": words.max, "verbatim_max": verbatim_max, "verbatim_cap": p.verbatim_max, "near_duplicates": dups, "uncited_items": uncited, "unresolved": unresolved, "structure_share": share})
}

/// Outcome of one document's epoch.
enum Step {
    Next(Value),
    /// The terminal failed: the run stops (fail-fast).
    Stop(Value),
}

struct Ctx<'a, 't> {
    c: &'a EpochConfig,
    epoch: u64,
    dossier: Store,
    j: Journal,
    live: bool,
    terminal: Option<&'t mut dyn PrivateTerminal>,
}

fn set_status(p: &Path, st: &mut Value, state: &str) -> Result<()> {
    st["state"] = json!(state);
    st["updated"] = json!(now());
    save(p, st)
}

fn doc_epoch(x: &mut Ctx<'_, '_>, paper: &PaperSource) -> Result<Step> {
    let (c, epoch) = (x.c, x.epoch);
    if !dossier::valid_doc(&paper.id) {
        return Ok(Step::Next(json!({"doc":paper.id,"state":"invalid_id"})));
    }
    let dir = c.epoch_dir(epoch).join("docs").join(safe(&paper.id));
    private_dir(&dir)?;
    let sp = dir.join("status.json");
    let state_dir = c.run_root.join("docs").join(safe(&paper.id));
    private_dir(&state_dir)?;
    let state_p = state_dir.join("state.json");
    let mut state = read_status(&state_p)?.unwrap_or_else(|| json!({"doc":paper.id,"epochs":{}}));
    if let Some(s) = state["saturated_at"].as_u64().filter(|s| *s < epoch) {
        x.j.log(json!({"event":"skip_saturated","doc":paper.id,"saturated_at":s,"epoch":epoch,"unix":now()}))?;
        return Ok(Step::Next(json!({"doc":paper.id,"state":"saturated_earlier","saturated_at":s})));
    }
    let mut st = read_status(&sp)?.unwrap_or_else(|| json!({"doc":paper.id,"epoch":epoch,"whale":c.whale,"state":"new","started":now()}));
    let s0 = st["state"].as_str().unwrap_or("new").to_string();
    if FINAL.contains(&s0.as_str()) {
        return Ok(Step::Next(st));
    }
    if s0 == "sol_asking" {
        // A question was sent (or may have been) and its answer never recorded: never re-sent.
        x.j.log(json!({"event":"uncertain","doc":paper.id,"epoch":epoch,"why":"a consultant question was sent without a recorded answer; it is never sent again","unix":now()}))?;
        set_status(&sp, &mut st, "uncertain")?;
        return Ok(Step::Next(st));
    }
    if file_hash(&paper.path)? != paper.sha256 {
        return Err(error(format!("{}: text hash mismatch; nothing written", paper.id)));
    }
    let original_text = std::fs::read_to_string(&paper.path)?;
    let open = open_licence(&paper.licence);
    let original = x.dossier.register(NewText { doc: &paper.id, epoch: 0, kind: "original", text: &original_text, source: paper.path.display().to_string(), licence: paper.licence.clone(), open, inputs: vec![], citations: Citations::default() })?;
    let original_entry = format!("{}. The original text (licence: {}).", if paper.title.is_empty() { &paper.id } else { &paper.title }, paper.licence);
    // 1-2. Mova's text.
    let mova_row: TextRow = if s0 == "mova" {
        serde_json::from_value(st["mova"]["row"].clone())?
    } else {
        let started = std::time::Instant::now();
        let near = c.world.neighbourhood(&paper.id, epoch)?;
        let mats = material(&x.dossier, &paper.id, epoch, &near)?;
        let r = reader::read_with(&original_text, c.mova.gates.verbatim_max)?;
        let mut readings: Vec<Reading> = vec![];
        let mut kept: Vec<&Material> = vec![];
        let mut unread = vec![];
        for m in &mats {
            match reader::read_with(&readable(&m.text, &m.title), c.mova.gates.verbatim_max) {
                Ok(rd) if !rd.sentences.is_empty() => {
                    readings.push(rd);
                    kept.push(m);
                }
                _ => unread.push(m.cite.clone()),
            }
        }
        let neighbours: Vec<Neighbour> = kept.iter().zip(readings.iter()).map(|(m, rd)| Neighbour { cite: m.cite.clone(), title: m.title.clone(), relation: m.relation.clone(), reading: rd }).collect();
        let (items, answers): (Vec<KItem>, Vec<String>) = match &c.mova.knowledge {
            Some(root) if root.join("knowledge").is_dir() => {
                let s = kn::Store::open(&root.join("knowledge"))?;
                let a = s.answers()?;
                (s.items, a)
            }
            _ => (vec![], vec![]),
        };
        let (known, _native) = Known::of(&r, &paper.id, &items);
        let mut guard_text = reader::repair(&original_text);
        // The verbatim cap holds against every text but Mova's own earlier texts (its own words).
        for m in kept.iter().filter(|m| m.relation != "own:mova") {
            guard_text.push_str("\n\n");
            guard_text.push_str(&m.text);
        }
        let guard = Guard::new(&guard_text, &answers, c.mova.gates.clone());
        let d = compose::compose_epoch(&r, &known, &guard, &neighbours, c.mova.words.max, c.mova.min_relevance);
        let prov: BTreeMap<String, Value> = items.iter().map(|k| (k.id.clone(), k.provenance.clone())).collect();
        let entries: BTreeMap<String, String> = kept.iter().map(|m| (m.cite.clone(), m.entry.clone())).collect();
        let title = format!("{} — Mova's text, epoch {epoch}", if paper.title.is_empty() { &paper.id } else { &paper.title });
        let cited = cite_draft(&d, &title, &original, &original_entry, &entries, &prov, &c.whale);
        let g = gates(&d, &guard, &c.mova.words, cited.uncited_items, cited.unresolved);
        let ms = started.elapsed().as_millis() as u64;
        save(&dir.join("draft.json"), &json!({"writer":compose::EPOCH_WRITER,"items":d.items,"available_words":d.available_words}))?;
        save(&dir.join("gates.json"), &g)?;
        write_sync(&dir.join("mova.md"), cited.markdown.as_bytes())?;
        let by_kind = |k: &str| d.items.iter().filter(|i| i.kind == k).map(|i| crate::contract::word_count(&i.text)).sum::<usize>();
        let world_cites: BTreeSet<&String> = d.items.iter().flat_map(|i| i.cites.iter()).collect();
        let m = json!({"words":d.words,"paper_words":by_kind("source")+by_kind("reading")+by_kind("explain"),"world_words":by_kind("world"),"structure_words":by_kind("structure"),"neighbour_texts":kept.len(),"neighbour_texts_cited":world_cites.len(),"unread":unread,"near_documents":near.len(),"sources":cited.sources.len(),"sentences":d.items.len(),"uncited":cited.uncited_items,"unresolved":cited.unresolved,"ms":ms,"sha256":hash(&d.text)});
        st["mova"] = json!({"metrics": m, "gates": g});
        x.j.log(json!({"event":"mova","doc":paper.id,"epoch":epoch,"metrics":m,"gates":g["failed"],"unix":now()}))?;
        if g["pass"] != true {
            let short = g["failed"].as_array().is_some_and(|f| f.iter().any(|v| v == "words"));
            set_status(&sp, &mut st, if short { "mova_short" } else { "mova_gates" })?;
            return Ok(Step::Next(st));
        }
        let inputs: Vec<String> = cited.sources.iter().map(|s| s.1.clone()).collect();
        let row = x.dossier.register(NewText { doc: &paper.id, epoch, kind: "mova", text: &cited.markdown, source: format!("mova-whale epoch ({}, {})", compose::EPOCH_WRITER, c.whale), licence: "internal (Mova's text)".into(), open: false, inputs, citations: Citations { count: cited.sources.len(), unresolved: cited.unresolved } })?;
        st["mova"]["row"] = serde_json::to_value(&row)?;
        set_status(&sp, &mut st, "mova")?;
        row
    };
    // 3-4. The consultant's text.
    let Some(sol) = c.sol.as_ref().filter(|_| x.live) else {
        return Ok(Step::Next(st));
    };
    let near = c.world.neighbourhood(&paper.id, epoch)?;
    let (given, left_out) = sol_inputs(sol, &x.dossier, &paper.id, epoch, &original, &mova_row, &near)?;
    let staged = stage(sol, &paper.id, epoch, given)?;
    let prompt = render_prompt(sol, c, paper, epoch, &staged)?;
    write_sync(&dir.join("sol-prompt.md"), prompt.as_bytes())?;
    save(&dir.join("sol-inputs.json"), &json!({"given":staged,"left_out":left_out}))?;
    st["sol"] = json!({"given": staged.len(), "left_out": left_out.len(), "prompt_sha256": hash(&prompt)});
    // Journaled before the question goes out: a crash after this point never re-sends it.
    set_status(&sp, &mut st, "sol_asking")?;
    x.j.log(json!({"event":"sol_asking","doc":paper.id,"epoch":epoch,"given":staged.len(),"left_out":left_out,"unix":now()}))?;
    let q = Question { id: format!("epoch-{epoch}-{}", safe(&paper.id)), kind: "article".into(), subject: format!("epoch:{epoch}:{}", paper.id), text: prompt, context: vec![], items: vec![] };
    let t = x.terminal.as_mut().ok_or_else(|| error("no terminal"))?;
    let a = match t.ask(&q) {
        Ok(a) => a,
        Err(e) => {
            st["sol"]["error"] = json!({"kind": e.kind(), "reason": e.reason()});
            set_status(&sp, &mut st, &format!("sol_{}", e.kind()))?;
            x.j.log(json!({"event":"sol_stop","doc":paper.id,"epoch":epoch,"kind":e.kind(),"reason":e.reason(),"unix":now()}))?;
            return Ok(Step::Stop(st));
        }
    };
    write_sync(&dir.join("sol-answer.md"), a.text.as_bytes())?;
    let words = crate::contract::word_count(&a.text);
    st["sol"]["answer"] = json!({"words": words, "elapsed_ms": a.elapsed_ms, "terminal_question": a.terminal_id, "sha256": hash(&a.text)});
    let checked = claims::check(&a.text, &staged);
    let refusal = match &checked {
        Err(r) => Some(r.reason()),
        Ok(_) if !(sol.words.min..=sol.words.max).contains(&words) => Some(format!("{words} words, outside {}..={}", sol.words.min, sol.words.max)),
        Ok(_) => None,
    };
    if let Some(why) = refusal {
        st["sol"]["refused"] = json!(why);
        set_status(&sp, &mut st, "sol_rejected")?;
        x.j.log(json!({"event":"sol_rejected","doc":paper.id,"epoch":epoch,"why":why,"unix":now()}))?;
        return Ok(Step::Next(st));
    }
    let (body, _) = checked.expect("checked");
    let cited_numbers: BTreeSet<usize> = body.iter().flat_map(|(_, ns)| ns.iter().copied()).collect();
    let row = x.dossier.register(NewText { doc: &paper.id, epoch, kind: "sol", text: &a.text, source: format!("terminal of {} ({})", c.whale, a.terminal_id.clone().unwrap_or_default()), licence: "internal (consultant text)".into(), open: false, inputs: staged.iter().map(|g| g.id.clone()).collect(), citations: Citations { count: cited_numbers.len(), unresolved: 0 } })?;
    let texts: BTreeMap<usize, String> = staged.iter().filter_map(|g| bounded_text(&g.path).map(|t| (g.n, t))).collect();
    let cl = claims::claims(&format!("e{epoch}"), &body, &staged, &texts, sol.min_support);
    write_sync(&dir.join("claims.jsonl"), cl.iter().map(|x| serde_json::to_string(x).unwrap_or_default() + "\n").collect::<String>().as_bytes())?;
    let mut by: BTreeMap<&str, usize> = BTreeMap::new();
    for k in &cl {
        *by.entry(k.status.as_str()).or_default() += 1;
    }
    let cited_share = if cl.is_empty() { 0.0 } else { cl.iter().filter(|k| k.status != "uncited").count() as f64 / cl.len() as f64 };
    st["sol"]["row"] = serde_json::to_value(&row)?;
    st["sol"]["claims"] = json!({"count": cl.len(), "by_status": by, "cited_share": cited_share, "sources_cited": cited_numbers.len(), "sources_given": staged.len()});
    if let Some(tc) = &c.topic_corpus {
        let draft = format!("epoch-{epoch}-{}", safe(&paper.id));
        let f = dir.join("sol-answer.md").display().to_string();
        let imp = tc.run("sol-import", &[&paper.id, &draft, &f]);
        let cls = if imp["ok"] == true { tc.run("sol-claims", &[&paper.id, &draft]) } else { json!({"ok":false,"skipped":"import failed"}) };
        x.j.log(json!({"event":"topic_corpus","doc":paper.id,"epoch":epoch,"import":imp,"claims":cls,"unix":now()}))?;
    }
    // 5. The stop rule.
    let keys_p = state_dir.join("claims.keys");
    let seen: BTreeSet<String> = if keys_p.exists() { std::fs::read_to_string(&keys_p)?.lines().map(String::from).collect() } else { BTreeSet::new() };
    let fresh: BTreeSet<String> = cl.iter().filter(|k| k.status == "sourced").map(|k| hash(claims::claim_key(&k.text).as_bytes())).filter(|h| !seen.contains(h)).collect();
    let best = state["best_sol_words"].as_u64().unwrap_or(0) as usize;
    let saturated = state["epochs"].as_object().is_some_and(|o| !o.is_empty()) && fresh.is_empty() && words <= best;
    let mut f = std::fs::OpenOptions::new().create(true).append(true).mode(0o600).open(&keys_p)?;
    for h in &fresh {
        f.write_all((h.clone() + "\n").as_bytes())?;
    }
    f.sync_data()?;
    state["epochs"][epoch.to_string()] = json!({"mova_words": mova_row.words, "sol_words": words, "new_sourced_claims": fresh.len()});
    state["best_sol_words"] = json!(best.max(words));
    if saturated {
        state["saturated_at"] = json!(epoch);
    }
    save(&state_p, &state)?;
    st["sol"]["new_sourced_claims"] = json!(fresh.len());
    set_status(&sp, &mut st, if saturated { "saturated" } else { "done" })?;
    x.j.log(json!({"event":"sol","doc":paper.id,"epoch":epoch,"words":words,"claims":st["sol"]["claims"],"new_sourced_claims":fresh.len(),"saturated":saturated,"unix":now()}))?;
    Ok(Step::Next(st))
}

/// A source for the consultant: a plain file, or a text to stage.
struct Input {
    id: String,
    kind: String,
    file: Option<PathBuf>,
    text: Option<String>,
}
impl Input {
    fn bytes(&self) -> u64 {
        match (&self.text, &self.file) {
            (Some(t), _) => t.len() as u64,
            (None, Some(f)) => std::fs::metadata(f).map(|m| m.len()).unwrap_or(u64::MAX),
            _ => u64::MAX,
        }
    }
}

fn row_input(d: &Store, r: &TextRow, kind: &str) -> Result<Input> {
    Ok(match d.file(r) {
        Some(f) => Input { id: r.id.clone(), kind: kind.into(), file: Some(f), text: None },
        None => Input { id: r.id.clone(), kind: kind.into(), file: None, text: Some(d.read(r)?) },
    })
}

/// The sources handed to the consultant, numbered: the original, this epoch's Mova text, the
/// document's earlier texts (newest first), then the nearest documents' key texts (their latest
/// consultant text, else their first text). Only hard limits cut the list (`max_inputs`,
/// `max_input_bytes`); what is left out is returned for the journal.
fn sol_inputs(s: &SolStep, d: &Store, doc: &str, epoch: u64, original: &TextRow, mova: &TextRow, near: &[NearDoc]) -> Result<(Vec<Input>, Vec<Value>)> {
    let mut cand: Vec<Input> = vec![row_input(d, original, "original")?, row_input(d, mova, "mova")?];
    let rank = |k: &str| ["sol", "site-old", "web", "mova"].iter().position(|x| *x == k).unwrap_or(9);
    let mut own: Vec<TextRow> = d.rows(doc)?.into_iter().filter(|r| r.epoch < epoch && r.kind != "original").collect();
    own.sort_by(|a, b| b.epoch.cmp(&a.epoch).then(rank(&a.kind).cmp(&rank(&b.kind))).then(a.id.cmp(&b.id)));
    for r in own {
        cand.push(row_input(d, &r, &r.kind)?);
    }
    for n in near.iter().filter(|n| n.doc != doc).take(s.neighbour_texts) {
        let key = n.texts.iter().filter(|t| t.kind == "sol").max_by(|a, b| a.epoch.cmp(&b.epoch).then(a.id.cmp(&b.id))).or_else(|| n.texts.first());
        match key {
            Some(t) => cand.push(Input { id: t.id.clone(), kind: format!("neighbour {}", if t.kind.is_empty() { "text" } else { &t.kind }), file: if t.content.is_none() { t.path.clone() } else { None }, text: t.content.clone() }),
            None => {
                let rows: Vec<TextRow> = if valid_doc(&n.doc) { d.rows(&n.doc)? } else { vec![] };
                if let Some(r) = rows.iter().filter(|r| r.kind == "sol").max_by(|a, b| a.epoch.cmp(&b.epoch).then(a.id.cmp(&b.id))).or_else(|| rows.iter().find(|r| r.kind == "original")) {
                    cand.push(row_input(d, r, &format!("neighbour {}", r.kind))?);
                }
            }
        }
    }
    let mut given = vec![];
    let mut left = vec![];
    let mut bytes = 0u64;
    for c in cand {
        let size = c.bytes();
        let required = given.len() < 2;
        if size == u64::MAX {
            left.push(json!({"id": c.id, "kind": c.kind, "why": "unreadable"}));
            continue;
        }
        if !required && (given.len() >= s.max_inputs || bytes.saturating_add(size) > s.max_input_bytes) {
            left.push(json!({"id": c.id, "kind": c.kind, "bytes": size, "why": if given.len() >= s.max_inputs { "max_inputs" } else { "max_input_bytes" }}));
            continue;
        }
        bytes += size;
        given.push(c);
    }
    Ok((given, left))
}

/// Number the sources and put each where the terminal reads it: texts (and, with `stage_inputs`,
/// files too) are written into the terminal's root (`inputs/<doc>/e<N>/`); the prompt names these
/// paths.
fn stage(s: &SolStep, doc: &str, epoch: u64, inputs: Vec<Input>) -> Result<Vec<Given>> {
    let p = s.terminal.load()?;
    let dir = p.root.join("inputs").join(safe(doc)).join(format!("e{epoch}"));
    let mut out = vec![];
    for (i, inp) in inputs.into_iter().enumerate() {
        let n = i + 1;
        let path = match (&inp.file, &inp.text) {
            (Some(f), None) if !s.stage_inputs => f.clone(),
            _ => {
                private_dir(&dir)?;
                let kind: String = inp.kind.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
                let to = dir.join(format!("{n:02}-{kind}.md"));
                let b = match (&inp.text, &inp.file) {
                    (Some(t), _) => t.as_bytes().to_vec(),
                    (None, Some(f)) => crate::util::read_bounded(f, 64 << 20)?,
                    _ => vec![],
                };
                if !to.exists() || std::fs::read(&to)? != b {
                    write_sync(&to, &b)?;
                }
                to
            }
        };
        out.push(Given { n, id: inp.id, kind: inp.kind, path });
    }
    Ok(out)
}

fn render_prompt(s: &SolStep, c: &EpochConfig, paper: &PaperSource, epoch: u64, given: &[Given]) -> Result<String> {
    if file_hash(&s.prompt.path)? != s.prompt.sha256 {
        return Err(error("sol prompt template hash mismatch; nothing asked"));
    }
    let t = std::fs::read_to_string(&s.prompt.path)?;
    for slot in ["{{SOURCES}}", "{{MIN_WORDS}}", "{{MAX_WORDS}}"] {
        if t.matches(slot).count() != 1 {
            return Err(error(format!("sol prompt: {slot} must occur exactly once")));
        }
    }
    let sources: String = given.iter().map(|g| format!("[{}] {} — `{}` — {}\n", g.n, g.kind, g.id, g.path.display())).collect();
    let title = if paper.title.is_empty() { paper.id.clone() } else { paper.title.clone() };
    // Slots are replaced in the template only, longest first, never inside substituted text.
    let mut out = String::new();
    let mut rest = t.as_str();
    let slots: [(&str, String); 7] = [("{{SOURCES}}", sources), ("{{MIN_WORDS}}", s.words.min.to_string()), ("{{MAX_WORDS}}", s.words.max.to_string()), ("{{TITLE}}", title), ("{{DOC}}", paper.id.clone()), ("{{EPOCH}}", epoch.to_string()), ("{{WHALE}}", c.whale.clone())];
    while let Some(i) = rest.find("{{") {
        out.push_str(&rest[..i]);
        let tail = &rest[i..];
        match slots.iter().find(|(k, _)| tail.starts_with(k)) {
            Some((k, v)) => {
                out.push_str(v);
                rest = &tail[k.len()..];
            }
            None => {
                out.push_str("{{");
                rest = &tail[2..];
            }
        }
    }
    out.push_str(rest);
    let max = s.terminal.load()?.question_max_bytes;
    if out.len() > max {
        return Err(error(format!("sol prompt is {} bytes, over the terminal's {max}", out.len())));
    }
    Ok(out)
}

/// Run one epoch over the configured documents. `live`: ask the consultant (needs `sol`).
pub fn run(c: &EpochConfig, epoch: u64, only: Option<&str>, t: Option<&mut dyn PrivateTerminal>) -> Result<Value> {
    c.validate()?;
    if epoch == 0 {
        return Err(error("epoch must be >= 1"));
    }
    private_dir(&c.run_root)?;
    let _l = try_lease(&c.run_root.join("epoch.lock"), true)?.ok_or_else(|| error("another epoch driver holds this run root (epoch.lock)"))?;
    private_dir(&c.epoch_dir(epoch))?;
    if let Some(t) = &t {
        if t.animal() != c.whale {
            return Err(error(format!("the terminal belongs to {}, not to {}", t.animal(), c.whale)));
        }
    }
    let live = t.is_some() && c.sol.is_some();
    let (mut docs, refused) = documents(c)?;
    if let Some(o) = only {
        docs.retain(|d| d.id == o);
        if docs.is_empty() {
            return Err(error(format!("{o} is not among this epoch's documents")));
        }
    }
    let mut x = Ctx { c, epoch, dossier: c.dossier.open(&c.epoch_dir(epoch).join("scratch")), j: Journal::open(&c.epoch_dir(epoch).join("journal.jsonl"))?, live, terminal: t };
    for r in &refused {
        x.j.log(json!({"event":"refused","detail":r,"unix":now()}))?;
    }
    x.j.log(json!({"event":"start","epoch":epoch,"whale":c.whale,"documents":docs.iter().map(|d| d.id.clone()).collect::<Vec<_>>(),"live":live,"writer":compose::EPOCH_WRITER,"unix":now()}))?;
    let mut out = vec![];
    let mut stopped = Value::Null;
    for p in &docs {
        match doc_epoch(&mut x, p)? {
            Step::Next(v) => out.push(v),
            Step::Stop(v) => {
                stopped = json!({"doc": p.id, "status": v["state"], "error": v["sol"]["error"]});
                out.push(v);
                break;
            }
        }
    }
    let summary = json!({"epoch": epoch, "whale": c.whale, "live": live, "documents": out.iter().map(|v| json!({"doc": v["doc"], "state": v["state"], "mova_words": v["mova"]["metrics"]["words"], "sol_words": v["sol"]["answer"]["words"], "claims": v["sol"]["claims"]})).collect::<Vec<_>>(), "stopped": stopped});
    x.j.log(json!({"event":"end","summary":summary,"unix":now()}))?;
    Ok(summary)
}

/// The CLI's run: the Whale holds its terminal for the whole run when live.
pub fn run_cli(c: &EpochConfig, epoch: u64, only: Option<&str>, live: bool) -> Result<Value> {
    match (&c.sol, live) {
        (Some(s), true) => {
            let mut t = MovaTerminal::open(&s.terminal)?;
            let _lease = TerminalLease::take(&t.profile)?;
            run(c, epoch, only, Some(&mut t))
        }
        (None, true) => Err(error("this epoch config has no `sol` step; run with --no-terminal")),
        _ => run(c, epoch, only, None),
    }
}

/// Every document's state in an epoch (read-only).
pub fn status(c: &EpochConfig, epoch: u64) -> Result<Value> {
    let (docs, _) = documents(c)?;
    let mut v = vec![];
    for p in &docs {
        let st = read_status(&c.epoch_dir(epoch).join("docs").join(safe(&p.id)).join("status.json"))?;
        let ds = read_status(&c.run_root.join("docs").join(safe(&p.id)).join("state.json"))?;
        v.push(json!({"doc": p.id, "state": st.as_ref().map_or(json!("new"), |s| s["state"].clone()), "mova_words": st.as_ref().map(|s| s["mova"]["metrics"]["words"].clone()), "sol_words": st.as_ref().map(|s| s["sol"]["answer"]["words"].clone()), "saturated_at": ds.as_ref().map(|s| s["saturated_at"].clone())}));
    }
    Ok(json!({"epoch": epoch, "documents": v}))
}

#[cfg(test)]
mod tests;
