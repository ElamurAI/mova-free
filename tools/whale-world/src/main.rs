//! `mova-whale-world`: the dossier and global-world command line. Every command prints one JSON
//! value (`world neighbourhood --content` prints JSON lines).
use anyhow::{bail, Context, Result};
use serde_json::{json, Value};
use std::path::Path;
use whale_world::config::Config;
use whale_world::dossier::{self, Dossier, Kind, NewText};
use whale_world::{query, world::World};

const USAGE: &str = "usage:
  mova-whale-world dossier import CONFIG DOC...
  mova-whale-world dossier add CONFIG DOC --epoch E --kind K --file PATH --source S [--licence L] [--input TEXT_ID]... [--created UNIX]
  mova-whale-world dossier list CONFIG DOC [--epoch E] [--kind K]
  mova-whale-world dossier verify CONFIG DOC
  mova-whale-world dossier text CONFIG TEXT_ID
  mova-whale-world world build CONFIG [--epoch E] [DOC...]
  mova-whale-world world stats CONFIG [--epoch E]
  mova-whale-world world entity CONFIG TERM [--epoch E]
  mova-whale-world world explain CONFIG TERM [K] [--offset N] [--epoch E]
  mova-whale-world world related CONFIG DOC [K] [--offset N] [--epoch E]
  mova-whale-world world neighbourhood CONFIG DOC [--radius R] [--max-docs N] [--epoch E] [--content]
  mova-whale-world world sentences CONFIG WORD [K] [--offset N] [--epoch E]
  mova-whale-world world context CONFIG DOC [--radius R] [--max-docs N] [--max-bytes N] [--epoch E]
  mova-whale-world export-texts EXPORT_CONFIG OUT_REPO [DOC...]";

/// Positional arguments and `--flag value` options (`--content` takes no value).
struct Args {
    pos: Vec<String>,
    opts: Vec<(String, String)>,
    flags: Vec<String>,
}

fn parse(a: &[String]) -> Result<Args> {
    let mut r = Args { pos: vec![], opts: vec![], flags: vec![] };
    let mut i = 0;
    while i < a.len() {
        if let Some(f) = a[i].strip_prefix("--") {
            if f == "content" {
                r.flags.push(f.into());
            } else {
                let v = a.get(i + 1).with_context(|| format!("--{f} needs a value"))?;
                r.opts.push((f.into(), v.clone()));
                i += 1;
            }
        } else {
            r.pos.push(a[i].clone());
        }
        i += 1;
    }
    Ok(r)
}

impl Args {
    fn opt(&self, k: &str) -> Option<&str> {
        self.opts.iter().find(|x| x.0 == k).map(|x| x.1.as_str())
    }
    fn num(&self, k: &str) -> Result<Option<u64>> {
        self.opt(k).map(|v| v.parse::<u64>().with_context(|| format!("--{k} must be a number"))).transpose()
    }
    fn all(&self, k: &str) -> Vec<String> {
        self.opts.iter().filter(|x| x.0 == k).map(|x| x.1.clone()).collect()
    }
}

fn print(v: &Value) {
    println!("{}", serde_json::to_string_pretty(v).unwrap());
}

fn max_epoch(cfg: &Config) -> Result<u64> {
    let mut m = 0;
    for d in Dossier::list(cfg)? {
        for r in Dossier::open(cfg, &d)?.rows()? {
            m = m.max(r.epoch);
        }
    }
    Ok(m + 1)
}

