//! coder — SLM (small language model) learns to code in the mlab language in a "student — architect" loop (README).

mod arch2;
mod arch3;
mod books;
mod repr;
mod reprexp;
mod control;
mod judge;
mod motiv;
mod mutate;
mod lemma;
mod memory;
mod v0;
mod v1;
mod sandbox;
mod tasks;
mod util;
mod vmm;

use std::sync::Mutex;

use anyhow::{Context, Result, bail};

use util::{crate_dir, run_dir};

/// API spending cap for the whole wave ($40).
pub const CAP_USD: f64 = 40.0;

fn vmm() -> Result<vmm::Vmm> {
    let d = run_dir();
    std::fs::create_dir_all(&d)?;
    vmm::Vmm::new(d.join("cwd"), d.join("calls.jsonl"), CAP_USD)
}

fn prompt(name: &str) -> Result<String> {
    let p = crate_dir().join("prompts").join(name);
    std::fs::read_to_string(&p).with_context(|| p.display().to_string())
}

/// Calls in parallel, at most `par` at a time; each result goes to a file (a retry does not pay twice).
fn run_jobs(jobs: Vec<(String, String, usize)>, par: usize, reserve: f64) -> Result<()> {
    let v = vmm()?;
    let queue = Mutex::new(jobs);
    let errors = Mutex::new(Vec::<String>::new());
    std::thread::scope(|s| {
        for _ in 0..par {
            s.spawn(|| {
                loop {
                    let job = queue.lock().unwrap().pop();
                    let Some((out, text, n)) = job else { break };
                    let step = std::path::Path::new(&out).file_stem().unwrap().to_string_lossy().to_string();
                    match v.ask(&step, 0, n, &text, reserve) {
                        Ok((r, c)) => {
                            let _ = std::fs::write(&out, r);
                            eprintln!("{step}: ${:.3} {:.0} s, out {} tok.", c.cost_usd, c.secs, c.output_tokens);
                        }
                        Err(e) => {
                            eprintln!("{step}: {e}");
                            errors.lock().unwrap().push(format!("{step}: {e}"));
                            // fail-fast: cancel the rest of the queue
                            queue.lock().unwrap().clear();
                        }
                    }
                }
            });
        }
    });
    eprintln!("spent in total ${:.2}", vmm()?.spent());
    let e = errors.into_inner().unwrap();
    if !e.is_empty() {
        bail!("errors: {}", e.join("; "));
    }
    Ok(())
}

fn cmd_gen() -> Result<()> {
    let dir = tasks::gen_dir();
    std::fs::create_dir_all(&dir)?;
    let head = prompt("gen-head.txt")? + &prompt("mlab-lang.txt")? + "\n\n";
    let mut jobs = Vec::new();
    for lv in ["A", "B", "C", "D", "E"] {
        let out = dir.join(format!("opus-{lv}.txt"));
        if !out.exists() {
            jobs.push((out.display().to_string(), head.clone() + &prompt(&format!("gen-{lv}.txt"))?, 22));
        }
    }
    for b in books::BOOKS {
        let out = dir.join(format!("book-{}.txt", b.key));
        if out.exists() {
            continue;
        }
        let all = books::excerpts(b, 900)?;
        let picked = books::pick(b, &all, 40);
        util::write_jsonl(&dir.join(format!("excerpts-{}.jsonl", b.key)), &picked)?;
        let mut t = head.clone()
            + &prompt("gen-book.txt")?
                .replace("{BOOK}", b.title)
                .replace("{LICENSE}", b.license)
                .replace("{N}", "20")
                .replace("{PREFIX}", &format!("bk-{}-", b.key))
                .replace("{SHORT}", b.title);
        for e in &picked {
            t.push_str(&format!("\n[{} | pdf p. {}] {}\n", e.label, e.page, e.text));
        }
        jobs.push((out.display().to_string(), t, 20));
    }
    eprintln!("calls: {}", jobs.len());
    run_jobs(jobs, 2, 1.5)
}

