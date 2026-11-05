//! langseg — the command line. See README.md.
//!
//!   langseg corpus                      build the pool: $MOVA_DATA/langseg/{train,dev,test}.tsv
//!   langseg train [--groups PCAMW]      learn the model: $MOVA_DATA/langseg/model.tsv
//!   langseg eval --split dev|test [--strip] [--shuffle] [--groups G]
//!                                       accuracy per variety, macro accuracy, confusion table
//!   langseg mix --split dev|test [--switch X | --sweep]
//!                                       mixed documents: word accuracy and boundary F1
//!   langseg seg [--pool] [--explain N] [--strip]
//!                                       segment stdin (one text, or `id<TAB>text` lines)
//!   langseg cues                        the manual cues with their learned log-odds
//!   langseg talmud <he.json> <en.json> <Tractate> <outdir> [--verses <pool.tsv.verses>]
//!                                       the Yerushalmi (Guggenheimer): original segments by language and the
//!                                       English by address; with the pool's verse map, the three-way index

use langseg::corpus::{self, Example};
use langseg::{explain, fnv, Counts, Featurizer, Groups, Lang, Model, Rng, Segmenter, HEB, K};
use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::path::PathBuf;

const TRAIN_CAP: usize = 20000;
const EVAL_CAP: usize = 2000;
const ALPHA: f64 = 0.5;
const MIN_COUNT: u32 = 2;
/// rounds of relabelling Hebrew segments of the Talmud corpora, chosen on the dev audit (2026-10-18)
const CLEAN_ROUNDS: usize = 2;
/// the switch cost in nats, chosen on dev with `langseg mix --split dev --sweep` (2026-10-18)
const SWITCH: f64 = 16.0;

/// The data directory: $MOVA_DATA, else ~/mova-data.
fn mova_data() -> PathBuf {
    std::env::var("MOVA_DATA").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap_or_default()).join("mova-data"))
}
fn data_dir() -> PathBuf {
    mova_data().join("langseg")
}
/// The raw folder of the research library: $RESEARCH_RAW, else $RESEARCH/raw, else $MOVA_DATA/research/raw.
fn raw_dir() -> PathBuf {
    std::env::var("RESEARCH_RAW")
        .map(PathBuf::from)
        .or_else(|_| std::env::var("RESEARCH").map(|r| PathBuf::from(r).join("raw")))
        .unwrap_or_else(|_| mova_data().join("research/raw"))
}

struct Args(Vec<String>);
impl Args {
    fn flag(&self, f: &str) -> bool {
        self.0.iter().any(|a| a == f)
    }
    fn val(&self, f: &str) -> Option<String> {
        self.0.iter().position(|a| a == f).and_then(|i| self.0.get(i + 1).cloned())
    }
}

fn main() {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let cmd = argv.first().cloned().unwrap_or_default();
    let a = Args(argv);
    let r = match cmd.as_str() {
        "corpus" => cmd_corpus(),
        "train" => cmd_train(&a),
        "eval" => cmd_eval(&a),
        "mix" => cmd_mix(&a),
        "seg" => cmd_seg(&a),
        "cues" => cmd_cues(),
        "audit" => cmd_audit(&a),
        "talmud" => cmd_talmud(&a),
        _ => Err("usage: langseg corpus | train | eval --split dev|test | mix --split dev|test | seg | cues".into()),
    };
    if let Err(e) = r {
        eprintln!("langseg: {e}");
        std::process::exit(1);
    }
}

fn cmd_corpus() -> Result<(), String> {
    let b = corpus::Builder::build(&raw_dir());
    let dir = data_dir();
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let mut by: BTreeMap<&str, Vec<&Example>> = BTreeMap::new();
    for e in &b.out {
        by.entry(corpus::split_of_example(e)).or_default().push(e);
    }
    let mut stats: BTreeMap<(Lang, &str), (usize, std::collections::BTreeSet<&str>)> = BTreeMap::new();
    for (split, ex) in &by {
        corpus::write_pool(&dir.join(format!("{split}.tsv")), ex).map_err(|e| e.to_string())?;
        for e in ex {
            let s = stats.entry((e.lang, split)).or_default();
            s.0 += 1;
            s.1.insert(e.doc.as_str());
        }
    }
    println!("lang\ttrain\tdev\ttest\t(segments; documents)");
    let langs: std::collections::BTreeSet<Lang> = stats.keys().map(|k| k.0).collect();
    for l in langs {
        let cell = |sp: &str| stats.get(&(l, sp)).map(|s| format!("{} ({})", s.0, s.1.len())).unwrap_or("0".into());
        println!("{}\t{}\t{}\t{}", l.code(), cell("train"), cell("dev"), cell("test"));
    }
    println!("pool written to {}", dir.display());
    Ok(())
}

