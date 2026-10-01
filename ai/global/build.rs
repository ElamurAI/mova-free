//! Compiles the global-level seeds (`seeds/**/*.md`) into Rust tables (`OUT_DIR/global.rs`).
//! Blocks: ```principle (a value), ```concept (a concept and its trigger words), ```link (shortcut "from -> to : why"),
//! ```verbs (verb class, effect on quantity, words). Build gate: links only between declared concepts
//! (verb classes are concepts too), no duplicates; a violation fails the build with an explanation.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

fn walk(d: &Path, out: &mut Vec<PathBuf>) {
    let mut es: Vec<_> = std::fs::read_dir(d).unwrap().flatten().map(|e| e.path()).collect();
    es.sort();
    for p in es {
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().is_some_and(|x| x == "md") {
            out.push(p);
        }
    }
}

fn blocks(text: &str) -> Vec<(String, Vec<String>)> {
    let mut out = Vec::new();
    let mut cur: Option<(String, Vec<String>)> = None;
    for l in text.lines() {
        let t = l.trim();
        if let Some(kind) = t.strip_prefix("```") {
            match cur.take() {
                Some(b) => out.push(b),
                None if !kind.is_empty() => cur = Some((kind.to_string(), Vec::new())),
                None => {}
            }
        } else if let Some((_, ls)) = cur.as_mut() {
            if !t.is_empty() {
                ls.push(t.to_string());
            }
        }
    }
    out
}

fn q(s: &str) -> String {
    format!("{s:?}")
}

