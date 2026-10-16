//! The principles the small ones derived, checked by their teacher.
//!
//! During training the small ones derive new facts from two known ones (inheritance, lacking an instrument). Each round
//! a batch of the derived principles not yet checked goes to the teacher in one prompt; the verdicts (true, false,
//! unclear) are kept in a cache (`<hot>/principles-checked.tsv`) so no principle is checked twice; the true ones are
//! written by the small ones at the end of their own world, «What I derived» (`<hot>/worlds/derived/derived.world.md`),
//! each with the two facts it came from and the teacher's word. The brain holds its contents and the path.
//!
//! The teacher costs money: at most `DAILY_CALLS` calls a day, one batch a round, and the first failed call stops the
//! checks until the next day (fail-fast, no retry). `MOVA_PRINCIPLES=0` turns the checks off.

use std::collections::BTreeSet;

use anyhow::{Result, bail};

use crate::know::{Fact, Link};

pub const BATCH: usize = 40;
pub const DAILY_CALLS: usize = 20;

fn dir() -> std::path::PathBuf {
    crate::state::root().join("principles")
}

fn checked_path() -> std::path::PathBuf {
    crate::state::root().join("principles-checked.tsv")
}

fn world_path() -> std::path::PathBuf {
    crate::state::root().join("worlds").join("derived").join("derived.world.md")
}

fn today() -> u64 {
    crate::talk::now() / 86400
}

/// A derived fact as a plain sentence.
pub fn statement(f: &Fact) -> String {
    let s = f.subj.clone();
    let cap = |x: &str| x.chars().next().map(|c| c.to_uppercase().collect::<String>() + &x[c.len_utf8()..]).unwrap_or_default();
    let body = match (f.link, f.negated) {
        (Link::IsA, false) => format!("a {s} is a {}", f.obj),
        (Link::IsA, true) => format!("a {s} is not a {}", f.obj),
        (Link::Has, false) => format!("a {s} has {}", f.obj),
        (Link::Has, true) => format!("a {s} does not have {}", f.obj),
        (Link::Can, false) => format!("a {s} can {}", f.obj),
        (Link::Can, true) => format!("a {s} cannot {}", f.obj),
        (Link::Does, false) => format!("a {s} does this: {}", f.obj),
        (Link::Does, true) => format!("a {s} does not do this: {}", f.obj),
        (l, n) => format!("a {s} {}{} {}", if n { "not " } else { "" }, crate::know::link_name(l), f.obj),
    };
    format!("{}.", cap(&body))
}

fn key(f: &Fact) -> String {
    format!("{}|{:?}|{}|{}", f.subj, f.link, f.obj, f.negated)
}

/// The checked principles: key → (verdict, statement, sources, note).
pub fn checked() -> Vec<(String, char, String, String, String)> {
    std::fs::read_to_string(checked_path())
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let c: Vec<&str> = l.splitn(6, '\t').collect();
            (c.len() == 6).then(|| (c[0].to_string(), c[1].chars().next().unwrap_or('U'), c[2].to_string(), c[3].to_string(), c[4].to_string()))
        })
        .collect()
}

/// The derived principles not yet checked, in a fixed order.
pub fn pending(k: &crate::know::Knowledge, n: usize) -> Vec<Fact> {
    let done: BTreeSet<String> = checked().into_iter().map(|x| x.0).collect();
    let mut v: Vec<Fact> = k.by_subj.values().flatten().filter(|f| f.source.starts_with("derived from")).filter(|f| !done.contains(&key(f))).cloned().collect();
    v.sort_by(|a, b| key(a).cmp(&key(b)));
    v.dedup_by(|a, b| key(a) == key(b));
    v.truncate(n);
    v
}

fn calls_today() -> usize {
    std::fs::read_to_string(dir().join("calls.tsv")).unwrap_or_default().lines().filter(|l| l.split('\t').next().and_then(|d| d.parse::<u64>().ok()) == Some(today())).count()
}

fn stopped_today() -> bool {
    std::fs::read_to_string(dir().join("stopped")).ok().and_then(|x| x.trim().parse::<u64>().ok()) == Some(today())
}

