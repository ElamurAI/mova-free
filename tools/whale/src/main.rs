//! `mova-whale` CLI. Every run subcommand reads one trusted JSON configuration file; `serve` and
//! `ask` speak English on a private socket.
use mova_whale::{agent, animal, whale, swarm, Result};
use serde_json::Value;
use std::{path::Path, process::ExitCode};

const USAGE: &str = "usage: mova-whale whale|whale-prepare|whale-status|whale-project-lengths|whale-swarm|whale-swarm-status|whale-discover|whale-research|whale-research-status|crab CONFIG [--once]
       mova-whale serve CONFIG DEPUTY_CONFIG SOCKET   answer English prompts on a private socket (STOP ends it)
       mova-whale ask SOCKET ID [urgent] < text       send arbitrary English; prints the English answer, then JSON
       mova-whale paper CONFIG [--no-terminal]       one Whale reads one paper, writes its own description, asks its private terminal (or journals the questions only)
       mova-whale learn CONFIG [--no-terminal]       one Whale learns on one paper, round after round, until its description passes the gates (or a limit, a stall, the round limit or the deadline)
       mova-whale learn CONFIG --replay ROOT...       the same, measured: a replay terminal answers with the answers recorded in earlier state or paper run roots; nothing is asked
       mova-whale learn-report STATE_ROOT...          per paper: rounds, words, rounds without gain, questions, terminal calls, facts kept per answer (read-only)
       mova-whale extraction-check TEXT...            a paper text's extraction quality (spacing residue, unknown words, truncated paragraphs, no headings) against the default limits; WHALE_PROSE=1 adds the prose words Mova reads
       mova-whale batch-plan CONFIG                   close finished assignments and give every free Whale its next paper (one batch each); writes the learners' configs
       mova-whale batch CONFIG [--no-terminal]        the rolling runner (one per run root): each Whale with its own terminal takes its next paper as soon as its paper is final; unfinished learners continue after wait_minutes; a usage limit pauses all; STOP stops between rounds. --no-terminal: one dry pass
       mova-whale paper-check CONFIG...               validate Whale paper configs side by side (no shared run root or terminal profile, session, owner marker or root)
       mova-whale epoch CONFIG [--epoch N] [--doc ID] [--no-terminal]  one epoch of texts: per document Mova writes a long cited text from the paper and its neighbourhood, then the Whale's terminal writes from the given sources by path (or Mova only)
       mova-whale epoch-status CONFIG [--epoch N]      every document's state in an epoch (read-only)";

