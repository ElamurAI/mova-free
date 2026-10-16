//! annot — the Mova annotator subproject, full UD. Design: take random sentences from an
//! unannotated corpus, have Opus annotate them, then the Rust annotator; Opus looks at the Rust
//! output, checks it and also checks its own initial annotation for errors; the result goes into
//! our own corpus. Open-source annotators may also be run on the same input for comparison.
//!
//! Cycle per batch of sentences:
//! 1. Opus annotates the tokens in full UD: UPOS, XPOS, FEATS, lemma, head, relation (first look, A).
//! 2. The Rust annotator (`en`) annotates the same tokens (B): tagger, parser, lemmas from the
//!    built-in dictionary, UPOS and FEATS — tables over the graph.
//! 3. Opus sees A, B and their disagreements and gives the final annotation (second look).
//! 4. Gates in Rust: forms, closed inventories, the UD 2.18 registry for English, the tree.
//! 5. Third opinion — UDPipe 2 (LINDAT, EWT model): for comparison only, into the report. Its
//!    models are NC-licensed, so its output goes neither into the corpus nor into training.
//! 6. Silver: final blocks that passed the gates are appended to our own corpus (`corpus/`),
//!    with the source and license of each sentence.
//!
//!   annot calib  <train.conllu>... -- <test.conllu> <n> <seed> <out-dir>
//!   annot silver <train.conllu>... -- <sentences.tsv> <n> <seed> <out-dir>
//!   annot batch-prepare <train.conllu>... -- <test.conllu | sentences.tsv> <n> <seed> <out-dir>
//!   annot batch-blind <patch-dir> <out-dir>   the same sentences, Haiku from scratch (no MMM draft) — anchoring check
//!   annot batch-run <out-dir>
//!   annot batch-apply <out-dir> <results.jsonl>
//!   annot direct-run <out-dir>      (same requests through claude -p on the subscription, one by one)
//!   annot report <out-dir> <test.conllu>
//!   annot sentences <papers-dir> <out.tsv> [max]
//!
//! Model and effort: `ANNOT_MODEL` (claude-opus-5-5), `ANNOT_EFFORT` (medium). `ANNOT_FLOW=draft` —
//! "draft" mode: Rust annotates first, Opus corrects once. In the report the "first look" then
//! equals the final one. Corpus — `ANNOT_CORPUS`
//! (default `$MOVA_DATA/corpus/mova-silver-en.conllu` — the corpus lives outside the repo, in the dev area, and in the database `$MOVA_DATA/db/mova.duckdb`). `ANNOT_UDPIPE=0` disables UDPipe.
//!
//! Message Batches (patch flow only, 50% of the API price, asynchronous): `batch-prepare` selects the
//! items and writes the Rust drafts and `batch-requests.jsonl` instead of calling `claude -p`;
//! `batch-run` submits, polls and downloads the results; `batch-apply` applies the replies through the
//! same patch parser, gates, report and corpus append as the live flow. See `batch.rs`.

mod ann;
mod batch;
mod hints;
mod llm;

use std::collections::{BTreeMap, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use en::conllu;
use en::gram::UPos;
use en::model::Model;
use en::ud::{Scores, Ud, View, kids_of};
use en::udreg::Registry;

use ann::{Ann, Item, Tok, parse_block, split_blocks, words, write_block};
use llm::{Call, Opus};

const UDPIPE_MODEL: &str = "english-ewt-ud-2.17-251125";

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("report") if args.len() >= 3 => return report(Path::new(&args[1]), Path::new(&args[2])),
        Some("refeat") if args.len() >= 4 => return refeat(&args[1..args.len() - 2], Path::new(&args[args.len() - 2]), Path::new(&args[args.len() - 1])),
        Some("udpipe-eval") if args.len() >= 2 => return udpipe_eval(Path::new(&args[1])),
        Some("feats-diff") if args.len() >= 3 => return feats_diff(Path::new(&args[1]), Path::new(&args[2]), args.get(3).and_then(|x| x.parse().ok()).unwrap_or(30)),
        Some("seed-examples") if args.len() >= 3 => return seed_examples(Path::new(&args[1]), Path::new(&args[2])),
        Some("bronze") => {
            let sep = args.iter().position(|a| a == "--").unwrap_or(args.len());
            let train: Vec<PathBuf> = args.get(1..sep).unwrap_or(&[]).iter().map(PathBuf::from).collect();
            let rest = args.get(sep + 1..).unwrap_or(&[]);
            if rest.len() < 2 {
                bail!("usage: annot bronze <train.conllu>... -- <sentences.tsv> <corpus.conllu>");
            }
            return bronze(&train, Path::new(&rest[0]), Path::new(&rest[1]));
        }
        Some("batch-apply") if args.len() >= 3 => return batch::apply(Path::new(&args[1]), Path::new(&args[2])),
        Some("batch-run") if args.len() >= 2 => return batch::run(Path::new(&args[1])),
        Some("batch-blind") if args.len() >= 3 => return batch::blind(Path::new(&args[1]), Path::new(&args[2])),
        Some("direct-run") if args.len() >= 2 => return batch::direct_run(Path::new(&args[1])),
        Some("sentences") if args.len() >= 3 => return sentences(Path::new(&args[1]), Path::new(&args[2]), args.get(3).and_then(|x| x.parse().ok()).unwrap_or(5000)),
        _ => {}
    }
    let sep = args.iter().position(|a| a == "--").unwrap_or(args.len());
    let cmd = args.first().map(String::as_str).unwrap_or("");
    let train: Vec<PathBuf> = args.get(1..sep).unwrap_or(&[]).iter().map(PathBuf::from).collect();
    let rest = args.get(sep + 1..).unwrap_or(&[]);
    if !matches!(cmd, "calib" | "silver" | "batch-prepare" | "drafts") || rest.len() < 4 {
        bail!(
            "usage: annot calib|silver|batch-prepare <train.conllu>... -- <test.conllu | sentences.tsv> <n> <seed> <out-dir> | annot drafts <train.conllu>... -- <sentences.tsv> <n> <seed> <out.conllu> | annot batch-run <out-dir> | annot direct-run <out-dir> | annot batch-apply <out-dir> <results.jsonl> | annot report <out-dir> <test.conllu> | annot sentences <papers-dir> <out.tsv> [max]"
        );
    }
    let (src, n, seed, out) = (Path::new(&rest[0]), rest[1].parse::<usize>()?, rest[2].parse::<u64>()?, PathBuf::from(&rest[3]));
    // ANNOT_DRAFTS=<file.conllu>: the drafts come ready (e.g. MMM + open parsers through treeagree) — the sentences and
    // their trees are taken from the file, no Rust annotator is trained
    if cmd == "batch-prepare" {
        if let Ok(f) = std::env::var("ANNOT_DRAFTS") {
            en::license::commercial_gate(&train)?;
            std::fs::create_dir_all(&out)?;
            let (items, b) = drafts_from(Path::new(&f))?;
            eprintln!("ready drafts: {} sentences from {f}", items.len());
            let corpus = std::env::var("ANNOT_CORPUS").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("corpus/mova-silver-en.conllu"));
            let model = std::env::var("ANNOT_MODEL").unwrap_or_else(|_| "claude-haiku-5-5".into());
            let effort = std::env::var("ANNOT_EFFORT").unwrap_or_else(|_| "medium".into());
            return batch::prepare(&b, &train, &items, &out, Some(&corpus), &model, &effort);
        }
    }
    // batch-prepare: a .conllu input is calib (gold), anything else is silver (sentences.tsv)
    let silver = cmd == "silver" || cmd == "drafts" || (cmd == "batch-prepare" && src.extension().is_none_or(|e| e != "conllu"));
    // silver goes into the corpus of a closed model: the Rust draft is trained only on commercially licensed data
    if silver {
        en::license::commercial_gate(&train)?;
    }
    if cmd != "drafts" {
        std::fs::create_dir_all(&out)?;
    }
    // ANNOT_EN_MODEL=<model.bin>: a saved Rust annotator (`en train`) instead of training one (3 min) — when its
    // tree is replaced anyway (the GPU draft) only tokens, lemmas and features are taken from it
    let (model, ud) = if let Ok(m) = std::env::var("ANNOT_EN_MODEL") {
        eprintln!("loading the Rust annotator {m}");
        let a = en::annotate::Annotator::load(Path::new(&m))?;
        (a.model, a.ud)
    } else {
        let mut sents = Vec::new();
        for p in &train {
            sents.extend(conllu::read(p)?.into_iter().filter(|s| s.tagged()));
        }
        eprintln!("training the Rust annotator…");
        let ud = Ud::train(&sents);
        (Model::train(sents), ud)
    };
    let corpus = silver.then(|| std::env::var("ANNOT_CORPUS").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("corpus/mova-silver-en.conllu")));
    // sentences already in the corpus are not sent to Opus again
    let mut have: HashSet<String> = corpus.as_deref().map(|p| std::fs::read_to_string(p).unwrap_or_default().lines().filter_map(|l| l.strip_prefix("# sent_id = ").map(str::to_string)).collect()).unwrap_or_default();
    // ANNOT_EXCLUDE=<file>: more sentence ids to skip, one per line (rounds still in flight, held-out test lessons)
    if let Ok(f) = std::env::var("ANNOT_EXCLUDE") {
        have.extend(std::fs::read_to_string(&f)?.lines().map(str::trim).filter(|l| !l.is_empty()).map(str::to_string));
    }
    let items = if silver { silver_items(&model, src, n, seed, &have)? } else { calib_items(src, n, seed)? };
    if silver {
        eprintln!("sentences to annotate: {} (already in the corpus — skipped)", items.len());
    }
    let opus = Opus {
        model: std::env::var("ANNOT_MODEL").unwrap_or_else(|_| "claude-haiku-5-5".into()),
        effort: std::env::var("ANNOT_EFFORT").unwrap_or_else(|_| "medium".into()),
        cwd: out.join("cwd"),
    };
    if cmd == "drafts" {
        // the MMM's drafts of the chosen sentences, for the open parsers and treeagree before the teacher
        let b = rust_drafts(&model, &ud, &items);
        let mut t = String::new();
        for it in &items {
            t.push_str(&format!("# text = {}\n", it.text));
            t.push_str(&write_block(&it.id, &b[&it.id]));
        }
        std::fs::write(&out, t)?;
        eprintln!("drafts → {}", out.display());
        return Ok(());
    }
    if cmd == "batch-prepare" {
        let b = rust_drafts(&model, &ud, &items);
        return batch::prepare(&b, &train, &items, &out, corpus.as_deref(), &opus.model, &opus.effort);
    }
    run(&model, &ud, &opus, &items, &out, corpus.as_deref())
}