fn main() -> Result<()> {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if argv.first().map(String::as_str) == Some("export-texts") {
        if argv.len() < 3 {
            bail!("{USAGE}");
        }
        let ec = whale_world::export::Config::load(Path::new(&argv[1]))?;
        let cfg = Config::load(&ec.world)?;
        let lib = cfg.library.as_ref().map(topic_corpus::library::Library::load).transpose()?;
        print(&whale_world::export::export(&ec, &cfg, lib.as_ref(), Path::new(&argv[2]), &argv[3..])?);
        return Ok(());
    }
    if argv.len() < 3 {
        bail!("{USAGE}");
    }
    let (group, cmd) = (argv[0].as_str(), argv[1].as_str());
    let cfg = Config::load(Path::new(&argv[2]))?;
    let a = parse(&argv[3..])?;
    let need = |n: usize| -> Result<()> {
        if a.pos.len() < n {
            bail!("{USAGE}");
        }
        Ok(())
    };
    let k = |i: usize, d: usize| -> Result<usize> { a.pos.get(i).map(|x| x.parse::<usize>().context("K must be a number")).transpose().map(|x| x.unwrap_or(d)) };
    match (group, cmd) {
        ("dossier", "import") => {
            need(1)?;
            let lib = cfg.library.as_ref().map(topic_corpus::library::Library::load).transpose()?;
            let mut out = vec![];
            for doc in &a.pos {
                out.push(dossier::import::all(&cfg, lib.as_ref(), doc)?);
            }
            print(&json!(out));
        }
        ("dossier", "add") => {
            need(1)?;
            let d = Dossier::open(&cfg, &a.pos[0])?;
            let file = a.opt("file").context("--file is required")?;
            let text = std::fs::read_to_string(file).with_context(|| file.to_string())?;
            let row = d.add(NewText {
                epoch: a.num("epoch")?.context("--epoch is required")?,
                kind: Kind::parse(a.opt("kind").context("--kind is required")?)?,
                text: &text,
                source: a.opt("source").context("--source is required")?,
                licence: a.opt("licence"),
                local_only: true,
                inputs: a.all("input"),
                created_unix: a.num("created")?,
            })?;
            print(&serde_json::to_value(row)?);
        }
        ("dossier", "list") => {
            need(1)?;
            let d = Dossier::open(&cfg, &a.pos[0])?;
            let kind = a.opt("kind").map(Kind::parse).transpose()?;
            print(&serde_json::to_value(d.select(a.num("epoch")?, kind)?)?);
        }
        ("dossier", "verify") => {
            need(1)?;
            let d = Dossier::open(&cfg, &a.pos[0])?;
            let bad = d.verify()?;
            print(&json!({"doc": a.pos[0], "texts": d.rows()?.len(), "changed": bad}));
            if !bad.is_empty() {
                std::process::exit(1);
            }
        }
        ("dossier", "text") => {
            need(1)?;
            let (doc, _) = a.pos[0].split_once('/').context("TEXT_ID is <doc>/<epoch>-<kind>-<n>")?;
            let d = Dossier::open(&cfg, doc)?;
            let r = d.rows()?.into_iter().find(|r| r.id == a.pos[0]).with_context(|| format!("{} is not in the dossier", a.pos[0]))?;
            print!("{}", d.read(&r)?);
        }
        ("world", "build") => {
            let e = match a.num("epoch")? {
                Some(e) => e,
                None => max_epoch(&cfg)?,
            };
            let mut w = World::open(&cfg)?;
            let t0 = std::time::Instant::now();
            let mut r = w.add(&cfg, e, &a.pos)?;
            r["seconds"] = json!((t0.elapsed().as_secs_f64() * 100.0).round() / 100.0);
            print(&r);
        }
        _ => {
            let mut w = World::open(&cfg)?;
            let e = a.num("epoch")?.unwrap_or(w.meta.epoch);
            let off = a.num("offset")?.unwrap_or(0) as usize;
            let radius = a.num("radius")?.map(|x| x as usize).unwrap_or(query::RADIUS);
            let maxd = a.num("max-docs")?.map(|x| x as usize).unwrap_or(query::MAX_DOCS);
            let v = match (group, cmd) {
                ("world", "stats") => w.stats(e),
                ("world", "entity") => {
                    need(1)?;
                    query::entity(&mut w, &a.pos[0], e)?
                }
                ("world", "explain") => {
                    need(1)?;
                    query::explain(&mut w, &a.pos[0], k(1, 5)?, off, e)?
                }
                ("world", "related") => {
                    need(1)?;
                    query::related(&mut w, &a.pos[0], k(1, 10)?, off, e)?
                }
                ("world", "sentences") => {
                    need(1)?;
                    query::sentences(&mut w, &a.pos[0], k(1, 10)?, off, e)?
                }
                ("world", "neighbourhood") => {
                    need(1)?;
                    if a.flags.iter().any(|f| f == "content") {
                        let mut out = std::io::BufWriter::new(std::io::stdout().lock());
                        query::neighbourhood_content(&mut w, &a.pos[0], radius, maxd, e, &mut |v| {
                            use std::io::Write;
                            writeln!(out, "{v}")?;
                            Ok(())
                        })?;
                        return Ok(());
                    }
                    query::neighbourhood(&mut w, &a.pos[0], radius, maxd, e)?
                }
                ("world", "context") => {
                    need(1)?;
                    let mb = a.num("max-bytes")?.map(|x| x as usize);
                    query::context(&mut w, &a.pos[0], radius, maxd, e, mb)?
                }
                _ => bail!("{USAGE}"),
            };
            print(&v);
        }
    }
    Ok(())
}
