//! Training questions through a real database: a schema v2 database without `qa` — `db qa` fails and writes nothing;
//! `db migrate` gives v4; two tales with paragraphs (`# newpar`) → `db qa` gives a run and `qa` rows with sid anchors,
//! the view `qa_pairs` sees them. Negative controls — an error, and nothing is added to the database: the same
//! run a second time, an anchor from another paragraph, from another tale, an unknown sentence, an unknown document, a type outside
//! the set and an empty answer (table CHECK), a foreign header, a repeated question.
//!
//! Replacement (`--replaces`, v4): a new run with a row of the same paragraph and number → `replaces` on the new one,
//! `replaced_by` on the old one, `qa_current` sees only current ones. Negative controls: no such run, no row with
//! that number, row already replaced. Requires the DuckDB CLI.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const SCHEMA: &str = include_str!("../../../db/schema.sql");

fn sent(id: &str, text: &str, newpar: bool, words: &[(&str, &str, &str)]) -> String {
    let mut s = String::new();
    if newpar {
        s.push_str("# newpar\n");
    }
    s.push_str(&format!("# sent_id = {id}\n# text = {text}\n"));
    for (i, (form, upos, rel)) in words.iter().enumerate() {
        let head = if *rel == "root" { 0 } else { 2 };
        s.push_str(&format!("{}\t{form}\t{}\t{upos}\t_\t_\t{head}\t{rel}\t_\t_\n", i + 1, form.to_lowercase()));
    }
    s.push('\n');
    s
}

fn tales() -> String {
    let fox = [("The", "DET", "det"), ("fox", "NOUN", "nsubj"), ("ran", "VERB", "root")];
    let hen = [("The", "DET", "det"), ("hen", "NOUN", "nsubj"), ("hid", "VERB", "root")];
    let night = [("Night", "NOUN", "nsubj"), ("fell", "VERB", "root")];
    [
        sent("gutenberg:1:tale-a:1", "The fox ran", true, &fox),
        sent("gutenberg:1:tale-a:2", "The hen hid", false, &hen),
        sent("gutenberg:1:tale-a:3", "Night fell", true, &night),
        sent("gutenberg:1:tale-b:1", "The hen hid", true, &hen),
    ]
    .concat()
}

const HEAD: &str = "doc_key\tpara\tk\tqtype\tex_or_im\tanchors\tquestion\tanswer\n";

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

fn fails(base: &str, dir: &Path, name: &str, body: &str, run: &str) {
    let f = dir.join(name);
    std::fs::write(&f, body).unwrap();
    let o = db(&["qa", base, f.to_str().unwrap(), "--run", run]);
    assert!(!o.status.success(), "{name}: should have failed, but got: {}", String::from_utf8_lossy(&o.stdout));
    eprintln!("{name}: {}", String::from_utf8_lossy(&o.stderr).trim());
}

