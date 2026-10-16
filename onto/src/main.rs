//! onto — world model v1: commands.
//!
//! ```text
//! onto build                          # OEWN 2025+ and FrameNet 1.7 → graph snapshot
//! onto isa apple fruit                # query with proof
//! onto can crow fly --layer fable
//! onto eval | controls | bridge | bench | stats
//! onto wd-probe <dump.bz2> <GB> <dir> · wd-lexemes <lexemes.bz2> <dir> · wd-map <dir>
//! onto wd-extract <dump.bz2> <dir> [--gb G] [--bz K] [--workers W]   # full pass → TSV zstd
//! onto wd-sample <dump.bz2> <dir> [--k 16] [--mb 16]                  # sample from the middle for budgeting
//! onto wd-lex <lexemes.bz2> <dir>                                    # lexemes: lemmas, forms, senses, translations
//! onto wd-closure <dir> [--materialize auto|all|wiki|none]            # P279+P171: cycle gate, closure
//! onto wd-align <dir> [--wd-map wd-map.tsv] [--generic 0.02] [--min-src 2] [--no-folk] [--out synset_q.tsv]
//!                                                                      # OEWN synset ↔ Q with an ancestor check
//! onto wd-walk <dir> [--align synset_q-v2.tsv] [--core data/core.tsv] [--out seeds/wd-candidates.tsv]
//!                                                                      # walker: a whale is not a fish, seed candidates
//! ```
use std::path::PathBuf;
use std::time::Instant;

use anyhow::{bail, Context, Result};

mod bridge;
mod build;
mod core;
mod coregate;
mod coresel;
mod corevmm;
mod describe;
mod eval;
mod hs;
mod hsmap;
mod judge;
mod kb;
mod model;
mod pron;
mod query;
mod store;
#[cfg(test)]
mod tests;
mod wdalign;
mod wdgraph;
mod wdlex;
mod wdvmm;
mod wdwalk;
mod wdx;
mod wsd;
mod wikidata;
mod zip;

fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".into()))
}

/// The data directory: $MOVA_DATA, else ~/mova-data.
fn mova_data() -> PathBuf {
    std::env::var("MOVA_DATA").map(PathBuf::from).unwrap_or_else(|_| home().join("mova-data"))
}

pub fn snapshot_path() -> PathBuf {
    std::env::var("ONTO_SNAPSHOT").map(PathBuf::from).unwrap_or_else(|_| mova_data().join("onto/onto-v1.bin"))
}

pub fn seeds_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("seeds")
}

/// Snapshot + seeds: layers, extra edges, properties.
pub fn load_kb() -> Result<kb::Kb> {
    let g = store::load(&snapshot_path())?;
    let mut k = kb::Kb::new(g);
    let d = seeds_dir();
    k.load_layers(&d.join("layers.tsv"))?;
    k.load_relations(&d.join("relations.tsv"))?;
    k.load_defaults(&d.join("defaults.tsv"))?;
    // v3: the "core ↔ graph" gate — a contradiction is not resolved silently
    let bad = core::check(&k);
    if !bad.is_empty() && std::env::var("ONTO_CORE_GATE").as_deref() == Ok("off") {
        eprintln!("WARNING: core gate disabled (ONTO_CORE_GATE=off, for diagnostics only): {} contradictions", bad.len());
    } else if !bad.is_empty() {
        bail!("core gate: contradiction between core and graph ({}):\n{}", bad.len(), bad.join("\n"));
    }
    Ok(k)
}

fn cmd_build(args: &[String]) -> Result<()> {
    let raw = mova_data().join("raw");
    let opt = |k: &str, d: PathBuf| -> PathBuf { args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).map(PathBuf::from).unwrap_or(d) };
    let plus = opt("--oewn", raw.join("en-oewn/english-wordnet-2025-plus.xml.gz"));
    let base = opt("--base", raw.join("en-oewn/english-wordnet-2025.xml.gz"));
    let fnz = opt("--framenet", raw.join("en-framenet-1-7/framenet_v17.zip"));
    let out = opt("--out", snapshot_path());
    let t0 = Instant::now();
    let base_ids = build::Builder::base_ids(&base)?;
    let t_base = t0.elapsed().as_secs_f64();
    let mut b = build::Builder::default();
    b.oewn(&plus, Some(&base_ids))?;
    let t_oewn = t0.elapsed().as_secs_f64();
    b.framenet(&fnz)?;
    let t_fn = t0.elapsed().as_secs_f64();
    let (g, report) = b.finish()?;
    let t_fin = t0.elapsed().as_secs_f64();
    let k = kb::Kb::new(g);
    if let Some(cyc) = k.isa_cycle() {
        bail!("gate: IsA cycle in OEWN: {}", cyc.iter().map(|c| k.g.show(*c)).collect::<Vec<_>>().join(" → "));
    }
    let t_cyc = t0.elapsed().as_secs_f64();
    if let Some(p) = out.parent() {
        std::fs::create_dir_all(p)?;
    }
    let bytes = store::save(&k.g, &out)?;
    for r in report {
        println!("{r}");
    }
    println!(
        "IsA cycle gate: green\ntime: base ids {t_base:.1} s, OEWN {:.1} s, FrameNet {:.1} s, graph assembly {:.1} s, cycles {:.2} s, total {:.1} s\nsnapshot: {} ({:.1} MB)",
        t_oewn - t_base,
        t_fn - t_oewn,
        t_fin - t_fn,
        t_cyc - t_fin,
        t0.elapsed().as_secs_f64(),
        out.display(),
        bytes as f64 / 1e6
    );
    Ok(())
}

fn runs_dir() -> PathBuf {
    std::env::var("ONTO_RUNS").map(PathBuf::from).unwrap_or_else(|_| mova_data().join("runs/onto-2026-09-26"))
}

fn data_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data")
}

fn repo_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().map(|p| p.to_path_buf()).unwrap_or_default()
}

fn write(name: &str, s: &str) -> Result<PathBuf> {
    let d = runs_dir();
    std::fs::create_dir_all(&d)?;
    let p = d.join(name);
    std::fs::write(&p, s)?;
    Ok(p)
}

fn cmd_eval(tag: &str) -> Result<()> {
    let k = load_kb()?;
    let (qs, hash) = eval::read_frozen(&data_dir().join("frozen.tsv"))?;
    let t0 = Instant::now();
    let rs = eval::run(&k, &qs);
    let dt = t0.elapsed().as_secs_f64();
    let rep = eval::report(&qs, &rs, &hash, &format!("Frozen world-model set v1 — {tag}"));
    let p = write(&format!("frozen-{tag}.md"), &rep)?;
    let mut tsv = String::from("# id\tgroup\tquery\tgold\tanswer\tvalues\tstatus\n");
    for (q, (a, st)) in qs.iter().zip(&rs) {
        tsv.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\t{:?}\n", q.id, q.group, q.query, q.gold, a.verdict.code(), a.values.join(","), st));
    }
    write(&format!("frozen-{tag}.tsv"), &tsv)?;
    println!("{}", eval::tally_md(&eval::tally(&qs, &rs)));
    println!("{} questions in {:.1} ms ({:.2} ms per question); report {}", qs.len(), dt * 1e3, dt * 1e3 / qs.len() as f64, p.display());
    Ok(())
}

