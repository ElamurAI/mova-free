//! The runtime dictionary store: the dictionary layers Mova loads in learning and in work, each with its provenance
//! (layer, state, licence, scope), chosen by a manifest at run time (`MOVA_DICT`), deterministic (`BTreeMap`).
//!
//! Two classes of layer, by licence:
//! - **shipped** (ours, Elamur Cosmic, or CC0 / public domain; `scope: public`): part of Mova, in the repository,
//!   releases and binaries. The built-in layers are the pupils' dictionary (`data/dictionary-improved.tsv`,
//!   `builtin:pupils`, admitted) and the commands dictionary (`commands.tsv` of the school data, `knowledge::dictionaries_dir`,
//!   `builtin:commands`, a Nautilus draft: shipped in the school archive, not yet reviewed; empty without it);
//! - **internal only** (CC BY-SA 4.0: the Wikipedia dictionary of `$MOVA_DATA/nautilus/layers/wikipedia.tsv`; `scope:
//!   internal`): loaded from the data directory for internal runs and measurements only (`MOVA_DICT_USE=internal`),
//!   never for a shipped use (`Purpose::Release`, the default), never a built-in layer: a built-in text whose header
//!   declares CC BY-SA is refused (`shippable`), and so is any file layer that declares it under a public scope.
//! - **add-on modules** (`scope: module`; docs/modules.md, crate `mova-modules`): dictionary layers of modules the user
//!   installed into `$MOVA_DATA/modules` (or `MOVA_MODULES_PATH`), each under the module's own licence. Named in a
//!   manifest as `module:NAME[@VERSION]/LAYER`, or added after the built-in layers with `MOVA_MODULES=all` or
//!   `MOVA_MODULES=name,name` (off by default: without it the store and its fingerprint are unchanged). A module layer
//!   is never built in; its file is checked against the module's SHA-256 on every load; its entries keep the
//!   provenance (module, version, licence, attribution) through their layer; a non-commercial / research-only module
//!   loads for internal use only. Whatever Mova writes from a store holding a share-alike or non-commercial module
//!   carries that licence (`derived_header`) and is refused inside a Git work tree and under a built-in file's name
//!   (`guard_derived_write`), so it never enters Mova's repository, releases or binaries.
//!
//! A layer file has the columns of the school layers (word, part of speech, meaning for a pupil, sense in the
//! lessons, examples, note; `#` lines are comments). A manifest is JSON:
//!
//! ```json
//! {"schema": 1, "allow_unreviewed": false, "layers": [
//!   {"id": "pupils", "path": "builtin:pupils", "state": "admitted", "licence": "elamur-cosmic", "scope": "public"},
//!   {"id": "commands", "path": "builtin:commands", "state": "draft", "licence": "elamur-cosmic", "scope": "public"},
//!   {"id": "wikipedia", "path": "../nautilus/layers/wikipedia.tsv", "sha256": "…", "state": "draft",
//!    "licence": "cc-by-sa-4.0", "scope": "internal"},
//!   {"id": "wikt", "path": "module:example-dict@2026.11.04/senses", "state": "draft", "licence": "CC-BY-SA-4.0",
//!    "scope": "module"}]}
//! ```
//!
//! A module layer's `licence` must repeat the module's licence (or its class), so a manifest names what it loads; its
//! optional `sha256` pins the module's module.json; `admitted` needs the module to declare the layer admitted.
//!
//! Rules, checked when the store loads (fail-fast: a bad manifest stops the process with the reason):
//! - every file layer names its SHA-256; a file whose bytes differ is refused (like a model file);
//! - a state is `admitted` (reviewed: the pupils' dictionary, or a layer `world dict admit` made from a review
//!   record), `draft` (made natively, not reviewed: Nautilus drafts) or `derived` (made by Mova from what it learned).
//!   A teacher's proposal is never a layer: `proposal` is refused; only `world dict admit` with a complete review
//!   record turns accepted senses into an admitted layer;
//! - a layer is *loaded* (shown, counted, shipped when public) whatever its state, but only admitted layers are
//!   *knowledge* (`Store::knowledge`): `draft` and `derived` count as knowledge only with `allow_unreviewed`.
//!
//!   world dict stats [MANIFEST]                       the layers, their states, licences, entries and hashes
//!   world dict lookup WORD…                           the entries of words in the loaded store, with provenance
//!   world dict check MANIFEST [--release]             validate a manifest (hashes, states, licences)
//!   world dict hash FILE                              the SHA-256 to pin a layer file with
//!   world dict admit PROPOSAL REVIEW OUT              accepted senses of a reviewed proposal → an admitted layer

use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// The built-in (shipped) layers: (path, id, state).
pub const BUILTINS: [(&str, &str, &str); 2] = [("builtin:pupils", "pupils", "admitted"), ("builtin:commands", "commands", "draft")];

fn builtin_text(path: &str) -> Option<&'static str> {
    match path {
        "builtin:pupils" => Some(include_str!("../data/dictionary-improved.tsv")),
        // the commands dictionary is school data outside the repository (`knowledge::dictionaries_dir`, release archive
        // `school-<version>`); without it the layer is empty
        "builtin:commands" => {
            static C: std::sync::OnceLock<String> = std::sync::OnceLock::new();
            Some(C.get_or_init(|| std::fs::read_to_string(crate::knowledge::dictionaries_dir().join("commands.tsv")).unwrap_or_default()).as_str())
        }
        _ => None,
    }
}

/// A text may ship with Mova only when its header declares neither a CC BY-SA nor a non-commercial / research-only
/// licence (licence/DATA.md; `derived_header` writes such a mark).
pub fn shippable(text: &str) -> Result<()> {
    for l in text.lines().take_while(|l| l.starts_with('#')) {
        let u = l.to_lowercase();
        ensure!(!(u.contains("cc by-sa") || u.contains("cc-by-sa")), "a CC BY-SA layer never ships with Mova: {l}");
        ensure!(!(u.contains("cc by-nc") || u.contains("cc-by-nc") || u.contains("nc-research")), "a non-commercial layer never ships with Mova: {l}");
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Admitted,
    Draft,
    Derived,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Public,
    Internal,
    /// A layer of an installed add-on module (docs/modules.md).
    Module,
}

/// What the store is loaded for: a shipped use (public layers only) or internal runs and measurements.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Purpose {
    Release,
    Internal,
}

