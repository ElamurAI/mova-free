//! Finite checks with negative controls: each assertion pair shows a broken variant turning red.

use crate::merge::*;
use crate::*;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap()
}

fn tmp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("mova-json-test-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn sh(dir: &Path, args: &[&str]) -> bool {
    Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(["-c", "user.name=t", "-c", "user.email=t@example.invalid", "-c", "commit.gpgsign=false"])
        .args(args)
        .output()
        .unwrap()
        .status
        .success()
}

#[test]
fn real_files_round_trip_byte_identical() {
    let root = repo_root();
    let mut checked = 0;
    for (rel, required) in [
        ("models/contracts.json", true),
        ("models/model-infrastructure.groupant/manifest.json", true),
        ("tools/module-overlay/src/main.rs.groupant/manifest.json", true),
        ("peta/crab/crab.json", false),
        ("peta/crab/deputy.json", false),
    ] {
        let p = root.join(rel);
        if !p.exists() {
            assert!(!required, "{rel} missing");
            continue;
        }
        let text = std::fs::read_to_string(&p).unwrap();
        let d = load_str(&text).unwrap();
        assert!(d.canonical, "{rel}: not reproduced byte for byte ({:?})", d.style);
        assert_eq!(render(&d.value, d.style), text, "{rel}");
        // Negative controls: a wrong indent, a lost trailing newline and a one-byte layout edit are all red.
        let other = match d.style.layout {
            Layout::Pretty(1) => Layout::Pretty(2),
            _ => Layout::Pretty(1),
        };
        assert_ne!(render(&d.value, Style { layout: other, newline: d.style.newline }), text);
        assert_ne!(render(&d.value, Style { layout: d.style.layout, newline: !d.style.newline }), text);
        let i = text.find(": ").unwrap();
        let mutated = format!("{}:  {}", &text[..i], &text[i + 2..]);
        assert!(!load_str(&mutated).unwrap().canonical, "{rel}: layout mutation not detected");
        checked += 1;
    }
    assert!(checked >= 3);
}

#[test]
fn strings_keep_their_source_escapes_and_numbers_their_lexemes() {
    let text = "{\n  \"a\": \"\\u00e9\\/x\",\n  \"b\": 1.50,\n  \"c\": -0e+3,\n  \"d\": \"é\\u001b\"\n}\n";
    let d = load_str(text).unwrap();
    assert!(d.canonical);
    assert_eq!(d.value.field("a").unwrap().as_str(), Some("é/x"));
    // Control: a fresh string with the same text renders in canonical escape, not the source escape.
    let mut v = d.value.clone();
    set(&mut v, &parse_path("a").unwrap(), Value::string("é/x")).unwrap();
    assert!(render(&v, d.style).contains("\"a\": \"é/x\""));
    assert!(parse("{\"a\":01}").is_err());
    assert!(parse("[1,]").is_err());
    assert!(parse("\"\\ud800\"").is_err());
}