fn cmd_controls() -> Result<()> {
    use kb::*;
    let mut out = String::from("# World-model negative controls (v1–v3)\n\nA gate that cannot show red is not a gate: every control is green on the real data and red on an injected flaw.\n\n");
    let mut all_ok = true;
    let mut line = |name: &str, green: bool, red: bool, detail: &str| {
        let ok = green && red;
        all_ok &= ok;
        out.push_str(&format!("| {} | {} | {} | {} |\n", name, if green { "green ✓" } else { "RED ✗" }, if red { "red ✓" } else { "does NOT go red ✗" }, detail));
    };
    let k = load_kb()?;
    // 1. IsA cycle
    let t = Instant::now();
    let green = k.isa_cycle().is_none();
    let t_cyc = t.elapsed().as_secs_f64();
    let crow = query::lookup(&k.g, "crow", model::Pos::Noun).context("crow")?.senses[0];
    let animal = query::lookup(&k.g, "animal", model::Pos::Noun).context("animal")?.senses[0];
    let mut k2 = load_kb()?;
    let red = match k2.add_extra(Extra { id: "x-cycle".into(), rel: model::Rel::IsA, from: animal, to: crow, source: "control".into() }) {
        Err(e) => {
            let s = e.to_string();
            s.contains("cycle")
        }
        Ok(_) => false,
    };
    // the same flaw past the gate (gates off): the full graph check must find the cycle
    let mut k2b = load_kb()?;
    k2b.gates = false;
    k2b.add_extra(Extra { id: "x-cycle".into(), rel: model::Rel::IsA, from: animal, to: crow, source: "control".into() })?;
    let found = k2b.isa_cycle();
    let red = red && found.is_some();
    line(
        "IsA cycle",
        green,
        red,
        &format!(
            "graph has no cycles (full check {:.2} s); injected edge animal IsA crow — refused by the seed gate, and past the gate the full check finds the cycle ({} concepts)",
            t_cyc,
            found.map(|c| c.len()).unwrap_or(0)
        ),
    );
    // 2. strictness
    let bird_eggs = k.asserts.iter().find(|a| a.id == "d42").context("d42")?.clone();
    let penguin = query::lookup(&k.g, "penguin", model::Pos::Noun).context("penguin")?.senses[0];
    let mut k3 = load_kb()?;
    let red = k3.add_assert(Assert { id: "x-strict".into(), concept: penguin, value: false, ..bird_eggs.clone() }).is_err();
    let fable = k3.layer_id("fable").context("fable")?;
    let green = k3.add_assert(Assert { id: "x-strict-fable".into(), concept: penguin, value: false, layer: fable, ..bird_eggs.clone() }).is_ok();
    line("Strict on its own layer", green, red, "penguin LaysEggs=no on real against strict d42 bird LaysEggs=yes — refused; the same exception on the fable layer — accepted");
    // 3. a seed with a foreign synset
    let red = k.resolve_seed_concept("crow", "oewn-01505702-n").is_err();
    let green = k.resolve_seed_concept("bird", "oewn-01505702-n").is_ok();
    line("lemma ∈ seed synset", green, red, "crow + synset bird — refused; bird + synset bird — accepted");
    // 4. the fable exception does not leak into the real layer
    let ro = eval::real_only(&k, store::load(&snapshot_path())?)?;
    let pr = eval::probes();
    let bad = eval::leak_check(&k, &ro, &pr);
    let green = bad.is_empty();
    let mut leaky = load_kb()?;
    leaky.leak_all_layers = true;
    let bad2 = eval::leak_check(&leaky, &ro, &pr);
    let red = !bad2.is_empty();
    let real_crow = query::prop(&k, "crow", Prop::CanSpeak, 0).verdict.code();
    let fable_crow = query::prop(&k, "crow", Prop::CanSpeak, fable).verdict.code();
    line(
        "layer exception does not leak",
        green && real_crow == "no" && fable_crow == "yes",
        red,
        &format!("{} pairs (word × property) on real: with all layers = real only; crow CanSpeak: real {real_crow}, fable {fable_crow}; the flaw \"all layers visible\" gives {} differences{}", pr.len(), bad2.len(), bad2.first().map(|b| format!(", e.g. {b}")).unwrap_or_default()),
    );
    // 5. unknown word
    let nonce = ["blorft", "zzyzx", "floob", "qwertz", "snorfle"];
    let bad = eval::unknown_check(&k, &nonce);
    let known = eval::unknown_check(&k, &["crow"]);
    line("unknown word → unknown", bad.is_empty(), !known.is_empty(), &format!("{} made-up words × 8 query kinds — all \"unknown\"; the same check on the word crow goes red ({} queries with an answer)", nonce.len(), known.len()));
    // 6. shuffled hierarchy
    let (qs, _) = eval::read_frozen(&data_dir().join("frozen.tsv"))?;
    let rs = eval::run(&k, &qs);
    let t_real = eval::tally(&qs, &rs);
    let mut sk = Kb::new(eval::shuffled(&k.g, 20260926));
    sk.gates = false;
    sk.layers = k.layers.clone();
    for e in &k.extra {
        sk.add_extra(e.clone())?;
    }
    for a in &k.asserts {
        sk.add_assert(a.clone())?;
    }
    let rs_s = eval::run(&sk, &qs);
    let t_shuf = eval::tally(&qs, &rs_s);
    let acc = |t: &[(String, [usize; 3])], g: &str| -> f64 {
        t.iter().find(|x| x.0 == g).map(|(_, v)| v[0] as f64 / v.iter().sum::<usize>().max(1) as f64).unwrap_or(0.0)
    };
    let green = acc(&t_real, "isa") >= 0.75;
    let red = acc(&t_shuf, "isa") <= 0.6 && acc(&t_shuf, "prop") < acc(&t_real, "prop");
    line(
        "shuffled hierarchy",
        green,
        red,
        &format!(
            "IsA: real {} → shuffled {} (base {}); properties: {} → {}; parts: {} → {}",
            eval::group_acc(&t_real, "isa"),
            eval::group_acc(&t_shuf, "isa"),
            eval::majority_baseline(&qs, "isa"),
            eval::group_acc(&t_real, "prop"),
            eval::group_acc(&t_shuf, "prop"),
            eval::group_acc(&t_real, "part"),
            eval::group_acc(&t_shuf, "part")
        ),
    );
    // 7. frozen set hash
    let fz = std::fs::read(data_dir().join("frozen.tsv"))?;
    let want = std::fs::read_to_string(data_dir().join("frozen.sha256"))?.split_whitespace().next().unwrap_or("").to_string();
    let green = eval::sha256_hex(&fz) == want;
    let mut bad_fz = fz.clone();
    let i = bad_fz.iter().position(|b| *b == b'q').unwrap_or(0);
    bad_fz[i] = b'Q';
    let red = eval::sha256_hex(&bad_fz) != want;
    line("frozen set hash", green, red, &format!("sha256 {} matches; one changed letter — mismatch", &want[..16]));
    // 8. snapshot
    let b = std::fs::read(snapshot_path())?;
    let green = store::unpack(&b).is_ok();
    let mut red = true;
    for pos in [0usize, 9, 20, 40, b.len() / 2, b.len() - 1] {
        let mut x = b.clone();
        x[pos] ^= 0x10;
        red &= store::unpack(&x).is_err();
    }
    red &= store::unpack(&b[..b.len() - 5]).is_err();
    line("graph snapshot", green, red, "a corrupted byte (6 places) or a truncated file — a load error, not a silently different graph");
    // 9. v3: core completeness gate — a cell removed → the build fails (the same check as in build.rs)
    let core_txt = std::fs::read_to_string(data_dir().join("core.tsv"))?;
    let props = std::fs::read_to_string(data_dir().join("core-props.tsv"))?;
    let green = coregate::parse(&core_txt, &props).is_ok();
    let cut: String = props.lines().filter(|l| !(l.starts_with("fox\tanimal\t") && l.split('\t').nth(3) == Some("habitat"))).map(|l| format!("{l}\n")).collect();
    let err = coregate::parse(&core_txt, &cut).err().unwrap_or_default();
    line("core completeness (build.rs)", green, err.contains("habitat") && err.contains("fox"), &format!("294 concepts × 11 fields — every cell explicit; removed cell fox × habitat → «{err}»"));
    // 10. v3: contradiction between core and graph — gate
    let green = core::check(&k).is_empty();
    let fox = query::lookup(&k.g, "fox", model::Pos::Noun).context("fox")?.senses[0];
    let mut k4 = load_kb()?;
    k4.gates = false; // past the seed gate (d03 is strict) — we test the core gate itself
    let d03 = k4.asserts.iter().find(|a| a.prop == Prop::Animate && a.value).context("seed Animate=yes")?.clone();
    k4.add_assert(Assert { id: "x-core".into(), concept: fox, value: false, source: "control: injected contradiction".into(), ..d03 })?;
    let bad = core::check(&k4);
    line("core ↔ graph — gate", green, bad.iter().any(|b| b.contains("Fox")), &format!("real data — 0 contradictions ({} lexfile warnings); injected seed fox Animate=no → {}", core::lexfile_warnings(&k.g).len(), bad.first().cloned().unwrap_or_default()));
    let head = "| control | on real data | on an injected flaw | details |\n|---|---|---|---|\n";
    let body = out.split_off(out.find("| ").unwrap_or(out.len()));
    let rep = format!("{out}{head}{body}\n**All controls: {}**\n", if all_ok { "yes" } else { "NO" });
    let p = write("controls.md", &rep)?;
    println!("{rep}\nreport {}", p.display());
    if !all_ok {
        bail!("not all controls passed");
    }
    Ok(())
}

