//! The tree store: millions of book sentences with their parse trees per model version, metadata and search indexes.
//!
//!   world store build <book-list> --out <dir> [--threads T] [--limit N]
//!   world store versions <dir>
//!   world store query <dir> "<query>" [--model <hash>] [--limit N] [--show N]
//!   world store update <dir> [--branch B]          (after a code change: a delta version with the changed trees)
//!   world store ref <dir> [<name> [<hash>]]        (branch references: refs/main, refs/exp-…)
//!
//! Non-blocking: versions are only added, never rewritten; a new version's files are written atomically (temporary
//! file + rename), its index before its line in versions.tsv, the branch ref last. Readers take a branch's hash once
//! and work on that snapshot; experiments update their own branch (`--branch exp-…`) from main while main is rebuilt.
//!   world store set save <dir> <name> "<query>" | set load <dir> <name> | set list <dir>
//!
//! Sets: sentence sets (metadata `book=N`, `sample=K/SEED`) are stable and kept; sets defined by tree features keep
//! their query and the model hash — on a descendant model only the sentences whose trees changed are checked again
//! ("Alice acts" carries over between runs); on an unrelated model the ids are recomputed from the query.
//!
//! Shuffled blocks: every book is cut into blocks of ~400 sentences at paragraph ends (chapters, tens of pages); the
//! blocks of all books are shuffled with a key of the corpus and the day (`order.tsv`, `order.bin`) — a new order
//! every day and on every corpus change. `prefix=1%` takes the first percent of that order: a mixture of pieces of
//! many books and genres, so an experiment on a percent of the corpus already sees variety.
//!
//! Layout of `<dir>`:
//! - `books.tsv` — book id, path, Gutenberg id, group;
//! - `text.bin` + `meta.bin` — sentence texts and, per sentence (24 bytes): book u32, paragraph u32, sentence in
//!   the paragraph u16, tokens u16, text offset u64, text length u32;
//! - `lemmas.tsv` — lemma id → lemma;
//! - `versions.tsv` — model hash (8 bytes, hex), parent hash, kind (ast), time, sentences stored, description;
//!   the hash is taken over the model file, the induced rules and the code commit;
//! - `trees-<hash>.bin` — tree records, per token 8 bytes: upos u8, rel u8, head u16, tag (XPOS) u8, lemma id u24;
//! - `dict-<hash>.bin` — per stored sentence (16 bytes): sentence u32, tree offset u64, length u32, sorted by
//!   sentence; a derived version stores only the trees that differ from its parent, the rest is looked up in the
//!   parent (version = hash + changes from the base);
//! - `index-<hash>.bin` + `features-<hash>.tsv` — postings per structural feature (relation, part of speech, number
//!   of predicates, number of subjects): (sentence u32, count u8).
//!
//! Query: a conjunction of `feature<op>count` terms, op one of >=, <=, =, > (`rel:nsubj>=1 pred>=2`); features are
//! `rel:<relation>`, `upos:<POS>`, `pred` (verbs heading a clause), `subj` (nsubj + nsubj:pass + csubj), `subj:<lemma>`
//! (who acts: `subj:alice>=1`); metadata terms `book=N`, `sample=K/SEED`.

use std::collections::BTreeMap;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Context, Result, bail};
use en::gram::{Rel, UPos};

/// Paragraphs and sentences of a book body: (paragraph number, sentence number in the paragraph, text).
fn sentences_numbered(body: &str) -> Vec<(u32, u16, String)> {
    let mut out = Vec::new();
    let mut p = 0u32;
    let body = body.replace("\r\n", "\n");
    for para in body.split("\n\n") {
        let para = para.split_whitespace().collect::<Vec<_>>().join(" ");
        if para.len() < 20 || para.chars().filter(|c| c.is_uppercase()).count() * 2 > para.len() {
            continue;
        }
        p += 1;
        for (k, s) in crate::babi::split_sentences(&para).into_iter().filter(|s| (3..=60).contains(&s.split_whitespace().count())).enumerate() {
            out.push((p, k as u16, s));
        }
    }
    out
}

