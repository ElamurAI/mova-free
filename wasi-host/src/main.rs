//! `wasi-run` — runs a wasm32-wasip1 module in a sandbox: in-memory file system (vfs), virtual clock,
//! seeded randomness, no network.
//!
//! ```text
//! wasi-run [--fs-import DIR[:VIRT]]… [--fs-export DIR[:VIRT]]… [--fs-limits total=256M,file=64M,files=65536]
//!          [--seed N] [--fuel N] [--mem-limit 1G] [--journal FILE.tsv] [--stats] MODULE.wasm [ARGS…]
//! ```

use std::process::ExitCode;
use wasi_host::{Ctx, Host};

fn usage() -> ExitCode {
    eprintln!(
        "usage: wasi-run [--fs-import DIR[:VIRT]]… [--fs-export DIR[:VIRT]]… [--fs-limits …] [--seed N] \
         [--fuel N] [--mem-limit 1G] [--journal FILE.tsv] [--stats] MODULE.wasm [ARGS…]"
    );
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (mut imports, mut exports) = (Vec::new(), Vec::new());
    let (mut limits, mut seed, mut fuel, mut mem, mut journal, mut stats) =
        (vfs::Limits::default(), None, None, 1usize << 30, None, false);
    let mut k = 0;
    while k < args.len() && args[k].starts_with("--") {
        let flag = args[k].as_str();
        if flag == "--stats" {
            stats = true;
            k += 1;
            continue;
        }
        let Some(v) = args.get(k + 1) else { return usage() };
        let pair = |s: &str| match s.split_once(':') {
            Some((a, b)) => (a.to_string(), b.to_string()),
            None => (s.to_string(), "/".to_string()),
        };
        let r: Result<(), String> = (|| {
            match flag {
                "--fs-import" => imports.push(pair(v)),
                "--fs-export" => exports.push(pair(v)),
                "--fs-limits" => limits = vfs::Limits::parse(v)?,
                "--seed" => seed = Some(v.parse::<u64>().map_err(|e| format!("--seed: {e}"))?),
                "--fuel" => fuel = Some(v.parse::<u64>().map_err(|e| format!("--fuel: {e}"))?),
                "--mem-limit" => mem = vfs::parse_size(v)? as usize,
                "--journal" => journal = Some(v.clone()),
                _ => return Err(format!("unknown flag {flag}")),
            }
            Ok(())
        })();
        if let Err(e) = r {
            eprintln!("error: {e}");
            return usage();
        }
        k += 2;
    }
    let Some(module_path) = args.get(k) else { return usage() };
    let wasm = match std::fs::read(module_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error: {module_path}: {e}");
            return ExitCode::from(2);
        }
    };
    let host = match Host::new(&wasm, fuel) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    let missing = host.missing_imports();
    if !missing.is_empty() {
        eprintln!("error: module imports what the host does not provide: {}", missing.join(", "));
        return ExitCode::from(2);
    }
    let mut v = vfs::Vfs::new().with_limits(limits);
    for (real, virt) in &imports {
        if let Err(e) = v.import_dir(std::path::Path::new(real), virt) {
            eprintln!("error: --fs-import {real}: {e}");
            return ExitCode::from(2);
        }
    }
    let name = std::path::Path::new(module_path).file_name().map_or("module".into(), |n| n.to_string_lossy().into_owned());
    let mut guest_args = vec![name];
    guest_args.extend(args[k + 1..].iter().cloned());
    let mut ctx = Ctx::new(v, guest_args);
    if let Some(s) = seed {
        ctx = ctx.with_seed(s);
    }
    ctx = ctx.with_memory_limit(mem);
    ctx.echo = true;
    let t0 = std::time::Instant::now();
    let mut out = host.run(ctx);
    let dt = t0.elapsed().as_secs_f64();
    if let Some(t) = &out.trap {
        eprintln!("wasi-run: guest trap: {t}");
    }
    let ok = out.exit_code == 0 && out.trap.is_none();
    for (real, virt) in &exports {
        if !ok {
            eprintln!("--fs-export {real}: skipped — guest exited with code {}", out.exit_code);
            continue;
        }
        if let Err(e) = out.ctx.vfs.export_dir(virt, std::path::Path::new(real)) {
            eprintln!("error: --fs-export {real}: {e}");
            return ExitCode::from(2);
        }
    }
    if let Some(j) = &journal {
        if let Err(e) = std::fs::write(j, out.ctx.vfs.journal.tsv()) {
            eprintln!("error: --journal {j}: {e}");
        }
    }
    if stats {
        let calls: Vec<String> = out.ctx.calls.iter().map(|(k, v)| format!("{k}={v}")).collect();
        eprintln!("wasi-run: code {}, {dt:.3} s, fuel {:?}", out.exit_code, out.fuel_used);
        eprintln!("wasi-run: WASI calls: {}", calls.join(" "));
        eprintln!("wasi-run: {}", out.ctx.vfs.journal.summary());
    }
    ExitCode::from(out.exit_code.clamp(0, 255) as u8)
}