/// The licences a layer may carry; anything else is refused.
const PUBLIC_LICENCES: [&str; 3] = ["elamur-cosmic", "cc0-1.0", "public-domain"];
const INTERNAL_LICENCES: [&str; 1] = ["cc-by-sa-4.0"];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LayerSpec {
    pub id: String,
    pub path: String,
    #[serde(default)]
    pub sha256: String,
    pub state: String,
    pub licence: String,
    pub scope: Scope,
    /// The review record an admitted file layer came from (`world dict admit`).
    #[serde(default)]
    pub review: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub schema: u32,
    #[serde(default)]
    pub allow_unreviewed: bool,
    pub layers: Vec<LayerSpec>,
}

/// The add-on module a layer comes from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleSource {
    pub name: String,
    pub version: String,
    pub licence: String,
    pub class: mova_modules::LicenceClass,
    pub attribution: String,
}

/// A loaded layer's provenance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layer {
    pub id: String,
    pub sha256: String,
    pub state: State,
    pub licence: String,
    pub scope: Scope,
    pub entries: usize,
    /// Its entries are knowledge: admitted, or another state with `allow_unreviewed`.
    pub knowledge: bool,
    /// The add-on module it comes from (`scope: module`).
    pub module: Option<ModuleSource>,
}

/// One entry of one layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// Index into `Store::layers`.
    pub layer: usize,
    pub pos: String,
    pub meaning: String,
    pub sense: String,
    pub examples: String,
    pub note: String,
    /// The columns after the sixth as one tab-separated text, when the layer has them (`EXTRA_COLUMNS`; an add-on
    /// dictionary such as the Wiktionary module writes them; the school layers have none). Kept whole and split on
    /// reading, so a large layer costs one allocation per row for them.
    pub extra: String,
}

/// The extra columns a layer may carry after the six of the school layers, in order (`tools/wikt-extract`): labels,
/// synonyms and antonyms by sense number («1:UK,slang; *:tube,TV»), forms («plural=tellies»), form-of links
/// («1:clipping of:checkmate»), templates the extractor had no handler for, and the row's provenance.
pub const EXTRA_COLUMNS: [&str; 7] = ["labels", "synonyms", "antonyms", "forms", "form of", "gaps", "provenance"];

impl Entry {
    /// An extra column by name (`EXTRA_COLUMNS`), empty when the layer lacks it.
    pub fn col(&self, name: &str) -> &str {
        EXTRA_COLUMNS.iter().position(|c| *c == name).and_then(|i| self.extra.split('\t').nth(i)).unwrap_or("")
    }

    /// The terms of a sense-numbered column («1:friend,buddy; *:tube,TV» → friend, buddy, tube, TV).
    pub fn terms(&self, name: &str) -> Vec<String> {
        self.col(name).split("; ").filter_map(|p| p.split_once(':')).flat_map(|(_, t)| t.split(',')).map(|t| t.trim().to_lowercase()).filter(|t| !t.is_empty()).collect()
    }
}

#[derive(Debug, Default)]
pub struct Store {
    pub layers: Vec<Layer>,
    entries: BTreeMap<String, Vec<Entry>>,
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// The SHA-256 of a layer file, as a manifest pins it.
pub fn file_hash(path: &Path) -> Result<String> {
    Ok(sha256(&std::fs::read(path).with_context(|| format!("reading {}", path.display()))?))
}

fn parse_state(s: &str) -> Result<State> {
    Ok(match s {
        "admitted" => State::Admitted,
        "draft" => State::Draft,
        "derived" => State::Derived,
        "proposal" => bail!("a teacher's proposal is not a layer: admit its reviewed senses with `world dict admit`"),
        other => bail!("unknown layer state «{other}» (admitted, draft or derived)"),
    })
}

/// The rows of a layer text: word (lower case) → (pos, meaning, sense, examples, note), and the extra columns.
fn rows(text: &str) -> Vec<(String, [String; 5], String)> {
    text.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).filter_map(|l| {
        let c: Vec<&str> = l.split('\t').collect();
        let w = c.first()?.trim().to_lowercase();
        let col = |i: usize| c.get(i).map(|x| x.to_string()).unwrap_or_default();
        let extra = if c.len() > 6 { c[6..].join("\t") } else { String::new() };
        (!w.is_empty() && c.len() >= 3).then(|| (w, [col(1), col(2), col(3), col(4), col(5)], extra))
    }).collect()
}

impl Store {
    /// The built-in store: the shipped layers (the pupils' dictionary, admitted; the commands dictionary, a draft).
    pub fn builtin() -> Store {
        let m = Manifest { schema: 1, allow_unreviewed: false, layers: BUILTINS.iter().map(|b| builtin_spec(b.0)).collect() };
        Store::from_manifest(&m, Path::new("."), Purpose::Release).expect("the built-in layer is valid")
    }

    /// Load a manifest file; layer paths are relative to its folder.
    pub fn load(manifest: &Path, purpose: Purpose) -> Result<Store> {
        let text = std::fs::read_to_string(manifest).with_context(|| format!("reading the dictionary manifest {}", manifest.display()))?;
        let m: Manifest = serde_json::from_str(&text).with_context(|| format!("parsing {}", manifest.display()))?;
        Store::from_manifest(&m, manifest.parent().unwrap_or(Path::new(".")), purpose)
    }

    pub fn from_manifest(m: &Manifest, base: &Path, purpose: Purpose) -> Result<Store> {
        Store::from_manifest_in(m, base, purpose, &mova_modules::roots())
    }

