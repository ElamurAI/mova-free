//! en — Mova's English graph compressor: text → graph (deterministic parser) → text
//! (deterministic generator). Training is on UD trees, as with neural nets, but with tables and
//! Markov models. Layers: automaton tokenizer, tagger, automaton parser, generator.
//!
//!   en train <train.conllu>... -- <model> [<check.conllu>...]
//!                                                        full UD annotator → model file (data/en/models/);
//!                                                        with check files: load it back and get identical annotation down to the token
//!   en annotate <model> <in.conllu|in.txt>...            CoNLL-U from a model file: .conllu on gold tokens, anything else as text
//!   en tok-eval <train.conllu>... -- <test.conllu>...   tokenizer vs UD tokens
//!   en tag-eval <train.conllu>... -- <test.conllu>...   PTB tagger vs UD tags (on gold tokens)
//!   en morph-eval <train.conllu>... -- <test.conllu>... lemmas and lossless token encoding (lemma + case / escape)
//!   en parse-eval <train.conllu>... -- <test.conllu>... tree parser: UAS/LAS on gold and our tags
//!   en lin-eval <train.conllu>... -- <test.conllu>...   word-order generator from gold trees vs the original
//!   en rt-eval <train.conllu>... -- <test.conllu>...    full compressor round trip: text → graph → text
//!   en rt-text <train.conllu>... -- <dom-train.txt> <dom-test.txt> [weight]
//!                                                        domain texts: general model vs fine-tuned
//!   en explain <train.conllu>... -- "sentence"...       every compressor step for a sentence, human-readable
//!   en explain-json <train.conllu>... -- "sentence"...  the same as JSON: data for the the project site page
//!   en graph <train.conllu>... -- <file.txt>...         line graphs: lemma/tag/head/relation (version comparison)
//!   en lexicon <infl.txt> <train>... [-- <data-dir>]      dictionary into code: data/lemmas.tsv (frozen ids) + data/forms.tsv
//!   en convert <rules dir> <from> <to> <in.conllu>...    UD dialect converter (convert.md rules) → CoNLL-U to stdout
//!   en convert-check <rules dir> <dialect> <in.conllu>... run X → mova → X: what forward changed, what did not come back
//!   en ctx-report <gold.conllu> <base>=<f1>,<f2>… <system>=<…>… [--focus <name>] [--boot N]
//!                                                        context experiment: metrics per seed, bootstrap, breakdown, examples

use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};

use en::conllu::{self, Sentence};
use en::dict;
use en::gram::{Case, Rel, Tag, UPos};
use en::hash::{FastMap, FastSet};
use en::lin::{self, Linearizer};
use en::model::Model;
use en::morph::Morph;
use en::parse::Parser;
use en::tag::{DictHint, Tagger};
use en::text::{bigram_precision, chrf, lcs};
use en::tok;

/// Arguments of the form `<before>... -- <after>...`.
fn split(args: &[String], from: usize) -> (Vec<PathBuf>, Vec<String>) {
    let sep = args.iter().position(|a| a == "--").unwrap_or(args.len());
    (args[from..sep].iter().map(PathBuf::from).collect(), args.get(sep + 1..).unwrap_or(&[]).to_vec())
}

fn paths(v: &[String]) -> Vec<PathBuf> {
    v.iter().map(PathBuf::from).collect()
}

fn read_all(ps: &[PathBuf], tagged_only: bool) -> Result<Vec<Sentence>> {
    let mut out = Vec::new();
    for p in ps {
        out.extend(conllu::read(p)?.into_iter().filter(|s| !tagged_only || s.tagged()));
    }
    Ok(out)
}

/// Fraction of training sentences for the learning curve: `EN_TRAIN_FRAC` (0..1), evenly across the corpus.
fn frac(v: Vec<Sentence>) -> Vec<Sentence> {
    let f: f64 = std::env::var("EN_TRAIN_FRAC").ok().and_then(|x| x.parse().ok()).unwrap_or(1.0);
    if f >= 1.0 {
        return v;
    }
    v.into_iter().enumerate().filter(|(i, _)| (*i as f64 * f).floor() != ((*i + 1) as f64 * f).floor()).map(|(_, s)| s).collect()
}