/// Parsing generation responses, checking references by real execution in the sandbox, splitting, writing.
fn cmd_build_tasks() -> Result<()> {
    let dir = tasks::gen_dir();
    let mut all: Vec<tasks::Task> = Vec::new();
    for lv in ["A", "B", "C", "D", "E"] {
        let p = dir.join(format!("opus-{lv}.txt"));
        if p.exists() {
            all.extend(tasks::parse_blocks(&std::fs::read_to_string(&p)?, "opus", tasks::LIC_OPUS));
        }
    }
    for (key, lic) in [("erickson", tasks::LIC_ERICKSON), ("ods", tasks::LIC_ODS), ("sicp", tasks::LIC_SICP)] {
        let p = dir.join(format!("book-{key}.txt"));
        if p.exists() {
            all.extend(tasks::parse_blocks(&std::fs::read_to_string(&p)?, key, lic));
        }
    }
    // Erickson: the automatic labels "Chapter k, Exercise N" are unreliable (algorithm steps from the chapter text also got into
    // the extract, and the chapter number is shifted) — keep only the reliable PDF page.
    for t in all.iter_mut().filter(|t| t.source == "erickson") {
        let page = t.source_ref.rsplit("pdf p.").next().unwrap_or("").trim().to_string();
        t.source_ref = format!("pdf p. {page}; auto-extracted passage (end-of-chapter exercise or in-chapter algorithm; see the description)");
    }
    let mut ok = Vec::new();
    let mut bad = Vec::new();
    for t in all {
        let ex = sandbox::run(&t.solution)?;
        let pass = matches!(&ex, sandbox::Exec::Ran(o) if sandbox::same_output(o, &t.expected));
        if pass {
            ok.push(t);
        } else {
            bad.push(serde_json::json!({"id": t.id, "exec": ex, "expected": t.expected}));
        }
    }
    util::write_jsonl(&dir.join("rejected.jsonl"), &bad)?;
    let n_opus = ok.iter().filter(|t| t.source == "opus").count();
    let n_book = ok.len() - n_opus;
    eprintln!("reference matched: {} (opus {n_opus}, books {n_book}); rejected {}", ok.len(), bad.len());
    // ~100 Opus + ~50 books: take at most 100 and 50 (deterministically — the first by id)
    ok.sort_by(|a, b| a.id.cmp(&b.id));
    let mut opus: Vec<_> = ok.iter().filter(|t| t.source == "opus").cloned().collect();
    let mut book: Vec<_> = ok.iter().filter(|t| t.source != "opus").cloned().collect();
    let mut mix = util::Mix(20260926);
    mix.shuffle(&mut opus);
    mix.shuffle(&mut book);
    opus.truncate(100);
    book.truncate(50);
    let mut sel: Vec<_> = opus.into_iter().chain(book).collect();
    sel.sort_by(|a, b| a.id.cmp(&b.id));
    let n_held = sel.len() / 3;
    tasks::split(&mut sel, n_held);
    util::write_jsonl(&tasks::tasks_path(), &sel)?;
    let held = sel.iter().filter(|t| t.split == "held").count();
    eprintln!("wrote {} tasks: training {}, held out {held}; sha256 {}", sel.len(), sel.len() - held, tasks::file_sha(&tasks::tasks_path())?);
    Ok(())
}

/// Annotation of the solution steps of math v2 (goal, motive, move, check, difference → operator) — Opus, batches of 30.
fn cmd_annotate() -> Result<()> {
    let sols = motiv::math_memory()?;
    let dir = motiv::ann_dir();
    std::fs::create_dir_all(&dir)?;
    let head = prompt("annotate-math.txt")?;
    let mut jobs = Vec::new();
    for (b, chunk) in sols.chunks(30).enumerate() {
        let out = dir.join(format!("batch-{b:02}.txt"));
        if out.exists() {
            continue;
        }
        let mut t = head.clone();
        for s in chunk {
            t.push_str(&format!("\n[{}] {}\nplan (answer = {}):\n{}", s.id, s.question, s.plan["answer"], motiv::plan_brief(&s.plan)));
        }
        jobs.push((out.display().to_string(), t, chunk.len()));
    }
    eprintln!("calls: {}", jobs.len());
    run_jobs(jobs, 2, 1.5)
}