#[test]
fn paths_select_and_edit() {
    let text = "{\n \"workers\": [\n  {\n   \"name\": \"a\",\n   \"argv\": [\n    \"x\",\n    \"-a\"\n   ]\n  },\n  {\n   \"name\": \"dolphin-modeler\",\n   \"argv\": [\n    \"run\",\n    \"-a\",\n    \"y\"\n   ]\n  }\n ],\n \"capabilities\": {\n  \"dolphin-groupant-start\": {\n   \"parameters\": {\n    \"worker\": \"w1\"\n   }\n  }\n }\n}\n";
    let d = load_str(text).unwrap();
    assert_eq!(d.style.layout, Layout::Pretty(1));
    let mut v = d.value.clone();
    let p = parse_path("workers[name=dolphin-modeler].argv").unwrap();
    insert_before(&mut v, &p, &Value::string("-a"), vec![Value::string("--add-dir"), Value::string("X")]).unwrap();
    let got: Vec<String> = match get(&v, &p).unwrap() {
        Value::Arr(a) => a.iter().map(|x| x.as_str().unwrap().to_string()).collect(),
        _ => panic!(),
    };
    assert_eq!(got, ["run", "--add-dir", "X", "-a", "y"]);
    set(&mut v, &parse_path("capabilities.dolphin-groupant-start.parameters.worker").unwrap(), Value::string("w2")).unwrap();
    assert_eq!(get(&v, &parse_path("capabilities.dolphin-groupant-start.parameters.worker").unwrap()).unwrap().as_str(), Some("w2"));
    assert_eq!(remove_value(&mut v, &parse_path("workers[0].argv").unwrap(), &Value::string("x")).unwrap(), 1);
    del(&mut v, &parse_path("workers[-1].argv[0]").unwrap()).unwrap();
    // Only the edited lines differ from the source layout.
    let out = render(&v, d.style);
    assert!(out.contains("    \"--add-dir\",\n    \"X\",\n    \"-a\",\n"));
    assert!(out.ends_with("}\n"));
    // Negative controls: an ambiguous selector, a missing anchor, an absent key and a bad path are red.
    let mut w = d.value.clone();
    push(&mut w, &parse_path("workers").unwrap(), vec![parse("{\"name\":\"a\"}").unwrap()]).unwrap();
    assert!(get(&w, &parse_path("workers[name=a]").unwrap()).is_err());
    assert!(insert_before(&mut w, &parse_path("workers[1].argv").unwrap(), &Value::string("-b"), vec![]).is_err());
    assert!(get(&w, &parse_path("capabilities.nope").unwrap()).is_err());
    assert!(remove_value(&mut w, &parse_path("workers[1].argv").unwrap(), &Value::string("zz")).is_err());
    assert!(parse_path("a..b").is_err() && parse_path("a[").is_err() && parse_path("a[x]").is_err());
    assert_eq!(parse_path("a\\.b[k=v\\]w]").unwrap(), vec![Seg::Key("a.b".into()), Seg::Select("k".into(), "v]w".into())]);
}

#[test]
fn new_keys_go_to_sorted_place_only_in_sorted_objects() {
    let mut s = parse("{\"a\":1,\"c\":3}").unwrap();
    set(&mut s, &[Seg::Key("b".into())], Value::Num("2".into())).unwrap();
    assert_eq!(compact(&s), "{\"a\":1,\"b\":2,\"c\":3}");
    let mut u = parse("{\"c\":3,\"a\":1}").unwrap();
    set(&mut u, &[Seg::Key("b".into())], Value::Num("2".into())).unwrap();
    assert_eq!(compact(&u), "{\"c\":3,\"a\":1,\"b\":2}");
}

#[test]
fn replace_str_counts_and_skips_keys() {
    let mut v = parse("{\"r3/\":\"/x/r3/a r3/b\",\"l\":[\"r3/\",1,\"q\"]}").unwrap();
    assert_eq!(replace_str(&mut v, "r3/", "r4/"), (2, 3));
    assert_eq!(compact(&v), "{\"r3/\":\"/x/r4/a r4/b\",\"l\":[\"r4/\",1,\"q\"]}");
    assert_eq!(replace_str(&mut v, "r3/", "r4/"), (0, 0));
}

#[test]
fn jsonl_tallies() {
    let text = "{\"s\":\"ok\",\"n\":1}\n{\"s\":\"bad\"}\n\n{\"s\":\"ok\"}\nnot json\n{\"t\":2}\n";
    let p = parse_path("s").unwrap();
    let (c, invalid) = jsonl_count(text, &p);
    assert_eq!(sorted_counts(&c), vec![(2, "ok"), (1, MISSING), (1, "bad")]);
    assert_eq!(invalid, vec![5]);
    assert_eq!(jsonl_filter(text, &p, "ok").len(), 2);
    assert_eq!(jsonl_filter(text, &parse_path("n").unwrap(), "1"), vec!["{\"s\":\"ok\",\"n\":1}"]);
    assert_eq!(split_assignment("a[k=v].b=x").unwrap(), ("a[k=v].b", "x"));
}

fn m3(b: &str, o: &str, t: &str) -> (Value, Vec<Conflict>) {
    let mut m = Merger::new(None);
    let v = m.merge(Some(&parse(b).unwrap()), &parse(o).unwrap(), &parse(t).unwrap(), "$");
    (v, m.conflicts)
}