fn cmd_bridge() -> Result<()> {
    let k = load_kb()?;
    let files = bridge::world_files(&repo_dir(), &mova_data().join("runs"));
    let (ents, used) = bridge::read_world(&files)?;
    let mut rep = String::from("# Bridge world model ↔ story world (v1, read-only)\n\n");
    rep.push_str(&bridge::coverage(&k, &ents, &used));
    let (qs, hash) = eval::read_frozen(&data_dir().join("fable-questions.tsv"))?;
    rep.push_str(&format!("\n## Questions about fables via inheritance\n\nSet `onto/data/fable-questions.tsv`, sha256 `{hash}` (verified).\n\n"));
    let mut ok = 0;
    for q in &qs {
        // in this set the "group" field is the tale, "query" is the question in English
        let a = bridge::answer_question(&k, &ents, &q.group, &q.query);
        let st = eval::score(&a, &q.gold);
        ok += (st == eval::Status::Correct) as usize;
        rep.push_str(&format!("### {} [{}] {} — reference {}, answer {} → {:?}\n\n```\n{}\n```\n\n", q.id, q.group, q.query, q.gold, a.verdict.code(), st, a.text()));
    }
    rep.push_str(&format!("**Correct {ok} of {}.**\n", qs.len()));
    let p = write("bridge.md", &rep)?;
    let cov_end = rep.find("| class |").unwrap_or(rep.len());
    println!("{}\n…\nquestions: correct {ok} of {}; report {}", &rep[..cov_end], qs.len(), p.display());
    Ok(())
}

fn rss_kb() -> (u64, u64) {
    let s = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let get = |k: &str| s.lines().find(|l| l.starts_with(k)).and_then(|l| l.split_whitespace().nth(1)).and_then(|v| v.parse().ok()).unwrap_or(0);
    (get("VmRSS:"), get("VmHWM:"))
}

fn cmd_bench() -> Result<()> {
    let (rss0, _) = rss_kb();
    let path = snapshot_path();
    let size = std::fs::metadata(&path)?.len();
    let mut times = Vec::new();
    let mut g = None;
    for _ in 0..5 {
        let t = Instant::now();
        let x = store::load(&path)?;
        times.push(t.elapsed().as_secs_f64());
        g = Some(x);
    }
    let g = g.context("snapshot")?;
    let (rss1, hwm1) = rss_kb();
    let heap = g.src.len() * 1
        + g.key.len() * 4
        + g.pos.len()
        + g.lexfile.len()
        + g.flags.len()
        + g.ili.len() * 4
        + (g.gloss_off.len() + g.name_off.len() + g.mem_off.len() + g.mem.len() + g.word_off.len() + g.sense_off.len() + g.senses.len() + g.out_off.len() + g.in_off.len()) * 4
        + g.gloss.len()
        + g.names.len()
        + g.words.len()
        + (g.out.len() + g.inn.len()) * std::mem::size_of::<model::Edge>()
        + g.lus.len() * std::mem::size_of::<model::Lu>();
    let t = Instant::now();
    let k = load_kb()?;
    let t_kb = t.elapsed().as_secs_f64();
    let (qs, _) = eval::read_frozen(&data_dir().join("frozen.tsv"))?;
    let t = Instant::now();
    let reps = 5;
    for _ in 0..reps {
        let _ = eval::run(&k, &qs);
    }
    let per_q = t.elapsed().as_secs_f64() / (reps * qs.len()) as f64;
    times.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let rep = format!(
        "# World model v1 speed and memory\n\n- snapshot `{}`: {:.1} MB; load (median of 5): {:.0} ms (min {:.0}, max {:.0})\n- snapshot + seeds (layers, edges, properties with gates): {:.0} ms\n- graph in memory (arrays): {:.1} MB; process RSS before {:.1} MB, after {:.1} MB (peak {:.1} MB, with two graph copies during the measurement)\n- query with proof: {:.2} ms on average ({} questions × {reps})\n- concepts {}, words {}, edges {}, lexical units {}\n",
        path.display(),
        size as f64 / 1e6,
        times[2] * 1e3,
        times[0] * 1e3,
        times[4] * 1e3,
        t_kb * 1e3,
        heap as f64 / 1e6,
        rss0 as f64 / 1e3,
        rss1 as f64 / 1e3,
        hwm1 as f64 / 1e3,
        per_q * 1e3,
        qs.len(),
        g.n_concepts(),
        g.n_words(),
        g.out.len(),
        g.lus.len()
    );
    write("bench.md", &rep)?;
    print!("{rep}");
    Ok(())
}

fn cmd_stats() -> Result<()> {
    let k = load_kb()?;
    let g = &k.g;
    let mut by_src = [0usize; 4];
    for s in &g.src {
        by_src[*s as usize] += 1;
    }
    let mut by_rel = vec![0usize; model::Rel::N];
    for e in &g.out {
        by_rel[e.rel as usize] += 1;
    }
    let mut s = format!("# World model v1: what is inside\n\n- concepts {}: OEWN synsets {} (only in -plus {}), frames {}, roles {}\n- words {}, senses {}, FrameNet lexical units {}\n- relations {}:\n", g.n_concepts(), by_src[0], g.flags.iter().filter(|f| **f & model::FLAG_PLUS != 0).count(), by_src[1], by_src[2], g.n_words(), g.senses.len(), g.lus.len(), g.out.len());
    for r in model::Rel::ALL {
        s.push_str(&format!("  - {}: {}\n", r.name(), by_rel[r as usize]));
    }
    let mut lk: Vec<(String, usize)> = Vec::new();
    for l in 0..k.layers.len() {
        lk.push((k.layers[l].name.clone(), k.asserts.iter().filter(|a| a.layer as usize == l).count()));
    }
    s.push_str(&format!(
        "- layers {} ({}); property seeds {} ({}), extra edges {}\n- enum properties {}: {}\n",
        k.layers.len(),
        k.layers.iter().map(|l| format!("{} {}", l.kind.uk(), l.name)).collect::<Vec<_>>().join(", "),
        k.asserts.len(),
        lk.iter().filter(|x| x.1 > 0).map(|(n, c)| format!("{n}: {c}")).collect::<Vec<_>>().join(", "),
        k.extra.len(),
        kb::Prop::ALL.len(),
        kb::Prop::ALL.map(|p| p.name()).join(" ")
    ));
    write("stats.md", &s)?;
    print!("{s}");
    Ok(())
}

