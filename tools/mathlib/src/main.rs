//! mathlib — the Mova math library (wave 0: catalog, licenses, sizes, pilot, textbooks).
//!
//! Design note: the original goal was to download hundreds of thousands of math papers so that the SLM can compute at
//! the level of a math student; the same day the scope was narrowed (many papers are not needed now): arXiv metadata
//! stays (a catalog to choose from), textbooks are the priority, a full-text pilot of ≤~200 papers and ≤1 GB.
//!
//!   mathlib oai <out-dir> [--set S] [--prefix arXivRaw] [--from YYYY-MM-DD] [--base URL]
//!               [--delay-ms 4000] [--max-pages N]
//!     OAI-PMH ListRecords → `<out-dir>/pNNNNN.xml.gz` + `state.tsv` (see `oai.rs`).
//!   mathlib parse <out.tsv> <oai-dir>...
//!     arXivRaw → catalog TSV (id, categories, license, year, versions, sizes; see `raw.rs`).
//!   mathlib sql <db> <file.sql> [key=value]... [--mode markdown]
//!     SQL template (`{key}` → value) via the DuckDB CLI: `sql/load.sql` (TSV → tables),
//!     `sql/stats.sql` (statistics), `sql/pilot.sql` (pilot selection → list for fetch).
//!   mathlib fetch <list.tsv> <raw-root> <manifest.jsonl> [--delay-ms 3500] [--max-bytes N]
//!               [--file-max N] [--min-free-gb 150] [--expect-pdf] [--compact-src] [--kind paper|textbook]
//!     Polite download from a list (url, license) → `<raw-root>/<slug>/`, manifest in the
//!     tools/sources format (size, md5 for `sources verify`) + sha256 and license of every file (see `fetch.rs`).
//!   mathlib gcs-list <bucket> <prefix> <out.tsv>
//!     Listing of a public GCS bucket (arXiv mirror `arxiv-dataset`): names and sizes, no downloading.
//!   mathlib books-fetch <catalog-dir> [all|openstax|aim|otl]
//!     Raw pages of textbook catalogs (OpenStax API, AIM, OTL with a 120 s pause; see `books.rs`).
//!   mathlib books-pdf <catalog-dir> <books.tsv>
//!     Search for direct PDFs on the sites of books with no PDF in the catalogs (one page per site) → sites-pdf.tsv.
//!   mathlib books <catalog-dir> <out.tsv>
//!     Offline catalog of open textbooks: license, topic, level, direct PDF, GitHub.

mod books;
mod duck;
mod fetch;
mod gcs;
mod http;
mod oai;
mod raw;
mod robots;

use std::path::Path;
use std::time::Duration;

use anyhow::{Result, bail};

fn flag<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).map(String::as_str)
}

const BOOL_FLAGS: [&str; 2] = ["--expect-pdf", "--compact-src"];

/// Positional arguments: everything that is neither a flag nor a flag's value.
fn positional(args: &[String]) -> Vec<&str> {
    let mut out = Vec::new();
    let mut skip = false;
    for a in args {
        if skip {
            skip = false;
            continue;
        }
        if a.starts_with("--") {
            // flags without a value
            skip = !BOOL_FLAGS.contains(&a.as_str());
            continue;
        }
        out.push(a.as_str());
    }
    out
}

fn num(args: &[String], name: &str, default: u64) -> Result<u64> {
    match flag(args, name) {
        Some(v) => Ok(v.replace('_', "").parse()?),
        None => Ok(default),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = args.first().map(String::as_str).unwrap_or("");
    let rest = &args[1.min(args.len())..];
    let pos = positional(rest);
    match cmd {
        "oai" if pos.len() == 1 => {
            let h = oai::Harvest {
                base: flag(rest, "--base").unwrap_or("https://oaipmh.arxiv.org/oai"),
                set: flag(rest, "--set"),
                prefix: flag(rest, "--prefix").unwrap_or("arXivRaw"),
                from: flag(rest, "--from"),
                delay: Duration::from_millis(num(rest, "--delay-ms", 4000)?.max(3000)),
                max_pages: num(rest, "--max-pages", 0)? as usize,
            };
            oai::run(&h, Path::new(pos[0]))
        }
        "parse" if pos.len() >= 2 => raw::run(Path::new(pos[0]), &pos[1..]),
        "books-fetch" if !pos.is_empty() => books::fetch(Path::new(pos[0]), pos.get(1).copied().unwrap_or("all")),
        "books" if pos.len() == 2 => books::build(Path::new(pos[0]), Path::new(pos[1])),
        "books-pdf" if pos.len() == 2 => books::find_pdfs(Path::new(pos[0]), Path::new(pos[1])),
        "fetch" if pos.len() == 3 => {
            let o = fetch::Opts {
                delay: Duration::from_millis(num(rest, "--delay-ms", 3500)?.max(1000)),
                max_bytes: num(rest, "--max-bytes", 1_000_000_000)?,
                file_max: num(rest, "--file-max", 300_000_000)?,
                // GB as in `df -h` (GiB): threshold — stop when less than 150 GB is free (since 26.09 16:50; 200 before that)
                min_free: num(rest, "--min-free-gb", 150)? * (1 << 30),
                expect_pdf: rest.iter().any(|a| a == "--expect-pdf"),
                compact: rest.iter().any(|a| a == "--compact-src"),
                kind: flag(rest, "--kind").unwrap_or("paper").to_string(),
            };
            fetch::run(Path::new(pos[0]), Path::new(pos[1]), Path::new(pos[2]), &o)
        }
        "gcs-list" if pos.len() == 3 => gcs::list(pos[0], pos[1], Path::new(pos[2])),
        "sql" if pos.len() >= 2 => {
            let sql = std::fs::read_to_string(pos[1])?;
            let vars: Vec<(&str, &str)> = pos[2..].iter().filter_map(|kv| kv.split_once('=')).collect();
            let mode = flag(rest, "--mode");
            print!("{}", duck::run(Path::new(pos[0]), &duck::fill(&sql, &vars), mode)?);
            Ok(())
        }
        _ => bail!("usage: mathlib oai|parse|sql|fetch|gcs-list|books-fetch|books-pdf|books … (see the header of src/main.rs)"),
    }
}
