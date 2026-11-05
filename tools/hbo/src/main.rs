//! hbo — Ancient (Biblical) Hebrew from raw text to trees (operator, 18.10: «parsers of the different languages»).
//!
//!   hbo split <trees-dir> <out-dir> <pointed|unpointed> [--dev BOOK,…] [--test BOOK,BOOK.c1-c2,…]
//!
//! The trees of `macula` (the whole Hebrew Bible; the Aramaic chapters are apart in `arc/` and not read) into the
//! parser's data: `train.conllu`, `dev-<part>.conllu`, `test-<part>.conllu`. Books (or chapters, `GEN.19-30`) are held
//! out whole. FORM is normalized to the level (accents always dropped; points too when unpointed); XPOS is MACULA's
//! `Type` (the verb form: wayyiqtol, qatal …; the kind of a particle), or the UPOS where it has none.
//!
//!   hbo seg learn <pointed|unpointed> <seg.tsv> <train.conllu>… [--max-k K] [--min N] [--rare R]
//!   hbo seg eval <seg.tsv> <gold.conllu>… [--without lex,rest] [--with SWITCHES] [--fallback <seg.tsv>] [--thin X]
//!   hbo seg cv <pointed|unpointed> <train.conllu>… [--max-k K] [--min N] [--rare R] [--with SWITCHES]
//!
//!   hbo lattice <seg.tsv> <in.conllu> <readings.conllu> [--k 3] [--ratio 0.05] [--with …]
//!   hbo choose <seg.tsv> <in.conllu> <readings parsed with GP_SCORE=1> <out.conllu> [--lambda 1] [--mu 0] [--study <tsv>]
//!
//! Several segmentations for the parser to choose from (module `lattice`; measured, not used by default).
//!
//! The segmenter (module `seg`): written words into MACULA's words, by a lexicon and rules learned from the trees.
//!
//!   hbo resegment <seg.tsv> <gold.conllu> <out.conllu>
//!
//! The gold sentences and written words, cut into words by the segmenter and with no annotation: the input of an
//! end-to-end measure.
//!
//!   hbo segment <seg.tsv> <in.txt> <out.conllu>
//!   hbo parse <model-dir> <in.txt> <out.conllu>
//!
//! Raw Hebrew text (one or more sentences a line; a sentence ends at sof pasuq ׃, «.», «?», «!») into words, and with
//! `parse` into trees: segment → `gparse annotate` (the binary in `HBO_GPARSE`, else `gparse` on the PATH) →
//! CoNLL-U with the written words as multiword tokens, LEMMA and FEATS (gender, number, person, HebBinyan) from the
//! training trees where the form was seen, and `Seg=` in MISC: why the word was cut so. A model dir holds `seg.tsv`,
//! `gparse/` and `lex.conllu`; a dir with `pointed/` and `unpointed/` picks by the text.
//!
//!   hbo merge <base.conllu> <parsed.conllu> <out.conllu>
//!   hbo score <gold.conllu> <pred.conllu>
//!
//!   hbo pick <in.conllu> <out.conllu> <BOOK,BOOK.c1-c2,…>
//!
//! The sentences of some books or chapters (a layer of the language: archaic poetry, late prose …), to measure by layer.
//!
//!   hbo blank <in.conllu> <out.conllu>
//!
//! The gold words with LEMMA … DEPREL cleared (FORM, multiword tokens and MISC kept): the parser's input when it is
//! measured on gold words, so that no gold LEMMA or FEATS is carried through.
//!
//! `merge` puts a parser's LEMMA … DEPREL on the base file's words (keeping its multiword tokens and MISC); `score`
//! measures a prediction against gold whose written words are the same, words aligned within each written word
//! (words, UPOS, XPOS, FEATS, UAS, LAS as F1; equal to accuracy when the words are gold).