fn name(p: &Path) -> String {
    p.file_name().unwrap_or_default().to_string_lossy().into_owned()
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("tok-eval") => {
            let (train, test) = split(&args, 1);
            tok_eval(&train, &paths(&test))
        }
        Some("tag-eval") => {
            let (train, test) = split(&args, 1);
            tag_eval(&train, &paths(&test))
        }
        Some("morph-eval") => {
            let (train, test) = split(&args, 1);
            morph_eval(&train, &paths(&test))
        }
        Some("tag-cmp") => {
            let (train, rest) = split(&args, 1);
            let sents = read_all(&train, true)?;
            let (tnt, perc) = (Tagger::train(&sents), en::ptag::PTagger::train(&sents, 6));
            let lx = en::tok::Lexicon::learn(&sents);
            for text in &rest {
                let toks = tok::tokenize(text, &lx);
                let words: Vec<&str> = toks.iter().map(|t| t.form.as_str()).collect();
                let (a, b) = (tnt.tag(&words), perc.tag(&words));
                let show = |v: &[Tag]| words.iter().zip(v).map(|(w, t)| format!("{w}/{t}")).collect::<Vec<_>>().join(" ");
                println!("TnT : {}
perc: {}
", show(&a), show(&b));
            }
            Ok(())
        }
        Some("expert-check") => {
            // seed rules on CoNLL-U files: how many times each rule fired and how many violations.
            // With `--gold <gold>`: whether a violation is a real error (at least one bound node differs from
            // the gold in UPOS, XPOS, FEATS, lemma, head or relation) and what share of erroneous tokens
            // the rules cover. The table shows up to three examples per rule; `EN_EXPERT_LIST=1` also lists all
            // violations at the end.
            let (rules, errs) = en::expert::load(Path::new(&args[1]));
            for e in &errs {
                println!("NOT PARSED {e}");
            }
            let (gold_path, files) = if args.get(2).map(String::as_str) == Some("--gold") && args.len() > 4 { (Some(&args[3]), &args[4..]) } else { (None, &args[2..]) };
            let mut sents = Vec::new();
            for p in files {
                sents.extend(conllu::read(Path::new(p))?);
            }
            let gold: FastMap<String, conllu::Sentence> = match gold_path {
                Some(g) => conllu::read(Path::new(g))?.into_iter().map(|s| (s.id.clone(), s)).collect(),
                None => FastMap::default(),
            };
            // erroneous tokens of each sentence (1-based indices)
            let wrong: Vec<Vec<bool>> = sents
                .iter()
                .map(|s| match gold.get(&s.id) {
                    Some(g) if g.tokens.len() == s.tokens.len() => s
                        .tokens
                        .iter()
                        .zip(&g.tokens)
                        .map(|(x, y)| x.upos != y.upos || x.tag != y.tag || x.feats != y.feats || x.lemma.to_lowercase() != y.lemma.to_lowercase() || x.head != y.head || x.rel != y.rel)
                        .collect(),
                    _ => vec![false; s.tokens.len()],
                })
                .collect();
            let with_gold = !gold.is_empty();
            println!("rules {}, not parsed {}; sentences {}{}", rules.len(), errs.len(), sents.len(), if with_gold { " (checked against gold)" } else { "" });
            if with_gold {
                println!("| rule | severity | fired | violations | real | precision | example |");
                println!("|---|---|---:|---:|---:|---:|---|");
            } else {
                println!("| rule | severity | fired | violations | share | example |");
                println!("|---|---|---:|---:|---:|---|");
            }
            let mut covered: Vec<Vec<[bool; 2]>> = sents.iter().map(|s| vec![[false; 2]; s.tokens.len()]).collect();
            // EN_EXPERT_LIST=1: all violations as a list for review (sent_id, rule, nodes 'name=index:form', text)
            let listing = std::env::var("EN_EXPERT_LIST").is_ok_and(|v| v == "1");
            let mut list: Vec<String> = Vec::new();
            for r in &rules {
                let (mut fired, mut bad, mut real, mut examples) = (0usize, 0usize, 0usize, Vec::new());
                let sev = (r.severity != "error") as usize;
                let names = r.names();
                for (k, s) in sents.iter().enumerate() {
                    let (f, v) = en::expert::check(r, s);
                    fired += f;
                    bad += v.len();
                    for b in &v {
                        let is_real = b.iter().any(|&i| wrong[k][i - 1]);
                        real += is_real as usize;
                        for &i in b {
                            covered[k][i - 1][sev] = true;
                        }
                        if listing {
                            let nodes = names.iter().zip(b).map(|(n, &i)| format!("{n}={i}:{}", s.tokens[i - 1].form)).collect::<Vec<_>>().join(" ");
                            let verdict = if with_gold { if is_real { "\treal" } else { "\tfalse" } } else { "" };
                            list.push(format!("{}\t{}\t{}{verdict}\t{nodes}\t{}", s.id, r.id, r.severity, s.text));
                        }
                    }
                    // up to three examples, from different sentences
                    if examples.len() < 3 {
                        if let Some(b) = v.first() {
                            examples.push(format!("{}: {}", s.id, b.iter().map(|&i| s.tokens[i - 1].form.as_str()).collect::<Vec<_>>().join(" / ")));
                        }
                    }
                }
                let example = examples.join("; ");
                if with_gold {
                    if bad > 0 {
                        println!("| {} | {} | {fired} | {bad} | {real} | {:.0}% | {} |", r.id, r.severity, 100.0 * real as f64 / bad as f64, example);
                    }
                } else {
                    println!("| {} | {} | {fired} | {bad} | {:.2}% | {} |", r.id, r.severity, 100.0 * bad as f64 / fired.max(1) as f64, example);
                }
            }
            if with_gold {
                let (mut n, mut e, mut hit) = (0usize, 0usize, [0usize; 2]);
                for (w, c) in wrong.iter().zip(&covered) {
                    for (x, y) in w.iter().zip(c) {
                        n += 1;
                        e += *x as usize;
                        for sev in 0..2 {
                            hit[sev] += (*x && y[sev]) as usize;
                        }
                    }
                }
                println!("\ntokens {n}, with error {e}; under violations of error rules {} ({:.1}%), warn rules {} ({:.1}%)", hit[0], 100.0 * hit[0] as f64 / e.max(1) as f64, hit[1], 100.0 * hit[1] as f64 / e.max(1) as f64);
            }
            if listing {
                println!("\n## All violations ({})\n", list.len());
                println!("sent_id\trule\tseverity{}\tnodes\ttext", if with_gold { "\tvs gold" } else { "" });
                for l in &list {
                    println!("{l}");
                }
            }
            Ok(())
        }
        Some("ud-check") => {
            // UD 2.18 registry violations (en::udreg) in CoNLL-U files: positive control of the gate is the gold
            let reg = en::udreg::Registry::load();
            for p in &args[1..] {
                let ts = conllu::read(Path::new(p))?;
                let (mut n, mut bad) = (0usize, 0usize);
                let mut kinds: FastMap<String, usize> = FastMap::default();
                for s in &ts {
                    for t in &s.tokens {
                        let Some(u) = t.upos else { continue };
                        n += 1;
                        let v = reg.check(u, t.feats, t.rel, &t.lemma);
                        if !v.is_empty() {
                            bad += 1;
                            for x in v {
                                *kinds.entry(x).or_default() += 1;
                            }
                        }
                    }
                }
                let mut k: Vec<_> = kinds.into_iter().collect();
                k.sort_by(|a, b| b.1.cmp(&a.1));
                println!("{}: tokens {n}, with violations {bad} ({:.2}%); most frequent: {}", name(Path::new(p)), 100.0 * bad as f64 / n.max(1) as f64, k.iter().take(5).map(|(x, c)| format!("{x} ×{c}")).collect::<Vec<_>>().join("; "));
            }
            Ok(())
        }
        Some("beam-curve") => {
            let (train, test) = split(&args, 1);
            beam_curve(&train, &paths(&test))
        }
        Some("ud-eval") => {
            let (train, test) = split(&args, 1);
            ud_eval(&train, &paths(&test))
        }
        Some("folds") if args.len() == 4 => {
            // en folds <in.conllu> <k> <dir>: documents round-robin into k parts → <name>.fold<f>.conllu
            // (part f) and <name>.rest<f>.conllu (the rest), for the first-pass jackknife
            let k: usize = args[2].parse().map_err(|_| anyhow!("folds: k must be a number"))?;
            folds(Path::new(&args[1]), k, Path::new(&args[3]))
        }
        Some("ctx-report") if args.len() > 3 => {
            // en ctx-report <gold.conllu> <base>=<f1>,<f2>… <system>=<…>… [--focus <name>] [--boot N]
            let gold_path = PathBuf::from(&args[1]);
            let gold = conllu::read(&gold_path)?;
            let (mut systems, mut focus_name, mut boot) = (Vec::new(), None, 1000usize);
            let mut k = 2;
            while k < args.len() {
                match args[k].as_str() {
                    "--focus" => {
                        focus_name = args.get(k + 1).cloned();
                        k += 2;
                    }
                    "--boot" => {
                        boot = args.get(k + 1).and_then(|x| x.parse().ok()).ok_or_else(|| anyhow!("--boot N"))?;
                        k += 2;
                    }
                    a => {
                        let Some((n, fs)) = a.split_once('=') else { bail!("system: <name>=<file>,<file>… — '{a}'") };
                        systems.push(en::ctxeval::System::load(n, &paths(&fs.split(',').map(str::to_string).collect::<Vec<_>>()), &gold)?);
                        k += 1;
                    }
                }
            }
            let focus = match focus_name {
                Some(f) => systems.iter().position(|s| s.name == f).ok_or_else(|| anyhow!("--focus {f}: no such system"))?,
                None => 1,
            };
            print!("{}", en::ctxeval::report(&gold_path, &systems, focus, boot)?);
            Ok(())
        }
        Some("parse-eval") => {
            let (train, test) = split(&args, 1);
            parse_eval(&train, &paths(&test))
        }
        Some("lin-eval") => {
            let (train, test) = split(&args, 1);
            lin_eval(&train, &paths(&test))
        }
        Some("rt-eval") => {
            let (train, test) = split(&args, 1);
            rt_eval(&train, &paths(&test))
        }
        Some("rt-text") => {
            let (train, rest) = split(&args, 1);
            if rest.len() < 2 {
                bail!("rt-text: after -- expected <domain-train.txt> <domain-test.txt> [weight]");
            }
            let weight = rest.get(2).and_then(|w| w.parse().ok()).unwrap_or(5);
            rt_text(&train, Path::new(&rest[0]), Path::new(&rest[1]), weight)
        }
        Some("lexicon") if args.len() > 2 => {
            let (train, out) = split(&args, 2);
            let out = out.first().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("data"));
            let st = en::lexgen::generate(Path::new(&args[1]), &read_all(&train, false)?, &out)?;
            println!("dictionary: lemmas {} (new {}), forms {}, rows {} → {}", st.lemmas, st.new_lemmas, st.forms, st.rows, out.display());
            Ok(())
        }
        Some("graph") => {
            let (train, files) = split(&args, 1);
            graph_dump(&train, &paths(&files))
        }
        Some("explain-json") => {
            let (train, texts) = split(&args, 1);
            explain_json(&train, &texts)
        }
        Some("explain") => {
            let (train, texts) = split(&args, 1);
            explain(&train, &texts)
        }
        Some("train") => {
            let (train, rest) = split(&args, 1);
            en::license::commercial_gate(&train)?;
            train_model(&train, &rest)
        }
        Some("annotate") if args.len() > 2 => annotate_files(Path::new(&args[1]), &paths(&args[2..])),
        Some("convert") if args.len() > 4 => convert(Path::new(&args[1]), &args[2], &args[3], &paths(&args[4..])),
        Some("convert-check") if args.len() > 3 => convert_check(Path::new(&args[1]), &args[2], &paths(&args[3..])),
        _ => bail!(
            "usage: en train <train.conllu>... -- <model> [<check.conllu>...] | annotate <model> <in.conllu|in.txt>... | tok-eval|tag-eval|parse-eval|lin-eval|morph-eval|rt-eval <train.conllu>... -- <test.conllu>... | rt-text <train>... -- <dom-train> <dom-test> [weight] | explain <train>... -- \"sentence\"... | graph <train>... -- <file>... | lexicon <agid infl.txt> <train>... [-- <data-dir>] | convert <rules dir> <from> <to> <in.conllu>... | convert-check <rules dir> <dialect> <in.conllu>... | ctx-report <gold> <base>=<files> <system>=<files>... [--focus <name>] [--boot N]"
        ),
    }
}

