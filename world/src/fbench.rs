//! The frame benchmark (the yardstick of the pragmatic reader, docs/pragmatics.md): ~240 lesson lines sampled once,
//! deterministically, from six courses; the teacher (Opus) wrote their frames once; the scorer
//! (`prag::fbench`) compares the reader's frames with them role by role, with no more calls.
//!
//!   world frames bench-make             — sample the lines (lines.tsv) and ask the teacher for the missing gold
//!   world frames bench [dev|held] [baseline] [worst N]  — score the reader (or the trivial baseline) on a split
//!
//! Files in `$MOVA_DATA/runs/prag-bench/`: lines.tsv (id, split, course, lesson, line index, text), gold.tsv (id, #k,
//! frame), notes.tsv (id, note), calls.jsonl, raw/ (the teacher's answers as they came).

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use prag::fbench::{GFrame, Prf, ROLES, overall, parse_frame, score_line};

use crate::frames::Ctx;

pub const COURSES: [&str; 6] = ["books", "world", "biology", "everyday", "history", "reflection"];
const PER_COURSE: usize = 40;

fn dir() -> PathBuf {
    PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("runs/prag-bench")
}

#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub split: String,
    pub course: String,
    pub lesson: usize,
    pub line: usize,
    pub text: String,
}

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn fnv(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    h
}

/// The kinds of line the sample covers in turn: quoted speech, cause/order markers, pronouns, long, short, any.
fn cats(text: &str) -> [bool; 6] {
    let low = format!(" {} ", text.to_lowercase().replace([',', ';', '.', ':', '!', '?'], " "));
    let has = |ws: &[&str]| ws.iter().any(|w| low.contains(&format!(" {w} ")));
    let n = text.split_whitespace().count();
    [
        text.contains('"') || text.contains('“') || text.contains('«'),
        has(&["because", "so", "then", "since", "after", "when", "until"]),
        has(&["he", "she", "it", "they", "him", "her", "them", "his", "its", "their"]),
        n > 30,
        n < 14,
        true,
    ]
}

/// The sample: per course, the lines in a hashed order, taken round-robin by kind; every 4th taken goes held-out.
pub fn sample() -> Result<Vec<Item>> {
    let mut out = Vec::new();
    for c in COURSES {
        let mut all: Vec<(u64, usize, usize, String)> = Vec::new();
        for l in crate::knowledge::course(c)? {
            for (li, t) in l.explain.iter().chain(&l.facts).enumerate() {
                all.push((mix(fnv(&format!("{c}/{}/{li}", l.n))), l.n, li, t.clone()));
            }
        }
        all.sort();
        let mut taken: BTreeSet<usize> = BTreeSet::new();
        let mut picks: Vec<usize> = Vec::new();
        let mut cat = 0;
        let mut stuck = 0;
        while picks.len() < PER_COURSE && stuck < 6 {
            match all.iter().enumerate().find(|(i, x)| !taken.contains(i) && cats(&x.3)[cat]) {
                Some((i, _)) => {
                    taken.insert(i);
                    picks.push(i);
                    stuck = 0;
                }
                None => stuck += 1,
            }
            cat = (cat + 1) % 6;
        }
        for (k, &i) in picks.iter().enumerate() {
            let (_, n, li, t) = &all[i];
            out.push(Item { id: out.len(), split: if k % 4 == 3 { "held".into() } else { "dev".into() }, course: c.into(), lesson: *n, line: *li, text: t.clone() });
        }
    }
    Ok(out)
}

pub fn load_items() -> Result<Vec<Item>> {
    let t = std::fs::read_to_string(dir().join("lines.tsv")).context("lines.tsv: run `world frames bench-make` first")?;
    t.lines().skip(1).filter(|l| !l.is_empty()).map(|l| {
        let c: Vec<&str> = l.splitn(6, '\t').collect();
        if c.len() < 6 {
            bail!("lines.tsv: «{l}»");
        }
        Ok(Item { id: c[0].parse()?, split: c[1].into(), course: c[2].into(), lesson: c[3].parse()?, line: c[4].parse()?, text: c[5].into() })
    }).collect()
}