fn fnv(bytes: &[u8], mut h: u64) -> u64 {
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Write a file atomically: a temporary file in the same folder, then a rename — readers never see half a file.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = path.with_extension(format!("tmp-{}", std::process::id()));
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

/// A branch reference (`<dir>/refs/<name>` → version hash).
pub fn ref_get(dir: &Path, name: &str) -> Option<String> {
    std::fs::read_to_string(dir.join("refs").join(name)).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

pub fn ref_set(dir: &Path, name: &str, hash: &str) -> Result<()> {
    std::fs::create_dir_all(dir.join("refs"))?;
    write_atomic(&dir.join("refs").join(name), format!("{hash}\n").as_bytes())
}

/// The model hash: the en model file, the shipped induced rules and the code commit.
pub fn model_hash() -> Result<String> {
    let mut h = 0xcbf29ce484222325u64;
    h = fnv(&std::fs::read(crate::tree::model_path()).context("en model")?, h);
    h = fnv(include_str!("../data/rules-induced.tsv").as_bytes(), h);
    let commit = std::process::Command::new("git").args(["-C", env!("CARGO_MANIFEST_DIR"), "rev-parse", "HEAD"]).output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
    h = fnv(commit.as_bytes(), h);
    Ok(format!("{h:016x}"))
}

/// `world store build`.
pub fn build(list: &Path, dir: &Path, threads: usize, limit: usize) -> Result<()> {
    let a = crate::tree::annotator()?;
    let hash = model_hash()?;
    std::fs::create_dir_all(dir)?;
    let books: Vec<String> = std::fs::read_to_string(list)?.lines().map(String::from).collect();
    let mut bt = std::io::BufWriter::new(std::fs::File::create(dir.join("books.tsv"))?);
    writeln!(bt, "id\tpath\tgutenberg\tgroup")?;
    for (i, b) in books.iter().enumerate() {
        let p = Path::new(b);
        let pg = p.file_stem().map(|s| s.to_string_lossy().trim_start_matches("pg").to_string()).unwrap_or_default();
        let group = p.parent().and_then(|x| x.file_name()).map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        writeln!(bt, "{i}\t{b}\t{pg}\t{group}")?;
    }
    bt.flush()?;
    // books are parsed in parallel; finished books are written to disk in book order as soon as all earlier books
    // are written (deterministic, memory bounded by the books in flight, not by the corpus)
    struct Sink {
        next: usize,
        pending: BTreeMap<usize, (Vec<(u32, u16, String)>, Vec<Vec<[u8; 8]>>)>,
        text: std::io::BufWriter<std::fs::File>,
        trees: std::io::BufWriter<std::fs::File>,
        meta: std::io::BufWriter<std::fs::File>,
        dict: std::io::BufWriter<std::fs::File>,
        text_off: u64,
        tree_off: u64,
        idx: u32,
    }
    impl Sink {
        fn flush_ready(&mut self) -> std::io::Result<()> {
            while let Some((sents, trees)) = self.pending.remove(&self.next) {
                let book = self.next as u32;
                for ((p, n, s), rec) in sents.into_iter().zip(trees) {
                    let mut m = [0u8; 24];
                    m[0..4].copy_from_slice(&book.to_le_bytes());
                    m[4..8].copy_from_slice(&p.to_le_bytes());
                    m[8..10].copy_from_slice(&n.to_le_bytes());
                    m[10..12].copy_from_slice(&(rec.len() as u16).to_le_bytes());
                    m[12..20].copy_from_slice(&self.text_off.to_le_bytes());
                    m[20..24].copy_from_slice(&(s.len() as u32).to_le_bytes());
                    self.meta.write_all(&m)?;
                    self.text.write_all(s.as_bytes())?;
                    self.text_off += s.len() as u64;
                    self.dict.write_all(&self.idx.to_le_bytes())?;
                    self.dict.write_all(&self.tree_off.to_le_bytes())?;
                    self.dict.write_all(&((rec.len() * 8) as u32).to_le_bytes())?;
                    for r in &rec {
                        self.trees.write_all(r)?;
                    }
                    self.tree_off += (rec.len() * 8) as u64;
                    self.idx += 1;
                }
                self.next += 1;
            }
            Ok(())
        }
    }
    let open = |n: String| -> Result<std::io::BufWriter<std::fs::File>> { Ok(std::io::BufWriter::new(std::fs::File::create(dir.join(n))?)) };
    let sink = Mutex::new(Sink {
        next: 0,
        pending: BTreeMap::new(),
        text: open("text.bin".into())?,
        trees: open(format!("trees-{hash}.bin"))?,
        meta: open("meta.bin".into())?,
        dict: open(format!("dict-{hash}.bin"))?,
        text_off: 0,
        tree_off: 0,
        idx: 0,
    });
    let lemmas: Mutex<BTreeMap<String, u32>> = Mutex::new(BTreeMap::new());
    let next = AtomicUsize::new(0);
    let count = AtomicUsize::new(0);
    let t0 = std::time::Instant::now();
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1) {
            sc.spawn(|| loop {
                let k = next.fetch_add(1, Ordering::SeqCst);
                let Some(b) = books.get(k) else { break };
                let mut sents = Vec::new();
                let mut trees = Vec::new();
                if count.load(Ordering::SeqCst) < limit {
                    if let Ok(t) = std::fs::read_to_string(b) {
                        for (p, n, s) in sentences_numbered(crate::events::book_body(&t)) {
                            let forms: Vec<String> = a.tokenize(&s).into_iter().map(|t| t.form).collect();
                            if forms.is_empty() || forms.len() > 250 {
                                continue;
                            }
                            let mut ws = a.annotate(&forms);
                            crate::rerank::repair_words(&mut ws);
                            let ids: Vec<u32> = {
                                let mut lx = lemmas.lock().unwrap();
                                ws.iter().map(|w| {
                                    let n = lx.len() as u32;
                                    *lx.entry(w.lemma.to_lowercase()).or_insert(n)
                                }).collect()
                            };
                            let rec: Vec<[u8; 8]> = ws.iter().zip(&ids).map(|(w, id)| {
                                let mut r = [0u8; 8];
                                r[0] = UPos::ALL.iter().position(|u| *u == w.upos).unwrap_or(0) as u8;
                                r[1] = Rel::ALL.iter().position(|x| *x == w.rel).unwrap_or(0) as u8;
                                r[2..4].copy_from_slice(&(w.head as u16).to_le_bytes());
                                r[4] = en::gram::Tag::ALL.iter().position(|t| *t == w.tag).unwrap_or(0) as u8;
                                r[5..8].copy_from_slice(&id.to_le_bytes()[0..3]);
                                r
                            }).collect();
                            sents.push((p, n, s));
                            trees.push(rec);
                        }
                    }
                }
                let n = count.fetch_add(sents.len(), Ordering::SeqCst) + sents.len();
                {
                    let mut sk = sink.lock().unwrap();
                    sk.pending.insert(k, (sents, trees));
                    let _ = sk.flush_ready();
                }
                if k % 200 == 0 {
                    eprintln!("[{k}/{}] books, {n} sentences, {:.0} s", books.len(), t0.elapsed().as_secs_f64());
                }
            });
        }
    });
    let mut sk = sink.into_inner().unwrap();
    sk.flush_ready()?;
    for w in [&mut sk.text, &mut sk.trees, &mut sk.meta, &mut sk.dict] {
        w.flush()?;
    }
    let (n_sent, tree_bytes) = (sk.idx as usize, sk.tree_off);
    let lx = lemmas.into_inner().unwrap();
    let mut lv: Vec<(&String, &u32)> = lx.iter().collect();
    lv.sort_by_key(|x| *x.1);
    std::fs::write(dir.join("lemmas.tsv"), lv.iter().map(|(l, i)| format!("{i}\t{l}\n")).collect::<String>())?;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut vf = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("versions.tsv"))?;
    writeln!(vf, "{hash}\t-\tast\t{now}\t{n_sent}\tbase: en model + induced rules + commit (world store build)")?;
    index(dir, &hash)?;
    ref_set(dir, "main", &hash)?;
    println!("store {}: {n_sent} sentences from {} books, {tree_bytes} tree bytes, model {hash}, {:.0} s", dir.display(), books.len(), t0.elapsed().as_secs_f64());
    Ok(())
}

