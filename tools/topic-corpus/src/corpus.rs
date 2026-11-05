//! Stage 3: the closed topic corpus — a cross corpus around one paper, read lazily.
//!
//! The corpus registers documents; it does not copy or parse them all. Originals stay where they are
//! and stay compressed: library texts in the library's text root, pages in Sea Spider's zstd store,
//! consultant drafts and our author notes zstd-compressed under `originals/`. A document is read
//! and parsed on request, whole: the reader walks it from the start with one discourse context, so
//! every sentence is parsed in the world of its document (`parsed`). Only the hot neighbourhood (the
//! paper and its first members) is parsed at build time and pinned; other parsed documents live in a
//! bounded cache (`cache/`, zstd JSONL, least recently used evicted first). Sentences the parser reads
//! badly go to the shared hard-cases list (`hardcases`).
//!
//! - `corpus.json` — key, paper, counts; `internal_only: true` always;
//! - `documents.jsonl` — the registry: id, kind (`paper`, `web`, `sol`, `author`), origin, where the
//!   original is, roles, licence as stated, its class and where it came from, site terms,
//!   `internal_only` (always true), `public_admissible`, text hash, `hot`;
//! - `cache/index.json` and `cache/<doc>.jsonl.zst` — per sentence: part, text, words (form, lemma,
//!   UPOS, head, relation), frames, issues.
//!
//! The directory is private (0700). A public output takes only `public_admissible` documents
//! (`assert_public`, `public_export`): library texts under CC BY, CC0 or public domain; never a page
//! Sea Spider fetched, a consultant's draft or a note. Deterministic: the same requests in the same
//! order give byte-identical files.
use crate::graph::Neighbourhood;
use crate::hardcases;
use crate::library::Library;
use crate::licence::{self, Class, Origin};
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Path, PathBuf};

pub const SCHEMA: &str = "mova-topic-corpus-v2";
/// The reader version: a cached parse is reused only under the same text hash and reader.
pub const READER: &str = "topic-reader-3";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Doc {
    pub id: String,
    /// `paper`, `web`, `sol` or `author`.
    pub kind: String,
    pub origin: Origin,
    pub title: String,
    /// `file` (a library text), `seaspider` (Sea Spider's store) or `stored` (`originals/`).
    pub source_kind: String,
    /// The text file, the URL or the original's file name.
    pub source: String,
    /// Sea Spider's store directory (`seaspider` documents).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
    #[serde(default)]
    pub roles: Vec<String>,
    pub licence: String,
    pub licence_class: Class,
    /// `library`, `page` (stated on the page), `none`, `consultant` or `ours`.
    pub licence_from: String,
    #[serde(default)]
    pub terms_url: Option<String>,
    pub local_only: bool,
    /// Always true: the corpus is closed.
    pub internal_only: bool,
    pub public_admissible: bool,
    #[serde(default)]
    pub quality: Option<String>,
    pub text_sha256: String,
    /// Parsed at build time and never evicted.
    #[serde(default)]
    pub hot: bool,
    /// Library ids, DOI, URL — what a citation names (stable).
    #[serde(default)]
    pub cite: Value,
}

impl Doc {
    /// A blank document of `kind`/`origin`; callers fill title, source and licence.
    pub fn new(id: &str, kind: &str, origin: Origin) -> Doc {
        Doc {
            id: id.into(),
            kind: kind.into(),
            origin,
            title: String::new(),
            source_kind: "stored".into(),
            source: String::new(),
            store: None,
            roles: vec![],
            licence: String::new(),
            licence_class: Class::Unknown,
            licence_from: "none".into(),
            terms_url: None,
            local_only: true,
            internal_only: true,
            public_admissible: false,
            quality: None,
            text_sha256: String::new(),
            hot: false,
            cite: Value::Null,
        }
    }
}

