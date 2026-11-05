//! Nautilus as one of Mova's animals: its configured layers, the work it does on them, and its English.
//!
//! A configuration (`nautilus.example.json`) names each layer it keeps: its source (`commands`, `wikipedia`, `course`
//! or `layer`), where the source is, the pupils' dictionary it takes meanings from, where the layer is written, how
//! many words a draft holds, the licence line of the layer, the licence class of its inputs and whether the layer
//! ships (is part of the repository or a release). `make` drafts a layer natively; `stats` reads one back. A `layer`
//! source names an existing layer file that Nautilus improves through review-only proposals and never writes.
//!
//! **Licences.** `licence_class` is one of [`LICENCE_CLASSES`]; `wikipedia` is always `cc-by-sa`. A shipped layer
//! (`shipped: true`) is refused with inputs that are `cc-by-sa`, `nc` or `research-only`, and also when a lesson or the
//! layer file it reads declares CC BY-SA in its header (checked when the layer is made or worked).
//!
//! In English, Nautilus takes arbitrary prompts through Mova's native deputy (`mova_octopus::Deputy`) with its own
//! hands (handler `nautilus`, operations `draft` and `stats`, the layer named in the trusted parameters) and answers
//! with sentences realized from the structured result (`mova_octopus::realize`). It never calls a teacher: a
//! teacher-written entry is a later step behind the operator's word (README).
use crate::layer::{self, Source};
use crate::sources;
use anyhow::{Context, Result, bail, ensure};
use mova_octopus::realize::{noun_count, paragraph, sentence};
use mova_octopus::{Action, CapabilityHandler, Deputy, Report};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The operations of the `nautilus` handler.
pub const OPERATIONS: [&str; 2] = ["draft", "stats"];

/// The licence classes of a layer's inputs.
pub const LICENCE_CLASSES: [&str; 7] = ["ours", "public-domain", "cc0", "cc-by", "cc-by-sa", "nc", "research-only"];
/// The classes that never go into a shipped layer.
pub const NOT_SHIPPED: [&str; 3] = ["cc-by-sa", "nc", "research-only"];
pub const SOURCES: [&str; 5] = ["commands", "wikipedia", "wikipedia-examples", "course", "layer"];
/// The sources whose inputs are Wikipedia (CC BY-SA 4.0 whatever is written).
pub const WIKIPEDIA: [&str; 2] = ["wikipedia", "wikipedia-examples"];

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayerConfig {
    /// `commands`, `wikipedia`, `wikipedia-examples`, `course` or `layer`
    pub source: String,
    /// the layer file written by `make` (not for `layer`, which writes no layer)
    #[serde(default)]
    pub out: Option<PathBuf>,
    /// the pupils' dictionary (`world/data/dictionary-improved.tsv`)
    pub base: PathBuf,
    /// commands: the course folder
    #[serde(default)]
    pub course: Option<PathBuf>,
    /// commands: the command corpus folder; wikipedia: the corpus root; wikipedia-examples: the large corpus's root
    #[serde(default)]
    pub corpus: Option<PathBuf>,
    /// commands: Mova's model (`en.bin`)
    #[serde(default)]
    pub model: Option<PathBuf>,
    /// wikipedia: read at most this many articles (name order)
    #[serde(default)]
    pub articles: Option<usize>,
    /// course: the course folders (`courses/<course>` of the school data, `$MOVA_DATA/school`)
    #[serde(default)]
    pub courses: Vec<PathBuf>,
    /// layer: the existing layer file improved through proposals (read only)
    #[serde(default)]
    pub file: Option<PathBuf>,
    /// layer: its format, one of `improve::FORMATS` (the proposals are written in it)
    #[serde(default)]
    pub format: Option<String>,
    /// the licence class of the inputs (`LICENCE_CLASSES`); `wikipedia` is `cc-by-sa` whatever is written,
    /// `commands` defaults to `ours`; `course` and `layer` must name it
    #[serde(default)]
    pub licence_class: Option<String>,
    /// the layer ships (repository or release): refused with CC BY-SA, NC or research-only inputs
    #[serde(default)]
    pub shipped: bool,
    /// wikipedia-examples: the `wikicorpus` binary (its `examples-batch`)
    #[serde(default)]
    pub wikicorpus: Option<PathBuf>,
    /// wikipedia-examples: examples fetched per word (1..100; default 100)
    #[serde(default)]
    pub per_word: Option<usize>,
    /// wikipedia-examples: the sampler's seed (default 1)
    #[serde(default)]
    pub seed: Option<u64>,
    /// wikipedia-examples: articles past tier 1 read lazily per word when tier 1 has too few (default 0)
    #[serde(default)]
    pub deep: Option<u64>,
    /// wikipedia-examples: the sampler's threads (default 4)
    #[serde(default)]
    pub jobs: Option<usize>,
    /// wikipedia-examples: the fewest tier-1 occurrences of a word (default 20)
    #[serde(default)]
    pub min_count: Option<u64>,
    /// wikipedia-examples: earlier layer files whose words are left out
    #[serde(default)]
    pub skip: Vec<PathBuf>,
    /// drafts: Mova may settle a word natively (default true); false asks every word that has examples (for a layer
    /// whose examples show different senses on purpose)
    #[serde(default)]
    pub native_picks: Option<bool>,
    /// wikipedia-examples: the examples file (default `<out>.examples.jsonl`)
    #[serde(default)]
    pub examples: Option<PathBuf>,
    /// drafts only: how many words (1..5000; wikipedia-examples 1..50000); ignored by `layer`
    #[serde(default = "d_top")]
    pub top: usize,
    /// the licence line written into the layer
    pub licence: String,
    /// the dictionary's name in English answers ("the commands dictionary")
    pub name: String,
}

