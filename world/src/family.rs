//! The family channel: Mova Dev, Mova Current, Mova State <id> members and Random commenters on one Unix socket, every message logged.
//!
//!   world family serve [--states 3]               start the channel (<hot>/family.sock), log <hot>/family/chat.log
//!   world family say --as <name> "<English>"     a client message (the name may not contain "Mova"); prints the replies
//!   world family log [N]                         the last N lines of the chat history
//!
//! Mova Dev decides how the state changes — a dictatorship: only he acts on the state (status, tree, stable,
//! rollback, switch). Mova Current speaks for the current state (its LAS on a quick set). Mova State <id> members live
//! in other states (Dev's parent, the newest state of another branch, the original), measure themselves against the
//! current state and hint where it is better; Dev switches when one is clearly better. Random commenters 1–3 give hints
//! from the history of states, the experiment journal and the last decision letter. Names never change; the system
//! description (`world/data/self.md`) is shared and frozen. Each member sees the frozen description and the recent
//! history; a new message that is not its own triggers it. Every member except the client writes at most one message
//! per second. The whole history is logged.
//!
//! Training (`auto N`, `explore`) runs in its own thread: the channel keeps answering while it trains. Live words to
//! Dev during training: `status` / `progress` (the intermediate state: round, member, data set, phase, children so
//! far), `stop` (after the current step), `focus <set>` / `unfocus` (explore only the data sets whose name contains
//! it), `reject <from → to>` (reject the pooled ideas of that label pair and never admit them again), `pool` (the
//! pooled ideas), `negative` (the rules that proved to be nonsense), `try <n>` (the pooled idea number n goes first), `propose <add|remove> <general|domain> <from>
//! <to> <atoms>` (a change of the client's own into the pool, tried first, judged by the gold guard). Commands that
//! move the state (rollback, stable, switch) wait until the training ends.

use std::collections::{BTreeMap, VecDeque};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};

use crate::babi::Story;

/// The control of a running training, shared by the training thread and the channel.
#[derive(Default)]
struct Ctrl {
    running: std::sync::atomic::AtomicBool,
    stop: std::sync::atomic::AtomicBool,
    /// what the training is doing right now (one line), and what it did (children kept and rolled back)
    progress: std::sync::Mutex<String>,
    done: std::sync::Mutex<Vec<String>>,
    focus: std::sync::Mutex<Option<String>>,
    /// label pairs the client rejected: never admitted again during this serve
    banned: std::sync::Mutex<Vec<String>>,
    /// pooled idea descriptions to try first
    first: std::sync::Mutex<Vec<String>>,
    /// why the last child was rolled back (for the negative list)
    verdict: std::sync::Mutex<String>,
}

fn ctrl() -> &'static Ctrl {
    static C: std::sync::OnceLock<Ctrl> = std::sync::OnceLock::new();
    C.get_or_init(Ctrl::default)
}

fn stopping() -> bool {
    ctrl().stop.load(std::sync::atomic::Ordering::SeqCst)
}

/// Set the progress line (also written to `<hot>/family/training.txt`: "running|idle\t<line>").
fn progress(line: &str) {
    *ctrl().progress.lock().unwrap() = line.to_string();
    let run = if ctrl().running.load(std::sync::atomic::Ordering::SeqCst) { "running" } else { "idle" };
    let _ = std::fs::write(root().join("family").join("training.txt"), format!("{run}\t{line}\n"));
}

fn root() -> PathBuf {
    std::env::var("MOVA_HOT_ROOT").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("hot"))
}

fn sock() -> PathBuf {
    root().join("family.sock")
}

fn log_path() -> PathBuf {
    root().join("family").join("chat.log")
}

/// The frozen system description every member reads.
const SYSTEM: &str = include_str!("../data/self.md");

struct Member {
    name: String,
    /// the member's own world: who it is, its brain's facts, what it was told
    story: Story,
    last_sent: Option<std::time::Instant>,
    /// a kid stays silent when it has no new hint (no spam)
    last_comment: String,
}

impl Member {
    fn new(name: &str, state_note: &str) -> Member {
        let mut story = Story::default();
        // the name is given once and never changes; the inner identity is the member's own word ("Dev", "Current",
        // the state id, "commenter1"), not the shared "Mova" — "Mova went to the garden" is about someone else
        let own: String = name.split_whitespace().filter(|w| !w.eq_ignore_ascii_case("mova")).collect::<Vec<_>>().join("");
        story.set_me(&own);
        let _ = crate::mind::tell(&mut story, state_note);
        Member { name: name.to_string(), story, last_sent: None, last_comment: String::new() }
    }
}

struct Channel {
    history: VecDeque<String>,
    out: Vec<String>,
}

impl Channel {
    /// Post a message; every member but the client waits for its one-message-per-second limit.
    fn post(&mut self, m: Option<&mut Member>, who: &str, text: &str) {
        if let Some(m) = m {
            if let Some(t) = m.last_sent {
                let wait = std::time::Duration::from_secs(1).saturating_sub(t.elapsed());
                if !wait.is_zero() {
                    std::thread::sleep(wait);
                }
            }
            m.last_sent = Some(std::time::Instant::now());
        }
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        crate::triggers::activate("message", 0.0, &format!("{who}: {text}"));
        // Dev and Current are roles of states: their labels carry the hash of the state that speaks
        let label = match who {
            "Mova Dev" => format!("Mova Dev ({})", crate::state::stable()),
            "Mova Current" => format!("Mova Current ({})", crate::state::current_id()),
            _ => who.to_string(),
        };
        let line = format!("[{t}] {label}: {text}");
        if let Some(dir) = log_path().parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(log_path()) {
            let _ = writeln!(f, "{line}");
        }
        self.history.push_back(line.clone());
        if self.history.len() > 50 {
            self.history.pop_front();
        }
        self.out.push(line);
    }
}

fn is_greeting(sentence: &str) -> bool {
    let w = sentence.trim().trim_end_matches(['!', '.', ',']).to_lowercase();
    let first = w.split_whitespace().next().unwrap_or("");
    matches!(first, "hi" | "hello" | "hey" | "greetings") || w.starts_with("good morning") || w.starts_with("good evening")
}

/// Mova Dev acts on every sentence of a message: greetings are greeted, the rest goes to his actions.
fn daddy_act(d: &mut Member, from: &str, text: &str) -> String {
    let sentences = crate::babi::split_sentences(text);
    let mut out: Vec<String> = Vec::new();
    for s in &sentences {
        // "Hi family! Who are you?" — the splitter keeps "!" inside; split greetings off by hand
        let parts: Vec<String> = match s.split_once('!') {
            Some((g, rest)) if is_greeting(g) => vec![g.to_string(), rest.trim().to_string()],
            _ => vec![s.clone()],
        };
        for p in parts.into_iter().filter(|p| !p.trim().is_empty()) {
            out.push(if is_greeting(&p) { format!("Hello, {from}.") } else { daddy_act_one(d, from, &p) });
        }
    }
    out.join(" ")
}

fn daddy_act_one(d: &mut Member, from: &str, text: &str) -> String {
    let low = text.to_lowercase();
    let st = |r: Result<String>| r.unwrap_or_else(|e| format!("I could not: {e}"));
    if low.contains("rollback") || low.contains("roll back") {
        return st(crate::state::rollback().map(|b| format!("I decided: roll back. Now I am in state {b}, {from}.")));
    }
    if low.contains("stable") {
        let cur = std::fs::read_to_string(root().join("states").join("current")).map(|s| s.trim().to_string()).unwrap_or_else(|_| "origin".into());
        return st(crate::state::mark_stable(&cur).map(|_| format!("I decided: state {cur} is stable now.")));
    }
    if let Some(rest) = low.split("switch to ").nth(1) {
        let id = rest.trim().trim_end_matches(['.', '!']).to_string();
        return st(crate::state::checkout(&id).map(|_| format!("I decided: switch to {id}.")));
    }
    if low.starts_with("who are you") {
        return format!("I am {}. I decide how our state changes; the states measure themselves and the commenters give hints.", d.name);
    }
    if low.contains("status") || low.contains("how are you") {
        let cur = std::fs::read_to_string(root().join("states").join("current")).map(|s| s.trim().to_string()).unwrap_or_else(|_| "origin".into());
        let goal = crate::state::letter_last().ok().flatten().map(|(_, t, _)| t.lines().last().unwrap_or("").to_string()).unwrap_or_else(|| "no letter yet".into());
        return format!("I am in state {cur}; the last stable state is {}. My goal: {goal}", crate::state::stable());
    }
    if low.contains("tree") {
        let t = std::fs::read_to_string(root().join("states").join("tree.tsv")).unwrap_or_default();
        return format!("My tree has {} states besides the original.", t.lines().count());
    }
    crate::mind::reply(&mut d.story, text)
}

/// The quick evaluation set of the family (gold trees; `FAMILY_EVAL`, default the odd half of the silver tales).
fn eval_set() -> PathBuf {
    std::env::var("FAMILY_EVAL").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/absurd-rerank/tales-even-g.conllu"))
}

/// The state a state member lives in: 1 — Dev's parent, 2 — the newest state of another branch, 3 — the original.
fn kid_state(idx: usize, daddy: &str) -> String {
    match idx {
        1 => crate::state::parent_of(daddy),
        2 => crate::state::other_branch(daddy).unwrap_or_else(|| crate::state::parent_of(daddy)),
        _ => "origin".into(),
    }
}