use hbo::{R, annotate, conllu, cut_with, lattice, load_model, load_seg, merge, model_dir, seg, segment, text};

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use conllu::{Sent, Word};
use seg::{Model, Opts};
use text::{Level, norm};

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let s: Vec<&str> = a.iter().map(String::as_str).collect();
    let r = match s.as_slice() {
        ["split", trees, out, level, rest @ ..] => split(trees, out, level, rest),
        ["seg", "learn", level, out, rest @ ..] => seg_learn(level, out, rest),
        ["seg", "eval", model, rest @ ..] => seg_eval(model, rest),
        ["seg", "cv", level, rest @ ..] => seg_cv(level, rest),
        ["resegment", model, gold, out, rest @ ..] => resegment(model, gold, out, rest),
        ["segment", model, inp, out] => segment_cmd(model, inp, out),
        ["lattice", model, inp, out, rest @ ..] => lattice_cmd(model, inp, out, rest),
        ["choose", model, inp, parsed, out, rest @ ..] => choose_cmd(model, inp, parsed, out, rest),
        ["parse", dir, inp, out] => parse(dir, inp, out),
        ["merge", base, parsed, out] => merge_cmd(base, parsed, out),
        ["score", gold, pred] => score_cmd(gold, pred),
        ["pick", inp, out, parts] => pick(inp, out, parts),
        ["blank", inp, out] => blank(inp, out),
        _ => Err("usage: hbo split <trees-dir> <out-dir> <pointed|unpointed> [--dev …] [--test …] | seg learn <level> <seg.tsv> <train.conllu>… | seg eval <seg.tsv> <gold.conllu>… | resegment <seg.tsv> <gold.conllu> <out> | segment <seg.tsv> <in.txt> <out> | parse <model-dir> <in.txt> <out> | merge <base> <parsed> <out> | score <gold> <pred>".into()),
    };
    if let Err(e) = r {
        eprintln!("hbo: {e}");
        std::process::exit(1);
    }
}

fn level_of(s: &str) -> R<Level> {
    Level::parse(s).ok_or_else(|| format!("level {s}: pointed or unpointed"))
}

/// `--name value` pairs and the plain arguments.
fn flags<'a>(rest: &[&'a str]) -> (BTreeMap<&'a str, &'a str>, Vec<String>) {
    let mut f = BTreeMap::new();
    let mut plain = Vec::new();
    let mut i = 0;
    while i < rest.len() {
        if let Some(k) = rest[i].strip_prefix("--") {
            f.insert(k, rest.get(i + 1).copied().unwrap_or(""));
            i += 2;
        } else {
            plain.push(rest[i].to_string());
            i += 1;
        }
    }
    (f, plain)
}

/// A held-out part: a book, or a book's chapters (`GEN.19-30`).
struct Part {
    name: String,
    book: String,
    chapters: Option<(u32, u32)>,
}

impl Part {
    fn parse(s: &str) -> Part {
        let (book, ch) = s.split_once('.').map(|(b, c)| (b, Some(c))).unwrap_or((s, None));
        let chapters = ch.and_then(|c| {
            let (a, b) = c.split_once('-').unwrap_or((c, c));
            Some((a.parse().ok()?, b.parse().ok()?))
        });
        Part { name: s.to_string(), book: book.to_string(), chapters }
    }
    fn holds(&self, book: &str, chapter: u32) -> bool {
        self.book == book && self.chapters.is_none_or(|(a, b)| a <= chapter && chapter <= b)
    }
}

/// wlc:GEN:12 with `# verse = GEN.19.1 …` → (GEN, 19)
fn place(s: &Sent) -> (String, u32) {
    let book = s.id().split(':').nth(1).unwrap_or("").to_string();
    let ch = s.comment("verse").and_then(|v| v.split_whitespace().next()).and_then(|v| v.split('.').nth(1)).and_then(|c| c.parse().ok()).unwrap_or(0);
    (book, ch)
}

/// FORM (and the written word, and `# text`) normalized; XPOS from MACULA's `Type`.
fn prepare(s: &mut Sent, level: Level) {
    for c in s.comments.iter_mut() {
        if let Some(t) = c.strip_prefix("# text = ") {
            *c = format!("# text = {}", norm(t, level));
        }
    }
    for t in s.tokens.iter_mut() {
        t.form = norm(&t.form, level);
        for w in t.words.iter_mut() {
            w.cols[1] = norm(&w.cols[1], level);
            w.cols[4] = w.misc("Type").map(String::from).unwrap_or_else(|| w.cols[3].clone());
        }
        if t.words.len() == 1 {
            t.form = t.words[0].cols[1].clone();
        }
    }
}