/// The examples of one split, Hebrew-script varieties only, at most `cap` per variety (the first by id hash).
fn load(split: &str, cap: usize) -> Result<Vec<Example>, String> {
    let mut ex = corpus::read_pool(&data_dir().join(format!("{split}.tsv")))?;
    ex.retain(|e| e.lang.col().is_some());
    ex.sort_by_key(|e| (e.lang, fnv(&e.id), e.id.clone()));
    let mut n: BTreeMap<Lang, usize> = BTreeMap::new();
    ex.retain(|e| {
        let c = n.entry(e.lang).or_default();
        *c += 1;
        *c <= cap
    });
    Ok(ex)
}

/// Learn the model from the train split. With `clean`, one round of relabelling first: a gemara segment of the
/// Bavli or the Yerushalmi that a first model reads as Mishnaic Hebrew (a baraita, a quoted Mishnah) is relabelled
/// `hbo-x-mishnaic`, because the corpus labels are by tractate, not by sentence. `shuffle` permutes the labels
/// (the negative control) and turns the cleaning off.
fn train(groups: Groups, shuffle: bool, clean: bool) -> Result<Model, String> {
    let ex = load("train", TRAIN_CAP)?;
    let mut labels: Vec<usize> = ex.iter().map(|e| e.lang.col().unwrap()).collect();
    if shuffle {
        Rng(0x5eed_1234).shuffle(&mut labels);
    }
    let fz = Featurizer::new(groups.clone());
    let count = |labels: &[usize]| {
        let mut c = Counts::default();
        for (e, &l) in ex.iter().zip(labels) {
            c.add(&fz, &e.text, l);
        }
        c.model(groups.clone(), ALPHA, MIN_COUNT)
    };
    let mut m = count(&labels);
    if !clean || shuffle {
        return Ok(m);
    }
    let rounds: usize = std::env::var("LANGSEG_CLEAN_ROUNDS").ok().and_then(|x| x.parse().ok()).unwrap_or(CLEAN_ROUNDS);
    let mish = Lang::HboMishnaic.col().unwrap();
    for round in 1..=rounds {
        let sg = Segmenter::new(m, SWITCH);
        let mut moved: BTreeMap<Lang, usize> = BTreeMap::new();
        for (e, l) in ex.iter().zip(labels.iter_mut()) {
            if *l != mish && matches!(e.lang, Lang::Tmr | Lang::Jpa) && sg.classify(&e.text).0 == Lang::HboMishnaic {
                *l = mish;
                *moved.entry(e.lang).or_default() += 1;
            }
        }
        eprintln!("cleaning round {round}: relabelled as hbo-x-mishnaic {}", moved.iter().map(|(l, n)| format!("{} {n}", l.code())).collect::<Vec<_>>().join(", "));
        m = count(&labels);
    }
    Ok(m)
}

fn groups_of(a: &Args) -> Groups {
    a.val("--groups").map(Groups).unwrap_or_else(Groups::all)
}

fn cmd_train(a: &Args) -> Result<(), String> {
    let m = train(groups_of(a), false, !a.flag("--no-clean"))?;
    let p = data_dir().join("model.tsv");
    m.save(&p).map_err(|e| e.to_string())?;
    println!("{} features -> {}", m.logp.len(), p.display());
    Ok(())
}

/// The held-out test split is read once per release: every read is logged, and a second read is refused
/// unless `--again` says why.
fn guard_test(split: &str, a: &Args, what: &str) -> Result<(), String> {
    if split != "test" {
        return Ok(());
    }
    let log = data_dir().join("test-reads.log");
    let prev = std::fs::read_to_string(&log).unwrap_or_default();
    let line = format!("{what} {}", a.0[1..].join(" "));
    if prev.lines().any(|l| l.split_once('\t').map(|x| x.1) == Some(line.as_str())) && !a.flag("--again") {
        return Err(format!("the test split was already read with `{line}`; see {}", log.display()));
    }
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&log).map_err(|e| e.to_string())?;
    let ts = std::process::Command::new("date").arg("-Is").output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default();
    writeln!(f, "{ts}\t{line}").map_err(|e| e.to_string())
}

