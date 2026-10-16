//! Lossless through a real database: init → load → export gives the same bytes, verify is green. Negative
//! controls: a corrupted row in the database and a corrupted file on disk must give red; an overlay annotation
//! (matching existing) lands on the same sentences, and a sentence that is absent is an error. Requires the DuckDB CLI
//! (`~/bin/duckdb` or `DUCKDB`).

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// Everything `en::conllu::read` drops, plus uneven blank lines and a file without a trailing LF.
const GOLD: &str = "\n\n# newdoc id = d1\n# meta::title = Test doc\n# meta::genre = fiction\n# newpar id = p1\n# sent_id = s1\n# text = Don't \"go\" \\home.\n\
1-2\tDon't\t_\t_\t_\t_\t_\t_\t_\tSpaceAfter=No\n\
1\tDo\tdo\tAUX\tVBP\tMood=Ind|Number=Sing|Person=1|Tense=Pres|VerbForm=Fin|Zzz=Q\t3\taux\t3:aux\t_\n\
2\tn't\tnot\tPART\tRB\tPolarity=Neg\t3\tadvmod\t3:advmod\t_\n\
3\tgo\tgo\tVERB\tVB\tVerbForm=Inf\t0\troot\t0:root\t_\n\
3.1\tgo\tgo\tVERB\tVB\t_\t_\t_\t3:conj\tCopyOf=3\n\
4\t\"\t\"\tPUNCT\t``\t_\t5\tpunct\t5:punct\tSpaceAfter=No\n\
5\t\\home\t\\home\tNOUN\tNN\tNumber=Sing\t3\tobj\t3:obj\tSpaceAfter=No\n\
6\t.\t.\tPUNCT\t.\t_\t3\tpunct\t3:punct\t_\n\n\n\
# sent_id = s2\n# text = Grüße\n1\tGrüße\tgrüße\tINTJ\tUH\t_\t0\troot\t0:root\t_";

/// The same annotation with a different tree — an overlay set (like rust-b on EWT test sentences).
const OVERLAY: &str = "# sent_id = s2\n# text = Grüße\n1\tGrüße\tgrüße\tNOUN\tNN\t_\t0\troot\t_\t_\n\n";

fn duckdb() -> PathBuf {
    let p = std::env::var("DUCKDB").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("HOME").unwrap()).join("bin/duckdb"));
    assert!(p.is_file(), "DuckDB CLI not found: {} (variable DUCKDB)", p.display());
    p
}

fn db(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_db")).args(args).env("DUCKDB", duckdb()).output().expect("db")
}

