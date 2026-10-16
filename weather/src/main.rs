//! weather — the Mova weather domain on top of the `en` graph compressor: typed forecasts and queries,
//! two language variants — US (°F, mph; NWS) and international English (°C, km/h; Environment Canada).
//!
//!   weather fetch-nws <cities.csv> <out-dir> [pause ms]     snapshot of NWS forecasts (serde, no retries)
//!   weather fetch-eccc <out-dir> [pause ms]                 snapshot of ECCC citypage: the freshest file per station
//!   weather build <out-dir> <nws-raw-dir>... -- <eccc-raw-dir>...
//!                                                           periods (JSONL) and train/test sentences split by place
//!   weather eval <train.conllu>... -- <data-dir> [weight]  general model vs the US and international models
//!   weather nlg <train.conllu>... -- <data-dir>            "data → text" generator: two variants, evaluation on other cities
//!   weather say <train.conllu>... -- <data-dir> <us|intl> <periods.jsonl>
//!                                                           forecast texts from data (units converted to the variant)
//!   weather slots <train.conllu>... -- <massive.jsonl> <snips.tsv>
//!                                                           input query parser: slots (F1) and WeatherQuery

mod data;
mod eccc;
mod nlg;
mod nws;
mod query;

use std::collections::BTreeMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Result, bail};

use data::{Period, Variant, sentences};

use en::conllu;
use en::model::Model;
use en::text::chrf;

/// The weather parser is greedy (beam 1). The beam is more accurate on UD (EWT LAS 79.0 → 79.8), but on US
/// forecasts it gives 90.2% exact vs 97.8%: trees of this domain are uncertain without silver («as high as 20 mph»
/// attaches sometimes to gusts, sometimes to wind), and the order generator cannot reproduce them. Bring the beam back after
/// Opus silver on weather sentences.
const GREEDY: usize = 1;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let pause = |i: usize| Duration::from_millis(args.get(i).and_then(|x| x.parse().ok()).unwrap_or(400));
    match args.first().map(String::as_str) {
        Some("fetch-nws") if args.len() >= 3 => {
            let cities = nws::read_cities(Path::new(&args[1]))?;
            let (ok, fail) = nws::collect(&cities, Path::new(&args[2]), pause(3))?;
            println!("NWS: cities {}, fetched {ok}, failures {fail}", cities.len());
            Ok(())
        }
        Some("fetch-eccc") if args.len() >= 2 => {
            let (ok, fail) = eccc::collect(Path::new(&args[1]), pause(2))?;
            println!("ECCC: fetched {ok}, failures {fail}");
            Ok(())
        }
        Some("build") if args.len() >= 3 => {
            let sep = args.iter().position(|a| a == "--").unwrap_or(args.len());
            let us: Vec<PathBuf> = args[2..sep].iter().map(PathBuf::from).collect();
            let intl: Vec<PathBuf> = args.get(sep + 1..).unwrap_or(&[]).iter().map(PathBuf::from).collect();
            build(Path::new(&args[1]), &us, &intl)
        }
        Some("eval") if args.len() >= 3 => {
            let sep = args.iter().position(|a| a == "--").unwrap_or(args.len());
            let train: Vec<PathBuf> = args[1..sep].iter().map(PathBuf::from).collect();
            let rest = args.get(sep + 1..).unwrap_or(&[]);
            let Some(dir) = rest.first() else { bail!("eval: a data directory is required after --") };
            let weight = rest.get(1).and_then(|w| w.parse().ok()).unwrap_or(5);
            eval(&train, Path::new(dir), weight)
        }
        Some("nlg") if args.len() >= 3 => {
            let sep = args.iter().position(|a| a == "--").unwrap_or(args.len());
            let train: Vec<PathBuf> = args[1..sep].iter().map(PathBuf::from).collect();
            let Some(dir) = args.get(sep + 1) else { bail!("nlg: a data directory is required after --") };
            nlg_eval(&train, Path::new(dir))
        }
        Some("say") if args.len() >= 3 => {
            let sep = args.iter().position(|a| a == "--").unwrap_or(args.len());
            let train: Vec<PathBuf> = args[1..sep].iter().map(PathBuf::from).collect();
            let rest = args.get(sep + 1..).unwrap_or(&[]);
            if rest.len() < 3 {
                bail!("say: <data-dir> <us|intl> <periods.jsonl> are required after --");
            }
            let v = match rest[1].as_str() {
                "us" => Variant::Us,
                "intl" => Variant::Intl,
                x => bail!("variant {x}: expected us or intl"),
            };
            say(&train, Path::new(&rest[0]), v, Path::new(&rest[2]))
        }
        Some("slots") if args.len() >= 3 => {
            let sep = args.iter().position(|a| a == "--").unwrap_or(args.len());
            let train: Vec<PathBuf> = args[1..sep].iter().map(PathBuf::from).collect();
            let rest = args.get(sep + 1..).unwrap_or(&[]);
            if rest.len() < 2 {
                bail!("slots: <massive.jsonl> <snips.tsv> are required after --");
            }
            let mut ud = Vec::new();
            for p in &train {
                ud.extend(conllu::read(p)?.into_iter().filter(|s| s.tagged()));
            }
            let model = Model::train_with(ud, GREEDY);
            let (tr, te) = query::load(Path::new(&rest[0]), Path::new(&rest[1]))?;
            query::eval(&model, &tr, &te);
            Ok(())
        }
        _ => bail!("usage: weather fetch-nws <cities.csv> <out> [ms] | fetch-eccc <out> [ms] | build <out> <nws-dir>... -- <eccc-dir>... | eval <train.conllu>... -- <data-dir> [weight]"),
    }
}

