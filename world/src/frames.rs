//! World's door to the pragmatic structure (`prag::frames`, docs/pragmatics.md): the reader with world's parser —
//! the `en` annotator on cased text with the absurdity-matrix repairs (`tree::Tree::parse_cased`).
//!
//!   world frames "<text>"        — the frames of a text (FRAMES_TREE=1: the trees too)
//!   world frames bench-make / bench [dev|held] — the frame benchmark (`crate::fbench`)
//!   world frames lexicon / vnfreq [course …] — VerbNet and WordNet coverage; the lessons' class preference
//!   world frames nominal-check [course …] — TESTS ONLY: the WordNet nouns against NOMLEX (outside the repo)

pub use prag::frames::*;



/// A sentence → words, by world's parser.
pub fn parse_words(s: &str) -> Vec<en::annotate::Word> {
    match crate::tree::annotator() {
        Ok(a) => crate::tree::Tree::parse_cased(a, s).w,
        Err(_) => Vec::new(),
    }
}

/// Any text → frames, with the discourse carried in `ctx`.
pub fn read(text: &str, ctx: &mut Ctx, question: bool) -> anyhow::Result<Read> {
    crate::tree::annotator()?;
    Ok(read_with(text, ctx, question, &parse_words))
}

/// `world frames lexicon [course …]`: how much of the lessons' verbs VerbNet knows (tokens and distinct lemmas of the
/// frames' verbs, the copula aside), what assigned the frames' roles, the nouns WordNet names an event of and how
/// many noun tokens get an event in the reader, and the commonest verbs VerbNet does not know.
pub fn lexicon_coverage(courses: &[String]) -> anyhow::Result<String> {
    use std::collections::{BTreeMap, BTreeSet};
    use std::fmt::Write as _;
    let lx = prag::vn::lex();
    let mut out = String::new();
    let mut all: BTreeMap<String, usize> = BTreeMap::new();
    let mut by: BTreeMap<&'static str, usize> = BTreeMap::new();
    let (mut nouns, mut noms) = (0usize, 0usize);
    let mut nom_types: BTreeSet<String> = BTreeSet::new();
    let mut events: BTreeMap<String, usize> = BTreeMap::new();
    for c in courses {
        let mut n_c: BTreeMap<String, usize> = BTreeMap::new();
        for l in crate::knowledge::course(c)? {
            let mut ctx = Ctx::default();
            for fact in &l.facts {
                let r = read(fact, &mut ctx, false)?;
                for f in &r.frames {
                    if f.how.by == By::Nominal {
                        *events.entry(f.how.class.clone()).or_default() += 1;
                    }
                    if f.verb.is_empty() || f.verb == "be" || f.kind == Kind::Bare || f.how.by == By::Nominal {
                        continue;
                    }
                    *n_c.entry(f.verb.clone()).or_default() += 1;
                    *by.entry(f.how.by.name()).or_default() += 1;
                }
                for t in &r.trees {
                    for w in &t.w {
                        if w.upos == UPos::NOUN {
                            nouns += 1;
                            if prag::vn::nom(&w.lemma).is_some() {
                                noms += 1;
                                nom_types.insert(w.lemma.clone());
                            }
                        }
                    }
                }
            }
        }
        writeln!(out, "{c}: {}", cover_line(lx, &n_c))?;
        for (k, v) in n_c {
            *all.entry(k).or_default() += v;
        }
    }
    writeln!(out, "all: {}", cover_line(lx, &all))?;
    writeln!(out, "roles of the frames (verbs other than «be») assigned by: {}", by.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "))?;
    let sv = prag::vn::speech_verbs();
    writeln!(out, "verbs of communication by VerbNet (every class 37.x): {}: {}", sv.len(), sv.join(", "))?;
    writeln!(out, "nouns that WordNet names the event of a verb: {noms} of {nouns} noun tokens, {} lemmas", nom_types.len())?;
    let ev: usize = events.values().sum();
    let mut top: Vec<(&String, &usize)> = events.iter().collect();
    top.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    writeln!(out, "noun tokens that got an event in the reader (with an argument): {ev}, {} lemmas; the commonest: {}", events.len(), top.iter().take(40).map(|(v, n)| format!("{v} {n}")).collect::<Vec<_>>().join(", "))?;
    let mut miss: Vec<(&String, &usize)> = all.iter().filter(|(v, _)| !known(lx, v)).collect();
    miss.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    writeln!(out, "the commonest verbs VerbNet does not know: {}", miss.iter().take(30).map(|(v, n)| format!("{v} {n}")).collect::<Vec<_>>().join(", "))?;
    Ok(out)
}

/// `world frames vnfreq [course …]`: the lessons' preference among a verb's VerbNet classes — every lesson line read,
/// and for each clause the class that alone matches it best (without the preference) counted — written to
/// prag/data/vnfreq.tsv (rebuild the reader after). Our own texts only.
pub fn vnfreq(courses: &[String], out: &std::path::Path) -> anyhow::Result<String> {
    use std::fmt::Write as _;
    prag::vn::record_start();
    for c in courses {
        for l in crate::knowledge::course(c)? {
            let mut ctx = Ctx::default();
            for fact in &l.facts {
                read(fact, &mut ctx, false)?;
            }
        }
    }
    let m = prag::vn::record_take();
    let mut t = format!(
        "# The lessons' preference among a verb's VerbNet classes: for each verb, how many of its clauses in our lessons ({}) one class alone matched best (frame score without this table). Made by `world frames vnfreq`.\n# rows: verb<TAB>class<TAB>clauses\n",
        courses.join(", ")
    );
    let verbs: std::collections::BTreeSet<&String> = m.keys().map(|(v, _)| v).collect();
    for ((v, c), n) in &m {
        writeln!(t, "{v}\t{c}\t{n}")?;
    }
    std::fs::write(out, &t)?;
    Ok(format!("{}: {} verbs, {} (verb, class) rows, {} clauses\n", out.display(), verbs.len(), m.len(), m.values().sum::<usize>()))
}

/// TESTS ONLY: the WordNet nouns against NOMLEX (NomBank, research-only), read at run time from outside the repo
/// ($MOVA_DATA/raw-derived/prag-lexicon/nombank.tsv, `prag lexicon convert`) — never built in, never fed to the reader
/// or the rule learner. Over the lessons' noun tokens: known to either, the same verb, the same transitivity.
pub fn nominal_check(courses: &[String]) -> anyhow::Result<String> {
    use std::collections::BTreeMap;
    use std::fmt::Write as _;
    let p = std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("raw-derived/prag-lexicon/nombank.tsv");
    let s = std::fs::read_to_string(&p).map_err(|e| anyhow::anyhow!("{}: {e} (make it with `prag lexicon convert`)", p.display()))?;
    // NOMLEX: noun → (verb, takes an object), the event nominalisations only
    let mut nl: BTreeMap<String, (String, bool)> = BTreeMap::new();
    for l in s.lines().filter(|l| l.starts_with("nomlex\t")) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() >= 7 && c[4].split(',').any(|x| x == "verb-nom") {
            nl.entry(c[1].to_string()).or_insert((c[2].to_string(), !c[6].is_empty()));
        }
    }
    let (mut nouns, mut wn, mut nb, mut both, mut same_v, mut same_t) = (0, 0, 0, 0, 0, 0);
    let mut only_nb: BTreeMap<String, usize> = BTreeMap::new();
    let mut only_wn: BTreeMap<String, usize> = BTreeMap::new();
    let mut diff_v: BTreeMap<String, usize> = BTreeMap::new();
    for c in courses {
        for l in crate::knowledge::course(c)? {
            let mut ctx = Ctx::default();
            for fact in &l.facts {
                let r = read(fact, &mut ctx, false)?;
                for t in &r.trees {
                    for w in t.w.iter().filter(|w| w.upos == UPos::NOUN) {
                        nouns += 1;
                        let a = prag::vn::nom(&w.lemma);
                        let b = nl.get(&w.lemma);
                        wn += a.is_some() as usize;
                        nb += b.is_some() as usize;
                        match (a, b) {
                            (Some(a), Some(b)) => {
                                both += 1;
                                if a.verb == b.0 {
                                    same_v += 1;
                                    same_t += (a.transitive == b.1) as usize;
                                } else {
                                    *diff_v.entry(format!("{}: {}/{}", w.lemma, a.verb, b.0)).or_default() += 1;
                                }
                            }
                            (None, Some(_)) => *only_nb.entry(w.lemma.clone()).or_default() += 1,
                            (Some(_), None) => *only_wn.entry(w.lemma.clone()).or_default() += 1,
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    let top = |m: &BTreeMap<String, usize>| {
        let mut v: Vec<(&String, &usize)> = m.iter().collect();
        v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        v.iter().take(40).map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", ")
    };
    let mut out = String::new();
    writeln!(out, "TEST ONLY (NOMLEX read from {}): {nouns} noun tokens; WordNet names an event for {wn}, NOMLEX for {nb}, both {both}: the same verb {same_v}, of them the same transitivity {same_t}", p.display())?;
    writeln!(out, "NOMLEX only: {}", top(&only_nb))?;
    writeln!(out, "WordNet only: {}", top(&only_wn))?;
    writeln!(out, "different verbs (WordNet/NOMLEX): {}", top(&diff_v))?;
    Ok(out)
}

/// VerbNet knows the verb: the phrasal verb, else its head verb.
fn known(lx: &prag::vn::Lex, v: &str) -> bool {
    let head = v.split(' ').next().unwrap_or(v);
    !lx.classes_of(v).is_empty() || !lx.classes_of(head).is_empty()
}

fn cover_line(lx: &prag::vn::Lex, n: &std::collections::BTreeMap<String, usize>) -> String {
    let (mut t, mut tv, mut y, mut yv) = (0, 0, 0, 0);
    for (v, &k) in n {
        t += k;
        y += 1;
        if known(lx, v) {
            tv += k;
            yv += 1;
        }
    }
    let pc = |a: usize, b: usize| if b == 0 { 0.0 } else { 100.0 * a as f64 / b as f64 };
    format!("{t} verb tokens ({y} lemmas): VerbNet {:.1}% ({:.1}% of lemmas)", pc(tv, t), pc(yv, y))
}

/// `world frames "<text>"`.
pub fn main_args(args: &[String]) -> anyhow::Result<()> {
    match args.get(1).map(String::as_str) {
        Some("bench-make") => {
            print!("{}", crate::fbench::make()?);
            return Ok(());
        }
        Some("rules") => {
            print!("{}", crate::prules::main(&args[2..])?);
            return Ok(());
        }
        Some("lexicon") => {
            let cs: Vec<String> = if args.len() > 2 { args[2..].to_vec() } else { ["books", "world", "biology", "everyday"].iter().map(|s| s.to_string()).collect() };
            print!("{}", lexicon_coverage(&cs)?);
            return Ok(());
        }
        Some("vnfreq") => {
            let cs: Vec<String> = if args.len() > 2 { args[2..].to_vec() } else { ["books", "world", "biology", "everyday"].iter().map(|s| s.to_string()).collect() };
            let out = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../prag/data/vnfreq.tsv");
            print!("{}", vnfreq(&cs, &out)?);
            return Ok(());
        }
        Some("nominal-check") => {
            let cs: Vec<String> = if args.len() > 2 { args[2..].to_vec() } else { ["books", "world", "biology", "everyday"].iter().map(|s| s.to_string()).collect() };
            print!("{}", nominal_check(&cs)?);
            return Ok(());
        }
        Some("bench") => {
            print!("{}", crate::fbench::bench(&args[2..])?);
            return Ok(());
        }
        _ => {}
    }
    let text = args[1..].join(" ");
    if std::env::var("FRAMES_TREE").is_ok() {
        for (s, _) in units(&text, text.contains('?')) {
            println!("{s}\n{}", prag::frames::Tree::of_words(parse_words(&s)).show());
        }
    }
    let mut ctx = Ctx::default();
    let r = read(&text, &mut ctx, text.contains('?'))?;
    for (i, f) in r.frames.iter().enumerate() {
        println!("#{i} {}", f.show(0));
        if std::env::var("FRAMES_HOW").is_ok() || f.how.by != By::Heuristic {
            println!("     roles by {}", f.how.show());
        }
    }
    for a in &r.asked {
        println!("asked: «{}» in #{}: {}", a.word, a.frame, a.what.name());
    }
    for (a, b) in &r.aliases {
        println!("same: {a} = {b}");
    }
    Ok(())
}