fn main() {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("seeds");
    println!("cargo:rerun-if-changed={}", root.display());
    let mut files = Vec::new();
    walk(&root, &mut files);
    // ablation (estimating a component's value): GLOBAL_EXCLUDE="substring,substring" — seeds whose path contains a substring are not compiled
    println!("cargo:rerun-if-env-changed=GLOBAL_EXCLUDE");
    if let Ok(ex) = std::env::var("GLOBAL_EXCLUDE") {
        let pats: Vec<&str> = ex.split(',').filter(|x| !x.is_empty()).collect();
        files.retain(|f| !pats.iter().any(|p| f.display().to_string().contains(p)));
    }
    let mut principles = Vec::new(); // (id, name, gist, against, towards, file)
    let mut concepts: BTreeMap<String, (Vec<String>, String)> = BTreeMap::new();
    let mut categories: BTreeSet<String> = BTreeSet::new();
    let mut links: Vec<(String, String, String, String)> = Vec::new();
    let mut verbs: Vec<(String, char, Vec<String>, String)> = Vec::new();
    let mut relations: Vec<(String, Vec<(String, String)>, String)> = Vec::new();
    for f in &files {
        println!("cargo:rerun-if-changed={}", f.display());
        let rel = f.strip_prefix(root.parent().unwrap()).unwrap().display().to_string();
        let text = std::fs::read_to_string(f).unwrap();
        for (kind, ls) in blocks(&text) {
            match kind.as_str() {
                "principle" => {
                    let get = |k: &str| ls.iter().find_map(|l| l.strip_prefix(&format!("{k}:")).map(|v| v.trim().to_string())).unwrap_or_else(|| panic!("{rel}: principle without \"{k}:\""));
                    let words = |k: &str| get(k).split_whitespace().map(|w| w.replace('_', " ")).collect::<Vec<_>>();
                    let rule = ls.iter().find_map(|l| l.strip_prefix("rule:").map(|v| v.trim().to_string())).unwrap_or_default();
                    principles.push((get("id"), get("name"), get("gist"), words("against"), words("towards"), rel.clone(), rule));
                }
                "concept" | "category" => {
                    for l in &ls {
                        let (name, ws) = l.split_once(':').unwrap_or_else(|| panic!("{rel}: concept \"{l}\" without \":\""));
                        let name = name.trim().to_string();
                        if concepts.contains_key(&name) {
                            panic!("{rel}: concept \"{name}\" is already declared in {}", concepts[&name].1);
                        }
                        if kind == "category" {
                            categories.insert(name.clone());
                        }
                        concepts.insert(name, (ws.split_whitespace().map(String::from).collect(), rel.clone()));
                    }
                }
                "link" => {
                    for l in &ls {
                        let (lhs, why) = l.split_once(" : ").unwrap_or_else(|| panic!("{rel}: link \"{l}\" without \" : why\""));
                        let (a, b) = lhs.split_once("->").unwrap_or_else(|| panic!("{rel}: link \"{l}\" without \"->\""));
                        links.push((a.trim().into(), b.trim().into(), why.trim().into(), rel.clone()));
                    }
                }
                "relation" => {
                    for l in &ls {
                        let (name, kv) = l.split_once(':').unwrap_or_else(|| panic!("{rel}: relation \"{l}\" without \":\""));
                        let kvs: Vec<(String, String)> = kv.split_whitespace().map(|x| {
                            let (k, v) = x.split_once('=').unwrap_or_else(|| panic!("{rel}: relation \"{l}\": \"{x}\" without \"=\""));
                            (k.to_string(), v.to_string())
                        }).collect();
                        if relations.iter().any(|(n, _, _): &(String, Vec<(String, String)>, String)| n == name.trim()) {
                            panic!("{rel}: relation \"{}\" is repeated", name.trim());
                        }
                        relations.push((name.trim().to_string(), kvs, rel.clone()));
                    }
                }
                "verbs" => {
                    for l in &ls {
                        let mut it = l.split_whitespace();
                        let class = it.next().unwrap().to_string();
                        let eff = it.next().unwrap_or_else(|| panic!("{rel}: verbs \"{l}\" without an effect (= + - ±)"));
                        let e = match eff { "=" => '=', "+" => '+', "-" => '-', "±" => '±', x => panic!("{rel}: effect \"{x}\" is not one of (= + - ±)") };
                        verbs.push((class, e, it.map(String::from).collect(), rel.clone()));
                    }
                }
                _ => {}
            }
        }
    }
    // verb classes are concepts too (their words are the triggers)
    for (c, _, ws, rel) in &verbs {
        if concepts.contains_key(c) {
            panic!("{rel}: verb class \"{c}\" clashes with a concept from {}", concepts[c].1);
        }
        concepts.insert(c.clone(), (ws.clone(), rel.clone()));
    }
    let names: Vec<&String> = concepts.keys().collect();
    let idx = |n: &str| names.iter().position(|x| x.as_str() == n);
    let mut seen = BTreeSet::new();
    for (a, b, _, rel) in &links {
        for x in [a, b] {
            if idx(x).is_none() {
                panic!("{rel}: link \"{a} -> {b}\": concept \"{x}\" is not declared");
            }
        }
        if !seen.insert((a.clone(), b.clone())) {
            panic!("{rel}: link \"{a} -> {b}\" is repeated");
        }
    }
    let mut ids = BTreeSet::new();
    for p in &principles {
        if !ids.insert(p.0.clone()) {
            panic!("{}: principle \"{}\" is repeated", p.5, p.0);
        }
    }
    let mut w = String::new();
    writeln!(w, "// generated by build.rs from seeds/ — do not edit").unwrap();
    writeln!(w, "pub const PRINCIPLES: &[PrincipleDef] = &[").unwrap();
    for (id, name, gist, ag, tw, rel, rule) in &principles {
        writeln!(w, "    PrincipleDef {{ id: {}, name: {}, gist: {}, against: &[{}], towards: &[{}], file: {}, rule: {} }},", q(id), q(name), q(gist), ag.iter().map(|x| q(x)).collect::<Vec<_>>().join(", "), tw.iter().map(|x| q(x)).collect::<Vec<_>>().join(", "), q(rel), q(rule)).unwrap();
    }
    writeln!(w, "];\npub const CONCEPTS: &[ConceptDef] = &[").unwrap();
    for (n, (ws, rel)) in &concepts {
        writeln!(w, "    ConceptDef {{ name: {}, words: &[{}], file: {}, category: {} }},", q(n), ws.iter().map(|x| q(x)).collect::<Vec<_>>().join(", "), q(rel), categories.contains(n)).unwrap();
    }
    writeln!(w, "];\n/// Relations (```relation): name → key=value (inverse, vec, transitive, order, …).\npub const RELATIONS: &[(&str, &[(&str, &str)])] = &[").unwrap();
    for (n, kvs, _) in &relations {
        writeln!(w, "    ({}, &[{}]),", q(n), kvs.iter().map(|(k, v)| format!("({}, {})", q(k), q(v))).collect::<Vec<_>>().join(", ")).unwrap();
    }
    writeln!(w, "];\npub const LINKS: &[LinkDef] = &[").unwrap();
    for (a, b, why, rel) in &links {
        writeln!(w, "    LinkDef {{ from: {}, to: {}, why: {}, file: {} }},", idx(a).unwrap(), idx(b).unwrap(), q(why), q(rel)).unwrap();
    }
    writeln!(w, "];\n/// Verb class by lemma: (class, effect on quantity).
\npub fn verb_class(lemma: &str) -> Option<(&'static str, char)> {{\n    match lemma {{").unwrap();
    let mut vseen = BTreeSet::new();
    for (c, e, ws, _) in &verbs {
        for x in ws {
            if vseen.insert(x.clone()) {
                writeln!(w, "        {} => Some(({}, {e:?})),", q(x), q(c)).unwrap();
            }
        }
    }
    writeln!(w, "        _ => None,\n    }}\n}}").unwrap();
    std::fs::write(PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("global.rs"), w).unwrap();
}