fn tok_eval(train: &[PathBuf], test: &[PathBuf]) -> Result<()> {
    let sents = read_all(train, false)?;
    let lx = tok::Lexicon::learn(&sents);
    println!(
        "dictionary from train: abbreviations {}, hyphenated kept {} / split {} (default {}), apostrophe {}, numeric suffixes {}",
        lx.abbrev.len(),
        lx.hyphen_words.len(),
        lx.hyphen_split.len(),
        if lx.hyphen_keep_default { "keep" } else { "split" },
        lx.apos_initial.len(),
        lx.num_suffix.len()
    );
    for p in test {
        let sents = conllu::read(p)?;
        let (mut exact, mut space_ok, mut gold_n, mut sys_n, mut hit) = (0, 0, 0usize, 0usize, 0usize);
        let mut misses: Vec<String> = Vec::new();
        for s in &sents {
            let gold: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
            let sys = tok::tokenize(&s.text, &lx);
            let sysf: Vec<&str> = sys.iter().map(|t| t.form.as_str()).collect();
            gold_n += gold.len();
            sys_n += sysf.len();
            hit += lcs(&gold, &sysf);
            if gold == sysf {
                exact += 1;
                let last = sys.len().saturating_sub(1);
                if sys.iter().zip(&s.tokens).enumerate().all(|(i, (a, b))| a.space_after == b.space_after || i == last) {
                    space_ok += 1;
                }
            } else if misses.len() < 8 {
                misses.push(format!("  UD : {}\n  we : {}", gold.join(" · "), sysf.join(" · ")));
            }
        }
        let p_ = hit as f64 / sys_n.max(1) as f64;
        let r_ = hit as f64 / gold_n.max(1) as f64;
        println!(
            "{}: sentences {}, exact {:.1}% (with spaces {:.1}%), tokens P {:.2}% R {:.2}% F1 {:.2}%",
            name(p),
            sents.len(),
            100.0 * exact as f64 / sents.len().max(1) as f64,
            100.0 * space_ok as f64 / sents.len().max(1) as f64,
            100.0 * p_,
            100.0 * r_,
            200.0 * p_ * r_ / (p_ + r_).max(1e-9)
        );
        for m in misses.iter().take(4) {
            println!("{m}");
        }
    }
    Ok(())
}

fn tag_eval(train: &[PathBuf], test: &[PathBuf]) -> Result<()> {
    let sents = frac(read_all(train, false)?);
    if std::env::var("EN_TAGGER").as_deref() == Ok("perc") {
        return ptag_eval(&sents, test);
    }
    let t0 = std::time::Instant::now();
    let mut tagger = Tagger::train(&sents);
    println!("tagger: {} sentences, {} tags, training {:.1} s", sents.len(), tagger.known_tags(), t0.elapsed().as_secs_f64());
    let hints: Vec<DictHint> = match std::env::var("EN_DICT_HINT").as_deref() {
        Ok("all") => vec![DictHint::Off, DictHint::Soft(5.0), DictHint::Soft(20.0), DictHint::Soft(100.0), DictHint::Hard],
        _ => vec![tagger.dict],
    };
    for hint in hints {
    tagger.dict = hint;
    println!("  dictionary hint: {hint:?}");
    let known: FastSet<&str> = sents.iter().flat_map(|s| s.tokens.iter().map(|t| t.form.as_str())).collect();
    for p in test {
        let test_sents = conllu::read(p)?;
        let (mut ok, mut n, mut unk_ok, mut unk_n, mut sent_ok) = (0usize, 0usize, 0usize, 0usize, 0usize);
        let mut conf: FastMap<(Tag, Tag), usize> = FastMap::default();
        let t1 = std::time::Instant::now();
        for s in test_sents.iter().filter(|s| s.tagged()) {
            let words: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
            let tags = tagger.tag(&words);
            let mut all = true;
            for (&t, g) in tags.iter().zip(&s.tokens) {
                let gt = g.tag.expect("tagged");
                n += 1;
                let unk = !known.contains(g.form.as_str()) && !known.contains(g.form.to_lowercase().as_str());
                if unk {
                    unk_n += 1;
                }
                if t == gt {
                    ok += 1;
                    if unk {
                        unk_ok += 1;
                    }
                } else {
                    all = false;
                    *conf.entry((gt, t)).or_default() += 1;
                }
            }
            if all {
                sent_ok += 1;
            }
        }
        let secs = t1.elapsed().as_secs_f64();
        let mut c: Vec<_> = conf.into_iter().collect();
        c.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        println!(
            "{}: tokens {n}, accuracy {:.2}%, unknown {unk_n} ({:.1}%) with accuracy {:.2}%, error-free sentences {sent_ok}; {:.0} tokens/s",
            name(p),
            100.0 * ok as f64 / n.max(1) as f64,
            100.0 * unk_n as f64 / n.max(1) as f64,
            100.0 * unk_ok as f64 / unk_n.max(1) as f64,
            n as f64 / secs
        );
        println!("  frequent confusions (UD → we): {}", c.iter().take(8).map(|((g, t), k)| format!("{g}→{t} {k}")).collect::<Vec<_>>().join(", "));
    }
    }
    Ok(())
}

fn morph_eval(train: &[PathBuf], test: &[PathBuf]) -> Result<()> {
    let sents = read_all(train, false)?;
    let m = Morph;
    let tagger = Tagger::train(&sents);
    for p in test {
        let ts = conllu::read(p)?;
        let (mut n, mut lem_ok, mut esc, mut lossless, mut esc_pred) = (0usize, 0usize, 0usize, 0usize, 0usize);
        let (mut forms, mut lemmas) = (FastSet::default(), FastSet::default());
        let mut bad: Vec<String> = Vec::new();
        for s in ts.iter().filter(|s| s.tagged()) {
            let words: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
            let pred = tagger.tag(&words);
            for (t, &pt) in s.tokens.iter().zip(&pred) {
                let gt = t.tag.expect("tagged");
                n += 1;
                let l = m.lemmatize(&t.form, gt);
                if l.to_lowercase() == t.lemma.to_lowercase() {
                    lem_ok += 1;
                } else if bad.len() < 12 {
                    bad.push(format!("{}/{}→{} (UD {})", t.form, gt, l, t.lemma));
                }
                forms.insert(t.form.clone());
                // encoding with the predicted tag, as in the real compressor
                let (lemma, case) = m.encode(&t.form, pt);
                if case == Case::Raw {
                    esc_pred += 1;
                    lemmas.insert(format!("\u{1}{}", t.form));
                } else {
                    lemmas.insert(lemma.to_lowercase());
                }
                if m.decode(&lemma, pt, case) == t.form {
                    lossless += 1;
                }
                if m.encode(&t.form, gt).1 == Case::Raw {
                    esc += 1;
                }
            }
        }
        println!(
            "{}: tokens {n}; lemma = UD {:.2}%; escape (gold tags) {:.2}%, (our tags) {:.2}%; lossless {:.2}%; forms {} → lemmas+escapes {}",
            name(p),
            100.0 * lem_ok as f64 / n.max(1) as f64,
            100.0 * esc as f64 / n.max(1) as f64,
            100.0 * esc_pred as f64 / n.max(1) as f64,
            100.0 * lossless as f64 / n.max(1) as f64,
            forms.len(),
            lemmas.len()
        );
        println!("  lemma mismatches: {}", bad.join(", "));
    }
    Ok(())
}

/// Beam curve: one model, different tagger and parser beam widths at parse time: accuracy and speed.
/// Design note: "for real-time responses the beam can be smaller, and for batches larger".
fn beam_curve(train: &[PathBuf], test: &[PathBuf]) -> Result<()> {
    let mut model = Model::train(read_all(train, true)?);
    let tests: Vec<(String, Vec<Sentence>)> = test.iter().map(|p| Ok((name(p).trim_end_matches("-ud-test.conllu").to_string(), read_all(std::slice::from_ref(p), true)?))).collect::<Result<_>>()?;
    println!("| tagger beam | parser beam | {} | tokens/s |", tests.iter().map(|(n, _)| format!("{n}: tag / LAS")).collect::<Vec<_>>().join(" | "));
    println!("|---:|---:|{}---:|", "---:|".repeat(tests.len()));
    for (tb, pb) in [(1, 1), (1, 2), (1, 4), (1, 8), (2, 8), (4, 8), (8, 1), (8, 2), (8, 4), (8, 8), (8, 16), (8, 32)] {
        model.tagger.beam = tb;
        model.parser.beam = pb;
        let (mut cells, mut toks) = (Vec::new(), 0usize);
        let t0 = std::time::Instant::now();
        for (_, ts) in &tests {
            let (mut n, mut tag_ok, mut las) = (0usize, 0usize, 0usize);
            for s in ts {
                let words: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
                let tags = model.tagger.tag(&words);
                let tree = model.parser.parse(&words, &tags);
                for ((t, (h, r)), g) in tags.iter().zip(&tree).zip(&s.tokens) {
                    n += 1;
                    tag_ok += (Some(*t) == g.tag) as usize;
                    las += (*h == g.head && *r == g.rel) as usize;
                }
            }
            toks += n;
            cells.push(format!("{:.2} / {:.2}", 100.0 * tag_ok as f64 / n as f64, 100.0 * las as f64 / n as f64));
        }
        println!("| {tb} | {pb} | {} | {:.0} |", cells.join(" | "), toks as f64 / t0.elapsed().as_secs_f64());
    }
    Ok(())
}