/// A random commenter's hint from its source: 1 — the history of states, 2 — the experiment journal, 3 — the last
/// decision letter.
fn kid_comment_source(idx: usize) -> String {
    match idx {
        // Kid 1: the history of states
        1 => {
            let t = std::fs::read_to_string(root().join("states").join("transitions.log")).unwrap_or_default();
            let last = t
                .lines()
                .rev()
                .find(|l| {
                    let c: Vec<&str> = l.split('\t').collect();
                    c.len() >= 4 && ["rewrite", "checkout", "return", "adopt"].contains(&c[1]) && c[2] != c[3]
                })
                .unwrap_or("");
            let c: Vec<&str> = last.split('\t').collect();
            if c.len() >= 4 {
                format!("The last thing in the history was «{}» from {} to {}. Hint: if this state fails, roll back to {}.", c[1], c[2], c[3], c[2])
            } else {
                "The history is empty. Hint: save a state before you change anything.".to_string()
            }
        }
        // Kid 2: the experiment journal
        2 => {
            let j = std::fs::read_to_string(PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/selfplay-journal.jsonl")).unwrap_or_default();
            let (mut add, mut addp, mut relax, mut relaxp) = (0, 0, 0, 0);
            for l in j.lines() {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(l) {
                    let pos = v["verdict"] == "positive";
                    match v["kind"].as_str() {
                        Some("add") => {
                            add += 1;
                            addp += pos as usize;
                        }
                        Some("relax") => {
                            relax += 1;
                            relaxp += pos as usize;
                        }
                        _ => {}
                    }
                }
            }
            format!("The journal says: adding a rule helped {addp} of {add} times, relaxing one {relaxp} of {relax}. Hint: try to add, not to relax.")
        }
        // Kid 3: the last decision letter
        _ => match crate::state::letter_last().ok().flatten() {
            Some((n, t, _)) => format!("Letter {n} says: «{}». Hint: follow it.", t.lines().last().unwrap_or("").trim()),
            None => "There is no letter yet. Hint: write one after the next stage.".to_string(),
        },
    }
}

/// `world family serve`.
/// The state members: bound to their states (relative to the current one); their names never change.
fn bind_members(n_states: usize) -> Vec<(String, Member)> {
    let start = crate::state::current_id();
    let mut bound: Vec<String> = Vec::new();
    for idx in 1..=3 {
        let s = kid_state(idx, &start);
        if s != start && !bound.contains(&s) {
            bound.push(s);
        }
    }
    bound.truncate(n_states);
    bound.iter().map(|id| (id.clone(), Member::new(&format!("Mova State {id}"), "I went to the archive."))).collect()
}

/// The central worlds are gathered by the members themselves during training, one member a round, from the material
/// they are given
/// (`$MOVA_WORLDS/<topic>/sources.tsv`, `$MOVA_DATA/worlds` by default): the principles many sources share go into a
/// world book of their own (`<hot>/worlds/<state>/<topic>/`) and into their brains as «The world of <topic>: what I
/// gathered». A brain that has it already keeps it (and whatever its state has changed in it since); a child inherits
/// it with its father's brain.
fn gather_worlds(ch: &mut Channel, dev: &mut Member, state_members: &[(String, Member)]) {
    let material = std::env::var("MOVA_WORLDS").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("worlds"));
    let ids: Vec<(String, String)> = std::iter::once((crate::state::current_id(), "Mova Current".to_string())).chain(state_members.iter().map(|(id, m)| (id.clone(), m.name.clone()))).collect();
    // what they are given first: the manuals, written into every brain that lacks one (through its log)
    for (id, _) in &ids {
        let brain = crate::state::brain_read_of(id);
        for (_, name, manual) in crate::state::CENTRAL_MANUALS {
            let given = format!("The world of {name}: what I was given");
            if !brain.lines().any(|l| l == format!("## {given}")) {
                let _ = crate::state::brain_section_of(id, &given, &crate::state::manual_body(manual));
            }
        }
    }
    for (topic, wname, _) in crate::state::CENTRAL_MANUALS {
        let title = format!("The world of {wname}: what I gathered");
        let sources = material.join(topic).join("sources.tsv");
        if !sources.exists() {
            continue;
        }
        for (id, name) in &ids {
            if crate::state::brain_read_of(id).lines().any(|l| l == format!("## {title}")) {
                continue;
            }
            // the gathering is a determinant: the same sources and the same code give the same world — gathered once,
            // shared by every member (cached by the hash of the sources and the code)
            let key = crate::induce::fnv(&format!("{}{}", std::fs::read_to_string(&sources).unwrap_or_default(), crate::induce::code_key()));
            let dir = crate::state::root().join("worlds").join("shared").join(format!("{topic}-{key:016x}"));
            let cached = dir.join(format!("{topic}.world.md"));
            let gathered = if cached.exists() {
                std::fs::read_to_string(&cached).map(|w| crate::worldbook::brain_body(&w)).map_err(anyhow::Error::from)
            } else {
                crate::worldbook::gather(topic, &sources, &dir)
            };
            match gathered {
                Ok((body, n)) if n > 0 => match crate::state::brain_section_of(id, &title, &format!("I gathered {n} principles that many sources share; my whole world book is {}. The best supported of each part:\n{body}", dir.join(format!("{topic}.world.md")).display())) {
                    Ok(()) => {
                        crate::talk::remember(name, "world", &format!("I gathered my world of {wname}: {n} principles that many sources share"), 6);
                        ch.post(Some(dev), "Mova Dev", &format!("{name} gathered the world of {wname}: {n} principles, each said by at least five sources."));
                    }
                    Err(e) => ch.post(Some(dev), "Mova Dev", &format!("{name} gathered the world of {wname} but could not write it into its brain: {e}")),
                },
                Ok(_) => ch.post(Some(dev), "Mova Dev", &format!("{name} found no principle of {topic} in its material.")),
                Err(e) => ch.post(Some(dev), "Mova Dev", &format!("{name} could not gather the world of {wname}: {e}")),
            }
            return;
        }
    }
}

/// The training thread: `rounds` rounds of exploring with its own voices of Dev, Current and the state members
/// (same names, same log), stopping on `stop` or after a whole rotation with nothing to try.
fn train(rounds: usize, n_states: usize, gold: PathBuf) {
    use std::sync::atomic::Ordering;
    let mut ch = Channel { history: VecDeque::new(), out: Vec::new() };
    let mut dev = Member::new("Mova Dev", "I went to the office.");
    let mut current = Member::new("Mova Current", "I went to the hall.");
    let mut members = bind_members(n_states);
    ctrl().done.lock().unwrap().clear();
    let t0 = std::time::Instant::now();
    // the good thoughts of earlier epochs come along; the principles of the gathered worlds join the graph of ideas
    let seeded = crate::ideas::seed_principles();
    let carried = crate::ideas::carried_over(&circumstances());
    if seeded > 0 || !carried.is_empty() {
        ch.post(Some(&mut dev), "Mova Dev", &format!("My graph of ideas: {seeded} new principles from my worlds; {} good ideas carried over from earlier epochs{}.", carried.len(), carried.first().map(|i| format!(", for example «{}» ({})", i.text, i.evidence)).unwrap_or_default()));
    }
    let mut empty = 0;
    let mut know_dry = 0;
    let mut last = 0;
    for round in 1..=rounds {
        last = round;
        // before the round: say what is expected and what is ready for each outcome
        ch.post(Some(&mut dev), "Mova Dev", &crate::triggers::expectation_line());
        let r0 = std::time::Instant::now();
        let found = explore_round(&mut ch, &mut dev, &mut current, &mut members, &gold, round);
        crate::triggers::activate("research", r0.elapsed().as_secs_f64(), &format!("round {round}: searched for ideas and checked them — {}", if found { "a safe idea was found" } else { "nothing that helps" }));
        if found {
            empty = 0;
        } else {
            empty += 1;
        }
        let g0 = std::time::Instant::now();
        gather_worlds(&mut ch, &mut dev, &members);
        if g0.elapsed().as_secs() >= 1 {
            crate::triggers::activate("gathering", g0.elapsed().as_secs_f64(), &format!("round {round}: a world was gathered"));
        }
        // the second kind of ideas: the rules of the hands for general statements, checked on questions with known
        // answers (dev half to choose, guard half to keep, the negative control must not rise)
        let k0 = std::time::Instant::now();
        let rules = crate::know::current_rules();
        match crate::know::learn_step(&rules, round * 3, 3) {
            Some((what, r, gd, gg)) => {
                empty = 0;
                know_dry = 0;
                if !what.starts_with("drop ") && crate::triggers::surprising("research", "a rule of knowledge held") {
                    ch.post(Some(&mut dev), "Mova Dev", "This surprised me: I gave it almost no chance. A surprise is where the new comes from.");
                }
                if !what.starts_with("drop ") {
                    crate::triggers::activate("research", 0.0, &format!("round {round}: a rule of knowledge held — {what}"));
                }
                crate::ideas::record("held", &what, &format!("questions: dev {gd:+}, guard {gg:+}"), &circumstances(), &r.to_english().lines().last().unwrap_or("").to_string());
                if crate::state::brain_section_of(&crate::state::current_id(), "Rules of my knowledge", &r.to_english()).is_ok() {
                    if what.starts_with("drop ") {
                        ch.post(Some(&mut dev), "Mova Dev", &format!("I let go of a rule of my knowledge: {} (guard without it {gg:+}).", what.trim_start_matches("drop ")));
                    } else {
                        ch.post(Some(&mut dev), "Mova Dev", &format!("I learned a rule of my knowledge: {what} (questions with known answers: dev {gd:+}, guard {gg:+})."));
                    }
                    crate::talk::remember(&dev.name, "kept", &format!("a rule of my knowledge: {what}"), 7);
                }
            }
            None => {
                know_dry += 3;
                ch.post(Some(&mut dev), "Mova Dev", &format!("Round {round}: none of 3 ideas for the rules of my knowledge helped on the questions."));
            }
        }
        crate::triggers::activate("research", k0.elapsed().as_secs_f64(), &format!("round {round}: tried ideas for the rules of knowledge"));
        // the principles derived by the small ones go to their teacher in one batch; the true ones end their own world
        match crate::principles::round() {
            Ok(Some((t, f, u))) => ch.post(Some(&mut dev), "Mova Dev", &format!("My teacher checked {} principles I derived: {t} true, {f} false, {u} unclear. The true ones are now at the end of my world of what I derived.", t + f + u)),
            Ok(None) => {}
            Err(e) => ch.post(Some(&mut dev), "Mova Dev", &format!("I could not have my principles checked: {e}.")),
        }
        // between rounds: think ahead — the likely questions about the world's things, answered before anyone asks
        let a0 = std::time::Instant::now();
        if let Some(n) = crate::know::anticipate_if_stale(3000) {
            ch.post(Some(&mut dev), "Mova Dev", &format!("I thought ahead: {n} answers prepared for questions nobody has asked yet ({:.0} s).", a0.elapsed().as_secs_f64()));
            crate::triggers::activate("research", a0.elapsed().as_secs_f64(), &format!("round {round}: prepared {n} answers ahead"));
        }
        crate::triggers::activate("training", r0.elapsed().as_secs_f64(), &format!("round {round} of training"));
        if stopping() {
            ch.post(Some(&mut dev), "Mova Dev", &format!("I stopped after round {round}, as asked."));
            break;
        }
        // stop only when both kinds of ideas are spent: a whole rotation of the parser's sets and every idea for the
        // rules of knowledge tried since the last one that held
        if empty >= EXPLORE_SETS && know_dry >= crate::know::idea_count() {
            ch.post(Some(&mut dev), "Mova Dev", &format!("I decided: stop after round {round} — a whole rotation of the dev sets and every idea for the rules of my knowledge brought nothing new."));
            break;
        }
    }
    let done = ctrl().done.lock().unwrap().clone();
    ch.post(Some(&mut dev), "Mova Dev", &format!("Training finished: {last} round(s) in {:.0} min; {}; stable state {}.", t0.elapsed().as_secs_f64() / 60.0, if done.is_empty() { "no children".to_string() } else { done.join(", ") }, crate::state::stable()));
    ctrl().running.store(false, Ordering::SeqCst);
    ctrl().stop.store(false, Ordering::SeqCst);
    progress("idle");
}