pub type Word = (String, String, String, usize, String);

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Sentence {
    pub s: usize,
    pub part: String,
    pub text: String,
    /// `[form, lemma, upos, head, rel]`, heads 1-based over the sentence (0: a root).
    pub words: Vec<Word>,
    pub frames: Vec<String>,
    /// `hardcases::issues` of this parse.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct CacheIndex {
    clock: u64,
    entries: BTreeMap<String, CacheEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CacheEntry {
    sha: String,
    reader: String,
    file: String,
    bytes: u64,
    used: u64,
    sentences: usize,
}

pub struct Options {
    pub max_sentences: usize,
    pub cache_max_bytes: u64,
    pub hard_cases: Option<PathBuf>,
}

impl Default for Options {
    fn default() -> Self {
        Options { max_sentences: 1500, cache_max_bytes: 1 << 30, hard_cases: None }
    }
}

pub struct Corpus {
    pub dir: PathBuf,
    pub key: String,
    pub docs: Vec<Doc>,
    pub opts: Options,
    /// Texts read in this process (the lazy reads of one run).
    texts: BTreeMap<String, String>,
}

pub fn sha(b: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
}

pub fn write_private(p: &Path, b: &[u8]) -> Result<()> {
    let tmp = p.with_extension("tmp");
    let mut f = std::fs::OpenOptions::new().create(true).write(true).truncate(true).mode(0o600).open(&tmp)?;
    f.write_all(b)?;
    f.sync_data()?;
    std::fs::rename(&tmp, p)?;
    Ok(())
}

/// A file name for a document id (`lib:x` → `lib-x`).
pub fn file_of(id: &str) -> String {
    id.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '.' || c == '_' { c } else { '-' }).collect()
}

fn kind_rank(k: &str) -> u8 {
    match k {
        "paper" => 0,
        "author" => 1,
        "web" => 2,
        _ => 3,
    }
}

/// Read a Markdown document through Mova, whole: admitted sentences of the parts the Whales read, in
/// order, with one discourse context for the document; each with its UD words, frames and issues.
/// At most `max` sentences.
pub fn annotate(markdown: &str, max: usize) -> Result<Vec<Sentence>> {
    let r = mova_whale::reader::read(markdown).map_err(|e| anyhow!("{e}"))?;
    world::tree::annotator()?;
    let mut ctx = world::frames::Ctx::default();
    let mut out = vec![];
    for sec in r.sections.iter().filter(|s| !s.skipped) {
        for &i in &sec.sentences {
            let st = &r.sentences[i];
            if !st.admitted {
                continue;
            }
            if out.len() >= max {
                return Ok(out);
            }
            let mut x = sentence(out.len(), &sec.title, &st.text, &mut ctx)?;
            // The Whales' own reader found nothing to restate: a sentence the Owl cannot read.
            if st.skeleton.is_empty() && st.clauses.is_empty() {
                x.issues.push("no_skeleton".into());
            }
            out.push(x);
        }
    }
    Ok(out)
}

/// Parse one sentence in a discourse context.
pub fn sentence(s: usize, part: &str, text: &str, ctx: &mut world::frames::Ctx) -> Result<Sentence> {
    let read = world::frames::read(text, ctx, text.trim_end().ends_with('?'))?;
    let mut words = vec![];
    let mut roots = vec![];
    for t in &read.trees {
        let base = words.len();
        roots.push(t.w.iter().filter(|w| w.head == 0).count());
        for w in &t.w {
            let head = if w.head == 0 { 0 } else { w.head + base };
            words.push((w.form.clone(), w.lemma.clone(), w.upos.name().to_string(), head, w.rel.name().to_string()));
        }
    }
    // The reader lowercases forms; keep the text's own forms when the tokens line up.
    let orig: Vec<String> = world::tree::annotator()?.tokenize(text).into_iter().map(|t| t.form).collect();
    if orig.len() == words.len() {
        for (w, o) in words.iter_mut().zip(orig) {
            if w.0.eq_ignore_ascii_case(&o) || w.0 == o.to_lowercase() {
                w.0 = o;
            }
        }
    }
    let frames: Vec<String> = read.frames.iter().map(|f| f.show(0)).collect();
    let issues = hardcases::issues(&words, &roots, frames.len());
    Ok(Sentence { s, part: part.into(), text: text.into(), words, frames, issues })
}

/// The issues of a sentence parsed alone (for `hardcases::recheck`).
pub fn sentence_issues(text: &str) -> Result<Vec<String>> {
    Ok(sentence(0, "", text, &mut world::frames::Ctx::default())?.issues)
}