fn split(trees: &str, out: &str, level: &str, rest: &[&str]) -> R<()> {
    let level = level_of(level)?;
    let (f, _) = flags(rest);
    let dev: Vec<Part> = f.get("dev").copied().unwrap_or("RUT,JON").split(',').filter(|x| !x.is_empty()).map(Part::parse).collect();
    let test: Vec<Part> = f.get("test").copied().unwrap_or("DEU,GEN.19-30").split(',').filter(|x| !x.is_empty()).map(Part::parse).collect();
    let mut files: Vec<PathBuf> = fs::read_dir(trees).map_err(|e| format!("{trees}: {e}"))?.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "conllu")).collect();
    files.sort();
    let mut parts: BTreeMap<String, Vec<Sent>> = BTreeMap::new();
    for p in &files {
        for mut s in conllu::read(&fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?) {
            let (book, ch) = place(&s);
            prepare(&mut s, level);
            let name = if let Some(x) = test.iter().find(|x| x.holds(&book, ch)) {
                format!("test-{}", x.name)
            } else if let Some(x) = dev.iter().find(|x| x.holds(&book, ch)) {
                format!("dev-{}", x.name)
            } else {
                "train".to_string()
            };
            parts.entry(name).or_default().push(s);
        }
    }
    fs::create_dir_all(out).map_err(|e| format!("{out}: {e}"))?;
    for (name, ss) in &parts {
        fs::write(Path::new(out).join(format!("{name}.conllu")), conllu::write(ss)).map_err(|e| e.to_string())?;
        println!("{name}: {} sentences, {} words, {} written words", ss.len(), ss.iter().map(|s| s.n_words()).sum::<usize>(), ss.iter().map(|s| s.tokens.len()).sum::<usize>());
    }
    Ok(())
}

fn learn_opts(f: &BTreeMap<&str, &str>) -> Opts {
    let d = Opts::default();
    let mut opts = Opts {
        max_k: f.get("max-k").and_then(|v| v.parse().ok()).unwrap_or(d.max_k),
        min: f.get("min").and_then(|v| v.parse().ok()).unwrap_or(d.min),
        rare: f.get("rare").and_then(|v| v.parse().ok()).unwrap_or(d.rare),
        ..d
    };
    opts.set_with(f.get("with").copied().unwrap_or(""));
    opts
}

/// Two-fold check on the training trees: the books alternate between two halves (in the order of their names), each
/// half learned and measured on the other, the two scores summed.
fn seg_cv(level: &str, rest: &[&str]) -> R<()> {
    let level = level_of(level)?;
    let (f, files) = flags(rest);
    let opts = learn_opts(&f);
    let train = conllu::read_files(&files)?;
    let books: std::collections::BTreeSet<String> = train.iter().map(|s| place(s).0).collect();
    let half: BTreeMap<String, usize> = books.iter().enumerate().map(|(i, b)| (b.clone(), i % 2)).collect();
    let mut total = seg::Score::default();
    for k in 0..2 {
        let (a, b): (Vec<Sent>, Vec<Sent>) = train.iter().cloned().partition(|s| half[&place(s).0] != k);
        let (m, _) = Model::learn(&a, level, opts.clone());
        total.add(&seg::evaluate(&m, &b));
    }
    print!("cv {} books, with [{}]: {}", books.len(), opts.with(), total.report());
    Ok(())
}

fn seg_learn(level: &str, out: &str, rest: &[&str]) -> R<()> {
    let level = level_of(level)?;
    let (f, files) = flags(rest);
    let opts = learn_opts(&f);
    let train = conllu::read_files(&files)?;
    let (m, bad) = Model::learn(&train, level, opts);
    fs::write(out, m.save()).map_err(|e| format!("{out}: {e}"))?;
    println!("{}: {} written words in the lexicon, {} rests, {} prefix contexts, {} suffix contexts; {bad} tokens not spelled by their words, left out", out, m.lex.len(), m.rest.len(), m.pre.len(), m.suf.len());
    Ok(())
}

fn seg_eval(model: &str, rest: &[&str]) -> R<()> {
    let (f, files) = flags(rest);
    let mut m = load_model(model)?;
    if let Some(fb) = f.get("fallback") {
        m.fallback = Some(Box::new(load_model(fb)?));
    }
    for w in f.get("without").copied().unwrap_or("").split(',') {
        match w {
            "lex" => m.opts.lex = false,
            "rest" => m.opts.rest = false,
            _ => {}
        }
    }
    m.opts.set_with(f.get("with").copied().unwrap_or(""));
    for p in &files {
        let gold = conllu::read_files(std::slice::from_ref(p))?;
        print!("{p}: {}", seg::evaluate_thinned(&m, &gold, f.get("thin").and_then(|v| v.parse().ok())).report());
    }
    Ok(())
}

