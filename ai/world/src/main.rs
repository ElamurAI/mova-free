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
    let args: Vec<String> = std::env::args().skip(1).collect();
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
        "babi-learn" => world::babi::learn_motivations(std::path::Path::new(&args[1]))?,
        "babi-contrast" => world::babi::contrast(std::path::Path::new(&args[1]), std::path::Path::new(&args[2]))?,
        "stepgame" => {
            // stepgame [folder] [epochs] [errors to show]
            let dir = std::path::PathBuf::from(args.get(1).map(String::as_str).unwrap_or("data/raw/stdata-stepgame"));
            world::stepgame::eval(&dir, args.get(2).and_then(|x| x.parse().ok()).unwrap_or(10), args.get(3).and_then(|x| x.parse().ok()).unwrap_or(0))?;
        }
        "spartqa" => world::spartqa::eval(std::path::Path::new(&args[1]), args.get(2).and_then(|x| x.parse().ok()).unwrap_or(0))?,
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
            let mut rep = format!("# FairytaleQA: MMM measured from state ({} tales, {} questions)\n\n", names.len(), rows.len());
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