fn config(p: &str) -> Result<Value> {
    if !Path::new(p).is_absolute() {
        return Err("CONFIG must be an absolute path".into());
    }
    mova_whale::util::load(Path::new(p), 2 << 20)
}
/// `--epoch N`, `--doc ID` and `--no-terminal`, in any order.
fn epoch_flags<'a>(default: u64, rest: &[&'a str]) -> Result<(u64, bool, Option<&'a str>)> {
    let (mut epoch, mut live, mut only, mut i) = (default, true, None, 0);
    while i < rest.len() {
        match rest[i] {
            "--no-terminal" => live = false,
            "--doc" if i + 1 < rest.len() => {
                only = Some(rest[i + 1]);
                i += 1;
            }
            "--epoch" if i + 1 < rest.len() => {
                epoch = rest[i + 1].parse().map_err(|_| "--epoch needs a number")?;
                i += 1;
            }
            _ => return Err(USAGE.into()),
        }
        i += 1;
    }
    Ok((epoch, live, only))
}
fn main_result() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    match a.as_slice() {
        ["serve", config, deputy, socket] => {
            for p in [config, deputy, socket] {
                if !Path::new(p).is_absolute() {
                    return Err("serve paths must be absolute".into());
                }
            }
            return animal::serve(Path::new(config), Path::new(deputy), Path::new(socket)).map_err(|e| format!("{e:#}").into());
        }
        ["ask", socket, id, rest @ ..] if rest.is_empty() || rest == ["urgent"] => {
            let mut prompt = String::new();
            std::io::Read::read_to_string(&mut std::io::Read::take(std::io::stdin(), 8193), &mut prompt)?;
            let v = animal::ask(Path::new(socket), id, &prompt, !rest.is_empty()).map_err(|e| format!("{e:#}"))?;
            match v["report"]["answer"].as_str() {
                Some(a) => println!("{a}"),
                None => println!("{}", v["error"].as_str().unwrap_or("no answer")),
            }
            println!("{v}");
            return if v["ok"] == true { Ok(()) } else { Err("the request failed".into()) };
        }
        ["paper", config, "--no-terminal"] => {
            let c = mova_whale::paper::PaperConfig::load(Path::new(config))?;
            let m = mova_whale::paper::run(&c, &mut mova_whale::terminal::NoTerminal(c.whale.clone()))?;
            println!("{m}");
            return Ok(());
        }
        ["paper", config] => {
            let c = mova_whale::paper::PaperConfig::load(Path::new(config))?;
            let mut t = mova_whale::terminal::MovaTerminal::open(&c.terminal)?;
            println!("{}", mova_whale::paper::run(&c, &mut t)?);
            return Ok(());
        }
        ["learn", config, rest @ ..] if rest.is_empty() || rest == ["--no-terminal"] => {
            let c = mova_whale::learn::LearnConfig::load(Path::new(config))?;
            let st = if rest.is_empty() {
                let mut t = mova_whale::terminal::MovaTerminal::open(&c.terminal)?;
                mova_whale::learn::run(&c, &mut t)?
            } else {
                mova_whale::learn::run(&c, &mut mova_whale::terminal::NoTerminal(c.whale.clone()))?
            };
            println!("{st}");
            return Ok(());
        }
        ["learn", config, "--replay", roots @ ..] if !roots.is_empty() => {
            let c = mova_whale::learn::LearnConfig::load(Path::new(config))?;
            let roots: Vec<std::path::PathBuf> = roots.iter().map(std::path::PathBuf::from).collect();
            let mut t = mova_whale::terminal::Replay::load(&c.whale, &roots)?;
            let st = mova_whale::learn::run(&c, &mut t)?;
            println!("{}", serde_json::json!({"status":st,"replay":{"recorded_answers":t.answers.len(),"asked":t.asked,"found":t.found}}));
            return Ok(());
        }
        ["learn-report", roots @ ..] if !roots.is_empty() => {
            for r in roots {
                println!("{}", mova_whale::learn::report(Path::new(r))?);
            }
            return Ok(());
        }
        ["extraction-check", texts @ ..] if !texts.is_empty() => {
            let limits = mova_whale::reader::ExtractionLimits::default();
            for p in texts {
                let text = std::fs::read_to_string(p)?;
                let m = mova_whale::reader::extraction(&text);
                let prose = if std::env::var_os("WHALE_PROSE").is_some() { mova_whale::batch::prose_words(&text).ok() } else { None };
                println!("{}", serde_json::json!({"text":p,"broken":m.broken(&limits),"extraction":m,"prose_words":prose}));
            }
            return Ok(());
        }
        ["batch-plan", config] => {
            let c = mova_whale::batch::BatchConfig::load(Path::new(config))?;
            println!("{}", mova_whale::batch::plan(&c)?);
            return Ok(());
        }
        ["batch", config, rest @ ..] if rest.is_empty() || rest == ["--no-terminal"] => {
            let c = mova_whale::batch::BatchConfig::load(Path::new(config))?;
            println!("{}", mova_whale::batch::run(&c, rest.is_empty())?);
            return Ok(());
        }
        ["epoch", config, rest @ ..] => {
            let c = mova_whale::epoch::EpochConfig::load(Path::new(config))?;
            let (epoch, live, only) = epoch_flags(c.epoch, rest)?;
            println!("{}", mova_whale::epoch::run_cli(&c, epoch, only, live)?);
            return Ok(());
        }
        ["epoch-status", config, rest @ ..] => {
            let c = mova_whale::epoch::EpochConfig::load(Path::new(config))?;
            let (epoch, _, _) = epoch_flags(c.epoch, rest)?;
            println!("{}", mova_whale::epoch::status(&c, epoch)?);
            return Ok(());
        }
        ["paper-check", configs @ ..] if !configs.is_empty() => {
            let cs = configs.iter().map(|p| mova_whale::paper::PaperConfig::load(Path::new(p))).collect::<Result<Vec<_>>>()?;
            mova_whale::paper::check_pod(&cs)?;
            println!("{} Whale configs: each names its own run root and its own terminal (profile, session, owner marker, root)", cs.len());
            return Ok(());
        }
        _ => {}
    }
    let once = match args.get(2).map(String::as_str) {
        None => false,
        Some("--once") if args.len() == 3 => true,
        _ => return Err(USAGE.into()),
    };
    let (Some(cmd), Some(path)) = (args.first(), args.get(1)) else {
        return Err(USAGE.into());
    };
    let c = config(path)?;
    // The subcommands' names before the Whale rename (`shark`, `shark-swarm`, ...) are accepted
    // for one release.
    let cmd = cmd.strip_prefix("shark").map_or_else(|| cmd.clone(), |rest| format!("whale{rest}"));
    match cmd.as_str() {
        "whale" => whale::run(&c, once)?,
        "whale-prepare" => println!("{}", whale::prepare(&c)?),
        "whale-status" => println!("{}", whale::status(&c)?),
        "whale-project-lengths" => println!("{}", whale::project_quarantined_lengths(&c)?),
        "whale-swarm" => swarm::run(&c, once)?,
        "whale-swarm-status" => println!("{}", swarm::status(&c)?),
        "whale-discover" => println!("{}", swarm::discover_preview(&c)?),
        "whale-research" => println!("{}", agent::run(&c)?),
        "whale-research-status" => println!("{}", agent::status(&c)?),
        "crab" => mova_whale::roles::serve(&c)?,
        _ => return Err(USAGE.into()),
    }
    Ok(())
}
fn main() -> ExitCode {
    match main_result() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("mova-whale: {e}");
            ExitCode::FAILURE
        }
    }
}