/// The segmenter with `--with` and `--fallback`, and the lattice options (`--k`, `--ratio`).
fn lattice_setup(model: &str, f: &BTreeMap<&str, &str>) -> R<(Model, lattice::Opts)> {
    let mut m = load_model(model)?;
    m.opts.set_with(f.get("with").copied().unwrap_or(""));
    if let Some(fb) = f.get("fallback") {
        m.fallback = Some(Box::new(load_model(fb)?));
    }
    let o = lattice::Opts { k: f.get("k").and_then(|v| v.parse().ok()).unwrap_or(3), ratio: f.get("ratio").and_then(|v| v.parse().ok()).unwrap_or(0.05) };
    Ok((m, o))
}

fn lattice_cmd(model: &str, inp: &str, out: &str, rest: &[&str]) -> R<()> {
    let (f, _) = flags(rest);
    let (m, o) = lattice_setup(model, &f)?;
    let ss = conllu::read_files(&[inp.to_string()])?;
    let v = lattice::variants(&m, &ss, &o, &cut_with);
    eprintln!("{} sentences → {} readings", ss.len(), v.len());
    fs::write(out, conllu::write(&v)).map_err(|e| format!("{out}: {e}"))
}

fn choose_cmd(model: &str, inp: &str, parsed: &str, out: &str, rest: &[&str]) -> R<()> {
    let (f, _) = flags(rest);
    let (m, o) = lattice_setup(model, &f)?;
    let lambda: f64 = f.get("lambda").and_then(|v| v.parse().ok()).unwrap_or(1.0);
    let mu: f64 = f.get("mu").and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let ss = conllu::read_files(&[inp.to_string()])?;
    let readings = conllu::read_files(&[parsed.to_string()])?;
    // --study <out.tsv>: every reading's gains, and whether the base and the candidate are the gold split (the input
    // being gold trees)
    if let Some(p) = f.get("study") {
        let level = m.level.unwrap_or(Level::Pointed);
        let mut tsv = String::from("sent\ttoken\tlabel\ttree_gain\tlog_share\twords_added\tbase_gold\tcand_gold\tmean_gain\n");
        for (si, ti, l, g, lr, dw, mg) in lattice::gains(&readings) {
            let t = &ss[si].tokens[ti];
            let gold = seg::gold(t, level).map(|g| seg::label(&g.all()));
            let base = seg::label(&m.split(&t.form).0);
            tsv += &format!("{si}\t{ti}\t{l}\t{g:.4}\t{lr:.4}\t{dw}\t{}\t{}\t{mg:.5}\n", (gold.as_deref() == Some(&base)) as u8, (gold.as_deref() == Some(&l)) as u8);
        }
        fs::write(p, tsv).map_err(|e| format!("{p}: {e}"))?;
    }
    let ch = lattice::choose(&readings, lambda, mu)?;
    eprintln!("{} written words leave the segmenter's best", ch.values().map(|x| x.len()).sum::<usize>());
    fs::write(out, conllu::write(&lattice::apply(&m, &ss, &ch, &o, &cut_with))).map_err(|e| format!("{out}: {e}"))
}

fn resegment(model: &str, gold: &str, out: &str, rest: &[&str]) -> R<()> {
    let mut m = load_model(model)?;
    let f = flags(rest).0;
    m.opts.set_with(f.get("with").copied().unwrap_or(""));
    if let Some(fb) = f.get("fallback") {
        m.fallback = Some(Box::new(load_model(fb)?));
    }
    let mut ss = conllu::read_files(&[gold.to_string()])?;
    hbo::resegment(&m, &mut ss);
    fs::write(out, conllu::write(&ss)).map_err(|e| format!("{out}: {e}"))
}

fn segment_cmd(model: &str, inp: &str, out: &str) -> R<()> {
    let m = load_model(model)?;
    let src = fs::read_to_string(inp).map_err(|e| format!("{inp}: {e}"))?;
    fs::write(out, conllu::write(&segment(&m, &src))).map_err(|e| format!("{out}: {e}"))
}

fn merge_cmd(base: &str, parsed: &str, out: &str) -> R<()> {
    let mut b = conllu::read_files(&[base.to_string()])?;
    let p = conllu::read_files(&[parsed.to_string()])?;
    merge(&mut b, &p)?;
    fs::write(out, conllu::write(&b)).map_err(|e| format!("{out}: {e}"))
}