/// One token record → (upos, rel, head, lemma id); the tag is read by `decode_tag`.
pub fn decode(r: &[u8]) -> (UPos, Rel, usize, u32) {
    (UPos::ALL[r[0] as usize], Rel::ALL[r[1] as usize], u16::from_le_bytes([r[2], r[3]]) as usize, u32::from_le_bytes([r[5], r[6], r[7], 0]))
}

pub fn decode_tag(r: &[u8]) -> en::gram::Tag {
    en::gram::Tag::ALL.get(r[4] as usize).copied().unwrap_or(en::gram::Tag::NN)
}

/// A tree of a sentence in a version (following parents for sentences the version did not change).
pub struct Store {
    pub(crate) dir: PathBuf,
    meta: Vec<u8>,
    pub(crate) versions: Vec<(String, String)>,
    main: Option<String>,
    /// per version: the dictionary (loaded once) and the open trees file (read at positions, thread-safe)
    cache: Mutex<BTreeMap<String, std::sync::Arc<(Vec<u8>, std::fs::File)>>>,
    text_file: Mutex<Option<std::sync::Arc<std::fs::File>>>,
}

impl Store {
    pub fn open(dir: &Path) -> Result<Store> {
        let meta = std::fs::read(dir.join("meta.bin")).context("meta.bin")?;
        let versions = std::fs::read_to_string(dir.join("versions.tsv")).context("versions.tsv")?.lines().filter_map(|l| {
            let c: Vec<&str> = l.split('\t').collect();
            (c.len() >= 2).then(|| (c[0].to_string(), c[1].to_string()))
        }).collect();
        Ok(Store { dir: dir.to_path_buf(), meta, versions, main: ref_get(dir, "main"), cache: Mutex::new(BTreeMap::new()), text_file: Mutex::new(None) })
    }
    pub fn len(&self) -> usize {
        self.meta.len() / 24
    }
    /// The main branch (`refs/main`), else the last version — a reader takes it once and works on that snapshot.
    pub fn latest(&self) -> Option<&str> {
        self.main.as_deref().or_else(|| self.versions.last().map(|v| v.0.as_str()))
    }
    /// The dictionary and trees file of a version, loaded once per store handle (tried 02.10: re-reading a 345 MB
    /// dictionary for every sentence stalled training).
    fn version(&self, h: &str) -> Result<std::sync::Arc<(Vec<u8>, std::fs::File)>> {
        let mut c = self.cache.lock().unwrap();
        if let Some(v) = c.get(h) {
            return Ok(v.clone());
        }
        let d = std::fs::read(self.dir.join(format!("dict-{h}.bin")))?;
        let f = std::fs::File::open(self.dir.join(format!("trees-{h}.bin")))?;
        let v = std::sync::Arc::new((d, f));
        c.insert(h.to_string(), v.clone());
        Ok(v)
    }