fn cmd_eval(a: &Args) -> Result<(), String> {
    let split = a.val("--split").unwrap_or("dev".into());
    guard_test(&split, a, "eval")?;
    let m = train(groups_of(a), a.flag("--shuffle"), !a.flag("--no-clean"))?;
    let mut sg = Segmenter::new(m, SWITCH);
    sg.strip = a.flag("--strip");
    let ex = load(&split, EVAL_CAP)?;
    let mut conf = [[0usize; K]; K];
    for e in &ex {
        let (p, _) = sg.classify(&e.text);
        conf[e.lang.col().unwrap()][p.col().unwrap()] += 1;
    }
    report(&conf, &format!("split={split} groups={} strip={} shuffle={} clean={}", groups_of(a).0, sg.strip, a.flag("--shuffle"), !a.flag("--no-clean")));
    if let Some(n) = a.val("--errors").and_then(|x| x.parse::<usize>().ok()) {
        let mut shown = 0;
        for e in &ex {
            let (p, _) = sg.classify(&e.text);
            if p != e.lang && shown < n {
                shown += 1;
                let seg = sg.segment(&e.text, 4);
                let why = seg.iter().filter_map(|s| s.why.as_ref()).map(explain).collect::<Vec<_>>().join(" | ");
                println!("ERR\t{}\tgold={}\tpred={}\t{}\t{}", e.id, e.lang.code(), p.code(), e.text, why);
            }
        }
    }
    Ok(())
}

fn report(conf: &[[usize; K]; K], title: &str) {
    println!("# {title}");
    println!("variety\tn\taccuracy");
    let mut macro_ = 0.0;
    let mut classes = 0;
    let (mut ok, mut all) = (0, 0);
    for (i, l) in HEB.iter().enumerate() {
        let n: usize = conf[i].iter().sum();
        if n == 0 {
            continue;
        }
        let acc = conf[i][i] as f64 / n as f64;
        macro_ += acc;
        classes += 1;
        ok += conf[i][i];
        all += n;
        println!("{}\t{n}\t{:.1}%", l.code(), 100.0 * acc);
    }
    println!("macro\t{all}\t{:.1}%", 100.0 * macro_ / classes as f64);
    println!("micro\t{all}\t{:.1}%", 100.0 * ok as f64 / all as f64);
    println!("confusion (rows gold, columns predicted):");
    let short = ["hbo", "mish", "he", "arc", "targ", "tmr", "jpa", "zohar"];
    println!("\t{}", short.join("\t"));
    for i in 0..K {
        let row: Vec<String> = conf[i].iter().map(|x| x.to_string()).collect();
        println!("{}\t{}", short[i], row.join("\t"));
    }
}