/// The gold: id → frames in order.
pub fn load_gold() -> Result<BTreeMap<usize, Vec<GFrame>>> {
    let mut g: BTreeMap<usize, Vec<(usize, GFrame)>> = BTreeMap::new();
    let p = dir().join("gold.tsv");
    if !p.exists() {
        return Ok(BTreeMap::new());
    }
    for l in std::fs::read_to_string(p)?.lines().skip(1) {
        let c: Vec<&str> = l.splitn(3, '\t').collect();
        if c.len() < 3 {
            continue;
        }
        let id: usize = c[0].parse()?;
        let k: usize = c[1].trim_start_matches('#').parse()?;
        let e = g.entry(id).or_default();
        if let Some(f) = parse_frame(c[2]) {
            e.push((k, f));
        }
    }
    Ok(g.into_iter().map(|(id, mut v)| {
        v.sort_by_key(|x| x.0);
        (id, v.into_iter().map(|x| x.1).collect())
    }).collect())
}

const PROMPT: &str = r#"You write the gold standard for a small transparent reader that turns English sentences into frames (their pragmatic structure: who did what to whom, where, why, what next, who said what). For each numbered TARGET line write the frames of that line only. The context lines before it (from the same lesson) are given only so that you can resolve pronouns; write no frames for them.

A frame is one predicate, written on one line:
`#k Kind agent [NOT] verb [patient=E] [to=E] [place=E] [<prep>=E ...] [attrs=a,b] [| says→#i,#j cause→#k next→#k quoted if]`

Predicates (one frame each), numbered #0, #1, ... in the order their predicate word appears in the line:
- every verb that names an action, state or happening: finite verbs, infinitives and -ing forms with their own action ("rushes off to get ready" → rush off, get), and a participle describing a noun when it says what that noun does ("the squirming baby" → Event baby squirm);
- every copula predicate ("is round", "is a mammal", "is in the wood");
- auxiliaries and modals (be/have/do as helpers, will, would, shall, can, could, must, may, might, should, "be going to") are never frames of their own.

Kind:
- Event — a particular action or happening;
- Fact(is a) — "X is a/the Y"; Fact(is) — "X is ADJ", "X is in/like ..."; Fact(has) — have = possess or contain; Fact(can) — any predicate with can/could/cannot; Fact(does) — a general timeless statement about a kind or about "you"/people ("bees make honey", "you breathe through your nose"); Fact(lives in) — live in/at a place; Fact(made of) — passive "is made of/from"; Fact(used for) — passive "is used for/to";
- Say(act) — speech or thought (say, tell, ask, answer, reply, shout, cry, call, whisper, mutter, think, wonder, explain, promise, beg, warn, exclaim, scream, add, remark, sing ...). act = what the utterance does, judged by its content: assert (a statement, also a thought), ask (a question, also wondering whether), request (an order or plea), promise, warn, thank, apologize, greet, agree, disagree, express (exclaiming a feeling: shout "Oh dear!"), suggest, offer, other;
- Bare — only for a line that has no predicate at all.

Roles:
- agent: the subject of an active verb; the subject of a copula ("the Earth is round" → Fact(is) earth be attrs=round); for a passive, the "by" phrase (else _), the passive subject is then the patient. A verb with no subject of its own takes the subject it shares with the verb it is coordinated with or controlled by ("Alice wants to go home" → want and go both have agent alice; "flings the baby and rushes off" → both have the same agent). Write _ when there is no agent at all.
- verb: the lemma in lower case with its particle ("put down", "rush off", "make up", "look after"); "be" for a copula.
- patient: the direct object, or the passive subject, or the Y of "X is a Y". A clause that is what is said, thought, known, seen or feared is NOT a patient: it goes in says→.
- to=: the recipient or the person spoken to (indirect object, "to Alice", the one told or asked).
- place=: where (in, on, at, under, into, onto, from, near, behind, through, across, inside ... a place).
- other prepositional phrases: prep=E (with=queen, for=croquet, about=cat, at=alice).
- times ("on Monday", "every day", "for a week", "in a day or two"), amounts and degrees are NOT roles: put them in attrs as when:day, or leave them out. attrs (adjectives of a copula, manner adverbs) are not scored; keep them short.