    /// `from_manifest` with the module roots named (`mova_modules::roots()` by default).
    pub fn from_manifest_in(m: &Manifest, base: &Path, purpose: Purpose, roots: &[PathBuf]) -> Result<Store> {
        ensure!(m.schema == 1, "unsupported dictionary manifest schema {}", m.schema);
        ensure!(!m.layers.is_empty(), "a dictionary manifest names at least one layer");
        let mut ids = BTreeSet::new();
        let mut store = Store::default();
        for spec in &m.layers {
            ensure!(ids.insert(spec.id.clone()), "layer id «{}» twice", spec.id);
            let state = parse_state(&spec.state).with_context(|| format!("layer «{}»", spec.id))?;
            if let Some(r) = spec.path.strip_prefix("module:") {
                let (text, hash, src) = module_layer(spec, r, purpose, roots).with_context(|| format!("layer «{}»", spec.id))?;
                store.push_layer(spec, state, text, hash, m.allow_unreviewed, Some(src));
                continue;
            }
            ensure!(spec.scope != Scope::Module, "layer «{}»: scope module needs a module: path", spec.id);
            let public_licence = PUBLIC_LICENCES.contains(&spec.licence.as_str());
            ensure!(public_licence || INTERNAL_LICENCES.contains(&spec.licence.as_str()), "layer «{}»: unknown licence «{}»", spec.id, spec.licence);
            ensure!(public_licence || spec.scope == Scope::Internal, "layer «{}»: licence {} must be scope internal (licence/DATA.md)", spec.id, spec.licence);
            ensure!(purpose == Purpose::Internal || spec.scope == Scope::Public, "layer «{}» is internal ({}): it never ships with Mova", spec.id, spec.licence);
            let (text, hash) = if let Some(t) = builtin_text(&spec.path) {
                let b = BUILTINS.iter().find(|b| b.0 == spec.path).expect("a built-in path");
                ensure!(spec.state == b.2 && spec.scope == Scope::Public && public_licence, "the built-in layer {} is {} and public", b.1, b.2);
                shippable(t)?;
                (t.to_string(), sha256(t.as_bytes()))
            } else {
                ensure!(!spec.path.starts_with("builtin:"), "unknown built-in layer {}", spec.path);
                ensure!(spec.state != "admitted" || spec.review.is_some(), "layer «{}»: an admitted file layer names its review record", spec.id);
                let p = base.join(&spec.path);
                let bytes = std::fs::read(&p).with_context(|| format!("layer «{}»: reading {}", spec.id, p.display()))?;
                let h = sha256(&bytes);
                ensure!(spec.sha256.len() == 64, "layer «{}»: no SHA-256 pinned", spec.id);
                ensure!(h == spec.sha256, "layer «{}»: {} has SHA-256 {h}, the manifest pins {}", spec.id, p.display(), spec.sha256);
                let t = String::from_utf8(bytes).with_context(|| format!("layer «{}» is not UTF-8", spec.id))?;
                // a public layer whose own header says CC BY-SA is mislabelled
                ensure!(spec.scope == Scope::Internal || shippable(&t).is_ok(), "layer «{}» declares CC BY-SA in its header but is labelled public", spec.id);
                (t, h)
            };
            store.push_layer(spec, state, text, hash, m.allow_unreviewed, None);
        }
        Ok(store)
    }

    fn push_layer(&mut self, spec: &LayerSpec, state: State, text: String, hash: String, allow_unreviewed: bool, module: Option<ModuleSource>) {
        let k = self.layers.len();
        // the rows are moved into the store (a large add-on layer is not held twice), the text dropped first
        let rs = rows(&text);
        drop(text);
        let n = rs.len();
        for (w, [pos, meaning, sense, examples, note], extra) in rs {
            self.entries.entry(w).or_default().push(Entry { layer: k, pos, meaning, sense, examples, note, extra });
        }
        self.layers.push(Layer { id: spec.id.clone(), sha256: hash, state, licence: spec.licence.clone(), scope: spec.scope, entries: n, knowledge: state == State::Admitted || allow_unreviewed, module });
    }

    /// The licences of the share-alike and non-commercial modules loaded, one item per module in layer order
    /// («CC-BY-SA-4.0 (module example-dict 2026.11.04)»); `None` when what Mova derives from this store is ours.
    pub fn derived_licence(&self) -> Option<String> {
        let mut seen = BTreeSet::new();
        let items: Vec<String> = self.layers.iter().filter_map(|l| l.module.as_ref()).filter(|s| s.class.taints() && seen.insert((s.name.clone(), s.version.clone()))).map(|s| format!("{} (module {} {}; {})", s.licence, s.name, s.version, s.class.as_str())).collect();
        (!items.is_empty()).then(|| items.join("; "))
    }

    /// The header line that marks a file Mova derives from this store, when a share-alike or non-commercial module is
    /// loaded: `shippable` refuses a text that carries it.
    pub fn derived_header(&self) -> Option<String> {
        self.derived_licence().map(|l| format!("# licence: derived with add-on modules under {l}; their terms apply to this file; never part of Mova's repository, releases or binaries"))
    }

    /// Refuse to write a file derived from this store inside a Git work tree or under a built-in file's name when a
    /// share-alike or non-commercial module is loaded (the negative control: the same store without such a module
    /// writes there).
    pub fn guard_derived_write(&self, path: &Path) -> Result<()> {
        let Some(l) = self.derived_licence() else { return Ok(()) };
        let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        ensure!(!BUILTIN_FILES.contains(&name.as_str()), "{} is a built-in dictionary's name; this output is derived under {l}", path.display());
        ensure!(!mova_modules::inside_git_worktree(path), "{} is inside a Git work tree; this output is derived under {l} and never enters Mova's repository (write it to the data directory)", path.display());
        Ok(())
    }

    /// The entries of a word (lower case), in the manifest's layer order.
    pub fn get(&self, word: &str) -> &[Entry] {
        self.entries.get(word).map(Vec::as_slice).unwrap_or(&[])
    }

    /// The entries of a word that are knowledge (see `Layer::knowledge`), in layer order.
    pub fn knowledge(&self, word: &str) -> impl Iterator<Item = &Entry> {
        self.get(word).iter().filter(|e| self.layers[e.layer].knowledge)
    }

    pub fn words(&self) -> usize {
        self.entries.len()
    }

    /// The identity of the loaded dictionary: the layers' ids, states, licences and hashes, in order.
    pub fn fingerprint(&self) -> String {
        let s: String = self.layers.iter().map(|l| match &l.module {
            None => format!("{}\t{:?}\t{}\t{}\n", l.id, l.state, l.licence, l.sha256),
            Some(m) => format!("{}\t{:?}\t{}\t{}\tmodule {}@{}\n", l.id, l.state, l.licence, l.sha256, m.name, m.version),
        }).collect();
        sha256(s.as_bytes())
    }

