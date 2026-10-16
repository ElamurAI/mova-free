//! prag — Mova pragmatics (closed part).
//!
//!   prag select <db> <tatoeba.tsv> <en model> <run dir> [--tales 2000] [--tatoeba 1000] [--seed 26]
//!       first-wave selection: whole tale paragraphs from the database and Tatoeba CC0 sentences → items.jsonl, batches;
//!       UD bronze for the Tatoeba sentences (tatoeba-ud.conllu) for `db load`, so the sentences are in the database
//!   prag silver <run dir> [--batches 0,38] [--jobs 4]
//!       LLM silver (claude -p, Opus medium) per batch, gates → silver.tsv, rejects.tsv, calls.jsonl
//!   prag prompt <run dir> <batch>
//!       batch prompt without calling the LLM — for review
//!   prag gate <run dir>
//!       only the gates over already received raw outputs
//!   prag eval <run dir> <en model> [--gum en_gum-ud-test.conllu] [--seed 26]
//!       features, training and measurement of SLM models (perceptron, IGTree, k-NN) against held-out silver,
//!       baseline, negative control, model agreement, speed; form vs GUM s_type (measurement only);
//!       SLM predictions (cross-validated by document) → rust-<model>.tsv for `db prag`
//!
//!   prag roles <in.conllu> <out.tsv>
//!       the reader's frames over parsed trees as (sent_id, dependent, predicate, role) — read back by the GPU parser
//!   prag label <silver run dir> <en model> <in.conllu> <out.tsv>
//!       the pragmatics SLM (perceptron on all silver) labels sentences read as documents: form, act, indirect,
//!       hedge, polarity, voice + confidences
//!   prag agree <labels.tsv> <witness.tsv> <trees.conllu> <out.tsv> <report.md>
//!       the pragmatics agreer: our labels stand unless agreeing open witnesses and written rules say otherwise
//!   prag world <in.conllu> <out.tsv>
//!       the temporary world before each sentence (whole documents in order): salient entities, last speaker,
//!       recent events — a page summary for the teacher's prompt
//!   prag lens <en model> <text>
//!   prag infer <en model> <text>
//!       per sentence: scalar implicatures and presuppositions, Quality and Manner implicatures (questions,
//!       exclamations, ordered "and"), and a sarcasm / irony candidate by incongruity — each with its reason
//!       expressions with their real meanings (idioms, proverbs, phrasal verbs, slang …) and the domains to switch on
//!   prag rsa scalar|reference             Rational Speech Acts (exact L0/S1/L1) for each utterance of a built-in game
//!   prag lexicon convert <verbnet3.4 dir> <propbank frames dir> <nombank.1.0 dir> <out dir> [<test-only dir>]
//!       VerbNet → <out dir>/verbnet.tsv (prag/data, built in); PropBank (CC BY-SA) and NomBank (research-only) →
//!       <test-only dir>/propbank.tsv, nombank.tsv (default $MOVA_DATA/raw-derived/prag-lexicon: tests and
//!       measurement only, never read by the reader or the rule learner) (`prag::vnconv`)
//!   prag lexicon nominals <wordnet 3.1 dict dir> <out dir>
//!       Princeton WordNet's derivational links → <out dir>/nominals.tsv (nouns naming a verb's event, built in)
//!
//! Variables: PRAG_MODEL (claude-haiku-5-5), PRAG_EFFORT (medium), DUCKDB (~/bin/duckdb).

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result, bail};

use prag::data::{self, Src};
use prag::silver;

fn usage() -> Result<()> {
    bail!(
        "usage: prag select <db> <tatoeba.tsv> <en model> <dir> [--tales N] [--tatoeba N] [--seed S] | \
         prag silver <dir> [--batches a,b,…] [--jobs N] | prag prompt <dir> <batch> | prag gate <dir> | prag eval <dir> <en model> [--gum test.conllu] [--seed S] | \
         prag lens <en model> <text> | prag infer <en model> <text> | prag rsa scalar|reference"
    )
}