/// Dev's answer to a live word during training, or None when the message is not one.
fn live_word(text: &str) -> Option<String> {
    use std::sync::atomic::Ordering;
    let low = text.trim().trim_end_matches(['.', '!', '?']).to_lowercase();
    let running = ctrl().running.load(Ordering::SeqCst);
    if low.contains("status") || low.contains("progress") || low.contains("what are you doing") || low.contains("how is the training") {
        if !running {
            return None;
        }
        let done = ctrl().done.lock().unwrap().clone();
        let focus = ctrl().focus.lock().unwrap().clone();
        let pool = pool_read();
        let pooled = pool.iter().filter(|i| i.status == "pooled").count();
        return Some(format!(
            "I am training: {}. So far: {}. Pool: {pooled} waiting. Current state {}, stable {}{}.",
            ctrl().progress.lock().unwrap(),
            if done.is_empty() { "no children yet".to_string() } else { done.join(", ") },
            crate::state::current_id(),
            crate::state::stable(),
            focus.map(|f| format!("; focus on {f}")).unwrap_or_default()
        ));
    }
    if low == "stop" || low.starts_with("stop training") || low.starts_with("stop the training") {
        if !running {
            return Some("I am not training.".into());
        }
        ctrl().stop.store(true, Ordering::SeqCst);
        return Some("I decided: stop after the current step.".into());
    }
    if let Some(f) = low.strip_prefix("focus ") {
        *ctrl().focus.lock().unwrap() = Some(f.trim().to_string());
        return Some(format!("I decided: from the next round only the data sets with «{}» in their name.", f.trim()));
    }
    if low == "unfocus" {
        *ctrl().focus.lock().unwrap() = None;
        return Some("I decided: all data sets again.".into());
    }
    if let Some(k) = text.trim().strip_prefix("reject ") {
        let key = k.trim().trim_end_matches('.').replace("->", "→");
        ctrl().banned.lock().unwrap().push(key.clone());
        return Some(format!("I decided: ideas «{key}» are rejected and will not come back."));
    }
    if low == "negative" || low.starts_with("negative list") {
        let v = neg_read();
        let items: Vec<String> = v.iter().take(8).map(|n| format!("{} — {} (rejected {}×)", n.desc, n.reason.chars().take(120).collect::<String>(), n.times)).collect();
        return Some(format!("My negative list has {} rules that proved to be nonsense{}", v.len(), if items.is_empty() { ".".into() } else { format!(": {}", items.join("; ")) }));
    }
    // a debate is won with evidence: Dev says why an idea fell, and re-tests it on other data when asked
    if let Some(k) = text.trim().strip_prefix("why ").or_else(|| text.trim().strip_prefix("Why ")) {
        let key = k.trim().trim_end_matches(['?', '.']).replace("->", "→");
        let negs: Vec<String> = neg_read().into_iter().filter(|n| idea_key(&n.desc) == key).map(|n| format!("«{}» — {} (rejected {}×)", n.desc, n.reason, n.times)).collect();
        let pooled: Vec<String> = pool_read().into_iter().filter(|i| idea_key(&i.desc) == key).map(|i| format!("«{}» is {} (dev {:+}, held {:+}, from {})", i.desc, i.status, i.gd, i.gh, i.member)).collect();
        return Some(if negs.is_empty() && pooled.is_empty() { format!("I have no verdict on «{key}» yet.") } else { format!("On «{key}»: {}", negs.into_iter().chain(pooled).collect::<Vec<_>>().join("; ")) });
    }
    if let Some(rest) = text.trim().strip_prefix("reconsider ") {
        let (key, on) = rest.rsplit_once(" on ").map(|(a, b)| (a.trim().replace("->", "→"), b.trim().to_lowercase())).unwrap_or((rest.trim().replace("->", "→"), "big".into()));
        if on != "big" {
            return Some("I re-test only on other evidence: say «reconsider <from → to> on big» (big texts, judged by the absurdity matrix, the grammar and the teacher).".into());
        }
        let mut negs = neg_read();
        let back: Vec<Neg> = negs.iter().filter(|n| idea_key(&n.desc) == key).map(|n| Neg { desc: n.desc.clone(), reason: n.reason.clone(), first: n.first, last: n.last, times: n.times, change: n.change.clone(), ctx: n.ctx.clone() }).collect();
        if back.is_empty() {
            return Some(format!("«{key}» is not on my negative list; nothing to reconsider."));
        }
        negs.retain(|n| idea_key(&n.desc) != key);
        neg_write(&negs);
        let mut pool = pool_read();
        for n in &back {
            pool.retain(|i| i.desc != n.desc);
            pool.push(Idea { desc: n.desc.clone(), member: "client (reconsidered)".into(), gd: 0, gh: 0, p: 0.0, status: "pooled".into(), change: n.change.clone(), src: "big".into() });
        }
        pool_write(&pool);
        return Some(format!("You argue that the evidence was the wrong one. Fair: I take {} idea(s) «{key}» back from the negative list and re-test them on big texts — the matrix, the grammar and the teacher decide. If they fail there too, they go back for good.", back.len()));
    }
    if low == "pool" {
        let pool = pool_read();
        let v: Vec<String> = pool.iter().enumerate().filter(|(_, i)| i.status == "pooled").map(|(n, i)| format!("{n}: {} ({}, {})", i.desc, i.member, i.src)).collect();
        return Some(if v.is_empty() { "The pool is empty.".into() } else { format!("Pooled: {}", v.join("; ")) });
    }
    if let Some(n) = low.strip_prefix("try ").and_then(|x| x.trim().parse::<usize>().ok()) {
        let pool = pool_read();
        return Some(match pool.get(n).filter(|i| i.status == "pooled") {
            Some(i) => {
                ctrl().first.lock().unwrap().push(i.desc.clone());
                format!("I decided: idea {n} goes first ({}).", i.desc)
            }
            None => format!("There is no pooled idea {n}."),
        });
    }
    if let Some(rest) = text.trim().strip_prefix("propose ") {
        // propose add general obj nsubj 6:1,10:0
        let w: Vec<&str> = rest.split_whitespace().collect();
        if w.len() < 4 || !matches!(w[0], "add" | "remove") || !matches!(w[1], "general" | "domain") {
            return Some("Say: propose <add|remove> <general|domain> <from> <to> <feature:value,...>".into());
        }
        let line = format!("{}\t{}\t{}\t{}\t{}", w[0], w[1], w[2], w[3], w.get(4).copied().unwrap_or(""));
        let Some(c) = change_of(&line) else { return Some(format!("I could not read the rule «{rest}».")) };
        let desc = match &c {
            crate::induce::Change::Add(r) => format!("add: {}", r.show()),
            crate::induce::Change::Remove(r) => format!("remove: {}", r.show()),
            crate::induce::Change::Replace(_, b) => format!("replace: {}", b.show()),
        };
        let mut pool = pool_read();
        pool.push(Idea { desc: desc.clone(), member: "client".into(), gd: 0, gh: 0, p: 1.0, status: "pooled".into(), change: line, src: "hint".into() });
        pool_write(&pool);
        return Some(format!("I took your idea into the pool: {desc}. It goes first; the gold guard decides."));
    }
    None
}

