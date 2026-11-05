//! Crab with Mova's own understanding: the single Rust deputy. It serves the usual Crab control
//! socket and additionally answers `op:"ask"` — arbitrary ordinary text — through the native
//! Octopus planner (Mova annotator and world reader). Nested Crab observations, configured
//! worker control (`crab_control`), the consult queue to the Dolphin consultant
//! (`crab_consult`) and work-flow observation (`crab_flow`) run in-process.
use anyhow::{Result, bail};
use mova_crab::angel::{self, NoExternalRoles, PromptHandler};
use mova_octopus::{CapabilityHandler, Config, CrabConsult, CrabControl, CrabFlow, CrabInProcess, Deputy, commands};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    io::{self, Read, Write},
    os::unix::fs::OpenOptionsExt,
    path::Path,
    sync::Arc,
};
struct Understanding(Deputy, Option<std::path::PathBuf>);
impl PromptHandler for Understanding {
    fn ask(
        &self,
        prompt: &str,
        invocation_id: &str,
        execute: bool,
        crab: &dyn Fn(&Value) -> std::result::Result<Value, String>,
    ) -> mova_crab::Result<Value> {
        let inproc = CrabInProcess(crab);
        let control = CrabControl(crab);
        let consult = CrabConsult(crab);
        let flow = CrabFlow(crab);
        let overrides: BTreeMap<String, &dyn CapabilityHandler> = BTreeMap::from([
            ("crab_observe".to_owned(), &inproc as &dyn CapabilityHandler),
            ("crab_control".to_owned(), &control as &dyn CapabilityHandler),
            ("crab_consult".to_owned(), &consult as &dyn CapabilityHandler),
            ("crab_flow".to_owned(), &flow as &dyn CapabilityHandler),
        ]);
        let report = self
            .0
            .run_with(prompt, invocation_id, execute, &overrides)
            .map_err(|e| format!("{e:#}"))?;
        // The command school's corpus: every command, anonymized, with Crab's reading.
        if let Some(corpus) = &self.1 {
            if let Err(e) = mova_octopus::commands::record(corpus, prompt, None, &report) {
                eprintln!("command corpus append failed: {e:#}");
            }
        }
        compact(serde_json::to_value(report)?)
    }
}
/// Fit the reply into Crab's 64 KiB response budget: keep plan, failure and followups; keep
/// result outputs while small, otherwise only their digests. The full report stays in the
/// deputy ledger (run mode).
fn compact(r: Value) -> mova_crab::Result<Value> {
    let mut v = json!({"prompt_sha256":r["prompt_sha256"],"plan":r["plan"],"failure":r["failure"],
        "followups":r["followups"],"results":r["results"],"training_authorized":r["training_authorized"],
        "pragmatic_engine":r["pragmatic_engine"]});
    if serde_json::to_vec(&v)?.len() > 48 * 1024 {
        if let Some(rs) = v["results"].as_array_mut() {
            for x in rs {
                x["output"] = json!({"omitted":"over response budget; see output_sha256 and the deputy ledger"});
            }
        }
    }
    if serde_json::to_vec(&v)?.len() > 48 * 1024 {
        v["followups"] = json!({"omitted":true});
    }
    Ok(v)
}
const USAGE: &str = "usage: crab serve CRAB_CONFIG DEPUTY_CONFIG [COMMAND_CORPUS]\n       crab ask SOCKET REQUEST_ID [urgent] < any-text\n       crab exam DEPUTY_CONFIG CORPUS...\n       crab review-queue OBSERVED [REVIEWED...]\n       crab promote OBSERVED DECISIONS REVIEWED\n       crab exercises DEPUTY_CONFIG NEW_OUT REVIEWED...\n       crab candidates REVIEW_QUEUE\n       crab audit CORPUS";
fn load_all(paths: &[String]) -> Result<Vec<commands::Entry>> {
    let mut v = Vec::new();
    for p in paths {
        v.extend(commands::load(Path::new(p))?);
    }
    Ok(v)
}
fn main() -> Result<()> {
    let a: Vec<String> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("serve") if a.len() == 3 || a.len() == 4 => {
            let crab: Value = serde_json::from_slice(&fs::read(&a[1])?)?;
            let deputy: Config = serde_json::from_slice(&fs::read(&a[2])?)?;
            let understanding =
                Understanding(Deputy::load(deputy)?, a.get(3).map(std::path::PathBuf::from));
            angel::serve_full(&crab, Arc::new(NoExternalRoles), Arc::new(understanding))
                .map_err(|e| anyhow::anyhow!("{e}"))
        }
        Some("ask") if (a.len() == 3 || (a.len() == 4 && a[3] == "urgent")) => {
            let mut prompt = String::new();
            io::stdin().read_to_string(&mut prompt)?;
            let v = angel::ask(Path::new(&a[1]), &a[2], &prompt, a.len() == 4)
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            println!("{}", serde_json::to_string_pretty(&v)?);
            if v["ok"] != true {
                std::process::exit(1);
            }
            Ok(())
        }
        Some("exam") if a.len() >= 3 => {
            let deputy: Config = serde_json::from_slice(&fs::read(&a[1])?)?;
            let entries = load_all(&a[2..])?;
            let report = commands::exam(&Deputy::load(deputy)?, &entries);
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Some("review-queue") if a.len() >= 2 => {
            let observed = commands::load(Path::new(&a[1]))?;
            let reviewed = load_all(&a[2..])?;
            println!("{}", serde_json::to_string_pretty(&commands::review_queue(&observed, &reviewed))?);
            Ok(())
        }
        Some("promote") if a.len() == 4 => {
            let observed = commands::load(Path::new(&a[1]))?;
            let decisions: Vec<commands::Decision> = fs::read_to_string(&a[2])?
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(serde_json::from_str)
                .collect::<std::result::Result<_, _>>()?;
            let target = Path::new(&a[3]);
            let reviewed = if target.exists() { commands::load(target)? } else { vec![] };
            let (admitted, refused) = commands::promote(&observed, &reviewed, &decisions);
            let mut f = fs::OpenOptions::new().create(true).append(true).mode(0o600).open(target)?;
            for e in &admitted {
                let mut line = serde_json::to_vec(e)?;
                line.push(b'\n');
                f.write_all(&line)?;
            }
            f.sync_all()?;
            println!("{}", serde_json::to_string_pretty(&json!({"admitted":admitted.len(),"refused":refused,"training_authorized":false}))?);
            Ok(())
        }
        Some("exercises") if a.len() >= 4 => {
            let deputy: Config = serde_json::from_slice(&fs::read(&a[1])?)?;
            let reviewed = load_all(&a[3..])?;
            let out = commands::exercises(&deputy, &reviewed);
            let mut f = fs::OpenOptions::new().create_new(true).write(true).mode(0o600).open(&a[2])?;
            for e in &out {
                let mut line = serde_json::to_vec(e)?;
                line.push(b'\n');
                f.write_all(&line)?;
            }
            f.sync_all()?;
            println!("{} exercises written", out.len());
            Ok(())
        }
        Some("candidates") if a.len() == 2 => {
            println!("{}", serde_json::to_string_pretty(&commands::group_candidates(Path::new(&a[1]))?)?);
            Ok(())
        }
        Some("audit") if a.len() == 2 => {
            let entries = mova_octopus::commands::load(Path::new(&a[1]))?;
            mova_octopus::commands::audit(&entries)?;
            println!("{} entries anonymized", entries.len());
            Ok(())
        }
        _ => bail!(USAGE),
    }
}