/// Raw snapshots → `periods-<variant>.jsonl` and sentences `<variant>-train.txt` / `-test.txt`
/// (every fifth place alphabetically goes to test: the model never sees texts of test cities).
fn build(out: &Path, us_dirs: &[PathBuf], intl_dirs: &[PathBuf]) -> Result<()> {
    std::fs::create_dir_all(out)?;
    for (variant, dirs) in [(Variant::Us, us_dirs), (Variant::Intl, intl_dirs)] {
        let mut periods: Vec<Period> = Vec::new();
        let mut places: BTreeMap<String, String> = BTreeMap::new();
        let (mut files, mut bad) = (0, 0);
        for d in dirs {
            let mut entries: Vec<PathBuf> = std::fs::read_dir(d)?.filter_map(|e| e.ok().map(|e| e.path())).collect();
            entries.sort();
            for p in entries {
                let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
                let site = p.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
                let body = match ext {
                    "json" | "xml" => std::fs::read_to_string(&p)?,
                    _ => continue,
                };
                files += 1;
                let parsed = match variant {
                    Variant::Us => nws::periods(&site, &body).map(|v| (site.clone(), v)),
                    Variant::Intl => eccc::periods(&site, &body),
                };
                match parsed {
                    Ok((place, v)) => {
                        places.insert(site.clone(), place);
                        periods.extend(v);
                    }
                    Err(e) => {
                        bad += 1;
                        eprintln!("  {}: {e}", p.display());
                    }
                }
            }
        }
        let sites: Vec<&String> = places.keys().collect();
        let test_sites: std::collections::BTreeSet<&str> = sites.iter().enumerate().filter(|(i, _)| i % 5 == 4).map(|(_, s)| s.as_str()).collect();
        let v = variant.name();
        let mut pj = std::io::BufWriter::new(std::fs::File::create(out.join(format!("periods-{v}.jsonl")))?);
        let mut tr = std::io::BufWriter::new(std::fs::File::create(out.join(format!("{v}-train.txt")))?);
        let mut te = std::io::BufWriter::new(std::fs::File::create(out.join(format!("{v}-test.txt")))?);
        let (mut n_tr, mut n_te) = (0, 0);
        for p in &periods {
            writeln!(pj, "{}", serde_json::to_string(p)?)?;
            let test = test_sites.contains(p.site.as_str());
            for s in sentences(&p.text) {
                if test {
                    writeln!(te, "{s}")?;
                    n_te += 1;
                } else {
                    writeln!(tr, "{s}")?;
                    n_tr += 1;
                }
            }
        }
        println!(
            "{v}: files {files} (parse failures {bad}), places {}, periods {}, sentences train {n_tr} / test {n_te} (test places {})",
            places.len(),
            periods.len(),
            test_sites.len()
        );
    }
    Ok(())
}