    /// The first meaning of a word in the loaded layers: its first sense's first sentence, without its labels
    /// («1) (informal, UK) Telly is a television set. 2) …» → «Telly is a television set»), with its layer.
    pub fn first_meaning(&self, word: &str) -> Option<(String, usize)> {
        self.knowledge(word).find_map(|e| {
            let m = first_sentence(&e.meaning);
            (!m.is_empty()).then_some((m, e.layer))
        })
    }
}

/// The first sentence of a meaning's first sense, without the sense number and leading labels in brackets.
pub fn first_sentence(meaning: &str) -> String {
    let first = meaning.split(" 2) ").next().unwrap_or("");
    let mut s = first.trim().trim_start_matches("1)").trim();
    while s.starts_with('(') {
        match s.find(')') {
            Some(i) => s = s[i + 1..].trim(),
            None => break,
        }
    }
    // the bracketed asides inside the sentence go too («A toque (also tuque) is …»); an unclosed one ends it
    let s = s.split(['.', ';', ':']).next().unwrap_or("");
    let mut out = String::new();
    let mut depth = 0usize;
    for ch in s.chars() {
        match ch {
            '(' => depth += 1,
            ')' if depth > 0 => depth -= 1,
            _ if depth == 0 => out.push(ch),
            _ => {}
        }
    }
    if depth > 0 {
        out = s[..s.find('(').unwrap_or(s.len())].to_string();
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ").trim_end_matches(',').trim().to_string()
}

/// The file names of the built-in layers.
const BUILTIN_FILES: [&str; 2] = ["dictionary-improved.tsv", "commands.tsv"];

/// Load a module layer (`module:NAME[@VERSION]/LAYER`): find the installed module, check its licence against the
/// spec and the purpose, and read the layer file verified against the module's SHA-256.
fn module_layer(spec: &LayerSpec, r: &str, purpose: Purpose, roots: &[PathBuf]) -> Result<(String, String, ModuleSource)> {
    ensure!(spec.scope == Scope::Module, "a module layer has scope module");
    let (m, layer) = r.split_once('/').context("a module layer path is module:NAME[@VERSION]/LAYER")?;
    let (name, version) = mova_modules::split_ref(m);
    let inst = mova_modules::find(roots, name, version)?;
    let man = &inst.manifest;
    ensure!(spec.licence.eq_ignore_ascii_case(&man.licence) || spec.licence == man.licence_class.as_str(), "the manifest says licence {}, module {}@{} is {} ({})", spec.licence, man.name, man.version, man.licence, man.licence_class.as_str());
    ensure!(spec.sha256.is_empty() || spec.sha256 == inst.manifest_sha256, "module {}@{}: module.json has SHA-256 {}, the manifest pins {}", man.name, man.version, inst.manifest_sha256, spec.sha256);
    ensure!(purpose == Purpose::Internal || !man.licence_class.research_only(), "module {}@{} is non-commercial / research-only ({}): internal use only (MOVA_DICT_USE=internal)", man.name, man.version, man.licence);
    let p = man.dictionary_layers().find(|p| p.layer.as_deref() == Some(layer)).with_context(|| format!("module {}@{} provides no dictionary layer «{layer}»", man.name, man.version))?;
    ensure!(spec.state != "admitted" || p.state() == "admitted", "module {}@{} does not declare layer «{layer}» admitted", man.name, man.version);
    let f = man.file(&p.path).context("a provided path is a listed file")?;
    let bytes = mova_modules::read_verified(&inst.dir, f)?;
    let text = String::from_utf8(bytes).context("a module layer is UTF-8")?;
    let src = ModuleSource { name: man.name.clone(), version: man.version.clone(), licence: man.licence.clone(), class: man.licence_class, attribution: man.attribution.line() };
    Ok((text, f.sha256.clone(), src))
}

/// The layer specs of installed modules selected by `MOVA_MODULES` (`all`, or `name[@version],…`): every dictionary
/// layer of each, in name order, with the state the module declares.
pub fn module_specs(selection: &str, roots: &[PathBuf]) -> Result<Vec<LayerSpec>> {
    let picked: Vec<mova_modules::Installed> = if selection.trim() == "all" {
        let d = mova_modules::discover(roots);
        let names: BTreeSet<&str> = d.modules.keys().map(|k| k.0.as_str()).collect();
        names.into_iter().map(|n| mova_modules::find(roots, n, None)).collect::<Result<_>>()?
    } else {
        selection.split(',').map(str::trim).filter(|s| !s.is_empty()).map(|r| {
            let (n, v) = mova_modules::split_ref(r);
            mova_modules::find(roots, n, v)
        }).collect::<Result<_>>()?
    };
    let mut out = Vec::new();
    for m in &picked {
        for p in m.manifest.dictionary_layers() {
            let l = p.layer.clone().unwrap_or_default();
            out.push(LayerSpec { id: format!("{}/{l}", m.manifest.name), path: format!("module:{}@{}/{l}", m.manifest.name, m.manifest.version), sha256: m.manifest_sha256.clone(), state: p.state().into(), licence: m.manifest.licence.clone(), scope: Scope::Module, review: None });
        }
    }
    Ok(out)
}

fn builtin_spec(path: &str) -> LayerSpec {
    let b = BUILTINS.iter().find(|b| b.0 == path).expect("a built-in path");
    LayerSpec { id: b.1.into(), path: b.0.into(), sha256: String::new(), state: b.2.into(), licence: "elamur-cosmic".into(), scope: Scope::Public, review: None }
}

/// The manifest this process loads: `MOVA_DICT` names a manifest file (its layer paths relative to its folder), else
/// the built-in layers; `MOVA_MODULES` appends the selected modules' layers.
pub fn process_manifest() -> Result<(Manifest, PathBuf)> {
    let (mut m, base) = match std::env::var("MOVA_DICT") {
        Ok(p) if !p.is_empty() => {
            let text = std::fs::read_to_string(&p).with_context(|| format!("reading the dictionary manifest {p}"))?;
            let m: Manifest = serde_json::from_str(&text).with_context(|| format!("parsing {p}"))?;
            (m, Path::new(&p).parent().unwrap_or(Path::new(".")).to_path_buf())
        }
        _ => (Manifest { schema: 1, allow_unreviewed: false, layers: BUILTINS.iter().map(|b| builtin_spec(b.0)).collect() }, PathBuf::from(".")),
    };
    if let Ok(sel) = std::env::var("MOVA_MODULES")
        && !sel.trim().is_empty()
    {
        m.layers.extend(module_specs(&sel, &mova_modules::roots())?);
    }
    Ok((m, base))
}

/// The store this process uses (`process_manifest`), loaded for `MOVA_DICT_USE=internal` use, else for a release. A
/// manifest or module that fails its checks stops the process: a dictionary is not silently replaced by another.
pub fn store() -> &'static Store {
    static S: std::sync::OnceLock<Store> = std::sync::OnceLock::new();
    S.get_or_init(|| {
        let purpose = if std::env::var("MOVA_DICT_USE").is_ok_and(|v| v == "internal") { Purpose::Internal } else { Purpose::Release };
        match process_manifest().and_then(|(m, base)| Store::from_manifest(&m, &base, purpose)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("the dictionary (MOVA_DICT / MOVA_MODULES) was refused: {e:#}");
                std::process::exit(2);
            }
        }
    })
}