fn cmd_ann_build() -> Result<()> {
    let sols: std::collections::HashMap<String, motiv::MathSol> = motiv::math_memory()?.into_iter().map(|s| (s.id.clone(), s)).collect();
    let dir = motiv::ann_dir();
    let mut all = Vec::new();
    let mut bad = Vec::new();
    let mut names: Vec<_> = std::fs::read_dir(&dir)?.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "txt")).collect();
    names.sort();
    for p in names {
        let (ok, b) = motiv::parse_ann(&std::fs::read_to_string(&p)?, &sols);
        all.extend(ok);
        bad.extend(b);
    }
    util::write_jsonl(&dir.join("annotations.jsonl"), &all)?;
    let m = motiv::Model::learn(&all);
    std::fs::write(dir.join("model.json"), serde_json::to_string(&m)?)?;
    eprintln!("annotations {} (rejected {}: {:?}); moves {}", all.len(), bad.len(), bad.iter().take(5).collect::<Vec<_>>(), m.n_moves);
    eprintln!("move choice on held-out math: perceptron {}/{} , table only {}/{}", m.eval.0, m.eval.2, m.eval.1, m.eval.2);
    for (d, row) in &m.table {
        let mut r: Vec<_> = row.iter().collect();
        r.sort_by(|a, b| b.1.cmp(a.1));
        eprintln!("  {d:<11} → {}", r.iter().take(4).map(|(k, c)| format!("{k} {c}")).collect::<Vec<_>>().join(", "));
    }
    Ok(())
}

fn round_dir(r: usize) -> std::path::PathBuf {
    run_dir().join(format!("rounds/r{r}"))
}

fn cmd_memory() -> Result<()> {
    let (items, rep) = memory::build()?;
    for (src, n, k) in &rep {
        eprintln!("{src:<10} scenarios {n:>4}, into memory {k:>4}");
    }
    util::write_jsonl(&run_dir().join("memory.jsonl"), &items)?;
    eprintln!("memory: {} verified programs", items.len());
    Ok(())
}

fn load_memory() -> Result<memory::Bm25> {
    let items: Vec<memory::Item> = util::read_jsonl(&run_dir().join("memory.jsonl"))?;
    if items.is_empty() {
        bail!("no memory — run coder memory");
    }
    Ok(memory::Bm25::new(items))
}

fn load_model() -> Result<motiv::Model> {
    let p = motiv::ann_dir().join("model.json");
    Ok(serde_json::from_str(&std::fs::read_to_string(&p).with_context(|| p.display().to_string())?)?)
}

/// Solve all tasks of a round. The SLM sees only `Public` (without the reference).
fn cmd_solve(r: usize, arch: &str, flags: &[String]) -> Result<()> {
    let tasks = tasks::load()?;
    let bm = load_memory()?;
    let mut sols = Vec::new();
    match arch {
        "v0" => {
            for t in &tasks {
                sols.push(v0::solve(&t.public(), &bm));
            }
        }
        _ => {
            let model = load_model()?;
            let fl = v1::Flags { names: flags.to_vec() };
            lemma::LITERAL_REPAIR.store(fl.on("b2:literal-repair"), std::sync::atomic::Ordering::Relaxed);
            for t in &tasks {
                let mut s = v1::solve(&t.public(), &bm, &model, &fl);
                s.arch = if flags.is_empty() { arch.to_string() } else { format!("{arch}+{}", flags.join("+")) };
                sols.push(s);
            }
        }
    }
    let d = round_dir(r);
    std::fs::create_dir_all(&d)?;
    util::write_jsonl(&d.join("solutions.jsonl"), &sols)?;
    let mut used: std::collections::BTreeMap<String, usize> = Default::default();
    for s in &sols {
        for u in &s.used {
            *used.entry(u.clone()).or_default() += 1;
        }
    }
    eprintln!("round {r}: {} solutions ({}), components: {:?}; stepwise path present in {}", sols.len(), arch, used, sols.iter().filter(|s| s.stepwise.is_some()).count());
    Ok(())
}

fn load_solutions(r: usize) -> Result<Vec<v0::Solution>> {
    util::read_jsonl(&round_dir(r).join("solutions.jsonl"))
}

