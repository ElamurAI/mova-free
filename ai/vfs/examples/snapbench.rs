//! Snapshot speed: `cargo run --release --example snapbench`.
//! A tree of N files of 1 KiB each plus a 64 MiB file; snapshot, restore, "lesson" (restore + change one file),
//! first write after a snapshot (path copying) — versus a deep copy of all contents.

use std::time::Instant;
use vfs::{Limits, Vfs};

fn per_op(n: u32, mut f: impl FnMut()) -> f64 {
    let t = Instant::now();
    for _ in 0..n {
        f();
    }
    t.elapsed().as_secs_f64() / n as f64
}

fn fmt(s: f64) -> String {
    if s < 1e-6 { format!("{:.0} ns", s * 1e9) } else if s < 1e-3 { format!("{:.1} µs", s * 1e6) } else { format!("{:.2} ms", s * 1e3) }
}

fn main() {
    println!("files\tsize, MiB\tsnapshot\trestore\tlesson (restore + write 1 file)\tfirst write after snapshot\tdeep copy (dump)");
    for n in [1_000usize, 10_000, 100_000] {
        let mut v = Vfs::new().with_limits(Limits::UNLIMITED);
        v.journal.cap = 0;
        for d in 0..(n / 1000) {
            v.mkdir(&format!("/d{d}")).unwrap();
            for i in 0..1000 {
                v.write(&format!("/d{d}/f{i}.csv"), &[b'x'; 1024]).unwrap();
            }
        }
        v.write("/big.bin", &vec![1u8; 64 << 20]).unwrap();
        let mb = v.usage().bytes as f64 / (1 << 20) as f64;
        let s = v.snapshot();
        let snap = per_op(1_000_000, || {
            std::hint::black_box(v.snapshot());
        });
        let rest = per_op(1_000_000, || v.restore(std::hint::black_box(&s)));
        let lesson = per_op(10_000, || {
            v.restore(&s);
            v.write("/d0/f7.csv", b"new,content\n").unwrap();
        });
        let first = per_op(1_000, || {
            v.restore(&s);
            v.write_at("/big.bin", 0, b"Q").unwrap(); // appending into the shared 64 MiB — copies the contents
        });
        let t = Instant::now();
        let dump = v.dump();
        let deep = t.elapsed().as_secs_f64();
        std::hint::black_box(dump);
        v.restore(&s);
        println!("{n}\t{mb:.1}\t{}\t{}\t{}\t{}\t{}", fmt(snap), fmt(rest), fmt(lesson), fmt(first), fmt(deep));
    }
}