/// Sentences and ready drafts from a CoNLL-U file (`# sent_id`, `# text`, word lines); a block the gates refuse is
/// left out with a note.
fn drafts_from(path: &Path) -> Result<(Vec<Item>, BTreeMap<String, Ann>)> {
    let reg = Registry::load();
    let (mut items, mut b) = (Vec::new(), BTreeMap::new());
    for block in std::fs::read_to_string(path)?.split("\n\n").filter(|x| !x.trim().is_empty()) {
        let (mut id, mut text, mut lines) = (String::new(), String::new(), Vec::new());
        for l in block.lines() {
            if let Some(x) = l.strip_prefix("# sent_id = ") {
                id = x.to_string();
            } else if let Some(x) = l.strip_prefix("# text = ") {
                text = x.to_string();
            } else if !l.starts_with('#') {
                lines.push(l.to_string());
            }
        }
        let forms: Vec<String> = lines.iter().filter_map(|l| l.split('\t').nth(1).map(str::to_string)).collect();
        match ann::parse_block(&lines, &forms, &reg) {
            Ok(a) => {
                b.insert(id.clone(), a);
                items.push(Item { id, text, forms, gold: None });
            }
            Err(e) => eprintln!("  draft {id} refused by the gates: {e:?}"),
        }
    }
    Ok((items, b))
}

/// Deterministic shuffle (xorshift).
fn shuffle<T>(v: &mut [T], seed: u64) {
    let mut r = seed.max(1);
    for i in (1..v.len()).rev() {
        r ^= r << 13;
        r ^= r >> 7;
        r ^= r << 17;
        v.swap(i, (r % (i as u64 + 1)) as usize);
    }
}

fn gold_tok(t: &conllu::Token) -> Tok {
    Tok { form: t.form.clone(), lemma: t.lemma.clone(), upos: t.upos.unwrap_or(UPos::X), tag: t.tag.expect("tagged"), feats: t.feats, head: t.head, rel: t.rel }
}

/// Sentences of the UD test set with gold annotation — of any length. Long complex sentences, like
/// short ones, are not filtered out (earlier only 5–30 tokens were taken, which made the measurement easier than the real task).
fn calib_items(test: &Path, n: usize, seed: u64) -> Result<Vec<Item>> {
    let mut ss: Vec<_> = conllu::read(test)?.into_iter().filter(|s| s.tagged() && !s.tokens.is_empty()).collect();
    shuffle(&mut ss, seed);
    Ok(ss
        .into_iter()
        .take(n)
        .map(|s| {
            let gold: Ann = s.tokens.iter().map(gold_tok).collect();
            Item { id: s.id.clone(), text: s.text.clone(), forms: gold.iter().map(|t| t.form.clone()).collect(), gold: Some(gold) }
        })
        .collect())
}

/// Random sentences (id \t text), tokens from our own tokenizer.
fn silver_items(model: &Model, file: &Path, n: usize, seed: u64, have: &HashSet<String>) -> Result<Vec<Item>> {
    let text = std::fs::read_to_string(file)?;
    let mut rows: Vec<(&str, &str)> = text.lines().filter_map(|l| l.split_once('\t')).collect();
    shuffle(&mut rows, seed);
    let mut out = Vec::new();
    for (id, t) in rows {
        let forms: Vec<String> = en::tok::tokenize(t, &model.lx).into_iter().map(|x| x.form).collect();
        let id = if id.contains(':') { id.to_string() } else { format!("tatoeba:{id}") };
        if !forms.is_empty() && !have.contains(&id) {
            out.push(Item { id, text: t.to_string(), forms, gold: None });
        }
        if out.len() == n {
            break;
        }
    }
    Ok(out)
}