fn d_top() -> usize {
    80
}

impl LayerConfig {
    /// The licence class of the inputs, as enforced.
    pub fn class(&self) -> &str {
        match (self.source.as_str(), self.licence_class.as_deref()) {
            (s, _) if WIKIPEDIA.contains(&s) => "cc-by-sa",
            (_, Some(c)) => c,
            ("commands", None) => "ours",
            _ => "",
        }
    }
    /// The layer file a teacher works on: the draft (`out`) or the improved file (`file`).
    pub fn rows_file(&self) -> Option<&Path> {
        if self.source == "layer" { self.file.as_deref() } else { self.out.as_deref() }
    }
    /// Paths the loop never writes under: the course folders and the improved file's folder.
    pub fn frozen(&self) -> Vec<PathBuf> {
        let mut v: Vec<PathBuf> = self.courses.clone();
        v.extend(self.course.clone());
        v.extend(self.file.as_ref().and_then(|f| f.parent()).map(Path::to_path_buf));
        v
    }
    /// Check one layer's configuration (paths, source fields, licence).
    pub fn check(&self, k: &str) -> Result<()> {
        ensure!(SOURCES.contains(&self.source.as_str()), "{k}: unknown source {}", self.source);
        let max_top = if self.source == "wikipedia-examples" { 50_000 } else { 5000 };
        ensure!(self.top > 0 && self.top <= max_top, "{k}: top is 1..{max_top}");
        ensure!(self.base.is_absolute() && self.out.as_ref().is_none_or(|o| o.is_absolute()), "{k}: absolute paths required");
        match self.source.as_str() {
            "commands" => ensure!(self.course.is_some() && self.model.is_some(), "{k}: commands needs course and model"),
            "wikipedia" => {
                ensure!(self.corpus.is_some(), "{k}: wikipedia needs corpus");
                ensure!(self.licence_class.as_deref().is_none_or(|c| c == "cc-by-sa"), "{k}: the Wikipedia corpus is CC BY-SA");
            }
            "wikipedia-examples" => {
                ensure!(self.corpus.is_some() && self.wikicorpus.is_some(), "{k}: wikipedia-examples needs corpus and wikicorpus");
                ensure!(self.licence_class.as_deref().is_none_or(|c| c == "cc-by-sa"), "{k}: the Wikipedia corpus is CC BY-SA");
                ensure!(self.per_word.is_none_or(|n| (1..=crate::sampling::PER_WORD_MAX).contains(&n)), "{k}: per_word is 1..{}", crate::sampling::PER_WORD_MAX);
                ensure!(self.skip.iter().chain(self.examples.iter()).all(|p| p.is_absolute()), "{k}: absolute paths required");
            }
            "course" => ensure!(
                !self.courses.is_empty() && self.courses.iter().all(|c| c.is_absolute()) && self.model.is_some(),
                "{k}: course needs absolute course folders and model"
            ),
            _ => {
                ensure!(self.file.as_ref().is_some_and(|f| f.is_absolute()), "{k}: layer needs an absolute file");
                ensure!(self.format.as_deref().is_some_and(|f| crate::improve::FORMATS.contains(&f)), "{k}: layer needs a format: {}", crate::improve::FORMATS.join(", "));
                ensure!(self.out.is_none(), "{k}: a layer source writes no layer; proposals go to the work state");
            }
        }
        if self.source != "layer" {
            ensure!(self.out.is_some(), "{k}: {} needs out", self.source);
        }
        ensure!(LICENCE_CLASSES.contains(&self.class()), "{k}: licence_class is one of {}", LICENCE_CLASSES.join(", "));
        ensure!(!(self.shipped && NOT_SHIPPED.contains(&self.class())), "{k}: a shipped layer never takes {} inputs", self.class());
        // a share-alike layer and its examples are written outside every Git work tree (never in the repository)
        if self.class() == "cc-by-sa" {
            for p in self.out.iter().chain(self.examples_file().as_ref()) {
                ensure!(git_tree(p).is_none(), "{k}: {} is inside the Git work tree {}; a CC BY-SA layer stays outside the repository", p.display(), git_tree(p).unwrap_or_default().display());
            }
        }
        Ok(())
    }
    /// wikipedia-examples: the examples file.
    pub fn examples_file(&self) -> Option<PathBuf> {
        (self.source == "wikipedia-examples").then(|| self.examples.clone().or_else(|| self.out.as_ref().map(|o| o.with_extension("examples.jsonl")))).flatten()
    }
    /// The data-level licence check of a shipped layer: no input file declares CC BY-SA.
    pub fn check_inputs(&self, k: &str, declared: &[PathBuf]) -> Result<()> {
        ensure!(
            !(self.shipped && !declared.is_empty()),
            "{k}: a shipped layer never takes CC BY-SA inputs, and {} declares it",
            declared[0].display()
        );
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub layers: BTreeMap<String, LayerConfig>,
    /// the shared sense store (`senses.jsonl`): a word's general sense fixed once for every layer and instance;
    /// absent: no store
    #[serde(default)]
    pub senses: Option<PathBuf>,
    /// words asked per teacher question (1..=40; default `work::BATCH`)
    #[serde(default)]
    pub batch: Option<usize>,
    /// seconds between two teacher questions of one instance (at least 30; default 600)
    #[serde(default)]
    pub pacing_secs: Option<u64>,
}

impl Config {
    pub fn load(path: &Path) -> Result<Config> {
        let c: Config = serde_json::from_slice(&std::fs::read(path).with_context(|| format!("{}", path.display()))?)?;
        for (k, l) in &c.layers {
            ensure!(crate::teacher::ident(k), "{k}: a layer name is an ASCII identifier");
            l.check(k)?;
        }
        ensure!(c.senses.as_ref().is_none_or(|p| p.is_absolute()), "the sense store is an absolute path");
        ensure!(c.batch.is_none_or(|b| (1..=40).contains(&b)), "batch is 1..40 words");
        ensure!(c.pacing_secs.is_none_or(|s| s >= 30), "pacing_secs is at least 30");
        Ok(c)
    }

