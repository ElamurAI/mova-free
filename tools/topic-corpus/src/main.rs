//! `mova-topic-corpus` — topic graphs and closed topic corpora for the paper-describing Whales.
use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use topic_corpus::config::Config;
use topic_corpus::corpus::{self, Corpus, Options};
use topic_corpus::library::{Library, RefEntry};
use topic_corpus::{cite, graph, hardcases, missing, seed, sol, web, world};

const USAGE: &str = "usage: mova-topic-corpus COMMAND CONFIG …
  graph CONFIG PAPER                         the paper's neighbourhood (JSON)
  missing CONFIG PAPER [--sol DRAFT] [--date YYYY-MM-DD] [--dry]
                                             rank missing cited works; write a new request run dir
  build CONFIG PAPER                         register the cross corpus; parse the hot neighbourhood
  explain CONFIG PAPER TERM [K]              sentences of neighbour papers explaining TERM
  world CONFIG PAPER                         the paper's small world (<corpus>/world.json)
  seed CONFIG PAPER WHALE STATE_ROOT [TERM…] retain term facts from the corpus in a Whale's knowledge
  web-plan CONFIG PAPER OUT_QUEUE [--sol DRAFT] [--urls FILE]
                                             Sea Spider's queue: author pages, claim pages, given URLs
  web-ask CONFIG QUEUE ID                    ask Sea Spider (as caller whale) to fetch the queue
  web-import CONFIG PAPER QUEUE              register the pages Sea Spider stored (internal only)
  cite CONFIG PAPER ROUND_DIR STATE_ROOT WHALE OUT [--public]
                                             a cited description with a bibliography
  sol-prompt CONFIG PAPER [WORDS]            the draft-article request
  sol-draft CONFIG PAPER PROFILE DRAFT_ID [WORDS]
                                             ask the Owl's own terminal for a draft (live)
  sol-import CONFIG PAPER DRAFT_ID FILE      register a draft text from a file
  sol-claims CONFIG PAPER DRAFT_ID           claims → citations → sources (<corpus>/sol/)
  public-export CONFIG PAPER OUT [DOC…]      annotations of public-admissible documents only
  hard-cases-recheck CONFIG                  mark open hard cases the parser now reads
  bundle CONFIG CORPUS DOC                   a document's sentences and trees as a hard-case bundle (JSON)";

fn print(v: &Value) {
    println!("{}", serde_json::to_string_pretty(v).unwrap());
}

fn lib_and_graph(c: &Config, paper: &str) -> Result<(Library, graph::Neighbourhood)> {
    let lib = Library::load(&c.library)?;
    let g = graph::build(&lib, paper, &c.limits)?;
    Ok((lib, g))
}

fn open(c: &Config, paper: &str) -> Result<Corpus> {
    Corpus::open(&c.corpora_root, paper, paper, Options { max_sentences: c.limits.max_sentences, cache_max_bytes: c.limits.cache_max_bytes, hard_cases: c.hard_cases.clone() })
}

/// An existing corpus of a paper (built before).
fn existing(c: &Config, paper: &str) -> Result<Corpus> {
    let dir = Corpus::dir_of(&c.corpora_root, paper)?;
    if !dir.join("corpus.json").exists() {
        bail!("no corpus for {paper} under {}: run `build` first", c.corpora_root.display());
    }
    open(c, paper)
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned()
}

fn requests(c: &Config, paper: &str, draft: &str) -> Result<Value> {
    let p = Corpus::dir_of(&c.corpora_root, paper)?.join("sol").join(format!("{draft}-requests.json"));
    Ok(serde_json::from_slice(&std::fs::read(&p).with_context(|| format!("{} (run sol-claims first)", p.display()))?)?)
}