/// Mixed documents: 2–4 pieces of different languages (the Hebrew-script varieties and English) joined by a
/// space. Gold is per token; measured: token accuracy, and boundary precision / recall / F1 (exact token).
fn cmd_mix(a: &Args) -> Result<(), String> {
    let split = a.val("--split").unwrap_or("dev".into());
    guard_test(&split, a, "mix")?;
    let mut ex = load(&split, EVAL_CAP)?;
    // English pieces for the script step
    let mut en: Vec<Example> = corpus::read_pool(&data_dir().join(format!("{split}.tsv")))?.into_iter().filter(|e| e.lang == Lang::En).collect();
    en.sort_by_key(|e| fnv(&e.id));
    en.truncate(EVAL_CAP);
    ex.extend(en);
    let mut by: BTreeMap<Lang, Vec<&Example>> = BTreeMap::new();
    for e in &ex {
        by.entry(e.lang).or_default().push(e);
    }
    let langs: Vec<Lang> = by.keys().copied().collect();
    let mut rng = Rng(0xd0c5_2026);
    let mut docs: Vec<(String, Vec<(usize, usize, Lang)>)> = Vec::new();
    for _ in 0..1000 {
        let pieces = 2 + rng.below(3);
        let mut text = String::new();
        let mut gold = Vec::new();
        let mut last: Option<Lang> = None;
        for _ in 0..pieces {
            let mut l = langs[rng.below(langs.len())];
            while Some(l) == last {
                l = langs[rng.below(langs.len())];
            }
            last = Some(l);
            let pool = &by[&l];
            let e = pool[rng.below(pool.len())];
            if !text.is_empty() {
                text.push(' ');
            }
            let off = text.len();
            text.push_str(&e.text);
            for t in langseg::tokenize(&e.text) {
                // English pieces may hold transliterations; gold is the tokenizer's script for them
                gold.push((off + t.start, off + t.end, l));
            }
        }
        docs.push((text, gold));
    }
    let m = train(groups_of(a), a.flag("--shuffle"), !a.flag("--no-clean"))?;
    let switches: Vec<f64> = if a.flag("--sweep") {
        vec![2.0, 4.0, 6.0, 8.0, 12.0, 16.0, 24.0, 32.0]
    } else {
        vec![a.val("--switch").and_then(|x| x.parse().ok()).unwrap_or(SWITCH)]
    };
    let mut sg = Segmenter::new(m, SWITCH);
    sg.strip = a.flag("--strip");
    println!("# mixed documents: split={split} docs={} strip={}", docs.len(), sg.strip);
    println!("switch\ttoken_acc\tboundary_P\tboundary_R\tboundary_F1");
    for sw in switches {
        sg.switch = sw;
        let (mut ok, mut all, mut tp, mut np, mut ng) = (0usize, 0usize, 0usize, 0usize, 0usize);
        for (text, gold) in &docs {
            let segs = sg.segment(text, 0);
            let pred: Vec<Lang> = gold
                .iter()
                .map(|&(s, e, _)| segs.iter().find(|g| g.start <= s && e <= g.end).map(|g| g.lang).unwrap_or(Lang::Other))
                .collect();
            for (g, p) in gold.iter().zip(&pred) {
                all += 1;
                ok += (g.2 == *p) as usize;
            }
            for i in 1..gold.len() {
                let gb = gold[i].2 != gold[i - 1].2;
                let pb = pred[i] != pred[i - 1];
                ng += gb as usize;
                np += pb as usize;
                tp += (gb && pb) as usize;
            }
        }
        let p = tp as f64 / np.max(1) as f64;
        let r = tp as f64 / ng.max(1) as f64;
        let f = if p + r > 0.0 { 2.0 * p * r / (p + r) } else { 0.0 };
        println!("{sw}\t{:.1}%\t{:.1}%\t{:.1}%\t{:.1}%", 100.0 * ok as f64 / all as f64, 100.0 * p, 100.0 * r, 100.0 * f);
    }
    Ok(())
}

fn cmd_seg(a: &Args) -> Result<(), String> {
    let p = a.val("--model").map(PathBuf::from).unwrap_or_else(|| data_dir().join("model.tsv"));
    let m = Model::load(&p)?;
    let mut sg = Segmenter::new(m, a.val("--switch").and_then(|x| x.parse().ok()).unwrap_or(SWITCH));
    sg.strip = a.flag("--strip");
    let ex: usize = a.val("--explain").and_then(|x| x.parse().ok()).unwrap_or(0);
    let stdin = std::io::stdin();
    let mut out = std::io::BufWriter::new(std::io::stdout());
    let mut emit = |id: &str, text: &str| {
        for s in sg.segment(text, ex) {
            let t = text[s.start..s.end].replace(['\t', '\n', '\r'], " ");
            let why = s.why.as_ref().map(|w| format!("\t{}", explain(w))).unwrap_or_default();
            let _ = writeln!(out, "{id}\t{}\t{}\t{}\t{t}{why}", s.start, s.end, s.lang.code());
        }
    };
    if a.flag("--pool") {
        for l in stdin.lock().lines() {
            let l = l.map_err(|e| e.to_string())?;
            if let Some((id, t)) = l.split_once('\t') {
                emit(id, t);
            }
        }
    } else {
        let mut s = String::new();
        std::io::Read::read_to_string(&mut stdin.lock(), &mut s).map_err(|e| e.to_string())?;
        emit("-", &s);
    }
    Ok(())
}

