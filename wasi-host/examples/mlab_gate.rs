//! Gate "native mlab (--fs mem) vs mlab.wasm in wasi-host": `cargo run --release --example mlab_gate -- SUITE.txt… [--vm] [--fuel N]`.
//! Prints per scenario: id, same output or not, native and guest time; a summary at the end. Exit code 1 means there are divergences.

use std::time::Instant;
use wasi_host::{Ctx, Host};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let vm = args.iter().any(|a| a == "--vm");
    let fuel = args.iter().position(|a| a == "--fuel").and_then(|k| args.get(k + 1)).and_then(|s| s.parse().ok());
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let wasm = std::fs::read(root.join("target/wasm32-wasip1/release/mlab.wasm")).expect("mlab.wasm");
    let host = Host::new(&wasm, fuel).unwrap();
    let (mut same, mut total, mut tn, mut tg) = (0, 0, 0.0, 0.0);
    let mut diffs = Vec::new();
    println!("scenario\toutput\tnative, ms\tguest, ms");
    for f in args.iter().filter(|a| a.ends_with(".txt")) {
        for s in mlab::suite::parse(&std::fs::read_to_string(f).unwrap()) {
            let t = Instant::now();
            let mut it = mlab::Interp::capture().with_fs(Box::new(vfs::Vfs::new()));
            it.vm_mode = vm;
            it.run(&s.code);
            let n = it.take_output();
            let dn = t.elapsed().as_secs_f64();
            let mut a: Vec<String> = ["mlab", "-e", &s.code, "--fs", "real"].iter().map(|x| x.to_string()).collect();
            if vm {
                a.push("--vm".into());
            }
            let mut ctx = Ctx::new(vfs::Vfs::new(), a);
            ctx.merge_stderr = true;
            let t = Instant::now();
            let o = host.run(ctx);
            let dg = t.elapsed().as_secs_f64();
            let g = String::from_utf8_lossy(&o.ctx.stdout).into_owned() + &o.trap.map(|t| format!("<trap: {t}>")).unwrap_or_default();
            total += 1;
            tn += dn;
            tg += dg;
            let ok = n == g;
            if ok {
                same += 1;
            } else {
                diffs.push(format!("{}\n  native: {n:?}\n  guest:  {g:?}", s.id));
            }
            println!("{}\t{}\t{:.2}\t{:.2}", s.id, if ok { "same" } else { "DIFFERENT" }, dn * 1e3, dg * 1e3);
        }
    }
    println!("TOTAL\t{same}/{total}\t{:.2} s\t{:.2} s", tn, tg);
    for d in &diffs {
        println!("{d}");
    }
    std::process::exit(if diffs.is_empty() { 0 } else { 1 });
}