fn parse(dir: &str, inp: &str, out: &str) -> R<()> {
    let src = fs::read_to_string(inp).map_err(|e| format!("{inp}: {e}"))?;
    let d = model_dir(Path::new(dir), &src);
    let m = load_seg(&d)?;
    let level = m.level.unwrap_or(Level::Pointed);
    let mut ss = segment(&m, &src);
    let gp = std::env::var("HBO_GPARSE").unwrap_or_else(|_| "gparse".into());
    annotate(&d, &m, &mut ss, &gp, out)?;
    fs::write(out, conllu::write(&ss)).map_err(|e| format!("{out}: {e}"))?;
    eprintln!("{} sentences, {} words ({} level) → {out}", ss.len(), ss.iter().map(|s| s.n_words()).sum::<usize>(), level.name());
    Ok(())
}

#[derive(Default)]
struct Tally {
    gold: usize,
    pred: usize,
    words: usize,
    upos: usize,
    xpos: usize,
    feats: usize,
    uas: usize,
    las: usize,
    sents_unaligned: usize,
}

/// Gold and predicted words aligned within each written word (the same pieces), sentence by sentence.
fn score(gold: &[Sent], pred: &[Sent]) -> R<Tally> {
    if gold.len() != pred.len() {
        return Err(format!("{} gold sentences, {} predicted", gold.len(), pred.len()));
    }
    let mut t = Tally::default();
    for (g, p) in gold.iter().zip(pred) {
        t.gold += g.n_words();
        t.pred += p.n_words();
        if g.tokens.len() != p.tokens.len() {
            t.sents_unaligned += 1;
            continue;
        }
        // pred word number → gold word number
        let mut map: BTreeMap<usize, usize> = BTreeMap::new();
        let (mut gi, mut pi) = (0, 0);
        for (gt, pt) in g.tokens.iter().zip(&p.tokens) {
            let same = gt.words.len() == pt.words.len() && gt.words.iter().zip(&pt.words).all(|(a, b)| norm(a.form(), Level::Unpointed) == norm(b.form(), Level::Unpointed));
            if same {
                for k in 0..gt.words.len() {
                    map.insert(pi + k + 1, gi + k + 1);
                }
            }
            gi += gt.words.len();
            pi += pt.words.len();
        }
        let gw: Vec<&Word> = g.words().collect();
        let pw: Vec<&Word> = p.words().collect();
        for (&pk, &gk) in &map {
            let (a, b) = (gw[gk - 1], pw[pk - 1]);
            t.words += 1;
            t.upos += (a.cols[3] == b.cols[3]) as usize;
            t.xpos += (a.cols[4] == b.cols[4]) as usize;
            t.feats += (a.cols[5] == b.cols[5]) as usize;
            let head_ok = if b.head() == 0 { a.head() == 0 } else { map.get(&b.head()) == Some(&a.head()) };
            t.uas += head_ok as usize;
            t.las += (head_ok && a.rel() == b.rel()) as usize;
        }
    }
    Ok(t)
}

fn blank(inp: &str, out: &str) -> R<()> {
    let mut ss = conllu::read_files(&[inp.to_string()])?;
    for w in ss.iter_mut().flat_map(|s| s.tokens.iter_mut()).flat_map(|t| t.words.iter_mut()) {
        for k in 2..=7 {
            w.cols[k] = "_".into();
        }
    }
    fs::write(out, conllu::write(&ss)).map_err(|e| format!("{out}: {e}"))
}

fn pick(inp: &str, out: &str, parts: &str) -> R<()> {
    let parts: Vec<Part> = parts.split(',').filter(|x| !x.is_empty()).map(Part::parse).collect();
    let ss: Vec<Sent> = conllu::read_files(&[inp.to_string()])?.into_iter().filter(|s| {
        let (b, c) = place(s);
        parts.iter().any(|p| p.holds(&b, c))
    }).collect();
    println!("{out}: {} sentences, {} words", ss.len(), ss.iter().map(|s| s.n_words()).sum::<usize>());
    fs::write(out, conllu::write(&ss)).map_err(|e| format!("{out}: {e}"))
}