/// FEATS from the Rust tables on top of an existing annotation (its UPOS, tags and tree), with verb agreement.
fn rust_feats(ud: &Ud, x: &Ann) -> Vec<en::gram::Feats> {
    let forms: Vec<&str> = x.iter().map(|t| t.form.as_str()).collect();
    let tags: Vec<en::gram::Tag> = x.iter().map(|t| t.tag).collect();
    let upos: Vec<UPos> = x.iter().map(|t| t.upos).collect();
    let heads: Vec<usize> = x.iter().map(|t| t.head).collect();
    let rels: Vec<en::gram::Rel> = x.iter().map(|t| t.rel).collect();
    ud.feats_sentence(&forms, &tags, &upos, &heads, &rels)
}

/// Rust annotator: tagger, parser, lemmas, UPOS and FEATS — on fixed tokens.
fn rust_annotate(model: &Model, ud: &Ud, forms: &[String]) -> Ann {
    let words: Vec<&str> = forms.iter().map(String::as_str).collect();
    let tags = model.tagger.tag(&words);
    let tree = model.parser.parse(&words, &tags);
    let heads: Vec<usize> = tree.iter().map(|x| x.0).collect();
    let rels: Vec<en::gram::Rel> = tree.iter().map(|x| x.1).collect();
    let kids = kids_of(&heads, &rels);
    let upos: Vec<UPos> = (0..words.len()).map(|i| ud.upos(&View { form: words[i], tag: tags[i], rel: rels[i], kids: kids[i], agr: 0 })).collect();
    let feats = ud.feats_sentence(&words, &tags, &upos, &heads, &rels);
    words
        .iter()
        .enumerate()
        .map(|(i, w)| Tok { form: w.to_string(), lemma: model.morph.lemmatize(w, tags[i]), upos: upos[i], tag: tags[i], feats: feats[i], head: heads[i], rel: rels[i] })
        .collect()
}

/// Third opinion — UDPipe 2 (LINDAT REST, EWT model): fixed tokens (input=horizontal), tagger and
/// parser. Returns the annotations that passed the gates and the number of rejects.
fn udpipe(items: &[Item], reg: &Registry) -> Result<(BTreeMap<String, Ann>, usize)> {
    let input: String = items.iter().map(|it| it.forms.join(" ")).collect::<Vec<_>>().join("\n");
    let resp: serde_json::Value = ureq::post("https://lindat.mff.cuni.cz/services/udpipe/api/process")
        .send_form(&[("model", UDPIPE_MODEL), ("input", "horizontal"), ("tagger", ""), ("parser", ""), ("data", &input)])
        .context("UDPipe")?
        .into_json()
        .context("UDPipe JSON")?;
    let conllu_text = resp["result"].as_str().unwrap_or_default();
    let mut out = BTreeMap::new();
    let mut failed = 0;
    let blocks: Vec<Vec<String>> = conllu_text.split("\n\n").map(|b| b.lines().filter(|l| l.starts_with(|c: char| c.is_ascii_digit())).map(str::to_string).collect()).filter(|v: &Vec<String>| !v.is_empty()).collect();
    for (it, lines) in items.iter().zip(blocks) {
        match parse_block(&lines, &it.forms, reg) {
            Ok(a) => {
                out.insert(it.id.clone(), a);
            }
            Err(_) => failed += 1,
        }
    }
    Ok((out, failed))
}

/// Blocks of the Opus reply → annotations by id (with gates); rejects go to the log.
fn collect(text: &str, items: &[&Item], reg: &Registry, bad: &mut BTreeMap<String, String>, step: &str) -> BTreeMap<String, Ann> {
    let blocks: BTreeMap<String, Vec<String>> = split_blocks(text).into_iter().collect();
    let mut out = BTreeMap::new();
    for it in items {
        match blocks.get(&it.id) {
            None => {
                bad.insert(format!("{step} {}", it.id), "block missing".into());
            }
            Some(lines) => match parse_block(lines, &it.forms, reg) {
                Ok(a) => {
                    out.insert(it.id.clone(), a);
                }
                Err(e) => {
                    bad.insert(format!("{step} {}", it.id), e.to_string());
                }
            },
        }
    }
    out
}

/// Opus patches to drafts: `<sent_id> <No.> <COLUMN>=<value>` or `<sent_id> OK`. A sentence without
/// a single line in the reply is rejected (Opus may have skipped it). After patching — the same gates.
fn apply_patches(text: &str, pairs: &[(&Item, &Ann)], reg: &Registry, bad: &mut BTreeMap<String, String>, prag: &mut BTreeMap<String, Vec<(String, String)>>) -> BTreeMap<String, Ann> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out: BTreeMap<String, Ann> = pairs.iter().map(|(it, b)| (it.id.clone(), (*b).clone())).collect();
    let mut broken: BTreeMap<String, String> = BTreeMap::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let mut parts = line.splitn(3, ' ');
        let (Some(id), Some(second)) = (parts.next(), parts.next()) else { continue };
        let Some(ann) = out.get_mut(id) else { continue };
        seen.insert(id.to_string());
        if second == "OK" {
            continue;
        }
        // `<sent_id> PRAG <field>=<value>`: a correction of the pragmatic reading, checked against the closed sets
        if second == "PRAG" {
            match parts.next().and_then(|kv| kv.split_once('=')).map(|(f, v)| (f.trim(), v.trim())) {
                Some((f, v)) if llm::prag_ok(f, v) => prag.entry(id.to_string()).or_default().push((f.to_string(), v.to_string())),
                _ => {
                    broken.insert(id.to_string(), format!("invalid PRAG patch \"{line}\""));
                }
            }
            continue;
        }
        let (Ok(n), Some(kv)) = (second.parse::<usize>(), parts.next()) else {
            broken.insert(id.to_string(), format!("unparseable line \"{line}\""));
            continue;
        };
        let Some((col, val)) = kv.split_once('=') else {
            broken.insert(id.to_string(), format!("unparseable line \"{line}\""));
            continue;
        };
        let Some(t) = ann.get_mut(n.wrapping_sub(1)) else {
            broken.insert(id.to_string(), format!("no word {n}"));
            continue;
        };
        let val = val.trim();
        let ok = match col.trim() {
            "LEMMA" => {
                t.lemma = val.to_string();
                true
            }
            "UPOS" => UPos::parse(val).map(|u| t.upos = u).is_some(),
            "XPOS" => en::gram::Tag::parse(val).map(|x| t.tag = x).is_some(),
            "FEATS" => {
                let (f, unknown) = en::gram::Feats::parse(val);
                t.feats = f;
                unknown == 0
            }
            "HEAD" => val.parse().map(|h| t.head = h).is_ok(),
            "DEPREL" => en::gram::Rel::parse(val).map(|r| t.rel = r).is_some(),
            _ => false,
        };
        if !ok {
            broken.insert(id.to_string(), format!("invalid patch \"{line}\""));
        }
    }
    let mut res = BTreeMap::new();
    for (it, _) in pairs {
        if !seen.contains(&it.id) {
            bad.insert(format!("P {}", it.id), "no line in the reply".into());
            continue;
        }
        if let Some(e) = broken.get(&it.id) {
            bad.insert(format!("P {}", it.id), e.clone());
            continue;
        }
        let a = out.remove(&it.id).expect("draft");
        match ann::check(&a, &it.forms, reg) {
            Ok(()) => {
                res.insert(it.id.clone(), a);
            }
            Err(e) => {
                bad.insert(format!("P {}", it.id), e.to_string());
            }
        }
    }
    res
}