/// Value of the `--k v` key among the arguments.
fn opt<'a>(args: &'a [String], k: &str) -> Option<&'a str> {
    args.iter().position(|a| a == k).and_then(|i| args.get(i + 1)).map(|s| s.as_str())
}

fn wd_cfg(args: &[String]) -> Result<wdx::Cfg> {
    let langs = match opt(args, "--langs") {
        Some(p) => Some(std::sync::Arc::new(std::fs::read_to_string(p)?.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty() && !l.starts_with('#')).collect())),
        None => None,
    };
    Ok(wdx::Cfg { skip_wm: args.iter().any(|a| a == "--skip-wm"), langs })
}

fn wd_runs() -> PathBuf {
    std::env::var("ONTO_WD_RUNS").map(PathBuf::from).unwrap_or_else(|_| mova_data().join("runs/wikidata-2026-09-26"))
}

fn cmd_wd_extract(args: &[String]) -> Result<()> {
    let dump = PathBuf::from(args.first().context("dump")?);
    let out = PathBuf::from(args.get(1).context("output directory")?);
    let gb: Option<f64> = opt(args, "--gb").map(|s| s.parse()).transpose()?;
    let bz: usize = opt(args, "--bz").unwrap_or("8").parse()?;
    let workers: usize = opt(args, "--workers").unwrap_or("2").parse()?;
    let level: i32 = opt(args, "--level").unwrap_or("3").parse()?;
    let prefix = opt(args, "--prefix").unwrap_or("w");
    let tag = opt(args, "--tag").unwrap_or(if gb.is_some() { "pilot" } else { "full" });
    let lbzip2 = PathBuf::from(opt(args, "--lbzip2").map(|s| s.to_string()).unwrap_or_else(|| home().join(".local/bin/lbzip2").display().to_string()));
    let cfg = wd_cfg(args)?;
    let runs = wd_runs();
    std::fs::create_dir_all(&runs)?;
    let s = wdx::extract(&dump, &lbzip2, bz, workers, gb.map(|g| (g * 1e9) as u64), &out, prefix, level, &cfg, &runs.join(format!("extract-{tag}.log")))?;
    let extra = format!(
        "- dump `{}` ({} B); compressed read {} B ({:.2}%), JSON {:.1} GB (×{:.1})\n- time {:.0} s ({:.2} h): {:.1} MB/s compressed, {:.0} MB/s JSON, {:.0} entities/s\n- lbzip2 -n {bz}, parser threads {workers}, zstd {level}; lbzip2 exit: {} {}\n- skip_wm={}, languages: {}\n",
        dump.display(),
        s.bz_total,
        s.bz_read,
        100.0 * s.bz_read as f64 / s.bz_total as f64,
        s.json_bytes as f64 / 1e9,
        s.json_bytes as f64 / s.bz_read.max(1) as f64,
        s.wall,
        s.wall / 3600.0,
        s.bz_read as f64 / 1e6 / s.wall,
        s.json_bytes as f64 / 1e6 / s.wall,
        s.stats.lines as f64 / s.wall,
        s.lbzip2_status,
        s.lbzip2_stderr.replace('\n', " "),
        cfg.skip_wm,
        cfg.langs.as_ref().map(|l| l.len().to_string()).unwrap_or("all".into()),
    );
    let scale = s.bz_total as f64 / s.bz_read.max(1) as f64;
    let rep = wdx::report(&format!("Wikidata: pass «{tag}»"), &s.stats, &s.files, if gb.is_some() { scale } else { 1.0 }, &extra);
    std::fs::write(runs.join(format!("extract-{tag}.md")), &rep)?;
    wdx::write_counts(&s.stats, &runs, &format!("extract-{tag}"))?;
    println!("{rep}");
    Ok(())
}

fn cmd_wd_sample(args: &[String]) -> Result<()> {
    let dump = PathBuf::from(args.first().context("dump")?);
    let out = PathBuf::from(args.get(1).context("output directory")?);
    let k: usize = opt(args, "--k").unwrap_or("16").parse()?;
    let mb: usize = opt(args, "--mb").unwrap_or("16").parse()?;
    let threads: usize = opt(args, "--threads").unwrap_or("8").parse()?;
    let level: i32 = opt(args, "--level").unwrap_or("3").parse()?;
    let tag = opt(args, "--tag").unwrap_or("sample");
    let cfg = wd_cfg(args)?;
    let total = std::fs::metadata(&dump)?.len();
    let (st, files, used, json, wall) = wdx::sample(&dump, k, mb, threads, &out, level, &cfg)?;
    let scale = total as f64 / used.max(1) as f64;
    let extra = format!(
        "- stratified sample: {k} chunks of {mb} MiB evenly across the file; decompressed from a bzip2 block at an arbitrary bit offset\n- compressed used {:.2} GB of {:.1} GB (factor for the whole dump ×{scale:.1}), JSON {:.2} GB; {:.0} s, threads {threads}\n- estimated entities in the whole dump: {:.1} M\n- skip_wm={}, languages: {}\n",
        used as f64 / 1e9,
        total as f64 / 1e9,
        json as f64 / 1e9,
        wall,
        st.lines as f64 * scale / 1e6,
        cfg.skip_wm,
        cfg.langs.as_ref().map(|l| l.len().to_string()).unwrap_or("all".into()),
    );
    let rep = wdx::report(&format!("Wikidata: sample «{tag}»"), &st, &files, scale, &extra);
    let runs = wd_runs();
    std::fs::create_dir_all(&runs)?;
    std::fs::write(runs.join(format!("{tag}.md")), &rep)?;
    wdx::write_counts(&st, &runs, tag)?;
    println!("{rep}");
    Ok(())
}

fn cmd_wd_lex(args: &[String]) -> Result<()> {
    let dump = PathBuf::from(args.first().context("lexeme dump")?);
    let out = PathBuf::from(args.get(1).context("output directory")?);
    let bz: usize = opt(args, "--bz").unwrap_or("8").parse()?;
    let level: i32 = opt(args, "--level").unwrap_or("9").parse()?;
    let lbzip2 = home().join(".local/bin/lbzip2");
    let (st, files) = wdlex::extract(&dump, &lbzip2, bz, &out, level)?;
    let rep = wdlex::report(&st, &files, &dump);
    let runs = wd_runs();
    std::fs::create_dir_all(&runs)?;
    std::fs::write(runs.join("lexemes.md"), &rep)?;
    println!("{rep}");
    Ok(())
}

fn cmd_wd_closure(args: &[String]) -> Result<()> {
    let dir = PathBuf::from(args.first().context("derivatives directory")?);
    let threads: usize = opt(args, "--threads").unwrap_or("8").parse()?;
    let mat = wdgraph::Materialize::parse(opt(args, "--materialize").unwrap_or("auto"))?;
    let limit: u64 = opt(args, "--limit").unwrap_or("400000000").parse()?;
    let level: i32 = opt(args, "--level").unwrap_or("9").parse()?;
    let inject = match opt(args, "--inject") {
        Some(x) => {
            let (a, b) = x.split_once(':').context("--inject Qa:Qb")?;
            Some((wdx::num_id(a, b'Q').context("Qa")?, wdx::num_id(b, b'Q').context("Qb")?))
        }
        None => None,
    };
    let runs = wd_runs();
    std::fs::create_dir_all(&runs)?;
    let rep = wdgraph::run_closure(&dir, &runs, threads, mat, limit, inject, level)?;
    println!("{rep}");
    Ok(())
}

