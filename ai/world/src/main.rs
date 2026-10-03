//! world — the world of a tale (closed part).
//!
//!   world prompt <doc>                              LLM prompt without a call — for review
//!   world vmm <doc> <dir>                           the LLM (claude -p, Opus high) writes changes → cmds-<doc>.txt
//!                                                   (one call; one retry, only on a format refusal)
//!   world build <doc> <dir> [--questions <tsv>]     gates and world → world-<doc>.md; answers → answers-<doc>.md
//!   world query <doc> <dir> <query…>                query to the world (see qa.rs)
//!
//! FairytaleQA (v2, a tale is `<name>` from `$FTQA/stories/<name>.tsv`):
//!   world ftqa-import <csv> <dir>                   Hugging Face CSV → stories/, questions/ and a summary
//!   world ftqa-prompt <name> [<part>]               v2 prompt without a call
//!   world ftqa-vmm <name> <dir>                     LLM v2 in parts → cmds-<name>.txt (up to 3 calls)
//!   world ftqa-build <name> <dir>                   gates and world → world-<name>.md
//!   world ftqa-query <name> <dir> <query…>          query to the world
//!
//! World v3 (non-linear narrative; testbed — `<dir>/stories/<name>.tsv`):
//!   world poly-prep <book.txt> <title> <end|-> <out.tsv>   cut out a story, sentences
//!   world v3-prompt <name> <dir> [<part>]           v3 prompt without a call
//!   world v3-estimate <dir> <name>…                 LLM cost estimate before a run
//!   world v3-vmm <name> <dir>                       LLM v3 in parts (ceiling WORLD_V3_BUDGET, $40)
//!   world v3-build <name> <dir>                     gates and world → story-<name>.md
//!   world v3-query <name> <dir> <query…>            v3 query (qa3.rs)
//!   world v3-eval <dir> <questions.tsv>             answers to frozen questions → answers-v3.md
//!   world v3-cost <dir>                             call accounting
//!
//! Sense check of trees (level-1 common sense, parse-error detector):
//!   world sense-check <in.conllu> [<gold.conllu>] [--show N]
//! Answers about a restricted book (short summary in our own words, copy gate, no quotations):
//!   world book-answer <book.txt> <question> [--index <book.events.tsv>] [--log <answers.jsonl>]
//! The SLM derives its own tree-repair rules (transformation-based learning with level-1 features):
//!   world induce <train.conllu>... --dev <dev.conllu> --test <test.conllu>... --out <dir> [--iters 40] [--min 8]
//!   (INDUCE_WITHOUT=matrix,idioms,verbclass,nouns,syntax,punct hides feature groups — knowledge ablation)
//!   world domains "<text>" [--genre tale]   (domain modules: slang, formal, humor, hidden meaning, with reasons)
//!   world register-stats <name> <register-books>... --ref <reference-books>... --out <words.tsv>   (register lexicon by log-odds)
//!   world domains-eval <set.jsonl> [--show N]   (precision/recall per domain, literal false alarms)
//!   world derive <train.conllu>... --test <test.conllu>... [--out <dir>] [--min 30] [--show N]   (trees by logical derivation from induced attachment rules)
//!   world derive-learn <book-list> [--sentences N] [--iters K] [--min M] --test <gold.conllu>...   (learn to derive the parser's trees)
//!   world family serve|say|log      (Mova Dev decides, Mova Current and Mova State <id> measure, Random commenters hint; all logged)
//!   world self      (who I am: the base self-description in plain English and the live state)
//!   world brain [show|parse|base|write <text|@file>|probe|serve|say] — the brain: the single cell (pragmatics in English)
//!   world state save|checkout|rollback|tree|diff|guard …   (the SLM's states: frozen original + a tree of deltas, rollback)
//!   world store build|versions|query …   (tree store: versions by model hash, metadata, feature index, queries)
//!   world tune <component> --dev <set> --held <set> [--gens N] [--threads T] [--adopt]   (any component's knobs: deltas, journal, reset)
//!   world selfplay --dev <conllu>... --held <conllu>... [--gens N] [--threads T] [--adopt]   (self-modification deltas tested in parallel, reported, reset)
//!   world derive-scan <book-list> [--rounds R] [--per-round K] [--threads T] [--limit N]   (whole corpus: what it derives already, learning in rounds)
//!   world rediscover-random <book-list> [--hide 2] [--trials 20] [--seed 1]   (hide random rules, re-induce them)
//!   world rediscover <book-list.txt> [--sentences N] [--iters K]   (hide each rule, re-induce it without gold)
//!   world repair-eval <gold.conllu>... [--domain real|tale]   (no repairs / hand / induced / both)
//! Event statistics of books (who did what to whom, counts only):
//!   world events <out-dir> <book.txt>... [--threads N]
//! Reranking the parser's k best trees with the absurdity matrix (λ on dev, LAS on test):
//!   world absurd-rerank <dev.conllu> <test.conllu> [--k 8]
//!   world absurd-repair <dev.conllu> <test.conllu> [--show N]   (label repairs driven by the matrix)
//!   world conllu-split <in.conllu> <even.conllu> <odd.conllu>
//!   world absurd-events <events.tsv> [--domain real|tale] [--top N]   (frequent absurd corpus events = systematic errors)
//! Absurdity matrix check (graded scores, judgments and gap journal):
//!   world absurd-check [<scores.json>] <in.conllu> [<gold.conllu>] --out <dir>
//!
//! Variables: MOVA_DB (data/db/mova.duckdb), DUCKDB (~/bin/duckdb), WORLD_TMP (temporary),
//! PRAG_MODEL (claude-opus-5-5), PRAG_EFFORT (high), FTQA (data/raw/fairytaleqa),
//! FTQA_PART (max sentences in one LLM call, 95).

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use world::{lang, md, qa, vmm, world as w};

fn home() -> PathBuf {
    PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into()))
}

fn db() -> PathBuf {
    std::env::var("MOVA_DB").map(PathBuf::from).unwrap_or_else(|_| home().join("db/mova.duckdb"))
}

fn tmp() -> PathBuf {
    std::env::var("WORLD_TMP").map(PathBuf::from).unwrap_or_else(|_| std::env::temp_dir().join("world"))
}

fn usage() -> Result<()> {
    bail!("usage: world prompt <doc> | world vmm <doc> <dir> | world build <doc> <dir> [--questions <tsv>] | world query <doc> <dir> <query…>")
}

fn ftqa_dir() -> PathBuf {
    std::env::var("FTQA").map(PathBuf::from).unwrap_or_else(|_| home().join("raw/fairytaleqa"))
}

/// `WORLD_V3=1` — old runs (fables, FairytaleQA) on the v3 executor with gates in story time.
fn v3_mode() -> bool {
    std::env::var("WORLD_V3").as_deref() == Ok("1")
}

fn ftqa_part() -> usize {
    std::env::var("FTQA_PART").ok().and_then(|x| x.parse().ok()).unwrap_or(95)
}

/// FairytaleQA tale: text from `stories/<name>.tsv` (doc is the ordinal number in the tale catalogue).
fn ftqa_text(name: &str) -> Result<w::Text> {
    let path = ftqa_dir().join("stories").join(format!("{name}.tsv"));
    let mut names: Vec<String> = std::fs::read_dir(ftqa_dir().join("stories"))?
        .filter_map(|e| e.ok()?.path().file_stem()?.to_str().map(str::to_string))
        .collect();
    names.sort();
    let doc = names.iter().position(|n| n == name).context(format!("no tale {name}"))? as i64 + 1;
    world::ftqa::read_story(&path, doc)
}

/// World of a FairytaleQA tale from the commands of a run folder.
pub fn load_ftqa(name: &str, dir: &Path) -> Result<(w::Run, Vec<lang::FormatErr>, usize)> {
    let t = ftqa_text(name)?;
    let path = dir.join(format!("cmds-{name}.txt"));
    let cmds = std::fs::read_to_string(&path).with_context(|| path.display().to_string())?;
    let (lines, errs) = lang::parse_all(&cmds);
    let total = lines.len() + errs.len();
    Ok((w::run_mode(t, &lines, v3_mode()), errs, total))
}