/// `--key value` options after the positional arguments.
fn opts(a: &[String]) -> Result<(Vec<String>, std::collections::BTreeMap<String, String>)> {
    let mut pos = Vec::new();
    let mut o = std::collections::BTreeMap::new();
    let mut i = 0;
    while i < a.len() {
        if let Some(k) = a[i].strip_prefix("--") {
            let v = a.get(i + 1).with_context(|| format!("--{k}: missing value"))?;
            o.insert(k.to_string(), v.clone());
            i += 2;
        } else {
            pos.push(a[i].clone());
            i += 1;
        }
    }
    Ok((pos, o))
}

fn num(o: &std::collections::BTreeMap<String, String>, k: &str, d: usize) -> Result<usize> {
    o.get(k).map(|v| v.parse::<usize>().with_context(|| format!("--{k} {v}"))).transpose().map(|x| x.unwrap_or(d))
}

/// `prag roles <in.conllu> <out.tsv>`: the reader's frames over every tree, as (sent_id, dependent, predicate, role)
/// rows — the pragmatics the GPU parser reads back on its next pass (`prag::roles`).
fn roles_file(inp: &Path, out: &Path) -> Result<()> {
    let mut t = String::from("sent_id\tdep\tpred\trole\n");
    let (mut n, mut r) = (0usize, 0usize);
    // documents read whole, in order, with one discourse state each (PRAG_ROLES_ALONE=1: every sentence alone)
    let alone = std::env::var("PRAG_ROLES_ALONE").as_deref() == Ok("1");
    let sents: Vec<(String, Vec<en::annotate::Word>)> = en::conllu::read(inp)?
        .into_iter()
        .map(|s| {
            let words = s.tokens.iter().map(|k| en::annotate::Word { form: k.form.clone(), lemma: k.lemma.to_lowercase(), upos: k.upos.unwrap_or(en::gram::UPos::X), tag: k.tag.unwrap_or(en::gram::Tag::X), feats: k.feats, head: k.head, rel: k.rel }).collect();
            (s.id, words)
        })
        .collect();
    let rows = if alone { sents.into_iter().map(|(id, w)| (id, prag::roles::roles(w))).collect() } else { prag::roles::roles_docs(sents) };
    for (id, rs) in rows {
        for (d, p, role) in rs {
            let _ = writeln!(t, "{id}\t{d}\t{p}\t{role}");
            r += 1;
        }
        n += 1;
    }
    std::fs::write(out, t)?;
    eprintln!("roles: {n} sentences, {r} roles → {}", out.display());
    Ok(())
}

