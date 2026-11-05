//! `nautilus` CLI: draft a layer natively, read its counts, print a teacher question, run the standing loop (teacher
//! questions only through a plugged-in private terminal and the operator's word), and answer English as one of
//! Mova's animals.
use anyhow::{Context, Result, bail};
use nautilus::{animal, layer, teacher};
use std::io::Read;
use std::path::Path;

const USAGE: &str = "usage:
  nautilus make CONFIG LAYER                       draft a configured layer natively (no teacher); prints JSON
  nautilus stats LAYER_TSV                         entries, examples and the draft mark of a layer file
  nautilus teacher-task LAYER_TSV SOURCE [N]       print the teacher task for the first N rows (nothing is sent)
  nautilus work CONFIG TEACHERS PROFILE STATE [STEPS|- [SECS]] the standing loop over the configured layers (STOP in
                                                   STATE ends it; a step every SECS, 30 by default; teacher only with the word)
  nautilus pool-work CONFIG TEACHERS PROFILE MEMBER STATE ASSIGNMENT REPORT LEDGERS [STEPS|- [SECS]]
                                                   a pool member: the job Crab leased in ASSIGNMENT, one step at a
                                                   time; REPORT and LEDGERS/<layer>/{ledger.jsonl,backlog.json} after each
  nautilus job-task CONFIG LAYER                   print the next teacher question of a layer's job (nothing is sent)
  nautilus dry CONFIG LAYER [ITEMS]                the first ITEMS (10) items of a layer's job, natively: words asked,
                                                   settled by Mova and never asked (nothing is sent or written)
  nautilus backlog CONFIG LEDGERS [LAYER]          open teacher questions per layer (native); writes each backlog.json
  nautilus pool-seed STATE LEDGERS LAYER MEMBER    seed a job's ledger once from an instance's own work journal
  nautilus replay CONFIG OUT TERMINAL_ROOT...      measure the loop on copies with earlier real answers (nothing is sent)
  nautilus work-summary STATE                      events, throughput, rejection and hold reasons
  nautilus work-resume STATE                       continue after a limit has passed
  nautilus plan DEPUTY_CONFIG CONFIG ID < text     read English, run it with Nautilus's hands, print the answer
  nautilus serve DEPUTY_CONFIG CONFIG SOCKET LOCK       answer English on a private socket (one per LOCK; STOP ends it)
  nautilus ask SOCKET ID [urgent] < text           send English to a serving Nautilus";

fn stdin() -> Result<String> {
    let mut p = String::new();
    std::io::stdin().take(8193).read_to_string(&mut p)?;
    Ok(p)
}

fn rows_of(path: &str) -> Result<Vec<layer::Row>> {
    Ok(layer::parse_rows(&std::fs::read_to_string(path).with_context(|| path.to_string())?))
}

fn deputy(path: &str, cfg: &animal::Config) -> Result<mova_octopus::Deputy> {
    let d: mova_octopus::Config = serde_json::from_slice(&std::fs::read(path)?)?;
    animal::check_capabilities(&d, cfg).map_err(anyhow::Error::msg)?;
    mova_octopus::Deputy::load(d)
}