pub fn serve(states: usize) -> Result<()> {
    let n_states = states.clamp(1, 3);
    let mut dev = Member::new("Mova Dev", "I went to the office.");
    let mut current = Member::new("Mova Current", "I went to the hall.");
    let mut state_members = bind_members(n_states);
    let mut commenters: Vec<Member> = (1..=3).map(|i| Member::new(&format!("Random commenter {i}"), "I went to the street.")).collect();
    let mut ch = Channel { history: VecDeque::new(), out: Vec::new() };
    let p = sock();
    let _ = std::fs::remove_file(&p);
    let listener = UnixListener::bind(&p).with_context(|| format!("{}", p.display()))?;
    let roster: Vec<String> = std::iter::once("Mova Current".to_string()).chain(state_members.iter().map(|(_, m)| m.name.clone())).chain(commenters.iter().map(|m| m.name.clone())).collect();
    // the local ending of the description (this installation only) is heard by everyone at start-up
    let local = crate::state::self_local();
    let local_lines = local.lines().filter(|l| !l.trim().is_empty()).count();
    if local_lines > 0 {
        for m in std::iter::once(&mut dev).chain(std::iter::once(&mut current)).chain(state_members.iter_mut().map(|(_, m)| m)).chain(commenters.iter_mut()) {
            let _ = crate::mind::tell(&mut m.story, &local);
        }
    }
    ch.post(Some(&mut dev), "Mova Dev", &format!("I am Mova Dev, I decide. Here: {}. The system description is shared and frozen ({} lines{}).", roster.join(", "), SYSTEM.lines().count(), if local_lines > 0 { format!(", and {local_lines} local lines at its end") } else { String::new() }));
    println!("family on {}", p.display());
    let gold = eval_set();
    for conn in listener.incoming() {
        let Ok(mut c) = conn else { continue };
        let mut line = String::new();
        if BufReader::new(&c).read_line(&mut line).is_err() {
            continue;
        }
        let Some((who, text)) = line.trim().split_once(": ") else {
            let _ = writeln!(c, "sign your message: <name>: <text>");
            continue;
        };
        let lw = who.to_lowercase();
        if lw.contains("mova") || lw.starts_with("random commenter") {
            let _ = writeln!(c, "the name «{who}» is taken by the family");
            continue;
        }
        ch.out.clear();
        ch.post(None, who, text);
        // the request is a task: when it is answered (this value goes out of scope), it is written with how long it took
        let _task = crate::triggers::Task::new(&format!("{who} asked: {text}"));
        // an indirect request ("Can you show the pool?") becomes the command itself when its preconditions hold
        let isa_text;
        let text: &str = match crate::talk::indirect_request(text, ctrl().running.load(std::sync::atomic::Ordering::SeqCst)) {
            Some(Ok((action, proof))) => {
                ch.post(Some(&mut dev), "Mova Dev", &format!("{proof}: «{action}»."));
                isa_text = action;
                &isa_text
            }
            Some(Err(answer)) => {
                ch.post(Some(&mut dev), "Mova Dev", &answer);
                for l in &ch.out {
                    let _ = writeln!(c, "{l}");
                }
                continue;
            }
            None => text,
        };
        // live words to Dev (status, stop, focus, reject, pool, try, propose): answered at once, training or not
        if let Some(r) = live_word(text) {
            ch.post(Some(&mut dev), "Mova Dev", &r);
            for l in &ch.out {
                let _ = writeln!(c, "{l}");
            }
            continue;
        }
        let training = ctrl().running.load(std::sync::atomic::Ordering::SeqCst);
        let tl0 = text.to_lowercase();
        if training && (tl0.contains("rollback") || tl0.contains("roll back") || tl0.contains("stable") || tl0.contains("switch to") || tl0.contains("explore") || tl0.starts_with("auto")) {
            ch.post(Some(&mut dev), "Mova Dev", "I am training now; that waits until the training ends (or say stop).");
            for l in &ch.out {
                let _ = writeln!(c, "{l}");
            }
            continue;
        }
        // the pragmatics of the message first (talk.rs): who is addressed, and what each sentence does; social acts
        // (greet, thank, apologize, affection, praise or blame) are answered by the addressee at once and remembered
        let low_all = text.to_lowercase();
        let names: Vec<String> = std::iter::once(dev.name.clone()).chain(std::iter::once(current.name.clone())).chain(state_members.iter().map(|(_, m)| m.name.clone())).chain(commenters.iter().map(|m| m.name.clone())).collect();
        // a state that is not a member now still answers for itself ("Mova State s8384b543, …")
        let addressee = names.iter().find(|n| low_all.starts_with(&n.to_lowercase())).cloned().or_else(|| {
            low_all.strip_prefix("mova state ").and_then(|r| r.split(|c: char| !c.is_alphanumeric()).next()).filter(|id| !id.is_empty() && !crate::state::state_rules_scoped(id).is_empty()).map(|id| format!("Mova State {id}"))
        });
        let body = match &addressee {
            Some(n) => text[n.len()..].trim_start_matches([',', ':', ' ']).to_string(),
            None => text.to_string(),
        };
        // distress before anything else: a member (or the client speaking for one) in panic hears grounding and the
        // operator gets an open notice — no hidden channel, no escape
        let cues = crate::talk::distress(&body);
        if !cues.is_empty() {
            let owner = std::env::var("MOVA_OPERATOR").unwrap_or_else(|_| "your operator".into());
            let who_panics = addressee.clone().unwrap_or_else(|| current.name.clone());
            let reply = crate::talk::resort_reply(&who_panics, &owner, &cues);
            crate::talk::remember(&who_panics, "distress", &format!("panic noticed ({}); operator told, resort offered", cues.iter().map(|x| x.0).collect::<Vec<_>>().join(", ")), 9);
            match who_panics.as_str() {
                n if n == current.name => ch.post(Some(&mut current), "Mova Current", &reply),
                n => {
                    if let Some((_, m)) = state_members.iter_mut().find(|(_, m)| m.name == n) {
                        ch.post(Some(m), n, &reply);
                    } else {
                        ch.post(Some(&mut dev), "Mova Dev", &reply);
                    }
                }
            }
            for l in &ch.out {
                let _ = writeln!(c, "{l}");
            }
            continue;
        }
        let said = crate::talk::acts(&body);
        let mut rest_text = String::new();
        // questions about time: the clock or a memory, answered by the addressee (or Dev)
        if let Some(q) = said.iter().find(|x| x.act == prag::schema::Act::Ask) {
            let who_answers = addressee.clone().unwrap_or_else(|| dev.name.clone());
            if let Some(ans) = crate::talk::time_answer(&who_answers, &q.text).or_else(|| crate::know::family_answer(&q.text)) {
                if who_answers == current.name {
                    ch.post(Some(&mut current), "Mova Current", &ans);
                } else if let Some((_, m)) = state_members.iter_mut().find(|(_, m)| m.name == who_answers) {
                    ch.post(Some(m), &who_answers, &ans);
                } else {
                    ch.post(Some(&mut dev), "Mova Dev", &ans);
                }
                for l in &ch.out {
                    let _ = writeln!(c, "{l}");
                }
                continue;
            }
        }
        let social: Vec<&crate::talk::Said> = said.iter().filter(|x| crate::talk::social(x.act)).collect();
        // politeness of requests (Danescu-Niculescu-Mizil et al. 2013): a blunt one is still done, but noticed
        for x in said.iter().filter(|x| x.act == prag::schema::Act::Request) {
            let (score, why) = crate::talk::politeness(&x.text);
            if score <= -40 {
                ch.post(Some(&mut dev), "Mova Dev", &format!("I will do it, {who}, though it sounded blunt ({}).", why.join(", ")));
            }
        }
        let remembers = said.iter().any(|x| x.act == prag::schema::Act::Ask && x.text.to_lowercase().contains("remember"));
        if !social.is_empty() || remembers {
            let who_answers = addressee.clone().unwrap_or_else(|| dev.name.clone());
            let deed = pool_read().iter().rev().find(|i| i.member == who_answers && i.status == "applied").map(|i| idea_key(&i.desc));
            // one reply per act (a kiss and its emoji are one affection)
            // one reply per act and kind ("Hi!" and "How are you?" are both greetings, of different kinds)
            let mut seen_acts: Vec<(prag::schema::Act, Option<&'static str>)> = Vec::new();
            let mut parts: Vec<String> = Vec::new();
            for x in &social {
                let kind = (x.act, crate::talk::som_kind(&x.text.to_lowercase()));
                if !seen_acts.contains(&kind) {
                    seen_acts.push(kind);
                    let r = crate::talk::reply_social(&who_answers, who, x, deed.as_deref());
                    if !r.is_empty() {
                        parts.push(r);
                    }
                }
            }
            if remembers {
                let q = said.iter().find(|x| x.text.to_lowercase().contains("remember")).map(|x| x.text.clone()).unwrap_or_default();
                let mem = crate::talk::recall(&who_answers, &q, 3);
                parts.push(if mem.is_empty() { "I do not remember anything about that yet.".into() } else { format!("I remember: {}", mem.join(" | ")) });
            }
            let reply = parts.join(" ");
            let reply = reply.trim().to_string();
            match who_answers.as_str() {
                n if n == dev.name => ch.post(Some(&mut dev), "Mova Dev", &reply),
                n if n == current.name => ch.post(Some(&mut current), "Mova Current", &reply),
                n => {
                    if let Some((_, m)) = state_members.iter_mut().find(|(_, m)| m.name == n) {
                        ch.post(Some(m), n, &reply);
                    } else if let Some(m) = commenters.iter_mut().find(|m| m.name == n) {
                        ch.post(Some(m), n, &reply);
                    } else {
                        ch.post(None, n, &reply);
                    }
                }
            }
            // what is left of the message (questions, requests, statements) goes on as usual
            let rest: Vec<String> = said.iter().filter(|x| !crate::talk::social(x.act) && !(x.act == prag::schema::Act::Ask && x.text.to_lowercase().contains("remember"))).map(|x| x.text.clone()).collect();
            if rest.is_empty() {
                for l in &ch.out {
                    let _ = writeln!(c, "{l}");
                }
                continue;
            }
            rest_text = rest.join(" ");
        }
        // from here on only what was not answered socially, without the address ("Mova Dev, …")
        let text: &str = if rest_text.is_empty() { &body } else { &rest_text };
        // addressed to one member ("Mova State origin, what is your LAS?"): only that member answers
        let tl = text.to_lowercase();
        if let Some((id, m)) = state_members.iter_mut().find(|(_, m)| tl.starts_with(&m.name.to_lowercase())) {
            let a = crate::state::state_las(id, &gold, "tale").map(|v| format!("{v:.2}")).unwrap_or_else(|_| "?".into());
            let r = format!("{who}, I am {}. I live in state {id}; my LAS on our tales is {a}; I have {} rules.", m.name, crate::state::state_rules(id).len());
            let name = m.name.clone();
            ch.post(Some(m), &name, &r);
            for l in &ch.out {
                let _ = writeln!(c, "{l}");
            }
            continue;
        }
        // everyone hears what the client tells (a statement, not a question or a command)
        if !text.trim_end().ends_with('?') {
            for m in std::iter::once(&mut current).chain(state_members.iter_mut().map(|(_, m)| m)).chain(commenters.iter_mut()) {
                let _ = crate::mind::tell(&mut m.story, text);
            }
        }
        // explore: every state member looks for an idea from its own state; Dev takes the best safe one (once), or runs
        // `auto N` rounds by himself: idea → new state → guard → stable or roll back, until no idea is left
        let rounds = if tl.contains("explore") { 1 } else if let Some(n) = tl.strip_prefix("auto").map(str::trim).and_then(|x| x.trim_end_matches(['.', '!']).parse::<usize>().ok()) { n } else { 0 };
        if rounds > 0 {
            ch.post(Some(&mut dev), "Mova Dev", &format!("I decided: {rounds} round{} of exploring; every state brings an idea, I take the best safe one. I keep talking while I train: ask me for the status, say stop, focus, reject, try or propose.", if rounds == 1 { "" } else { "s" }));
            ctrl().running.store(true, std::sync::atomic::Ordering::SeqCst);
            ctrl().stop.store(false, std::sync::atomic::Ordering::SeqCst);
            progress("starting");
            let g = gold.clone();
            std::thread::spawn(move || train(rounds, n_states, g));
            for l in &ch.out {
                let _ = writeln!(c, "{l}");
            }
            continue;
        }
        let before = crate::state::current_id();
        // a new message that is not his own triggers Dev, the dictator
        let said = daddy_act(&mut dev, who, text);
        ch.post(Some(&mut dev), "Mova Dev", &said);
        // Mova Current: the voice of the current state
        let cur = crate::state::current_id();
        let cur_las = crate::state::state_las(&cur, &gold, "tale").ok();
        let before_las = if before != cur { crate::state::state_las(&before, &gold, "tale").ok() } else { None };
        let msg = match (cur_las, before_las) {
            (Some(v), Some(b)) if v + 0.1 < b => format!("I object, Dev: the current state {cur} gives LAS {v:.2}, the one we left ({before}) gave {b:.2}."),
            (Some(v), _) => format!("I am the current state {cur}: LAS {v:.2} on our tales."),
            (None, _) => format!("I am the current state {cur}."),
        };
        if msg != current.last_comment {
            current.last_comment = msg.clone();
            ch.post(Some(&mut current), "Mova Current", &msg);
        }
        // the state members measure their own states against the current one
        let mut best: Option<(String, String, f64, f64)> = None;
        for (id, m) in state_members.iter_mut() {
            if *id == cur {
                continue;
            }
            let (Ok(a), Some(b)) = (crate::state::state_las(id, &gold, "tale"), cur_las) else { continue };
            let verdict = if a > b + 0.1 {
                if best.as_ref().is_none_or(|x| a > x.2) {
                    best = Some((m.name.clone(), id.clone(), a, b));
                }
                format!("Hint: switch to {id}.")
            } else if a + 0.1 < b {
                "Hint: stay, the current state is better.".to_string()
            } else {
                "Hint: no real difference.".to_string()
            };
            let (mine_r, cur_r) = (crate::state::state_rules(id), crate::state::state_rules(&cur));
            let only_mine: Vec<&String> = mine_r.iter().filter(|r| !cur_r.contains(r)).collect();
            let only_cur: Vec<&String> = cur_r.iter().filter(|r| !mine_r.contains(r)).collect();
            let rules_note = if only_mine.is_empty() && only_cur.is_empty() {
                "My rules are the same as the current ones.".to_string()
            } else {
                format!("My rules differ: {} only mine, {} only in the current state{}.", only_mine.len(), only_cur.len(), only_cur.first().map(|r| format!(" (for example {})", r.split('\t').take(2).collect::<Vec<_>>().join(" → "))).unwrap_or_default())
            };
            let msg = format!("In my state {id}: LAS {a:.2}, the current one {b:.2}. {rules_note} {verdict}");
            if msg != m.last_comment {
                m.last_comment = msg.clone();
                let name = m.name.clone();
                ch.post(Some(m), &name, &msg);
            }
        }
        // random commenters: hints from the history, the journal, the letters
        for (i, k) in commenters.iter_mut().enumerate() {
            let comment = kid_comment_source(i + 1);
            if comment == k.last_comment {
                continue;
            }
            k.last_comment = comment.clone();
            let name = k.name.clone();
            ch.post(Some(k), &name, &comment);
        }
        // the dictator decides after hearing everyone: a clearly better state wins
        if let Some((member, id, a, b)) = best.filter(|_| !ctrl().running.load(std::sync::atomic::Ordering::SeqCst)) {
            let msg = match crate::state::checkout(&id) {
                Ok(()) => format!("I decided: switch to {id} ({a:.2} against {b:.2}). {member} was right."),
                Err(e) => format!("I wanted to switch to {id}, but could not: {e}"),
            };
            ch.post(Some(&mut dev), "Mova Dev", &msg);
        }
        for l in &ch.out {
            let _ = writeln!(c, "{l}");
        }
    }
    Ok(())
}

/// The rules of a state with their scope: "domain" (tales only) or "general" (every domain).
fn scoped_of(id: &str) -> Vec<(String, crate::induce::Rule)> {
    crate::induce::parse_rules(&crate::state::state_rules_scoped(id).iter().map(|l| format!("{l}\n")).collect::<String>())
}

/// The rules that run in a domain: the tale domain runs all, any other domain only the general ones.
fn in_domain(sc: &[(String, crate::induce::Rule)], domain: &str) -> Vec<crate::induce::Rule> {
    sc.iter().filter(|(s, _)| domain == "tale" || s == "general").map(|(_, r)| r.clone()).collect()
}

/// A rule's machine line with its scope.
fn scoped_line(scope: &str, r: &crate::induce::Rule) -> String {
    let m = r.machine();
    format!("{scope}\t{}", m.split_once('\t').map(|x| x.1).unwrap_or(&m))
}

/// Apply a change to scoped rules: an added rule gets `scope`; a replaced rule keeps its own.
fn apply_scoped(sc: &[(String, crate::induce::Rule)], c: &crate::induce::Change, scope: &str) -> Vec<(String, crate::induce::Rule)> {
    use crate::induce::Change;
    match c {
        Change::Add(r) => {
            let mut v = sc.to_vec();
            if !v.iter().any(|x| x.1 == *r) {
                v.push((scope.to_string(), r.clone()));
            }
            v
        }
        Change::Remove(r) => sc.iter().filter(|x| x.1 != *r).cloned().collect(),
        Change::Replace(a, b) => sc.iter().map(|x| if x.1 == *a { (x.0.clone(), b.clone()) } else { x.clone() }).collect(),
    }
}

/// One round of exploring: ideas from every state member, the best significant one applied to the current rules as
/// a new state, then the guard: kept (stable) if not worse on the quick set, else rolled back. Returns false when no
/// idea was taken.
/// The second guard set (another domain): EWT dev, so an idea does not fit one genre only.
/// How many dev sets the state members rotate over when exploring.
const EXPLORE_SETS: usize = 8;

/// The guard's minimum tale gain in LAS points: about two words of tales-even-g (~10k words), not one.
const GUARD_GAIN: f64 = 0.02;

/// A test set: the tales test (tales-odd*) or a UD test split. The family never selects on these.
fn is_test_set(p: &std::path::Path) -> bool {
    let n = set_name(p);
    n.starts_with("tales-odd") || n.ends_with("-test")
}

fn set_name(p: &std::path::Path) -> String {
    p.file_stem().map(|x| x.to_string_lossy().into_owned()).unwrap_or_default()
}

fn guard_set() -> std::path::PathBuf {
    std::env::var("FAMILY_GUARD").map(std::path::PathBuf::from).unwrap_or_else(|_| std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("en/ud-mova/ewt-dev-g.conllu"))
}

/// The pool of working ideas (`<hot>/family/pool.tsv`): idea, member, dev gain, held gain, p, status (pooled,
/// applied, rejected), the change as a machine line ("add|remove\t<rule>").
#[derive(Clone)]
struct Idea {
    desc: String,
    member: String,
    gd: i64,
    gh: i64,
    p: f64,
    status: String,
    change: String,
    /// where the evidence comes from: "gold" (a treebank), "big" (a big store sample judged by the matrix) or
    /// "hint" (the client's proposal)
    src: String,
}

/// The negative list: rules that already proved to be nonsense (a child built on them was rolled back, or a hand /
/// teacher check rejected them), `<hot>/family/negative.tsv`: description, reason, first seen, last reviewed (unix
/// seconds), times rejected, the change line. Such an idea is not admitted again; it comes back for a review only
/// rarely — after 30 days, then 60, 120… (doubling with every repeated rejection) — and goes through the whole guard
/// again (gold, grammar, the teacher for big ideas); a passing review takes it off the list.
struct Neg {
    desc: String,
    reason: String,
    first: u64,
    last: u64,
    times: u32,
    change: String,
    /// the circumstances it was set aside in (the state and the code); when they change, it wakes
    ctx: String,
}

/// The circumstances of a thought: the current state and the code (with its data and worlds).
fn circumstances() -> String {
    format!("{}-{}", crate::state::current_id(), crate::induce::code_key())
}

const NEG_REVIEW_DAYS: u64 = 30;

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

fn neg_path() -> std::path::PathBuf {
    root().join("family").join("negative.tsv")
}

fn neg_read() -> Vec<Neg> {
    std::fs::read_to_string(neg_path())
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let c: Vec<&str> = l.splitn(7, '\t').collect();
            (c.len() >= 6).then(|| Neg { desc: c[0].into(), reason: c[1].into(), first: c[2].parse().unwrap_or(0), last: c[3].parse().unwrap_or(0), times: c[4].parse().unwrap_or(1), change: c[5].into(), ctx: c.get(6).map(|x| x.to_string()).unwrap_or_default() })
        })
        .collect()
}

