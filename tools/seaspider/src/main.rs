//! `seaspider` CLI: fetch a queue, read a stored page back, explain a robots.txt decision, count;
//! serve and ask in English as one of Mova's animals.

use seaspider::config::Config;
use seaspider::crawl::{load_queue, Crawler};
use seaspider::singleton;
use seaspider::store::{self, Reader};
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = "usage:
  seaspider fetch CONFIG QUEUE     fetch a queue (JSONL {\"url\":...} or TSV/plain lines), print a summary
  seaspider get STORE URL          write the latest stored body of URL (or sha256:HEX) to stdout
  seaspider robots CONFIG URL      fetch robots.txt politely and explain the decision for URL
  seaspider stats STORE            counts of the journal and segments
  seaspider serve CONFIG DEPUTY_CONFIG SOCKET   answer English prompts on a private socket (STOP ends it)
  seaspider ask [--from CALLER] SOCKET ID [urgent] < text
                                   send arbitrary English; prints the English answer, then JSON
fetch, robots and serve take the host-wide singleton lock (service.lock); a second one exits 5.";

fn runtime() -> Result<tokio::runtime::Runtime, String> {
    tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|e| e.to_string())
}

/// Why the command failed: a refused second start, or anything else.
enum Failure {
    Running(String),
    Other(String),
}

impl From<String> for Failure {
    fn from(e: String) -> Self {
        Failure::Other(e)
    }
}

fn lease(cfg: &Config, role: String) -> Result<singleton::Singleton, Failure> {
    singleton::acquire(&cfg.lock_path(), &role).map_err(|e| match e {
        r @ singleton::Refused::Running { .. } => Failure::Running(r.to_string()),
        r => Failure::Other(r.to_string()),
    })
}

fn run(args: &[String]) -> Result<(), Failure> {
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    match a.as_slice() {
        ["fetch", config, queue] => {
            let (cfg, dir) = Config::load(Path::new(config))?;
            let _lease = lease(&cfg, format!("fetch {queue}"))?;
            let urls = load_queue(Path::new(queue))?;
            let rt = runtime()?;
            let sum = rt.block_on(async {
                let mut c = Crawler::new(cfg, &dir)?;
                c.run(&urls).await
            })?;
            println!("{}", serde_json::to_string_pretty(&sum).map_err(|e| e.to_string())?);
            Ok(())
        }
        ["get", dir, key] => {
            let r = Reader::open(Path::new(dir))?;
            let (rec, body) = r.get(key)?;
            eprintln!("{} {} {} bytes sha256:{}", rec.status.map_or("-".into(), |s| s.to_string()), rec.final_url.as_deref().unwrap_or(&rec.url), body.len(), rec.sha256.as_deref().unwrap_or(""));
            Ok(std::io::stdout().write_all(&body).map_err(|e| e.to_string())?)
        }
        ["robots", config, url] => {
            let (cfg, dir) = Config::load(Path::new(config))?;
            let _lease = lease(&cfg, format!("robots {url}"))?;
            let rt = runtime()?;
            let v = rt.block_on(async {
                let mut c = Crawler::new(cfg, &dir)?;
                c.explain(url).await
            })?;
            println!("{}", serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?);
            Ok(())
        }
        ["stats", dir] => {
            let r = Reader::open(Path::new(dir))?;
            println!("{}", serde_json::to_string_pretty(&store::stats(&r)).map_err(|e| e.to_string())?);
            Ok(())
        }
        ["serve", config, deputy, socket] => seaspider::animal::serve(Path::new(config), Path::new(deputy), Path::new(socket)).map_err(|e| {
            match e.downcast_ref::<singleton::Refused>() {
                Some(r @ singleton::Refused::Running { .. }) => Failure::Running(r.to_string()),
                _ => Failure::Other(format!("{e:#}")),
            }
        }),
        ["ask", "--from", from, socket, id, rest @ ..] if rest.is_empty() || rest == ["urgent"] => ask(socket, Some(from), id, !rest.is_empty()),
        ["ask", socket, id, rest @ ..] if rest.is_empty() || rest == ["urgent"] => ask(socket, None, id, !rest.is_empty()),
        _ => Err(USAGE.to_string().into()),
    }
}

fn ask(socket: &str, from: Option<&str>, id: &str, urgent: bool) -> Result<(), Failure> {
    {
        {
            let mut prompt = String::new();
            std::io::Read::read_to_string(&mut std::io::Read::take(std::io::stdin(), 8193), &mut prompt).map_err(|e| e.to_string())?;
            let v = match from {
                Some(f) => seaspider::animal::ask_from(Path::new(socket), f, id, &prompt, urgent),
                None => seaspider::animal::ask(Path::new(socket), id, &prompt, urgent),
            }
            .map_err(|e| format!("{e:#}"))?;
            match v["report"]["answer"].as_str() {
                Some(a) => println!("{a}"),
                None => println!("{}", v["error"].as_str().unwrap_or("no answer")),
            }
            println!("{v}");
            if v["ok"] != true {
                return Err("the request failed".to_string().into());
            }
            Ok(())
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(Failure::Running(e)) => {
            eprintln!("seaspider: {e}");
            ExitCode::from(singleton::EXIT_RUNNING)
        }
        Err(Failure::Other(e)) => {
            eprintln!("seaspider: {e}");
            ExitCode::FAILURE
        }
    }
}