    /// Raw records of sentence `i` in version `hash`, following parents.
    pub fn records(&self, hash: &str, i: usize) -> Result<Vec<[u8; 8]>> {
        use std::os::unix::fs::FileExt;
        let mut h = hash.to_string();
        loop {
            let v = self.version(&h)?;
            let d = &v.0;
            let n = d.len() / 16;
            let at = |k: usize| u32::from_le_bytes(d[k * 16..k * 16 + 4].try_into().unwrap()) as usize;
            let (mut lo, mut hi) = (0usize, n);
            while lo < hi {
                let mid = (lo + hi) / 2;
                if at(mid) < i { lo = mid + 1 } else { hi = mid }
            }
            if lo < n && at(lo) == i {
                let off = u64::from_le_bytes(d[lo * 16 + 4..lo * 16 + 12].try_into()?);
                let len = u32::from_le_bytes(d[lo * 16 + 12..lo * 16 + 16].try_into()?) as usize;
                let mut buf = vec![0u8; len];
                v.1.read_exact_at(&mut buf, off)?;
                return Ok(buf.chunks(8).map(|c| c.try_into().unwrap()).collect());
            }
            match self.versions.iter().find(|x| x.0 == h).map(|x| x.1.clone()) {
                Some(p) if p != "-" => h = p,
                _ => bail!("sentence {i} not in version {hash} or its parents"),
            }
        }
    }

    fn m(&self, i: usize) -> &[u8] {
        &self.meta[i * 24..i * 24 + 24]
    }
    pub fn text(&self, i: usize) -> Result<String> {
        use std::os::unix::fs::FileExt;
        let m = self.m(i);
        let off = u64::from_le_bytes(m[12..20].try_into()?);
        let len = u32::from_le_bytes(m[20..24].try_into()?) as usize;
        let f = {
            let mut g = self.text_file.lock().unwrap();
            if g.is_none() {
                *g = Some(std::sync::Arc::new(std::fs::File::open(self.dir.join("text.bin"))?));
            }
            g.clone().unwrap()
        };
        let mut buf = vec![0u8; len];
        f.read_exact_at(&mut buf, off)?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
    /// (book, paragraph, sentence in the paragraph)
    pub fn place(&self, i: usize) -> (u32, u32, u16) {
        let m = self.m(i);
        (u32::from_le_bytes(m[0..4].try_into().unwrap()), u32::from_le_bytes(m[4..8].try_into().unwrap()), u16::from_le_bytes(m[8..10].try_into().unwrap()))
    }
    /// Tree records of sentence `i` in version `hash`: (upos, rel, head, lemma id) per token.
    pub fn tree(&self, hash: &str, i: usize) -> Result<Vec<(UPos, Rel, usize, u32)>> {
        Ok(self.records(hash, i)?.iter().map(|r| decode(r)).collect())
    }
}

/// The lemma table of a version (`lemmas-<hash>.tsv` for delta versions, `lemmas.tsv` for the base) — each version
/// has its own, so branches built in parallel never write the same file.
pub fn lemma_table_for(dir: &Path, hash: &str) -> Vec<String> {
    let p = dir.join(format!("lemmas-{hash}.tsv"));
    lemma_file(if p.exists() { p } else { dir.join("lemmas.tsv") })
}

/// The lemma table of the main version.
pub fn lemma_table(dir: &Path) -> Vec<String> {
    match Store::open(dir).ok().and_then(|s| s.latest().map(String::from)) {
        Some(h) => lemma_table_for(dir, &h),
        None => lemma_file(dir.join("lemmas.tsv")),
    }
}

fn lemma_file(path: PathBuf) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    if let Ok(t) = std::fs::read_to_string(path) {
        for l in t.lines() {
            if let Some((i, x)) = l.split_once('\t') {
                let i: usize = i.parse().unwrap_or(0);
                if v.len() <= i {
                    v.resize(i + 1, String::new());
                }
                v[i] = x.to_string();
            }
        }
    }
    v
}

/// Features with the subject's lemma ("subj:alice" — who acts) on top of the structural ones.
fn features_lex(t: &[(UPos, Rel, usize, u32)], lemmas: &[String]) -> BTreeMap<String, u8> {
    let mut f = features(t);
    for (_, r, _, l) in t {
        if matches!(r, Rel::Nsubj | Rel::NsubjPass) {
            if let Some(x) = lemmas.get(*l as usize).filter(|x| x.chars().all(char::is_alphabetic)) {
                let e = f.entry(format!("subj:{x}")).or_default();
                *e = e.saturating_add(1);
            }
        }
    }
    f
}

/// Structural features of a tree: (feature, count).
fn features(t: &[(UPos, Rel, usize, u32)]) -> BTreeMap<String, u8> {
    let mut f: BTreeMap<String, u8> = BTreeMap::new();
    let mut add = |k: String| {
        let e = f.entry(k).or_default();
        *e = e.saturating_add(1);
    };
    for (u, r, _, _) in t {
        add(format!("rel:{r}"));
        add(format!("upos:{}", u.name()));
        if *u == UPos::VERB && matches!(r, Rel::Root | Rel::Conj | Rel::Ccomp | Rel::Advcl | Rel::Xcomp | Rel::Parataxis | Rel::AclRelcl) {
            add("pred".into());
        }
        if matches!(r, Rel::Nsubj | Rel::NsubjPass | Rel::Csubj) {
            add("subj".into());
        }
    }
    f
}