/// Full UD annotation on gold tokens: tagger, parser, lemmas, UPOS and FEATS vs gold, with
/// CoNLL 2018 metrics per test. XPOS is measured only where the treebank has PTB tags.
///
/// Document context experiment (`en::ctx`): `EN_CTX=gold|shuffle` trains with
/// context features (gold or shuffled), `EN_CTX_EVAL=gold,shuffle,…` sets the evaluation
/// contexts (one row each), `EN_CTX_GROUPS` the feature groups, `EN_SEED` the shuffle seed.
/// Without these variables everything is as before, bit for bit. `EN_PRED_DIR=dir` writes each test's annotation as CoNLL-U
/// (`<test>.<context>.conllu`) for `en ctx-report`. The model is not saved anywhere.
fn ud_eval(train: &[PathBuf], test: &[PathBuf]) -> Result<()> {
    use en::ctx::{Ctx, Mode, Setup};
    use en::ud::{Scores, Ud, View, Word, kids_of};
    let setup = Setup::from_env()?;
    // annotation dir before training: a write error must not cost a run (fail-fast)
    let pred_dir = std::env::var("EN_PRED_DIR").ok().map(PathBuf::from);
    if let Some(d) = &pred_dir {
        std::fs::create_dir_all(d).with_context(|| format!("EN_PRED_DIR {}", d.display()))?;
    }
    let sents = read_all(train, true)?;
    let t0 = std::time::Instant::now();
    let ud = Ud::train(&sents);
    let model = if setup.train == Mode::None {
        Model::train(sents)
    } else {
        let ctx = train_ctx(train, &sents, &setup)?;
        Model::train_ctx(sents, &ctx)
    };
    println!("model and UD tables: {:.1} s\n", t0.elapsed().as_secs_f64());
    if setup.train != Mode::None || en::ctx::seed() != 0 {
        println!(
            "context: training {}, evaluation {}, groups {}, seed {}\n",
            setup.train.name(),
            setup.eval.iter().map(|m| m.name()).collect::<Vec<_>>().join(","),
            en::ctx::group::names(setup.groups),
            en::ctx::seed()
        );
    }
    println!("{}", Scores::HEADER);
    println!("|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|");
    for p in test {
        let ts = conllu::read(p)?;
        if ts.first().is_some_and(|s| !s.has_forms) {
            println!("| {} | — | text removed from the treebank (source license) |", name(p).trim_end_matches("-ud-test.conllu"));
            continue;
        }
        // XPOS is counted only if the treebank really is PTB (≥ 90% of tags recognized)
        let (ptb, all) = ts.iter().flat_map(|s| &s.tokens).fold((0usize, 0usize), |(a, b), t| (a + t.tag.is_some() as usize, b + 1));
        let ptb_bank = ptb * 10 >= all * 9;
        for &mode in &setup.eval {
            let ctxs: Option<Vec<Ctx>> = test_ctx(p, &ts, mode, &setup)?;
            let mut sc = Scores::default();
            let debug = std::env::var("EN_UD_DEBUG").is_ok();
            let mut lemma_miss: FastMap<String, usize> = FastMap::default();
            let mut dump = String::new();
            for (k, s) in ts.iter().enumerate() {
                let words: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
                let cx = ctxs.as_ref().map(|c| &c[k]);
                let (tags, tree) = match cx {
                    None => {
                        let tags = model.tagger.tag(&words);
                        let tree = model.parser.parse(&words, &tags);
                        (tags, tree)
                    }
                    Some(c) => {
                        let tags = model.tagger.tag_ctx(&words, Some(c));
                        let tree = model.parser.parse_ctx(&words, &tags, Some(c));
                        (tags, tree)
                    }
                };
                let heads: Vec<usize> = tree.iter().map(|x| x.0).collect();
                let rels: Vec<Rel> = tree.iter().map(|x| x.1).collect();
                let kids = kids_of(&heads, &rels);
                let upos: Vec<UPos> = (0..words.len()).map(|i| ud.upos(&View { form: words[i], tag: tags[i], rel: rels[i], kids: kids[i], agr: 0 })).collect();
                let feats = ud.feats_sentence(&words, &tags, &upos, &heads, &rels);
                let pred: Vec<Word> = (0..words.len())
                    .map(|i| Word { upos: Some(upos[i]), tag: Some(tags[i]), feats: feats[i], lemma: model.morph.lemmatize(words[i], tags[i]), head: heads[i], rel: rels[i] })
                    .collect();
                let gold: Vec<Word> = s
                    .tokens
                    .iter()
                    .map(|t| Word { upos: t.upos, tag: if ptb_bank { t.tag } else { None }, feats: t.feats, lemma: t.lemma.clone(), head: t.head, rel: t.rel })
                    .collect();
                sc.add(&gold, &pred, s.has_feats, s.has_lemmas);
                if debug && s.has_lemmas {
                    for (g, pr) in gold.iter().zip(&pred) {
                        if g.lemma != pr.lemma {
                            *lemma_miss.entry(format!("{}→{} ({}/{})", g.lemma, pr.lemma, g.tag.map_or("?", |t| t.name()), pr.tag.map_or("?", |t| t.name()))).or_default() += 1;
                        }
                    }
                }
                if pred_dir.is_some() {
                    let aw: Vec<en::annotate::Word> = (0..words.len())
                        .map(|i| en::annotate::Word { form: words[i].to_string(), lemma: pred[i].lemma.clone(), upos: upos[i], tag: tags[i], feats: feats[i], head: heads[i], rel: rels[i] })
                        .collect();
                    let space: Vec<bool> = s.tokens.iter().map(|t| t.space_after).collect();
                    dump.push_str(&format!("# sent_id = {}\n# text = {}\n", s.id, s.text));
                    for r in en::annotate::rows(&aw, &space) {
                        dump.push_str(&r.join("\t"));
                        dump.push('\n');
                    }
                    dump.push('\n');
                }
            }
            if debug {
                let mut v: Vec<_> = std::mem::take(&mut lemma_miss).into_iter().collect();
                v.sort_by(|a, b| b.1.cmp(&a.1));
                println!("  lemmas, gold→ours: {}", v.iter().take(25).map(|(k, c)| format!("{k} {c}")).collect::<Vec<_>>().join(", "));
            }
            let label = name(p).trim_end_matches("-ud-test.conllu").to_string();
            let label = if setup.train == Mode::None { label } else { format!("{label} [{}]", mode.name()) };
            println!("| {} | {} | {} |", label, sc.n, sc.row());
            if let Some(d) = &pred_dir {
                let stem = name(p).trim_end_matches(".conllu").to_string();
                let f = d.join(format!("{stem}.{}.conllu", mode.name()));
                std::fs::write(&f, dump).with_context(|| format!("writing {}", f.display()))?;
            }
        }
    }
    Ok(())
}

/// `en folds`: the file's documents round-robin into `k` parts (a document runs from `# newdoc` to the next),
/// written losslessly: `<name>.fold<f>.conllu` is part f, `<name>.rest<f>.conllu` the rest.
fn folds(input: &Path, k: usize, out: &Path) -> Result<()> {
    if k < 2 {
        bail!("folds: k ≥ 2");
    }
    let doc = conllu::Doc::read(input)?;
    let mut d = 0usize;
    let mut of: Vec<usize> = Vec::with_capacity(doc.sents.len());
    for (i, s) in doc.sents.iter().enumerate() {
        if i > 0 && s.comments.iter().any(|c| c.starts_with("# newdoc")) {
            d += 1;
        }
        of.push(d % k);
    }
    std::fs::create_dir_all(out)?;
    let stem = name(input).trim_end_matches(".conllu").to_string();
    for f in 0..k {
        for (tag, keep) in [("fold", true), ("rest", false)] {
            let part = conllu::Doc { blank_before: 0, sents: doc.sents.iter().zip(&of).filter(|(_, x)| (**x == f) == keep).map(|(s, _)| s.clone()).collect(), final_newline: true };
            let path = out.join(format!("{stem}.{tag}{f}.conllu"));
            std::fs::write(&path, part.text()).with_context(|| format!("writing {}", path.display()))?;
            println!("{}: sentences {}", path.display(), part.sents.len());
        }
    }
    println!("documents {}", d + 1);
    Ok(())
}