#[test]
fn merge3_clean_cases() {
    let (v, c) = m3("{\"a\":1,\"b\":1}", "{\"a\":2,\"b\":1}", "{\"a\":1,\"b\":3,\"c\":4}");
    assert!(c.is_empty());
    assert_eq!(compact(&v), "{\"a\":2,\"b\":3,\"c\":4}");
    // Keyed arrays: ours edits one node, theirs adds another and deletes an unchanged one.
    let b = r#"{"nodes":[{"id":"x","v":1},{"id":"y","v":1},{"id":"z","v":1}]}"#;
    let o = r#"{"nodes":[{"id":"x","v":2},{"id":"y","v":1},{"id":"z","v":1}]}"#;
    let t = r#"{"nodes":[{"id":"x","v":1},{"id":"y","v":1},{"id":"w","v":5}]}"#;
    let (v, c) = m3(b, o, t);
    assert!(c.is_empty(), "{c:?}");
    assert_eq!(compact(&v), r#"{"nodes":[{"id":"x","v":2},{"id":"y","v":1},{"id":"w","v":5}]}"#);
    // Path-sorted bindings: a theirs addition lands at its sorted place.
    let (v, c) = m3(r#"[{"path":"a","sha256":"1"},{"path":"c","sha256":"1"}]"#, r#"[{"path":"a","sha256":"2"},{"path":"c","sha256":"1"}]"#, r#"[{"path":"a","sha256":"1"},{"path":"b","sha256":"1"},{"path":"c","sha256":"1"}]"#);
    assert!(c.is_empty());
    assert_eq!(compact(&v), r#"[{"path":"a","sha256":"2"},{"path":"b","sha256":"1"},{"path":"c","sha256":"1"}]"#);
    // Sequences: separate regions merge; unique appends at one point join.
    let (v, c) = m3(r#"["a","b","c","d"]"#, r#"["A","b","c","d"]"#, r#"["a","b","c","D"]"#);
    assert!(c.is_empty());
    assert_eq!(compact(&v), r#"["A","b","c","D"]"#);
    let (v, c) = m3(r#"["a"]"#, r#"["a","x"]"#, r#"["a","y"]"#);
    assert!(c.is_empty());
    assert_eq!(compact(&v), r#"["a","x","y"]"#);
}

#[test]
fn merge3_true_conflicts_are_red() {
    assert_eq!(m3("{\"a\":1}", "{\"a\":2}", "{\"a\":3}").1.len(), 1);
    assert_eq!(m3("{\"a\":1}", "{}", "{\"a\":3}").1.len(), 1);
    assert_eq!(m3(r#"[{"id":"x","v":1}]"#, r#"[{"id":"x","v":2}]"#, r#"[]"#).1.len(), 1);
    // argv: both add "--add-dir" with different values at one point; joining would repeat "--add-dir".
    let (_, c) = m3(r#"["run","-a"]"#, r#"["run","-a","--add-dir","A"]"#, r#"["run","-a","--add-dir","B"]"#);
    assert_eq!(c.len(), 1, "{c:?}");
    assert_eq!(m3(r#"["a","b"]"#, r#"["a","X"]"#, r#"["a","Y"]"#).1.len(), 1);
    // Negative control: identical changes on both sides are not a conflict.
    assert!(m3("{\"a\":1}", "{\"a\":2}", "{\"a\":2}").1.is_empty());
}

#[test]
fn offset_mapping_follows_unchanged_lines() {
    let old = b"fn a() {}\nfn b() {\n  1\n}\nfn c() {}\n";
    let new = b"// head\nfn a() {}\nfn b() {\n  2\n}\nfn c() {}\n";
    let blocks = byte_blocks(old, new).unwrap();
    let a = (0, 10);
    let (s, e) = (map_offset(&blocks, a.0, false), map_offset(&blocks, a.1, true));
    assert_eq!(&new[s..e], &old[a.0..a.1]);
    let c = (old.len() - 10, old.len());
    let (s, e) = (map_offset(&blocks, c.0, false), map_offset(&blocks, c.1, true));
    assert_eq!(&new[s..e], b"fn c() {}\n");
    // fn b's range covers the changed line: it maps to the new body, whose text differs.
    let b = (10, 25);
    let (s, e) = (map_offset(&blocks, b.0, false), map_offset(&blocks, b.1, true));
    assert_eq!(&new[s..e], b"fn b() {\n  2\n}\n");
    // Negative control: an identity map (no diff) points fn c's range at the wrong bytes.
    assert_ne!(&new[c.0..c.1], b"fn c() {}\n");
}

#[test]
fn rebind_refreshes_and_adds_sorted_bindings() {
    let d = tmp("rebind");
    std::fs::write(d.join("a.rs"), "a").unwrap();
    std::fs::write(d.join("b.rs"), "b").unwrap();
    std::fs::write(d.join("c.rs"), "c").unwrap();
    let m = d.join("manifest.json");
    std::fs::write(&m, "{\n  \"id\": \"g\",\n  \"source_bindings\": [\n    {\n      \"path\": \"a.rs\",\n      \"sha256\": \"stale\"\n    },\n    {\n      \"path\": \"c.rs\",\n      \"sha256\": \"stale\"\n    }\n  ]\n}\n").unwrap();
    let paths = vec!["a.rs".to_string(), "b.rs".to_string()];
    assert_eq!(rebind_manifest(&d, &m, &paths, false, false).unwrap(), 1);
    assert_eq!(rebind_manifest(&d, &m, &paths, true, false).unwrap(), 1);
    let v = load_file(&m).unwrap();
    assert!(v.canonical);
    let bs = get(&v.value, &parse_path("source_bindings").unwrap()).unwrap();
    let order: Vec<&str> = match bs {
        Value::Arr(a) => a.iter().map(|x| x.field("path").unwrap().as_str().unwrap()).collect(),
        _ => panic!(),
    };
    assert_eq!(order, ["a.rs", "b.rs", "c.rs"]);
    assert_eq!(get(&v.value, &parse_path("source_bindings[path=a.rs].sha256").unwrap()).unwrap().as_str(), Some(sha_hex(b"a").as_str()));
    // c.rs was not listed: its stale hash stays (no silent refresh).
    assert_eq!(get(&v.value, &parse_path("source_bindings[path=c.rs].sha256").unwrap()).unwrap().as_str(), Some("stale"));
    // Negative controls: a missing bound file and a non-reproducible layout are refused.
    assert!(rebind_manifest(&d, &m, &["zz.rs".to_string()], true, false).is_err());
    std::fs::write(&m, "{\"id\":\"g\",  \"source_bindings\":[]}\n").unwrap();
    assert!(rebind_manifest(&d, &m, &paths, true, false).is_err());
    std::fs::remove_dir_all(&d).unwrap();
}

#[test]
fn rebind_groupant_maps_fragment_ranges_and_group_bindings() {
    let d = tmp("groupant");
    assert!(sh(&d, &["init", "-q", "-b", "main"]));
    let old = "fn a() {}\nfn b() {\n  1\n}\n";
    std::fs::create_dir_all(d.join("src/x.rs.groupant/frag")).unwrap();
    std::fs::create_dir_all(d.join("models/g.groupant")).unwrap();
    std::fs::write(d.join("src/x.rs"), old).unwrap();
    let h0 = sha_hex(old.as_bytes());
    let frag = format!(
        "{{\n  \"coverage\": {{\n    \"byte_range\": {{\n      \"end\": 25,\n      \"start\": 10\n    }},\n    \"source_bytes_sha256\": \"{}\"\n  }},\n  \"source\": {{\n    \"path\": \"../../x.rs\",\n    \"sha256\": \"{h0}\"\n  }}\n}}\n",
        sha_hex(&old.as_bytes()[10..25])
    );
    std::fs::write(d.join("src/x.rs.groupant/frag/manifest.json"), frag).unwrap();
    // The file-level manifest covers the whole file: a null byte_range.
    std::fs::write(d.join("src/x.rs.groupant/manifest.json"), format!("{{\n  \"coverage\": {{\n    \"byte_range\": null\n  }},\n  \"source\": {{\n    \"path\": \"../x.rs\",\n    \"sha256\": \"{h0}\"\n  }}\n}}\n")).unwrap();
    std::fs::write(d.join("models/g.groupant/manifest.json"), format!("{{\n  \"source_bindings\": [\n    {{\n      \"path\": \"src/x.rs\",\n      \"sha256\": \"{h0}\"\n    }}\n  ]\n}}\n")).unwrap();
    assert!(sh(&d, &["add", "-A"]) && sh(&d, &["commit", "-qm", "base"]));
    let new = "// head\nfn a() {}\nfn b() {\n  1\n}\n";
    std::fs::write(d.join("src/x.rs"), new).unwrap();
    let h1 = sha_hex(new.as_bytes());
    let report = rebind_groupant(&d, "HEAD", "src/x.rs", false).unwrap();
    assert!(report.iter().any(|l| l.contains("[10,25) -> [18,33) same text")), "{report:?}");
    let f = load_file(&d.join("src/x.rs.groupant/frag/manifest.json")).unwrap();
    assert_eq!(get(&f.value, &parse_path("source.sha256").unwrap()).unwrap().as_str(), Some(h1.as_str()));
    assert_eq!(get(&f.value, &parse_path("coverage.source_bytes_sha256").unwrap()).unwrap().as_str(), Some(sha_hex(&new.as_bytes()[18..33]).as_str()));
    let root = load_file(&d.join("src/x.rs.groupant/manifest.json")).unwrap();
    assert_eq!(get(&root.value, &parse_path("source.sha256").unwrap()).unwrap().as_str(), Some(h1.as_str()));
    let g = load_file(&d.join("models/g.groupant/manifest.json")).unwrap();
    assert_eq!(get(&g.value, &parse_path("source_bindings[0].sha256").unwrap()).unwrap().as_str(), Some(h1.as_str()));
    assert!(f.canonical && g.canonical);
    // Negative control: a changed body inside the range is reported as TEXT CHANGED.
    assert!(sh(&d, &["add", "-A"]) && sh(&d, &["commit", "-qm", "head"]));
    std::fs::write(d.join("src/x.rs"), "// head\nfn a() {}\nfn b() {\n  2\n}\n").unwrap();
    let report = rebind_groupant(&d, "HEAD", "src/x.rs", false).unwrap();
    assert!(report.iter().any(|l| l.contains("TEXT CHANGED")), "{report:?}");
    std::fs::remove_dir_all(&d).unwrap();
}

#[test]
fn merge3_file_inside_a_git_merge() {
    let d = tmp("merge3");
    assert!(sh(&d, &["init", "-q", "-b", "main"]));
    std::fs::write(d.join("x.rs"), "x0").unwrap();
    let j = |sha: &str, extra: &str| format!("{{\n \"bind\": {{\n  \"path\": \"x.rs\",\n  \"sha256\": \"{sha}\"\n }},\n \"list\": [\n  \"a\"{extra}\n ]\n}}\n");
    std::fs::write(d.join("m.json"), j("h0", "")).unwrap();
    assert!(sh(&d, &["add", "-A"]) && sh(&d, &["commit", "-qm", "base"]));
    assert!(sh(&d, &["checkout", "-qb", "topic"]));
    std::fs::write(d.join("x.rs"), "x1").unwrap();
    std::fs::write(d.join("m.json"), j("h1", ",\n  \"t\"")).unwrap();
    assert!(sh(&d, &["commit", "-qam", "topic"]));
    assert!(sh(&d, &["checkout", "-q", "main"]));
    std::fs::write(d.join("m.json"), j("h2", ",\n  \"m\"")).unwrap();
    assert!(sh(&d, &["commit", "-qam", "main"]));
    assert!(!sh(&d, &["merge", "-q", "topic"]), "expected a textual conflict");
    let cwd = std::env::current_dir().unwrap();
    std::env::set_current_dir(&d).unwrap();
    let r = merge3_file("m.json", false, false);
    std::env::set_current_dir(cwd).unwrap();
    let r = r.unwrap();
    assert!(r.conflicts.is_empty(), "{:?}", r.conflicts);
    assert_eq!(r.resolved.len(), 1);
    let out = std::fs::read_to_string(d.join("m.json")).unwrap();
    assert_eq!(out, j(&sha_hex(b"x1"), ",\n  \"m\",\n  \"t\""));
    std::fs::remove_dir_all(&d).unwrap();
}
