//! CLI: `mlab run file.m`, `mlab -e "code"`, `mlab test data/suite-v1.txt [--fault NAME] [-v]`.

use mlab::{Interp, suite};
use std::collections::BTreeMap;
use std::process::ExitCode;

fn usage() -> ExitCode {
    eprintln!("usage:\n  mlab run file.m [--fault NAME] [--vm] [FS]\n  mlab -e \"code\" [--fault NAME] [--vm] [FS]\n  mlab test data/suite-v1.txt [--fault NAME] [-v] [--vm] [--fs …]\n  FS: --fs mem|dir:<dir>|real (default mem) [--fs-import DIR[:VIRT]]… [--fs-export DIR[:VIRT]]… [--fs-limits total=256M,file=64M,files=65536] [--fs-journal FILE.tsv]\n  mlab vmgate data/suite-v1.txt data/suite-v2.txt [--fault vm:add]\n  mlab vmdump file.m\n  mlab builtins\n  mlab mcgate [-k 20000] [--fault p:ttest]");
    ExitCode::from(2)
}

/// File system (vfs, 26.09): `--fs mem` (default: foreign and generated code does not see the real FS),
/// `--fs dir:<dir>` (a real directory, the root is the boundary), `--fs real` (only explicitly, backward compatibility).
struct FsOpts {
    spec: String,
    limits: vfs::Limits,
    /// (real directory, virtual directory)
    imports: Vec<(String, String)>,
    exports: Vec<(String, String)>,
    journal: Option<String>,
}

fn flag_values<'a>(args: &'a [String], name: &str) -> Vec<&'a str> {
    args.windows(2).filter(|w| w[0] == name).map(|w| w[1].as_str()).collect()
}

fn fs_opts(args: &[String]) -> Result<FsOpts, String> {
    let spec = flag_values(args, "--fs").last().map_or("mem", |s| s).to_string();
    let limits = match flag_values(args, "--fs-limits").last() {
        Some(s) => vfs::Limits::parse(s).map_err(|e| format!("--fs-limits: {e}"))?,
        None => vfs::Limits::default(),
    };
    let pair = |s: &str| match s.split_once(':') {
        Some((real, virt)) => (real.to_string(), virt.to_string()),
        None => (s.to_string(), "/".to_string()),
    };
    let imports: Vec<_> = flag_values(args, "--fs-import").into_iter().map(pair).collect();
    let exports: Vec<_> = flag_values(args, "--fs-export").into_iter().map(pair).collect();
    if spec != "mem" && !(imports.is_empty() && exports.is_empty()) {
        return Err("--fs-import / --fs-export — only with --fs mem".into());
    }
    let journal = flag_values(args, "--fs-journal").last().map(|s| s.to_string());
    Ok(FsOpts { spec, limits, imports, exports, journal })
}

fn make_fs(o: &FsOpts) -> Result<Box<dyn vfs::FileSystem>, String> {
    let mut fs = vfs::open(&o.spec, o.limits)?;
    if let Some(m) = fs.as_mem() {
        for (real, virt) in &o.imports {
            m.import_dir(std::path::Path::new(real), virt).map_err(|e| format!("--fs-import {real}: {e}"))?;
        }
    }
    Ok(fs)
}

/// After the run: export (only if the script finished without an error — only approved results go out) and the journal.
fn finish_fs(it: &mut Interp, o: &FsOpts, ok: bool) -> Result<(), String> {
    if let Some(m) = it.fs.as_mem() {
        for (real, virt) in &o.exports {
            if !ok {
                eprintln!("--fs-export {real}: skipped — script exited with an error");
                continue;
            }
            m.export_dir(virt, std::path::Path::new(real)).map_err(|e| format!("--fs-export {real}: {e}"))?;
        }
    }
    if let Some(path) = &o.journal {
        let text = it.fs.journal().map(|j| j.tsv()).unwrap_or_default();
        std::fs::write(path, text).map_err(|e| format!("--fs-journal {path}: {e}"))?;
    }
    Ok(())
}