/// CoNLL 2018 metrics of annotation `x` against `y` on the shared sentences.
fn score(x: &BTreeMap<String, Ann>, y: &BTreeMap<String, Ann>) -> Scores {
    let mut s = Scores::default();
    for (id, a) in x {
        if let Some(b) = y.get(id) {
            s.add(&words(b), &words(a), true, true);
        }
    }
    s
}

/// Source and license of a sentence by its id: `tatoeba:<n>` — CC0, `papers:<folder>:<n>` — CC BY 4.0.
fn provenance(id: &str) -> String {
    if let Some(n) = id.strip_prefix("tatoeba:") {
        format!("Tatoeba #{n} (CC0 1.0)")
    } else if let Some(rest) = id.strip_prefix("papers:") {
        let folder = rest.rsplit_once(':').map_or(rest, |x| x.0);
        format!("papers/{folder} (CC BY 4.0, ACL Anthology)")
    } else if id.starts_with("school:") {
        "school lesson (ours)".into()
    } else if let Some(rest) = id.strip_prefix("seeds:") {
        format!("en/seeds/{} (Mova seed; example from a grammar or our own)", rest.rsplit_once(':').map_or(rest, |x| x.0))
    } else {
        "unknown".into()
    }
}

/// What the Opus looks produced: first look, final, raw replies, calls, gate rejects, hint counts and
/// free-form report notes. Filled by the live loop (`run`) or from batch results (`batch::apply`).
#[derive(Default)]
struct Outcome {
    a: BTreeMap<String, Ann>,
    fin: BTreeMap<String, Ann>,
    ta: String,
    tf: String,
    calls: Vec<Call>,
    bad: BTreeMap<String, String>,
    hints_on: bool,
    hint_count: (usize, usize),
    notes: Vec<String>,
    /// the teacher's PRAG patches: sentence → (field, value)
    prag: BTreeMap<String, Vec<(String, String)>>,
}

impl Outcome {
    /// One patch reply for a batch of drafts: raw text kept, patches applied, gates run.
    fn add_patch_reply(&mut self, text: &str, pairs: &[(&Item, &Ann)], reg: &Registry) {
        self.tf.push_str(text);
        self.tf.push('\n');
        let f_k = apply_patches(text, pairs, reg, &mut self.bad, &mut self.prag);
        self.a.extend(f_k.clone());
        self.fin.extend(f_k);
    }
}

/// Rust drafts for all items, by id.
fn rust_drafts(model: &Model, ud: &Ud, items: &[Item]) -> BTreeMap<String, Ann> {
    items.iter().map(|it| (it.id.clone(), rust_annotate(model, ud, &it.forms))).collect()
}

/// Sentences per Opus call: ANNOT_BATCH (20) — a large corpus does not fit into one call.
fn batch_size() -> usize {
    std::env::var("ANNOT_BATCH").ok().and_then(|x| x.parse().ok()).unwrap_or(20usize).max(1)
}

/// ANNOT_HINTS=error|warn — hints from seed rules (ANNOT_SEEDS, default en/seeds) in patch mode.
fn load_hints(patch: bool) -> Result<Option<hints::Hints>> {
    match std::env::var("ANNOT_HINTS") {
        Ok(v) if patch => {
            let seeds = std::env::var("ANNOT_SEEDS").unwrap_or_else(|_| "en/seeds".into());
            let h = hints::Hints::load(Path::new(&seeds), v == "warn")?;
            eprintln!("hints: {} rules ({})", h.len(), if v == "warn" { "error and warn" } else { "error" });
            Ok(Some(h))
        }
        _ => Ok(None),
    }
}

/// Hints for the drafts of one batch, by sentence id (only sentences with hints).
fn chunk_hints(hints: Option<&hints::Hints>, chunk: &[Item], b: &BTreeMap<String, Ann>) -> BTreeMap<String, Vec<String>> {
    match hints {
        Some(h) => chunk.iter().map(|it| (it.id.clone(), h.for_ann(&it.id, &it.text, &b[&it.id]))).filter(|x| !x.1.is_empty()).collect(),
        None => BTreeMap::new(),
    }
}

fn run(model: &Model, ud: &Ud, opus: &Opus, items: &[Item], out: &Path, corpus: Option<&Path>) -> Result<()> {
    let reg = Registry::load();
    let mut o = Outcome::default();
    let b = rust_drafts(model, ud, items);
    let flow = std::env::var("ANNOT_FLOW").unwrap_or_default();
    let draft = flow == "draft";
    let patch = flow == "patch";
    let hints = load_hints(patch)?;
    o.hints_on = hints.is_some();
    for (k, chunk) in items.chunks(batch_size()).enumerate() {
        let refs: Vec<&Item> = chunk.iter().collect();
        if patch {
            // "patches only": Opus emits patches to the Rust draft, Rust applies them, the gates check
            let pairs: Vec<(&Item, &Ann)> = chunk.iter().map(|it| (it, &b[&it.id])).collect();
            let hs = chunk_hints(hints.as_ref(), chunk, &b);
            o.hint_count.0 += hs.values().map(Vec::len).sum::<usize>();
            o.hint_count.1 += hs.len();
            eprintln!("batch {}: Opus, patches to the Rust draft for {} sentences ({} hints)…", k + 1, pairs.len(), hs.values().map(Vec::len).sum::<usize>());
            let (t, c) = opus.patch(&pairs, &hs)?;
            o.calls.push(c);
            o.add_patch_reply(&t, &pairs, &reg);
            continue;
        }
        if draft {
            // "draft": Rust annotates first, Opus corrects once
            let pairs: Vec<(&Item, &Ann)> = chunk.iter().map(|it| (it, &b[&it.id])).collect();
            eprintln!("batch {}: Opus corrects the Rust draft for {} sentences…", k + 1, pairs.len());
            let (t, c) = opus.correct(&pairs)?;
            o.calls.push(c);
            let f_k = collect(&t, &refs, &reg, &mut o.bad, "F");
            o.tf.push_str(&t);
            o.tf.push('\n');
            o.a.extend(f_k.clone());
            o.fin.extend(f_k);
            continue;
        }
        eprintln!("batch {}: Opus, first look at {} sentences…", k + 1, refs.len());
        let (t1, c1) = opus.annotate(&refs)?;
        o.calls.push(c1);
        let a_k = collect(&t1, &refs, &reg, &mut o.bad, "A");
        o.ta.push_str(&t1);
        o.ta.push('\n');
        let pairs: Vec<(&Item, &Ann, &Ann)> = chunk.iter().filter_map(|it| a_k.get(&it.id).map(|x| (it, x, &b[&it.id]))).collect();
        if !pairs.is_empty() {
            eprintln!("batch {}: Opus, second look at {} sentences…", k + 1, pairs.len());
            let (t2, c2) = opus.adjudicate(&pairs)?;
            o.calls.push(c2);
            let fin_refs: Vec<&Item> = pairs.iter().map(|p| p.0).collect();
            o.fin.extend(collect(&t2, &fin_refs, &reg, &mut o.bad, "F"));
            o.tf.push_str(&t2);
            o.tf.push('\n');
        }
        o.a.extend(a_k);
    }
    finish(Some(ud), items, &b, o, out, corpus, &flow, &opus.model, &opus.effort)
}