fn cmd_wd_align(args: &[String]) -> Result<()> {
    let dir = PathBuf::from(args.first().context("derivatives directory")?);
    let runs = wd_runs();
    std::fs::create_dir_all(&runs)?;
    let wd_map = opt(args, "--wd-map").map(PathBuf::from).unwrap_or_else(|| runs.join("wd-map.tsv"));
    // v2: generality threshold 2% (v1 — 5%), two supporting sources, folk ancestors (P171 up to class)
    let share: f64 = opt(args, "--generic").unwrap_or("0.02").parse()?;
    let min_src: u8 = opt(args, "--min-src").unwrap_or("2").parse()?;
    let folk = !args.iter().any(|a| a == "--no-folk");
    let seed: u64 = opt(args, "--seed").unwrap_or("20260926").parse()?;
    let n: usize = opt(args, "--sample").unwrap_or("50").parse()?;
    let out = opt(args, "--out").map(PathBuf::from).unwrap_or_else(|| dir.join("synset_q.tsv"));
    let gold = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/align-gold.tsv");
    let g = store::load(&snapshot_path())?;
    let r = wdalign::run(&g, &dir, &runs, &wd_map, share, seed, n, min_src, folk, &gold)?;
    let k = wdalign::write_rows(&g, &r.rows, &out)?;
    println!("{}\nalignment rows: {k} → {}", r.report, out.display());
    Ok(())
}

/// Disputed candidates → LLM "yes / no / refine + why" → `seeds/wd-confirm.tsv`.
fn cmd_wd_confirm(args: &[String]) -> Result<()> {
    let n: usize = opt(args, "--n").unwrap_or("60").parse()?;
    let batch: usize = opt(args, "--batch").unwrap_or("30").parse()?;
    let cap: f64 = opt(args, "--cap").unwrap_or("10").parse()?;
    let runs = wd_runs();
    std::fs::create_dir_all(&runs)?;
    let src = seeds_dir().join("wd-candidates.tsv");
    let lo: f64 = opt(args, "--lo").unwrap_or("0.45").parse()?;
    let hi: f64 = opt(args, "--hi").unwrap_or("0.8").parse()?;
    let dest = opt(args, "--out").map(PathBuf::from).unwrap_or_else(|| seeds_dir().join("wd-confirm.tsv"));
    let rows = wdvmm::pick_disputed(&std::fs::read_to_string(&src)?, n, lo, hi);
    let log = runs.join("vmm-calls.jsonl");
    let cwd = runs.join("vmm-cwd");
    let mut out = format!("# onto v2: candidates from wd-candidates.tsv (confidence {lo}–{hi}) — second opinion of the LLM (claude-opus-5-5, effort high). Not truth: into the core only after a human.\n# concept\tsynset\tprop\tvalue\tsource\tconfidence\tvmm\twhy\n");
    let (mut cost, mut tally) = (0.0, std::collections::BTreeMap::<String, usize>::new());
    for (bi, ch) in rows.chunks(batch.max(1)).enumerate() {
        let (ans, c) = wdvmm::ask(&cwd, &log, cap, 1.5, &wdvmm::prompt(ch))?;
        cost += c;
        for (f, v) in ch.iter().zip(wdvmm::parse(&ans, ch.len())) {
            let (verdict, why) = v.unwrap_or(("no answer".into(), String::new()));
            *tally.entry(verdict.clone()).or_default() += 1;
            out.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\t{verdict}\t{}\n", f[0], f[2], f[3], f[4], f[5], f[6], why.replace('\t', " ")));
        }
        println!("batch {}: {} rows, ${c:.3}", bi + 1, ch.len());
    }
    std::fs::write(&dest, &out)?;
    let rep = format!("# LLM: disputed candidates\n\n- rows: {}; verdicts: {}\n- cost: ${cost:.2} (total by the log ${:.2}, cap ${cap:.0})\n", rows.len(), tally.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(" · "), wdvmm::spent(&log));
    std::fs::write(runs.join(format!("confirm-{lo}-{hi}.md")), &rep)?;
    println!("{rep}");
    Ok(())
}

/// v3: `describe <word> [--layer L] [--no-core] [--ctx noun.animal]` — a concept description with proofs.
fn cmd_describe(args: &[String]) -> Result<()> {
    let k = load_kb()?;
    let word = args.first().context("describe <word>")?;
    let layer = match opt(args, "--layer") {
        Some(l) => k.layer_id(l).with_context(|| format!("unknown layer «{l}»"))?,
        None => 0,
    };
    let use_core = !args.iter().any(|a| a == "--no-core");
    let ctx = opt(args, "--ctx").and_then(|c| model::LEXFILES.iter().find(|x| **x == c).copied());
    match describe::describe(&k, word, ctx, layer, use_core) {
        Some(d) => println!("{}completeness: {} of {} applicable fields", d.text, d.filled(), d.applicable()),
        None => println!("unknown: the word «{word}» is not in the world model — not guessing"),
    }
    Ok(())
}

/// hs (26.09): the HS22 goods classification as a world model — `onto hs build | describe <code> [--text] | confirm | judge`.
fn cmd_hs(args: &[String]) -> Result<()> {
    let runs = std::env::var("ONTO_RUNS").map(PathBuf::from).unwrap_or_else(|_| mova_data().join("runs/hsonto-2026-09-26"));
    std::fs::create_dir_all(&runs)?;
    let k = load_kb()?;
    let out = hs::data_dir();
    let num = |key: &str, d: &str| -> Result<f64> { Ok(opt(args, key).unwrap_or(d).parse()?) };
    match args.first().map(|s| s.as_str()) {
        Some("build") => {
            let conf = if args.iter().any(|a| a == "--no-confirm") { Default::default() } else { hsmap::load_confirm(&out.join("sense-confirm.tsv")) };
            let b = hsmap::build(&k, &out, &runs, &conf)?;
            std::fs::write(runs.join("hs-report.md"), &b.report)?;
            println!("{}\nreport {}", b.report, runs.join("hs-report.md").display());
        }
        Some("parse") => {
            // parsing debug: tokens and features (WCO text — only on screen, not in files)
            let a = en::annotate::Annotator::load(&hs::model_path())?;
            for code in &args[1..] {
                let v = hs::read_codes(&hs::csv_path())?;
                let Some((_, d)) = v.iter().find(|x| &x.0 == code) else { continue };
                let t = hs::annotate(&a, d);
                println!("{code}: {d}\n  {}", t.iter().map(|x| format!("{}/{}/{}", x.form, x.lemma, x.upos)).collect::<Vec<_>>().join(" "));
                for f in hs::parse(&t).feats {
                    println!("  {:?} L{} {}{} {}", f.kind, f.level, if f.neg { "¬" } else { "" }, f.val, f.bound.map(|b| b.show()).unwrap_or_default());
                }
            }
        }
        Some("describe") => {
            let code = args.get(1).context("hs describe <code>")?;
            print!("{}", hsmap::describe(&k, code, args.iter().any(|a| a == "--text"))?);
        }
        Some("confirm") => {
            let b = hsmap::build(&k, &out, &runs, &Default::default())?;
            let r = hsmap::confirm(&k, &b, &runs, &out, num("--n", "120")? as usize, num("--margin", "0.15")?, num("--batch", "30")? as usize, num("--cap", "15")?)?;
            std::fs::write(runs.join("hs-confirm.md"), &r)?;
            println!("{r}");
        }
        Some("labels") => {
            let r = hsmap::labels(&runs, &out)?;
            std::fs::write(runs.join("hs-labels.md"), &r)?;
            println!("{r}");
        }
        Some("judge") => {
            let conf = hsmap::load_confirm(&out.join("sense-confirm.tsv"));
            let b = hsmap::build(&k, &out, &runs, &conf)?;
            let r = hsmap::judge(&k, &b, &runs, num("--n", "150")? as usize, num("--decoys", "20")? as usize, num("--batch", "20")? as usize, num("--cap", "15")?)?;
            std::fs::write(runs.join("hs-judge.md"), &r)?;
            println!("{r}");
        }
        _ => bail!("onto hs build [--no-confirm] | describe <code> [--text] | parse <code>… | labels | confirm [--n 120] [--margin 0.15] [--cap 15] | judge [--n 150] [--decoys 20] [--cap 15]"),
    }
    Ok(())
}

