//! roundtrip — the round-trip (autoencoder) diagnostic of Mova's layers: text → tokens → UD tree → frames → world
//! and back, to see where each layer loses information and what it would take to keep it.
//!
//!   roundtrip run --out <dir> [--model <en.bin>] [--exact] [--normal] [--threads N] [--drop <class>]
//!                 <name>=<kind>:<path>[@<stride>]...
//!       kinds: txt (a book: paragraphs), tsv (id<TAB>text lines), lessons (a school directory, every stride-th lesson),
//!              conllu (a treebank: its text, its trees as gold, every stride-th document)
//!       --exact: the lossless mode — every layer's residual, the volumes decoded back byte for byte (the gate)
//!       --normal: no residual — the text decoded from every start (tokens, tree, frames, world; gold tree), by class
//!       (neither: both). --drop <class>: a negative control, the tree decoder drops every word of the class.
//!       Writes report.md, examples.md, residual.tsv, normal.tsv to <dir>.
//!
//!   roundtrip probe [--model <en.bin>] "<text>"...    every layer of a text, both ways, with its residual
//!
//!   roundtrip compare <before dir> <after dir>          two runs side by side: residual and normal mode, what changed
//!   roundtrip mixed --out <dir> …                     the meta layer test: many languages and styles in one stream (`mixed`)
//!   roundtrip blocks --out <dir> …                    the block-level training gate: whole lessons and chapters (`blocks`)
//!   roundtrip site-learn --site-md <root> --out <dir> what the site's training pages teach: the markup dictionary, the
//!                                                     order generator adapted (`blocks::site_learn`)

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Context, Result, bail};
use en::annotate::Annotator;

use roundtrip::tally::{self, Tally};
use roundtrip::{class, compare, frames, mixed, model_path, pipeline, report, tree, vol, world};

fn class_by_name(s: &str) -> Option<class::Class> {
    class::CLASSES.iter().copied().find(|c| c.name() == s)
}

/// Volumes of a set spec `<kind>:<path>[@<stride>]`.
fn volumes(spec: &str) -> Result<Vec<vol::Volume>> {
    let (kind, rest) = spec.split_once(':').context("a set is <name>=<kind>:<path>[@stride]")?;
    let (path, stride) = match rest.rsplit_once('@') {
        Some((p, s)) => (p, s.parse::<usize>().context("stride")?),
        None => (rest, 1),
    };
    let p = Path::new(path);
    Ok(match kind {
        "txt" => vec![vol::read_txt(p)?],
        "tsv" => vec![vol::read_tsv(p)?],
        "lessons" => vol::read_lessons(p, stride)?,
        "conllu" => vol::read_conllu(p, stride)?,
        _ => bail!("unknown kind {kind}: txt, tsv, lessons, conllu"),
    })
}