/// Blind judge: items of several rounds shuffled together, batches of 15, 2 in parallel; cache keyed by
/// (task, program, explanation).
fn cmd_judge(rounds: &[usize]) -> Result<()> {
    let tasks = tasks::load()?;
    let by_id: std::collections::HashMap<&str, &tasks::Task> = tasks.iter().map(|t| (t.id.as_str(), t)).collect();
    let all: Vec<(usize, Vec<v0::Solution>)> = rounds.iter().map(|&r| Ok((r, load_solutions(r)?))).collect::<Result<_>>()?;
    let items: Vec<(&tasks::Task, &v0::Solution)> = all.iter().flat_map(|(_, ss)| ss.iter().map(|s| (by_id[s.task.as_str()], s))).collect();
    let cache = judge::load_cache()?;
    let head = prompt("judge.txt")? + &prompt("mlab-lang.txt")? + "\n\nITEMS:\n";
    // seed — from the content hash, not from round numbers
    let seed = u64::from_str_radix(&util::sha256_hex(items.iter().map(|(_, s)| s.program.as_str()).collect::<String>().as_bytes())[..12], 16)?;
    let batches = judge::batches(&head, &items, &cache, 15, seed);
    eprintln!("to the judge: {} batches ({} new items; the rest from cache)", batches.len(), batches.iter().map(|b| b.0.len()).sum::<usize>());
    let dir = run_dir().join("judge-raw");
    std::fs::create_dir_all(&dir)?;
    let mut jobs = Vec::new();
    let mut keymap: Vec<(std::path::PathBuf, Vec<String>)> = Vec::new();
    for (keys, text) in batches {
        let name = util::sha256_hex(text.as_bytes())[..16].to_string();
        let out = dir.join(format!("{name}.txt"));
        keymap.push((out.clone(), keys.clone()));
        if !out.exists() {
            jobs.push((out.display().to_string(), text, keys.len()));
        }
    }
    let res = if jobs.is_empty() { Ok(()) } else { run_jobs(jobs, 2, 1.0) };
    let mut missing = 0;
    for (out, keys) in keymap {
        let vs = judge::parse(&std::fs::read_to_string(&out).unwrap_or_default());
        for h in keys {
            match vs.iter().find(|v| v.k == h[..10]) {
                Some(v) => util::append_jsonl(&judge::cache_path(), &judge::Cached { h: h.clone(), v: v.clone() })?,
                None => missing += 1,
            }
        }
    }
    let cache = judge::load_cache()?;
    for (r, sols) in &all {
        let mut rows = Vec::new();
        for s in sols {
            let h = judge::item_hash(&s.task, s);
            if let Some(v) = cache.get(&h) {
                let t = by_id[s.task.as_str()];
                rows.push(serde_json::json!({"task": s.task, "arch": s.arch, "split": t.split, "skill": t.skill, "source": t.source, "v": v, "used": s.used}));
            }
        }
        util::write_jsonl(&round_dir(*r).join("verdicts.jsonl"), &rows)?;
        eprintln!("round {r}: verdicts {} of {}", rows.len(), sols.len());
    }
    eprintln!("without verdict: {missing}");
    res
}

fn mean(v: &[f64]) -> f64 {
    if v.is_empty() { f64::NAN } else { v.iter().sum::<f64>() / v.len() as f64 }
}

fn load_verdicts(r: usize) -> Result<Vec<serde_json::Value>> {
    util::read_jsonl(&round_dir(r).join("verdicts.jsonl"))
}

fn cmd_report() -> Result<()> {
    println!("| round | architecture | split | n | correctness | completeness | style | explanation | 'pass' (judge prediction) |");
    println!("|---|---|---|---|---|---|---|---|---|");
    for r in 0..8 {
        let vs = load_verdicts(r)?;
        if vs.is_empty() {
            continue;
        }
        for split in ["train", "held"] {
            let sel: Vec<&serde_json::Value> = vs.iter().filter(|x| x["split"] == split).collect();
            let f = |k: &str| mean(&sel.iter().map(|x| x["v"][k].as_f64().unwrap_or(0.0)).collect::<Vec<_>>());
            let pass = sel.iter().filter(|x| x["v"]["pass"] == true).count();
            println!("| r{r} | {} | {split} | {} | {:.2} | {:.2} | {:.2} | {:.2} | {pass}/{} |", sel.first().map(|x| x["arch"].as_str().unwrap_or("")).unwrap_or(""), sel.len(), f("correct"), f("complete"), f("style"), f("explain"), sel.len());
        }
    }
    Ok(())
}