/// v3: `describe-eval` — description of all classes of the tale and fable worlds before (no core, as in v2) and after (core v3); completeness
/// by category; descriptions — `describe-{before,after}.tsv` for the judge.
fn cmd_describe_eval() -> Result<()> {
    let k = load_kb()?;
    let files = bridge::world_files(&repo_dir(), &mova_data().join("runs"));
    let (ents, _) = bridge::read_world(&files)?;
    let classes = describe::world_classes(&ents);
    let mut rep = format!("# Descriptions of tale and fable world classes: before (no core) and after (core v3)\n\nClasses: {}.\n", classes.len());
    for (tag, use_core) in [("before", false), ("after", true)] {
        let mut ds = Vec::new();
        let mut tsv = String::from("# class\tctx\tcategory\tfilled\tapplicable\ttext (\\n — line break)\n");
        for (c, ctx) in &classes {
            if let Some(d) = describe::describe(&k, c, *ctx, 0, use_core) {
                tsv.push_str(&format!("{c}\t{}\t{}\t{}\t{}\t{}\n", ctx.unwrap_or("-"), d.category, d.filled(), d.applicable(), d.text.trim_end().replace('\t', " ").replace('\n', "\\n")));
                ds.push((c.clone(), d));
            }
        }
        write(&format!("describe-{tag}.tsv"), &tsv)?;
        rep.push_str(&format!("\n## {}\n\nDescribed {} of {}.\n\n{}", if use_core { "After: core v3, then graph" } else { "Before: graph and seeds (v2, no core)" }, ds.len(), classes.len(), describe::completeness(&ds)));
    }
    let p = write("describe.md", &rep)?;
    println!("{rep}\nreport {}", p.display());
    Ok(())
}

/// Target words for prior weights: the WSD set, the core, the frozen set, world classes.
fn wsd_targets() -> Result<Vec<String>> {
    let mut t: Vec<String> = Vec::new();
    let (qs, _) = eval::read_frozen(&data_dir().join("wsd-frozen.tsv"))?;
    t.extend(qs.iter().map(|q| q.group.clone()));
    for l in std::fs::read_to_string(data_dir().join("core.tsv"))?.lines().filter(|l| !l.starts_with('#')) {
        t.extend(l.split('\t').next().map(|x| x.replace('_', " ")));
    }
    let (fz, _) = eval::read_frozen(&data_dir().join("frozen.tsv"))?;
    for q in &fz {
        t.extend(q.query.split_whitespace().skip(1).filter(|w| !w.starts_with("--")).map(String::from));
    }
    let (ents, _) = bridge::read_world(&bridge::world_files(&repo_dir(), &mova_data().join("runs")))?;
    t.extend(ents.iter().filter_map(|e| e.class.clone()));
    t.sort();
    t.dedup();
    Ok(t)
}

/// v3: `wsd-prior [--k 40]` — preferred senses from the Leipzig corpora (McCarthy 2004) → ONTO_PRIOR.
fn cmd_wsd_prior(args: &[String]) -> Result<()> {
    let k: usize = opt(args, "--k").unwrap_or("40").parse()?;
    let kb = load_kb()?;
    let raw = mova_data().join("raw/en-leipzig-corpora");
    let corpora: Vec<(PathBuf, String)> = ["eng_wikipedia_2016_1M", "eng_news_2024_1M"].iter().map(|n| (raw.join(format!("{n}.tar.gz")), n.to_string())).collect();
    let targets = wsd_targets()?;
    println!("{}", wsd::build_prior(&kb, &corpora, &targets, k, &wsd::prior_path())?);
    Ok(())
}

/// v3: `wsd <word> <sentence…>` — sense choice with an explanation.
fn cmd_wsd(args: &[String]) -> Result<()> {
    let kb = load_kb()?;
    let w = wsd::Wsd::new(&kb, wsd::Prior::load(&wsd::prior_path())?);
    let word = args.first().context("wsd <word> <sentence>")?;
    let sent = args[1..].join(" ");
    match w.choose(word, &sent) {
        Some(ch) => print!("{}", ch.explain(&kb.g, &w.prior)),
        None => println!("unknown: the word «{word}» is not among the OEWN nouns — not guessing"),
    }
    Ok(())
}

/// WSD accuracy on the frozen set: (correct, total, errors).
fn wsd_acc(w: &wsd::Wsd, qs: &[eval::Q]) -> (usize, usize, Vec<String>) {
    let (mut ok, mut bad) = (0, Vec::new());
    for q in qs {
        let gold: Vec<&str> = q.query.split('|').collect();
        let pick = w.choose(&q.group, &q.gold).map(|c| w.kb.g.cid_str(c.sense()));
        if pick.as_deref().is_some_and(|p| gold.contains(&p)) {
            ok += 1;
        } else {
            bad.push(format!("{} {}: {} (reference {})", q.id, q.group, pick.unwrap_or("-".into()), q.query));
        }
    }
    (ok, qs.len(), bad)
}