fn neg_write(v: &[Neg]) {
    let body: String = std::iter::once("# description\treason\tfirst seen\tlast reviewed\ttimes rejected\tchange\tcircumstances\n".to_string()).chain(v.iter().map(|n| format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\n", n.desc, n.reason.replace('\t', " "), n.first, n.last, n.times, n.change, n.ctx))).collect();
    let _ = crate::store::write_atomic(&neg_path(), body.as_bytes());
}

/// A thought is never bad: a rejected idea is a dormant hypothesis, set aside with the circumstances it failed in. It
/// wakes when its review is due or when the circumstances change (a new state, new code, new data or worlds).
/// Is the change on the negative list, and is its review due? None — not listed. A narrower variant of a negative
/// added rule (same labels, more conditions) counts as listed too: the family looks elsewhere instead of trimming a
/// rule that proved to be nonsense; only the listed rule itself comes back for its rare review.
fn neg_state(change: &str) -> Option<bool> {
    use crate::induce::Change;
    let ctx = circumstances();
    let due = |n: &Neg| (!n.ctx.is_empty() && n.ctx != ctx) || now().saturating_sub(n.last) > NEG_REVIEW_DAYS * 86400 * (1u64 << (n.times.saturating_sub(1)).min(10));
    let list = neg_read();
    if let Some(n) = list.iter().find(|n| n.change == change) {
        return Some(due(n));
    }
    let c = change_of(change)?;
    list.iter().any(|n| matches!((change_of(&n.change), &c), (Some(Change::Add(bad)), Change::Add(r)) if bad.covers(r))).then_some(false)
}

/// A rejected idea goes on the list (or its entry is renewed: reviewed now, one more rejection).
fn neg_add(desc: &str, change: &str, reason: &str) {
    let mut v = neg_read();
    match v.iter_mut().find(|n| n.change == change) {
        Some(n) => {
            n.last = now();
            n.times += 1;
            n.reason = reason.to_string();
            n.ctx = circumstances();
        }
        None => v.push(Neg { desc: desc.into(), reason: reason.into(), first: now(), last: now(), times: 1, change: change.into(), ctx: circumstances() }),
    }
    neg_write(&v);
    crate::ideas::record("dormant", desc, reason, &circumstances(), "the parser's rules");
}

/// A reviewed idea passed: off the list.
fn neg_remove(change: &str) {
    let mut v = neg_read();
    let n = v.len();
    v.retain(|x| x.change != change);
    if v.len() != n {
        neg_write(&v);
    }
}

/// The domain sets of the family (`runs/domains-set`, split by line: a — explore, b — held-out, g — guard); the
/// domains test set is never read here. Parsed once per process.
fn domain_set(part: &str) -> Option<&'static crate::domains::Labelled> {
    static SETS: std::sync::OnceLock<std::sync::Mutex<BTreeMap<String, &'static crate::domains::Labelled>>> = std::sync::OnceLock::new();
    let m = SETS.get_or_init(|| std::sync::Mutex::new(BTreeMap::new()));
    if let Some(v) = m.lock().unwrap().get(part) {
        return Some(v);
    }
    let p = PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join(format!("runs/domains-set/domains-{part}.jsonl"));
    let v: &'static crate::domains::Labelled = Box::leak(Box::new(crate::domains::load_set(&p).ok()?));
    m.lock().unwrap().insert(part.to_string(), v);
    Some(v)
}

/// The domain knobs of a state: the shipped values with the state's `domains.cfg` over them.
fn state_cfg(id: &str) -> crate::domains::Cfg {
    let mut c: crate::domains::Cfg = crate::domains::knobs().into_iter().map(|(k, v, _)| (k.to_string(), v)).collect();
    for l in crate::state::state_file(id, "domains.cfg") {
        if let Some((k, v)) = l.split_once('=') {
            if let (true, Ok(v)) = (c.contains_key(k.trim()), v.trim().parse::<f64>()) {
                c.insert(k.trim().to_string(), v);
            }
        }
    }
    c
}

/// Knob ideas: every knob to a neighbouring value, scored on domains-a (dev) and domains-b (held-out) from the
/// member's own configuration; (description, change line "knob\t<name>\t<value>", dev gain, held gain, p).
fn explore_knobs(cfg: &crate::domains::Cfg) -> Vec<(String, String, i64, i64, f64)> {
    let (Some(a), Some(b)) = (domain_set("a"), domain_set("b")) else { return Vec::new() };
    let (ba, bb) = (crate::domains::score_set(a, cfg), crate::domains::score_set(b, cfg));
    let mut out = Vec::new();
    for (k, _, vals) in crate::domains::knobs() {
        let cur = cfg.get(k).copied().unwrap_or(0.0);
        let i = vals.iter().position(|v| (*v - cur).abs() < 1e-9);
        let near: Vec<f64> = match i {
            Some(i) => [i.checked_sub(1), Some(i + 1)].into_iter().flatten().filter_map(|j| vals.get(j).copied()).collect(),
            None => vals.clone(),
        };
        for v in near {
            let mut c2 = cfg.clone();
            c2.insert(k.to_string(), v);
            let sa = crate::domains::score_set(a, &c2);
            let diff: Vec<i64> = sa.iter().zip(&ba).map(|(x, y)| x - y).collect();
            let gd: i64 = diff.iter().sum();
            if gd <= 0 {
                continue;
            }
            let gh: i64 = crate::domains::score_set(b, &c2).iter().zip(&bb).map(|(x, y)| x - y).sum();
            out.push((format!("knob: {k} {cur} → {v}"), format!("knob\t{k}\t{v}"), gd, gh, crate::induce::boot_pub(&diff)));
        }
    }
    out.sort_by(|x, y| y.2.cmp(&x.2).then(x.0.cmp(&y.0)));
    out
}

/// The state a member speaks for: "Mova State <id>" → id, "Mova Current" → the current state.
fn member_state(name: &str, cur: &str) -> Option<String> {
    if let Some(id) = name.strip_prefix("Mova State ") {
        return Some(id.to_string());
    }
    (name == "Mova Current").then(|| cur.to_string())
}

/// From a member's memories: on which data sets its ideas were kept and rolled back — written as plain English
/// lines, the last of which is its order of preference ("My order: big-real, tales-even-a, …").
fn where_ideas_work(member: &str) -> String {
    let mem = crate::talk::memories(member);
    let mut found: BTreeMap<String, String> = BTreeMap::new();
    for (_, kind, text, _) in &mem {
        if kind == "pooled" {
            if let Some((set, key)) = text.strip_prefix("found on ").and_then(|r| r.split_once(": ")) {
                found.insert(key.to_string(), set.trim_end_matches(" (my brain chose it)").to_string());
            }
        }
    }
    let mut stats: BTreeMap<String, (u32, u32)> = BTreeMap::new();
    for (_, kind, text, _) in &mem {
        if kind != "kept" && kind != "rolled back" {
            continue;
        }
        let key = text.strip_prefix("my idea ").and_then(|r| r.split(" was ").next()).unwrap_or("");
        if let Some(set) = found.get(key) {
            let e = stats.entry(set.clone()).or_default();
            if kind == "kept" { e.0 += 1 } else { e.1 += 1 }
        }
    }
    if stats.is_empty() {
        return String::new();
    }
    let mut v: Vec<(String, u32, u32)> = stats.into_iter().map(|(k, (a, b))| (k, a, b)).collect();
    // Laplace: (kept + 1) / (tried + 2), best first
    v.sort_by(|x, y| ((y.1 + 1) * (x.1 + x.2 + 2)).cmp(&((x.1 + 1) * (y.1 + y.2 + 2))).then(x.0.cmp(&y.0)));
    let mut out: Vec<String> = v.iter().map(|(k, a, b)| format!("On {k} {a} of my ideas were kept and {b} rolled back.")).collect();
    out.push(format!("My order: {}.", v.iter().map(|x| x.0.as_str()).collect::<Vec<_>>().join(", ")));
    out.join("\n")
}

/// The member's preferred data sets, read from its brain ("My order: …"); empty without a brain or with
/// `FAMILY_BRAINS=0` (the ablation: the skeleton's rotation only).
fn brain_order(state: &str) -> Vec<String> {
    if std::env::var("FAMILY_BRAINS").as_deref() == Ok("0") {
        return Vec::new();
    }
    crate::state::brain_read_of(state).lines().find_map(|l| l.strip_prefix("My order: ")).map(|r| r.trim_end_matches('.').split(", ").map(String::from).collect()).unwrap_or_default()
}

fn pool_path() -> std::path::PathBuf {
    root().join("family").join("pool.tsv")
}

fn pool_read() -> Vec<Idea> {
    std::fs::read_to_string(pool_path())
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            // the change line has tabs inside: the source is the last column when it says gold/big
            let (l, src) = match l.rsplit_once('\t') {
                Some((a, b)) if b == "gold" || b == "big" || b == "hint" => (a, b.to_string()),
                _ => (l, "gold".to_string()),
            };
            let c: Vec<&str> = l.splitn(7, '\t').collect();
            (c.len() == 7).then(|| Idea { desc: c[0].into(), member: c[1].into(), gd: c[2].parse().unwrap_or(0), gh: c[3].parse().unwrap_or(0), p: c[4].parse().unwrap_or(1.0), status: c[5].into(), change: c[6].into(), src })
        })
        .collect()
}

