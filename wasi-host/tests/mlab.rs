//! WASI gate: the same mlab scenario gives the same output natively (`--fs mem`) and as `mlab.wasm` in our host
//! (`--fs real` in the guest = WASI calls → the same `vfs`). Plus guest limits: escaping the root, vfs limits, memory ceiling,
//! fuel, reproducibility, snapshot between runs.
//!
//! `mlab.wasm` is built separately: `cd mlab && cargo build --release --target wasm32-wasip1`. If it is missing or stale,
//! the test fails with this hint (fail-fast) instead of being silently skipped.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use wasi_host::{Ctx, Host};

const SUITE_FS: &str = include_str!("../../mlab/data/suite-fs.txt");
/// Known divergence between the native and WASM builds of mlab, not the host: la-18 (suite errata, singular matrix) — `norm` of
/// `inv(A)*A` with Inf/NaN: native faer (AVX2/FMA) gives a number, the wasm32 build (simd128) gives "svd: failed to converge".
/// V8 (Node 24, `node:wasi`) gives the same guest output too (verified on 26.09), so this is faer under wasm, not wasmi.
const WASM_BUILD_DIVERGENCE: &[&str] = &["la-18"];
const SUITES: [&str; 2] = [include_str!("../../mlab/data/suite-v1.txt"), include_str!("../../mlab/data/suite-v2.txt")];

fn newest_mtime(dir: &Path) -> std::time::SystemTime {
    let mut t = std::time::UNIX_EPOCH;
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        let m = if p.is_dir() { newest_mtime(&p) } else { e.metadata().unwrap().modified().unwrap() };
        t = t.max(m);
    }
    t
}

fn wasm_path() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let w = root.join("target/wasm32-wasip1/release/mlab.wasm");
    let hint = "build it: cd mlab && cargo build --release --target wasm32-wasip1";
    let wt = std::fs::metadata(&w).unwrap_or_else(|_| panic!("missing {} — {hint}", w.display())).modified().unwrap();
    for src in ["mlab/src", "vfs/src"] {
        assert!(newest_mtime(&root.join(src)) <= wt, "{} is older than {src} — {hint}", w.display());
    }
    w
}

fn host() -> &'static Host {
    static H: OnceLock<Host> = OnceLock::new();
    H.get_or_init(|| {
        let h = Host::new(&std::fs::read(wasm_path()).unwrap(), None).unwrap();
        assert!(h.missing_imports().is_empty(), "{:?}", h.missing_imports());
        h
    })
}

fn native(code: &str, vm: bool) -> String {
    let mut it = mlab::Interp::capture().with_fs(Box::new(vfs::Vfs::new()));
    it.vm_mode = vm;
    it.run(code);
    it.take_output()
}

fn guest_with(code: &str, vm: bool, v: vfs::Vfs) -> wasi_host::Outcome {
    let mut args: Vec<String> = ["mlab", "-e", code, "--fs", "real"].iter().map(|s| s.to_string()).collect();
    if vm {
        args.push("--vm".into());
    }
    let mut ctx = Ctx::new(v, args);
    ctx.merge_stderr = true;
    host().run(ctx)
}

fn guest(code: &str, vm: bool) -> String {
    let o = guest_with(code, vm, vfs::Vfs::new());
    assert!(o.trap.is_none(), "trap: {:?}", o.trap);
    String::from_utf8(o.ctx.stdout).unwrap()
}

#[test]
fn mlab_wasm_same_output_file_scenarios() {
    let sc = mlab::suite::parse(SUITE_FS);
    assert_eq!(sc.len(), 25);
    let mut same = 0;
    for s in &sc {
        for vm in [false, true] {
            let (n, g) = (native(&s.code, vm), guest(&s.code, vm));
            assert_eq!(n, g, "{} (vm={vm}): native ≠ guest", s.id);
            same += 1;
        }
        // and the suite's expected output too
        assert!(mlab::suite::check(&s.expect, &guest(&s.code, false)).is_none(), "{}", s.id);
    }
    assert_eq!(same, 50);
}

#[test]
fn mlab_wasm_same_output_frozen_suites() {
    let (mut n, mut same) = (0, 0);
    let mut diff = Vec::new();
    for text in SUITES {
        for s in mlab::suite::parse(text) {
            let (a, b) = (native(&s.code, false), guest(&s.code, false));
            n += 1;
            if WASM_BUILD_DIVERGENCE.contains(&s.id.as_str()) {
                assert_ne!(a, b, "{}: divergence is gone — remove it from WASM_BUILD_DIVERGENCE", s.id);
            } else if a != b {
                diff.push(format!("{}:\n  native: {a:?}\n  guest:  {b:?}", s.id));
            } else {
                same += 1;
            }
        }
    }
    assert!(diff.is_empty(), "{} divergences:\n{}", diff.len(), diff.join("\n"));
    assert_eq!((n, same), (250, 249));
}