fn read_lines(p: &Path) -> Result<Vec<String>> {
    Ok(std::fs::read_to_string(p)?.lines().map(str::trim).filter(|l| !l.is_empty()).map(str::to_string).collect())
}

/// (exact %, chrF) of the generative mode on lines.
fn score(model: &Model, test: &[String]) -> (f64, f64) {
    let (mut ok, mut c) = (0usize, 0.0f64);
    for t in test {
        let out = model.decode(&model.encode(t), false);
        if out == *t {
            ok += 1;
        }
        c += chrf(&out, t);
    }
    let n = test.len().max(1) as f64;
    (100.0 * ok as f64 / n, 100.0 * c / n)
}

/// General model vs the two variant models: each fine-tuned on queries + forecasts of its own variant.
fn eval(train: &[PathBuf], dir: &Path, weight: usize) -> Result<()> {
    let mut ud = Vec::new();
    for p in train {
        ud.extend(conllu::read(p)?.into_iter().filter(|s| s.tagged()));
    }
    let mut model = Model::train_with(ud, GREEDY);
    let q_train = read_lines(&dir.join("q_train.txt"))?;
    let q_test = read_lines(&dir.join("q_test.txt"))?;
    let tests: Vec<(&str, Vec<String>)> = vec![("queries", q_test), ("US forecasts", read_lines(&dir.join("us-test.txt"))?), ("intl. forecasts", read_lines(&dir.join("intl-test.txt"))?)];
    println!("test: {}", tests.iter().map(|(n, t)| format!("{n} {}", t.len())).collect::<Vec<_>>().join(", "));
    let row = |name: &str, model: &Model| {
        let cells: Vec<String> = tests.iter().map(|(_, t)| score(model, t)).map(|(e, c)| format!("{e:5.1}% / {c:4.1}")).collect();
        println!("| {name:<28} | {} |", cells.join(" | "));
    };
    println!("| model (exact / chrF)         | {} |", tests.iter().map(|(n, _)| format!("{n:<15}")).collect::<Vec<_>>().join(" | "));
    row("general (UD)", &model);
    for v in [Variant::Us, Variant::Intl] {
        let mut dom = q_train.clone();
        dom.extend(read_lines(&dir.join(format!("{}-train.txt", v.name())))?);
        model.adapt(&dom, weight);
        let name = match v {
            Variant::Us => "US (°F, mph)",
            Variant::Intl => "international (°C, km/h)",
        };
        row(&format!("{name}: {} sentences", dom.len()), &model);
    }
    Ok(())
}

/// Periods of a variant: (train, test) — test places are the same as in `build` (every fifth alphabetically).
fn split_periods(all: &[Period]) -> (Vec<&Period>, Vec<&Period>) {
    let sites: std::collections::BTreeSet<&str> = all.iter().map(|p| p.site.as_str()).collect();
    let test: std::collections::BTreeSet<&str> = sites.iter().enumerate().filter(|(i, _)| i % 5 == 4).map(|(_, s)| *s).collect();
    all.iter().partition(|p| !test.contains(p.site.as_str()))
}