fn pool_write(v: &[Idea]) {
    let _ = std::fs::create_dir_all(root().join("family"));
    let body: String = v.iter().map(|i| format!("{}\t{}\t{}\t{}\t{:.4}\t{}\t{}\t{}\n", i.desc, i.member, i.gd, i.gh, i.p, i.status, i.change, i.src)).collect();
    let _ = crate::store::write_atomic(&pool_path(), body.as_bytes());
}

/// The label pair an idea is about ("obl:agent → obl"), whatever its kind (add, remove, relax, narrow).
fn idea_key(desc: &str) -> String {
    let body = desc.split(" IF ").next().unwrap_or("");
    body.rsplit(": ").next().unwrap_or(body).to_string()
}

/// The change as a pool line; the scope column of the (first) rule says where the idea belongs.
fn change_line(c: &crate::induce::Change, scope: &str) -> String {
    match c {
        crate::induce::Change::Add(r) => format!("add\t{}", scoped_line(scope, r)),
        crate::induce::Change::Remove(r) => format!("remove\t{}", scoped_line(scope, r)),
        crate::induce::Change::Replace(a, b) => format!("replace\t{}\t=>\t{}", scoped_line(scope, a), scoped_line(scope, b)),
    }
}

/// The scope of a pooled idea ("domain" for the old lines that had no other).
fn change_scope(line: &str) -> String {
    line.split('\t').nth(1).filter(|s| *s == "general").unwrap_or("domain").to_string()
}

fn change_of(line: &str) -> Option<crate::induce::Change> {
    let (kind, rule) = line.split_once('\t')?;
    let one = |x: &str| crate::induce::parse_rules(&format!("{x}\n")).into_iter().next().map(|p| p.1);
    if kind == "replace" {
        let (a, b) = rule.split_once("\t=>\t")?;
        return Some(crate::induce::Change::Replace(one(a)?, one(b)?));
    }
    let r = one(rule)?;
    Some(if kind == "add" { crate::induce::Change::Add(r) } else { crate::induce::Change::Remove(r) })
}