/// v3: `wsd-eval` — the frozen WSD set: the mix and individual terms vs the "first sense" baseline; control with shuffled
/// glosses.
fn cmd_wsd_eval() -> Result<()> {
    let kb = load_kb()?;
    // fields in the WSD set: id, word (group), gold (query), sentence (gold) — the read_frozen order
    let (qs, hash) = eval::read_frozen(&data_dir().join("wsd-frozen.tsv"))?;
    let prior = wsd::Prior::load(&wsd::prior_path())?;
    let mut w = wsd::Wsd::new(&kb, prior);
    let base = qs.iter().filter(|q| query::lookup(&kb.g, &q.group, model::Pos::Noun).map(|f| q.query.split('|').any(|x| x == kb.g.cid_str(f.senses[0]))).unwrap_or(false)).count();
    let mut rep = format!("# WSD v1 — frozen set\n\nSet `onto/data/wsd-frozen.tsv`, sha256 `{hash}` (verified). Weights fixed in advance: Lesk {}, corpus {}, core {}, domain {}.\n\n| system | correct |\n|---|---:|\n| OEWN \"first sense\" baseline | {base}/{} ({:.0}%) |\n", wsd::W_LESK, wsd::W_PRIOR, wsd::W_CORE, wsd::W_DOMAIN, qs.len(), 100.0 * base as f64 / qs.len() as f64);
    let mixes = [
        ("all together", wsd::Mix::ALL),
        ("Lesk only", wsd::Mix { lesk: true, prior: false, core: false, domain: false }),
        ("corpus only (McCarthy)", wsd::Mix { lesk: false, prior: true, core: false, domain: false }),
        ("corpus + core (no context)", wsd::Mix { lesk: false, prior: true, core: true, domain: false }),
        ("without corpus", wsd::Mix { lesk: true, prior: false, core: true, domain: true }),
        ("without Lesk", wsd::Mix { lesk: false, prior: true, core: true, domain: true }),
    ];
    let mut all_bad = Vec::new();
    let mut res = std::collections::HashMap::new();
    for (name, m) in mixes {
        w.mix = m;
        let (ok, n, bad) = wsd_acc(&w, &qs);
        res.insert(name, ok);
        rep.push_str(&format!("| {name} | {ok}/{n} ({:.0}%) |\n", 100.0 * ok as f64 / n as f64));
        if name == "all together" {
            all_bad = bad;
        }
    }
    // control: shuffled glosses — each sense's gloss is taken from a random other synset
    let mut ws = wsd::Wsd::new(&kb, wsd::Prior::load(&wsd::prior_path())?);
    ws.perm = Some(wsd::synset_perm(&kb.g, 20260926));
    ws.mix = wsd::Mix { lesk: true, prior: false, core: false, domain: false };
    let (lesk_sh, n, _) = wsd_acc(&ws, &qs);
    ws.mix = wsd::Mix::ALL;
    let (all_sh, _, _) = wsd_acc(&ws, &qs);
    // glosses only (lemmas and neighbours are real) — for information, not a gate
    let gs = wsd::shuffled_glosses(&kb.g, 20260926);
    let mut ks = kb::Kb::new(gs);
    ks.layers = kb.layers.clone();
    let mut wg = wsd::Wsd::new(&ks, wsd::Prior::default());
    wg.mix = wsd::Mix { lesk: true, prior: false, core: false, domain: false };
    let (lesk_gl, _, _) = wsd_acc(&wg, &qs);
    let lesk = res["Lesk only"];
    let ctxfree = res["corpus + core (no context)"];
    let red = lesk_sh <= base + 3 && all_sh <= ctxfree.max(base) + 3;
    rep.push_str(&format!("\n**Control — shuffled glosses** (a sense's gloss comes from a random synset): Lesk only {lesk} → {lesk_sh}/{n} (baseline {base}); all together {} → {all_sh}/{n} (no context {ctxfree}). {}\nFor information: only the gloss texts are shuffled, lemmas and neighbours are real — Lesk only {lesk} → {lesk_gl}/{n}: neighbour lemmas (whale, cricket bat…) carry part of the signal.\n", res["all together"], if red { "Goes red ✓ — the context gain drops to the baseline." } else { "Does NOT go red ✗" }));
    rep.push_str("\n**Errors (all together):**\n\n");
    for b in &all_bad {
        rep.push_str(&format!("- {b}\n"));
    }
    rep.push_str("\n## Every choice with an explanation\n\n");
    w.mix = wsd::Mix::ALL;
    for q in &qs {
        if let Some(ch) = w.choose(&q.group, &q.gold) {
            rep.push_str(&format!("### {} «{}» — reference {}\n\n```\n{}```\n\n", q.id, q.gold, q.query, ch.explain(&kb.g, &w.prior)));
        }
    }
    let p = write("wsd.md", &rep)?;
    let head = rep.split("\n## Every choice").next().unwrap_or("");
    println!("{head}\nreport {}", p.display());
    if !red {
        bail!("the shuffled-gloss control does not go red");
    }
    Ok(())
}

/// v3: `describe-judge [--batch 12] [--cap 15]` — blind Opus judge; `describe-judge report` — the report.
fn cmd_describe_judge(args: &[String]) -> Result<()> {
    let runs = runs_dir();
    if args.first().map(|s| s.as_str()) == Some("architect") {
        println!("{}", judge::architect(&runs, opt(args, "--round").unwrap_or("1").parse()?, opt(args, "--cap").unwrap_or("30").parse()?)?);
        return Ok(());
    }
    if args.first().map(|s| s.as_str()) != Some("report") {
        let batch: usize = opt(args, "--batch").unwrap_or("12").parse()?;
        let cap: f64 = opt(args, "--cap").unwrap_or("15").parse()?;
        println!("{}", judge::run(&runs, batch, cap)?);
    }
    let rep = judge::report(&runs)?;
    let p = write("describe-judge.md", &rep)?;
    println!("{rep}\nreport {}", p.display());
    Ok(())
}