// ---- admission: a reviewed proposal → an admitted layer --------------------------------------------------------

/// A review record of one teacher proposal (Nautilus `STATE/proposals/<id>.txt`, lines `word<TAB>sense`): who reviewed
/// it, when, the proposal's SHA-256, and the words whose senses are accepted or rejected. Every word of the proposal
/// must be decided; an accepted word must be in the proposal.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Review {
    pub proposal_sha256: String,
    pub reviewer: String,
    pub date: String,
    pub accept: Vec<String>,
    pub reject: Vec<String>,
    #[serde(default)]
    pub note: String,
}

/// The admitted layer text of a reviewed proposal: one row per accepted word, its sense in the «sense in the
/// lessons» column and its provenance in the note.
pub fn admit(proposal: &[u8], proposal_id: &str, review: &Review) -> Result<String> {
    ensure!(sha256(proposal) == review.proposal_sha256, "the review is of another proposal (SHA-256 differs)");
    ensure!(!review.reviewer.trim().is_empty() && !review.date.trim().is_empty(), "a review names its reviewer and date");
    let text = std::str::from_utf8(proposal).context("the proposal is not UTF-8")?;
    let mut senses: BTreeMap<String, String> = BTreeMap::new();
    for l in text.lines().filter(|l| !l.trim().is_empty()) {
        let (w, s) = l.split_once('\t').with_context(|| format!("a proposal line without a sense: {l}"))?;
        ensure!(!s.contains('\t'), "a proposal sense holds a tab: {w}");
        ensure!(senses.insert(w.trim().to_lowercase(), s.trim().to_string()).is_none(), "a word twice in the proposal: {w}");
    }
    let accept: BTreeSet<String> = review.accept.iter().map(|w| w.to_lowercase()).collect();
    let reject: BTreeSet<String> = review.reject.iter().map(|w| w.to_lowercase()).collect();
    ensure!(accept.is_disjoint(&reject), "a word both accepted and rejected");
    for w in accept.iter().chain(&reject) {
        ensure!(senses.contains_key(w), "the review decides «{w}», which the proposal does not hold");
    }
    for w in senses.keys() {
        ensure!(accept.contains(w) || reject.contains(w), "the review leaves «{w}» undecided");
    }
    let mut out = String::from("# word\tpart of speech\tmeaning for a pupil\tsense in the lessons\texamples\tnote\n");
    out.push_str(&format!("# admitted: proposal {proposal_id} (SHA-256 {}), reviewed by {} on {}; {} accepted, {} rejected\n", review.proposal_sha256, review.reviewer, review.date, accept.len(), reject.len()));
    for w in &accept {
        out.push_str(&format!("{w}\t\t\t{}\t\tadmitted from {proposal_id}, reviewed by {} on {}\n", senses[w], review.reviewer, review.date));
    }
    Ok(out)
}

