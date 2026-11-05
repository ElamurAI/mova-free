//! `turtle` CLI: check a course, make gap questions from a dictionary layer, count what was made, print a lesson task
//! for a teacher, run the standing loop (teacher questions only through a plugged-in private terminal and the
//! operator's word), and answer English as one of Mova's animals.
use anyhow::{Result, bail};
use std::io::Read;
use std::path::Path;
use turtle::{animal, check, make};

const USAGE: &str = "usage:
  turtle check COURSE|DIR                          the school's rule and the marks of every lesson (JSON; fails on ✗)
  turtle gap LAYER_TSV COURSE SOURCE OUT JOURNAL MODEL   gap questions from a Nautilus layer into the new folder
                                                   OUT (MODEL: Mova's en.bin, which reads each gap's slot)
  turtle made JOURNAL                              makings, lessons and questions recorded in the journal
  turtle lesson-task COURSE TOPIC FIRST N          print a teacher task to write N lessons (nothing is sent)
  turtle work CONFIG TEACHERS PROFILE STATE [STEPS|- [SECS [BATCH]]] the standing loop over the configured courses:
                                                   native checks in sweeps of up to BATCH lessons (500) with no pause;
                                                   a teacher step only when a sweep closed nothing, then a pause of
                                                   SECS (30); teacher only with the word; STOP in STATE ends it
  turtle work-summary STATE                        events, throughput, rejection and hold reasons
  turtle work-resume STATE                         continue after a limit has passed
  turtle plan DEPUTY_CONFIG CONFIG ID < text       read English, run it with the Turtle's hands, print the answer
  turtle serve DEPUTY_CONFIG CONFIG SOCKET LOCK         answer English on a private socket (one per LOCK; STOP ends it)
  turtle ask SOCKET ID [urgent] < text             send English to a serving Turtle";

fn stdin() -> Result<String> {
    let mut p = String::new();
    std::io::stdin().take(8193).read_to_string(&mut p)?;
    Ok(p)
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn deputy(path: &str, c: &animal::Config) -> Result<mova_octopus::Deputy> {
    let d: mova_octopus::Config = serde_json::from_slice(&std::fs::read(path)?)?;
    animal::check_capabilities(&d, c).map_err(anyhow::Error::msg)?;
    mova_octopus::Deputy::load(d)
}

fn run(a: &[String]) -> Result<()> {
    let a: Vec<&str> = a.iter().map(String::as_str).collect();
    match a.as_slice() {
        ["check", course] => {
            let r = check::course(course)?;
            println!("{}", serde_json::to_string_pretty(&r)?);
            eprintln!("{}", animal::say_check(&r));
            if r.with_problems > 0 {
                bail!("{} lessons with a problem", r.with_problems);
            }
        }
        ["gap", layer, course, source, out, journal, model] => {
            println!("{}", serde_json::to_string_pretty(&make::make_gap_course(Path::new(layer), course, source, Path::new(out), Path::new(journal), now(), Path::new(model))?)?);
        }
        ["made", journal] => println!("{}", make::made(Path::new(journal))?),
        ["lesson-task", course, topic, first, n] => print!("{}", make::lessons_task(course, topic, first.parse()?, n.parse()?)),
        ["work", config, teachers, profile, state, rest @ ..] if rest.len() <= 3 => {
            let c = animal::Config::load(Path::new(config))?;
            let t: nautilus::teacher::Config = serde_json::from_slice(&std::fs::read(teachers)?)?;
            let job = make::LessonJob { courses: c.courses.clone() };
            let frozen = c.courses.iter().map(|x| check::folder(x)).collect();
            let l = nautilus::work::Loop { state: Path::new(state).to_path_buf(), teachers: &t, profile: profile.to_string(), min_gap_secs: 600, frozen, ledger: None, member: profile.to_string() };
            // the animal's own mova-terminal (its profile in TEACHERS); unreadable: unplugged, questions are held
            let term: Box<dyn nautilus::teacher::PrivateTerminal> = match t.profile(profile) {
                Some(p) => nautilus::teacher::plug(p),
                None => Box::new(nautilus::teacher::Unplugged(String::new())),
            };
            // one loop per state folder: a second start reports the running one
            let _one = match nautilus::animal::singleton(&Path::new(state).join("work.lock")) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(5);
                }
            };
            let steps = rest.first().filter(|x| **x != "-").map(|x| x.parse()).transpose()?;
            let every = rest.get(1).map(|x| x.parse()).transpose()?.unwrap_or(30);
            let batch = rest.get(2).map(|x| x.parse()).transpose()?.unwrap_or(500);
            println!("{}", turtle::work::run(&l, &job, term.as_ref(), every, steps, batch)?);
        }
        ["work-summary", state] => println!("{}", serde_json::to_string_pretty(&nautilus::work::summary(Path::new(state))?)?),
        ["work-resume", state] => nautilus::work::resume(Path::new(state), now())?,
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
                    eprintln!("turtle: {e}");
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
            eprintln!("turtle: {e:#}");
            std::process::ExitCode::FAILURE
        }
    }
}