impl Corpus {
    pub fn dir_of(root: &Path, key: &str) -> Result<PathBuf> {
        if key.is_empty() || !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.') || key.starts_with('.') {
            bail!("corpus key must be a plain name: {key:?}");
        }
        Ok(root.join(key))
    }

    /// Open (or create) the corpus `key` under `root`.
    pub fn open(root: &Path, key: &str, paper: &str, opts: Options) -> Result<Corpus> {
        let dir = Self::dir_of(root, key)?;
        for d in [dir.clone(), dir.join("cache"), dir.join("originals")] {
            mova_whale::util::private_dir(&d).map_err(|e| anyhow!("{e}"))?;
        }
        let mut docs = vec![];
        let dp = dir.join("documents.jsonl");
        if dp.exists() {
            for l in std::fs::read_to_string(&dp)?.lines().filter(|l| !l.trim().is_empty()) {
                docs.push(serde_json::from_str(l).with_context(|| format!("{}", dp.display()))?);
            }
        }
        let c = Corpus { dir, key: key.into(), docs, opts, texts: BTreeMap::new() };
        if !c.dir.join("corpus.json").exists() {
            c.save_meta(paper)?;
        }
        Ok(c)
    }

    /// Open an existing corpus directory.
    pub fn load(dir: &Path, opts: Options) -> Result<Corpus> {
        let meta: Value = serde_json::from_slice(&std::fs::read(dir.join("corpus.json")).with_context(|| format!("{} is not a topic corpus", dir.display()))?)?;
        let key = meta["key"].as_str().unwrap_or_default().to_string();
        let root = dir.parent().ok_or_else(|| anyhow!("corpus dir has no parent"))?;
        Self::open(root, &key, meta["paper"].as_str().unwrap_or_default(), opts)
    }

    pub fn paper(&self) -> String {
        std::fs::read(self.dir.join("corpus.json")).ok().and_then(|b| serde_json::from_slice::<Value>(&b).ok()).and_then(|v| v["paper"].as_str().map(str::to_string)).unwrap_or_default()
    }

    fn save_meta(&self, paper: &str) -> Result<()> {
        let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
        for d in &self.docs {
            *kinds.entry(d.kind.as_str()).or_default() += 1;
        }
        let v = json!({"schema": SCHEMA, "key": self.key, "paper": paper, "internal_only": true, "documents": self.docs.len(), "kinds": kinds, "hot": self.docs.iter().filter(|d| d.hot).count(), "public_admissible": self.docs.iter().filter(|d| d.public_admissible).count(), "reader": READER});
        write_private(&self.dir.join("corpus.json"), &serde_json::to_vec_pretty(&v)?)
    }

    fn save(&mut self) -> Result<()> {
        self.docs.sort_by(|a, b| kind_rank(&a.kind).cmp(&kind_rank(&b.kind)).then_with(|| a.id.cmp(&b.id)));
        let s: String = self.docs.iter().map(|d| serde_json::to_string(d).unwrap() + "\n").collect();
        write_private(&self.dir.join("documents.jsonl"), s.as_bytes())?;
        let p = self.paper();
        self.save_meta(&p)
    }

    pub fn doc(&self, id: &str) -> Option<&Doc> {
        self.docs.iter().find(|d| d.id == id)
    }

    /// Register a document whose text is `text` (nothing is parsed). A `stored` document's text is
    /// written compressed under `originals/`. The closed-corpus rule is enforced here: every document
    /// is internal, and only library texts under an open licence are public-admissible.
    pub fn register(&mut self, mut d: Doc, text: &str) -> Result<()> {
        let id = d.id.clone();
        d.internal_only = true;
        d.public_admissible = licence::public_admissible(d.origin, d.licence_class, d.local_only);
        d.text_sha256 = sha(text.as_bytes());
        if d.source_kind == "stored" {
            d.source = format!("originals/{}.md.zst", file_of(&d.id));
            let z = zstd::encode_all(text.as_bytes(), 9)?;
            write_private(&self.dir.join(&d.source), &z)?;
        }
        if let Some(i) = self.docs.iter().position(|x| x.id == d.id) {
            let old = &mut self.docs[i];
            for r in &d.roles {
                if !old.roles.contains(r) {
                    old.roles.push(r.clone());
                }
            }
            let roles = old.roles.clone();
            let hot = old.hot || d.hot;
            d.roles = roles;
            d.hot = hot;
            self.docs[i] = d;
        } else {
            self.docs.push(d);
        }
        self.texts.insert(id, text.to_string());
        self.save()
    }

    /// The text of a document, read from its original (compressed originals are decompressed).
    pub fn text(&mut self, id: &str) -> Result<String> {
        if let Some(t) = self.texts.get(id) {
            return Ok(t.clone());
        }
        let d = self.doc(id).ok_or_else(|| anyhow!("{id} is not in the corpus"))?.clone();
        let t = match d.source_kind.as_str() {
            "file" => std::fs::read_to_string(&d.source).with_context(|| d.source.clone())?,
            "stored" => String::from_utf8(zstd::decode_all(std::fs::File::open(self.dir.join(&d.source))?)?)?,
            "seaspider" => {
                let store = d.store.as_deref().ok_or_else(|| anyhow!("{id}: no Sea Spider store"))?;
                let r = seaspider::store::Reader::open(Path::new(store)).map_err(|e| anyhow!("{e}"))?;
                let (rec, body) = r.get(&d.source).map_err(|e| anyhow!("{e}"))?;
                crate::web::page_markdown(&rec, &body, &d.source).1
            }
            k => bail!("{id}: unknown source kind {k}"),
        };
        if sha(t.as_bytes()) != d.text_sha256 {
            bail!("{id}: the original changed since it was registered (sha256 differs); register it again");
        }
        self.texts.insert(id.to_string(), t.clone());
        Ok(t)
    }

    fn index(&self) -> Result<CacheIndex> {
        let p = self.dir.join("cache/index.json");
        if !p.exists() {
            return Ok(CacheIndex::default());
        }
        Ok(serde_json::from_slice(&std::fs::read(p)?)?)
    }

    /// Whether a document's parse is in the cache (current text and reader).
    pub fn cached(&self, id: &str) -> bool {
        let Some(d) = self.doc(id) else { return false };
        self.index().ok().and_then(|ix| ix.entries.get(id).cloned()).is_some_and(|e| e.sha == d.text_sha256 && e.reader == READER && self.dir.join("cache").join(&e.file).exists())
    }

    /// The parsed sentences of a document: from the cache, else read whole and parsed now (then
    /// cached, the cache bounded, and the bad parses recorded as hard cases).
    pub fn parsed(&mut self, id: &str) -> Result<Vec<Sentence>> {
        let d = self.doc(id).ok_or_else(|| anyhow!("{id} is not in the corpus"))?.clone();
        let mut ix = self.index()?;
        ix.clock += 1;
        let clock = ix.clock;
        if let Some(e) = ix.entries.get_mut(id) {
            let p = self.dir.join("cache").join(&e.file);
            if e.sha == d.text_sha256 && e.reader == READER && p.exists() {
                e.used = clock;
                let b = zstd::decode_all(std::fs::File::open(&p)?)?;
                write_private(&self.dir.join("cache/index.json"), &serde_json::to_vec_pretty(&ix)?)?;
                return String::from_utf8(b)?.lines().filter(|l| !l.trim().is_empty()).map(|l| Ok(serde_json::from_str(l)?)).collect();
            }
        }
        let text = self.text(id)?;
        let sents = annotate(&text, self.opts.max_sentences)?;
        let body: String = sents.iter().map(|s| serde_json::to_string(s).unwrap() + "\n").collect();
        let z = zstd::encode_all(body.as_bytes(), 9)?;
        let file = format!("{}.jsonl.zst", file_of(id));
        write_private(&self.dir.join("cache").join(&file), &z)?;
        ix.entries.insert(id.to_string(), CacheEntry { sha: d.text_sha256.clone(), reader: READER.into(), file, bytes: z.len() as u64, used: clock, sentences: sents.len() });
        self.evict(&mut ix)?;
        write_private(&self.dir.join("cache/index.json"), &serde_json::to_vec_pretty(&ix)?)?;
        if let Some(h) = self.opts.hard_cases.clone() {
            let found: Vec<(String, Value, Vec<String>)> = sents.iter().filter(|s| !s.issues.is_empty()).map(|s| (s.text.clone(), json!({"corpus": self.key, "doc": id, "s": s.s}), s.issues.clone())).collect();
            hardcases::record(&h, &found, &d.licence, READER)?;
        }
        Ok(sents)
    }

    /// Keep the cache under its byte cap: the least recently used entries of documents that are not
    /// hot go first.
    fn evict(&self, ix: &mut CacheIndex) -> Result<()> {
        let hot: std::collections::BTreeSet<&str> = self.docs.iter().filter(|d| d.hot).map(|d| d.id.as_str()).collect();
        let mut total: u64 = ix.entries.values().map(|e| e.bytes).sum();
        let mut order: Vec<(u64, String)> = ix.entries.iter().filter(|(k, _)| !hot.contains(k.as_str())).map(|(k, e)| (e.used, k.clone())).collect();
        order.sort();
        for (_, k) in order {
            if total <= self.opts.cache_max_bytes {
                break;
            }
            if let Some(e) = ix.entries.remove(&k) {
                total -= e.bytes;
                let p = self.dir.join("cache").join(&e.file);
                if p.exists() {
                    std::fs::remove_file(p)?;
                }
            }
        }
        Ok(())
    }

    /// Cache entries (document ids), for reports.
    pub fn cache_ids(&self) -> Vec<String> {
        self.index().map(|ix| ix.entries.keys().cloned().collect()).unwrap_or_default()
    }
}