/// One round of checks: a batch to the teacher, the verdicts cached, the world of what was derived written anew.
/// Returns (true, false, unclear) of this batch, or none when there is nothing to check or no call left today.
pub fn round() -> Result<Option<(usize, usize, usize)>> {
    if std::env::var("MOVA_PRINCIPLES").as_deref() == Ok("0") || stopped_today() || calls_today() >= DAILY_CALLS {
        return Ok(None);
    }
    let k = crate::know::knowledge();
    let batch = pending(&k, BATCH);
    if batch.is_empty() {
        return Ok(None);
    }
    let mut prompt = String::from(
        "You check general statements that a small language model derived by itself, each from two facts it read. For each numbered statement answer on its own line: the number, a space, then T if it is true in general, F if it is false, U if it is unclear, too vague or meaningless; then optionally « - » and a few words why. Nothing else.\n\n",
    );
    for (i, f) in batch.iter().enumerate() {
        prompt += &format!("{}. {} ({})\n", i + 1, statement(f), f.source.trim_start_matches("derived "));
    }
    std::fs::create_dir_all(dir())?;
    let opus = prag::opus::Opus::from_env(dir().join("cwd"));
    let ans = match opus.ask("principles-check", calls_today(), batch.len(), &prompt) {
        Ok((a, _)) => a,
        Err(e) => {
            std::fs::write(dir().join("stopped"), today().to_string())?;
            bail!("the teacher's call failed ({e}); no more checks today");
        }
    };
    let mut log = std::fs::OpenOptions::new().create(true).append(true).open(dir().join("calls.tsv"))?;
    std::io::Write::write_all(&mut log, format!("{}\t{}\n", today(), batch.len()).as_bytes())?;
    let mut verdict: Vec<(char, String)> = vec![('U', String::new()); batch.len()];
    for l in ans.lines() {
        let l = l.trim();
        let (num, rest) = l.split_once(' ').unwrap_or((l, ""));
        let Ok(i) = num.trim_end_matches(['.', ')', ':']).parse::<usize>() else { continue };
        if (1..=batch.len()).contains(&i) {
            let v = rest.trim().chars().next().filter(|c| matches!(c, 'T' | 'F' | 'U')).unwrap_or('U');
            let note = rest.split_once(" - ").map(|x| x.1.trim().to_string()).unwrap_or_default();
            verdict[i - 1] = (v, note);
        }
    }
    let mut out = std::fs::OpenOptions::new().create(true).append(true).open(checked_path())?;
    let (mut t, mut fl, mut u) = (0, 0, 0);
    for (f, (v, note)) in batch.iter().zip(&verdict) {
        match v {
            'T' => t += 1,
            'F' => fl += 1,
            _ => u += 1,
        }
        let clean = |x: &str| x.replace(['\t', '\n'], " ");
        std::io::Write::write_all(&mut out, format!("{}\t{v}\t{}\t{}\t{}\t{}\n", key(f), clean(&statement(f)), clean(&f.source), clean(note), crate::talk::now()).as_bytes())?;
        crate::ideas::record("derived", &statement(f), &format!("teacher: {v}{}", if note.is_empty() { String::new() } else { format!(" — {note}") }), "", &f.source);
    }
    write_world()?;
    Ok(Some((t, fl, u)))
}

/// The small ones' own world of what they derived: the true principles, each with its sources and the teacher's word.
pub fn write_world() -> Result<usize> {
    let all = checked();
    let good: Vec<&(String, char, String, String, String)> = all.iter().filter(|x| x.1 == 'T').collect();
    let mut md = format!(
        "# What I derived\n\nThe principles I derived myself, each from two facts of my worlds, and that my teacher found true: {} of {} checked ({} false, {} unclear).\n\n## Principles\n\n",
        good.len(),
        all.len(),
        all.iter().filter(|x| x.1 == 'F').count(),
        all.iter().filter(|x| x.1 == 'U').count()
    );
    for (i, x) in good.iter().enumerate() {
        md += &format!("{} [^{}]\n", x.2, i + 1);
    }
    md += "\n";
    for (i, x) in good.iter().enumerate() {
        md += &format!("[^{}]: {}{}.\n", i + 1, x.3, if x.4.is_empty() { String::new() } else { format!("; teacher: {}", x.4) });
    }
    let p = world_path();
    std::fs::create_dir_all(p.parent().unwrap())?;
    crate::store::write_atomic(&p, md.as_bytes())?;
    let id = crate::state::current_id();
    let body = format!("I derived {} principles my teacher found true; my world of what I derived is {}. The first of them:\n{}", good.len(), p.display(), good.iter().take(12).map(|x| x.2.clone()).collect::<Vec<_>>().join("\n"));
    let _ = crate::state::brain_section_of(&id, "The world of what I derived", &body);
    Ok(good.len())
}

/// `world principles` — what is checked and what waits; `world principles --round` — one batch now.
pub fn main_args(args: &[String]) -> Result<()> {
    if args.get(1).map(String::as_str) == Some("--round") {
        match round()? {
            Some((t, f, u)) => println!("checked: {t} true, {f} false, {u} unclear"),
            None => println!("nothing to check now (or no call left today)"),
        }
        return Ok(());
    }
    if args.get(1).map(String::as_str) == Some("--list") {
        let n: usize = args.get(2).and_then(|x| x.parse().ok()).unwrap_or(40);
        let all = pending(&crate::know::knowledge(), 100_000);
        for f in all.iter().step_by((all.len() / n).max(1)).take(n) {
            let first = f.source.trim_start_matches("derived from «").split("» and «").next().unwrap_or("");
            println!("{}\t{}\t{}", crate::know::world_of(first), statement(f), f.source);
        }
        return Ok(());
    }
    let k = crate::know::knowledge();
    let c = checked();
    println!("checked {} (true {}, false {}, unclear {}); calls today {}/{}; waiting at least {}", c.len(), c.iter().filter(|x| x.1 == 'T').count(), c.iter().filter(|x| x.1 == 'F').count(), c.iter().filter(|x| x.1 == 'U').count(), calls_today(), DAILY_CALLS, pending(&k, 100_000).len());
    for f in pending(&k, 5) {
        println!("  next: {} ({})", statement(&f), f.source);
    }
    Ok(())
}