    /// Words asked per teacher question.
    pub fn batch(&self) -> usize {
        self.batch.unwrap_or(crate::work::BATCH)
    }
    /// Seconds between two teacher questions of one instance.
    pub fn pacing(&self) -> u64 {
        self.pacing_secs.unwrap_or(600)
    }
    /// The layers whose words are always asked (`native_picks: false`).
    pub fn ask_all(&self) -> std::collections::BTreeSet<String> {
        self.layers.iter().filter(|(_, l)| l.native_picks == Some(false)).map(|(k, _)| k.clone()).collect()
    }
    /// The shared sense store.
    pub fn store(&self) -> crate::senses::Store {
        crate::senses::store_at(self.senses.as_deref())
    }

    /// The job of a configured layer, part `part` of `parts`: the improvement of an existing layer (`layer`), or
    /// the senses of a native draft (the others; the draft must be made first).
    pub fn job(&self, key: &str, part: u32, parts: u32) -> Result<Box<dyn crate::work::Job>> {
        let l = self.layers.get(key).with_context(|| format!("layer {key} is not configured"))?;
        ensure!(parts >= 1 && part < parts, "part {part} of {parts}");
        if l.source == "layer" {
            let file = l.file.clone().context("file")?;
            let text = std::fs::read_to_string(&file).with_context(|| format!("{}", file.display()))?;
            if crate::improve::declares_cc_by_sa(&text) {
                l.check_inputs(key, std::slice::from_ref(&file))?;
            }
            let base = layer::read_base(&std::fs::read_to_string(&l.base).with_context(|| format!("{}", l.base.display()))?);
            return Ok(Box::new(crate::improve::LayerJob {
                layer: key.into(),
                name: l.name.clone(),
                file,
                format: l.format.clone().unwrap_or_default(),
                base,
                part,
                parts,
                batch: self.batch(),
                store: self.store(),
            }));
        }
        let out = l.out.clone().context("out")?;
        Ok(Box::new(crate::work::DictJob { layers: vec![(key.to_string(), out, l.name.clone())], part, parts, batch: self.batch(), store: self.store(), ask_all: self.ask_all() }))
    }
}

/// The Git work tree a path is in (an ancestor holding `.git`), if any.
pub fn git_tree(p: &Path) -> Option<PathBuf> {
    p.ancestors().skip(1).find(|a| a.join(".git").exists()).map(Path::to_path_buf)
}

/// Draft the sampled Wikipedia layer: the sampling order, the examples fetched for the words still missing from the
/// examples file, the rows of the first `top` words with at least two chosen examples (`sampling`).
fn make_sampled(l: &LayerConfig, base: &layer::Base) -> Result<Value> {
    let corpus = l.corpus.as_deref().context("corpus")?;
    let wf = corpus.join("tier1/words.tsv");
    let words = crate::sampling::read_words(&std::fs::read_to_string(&wf).with_context(|| format!("{}: run `wikicorpus words`", wf.display()))?);
    let skip = crate::sampling::layer_words(&l.skip)?;
    let order = crate::sampling::order(&words, base, &skip, l.min_count.unwrap_or(20));
    let ex_path = l.examples_file().context("examples")?;
    let mut have: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    crate::sampling::scan(&ex_path, |s| {
        have.insert(s.word);
        Ok(())
    })?;
    let sampler = crate::sampling::Sampler {
        bin: l.wikicorpus.clone().context("wikicorpus")?,
        root: corpus.to_path_buf(),
        per_word: l.per_word.unwrap_or(crate::sampling::PER_WORD_MAX),
        seed: l.seed.unwrap_or(1),
        deep: l.deep.unwrap_or(0),
        jobs: l.jobs.unwrap_or(4),
    };
    // the words of the order up to `window` get examples (fetched when missing) and rows; a word whose examples give
    // fewer than two chosen ones is thin and left out, and the window grows until `top` rows or the order's end
    let (mut fetched, mut window) = (0, l.top.min(order.len()));
    let (rows, thin) = loop {
        let missing: Vec<&crate::sampling::Candidate> = order[..window].iter().filter(|c| !have.contains(&c.word)).collect();
        if !missing.is_empty() {
            fetched += sampler.fetch(&missing, &ex_path)?;
            have.extend(missing.iter().map(|c| c.word.clone()));
        }
        let want: BTreeMap<&str, &crate::sampling::Candidate> = order[..window].iter().map(|c| (c.word.as_str(), c)).collect();
        let mut made: BTreeMap<String, layer::Row> = BTreeMap::new();
        crate::sampling::scan(&ex_path, |s| {
            if let Some(c) = want.get(s.word.as_str()) {
                made.insert(s.word.clone(), crate::sampling::row(c, &s, base));
            }
            Ok(())
        })?;
        let mut rows = Vec::new();
        let mut thin = 0;
        for c in &order[..window] {
            match made.remove(&c.word) {
                Some(r) if r.examples.len() >= 2 => rows.push(r),
                Some(_) => thin += 1,
                None => bail!("the examples file has no line for «{}»", c.word),
            }
        }
        rows.truncate(l.top);
        if rows.len() >= l.top || window == order.len() {
            break (rows, thin);
        }
        window = (window + (l.top - rows.len()) * 2).min(order.len());
    };
    let out = l.out.as_ref().context("out")?;
    let text = crate::sampling::render(&rows, have.len(), &l.licence);
    if let Some(d) = out.parent() {
        std::fs::create_dir_all(d)?;
    }
    std::fs::write(out, &text)?;
    let st = layer::stats(&text);
    let head: Vec<Value> = order.iter().take(10).map(|c| json!({"word":c.word,"group":c.group,"f":c.f,"senses":c.senses,"interest":(c.interest * 100.0).round() / 100.0,"upos":c.upos})).collect();
    Ok(json!({"out":out,"examples_file":ex_path,"source":{"words":order.len(),"skipped":skip.len(),"fetched_now":fetched,"with_examples":have.len(),"thin":thin,"first":head},
        "stats":st,"licence_class":l.class(),"shipped":l.shipped,"sha256":mova_octopus::digest(text.as_bytes())}))
}

/// Draft a layer: read its source, count, write the file. Returns what was made.
pub fn make(l: &LayerConfig) -> Result<Value> {
    let base = layer::read_base(&std::fs::read_to_string(&l.base).with_context(|| format!("{}", l.base.display()))?);
    if l.source == "wikipedia-examples" {
        return make_sampled(l, &base);
    }
    let (units, src, extra) = match l.source.as_str() {
        "commands" => {
            let (raw, lessons, prompts) = sources::commands_raw(l.course.as_deref().context("course")?, l.corpus.as_deref())?;
            let a = en::annotate::Annotator::load(l.model.as_deref().context("model")?)?;
            let units = sources::annotate(&a, raw);
            let src = Source { name: "the command course and its corpus".into(), doc_noun: "lesson".into(), doc_plural: "lessons and prompts".into(), docs: lessons + prompts };
            (units, src, json!({"lessons":lessons,"prompts":prompts}))
        }
        "wikipedia" => {
            let (units, n) = sources::wikipedia(l.corpus.as_deref().context("corpus")?, l.articles)?;
            let src = Source { name: "the Wikipedia school's corpus".into(), doc_noun: "article".into(), doc_plural: "articles".into(), docs: n };
            (units, src, json!({"articles":n}))
        }
        "course" => {
            let c = sources::courses_raw(&l.courses)?;
            l.check_inputs(&l.name, &c.cc_by_sa)?;
            let a = en::annotate::Annotator::load(l.model.as_deref().context("model")?)?;
            let names: Vec<String> = l.courses.iter().filter_map(|p| p.file_name().and_then(|s| s.to_str()).map(String::from)).collect();
            let units = sources::annotate(&a, c.raw);
            let src = Source { name: format!("the school course{} {}", if names.len() == 1 { "" } else { "s" }, names.join(", ")), doc_noun: "lesson".into(), doc_plural: "lessons".into(), docs: c.lessons };
            (units, src, json!({"lessons":c.lessons}))
        }
        s => bail!("{s} layers are not drafted: Nautilus improves them through proposals"),
    };
    let out = l.out.as_ref().context("out")?;
    let rows = layer::build(&units, &base, &src, l.top);
    let text = layer::render(&rows, &src, &l.licence);
    if let Some(d) = out.parent() {
        std::fs::create_dir_all(d)?;
    }
    std::fs::write(out, &text)?;
    let st = layer::stats(&text);
    Ok(json!({"out":out,"units":units.len(),"source":extra,"stats":st,"licence_class":l.class(),"shipped":l.shipped,"sha256":mova_octopus::digest(text.as_bytes())}))
}

/// What a layer file holds now, or that it is not made yet; for an improved layer also its weak entries.
pub fn stats(l: &LayerConfig) -> Result<Value> {
    let file = l.rows_file().context("no layer file")?;
    match std::fs::read_to_string(file) {
        Ok(t) if l.source == "layer" => {
            let base = layer::read_base(&std::fs::read_to_string(&l.base).with_context(|| format!("{}", l.base.display()))?);
            let f = l.format.as_deref().unwrap_or("");
            let rows = crate::improve::parse(&t);
            let weak = rows.iter().filter(|r| crate::improve::weakness(f, r, &base).is_some()).count();
            let ask = rows.iter().filter(|r| crate::improve::askable(f, r, &base)).count();
            let thin = rows.iter().filter(|r| crate::improve::thin(f, r)).count();
            let examples: usize = rows.iter().map(|r| crate::improve::examples_of(f, r).len()).sum();
            let st = json!({"entries":rows.len(),"examples":examples,"draft":false});
            Ok(json!({"made":true,"out":file,"stats":st,"weak":weak,"askable":ask,"thin":thin,"teacher_steps":ask.div_ceil(crate::work::BATCH),
                "licence":l.licence,"licence_class":l.class(),"shipped":l.shipped}))
        }
        Ok(t) => Ok(json!({"made":true,"out":file,"stats":layer::stats(&t),"licence":l.licence,"licence_class":l.class(),"shipped":l.shipped})),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(json!({"made":false,"out":file})),
        Err(e) => Err(e.into()),
    }
}

/// The English of a draft just made.
pub fn say_made(name: &str, v: &Value) -> String {
    let st = &v["stats"];
    let src = &v["source"];
    let from = match (src["lessons"].as_u64(), src["prompts"].as_u64(), src["articles"].as_u64()) {
        (Some(l), Some(p), _) => format!("{} and {}", noun_count(l, "lesson"), noun_count(p, "prompt")),
        (_, _, Some(a)) => noun_count(a, "article"),
        (Some(l), None, _) => noun_count(l, "lesson"),
        _ => "its source".into(),
    };
    paragraph(&[
        sentence(&format!(
            "I have made {name} as a draft: {} with {}, from {from}",
            noun_count(st["entries"].as_u64().unwrap_or(0), "entry"),
            noun_count(st["examples"].as_u64().unwrap_or(0), "example")
        )),
        sentence("It is not reviewed yet, and no teacher wrote any of it"),
    ])
}

/// The English of a layer's counts.
pub fn say_stats(name: &str, v: &Value) -> String {
    if v["made"] != true {
        return sentence(&format!("I have not made {name} yet"));
    }
    let st = &v["stats"];
    let n = st["entries"].as_u64().unwrap_or(0);
    if let Some(w) = v["weak"].as_u64() {
        return paragraph(&[
            sentence(&format!("{name} has {}", noun_count(n, "entry"))),
            sentence(&format!(
                "{w} of them {} weak, and {} {} enough evidence to ask a teacher about, in {}",
                if w == 1 { "is" } else { "are" },
                v["askable"].as_u64().unwrap_or(0),
                if v["askable"].as_u64() == Some(1) { "has" } else { "have" },
                noun_count(v["teacher_steps"].as_u64().unwrap_or(0), "question")
            )),
        ]);
    }
    paragraph(&[
        sentence(&format!(
            "{name} has {}{}",
            noun_count(n, "entry"),
            if st["draft"] == true { ", and it is still a draft" } else { "" }
        )),
        sentence(&format!(
            "{} {} a meaning from the pupils' dictionary, and the entries have {} in all",
            st["with_meaning"].as_u64().unwrap_or(0),
            if st["with_meaning"].as_u64() == Some(1) { "has" } else { "have" },
            noun_count(st["examples"].as_u64().unwrap_or(0), "example")
        )),
    ])
}

/// The whole answer to a prompt: each action's sentence, or why nothing was done.
pub fn report(r: &Report) -> String {
    let done: Vec<String> = r.results.iter().filter_map(|x| x["output"]["answer"].as_str().map(str::to_string)).collect();
    if let Some(f) = &r.failure {
        let reason = f.reason.trim_end_matches('.');
        let mut out = done;
        out.push(if f.stage == "execution_failure" {
            sentence(&format!("I could not finish this request: {reason}"))
        } else {
            sentence(&format!("I did not act on this request because I could not read it as one I can do: {reason}"))
        });
        if let Some(id) = r.followups.first().and_then(|c| c["candidate_id"].as_str()) {
            out.push(sentence(&format!("I kept it for review as candidate {}", &id[..id.len().min(12)])));
        }
        return paragraph(&out);
    }
    if done.is_empty() {
        return sentence("I understood the request, but nothing was executed");
    }
    paragraph(&done)
}

/// Check the deputy configuration against this animal: every `nautilus` capability names a known operation and a
/// configured layer, with no slots.
pub fn check_capabilities(d: &mova_octopus::Config, c: &Config) -> Result<(), String> {
    let mut n = 0;
    for (name, cap) in d.capabilities.iter().filter(|(_, c)| c.handler == "nautilus") {
        let op = cap.parameters["operation"].as_str().ok_or(format!("{name}: operation required"))?;
        if !OPERATIONS.contains(&op) {
            return Err(format!("{name}: unknown operation {op}"));
        }
        let layer = cap.parameters["layer"].as_str().ok_or(format!("{name}: layer required"))?;
        if !c.layers.contains_key(layer) {
            return Err(format!("{name}: layer {layer} is not configured"));
        }
        if !cap.slots.is_empty() {
            return Err(format!("{name}: nautilus capabilities take no slots"));
        }
        // Only reading operations may answer a question (`IntentRule::observes`).
        if cap.intent.observes && op == "draft" {
            return Err(format!("{name}: operation {op} acts; it cannot be declared observing"));
        }
        n += 1;
    }
    if n == 0 {
        return Err("no nautilus capability configured".into());
    }
    Ok(())
}

/// The capability handler: the layer's work, with its English answer.
pub struct Hands<'a> {
    pub config: &'a Config,
}