/// Everything after the Opus looks: optional Rust FEATS, UDPipe, CoNLL-U dumps, call log, report,
/// silver into the corpus. `ud` is needed only for ANNOT_FEATS=rust; `effort` goes into the corpus
/// annotation line.
#[allow(clippy::too_many_arguments)]
fn finish(ud: Option<&Ud>, items: &[Item], b: &BTreeMap<String, Ann>, o: Outcome, out: &Path, corpus: Option<&Path>, flow: &str, model: &str, effort: &str) -> Result<()> {
    let reg = Registry::load();
    let Outcome { a, mut fin, ta, tf, calls, bad, hints_on, hint_count, notes, prag } = o;
    // the pragmatic reading after the teacher: the agreed labels with its PRAG patches, for the sentences that passed
    let base = llm::prag_labels();
    if !base.is_empty() {
        let mut t = format!("sent_id\t{}\tteacher\n", llm::PRAG_FIELDS.join("\t"));
        let mut changed = 0;
        for id in fin.keys() {
            let mut m = base.get(id).cloned().unwrap_or_default();
            let ps = prag.get(id).cloned().unwrap_or_default();
            for (f, v) in &ps {
                m.insert(f.clone(), v.clone());
            }
            changed += !ps.is_empty() as usize;
            let vals: Vec<String> = llm::PRAG_FIELDS.iter().map(|f| m.get(*f).cloned().unwrap_or_else(|| "?".into())).collect();
            let _ = writeln!(t, "{id}\t{}\t{}", vals.join("\t"), if ps.is_empty() { "-".into() } else { ps.iter().map(|(f, v)| format!("{f}={v}")).collect::<Vec<_>>().join(",") });
        }
        std::fs::write(out.join("prag-final.tsv"), t)?;
        eprintln!("pragmatics: the teacher changed {changed} of {} sentences → prag-final.tsv", fin.len());
    }
    std::fs::write(out.join("opus-a.txt"), &ta)?;
    std::fs::write(out.join("opus-final.txt"), &tf)?;
    // ANNOT_FEATS=rust — features from the Rust tables on top of the final Opus annotation (for high: +1–3 UFeats)
    if std::env::var("ANNOT_FEATS").as_deref() == Ok("rust") {
        let ud = ud.context("ANNOT_FEATS=rust needs the trained Rust tables")?;
        for x in fin.values_mut() {
            let feats = rust_feats(ud, x);
            for (t, f) in x.iter_mut().zip(feats) {
                t.feats = f;
            }
        }
    }
    let udp = if std::env::var("ANNOT_UDPIPE").as_deref() == Ok("0") {
        None
    } else {
        match udpipe(items, &reg) {
            Ok(x) => Some(x),
            Err(e) => {
                eprintln!("UDPipe unavailable: {e:#}");
                None
            }
        }
    };
    let dump = |name: &str, m: &BTreeMap<String, Ann>| -> Result<()> {
        let mut s = String::new();
        for it in items {
            if let Some(x) = m.get(&it.id) {
                let _ = writeln!(s, "# text = {}", it.text);
                s.push_str(&write_block(&it.id, x));
            }
        }
        std::fs::write(out.join(name), s)?;
        Ok(())
    };
    dump("opus-a.conllu", &a)?;
    dump("rust-b.conllu", b)?;
    dump("final.conllu", &fin)?;
    if let Some((c, _)) = &udp {
        dump("udpipe-c.conllu", c)?;
    }
    llm::log_calls(&out.join("calls.jsonl"), &calls)?;
    // report
    let mut r = String::new();
    let _ = writeln!(r, "sentences {}; passed the gates: first look {}, final {}", items.len(), a.len(), fin.len());
    if hints_on {
        let _ = writeln!(r, "rule hints: {} in {} sentences", hint_count.0, hint_count.1);
    }
    let fixed = ann::EWT_FIXED.load(std::sync::atomic::Ordering::Relaxed);
    if fixed > 0 {
        let _ = writeln!(r, "relations normalized to EWT conventions (det:poss, :tmod, :npmod): {fixed}");
    }
    if let Some((c, failed)) = &udp {
        let _ = writeln!(r, "UDPipe 2 ({UDPIPE_MODEL}): passed the gates {}, rejected {failed}", c.len());
    }
    for (k, v) in &bad {
        let _ = writeln!(r, "  gate: {k}: {v}");
    }
    let _ = writeln!(r, "\n{}", Scores::HEADER.replace("| test | tokens |", "| annotation | against |"));
    let _ = writeln!(r, "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    let base = |m: &BTreeMap<String, Ann>| -> BTreeMap<String, Ann> { m.iter().filter(|(k, _)| fin.contains_key(*k)).map(|(k, v)| (k.clone(), v.clone())).collect() };
    let gold: BTreeMap<String, Ann> = items.iter().filter_map(|it| it.gold.clone().map(|g| (it.id.clone(), g))).collect();
    if !gold.is_empty() {
        let mut rows = vec![("Opus, first look", base(&a)), ("Rust", base(b)), ("Opus, final", fin.clone())];
        if let Some((c, _)) = &udp {
            rows.push(("UDPipe 2", base(c)));
        }
        for (name, m) in rows {
            let _ = writeln!(r, "| {name} | gold | {} |", score(&m, &gold).row());
        }
    }
    let _ = writeln!(r, "| final | first look | {} |", score(&fin, &base(&a)).row());
    let _ = writeln!(r, "| final | Rust | {} |", score(&fin, &base(b)).row());
    if let Some((c, _)) = &udp {
        let _ = writeln!(r, "| final | UDPipe 2 | {} |", score(&fin, &base(c)).row());
        let _ = writeln!(r, "| Rust | UDPipe 2 | {} |", score(&base(b), &base(c)).row());
    }
    for c in &calls {
        let _ = writeln!(
            r,
            "call {}: {} sentences, {:.0} s ({:.1} s/sentence), input {} tokens, cache read {} / write {} tokens, output {} tokens (API equivalent ${:.3})",
            c.step,
            c.sentences,
            c.secs,
            c.secs / c.sentences.max(1) as f64,
            c.input_tokens,
            c.cache_read_tokens,
            c.cache_write_tokens,
            c.output_tokens,
            c.cost_usd
        );
    }
    for n in &notes {
        let _ = writeln!(r, "{n}");
    }
    // silver goes into our own corpus, without duplicates; how it was annotated depends on the mode
    let feats = if std::env::var("ANNOT_FEATS").as_deref() == Ok("rust") { " → FEATS from Rust" } else { "" };
    // the teacher by name: "claude-haiku-5-5" → "Haiku 5.5"
    let teacher = model.strip_prefix("claude-").map_or(model.to_string(), |m| {
        let mut it = m.splitn(2, '-');
        let (name, ver) = (it.next().unwrap_or(m), it.next().unwrap_or("").replace('-', "."));
        format!("{}{} {ver}", name[..1].to_uppercase(), &name[1..])
    });
    let draft = std::env::var("ANNOT_DRAFT_LABEL").unwrap_or_else(|_| "Rust (en)".into());
    let how = match flow {
        "patch" => format!("Mova annot: {draft} → {teacher} ({}) patches{feats}", effort),
        "draft" => format!("Mova annot: Rust (en) → {teacher} ({}) final{feats}", effort),
        "blind" => format!("Mova annot: {teacher} ({}) from scratch, no draft{feats}", effort),
        _ => format!("Mova annot: {teacher} ({}) A → Rust B → {teacher} final{feats}", effort),
    };
    if let Some(path) = corpus {
        let have: HashSet<String> = std::fs::read_to_string(path).unwrap_or_default().lines().filter_map(|l| l.strip_prefix("# sent_id = ").map(str::to_string)).collect();
        let mut add = String::new();
        let mut n = 0;
        for it in items {
            if let (Some(x), false) = (fin.get(&it.id), have.contains(&it.id)) {
                let _ = writeln!(add, "# source = {}", provenance(&it.id));
                let _ = writeln!(add, "# annotation = {how}; gates UD 2.18; {}", out.display());
                let _ = writeln!(add, "# text = {}", it.text);
                add.push_str(&write_block(&it.id, x));
                n += 1;
            }
        }
        if let Some(d) = path.parent() {
            std::fs::create_dir_all(d)?;
        }
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
        std::io::Write::write_all(&mut f, add.as_bytes())?;
        let _ = writeln!(r, "into corpus {}: +{n} sentences (was {})", path.display(), have.len());
    }
    std::fs::write(out.join("report.txt"), &r)?;
    print!("{r}");
    Ok(())
}

/// Annotations from a CoNLL-U file written by `run`.
fn read_anns(path: &Path, reg: &Registry) -> Result<BTreeMap<String, Ann>> {
    let text = std::fs::read_to_string(path)?;
    let mut out = BTreeMap::new();
    for (id, lines) in split_blocks(&text) {
        let forms: Vec<String> = lines.iter().filter_map(|l| l.split('\t').nth(1).map(str::to_string)).collect();
        match parse_block(&lines, &forms, reg) {
            Ok(a) => {
                out.insert(id, a);
            }
            Err(e) => bail!("{}: {id}: {e}", path.display()),
        }
    }
    Ok(out)
}

/// What the second look changed in the first one, against gold: by UPOS, tags, features, lemmas and
/// head+relation pairs.
fn report(out: &Path, test: &Path) -> Result<()> {
    let reg = Registry::load();
    let a = read_anns(&out.join("opus-a.conllu"), &reg)?;
    let b = read_anns(&out.join("rust-b.conllu"), &reg)?;
    let f = read_anns(&out.join("final.conllu"), &reg)?;
    let gold: BTreeMap<String, Ann> = conllu::read(test)?.into_iter().filter(|s| f.contains_key(&s.id)).map(|s| (s.id.clone(), s.tokens.iter().map(gold_tok).collect())).collect();
    let names = ["UPOS", "tag", "features", "lemma", "head+relation"];
    let key = |t: &Tok, k: usize| match k {
        0 => t.upos.name().to_string(),
        1 => t.tag.name().to_string(),
        2 => t.feats.to_string(),
        3 => t.lemma.to_lowercase(),
        _ => format!("{}:{}", t.head, t.rel.name()),
    };
    let mut ch = [[0usize; 3]; 5];
    let mut hint = [[0usize; 2]; 5];
    for (id, fa) in &f {
        let (Some(aa), Some(ba), Some(ga)) = (a.get(id), b.get(id), gold.get(id)) else { continue };
        for i in 0..ga.len() {
            for k in 0..5 {
                let (x, y, z, g) = (key(&aa[i], k), key(&fa[i], k), key(&ba[i], k), key(&ga[i], k));
                if x != y {
                    ch[k][if y == g { 0 } else if x == g { 1 } else { 2 }] += 1;
                }
                if x != g && z == g {
                    hint[k][0] += 1;
                    hint[k][1] += (y == g) as usize;
                }
            }
        }
    }
    println!("second look against the first ({} sentences):", f.len());
    for k in 0..5 {
        println!(
            "  {:<15} changes {:>2}: fixed {}, broke {}, wrong → wrong {}; Rust was right where Opus erred: {}, of them taken {}",
            names[k],
            ch[k].iter().sum::<usize>(),
            ch[k][0],
            ch[k][1],
            ch[k][2],
            hint[k][0],
            hint[k][1]
        );
    }
    Ok(())
}

/// UDPipe 2 (LINDAT) on the whole test file — a baseline for comparison only: UDPipe models
/// are NC-licensed, so their output goes neither into the corpus nor into training.
///
///   annot udpipe-eval <test.conllu>
fn udpipe_eval(test: &Path) -> Result<()> {
    let reg = Registry::load();
    let items = calib_items(test, usize::MAX, 1)?;
    let (mut out, mut failed) = (BTreeMap::new(), 0usize);
    for chunk in items.chunks(200) {
        let (a, f) = udpipe(chunk, &reg)?;
        out.extend(a);
        failed += f;
    }
    let gold: BTreeMap<String, Ann> = items.iter().filter(|it| out.contains_key(&it.id)).map(|it| (it.id.clone(), it.gold.clone().expect("gold"))).collect();
    println!("UDPipe 2 ({UDPIPE_MODEL}): {} sentences, passed the gates {}, rejected {failed}", items.len(), out.len());
    println!("| annotation | UPOS | XPOS | UFeats | Feats | AllTags | Lemmas | UAS | LAS | LAS with subtypes | CLAS | MLAS | BLEX |");
    println!("| UDPipe 2 | {} |", score(&out, &gold).row());
    Ok(())
}

/// Where FEATS diverge from gold: "UPOS: −gold_pair +our_pair" by frequency, with examples.
/// Only words with the correct UPOS — to see the feature tables, not tagging errors.
///
///   annot feats-diff <file.conllu> <test.conllu> [top]
fn feats_diff(file: &Path, test: &Path, top: usize) -> Result<()> {
    let reg = Registry::load();
    let ours = read_anns(file, &reg)?;
    let gold: BTreeMap<String, Ann> = conllu::read(test)?.into_iter().filter(|s| ours.contains_key(&s.id)).map(|s| (s.id.clone(), s.tokens.iter().map(gold_tok).collect())).collect();
    let pairs = |t: &Tok| -> Vec<String> { t.feats.to_string().split('|').filter(|x| *x != "_").map(str::to_string).collect() };
    let mut c: BTreeMap<String, (usize, Vec<String>)> = BTreeMap::new();
    let (mut words, mut wrong) = (0usize, 0usize);
    for (id, a) in &ours {
        let Some(g) = gold.get(id) else { continue };
        for (x, y) in a.iter().zip(g) {
            if x.upos != y.upos {
                continue;
            }
            words += 1;
            if x.feats == y.feats {
                continue;
            }
            wrong += 1;
            let (xs, ys) = (pairs(x), pairs(y));
            let mut d: Vec<String> = ys.iter().filter(|p| !xs.contains(p)).map(|p| format!("−{p}")).collect();
            d.extend(xs.iter().filter(|p| !ys.contains(p)).map(|p| format!("+{p}")));
            let e = c.entry(format!("{}: {}", y.upos, d.join(" "))).or_default();
            e.0 += 1;
            if e.1.len() < 4 && !e.1.contains(&x.form.to_lowercase()) {
                e.1.push(x.form.to_lowercase());
            }
        }
    }
    let mut v: Vec<_> = c.into_iter().collect();
    v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then(a.0.cmp(&b.0)));
    println!("words with correct UPOS {words}, wrong features in {wrong} ({:.2}%)", 100.0 * wrong as f64 / words.max(1) as f64);
    for (k, (n, ex)) in v.into_iter().take(top) {
        println!("{n:>4}  {k}   [{}]", ex.join(", "));
    }
    Ok(())
}