/// The Talmud labels are by tractate. An audit is an independent gold for a sample of the Talmud segments, read
/// by hand: `id<TAB>A|H|M` (Aramaic as the corpus says, Hebrew, mixed). `--sample N` prints the sample to label
/// (the first N per Talmud variety in id-hash order); `--gold FILE` measures on it (H counts as hbo-x-mishnaic,
/// M is left out).
fn cmd_audit(a: &Args) -> Result<(), String> {
    let split = a.val("--split").unwrap_or("dev".into());
    let mut ex = load(&split, EVAL_CAP)?;
    ex.retain(|e| matches!(e.lang, Lang::Tmr | Lang::Jpa));
    if let Some(n) = a.val("--sample").and_then(|x| x.parse::<usize>().ok()) {
        guard_test(&split, a, "audit-sample")?;
        let mut per: BTreeMap<Lang, usize> = BTreeMap::new();
        for e in &ex {
            let c = per.entry(e.lang).or_default();
            *c += 1;
            if *c <= n {
                println!("{}\t{}\t{}", e.id, e.lang.code(), e.text);
            }
        }
        return Ok(());
    }
    let gold_path = a.val("--gold").ok_or("audit needs --sample N or --gold FILE")?;
    guard_test(&split, a, "audit")?;
    let gold: BTreeMap<String, char> = std::fs::read_to_string(&gold_path)
        .map_err(|e| e.to_string())?
        .lines()
        .filter_map(|l| l.split_once('\t').map(|(i, g)| (i.to_string(), g.trim().chars().next().unwrap_or('M'))))
        .collect();
    let m = train(groups_of(a), a.flag("--shuffle"), !a.flag("--no-clean"))?;
    let mut sg = Segmenter::new(m, SWITCH);
    sg.strip = a.flag("--strip");
    let mut conf = [[0usize; K]; K];
    let mut tally: BTreeMap<char, usize> = BTreeMap::new();
    for e in &ex {
        let Some(&g) = gold.get(&e.id) else { continue };
        *tally.entry(g).or_default() += 1;
        let gl = match g {
            'A' => e.lang,
            'H' => Lang::HboMishnaic,
            _ => continue,
        };
        let (p, _) = sg.classify(&e.text);
        conf[gl.col().unwrap()][p.col().unwrap()] += 1;
    }
    println!("audited labels: {}", tally.iter().map(|(g, n)| format!("{g} {n}")).collect::<Vec<_>>().join(", "));
    report(&conf, &format!("audit split={split} clean={} strip={} shuffle={}", !a.flag("--no-clean"), sg.strip, a.flag("--shuffle")));
    Ok(())
}

fn cmd_cues() -> Result<(), String> {
    let m = Model::load(&data_dir().join("model.tsv"))?;
    println!("feature\ttop\tlog-odds vs mean\tmanual expects");
    for (f, row) in &m.logp {
        if !(f.starts_with("C:") || f.starts_with("A:") || f.starts_with("P:")) {
            continue;
        }
        let mean = row.iter().map(|&x| x as f64).sum::<f64>() / K as f64;
        let top = (0..K).max_by(|&x, &y| row[x].total_cmp(&row[y]).then(y.cmp(&x))).unwrap();
        println!("{f}\t{}\t{:+.2}\t{}", HEB[top].code(), row[top] as f64 - mean, langseg::cues::gloss(f).unwrap_or(""));
    }
    Ok(())
}

