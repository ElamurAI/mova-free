//! Compiling the dictionary into code: `data/lemmas.tsv` and `data/forms.tsv` → static hash tables
//! (`lexicon.bin`, open addressing, the same hash as at run time) and word constants
//! (`words.rs`: `pub const THE: Sym = …`). Without dictionary files — empty tables (first build).

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;

#[allow(dead_code)]
#[path = "src/hash.rs"]
mod hash;

/// Tags in the order of enum `Tag` (gram.rs) — the tag number in records.
const TAGS: &[&str] = &[
    "CC", "CD", "DT", "EX", "FW", "IN", "JJ", "JJR", "JJS", "LS", "MD", "NN", "NNS", "NNP", "NNPS", "PDT", "POS", "PRP", "PRP$", "RB", "RBR",
    "RBS", "RP", "SYM", "TO", "UH", "VB", "VBD", "VBG", "VBN", "VBP", "VBZ", "WDT", "WP", "WP$", "WRB", ",", ".", ":", "-LRB-", "-RRB-", "``",
    "''", "#", "$", "HYPH", "NFP", "ADD", "AFX", "GW", "XX", "\"", "X",
];

/// How many of the most frequent lemmas get a constant automatically (the rest — via `data/consts.txt`).
const TOP: usize = 5000;

/// Constant names for punctuation.
const PUNCT: &[(&str, &str)] = &[
    (",", "COMMA"),
    (".", "STOP"),
    ("?", "QMARK"),
    ("!", "EXCL"),
    (":", "COLON"),
    (";", "SEMI"),
    ("'", "APOS"),
    ("\"", "QUOTE"),
    ("(", "LPAR"),
    (")", "RPAR"),
    ("-", "HYPHEN"),
    ("--", "DASH"),
    ("$", "DOLLAR"),
    ("%", "PERCENT"),
    ("&", "AMP"),
    ("/", "SLASH"),
    ("...", "ELLIPSIS"),
    ("n't", "NOT_CLITIC"),
    ("'s", "S_CLITIC"),
];

fn put32(b: &mut Vec<u8>, x: u32) {
    b.extend_from_slice(&x.to_le_bytes());
}

fn pad4(b: &mut Vec<u8>) {
    while b.len() % 4 != 0 {
        b.push(0);
    }
}

/// Open-addressing table: slots hold number+1 (0 — empty), size — a power of two ≥ 2n.
fn index(keys: &[&str]) -> Vec<u32> {
    let m = (keys.len() * 2).next_power_of_two().max(2);
    let mut slots = vec![0u32; m];
    for (i, k) in keys.iter().enumerate() {
        let mut s = hash::str_key(k) as usize & (m - 1);
        while slots[s] != 0 {
            s = (s + 1) & (m - 1);
        }
        slots[s] = i as u32 + 1;
    }
    slots
}

struct Rec {
    tag: u8,
    src: u8,
    id: u32,
    count: u32,
}

fn src_bits(s: &str) -> u8 {
    s.chars().map(|c| match c {
        'u' => 1,
        'a' => 2,
        'r' => 4,
        'b' => 8,
        _ => 0,
    }).fold(0, |a, b| a | b)
}