#[test]
fn guest_escape_attempts_fail_and_are_journaled() {
    for a in ["fileread('../../etc/passwd')", "writelines('evil', '/../escape.txt')", "readtable('/tmp/../../x.csv')"] {
        let code = format!("try\n  {a};\n  disp('PASSED')\ncatch err\n  disp(err)\nend");
        let o = guest_with(&code, false, vfs::Vfs::new());
        let out = String::from_utf8(o.ctx.stdout).unwrap();
        assert!(out.contains("Operation not permitted") && !out.contains("PASSED"), "{a}: {out}");
        assert!(o.ctx.vfs.journal.denied() >= 1, "{a}: the journal did not see the attempt");
    }
    // an absolute host path is just a path in vfs
    let o = guest_with("try\n fileread('/etc/passwd');\ncatch err\n disp(err)\nend", false, vfs::Vfs::new());
    assert_eq!(String::from_utf8(o.ctx.stdout).unwrap(), "fileread: cannot open '/etc/passwd': No such file or directory (os error 2)\n");
}

#[test]
fn host_files_in_guest_writes_back_and_snapshot_gives_clean_machine() {
    let mut v = vfs::Vfs::new();
    v.mkdir("/data").unwrap();
    v.write("/data/sales.csv", b"region,amount\nN,10\nS,20\nN,5\n").unwrap();
    let clean = v.snapshot();
    let before = v.dump();
    let code = "T = readtable('/data/sales.csv'); G = groupsummary(T, 'region', 'sum', 'amount'); \
                mkdir('/out'); writetable(G, '/out/g.csv'); disp(height(G))";
    let o = guest_with(code, false, v);
    assert_eq!(String::from_utf8(o.ctx.stdout.clone()).unwrap(), "2\n");
    let mut after = o.ctx.vfs;
    assert_eq!(after.read("/out/g.csv").unwrap(), b"region,GroupCount,sum_amount\nN,2,15\nS,1,20\n");
    // snapshot → clean machine for the next lesson, byte for byte
    after.restore(&clean);
    assert_eq!(after.dump(), before);
    assert!(after.read("/out/g.csv").is_err());
}

#[test]
fn guest_is_deterministic() {
    let code = "tic; x = rand(3); t = toc; disp(t > 0); writematrix(x, '/tmp/r.csv'); disp(numel(fileread('/tmp/r.csv')) > 0)";
    let a = guest_with(code, false, vfs::Vfs::new());
    let b = guest_with(code, false, vfs::Vfs::new());
    assert_eq!(a.ctx.stdout, b.ctx.stdout);
    assert_eq!(a.ctx.vfs.fingerprint(), b.ctx.vfs.fingerprint(), "same run — same vfs state");
    assert_eq!(a.ctx.calls, b.ctx.calls);
    assert!(a.ctx.calls.get("clock_time_get").copied().unwrap_or(0) >= 2, "{:?}", a.ctx.calls);
    // toc in the guest is virtual and deterministic
    let t = guest("tic; toc", false);
    assert_eq!(t, guest("tic; toc", false));
    assert!(t.starts_with("Elapsed time is 0.0000"), "{t}");
}

#[test]
fn guest_limits_vfs_memory_and_fuel() {
    // vfs limit → errno in the guest
    let v = vfs::Vfs::new().with_limits(vfs::Limits { max_file_size: 100, ..vfs::Limits::default() });
    let o = guest_with("try\n writematrix(magic(20), '/tmp/m.csv');\ncatch err\n disp(err)\nend", false, v);
    assert_eq!(String::from_utf8(o.ctx.stdout).unwrap(), "writematrix: cannot write '/tmp/m.csv': File too large (os error 27)\n");
    assert_eq!(o.ctx.vfs.journal.limited(), 1);
    // guest memory ceiling: 2000×2000 f64 = 32 MB does not fit into 16 MB
    let mut ctx = Ctx::new(vfs::Vfs::new(), vec!["mlab".into(), "-e".into(), "A = zeros(2000); disp(size(A))".into()]);
    ctx = ctx.with_memory_limit(16 << 20);
    ctx.merge_stderr = true;
    let o = host().run(ctx);
    assert!(o.exit_code != 0 || o.trap.is_some(), "should have hit the memory ceiling: {:?}", String::from_utf8_lossy(&o.ctx.stdout));
    // fuel: an infinite loop stops
    let h = Host::new(&std::fs::read(wasm_path()).unwrap(), Some(50_000_000)).unwrap();
    let ctx = Ctx::new(vfs::Vfs::new(), vec!["mlab".into(), "-e".into(), "while true\n x = 1;\nend".into()]);
    let o = h.run(ctx);
    let trap = o.trap.expect("expected a fuel trap");
    assert!(trap.to_lowercase().contains("fuel"), "{trap}");
    assert!(o.fuel_used.is_some_and(|f| f > 49_000_000 && f <= 50_000_000), "{:?}", o.fuel_used);
}
