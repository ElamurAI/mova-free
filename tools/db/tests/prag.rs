//! The pragmatics layer through a real database: init (schema with migration v2) → load of two sentences → `db prag` gives
//! a run, sets and `pragmatics` rows, the view `prag_labels` sees them; `db migrate` on a fresh database changes
//! nothing. Negative controls: the same run a second time, an unknown sentence, a non-bit in indirect and a foreign
//! header — an error, and nothing is added to the database. Requires the DuckDB CLI (`~/bin/duckdb` or `DUCKDB`).

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const TEXT: &str = "# sent_id = tatoeba:1\n# text = Can you open the window?\n\
1\tCan\tcan\tAUX\tMD\tVerbForm=Fin\t3\taux\t_\t_\n\
2\tyou\tyou\tPRON\tPRP\tCase=Nom|Person=2|PronType=Prs\t3\tnsubj\t_\t_\n\
3\topen\topen\tVERB\tVB\tVerbForm=Inf\t0\troot\t_\t_\n\
4\tthe\tthe\tDET\tDT\tDefinite=Def|PronType=Art\t5\tdet\t_\t_\n\
5\twindow\twindow\tNOUN\tNN\tNumber=Sing\t3\tobj\t_\tSpaceAfter=No\n\
6\t?\t?\tPUNCT\t.\t_\t3\tpunct\t_\t_\n\n\
# sent_id = tatoeba:2\n# text = Hello!\n\
1\tHello\thello\tINTJ\tUH\t_\t0\troot\t_\tSpaceAfter=No\n\
2\t!\t!\tPUNCT\t.\t_\t1\tpunct\t_\t_\n\n";

const HEAD: &str = "sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tmeans\ttrace\n";

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

fn query(base: &Path, q: &str) -> String {
    let o = Command::new(duckdb()).args(["-bail", "-readonly", "-list", "-noheader"]).arg(base).arg("-c").arg(q).output().unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap().trim().to_string()
}

#[test]
fn prag_layer_and_negative_controls() {
    let dir = std::env::temp_dir().join(format!("mova-db-prag-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let base = dir.join("t.duckdb");
    let b = base.to_str().unwrap();
    let text = dir.join("tatoeba.conllu");
    std::fs::write(&text, TEXT).unwrap();
    ok(&db(&["init", b]));
    // v3–v4 (table `qa`, pair replacement) come after v2 — a fresh database is already at the latest version
    assert!(ok(&db(&["migrate", b])).contains("schema v4"));
    ok(&db(&["load", b, "bronze", "ud-2.18", "--docs", "idprefix", "--match", "auto", text.to_str().unwrap()]));

    let good = dir.join("silver.tsv");
    std::fs::write(&good, format!("{HEAD}tatoeba:1\tq\trequest\t1\t0\tnone\tnone\tasks the listener to open the window\t\ntatoeba:2\tintj\tgreet\t0\t0\tpos\tnone\t-\t{{\"act\":{{\"v\":\"greet\"}}}}\n")).unwrap();
    let out = ok(&db(&["prag", b, good.to_str().unwrap(), "--run", "prag-test", "--source", "silver", "--origin", "prag-test/silver"]));
    assert!(out.contains("rows 2"), "{out}");
    assert_eq!(query(&base, "SELECT form || ' ' || act || ' ' || indirect || ' ' || source || ' ' || tool FROM prag_labels ORDER BY sent_id"), "q request true silver opus\nintj greet false silver opus");
    assert_eq!(query(&base, "SELECT count(*) FROM annsets WHERE layer = 'pragmatics'"), "2");
    assert_eq!(query(&base, "SELECT trace->>'$.act.v' FROM pragmatics WHERE trace IS NOT NULL"), "greet");

    // negative controls: nothing is added
    let count = || query(&base, "SELECT (SELECT count(*) FROM pragmatics) || ' ' || (SELECT count(*) FROM annsets) || ' ' || (SELECT count(*) FROM runs)");
    let before = count();
    // the same run a second time
    assert!(!db(&["prag", b, good.to_str().unwrap(), "--run", "prag-test", "--source", "silver"]).status.success());
    // unknown sentence
    let unknown = dir.join("unknown.tsv");
    std::fs::write(&unknown, format!("{HEAD}tatoeba:1\tq\trequest\t1\t0\tnone\tnone\t-\t\ntatoeba:999\tdecl\tassert\t0\t0\tnone\tnone\t-\t\n")).unwrap();
    assert!(!db(&["prag", b, unknown.to_str().unwrap(), "--run", "prag-test-2", "--source", "rust"]).status.success());
    // non-bit in indirect
    let badbit = dir.join("badbit.tsv");
    std::fs::write(&badbit, format!("{HEAD}tatoeba:1\tq\trequest\tyes\t0\tnone\tnone\t-\t\n")).unwrap();
    assert!(!db(&["prag", b, badbit.to_str().unwrap(), "--run", "prag-test-3", "--source", "rust"]).status.success());
    // foreign header
    let badhead = dir.join("badhead.tsv");
    std::fs::write(&badhead, "sent_id\ts_type\tact\n tatoeba:1\tq\trequest\n").unwrap();
    assert!(!db(&["prag", b, badhead.to_str().unwrap(), "--run", "prag-test-4", "--source", "rust"]).status.success());
    assert_eq!(count(), before);

    // SLM (small language model) prediction: it does not write means — NULL, not an empty string; the decision trace goes to trace
    let rust = dir.join("rust.tsv");
    std::fs::write(&rust, format!("{HEAD}tatoeba:1\tq\trequest\t1\t0\tnone\tnone\t\t{{\"act\":{{\"v\":\"request\",\"why\":[[\"w1=can\",1.5]]}}}}\n")).unwrap();
    ok(&db(&["prag", b, rust.to_str().unwrap(), "--run", "prag-test:perceptron", "--source", "rust"]));
    assert_eq!(query(&base, "SELECT (means IS NULL) || ' ' || tool || ' ' || (trace->'act'->'why'->0->>0) FROM prag_labels WHERE source = 'rust'"), "true rust w1=can");
    let _ = std::fs::remove_dir_all(&dir);
}