/// The Jerusalem Talmud (Guggenheimer's edition, Sefaria JSON) for the Judaism corpus: `<outdir>/<tractate>.tsv`
/// (address, lang, text: the original cut into language segments), `<outdir>/<tractate>.en.tsv` (address, text: the
/// translation without the footnotes) and, with `--verses` (the English pool's map `id<TAB>address`),
/// `<outdir>/index-<tractate>.tsv` (address, part, the original's segment languages in order, the English sentence ids).
fn cmd_talmud(a: &Args) -> Result<(), String> {
    use langseg::talmud;
    let pos: Vec<&String> = {
        let mut v = Vec::new();
        let mut skip = false;
        for x in a.0.iter().skip(1) {
            if skip {
                skip = false;
            } else if x.starts_with("--") {
                skip = true;
            } else {
                v.push(x);
            }
        }
        v
    };
    let [he, en, tractate, outdir] = pos[..] else {
        return Err("usage: langseg talmud <he.json> <en.json> <Tractate> <outdir> [--verses <pool.tsv.verses>] [--model M]".into());
    };
    let m = Model::load(&a.val("--model").map(PathBuf::from).unwrap_or_else(|| data_dir().join("model.tsv")))?;
    let sg = Segmenter::new(m, SWITCH);
    let (he, en) = (talmud::read(std::path::Path::new(he))?, talmud::read(std::path::Path::new(en))?);
    let addr = |(c, h, g): (usize, usize, usize)| format!("{tractate}.{c}.{h}.{g}");
    let low = tractate.to_lowercase();
    let out = PathBuf::from(outdir);
    std::fs::create_dir_all(&out).map_err(|e| e.to_string())?;
    // the original: segments by language; the part carried from the last heading inside the halakhah
    let (mut orig, mut part_of, mut langs_of) = (String::new(), BTreeMap::new(), BTreeMap::<String, Vec<&'static str>>::new());
    let mut count: BTreeMap<&'static str, (usize, usize)> = BTreeMap::new();
    let mut part = talmud::Part::Mishnah;
    let mut last_hal = (0, 0);
    for (at, raw) in &he {
        if (at.0, at.1) != last_hal {
            last_hal = (at.0, at.1);
            part = talmud::Part::Mishnah;
        }
        let (p, text) = talmud::original(raw);
        if let Some(p) = p {
            part = p;
        }
        let ad = addr(*at);
        part_of.insert(ad.clone(), part);
        for (l, t) in talmud::segments(&sg, &text) {
            orig.push_str(&format!("{ad}\t{}\t{t}\n", l.code()));
            langs_of.entry(ad.clone()).or_default().push(l.code());
            let e = count.entry(l.code()).or_default();
            e.0 += 1;
            e.1 += t.split_whitespace().count();
        }
    }
    std::fs::write(out.join(format!("{low}.tsv")), orig).map_err(|e| e.to_string())?;
    let mut eng = String::new();
    let mut en_words = 0;
    for (at, raw) in &en {
        let (_, text) = talmud::english(raw);
        if !text.is_empty() {
            en_words += text.split_whitespace().count();
            eng.push_str(&format!("{}\t{text}\n", addr(*at)));
        }
    }
    std::fs::write(out.join(format!("{low}.en.tsv")), eng).map_err(|e| e.to_string())?;
    println!("{tractate}: {} addresses in the original, {} in the English ({en_words} words without the footnotes)", he.len(), en.len());
    for (l, (n, w)) in &count {
        println!("  {l}\t{n} segments\t{w} words");
    }
    if let Some(v) = a.val("--verses") {
        let map = std::fs::read_to_string(&v).map_err(|e| format!("{v}: {e}"))?;
        let mut ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for l in map.lines() {
            if let Some((id, ad)) = l.split_once('\t') {
                ids.entry(ad.to_string()).or_default().push(id.to_string());
            }
        }
        let mut idx = String::from("address\tpart\toriginal\ten\n");
        let mut all: Vec<(usize, usize, usize)> = he.iter().map(|x| x.0).chain(en.iter().map(|x| x.0)).collect();
        all.sort();
        all.dedup();
        let mut linked = 0;
        for at in all {
            let ad = addr(at);
            let e = ids.get(&ad).map(|v| v.join(",")).unwrap_or_default();
            linked += ids.get(&ad).map_or(0, Vec::len);
            let o = langs_of.get(&ad).map(|v| v.join(",")).unwrap_or_default();
            let p = part_of.get(&ad).map_or("-", |p| p.name());
            idx.push_str(&format!("{ad}\t{p}\t{o}\t{e}\n"));
        }
        let total: usize = ids.values().map(Vec::len).sum();
        std::fs::write(out.join(format!("index-{low}.tsv")), idx).map_err(|e| e.to_string())?;
        println!("  index: {linked} of {total} English sentences of this tractate's map linked to an address");
    }
    Ok(())
}

#[cfg(test)]
mod wave31 {
    use super::*;
    #[test]
    fn argument_first_match_and_missing_value() {
        let a = Args(
            vec!["seg", "--groups", "C", "--groups", "W", "--strip"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        );
        assert_eq!(groups_of(&a).0, "C");
        assert!(a.flag("--strip"));
        assert_eq!(a.val("--strip"), None);
        assert_eq!(a.val("--absent"), None);
        let b = Args(vec!["seg".into(), "--groups".into(), "--strip".into()]);
        assert_eq!(groups_of(&b).0, "--strip");
        assert_eq!(groups_of(&Args(vec![])).0, "PCAMW");
    }
}