/// v3: `core-select [--align synset_q-v2.tsv]` — core selection → `data/core-props.tsv` (compiled by build.rs).
fn cmd_core_select(args: &[String]) -> Result<()> {
    let g = store::load(&snapshot_path())?;
    let align = opt(args, "--align").map(PathBuf::from).unwrap_or_else(|| mova_data().join("runs/onto2-2026-09-26/synset_q-v2.tsv"));
    let out = data_dir().join("core-props.tsv");
    let r = coresel::select(&g, &data_dir().join("core.tsv"), &seeds_dir(), &align, &out)?;
    println!("{}
→ {} (rebuild: cargo build — build.rs will check completeness)", r.text, out.display());
    Ok(())
}

/// v3: `core-vmm confirm [--batch 50] [--cap 7] [--limit N]` — the LLM pushes through candidates without a verdict;
/// `core-vmm risks [--batch 50] [--cap 8]` — risks of core concepts. Runs — ONTO_WD_RUNS.
fn cmd_core_vmm(args: &[String]) -> Result<()> {
    let runs = wd_runs();
    std::fs::create_dir_all(&runs)?;
    let batch: usize = opt(args, "--batch").unwrap_or("50").parse()?;
    let cap: f64 = opt(args, "--cap").unwrap_or("7").parse()?;
    let g = store::load(&snapshot_path())?;
    let rep = match args.first().map(|s| s.as_str()) {
        Some("confirm") => corevmm::confirm(&g, &seeds_dir(), &runs, batch, cap, opt(args, "--limit").unwrap_or("100000").parse()?)?,
        Some("risks") => {
            let core = wdwalk::read_core(&g, &data_dir().join("core.tsv"))?;
            let rows: Vec<(String, String, String)> = core.iter().map(|c| (c.lemma.clone(), c.domain.clone(), c.synset.map(|x| g.cid_str(x)).unwrap_or_default())).collect();
            corevmm::risks(&g, &rows, &seeds_dir(), &runs, batch, cap)?
        }
        _ => bail!("core-vmm confirm|risks"),
    };
    println!("{rep}");
    Ok(())
}

fn cmd_wd_walk(args: &[String]) -> Result<()> {
    let dir = PathBuf::from(args.first().context("derivatives directory")?);
    let runs = wd_runs();
    std::fs::create_dir_all(&runs)?;
    let align = opt(args, "--align").map(PathBuf::from).unwrap_or_else(|| runs.join("synset_q-v2.tsv"));
    let core = opt(args, "--core").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/core.tsv"));
    let out = opt(args, "--out").map(PathBuf::from).unwrap_or_else(|| seeds_dir().join("wd-candidates.tsv"));
    let g = store::load(&snapshot_path())?;
    let w = wdwalk::run(&g, &dir, &align, &core, &out)?;
    std::fs::write(runs.join("walk.md"), &w.report)?;
    println!("{}", w.report);
    if !w.controls_ok {
        bail!("control queries or the cycle gate are red (report: {})", runs.join("walk.md").display());
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let a = |i: usize| -> Result<&str> { args.get(i).map(|s| s.as_str()).context(format!("missing argument {i}")) };
    match args.first().map(|s| s.as_str()) {
        Some("build") => cmd_build(&args[1..]),
        Some("up" | "isa" | "can" | "prop" | "haspart" | "partof" | "memberof" | "madeof" | "parts" | "frame" | "role" | "inherits" | "entails" | "causes" | "word") => {
            let k = load_kb()?;
            let ans = query::ask(&k, &args.join(" "))?;
            println!("{}", ans.text());
            Ok(())
        }
        Some("eval") => cmd_eval(args.get(1).map(|s| s.as_str()).unwrap_or("blind")),
        Some("vmm") => cmd_vmm(&args[1..]),
        Some("controls") => cmd_controls(),
        Some("bridge") => cmd_bridge(),
        Some("bench") => cmd_bench(),
        Some("stats") => cmd_stats(),
        Some("wd-map") => {
            let dir = PathBuf::from(a(1)?);
            let g = store::load(&snapshot_path())?;
            wikidata::map_trial(&g, &dir.join("wd-senses-en.tsv"), &dir.join("wd-map.tsv"), &dir.join("wd-map.md"))
        }
        Some("wd-probe") => {
            let gb: f64 = a(2)?.parse()?;
            let dir = PathBuf::from(a(3)?);
            wikidata::probe(&PathBuf::from(a(1)?), (gb * (1u64 << 30) as f64) as u64, &dir.join("wd-probe-classes.tsv"), &dir.join("wd-probe.md"))
        }
        Some("wd-extract") => cmd_wd_extract(&args[1..]),
        Some("wd-sample") => cmd_wd_sample(&args[1..]),
        Some("wd-lex") => cmd_wd_lex(&args[1..]),
        Some("wd-closure") => cmd_wd_closure(&args[1..]),
        Some("wd-align") => cmd_wd_align(&args[1..]),
        Some("wd-walk") => cmd_wd_walk(&args[1..]),
        Some("wd-confirm") => cmd_wd_confirm(&args[1..]),
        Some("core-vmm") => cmd_core_vmm(&args[1..]),
        Some("core-select") => cmd_core_select(&args[1..]),
        Some("describe") => cmd_describe(&args[1..]),
        Some("hs") => cmd_hs(&args[1..]),
        Some("describe-eval") => cmd_describe_eval(),
        Some("describe-judge") => cmd_describe_judge(&args[1..]),
        Some("wsd-prior") => cmd_wsd_prior(&args[1..]),
        Some("wsd") => cmd_wsd(&args[1..]),
        Some("wsd-eval") => cmd_wsd_eval(),
        Some("wd-senses") => {
            // English senses of lexemes from P5137/P8814 → wd-senses-en.tsv (for `onto wd-map`)
            let dir = PathBuf::from(a(1)?);
            let out = PathBuf::from(a(2)?);
            let (n, q) = wdlex::senses_en(&dir, &out)?;
            println!("English senses from P5137 or P8814: {n}, from P5137: {q} → {}", out.display());
            Ok(())
        }
        Some("wd-lexemes") => {
            let dir = PathBuf::from(a(2)?);
            wikidata::lexemes(&PathBuf::from(a(1)?), &dir.join("wd-senses-en.tsv"), &dir.join("wd-lexemes.md"))
        }
        _ => bail!("commands: build · isa|can|prop|haspart|partof|memberof|madeof|parts|frame|role|inherits|entails|causes|word … [--layer L] · wd-probe · wd-lexemes · wd-map · wd-extract · wd-sample"),
    }
}

/// tests3: LLM questions (independent author, `tests/data/vmm/*.tsv`, frozen by hashes). Queries v2 does not
/// know yet (`measure`, `sense`) count as "not supported", not "wrong". Report — `vmm-<tag>.md` and `.tsv` in ONTO_RUNS.
fn cmd_vmm(args: &[String]) -> Result<()> {
    const FROZEN: &[(&str, &str)] = &[
        ("isa.tsv", "f4d9cc1a81e97ac4717dd96159b22dce0ce3c8725689764d2a863214014d7b15"),
        ("part.tsv", "2fda493a4a7b0f096ffb82b65ed404883962196568e7f82f6d661b2acfb35354"),
        ("prop.tsv", "c8f572f3f2c752e7e4dadba4095968428b18eed3fc15b16bc14c078df41279c1"),
        ("measure.tsv", "a6fd68e74237376e7e8c7cf9e20f8d8823085ebfde64e6cc83c2299b552c2b52"),
        ("sense.tsv", "ceb45d3fc94d8c2104d985a388e88126bc79114ba251bdb58cdffc39df37ad8c"),
    ];
    let tag = args.first().map(|s| s.as_str()).unwrap_or("v2");
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data/vmm");
    let k = load_kb()?;
    let mut tsv = String::from("# id\tgroup\tquery\tgold\tanswer\tstatus\n");
    // group → [correct, unknown, wrong, not supported]
    let mut tally: std::collections::BTreeMap<String, [usize; 4]> = Default::default();
    for (file, want) in FROZEN {
        let bytes = std::fs::read(dir.join(file))?;
        let h = eval::sha256_hex(&bytes);
        if h != *want {
            anyhow::bail!("gate: {file} changed after freezing ({h})");
        }
        for l in String::from_utf8(bytes)?.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            let f: Vec<&str> = l.split('\t').collect();
            let (id, group, q, gold) = (f[0], f[1], f[2], f[3].trim());
            let head = q.split_whitespace().next().unwrap_or("");
            let (ans, st) = if matches!(head, "measure" | "sense") {
                ("—".to_string(), 3)
            } else {
                match query::ask(&k, q) {
                    Ok(a) => {
                        let s = match eval::score(&a, gold) {
                            eval::Status::Correct => 0,
                            eval::Status::Unknown => 1,
                            eval::Status::Wrong => 2,
                        };
                        (a.verdict.code().to_string(), s)
                    }
                    Err(e) => (format!("error: {e}"), if gold == "unknown" { 0 } else { 1 }),
                }
            };
            tally.entry(group.to_string()).or_default()[st] += 1;
            let sname = ["correct", "unknown", "wrong", "not supported"][st];
            tsv.push_str(&format!("{id}\t{group}\t{q}\t{gold}\t{ans}\t{sname}\n"));
        }
    }
    let mut md = format!("# LLM questions to onto ({tag})\n\n| group | correct | \"unknown\" | wrong | not supported |\n|---|---:|---:|---:|---:|\n");
    let mut tot = [0usize; 4];
    for (g, t) in &tally {
        md.push_str(&format!("| {g} | {} | {} | {} | {} |\n", t[0], t[1], t[2], t[3]));
        for i in 0..4 {
            tot[i] += t[i];
        }
    }
    md.push_str(&format!("| **total** | **{}** | **{}** | **{}** | **{}** |\n", tot[0], tot[1], tot[2], tot[3]));
    // external hasPart KB sample (HF datasets-server rows, JSON): all facts are positive — reference "yes"
    if let Some(i) = args.iter().position(|a| a == "--haspart") {
        let path = args.get(i + 1).context("--haspart <rows.json>")?;
        let v: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
        let mut t = [0usize; 3];
        for r in v["rows"].as_array().context("rows")? {
            let (a, b) = (r["row"]["arg1"].as_str().unwrap_or(""), r["row"]["arg2"].as_str().unwrap_or(""));
            let q = format!("haspart {} {}", a.replace(' ', "_"), b.replace(' ', "_"));
            let (ans, st) = match query::ask(&k, &q) {
                Ok(x) => (x.verdict.code().to_string(), match eval::score(&x, "yes") { eval::Status::Correct => 0, eval::Status::Unknown => 1, eval::Status::Wrong => 2 }),
                Err(e) => (format!("error: {e}"), 1),
            };
            t[st] += 1;
            tsv.push_str(&format!("hp\thaspart-ext\t{q}\tyes\t{ans}\t{}\n", ["correct", "unknown", "wrong"][st]));
        }
        md.push_str(&format!("\nhasPart KB (sample of {} facts, reference yes): correct {}, \"unknown\" {}, wrong {}\n", t.iter().sum::<usize>(), t[0], t[1], t[2]));
    }
    write(&format!("vmm-{tag}.tsv"), &tsv)?;
    let p = write(&format!("vmm-{tag}.md"), &md)?;
    println!("{md}\nreport {}", p.display());
    Ok(())
}