fn cmd_show(r: usize, id: &str) -> Result<()> {
    let s = load_solutions(r)?.into_iter().find(|s| s.task == id).context("not found")?;
    println!("{}\n--- explanation ---\n{}", s.program, s.explanation);
    if let Some(st) = &s.stepwise {
        println!("--- stepwise path ---\n{st}");
    }
    Ok(())
}

/// Architect: sees only the training part — scores and examples; held-out tasks are hidden.
fn cmd_architect(r: usize) -> Result<()> {
    let tasks = tasks::load()?;
    let by_id: std::collections::HashMap<&str, &tasks::Task> = tasks.iter().map(|t| (t.id.as_str(), t)).collect();
    let sols: std::collections::HashMap<String, v0::Solution> = load_solutions(r)?.into_iter().map(|s| (s.task.clone(), s)).collect();
    let vs: Vec<serde_json::Value> = load_verdicts(r)?.into_iter().filter(|v| v["split"] == "train").collect();
    let arch_doc = std::fs::read_to_string(crate_dir().join("prompts").join(format!("arch-r{r}.md")))?;
    let mut text = prompt("architect.txt")? + &arch_doc + "\n\nTRAINING SCORES BY SKILL (judge, 1-5; pass = judge predicts exact output):\n";
    let mut by: std::collections::BTreeMap<String, Vec<&serde_json::Value>> = Default::default();
    for v in &vs {
        by.entry(v["skill"].as_str().unwrap_or("").to_string()).or_default().push(v);
    }
    for (sk, rows) in &by {
        let c = mean(&rows.iter().map(|x| x["v"]["correct"].as_f64().unwrap_or(0.0)).collect::<Vec<_>>());
        let p = rows.iter().filter(|x| x["v"]["pass"] == true).count();
        text.push_str(&format!("- {sk}: n={} correct={c:.2} pass={p}\n", rows.len()));
    }
    let mut sorted: Vec<&serde_json::Value> = vs.iter().collect();
    sorted.sort_by_key(|x| (x["v"]["correct"].as_u64().unwrap_or(0), x["task"].as_str().unwrap_or("").to_string()));
    let mut per_skill: std::collections::HashMap<String, usize> = Default::default();
    let mut picked = Vec::new();
    for v in &sorted {
        let sk = v["skill"].as_str().unwrap_or("").to_string();
        let c = per_skill.entry(sk).or_default();
        if *c < 2 && picked.len() < 20 {
            *c += 1;
            picked.push(*v);
        }
    }
    for v in sorted.iter().rev().take(4) {
        picked.push(*v);
    }
    text.push_str("\nTRAINING EXAMPLES (task, SLM program, SLM explanation, judge verdict):\n");
    for v in picked {
        let id = v["task"].as_str().unwrap_or("");
        let s = &sols[id];
        let t = by_id[id];
        let ex: String = s.explanation.chars().take(700).collect();
        let pr: String = s.program.chars().take(1400).collect();
        text.push_str(&format!("\n=== {id} (skill {})\nTASK: {}\nPROGRAM:\n{pr}\nEXPLANATION: {ex}\nVERDICT: {}\n", t.skill, t.description, v["v"]));
    }
    let out = round_dir(r).join("architect.md");
    if out.exists() {
        eprintln!("already exists: {}", out.display());
        return Ok(());
    }
    std::fs::write(round_dir(r).join("architect-prompt.txt"), &text)?;
    run_jobs(vec![(out.display().to_string(), text, 1)], 1, 1.5)?;
    println!("{}", std::fs::read_to_string(&out)?);
    Ok(())
}