Entities:
- the lemma of the head noun, lower case, singular, one word: "the Cheshire Cat" → cat, "the March Hare" → hare, "Alice" → alice, "his big sister" → sister, "New Zealand" → zealand, "star-fish" → star-fish. Coordinated: hatter+hare.
- a personal pronoun in a role is written as what it refers to, with the pronoun as written in «»: baby(«it»), alice(«she»), cat(«him»), children(«they»). If it refers to nothing nameable (a dummy "it rains", "it is easy to ...", "it depends") or to a whole clause or situation, write the pronoun itself: it.
- generic "you", "we", "one" in the narration or in a general statement: you, we, one.
- inside quoted speech, I/me/my → the speaker: rabbit(«I»); you → the one spoken to, if known: alice(«you»).
- relative pronouns (who, which, that, whose) are replaced by the noun they stand for, with no «».
- possessives are not roles ("her sister's book" → book).

Links (only when the text really says so; each frame has at most one cause and one next):
- cause→#k: #k is this frame's cause or purpose. "A because/since/as B" → A cause→B; "A to/in order to/so that B" (purpose) → A cause→B; "A, so B" / "A; therefore B" → B cause→A.
- next→#k: #k happened right after this frame in the time of the story: "A and then B", "A, then B", "after A, B", "when A, B", "A before B", actions told one after another ("flings the baby and rushes off" → fling next→rush off). No next between states, general facts, definitions, or things at the same time.
- says→#i,#j: the frames of what a Say frame says, asks or thinks (quoted speech or a that/whether/wh-/if clause), and also what a verb of knowing, seeing, hearing, fearing, hoping, deciding, realizing, remembering, noticing, meaning has as its clause; only the top frame(s) of that content.
- quoted: the frame's predicate is inside quotation marks; if: it is inside an if/unless clause.
- NOT before the verb when it is negated.

Example. Context: "Alice is holding the Duchess's baby." Target: The Duchess flings the baby at Alice and rushes off to get ready for croquet; Alice carries the squirming baby outside, afraid that they would kill it.
=== 7
#0 Event duchess fling patient=baby at=alice | next→#1
#1 Event duchess rush off | cause→#2
#2 Event duchess get attrs=ready for=croquet
#3 Event alice carry patient=baby attrs=outside
#4 Event baby squirm
#5 Fact(is) alice be attrs=afraid | says→#6
#6 Event they kill patient=baby(«it»)
note: «they» are the people of the Duchess's house and cannot be named from the line; «it» is the baby.

Example. Target: The White Rabbit says "I shall be late!" and runs past, because it is in a hurry.
=== 8
#0 Say(express) rabbit say | says→#1 next→#2
#1 Fact(is) rabbit(«I») be attrs=late | quoted
#2 Event rabbit run past | cause→#3
#3 Fact(is) rabbit(«it») be attrs=hurry
note: -

Answer ONLY in this format: for each target, a line `=== <id>`, its frames, then one line `note: <the hard points, short, or ->`. Nothing else.

"#;

