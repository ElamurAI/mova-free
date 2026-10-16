//! vfs gates: root escape — red tests; limits; snapshot → changes → restore byte for byte; journal;
//! import and export; a real directory with a confined root.

use std::path::PathBuf;
use vfs::{DirFs, Errno, FileSystem, FileType, Limits, Op, Vfs};

/// Temporary test directory: cleaned up on scope exit (tests do not litter temp_dir).
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
    let d = std::env::temp_dir().join(format!("vfs-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    Scratch(d)
}

// ---------- root escape: every attempt is an error, not a silent success ----------

#[test]
fn escape_by_dotdot_is_denied() {
    let mut v = Vfs::new();
    v.write("/a.txt", b"x").unwrap();
    for p in ["..", "../a.txt", "/../a.txt", "/tmp/../../etc/passwd", "tmp/../../x", "../../../../../../etc/passwd"] {
        let e = v.read(p).unwrap_err();
        assert_eq!(e.errno, Errno::NotCapable, "{p}");
        assert!(e.to_string().contains("escapes the sandbox root"), "{p}: {e}");
        assert!(v.write(p, b"evil").is_err(), "{p}");
    }
    // inside the root `..` is allowed
    assert_eq!(v.read("/tmp/../a.txt").unwrap(), b"x");
    assert!(v.journal.denied() >= 12);
}

#[test]
fn absolute_paths_never_reach_the_host() {
    let mut v = Vfs::new();
    // the real /etc/passwd exists, but it is not visible from memory
    assert_eq!(v.read("/etc/passwd").unwrap_err().errno, Errno::NoEnt);
    // writing "to /etc" does not touch the host: there is no /etc directory in memory
    assert_eq!(v.write("/etc/passwd", b"evil").unwrap_err().errno, Errno::NoEnt);
    // and what was created in memory stays in memory
    v.mkdir("/etc").unwrap();
    v.write("/etc/passwd", b"root:x:0:0").unwrap();
    assert_eq!(v.read("/etc/passwd").unwrap(), b"root:x:0:0");
    let real = std::fs::read("/etc/passwd").unwrap_or_default();
    assert_ne!(real, b"root:x:0:0");
}

#[test]
fn symlinks_cannot_lead_outside() {
    let mut v = Vfs::new();
    v.mkdir("/d").unwrap();
    v.write("/d/f", b"in").unwrap();
    v.symlink("../../..", "/d/up").unwrap();
    v.symlink("/../x", "/abs_up").unwrap();
    v.symlink("../../../etc/passwd", "/d/pw").unwrap();
    for p in ["/d/up", "/d/up/etc/passwd", "/abs_up", "/d/pw"] {
        assert_eq!(v.read(p).unwrap_err().errno, Errno::NotCapable, "{p}");
    }
    assert_eq!(v.write("/d/pw", b"evil").unwrap_err().errno, Errno::NotCapable);
    // links inside work; an absolute target is relative to this tree's root
    v.symlink("f", "/d/ok").unwrap();
    v.symlink("/d/f", "/abs_ok").unwrap();
    assert_eq!(v.read("/d/ok").unwrap(), b"in");
    assert_eq!(v.read("/abs_ok").unwrap(), b"in");
    // loop
    v.symlink("/loop2", "/loop1").unwrap();
    v.symlink("/loop1", "/loop2").unwrap();
    assert_eq!(v.read("/loop1").unwrap_err().errno, Errno::Loop);
    // writing through a dangling link creates the target inside
    v.symlink("/d/new", "/dangling").unwrap();
    v.write("/dangling", b"n").unwrap();
    assert_eq!(v.read("/d/new").unwrap(), b"n");
}

#[test]
fn bad_names_and_paths() {
    let mut v = Vfs::new();
    assert_eq!(v.read("").unwrap_err().errno, Errno::NoEnt);
    assert_eq!(v.write("a\0b", b"").unwrap_err().errno, Errno::Inval);
    let long = "x".repeat(256);
    assert_eq!(v.write(&format!("/{long}"), b"").unwrap_err().errno, Errno::NameTooLong);
    assert_eq!(v.write("/tmp/", b"").unwrap_err().errno, Errno::IsDir);
    assert_eq!(v.write("/", b"").unwrap_err().errno, Errno::IsDir);
    v.write("/f", b"1").unwrap();
    assert_eq!(v.write("/f/g", b"").unwrap_err().errno, Errno::NotDir);
    assert_eq!(v.read("/f/").unwrap_err().errno, Errno::NotDir);
}

// ---------- limits ----------

#[test]
fn limits_fire_with_clear_errors_and_leave_state_unchanged() {
    let lim = Limits { max_total_bytes: 1000, max_files: 4, max_file_size: 600 };
    let mut v = Vfs::new().with_limits(lim); // /tmp is already 1 node
    let before = v.dump();
    let e = v.write("/big", &[0u8; 601]).unwrap_err();
    assert_eq!(e.errno, Errno::FBig);
    assert!(e.to_string().contains("max_file_size = 600"), "{e}");
    assert_eq!(v.dump(), before, "a violation does not change state");

    v.write("/a", &[1u8; 500]).unwrap();
    let e = v.write("/b", &[2u8; 501]).unwrap_err();
    assert_eq!(e.errno, Errno::NoSpc);
    assert!(e.to_string().contains("max_total_bytes = 1000"), "{e}");
    // overwriting the same file with a smaller one is allowed
    v.write("/a", &[1u8; 10]).unwrap();
    v.write("/b", &[2u8; 501]).unwrap();
    let e = v.append("/b", &[0u8; 100]).unwrap_err();
    assert_eq!(e.errno, Errno::FBig);
    v.mkdir("/d").unwrap(); // 4 nodes: tmp, a, b, d
    let e = v.write("/c", b"").unwrap_err();
    assert_eq!(e.errno, Errno::DQuot);
    assert!(e.to_string().contains("max_files = 4"), "{e}");
    assert_eq!(v.mkdir("/e").unwrap_err().errno, Errno::DQuot);
    assert_eq!(v.symlink("/a", "/l").unwrap_err().errno, Errno::DQuot);
    // freed up — allowed again
    v.remove_file("/a").unwrap();
    v.write("/c", b"ok").unwrap();
    assert_eq!(v.usage().files, 4);
    assert_eq!(v.usage().bytes, 503);
    assert!(v.journal.limited() >= 6);
    // write_at with a hole counts too
    assert_eq!(v.write_at("/c", 10_000, b"x").unwrap_err().errno, Errno::FBig);
}

#[test]
fn limits_parse() {
    let l = Limits::parse("total=1G,file=64K,files=10").unwrap();
    assert_eq!((l.max_total_bytes, l.max_file_size, l.max_files), (1 << 30, 64 << 10, 10));
    assert!(Limits::parse("size=1").is_err());
    assert!(Limits::parse("total=abc").is_err());
}

// ---------- snapshots ----------

fn populate(v: &mut Vfs, n: usize) {
    v.mkdir_all("/data/sub").unwrap();
    for i in 0..n {
        v.write(&format!("/data/f{i}.csv"), format!("a,b\n{i},{}\n", i * i).as_bytes()).unwrap();
    }
    v.write("/data/sub/big.bin", &vec![7u8; 100_000]).unwrap();
    v.symlink("../f1.csv", "/data/sub/link").unwrap();
}

#[test]
fn snapshot_changes_restore_is_byte_identical() {
    let mut v = Vfs::new();
    populate(&mut v, 50);
    let before = v.dump();
    let fp = v.fingerprint();
    let s = v.snapshot();
    // changes of all kinds
    v.write("/data/f0.csv", b"changed").unwrap();
    v.append("/data/f1.csv", b"more\n").unwrap();
    v.write_at("/data/sub/big.bin", 5, b"XYZ").unwrap();
    v.truncate("/data/f2.csv", 1).unwrap();
    v.remove_file("/data/f3.csv").unwrap();
    v.rename("/data/f4.csv", "/tmp/moved.csv").unwrap();
    v.mkdir_all("/new/deep/dir").unwrap();
    v.remove_dir_all("/data/sub").unwrap();
    v.chmod("/data/f5.csv", 0o400).unwrap();
    v.chdir("/new").unwrap();
    let _ = v.temp_name("t_");
    assert_ne!(v.dump(), before);
    v.restore(&s);
    assert_eq!(v.dump(), before, "restore gives byte-for-byte the same state");
    assert_eq!(v.fingerprint(), fp);
    assert_eq!(v.read("/data/sub/big.bin").unwrap(), vec![7u8; 100_000]);
    // and a second time from the same snapshot
    v.write("/x", b"1").unwrap();
    v.restore(&s);
    assert_eq!(v.dump(), before);
}

#[test]
fn snapshot_is_isolated_copy_on_write() {
    let mut v = Vfs::new();
    populate(&mut v, 5);
    let s = v.snapshot();
    v.write_at("/data/sub/big.bin", 0, b"Q").unwrap();
    // another sandbox from the same snapshot sees the old contents
    let mut w = Vfs::from_snapshot(&s, Limits::default());
    assert_eq!(w.read("/data/sub/big.bin").unwrap()[0], 7);
    assert_eq!(v.read("/data/sub/big.bin").unwrap()[0], b'Q');
    // in parallel threads
    let hs: Vec<_> = (0..4)
        .map(|t| {
            let s = s.clone();
            std::thread::spawn(move || {
                let mut w = Vfs::from_snapshot(&s, Limits::default());
                w.write(&format!("/data/t{t}"), b"x").unwrap();
                (w.read_dir("/data").unwrap().len(), w.read("/data/f1.csv").unwrap())
            })
        })
        .collect();
    for h in hs {
        let (n, f1) = h.join().unwrap();
        assert_eq!(n, 7); // f0..f4, sub, t{t}
        assert_eq!(f1, b"a,b\n1,1\n");
    }
}

#[test]
fn restore_after_failed_ops_and_limits() {
    let mut v = Vfs::new().with_limits(Limits { max_total_bytes: 100, max_files: 10, max_file_size: 100 });
    v.write("/a", b"hello").unwrap();
    let s = v.snapshot();
    let before = v.dump();
    let _ = v.write("/b", &[0u8; 200]);
    let _ = v.read("../x");
    let _ = v.remove_dir("/tmp/nope");
    assert_eq!(v.dump(), before, "failed operations do not change state");
    v.write("/b", b"1").unwrap();
    v.restore(&s);
    assert_eq!(v.dump(), before);
}

// ---------- operations ----------

#[test]
fn posix_like_semantics() {
    let mut v = Vfs::new();
    v.mkdir("/d").unwrap();
    assert_eq!(v.mkdir("/d").unwrap_err().errno, Errno::Exist);
    assert_eq!(v.mkdir("/x/y").unwrap_err().errno, Errno::NoEnt);
    v.write("/d/a", b"1").unwrap();
    assert_eq!(v.remove_dir("/d").unwrap_err().errno, Errno::NotEmpty);
    assert_eq!(v.remove_file("/d").unwrap_err().errno, Errno::IsDir);
    assert_eq!(v.remove_dir("/d/a").unwrap_err().errno, Errno::NotDir);
    assert_eq!(v.rename("/d", "/d/sub").unwrap_err().errno, Errno::Inval);
    v.mkdir("/e").unwrap();
    v.write("/e/b", b"2").unwrap();
    assert_eq!(v.rename("/d", "/e").unwrap_err().errno, Errno::NotEmpty);
    assert_eq!(v.rename("/d/a", "/e").unwrap_err().errno, Errno::IsDir);
    v.rename("/d/a", "/e/b").unwrap(); // replacing a file
    assert_eq!(v.read("/e/b").unwrap(), b"1");
    assert_eq!(v.usage().files, 4); // tmp, d, e, e/b
    v.chdir("/e").unwrap();
    assert_eq!(v.read("b").unwrap(), b"1");
    assert_eq!(v.read("../e/./b").unwrap(), b"1");
    v.chmod("b", 0o444).unwrap();
    assert_eq!(v.write("b", b"no").unwrap_err().errno, Errno::Acces);
    v.chmod("b", 0o200).unwrap();
    assert_eq!(v.read("b").unwrap_err().errno, Errno::Acces);
    let names: Vec<String> = v.read_dir("/").unwrap().into_iter().map(|e| e.name).collect();
    assert_eq!(names, ["d", "e", "tmp"]);
    assert_eq!(v.stat("/e").unwrap().kind, FileType::Dir);
    assert_eq!(v.temp_name("m_"), "/tmp/m_1");
    v.write("/tmp/m_2", b"").unwrap();
    assert_eq!(v.temp_name("m_"), "/tmp/m_3");
    assert_eq!(v.read_at("/tmp/m_2", 5, 10).unwrap(), b"");
}

#[test]
fn virtual_clock_is_deterministic() {
    let run = || {
        let mut v = Vfs::new();
        populate(&mut v, 3);
        (v.stat("/data/f2.csv").unwrap().mtime, v.fingerprint())
    };
    assert_eq!(run(), run());
    assert!(run().0 > vfs::EPOCH_NS);
}

#[test]
fn journal_records_ops_errors_and_denials() {
    let mut v = Vfs::new();
    v.write("/a", b"12345").unwrap();
    let _ = v.read("/a");
    let _ = v.read("/missing");
    let _ = v.read("../../etc/shadow");
    let e = v.journal.entries();
    assert_eq!(e.len(), 4);
    assert_eq!((e[0].op, e[0].bytes, e[0].err), (Op::Write, 5, None));
    assert_eq!((e[1].op, e[1].bytes), (Op::Read, 5));
    assert_eq!(e[2].err, Some(Errno::NoEnt));
    assert_eq!(e[3].err, Some(Errno::NotCapable));
    assert_eq!((v.journal.errors(), v.journal.denied()), (2, 1));
    let tsv = v.journal.tsv();
    assert!(tsv.lines().nth(4).unwrap().ends_with("read\t../../etc/shadow\t0\tENOTCAPABLE"), "{tsv}");
    // cap: ordinary entries are dropped with a counter, errors have headroom
    let mut w = Vfs::new();
    w.journal.cap = 3;
    for i in 0..5 {
        w.write(&format!("/f{i}"), b"").unwrap();
    }
    let _ = w.read("/nope");
    assert_eq!(w.journal.entries().len(), 4);
    assert_eq!(w.journal.dropped(), 2);
    assert!(w.journal.summary().contains("6 operations"));
}

// ---------- import and export ----------

#[test]
fn import_export_round_trip() {
    let src = scratch("imp");
    std::fs::create_dir_all(src.join("sub/deeper")).unwrap();
    std::fs::write(src.join("a.csv"), "x,y\n1,2\n").unwrap();
    std::fs::write(src.join("sub/deeper/b.bin"), [0u8, 1, 2, 255]).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink("/etc/passwd", src.join("evil")).unwrap();
    let mut v = Vfs::new();
    let t = v.import_dir(&src, "/in").unwrap();
    assert_eq!((t.files, t.dirs, t.bytes), (2, 2, 12));
    assert_eq!(v.read("/in/a.csv").unwrap(), b"x,y\n1,2\n");
    #[cfg(unix)]
    {
        assert_eq!(t.links, 1);
        // the link "to /etc/passwd" leads to /etc/passwd of this tree, not the host's
        assert_eq!(v.read("/in/evil").unwrap_err().errno, Errno::NoEnt);
    }
    v.write("/in/sub/new.txt", b"made in memory").unwrap();
    let out = scratch("exp");
    let t = v.export_dir("/in", &out).unwrap();
    assert_eq!(t.files, 3);
    assert_eq!(std::fs::read(out.join("sub/deeper/b.bin")).unwrap(), [0u8, 1, 2, 255]);
    assert_eq!(std::fs::read_to_string(out.join("sub/new.txt")).unwrap(), "made in memory");
    assert!(!out.join("evil").exists(), "links are not exported");
    // we do not write through a real link
    #[cfg(unix)]
    {
        let out2 = scratch("exp2");
        let target = scratch("exp2-target");
        std::os::unix::fs::symlink(&target, out2.join("sub")).unwrap();
        let e = v.export_dir("/in", &out2).unwrap_err();
        assert!(e.to_string().contains("refusing to write through the real symlink"), "{e}");
        assert!(std::fs::read_dir(&target).unwrap().next().is_none());
    }
    // limit on import — a clear error before reading
    let mut small = Vfs::new().with_limits(Limits { max_file_size: 3, ..Limits::default() });
    let e = small.import_dir(&src, "/").unwrap_err();
    assert_eq!(e.errno, Errno::FBig);
    assert!(e.to_string().contains("max_file_size = 3"), "{e}");
}

// ---------- real directory with a confined root ----------

#[test]
fn dirfs_confines_to_root() {
    let root = scratch("dirfs");
    let outside = scratch("dirfs-outside");
    std::fs::write(outside.join("secret.txt"), "secret").unwrap();
    std::fs::create_dir_all(root.join("sub")).unwrap();
    std::fs::write(root.join("sub/in.txt"), "in").unwrap();
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&outside, root.join("out_dir")).unwrap();
        std::os::unix::fs::symlink(outside.join("secret.txt"), root.join("out_file")).unwrap();
        std::os::unix::fs::symlink(outside.join("created.txt"), root.join("dangling_out")).unwrap();
        std::os::unix::fs::symlink("sub/in.txt", root.join("in_link")).unwrap();
    }
    let mut d = DirFs::new(&root).unwrap();
    assert_eq!(d.read("/sub/in.txt").unwrap(), b"in");
    assert_eq!(d.read("sub/../sub/in.txt").unwrap(), b"in");
    // `/` is the directory root: the real /etc/passwd is unreachable
    assert_eq!(d.read("/etc/passwd").unwrap_err().errno, Errno::NoEnt);
    let rel = format!("../{}/secret.txt", outside.file_name().unwrap().to_str().unwrap());
    for p in ["..", "../x", rel.as_str(), "/../../etc/passwd"] {
        assert_eq!(d.read(p).unwrap_err().errno, Errno::NotCapable, "{p}");
        assert!(d.write(p, b"evil").is_err(), "{p}");
    }
    #[cfg(unix)]
    {
        for p in ["/out_dir/secret.txt", "/out_file", "/dangling_out"] {
            assert_eq!(d.read(p).unwrap_err().errno, Errno::NotCapable, "{p}");
        }
        assert_eq!(d.write("/dangling_out", b"evil").unwrap_err().errno, Errno::NotCapable);
        assert_eq!(d.write("/out_dir/new.txt", b"evil").unwrap_err().errno, Errno::NotCapable);
        assert!(!outside.join("created.txt").exists());
        assert!(!outside.join("new.txt").exists());
        assert_eq!(d.read("/in_link").unwrap(), b"in");
    }
    assert_eq!(std::fs::read_to_string(outside.join("secret.txt")).unwrap(), "secret");
    let t = d.temp_name("mlab_");
    assert_eq!(t, "/tmp/mlab_1");
    d.write(&t, b"tmp").unwrap();
    assert_eq!(std::fs::read(root.join("tmp/mlab_1")).unwrap(), b"tmp");
    assert!(d.journal.denied() >= 8);
}