impl CapabilityHandler for Hands<'_> {
    fn execute(&self, p: &Value, _: &Action, _: &str) -> Result<Value> {
        let key = p["layer"].as_str().context("layer")?;
        let l = self.config.layers.get(key).with_context(|| format!("layer {key} is not configured"))?;
        match p["operation"].as_str() {
            Some("draft") => {
                let v = make(l)?;
                Ok(json!({"operation":"draft","layer":key,"answer":say_made(&l.name, &v),"result":v}))
            }
            Some("stats") => {
                let v = stats(l)?;
                Ok(json!({"operation":"stats","layer":key,"answer":say_stats(&l.name, &v),"result":v}))
            }
            other => bail!("unknown nautilus operation {other:?}"),
        }
    }
}

/// A host-wide singleton: an exclusive lock on `lock` held for the life of the returned file, with this process's
/// id written into it. When another process holds it, `Err` names that holder ("already running as pid N"); the
/// caller exits without starting a second instance.
pub fn singleton(lock: &Path) -> Result<std::fs::File> {
    use std::io::{Read, Seek, Write};
    ensure!(lock.is_absolute(), "the lock is an absolute path");
    if let Some(d) = lock.parent() {
        std::fs::create_dir_all(d)?;
    }
    let mut f = std::fs::OpenOptions::new().read(true).write(true).create(true).truncate(false).open(lock)?;
    match f.try_lock() {
        Ok(()) => {
            f.set_len(0)?;
            f.rewind()?;
            write!(f, "{}\n", std::process::id())?;
            f.sync_data()?;
            Ok(f)
        }
        Err(std::fs::TryLockError::WouldBlock) => {
            let mut pid = String::new();
            let _ = f.read_to_string(&mut pid);
            bail!("already running as pid {} (lock {})", pid.trim(), lock.display())
        }
        Err(std::fs::TryLockError::Error(e)) => Err(e.into()),
    }
}

