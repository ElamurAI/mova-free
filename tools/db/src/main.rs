//! db — the Mova annotation database on DuckDB. Design note: parses of fairy-tale sentences (the same trees,
//! pragmatics and everything else) are written there; it lives in the dev workspace; the whole Mova corpus is
//! moved into it as well. Database — `$MOVA_DATA/db/mova.duckdb`; schema and queries — `db/` in the repo.
//!
//!   db init   <db>
//!   db load   <db> <source> <dialect> [--kind K] [--docs newdoc|idprefix|file] [--match new|auto|existing]
//!             [--run name] [--layer ud] [--lang en] [--origin O] [--method text] [--license L] [--genre G]
//!             [--tool T] <files.conllu>…
//!   db corpus <db> [corpus.tsv]                    the whole Mova corpus per the manifest (`db/corpus.tsv`)
//!   db tales  <db> <manifest.jsonl> <raw> <run dir> <train.conllu>…
//!   db export <db> <key=value>… > out.conllu        keys: file run source tool layer dialect origin doc sent sid aid gate
//!   db verify <db> [file]                           lossless: export = file, counters, size from the card
//!   db stats  <db>                                  overview (`db/sql/stats.sql`)
//!   db sql    <db> <file.sql>                       query (e.g. `db/queries/*.sql`), markdown output
//!   db migrate <db>                                 schema migrations (`schema.sql` from "-- migrate:v"), idempotent
//!   db prag   <db> <labels.tsv> --run name --source silver|rust [--tool opus|rust] [--origin O] [--method M]
//!             [--params params.json] [--path P] [--note N]   pragmatics layer (crate `prag`)
//!   db qa     <db> <qa.tsv> --run name [--source opus] [--tool T] [--params params.json] [--path P] [--note N]
//!             [--replaces run]
//!             training questions and answers per paragraph (crate `qagen`, table `qa`, migrations v3–v4);
//!             with --replaces the file's rows replace rows of that run (old ones stay with `replaced_by`)
//!
//! DuckDB — CLI `~/bin/duckdb` (variable `DUCKDB`): Rust writes TSV, the CLI loads it with COPY and exports.
//! Source: gold | silver | bronze | opus | rust | udpipe. Dialect: ud-2.18 | mova | …

mod duck;
mod export;
mod load;
mod prag;
mod qa;
mod tales;
mod tsv;
mod util;

use std::io::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use duck::Duck;
use load::{Docs, Match, Opts};

const SCHEMA: &str = include_str!("../../../db/schema.sql");
const CORPUS: &str = include_str!("../../../db/corpus.tsv");
const STATS: &str = include_str!("../../../db/sql/stats.sql");

fn usage() -> Result<()> {
    bail!(
        "usage: db init <db> | db load <db> <source> <dialect> [--options] <files.conllu>… | db corpus <db> [corpus.tsv] | \
         db tales <db> <manifest.jsonl> <raw> <run dir> <train.conllu>… | db export <db> <key=value>… | \
         db verify <db> [file] | db stats <db> | db sql <db> <file.sql> | db migrate <db> | \
         db prag <db> <labels.tsv> --run name --source silver|rust [--tool T] [--origin O] [--method M] [--params f.json] [--path P] [--note N] | \
         db qa <db> <qa.tsv> --run name [--source opus] [--tool T] [--params f.json] [--path P] [--note N] [--replaces run]"
    )
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first().map(String::as_str) else { return usage() };
    if args.len() < 2 {
        return usage();
    }
    let db = Path::new(&args[1]);
    let duck = Duck::open(db);
    match cmd {
        "init" => init(&duck),
        "load" => load_cmd(&duck, &args[2..]),
        "corpus" => corpus(&duck, args.get(2).map(String::as_str)),
        "tales" if args.len() >= 6 => {
            let train: Vec<PathBuf> = args[5..].iter().map(PathBuf::from).collect();
            tales::run(&duck, Path::new(&args[2]), Path::new(&args[3]), Path::new(&args[4]), &train)
        }
        "export" if args.len() >= 3 => {
            let out = export::export(&duck, &export::filters(&args[2..])?)?;
            match std::io::stdout().lock().write_all(out.as_bytes()) {
                // `| head` closed the pipe — not an error
                Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => Ok(()),
                r => Ok(r?),
            }
        }
        "verify" => verify(&duck, args.get(2).map(String::as_str)),
        "stats" => {
            print!("{}", duck.run(STATS, true, Some("markdown"))?);
            Ok(())
        }
        "migrate" => {
            duck.exec(prag::migrate_sql(SCHEMA)?)?;
            println!("database {} — schema v{}", duck.db.display(), schema_version(&duck)?);
            Ok(())
        }
        "prag" if args.len() >= 3 => prag_cmd(&duck, &args[2..]),
        "qa" if args.len() >= 3 => qa_cmd(&duck, &args[2..]),
        "sql" if args.len() == 3 => {
            let sql = std::fs::read_to_string(&args[2]).with_context(|| args[2].clone())?;
            print!("{}", duck.run(&sql, true, Some("markdown"))?);
            Ok(())
        }
        _ => usage(),
    }
}