/// `prag world <in.conllu> <out.tsv>`: for each sentence, the temporary world before it (documents read whole, in
/// order): salient entities, last speaker, recent events — the page summary the teacher gets instead of the pages.
fn world_file(inp: &Path, out: &Path) -> Result<()> {
    let sents: Vec<(String, Vec<en::annotate::Word>)> = en::conllu::read(inp)?
        .into_iter()
        .map(|s| {
            let words = s.tokens.iter().map(|k| en::annotate::Word { form: k.form.clone(), lemma: k.lemma.to_lowercase(), upos: k.upos.unwrap_or(en::gram::UPos::X), tag: k.tag.unwrap_or(en::gram::Tag::X), feats: k.feats, head: k.head, rel: k.rel }).collect();
            (s.id, words)
        })
        .collect();
    let mut t = String::from("sent_id\tworld\n");
    let n = sents.len();
    for (id, w) in prag::roles::world_docs(sents, 6) {
        let _ = writeln!(t, "{id}\t{}", w.replace('\t', " "));
    }
    std::fs::write(out, t)?;
    eprintln!("world: {n} sentences → {}", out.display());
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first().map(String::as_str) else { return usage() };
    let (pos, o) = opts(&args[1..])?;
    match (cmd, pos.len()) {
        ("roles", 2) => roles_file(Path::new(&pos[0]), Path::new(&pos[1])),
        ("world", 2) => world_file(Path::new(&pos[0]), Path::new(&pos[1])),
        ("agree", 5) => {
            let (out, rep) = prag::agree::run(&std::fs::read_to_string(&pos[0])?, &std::fs::read_to_string(&pos[1])?, &std::fs::read_to_string(&pos[2])?);
            std::fs::write(&pos[3], out)?;
            std::fs::write(&pos[4], &rep)?;
            print!("{rep}");
            Ok(())
        }
        ("label", 4) => prag::eval::label(Path::new(&pos[0]), Path::new(&pos[1]), Path::new(&pos[2]), Path::new(&pos[3])),
        ("select", 4) => select(Path::new(&pos[0]), Path::new(&pos[1]), Path::new(&pos[2]), Path::new(&pos[3]), num(&o, "tales", 2000)?, num(&o, "tatoeba", 1000)?, num(&o, "seed", 26)? as u64),
        ("silver", 1) => {
            let dir = Path::new(&pos[0]);
            let items = data::read_items(&dir.join("items.jsonl"))?;
            let which: Vec<usize> = match o.get("batches") {
                Some(v) => v.split(',').map(|x| x.trim().parse::<usize>().with_context(|| format!("--batches {v}"))).collect::<Result<_>>()?,
                None => Vec::new(),
            };
            let r = silver::run(dir, &items, &which, num(&o, "jobs", 4)?);
            gate_report(dir, &items)?;
            r
        }
        ("prompt", 2) => {
            // batch prompt without a call — for review
            let items = data::read_items(&Path::new(&pos[0]).join("items.jsonl"))?;
            let b = silver::batches(&items);
            let its = b.get(&pos[1].parse::<usize>()?).context("no such batch")?;
            print!("{}", silver::prompt(its));
            Ok(())
        }
        ("gate", 1) => {
            let dir = Path::new(&pos[0]);
            let items = data::read_items(&dir.join("items.jsonl"))?;
            gate_report(dir, &items)
        }
        ("lens", 2) => {
            // lens <en model> <text>: expressions with their real meanings and the domains to switch on
            let a = en::annotate::Annotator::load(Path::new(&pos[0]))?;
            let forms: Vec<String> = a.tokenize(&pos[1]).into_iter().map(|t| t.form).collect();
            let words = a.annotate(&forms);
            let l = prag::lens::lens(&words);
            for r in &l.readings {
                println!("{:24} {:12} {}{}", r.text, r.kind, r.meaning, if r.register.is_empty() { String::new() } else { format!("  [{}]", r.register) });
            }
            println!("domains: {}", if l.domains.is_empty() { "-".to_string() } else { l.domains.join(", ") });
            println!("means: {}", prag::lens::means(&l));
            Ok(())
        }
        ("infer", 2) => {
            // infer <en model> <text>: scalar implicatures and presuppositions, sentence by sentence, with reasons
            let a = en::annotate::Annotator::load(Path::new(&pos[0]))?;
            let mut earlier: Vec<Vec<en::annotate::Word>> = Vec::new();
            for s in sentences(&pos[1]) {
                let s = s.as_str();
                let forms: Vec<String> = a.tokenize(s).into_iter().map(|t| t.form).collect();
                let words = a.annotate(&forms);
                println!("{s}");
                for i in prag::infer::implicatures(&words).into_iter().chain(prag::infer::presuppositions(&words, &earlier)).chain(prag::infer::maxims(&words, s)) {
                    println!("  {} [{}]: {}{} — {}", i.kind, i.trigger, i.content, if i.licensed { "" } else { " (new)" }, i.why);
                }
                if let Some(r) = prag::irony::irony(&words, s) {
                    println!("  irony candidate (score {}) [{}]: «{}» about «{}» — {}", r.score, r.features.join(", "), r.pair.0, r.pair.1, r.why);
                }
                earlier.push(words);
            }
            Ok(())
        }
        ("rsa", 1) => {
            // rsa scalar|reference: the L0/S1/L1 rows and the preferred reading of every utterance
            let g = match pos[0].as_str() {
                "scalar" => prag::rsa::scalar(),
                "reference" => prag::rsa::reference(),
                other => bail!("rsa: unknown game {other} (scalar, reference)"),
            };
            for u in 0..g.utterances.len() {
                println!("{}", g.explain(u));
            }
            Ok(())
        }
        ("lexicon", 5 | 6) if pos[0] == "convert" => {
            let test = pos.get(5).map(std::path::PathBuf::from).unwrap_or_else(|| std::path::PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()))).join("raw-derived/prag-lexicon"));
            print!("{}", prag::vnconv::convert(Path::new(&pos[1]), Path::new(&pos[2]), Path::new(&pos[3]), Path::new(&pos[4]), &test)?);
            Ok(())
        }
        ("lexicon", 3) if pos[0] == "nominals" => {
            print!("{}", prag::vnconv::nominals(Path::new(&pos[1]), Path::new(&pos[2]))?);
            Ok(())
        }
        ("eval", 2) => prag::eval::run(Path::new(&pos[0]), Path::new(&pos[1]), o.get("gum").map(std::path::PathBuf::from).as_deref(), num(&o, "seed", 26)? as u64),
        _ => usage(),
    }
}