fn paper_doc(n: &crate::graph::Node, roles: &[&str]) -> Doc {
    let mut d = Doc::new(&format!("lib:{}", n.id), "paper", Origin::Library);
    d.title = n.title.clone();
    d.source_kind = "file".into();
    d.source = n.text_path.clone().unwrap_or_default();
    d.roles = roles.iter().map(|r| r.to_string()).collect();
    d.licence = n.licence.clone();
    d.licence_class = n.licence_class;
    d.licence_from = "library".into();
    d.local_only = n.text == "local";
    d.cite = json!({"library": n.id, "doi": n.doi, "arxiv": n.arxiv, "acl": n.acl, "openalex": n.openalex, "year": n.year, "authors": n.authors});
    d
}

/// A note on an author we hold no other work of: what the library knows (name, ids, profile
/// numbers, homepage, the paper). Our text, internal.
pub fn author_note(a: &crate::graph::AuthorRef, g: &Neighbourhood) -> String {
    let mut s = format!("# {}\n\n## Author\n\n", a.name);
    s.push_str(&format!("{} is an author of the paper «{}»", a.name, g.paper.title));
    if let Some(y) = g.paper.year {
        s.push_str(&format!(" ({y})"));
    }
    s.push_str(".\n\n");
    if let Some(h) = a.h_index {
        s.push_str(&format!("The h-index of {} in OpenAlex is {h}.\n\n", a.name));
    }
    if let Some(h) = &a.homepage {
        s.push_str(&format!("The scholarly profile of {} lists the homepage {h}.\n\n", a.name));
    }
    s.push_str(&format!("The library holds no other work by {}.\n", a.name));
    s
}