fn ok(o: &Output) -> String {
    assert!(o.status.success(), "stdout: {}\nstderr: {}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout.clone()).unwrap()
}

fn sql(base: &Path, q: &str) {
    let o = Command::new(duckdb()).arg("-bail").arg(base).arg("-c").arg(q).output().unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
}

#[test]
fn lossless_round_trip_and_negative_controls() {
    let dir = std::env::temp_dir().join(format!("mova-db-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let base = dir.join("t.duckdb");
    let gold = dir.join("gold.conllu");
    let over = dir.join("rust-b.conllu");
    std::fs::write(&gold, GOLD).unwrap();
    std::fs::write(&over, OVERLAY).unwrap();
    let b = base.to_str().unwrap();
    let g = gold.to_str().unwrap();
    ok(&db(&["init", b]));
    assert!(!db(&["init", b]).status.success(), "init does not overwrite a database");
    let out = ok(&db(&["load", b, "gold", "ud-2.18", "--kind", "file", "--match", "new", g]));
    assert!(out.contains("blocks 2, new sentences 2"), "{out}");
    // loading the same file again — skipped
    assert!(ok(&db(&["load", b, "gold", "ud-2.18", "--match", "new", g])).contains("skipped"));

    // export — the same bytes (blank lines at the start, two between sentences, no trailing LF)
    let exp = ok(&db(&["export", b, &format!("file={g}")]));
    assert_eq!(exp, GOLD);
    assert!(ok(&db(&["verify", b])).contains("byte for byte 1"));

    // an overlay set lands on an existing sentence; an absent sentence is an error
    let out = ok(&db(&["load", b, "rust", "ud-2.18", "--kind", "file", "--match", "existing", over.to_str().unwrap()]));
    assert!(out.contains("onto existing 1"), "{out}");
    let two = ok(&db(&["export", b, "sent=s2"]));
    assert_eq!(two.matches("# sent_id = s2").count(), 2, "{two}");
    assert_eq!(ok(&db(&["export", b, "sent=s2", "source=rust"])), OVERLAY);
    let stray = dir.join("stray.conllu");
    std::fs::write(&stray, "# sent_id = zzz\n# text = Hm\n1\tHm\thm\tINTJ\tUH\t_\t0\troot\t_\t_\n\n").unwrap();
    let e = db(&["load", b, "opus", "ud-2.18", "--match", "existing", stray.to_str().unwrap()]);
    assert!(!e.status.success() && String::from_utf8_lossy(&e.stderr).contains("is not in the database"));
    // a filter that matched nothing is an error, not an empty file
    assert!(!db(&["export", b, "sent=zzz"]).status.success());
    assert!(!db(&["export", b, "nosuch=1"]).status.success());
    assert!(ok(&db(&["verify", b])).contains("byte for byte 2"));

    // negative control 1: a corrupted row in the database — the export differs, verify is red
    sql(&base, "UPDATE tokens SET form = 'went' WHERE form = 'go' AND id = '3'");
    assert_ne!(ok(&db(&["export", b, &format!("file={g}")])), GOLD);
    let v = db(&["verify", b]);
    assert!(!v.status.success(), "verify must fail on a corrupted row");
    assert!(String::from_utf8_lossy(&v.stdout).contains("FAIL"), "{}", String::from_utf8_lossy(&v.stdout));
    sql(&base, "UPDATE tokens SET form = 'go' WHERE form = 'went' AND id = '3'");
    ok(&db(&["verify", b]));

    // negative control 2: a changed file on disk — verify is red, loading again is an error
    std::fs::write(&gold, GOLD.replace("Grüße\tgrüße\tINTJ", "Grüße\tgrüße\tNOUN")).unwrap();
    assert!(!db(&["verify", b]).status.success());
    assert!(!db(&["load", b, "gold", "ud-2.18", "--match", "new", g]).status.success());
    std::fs::write(&gold, GOLD).unwrap();
    ok(&db(&["verify", b]));

    // the file is removed from disk — the database reproduces the same bytes itself: NOTE, not a failure
    let moved = dir.join("gold.moved");
    std::fs::rename(&gold, &moved).unwrap();
    assert!(ok(&db(&["verify", b])).contains("NOTE"));
    assert_eq!(ok(&db(&["export", b, &format!("file={g}")])), GOLD);
    std::fs::rename(&moved, &gold).unwrap();

    // a document with meta::*, a paragraph, sentence comments; gate: an unknown FEATS pair (Zzz=Q) — not ok
    let st = dir.join("q.sql");
    std::fs::write(
        &st,
        "COPY (SELECT d.title, d.genre, s.para, s.comments->>'newpar id', a.gate FROM sentences s JOIN docs d USING (doc) JOIN annsets a ON a.sid = s.sid AND a.source = 'gold' ORDER BY s.sid) TO '/dev/stdout' (FORMAT csv, DELIMITER '|', HEADER false);",
    )
    .unwrap();
    let q = ok(&db(&["sql", b, st.to_str().unwrap()]));
    assert!(q.contains("Test doc|fiction|1|p1|\"word 1: FEATS"), "{q}");
    assert!(q.contains("Test doc|fiction|1||ok"), "{q}");
    let _ = std::fs::remove_dir_all(&dir);
}