/// "Data → text" generator: training on training places, evaluation on other places.
fn nlg_eval(train: &[PathBuf], dir: &Path) -> Result<()> {
    let mut ud = Vec::new();
    for p in train {
        ud.extend(conllu::read(p)?);
    }
    let load = |v: Variant| -> Result<Vec<Period>> {
        let text = std::fs::read_to_string(dir.join(format!("periods-{}.jsonl", v.name())))?;
        Ok(text.lines().filter(|l| !l.trim().is_empty()).map(serde_json::from_str).collect::<Result<_, _>>()?)
    };
    let mut models = Vec::new();
    let mut tests = Vec::new();
    for v in [Variant::Us, Variant::Intl] {
        let all = load(v)?;
        let (tr, te) = split_periods(&all);
        let t0 = std::time::Instant::now();
        let mut m = nlg::Nlg::train(v, en::tok::Lexicon::learn(&ud), &tr);
        println!("\n== {} — training: periods {} ({:.1} s), test: {} periods from other places", v.name(), tr.len(), t0.elapsed().as_secs_f64(), te.len());
        let r = nlg::evaluate(&m, &te, 4);
        let pr = |name: &str, r: &nlg::Report| {
            let comps: Vec<String> = r.comp_hit.iter().map(|(c, (h, n))| format!("{c:?} {:.0}% ({n})", 100.0 * *h as f64 / (*n).max(1) as f64)).collect();
            println!(
                "  {name}: exact text {:.1}%, chrF {:.1}; sentences by component: {}",
                100.0 * r.exact as f64 / r.n.max(1) as f64,
                100.0 * r.chrf / r.n.max(1) as f64,
                comps.join(", ")
            );
        };
        pr("generator", &r);
        println!(
            "  coverage: {} of {} reference sentences ({:.1}%) fit the data; the rest: {}",
            r.ref_ok,
            r.ref_sent,
            100.0 * r.ref_ok as f64 / r.ref_sent.max(1) as f64,
            r.why.iter().map(|(w, n)| format!("{w} {n}")).collect::<Vec<_>>().join(", ")
        );
        println!(
            "  fact gate: generator sentences that do not fit their own data: {} of {}; negative control (other data): {} of {} ({:.0}%)",
            r.gen_bad,
            r.gen_sent,
            r.neg_bad,
            r.neg_sent,
            100.0 * r.neg_bad as f64 / r.neg_sent.max(1) as f64
        );
        m.baseline = true;
        let b = nlg::evaluate(&m, &te, 0);
        pr("baseline (most frequent template; key only by sky or phenomenon)", &b);
        m.baseline = false;
        models.push(m);
        tests.push(te.into_iter().take(3).cloned().collect::<Vec<Period>>());
    }
    // same data — two styles
    println!("\n== same data — two styles (units converted)");
    for (i, te) in tests.iter().enumerate() {
        for p in te.iter().take(2) {
            let own = &models[i];
            let other = &models[1 - i];
            println!("  {} «{}»:\n    {:<5}: {}\n    {:<5}: {}", p.site, p.name, own.variant.name(), own.generate(p), other.variant.name(), other.generate(&nlg::convert(p, other.variant)));
        }
    }
    Ok(())
}

/// Forecast texts for periods from a file: the variant model is trained on all data of the variant.
fn say(train: &[PathBuf], dir: &Path, v: Variant, file: &Path) -> Result<()> {
    let mut ud = Vec::new();
    for p in train {
        ud.extend(conllu::read(p)?);
    }
    let text = std::fs::read_to_string(dir.join(format!("periods-{}.jsonl", v.name())))?;
    let all: Vec<Period> = text.lines().filter(|l| !l.trim().is_empty()).map(serde_json::from_str).collect::<Result<_, _>>()?;
    let refs: Vec<&Period> = all.iter().collect();
    let m = nlg::Nlg::train(v, en::tok::Lexicon::learn(&ud), &refs);
    for line in std::fs::read_to_string(file)?.lines().filter(|l| !l.trim().is_empty()) {
        let p: Period = serde_json::from_str(line)?;
        let p = if p.variant == v { p } else { nlg::convert(&p, v) };
        println!("{}\t{}", p.name, m.generate(&p));
    }
    Ok(())
}