/// v3 testbed: text from `<dir>/stories/<name>.tsv`, title from the list (for the prompt).
fn poly_text(name: &str, dir: &Path) -> Result<w::Text> {
    let path = dir.join("stories").join(format!("{name}.tsv"));
    let doc = match name {
        "speckled-band" => 1661,
        "arabian-nights" => 128,
        _ => 0,
    };
    let mut t = world::ftqa::read_story(&path, doc)?;
    t.title = match name {
        "speckled-band" => "The Adventure of the Speckled Band (Arthur Conan Doyle)".into(),
        "arabian-nights" => "The Arabian Nights: the frame of Scheherazade and The Story of the Merchant and the Genius (Andrew Lang)".into(),
        _ => t.title,
    };
    Ok(t)
}

/// World of a v3 testbed from the commands of a run folder.
fn load_poly(name: &str, dir: &Path) -> Result<(w::Run, Vec<lang::FormatErr>, usize)> {
    let t = poly_text(name, dir)?;
    let path = dir.join(format!("cmds-{name}.txt"));
    let cmds = std::fs::read_to_string(&path).with_context(|| path.display().to_string())?;
    let (lines, errs) = lang::parse_all(&cmds);
    let total = lines.len() + errs.len();
    Ok((w::run_v3(t, &lines), errs, total))
}

fn v3_part() -> usize {
    std::env::var("V3_PART").ok().and_then(|x| x.parse().ok()).unwrap_or(95)
}

/// World of a document from the commands of a run folder: parsing, gates.
fn load(doc: i64, dir: &Path) -> Result<(w::Run, Vec<lang::FormatErr>, usize)> {
    let t = vmm::read_text(&db(), doc, &tmp())?;
    let path = dir.join(format!("cmds-{doc}.txt"));
    let cmds = std::fs::read_to_string(&path).with_context(|| path.display().to_string())?;
    let (lines, errs) = lang::parse_all(&cmds);
    let total = lines.len() + errs.len();
    Ok((w::run_mode(t, &lines, v3_mode()), errs, total))
}

fn dir_at(args: &[String], i: usize) -> Result<PathBuf> {
    Ok(PathBuf::from(args.get(i).context("missing folder")?))
}