/// Sentences: a run of . ! ? ends one (so "!!" and "?!" stay with their sentence); a piece without letters
/// (a trailing emoticon) joins the previous sentence.
fn sentences(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        cur.push(c);
        if matches!(c, '.' | '!' | '?') && !chars.peek().is_some_and(|n| matches!(n, '.' | '!' | '?')) {
            out.push(std::mem::take(&mut cur));
        }
    }
    out.push(cur);
    let mut merged: Vec<String> = Vec::new();
    for p in out.into_iter().map(|p| p.trim().to_string()).filter(|p| !p.is_empty()) {
        match merged.last_mut() {
            Some(last) if !p.chars().any(char::is_alphabetic) => {
                last.push(' ');
                last.push_str(&p);
            }
            _ => merged.push(p),
        }
    }
    merged
}

fn select(db: &Path, tatoeba: &Path, model: &Path, dir: &Path, n_tales: usize, n_tat: usize, seed: u64) -> Result<()> {
    if n_tales + n_tat > silver::WAVE_MAX {
        bail!("{} sentences > {} — wave ceiling", n_tales + n_tat, silver::WAVE_MAX);
    }
    data::license_gate(&[Src::Tale, Src::Tatoeba])?;
    std::fs::create_dir_all(dir)?;
    let tales = data::read_tales(db, &std::env::temp_dir().join("prag"))?;
    let wins = data::select_tales(&tales, n_tales, seed, (10, 18), 14);
    let tat = data::select_tatoeba(tatoeba, n_tat, seed)?;
    let items = data::make_items(&tales, &wins, &tat, 60, 50);
    data::write_items(&dir.join("items.jsonl"), &items)?;
    // UD bronze for the Tatoeba sentences, so they go into the database (`db load --docs idprefix --match auto`)
    let a = en::annotate::Annotator::load(model)?;
    let mut conllu = String::new();
    for (id, t) in &tat {
        let toks = a.tokenize(t);
        let forms: Vec<&str> = toks.iter().map(|x| x.form.as_str()).collect();
        let space: Vec<bool> = toks.iter().map(|x| x.space_after).collect();
        let _ = writeln!(conllu, "# source = Tatoeba #{id} (CC0 1.0)");
        let _ = writeln!(conllu, "# annotation = Mova prag: Rust (en, {}) — UD bronze for pragmatics features; {}", model.file_name().unwrap_or_default().to_string_lossy(), dir.display());
        let _ = writeln!(conllu, "# text = {t}\n# sent_id = tatoeba:{id}");
        for r in en::annotate::rows(&a.annotate(&forms), &space) {
            let _ = writeln!(conllu, "{}", r.join("\t"));
        }
        conllu.push('\n');
    }
    std::fs::write(dir.join("tatoeba-ud.conllu"), conllu)?;
    // selection overview
    let b = silver::batches(&items);
    let docs: std::collections::BTreeSet<&str> = items.iter().filter(|x| x.src == Src::Tale).map(|x| x.doc.as_str()).collect();
    let n_t = items.iter().filter(|x| x.src == Src::Tale).count();
    let quoted = wins.iter().flatten().flatten().filter(|&&i| data::has_quote(&tales[i].text)).count();
    let n_paras: usize = wins.iter().map(Vec::len).sum();
    let mut by_src: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for it in items.iter().filter(|x| x.src == Src::Tale) {
        let book = it.doc.split(':').take(2).collect::<Vec<_>>().join(":");
        *by_src.entry(book).or_default() += 1;
    }
    let sizes: Vec<usize> = b.values().map(Vec::len).collect();
    let mut s = String::new();
    let _ = writeln!(s, "# prag selection (seed {seed})\n");
    let _ = writeln!(s, "- tales: {n_t} sentences in {} windows ({n_paras} paragraphs) from {} tales; sentences with quotes {quoted}", wins.len(), docs.len());
    let _ = writeln!(s, "- by book: {}", by_src.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
    let _ = writeln!(s, "- Tatoeba: {} sentences", tat.len());
    let _ = writeln!(s, "- {} batches: size from {} to {}", b.len(), sizes.iter().min().unwrap_or(&0), sizes.iter().max().unwrap_or(&0));
    std::fs::write(dir.join("select.md"), &s)?;
    print!("{s}");
    Ok(())
}

fn gate_report(dir: &Path, items: &[data::Item]) -> Result<()> {
    let (ok, bad, missing) = silver::collect(dir, items)?;
    let calls = prag::opus::read_calls(&dir.join("calls.jsonl"))?;
    let secs: f64 = calls.iter().map(|c| c.secs).sum();
    let (i, out, cr, cw): (u64, u64, u64, u64) = calls.iter().fold((0, 0, 0, 0), |a, c| (a.0 + c.input_tokens, a.1 + c.output_tokens, a.2 + c.cache_read_tokens, a.3 + c.cache_write_tokens));
    println!(
        "silver: accepted {ok}, rejected {bad}, batches without output {missing}; {} calls, {:.0} s total; tokens: input {i}, output {out}, cache read {cr}, cache write {cw}",
        calls.len(),
        secs
    );
    // run provenance for the database (`db prag --params`): model, effort, call accounting
    let opus = prag::opus::Opus::from_env(dir.join("cwd"));
    let params = serde_json::json!({
        "model": opus.model, "effort": opus.effort, "via": "claude -p (--settings env CLAUDE_CODE_EFFORT_LEVEL)",
        "scheme": "docs/context.md — Pragmatics, first cut (26.09.2026)", "prompt": "prag/src/silver.rs RULES",
        "sentences": items.len(), "accepted": ok, "rejected": bad, "batches": silver::batches(items).len(),
        "calls": calls.len(), "call_secs_sum": secs, "input_tokens": i, "output_tokens": out, "cache_read_tokens": cr, "cache_write_tokens": cw,
        "cost_usd_estimate": calls.iter().map(|c| c.cost_usd).sum::<f64>(),
        "texts": {"tales": "gutenberg-tales (public domain)", "tatoeba": "tatoeba-eng-cc0 (CC0 1.0)"},
    });
    std::fs::write(dir.join("silver-params.json"), serde_json::to_string_pretty(&params)?)?;
    Ok(())
}