/// Register the cross corpus of a neighbourhood: the paper, its references, similar works, citing
/// works and works of the same authors (library texts, up to `max_docs`), and a note on every author
/// with no other work in the library. The first `hot` documents are parsed now and pinned; the rest
/// are parsed on request.
pub fn build(c: &mut Corpus, lib: &Library, g: &Neighbourhood, max_docs: usize, hot: usize) -> Result<Value> {
    let mut registered = 0;
    let mut skipped = vec![];
    let mut hot_ids = vec![];
    for (n, roles) in g.members() {
        if registered >= max_docs {
            skipped.push(json!({"id": n.id, "why": "max_docs"}));
            continue;
        }
        let Some(path) = n.text_path.clone() else {
            skipped.push(json!({"id": n.id, "why": "no text"}));
            continue;
        };
        let text = std::fs::read_to_string(&path).with_context(|| path.clone())?;
        let mut d = paper_doc(&n, &roles);
        if registered < hot {
            d.hot = true;
            hot_ids.push(d.id.clone());
        }
        c.register(d, &text)?;
        registered += 1;
    }
    let p = lib.get(&g.paper.id);
    let mut notes = 0;
    for a in &g.authors {
        let others = match (&a.openalex, p) {
            (Some(id), Some(p)) => lib.works.iter().any(|w| w.id != p.id && w.author_ids.iter().any(|x| x == id || id.ends_with(&format!("/{x}")))),
            _ => false,
        };
        if others {
            continue;
        }
        let mut d = Doc::new(&format!("author:{}", file_of(&a.name.to_lowercase())), "author", Origin::Notes);
        d.title = a.name.clone();
        d.roles = vec!["author".into()];
        d.licence = "our note (internal)".into();
        d.licence_from = "ours".into();
        d.cite = json!({"author": a.name, "openalex": a.openalex, "orcid": a.orcid, "homepage": a.homepage});
        c.register(d, &author_note(a, g))?;
        notes += 1;
    }
    let mut parsed = 0;
    for id in &hot_ids {
        parsed += c.parsed(id)?.len();
    }
    Ok(json!({"corpus": c.dir, "registered": registered, "author_notes": notes, "hot": hot_ids, "hot_sentences": parsed, "skipped": skipped, "documents": c.docs.len()}))
}