fn run(args: &[String]) -> Result<()> {
    if args.len() < 2 {
        bail!("{USAGE}");
    }
    let cmd = args[0].as_str();
    let cfg = Config::load(Path::new(&args[1]))?;
    let a = &args[2..];
    let need = |n: usize| -> Result<()> {
        if a.len() < n {
            bail!("{USAGE}");
        }
        Ok(())
    };
    match cmd {
        "graph" => {
            need(1)?;
            let (_, g) = lib_and_graph(&cfg, &a[0])?;
            print(&serde_json::to_value(&g)?);
        }
        "missing" => {
            need(1)?;
            let d = cfg.downloads.as_ref().context("the configuration has no `downloads`")?;
            let (lib, g) = lib_and_graph(&cfg, &a[0])?;
            let mut extra: BTreeMap<String, RefEntry> = BTreeMap::new();
            if let Some(draft) = flag(a, "--sol") {
                extra = serde_json::from_value(requests(&cfg, &g.paper.id, &draft)?["downloads"].clone())?;
            }
            let cands = missing::candidates_with(&lib, &g, &d.papers_bin, &extra)?;
            if a.iter().any(|x| x == "--dry") {
                print(&json!({"paper": g.paper.id, "candidates": cands}));
            } else {
                let date = flag(a, "--date").unwrap_or_else(missing::today);
                let (dir, n) = missing::write_request(&lib, &g, d, &cands, &date, missing::MIN_FREE_GB)?;
                print(&json!({"request": dir, "queued": n, "candidates": cands.len(), "start": format!("cd {} && setsid nohup bash run.sh >> logs-nohup.out 2>&1 < /dev/null &", dir.display())}));
            }
        }
        "build" => {
            need(1)?;
            let (lib, g) = lib_and_graph(&cfg, &a[0])?;
            let mut c = open(&cfg, &g.paper.id)?;
            corpus::write_private(&c.dir.join("graph.json"), &serde_json::to_vec_pretty(&g)?)?;
            print(&corpus::build(&mut c, &lib, &g, cfg.limits.max_docs, cfg.limits.hot)?);
        }
        "explain" => {
            need(2)?;
            let mut c = existing(&cfg, &a[0])?;
            let k = a.get(2).and_then(|x| x.parse().ok()).unwrap_or(5);
            let e = corpus::explain(&mut c, &a[1], Some(&format!("lib:{}", a[0])), &["paper", "author", "web"], k)?;
            print(&serde_json::to_value(e)?);
        }
        "world" => {
            need(1)?;
            let mut c = existing(&cfg, &a[0])?;
            let v = world::build(&mut c, 40)?;
            print(&json!({"world": c.dir.join("world.json"), "sentences": v["sentences"], "kinds": v["kinds"], "entities": v["entities"].as_array().map(|x| x.len())}));
        }
        "seed" => {
            need(3)?;
            let mut c = existing(&cfg, &a[0])?;
            let lib = Library::load(&cfg.library)?;
            let w = lib.get(&a[0]).context("paper not in the library")?;
            let (text, _) = lib.text(w).context("no text for the paper")?;
            let md = std::fs::read_to_string(text)?;
            let terms: Vec<String> = a[3..].to_vec();
            print(&seed::seed(&mut c, &seed::SeedArgs { whale: &a[1], state_root: Path::new(&a[2]), paper: &a[0], paper_text: &md, max_terms: 24, terms: &terms })?);
        }
        "web-plan" => {
            need(2)?;
            let (_, g) = lib_and_graph(&cfg, &a[0])?;
            let mut extra = vec![];
            if let Some(f) = flag(a, "--urls") {
                extra.extend(web::load_targets(Path::new(&f))?);
            }
            if let Some(draft) = flag(a, "--sol") {
                extra.extend(serde_json::from_value::<Vec<web::Target>>(requests(&cfg, &g.paper.id, &draft)?["pages"].clone())?);
            }
            let t = web::plan(&g, &extra);
            let out = PathBuf::from(&a[1]);
            if out.exists() {
                bail!("{} exists", out.display());
            }
            std::fs::write(&out, web::queue(&t))?;
            print(&json!({"queue": out, "pages": t.len(), "prompts": web::prompts(&t)}));
        }
        "web-ask" => {
            need(2)?;
            let s = cfg.seaspider.as_ref().and_then(|s| s.socket.clone()).context("the configuration has no seaspider.socket")?;
            let t = web::load_targets(Path::new(&a[0]))?;
            print(&json!(web::ask(&s, &a[1], &t)?));
        }
        "web-import" => {
            need(2)?;
            let s = cfg.seaspider.as_ref().context("the configuration has no seaspider")?;
            let mut c = existing(&cfg, &a[0])?;
            let t = web::load_targets(Path::new(&a[1]))?;
            print(&web::import(&mut c, &s.store, &t)?);
        }
        "cite" => {
            need(5)?;
            let c = existing(&cfg, &a[0])?;
            let out = PathBuf::from(&a[4]);
            if out.exists() {
                bail!("{} exists", out.display());
            }
            let r = cite::cite(&c, &a[0], Path::new(&a[1]), Path::new(&a[2]), &a[3], a.iter().any(|x| x == "--public"))?;
            std::fs::write(&out, &r.markdown)?;
            print(&json!({"out": out, "sentences": r.sentences.len(), "sources": r.sources}));
        }
        "sol-prompt" => {
            need(1)?;
            let (_, g) = lib_and_graph(&cfg, &a[0])?;
            println!("{}", sol::prompt(&g, a.get(1).and_then(|x| x.parse().ok()).unwrap_or(3000)));
        }
        "sol-draft" => {
            need(3)?;
            let (_, g) = lib_and_graph(&cfg, &a[0])?;
            let mut c = existing(&cfg, &a[0])?;
            let tc = mova_whale::terminal::TerminalConfig { profile: PathBuf::from(&a[1]), poll_ms: 2000 };
            let mut t = mova_whale::terminal::MovaTerminal::open(&tc).map_err(|e| anyhow::anyhow!("{e}"))?;
            // The terminal is held for the whole question: refused while its Owl holds it.
            let _lease = mova_whale::terminal::TerminalLease::take(&t.profile).map_err(|e| anyhow::anyhow!("{e}"))?;
            let text = sol::draft(&mut c, &mut t, &g, &a[2], a.get(3).and_then(|x| x.parse().ok()).unwrap_or(3000))?;
            print(&json!({"draft": format!("sol:{}", a[2]), "words": text.split_whitespace().count()}));
        }
        "sol-import" => {
            need(3)?;
            let mut c = existing(&cfg, &a[0])?;
            let text = std::fs::read_to_string(&a[2])?;
            sol::store(&mut c, &a[1], &format!("Draft article for {}", a[0]), &text, json!({"imported_from": a[2]}))?;
            print(&json!({"draft": format!("sol:{}", a[1])}));
        }
        "sol-claims" => {
            need(2)?;
            let lib = Library::load(&cfg.library)?;
            let mut c = existing(&cfg, &a[0])?;
            let (cl, req) = sol::claims(&mut c, &lib, &a[1])?;
            print(&sol::save(&c, &a[1], &cl, &req)?);
        }
        "public-export" => {
            need(2)?;
            let mut c = existing(&cfg, &a[0])?;
            print(&corpus::public_export(&mut c, Path::new(&a[1]), &a[2..])?);
        }
        "bundle" => {
            need(2)?;
            let mut c = existing(&cfg, &a[0])?;
            let d = c.doc(&a[1]).with_context(|| format!("{} is not in the corpus", a[1]))?.clone();
            let sents = c.parsed(&a[1])?;
            print(&hardcases::bundle(&a[0], &d, &sents));
        }
        "hard-cases-recheck" => {
            let h = cfg.hard_cases.clone().context("the configuration has no hard_cases")?;
            let (open, learned) = hardcases::recheck(&h, corpus::READER, &corpus::sentence_issues)?;
            print(&json!({"open": open, "learned": learned}));
        }
        _ => bail!("{USAGE}"),
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Err(e) = run(&args) {
        eprintln!("mova-topic-corpus: {e:#}");
        std::process::exit(1);
    }
}