fn init(duck: &Duck) -> Result<()> {
    if duck.db.exists() {
        bail!("{} already exists — init does not overwrite a database", duck.db.display());
    }
    if let Some(d) = duck.db.parent() {
        std::fs::create_dir_all(d)?;
    }
    duck.exec(SCHEMA)?;
    println!("database {} — schema v{}", duck.db.display(), schema_version(duck)?);
    Ok(())
}

fn load_cmd(duck: &Duck, a: &[String]) -> Result<()> {
    if a.len() < 3 {
        return usage();
    }
    let mut o = Opts::new(&a[0], &a[1])?;
    let mut files = Vec::new();
    let mut i = 2;
    while i < a.len() {
        let flag = a[i].as_str();
        if let Some(name) = flag.strip_prefix("--") {
            let v = a.get(i + 1).with_context(|| format!("{flag}: missing value"))?.clone();
            match name {
                "kind" => o.kind = v,
                "docs" => o.docs = Docs::parse(&v)?,
                "match" => o.matching = Match::parse(&v)?,
                "run" => o.run = Some(v),
                "layer" => o.layer = v,
                "lang" => o.lang = v,
                "origin" => o.origin = Some(v),
                "method" => o.method = Some(v),
                "license" => o.license = Some(v),
                "genre" => o.genre = Some(v),
                "tool" => o.tool = Some(v),
                _ => bail!("unknown option {flag}"),
            }
            i += 2;
        } else {
            files.push(PathBuf::from(flag));
            i += 1;
        }
    }
    if files.is_empty() {
        bail!("no files");
    }
    let reg = en::udreg::Registry::load();
    for f in &files {
        println!("{}", load::load_file(duck, &reg, &o, f)?);
    }
    Ok(())
}

/// The whole corpus per the manifest: rows "kind, source, dialect, documents, matching, glob". Order matters:
/// texts first (treebanks, corpus), then annotations on existing sentences (calibration).
fn corpus(duck: &Duck, manifest: Option<&str>) -> Result<()> {
    let text = match manifest {
        Some(p) => std::fs::read_to_string(p).with_context(|| p.to_string())?,
        None => CORPUS.to_string(),
    };
    let reg = en::udreg::Registry::load();
    let (mut n, mut skipped) = (0usize, 0usize);
    for (no, line) in text.lines().enumerate() {
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() != 6 {
            bail!("corpus.tsv:{}: {} columns instead of 6", no + 1, c.len());
        }
        let mut o = Opts::new(c[1], c[2])?;
        o.kind = c[0].to_string();
        if c[3] != "-" {
            o.docs = Docs::parse(c[3])?;
        }
        o.matching = Match::parse(c[4])?;
        let files = util::glob(c[5])?;
        if files.is_empty() {
            bail!("corpus.tsv:{}: no files match \"{}\"", no + 1, c[5]);
        }
        for f in files {
            let r = load::load_file(duck, &reg, &o, &f)?;
            if r.skipped {
                skipped += 1;
            } else {
                n += 1;
            }
            println!("{r}");
        }
    }
    println!("corpus: files loaded {n}, already present {skipped}");
    Ok(())
}