/// Build the feature index of a version.
pub fn index(dir: &Path, hash: &str) -> Result<()> {
    let st = Store::open(dir)?;
    let lemmas = lemma_table_for(dir, hash);
    // trees are read sequentially (the dictionary of a base version is in file order), not loaded whole
    let mut trees = std::io::BufReader::new(std::fs::File::open(dir.join(format!("trees-{hash}.bin")))?);
    let d = std::fs::read(dir.join(format!("dict-{hash}.bin")))?;
    let mut postings: BTreeMap<String, Vec<(u32, u8)>> = BTreeMap::new();
    let mut pos = 0u64;
    let mut buf = Vec::new();
    for k in 0..d.len() / 16 {
        let i = u32::from_le_bytes(d[k * 16..k * 16 + 4].try_into()?);
        let off = u64::from_le_bytes(d[k * 16 + 4..k * 16 + 12].try_into()?);
        let len = u32::from_le_bytes(d[k * 16 + 12..k * 16 + 16].try_into()?) as usize;
        if off != pos {
            trees.seek(SeekFrom::Start(off))?;
        }
        buf.resize(len, 0);
        trees.read_exact(&mut buf)?;
        pos = off + len as u64;
        let t: Vec<(UPos, Rel, usize, u32)> = buf.chunks(8).map(decode).collect();
        for (f, c) in features_lex(&t, &lemmas) {
            postings.entry(f).or_default().push((i, c));
        }
    }
    let mut bin = Vec::new();
    let mut tsv = String::new();
    for (f, ps) in &postings {
        tsv += &format!("{f}\t{}\t{}\n", bin.len(), ps.len());
        for (i, c) in ps {
            bin.extend_from_slice(&i.to_le_bytes());
            bin.push(*c);
        }
    }
    write_atomic(&dir.join(format!("index-{hash}.bin")), &bin)?;
    write_atomic(&dir.join(format!("features-{hash}.tsv")), tsv.as_bytes())?;
    let _ = st.len();
    Ok(())
}

/// `world store query`.
pub fn query(dir: &Path, q: &str, model: Option<&str>, limit: usize, show: usize) -> Result<()> {
    let st = Store::open(dir)?;
    let hash = model.map(String::from).or_else(|| st.latest().map(String::from)).context("no version in the store")?;
    let cand = select(dir, q, &hash)?;
    println!("model {hash}: {} of {} sentences match «{q}»", cand.len(), st.len());
    let books: Vec<String> = std::fs::read_to_string(dir.join("books.tsv")).unwrap_or_default().lines().skip(1).map(|l| l.split('\t').nth(3).unwrap_or("").to_string()).collect();
    for &i in cand.iter().take(limit) {
        let (b, p, n) = st.place(i as usize);
        println!("  #{i} book {b} ({}) paragraph {p} sentence {n}: {}", books.get(b as usize).map(String::as_str).unwrap_or(""), st.text(i as usize)?);
        if show > 0 && (i as usize) < st.len() {
            let t = st.tree(&hash, i as usize)?;
            println!("    {}", t.iter().enumerate().map(|(k, (u, r, h, _))| format!("{}:{}:{}→{h}", k + 1, u.name(), r)).collect::<Vec<_>>().join(" "));
        }
    }
    Ok(())
}

/// `world store versions`.
pub fn versions(dir: &Path) -> Result<()> {
    print!("{}", std::fs::read_to_string(dir.join("versions.tsv"))?);
    Ok(())
}

/// Sentence ids matching a query in a version (the engine of `query`, returned instead of printed).
pub fn select(dir: &Path, q: &str, hash: &str) -> Result<Vec<u32>> {
    let st = Store::open(dir)?;
    let feats: BTreeMap<String, (usize, usize)> = std::fs::read_to_string(dir.join(format!("features-{hash}.tsv")))?.lines().filter_map(|l| {
        let c: Vec<&str> = l.split('\t').collect();
        (c.len() == 3).then(|| (c[0].to_string(), (c[1].parse().unwrap_or(0), c[2].parse().unwrap_or(0))))
    }).collect();
    let bin = std::fs::read(dir.join(format!("index-{hash}.bin")))?;
    let posting = |f: &str| -> BTreeMap<u32, u8> {
        feats.get(f).map(|&(off, n)| (0..n).map(|k| {
            let b = &bin[off + k * 5..off + k * 5 + 5];
            (u32::from_le_bytes(b[0..4].try_into().unwrap()), b[4])
        }).collect()).unwrap_or_default()
    };
    let ok = |op: &str, have: u8, want: u8| match op { ">=" => have >= want, "<=" => have <= want, ">" => have > want, _ => have == want };
    let mut cand: Vec<u32> = (0..st.len() as u32).collect();
    for t in q.split_whitespace() {
        // metadata terms are stable over versions: book=N, book<N, sample=K/SEED
        if let Some(v) = t.strip_prefix("book=") {
            let b: u32 = v.parse()?;
            cand.retain(|&i| st.place(i as usize).0 == b);
            continue;
        }
        if let Some(v) = t.strip_prefix("prefix=") {
            let pct: f64 = v.trim_end_matches('%').parse()?;
            let have: std::collections::BTreeSet<u32> = cand.iter().copied().collect();
            // keep the shuffled order: a prefix walks blocks of many books
            cand = prefix(dir, pct)?.into_iter().filter(|i| have.contains(i)).collect();
            continue;
        }
        if let Some(v) = t.strip_prefix("sample=") {
            let (k, seed) = v.split_once('/').unwrap_or((v, "1"));
            let (k, seed): (usize, u64) = (k.parse()?, seed.parse()?);
            let mut x = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
            let mut keyed: Vec<(u64, u32)> = cand.iter().map(|&i| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                (x ^ (i as u64).wrapping_mul(0xff51afd7ed558ccd), i)
            }).collect();
            keyed.sort();
            cand = keyed.into_iter().take(k).map(|(_, i)| i).collect();
            cand.sort();
            continue;
        }
        let op = [">=", "<=", ">", "="].into_iter().find(|o| t.contains(o)).with_context(|| format!("term {t}: no operator"))?;
        let (f, c) = t.split_once(op).unwrap();
        let c: u8 = c.parse()?;
        let p = posting(f);
        cand.retain(|i| ok(op, p.get(i).copied().unwrap_or(0), c));
    }
    Ok(cand)
}