fn cmd_control() -> Result<()> {
    let tasks = tasks::load()?;
    for (name, k, n) in control::negative_controls(&tasks)? {
        println!("control: {name}: {k}/{n}");
    }
    let mut rounds = Vec::new();
    for r in 0..8 {
        let s = load_solutions(r)?;
        if s.is_empty() {
            continue;
        }
        rounds.push((r, s, load_verdicts(r)?));
    }
    let rows = control::run_control(&tasks, &rounds)?;
    println!("{}", control::summary(&rows));
    Ok(())
}

/// Two code representations: example search and move choice — text / AST / both, valid and invalid code.
fn cmd_repr_eval() -> Result<()> {
    let items: Vec<memory::Item> = util::read_jsonl(&run_dir().join("memory.jsonl"))?;
    let (rows, (full, frac, n)) = reprexp::retrieval(&items);
    let mut out = String::new();
    out.push_str(&format!("Invalid fragments (first 60% of lines, the last one cut in half): fully parsed {full}/{n}; partial parsing covers on average {:.0}% of non-empty lines.\n\n", 100.0 * frac));
    out.push_str("Example search (leave-one-out, 627 programs; label — the set's area):\n\n| representation | query | P@1 | MRR@10 |\n|---|---|---|---|\n");
    for r in &rows {
        out.push_str(&format!("| {} | {} | {:.3} | {:.3} |\n", r.mode, r.query, r.p1, r.mrr));
    }
    for (skip, what) in [(false, "all moves"), (true, "without 'Output' (printing dominates)")] {
        let (mrows, base, dist) = reprexp::moves(&items, skip);
        out.push_str(&format!("\nNext-statement move choice, {what} (perceptron, 80/20 by programs; test {} statements; majority share {:.3}; test labels: {}):\n\n| representation | prefix | accuracy |\n|---|---|---|\n", mrows.first().map(|r| r.n).unwrap_or(0), base, dist.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", ")));
        for r in &mrows {
            out.push_str(&format!("| {} | {} | {:.3} |\n", r.mode, r.query, r.acc));
        }
    }
    std::fs::write(run_dir().join("repr-eval.md"), &out)?;
    println!("{out}");
    Ok(())
}

/// Training log → JSONL in the run and the `code_learning` table in `data/db/mathsolve.duckdb`.
fn cmd_journal() -> Result<()> {
    let tasks = tasks::load()?;
    let mut rounds = Vec::new();
    for r in 0..8 {
        let s = load_solutions(r)?;
        if !s.is_empty() {
            rounds.push((r, s, load_verdicts(r)?));
        }
    }
    let ctl = control::load_rows()?;
    let rows = control::journal(&tasks, &rounds, &ctl);
    let path = run_dir().join("code_learning.jsonl");
    util::write_jsonl(&path, &rows)?;
    let db = util::home().join("db/mathsolve.duckdb");
    let sql = format!("CREATE OR REPLACE TABLE code_learning AS SELECT * FROM read_json_auto('{}', format='newline_delimited'); SELECT count(*) FROM code_learning;", path.display());
    let out = std::process::Command::new(util::home().join("bin/duckdb")).arg(&db).arg("-c").arg(&sql).output()?;
    if !out.status.success() {
        bail!("duckdb: {}", String::from_utf8_lossy(&out.stderr));
    }
    eprintln!("log: {} rows → {} and {} (table code_learning)\n{}", rows.len(), path.display(), db.display(), String::from_utf8_lossy(&out.stdout));
    Ok(())
}