pub fn main_args(args: &[String]) -> Result<()> {
    match args.get(1).map(String::as_str) {
        Some("hash") => {
            for p in &args[2..] {
                println!("{}\t{p}", file_hash(Path::new(p))?);
            }
        }
        Some("check") => {
            let p = args.get(2).context("usage: world dict check MANIFEST [--release]")?;
            let purpose = if args.iter().any(|a| a == "--release") { Purpose::Release } else { Purpose::Internal };
            let s = Store::load(Path::new(p), purpose)?;
            println!("ok: {} layers, {} words, fingerprint {}", s.layers.len(), s.words(), s.fingerprint());
        }
        Some("stats") => {
            let s = match args.get(2) {
                Some(p) => Store::load(Path::new(p), Purpose::Internal)?,
                None => {
                    let (m, base) = process_manifest()?;
                    Store::from_manifest(&m, &base, Purpose::Internal)?
                }
            };
            for l in &s.layers {
                println!("{}\t{:?}\t{}\t{:?}\t{} entries\t{}\t{}", l.id, l.state, l.licence, l.scope, l.entries, if l.knowledge { "knowledge" } else { "not knowledge" }, l.sha256);
                if let Some(m) = &l.module {
                    println!("#\tmodule {}@{} [{}]: {}", m.name, m.version, m.class.as_str(), m.attribution);
                }
            }
            if let Some(h) = s.derived_header() {
                println!("{h}");
            }
            println!("{} words, fingerprint {}", s.words(), s.fingerprint());
        }
        Some("lookup") => {
            let s = store();
            for w in &args[2..] {
                let w = w.to_lowercase();
                for e in s.get(&w) {
                    let l = &s.layers[e.layer];
                    let lic = l.module.as_ref().map(|m| format!("\t[module {}@{}, {}]", m.name, m.version, m.licence)).unwrap_or_default();
                    println!("{w}\t{}\t{:?}{}\t{}\t{}{lic}", l.id, l.state, if l.knowledge { "" } else { " (not knowledge)" }, e.pos, first_sentence(&e.meaning));
                }
            }
        }
        Some("admit") => {
            let (p, r, out) = (args.get(2), args.get(3), args.get(4));
            let (Some(p), Some(r), Some(out)) = (p, r, out) else { bail!("usage: world dict admit PROPOSAL REVIEW OUT") };
            let review: Review = serde_json::from_str(&std::fs::read_to_string(r)?).with_context(|| format!("parsing {r}"))?;
            let id = Path::new(p).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
            let text = admit(&std::fs::read(p)?, &id, &review)?;
            let out = PathBuf::from(out);
            let mut f = std::fs::OpenOptions::new().create_new(true).write(true).open(&out).with_context(|| format!("{} exists or cannot be made", out.display()))?;
            std::io::Write::write_all(&mut f, text.as_bytes())?;
            println!("{}\t{}\t{} accepted", sha256(text.as_bytes()), out.display(), review.accept.len());
        }
        _ => bail!("usage: world dict stats|lookup|check|hash|admit …"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("mova-dictstore-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn spec(id: &str, path: &str, sha: &str, state: &str, licence: &str, scope: Scope) -> LayerSpec {
        LayerSpec { id: id.into(), path: path.into(), sha256: sha.into(), state: state.into(), licence: licence.into(), scope, review: Some("r.json".into()) }
    }

    const LAYER: &str = "# word\tpart of speech\tmeaning for a pupil\tsense\texamples\tnote\ntelly\tnoun\t1) (informal, UK) Telly is a television set. 2) Programmes.\t\t\t\n";

    #[test]
    fn builtin_layers_ship_the_pupils_and_commands_dictionaries() {
        let s = Store::builtin();
        assert_eq!(s.layers.iter().map(|l| (l.id.as_str(), l.state, l.knowledge)).collect::<Vec<_>>(), vec![("pupils", State::Admitted, true), ("commands", State::Draft, false)]);
        assert!(s.words() > 10_000);
        assert_eq!(s.first_meaning("telly").map(|x| x.0).as_deref(), Some("Telly is a television set"));
        // the commands draft is loaded (when the school data is installed), but not knowledge until reviewed
        if !builtin_text("builtin:commands").unwrap().is_empty() {
            assert!(s.get("say").iter().any(|e| s.layers[e.layer].id == "commands"));
        }
        assert!(s.knowledge("say").all(|e| s.layers[e.layer].id == "pupils"));
        // the negative control: a word the dictionary does not hold
        assert!(s.get("zorpish").is_empty());
        assert_eq!(s.fingerprint(), Store::builtin().fingerprint());
    }

    /// The shipping rule by licence class: every built-in text is shippable, and the check goes red on a CC BY-SA
    /// header (its negative control: the Wikipedia layer's own header line).
    #[test]
    fn cc_by_sa_never_ships() {
        for b in BUILTINS {
            shippable(builtin_text(b.0).unwrap()).unwrap();
        }
        let wiki = "# word\tpart of speech\n# draft: made natively by Nautilus (tools/nautilus) from the Wikipedia school's corpus; CC BY-SA 4.0 like its source (English Wikipedia): internal, never in the repository or a release\ncity\tnoun\tA city is a large town.\n";
        assert!(shippable(wiki).is_err(), "a CC BY-SA header must be refused");
        assert!(shippable("# ours, Elamur Cosmic License\ncity\tnoun\tA town; CC BY-SA in a row is no header\n").is_ok());
        let d = dir("ship");
        std::fs::write(d.join("w.tsv"), wiki).unwrap();
        let h = file_hash(&d.join("w.tsv")).unwrap();
        let load = |licence: &str, scope: Scope, p: Purpose| Store::from_manifest(&Manifest { schema: 1, allow_unreviewed: true, layers: vec![spec("wikipedia", "w.tsv", &h, "draft", licence, scope)] }, &d, p);
        // internal runs may load it; a shipped use never; mislabelled as ours and public: refused by its header
        assert!(load("cc-by-sa-4.0", Scope::Internal, Purpose::Internal).is_ok());
        assert!(load("cc-by-sa-4.0", Scope::Internal, Purpose::Release).is_err());
        assert!(load("cc-by-sa-4.0", Scope::Public, Purpose::Internal).is_err());
        assert!(load("elamur-cosmic", Scope::Public, Purpose::Internal).is_err());
    }

    #[test]
    fn hashes_states_and_licences_are_enforced() {
        let d = dir("rules");
        std::fs::write(d.join("a.tsv"), LAYER).unwrap();
        let h = file_hash(&d.join("a.tsv")).unwrap();
        let load = |layers: Vec<LayerSpec>, allow: bool, p: Purpose| Store::from_manifest(&Manifest { schema: 1, allow_unreviewed: allow, layers }, &d, p);
        // admitted, ours, pinned: loads for a shipped use, as knowledge
        let s = load(vec![spec("a", "a.tsv", &h, "admitted", "elamur-cosmic", Scope::Public)], false, Purpose::Release).unwrap();
        assert_eq!(s.knowledge("telly").count(), 1);
        // a changed byte is refused
        assert!(load(vec![spec("a", "a.tsv", &"0".repeat(64), "admitted", "elamur-cosmic", Scope::Public)], false, Purpose::Release).is_err());
        // a proposal is never a layer, even with allow_unreviewed
        assert!(load(vec![spec("a", "a.tsv", &h, "proposal", "elamur-cosmic", Scope::Public)], true, Purpose::Internal).is_err());
        // a draft loads but is knowledge only with allow_unreviewed
        let s = load(vec![spec("a", "a.tsv", &h, "draft", "elamur-cosmic", Scope::Public)], false, Purpose::Release).unwrap();
        assert_eq!((s.get("telly").len(), s.knowledge("telly").count(), s.first_meaning("telly")), (1, 0, None));
        let s = load(vec![spec("a", "a.tsv", &h, "draft", "elamur-cosmic", Scope::Public)], true, Purpose::Internal).unwrap();
        assert_eq!(s.knowledge("telly").count(), 1);
        // an unknown licence, an unknown built-in, a built-in with another state: refused
        assert!(load(vec![spec("a", "a.tsv", &h, "admitted", "nc", Scope::Internal)], false, Purpose::Internal).is_err());
        assert!(load(vec![spec("x", "builtin:wikipedia", "", "draft", "elamur-cosmic", Scope::Public)], true, Purpose::Internal).is_err());
        assert!(load(vec![spec("commands", "builtin:commands", "", "admitted", "elamur-cosmic", Scope::Public)], false, Purpose::Internal).is_err());
        // layer order is kept: the built-in first, then the file
        let s = load(vec![builtin_spec("builtin:pupils"), spec("a", "a.tsv", &h, "admitted", "elamur-cosmic", Scope::Public)], false, Purpose::Release).unwrap();
        assert_eq!(s.get("telly").iter().map(|e| e.layer).collect::<Vec<_>>(), vec![0, 1]);
        assert_ne!(s.fingerprint(), Store::builtin().fingerprint());
    }

    #[test]
    fn admission_needs_a_complete_review_of_the_same_proposal() {
        let p = b"job\tA task someone must do.\nwait\tTo let time pass.\n";
        let ok = Review { proposal_sha256: sha256(p), reviewer: "a reviewer".into(), date: "2026-11-04".into(), accept: vec!["job".into()], reject: vec!["wait".into()], note: String::new() };
        let t = admit(p, "w1", &ok).unwrap();
        let r = rows(&t);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].0, "job");
        assert_eq!(r[0].1[2], "A task someone must do.");
        // another proposal's review, an undecided word, an unknown word, no reviewer: refused
        assert!(admit(b"job\tX\n", "w1", &ok).is_err());
        assert!(admit(p, "w1", &Review { reject: vec![], ..ok.clone() }).is_err());
        assert!(admit(p, "w1", &Review { accept: vec!["job".into(), "zorp".into()], ..ok.clone() }).is_err());
        assert!(admit(p, "w1", &Review { reviewer: " ".into(), ..ok.clone() }).is_err());
        // an admitted layer loads with its review named and its hash pinned
        let d = dir("admit");
        std::fs::write(d.join("job.tsv"), &t).unwrap();
        let h = file_hash(&d.join("job.tsv")).unwrap();
        let s = Store::from_manifest(&Manifest { schema: 1, allow_unreviewed: false, layers: vec![spec("job", "job.tsv", &h, "admitted", "elamur-cosmic", Scope::Public)] }, &d, Purpose::Release).unwrap();
        assert_eq!(s.get("job")[0].sense, "A task someone must do.");
        let mut no_review = spec("job", "job.tsv", &h, "admitted", "elamur-cosmic", Scope::Public);
        no_review.review = None;
        assert!(Store::from_manifest(&Manifest { schema: 1, allow_unreviewed: false, layers: vec![no_review] }, &d, Purpose::Release).is_err());
    }

    /// An installed module `<root>/<name>/<version>` with one dictionary layer `senses`.
    fn module(root: &Path, name: &str, version: &str, class: &str, licence: &str, extra: serde_json::Value) -> PathBuf {
        let d = root.join(name).join(version);
        std::fs::create_dir_all(&d).unwrap();
        std::fs::write(d.join("senses.tsv"), LAYER).unwrap();
        let mut provide = serde_json::json!({"kind": "dictionary-layer", "layer": "senses", "path": "senses.tsv"});
        if let (Some(p), Some(e)) = (provide.as_object_mut(), extra.as_object()) {
            p.extend(e.clone());
        }
        let m = serde_json::json!({"schema": 1, "name": name, "version": version, "kind": "dictionary-layer", "licence": licence, "licence_class": class,
            "attribution": {"text": "a test source", "sources": ["https://example.org/"]},
            "files": [{"path": "senses.tsv", "sha256": sha256(LAYER.as_bytes()), "bytes": LAYER.len()}], "provides": [provide], "requires_mova": ">=0.5"});
        std::fs::write(d.join("module.json"), serde_json::to_vec(&m).unwrap()).unwrap();
        d
    }

    fn mspec(id: &str, path: &str, licence: &str) -> LayerSpec {
        LayerSpec { id: id.into(), path: path.into(), sha256: String::new(), state: "draft".into(), licence: licence.into(), scope: Scope::Module, review: None }
    }

    /// Module layers load with their provenance; licence, scope, pins, bytes and research-only terms are enforced.
    #[test]
    fn module_layers_keep_provenance_and_licence_gates() {
        let root = dir("modules");
        let roots = vec![root.clone()];
        let sa = module(&root, "wikt", "1.0", "cc-by-sa", "CC-BY-SA-4.0", serde_json::json!({}));
        module(&root, "nc", "1", "nc-research", "CC-BY-NC-4.0", serde_json::json!({}));
        module(&root, "cc", "1", "cc-by", "CC-BY-4.0", serde_json::json!({}));
        let load = |layers: Vec<LayerSpec>, p: Purpose| Store::from_manifest_in(&Manifest { schema: 1, allow_unreviewed: false, layers }, Path::new("."), p, &roots);
        let s = load(vec![builtin_spec("builtin:pupils"), mspec("wikt", "module:wikt@1.0/senses", "CC-BY-SA-4.0")], Purpose::Release).unwrap();
        let m = s.layers[1].module.as_ref().unwrap();
        assert_eq!((m.name.as_str(), m.version.as_str(), m.class), ("wikt", "1.0", mova_modules::LicenceClass::CcBySa));
        assert!(m.attribution.contains("https://example.org/"));
        // the module's entry is loaded after the pupils' one, as a draft: shown, not knowledge
        assert_eq!(s.get("telly").iter().map(|e| e.layer).collect::<Vec<_>>(), vec![0, 1]);
        assert!(s.knowledge("telly").all(|e| e.layer == 0));
        assert!(s.derived_licence().unwrap().contains("CC-BY-SA-4.0 (module wikt 1.0"));
        assert_ne!(s.fingerprint(), Store::builtin().fingerprint());
        // the licence class may stand for the licence; the latest version is found without one
        assert!(load(vec![mspec("w", "module:wikt/senses", "cc-by-sa")], Purpose::Release).is_ok());
        // negative controls: another licence, a public scope, a wrong module.json pin, a layer it lacks, admitted
        assert!(load(vec![mspec("w", "module:wikt/senses", "elamur-cosmic")], Purpose::Release).is_err());
        assert!(load(vec![LayerSpec { scope: Scope::Public, ..mspec("w", "module:wikt/senses", "CC-BY-SA-4.0") }], Purpose::Release).is_err());
        assert!(load(vec![LayerSpec { sha256: "0".repeat(64), ..mspec("w", "module:wikt/senses", "CC-BY-SA-4.0") }], Purpose::Release).is_err());
        assert!(load(vec![mspec("w", "module:wikt/glosses", "CC-BY-SA-4.0")], Purpose::Release).is_err());
        assert!(load(vec![LayerSpec { state: "admitted".into(), ..mspec("w", "module:wikt/senses", "CC-BY-SA-4.0") }], Purpose::Release).is_err());
        // a module layer with a public licence of its own is still not a file layer: scope module needs module:
        assert!(load(vec![LayerSpec { path: "senses.tsv".into(), ..mspec("w", "", "elamur-cosmic") }], Purpose::Release).is_err());
        // research-only: internal use only
        assert!(load(vec![mspec("n", "module:nc/senses", "CC-BY-NC-4.0")], Purpose::Release).is_err());
        assert!(load(vec![mspec("n", "module:nc/senses", "CC-BY-NC-4.0")], Purpose::Internal).unwrap().derived_licence().unwrap().contains("nc-research"));
        // attribution-only: nothing to mark
        assert_eq!(load(vec![mspec("c", "module:cc/senses", "CC-BY-4.0")], Purpose::Release).unwrap().derived_licence(), None);
        // MOVA_MODULES=all selects every module's layers, in name order, with their own licences
        let all = module_specs("all", &roots).unwrap();
        assert_eq!(all.iter().map(|l| l.id.as_str()).collect::<Vec<_>>(), vec!["cc/senses", "nc/senses", "wikt/senses"]);
        assert!(module_specs("absent", &roots).is_err());
        // a changed byte in the installed layer is refused
        std::fs::write(sa.join("senses.tsv"), LAYER.replace("television", "televisiom")).unwrap();
        assert!(load(vec![mspec("w", "module:wikt@1.0/senses", "CC-BY-SA-4.0")], Purpose::Release).is_err());
    }

    /// What Mova derives from a store with a share-alike module is marked, the mark never ships, and it is never
    /// written into a repository or under a built-in file's name. Red control: the same writes from the built-in store
    /// (no module) pass, and with the mark removed `shippable` would accept the text.
    #[test]
    fn derived_output_of_a_share_alike_module_never_ships() {
        let root = dir("derived");
        module(&root, "wikt", "1.0", "cc-by-sa", "CC-BY-SA-4.0", serde_json::json!({}));
        let s = Store::from_manifest_in(&Manifest { schema: 1, allow_unreviewed: false, layers: vec![mspec("w", "module:wikt/senses", "CC-BY-SA-4.0")] }, Path::new("."), Purpose::Release, &[root.clone()]).unwrap();
        let header = s.derived_header().unwrap();
        let derived = format!("# word\tpart of speech\n{header}\ntelly\tnoun\tA set.\n");
        assert!(shippable(&derived).is_err(), "the mark must make the text unshippable");
        assert!(shippable(&derived.replace(&header, "# ours")).is_ok());
        let repo = root.join("repo");
        std::fs::create_dir_all(repo.join(".git")).unwrap();
        let built_in = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/dictionary-improved.tsv");
        assert!(s.guard_derived_write(&repo.join("out/layer.tsv")).is_err());
        assert!(s.guard_derived_write(&built_in).is_err());
        assert!(s.guard_derived_write(&root.join("elsewhere/commands.tsv")).is_err());
        assert!(s.guard_derived_write(&root.join("elsewhere/layer.tsv")).is_ok());
        let ours = Store::builtin();
        assert!(ours.derived_header().is_none());
        assert!(ours.guard_derived_write(&repo.join("out/layer.tsv")).is_ok());
        // the built-in layers never come from a module, and their texts carry no share-alike or NC mark
        assert!(ours.layers.iter().all(|l| l.module.is_none() && l.scope == Scope::Public));
        assert!(shippable("# licence: CC-BY-NC-4.0 (module x 1; nc-research)\n").is_err());
    }

    /// A layer with the extra columns of the Wiktionary module keeps them; a school layer has none (its negative
    /// control: `col` and `terms` are empty).
    #[test]
    fn extra_columns_are_kept() {
        let d = dir("extra");
        let t = "# License: CC BY-SA 4.0\ntelly\tnoun\t1) Television. 2) A television set.\t\t1: Not much on telly.\tetymology 1\t*:UK,colloquial\t*:tube,TV\t\tcountable=both; plural=tellies\t\t\t74428 rev 91706010 f.bz2@42\nbrekkie\tnoun\t1) Diminutive of breakfast.\t\t\t\t\t\t\t\t1:diminutive of:breakfast\t\t4246 rev 1 f.bz2@7\n";
        std::fs::write(d.join("w.tsv"), t).unwrap();
        let h = file_hash(&d.join("w.tsv")).unwrap();
        let s = Store::from_manifest(&Manifest { schema: 1, allow_unreviewed: true, layers: vec![spec("wikt", "w.tsv", &h, "draft", "cc-by-sa-4.0", Scope::Internal)] }, &d, Purpose::Internal).unwrap();
        let e = &s.get("telly")[0];
        assert_eq!(e.terms("synonyms"), vec!["tube", "tv"]);
        assert_eq!(e.col("forms"), "countable=both; plural=tellies");
        assert_eq!(e.col("provenance"), "74428 rev 91706010 f.bz2@42");
        assert_eq!(s.get("brekkie")[0].col("form of"), "1:diminutive of:breakfast");
        assert_eq!(s.first_meaning("telly").map(|x| x.0).as_deref(), Some("Television"));
        let b = Store::builtin();
        let p = &b.get("telly")[0];
        assert!(p.extra.is_empty() && p.col("synonyms").is_empty() && p.terms("synonyms").is_empty());
    }

    #[test]
    fn first_sentence_drops_numbers_and_labels() {
        assert_eq!(first_sentence("1) (Canada) A toque (also tuque) is a knitted winter hat; a beanie. 2) A chef's hat."), "A toque is a knitted winter hat");
        assert_eq!(first_sentence("In the UK, a jumper is a warm top (in the US: a sweater)"), "In the UK, a jumper is a warm top");
        assert_eq!(first_sentence("(informal) Doc is a short word for a doctor."), "Doc is a short word for a doctor");
        assert_eq!(first_sentence(""), "");
    }
}