/// Answer one prompt: plan and execute through the deputy with Nautilus's hands.
pub fn answer(deputy: &Deputy, config: &Config, id: &str, prompt: &str) -> Result<Value> {
    let hands = Hands { config };
    let overrides: BTreeMap<String, &dyn CapabilityHandler> = BTreeMap::from([("nautilus".to_owned(), &hands as &dyn CapabilityHandler)]);
    let r = deputy.run_with(prompt, id, true, &overrides)?;
    Ok(json!({"answer":report(&r),"plan":r.plan,"failure":r.failure,"results":r.results}))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn english_of_results() {
        let made = json!({"stats":{"entries":80,"examples":231,"with_meaning":70,"draft":true},"source":{"lessons":30,"prompts":56}});
        assert_eq!(
            say_made("the commands dictionary", &made),
            "I have made the commands dictionary as a draft: 80 entries with 231 examples, from 30 lessons and 56 prompts. It is not reviewed yet, and no teacher wrote any of it."
        );
        let st = json!({"made":true,"stats":{"entries":1,"examples":3,"with_meaning":1,"draft":true}});
        assert_eq!(
            say_stats("the commands dictionary", &st),
            "The commands dictionary has 1 entry, and it is still a draft. 1 has a meaning from the pupils' dictionary, and the entries have 3 examples in all."
        );
        assert_eq!(say_stats("the Wikipedia dictionary", &json!({"made":false})), "I have not made the Wikipedia dictionary yet.");
    }
    #[test]
    fn the_example_configurations_hold() {
        let here = Path::new(env!("CARGO_MANIFEST_DIR"));
        let c = Config::load(&here.join("nautilus.example.json")).unwrap();
        let d: mova_octopus::Config = serde_json::from_slice(&std::fs::read(here.join("deputy.example.json")).unwrap()).unwrap();
        check_capabilities(&d, &c).unwrap();
        let t: crate::teacher::Config = serde_json::from_slice(&std::fs::read(here.join("teachers.example.json")).unwrap()).unwrap();
        t.validate().unwrap();
        // the public default teacher is Opus, and the workers do not start by themselves
        assert_eq!((t.profiles[0].provider.as_str(), t.profiles[0].model.as_str()), ("claude-cli", "opus"));
        // the teacher's own mova-terminal profile, and the profile named in the teacher configuration
        let tp = terminal::Profile::load(&here.join("terminal.example.json")).unwrap();
        assert_eq!(tp.animal, t.profiles[0].terminal);
        let w: Value = serde_json::from_slice(&std::fs::read(here.join("crab-workers.example.json")).unwrap()).unwrap();
        assert!(w["workers"].as_array().unwrap().iter().all(|w| w["autostart"] == false));
        // a capability for an unconfigured layer is refused
        let mut c2 = c.clone();
        c2.layers.remove("wikipedia");
        assert!(check_capabilities(&d, &c2).unwrap_err().contains("not configured"));
    }
    #[test]
    fn licences_are_enforced_by_configuration() {
        let lay = |extra: Value| -> Result<Config> {
            let mut l = json!({"source":"layer","file":"/x/emode-senses.tsv","format":"emode-senses","base":"/x/base.tsv","licence":"ours","name":"the emode dictionary"});
            for (k, v) in extra.as_object().unwrap() {
                l[k] = v.clone();
            }
            let c: Config = serde_json::from_value(json!({"layers":{"emode":l}}))?;
            for (k, l) in &c.layers {
                l.check(k)?;
            }
            Ok(c)
        };
        assert!(lay(json!({"licence_class":"ours","shipped":true})).is_ok());
        // red: CC BY-SA, NC and research-only inputs never into a shipped layer; a class is required
        for bad in ["cc-by-sa", "nc", "research-only"] {
            let e = lay(json!({"licence_class":bad,"shipped":true})).unwrap_err().to_string();
            assert!(e.contains("never takes"), "{e}");
            assert!(lay(json!({"licence_class":bad,"shipped":false})).is_ok());
        }
        assert!(lay(json!({"shipped":true})).is_err(), "a layer source names its licence class");
        assert!(lay(json!({"licence_class":"gpl"})).is_err());
        // the Wikipedia corpus is CC BY-SA whatever is written, and never ships
        let w = |extra: Value| -> Result<()> {
            let mut l = json!({"source":"wikipedia","out":"/x/w.tsv","base":"/x/b.tsv","corpus":"/x/c","licence":"CC BY-SA","name":"w"});
            for (k, v) in extra.as_object().unwrap() {
                l[k] = v.clone();
            }
            let l: LayerConfig = serde_json::from_value(l)?;
            assert_eq!(l.class(), "cc-by-sa");
            l.check("w")
        };
        assert!(w(json!({})).is_ok());
        assert!(w(json!({"licence_class":"ours"})).is_err());
        assert!(w(json!({"shipped":true})).is_err());
        // the sampled Wikipedia layer: CC BY-SA too, never shipped, never written inside a Git work tree
        let s = |extra: Value| -> Result<()> {
            let mut l = json!({"source":"wikipedia-examples","out":"/x/w2.tsv","base":"/x/b.tsv","corpus":"/x/c","wikicorpus":"/x/wikicorpus","top":20000,"licence":"CC BY-SA","name":"w2"});
            for (k, v) in extra.as_object().unwrap() {
                l[k] = v.clone();
            }
            let l: LayerConfig = serde_json::from_value(l)?;
            assert_eq!(l.class(), "cc-by-sa");
            l.check("w2")
        };
        assert!(s(json!({})).is_ok());
        for red in [json!({"shipped":true}), json!({"licence_class":"ours"}), json!({"wikicorpus":null}), json!({"per_word":101}), json!({"top":50001}), json!({"skip":["rel/x.tsv"]})] {
            assert!(s(red.clone()).is_err(), "{red}");
        }
        // red: the layer or its examples file inside a Git work tree (this checkout)
        let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(git_tree(repo).is_some(), "the control needs this checkout");
        let inside = repo.join("target-never/w2.tsv");
        let e = s(json!({"out":inside})).unwrap_err().to_string();
        assert!(e.contains("outside the repository"), "{e}");
        assert!(s(json!({"examples":repo.join("w2.examples.jsonl")})).is_err());
        // the plain Wikipedia source too
        assert!(w(json!({"out":inside})).is_err());
        assert_eq!(serde_json::from_value::<LayerConfig>(json!({"source":"wikipedia-examples","out":"/x/w2.tsv","base":"/x/b","licence":"l","name":"n"})).unwrap().examples_file(), Some(PathBuf::from("/x/w2.examples.jsonl")));
    }
    #[test]
    fn one_instance_per_lock() {
        let d = std::env::temp_dir().join(format!("nautilus-lock-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let lock = d.join("nautilus.lock");
        let held = singleton(&lock).unwrap();
        // a second start reports the running one and does not start
        let e = singleton(&lock).unwrap_err().to_string();
        assert!(e.starts_with(&format!("already running as pid {}", std::process::id())), "{e}");
        drop(held);
        assert!(singleton(&lock).is_ok());
    }
}