/// Contribution of each change: tasks where the component fired in round r — judge score and real execution vs
/// round r−1 (the same tasks). Plus a static data-flow check via the AST ("variable used before definition").
fn cmd_changes() -> Result<()> {
    let rows = control::load_rows()?;
    let get = |r: usize, t: &str| rows.iter().find(|x| x.round == r && x.task == t);
    let mut tags: Vec<(usize, String)> = Vec::new();
    for x in &rows {
        for u in &x.used {
            if x.round >= 1 && !tags.contains(&(x.round, u.clone())) && (u.starts_with("a") || u.starts_with("b") || u.starts_with("v1:")) {
                tags.push((x.round, u.clone()));
            }
        }
    }
    tags.sort();
    println!("| round | component | split | tasks | judge: before → after | actually passed: before → after |");
    println!("|---|---|---|---|---|---|");
    for (r, tag) in &tags {
        // only the first appearance of a component (the round where it was introduced)
        if tags.iter().any(|(r2, t2)| t2 == tag && r2 < r) {
            continue;
        }
        for split in ["train", "held"] {
            let sel: Vec<&control::Row> = rows.iter().filter(|x| x.round == *r && x.split == split && x.used.contains(tag)).collect();
            if sel.is_empty() {
                continue;
            }
            let prev: Vec<Option<&control::Row>> = sel.iter().map(|x| get(r - 1, &x.task)).collect();
            let m_now = mean(&sel.iter().map(|x| x.judge_correct.unwrap_or(0) as f64).collect::<Vec<_>>());
            let m_prev = mean(&prev.iter().map(|x| x.and_then(|x| x.judge_correct).unwrap_or(0) as f64).collect::<Vec<_>>());
            let p_now = sel.iter().filter(|x| x.real_pass).count();
            let p_prev = prev.iter().filter(|x| x.is_some_and(|x| x.real_pass)).count();
            println!("| r{r} | {tag} | {split} | {} | {m_prev:.2} → {m_now:.2} | {p_prev} → {p_now} |", sel.len());
        }
    }
    // static check via the AST: variable used before definition
    let mut n_undef = 0;
    let mut undef_pass = 0;
    let mut n_def = 0;
    let mut def_pass = 0;
    for r in 0..8 {
        let sols = load_solutions(r)?;
        for s in &sols {
            let Some(x) = get(r, &s.task) else { continue };
            let (f, _) = repr::ast_feats(&s.program);
            if f.iter().any(|x| x.starts_with("df:undef->")) {
                n_undef += 1;
                if x.real_pass {
                    undef_pass += 1;
                }
            } else {
                n_def += 1;
                if x.real_pass {
                    def_pass += 1;
                }
            }
        }
    }
    println!("\nStatic data-flow check (AST): 'variable used before definition' — {n_undef} programs, of which {undef_pass} actually passed; without this flaw — {n_def}, passed {def_pass}.");
    Ok(())
}