/// Does a query depend on the parse (tree features) or only on sentences (metadata, samples)?
fn tree_dependent(q: &str) -> bool {
    q.split_whitespace().any(|t| !t.starts_with("book=") && !t.starts_with("sample=") && !t.starts_with("prefix="))
}

/// Named sets (`<dir>/sets/<name>.tsv`): the query that defines the set, the model hash it was computed on (or
/// "stable" for sets of sentences that do not depend on the parse), and the ids. A tree-dependent set computed on
/// another model is invalid: its ids are recomputed from the query on load.
pub fn set_save(dir: &Path, name: &str, q: &str, hash: &str) -> Result<usize> {
    let ids = select(dir, q, hash)?;
    std::fs::create_dir_all(dir.join("sets"))?;
    let base = if tree_dependent(q) { hash.to_string() } else { "stable".to_string() };
    let stamp = if q.contains("prefix=") { format!("order:{}|{base}", order(dir, 400)?.0) } else { base };
    let body = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join("\n");
    std::fs::write(dir.join("sets").join(format!("{name}.tsv")), format!("# query\t{q}\n# model\t{stamp}\n{body}\n"))?;
    Ok(ids.len())
}

pub fn set_load(dir: &Path, name: &str, hash: &str) -> Result<Vec<u32>> {
    let t = std::fs::read_to_string(dir.join("sets").join(format!("{name}.tsv"))).with_context(|| format!("set {name}"))?;
    let q = t.lines().find_map(|l| l.strip_prefix("# query\t")).unwrap_or("").to_string();
    let stamp = t.lines().find_map(|l| l.strip_prefix("# model\t")).unwrap_or("");
    if let Some(rest) = stamp.strip_prefix("order:") {
        let (k, base) = rest.split_once('|').unwrap_or((rest, "stable"));
        if k != order(dir, 400)?.0 || (base != "stable" && base != hash) {
            eprintln!("set {name}: the shuffled order or the model changed — recomputed from «{q}»");
            set_save(dir, name, &q, hash)?;
            return select(dir, &q, hash);
        }
        return Ok(t.lines().filter(|l| !l.starts_with('#')).filter_map(|l| l.parse().ok()).collect());
    }
    if stamp == "stable" || stamp == hash {
        return Ok(t.lines().filter(|l| !l.starts_with('#')).filter_map(|l| l.parse().ok()).collect());
    }
    // carry the set over: if the old model is an ancestor of the new one, only sentences whose trees changed on the
    // way are checked again; the rest keep their membership
    let st = Store::open(dir)?;
    let mut chain = Vec::new();
    let mut h = hash.to_string();
    while h != stamp {
        chain.push(h.clone());
        match st.versions.iter().find(|v| v.0 == h).map(|v| v.1.clone()) {
            Some(p) if p != "-" => h = p,
            _ => {
                eprintln!("set {name}: computed on model {stamp}, not an ancestor of {hash} — recomputed from its query «{q}»");
                set_save(dir, name, &q, hash)?;
                return select(dir, &q, hash);
            }
        }
    }
    let mut changed: std::collections::BTreeSet<u32> = Default::default();
    for v in &chain {
        let d = std::fs::read(dir.join(format!("dict-{v}.bin")))?;
        for k in 0..d.len() / 16 {
            changed.insert(u32::from_le_bytes(d[k * 16..k * 16 + 4].try_into()?));
        }
    }
    let old: std::collections::BTreeSet<u32> = t.lines().filter(|l| !l.starts_with('#')).filter_map(|l| l.parse().ok()).collect();
    let fresh: std::collections::BTreeSet<u32> = select(dir, &q, hash)?.into_iter().filter(|i| changed.contains(i)).collect();
    let ids: Vec<u32> = old.iter().copied().filter(|i| !changed.contains(i)).chain(fresh).collect::<std::collections::BTreeSet<u32>>().into_iter().collect();
    eprintln!("set {name}: carried over from {stamp} to {hash}; {} changed sentences checked again", changed.len());
    std::fs::write(dir.join("sets").join(format!("{name}.tsv")), format!("# query\t{q}\n# model\t{hash}\n{}\n", ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join("\n")))?;
    Ok(ids)
}

pub fn set_list(dir: &Path) -> Result<()> {
    let Ok(rd) = std::fs::read_dir(dir.join("sets")) else { return Ok(()) };
    let mut names: Vec<PathBuf> = rd.flatten().map(|e| e.path()).collect();
    names.sort();
    for p in names {
        let t = std::fs::read_to_string(&p)?;
        let q = t.lines().find_map(|l| l.strip_prefix("# query\t")).unwrap_or("");
        let m = t.lines().find_map(|l| l.strip_prefix("# model\t")).unwrap_or("");
        let n = t.lines().filter(|l| !l.starts_with('#') && !l.is_empty()).count();
        println!("{}\t{n} sentences\tmodel {m}\t{q}", p.file_stem().unwrap_or_default().to_string_lossy());
    }
    Ok(())
}