fn run_set(a: &Annotator, vols: &[vol::Volume], o: &pipeline::Opts, threads: usize) -> Result<Tally> {
    let next = AtomicUsize::new(0);
    let out: Mutex<Vec<Option<Result<Tally>>>> = Mutex::new((0..vols.len()).map(|_| None).collect());
    std::thread::scope(|s| {
        for _ in 0..threads.max(1) {
            s.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::SeqCst);
                    if i >= vols.len() {
                        break;
                    }
                    let r = pipeline::run(a, &vols[i], o);
                    out.lock().unwrap()[i] = Some(r);
                }
            });
        }
    });
    let mut t = Tally::default();
    for r in out.into_inner().unwrap() {
        t.merge(&r.unwrap()?);
    }
    Ok(t)
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut model = model_path();
    if matches!(args.first().map(String::as_str), Some("mixed" | "blocks" | "site-learn")) {
        if let Some(i) = args.iter().position(|a| a == "--model") {
            model = PathBuf::from(args.get(i + 1).context("--model <file>")?);
        }
        if args[0] == "blocks" {
            return roundtrip::blocks::run::main(&args[1..], &model);
        }
        if args[0] == "site-learn" {
            return roundtrip::blocks::site_learn::main(&args[1..], &model);
        }
        return mixed::main(&args[1..], &model);
    }
    let (mut exact, mut normal, mut threads, mut out, mut drop) = (false, false, 8usize, None, None);
    let mut rest = Vec::new();
    let mut it = args.iter().skip(1);
    while let Some(x) = it.next() {
        match x.as_str() {
            "--model" => model = PathBuf::from(it.next().context("--model <file>")?),
            "--exact" => exact = true,
            "--normal" => normal = true,
            "--threads" => threads = it.next().context("--threads N")?.parse()?,
            "--out" => out = Some(PathBuf::from(it.next().context("--out <dir>")?)),
            "--drop" => drop = Some(class_by_name(it.next().context("--drop <class>")?).context("unknown class")?),
            _ => rest.push(x.clone()),
        }
    }
    if !exact && !normal {
        exact = true;
        normal = true;
    }
    let opts = pipeline::Opts { exact, normal, drop, ..Default::default() };
    match args.first().map(String::as_str) {
        Some("run") => {
            let out = out.context("--out <dir>")?;
            std::fs::create_dir_all(&out)?;
            let t0 = std::time::Instant::now();
            let a = Annotator::load(&model)?;
            eprintln!("model {} loaded in {:.1}s", model.display(), t0.elapsed().as_secs_f64());
            let mut sets: Vec<(String, Tally)> = Vec::new();
            for spec in &rest {
                let (name, spec) = spec.split_once('=').context("a set is <name>=<kind>:<path>[@stride]")?;
                let t1 = std::time::Instant::now();
                let vols = volumes(spec)?;
                let t = run_set(&a, &vols, &opts, threads)?;
                eprintln!("{name}: {} volumes, {} sentences, exact {}/{} in {:.1}s", t.volumes, t.sents, t.exact_ok, t.volumes, t1.elapsed().as_secs_f64());
                sets.push((name.to_string(), t));
            }
            let mut md = format!(
                "# Round trip of Mova's layers\n\nModel `{}`; modes: {}{}{}.\n\n",
                model.file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or_default(),
                if exact { "exact " } else { "" },
                if normal { "normal" } else { "" },
                drop.map(|c| format!("; negative control: the tree decoder drops «{}»", c.name())).unwrap_or_default()
            );
            for (n, t) in &sets {
                md += &report::set(n, t, exact, normal);
            }
            if exact {
                md += "## Residual classes ranked (mean over the sets)\n\n";
                md += &report::ranked(&sets);
            }
            std::fs::write(out.join("report.md"), &md)?;
            let ex: String = sets.iter().map(|(n, t)| report::examples(n, t)).collect();
            std::fs::write(out.join("examples.md"), format!("# Round-trip examples\n\n{ex}"))?;
            let (r, n) = report::tsv(&sets);
            std::fs::write(out.join("residual.tsv"), r)?;
            std::fs::write(out.join("normal.tsv"), n)?;
            println!("{md}");
            Ok(())
        }
        Some("compare") => {
            anyhow::ensure!(rest.len() == 2, "roundtrip compare <before dir> <after dir>");
            print!("{}", compare::compare(Path::new(&rest[0]), Path::new(&rest[1]))?);
            Ok(())
        }
        Some("probe") => {
            let a = Annotator::load(&model)?;
            for text in &rest {
                probe(&a, text)?;
            }
            Ok(())
        }
        _ => {
            eprintln!("usage: roundtrip run --out <dir> [--model <en.bin>] [--exact] [--normal] [--threads N] [--drop <class>] <name>=<kind>:<path>[@stride]... | probe [--model <en.bin>] \"<text>\"... | compare <before> <after>");
            Ok(())
        }
    }
}