/// `world frames bench-make`: the lines sampled once (lines.tsv is kept if it exists), then the teacher writes the
/// gold of the lines that have none yet, 20 per call. Fail-fast: a failed call stops the run (it can be resumed).
pub fn make() -> Result<String> {
    let d = dir();
    std::fs::create_dir_all(d.join("raw"))?;
    let lp = d.join("lines.tsv");
    let items = if lp.exists() {
        load_items()?
    } else {
        let v = sample()?;
        let mut s = String::from("id\tsplit\tcourse\tlesson\tline\ttext\n");
        for x in &v {
            s += &format!("{}\t{}\t{}\t{}\t{}\t{}\n", x.id, x.split, x.course, x.lesson, x.line, x.text.replace(['\t', '\n'], " "));
        }
        std::fs::write(&lp, s)?;
        v
    };
    if std::env::var("BENCH_SAMPLE_ONLY").is_ok() {
        return Ok(format!("{} lines sampled\n", items.len()));
    }
    let have = load_gold()?;
    let todo: Vec<&Item> = items.iter().filter(|x| !have.contains_key(&x.id)).collect();
    let gp = d.join("gold.tsv");
    if !gp.exists() {
        std::fs::write(&gp, "id\tk\tframe\n")?;
    }
    let np = d.join("notes.tsv");
    if !np.exists() {
        std::fs::write(&np, "id\tnote\n")?;
    }
    // the lesson lines for the context
    let mut lessons: BTreeMap<(String, usize), Vec<String>> = BTreeMap::new();
    for c in COURSES {
        for l in crate::knowledge::course(c)? {
            lessons.insert((c.to_string(), l.n), l.explain.iter().chain(&l.facts).cloned().collect());
        }
    }
    let opus = prag::opus::Opus::from_env(d.join("empty"));
    let (mut done, mut frames) = (0, 0);
    let max: usize = std::env::var("BENCH_BATCHES").ok().and_then(|x| x.parse().ok()).unwrap_or(usize::MAX);
    for (bi, chunk) in todo.chunks(20).enumerate().take(max) {
        let mut prompt = PROMPT.to_string();
        for x in chunk {
            let ls = &lessons[&(x.course.clone(), x.lesson)];
            let ctx: Vec<&String> = ls[x.line.saturating_sub(2)..x.line].iter().collect();
            prompt += &format!("--- id {}\n", x.id);
            for c in ctx {
                prompt += &format!("context: {c}\n");
            }
            prompt += &format!("TARGET: {}\n\n", x.text);
        }
        let (out, call) = opus.ask("prag-bench", bi, chunk.len(), &prompt)?;
        prag::opus::append_call(&d.join("calls.jsonl"), &call)?;
        std::fs::write(d.join("raw").join(format!("batch-{}-{bi}.txt", chunk[0].id)), &out)?;
        let ids: BTreeSet<usize> = chunk.iter().map(|x| x.id).collect();
        let (mut rows, mut notes) = (String::new(), String::new());
        let mut cur: Option<usize> = None;
        let mut got: BTreeSet<usize> = BTreeSet::new();
        for l in out.lines().map(str::trim) {
            if let Some(v) = l.strip_prefix("===") {
                cur = v.trim().trim_start_matches("id").trim().parse().ok().filter(|i| ids.contains(i));
            } else if let (Some(id), Some(n)) = (cur, l.strip_prefix("note:")) {
                notes += &format!("{id}\t{}\n", n.trim().replace('\t', " "));
            } else if let (Some(id), true) = (cur, l.starts_with('#')) {
                let (k, f) = l.split_once(' ').unwrap_or((l, ""));
                if parse_frame(f).is_some() {
                    rows += &format!("{id}\t{k}\t{}\n", f.trim().replace('\t', " "));
                    got.insert(id);
                    frames += 1;
                } else {
                    eprintln!("id {id}: unreadable frame «{l}»");
                }
            }
        }
        let mut f = std::fs::OpenOptions::new().append(true).open(&gp)?;
        std::io::Write::write_all(&mut f, rows.as_bytes())?;
        let mut f = std::fs::OpenOptions::new().append(true).open(&np)?;
        std::io::Write::write_all(&mut f, notes.as_bytes())?;
        done += got.len();
        eprintln!("batch {bi}: {} of {} lines with gold, {:.0} s, out {} tok", got.len(), chunk.len(), call.secs, call.output_tokens);
    }
    Ok(format!("{} lines; {} had gold; {done} written now ({frames} frames)\n", items.len(), have.len()))
}

/// The reader's frames of every item, as notation (each lesson read from its start, one discourse per lesson, as
/// the student reads it).
pub fn system(items: &[&Item]) -> Result<BTreeMap<usize, Vec<String>>> {
    let mut want: BTreeMap<(String, usize), BTreeMap<usize, usize>> = BTreeMap::new();
    for x in items {
        want.entry((x.course.clone(), x.lesson)).or_default().insert(x.line, x.id);
    }
    let mut out = BTreeMap::new();
    let mut by_course: BTreeMap<String, BTreeMap<usize, crate::knowledge::Lesson>> = BTreeMap::new();
    for (c, _) in want.keys() {
        if !by_course.contains_key(c) {
            by_course.insert(c.clone(), crate::knowledge::course(c)?.into_iter().map(|l| (l.n, l)).collect());
        }
    }
    for ((c, n), lines) in &want {
        let l = &by_course[c][n];
        let last = *lines.keys().max().unwrap();
        let mut ctx = Ctx::default();
        for (li, t) in l.explain.iter().chain(&l.facts).enumerate().take(last + 1) {
            let r = crate::frames::read(t, &mut ctx, false)?;
            if let Some(id) = lines.get(&li) {
                out.insert(*id, prag::frames::scored(&r.frames).iter().map(|f| f.show(0)).collect());
            }
        }
    }
    Ok(out)
}