/// Measurement: FEATS (and UPOS, if set) from the Rust tables on top of the Opus annotation — its tags,
/// relations and tree. Can the expensive adjudication for features be replaced by deterministic code?
///
///   annot refeat <train.conllu>... <run-dir> <test.conllu>
fn refeat(train: &[String], run: &Path, test: &Path) -> Result<()> {
    let reg = Registry::load();
    let mut sents = Vec::new();
    for p in train {
        sents.extend(conllu::read(Path::new(p))?.into_iter().filter(|s| s.tagged()));
    }
    let ud = Ud::train(&sents);
    let gold: BTreeMap<String, Ann> = conllu::read(test)?.into_iter().map(|s| (s.id.clone(), s.tokens.iter().map(gold_tok).collect())).collect();
    for name in ["opus-a.conllu", "final.conllu"] {
        let Ok(a) = read_anns(&run.join(name), &reg) else { continue };
        let re: BTreeMap<String, Ann> = a
            .iter()
            .map(|(id, x)| {
                let y: Ann = x.iter().zip(rust_feats(&ud, x)).map(|(t, f)| Tok { feats: f, ..t.clone() }).collect();
                (id.clone(), y)
            })
            .collect();
        let g: BTreeMap<String, Ann> = gold.iter().filter(|(k, _)| a.contains_key(*k)).map(|(k, v)| (k.clone(), v.clone())).collect();
        println!("{name}: as is          | {} |", score(&a, &g).row());
        println!("{name}: FEATS from Rust | {} |", score(&re, &g).row());
    }
    Ok(())
}