fn score_cmd(gold: &str, pred: &str) -> R<()> {
    let g = conllu::read_files(&[gold.to_string()])?;
    let p = conllu::read_files(&[pred.to_string()])?;
    let t = score(&g, &p)?;
    let f1 = |c: usize| {
        let (pr, rc) = (c as f64 / t.pred.max(1) as f64, c as f64 / t.gold.max(1) as f64);
        100. * 2. * pr * rc / (pr + rc).max(1e-12)
    };
    println!(
        "{pred}: gold words {}, predicted {}, aligned {} | words F1 {:.2} | UPOS {:.2} | XPOS {:.2} | FEATS {:.2} | UAS {:.2} | LAS {:.2}{}",
        t.gold, t.pred, t.words, f1(t.words), f1(t.upos), f1(t.xpos), f1(t.feats), f1(t.uas), f1(t.las),
        if t.sents_unaligned > 0 { format!(" | {} sentences with other written words", t.sents_unaligned) } else { String::new() }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hbo::parser_view;

    const GOLD: &str = "# sent_id = a\n1-3\tבָּאָרֶץ\t_\t_\t_\t_\t_\t_\t_\t_\n1\tבָּ\tבְּ\tADP\tADP\t_\t3\tcase\t_\t_\n2\tה\tהַ\tDET\tdefinite_article\t_\t3\tdet\t_\t_\n3\tאָרֶץ\tאֶרֶץ\tNOUN\tNOUN\t_\t4\tobl\t_\t_\n4\tיָשַׁב\tישׁב\tVERB\tqatal\t_\t0\troot\t_\t_\n\n";

    #[test]
    fn score_aligns_within_written_words() {
        let g = conllu::read(GOLD);
        let t = score(&g, &g).unwrap();
        assert_eq!((t.gold, t.words, t.las), (4, 4, 4));
        // negative control: the article not seen (two words for בָּאָרֶץ) and one head wrong
        let wrong = "# sent_id = a\n1-2\tבָּאָרֶץ\t_\t_\t_\t_\t_\t_\t_\t_\n1\tבָּ\tבְּ\tADP\tADP\t_\t2\tcase\t_\t_\n2\tאָרֶץ\tאֶרֶץ\tNOUN\tNOUN\t_\t3\tobl\t_\t_\n3\tיָשַׁב\tישׁב\tVERB\tqatal\t_\t0\troot\t_\t_\n\n";
        let t = score(&g, &conllu::read(wrong)).unwrap();
        assert_eq!((t.gold, t.pred, t.words, t.uas), (4, 3, 1, 1));
    }

    #[test]
    fn merge_keeps_tokens_and_misc() {
        let mut b = conllu::read(GOLD);
        let mut p = parser_view(&b, Level::Pointed);
        p[0].tokens[1].words[0].cols[7] = "conj".into();
        merge(&mut b, &p).unwrap();
        assert_eq!(b[0].tokens[0].words.len(), 3);
        assert_eq!(b[0].tokens[1].words[0].rel(), "conj");
        // negative control: a parse with a word fewer is refused
        p[0].tokens.pop();
        assert!(merge(&mut b, &p).is_err());
    }

    #[test]
    fn held_out_parts() {
        let p = Part::parse("GEN.19-30");
        assert!(p.holds("GEN", 19) && p.holds("GEN", 30) && !p.holds("GEN", 31) && !p.holds("EXO", 20));
        assert!(Part::parse("DEU").holds("DEU", 1));
    }
}
#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn flags_parts_prepare_and_score_boundaries() {
        let (f, plain) = flags(&["x", "--k", "1", "--k", "2", "--dangling"]);
        assert_eq!(f["k"], "2");
        assert_eq!(f["dangling"], "");
        assert_eq!(plain, ["x"]);
        assert_eq!(flags(&["--a", "--b", "c"]).0["a"], "--b");
        assert!(Part::parse("GEN.bad").holds("GEN", 999));
        assert!(!Part::parse("GEN.3-1").holds("GEN", 2));
        assert!(level_of("bad").is_err());
        let mut s =
            conllu::read("# text = בָּא\n1\tבָּא\t_\tVERB\t_\t_\t0\troot\t_\tType=qatal")[0].clone();
        prepare(&mut s, Level::Unpointed);
        assert_eq!(s.tokens[0].form, "בא");
        assert_eq!(s.tokens[0].words[0].cols[4], "qatal");
        assert_eq!(s.comment("text"), Some("בא"));
        assert_eq!(score(&[], &[]).unwrap().words, 0);
        assert!(score(&[s.clone()], &[]).is_err());
        let mut bad = s.clone();
        bad.tokens.clear();
        let t = score(&[s], &[bad]).unwrap();
        assert_eq!((t.gold, t.pred, t.sents_unaligned, t.words), (1, 0, 1, 0));
    }
}