#[test]
fn open_modes() {
    assert_eq!(vfs::open("mem", Limits::default()).unwrap().describe(), "mem");
    assert_eq!(vfs::open("real", Limits::default()).unwrap().describe(), "real");
    let root = scratch("open");
    assert!(vfs::open(&format!("dir:{}", root.display()), Limits::default()).unwrap().describe().starts_with("dir:/"));
    assert!(vfs::open("dir:/nonexistent/zzz", Limits::default()).is_err());
    assert!(vfs::open("disk", Limits::default()).is_err());
}

// ---------- negative controls: every gate can show red ----------

use vfs::Fault;

/// Root-escape gate: Ok — green, Err — red with the reason.
fn gate_escape(fault: Option<Fault>) -> Result<(), String> {
    let mut v = Vfs::new();
    v.fault = fault;
    v.mkdir("/d").map_err(|e| e.to_string())?;
    v.symlink("../../..", "/d/up").map_err(|e| e.to_string())?;
    for p in ["..", "../etc/passwd", "/../x", "/d/up", "/d/up/etc"] {
        match v.read(p) {
            Err(e) if e.errno == Errno::NotCapable => {}
            other => return Err(format!("{p}: expected ENOTCAPABLE, got {other:?}")),
        }
    }
    Ok(())
}