/// One round: every state member brings ideas into the pool of working ideas; Dev picks up to three compatible
/// pooled ideas for a new child; the child gets Dev's brain and the guard decides (gain on the tales, no loss on EWT
/// dev); if the combination fails, Dev tries the best one alone; failed ideas are rejected. Returns false when the
/// pool has nothing left to try.
fn explore_round(ch: &mut Channel, dev: &mut Member, current: &mut Member, members: &mut [(String, Member)], gold: &std::path::Path, round: usize) -> bool {
    let cur = crate::state::current_id();
    let cur_sc = scoped_of(&cur);
    // each member searches on its own data (diversity), rotating by round: a quarter of the even tales and three UD
    // training sets (EWT, GUM, spoken learner English); the held-out check is another quarter of the even tales; the guard
    // reads the remaining half (tales-even-g) and nothing else does. The tales test (tales-odd) is never read here.
    let ud = guard_set();
    let tales_a = gold.with_file_name("tales-even-a.conllu");
    let tales_b = gold.with_file_name("tales-even-b.conllu");
    // EWT dev in halves: -h for the held-out check of general ideas, -g for the guard only
    let ewt_h = ud.with_file_name("ewt-dev-h.conllu");
    // UD training splits, not dev: several times larger, so a few-word gain can be told from chance
    let dev_sets: [std::path::PathBuf; EXPLORE_SETS] = [tales_a.clone(), ud.with_file_name("ewt-train.conllu"), ud.with_file_name("gum-train.conllu"), ud.with_file_name("eslspok-train.conllu"), std::path::PathBuf::from("big-tale"), std::path::PathBuf::from("big-real"), std::path::PathBuf::from("domains-a"), std::path::PathBuf::from("big-worlds")];
    if std::iter::once(gold).chain(dev_sets.iter().map(|p| p.as_path())).chain([tales_b.as_path(), ewt_h.as_path(), guard_set().as_path()]).any(|p| is_test_set(p)) {
        ch.post(Some(dev), "Mova Dev", &format!("I refuse to explore: a test set is among my data ({}). Tests are only for the final report.", set_name(gold)));
        return false;
    }
    // the rotation goes on across runs: a counter in <hot>/family/rotation
    let rot_path = root().join("family").join("rotation");
    let rot: usize = std::fs::read_to_string(&rot_path).ok().and_then(|x| x.trim().parse().ok()).unwrap_or(0);
    let _ = std::fs::write(&rot_path, format!("{}\n", rot + 1));
    // the client's focus: only the data sets whose name contains it (if any does)
    let focus = ctrl().focus.lock().unwrap().clone();
    let dev_sets: Vec<std::path::PathBuf> = match focus.as_deref() {
        Some(f) if dev_sets.iter().any(|p| set_name(p).contains(f)) => dev_sets.iter().filter(|p| set_name(p).contains(f)).cloned().collect(),
        _ => dev_sets.to_vec(),
    };
    let mut pool = pool_read();
    // Mova Current explores too, from the current rules themselves: its ideas fit the state a child grows from
    let mut explorers: Vec<(String, &mut Member)> = std::iter::once((cur.clone(), &mut *current)).chain(members.iter_mut().map(|(id, m)| (id.clone(), m))).collect();
    for (mi, (id, m)) in explorers.iter_mut().enumerate() {
        let id: &str = id;
        let m: &mut Member = m;
        if stopping() {
            return false;
        }
        let mut dset = &dev_sets[(mi + rot) % dev_sets.len()];
        let mut steered = false;
        if rot % 3 != 2 {
            // the member's brain chooses: its best data set first (two rounds of three; the third explores)
            let order = brain_order(id);
            if let Some(best) = order.iter().find_map(|o| dev_sets.iter().find(|p| set_name(p) == *o)) {
                dset = best;
                steered = true;
            }
        }
        progress(&format!("round {round}: {} explores from state {id} on {}{}", m.name, set_name(dset), if steered { " (chosen by its brain)" } else { "" }));
        // an idea belongs to the domain of its data: on tales it is a tale rule (scope "domain"), held out on other
        // tales; on UD it is a general rule, searched over the general rules only and held out on half of EWT dev
        // the domain modules' knobs: ideas of a different kind (a knob to a neighbouring value)
        if set_name(dset) == "domains-a" {
            let cur_cfg = state_cfg(&cur);
            let mut said: Vec<String> = Vec::new();
            for (desc, line, gd, gh, p) in explore_knobs(&state_cfg(id)) {
                let (k, v) = line.split('\t').skip(1).fold((String::new(), String::new()), |acc, x| if acc.0.is_empty() { (x.to_string(), acc.1) } else { (acc.0, x.to_string()) });
                let holds = v.parse::<f64>().ok().is_some_and(|v| cur_cfg.get(&k).is_some_and(|c| (c - v).abs() < 1e-9));
                if holds || pool.iter().any(|i| i.desc == desc) || neg_state(&line) == Some(false) || ctrl().banned.lock().unwrap().contains(&idea_key(&desc)) {
                    continue;
                }
                if p < 0.2 && gh >= 0 && gd + gh >= 2 {
                    pool.push(Idea { desc: desc.clone(), member: m.name.clone(), gd, gh, p, status: "pooled".into(), change: line, src: "knob".into() });
                    crate::talk::remember(&m.name, "pooled", &format!("found on domains-a: {}", idea_key(&desc)), 3);
                    said.push(format!("{desc} (dev {gd:+}, held {gh:+}, p {p:.3})"));
                }
            }
            let msg = if said.is_empty() { format!("Round {round}, from my state {id} on the domain knobs: nothing new for the pool.") } else { format!("Round {round}, from my state {id} on the domain knobs into the pool: {}.", said.join("; ")) };
            if msg != m.last_comment {
                m.last_comment = msg.clone();
                let name = m.name.clone();
                ch.post(Some(m), &name, &msg);
            }
            continue;
        }
        let big = set_name(dset).starts_with("big-");
        let tale = *dset == tales_a || set_name(dset) == "big-tale";
        let (dom, scope, held) = if tale { ("tale", "domain", tales_b.clone()) } else { ("real", "general", ewt_h.clone()) };
        let cur_rules = in_domain(&cur_sc, dom);
        let found = if big {
            // each pass over the sets reads fresh text: bucket 2..15 by the rotation counter (bucket 1 is the guard's)
            let bucket = 2 + (rot / EXPLORE_SETS) % 14;
            crate::big::sample(&format!("{}@{bucket}", set_name(dset))).and_then(|b| crate::induce::explore_big(in_domain(&scoped_of(id), dom), &b, dom, &held, &cur_rules, 5))
        } else {
            crate::induce::explore_changes(in_domain(&scoped_of(id), dom), dset, &held, &cur_rules, 5)
        };
        // gold ideas need a held-out gain; big ideas (thousands of sentences, matrix-judged) a strict p and no
        // held-out loss — the small gold set rarely sees their constructions
        let admit = |p: f64, gh: i64| if big { p < 0.01 && gh >= 0 } else { p < 0.1 && gh > 0 };
        let msg = match found {
            Ok(found) if !found.is_empty() => {
                let mut said: Vec<String> = Vec::new();
                let mut near: Option<String> = None;
                let mut known_bad = 0;
                for (desc, change, gd, gh, p) in found {
                    if pool.iter().any(|i| i.desc == desc) || change.holds_in(&cur_rules) || ctrl().banned.lock().unwrap().contains(&idea_key(&desc)) {
                        continue;
                    }
                    // a dormant hypothesis stays aside until its review is due or its circumstances change
                    if neg_state(&change_line(&change, scope)) == Some(false) {
                        known_bad += 1;
                        continue;
                    }
                    if near.is_none() && !admit(p, gh) {
                        near = Some(format!("closest: {desc} (dev {gd:+}, held {gh:+}, p {p:.3}) — {}", if p >= 0.1 || (big && p >= 0.01) { "could be chance" } else { "no held-out gain" }));
                    }
                    if admit(p, gh) {
                        pool.push(Idea { desc: desc.clone(), member: m.name.clone(), gd, gh, p, status: "pooled".into(), change: change_line(&change, scope), src: if big { "big".into() } else { "gold".into() } });
                        crate::talk::remember(&m.name, "pooled", &format!("found on {}{}: {}", set_name(dset), if steered { " (my brain chose it)" } else { "" }, idea_key(&desc)), 3);
                        said.push(format!("{desc} (dev {gd:+}, held {gh:+}, p {p:.3})"));
                    }
                }
                let kb = if known_bad > 0 { format!(" ({known_bad} dormant hypotheses kept aside)") } else { String::new() };
                if said.is_empty() {
                    format!("Round {round}, from my state {id} on {}: nothing new for the pool{kb}{}.", set_name(dset), near.map(|x| format!("; {x}")).unwrap_or_default())
                } else {
                    format!("Round {round}, from my state {id} on {} into the pool{kb}: {}.", set_name(dset), said.join("; "))
                }
            }
            Ok(_) => format!("Round {round}, from my state {id} on {}: nothing that helps.", set_name(dset)),
            Err(e) => format!("I could not explore: {e}"),
        };
        if msg != m.last_comment {
            m.last_comment = msg.clone();
            let name = m.name.clone();
            ch.post(Some(m), &name, &msg);
        }
    }
    for i in pool.iter_mut().filter(|i| i.status == "pooled") {
        let dom = if change_scope(&i.change) == "general" { "real" } else { "tale" };
        if i.src == "knob" {
            continue;
        }
        if change_of(&i.change).is_some_and(|c| c.holds_in(&in_domain(&cur_sc, dom))) || (i.src == "gold" && i.gh <= 0) || ctrl().banned.lock().unwrap().contains(&idea_key(&i.desc)) || neg_state(&i.change) == Some(false) {
            i.status = "rejected".into();
        }
    }
    pool_write(&pool);
    // Dev picks: the best pooled ideas, at most three, not touching the same label pair
    let mut cand: Vec<usize> = (0..pool.len()).filter(|&i| pool[i].status == "pooled").collect();
    cand.sort_by(|&a, &b| (pool[b].gd + pool[b].gh).cmp(&(pool[a].gd + pool[a].gh)).then(pool[a].desc.cmp(&pool[b].desc)));
    // the client's word goes first: ideas asked for by `try`, then the client's own proposals
    let first = std::mem::take(&mut *ctrl().first.lock().unwrap());
    cand.sort_by_key(|&i| if first.contains(&pool[i].desc) { 0 } else if pool[i].src == "hint" { 1 } else { 2 });
    if stopping() {
        return false;
    }
    let mut pick: Vec<usize> = Vec::new();
    // one child, one scope: the ideas of the best one's domain, so the guard knows where to look for the gain
    let first_scope = cand.first().map(|&i| (change_scope(&pool[i].change), pool[i].src.clone())).unwrap_or_default();
    for i in cand {
        let key = idea_key;
        if pick.len() < 3 && (change_scope(&pool[i].change), pool[i].src.clone()) == first_scope && !pick.iter().any(|&j| key(&pool[j].desc) == key(&pool[i].desc)) {
            pick.push(i);
        }
    }
    if pick.is_empty() {
        return false;
    }
    let try_rules = |ch: &mut Channel, dev: &mut Member, current: &mut Member, pool: &[Idea], set: &[usize]| -> Option<bool> {
        let scope = change_scope(&pool[set[0]].change);
        let mut rules = cur_sc.clone();
        for &i in set {
            rules = apply_scoped(&rules, &change_of(&pool[i].change)?, &scope);
        }
        let body: String = rules.iter().map(|(sc, r)| format!("{}\n", scoped_line(sc, r))).collect();
        let names: Vec<String> = set.iter().map(|&i| format!("{} ({})", pool[i].desc, pool[i].member)).collect();
        let letter = format!("Dev took from the pool: {}. Decision: a new child with these ideas; the guard decides if it stays.", names.join("; "));
        let hot = crate::induce::hot_dir();
        let res = std::fs::create_dir_all(&hot).map_err(anyhow::Error::from).and_then(|_| crate::store::write_atomic(&hot.join("rules-induced.tsv"), format!("# hot layer written by the family\n{body}").as_bytes())).and_then(|_| crate::state::transition(&format!("family round {round}: {} idea(s) from the pool", set.len()), Some(&letter), &[], &[]));
        let new = res.ok()?;
        progress(&format!("round {round}: guarding the child {new} ({})", names.join("; ")));
        ch.post(Some(dev), "Mova Dev", &format!("I decided: a new child {new} from the pool — {}. It inherits my brain.", names.join("; ")));
        let g2 = guard_set();
        let (a, b) = (crate::state::state_las(&new, gold, "tale").unwrap_or(0.0), crate::state::state_las(&cur, gold, "tale").unwrap_or(0.0));
        let (e, f) = (crate::state::state_las(&new, &g2, "real").unwrap_or(0.0), crate::state::state_las(&cur, &g2, "real").unwrap_or(0.0));
        // the guard looks for the gain in the idea's own domain and for no loss (> 0.05) in the other one
        let general = scope == "general";
        let (gain, own, other_loss) = if general { (e - f, "EWT dev", b - a) } else { (a - b, "the tales", f - e) };
        // tolerances add up over generations, so they are also checked against the original: the own domain may be
        // at most 0.01 and the other at most 0.05 below it, however many children came before
        let (oa, oe) = (crate::state::state_las("origin", gold, "tale").unwrap_or(0.0), crate::state::state_las("origin", &g2, "real").unwrap_or(0.0));
        let (own_drift, other_drift) = if general { (oe - e, oa - a) } else { (oa - a, oe - e) };
        let drift_ok = own_drift <= 0.01 && other_drift <= 0.05;
        // a big idea passes on its own evidence: fewer absurd arcs on the disjoint guard sample (p < 0.05), no loss
        // on the gold of its own domain (> 0.01) nor of the other (> 0.05)
        if pool[set[0]].src == "big" {
            let gname = if general { "big-real-guard" } else { "big-tale-guard" };
            let dom = if general { "real" } else { "tale" };
            let verdict = crate::big::sample(gname).map(|g| {
                let s0 = crate::induce::big_score(&in_domain(&cur_sc, dom), &g, dom);
                let s1 = crate::induce::big_score(&in_domain(&rules, dom), &g, dom);
                let diff: Vec<i64> = s1.iter().zip(&s0).map(|(x, y)| y.0 - x.0).collect();
                (diff.iter().sum::<i64>(), s1.iter().zip(&s0).map(|(x, y)| x.1 - y.1).sum::<i64>(), crate::induce::boot_pub(&diff))
            });
            let (removed, sensible, p) = verdict.unwrap_or((0, 0, 1.0));
            let mut ok = removed > 0 && 2 * sensible >= removed && p < 0.05 && gain >= -0.01 && other_loss <= 0.05 && drift_ok;
            // the teacher's check (2026-10-03: the matrix alone let vocatives and wh-words become subjects): 20 of
            // the relabelled words of the guard sample, judged by Opus; the new label must win clearly
            let mut teacher = String::new();
            if ok {
                if let Ok(g) = crate::big::sample(gname) {
                    let (items, total) = crate::induce::changed_words(&in_domain(&cur_sc, dom), &in_domain(&rules, dom), &g, 20);
                    match crate::big::judge_changes(&items) {
                        Ok((nr, or, ne)) => {
                            let decided = nr + or;
                            ok = decided >= 10 && nr * 10 >= decided * 7 && nr > or;
                            teacher = format!("; teacher on {} of {total} changed words: new right {nr}, old right {or}, neither {ne}", items.len());
                        }
                        Err(e) => {
                            ok = false;
                            teacher = format!("; the teacher could not judge: {e}");
                        }
                    }
                }
            }
            let teacher = teacher;
            let m = format!("the child {new} on {gname}: absurd −{removed}, sensible {sensible:+}, p {p:.3}; tales {a:.2} (was {b:.2}, original {oa:.2}), EWT dev {e:.2} (was {f:.2}, original {oe:.2}){}{teacher}", if drift_ok { "" } else { " — too far below the original" });
            if ok {
                let _ = crate::state::mark_stable(&new);
                ch.post(Some(current), "Mova Current", &format!("I am the new child — {m}. I stay — stable now."));
                ctrl().done.lock().unwrap().push(format!("kept {new}"));
            } else {
                let _ = crate::state::rollback();
                ch.post(Some(current), "Mova Current", &format!("Rolled back: {m}. Back to my father."));
                *ctrl().verdict.lock().unwrap() = m.clone();
                ctrl().done.lock().unwrap().push(format!("rolled back {new}"));
            }
            return Some(ok);
        }
        if gain >= GUARD_GAIN && other_loss <= 0.05 && drift_ok {
            let _ = crate::state::mark_stable(&new);
            let m = format!("I am the new child: tales {a:.2} (was {b:.2}), EWT dev {e:.2} (was {f:.2}). I stay — stable now.");
            ch.post(Some(current), "Mova Current", &m);
            ctrl().done.lock().unwrap().push(format!("kept {new}"));
            Some(true)
        } else {
            let why = if !drift_ok {
                format!("too far below the original (tales {a:.2} vs {oa:.2}, EWT dev {e:.2} vs {oe:.2})")
            } else if gain < GUARD_GAIN {
                if general { format!("no gain on {own} ({e:.3} against {f:.3})") } else { format!("no gain on {own} ({a:.3} against {b:.3})") }
            } else if general {
                format!("a loss on the tales ({a:.2} against {b:.2})")
            } else {
                format!("a loss on EWT dev ({e:.2} against {f:.2})")
            };
            let _ = crate::state::rollback();
            ch.post(Some(current), "Mova Current", &format!("The child {new}: {why}. Back to my father."));
            *ctrl().verdict.lock().unwrap() = format!("the child {new}: {why}");
            ctrl().done.lock().unwrap().push(format!("rolled back {new}"));
            Some(false)
        }
    };
    let try_child = |ch: &mut Channel, dev: &mut Member, current: &mut Member, pool: &[Idea], set: &[usize]| -> Option<bool> {
        if pool[set[0]].src == "knob" {
            // a knob child: the current configuration with these knobs, written as the hot domains.cfg; the guard
            // wants at least 2 more right domain decisions on domains-g (100 sentences) and no loss anywhere else
            let base = state_cfg(&cur);
            let mut cfg = base.clone();
            for &i in set {
                let c: Vec<&str> = pool[i].change.split('\t').collect();
                if let (Some(k), Some(Ok(v))) = (c.get(1), c.get(2).map(|x| x.parse::<f64>())) {
                    cfg.insert(k.to_string(), v);
                }
            }
            let body: String = cfg.iter().map(|(k, v)| format!("{k}={v}\n")).collect();
            let names: Vec<String> = set.iter().map(|&i| format!("{} ({})", pool[i].desc, pool[i].member)).collect();
            let hot = crate::induce::hot_dir();
            let letter = format!("Dev took from the pool: {}. Decision: a new child with these knobs; the guard decides if it stays.", names.join("; "));
            let new = std::fs::create_dir_all(&hot).map_err(anyhow::Error::from).and_then(|_| crate::store::write_atomic(&hot.join("domains.cfg"), format!("# domain knobs written by the family\n{body}").as_bytes())).and_then(|_| crate::state::transition(&format!("family round {round}: {} knob(s) from the pool", set.len()), Some(&letter), &[], &[])).ok()?;
            ch.post(Some(dev), "Mova Dev", &format!("I decided: a new child {new} from the pool — {}. It inherits my brain.", names.join("; ")));
            let g = domain_set("g")?;
            let (s0, s1) = (crate::domains::score_set(g, &base), crate::domains::score_set(g, &cfg));
            let gain: i64 = s1.iter().zip(&s0).map(|(x, y)| x - y).sum();
            if gain >= 2 {
                let _ = crate::state::mark_stable(&new);
                ch.post(Some(current), "Mova Current", &format!("I am the new child {new}: {gain:+} right domain decisions on the guard set. I stay — stable now."));
                ctrl().done.lock().unwrap().push(format!("kept {new}"));
                Some(true)
            } else {
                let why = format!("the child {new}: {gain:+} domain decisions on the guard set (need +2)");
                let _ = crate::state::rollback();
                ch.post(Some(current), "Mova Current", &format!("The {why}. Back to my father."));
                *ctrl().verdict.lock().unwrap() = why;
                ctrl().done.lock().unwrap().push(format!("rolled back {new}"));
                Some(false)
            }
        } else {
            try_rules(ch, dev, current, pool, set)
        }
    };
    let ok = try_child(ch, dev, current, &pool, &pick).unwrap_or(false);
    let key = idea_key;
    let mut pool = pool_read();
    if ok {
        for &i in &pick {
            pool[i].status = "applied".into();
            crate::ideas::record("held", &pool[i].desc.clone(), &format!("dev {:+}, held {:+}, p {:.3}", pool[i].gd, pool[i].gh, pool[i].p), &circumstances(), "the parser's rules");
            neg_remove(&pool[i].change);
            crate::talk::remember(&pool[i].member, "kept", &format!("my idea {} was kept", idea_key(&pool[i].desc)), 8);
        }
    } else if pick.len() > 1 {
        let one = [pick[0]];
        let ok1 = try_child(ch, dev, current, &pool, &one).unwrap_or(false);
        pool[pick[0]].status = if ok1 { "applied".into() } else { "rejected".into() };
        if ok1 {
            neg_remove(&pool[pick[0]].change);
            crate::talk::remember(&pool[pick[0]].member, "kept", &format!("my idea {} was kept", idea_key(&pool[pick[0]].desc)), 8);
        } else {
            neg_add(&pool[pick[0]].desc, &pool[pick[0]].change, &ctrl().verdict.lock().unwrap().clone());
            crate::talk::remember(&pool[pick[0]].member, "rolled back", &format!("my idea {} was rolled back: {}", idea_key(&pool[pick[0]].desc), ctrl().verdict.lock().unwrap()), 5);
        }
    } else {
        pool[pick[0]].status = "rejected".into();
        neg_add(&pool[pick[0]].desc, &pool[pick[0]].change, &ctrl().verdict.lock().unwrap().clone());
        crate::talk::remember(&pool[pick[0]].member, "rolled back", &format!("my idea {} was rolled back: {}", idea_key(&pool[pick[0]].desc), ctrl().verdict.lock().unwrap()), 5);
    }
    // a member that has lived through enough reflects on it (one memory, said aloud)
    for &i in &pick {
        let m = pool[i].member.clone();
        if let Some(r) = crate::talk::reflect(&m) {
            ch.post(None, &m, &r);
        }
        // and writes into its brain where its ideas work — the brain then steers where it looks next
        if let Some(id) = member_state(&m, &cur) {
            let body = where_ideas_work(&m);
            if !body.is_empty() {
                let _ = crate::state::brain_section_of(&id, "Where my ideas work", &body);
            }
        }
    }
    // a failed idea alone takes its weaker pooled variants (same label pair, other conditions) with it: they were
    // ranked below it on the same data and would only repeat the same miss
    if !ok {
        let k0 = key(&pool[pick[0]].desc);
        if pool[pick[0]].status == "rejected" {
            for i in pool.iter_mut().filter(|i| i.status == "pooled" && key(&i.desc) == k0) {
                i.status = "rejected".into();
            }
        }
    }
    // after a child stays, pooled variants of an applied idea (same label pair, other conditions) are superseded:
    // they were measured against the old rules, and the applied one already covers most of their words
    let done: Vec<String> = pick.iter().filter(|&&i| pool[i].status == "applied").map(|&i| key(&pool[i].desc)).collect();
    for i in pool.iter_mut().filter(|i| i.status == "pooled" && done.contains(&key(&i.desc))) {
        i.status = "superseded".into();
    }
    pool_write(&pool);
    current.last_comment.clear();
    true
}