/// English examples from the seeds (`en/seeds/**.md`) into a file `seeds:<file>:<hash> \t sentence`.
/// A seed, as in a cherry stone: the kernel from which a rule grows, hence `seeds`. The text hash
/// keeps the id stable when a seed is edited; identical sentences from different seeds go once.
fn seed_examples(dir: &Path, out: &Path) -> Result<()> {
    let mut stack = vec![dir.to_path_buf()];
    let mut files = Vec::new();
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d)?.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == "md") {
                files.push(p);
            }
        }
    }
    files.sort();
    let mut seen = HashSet::new();
    let mut rows = Vec::new();
    for f in files {
        let rel = f.strip_prefix(dir).unwrap_or(&f).display().to_string();
        for p in examples_in(&std::fs::read_to_string(&f)?) {
            if seen.insert(p.clone()) {
                let h = p.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3));
                rows.push(format!("seeds:{rel}:{:08x}\t{p}", h >> 32));
            }
        }
    }
    std::fs::write(out, rows.join("\n") + "\n")?;
    println!("examples {} → {}", rows.len(), out.display());
    Ok(())
}

/// Examples from the "Examples" section of a seed (the old Ukrainian header is also accepted):
/// fragments in quotes «» "" “” or italics *…*, and without them — a list item up to the dash. Only
/// plain sentences are taken: Latin letters, digits, punctuation; ≥ 3 words, ≥ 2 of them lowercase
/// (not a tag list). No ungrammatical examples — the corpus gets only what a speaker would say:
/// asterisk `*He go`, a leading `?`, a line with ✗, "ungrammatical…", "incorrect…" are skipped.
fn examples_in(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut on = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with("**Examples") || t.starts_with("**\u{41f}\u{440}\u{438}\u{43a}\u{43b}\u{430}\u{434}\u{438}") {
            on = true;
        } else if on && (t.starts_with("**") || t.starts_with("```") || t.starts_with('#')) {
            on = false;
        }
        let lower = t.to_lowercase();
        if !on || t.contains('✗') || lower.contains("ungrammatical") || lower.contains("incorrect") {
            continue;
        }
        let mut parts: Vec<&str> = Vec::new();
        for (open, close) in [('«', '»'), ('"', '"'), ('“', '”'), ('*', '*')] {
            let mut rest = t;
            while let Some(a) = rest.find(open) {
                // a doubt mark before the quote or italics: ?*…*, #«…»
                let marked = rest[..a].ends_with(['?', '#', '\\']);
                let after = &rest[a + open.len_utf8()..];
                let Some(b) = after.find(close) else { break };
                parts.push(if marked { "" } else { &after[..b] });
                rest = &after[b + close.len_utf8()..];
            }
        }
        if parts.is_empty() {
            if let Some(item) = t.strip_prefix("- ") {
                parts.push(item.split(" — ").next().unwrap_or(""));
            }
        }
        for p in parts {
            let p = p.trim();
            let plain = p.chars().all(|c| c.is_ascii_alphanumeric() || c.is_whitespace() || "'’-,.!?\"“”".contains(c));
            let words = p.split_whitespace().count();
            let lower_words = p.split_whitespace().filter(|w| w.chars().filter(char::is_ascii_lowercase).count() >= 2).count();
            if plain && !p.starts_with(['?', '-', '\'']) && !p.contains("...") && words >= 3 && lower_words >= 2 {
                out.push(p.to_string());
            }
        }
    }
    out
}