/// The trivial baseline (the negative control): one frame per line — the first noun is the agent, the first verb the
/// verb, the next noun after it the patient.
fn baseline(text: &str) -> Vec<String> {
    let w = crate::frames::parse_words(&crate::frames::plain(text));
    use crate::frames::UPos;
    let noun = |from: usize| (from..w.len()).find(|&i| matches!(w[i].upos, UPos::NOUN | UPos::PROPN));
    let Some(v) = (0..w.len()).find(|&i| w[i].upos == UPos::VERB) else { return Vec::new() };
    let a = noun(0).filter(|&i| i < v).map(|i| w[i].lemma.clone()).unwrap_or_else(|| "_".into());
    let p = noun(v + 1).map(|i| format!(" patient={}", w[i].lemma)).unwrap_or_default();
    vec![format!("Event {a} {}{p}", w[v].lemma)]
}

/// `world frames bench [dev|held|all] [baseline] [worst N]`.
pub fn bench(args: &[String]) -> Result<String> {
    let split = args.iter().find(|a| matches!(a.as_str(), "dev" | "held" | "all")).cloned().unwrap_or_else(|| "dev".into());
    let base = args.iter().any(|a| a == "baseline");
    let worst: usize = args.iter().position(|a| a == "worst").and_then(|i| args.get(i + 1)).and_then(|x| x.parse().ok()).unwrap_or(8);
    let items = load_items()?;
    let gold = load_gold()?;
    let sel: Vec<&Item> = items.iter().filter(|x| (split == "all" || x.split == split) && gold.contains_key(&x.id)).collect();
    if sel.is_empty() {
        bail!("no gold for the split «{split}»");
    }
    let sys: BTreeMap<usize, Vec<String>> = if base { sel.iter().map(|x| (x.id, baseline(&x.text))).collect() } else { system(&sel)? };
    let mut tot = [Prf::default(); 10];
    let mut by_course: BTreeMap<String, [Prf; 10]> = BTreeMap::new();
    let mut lines: Vec<(f64, usize)> = Vec::new();
    for x in &sel {
        let s: Vec<GFrame> = sys[&x.id].iter().filter_map(|f| parse_frame(f)).collect();
        let c = score_line(&gold[&x.id], &s);
        for (k, p) in c.iter().enumerate() {
            tot[k].add(*p);
            by_course.entry(x.course.clone()).or_default()[k].add(*p);
        }
        lines.push((overall(&c).f1(), x.id));
    }
    let o = overall(&tot);
    let mut out = format!("frame bench «{split}»{}: {} lines, {} gold frames\n", if base { " — the trivial baseline" } else { "" }, sel.len(), sel.iter().map(|x| gold[&x.id].len()).sum::<usize>());
    out += &format!("overall F1 {:.3} (P {:.3} R {:.3}; {} right of {} made, {} gold)\n", o.f1(), o.p(), o.r(), o.tp, o.sys, o.gold);
    out += "role        F1     P      R      right/made/gold\n";
    for (k, r) in ROLES.iter().enumerate() {
        let p = tot[k];
        out += &format!("{r:<10} {:.3}  {:.3}  {:.3}  {}/{}/{}\n", p.f1(), p.p(), p.r(), p.tp, p.sys, p.gold);
    }
    out += "by course: ";
    out += &by_course.iter().map(|(c, v)| format!("{c} {:.3}", overall(v).f1())).collect::<Vec<_>>().join(", ");
    out += "\n";
    if let Some(role) = args.iter().position(|a| a == "errors").and_then(|i| args.get(i + 1)) {
        let k = ROLES.iter().position(|r| r == role).context("errors <role>")?;
        out += &format!("\nlines with {role} errors:\n");
        for x in &sel {
            let s: Vec<GFrame> = sys[&x.id].iter().filter_map(|f| parse_frame(f)).collect();
            let c = score_line(&gold[&x.id], &s)[k];
            if c.tp == c.sys && c.tp == c.gold {
                continue;
            }
            out += &format!("[{}] {} {}:{} {role} right {} made {} gold {}: {}\n", x.id, x.course, x.lesson, x.line, c.tp, c.sys, c.gold, x.text);
            for (k, f) in gold[&x.id].iter().enumerate() {
                out += &format!("   gold #{k} {}\n", show_g(f));
            }
            for (k, f) in sys[&x.id].iter().enumerate() {
                out += &format!("   mine #{k} {f}\n");
            }
        }
    }
    if args.iter().any(|a| a == "agents") {
        // aligned frames whose agents differ, and frames left unaligned, counted by kind
        let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
        let mut rows = String::new();
        for x in &sel {
            let s: Vec<GFrame> = sys[&x.id].iter().filter_map(|f| parse_frame(f)).collect();
            let g = &gold[&x.id];
            let m = prag::fbench::align(g, &s);
            for (i, j) in &m {
                let (a, b) = (&g[*i].agent, &s[*j].agent);
                let k = match (a, b) {
                    (Some(a), Some(b)) if a.same(b) => continue,
                    (Some(_), None) => "missing",
                    (None, Some(_)) => "spurious",
                    (Some(_), Some(_)) => "wrong",
                    (None, None) => continue,
                };
                *kinds.entry(k).or_default() += 1;
                let sh = |e: &Option<prag::fbench::GEnt>| e.as_ref().map(|e| e.ids.join("+")).unwrap_or_else(|| "_".into());
                rows += &format!("{k}\t{}\t{}\t{}\t[{}]\n", g[*i].verb, sh(a), sh(b), x.id);
            }
            *kinds.entry("gold unaligned (with agent)").or_default() += (0..g.len()).filter(|i| !m.contains_key(i) && g[*i].agent.is_some()).count();
            let used: BTreeSet<usize> = m.values().copied().collect();
            *kinds.entry("mine unaligned (with agent)").or_default() += (0..s.len()).filter(|j| !used.contains(j) && s[*j].agent.is_some()).count();
        }
        out += &format!("\nagent errors: {}\n{rows}", kinds.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
    }
    if args.iter().any(|a| a == "prons") {
        out += "\npronouns (gold → mine):\n";
        for x in &sel {
            let s: Vec<GFrame> = sys[&x.id].iter().filter_map(|f| parse_frame(f)).collect();
            let (g, m) = (prag::fbench::pronouns(&gold[&x.id]), prag::fbench::pronouns(&s));
            if g.iter().chain(&m).all(|(p, h)| p == h) {
                continue;
            }
            let sh = |v: &[(String, String)]| v.iter().map(|(p, h)| format!("{p}→{h}")).collect::<Vec<_>>().join(" ");
            out += &format!("[{}] gold: {} | mine: {}\n", x.id, sh(&g), sh(&m));
        }
    }
    lines.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap().then(a.1.cmp(&b.1)));
    if worst > 0 {
        out += "\nthe worst lines:\n";
        for (f1, id) in lines.iter().take(worst) {
            let x = sel.iter().find(|x| x.id == *id).unwrap();
            out += &format!("[{id}] {} {}:{} F1 {f1:.2}: {}\n", x.course, x.lesson, x.line, x.text);
            for (k, f) in gold[id].iter().enumerate() {
                out += &format!("   gold #{k} {}\n", show_g(f));
            }
            for (k, f) in sys[id].iter().enumerate() {
                out += &format!("   mine #{k} {f}\n");
            }
        }
    }
    Ok(out)
}