fn main() {
    let data = Path::new("data");
    for f in ["data/lemmas.tsv", "data/forms.tsv", "data/consts.txt", "src/hash.rs"] {
        println!("cargo:rerun-if-changed={f}");
    }
    let read = |p: &str| std::fs::read_to_string(data.join(p)).unwrap_or_default();
    let lemma_txt = read("lemmas.tsv");
    let forms_txt = read("forms.tsv");
    let consts_txt = read("consts.txt");

    // lemmas
    let mut lemmas: Vec<&str> = Vec::new();
    let mut flags: Vec<u8> = Vec::new();
    for line in lemma_txt.lines() {
        let mut it = line.split('\t');
        lemmas.push(it.next().unwrap_or(""));
        flags.push(u8::from(it.next() == Some("k")));
    }
    let lid: HashMap<&str, u32> = lemmas.iter().enumerate().map(|(i, s)| (*s, i as u32)).collect();
    let lower: Vec<u32> = lemmas.iter().map(|s| lid.get(s.to_lowercase().as_str()).copied().unwrap_or(u32::MAX)).collect();

    // forms, analyses (form → records) and paradigms (lemma → records)
    let mut forms: Vec<&str> = Vec::new();
    let mut fid: HashMap<&str, u32> = HashMap::new();
    let mut ana: Vec<Vec<Rec>> = Vec::new();
    let mut par: Vec<Vec<Rec>> = (0..lemmas.len()).map(|_| Vec::new()).collect();
    for line in forms_txt.lines() {
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 5 {
            continue;
        }
        let tag = TAGS.iter().position(|t| *t == c[1]).unwrap_or_else(|| panic!("forms.tsv: unknown tag {}", c[1])) as u8;
        let lemma: u32 = c[2].parse().expect("lemma number");
        let count: u32 = c[3].parse().expect("frequency");
        let src = src_bits(c[4]);
        let f = *fid.entry(c[0]).or_insert_with(|| {
            forms.push(c[0]);
            ana.push(Vec::new());
            forms.len() as u32 - 1
        });
        ana[f as usize].push(Rec { tag, src, id: lemma, count });
        // paradigm: UD — under the lowercased lemma, identical (tag, form) are merged; AGID — as is
        if src & 1 != 0 {
            let l = lower[lemma as usize];
            if l != u32::MAX {
                let p = &mut par[l as usize];
                match p.iter_mut().find(|r| r.src & 1 != 0 && r.tag == tag && r.id == f) {
                    Some(r) => r.count += count,
                    None => p.push(Rec { tag, src: 1, id: f, count }),
                }
            }
        }
        if src & 2 != 0 {
            par[lemma as usize].push(Rec { tag, src: 2, id: f, count: 0 });
        }
    }

    // assembling the blob
    let mut blob: Vec<u8> = Vec::new();
    let mut layout = String::new();
    sec("L_STR", &blob, &mut layout);
    let mut offs = vec![0u32];
    for l in &lemmas {
        blob.extend_from_slice(l.as_bytes());
        offs.push((blob.len()) as u32);
    }
    pad4(&mut blob);
    sec("L_OFF", &blob, &mut layout);
    for o in offs {
        put32(&mut blob, o);
    }
    sec("L_LOWER", &blob, &mut layout);
    for &l in &lower {
        put32(&mut blob, l);
    }
    sec("L_FLAGS", &blob, &mut layout);
    blob.extend_from_slice(&flags);
    pad4(&mut blob);
    let li = index(&lemmas);
    sec("L_IDX", &blob, &mut layout);
    for s in &li {
        put32(&mut blob, *s);
    }
    writeln!(layout, "const L_MASK: usize = {};", li.len() - 1).unwrap();
    sec("F_STR", &blob, &mut layout);
    let base = blob.len() as u32;
    let mut offs = vec![0u32];
    for f in &forms {
        blob.extend_from_slice(f.as_bytes());
        offs.push(blob.len() as u32 - base);
    }
    pad4(&mut blob);
    sec("F_OFF", &blob, &mut layout);
    for o in offs {
        put32(&mut blob, o);
    }
    let fi = index(&forms);
    sec("F_IDX", &blob, &mut layout);
    for s in &fi {
        put32(&mut blob, *s);
    }
    writeln!(layout, "const F_MASK: usize = {};", fi.len() - 1).unwrap();
    for (name, groups) in [("A", &ana), ("P", &par)] {
        sec(&format!("{name}_OFF"), &blob, &mut layout);
        let mut n = 0u32;
        put32(&mut blob, 0);
        for g in groups.iter() {
            n += g.len() as u32;
            put32(&mut blob, n);
        }
        sec(&format!("{name}_REC"), &blob, &mut layout);
        for g in groups.iter() {
            for r in g {
                blob.push(r.tag);
                blob.push(r.src);
                blob.extend_from_slice(&[0, 0]);
                put32(&mut blob, r.id);
                put32(&mut blob, r.count);
            }
        }
    }
    writeln!(layout, "/// Tag names in record-number order (checked against enum Tag in a test).\n#[allow(dead_code)]\nconst TAG_NAMES: &[&str] = &{TAGS:?};").unwrap();
    writeln!(layout, "/// Lemmas in the heap.\npub const N_LEMMAS: usize = {};\n/// Distinct forms.\npub const N_FORMS: usize = {};", lemmas.len(), forms.len()).unwrap();

    // word constants
    let mut words = String::new();
    let mut names: HashMap<String, u32> = HashMap::new();
    let mut add = |name: String, id: u32, lemma: &str, words: &mut String| {
        if names.contains_key(&name) {
            return;
        }
        names.insert(name.clone(), id);
        writeln!(words, "/// «{}»\npub const {name}: Sym = Sym::global({id});", lemma.replace('*', "\\*")).unwrap();
    };
    for (p, name) in PUNCT {
        if let Some(&id) = lid.get(p) {
            add(name.to_string(), id, p, &mut words);
        }
    }
    let ident = |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase());
    for (i, l) in lemmas.iter().enumerate().take(TOP) {
        if ident(l) {
            add(l.to_ascii_uppercase(), i as u32, l, &mut words);
        }
    }
    for w in consts_txt.lines().map(str::trim).filter(|w| !w.is_empty() && !w.starts_with('#')) {
        match lid.get(w) {
            Some(&id) if ident(w) => add(w.to_ascii_uppercase(), id, w, &mut words),
            _ => panic!("data/consts.txt: '{w}' is not in the lemma heap or is not a word [a-z]+"),
        }
    }

    // number → constant name (for explanations: "the" — w::THE = 0)
    let mut named: Vec<(&u32, &String)> = names.iter().map(|(n, id)| (id, n)).collect();
    named.sort();
    writeln!(words, "/// Lemma number → constant name, sorted by number.\npub const NAMES: &[(u32, &str)] = &[").unwrap();
    for (id, n) in named {
        writeln!(words, "    ({id}, \"{n}\"),").unwrap();
    }
    writeln!(words, "];").unwrap();

    let out = std::env::var("OUT_DIR").expect("OUT_DIR");
    let out = Path::new(&out);
    std::fs::write(out.join("lexicon.bin"), &blob).expect("lexicon.bin");
    std::fs::write(out.join("lexicon_layout.rs"), layout).expect("layout");
    std::fs::write(out.join("words.rs"), words).expect("words.rs");
}

/// Start of a blob section — a constant in layout.
fn sec(name: &str, blob: &[u8], layout: &mut String) {
    writeln!(layout, "const {name}: usize = {};", blob.len()).unwrap();
}
