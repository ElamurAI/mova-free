//! CLI of the latex crate:
//!   latex parse '<formula>' [--macros FILE.tex]   — tree, canonical LaTeX, text, round trip (and math::Expr)
//!   latex tex FILE.tex [OTHER.tex…] [-v]             — document formulas with its macros (from all files)
//!   latex corpus PATHS… [--out DIR] [--limit N] [--fault swapfrac|dropsup|droplast] [--math]
//!                                                   — run over real formulas (arXiv OAI .xml.gz, .tex)

mod corpus;

use std::path::PathBuf;
use std::process::ExitCode;

use latex::gate::{self, RoundTrip};
use latex::print::{Fault, to_sexpr};
use latex::{Macros, extract, parse_with, to_latex, to_text};

fn usage() -> ExitCode {
    eprintln!(
        "latex parse '<formula>' [--macros FILE.tex]\nlatex tex FILE.tex [OTHER.tex…] [-v]\nlatex corpus PATHS… [--out DIR] [--limit N] [--fault swapfrac|dropsup|droplast] [--math]"
    );
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first() else { return usage() };
    match cmd.as_str() {
        "parse" => cmd_parse(&args[1..]),
        "tex" => cmd_tex(&args[1..]),
        "corpus" => cmd_corpus(&args[1..]),
        _ => usage(),
    }
}

fn cmd_parse(args: &[String]) -> ExitCode {
    let mut src = None;
    let mut macro_file = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--macros" && i + 1 < args.len() {
            macro_file = Some(args[i + 1].clone());
            i += 2;
        } else {
            src = Some(args[i].clone());
            i += 1;
        }
    }
    let Some(src) = src else { return usage() };
    let tex = match &macro_file {
        Some(f) => match std::fs::read_to_string(f) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{f}: {e}");
                return ExitCode::FAILURE;
            }
        },
        None => String::new(),
    };
    let mut m = Macros::new();
    m.scan(&tex);
    println!("source:   {src}");
    let tree = match parse_with(&src, &m) {
        Ok(t) => t,
        Err(e) => {
            println!("error:    {e}");
            println!("location: {}", e.context(&src));
            return ExitCode::FAILURE;
        }
    };
    println!("tree:     {}", to_sexpr(&tree));
    println!("latex:    {}", to_latex(&tree));
    println!("text:     {}", to_text(&tree));
    for (name, span) in tree.unknowns() {
        println!("unknown:  {name} (position {})", span.start);
    }
    match gate::check(&tree) {
        RoundTrip::Ok => println!("round trip: yes"),
        RoundTrip::Mismatch { printed, reparsed } => println!("round trip: NO — {printed} → {}", to_sexpr(&reparsed)),
        RoundTrip::ReparseError { printed, error } => println!("round trip: NO — {printed}: {error}"),
    }
    bridge(&tree);
    ExitCode::SUCCESS
}

#[cfg(feature = "math")]
fn bridge(tree: &latex::Node) {
    match latex::tomath::to_expr(tree) {
        Ok(e) => {
            println!("math:     {e}");
            println!("math tree: {e:?}");
            if let Ok(back) = latex::tomath::from_expr(&e) {
                println!("back:     {}", to_latex(&back));
            }
        }
        Err(why) => println!("math:     — {why}"),
    }
}

#[cfg(not(feature = "math"))]
fn bridge(_: &latex::Node) {}

fn cmd_tex(args: &[String]) -> ExitCode {
    let verbose = args.iter().any(|a| a == "-v");
    let paths: Vec<&String> = args.iter().filter(|a| *a != "-v").collect();
    if paths.is_empty() {
        return usage();
    }
    let mut texts = Vec::new();
    for p in &paths {
        match std::fs::read_to_string(p) {
            Ok(t) => texts.push(t),
            Err(e) => {
                eprintln!("{p}: {e}");
                return ExitCode::FAILURE;
            }
        }
    }
    // macros come from all files of the document (the preamble is usually in the main one)
    let mut m = Macros::new();
    let mut defined = 0;
    let mut skipped = 0;
    for t in &texts {
        let ms = m.scan(t);
        defined += ms.defined;
        skipped += ms.skipped;
    }
    let (mut n, mut ok, mut clean, mut rt, mut unterminated) = (0, 0, 0, 0, 0);
    let mut unknown: std::collections::BTreeMap<String, usize> = Default::default();
    for (p, t) in paths.iter().zip(&texts) {
        let (formulas, bad) = extract(t);
        unterminated += bad.len();
        for f in &formulas {
            n += 1;
            match parse_with(f.body, &m) {
                Ok(tree) => {
                    ok += 1;
                    let u = tree.unknowns();
                    if u.is_empty() {
                        clean += 1;
                    }
                    let g = gate::check(&tree).is_ok();
                    if g {
                        rt += 1;
                    }
                    let names: Vec<String> = u.into_iter().map(|(n, _)| n).collect();
                    for name in &names {
                        *unknown.entry(name.clone()).or_default() += 1;
                    }
                    if verbose {
                        println!("{p}:{}	{}	{}	{}", f.start, if g { "yes" } else { "NO" }, names.join(","), to_latex(&tree));
                    }
                }
                Err(e) => {
                    if verbose {
                        println!("{p}:{}	error	{e}	{}", f.start, f.body.split_whitespace().collect::<Vec<_>>().join(" "));
                    }
                }
            }
        }
    }
    println!(
        "files: {}; macros: {defined} (broken {skipped}); formulas: {n}; parsed {ok}; without Unknown {clean}; round trip {rt}; unterminated {unterminated}",
        paths.len()
    );
    if !unknown.is_empty() {
        let list: Vec<String> = unknown.iter().map(|(k, v)| format!("{k} {v}")).collect();
        println!("unknown: {}", list.join(", "));
    }
    ExitCode::SUCCESS
}

fn cmd_corpus(args: &[String]) -> ExitCode {
    let mut o = corpus::Opts::default();
    let mut paths = Vec::new();
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        let next = args.get(i + 1);
        match (a, next) {
            ("--out", Some(v)) => {
                o.out = Some(PathBuf::from(v));
                i += 2;
            }
            ("--limit", Some(v)) => {
                o.limit = v.parse().ok();
                i += 2;
            }
            ("--fault", Some(v)) => {
                o.fault = match v.as_str() {
                    "swapfrac" => Some(Fault::SwapFrac),
                    "dropsup" => Some(Fault::DropSup),
                    "droplast" => Some(Fault::DropLast),
                    _ => return usage(),
                };
                i += 2;
            }
            ("--math", _) => {
                if !cfg!(feature = "math") {
                    eprintln!("--math: build with --features math");
                    return ExitCode::from(2);
                }
                o.bridge = true;
                i += 1;
            }
            _ => {
                paths.push(PathBuf::from(a));
                i += 1;
            }
        }
    }
    match corpus::run(&paths, &o) {
        Ok(report) => {
            print!("{report}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