/// Bronze: sentences annotated by Rust only (no Opus), full UD; appended to the corpus with their source.
/// Seed examples are also fed into the corpus as bronze.
fn bronze(train: &[PathBuf], file: &Path, corpus: &Path) -> Result<()> {
    en::license::commercial_gate(train)?;
    let mut sents = Vec::new();
    for p in train {
        sents.extend(conllu::read(p)?.into_iter().filter(|s| s.tagged()));
    }
    let ud = Ud::train(&sents);
    let model = Model::train(sents);
    let reg = Registry::load();
    let have: HashSet<String> = std::fs::read_to_string(corpus).unwrap_or_default().lines().filter_map(|l| l.strip_prefix("# sent_id = ").map(str::to_string)).collect();
    let mut add = String::new();
    let (mut n, mut skipped, mut bad) = (0usize, 0usize, 0usize);
    for line in std::fs::read_to_string(file)?.lines() {
        let Some((id, text)) = line.split_once('\t') else { continue };
        if have.contains(id) {
            skipped += 1;
            continue;
        }
        let forms: Vec<String> = en::tok::tokenize(text, &model.lx).into_iter().map(|x| x.form).collect();
        if forms.is_empty() {
            continue;
        }
        let a = rust_annotate(&model, &ud, &forms);
        // the same gates as for silver: UD 2.18 registry, tree
        if let Err(e) = ann::check(&a, &forms, &reg) {
            eprintln!("gate: {id}: {e}");
            bad += 1;
            continue;
        }
        let src = provenance(id);
        let _ = writeln!(add, "# source = {src}");
        let _ = writeln!(add, "# annotation = Mova bronze: Rust only (en), not checked by Opus");
        let _ = writeln!(add, "# text = {text}");
        add.push_str(&write_block(id, &a));
        n += 1;
    }
    if let Some(d) = corpus.parent() {
        std::fs::create_dir_all(d)?;
    }
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(corpus)?;
    std::io::Write::write_all(&mut f, add.as_bytes())?;
    println!("bronze: +{n} sentences into {} (already there: {skipped}, failed the gate: {bad})", corpus.display());
    Ok(())
}

/// Sentences from the open papers of the library (CC BY 4.0) for silver: `papers:<folder>:<n> \t sentence`.
/// Plain paragraphs are taken (not headings, captions, lists, references), sentences of 6–40 words,
/// mostly letters, without links, citation brackets and formulas.
fn sentences(root: &Path, out: &Path, max: usize) -> Result<()> {
    let mut rows: Vec<String> = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    let mut dirs = Vec::new();
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d)?.flatten() {
            let p = e.path();
            if p.is_dir() {
                if p.join("meta.yaml").is_file() {
                    dirs.push(p);
                } else {
                    stack.push(p);
                }
            }
        }
    }
    dirs.sort();
    for d in dirs {
        let meta = std::fs::read_to_string(d.join("meta.yaml")).unwrap_or_default();
        if !meta.contains("redistributable: true") || !meta.contains("CC BY 4.0") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(d.join("text.md")) else { continue };
        let body = text.split("## References").next().unwrap_or(&text);
        let folder = d.strip_prefix(root).unwrap_or(&d).display().to_string();
        let mut k = 0;
        for para in body.split("\n\n") {
            let p = para.trim();
            if p.starts_with(['#', '*', '!', '<', '|', '-']) || p.split_whitespace().count() < 12 {
                continue;
            }
            let mut cur = String::new();
            for w in p.split_whitespace() {
                if !cur.is_empty() {
                    cur.push(' ');
                }
                cur.push_str(w);
                let abbr = ["e.g.", "i.e.", "al.", "Fig.", "cf.", "vs.", "etc.", "Eq.", "Sec.", "No."].iter().any(|a| w.ends_with(a));
                if (w.ends_with('.') || w.ends_with('?') || w.ends_with('!')) && !abbr {
                    let s = cur.trim().to_string();
                    cur.clear();
                    let n = s.split_whitespace().count();
                    let letters = s.chars().filter(|c| c.is_alphabetic()).count();
                    let clean = !s.contains("http") && !s.contains('[') && !s.contains('=') && !s.contains('@') && !s.contains('(') && s.starts_with(char::is_uppercase);
                    // not a length filter but a PDF-junk filter: "sentences" of 1–2 words are headings and captions
                    if n >= 3 && letters * 10 >= s.chars().count() * 7 && clean {
                        k += 1;
                        rows.push(format!("papers:{folder}:{k}\t{s}"));
                    }
                }
            }
        }
    }
    shuffle(&mut rows, 7);
    rows.truncate(max);
    std::fs::write(out, rows.join("\n") + "\n")?;
    println!("sentences {} → {}", rows.len(), out.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::examples_in;

    #[test]
    fn seed_examples_skip_ungrammatical() {
        let md = "# X\n\n**Gist.** About *she walks to school*.\n**Examples.** *She walks to school.*; «They have left.»\n\
                  - *\\*She walk to school.*\n- ✗ *He go home now.*\n- *He go home now.* — ungrammatical\n\
                  - *They goes home now.* — incorrect\n\
                  - *walk → walked → walked*\n- NN, NNS, NNP\n- ?*The more is the better.*\n\
                  - The cat sat on the mat — sentence\n**In UD.** *not an example here*\n";
        let mut ex = examples_in(md);
        ex.sort();
        // negative control: asterisk, ✗, "ungrammatical", "incorrect", arrows, tags, "?" and text outside the section are not taken
        assert_eq!(ex, ["She walks to school.", "The cat sat on the mat", "They have left."]);
    }

    #[test]
    fn prag_patches_are_checked() {
        use super::*;
        let reg = Registry::load();
        let forms: Vec<String> = ["It", "rains", "."].iter().map(|x| x.to_string()).collect();
        let lines: Vec<String> = [
            "1\tIt\tit\tPRON\tPRP\tCase=Nom|Gender=Neut|Number=Sing|Person=3|PronType=Prs\t2\tnsubj\t_\t_",
            "2\trains\train\tVERB\tVBZ\tMood=Ind|Number=Sing|Person=3|Tense=Pres|VerbForm=Fin\t0\troot\t_\t_",
            "3\t.\t.\tPUNCT\t.\t_\t2\tpunct\t_\t_",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        let a = ann::parse_block(&lines, &forms, &reg).unwrap();
        let it = Item { id: "s1".into(), text: "It rains.".into(), forms: forms.clone(), gold: None };
        let it2 = Item { id: "s2".into(), ..it.clone() };
        let pairs = vec![(&it, &a), (&it2, &a)];
        let (mut bad, mut prag) = (BTreeMap::new(), BTreeMap::new());
        let out = apply_patches("s1 PRAG act=assert\ns2 PRAG act=shout\n", &pairs, &reg, &mut bad, &mut prag);
        assert_eq!(prag.get("s1"), Some(&vec![("act".to_string(), "assert".to_string())]));
        // negative control: a value outside the closed set refuses the sentence
        assert!(out.contains_key("s1") && !out.contains_key("s2"));
        assert!(bad.keys().any(|k| k.contains("s2")));
    }
}