fn verify(duck: &Duck, only: Option<&str>) -> Result<()> {
    let v = export::verify(duck, only)?;
    for n in &v.notes {
        println!("NOTE {n}");
    }
    for b in &v.bad {
        println!("FAIL {b}");
    }
    println!(
        "VERIFY files {}: byte for byte {}, counters {}, size as in card {} of {}",
        v.files, v.bytes_ok, v.counts_ok, v.card_ok, v.card_checked
    );
    if !v.bad.is_empty() || v.files == 0 {
        std::process::exit(1);
    }
    Ok(())
}

/// Database schema version (`schema_info`).
fn schema_version(duck: &Duck) -> Result<i64> {
    let out = duck.run("SELECT max(version) FROM schema_info;", true, Some("list"))?;
    out.lines().filter_map(|l| l.trim().parse::<i64>().ok()).next_back().context("schema_info is empty")
}

/// `db prag <db> <labels.tsv> --run … --source …` — the pragmatics layer from a TSV of crate `prag`.
fn prag_cmd(duck: &Duck, a: &[String]) -> Result<()> {
    let labels = PathBuf::from(&a[0]);
    let mut kv = std::collections::BTreeMap::new();
    let mut i = 1;
    while i < a.len() {
        let k = a[i].strip_prefix("--").with_context(|| format!("unexpected argument {}", a[i]))?;
        let v = a.get(i + 1).with_context(|| format!("--{k}: missing value"))?;
        if !matches!(k, "run" | "source" | "tool" | "origin" | "method" | "params" | "path" | "note") {
            bail!("unknown option --{k}");
        }
        kv.insert(k.to_string(), v.clone());
        i += 2;
    }
    let run = kv.get("run").context("--run is required")?.clone();
    let source = kv.get("source").context("--source is required")?.clone();
    let tool = kv.get("tool").cloned().unwrap_or_else(|| if source == "silver" { "opus".into() } else { "rust".into() });
    let params: serde_json::Value = match kv.get("params") {
        Some(p) => serde_json::from_str(&std::fs::read_to_string(p).with_context(|| p.clone())?).with_context(|| p.clone())?,
        None => serde_json::json!({}),
    };
    let o = prag::Opts {
        origin: kv.get("origin").cloned().unwrap_or_else(|| run.clone()),
        run,
        source,
        tool,
        method: kv.get("method").cloned(),
        params,
        path: kv.get("path").cloned(),
        note: kv.get("note").cloned(),
    };
    let (run_id, n) = prag::load(duck, &labels, &o)?;
    println!("pragmatics: run {} (run {run_id}), sets and rows {n}", o.run);
    Ok(())
}

/// `db qa <db> <qa.tsv> --run … [--source opus]` — training questions and answers from a TSV of crate `qagen`.
fn qa_cmd(duck: &Duck, a: &[String]) -> Result<()> {
    let file = PathBuf::from(&a[0]);
    let mut kv = std::collections::BTreeMap::new();
    let mut i = 1;
    while i < a.len() {
        let k = a[i].strip_prefix("--").with_context(|| format!("unexpected argument {}", a[i]))?;
        let v = a.get(i + 1).with_context(|| format!("--{k}: missing value"))?;
        if !matches!(k, "run" | "source" | "tool" | "params" | "path" | "note" | "replaces") {
            bail!("unknown option --{k}");
        }
        kv.insert(k.to_string(), v.clone());
        i += 2;
    }
    let run = kv.get("run").context("--run is required")?.clone();
    let source = kv.get("source").cloned().unwrap_or_else(|| "opus".into());
    let params: serde_json::Value = match kv.get("params") {
        Some(p) => serde_json::from_str(&std::fs::read_to_string(p).with_context(|| p.clone())?).with_context(|| p.clone())?,
        None => serde_json::json!({}),
    };
    let o = qa::Opts {
        tool: kv.get("tool").cloned().unwrap_or_else(|| format!("qagen {source}")),
        run,
        source,
        params,
        path: kv.get("path").cloned(),
        note: kv.get("note").cloned(),
        replaces: kv.get("replaces").cloned(),
    };
    let (run_id, n) = qa::load(duck, &file, &o)?;
    match &o.replaces {
        Some(old) => println!("questions: run {} (run {run_id}), rows {n}, each replaces a row of run {old}", o.run),
        None => println!("questions: run {} (run {run_id}), rows {n}", o.run),
    }
    Ok(())
}