fn show_g(f: &GFrame) -> String {
    let e = |e: &Option<prag::fbench::GEnt>| e.as_ref().map(|e| format!("{}{}", e.ids.join("+"), e.pron.as_ref().map(|p| format!("(«{p}»)")).unwrap_or_default())).unwrap_or_else(|| "_".into());
    let mut s = format!("{} {}{} {}", f.kind, e(&f.agent), if f.neg { " NOT" } else { "" }, f.verb);
    for (n, x) in [("patient", &f.patient), ("to", &f.to), ("place", &f.place)] {
        if x.is_some() {
            s += &format!(" {n}={}", e(x));
        }
    }
    for (p, x) in &f.obls {
        s += &format!(" {p}={}", e(&Some(x.clone())));
    }
    let mut l = Vec::new();
    if !f.says.is_empty() {
        l.push(format!("says→{}", f.says.iter().map(|x| format!("#{x}")).collect::<Vec<_>>().join(",")));
    }
    if let Some(c) = f.cause {
        l.push(format!("cause→#{c}"));
    }
    if let Some(c) = f.next {
        l.push(format!("next→#{c}"));
    }
    if f.quoted {
        l.push("quoted".into());
    }
    if !l.is_empty() {
        s += &format!(" | {}", l.join(" "));
    }
    s
}