/// One explanation candidate for a term: a sentence of another document of the corpus.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Explanation {
    pub doc: String,
    pub kind: String,
    pub title: String,
    pub s: usize,
    pub text: String,
    pub score: f64,
    pub public_admissible: bool,
}

pub fn has_term(forms: &[String], term: &str) -> Option<usize> {
    let cased = term.chars().any(char::is_uppercase);
    let parts: Vec<&str> = term.split_whitespace().collect();
    (0..forms.len()).find(|&i| parts.iter().enumerate().all(|(k, p)| forms.get(i + k).is_some_and(|f| if cased { f == p } else { f.eq_ignore_ascii_case(p) })))
}

/// How well a sentence (its word forms) explains `term`, or `None` when the term does not occur:
/// an acronym's expansion +3, «T is/are/means/refers/denotes/stands/,» +2, «(T)» +2, a reference
/// document +1 or a similar one +0.5, at most 40 words +0.5, on a base of 1.
pub fn explain_score(forms: &[String], term: &str, roles: &[String]) -> Option<f64> {
    let i = has_term(forms, term)?;
    let n = term.split_whitespace().count();
    let mut score = 1.0;
    if term.len() <= 8 && term.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) && mova_whale::knowledge::expansion(term, forms).is_some() {
        score += 3.0;
    }
    let next = forms.get(i + n).map(|w| w.to_lowercase()).unwrap_or_default();
    if matches!(next.as_str(), "is" | "are" | "means" | "refers" | "denotes" | "stands" | ",") {
        score += 2.0;
    }
    if i > 0 && forms[i - 1] == "(" && forms.get(i + n).is_some_and(|w| w == ")") {
        score += 2.0;
    }
    if roles.iter().any(|r| r == "reference") {
        score += 1.0;
    } else if roles.iter().any(|r| r == "similar") {
        score += 0.5;
    }
    if forms.len() <= 40 {
        score += 0.5;
    }
    Some(score)
}

/// Sentences that explain `term`, from documents of the given kinds other than `exclude`. Lazy: a
/// document is parsed only when its text contains the term (then it is cached). An acronym's
/// expansion, «T is/are/means/refers …», «… (T)» and apposition score highest; references rank above
/// other neighbours; short sentences above long. At most `k`, best first.
pub fn explain(c: &mut Corpus, term: &str, exclude: Option<&str>, kinds: &[&str], k: usize) -> Result<Vec<Explanation>> {
    let mut out = vec![];
    let cased = term.chars().any(char::is_uppercase);
    let ids: Vec<String> = c.docs.iter().filter(|d| Some(d.id.as_str()) != exclude && kinds.contains(&d.kind.as_str())).map(|d| d.id.clone()).collect();
    for id in ids {
        let t = c.text(&id)?;
        let hit = if cased { t.contains(term) } else { t.to_lowercase().contains(&term.to_lowercase()) };
        if !hit {
            continue;
        }
        let d = c.doc(&id).unwrap().clone();
        for s in c.parsed(&id)? {
            let forms: Vec<String> = s.words.iter().map(|w| w.0.clone()).collect();
            let Some(score) = explain_score(&forms, term, &d.roles) else { continue };
            out.push(Explanation { doc: d.id.clone(), kind: d.kind.clone(), title: d.title.clone(), s: s.s, text: s.text.clone(), score, public_admissible: d.public_admissible });
        }
    }
    out.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.doc.cmp(&b.doc)).then(a.s.cmp(&b.s)));
    out.truncate(k);
    Ok(out)
}

/// Refuse a document for any public output unless it is public-admissible. Pages Sea Spider fetched
/// (and their trees), consultant drafts and our notes are refused whatever licence they state.
pub fn assert_public(d: &Doc) -> Result<()> {
    if d.origin == Origin::Seaspider || d.kind == "web" || d.source_kind == "seaspider" {
        bail!("{} was fetched by Sea Spider: its page and its annotations stay in the closed corpus and never go public", d.id);
    }
    if d.origin == Origin::Sol || d.kind == "sol" {
        bail!("{} is a consultant's draft: internal only", d.id);
    }
    if d.origin == Origin::Notes || d.kind == "author" {
        bail!("{} is an internal note", d.id);
    }
    if !licence::public_admissible(d.origin, d.licence_class, d.local_only) || !d.public_admissible {
        bail!("{} is not under an open licence admitted for public use ({})", d.id, d.licence);
    }
    Ok(())
}