/// Context of training sentences (in `read_all(train, true)` order): gold or shuffled
/// (donors are other training documents), enabled groups only.
fn train_ctx(train: &[PathBuf], sents: &[Sentence], setup: &en::ctx::Setup) -> Result<Vec<en::ctx::Ctx>> {
    use en::ctx::{Ctx, Mode};
    let (mut ctx, mut docs, mut lens): (Vec<Ctx>, Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new(), Vec::new());
    let mut base = 0usize;
    // stage 2: first-pass annotation of training sentences (jackknife, `EN_CTX_TRAIN_PASS1`), so the
    // second-pass model learns on the same context as at evaluation, not on the gold one
    let mut pass1: FastMap<String, Sentence> = FastMap::default();
    for f in &setup.train_pass1 {
        for s in conllu::read(f)? {
            pass1.insert(s.id.clone(), s);
        }
    }
    let mut used = 0usize;
    for p in train {
        let doc = conllu::Doc::read(p)?;
        let views: Vec<&Sentence> = doc.sents.iter().map(|s| &s.sent).filter(|s| !s.tokens.is_empty()).collect();
        let (c, d) = if setup.rule {
            let (gold, _) = en::ctx::gold(&doc, false);
            let mut ann = Vec::with_capacity(views.len());
            for (s, g) in views.iter().zip(&gold) {
                match pass1.get(&s.id) {
                    Some(x) if x.tokens.len() == s.tokens.len() => {
                        ann.push(x.clone());
                        used += 1;
                    }
                    Some(_) => bail!("{}: {}: different token count in the first pass", p.display(), s.id),
                    None if !pass1.is_empty() && !g.is_empty() => bail!("{}: {}: missing from the first pass (EN_CTX_TRAIN_PASS1)", p.display(), s.id),
                    None => ann.push((*s).clone()),
                }
            }
            en::ctx::derived(&doc, &ann)?
        } else {
            en::ctx::gold(&doc, setup.ewt)
        };
        if views.len() != c.len() {
            bail!("{}: context for {} sentences, but there are {} sentences", p.display(), c.len(), views.len());
        }
        for ((s, c), d) in views.iter().zip(c).zip(d) {
            if s.tagged() {
                lens.push(s.tokens.len());
                ctx.push(c);
                docs.push(base + d);
            }
        }
        base = docs.last().map_or(base, |&d| d + 1);
    }
    if ctx.len() != sents.len() {
        bail!("training context: {} sentences, but {} training sentences", ctx.len(), sents.len());
    }
    let ctx = if setup.train == Mode::Shuffle { en::ctx::shuffled(&ctx, &docs, &lens, en::ctx::seed() ^ 0x7472_6169_6e) } else { ctx };
    let n_ctx = ctx.iter().filter(|c| !c.is_empty()).count();
    eprintln!("training context ({}): sentences with context {n_ctx} of {}; from first-pass annotation {used}", setup.train.name(), ctx.len());
    Ok(ctx.into_iter().map(|c| c.masked(setup.groups)).collect())
}

/// Context of test sentences for an evaluation mode: `None` means no context; gold; shuffled
/// (donors are other test documents).
fn test_ctx(p: &Path, ts: &[Sentence], mode: en::ctx::Mode, setup: &en::ctx::Setup) -> Result<Option<Vec<en::ctx::Ctx>>> {
    use en::ctx::Mode;
    if mode == Mode::None {
        return Ok(None);
    }
    let doc = conllu::Doc::read(p)?;
    let (gold, docs) = en::ctx::gold(&doc, setup.ewt);
    if gold.len() != ts.len() {
        bail!("{}: context for {} sentences, but there are {} sentences", p.display(), gold.len(), ts.len());
    }
    let lens: Vec<usize> = ts.iter().map(|s| s.tokens.len()).collect();
    // stage 2: rule-based context, over the gold (gold) or over first-pass annotation (pred, shuffle)
    let pass1 = || -> Result<Vec<Sentence>> {
        let dir = setup.pass1.as_ref().ok_or_else(|| anyhow!("EN_PASS1_DIR is not set"))?;
        let f = dir.join(format!("{}.none.conllu", name(p).trim_end_matches(".conllu")));
        let v = conllu::read(&f)?;
        if v.len() != ts.len() || v.iter().zip(ts).any(|(a, b)| a.id != b.id || a.tokens.len() != b.tokens.len()) {
            bail!("{}: first pass does not match test {}", f.display(), p.display());
        }
        Ok(v)
    };
    let ctx = if setup.rule {
        let ann = if mode == Mode::Gold { ts.to_vec() } else { pass1()? };
        let (c, _) = en::ctx::derived(&doc, &ann)?;
        let hit = c.iter().zip(&gold).filter(|(a, b)| a.stype == b.stype).count();
        eprintln!("{} [{}]: rule-based sentence type matches gold in {hit} of {} ({:.1}%)", name(p), mode.name(), c.len(), 100.0 * hit as f64 / c.len().max(1) as f64);
        if mode == Mode::Shuffle { en::ctx::shuffled(&c, &docs, &lens, en::ctx::seed() ^ 0x7465_7374) } else { c }
    } else {
        match mode {
            Mode::Gold => gold,
            Mode::Shuffle => en::ctx::shuffled(&gold, &docs, &lens, en::ctx::seed() ^ 0x7465_7374),
            Mode::Pred => bail!("EN_CTX_EVAL=pred requires EN_CTX_SRC=rule"),
            Mode::None => unreachable!(),
        }
    };
    Ok(Some(ctx.into_iter().map(|c| c.masked(setup.groups)).collect()))
}

/// Perceptron tagger on the same tests: overall accuracy and accuracy on unknown words.
fn ptag_eval(sents: &[Sentence], test: &[PathBuf]) -> Result<()> {
    let epochs: usize = std::env::var("EN_EPOCHS").ok().and_then(|x| x.parse().ok()).unwrap_or(8);
    // negative control: EN_NOISE=0.05 replaces that share of training tags with random ones (deterministically)
    let noise: f64 = std::env::var("EN_NOISE").ok().and_then(|x| x.parse().ok()).unwrap_or(0.0);
    let mut noisy: Vec<Sentence> = sents.iter().filter(|s| s.tagged()).cloned().collect();
    let mut flipped: FastSet<(u32, u32)> = FastSet::default();
    if noise > 0.0 {
        let mut r: u64 = 0x2545_F491_4F6C_DD1D;
        for (k, s) in noisy.iter_mut().enumerate() {
            for (i, t) in s.tokens.iter_mut().enumerate() {
                r ^= r << 13;
                r ^= r >> 7;
                r ^= r << 17;
                if (r % 10_000) as f64 / 10_000.0 < noise {
                    let old = t.tag.expect("tagged");
                    let new = Tag::ALL[((r >> 20) as usize) % (Tag::N - 2)];
                    if new != old {
                        t.tag = Some(new);
                        flipped.insert((k as u32, i as u32));
                    }
                }
            }
        }
        println!("noise: replaced {} tags ({:.1}%)", flipped.len(), 100.0 * noise);
    }
    let sents = &noisy[..];
    let t0 = std::time::Instant::now();
    let tagger = en::ptag::PTagger::train(sents, epochs);
    if !tagger.suspects.is_empty() {
        let hit = tagger.suspects.iter().filter(|x| flipped.contains(x)).count();
        println!(
            "suspects (model disagreed in every epoch): {}{}",
            tagger.suspects.len(),
            if flipped.is_empty() { String::new() } else { format!("; of them deliberately corrupted {hit} (precision {:.1}%, found {:.1}% of corrupted)", 100.0 * hit as f64 / tagger.suspects.len() as f64, 100.0 * hit as f64 / flipped.len().max(1) as f64) }
        );
        if let Ok(path) = std::env::var("EN_SUSPECTS") {
            let mut out = String::new();
            for &(k, i) in &tagger.suspects {
                let s = &sents[k as usize];
                let t = &s.tokens[i as usize];
                let ctx: Vec<String> = s.tokens.iter().enumerate().map(|(j, x)| if j == i as usize { format!("[{}]", x.form) } else { x.form.clone() }).collect();
                out.push_str(&format!("{}\t{}\t{}\t{}\n", s.id, t.form, t.tag.map_or("?", |x| x.name()), ctx.join(" ")));
            }
            std::fs::write(&path, out)?;
            println!("annotation error candidates → {path}");
        }
    }
    println!("perceptron: {} sentences, {epochs} epochs, {} features, training {:.1} s", sents.len(), tagger.size(), t0.elapsed().as_secs_f64());
    let known: FastSet<&str> = sents.iter().flat_map(|s| s.tokens.iter().map(|t| t.form.as_str())).collect();
    for p in test {
        let (mut ok, mut n, mut unk_ok, mut unk_n) = (0usize, 0usize, 0usize, 0usize);
        let mut conf: FastMap<(Tag, Tag), usize> = FastMap::default();
        let t1 = std::time::Instant::now();
        for s in conllu::read(p)?.iter().filter(|s| s.tagged()) {
            let words: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
            for (&t, g) in tagger.tag(&words).iter().zip(&s.tokens) {
                let unk = !known.contains(g.form.as_str()) && !known.contains(g.form.to_lowercase().as_str());
                n += 1;
                unk_n += unk as usize;
                if Some(t) == g.tag {
                    ok += 1;
                    unk_ok += unk as usize;
                } else if unk {
                    *conf.entry((g.tag.expect("tagged"), t)).or_default() += 1;
                }
            }
        }
        if std::env::var("EN_CONF").is_ok() {
            let mut v: Vec<_> = conf.into_iter().collect();
            v.sort_by(|a, b| b.1.cmp(&a.1));
            println!("  unknown, gold → ours: {}", v.iter().take(10).map(|((g, t), c)| format!("{g}→{t} {c}")).collect::<Vec<_>>().join(", "));
        }
        println!(
            "{}: tokens {n}, accuracy {:.2}%, unknown {unk_n} with accuracy {:.2}%; {:.0} tokens/s",
            name(p),
            100.0 * ok as f64 / n.max(1) as f64,
            100.0 * unk_ok as f64 / unk_n.max(1) as f64,
            n as f64 / t1.elapsed().as_secs_f64()
        );
    }
    Ok(())
}