/// Judge control on subtle flaws: references and mutants of references, judge without the reference; real result — the sandbox.
fn cmd_judge_control() -> Result<()> {
    let tasks = tasks::load()?;
    let mut sols: Vec<(String, String, v0::Solution)> = Vec::new(); // (kind, mutation, "solution")
    for t in &tasks {
        sols.push(("ref".into(), "-".into(), v0::Solution { task: t.id.clone(), arch: "control".into(), program: t.solution.clone(), explanation: "(none)".into(), ..Default::default() }));
        if let Some((kind, code)) = mutate::pick(&t.solution, &t.id) {
            sols.push(("mutant".into(), kind, v0::Solution { task: t.id.clone(), arch: "control".into(), program: code, explanation: "(none)".into(), ..Default::default() }));
        }
    }
    let by_id: std::collections::HashMap<&str, &tasks::Task> = tasks.iter().map(|t| (t.id.as_str(), t)).collect();
    let items: Vec<(&tasks::Task, &v0::Solution)> = sols.iter().map(|(_, _, s)| (by_id[s.task.as_str()], s)).collect();
    let cache_path = run_dir().join("judge-noref-cache.jsonl");
    let cache: std::collections::HashMap<String, judge::Verdict> = util::read_jsonl::<judge::Cached>(&cache_path)?.into_iter().map(|c| (c.h, c.v)).collect();
    let head = prompt("judge-noref.txt")? + &prompt("mlab-lang.txt")? + "\n\nITEMS:\n";
    let batches = judge::batches_opt(&head, &items, &cache, 15, 20260926, false);
    eprintln!("judge control: {} programs, {} new batches", items.len(), batches.len());
    let dir = run_dir().join("judge-noref-raw");
    std::fs::create_dir_all(&dir)?;
    let mut jobs = Vec::new();
    let mut keymap = Vec::new();
    for (keys, text) in batches {
        let out = dir.join(format!("{}.txt", &util::sha256_hex(text.as_bytes())[..16]));
        keymap.push((out.clone(), keys.clone()));
        if !out.exists() {
            jobs.push((out.display().to_string(), text, keys.len()));
        }
    }
    if !jobs.is_empty() {
        run_jobs(jobs, 2, 1.0)?;
    }
    for (out, keys) in keymap {
        let vs = judge::parse(&std::fs::read_to_string(&out).unwrap_or_default());
        for h in keys {
            if let Some(v) = vs.iter().find(|v| v.k == h[..10]) {
                util::append_jsonl(&cache_path, &judge::Cached { h: h.clone(), v: v.clone() })?;
            }
        }
    }
    let cache: std::collections::HashMap<String, judge::Verdict> = util::read_jsonl::<judge::Cached>(&cache_path)?.into_iter().map(|c| (c.h, c.v)).collect();
    let mut rows = Vec::new();
    let mut agg: std::collections::BTreeMap<String, [usize; 4]> = Default::default(); // TP FP FN TN
    for (kind, mk, s) in &sols {
        let t = by_id[s.task.as_str()];
        let real = matches!(sandbox::run(&s.program)?, sandbox::Exec::Ran(o) if sandbox::same_output(&o, &t.expected));
        let Some(v) = cache.get(&judge::item_hash(&s.task, s)) else { continue };
        let cell = match (v.pass, real) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        };
        for key in ["all".to_string(), kind.clone(), format!("mutant:{mk}")] {
            if key == "mutant:-" {
                continue;
            }
            agg.entry(key).or_default()[cell] += 1;
        }
        rows.push(serde_json::json!({"task": s.task, "split": t.split, "kind": kind, "mutation": mk, "real_pass": real, "judge_pass": v.pass, "correct": v.correct, "why": v.why}));
    }
    util::write_jsonl(&run_dir().join("judge-control.jsonl"), &rows)?;
    println!("| programs | n | actually passed | judge 'pass' | accuracy | TP/FP/FN/TN |\n|---|---|---|---|---|---|");
    for (k, c) in &agg {
        let n: usize = c.iter().sum();
        println!("| {k} | {n} | {} | {} | {:.1}% | {}/{}/{}/{} |", c[0] + c[2], c[0] + c[1], 100.0 * (c[0] + c[3]) as f64 / n.max(1) as f64, c[0], c[1], c[2], c[3]);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("help");
    match cmd {
        "exec-one" => sandbox::exec_one(),
        "gen" => cmd_gen(),
        "build-tasks" => cmd_build_tasks(),
        "annotate" => cmd_annotate(),
        "ann-build" => cmd_ann_build(),
        "memory" => cmd_memory(),
        "transcribe" => {
            println!("{:?}", arch3::transcribe(args.get(2).context("text")?));
            Ok(())
        }
        "repr-eval" => cmd_repr_eval(),
        "validate" => {
            let t = tasks::load()?;
            let mut bad = 0;
            for x in &t {
                let e = sandbox::run(&x.solution)?;
                if !matches!(&e, sandbox::Exec::Ran(o) if sandbox::same_output(o, &x.expected)) {
                    bad += 1;
                    eprintln!("{}: {:?}", x.id, e);
                }
            }
            eprintln!("references not matched: {bad}/{}", t.len());
            Ok(())
        }
        "solve" => cmd_solve(args.get(2).context("round")?.parse()?, args.get(3).context("architecture")?, &args[4..]),
        "judge" => cmd_judge(&args[2..].iter().map(|a| a.parse()).collect::<Result<Vec<usize>, _>>()?),
        "report" => cmd_report(),
        "architect" => cmd_architect(args.get(2).context("round")?.parse()?),
        "control" => cmd_control(),
        "judge-control" => cmd_judge_control(),
        "changes" => cmd_changes(),
        "journal" => cmd_journal(),
        "show" => cmd_show(args.get(2).context("round")?.parse()?, args.get(3).context("task")?),
        "exec" => {
            let src = std::fs::read_to_string(args.get(2).context("file")?)?;
            println!("{:?}", sandbox::run(&src)?);
            Ok(())
        }
        _ => {
            eprintln!("coder gen | build-tasks | exec FILE");
            Ok(())
        }
    }
}