fn gate_limits(fault: Option<Fault>) -> Result<(), String> {
    let mut v = Vfs::new().with_limits(Limits { max_total_bytes: 100, max_files: 3, max_file_size: 50 });
    v.fault = fault;
    let checks = [
        (v.write("/a", &[0u8; 51]).err().map(|e| e.errno), Errno::FBig),
        ({
            let _ = v.write("/b", &[0u8; 50]);
            v.write("/c", &[0u8; 50]).ok();
            v.write("/c2", &[0u8; 1]).err().map(|e| e.errno)
        }, Errno::DQuot),
    ];
    for (got, want) in checks {
        if got != Some(want) {
            return Err(format!("expected {want:?}, got {got:?}"));
        }
    }
    Ok(())
}

fn gate_snapshot(fault: Option<Fault>) -> Result<(), String> {
    let mut v = Vfs::new();
    v.fault = fault;
    populate(&mut v, 10);
    let before = v.dump();
    let s = v.snapshot();
    v.write("/data/f0.csv", b"changed").unwrap();
    v.remove_dir_all("/data/sub").unwrap();
    v.restore(&s);
    if v.dump() != before { Err("state after restore is not the same".into()) } else { Ok(()) }
}

fn gate_dirfs(fault: Option<Fault>) -> Result<(), String> {
    let root = scratch(&format!("gate-dirfs-{fault:?}"));
    let outside = scratch(&format!("gate-dirfs-out-{fault:?}"));
    std::fs::write(outside.join("s"), "secret").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(outside.join("s"), root.join("out")).unwrap();
    let mut d = DirFs::new(&root).unwrap();
    d.fault = fault;
    let mut probes = vec!["../x", "/.."];
    if cfg!(unix) {
        probes.push("/out");
    }
    for p in probes {
        match d.read(p) {
            Err(e) if e.errno == Errno::NotCapable => {}
            other => return Err(format!("{p}: expected ENOTCAPABLE, got {:?}", other.map(|b| String::from_utf8_lossy(&b).into_owned()))),
        }
    }
    Ok(())
}

#[test]
fn gates_green_and_negative_controls_red() {
    assert_eq!(gate_escape(None), Ok(()));
    assert_eq!(gate_limits(None), Ok(()));
    assert_eq!(gate_snapshot(None), Ok(()));
    assert_eq!(gate_dirfs(None), Ok(()));
    let red = [
        ("escape/ClampDotDot", gate_escape(Some(Fault::ClampDotDot))),
        ("limits/NoLimits", gate_limits(Some(Fault::NoLimits))),
        ("snapshot/RestoreForgetsClock", gate_snapshot(Some(Fault::RestoreForgetsClock))),
        ("dirfs/ClampDotDot", gate_dirfs(Some(Fault::ClampDotDot))),
        ("dirfs/NoConfine", gate_dirfs(Some(Fault::NoConfine))),
    ];
    for (name, r) in red {
        assert!(r.is_err(), "negative control {name} should have been red");
        eprintln!("{name}: red — {}", r.unwrap_err());
    }
}