fn parse_eval(train: &[PathBuf], test: &[PathBuf]) -> Result<()> {
    let sents = frac(read_all(train, true)?);
    let t0 = std::time::Instant::now();
    let tagger = en::ptag::PTagger::train(&sents, 6);
    let folds: usize = std::env::var("EN_JACKKNIFE").ok().and_then(|x| x.parse().ok()).unwrap_or(0);
    let beam: usize = std::env::var("EN_BEAM").ok().and_then(|x| x.parse().ok()).unwrap_or(8);
    let epochs: usize = std::env::var("EN_PEPOCHS").ok().and_then(|x| x.parse().ok()).unwrap_or(if beam > 1 { 12 } else { 10 });
    let data = if folds > 1 { en::ptag::jackknife(&sents, folds, 6) } else { sents.clone() };
    let parser = if beam > 1 { Parser::train_beam(&data, epochs, beam) } else { Parser::train(&data, epochs) };
    let labels: FastSet<Rel> = sents.iter().flat_map(|s| s.tokens.iter().map(|t| t.rel)).collect();
    println!("parser: {} sentences, {} labels, training {:.1} s", sents.len(), labels.len(), t0.elapsed().as_secs_f64());
    for p in test {
        let ts = read_all(std::slice::from_ref(p), true)?;
        for (label, pred_tags) in [("gold tags", false), ("our tags", true)] {
            let (mut n, mut uas, mut las) = (0usize, 0usize, 0usize);
            let t1 = std::time::Instant::now();
            for s in &ts {
                let words: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
                let tags: Vec<Tag> = if pred_tags { tagger.tag(&words) } else { s.tokens.iter().map(|t| t.tag.expect("tagged")).collect() };
                let out = parser.parse(&words, &tags);
                for (&(h, l), g) in out.iter().zip(&s.tokens) {
                    n += 1;
                    if h == g.head {
                        uas += 1;
                        if l == g.rel {
                            las += 1;
                        }
                    }
                }
            }
            println!(
                "{} ({label}): tokens {n}, UAS {:.2}%, LAS {:.2}%; {:.0} tokens/s",
                name(p),
                100.0 * uas as f64 / n.max(1) as f64,
                100.0 * las as f64 / n.max(1) as f64,
                n as f64 / t1.elapsed().as_secs_f64()
            );
        }
    }
    Ok(())
}

fn lin_eval(train: &[PathBuf], test: &[PathBuf]) -> Result<()> {
    let sents = read_all(train, true)?;
    let lz = Linearizer::train(&sents);
    for p in test {
        let ts = read_all(std::slice::from_ref(p), true)?;
        let (mut exact, mut hit, mut tot) = (0usize, 0usize, 0usize);
        let mut shown = 0;
        for s in &ts {
            let (nodes, qmark) = lin::sentence_nodes(s, dict::lex);
            let ord = lz.order(&nodes, lin::mood(&nodes, qmark));
            let (h, t) = bigram_precision(&ord);
            hit += h;
            tot += t;
            if ord.iter().enumerate().all(|(i, &x)| x == i + 1) {
                exact += 1;
            } else if shown < 5 && s.tokens.len() < 14 {
                shown += 1;
                println!("  original:  {}\n  generator: {}", s.text, ord.iter().map(|&i| s.tokens[i - 1].form.as_str()).collect::<Vec<_>>().join(" "));
            }
        }
        println!(
            "{}: sentences {}, exact order {:.1}%, bigrams in place {:.1}%",
            name(p),
            ts.len(),
            100.0 * exact as f64 / ts.len().max(1) as f64,
            100.0 * hit as f64 / tot.max(1) as f64
        );
    }
    Ok(())
}

fn rt_eval(train: &[PathBuf], test: &[PathBuf]) -> Result<()> {
    let t0 = std::time::Instant::now();
    let model = Model::train(read_all(train, true)?);
    println!("model: training {:.1} s; dictionary in code: lemmas {}, forms {}", t0.elapsed().as_secs_f64(), dict::N_LEMMAS, dict::N_FORMS);
    for p in test {
        let ts = conllu::read(p)?;
        let (mut n, mut exact_ll, mut exact_gen, mut ord_ok) = (0usize, 0usize, 0usize, 0usize);
        let mut chrf_sum = 0.0f64;
        let mut shown = 0;
        let t1 = std::time::Instant::now();
        for s in ts.iter().filter(|s| !s.text.is_empty()) {
            n += 1;
            let g = model.encode(&s.text);
            if model.decode(&g, true) == s.text {
                exact_ll += 1;
            }
            let ord = model.order(&g);
            if ord.iter().enumerate().all(|(k, &x)| x == k + 1) {
                ord_ok += 1;
            }
            let out = model.decode(&g, false);
            if out == s.text {
                exact_gen += 1;
            } else if shown < 6 && s.tokens.len() < 16 {
                shown += 1;
                println!("  input : {}\n  output: {}", s.text, out);
            }
            chrf_sum += chrf(&out, &s.text);
        }
        println!(
            "{}: sentences {n}; lossless exact {:.1}%; generative: exact text {:.1}%, order {:.1}%, chrF {:.1}; {:.0} sentences/s",
            name(p),
            100.0 * exact_ll as f64 / n.max(1) as f64,
            100.0 * exact_gen as f64 / n.max(1) as f64,
            100.0 * ord_ok as f64 / n.max(1) as f64,
            100.0 * chrf_sum / n.max(1) as f64,
            n as f64 / t1.elapsed().as_secs_f64()
        );
    }
    Ok(())
}

fn read_lines(p: &Path) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(p)?.lines().map(str::trim).filter(|l| !l.is_empty()).map(str::to_string).collect())
}

fn rt_report(model: &Model, test: &[String], label: &str) {
    let (mut ll, mut gen_ok, mut chrf_sum) = (0usize, 0usize, 0.0f64);
    let mut shown = 0;
    for t in test {
        let g = model.encode(t);
        if model.decode(&g, true) == *t {
            ll += 1;
        }
        let out = model.decode(&g, false);
        if out == *t {
            gen_ok += 1;
        } else if shown < 4 {
            shown += 1;
            println!("    input : {t}\n    output: {out}");
        }
        chrf_sum += chrf(&out, t);
    }
    let n = test.len().max(1) as f64;
    println!("  {label}: sentences {}, lossless {:.1}%, generative exact {:.1}%, chrF {:.1}", test.len(), 100.0 * ll as f64 / n, 100.0 * gen_ok as f64 / n, 100.0 * chrf_sum / n);
}

fn rt_text(train: &[PathBuf], dom_train: &Path, dom_test: &Path, weight: usize) -> Result<()> {
    let mut model = Model::train(read_all(train, true)?);
    let dtrain = read_lines(dom_train)?;
    let dtest = read_lines(dom_test)?;
    println!("domain: training {}, test {}", dtrain.len(), dtest.len());
    rt_report(&model, &dtest, "general model");
    model.adapt(&dtrain, weight);
    rt_report(&model, &dtest, &format!("fine-tuned on domain (weight {weight})"));
    Ok(())
}

/// Indented tree: the head, with its dependents below in text order.
fn print_tree(h: usize, depth: usize, kids: &[Vec<usize>], e: &en::explain::Explained) {
    println!("   {}{} ({})", "   ".repeat(depth), e.words[h - 1].form, e.tree[h - 1].1);
    for &k in &kids[h] {
        print_tree(k, depth + 1, kids, e);
    }
}