/// The raw 8-byte records of sentence `i` in version `hash` (following parents).
pub fn raw_tree(st: &Store, hash: &str, i: usize) -> Result<Vec<[u8; 8]>> {
    st.records(hash, i)
}

/// Annotated words of sentence `i` in version `hash`, rebuilt from the store (forms by the tokenizer over the text,
/// lemmas from the lemma table) — experiments take trees from the store instead of parsing again.
pub fn words(st: &Store, lemmas: &[String], hash: &str, i: usize) -> Result<Vec<en::annotate::Word>> {
    let a = crate::tree::annotator()?;
    let forms: Vec<String> = a.tokenize(&st.text(i)?).into_iter().map(|t| t.form).collect();
    let recs = st.records(hash, i)?;
    if recs.len() != forms.len() {
        bail!("sentence {i}: {} tokens in the store, {} from the tokenizer", recs.len(), forms.len());
    }
    Ok(recs.iter().zip(forms).map(|(r, f)| {
        let (u, rel, head, lid) = decode(r);
        en::annotate::Word { form: f, lemma: lemmas.get(lid as usize).cloned().unwrap_or_default(), upos: u, tag: decode_tag(r), feats: Default::default(), head, rel }
    }).collect())
}

/// `world store update <dir>`: after the code changed, parse every sentence again and add a version that stores
/// only the trees that differ from the latest version (version = hash + changes from its parent); the feature index
/// of the new version covers all sentences.
pub fn update(dir: &Path, threads: usize, branch: &str) -> Result<()> {
    let st = Store::open(dir)?;
    // the parent is the branch's version (a new branch starts from main); readers of other branches are not touched
    let parent = ref_get(dir, branch).or_else(|| st.latest().map(String::from)).context("empty store: build first")?;
    let hash = if branch == "main" { model_hash()? } else { format!("{:016x}", fnv(format!("{}|{branch}", model_hash()?).as_bytes(), 0xcbf29ce484222325)) };
    if hash == parent {
        println!("model {hash} is already the version of branch {branch} — nothing to do");
        return Ok(());
    }
    let a = crate::tree::annotator()?;
    let mut lex: BTreeMap<String, u32> = lemma_table_for(dir, &parent).into_iter().enumerate().map(|(i, l)| (l, i as u32)).collect();
    let lex_m = Mutex::new(&mut lex);
    let n = st.len();
    let changed: Mutex<BTreeMap<u32, Vec<u8>>> = Mutex::new(BTreeMap::new());
    let next = AtomicUsize::new(0);
    std::thread::scope(|sc| {
        for _ in 0..threads.max(1) {
            sc.spawn(|| loop {
                let k = next.fetch_add(1024, Ordering::SeqCst);
                if k >= n {
                    break;
                }
                for i in k..(k + 1024).min(n) {
                    let Ok(text) = st.text(i) else { continue };
                    let forms: Vec<String> = a.tokenize(&text).into_iter().map(|t| t.form).collect();
                    let mut ws = a.annotate(&forms);
                    crate::rerank::repair_words(&mut ws);
                    let Ok(old) = st.tree(&parent, i) else { continue };
                    let same = old.len() == ws.len() && old.iter().zip(&ws).all(|(o, w)| o.0 == w.upos && o.1 == w.rel && o.2 == w.head);
                    if same {
                        continue;
                    }
                    let mut rec = Vec::with_capacity(ws.len() * 8);
                    for w in &ws {
                        let id = {
                            let mut lx = lex_m.lock().unwrap();
                            let nn = lx.len() as u32;
                            *lx.entry(w.lemma.to_lowercase()).or_insert(nn)
                        };
                        rec.push(UPos::ALL.iter().position(|u| *u == w.upos).unwrap_or(0) as u8);
                        rec.push(Rel::ALL.iter().position(|x| *x == w.rel).unwrap_or(0) as u8);
                        rec.extend_from_slice(&(w.head as u16).to_le_bytes());
                        rec.push(en::gram::Tag::ALL.iter().position(|t| *t == w.tag).unwrap_or(0) as u8);
                        rec.extend_from_slice(&id.to_le_bytes()[0..3]);
                    }
                    changed.lock().unwrap().insert(i as u32, rec);
                }
            });
        }
    });
    let changed = changed.into_inner().unwrap();
    let mut trees = Vec::new();
    let mut d = Vec::new();
    for (i, rec) in &changed {
        d.extend_from_slice(&i.to_le_bytes());
        d.extend_from_slice(&(trees.len() as u64).to_le_bytes());
        d.extend_from_slice(&(rec.len() as u32).to_le_bytes());
        trees.extend_from_slice(rec);
    }
    // publication order: the version's own files (atomic), its index, then the versions line, then the branch ref —
    // a reader sees either the old state or the complete new version
    write_atomic(&dir.join(format!("trees-{hash}.bin")), &trees)?;
    write_atomic(&dir.join(format!("dict-{hash}.bin")), &d)?;
    let mut lv: Vec<(String, u32)> = lex.into_iter().collect();
    lv.sort_by_key(|x| x.1);
    write_atomic(&dir.join(format!("lemmas-{hash}.tsv")), lv.iter().map(|(l, i)| format!("{i}\t{l}\n")).collect::<String>().as_bytes())?;
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    {
        // versions.tsv must already list the new version for the index to follow its parents
        let mut vf = std::fs::OpenOptions::new().create(true).append(true).open(dir.join("versions.tsv"))?;
        writeln!(vf, "{hash}\t{parent}\tast\t{now}\t{}\tbranch {branch}: delta over {parent} (world store update)", changed.len())?;
    }
    index_full(dir, &hash)?;
    ref_set(dir, branch, &hash)?;
    println!("model {hash}: {} of {n} trees changed against {parent}", changed.len());
    Ok(())
}