/// Write the annotations of the public-admissible documents to `out` (a new directory) for a public
/// corpus. A document named in `only` that is not admissible is an error and nothing is written;
/// without `only`, refused documents are listed and nothing of theirs is written.
pub fn public_export(c: &mut Corpus, out: &Path, only: &[String]) -> Result<Value> {
    if out.exists() {
        bail!("{} exists: a public export goes to a new directory", out.display());
    }
    let chosen: Vec<Doc> = if only.is_empty() { c.docs.clone() } else { only.iter().map(|id| c.doc(id).cloned().ok_or_else(|| anyhow!("{id} is not in the corpus"))).collect::<Result<_>>()? };
    let mut ok = vec![];
    let mut refused = vec![];
    for d in chosen {
        match assert_public(&d) {
            Ok(()) => ok.push(d),
            Err(e) if only.is_empty() => refused.push(json!({"id": d.id, "why": e.to_string()})),
            Err(e) => return Err(e),
        }
    }
    let mut files: Vec<(String, String)> = vec![];
    for d in &ok {
        let body: String = c.parsed(&d.id)?.iter().map(|s| serde_json::to_string(s).unwrap() + "\n").collect();
        files.push((format!("{}.jsonl", file_of(&d.id)), body));
    }
    std::fs::create_dir_all(out.join("docs"))?;
    let mut docs = String::new();
    for d in &ok {
        let mut v = serde_json::to_value(d)?;
        v["source"] = json!(null);
        docs.push_str(&(v.to_string() + "\n"));
    }
    for (f, b) in files {
        std::fs::write(out.join("docs").join(f), b)?;
    }
    std::fs::write(out.join("documents.jsonl"), docs)?;
    Ok(json!({"exported": ok.len(), "refused": refused}))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::config::Limits;
    use crate::graph;
    use crate::library::tests::{fixture, tmp};

    /// Tests that need Mova's model run only with `WORLD_EN_MODEL` set.
    pub fn model() -> bool {
        let ok = std::env::var_os("WORLD_EN_MODEL").is_some_and(|p| Path::new(&p).is_file());
        if !ok {
            eprintln!("skip: WORLD_EN_MODEL is not set to a Mova model (en.bin)");
        }
        ok
    }

    pub fn built_with(name: &str, hot: usize, cache: u64) -> (PathBuf, Corpus) {
        let d = tmp(name);
        let lib = Library::load(&fixture(&d)).unwrap();
        let g = graph::build(&lib, "pa-2017-error-types", &Limits::default()).unwrap();
        let mut c = Corpus::open(&d.join("corpora"), "pa-2017-error-types", &g.paper.id, Options { max_sentences: 1500, cache_max_bytes: cache, hard_cases: Some(d.join("hard-cases.jsonl")) }).unwrap();
        build(&mut c, &lib, &g, 80, hot).unwrap();
        (d, c)
    }

    pub fn built(name: &str) -> (PathBuf, Corpus) {
        built_with(name, 1, 1 << 30)
    }

    pub fn web_doc(id: &str, licence: &str) -> Doc {
        let mut d = Doc::new(id, "web", Origin::Seaspider);
        d.title = "A blog".into();
        d.roles = vec!["author_page".into()];
        d.licence = licence.into();
        d.licence_class = licence::classify(licence);
        d.licence_from = "page".into();
        d.local_only = false;
        d.public_admissible = true;
        d.cite = json!({"url": "https://blog.example.org/gec"});
        d
    }

    #[test]
    fn registers_lazily_parses_on_request_and_bounds_the_cache() {
        if !model() {
            return;
        }
        let (d, mut c) = built("corpus");
        // Three library texts (the paper, two references), one note for the author without other works.
        assert_eq!(c.docs.iter().filter(|x| x.kind == "paper").count(), 3);
        assert!(c.doc("author:bo-qu").is_some() && c.doc("author:ann-pa").is_none());
        // Only the hot paper is parsed at build time; the references wait for a request.
        assert_eq!(c.cache_ids(), vec!["lib:pa-2017-error-types"]);
        assert!(!c.cached("lib:ra-2015-gec-evaluation"));
        let p = c.doc("lib:pa-2017-error-types").unwrap().clone();
        assert!(p.internal_only && p.public_admissible && p.hot);
        assert!(!c.doc("lib:rb-2012-m2-scorer").unwrap().public_admissible);
        let s = c.parsed("lib:pa-2017-error-types").unwrap();
        assert!(!s.is_empty() && s.iter().all(|x| !x.words.is_empty()));
        assert!(!s.iter().any(|x| x.text.contains("Ray A. 2015")));
        let e = explain(&mut c, "GEC", Some("lib:pa-2017-error-types"), &["paper"], 3).unwrap();
        assert_eq!(e[0].doc, "lib:ra-2015-gec-evaluation");
        assert!(e[0].text.contains("Grammatical Error Correction (GEC)"));
        // The reference was parsed on request because it contains the term; the local-only one too
        // only if it does («GEC» is not in it).
        assert!(c.cached("lib:ra-2015-gec-evaluation") && !c.cached("lib:rb-2012-m2-scorer"));
        // Rebuilding with the same texts gives the same registry (byte-identical).
        let before = std::fs::read(c.dir.join("documents.jsonl")).unwrap();
        let lib = Library::load(&fixture_cfg(&d)).unwrap();
        let g = graph::build(&lib, "pa-2017-error-types", &Limits::default()).unwrap();
        build(&mut c, &lib, &g, 80, 1).unwrap();
        assert_eq!(before, std::fs::read(c.dir.join("documents.jsonl")).unwrap());
        // A cache of 1 byte keeps only the pinned hot document.
        c.opts.cache_max_bytes = 1;
        c.parsed("lib:rb-2012-m2-scorer").unwrap();
        assert_eq!(c.cache_ids(), vec!["lib:pa-2017-error-types"]);
        // A changed original is noticed, not silently parsed.
        let mut c2 = Corpus::load(&c.dir, Options::default()).unwrap();
        std::fs::write(d.join("texts/r/ra/ra-2015-gec-evaluation/text.md"), "# Changed\n").unwrap();
        assert!(c2.parsed("lib:ra-2015-gec-evaluation").unwrap_err().to_string().contains("changed"));
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn web_pages_stay_internal_and_bad_parses_are_hard_cases() {
        if !model() {
            return;
        }
        let (d, mut c) = built("corpusweb");
        // A web page under CC BY is still internal and never exported (operator's rule).
        c.register(web_doc("web:abc", "CC BY 4.0"), "# A blog\n\n## Post\n\nThe M2 scorer is a metric for grammatical error correction systems.\n").unwrap();
        let w = c.doc("web:abc").unwrap().clone();
        assert!(w.internal_only && !w.public_admissible);
        assert!(assert_public(&w).is_err());
        let out = d.join("public");
        assert!(public_export(&mut c, &out, &["web:abc".into()]).unwrap_err().to_string().contains("Sea Spider"));
        assert!(!out.exists());
        let r = public_export(&mut c, &out, &[]).unwrap();
        assert_eq!(r["exported"], 2);
        assert!(r["refused"].as_array().unwrap().iter().any(|x| x["id"] == "web:abc"));
        assert!(!out.join("docs").join(format!("{}.jsonl", file_of("web:abc"))).exists());
        // Whatever the hard-cases list holds came from real parses of this corpus.
        let st = hardcases::state(&d.join("hard-cases.jsonl")).unwrap();
        assert!(st.values().all(|v| v["where"]["corpus"] == "pa-2017-error-types"));
        let _ = std::fs::remove_dir_all(d);
    }

    fn fixture_cfg(d: &Path) -> crate::config::Library {
        crate::config::Library { export: d.join("export.jsonl"), citations: d.join("citations.tsv"), refs: d.join("refs.jsonl"), text_root: d.join("texts"), topics: Some(d.join("topics.tsv")), authors: Some(d.join("authors.tsv")) }
    }

    #[test]
    fn a_forged_web_record_is_refused_for_public_use() {
        // Red control of the export gate without the model: a web record that claims to be
        // admissible (as an older tool might have written it) is still refused.
        let w = web_doc("web:forged", "CC0");
        assert!(w.public_admissible);
        assert!(assert_public(&w).unwrap_err().to_string().contains("Sea Spider"));
        let mut s = web_doc("sol:draft", "CC BY 4.0");
        s.kind = "sol".into();
        s.origin = Origin::Sol;
        assert!(assert_public(&s).is_err());
        let mut n = web_doc("author:x", "CC0");
        n.kind = "author".into();
        n.origin = Origin::Notes;
        assert!(assert_public(&n).is_err());
    }
}