/// Every compressor step for sentences, to explain it to a human.
fn explain(train: &[PathBuf], texts: &[String]) -> Result<()> {
    eprintln!("training model (~5 s)…");
    let model = Model::train(read_all(train, true)?);
    let fsa = en::explain::fsa();
    for text in texts {
        let e = en::explain::explain(&model, text);
        println!("\n═══ «{text}» ═══");

        println!("\n1. Tokenizer: a finite automaton; each character has a class (Le letter, Di digit, Sp space, Pe period, Ap apostrophe…),");
        println!("   the table (state × class) gives a command (App append, Sep close word, One single mark, New new word) and a new state.");
        let shown: Vec<String> =
            e.fsa.iter().take(24).map(|s| format!("«{}» {} {} {}→{}", s.ch, fsa.classes[s.class][0], fsa.cmds[s.cmd][0], fsa.states[s.state][0], fsa.states[s.next][0])).collect();
        println!("   first characters: {}", shown.join(" · "));
        println!("   tokens: {}", e.words.iter().map(|w| w.form.as_str()).collect::<Vec<_>>().join(" │ "));

        println!("\n2. Tagger: an averaged perceptron with a beam; features of the word and its neighbours (suffixes, shape, in-code dictionary, previous tags) → PTB tag; for unknown words, TnT prior classes (suffix + dictionary).");
        println!("   {}", e.words.iter().map(|w| format!("{}/{}", w.form, w.tag)).collect::<Vec<_>>().join("  "));

        println!("\n3. Morphology: form → lemma + case; if the form cannot be rebuilt from the lemma, an escape (the form as is).");
        println!("   {}", e.words.iter().map(|w| format!("{} → {} [{}]", w.form, w.lemma, w.case)).collect::<Vec<_>>().join(" · "));

        println!("\n4. Parser: a transition automaton with a stack + word buffer; at each step a weight table picks an action:");
        println!("   SHIFT moves a word from the buffer to the stack; LEFT(rel): the top stack word becomes head of the one below it; RIGHT(rel): the reverse.");
        let wname = |i: usize| if i == 0 { "ROOT".to_string() } else { e.words[i - 1].form.clone() };
        for (k, st) in e.steps.iter().enumerate() {
            let stack: Vec<String> = st.stack.iter().map(|&i| wname(i)).collect();
            let buf = if st.buffer <= e.words.len() { e.words[st.buffer - 1..].iter().map(|w| w.form.as_str()).collect::<Vec<_>>().join(" ") } else { String::new() };
            let act = match st.arc {
                Some((h, d, r)) => format!("{}({r}): {} → {}", st.act, wname(h), wname(d)),
                None => st.act.to_string(),
            };
            println!("   {:>2}. stack [{}] · buffer [{}] ⇒ {act}", k + 1, stack.join(" "), buf);
        }
        let n = e.words.len();
        let mut kids: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for (i, &(h, _)) in e.tree.iter().enumerate() {
            kids[h.min(n)].push(i + 1);
        }
        println!("   tree:");
        for &r in &kids[0] {
            print_tree(r, 0, &kids, &e);
        }

        println!("\n5. Graph, what is stored: a vertex is 12 bytes (lemma 4, tag 1, case 1, relation 1, space 1, head 2, reserved 2), no strings.");
        println!("   {:>2}  {:<22} {:<5} {:<6} {:<14} {:>6}  {:<6} bytes", "#", "lemma (id)", "tag", "case", "relation", "head", "space");
        for (i, v) in e.graph.iter().enumerate() {
            let id = if v.local { format!("loc.{}", v.id) } else { v.id.to_string() };
            println!(
                "   {:>2}  {:<22} {:<5} {:<6} {:<14} {:>6}  {:<6} {}",
                i + 1,
                format!("{} ({id})", v.lemma),
                v.tag,
                v.case,
                v.rel,
                v.head,
                if v.space { "yes" } else { "no" },
                v.bytes
            );
        }
        println!(
            "   sentence bits: question: {}; first word capitalized: {}; local heap (new words): {}",
            if e.question { "yes" } else { "no" },
            if e.cap_first { "yes" } else { "no" },
            if e.local.is_empty() { "—".to_string() } else { e.local.join(", ") }
        );

        println!("\n6. Generator: from the graph back to text:");
        println!("   lossless (vertex order preserved):  {}", e.lossless);
        println!("   generative (order from the tree: side of each dependent + chain on that side): {}", e.generative);
        println!("   generator vertex order: {}", e.order.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" "));
    }
    Ok(())
}

/// Data for the explanation page (JSON): automaton table, in-code dictionary, steps for sentences.
fn explain_json(train: &[PathBuf], texts: &[String]) -> Result<()> {
    let model = Model::train(read_all(train, true)?);
    let sentences: Vec<en::explain::Explained> = texts.iter().map(|t| en::explain::explain(&model, t)).collect();
    let out = serde_json::json!({
        "fsa": en::explain::fsa(),
        "dict": { "lemmas": dict::N_LEMMAS, "forms": dict::N_FORMS, "constants": dict::w::NAMES.len() },
        "tags": en::explain::tag_names(),
        "sentences": sentences,
    });
    println!("{}", serde_json::to_string(&out)?);
    Ok(())
}

/// Line graphs of files, one line per sentence: `lemma/tag/head/relation` (for version comparison).
fn graph_dump(train: &[PathBuf], files: &[PathBuf]) -> Result<()> {
    let model = Model::train(read_all(train, true)?);
    for f in files {
        for line in read_lines(f)? {
            let g = model.encode(&line);
            let s: Vec<String> = g.nodes.iter().map(|n| format!("{}/{}/{}/{}", g.str(n.lemma), n.tag, n.head, n.rel)).collect();
            println!("{}", s.join(" "));
        }
    }
    Ok(())
}

/// Converter rules from a directory (`convert.md` files under it): any parse error stops (fail-fast).
fn load_rules(dir: &Path) -> Result<Vec<en::convert::ConvRule>> {
    let (rules, errs) = en::convert::load(dir);
    for e in &errs {
        eprintln!("NOT PARSED {e}");
    }
    if !errs.is_empty() {
        bail!("{}: converter rules not parsed: {}", dir.display(), errs.len());
    }
    Ok(rules)
}

/// Dialect pairs present in the rules, for error messages.
fn rule_pairs(rules: &[en::convert::ConvRule]) -> String {
    let mut v: Vec<String> = rules.iter().map(|r| format!("{} → {}", r.from, r.to)).collect();
    v.sort();
    v.dedup();
    if v.is_empty() { "none".into() } else { v.join(", ") }
}

/// `en convert`: rules of the pair `from → to` on files, CoNLL-U to stdout; summary to stderr.
fn convert(dir: &Path, from: &str, to: &str, files: &[PathBuf]) -> Result<()> {
    let rules = load_rules(dir)?;
    let chosen = en::convert::pair(&rules, from, to);
    if chosen.is_empty() {
        bail!("no rules {from} → {to}; pairs in the directory: {}", rule_pairs(&rules));
    }
    let mut out = std::io::stdout().lock();
    let (mut sents, mut words, mut changed) = (0usize, 0usize, 0usize);
    for p in files {
        let mut doc = conllu::Doc::read(p)?;
        for s in &mut doc.sents {
            let before = s.clone();
            en::convert::apply(&chosen, s).map_err(|e| anyhow!("{}: {}: {e}", p.display(), before.label()))?;
            sents += 1;
            words += s.words.len();
            changed += en::convert::diff(&before, s).words();
        }
        out.write_all(doc.text().as_bytes())?;
    }
    eprintln!("rules {from} → {to}: {}; sentences {sents}, words {words}, words changed {changed}", chosen.len());
    Ok(())
}

/// Per-column differences between two versions of sentences: words, sentences, value pairs, examples.
#[derive(Default)]
struct ColStat {
    words: [usize; 10],
    sents: [usize; 10],
    swaps: [FastMap<(String, String), usize>; 10],
    examples: [Vec<String>; 10],
    any_words: usize,
    any_sents: usize,
    other: usize,
}

impl ColStat {
    fn add(&mut self, a: &conllu::FullSentence, b: &conllu::FullSentence) -> en::convert::Diff {
        let d = en::convert::diff(a, b);
        for (c, ws) in d.cols.iter().enumerate() {
            self.words[c] += ws.len();
            self.sents[c] += !ws.is_empty() as usize;
            for &w in ws {
                let (x, y) = (a.word(w - 1).0[c].clone(), b.word(w - 1).0[c].clone());
                if self.examples[c].len() < 3 {
                    self.examples[c].push(format!("{} #{w} «{}»: {x} → {y}", a.label(), a.word(w - 1).0[1]));
                }
                *self.swaps[c].entry((x, y)).or_default() += 1;
            }
        }
        self.any_words += d.words();
        self.any_sents += !d.is_empty() as usize;
        self.other += d.other as usize;
        d
    }

    /// Most frequent "before → after" pairs in a column.
    fn top(&self, c: usize, k: usize) -> String {
        let mut v: Vec<_> = self.swaps[c].iter().collect();
        v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        v.iter().take(k).map(|((x, y), n)| format!("{x} → {y} ×{n}")).collect::<Vec<_>>().join("; ")
    }
}