fn main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    // the state of this run: `--state <hash>` (or MOVA_STATE) unpacks that state; without it, the last stable state;
    // the processing state and the last stable one are reported with every run (stderr), for reproducibility
    let requested = match args.iter().position(|a| a == "--state") {
        Some(i) if i + 1 < args.len() => {
            let h = args.remove(i + 1);
            args.remove(i);
            Some(h)
        }
        _ => std::env::var("MOVA_STATE").ok(),
    };
    let (processing, stable) = world::state::activate(requested.as_deref())?;
    // SAFETY: start-up, single-threaded
    unsafe { std::env::set_var("MOVA_PROCESSING_STATE", &processing) };
    if !matches!(args.first().map(String::as_str), Some("state") | None) {
        eprintln!("[state: processing {processing}, last stable {stable}]");
    }
    let Some(cmd) = args.first().map(String::as_str) else { return usage() };
    let doc = || -> Result<i64> { args.get(1).context("missing doc")?.parse().context("doc must be a number") };
    let dir = || -> Result<PathBuf> { Ok(PathBuf::from(args.get(2).context("missing folder")?)) };
    match cmd {
        "read" => {
            // the snake reads a book: read <book.txt> <dir> [--limit stories]
            let book = std::path::PathBuf::from(args.get(1).context("book")?);
            let d = dir_at(&args, 2)?;
            let limit = args.iter().position(|a| a == "--limit").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
            let n = world::read::read_book(&book, &d, limit)?;
            println!("paragraphs read: {n} → {}", d.join("summaries.jsonl").display());
        }
        "read-check" => {
            let d = dir_at(&args, 1)?;
            let limit = args.iter().position(|a| a == "--limit").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(usize::MAX);
            world::read::check(&d, limit, 15)?;
        }
        "read-qgen" => {
            // short questions from the LLM: read-qgen <book folder with summaries.jsonl> <dir> [--stories N]
            let book = std::path::PathBuf::from(args.get(1).context("book folder")?);
            let d = dir_at(&args, 2)?;
            let n = args.iter().position(|a| a == "--stories").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(20);
            world::read::qgen(&book, &d, n, 8)?;
        }
        "babi" => {
            // bAbI: babi <en folder> [test|train] [errors to show]
            let dir = std::path::PathBuf::from(args.get(1).map(String::as_str).unwrap_or("data/raw/babi/tasks_1-20_v1-2/en"));
            let which = args.get(2).map(String::as_str).unwrap_or("test");
            let show = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(0);
            world::babi::eval_dir(&dir, which, show)?;
        }
        "sense-check" => {
            // sense-check <in.conllu> [<gold.conllu>] [--show N]
            let show = args.iter().position(|a| a == "--show").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(10);
            let pos: Vec<&String> = args[1..].iter().enumerate().filter(|(i, a)| !a.starts_with("--") && (*i == 0 || args[*i] != "--show")).map(|(_, a)| a).collect();
            let input = pos.first().context("usage: world sense-check <in.conllu> [<gold.conllu>] [--show N]")?;
            world::sense::run(Path::new(input.as_str()), pos.get(1).map(|g| Path::new(g.as_str())), show)?;
        }
        "book-answer" => {
            // book-answer <book.txt> <question> [--index <book.events.tsv>] [--log <answers.jsonl>]
            let usage = "usage: world book-answer <book.txt> <question> [--index <book.events.tsv>] [--log <answers.jsonl>]";
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let (book, q) = (args.get(1).context(usage)?, args.get(2).context(usage)?);
            let (ix, log) = (opt("--index"), opt("--log"));
            world::bookans::run(Path::new(book.as_str()), q, ix.as_deref().map(Path::new), log.as_deref().map(Path::new))?;
        }
        "domains" => {
            // domains "<text>" [--genre tale]
            let genre = args.iter().position(|a| a == "--genre").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "real".into());
            world::domains::run(args.get(1).context("usage: world domains \"<text>\" [--genre tale]")?, &genre)?;
        }
        "register-stats" => {
            // register-stats <name> <register-books>... --ref <reference-books>... --out <words.tsv> [--z 3] [--min 20]
            let usage = "usage: world register-stats <name> <register-books>... --ref <reference-books>... --out <words.tsv> [--z 3] [--min 20]";
            let name = args.get(1).context(usage)?.clone();
            let (mut reg, mut refs) = (Vec::new(), Vec::new());
            let (mut out, mut z, mut min) = (None, 3.0f64, 20u64);
            let mut mode = 0;
            let mut i = 2;
            while i < args.len() {
                match args[i].as_str() {
                    "--ref" => mode = 1,
                    "--out" => { out = args.get(i + 1).cloned(); i += 1; }
                    "--z" => { z = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(3.0); i += 1; }
                    "--min" => { min = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(20); i += 1; }
                    a => { if mode == 0 { reg.push(std::path::PathBuf::from(a)) } else { refs.push(std::path::PathBuf::from(a)) } }
                }
                i += 1;
            }
            world::register::run(&name, &reg, &refs, Path::new(&out.context(usage)?), z, min)?;
        }
        "domains-eval" => {
            // domains-eval <set.jsonl> [--show N]
            let show = args.iter().position(|a| a == "--show").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(30);
            world::domains::eval(Path::new(args.get(1).context("usage: world domains-eval <set.jsonl> [--show N]")?), show)?;
        }
        "self" => world::state::print_self()?,
        "brain" => {
            // brain [show|parse|base]
            match args.get(1).map(String::as_str) {
                Some("probe") => {
                    let list = |k: &str| -> Vec<String> { args.windows(2).filter(|w| w[0] == k).map(|w| w[1].clone()).collect() };
                    world::mind::probe(Path::new(args.get(2).context("usage: world brain probe <prompt.md> [--letter T]... [--ask Q]...")?), &list("--letter"), &list("--ask"))?;
                }
                Some("serve") => world::mind::serve()?,
                Some("say") => println!("{}", world::mind::say(args.get(2).context("usage: world brain say \"<English line>\"")?)?),
                Some("parse") => {
                    let (sents, cached) = world::state::brain_parse()?;
                    println!("{} sentences{}", sents.len(), if cached { " (from the parse cache)" } else { " (parsed, cached now)" });
                }
                Some("base") => print!("{}", world::state::brain_describe(&[], &[])),
                Some("write") => {
                    let t = args.get(2).context("usage: world brain write <text|@file>")?;
                    let text = match t.strip_prefix('@') { Some(f) => std::fs::read_to_string(f)?, None => t.clone() };
                    world::state::brain_write(&text)?;
                }
                Some("versions") => {
                    if !world::state::in_recovery() {
                        anyhow::bail!("earlier brain snapshots are listed only in the recovery window after a crash");
                    }
                    for (k, d) in world::state::brain_versions().into_iter().enumerate() {
                        println!("{k}\t{d}");
                    }
                }
                Some("recover") => world::state::brain_recover(args.iter().position(|a| a == "--version").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).context("usage: world brain recover --version K")?)?,
                _ => print!("{}", world::state::brain_read()),
            }
        }
        "family" => {
            // family serve [--kids N] | say --as <name> "<text>" | log [N]
            let usage = "usage: world family serve [--states N] | world family say --as <name> \"<English>\" | world family log [N]";
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            match args.get(1).map(String::as_str) {
                Some("serve") => world::family::serve(opt("--states").and_then(|s| s.parse().ok()).unwrap_or(3))?,
                Some("say") => {
                    let who = opt("--as").context(usage)?;
                    let text = args.iter().skip(2).filter(|a| **a != "--as" && **a != who).cloned().collect::<Vec<_>>().join(" ");
                    print!("{}", world::family::say(&who, &text)?);
                }
                Some("log") => world::family::log(args.get(2).and_then(|s| s.parse().ok()).unwrap_or(30))?,
                _ => anyhow::bail!(usage),
            }
        }
        "letters" => {
            // letters [N] — the frozen letters; the last one is the goal of the current training
            match args.get(1).and_then(|x| x.parse::<usize>().ok()) {
                Some(n) => print!("{}", world::state::letter_read(n)?),
                None => {
                    for n in 0..world::state::letter_count() {
                        println!("{n}\t{}", world::state::letter_read(n)?.lines().next().unwrap_or(""));
                    }
                }
            }
        }
        "state" => {
            // state save <note> [--parent ID] [--metric M] | checkout <ID> | rollback | tree | diff <A> <B> | guard --eval <gold> [--domain D] [--max-drop W]
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let usage = "usage: world state save <note> [--parent ID] [--metric M] [--letter TEXT] | return <report> | become [note] | story | checkout <ID|origin> | rollback | tree | diff <A> <B> | guard --eval <gold.conllu> [--domain tale] [--max-drop W]";
            match args.get(1).map(String::as_str) {
                Some("save") => println!("{}", world::state::save_with_letter(args.get(2).context(usage)?, opt("--parent").as_deref(), &opt("--metric").unwrap_or_else(|| "-".into()), opt("--letter").as_deref())?),
                Some("return") => println!("returned to {}", world::state::return_with(args.get(2).context(usage)?)?),
                Some("become") => world::state::become_new(args.get(2).map(String::as_str).unwrap_or(""))?,
                Some("story") => print!("{}", world::state::story()?),
                Some("stable") => match args.get(2) {
                    Some(id) => world::state::mark_stable(id)?,
                    None => println!("{}", world::state::stable()),
                },
                Some("checkout") => world::state::checkout(args.get(2).context(usage)?)?,
                // state las <id> <gold.conllu> <tale|real>: LAS of a state (for the final report on test sets)
                Some("las") => println!("{:.2}", world::state::state_las(args.get(2).context(usage)?, std::path::Path::new(args.get(3).context(usage)?), args.get(4).map(String::as_str).unwrap_or("real"))?),
                Some("rollback") => println!("rolled back to {}", world::state::rollback()?),
                Some("tree") => world::state::print_tree()?,
                Some("diff") => world::state::print_diff(args.get(2).context(usage)?, args.get(3).context(usage)?)?,
                Some("guard") => world::state::guard(Path::new(&opt("--eval").context(usage)?), &opt("--domain").unwrap_or_else(|| "real".into()), opt("--max-drop").and_then(|s| s.parse().ok()).unwrap_or(0.05))?,
                _ => anyhow::bail!(usage),
            }
        }
        "store" => {
            // store build <book-list> --out <dir> [--threads T] [--limit N] | store versions <dir> | store query <dir> "<query>" [--model H] [--limit N] [--show N]
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let num = |k: &str, d: usize| opt(k).and_then(|s| s.parse().ok()).unwrap_or(d);
            let usage = "usage: world store build <book-list> --out <dir> [--threads T] [--limit N] | world store update <dir> | world store set save|load|list <dir> [<name> [\"<query>\"]] | world store versions <dir> | world store query <dir> \"<query>\" [--model H] [--limit N] [--show N]";
            match args.get(1).map(String::as_str) {
                Some("build") => world::store::build(Path::new(args.get(2).context(usage)?), Path::new(&opt("--out").context(usage)?), num("--threads", 10), num("--limit", usize::MAX))?,
                Some("versions") => world::store::versions(Path::new(args.get(2).context(usage)?))?,
                Some("update") => world::store::update(Path::new(args.get(2).context(usage)?), num("--threads", 10), &opt("--branch").unwrap_or_else(|| "main".into()))?,
                Some("ref") => {
                    let dir = Path::new(args.get(2).context(usage)?);
                    match (args.get(3), args.get(4)) {
                        (Some(n), Some(h)) => world::store::ref_set(dir, n, h)?,
                        (Some(n), None) => println!("{}", world::store::ref_get(dir, n).unwrap_or_else(|| "-".into())),
                        _ => {
                            if let Ok(rd) = std::fs::read_dir(dir.join("refs")) {
                                for e in rd.flatten() {
                                    println!("{}\t{}", e.file_name().to_string_lossy(), std::fs::read_to_string(e.path()).unwrap_or_default().trim());
                                }
                            }
                        }
                    }
                }
                Some("set") => {
                    let dir = Path::new(args.get(3).context(usage)?);
                    let st = world::store::Store::open(dir)?;
                    let hash = opt("--model").or_else(|| st.latest().map(String::from)).context("no version")?;
                    match args.get(2).map(String::as_str) {
                        Some("save") => println!("{} sentences", world::store::set_save(dir, args.get(4).context(usage)?, args.get(5).context(usage)?, &hash)?),
                        Some("load") => println!("{} sentences", world::store::set_load(dir, args.get(4).context(usage)?, &hash)?.len()),
                        _ => world::store::set_list(dir)?,
                    }
                }
                Some("query") => world::store::query(Path::new(args.get(2).context(usage)?), args.get(3).context(usage)?, opt("--model").as_deref(), num("--limit", 20), num("--show", 5))?,
                _ => anyhow::bail!(usage),
            }
        }
        "tune" => {
            // tune <component> --dev <set> --held <set> [--gens N] [--threads T] [--adopt]
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let usage = "usage: world tune <component> --dev <set> --held <set> [--gens N] [--threads T] [--adopt]";
            world::tune::run(args.get(1).context(usage)?, Path::new(&opt("--dev").context(usage)?), Path::new(&opt("--held").context(usage)?), opt("--gens").and_then(|s| s.parse().ok()).unwrap_or(4), opt("--threads").and_then(|s| s.parse().ok()).unwrap_or(16), args.iter().any(|a| a == "--adopt"))?;
        }
        "selfplay" => {
            // selfplay --dev <conllu>... --held <conllu>... [--gens 3] [--threads 24] [--adopt] [--out <dir>]
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let list = |k: &str| -> Vec<std::path::PathBuf> { args.iter().position(|a| a == k).map(|i| args[i + 1..].iter().take_while(|a| !a.starts_with("--")).map(std::path::PathBuf::from).collect()).unwrap_or_default() };
            let gens = opt("--gens").and_then(|s| s.parse().ok()).unwrap_or(3);
            let threads = opt("--threads").and_then(|s| s.parse().ok()).unwrap_or(24);
            let out = opt("--out").unwrap_or_else(|| "selfplay".into());
            world::induce::selfplay(&list("--dev"), &list("--held"), gens, threads, args.iter().any(|a| a == "--adopt"), Path::new(&out))?;
        }
        "derive-scan" => {
            // derive-scan <book-list> [--rounds R] [--per-round K] [--threads T] [--limit N] [--out <dir>]
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let num = |k: &str, d: usize| opt(k).and_then(|s| s.parse().ok()).unwrap_or(d);
            let out = opt("--out").unwrap_or_else(|| "derive-scan".into());
            match opt("--store") {
                Some(dir) => world::derive::scan_store(Path::new(&dir), &opt("--query").unwrap_or_else(|| "prefix=1%".into()), num("--rounds", 5), num("--per-round", 8), num("--threads", 10), Path::new(&out))?,
                None => world::derive::scan(Path::new(args.get(1).context("usage: world derive-scan <book-list> | --store <dir> [--query Q] [--rounds R] [--per-round K] [--threads T] [--limit N] [--out <dir>]")?), num("--rounds", 5), num("--per-round", 8), num("--threads", 10), num("--limit", usize::MAX), Path::new(&out))?,
            }
        }
        "derive-learn" => {
            // derive-learn <book-list> [--sentences N] [--iters K] [--min M] [--out <dir>] --test <gold.conllu>...
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let num = |k: &str, d: usize| opt(k).and_then(|s| s.parse().ok()).unwrap_or(d);
            let test: Vec<std::path::PathBuf> = args.iter().position(|a| a == "--test").map(|i| args[i + 1..].iter().take_while(|a| !a.starts_with("--")).map(std::path::PathBuf::from).collect()).unwrap_or_default();
            world::derive::run_learn(Path::new(args.get(1).context("usage: world derive-learn <book-list> [--sentences N] [--iters K] [--min M] [--out <dir>] --test <gold.conllu>...")?), num("--sentences", 20000), num("--iters", 60), num("--min", 20), &test, opt("--out").as_deref().map(Path::new))?;
        }
        "derive" => {
            // derive <train.conllu>... --test <test.conllu>... [--out <dir>] [--min 30] [--show N]
            let (mut train, mut test) = (Vec::new(), Vec::new());
            let (mut out, mut min, mut show) = (None, 30usize, 0usize);
            let mut mode = 0;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--test" => mode = 1,
                    "--out" => { out = args.get(i + 1).cloned(); i += 1; }
                    "--min" => { min = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(30); i += 1; }
                    "--show" => { show = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(0); i += 1; }
                    a => { if mode == 0 { train.push(std::path::PathBuf::from(a)) } else { test.push(std::path::PathBuf::from(a)) } }
                }
                i += 1;
            }
            world::derive::run(&train, &test, out.as_deref().map(Path::new), min, show)?;
        }
        "rediscover-random" => {
            // rediscover-random <book-list> [--hide 2] [--trials 20] [--seed 1] [--sentences N] [--iters K]
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).and_then(|s| s.parse::<u64>().ok());
            world::induce::rediscover_random(Path::new(args.get(1).context("usage: world rediscover-random <book-list> [--hide 2] [--trials 20] [--seed 1] [--sentences N] [--iters K]")?), opt("--hide").unwrap_or(2) as usize, opt("--trials").unwrap_or(20) as usize, opt("--seed").unwrap_or(1), opt("--sentences").unwrap_or(8000) as usize, opt("--iters").unwrap_or(5) as usize)?;
        }
        "rediscover" => {
            // rediscover <book-list.txt> [--sentences N] [--iters K]
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let n = opt("--sentences").and_then(|s| s.parse().ok()).unwrap_or(6000);
            let k = opt("--iters").and_then(|s| s.parse().ok()).unwrap_or(4);
            world::induce::rediscover(Path::new(args.get(1).context("usage: world rediscover <book-list.txt> [--sentences N] [--iters K]")?), n, k)?;
        }
        "repair-eval" => {
            // repair-eval <gold.conllu>... [--domain real|tale]
            let domain = args.iter().position(|a| a == "--domain").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "real".into());
            let paths: Vec<std::path::PathBuf> = args[1..].iter().filter(|a| !a.starts_with("--") && **a != domain).map(std::path::PathBuf::from).collect();
            world::rerank::repair_eval(&paths, &domain)?;
        }
        "induce" => {
            // induce <train.conllu>... --dev <dev.conllu> --test <test.conllu>... --out <dir> [--iters 40] [--min 8]
            let usage = "usage: world induce <train.conllu>... --dev <dev.conllu> --test <test.conllu>... --out <dir> [--iters 40] [--min 8]";
            let mut train = Vec::new();
            let mut test = Vec::new();
            let (mut dev, mut out, mut iters, mut min) = (None, None, 40usize, 8usize);
            let mut mode = 0;
            let mut i = 1;
            while i < args.len() {
                match args[i].as_str() {
                    "--dev" => { dev = args.get(i + 1).cloned(); i += 2; continue; }
                    "--out" => { out = args.get(i + 1).cloned(); i += 2; continue; }
                    "--iters" => { iters = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(40); i += 2; continue; }
                    "--min" => { min = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(8); i += 2; continue; }
                    "--test" => { mode = 1; i += 1; continue; }
                    a => { if mode == 0 { train.push(std::path::PathBuf::from(a)) } else { test.push(std::path::PathBuf::from(a)) } }
                }
                i += 1;
            }
            world::induce::run(&train, Path::new(&dev.context(usage)?), &test, Path::new(&out.context(usage)?), iters, min)?;
        }
        "idioms" => {
            // idioms <cells.tsv> <events.tsv> <wikt-multiword.tsv> --out <dir> [--min 20] [--books 10]
            let usage = "usage: world idioms <cells.tsv> <events.tsv> <wikt-multiword.tsv> [--mwe <mwe-en-layer.tsv>] --out <dir> [--min 20] [--books 10]";
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let out = opt("--out").context(usage)?;
            let min = opt("--min").and_then(|s| s.parse().ok()).unwrap_or(20);
            let books = opt("--books").and_then(|s| s.parse().ok()).unwrap_or(10);
            let mwe = opt("--mwe");
            world::idioms::run(Path::new(args.get(1).context(usage)?), Path::new(args.get(2).context(usage)?), Path::new(args.get(3).context(usage)?), mwe.as_deref().map(Path::new), Path::new(&out), min, books)?;
        }
        "events-show" => {
            // events-show <book.txt> <n>...
            let book = args.get(1).context("usage: world events-show <book.txt> <n>...")?;
            let ns: Vec<usize> = args[2..].iter().filter_map(|x| x.parse().ok()).collect();
            world::events::show(Path::new(book.as_str()), &ns)?;
        }
        "events" => {
            // events <out-dir> <book.txt>... [--threads N]
            let threads = args.iter().position(|a| a == "--threads").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(8);
            let mut pos = Vec::new();
            let mut i = 1;
            while i < args.len() {
                if args[i] == "--threads" { i += 2 } else { pos.push(args[i].clone()); i += 1 }
            }
            let out = pos.first().context("usage: world events <out-dir> <book.txt>... [--threads N]")?;
            let books: Vec<std::path::PathBuf> = pos[1..].iter().map(std::path::PathBuf::from).collect();
            world::events::run(Path::new(out.as_str()), &books, threads)?;
        }
        "absurd-events" => {
            // absurd-events <events.tsv> [--domain real|tale] [--top N]
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let domain = opt("--domain").unwrap_or_else(|| "real".into());
            let top = opt("--top").and_then(|s| s.parse().ok()).unwrap_or(60);
            world::absurd::events(Path::new(args.get(1).context("usage: world absurd-events <events.tsv> [--domain real|tale] [--top N]")?), &domain, top)?;
        }
        "absurd-rerank" => {
            // absurd-rerank <dev.conllu> <test.conllu> [--k 8]
            let k = args.iter().position(|a| a == "--k").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(8);
            let usage = "usage: world absurd-rerank <dev.conllu> <test.conllu> [--k 8]";
            world::rerank::run(Path::new(args.get(1).context(usage)?), Path::new(args.get(2).context(usage)?), k)?;
        }
        "absurd-repair" => {
            // absurd-repair <dev.conllu> <test.conllu> [--show N]
            let show = args.iter().position(|a| a == "--show").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(0);
            let usage = "usage: world absurd-repair <dev.conllu> <test.conllu> [--show N]";
            world::rerank::run_repair(Path::new(args.get(1).context(usage)?), Path::new(args.get(2).context(usage)?), show)?;
        }
        "conllu-split" => {
            // conllu-split <in.conllu> <even.conllu> <odd.conllu>
            let usage = "usage: world conllu-split <in.conllu> <even.conllu> <odd.conllu>";
            world::rerank::split(Path::new(args.get(1).context(usage)?), Path::new(args.get(2).context(usage)?), Path::new(args.get(3).context(usage)?))?;
        }
        "absurd-check" => {
            // absurd-check [<scores.json>] <in.conllu> [<gold.conllu>] --out <dir>
            let usage = "usage: world absurd-check [<scores.json>] <in.conllu> [<gold.conllu>] --out <dir> [--domain real|tale]";
            let o = args.iter().position(|a| a == "--out").and_then(|i| args.get(i + 1)).context(usage)?;
            let domain = args.iter().position(|a| a == "--domain").and_then(|i| args.get(i + 1)).cloned().unwrap_or_else(|| "real".into());
            let pos: Vec<&String> = args[1..].iter().filter(|a| !a.starts_with("--") && *a != o && **a != domain).collect();
            let sc = pos.first().filter(|p| p.ends_with(".json")).map(|p| Path::new(p.as_str()));
            let rest = &pos[sc.is_some() as usize..];
            let input = rest.first().context(usage)?;
            world::absurd::run(sc, Path::new(input.as_str()), rest.get(1).map(|g| Path::new(g.as_str())), Path::new(o.as_str()), &domain)?;
        }
        "babi-learn" => world::babi::learn_motivations(std::path::Path::new(&args[1]))?,
        "babi-contrast" => world::babi::contrast(std::path::Path::new(&args[1]), std::path::Path::new(&args[2]))?,
        "stepgame" => {
            // stepgame [folder] [epochs] [errors to show]
            let dir = std::path::PathBuf::from(args.get(1).map(String::as_str).unwrap_or("data/raw/stdata-stepgame"));
            world::stepgame::eval(&dir, args.get(2).and_then(|x| x.parse().ok()).unwrap_or(10), args.get(3).and_then(|x| x.parse().ok()).unwrap_or(0))?;
        }
        "big-diff" => {
            // big-diff <state> <tale|real> [n]: words whose label the state changes against the original on a big sample
            let id = args.get(1).context("usage: world big-diff <state> <tale|real> [n]")?;
            let dom = args.get(2).map(String::as_str).unwrap_or("real");
            let n: usize = args.get(3).and_then(|x| x.parse().ok()).unwrap_or(40);
            let big = world::big::sample(&format!("big-{dom}@3"))?;
            let pick = |lines: Vec<String>| -> Vec<world::induce::Rule> {
                world::induce::parse_rules(&lines.iter().map(|l| format!("{l}\n")).collect::<String>()).into_iter().filter(|(s, _)| dom == "tale" || s == "general").map(|(_, r)| r).collect()
            };
            let (r0, r1) = (pick(world::state::state_rules_scoped("origin")), pick(world::state::state_rules_scoped(id)));
            let m = world::absurd::Matrix::global();
            let (mut shown, mut changed) = (0, 0);
            for ws in big.iter() {
                let (mut a, mut b) = (ws.clone(), ws.clone());
                world::induce::apply_words(&m, &mut a, &r0);
                world::induce::apply_words(&m, &mut b, &r1);
                for i in 0..a.len() {
                    if a[i].rel != b[i].rel {
                        changed += 1;
                        if shown < n {
                            shown += 1;
                            let h = a[i].head;
                            let head = if h > 0 { a[h - 1].form.as_str() } else { "ROOT" };
                            println!("{} → {}  «{}» of «{}»  | {}", a[i].rel, b[i].rel, a[i].form, head, a.iter().map(|w| w.form.as_str()).collect::<Vec<_>>().join(" "));
                        }
                    }
                }
            }
            println!("changed words: {changed} in {} sentences", big.len());
        }
        "big-judge" => {
            // big-judge <state> <tale|real>: the teacher's check (Opus) and the grammar check of a state's relabelled words
            let id = args.get(1).context("usage: world big-judge <state> <tale|real>")?;
            let dom = args.get(2).map(String::as_str).unwrap_or("real");
            let big = world::big::sample(&format!("big-{dom}@3"))?;
            let pick = |lines: Vec<String>| -> Vec<world::induce::Rule> {
                world::induce::parse_rules(&lines.iter().map(|l| format!("{l}\n")).collect::<String>()).into_iter().filter(|(s, _)| dom == "tale" || s == "general").map(|(_, r)| r).collect()
            };
            let (r0, r1) = (pick(world::state::state_rules_scoped("origin")), pick(world::state::state_rules_scoped(id)));
            let m = world::absurd::Matrix::global();
            let (mut v0, mut v1) = (0i64, 0i64);
            for ws in big.iter() {
                let (mut a, mut b) = (ws.clone(), ws.clone());
                world::induce::apply_words(&m, &mut a, &r0);
                world::induce::apply_words(&m, &mut b, &r1);
                v0 += world::induce::violations(&a);
                v1 += world::induce::violations(&b);
            }
            println!("grammar violations: original {v0}, state {v1} ({:+})", v1 - v0);
            let (items, total) = world::induce::changed_words(&r0, &r1, &big, 20);
            let (nr, or, ne) = world::big::judge_changes(&items)?;
            println!("teacher on {} of {total} changed words: new right {nr}, old right {or}, neither {ne}", items.len());
        }
        "big-explore" => {
            // big-explore <tale|real>: ideas from a big store sample judged by the absurdity matrix (no gold)
            let dom = args.get(1).map(String::as_str).unwrap_or("tale");
            let name = if dom == "tale" { "big-tale" } else { "big-real" };
            let big = world::big::sample(name)?;
            let home = std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into());
            let held = if dom == "tale" { format!("{home}/runs/absurd-rerank/tales-even-b.conllu") } else { format!("{home}/en/ud-mova/ewt-dev-h.conllu") };
            let base: Vec<_> = world::induce::cold_rules(dom);
            let t = std::time::Instant::now();
            for (d, _, removed, gh, p) in world::induce::explore_big(base, &big, dom, std::path::Path::new(&held), &[], 10)? {
                println!("{removed:>5} absurd removed, held {gh:+}, p {p:.3}  {d}");
            }
            eprintln!("explored in {:.0} s", t.elapsed().as_secs_f64());
        }
        "stepgame-contrast" => {
            // stepgame-contrast [train folder] [per k] [--test]
            let dir = std::path::PathBuf::from(args.get(1).map(String::as_str).unwrap_or("data/raw/stdata-stepgame"));
            world::stepgame::contrast(&dir, args.get(2).and_then(|x| x.parse().ok()).unwrap_or(500), args.iter().any(|a| a == "--test"))?;
        }
        "spartqa" => world::spartqa::eval(std::path::Path::new(&args[1]), args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0))?,
        "spartqa-contrast" => world::spartqa::contrast(std::path::Path::new(&args[1]), std::path::Path::new(&args[2]))?,
        "gaps" => {
            // gap journal: gaps <out.jsonl> <book.txt>...
            let books: Vec<std::path::PathBuf> = args[2..].iter().map(std::path::PathBuf::from).collect();
            world::babi::gaps(&books, std::path::Path::new(&args[1]))?;
        }
        "babi-probe" => world::babi::probe(&args[1..])?,
        "read-train" => {
            // training the snake on short questions of all books: read-train <book> <qa> [<book> <qa> …]
            let v: Vec<std::path::PathBuf> = args[1..].iter().map(std::path::PathBuf::from).collect();
            let pairs: Vec<(std::path::PathBuf, std::path::PathBuf)> = v.chunks(2).filter(|c| c.len() == 2).map(|c| (c[0].clone(), c[1].clone())).collect();
            world::read::train_short(&pairs)?;
        }
        "read-classify" => {
            let d = dir_at(&args, 1)?;
            let lim = args.iter().position(|a| a == "--limit").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(200);
            world::read::classify(&d, lim)?;
        }
        "read-short" => {
            // the snake answers short questions; check without the LLM
            let book = std::path::PathBuf::from(args.get(1).context("book folder")?);
            let d = dir_at(&args, 2)?;
            let show = args.iter().position(|a| a == "--show").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(0);
            world::read::eval_short(&book, &d, show)?;
        }
        "read-feel" => world::read::eval_feel(&home().join("raw/fairytaleqa-valid"), &ftqa_dir())?,
        "read-events" => {
            // event graph: traversal policy by question type — training on valid, measurement on test
            world::read::train_events(&home().join("raw/fairytaleqa-valid"), &ftqa_dir())?;
        }
        "read-graph" => {
            // training context-graph traversals on valid, measurement on test
            let valid = home().join("raw/fairytaleqa-valid");
            world::read::train_graph(&valid, &ftqa_dir())?;
        }
        "read-qa" => {
            // the snake answers FairytaleQA questions itself (without a world from the LLM): read-qa [--show N]
            let show = args.iter().position(|a| a == "--show").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(0);
            world::read::eval_ftqa(&ftqa_dir(), show)?;
        }
        "values" => {
            // values as predicates over the story world + level-1 gaps
            let text = std::fs::read_to_string(args.get(1).context("world command file")?)?;
            for j in world::values::judge(&text) {
                println!("{} «{}» — {}: {} (line {})", if j.violated { "✗ against" } else { "✓ in the spirit" }, j.name, j.agent, j.because, j.line);
            }
            let g = world::values::gaps(&text);
            if !g.is_empty() {
                println!("level-1 gaps ({}):", g.len());
                for x in g { println!("  · {x}"); }
            }
        }
        "quant-vmm" => {
            // quantitative layer: the LLM writes world scripts for dataset problems, the gate checks (MOVA_SUBSCRIPTION=1 — subscription)
            let set = args.get(1).context("dataset: svamp-train | gsm8k-train | …")?;
            let d = dir_at(&args, 2)?;
            let flag = |n: &str, dflt: f64| args.iter().position(|a| a == n).and_then(|i| args.get(i + 1)).and_then(|s| s.parse::<f64>().ok()).unwrap_or(dflt);
            world::quant::vmm_run(set, &d, flag("--limit", 1e9) as usize, flag("--batch", 10.0) as usize, flag("--par", 2.0) as usize, flag("--cap", 10.0))?;
        }
        "quant-stats" => world::quant::stats(&dir_at(&args, 1)?)?,
        "quant-check" => {
            // script from a file against a dataset problem: quant-check <dataset> <id> <file>
            let set = args.get(1).context("dataset")?;
            let id = args.get(2).context("id")?;
            let it = world::quant::load_set(set)?.into_iter().find(|x| &x.id == id).context("no such problem")?;
            let sc = std::fs::read_to_string(args.get(3).context("script file")?)?;
            let (a, e) = world::quant::check(&sc, &it.question, &it.gold);
            println!("answer {:?}; errors {}", a.map(|x| math::big::show(&x)), e.len());
            for x in e { println!("  {x}"); }
        }
        "v3-prompt" => {
            // v3 prompt of a part without a call: <name> <dir> [<part>]
            let name = args.get(1).context("missing testbed")?;
            let dir = dir()?;
            let t = poly_text(name, &dir)?;
            let ps = vmm::parts(&t, v3_part());
            let i: usize = args.get(3).map(|x| x.parse()).transpose()?.unwrap_or(1);
            let part = *ps.get(i - 1).context("no such part")?;
            eprintln!("parts: {ps:?}");
            let prev = if part.0 > 1 && dir.join(format!("fixed-{name}-p{}.txt", i - 1)).exists() {
                let mut all = String::new();
                for k in 1..i {
                    all.push_str(&std::fs::read_to_string(dir.join(format!("fixed-{name}-p{k}.txt")))?);
                    all.push('\n');
                }
                let (lines, _) = lang::parse_all(&all);
                Some(w::run_v3(t.clone(), &lines).world)
            } else {
                None
            };
            let p = format!("{}Commands:\n", vmm::body_v3(&t, part, prev.as_ref()));
            eprintln!("characters {}, call estimate ${:.2}", p.len(), vmm::estimate_call((part.1 - part.0 + 1) as usize, p.len()));
            print!("{p}");
        }
        "v3-estimate" => {
            // LLM v3 run cost estimate before the call: <dir> <name>…
            let dir = dir_at(&args, 1)?;
            let mut total = 0.0;
            for name in &args[2..] {
                let t = poly_text(name, &dir)?;
                let ps = vmm::parts(&t, v3_part());
                for (i, &(a, b)) in ps.iter().enumerate() {
                    let ctx: usize = t.sents.iter().take(a as usize - 1).map(|x| x.1.len() + 16).sum();
                    let body = vmm::body_v3(&t, (a, b), None).len() + ctx / 1 + 6000;
                    let e = vmm::estimate_call((b - a + 1) as usize, body);
                    total += e;
                    println!("{name} p{}: s{a}-s{b} ({} sentences), prompt ≈{} characters, estimate ${e:.2}", i + 1, b - a + 1, body);
                }
            }
            println!("total estimate ${total:.2} (plus format repairs ≈$0.3 each); already spent ${:.2}", vmm::spent(&dir)?);
        }
        "v3-vmm" => {
            let name = args.get(1).context("missing testbed")?;
            let dir = dir()?;
            let t = poly_text(name, &dir)?;
            let budget: f64 = std::env::var("WORLD_V3_BUDGET").ok().and_then(|x| x.parse().ok()).unwrap_or(40.0);
            let out = vmm::call_v3(&t, name, &dir, v3_part(), budget)?;
            let (lines, errs) = lang::parse_all(&out);
            println!("{name}: lines {}, format refusals {}; spent in total ${:.2}", lines.len(), errs.len(), vmm::spent(&dir)?);
        }
        "v3-build" => {
            let name = args.get(1).context("missing testbed")?;
            let dir = dir()?;
            let (r, errs, total) = load_poly(name, &dir)?;
            std::fs::write(dir.join(format!("story-{name}.md")), md::story_md(&r, &errs, total))?;
            let st = r.world.story().expect("narrative layer");
            println!(
                "{name}: sentences {}, lines {total}, accepted {}, format {}, gates {}, sentences without commands {}; events {}, frames {}, levels {}, claims {}, reveals {}, knowledge {}, warnings {}",
                r.world.text.sents.len(),
                r.accepted,
                errs.len(),
                r.rejects.len(),
                r.uncovered.len(),
                st.ev.len(),
                st.frames.len(),
                st.depth(),
                r.world.ents(world::types::Reg::K).count(),
                st.reveals.len(),
                st.knows.len(),
                st.warnings.len()
            );
            for (l, e) in r.rejects.iter().take(60) {
                println!("  line {}: {} — {e}", l.no, l.raw);
            }
        }
        "v3-query" => {
            let name = args.get(1).context("missing testbed")?;
            let q = args[3..].join(" ");
            let (r, _, _) = load_poly(name, &dir()?)?;
            println!("{}", world::qa3::answer(&r.world, &q).text);
        }
        "v3-eval" => {
            // answers to frozen questions: <dir> <questions.tsv> → answers-v3.md
            let dir = dir_at(&args, 1)?;
            let qf = args.get(2).context("missing questions file")?;
            let qs = world::qa3::read_questions(&std::fs::read_to_string(qf)?).map_err(anyhow::Error::msg)?;
            let mut polys: Vec<String> = qs.iter().map(|q| q.poly.clone()).collect();
            polys.dedup();
            let mut rep = String::from("# World v3: answers to frozen questions\n\n");
            let mut all: Vec<(String, String, world::qa::Verdict)> = Vec::new();
            for p in &polys {
                let (r, _, _) = load_poly(p, &dir)?;
                let mine: Vec<world::qa3::Q3> = qs.iter().filter(|q| &q.poly == p).cloned().collect();
                let (md, v) = world::qa3::answers_md(&r.world, &mine);
                rep.push_str(&format!("## {p}\n\n{md}\n"));
                all.extend(v.into_iter().map(|(k, x)| (p.clone(), k, x)));
            }
            rep.push_str("## Summary\n\n| testbed | type | questions | hit | miss | not in state |\n|---|---|---|---|---|---|\n");
            let mut keys: Vec<(String, String)> = all.iter().map(|(p, k, _)| (p.clone(), k.clone())).collect();
            keys.sort();
            keys.dedup();
            for (p, k) in keys.iter().chain([(String::from("all"), String::from("all"))].iter()) {
                let sel: Vec<&world::qa::Verdict> = all.iter().filter(|(pp, kk, _)| (p == "all" || pp == p) && (k == "all" || kk == k)).map(|x| &x.2).collect();
                let c = |v: world::qa::Verdict| sel.iter().filter(|x| ***x == v).count();
                rep.push_str(&format!("| {p} | {k} | {} | {} | {} | {} |\n", sel.len(), c(world::qa::Verdict::Hit), c(world::qa::Verdict::Miss), c(world::qa::Verdict::NotInState)));
            }
            std::fs::write(dir.join("answers-v3.md"), &rep)?;
            print!("{rep}");
        }
        "v3-cost" => {
            let dir = dir_at(&args, 1)?;
            let calls = prag::opus::read_calls(&dir.join("calls.jsonl"))?;
            let mut t = (0.0, 0u64, 0u64, 0u64, 0.0);
            for c in &calls {
                println!("{}: {} sentences, {:.0} s, output {}, cache write {}, cache read {}, ${:.3}", c.step, c.sentences, c.secs, c.output_tokens, c.cache_write_tokens, c.cache_read_tokens, c.cost_usd);
                t.0 += c.cost_usd;
                t.1 += c.output_tokens;
                t.2 += c.cache_write_tokens;
                t.3 += c.cache_read_tokens;
                t.4 += c.secs;
            }
            println!("total: calls {}, output {}, cache write {}, cache read {}, {:.0} s, ${:.2}", calls.len(), t.1, t.2, t.3, t.4, t.0);
        }
        "poly-prep" => {
            // v3 testbed: <Gutenberg book .txt> <title line> <end line|-> <output .tsv>
            let raw = args.get(1).context("missing book")?;
            let start = args.get(2).context("missing title")?;
            let end = args.get(3).context("missing end")?;
            let out = args.get(4).context("missing output")?;
            println!("{}", world::poly::prep(Path::new(raw), start, if end == "-" { None } else { Some(end.as_str()) }, Path::new(out))?);
        }
        "ftqa-import" => {
            let csv = args.get(1).context("missing CSV")?;
            let out = args.get(2).context("missing folder")?;
            print!("{}", world::ftqa::import(Path::new(csv), Path::new(out))?);
        }
        "ftqa-prompt" => {
            let name = args.get(1).context("missing tale")?;
            let t = ftqa_text(name)?;
            let ps = vmm::parts(&t, ftqa_part());
            let i: usize = args.get(2).map(|x| x.parse()).transpose()?.unwrap_or(1);
            let part = *ps.get(i - 1).context("no such part")?;
            eprintln!("parts: {ps:?}");
            print!("{}Commands:\n", vmm::body_v2(&t, part, None));
        }
        "ftqa-vmm" => {
            let name = args.get(1).context("missing tale")?;
            let dir = dir()?;
            let t = ftqa_text(name)?;
            let out = vmm::call_v2(&t, name, &dir, ftqa_part())?;
            let (lines, errs) = lang::parse_all(&out);
            println!("{name}: lines {}, format refusals {}", lines.len(), errs.len());
        }
        "ftqa-build" => {
            let name = args.get(1).context("missing tale")?;
            let dir = dir()?;
            let (r, errs, total) = load_ftqa(name, &dir)?;
            std::fs::write(dir.join(format!("world-{name}.md")), md::world_md(&r, &errs, total))?;
            println!(
                "{name}: sentences {}, lines {total}, accepted {}, format {}, gates {}, sentences without commands {}",
                r.world.text.sents.len(),
                r.accepted,
                errs.len(),
                r.rejects.len(),
                r.uncovered.len()
            );
            for (l, e) in r.rejects.iter().take(40) {
                println!("  line {}: {} — {e}", l.no, l.raw);
            }
        }
        "ftqa-parse" => {
            // question parsing (development on the validation split): lines `type<TAB>e/i<TAB>question`
            let f = args.get(1).context("missing questions file")?;
            for l in std::fs::read_to_string(f)?.lines() {
                let q = l.rsplit('\t').next().unwrap_or(l);
                let fr = world::ans::parse_question(q);
                if args.iter().any(|a| a == "--frame") {
                    println!("{q}\n    {}", world::ans::frame_str(&fr));
                } else {
                    println!("{:<9} {}\t{q}", fr.intent.name(), l.split('\t').next().unwrap_or(""));
                }
            }
        }
        "reader-parse" => {
            // reader v2: question parsing with a UD tree (development on the validation split): `type<TAB>e/i<TAB>question`
            let f = args.get(1).context("missing questions file")?;
            world::tree::annotator()?;
            for l in std::fs::read_to_string(f)?.lines() {
                let q = l.rsplit('\t').next().unwrap_or(l);
                let fr = world::qframe::parse(q);
                println!("{}\t{q}\n    {}", l.split('\t').next().unwrap_or(""), world::qframe::show(&fr));
            }
        }
        "reader-eval" => {
            // reader v2: all tales from <worlds folder>/tales.txt → <output folder>: answers, ROUGE-L, report;
            // --dev <tsv> — only these questions (development); --control shuffle|noindex — negative controls;
            // --sample k --seed s --exclude <tsv>… — a sample for manual checking
            let wdir = PathBuf::from(args.get(1).context("missing worlds folder")?);
            let out = PathBuf::from(args.get(2).context("missing output folder")?);
            std::fs::create_dir_all(&out)?;
            world::tree::annotator()?;
            let opt = |k: &str| args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).cloned();
            let pairs = |f: &str| -> Result<std::collections::BTreeSet<(String, usize)>> {
                let mut s = std::collections::BTreeSet::new();
                for l in std::fs::read_to_string(f).with_context(|| f.to_string())?.lines().filter(|l| !l.starts_with('#')) {
                    let c: Vec<&str> = l.split('\t').collect();
                    if c.len() >= 2
                        && let Ok(n) = c[1].trim().parse::<usize>()
                    {
                        s.insert((c[0].to_string(), n));
                    }
                }
                Ok(s)
            };
            let dev = opt("--dev").map(|f| pairs(&f)).transpose()?;
            // --skip <tsv>: report without these questions (after freezing — the table on non-dev questions)
            let skip = opt("--skip").map(|f| pairs(&f)).transpose()?.unwrap_or_default();
            let control = opt("--control");
            let names: Vec<String> = std::fs::read_to_string(wdir.join("tales.txt"))?.lines().filter(|l| !l.trim().is_empty()).map(str::to_string).collect();
            let mut tales = Vec::new();
            for name in &names {
                let (r, _, _) = load_ftqa(name, &wdir)?;
                let ix = world::index::Index::build(&r.world);
                let tr = world::reader::trees(&r.world.text)?;
                let qs = world::ftqa::read_qa(&ftqa_dir().join("questions").join(format!("{name}.tsv")))?;
                tales.push((name.clone(), r.world, ix, tr, qs));
            }
            let empty = world::index::Index::empty();
            let n = tales.len();
            let mut rows = Vec::new();
            for i in 0..n {
                let (name, own, ix, tr, qs) = &tales[i];
                let qs: Vec<world::ftqa::Qa> = qs.iter().filter(|q| dev.as_ref().is_none_or(|d| d.contains(&(name.clone(), q.n))) && !skip.contains(&(name.clone(), q.n))).cloned().collect();
                let (fw, fix) = match control.as_deref() {
                    Some("shuffle") => (&tales[(i + 1) % n].1, &tales[(i + 1) % n].2),
                    Some("noindex") => (own, &empty),
                    None => (own, ix),
                    Some(x) => bail!("unknown control {x}"),
                };
                let ans = world::reader::read_all(fw, fix, own, tr, &qs);
                rows.extend(world::rreport::rows(own, &qs, ans));
            }
            let tag = control.as_deref().map(|c| format!("-{c}")).unwrap_or_default();
            let tag = if dev.is_some() { format!("{tag}-dev") } else { tag };
            let tag = if skip.is_empty() { tag } else { format!("{tag}-clean") };
            std::fs::write(out.join(format!("answers{tag}.tsv")), world::rreport::tsv(&rows))?;
            if dev.is_some() {
                std::fs::write(out.join(format!("dev{tag}.txt")), world::rreport::dev(&rows))?;
            }
            let mut rep = format!("# Reader v2 on FairytaleQA{} ({} tales, {} questions)\n\n", control.as_deref().map(|c| format!(", control «{c}»")).unwrap_or_default(), names.len(), rows.len());
            rep.push_str(&world::rreport::report(&rows));
            if let Some(k) = opt("--sample").and_then(|x| x.parse::<usize>().ok()) {
                let salt = opt("--seed").unwrap_or_default();
                let mut ex = std::collections::BTreeSet::new();
                for (i, a) in args.iter().enumerate() {
                    if a == "--exclude"
                        && let Some(f) = args.get(i + 1)
                    {
                        ex.extend(pairs(f)?);
                    }
                }
                let sample = world::rreport::sample(&rows, k, &salt, &ex);
                let vpath = out.join("manual-verdicts.tsv");
                if vpath.exists() {
                    let v = world::rreport::read_verdicts(&std::fs::read_to_string(&vpath)?);
                    rep.push_str("\n## Manual check (fresh sample)\n\n");
                    rep.push_str(&world::rreport::manual(&sample, &v));
                } else {
                    let mut st = String::from("# story\tn\tv2\tv1\tbaseline\tstate\tcomment\n");
                    for x in &sample {
                        st.push_str(&format!("{}\t{}\n", x.q.story, x.q.n));
                    }
                    std::fs::write(out.join("manual-sample.tsv"), st)?;
                    println!("sample: {} questions ({} excluded)", sample.len(), ex.len());
                }
            }
            std::fs::write(out.join(format!("report{tag}.md")), &rep)?;
            print!("{rep}");
        }
        "reader-ask" => {
            // reader v2, development: one tale question (allowed questions only): frame, facts, anchor sentence tree
            let name = args.get(1).context("missing tale")?;
            let wdir = PathBuf::from(args.get(2).context("missing worlds folder")?);
            let n: usize = args.get(3).context("missing question number")?.parse()?;
            let (r, _, _) = load_ftqa(name, &wdir)?;
            let ix = world::index::Index::build(&r.world);
            let tr = world::reader::trees(&r.world.text)?;
            let qs = world::ftqa::read_qa(&ftqa_dir().join("questions").join(format!("{name}.tsv")))?;
            let q = qs.iter().find(|q| q.n == n).context("no such question")?;
            let a = world::reader::read(&world::reader::Tale { fw: &r.world, ix: &ix, own: &r.world, tr: &tr }, q);
            println!("{}\nG: {} | {}\n{a:#?}", q.question, q.a1, q.a2);
            if let Some(s) = a.sent {
                println!("^s{s}: {}", tr[s as usize - 1].show());
            }
            if args.iter().any(|x| x == "--facts") {
                for f in &ix.facts {
                    println!("{} | {}", f.label(), f.fields.iter().map(|x| format!("{}={}", x.slot.name(), x.text)).collect::<Vec<_>>().join("; "));
                }
            }
        }
        "reader-span-dev" => {
            // reader v2: sentence spans on the validation split (without a world): oracle sentence (highest ROUGE with
            // the reference among sections) and the baseline sentence → span by question type; span ROUGE against the sentence
            let vdir = PathBuf::from(args.get(1).context("missing validation folder")?);
            let show = args.iter().position(|a| a == "--show").and_then(|i| args.get(i + 1)).cloned();
            print!("{}", world::reader::span_dev(&vdir, show.as_deref())?);
        }
        "ftqa-eval" => {
            // all tales from <dir>/tales.txt: answers, ROUGE-L, report, sample for manual checking
            let dir = PathBuf::from(args.get(1).context("missing folder")?);
            let names: Vec<String> = std::fs::read_to_string(dir.join("tales.txt"))?.lines().filter(|l| !l.trim().is_empty()).map(str::to_string).collect();
            let mut rows = Vec::new();
            for name in &names {
                let (r, _, _) = load_ftqa(name, &dir)?;
                let qs = world::ftqa::read_qa(&ftqa_dir().join("questions").join(format!("{name}.tsv")))?;
                let rs = world::report::eval_tale(&r.world, &qs);
                std::fs::write(dir.join(format!("answers-{name}.md")), world::report::tale_md(name, &rs))?;
                rows.extend(rs);
            }
            std::fs::write(dir.join("answers-all.tsv"), world::report::rows_tsv(&rows))?;
            let k: usize = args.iter().position(|a| a == "--sample").and_then(|i| args.get(i + 1)).and_then(|x| x.parse().ok()).unwrap_or(10);
            let salt = args.iter().position(|a| a == "--seed").and_then(|i| args.get(i + 1)).cloned().unwrap_or_default();
            let mut exclude = std::collections::BTreeSet::new();
            if let Some(f) = args.iter().position(|a| a == "--exclude").and_then(|i| args.get(i + 1)) {
                for l in std::fs::read_to_string(f)?.lines().filter(|l| !l.starts_with('#')) {
                    let c: Vec<&str> = l.split('\t').collect();
                    if c.len() >= 2 && let Ok(n) = c[1].parse::<usize>() {
                        exclude.insert((c[0].to_string(), n));
                    }
                }
            }
            let sample = world::report::sample(&rows, k, &salt, &exclude);
            let mut st = String::from("# story\tn\tmmm(1/0.5/0)\tbaseline(1/0.5/0)\tcomment\n");
            for r in &sample {
                st.push_str(&format!("{}\t{}\t\t\t\n", r.q.story, r.q.n));
            }
            std::fs::write(dir.join("manual-sample.tsv"), st)?;
            let mut rep = format!("# FairytaleQA: SLM measured from state ({} tales, {} questions)\n\n", names.len(), rows.len());
            rep.push_str(&world::report::report(&rows));
            let vpath = dir.join("manual-verdicts.tsv");
            if vpath.exists() {
                let v = world::report::read_verdicts(&std::fs::read_to_string(&vpath)?);
                rep.push_str("\n## Manual check on a sample\n\n");
                rep.push_str(&world::report::manual_md(&sample, &v));
            }
            std::fs::write(dir.join("report.md"), &rep)?;
            print!("{rep}");
        }
        "ftqa-query" => {
            let name = args.get(1).context("missing tale")?;
            let q = args[3..].join(" ");
            let (r, _, _) = load_ftqa(name, &dir()?)?;
            println!("{}", qa::answer(&r.world, &q).text);
        }
        "prompt" => {
            let t = vmm::read_text(&db(), doc()?, &tmp())?;
            print!("{}", vmm::prompt(&t));
        }
        "vmm" => {
            let t = vmm::read_text(&db(), doc()?, &tmp())?;
            let out = vmm::call(&t, &dir()?)?;
            let (lines, errs) = lang::parse_all(&out);
            println!("doc {}: lines {}, format refusals {}", t.doc, lines.len(), errs.len());
        }
        "build" => {
            let (doc, dir) = (doc()?, dir()?);
            let (r, errs, total) = load(doc, &dir)?;
            std::fs::write(dir.join(format!("world-{doc}.md")), md::world_md(&r, &errs, total))?;
            println!(
                "doc {doc} «{}»: lines {total}, accepted {}, format {}, gates {}, sentences without commands {}",
                r.world.text.title,
                r.accepted,
                errs.len(),
                r.rejects.len(),
                r.uncovered.len()
            );
            for (l, e) in &r.rejects {
                println!("  line {}: {} — {e}", l.no, l.raw);
            }
            if let Some(i) = args.iter().position(|a| a == "--questions") {
                let f = args.get(i + 1).context("--questions: missing file")?;
                let text = std::fs::read_to_string(f).with_context(|| f.clone())?;
                let qs: Vec<qa::Question> = qa::read_questions(&text).map_err(anyhow::Error::msg)?.into_iter().filter(|q| q.doc == doc).collect();
                if qs.is_empty() {
                    bail!("{f}: no questions for doc {doc}");
                }
                let (a, n) = qa::answers_md(&r.world, &qs);
                std::fs::write(dir.join(format!("answers-{doc}.md")), a)?;
                println!("  answers: hit {}, miss {}, not in state {}; honesty control {}/{}", n.hit, n.miss, n.none, n.ctrl_ok, n.ctrl);
            }
        }
        "query" => {
            let (doc, dir) = (doc()?, dir()?);
            let q = args[3..].join(" ");
            if q.is_empty() {
                return usage();
            }
            let (r, _, _) = load(doc, &dir)?;
            println!("{}", qa::answer(&r.world, &q).text);
        }
        _ => return usage(),
    }
    Ok(())
}