fn run(a: &[String]) -> Result<()> {
    let a: Vec<&str> = a.iter().map(String::as_str).collect();
    match a.as_slice() {
        ["make", config, name] => {
            let c = animal::Config::load(Path::new(config))?;
            let l = c.layers.get(*name).with_context(|| format!("no layer {name} in {config}"))?;
            println!("{}", serde_json::to_string_pretty(&animal::make(l)?)?);
        }
        ["stats", file] => println!("{}", serde_json::to_string_pretty(&layer::stats(&std::fs::read_to_string(file)?))?),
        ["teacher-task", file, source, rest @ ..] if rest.len() <= 1 => {
            let n: usize = rest.first().map(|x| x.parse()).transpose()?.unwrap_or(usize::MAX);
            let rows = rows_of(file)?;
            let (t, k) = teacher::dictionary_task(source, &rows[..rows.len().min(n)]);
            eprintln!("{k} rows, {} bytes", t.len());
            print!("{t}");
        }
        ["work", config, teachers, profile, state, rest @ ..] if rest.len() <= 2 => {
            let c = animal::Config::load(Path::new(config))?;
            let t: teacher::Config = serde_json::from_slice(&std::fs::read(teachers)?)?;
            // the drafts' senses, layer by layer in name order (an improved layer is worked through a pool)
            let mut job = nautilus::work::DictJob::whole(c.layers.iter().filter(|(_, l)| l.source != "layer").filter_map(|(k, l)| Some((k.clone(), l.out.clone()?, l.name.clone()))).collect());
            (job.batch, job.store, job.ask_all) = (c.batch(), c.store(), c.ask_all());
            let frozen = c.layers.values().flat_map(|l| l.frozen()).collect();
            let l = nautilus::work::Loop { state: Path::new(state).to_path_buf(), teachers: &t, profile: profile.to_string(), min_gap_secs: c.pacing(), frozen, ledger: None, member: profile.to_string() };
            // the animal's own mova-terminal (its profile in TEACHERS); unreadable: unplugged, questions are held
            let term: Box<dyn teacher::PrivateTerminal> = match t.profile(profile) {
                Some(p) => teacher::plug(p),
                None => Box::new(teacher::Unplugged(String::new())),
            };
            // one loop per state folder: a second start reports the running one
            let _one = match animal::singleton(&Path::new(state).join("work.lock")) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(5);
                }
            };
            let steps = rest.first().filter(|x| **x != "-").map(|x| x.parse()).transpose()?;
            let every = rest.get(1).map(|x| x.parse()).transpose()?.unwrap_or(30);
            println!("{}", nautilus::work::run(&l, &job, term.as_ref(), every, steps)?);
        }
        ["pool-work", config, teachers, profile, member, state, assignment, report, ledgers, rest @ ..] if rest.len() <= 2 => {
            let c = animal::Config::load(Path::new(config))?;
            let t: teacher::Config = serde_json::from_slice(&std::fs::read(teachers)?)?;
            let term: Box<dyn teacher::PrivateTerminal> = match t.profile(profile) {
                Some(p) => teacher::plug(p),
                None => Box::new(teacher::Unplugged(String::new())),
            };
            let _one = match animal::singleton(&Path::new(state).join("work.lock")) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(5);
                }
            };
            let m = nautilus::pool::Member {
                name: member.to_string(),
                config: &c,
                teachers: &t,
                profile: profile.to_string(),
                state: Path::new(state).to_path_buf(),
                assignment: Path::new(assignment).to_path_buf(),
                report: Path::new(report).to_path_buf(),
                ledgers: Path::new(ledgers).to_path_buf(),
                min_gap_secs: c.pacing(),
            };
            anyhow::ensure!(teacher::ident(member), "the member name is an ASCII identifier");
            let steps = rest.first().filter(|x| **x != "-").map(|x| x.parse()).transpose()?;
            let every = rest.get(1).map(|x| x.parse()).transpose()?.unwrap_or(120);
            println!("{}", nautilus::pool::run(&m, term.as_ref(), every, steps)?);
        }
        ["backlog", config, ledgers, rest @ ..] if rest.len() <= 1 => {
            let c = animal::Config::load(Path::new(config))?;
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();
            for k in c.layers.keys().filter(|k| rest.first().is_none_or(|r| r == k)) {
                let v = nautilus::pool::backlog(&c, k, Path::new(ledgers), now)?;
                let st = animal::stats(&c.layers[k]).unwrap_or_default();
                println!("{}", serde_json::json!({"layer":k,"backlog":v,"stats":st}));
            }
        }
        ["job-task", config, layer] => {
            let c = animal::Config::load(Path::new(config))?;
            let job = c.job(layer, 0, 1)?;
            match job.next(&Default::default())? {
                Some(item) => match job.native(&item) {
                    nautilus::work::Native::Unsure(q) => {
                        eprintln!("{} ({} bytes)", item.key, q.len());
                        print!("{q}");
                    }
                    nautilus::work::Native::Done(n) => println!("{}: native ({n})", item.key),
                },
                None => println!("nothing to ask"),
            }
        }
        ["dry", config, layer, rest @ ..] if rest.len() <= 1 => {
            let c = animal::Config::load(Path::new(config))?;
            let job = c.job(layer, 0, 1)?;
            let items = rest.first().map(|x| x.parse()).transpose()?.unwrap_or(10);
            println!("{}", serde_json::to_string_pretty(&nautilus::work::dry(job.as_ref(), items)?)?);
        }
        ["pool-seed", state, ledgers, layer, member] => {
            println!("{} lines seeded", nautilus::pool::seed(Path::new(state), Path::new(ledgers), layer, member)?);
        }
        ["replay", config, out, roots @ ..] if !roots.is_empty() => {
            let c = animal::Config::load(Path::new(config))?;
            let roots: Vec<&Path> = roots.iter().map(Path::new).collect();
            println!("{}", serde_json::to_string_pretty(&nautilus::replay::run(&c, Path::new(out), &roots)?)?);
        }
        ["work-summary", state] => println!("{}", serde_json::to_string_pretty(&nautilus::work::summary(Path::new(state))?)?),
        ["work-resume", state] => nautilus::work::resume(Path::new(state), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs())?,
        ["plan", dcfg, config, id] => {
            let c = animal::Config::load(Path::new(config))?;
            let d = deputy(dcfg, &c)?;
            let v = animal::answer(&d, &c, id, &stdin()?)?;
            println!("{}", v["answer"].as_str().unwrap_or(""));
            println!("{v}");
        }
        ["serve", dcfg, config, socket, lock] => {
            let _one = match nautilus::animal::singleton(Path::new(lock)) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("nautilus: {e}");
                    std::process::exit(5);
                }
            };
            let c = animal::Config::load(Path::new(config))?;
            let d = deputy(dcfg, &c)?;
            mova_octopus::socket::angel_prioritized(
                Path::new(socket),
                &|| Ok(|id: &str, _execute: bool, prompt: &str, _urgent: bool| animal::answer(&d, &c, id, prompt)),
                std::time::Duration::from_secs(60),
            )?;
        }
        ["ask", socket, id, rest @ ..] if rest.is_empty() || rest == ["urgent"] => {
            let v = mova_octopus::socket::ask_with(Path::new(socket), id, true, &stdin()?, !rest.is_empty())?;
            println!("{}", v["report"]["answer"].as_str().unwrap_or_else(|| v["error"].as_str().unwrap_or("no answer")));
            println!("{v}");
            if v["ok"] != true {
                bail!("the request failed");
            }
        }
        _ => bail!(USAGE),
    }
    Ok(())
}

fn main() -> std::process::ExitCode {
    let a: Vec<String> = std::env::args().skip(1).collect();
    match run(&a) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("nautilus: {e:#}");
            std::process::ExitCode::FAILURE
        }
    }
}