/// Every layer of a text and every decode, sentence by sentence.
fn probe(a: &Annotator, text: &str) -> Result<()> {
    let v = vol::txt_volume("probe", text);
    let dec = tree::Dec { a };
    let mut ctx = prag::frames::Ctx::default();
    let mut real = frames::Realizer::default();
    let mut all: Vec<Vec<frames::F>> = Vec::new();
    let mut origs = Vec::new();
    for (b, blk) in v.blocks.iter().enumerate() {
        let seg = vol::segment(a, &blk.text, b, false, None)?;
        for s in &seg.sents {
            let w = a.annotate(&s.forms);
            let t = tree::of_words(&w);
            println!("text    {}", s.original());
            println!("tokens  {}", s.forms.join(" | "));
            let (_, f, sp) = dec.decode(&t);
            println!("tree    {}", vol::join(&f, &sp));
            let fs = frames::read(&w, &mut ctx);
            for f in &fs {
                println!("  frame {}", f.show());
            }
            let rt = real.sentence(&fs);
            let (_, f, sp) = dec.decode(&rt);
            println!("frames  {}", vol::join(&f, &sp));
            let mut res = tally::Residual::default();
            dec.encode(&t, &s.forms, &s.space(), &mut res);
            frames::encode(&t, &rt, &s.forms, &mut res);
            for ((l, c), cell) in &res.cells {
                println!("  residual {} {}: {} items, {:.0} bits: {}", l.name(), c.name(), cell.items, cell.bits, cell.examples.join(" · "));
            }
            all.push(fs);
            origs.push(s.original());
        }
    }
    let mut wres = tally::Residual::default();
    let (w, r4) = world::encode(&all, &mut wres);
    println!("world   {} entities, {} events", w.ents.len(), w.line.len());
    for (id, e) in &w.ents {
        let mut what = vec![if e.named { "named" } else { "common" }, if e.plural { "plural" } else { "singular" }];
        if let Some(p) = e.pro {
            what.push(p);
        }
        let attrs: Vec<&str> = e.attrs.iter().map(String::as_str).collect();
        println!("  entity {id}: {}{}", what.join(", "), if attrs.is_empty() { String::new() } else { format!("; {}", attrs.join(", ")) });
    }
    for (i, ev) in w.line.iter().enumerate() {
        let mut roles = Vec::new();
        for (name, r) in [("agent", &ev.agent), ("patient", &ev.patient), ("recipient", &ev.recipient), ("place", &ev.place)] {
            if let Some(r) = r {
                roles.push(format!("{name}={}", r.id));
            }
        }
        println!("  event {}: {:?} {}{} {}", i + 1, ev.kind, if ev.neg { "not " } else { "" }, ev.verb, roles.join(" "));
    }
    for ((l, c), cell) in &wres.cells {
        println!("  residual {} {}: {} items, {:.0} bits: {}", l.name(), c.name(), cell.items, cell.bits, cell.examples.join(" · "));
    }
    // the exact gate: the world with its residual gives every sentence's frames back
    let back = world::decode_exact(&w, &r4, all.len());
    println!("exact   world + residual → frames: {}", if back == all { "equal" } else { "DIFFERENT" });
    let mut rw = frames::Realizer::default();
    for (i, fs) in world::decode_in(&w, &all).iter().enumerate() {
        let (_, f, sp) = dec.decode(&rw.sentence(fs));
        println!("world → «{}» (text «{}»)", vol::join(&f, &sp), origs[i]);
    }
    Ok(())
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn spec_failure_order_and_classes() {
        assert_eq!(class_by_name("negation"), Some(class::Class::Neg));
        assert_eq!(class_by_name("NEGATION"), None);
        assert!(
            volumes("missing-colon")
                .unwrap_err()
                .to_string()
                .contains("a set is")
        );
        assert!(
            volumes("unknown:unused@bad")
                .unwrap_err()
                .to_string()
                .contains("stride")
        );
        assert!(
            volumes("unknown:unused@1")
                .unwrap_err()
                .to_string()
                .contains("unknown kind")
        );
    }
}