#[test]
fn qa_table_loader_and_negative_controls() {
    let dir = std::env::temp_dir().join(format!("mova-db-qa-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let base = dir.join("t.duckdb");
    let b = base.to_str().unwrap();

    // a schema v2 database (everything before migration v3): no `qa` — `db qa` fails, writes nothing; `db migrate` → v4
    let v2 = &SCHEMA[..SCHEMA.find("-- migrate:v3").expect("migration v3 in schema.sql")];
    let o = Command::new(duckdb()).arg("-bail").arg(&base).arg("-c").arg(v2).output().unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    let text = dir.join("tales.conllu");
    std::fs::write(&text, tales()).unwrap();
    ok(&db(&["load", b, "bronze", "ud-2.18", "--docs", "idprefix", "--match", "auto", text.to_str().unwrap()]));
    assert_eq!(query(&base, "SELECT string_agg(sent_id || '/' || para, ' ' ORDER BY sid) FROM sentences"), "gutenberg:1:tale-a:1/1 gutenberg:1:tale-a:2/1 gutenberg:1:tale-a:3/2 gutenberg:1:tale-b:1/1");
    let good = format!(
        "{HEAD}gutenberg:1:tale-a\t1\t1\tcharacter\texplicit\tgutenberg:1:tale-a:1\tWho ran?\tThe fox.\n\
         gutenberg:1:tale-a\t1\t2\tcausal relationship\timplicit\tgutenberg:1:tale-a:1,gutenberg:1:tale-a:2\tWhy did the hen hide?\tShe was afraid of the fox.\n\
         gutenberg:1:tale-a\t2\t1\tsetting\texplicit\tgutenberg:1:tale-a:3\tWhen did it happen?\tAt nightfall.\n"
    );
    let runs_v2 = query(&base, "SELECT count(*) FROM runs");
    fails(b, &dir, "good-v2.tsv", &good, "qa-test");
    assert_eq!(query(&base, "SELECT count(*) FROM runs"), runs_v2);
    assert!(ok(&db(&["migrate", b])).contains("schema v4"));
    assert!(ok(&db(&["migrate", b])).contains("schema v4"), "migration is idempotent");

    let goodf = dir.join("good.tsv");
    std::fs::write(&goodf, &good).unwrap();
    let out = ok(&db(&["qa", b, goodf.to_str().unwrap(), "--run", "qa-test", "--path", "/runs/qa-test"]));
    assert!(out.contains("rows 3"), "{out}");
    assert_eq!(
        query(&base, "SELECT string_agg(doc_key || ' ' || para || '.' || k || ' ' || qtype || ' ' || ex_or_im || ' ' || source || ' ' || run_name, ' | ' ORDER BY qid) FROM qa_pairs"),
        "gutenberg:1:tale-a 1.1 character explicit opus qa-test | gutenberg:1:tale-a 1.2 causal relationship implicit opus qa-test | gutenberg:1:tale-a 2.1 setting explicit opus qa-test"
    );
    // anchors — sids of this paragraph's sentences
    assert_eq!(
        query(&base, "SELECT string_agg(s.sent_id, ' ' ORDER BY s.sid) FROM qa q, unnest(CAST(q.anchors AS INTEGER[])) AS a(sid) JOIN sentences s ON s.sid = a.sid WHERE q.k = 2"),
        "gutenberg:1:tale-a:1 gutenberg:1:tale-a:2"
    );
    assert_eq!(query(&base, "SELECT kind || ' ' || tool || ' ' || path FROM runs WHERE name = 'qa-test'"), "qa qagen opus /runs/qa-test");

    // negative controls: nothing is added
    let count = || query(&base, "SELECT (SELECT count(*) FROM qa) || ' ' || (SELECT count(*) FROM runs)");
    let before = count();
    let row = |anchors: &str, qtype: &str, answer: &str| format!("{HEAD}gutenberg:1:tale-a\t1\t1\t{qtype}\texplicit\t{anchors}\tWho ran?\t{answer}\n");
    // the same run a second time
    fails(b, &dir, "again.tsv", &good, "qa-test");
    // anchor from another paragraph of the same tale
    fails(b, &dir, "para.tsv", &row("gutenberg:1:tale-a:3", "character", "The fox."), "qa-t1");
    // anchor from another tale
    fails(b, &dir, "doc.tsv", &row("gutenberg:1:tale-b:1", "character", "The fox."), "qa-t2");
    // unknown sentence
    fails(b, &dir, "unknown.tsv", &row("gutenberg:1:tale-a:1,gutenberg:1:tale-a:99", "character", "The fox."), "qa-t3");
    // unknown document
    fails(b, &dir, "nodoc.tsv", &format!("{HEAD}gutenberg:1:tale-z\t1\t1\tcharacter\texplicit\tgutenberg:1:tale-a:1\tWho ran?\tThe fox.\n"), "qa-t4");
    // type outside the set — table CHECK
    fails(b, &dir, "type.tsv", &row("gutenberg:1:tale-a:1", "causal", "The fox."), "qa-t5");
    // empty answer — table CHECK
    fails(b, &dir, "empty.tsv", &row("gutenberg:1:tale-a:1", "character", " "), "qa-t6");
    // foreign header
    fails(b, &dir, "head.tsv", "doc_key\tpara\tquestion\tanswer\ngutenberg:1:tale-a\t1\tWho ran?\tThe fox.\n", "qa-t7");
    // repeat of (document, paragraph, number)
    fails(b, &dir, "dup.tsv", &format!("{good}gutenberg:1:tale-a\t1\t1\taction\texplicit\tgutenberg:1:tale-a:1\tWhat did the fox do?\tIt ran.\n"), "qa-t8");
    // no anchors
    fails(b, &dir, "noanchor.tsv", &row("", "character", "The fox."), "qa-t9");
    assert_eq!(count(), before);

    // replacement: question 1.1 of run qa-test — as a verbatim span; the old row stays, marked
    assert_eq!(query(&base, "SELECT count(*) FROM qa WHERE replaces IS NOT NULL OR replaced_by IS NOT NULL"), "0");
    let fix = |k: &str, answer: &str| format!("{HEAD}gutenberg:1:tale-a\t1\t{k}\tcharacter\texplicit\tgutenberg:1:tale-a:1\tWho ran?\t{answer}\n");
    let (n0, r0) = (query(&base, "SELECT count(*) FROM qa"), query(&base, "SELECT count(*) FROM runs"));
    // negative controls: no such run, no row with that number (1.3) — nothing is added
    let o = db(&["qa", b, write(&dir, "fix0.tsv", &fix("1", "The fox")).as_str(), "--run", "qa-fix0", "--replaces", "qa-nope"]);
    assert!(!o.status.success() && String::from_utf8_lossy(&o.stderr).contains("run \"qa-nope\" does not exist"));
    let o = db(&["qa", b, write(&dir, "fix1.tsv", &fix("3", "The fox")).as_str(), "--run", "qa-fix1", "--replaces", "qa-test"]);
    assert!(!o.status.success() && String::from_utf8_lossy(&o.stderr).contains("has no such row"));
    assert_eq!((query(&base, "SELECT count(*) FROM qa"), query(&base, "SELECT count(*) FROM runs")), (n0.clone(), r0.clone()));
    let out = ok(&db(&["qa", b, write(&dir, "fix.tsv", &fix("1", "The fox")).as_str(), "--run", "qa-fix", "--replaces", "qa-test"]));
    assert!(out.contains("rows 1, each replaces a row of run qa-test"), "{out}");
    assert_eq!(
        query(&base, "SELECT o.qid || ' ' || o.answer || ' -> ' || n.qid || ' ' || n.answer FROM qa o JOIN qa n ON n.qid = o.replaced_by AND n.replaces = o.qid"),
        "1 The fox. -> 4 The fox"
    );
    assert_eq!(query(&base, "SELECT string_agg(qid || ' ' || run_name, ' | ' ORDER BY qid) FROM qa_current"), "2 qa-test | 3 qa-test | 4 qa-fix");
    assert_eq!(query(&base, "SELECT count(*) FROM qa_pairs"), "4");
    // row already replaced — not allowed a second time
    let o = db(&["qa", b, write(&dir, "fix2.tsv", &fix("1", "fox")).as_str(), "--run", "qa-fix2", "--replaces", "qa-test"]);
    assert!(!o.status.success() && String::from_utf8_lossy(&o.stderr).contains("already replaced"));
    assert_eq!(query(&base, "SELECT (SELECT count(*) FROM qa) || ' ' || (SELECT count(*) FROM runs)"), format!("{} {}", n0.parse::<i64>().unwrap() + 1, r0.parse::<i64>().unwrap() + 1));
    let _ = std::fs::remove_dir_all(&dir);
}

fn write(dir: &Path, name: &str, body: &str) -> String {
    let f = dir.join(name);
    std::fs::write(&f, body).unwrap();
    f.to_str().unwrap().to_string()
}
