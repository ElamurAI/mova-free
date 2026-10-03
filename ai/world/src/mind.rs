//! Talking to the SLM's brain in English. The brain's text (a self-description prompt, the brain file, letters) is
//! read sentence by sentence into the story reader's world (`babi::Story`); questions are answered from that world
//! with an explanation, or with a gap. No state changes: this is the brain alone.
//!
//!   world brain probe <prompt.md> [--letter <text>]... [--ask "<question?>"]...
//!   world brain serve                     a Unix socket (<hot>/brain.sock), one line in, one line out
//!   world brain say "<English line>"      send a line to the running brain and print its reply
//!
//! A line ending with "?" is a question; any other line is told to the brain (read into its world). Replies are
//! English: the answer and why, "I do not understand: …" for what it could not read, "I do not know: …" for a gap.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::babi::Story;

fn socket() -> PathBuf {
    std::env::var("MOVA_HOT_ROOT").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("hot")).join("brain.sock")
}

/// Sentences of a text: lines that are not headings, split into sentences.
fn sentences(text: &str) -> Vec<String> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| l.trim_start_matches("- ").to_string())
        .flat_map(|l| crate::babi::split_sentences(&l))
        .collect()
}

/// Read a text into the world: (understood, not understood with reasons).
pub fn tell(st: &mut Story, text: &str) -> (usize, Vec<(String, String)>) {
    let mut ok = 0;
    let mut gaps = Vec::new();
    for s in sentences(text) {
        match st.read(&s) {
            Ok(None) => ok += 1,
            Ok(Some(why)) => gaps.push((s, why)),
            Err(e) => gaps.push((s, format!("{e}"))),
        }
    }
    (ok, gaps)
}

/// One English reply to one English line.
pub fn reply(st: &mut Story, line: &str) -> String {
    let line = line.trim();
    if line.ends_with('?') {
        match st.ask(line) {
            Ok((a, why)) if a != "?" => format!("{a}. Because: {why}"),
            Ok((_, why)) => format!("I do not know: {why}"),
            Err(e) => format!("I do not know: {e}"),
        }
    } else {
        let (ok, gaps) = tell(st, line);
        if gaps.is_empty() {
            format!("Understood ({ok} sentence{}).", if ok == 1 { "" } else { "s" })
        } else {
            format!("I do not understand: {}", gaps.iter().map(|(s, w)| format!("«{s}» — {w}")).collect::<Vec<_>>().join("; "))
        }
    }
}

/// `world brain probe`: which brain arises from a prompt, and how it reacts to letters.
pub fn probe(prompt: &Path, letters: &[String], questions: &[String]) -> Result<()> {
    let text = std::fs::read_to_string(prompt).with_context(|| format!("{}", prompt.display()))?;
    let mut st = Story::default();
    let (ok, gaps) = tell(&mut st, &text);
    println!("prompt {}: {ok} sentences understood, {} not", prompt.display(), gaps.len());
    for (s, w) in gaps.iter().take(5) {
        println!("  not understood: «{s}» — {w}");
    }
    let ask = |st: &Story, when: &str| {
        for q in questions {
            let r = match st.ask(q) {
                Ok((a, why)) if a != "?" => format!("{a} ({why})"),
                Ok((_, why)) => format!("gap: {why}"),
                Err(e) => format!("gap: {e}"),
            };
            println!("  [{when}] {q} → {r}");
        }
    };
    ask(&st, "before letters");
    for (k, l) in letters.iter().enumerate() {
        let (ok, gaps) = tell(&mut st, l);
        println!("letter {k}: {ok} sentences understood, {} not", gaps.len());
        ask(&st, &format!("after letter {k}"));
    }
    Ok(())
}

/// `world brain serve`: load the brain and the last letter, answer on a Unix socket.
pub fn serve() -> Result<()> {
    let mut st = Story::default();
    let (ok, gaps) = tell(&mut st, &crate::state::brain_read());
    let mut loaded = format!("brain: {ok} sentences understood, {} not", gaps.len());
    if let Some((n, t, _)) = crate::state::letter_last()? {
        let (o, g) = tell(&mut st, &t);
        loaded += &format!("; letter {n}: {o} understood, {} not", g.len());
    }
    let p = socket();
    let _ = std::fs::remove_file(&p);
    let listener = UnixListener::bind(&p).with_context(|| format!("{}", p.display()))?;
    println!("{loaded}; listening on {}", p.display());
    for conn in listener.incoming() {
        let Ok(mut c) = conn else { continue };
        let mut line = String::new();
        if BufReader::new(&c).read_line(&mut line).is_ok() && !line.trim().is_empty() {
            let r = reply(&mut st, &line);
            // every answer carries the state it was processed in and the last stable state
            let _ = writeln!(c, "{r} [state {}, stable {}]", std::env::var("MOVA_PROCESSING_STATE").unwrap_or_default(), crate::state::stable());
        }
    }
    Ok(())
}

/// `world brain say`: one line to the running brain.
pub fn say(line: &str) -> Result<String> {
    let mut c = UnixStream::connect(socket()).context("the brain is not running: start `world brain serve`")?;
    writeln!(c, "{line}")?;
    let mut r = String::new();
    BufReader::new(&c).read_line(&mut r)?;
    Ok(r.trim().to_string())
}