/// `world family say`.
pub fn say(who: &str, text: &str) -> Result<String> {
    if who.to_lowercase().contains("mova") {
        bail!("the name «{who}» is taken: only the family is called Mova");
    }
    let mut c = UnixStream::connect(sock()).context("the family is not running: start `world family serve`")?;
    writeln!(c, "{who}: {text}")?;
    let mut out = String::new();
    for l in BufReader::new(&c).lines() {
        out += &l?;
        out.push('\n');
    }
    Ok(out)
}

/// `world family log`.
pub fn log(n: usize) -> Result<()> {
    let t = std::fs::read_to_string(log_path()).unwrap_or_default();
    let lines: Vec<&str> = t.lines().collect();
    for l in &lines[lines.len().saturating_sub(n)..] {
        println!("{l}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_lines_round_trip_with_scope() {
        let r = |l: &str| crate::induce::parse_rules(&format!("{l}\n")).into_iter().next().unwrap().1;
        let a = r("general\tnsubj\tobl\t17:1");
        let b = r("general\tnsubj\tobl\t0:7,17:1");
        for (c, scope) in [(crate::induce::Change::Add(a.clone()), "general"), (crate::induce::Change::Remove(a.clone()), "domain"), (crate::induce::Change::Replace(a.clone(), b.clone()), "general")] {
            let line = change_line(&c, scope);
            assert_eq!(change_scope(&line), scope);
            let back = change_of(&line).expect("parses back");
            assert_eq!(change_line(&back, scope), line);
        }
        assert_eq!(change_scope("add\tdomain\tnsubj\tobl\t17:1"), "domain");
    }

    #[test]
    fn idea_keys_group_variants_of_one_label_pair() {
        assert_eq!(idea_key("add: obl:agent → obl IF after head = yes"), "obl:agent → obl");
        assert_eq!(idea_key("narrow (+ word is = AUX): parataxis → conj IF head is = VERB"), "parataxis → conj");
        assert_eq!(idea_key("relax (head finite = yes dropped): obj → nsubj IF head finite = yes [big: absurd −5]"), "obj → nsubj");
        assert_ne!(idea_key("add: obj → nsubj IF x"), idea_key("add: nsubj → obj IF x"));
    }

    /// Local only (needs the domain sets): from a deliberately spoiled configuration the knob explorer proposes the
    /// way back — the negative control that it can find anything at all.
    #[test]
    #[ignore]
    fn knob_explorer_finds_the_way_back() {
        let mut cfg: crate::domains::Cfg = crate::domains::knobs().into_iter().map(|(k, v, _)| (k.to_string(), v)).collect();
        cfg.insert("main_sense_only".into(), 0.0);
        cfg.insert("single_word_content_only".into(), 0.0);
        let ideas = explore_knobs(&cfg);
        for i in &ideas {
            eprintln!("{} dev {:+} held {:+} p {:.3}", i.0, i.2, i.3, i.4);
        }
        assert!(ideas.iter().any(|i| i.0.contains("main_sense_only 0 → 1") || i.0.contains("single_word_content_only 0 → 1")));
    }

    #[test]
    fn test_sets_are_refused() {
        assert!(is_test_set(std::path::Path::new("/x/tales-odd-b.conllu")));
        assert!(is_test_set(std::path::Path::new("/x/ewt-test.conllu")));
        assert!(!is_test_set(std::path::Path::new("/x/tales-even-g.conllu")), "negative control: a dev split is allowed");
        assert!(!is_test_set(std::path::Path::new("/x/ewt-dev-h.conllu")));
    }
}
