//! vfs in mlab (26.09): frozen suites — the same output in memory, in a directory and on the real FS; the file suite;
//! attempts to escape the root — red; limits; CLI `--fs`, `--fs-import`, `--fs-export`, `--fs-journal`.

use mlab::{Interp, suite};
use std::path::PathBuf;

const SUITES: [&str; 2] = [include_str!("../data/suite-v1.txt"), include_str!("../data/suite-v2.txt")];
const SUITE_FS: &str = include_str!("../data/suite-fs.txt");

/// Temporary test directory: removed when it goes out of scope (tests do not litter temp_dir).
struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
impl std::ops::Deref for Scratch {
    type Target = std::path::Path;
    fn deref(&self) -> &std::path::Path {
        &self.0
    }
}
impl AsRef<std::path::Path> for Scratch {
    fn as_ref(&self) -> &std::path::Path {
        &self.0
    }
}

fn scratch(name: &str) -> Scratch {
    let d = std::env::temp_dir().join(format!("mlab-fs-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    Scratch(d)
}

/// A fresh «machine» directory for `--fs dir:` — root and `tmp/`, like a fresh `Vfs::new()`; the directory lives as long as the guard.
fn fresh_dir(tag: &str) -> (Box<dyn vfs::FileSystem>, Scratch) {
    let d = scratch(tag);
    std::fs::create_dir(d.join("tmp")).unwrap();
    (Box::new(vfs::DirFs::new(&d).unwrap()), d)
}

fn run_with(code: &str, fs: Box<dyn vfs::FileSystem>, vm: bool) -> String {
    let mut it = Interp::capture().with_fs(fs);
    it.vm_mode = vm;
    it.run(code);
    it.take_output()
}

#[test]
fn frozen_suites_same_output_in_mem_real_and_dir() {
    let mut n = 0;
    for text in SUITES {
        for s in suite::parse(text) {
            for vm in [false, true] {
                let mem = run_with(&s.code, Box::new(vfs::Vfs::new()), vm);
                let real = run_with(&s.code, Box::new(vfs::RealFs::new()), vm);
                let (fs, _dir) = fresh_dir(&format!("{}-{vm}", s.id));
                let dir = run_with(&s.code, fs, vm);
                assert_eq!(mem, real, "{} (vm={vm}): mem ≠ real", s.id);
                assert_eq!(mem, dir, "{} (vm={vm}): mem ≠ dir", s.id);
                n += 1;
            }
        }
    }
    assert_eq!(n, 2 * (206 + 44));
    // the real FS (as before vfs) leaves temporary files `mlab_<pid>_…` in temp_dir — we clean up our own
    let mine = format!("mlab_{}_", std::process::id());
    for e in std::fs::read_dir(std::env::temp_dir()).unwrap().flatten() {
        if e.file_name().to_string_lossy().starts_with(&mine) {
            let _ = std::fs::remove_file(e.path());
        }
    }
}

#[test]
fn suite_fs_passes_in_mem_and_dir_with_same_output() {
    let sc = suite::parse(SUITE_FS);
    assert_eq!(sc.len(), 25);
    let red: Vec<String> = suite::run(&sc, None).into_iter().filter(|r| !r.pass).map(|r| format!("{} {:?}", r.id, r.first_diff)).collect();
    assert!(red.is_empty(), "mem, red: {red:?}");
    for s in &sc {
        for vm in [false, true] {
            let mem = run_with(&s.code, Box::new(vfs::Vfs::new()), vm);
            let (fs, _dir) = fresh_dir(&format!("fs-{}-{vm}", s.id));
            let dir = run_with(&s.code, fs, vm);
            assert_eq!(mem, dir, "{} (vm={vm})", s.id);
        }
    }
}

#[test]
fn default_fs_is_memory_and_invisible_outside() {
    let probe = std::env::temp_dir().join(format!("mlab_vfs_probe_{}.txt", std::process::id()));
    let p = probe.to_string_lossy().into_owned();
    let dir = std::env::temp_dir().display().to_string();
    let code = format!("if ~isfolder('{dir}')\n  mkdir('{dir}');\nend\nwritelines('secret', '{p}'); disp(fileread('{p}'))");
    let out = run_with(&code, Box::new(vfs::Vfs::new()), false);
    assert_eq!(out, "secret\n\n");
    assert!(!probe.exists(), "a write in memory must not reach the disk");
    // and by default Interp is in memory too
    let mut it = Interp::capture();
    assert_eq!(it.fs.describe(), "mem");
    it.run(&format!("writelines('x', '{p}');"));
    assert!(!probe.exists());
}

#[test]
fn escape_attempts_are_red_in_mem_and_dir() {
    let attempts = [
        "fileread('../../../../etc/passwd')",
        "fileread('/../etc/passwd')",
        "writelines('evil', '../escape.txt')",
        "readtable('/tmp/../../x.csv')",
        "writematrix(1, '../../x.csv')",
        "mkdir('../up')",
    ];
    let root = scratch("escape-root");
    let outside = scratch("escape-outside");
    std::fs::write(outside.join("secret.txt"), "secret").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(outside.join("secret.txt"), root.join("link")).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(&outside, root.join("dirlink")).unwrap();
    for a in attempts {
        let code = format!("try\n  {a};\n  disp('PASSED')\ncatch err\n  disp(err)\nend");
        for (mode, fs) in [("mem", Box::new(vfs::Vfs::new()) as Box<dyn vfs::FileSystem>), ("dir", Box::new(vfs::DirFs::new(&root).unwrap()))] {
            let out = run_with(&code, fs, false);
            assert!(out.contains("escapes the sandbox root"), "{mode}: {a} → {out}");
        }
    }
    #[cfg(unix)]
    for a in ["fileread('/link')", "writelines('evil', '/dirlink/new.txt')", "fileread('dirlink/secret.txt')"] {
        let code = format!("try\n  {a};\n  disp('PASSED')\ncatch err\n  disp(err)\nend");
        let out = run_with(&code, Box::new(vfs::DirFs::new(&root).unwrap()), false);
        assert!(out.contains("escapes the sandbox root"), "dir: {a} → {out}");
    }
    assert!(!outside.join("new.txt").exists());
    assert!(!root.parent().unwrap().join("escape.txt").exists());
    // `/etc/passwd` in memory — there is simply no such file
    let out = run_with("try\n fileread('/etc/passwd');\ncatch err\n disp(err)\nend", Box::new(vfs::Vfs::new()), false);
    assert_eq!(out, "fileread: cannot open '/etc/passwd': No such file or directory (os error 2)\n");
}

#[test]
fn fs_limits_fire_with_clear_errors() {
    let lim = vfs::Limits { max_total_bytes: 10_000, max_files: 5, max_file_size: 2_000 };
    let fs = Box::new(vfs::Vfs::new().with_limits(lim));
    let out = run_with(
        "try\n  writematrix(magic(40), '/big.csv');\ncatch err\n  disp(err)\nend\n\
         for k = 1:6\n  try\n    writelines('x', sprintf('/f%d.txt', k));\n  catch err\n    disp(err)\n  end\nend",
        fs,
        false,
    );
    assert!(out.contains("writematrix: cannot write '/big.csv': vfs limit: file '/big.csv' would be"), "{out}");
    assert!(out.contains("max_file_size = 2000"), "{out}");
    // /tmp + f1…f4 = 5 nodes; f5 and f6 hit the limit
    assert_eq!(out.matches("max_files = 5").count(), 2, "{out}");
}

// ---------- CLI ----------

fn mlab(args: &[&str]) -> (i32, String, String) {
    let o = std::process::Command::new(env!("CARGO_BIN_EXE_mlab")).args(args).output().unwrap();
    (o.status.code().unwrap_or(-1), String::from_utf8_lossy(&o.stdout).into_owned(), String::from_utf8_lossy(&o.stderr).into_owned())
}

#[test]
fn cli_import_export_journal() {
    let src = scratch("cli-src");
    std::fs::write(src.join("sales.csv"), "region,amount\nN,10\nS,20\nN,5\n").unwrap();
    let out = scratch("cli-out");
    let jdir = scratch("cli-j");
    let journal = jdir.join("j.tsv");
    let code = "T = readtable('/in/sales.csv'); G = groupsummary(T, 'region', 'sum', 'amount'); mkdir('/out'); writetable(G, '/out/g.csv'); disp(height(G))";
    let (rc, so, se) = mlab(&[
        "-e",
        code,
        "--fs-import",
        &format!("{}:/in", src.display()),
        "--fs-export",
        &format!("{}:/out", out.display()),
        "--fs-journal",
        journal.to_str().unwrap(),
    ]);
    assert_eq!((rc, so.as_str(), se.as_str()), (0, "2\n", ""));
    let g = std::fs::read_to_string(out.join("g.csv")).unwrap();
    assert!(g.starts_with("region,GroupCount,sum_amount\n"), "{g}");
    assert!(!out.join("in").exists(), "only /out is exported");
    let j = std::fs::read_to_string(&journal).unwrap();
    assert!(j.contains("\timport\t") && j.contains("\texport\t") && j.contains("\twrite\t/out/g.csv\t"), "{j}");
    // script error — export skipped
    let out2 = scratch("cli-out2");
    let (rc, _, se) = mlab(&["-e", "mkdir('/out'); writelines('x', '/out/a'); error('boom')", "--fs-export", &format!("{}:/out", out2.display())]);
    assert_eq!(rc, 1);
    assert!(se.contains("skipped"), "{se}");
    assert!(!out2.join("a").exists());
    // import/export only with mem
    let (rc, _, se) = mlab(&["-e", "1", "--fs", "real", "--fs-import", "/tmp"]);
    assert_eq!(rc, 2, "{se}");
    // unknown mode
    let (rc, _, se) = mlab(&["-e", "1", "--fs", "disk"]);
    assert_eq!(rc, 2, "{se}");
    // limits from the CLI
    let (rc, _, se) = mlab(&["-e", "writematrix(magic(30), '/tmp/m.csv')", "--fs-limits", "file=1K"]);
    assert_eq!(rc, 1);
    assert!(se.contains("max_file_size = 1024"), "{se}");
}

#[test]
fn cli_dir_mode_reads_and_writes_inside_root_only() {
    let root = scratch("cli-dir");
    std::fs::write(root.join("a.csv"), "x\n1\n2\n").unwrap();
    let (rc, so, se) = mlab(&["-e", "M = readmatrix('a.csv'); writematrix(M * 10, '/b.csv'); disp(sum(M))", "--fs", &format!("dir:{}", root.display())]);
    assert_eq!((rc, so.as_str(), se.as_str()), (0, "3\n", ""));
    assert_eq!(std::fs::read_to_string(root.join("b.csv")).unwrap(), "10\n20\n");
    let (rc, _, se) = mlab(&["-e", "fileread('../x')", "--fs", &format!("dir:{}", root.display())]);
    assert_eq!(rc, 1);
    assert!(se.contains("escapes the sandbox root"), "{se}");
}