/// Running code with the file system selected by flags.
fn run_code(code: &str, args: &[String]) -> ExitCode {
    let o = match fs_opts(args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    let fs = match make_fs(&o) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    let mut it = Interp::console().with_fs(fs);
    it.fault = fault_arg(args);
    it.vm_mode = args.iter().any(|a| a == "--vm");
    let ok = it.run(code);
    if let Err(e) = finish_fs(&mut it, &o, ok) {
        eprintln!("error: {e}");
        return ExitCode::from(2);
    }
    if ok { ExitCode::SUCCESS } else { ExitCode::from(1) }
}

/// `--fault NAME` — negative control: a broken builtin (`sum`) or a wrong answer before the gate (`gate:mldivide`).
fn fault_arg(args: &[String]) -> Option<String> {
    args.iter().position(|a| a == "--fault").and_then(|k| args.get(k + 1)).cloned()
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("run") => {
            let Some(path) = args.get(1) else { return usage() };
            let src = match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: {path}: {e}");
                    return ExitCode::from(2);
                }
            };
            run_code(&src, &args)
        }
        Some("-e") => {
            let Some(code) = args.get(1) else { return usage() };
            run_code(code, &args)
        }
        Some("builtins") => {
            let r = mlab::builtins::Registry::with_defaults();
            let names = r.names();
            println!("{} builtins:", names.len());
            println!("{}", names.join(" "));
            ExitCode::SUCCESS
        }
        Some("test") => {
            let Some(path) = args.get(1) else { return usage() };
            let text = match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: {path}: {e}");
                    return ExitCode::from(2);
                }
            };
            let fault = args.iter().position(|a| a == "--fault").and_then(|k| args.get(k + 1)).map(|s| s.as_str());
            let verbose = args.iter().any(|a| a == "-v");
            let sc = suite::parse(&text);
            let o = match fs_opts(&args) {
                Ok(o) if o.imports.is_empty() && o.exports.is_empty() => o,
                Ok(_) => {
                    eprintln!("error: mlab test: --fs-import / --fs-export are not supported");
                    return ExitCode::from(2);
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    return ExitCode::from(2);
                }
            };
            if let Err(e) = vfs::open(&o.spec, o.limits) {
                eprintln!("error: {e}");
                return ExitCode::from(2);
            }
            let mk = || vfs::open(&o.spec, o.limits).expect("--fs");
            let t0 = std::time::Instant::now();
            let res = suite::run_fs(&sc, fault, args.iter().any(|a| a == "--vm"), &mk);
            let dt = t0.elapsed().as_secs_f64();
            let mut by_cat: BTreeMap<String, (usize, usize)> = BTreeMap::new();
            for r in &res {
                let e = by_cat.entry(r.category.clone()).or_default();
                e.1 += 1;
                if r.pass {
                    e.0 += 1;
                }
            }
            for r in res.iter().filter(|r| !r.pass) {
                if let Some((line, exp, got)) = &r.first_diff {
                    println!("✗ {} — line {line}: expected «{exp}», got «{got}»", r.id);
                }
                if verbose {
                    println!("  full output:\n{}", r.got.lines().map(|l| format!("    |{l}")).collect::<Vec<_>>().join("\n"));
                }
            }
            println!("category\tpassed\ttotal");
            for (c, (p, n)) in &by_cat {
                println!("{c}\t{p}\t{n}");
            }
            let pass = res.iter().filter(|r| r.pass).count();
            println!("TOTAL\t{pass}\t{}\t({dt:.2} s)", res.len());
            if pass == res.len() { ExitCode::SUCCESS } else { ExitCode::from(1) }
        }
        Some("vmgate") => {
            // VM equivalence gate: tree vs VM, byte for byte, fail-fast
            let fault = fault_arg(&args);
            let files: Vec<&String> = args[1..].iter().filter(|a| a.ends_with(".txt")).collect();
            if files.is_empty() {
                return usage();
            }
            let mut red = false;
            println!("suite\tscenarios\tsame output\tfully in VM\tinstructions in VM\tfallback (static)\tfallback (times)\tDynEval (times)");
            for path in files {
                let text = match std::fs::read_to_string(path) {
                    Ok(s) => s,
                    Err(e) => {
                        eprintln!("error: {path}: {e}");
                        return ExitCode::from(2);
                    }
                };
                let sc = suite::parse(&text);
                let r = suite::vm_gate(&sc, fault.as_deref());
                println!(
                    "{path}\t{}/{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    r.total, sc.len(), r.same, r.fully_native, r.native, r.fallback, r.fallback_runs, r.dyn_runs
                );
                if let Some((id, line, t, v)) = &r.mismatch {
                    println!("RED: {id}, line {line}: tree «{t}», VM «{v}»");
                    red = true;
                    break;
                }
            }
            if red { ExitCode::from(1) } else { ExitCode::SUCCESS }
        }
        Some("vmbench") => {
            // tree vs VM: each program 3 times in each mode, best time; the output must match
            let dir = args.get(1).map(|s| s.as_str()).unwrap_or("examples/vmbench");
            let mut files: Vec<std::path::PathBuf> = match std::fs::read_dir(dir) {
                Ok(rd) => rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.extension().is_some_and(|x| x == "m")).collect(),
                Err(e) => {
                    eprintln!("error: {dir}: {e}");
                    return ExitCode::from(2);
                }
            };
            files.sort();
            let mut red = false;
            println!("task\ttree, s\tVM, s\tspeedup\toutput");
            for f in files {
                let src = std::fs::read_to_string(&f).unwrap_or_default();
                let mut best = [f64::INFINITY; 2];
                let mut outs = [String::new(), String::new()];
                for _ in 0..3 {
                    for (mi, vm) in [false, true].into_iter().enumerate() {
                        let mut it = Interp::capture();
                        it.vm_mode = vm;
                        let t0 = std::time::Instant::now();
                        it.run(&src);
                        best[mi] = best[mi].min(t0.elapsed().as_secs_f64());
                        outs[mi] = it.take_output();
                    }
                }
                let same = outs[0] == outs[1];
                red |= !same;
                let name = f.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                println!(
                    "{name}\t{:.3}\t{:.3}\t{:.1}×\t{}",
                    best[0],
                    best[1],
                    best[0] / best[1],
                    if same { format!("same: {}", outs[0].trim_end().replace('\n', " | ")) } else { "DIFFERENT".into() }
                );
            }
            if red { ExitCode::from(1) } else { ExitCode::SUCCESS }
        }
        Some("vmdump") => {
            let Some(path) = args.get(1) else { return usage() };
            let src = match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error: {path}: {e}");
                    return ExitCode::from(2);
                }
            };
            let prog = match mlab::parser::parse_program(&src) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(1);
                }
            };
            let mut it = Interp::capture();
            it.load_functions(&prog);
            let (vp, script) = mlab::vm::compile_program(&it, &prog.body, &[]);
            print!("{}", mlab::vm::dump(&script));
            for p in &vp.protos {
                print!("{}", mlab::vm::dump(p));
            }
            ExitCode::SUCCESS
        }
        Some("mcgate") => {
            let fault = fault_arg(&args);
            let k = args.iter().position(|a| a == "-k").and_then(|i| args.get(i + 1)).and_then(|s| s.parse().ok()).unwrap_or(20000);
            let t0 = std::time::Instant::now();
            let rows = mlab::mcgate::run(k, 20260926, fault.as_deref());
            println!("test\tα\tfraction p ≤ α\ttolerance\tverdict\t(K = {k}, seed 20260926)");
            for r in &rows {
                println!("{}\t{}\t{:.4}\t±{:.4}\t{}", r.test, r.alpha, r.frac, r.tol, if r.ok { "ok" } else { "RED" });
            }
            let red = rows.iter().filter(|r| !r.ok).count();
            println!("TOTAL\t{} rows\t{red} red\t({:.2} s)", rows.len(), t0.elapsed().as_secs_f64());
            if red == 0 { ExitCode::SUCCESS } else { ExitCode::from(1) }
        }
        _ => usage(),
    }
}