/// The feature index of a version over all sentences (its own trees and its parents').
pub fn index_full(dir: &Path, hash: &str) -> Result<()> {
    let st = Store::open(dir)?;
    let lemmas = lemma_table_for(dir, hash);
    let mut postings: BTreeMap<String, Vec<(u32, u8)>> = BTreeMap::new();
    for i in 0..st.len() {
        let Ok(t) = st.tree(hash, i) else { continue };
        for (f, c) in features_lex(&t, &lemmas) {
            postings.entry(f).or_default().push((i as u32, c));
        }
    }
    let mut bin = Vec::new();
    let mut tsv = String::new();
    for (f, ps) in &postings {
        tsv += &format!("{f}\t{}\t{}\n", bin.len(), ps.len());
        for (i, c) in ps {
            bin.extend_from_slice(&i.to_le_bytes());
            bin.push(*c);
        }
    }
    write_atomic(&dir.join(format!("index-{hash}.bin")), &bin)?;
    write_atomic(&dir.join(format!("features-{hash}.tsv")), tsv.as_bytes())?;
    Ok(())
}

/// Blocks of about `size` sentences per book, cut at paragraph ends — chapter-sized pieces (tens of pages).
fn blocks(st: &Store, size: usize) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    let n = st.len();
    let mut start = 0usize;
    for i in 0..n {
        let (b, p, _) = st.place(i);
        let next = (i + 1 < n).then(|| st.place(i + 1));
        let book_ends = next.is_none_or(|(nb, _, _)| nb != b);
        let para_ends = next.is_none_or(|(nb, np, _)| nb != b || np != p);
        if book_ends || (i + 1 - start >= size && para_ends) {
            out.push((start as u32, (i + 1 - start) as u32));
            start = i + 1;
        }
    }
    out
}

/// The order key: the corpus (sentence count, books file size, base version) and the day — a new shuffle every day
/// and whenever the corpus changes, reproducible within a day.
fn order_key(dir: &Path, st: &Store) -> String {
    let day = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() / 86400).unwrap_or(0);
    let books = std::fs::metadata(dir.join("books.tsv")).map(|m| m.len()).unwrap_or(0);
    let base = st.versions.first().map(|v| v.0.clone()).unwrap_or_default();
    format!("{:016x}", fnv(format!("{}|{books}|{base}|{day}", st.len()).as_bytes(), 0xcbf29ce484222325))
}

/// The shuffled block order (`<dir>/order.bin`: start u32, len u32 per block; `order.tsv`: key, blocks, size),
/// rebuilt when the key changed.
pub fn order(dir: &Path, size: usize) -> Result<(String, Vec<(u32, u32)>)> {
    let st = Store::open(dir)?;
    let key = order_key(dir, &st);
    if let (Ok(head), Ok(bin)) = (std::fs::read_to_string(dir.join("order.tsv")), std::fs::read(dir.join("order.bin"))) {
        if head.lines().next().and_then(|l| l.strip_prefix("key\t")) == Some(key.as_str()) {
            let bl = bin.chunks(8).map(|c| (u32::from_le_bytes(c[0..4].try_into().unwrap()), u32::from_le_bytes(c[4..8].try_into().unwrap()))).collect();
            return Ok((key, bl));
        }
    }
    let mut bl = blocks(&st, size);
    // Fisher–Yates with a xorshift seeded by the key
    let mut x = u64::from_str_radix(&key, 16).unwrap_or(1) | 1;
    for i in (1..bl.len()).rev() {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        bl.swap(i, (x % (i as u64 + 1)) as usize);
    }
    let mut bin = Vec::with_capacity(bl.len() * 8);
    for (s, l) in &bl {
        bin.extend_from_slice(&s.to_le_bytes());
        bin.extend_from_slice(&l.to_le_bytes());
    }
    std::fs::write(dir.join("order.bin"), bin)?;
    std::fs::write(dir.join("order.tsv"), format!("key\t{key}\nblocks\t{}\nsize\t{size}\n", bl.len()))?;
    Ok((key, bl))
}

/// Sentence ids of the first `pct` percent of the shuffled order (by sentences), in that order.
pub fn prefix(dir: &Path, pct: f64) -> Result<Vec<u32>> {
    let st = Store::open(dir)?;
    let (_, bl) = order(dir, 400)?;
    let want = ((st.len() as f64) * pct / 100.0).ceil() as usize;
    let mut out = Vec::with_capacity(want);
    for (s, l) in bl {
        if out.len() >= want {
            break;
        }
        out.extend(s..s + l);
    }
    Ok(out)
}