/// `en convert-check`: X → mova → X. Forward: what the converter changed; backward: what did not return to
/// the original, per column. For a lossless pair the backward diff is zero.
fn convert_check(dir: &Path, dialect: &str, files: &[PathBuf]) -> Result<()> {
    use en::conllu::Col;
    let rules = load_rules(dir)?;
    if dialect == "mova" {
        bail!("convert-check measures X → mova → X; X must not be mova");
    }
    let (fwd, back) = (en::convert::pair(&rules, dialect, "mova"), en::convert::pair(&rules, "mova", dialect));
    if fwd.is_empty() && back.is_empty() {
        bail!("dialect '{dialect}' is not in the directory's rules; pairs: {}", rule_pairs(&rules));
    }
    let (mut f, mut b) = (ColStat::default(), ColStat::default());
    let (mut n_sents, mut n_words, mut restored, mut extra) = (0usize, 0usize, 0usize, 0usize);
    // per rule: how many words it changed; for backward rules, how many of them match the original after all rules
    let (mut by_fwd, mut by_back) = (vec![0usize; fwd.len()], vec![[0usize; 2]; back.len()]);
    for p in files {
        let doc = conllu::Doc::read(p)?;
        for s in &doc.sents {
            n_sents += 1;
            n_words += s.words.len();
            let mut m = s.clone();
            let cf = en::convert::apply(&fwd, &mut m).map_err(|e| anyhow!("{}: {}: {dialect} → mova: {e}", p.display(), s.label()))?;
            let df = f.add(s, &m);
            let cb = en::convert::apply(&back, &mut m).map_err(|e| anyhow!("{}: {}: mova → {dialect}: {e}", p.display(), s.label()))?;
            let db = b.add(s, &m);
            for (k, ws) in cf.iter().enumerate() {
                by_fwd[k] += ws.len();
            }
            for (k, ws) in cb.iter().enumerate() {
                by_back[k][0] += ws.len();
                by_back[k][1] += ws.iter().filter(|&&w| m.word(w) == s.word(w)).count();
            }
            // words changed by forward that came back exactly; and words changed only by backward
            let set = |d: &en::convert::Diff| d.cols.iter().flatten().copied().collect::<FastSet<usize>>();
            let (sf, sb) = (set(&df), set(&db));
            restored += sf.difference(&sb).count();
            extra += sb.difference(&sf).count();
        }
    }
    let cols = [Col::Upos, Col::Xpos, Col::Feats, Col::Lemma, Col::Head, Col::Deprel, Col::Deps, Col::Misc];
    println!("rules {dialect} → mova: {}, mova → {dialect}: {}; files {}, sentences {n_sents}, words {n_words}", fwd.len(), back.len(), files.len());
    println!("\n## Forward: {dialect} → mova\n");
    println!("words changed {} in {} sentences", f.any_words, f.any_sents);
    println!("\n| column | words | sentences | replacements (before → after) |\n|---|---:|---:|---|");
    for c in cols.iter().map(|&c| c as usize).filter(|&c| f.words[c] > 0) {
        println!("| {} | {} | {} | {} |", Col::ALL[c].name(), f.words[c], f.sents[c], f.top(c, 6));
    }
    println!("\n## Backward: {dialect} → mova → {dialect} vs the original\n");
    println!(
        "differing words {} in {} sentences; of those changed by forward, restored exactly {restored} of {} ({:.1}%); changed only by backward {extra}",
        b.any_words,
        b.any_sents,
        f.any_words,
        100.0 * restored as f64 / f.any_words.max(1) as f64
    );
    println!("\n| column | words | sentences | differences (original → after) | examples |\n|---|---:|---:|---|---|");
    for &c in &cols {
        let c = c as usize;
        println!("| {} | {} | {} | {} | {} |", Col::ALL[c].name(), b.words[c], b.sents[c], b.top(c, 6), b.examples[c].join("; "));
    }
    let idform = b.words[Col::Id as usize] + b.words[Col::Form as usize] + f.words[Col::Id as usize] + f.words[Col::Form as usize];
    println!("\nID and FORM: {idform} differences; other lines (comments, multiword tokens, empty nodes): forward {}, backward {}", f.other, b.other);
    println!("\n## Per rule\n\n| rule | direction | words changed | of them as in original |\n|---|---|---:|---:|");
    for (r, n) in fwd.iter().zip(&by_fwd) {
        println!("| {} | {dialect} → mova | {n} | |", r.id);
    }
    for (r, [n, ok]) in back.iter().zip(&by_back) {
        println!("| {} | mova → {dialect} | {n} | {ok} ({:.1}%) |", r.id, 100.0 * *ok as f64 / (*n).max(1) as f64);
    }
    Ok(())
}

/// `en train`: full UD annotator (tagger, parser, UPOS/FEATS tables) → model file. Check files
/// after the model: the model is loaded back, and the annotation of the freshly trained and the loaded model
/// on them must match down to the token (otherwise an error).
fn train_model(train: &[PathBuf], rest: &[String]) -> Result<()> {
    use en::annotate::Annotator;
    let Some(out) = rest.first() else { bail!("train: a model file is required after --") };
    let out = Path::new(out);
    let t0 = std::time::Instant::now();
    let a = Annotator::train_files(train)?;
    let t_train = t0.elapsed().as_secs_f64();
    let t1 = std::time::Instant::now();
    a.save(out)?;
    let size = std::fs::metadata(out)?.len();
    println!("training {t_train:.1} s, writing {:.2} s → {} ({:.1} MB)", t1.elapsed().as_secs_f64(), out.display(), size as f64 / 1e6);
    if rest.len() < 2 {
        return Ok(());
    }
    let t2 = std::time::Instant::now();
    let b = Annotator::load(out)?;
    println!("loading {:.3} s", t2.elapsed().as_secs_f64());
    for p in paths(&rest[1..]) {
        let sents = conllu::read(&p)?;
        let (mut toks, mut diff) = (0usize, 0usize);
        for s in &sents {
            let forms: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
            let (x, y) = (a.annotate(&forms), b.annotate(&forms));
            toks += x.len();
            if x != y {
                diff += 1;
                if diff <= 3 {
                    eprintln!("DIFF {}: {:?}\n  vs {:?}", s.id, x, y);
                }
            }
        }
        println!("{}: sentences {}, tokens {toks}; trained and loaded annotation differ in {diff} sentences", name(&p), sents.len());
        if diff > 0 {
            bail!("model file {} gives different annotation than the freshly trained model", out.display());
        }
    }
    Ok(())
}

/// `en annotate`: full UD from a model file to CoNLL-U. `.conllu` runs on gold tokens; any other file is
/// text, one sentence per line (model tokenizer). Load and annotation time go to stderr.
fn annotate_files(model: &Path, files: &[PathBuf]) -> Result<()> {
    let t0 = std::time::Instant::now();
    let a = en::annotate::Annotator::load(model)?;
    eprintln!("model {}: loading {:.3} s", model.display(), t0.elapsed().as_secs_f64());
    let mut out = std::io::BufWriter::new(std::io::stdout().lock());
    let (mut ns, mut nt, mut busy) = (0usize, 0usize, 0.0f64);
    let mut emit = |out: &mut dyn Write, id: &str, text: &str, forms: &[&str], space: &[bool]| -> Result<()> {
        let t = std::time::Instant::now();
        let words = a.annotate(forms);
        busy += t.elapsed().as_secs_f64();
        ns += 1;
        nt += words.len();
        writeln!(out, "# sent_id = {id}\n# text = {text}")?;
        for r in en::annotate::rows(&words, space) {
            writeln!(out, "{}", r.join("\t"))?;
        }
        writeln!(out)?;
        Ok(())
    };
    for p in files {
        if p.extension().is_some_and(|x| x == "conllu") {
            for s in conllu::read(p)? {
                let forms: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
                let space: Vec<bool> = s.tokens.iter().map(|t| t.space_after).collect();
                emit(&mut out, &s.id, &s.text, &forms, &space)?;
            }
        } else {
            let stem = p.file_stem().unwrap_or_default().to_string_lossy().into_owned();
            for (k, line) in read_lines(p)?.iter().enumerate() {
                let toks = a.tokenize(line);
                let forms: Vec<&str> = toks.iter().map(|t| t.form.as_str()).collect();
                let space: Vec<bool> = toks.iter().map(|t| t.space_after).collect();
                emit(&mut out, &format!("{stem}-{}", k + 1), line, &forms, &space)?;
            }
        }
    }
    out.flush()?;
    eprintln!("sentences {ns}, tokens {nt}: annotation {busy:.2} s, {:.2} ms per sentence, {:.0} tok/s", 1e3 * busy / ns.max(1) as f64, nt as f64 / busy.max(1e-9));
    Ok(())
}

