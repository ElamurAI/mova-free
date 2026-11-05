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
    for f in ["data/lemmas.tsv", "data/forms.tsv", "data/consts.txt", "data/lexicon-ext.tsv", "data/acronyms.tsv", "src/hash.rs"] {
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

    // the lexicon extension (`lexext.rs`): a separate blob, so the dictionary's fingerprint, and with it every
    // trained model, stays as it is
    let ext_txt = read("lexicon-ext.tsv");
    let acr_txt = read("acronyms.tsv");
    let (ext, ext_layout) = extension(&ext_txt, &acr_txt);
    std::fs::write(out.join("lexext.bin"), ext).expect("lexext.bin");
    std::fs::write(out.join("lexext_layout.rs"), ext_layout).expect("lexext layout");
}

/// Source letters of the extension → bits: OEWN, SPECIALIST, Census surnames, Mova's own, the Names Corpus.
fn ext_src(s: &str) -> u8 {
    s.split(',').map(|x| match x.chars().next() {
        Some('o') => 1,
        Some('s') => 2,
        Some('c') => 4,
        Some('m') => 8,
        Some('k') => 16,
        _ => 0,
    }).fold(0, |a, b| a | b)
}

/// Strings section: `<P>_STR` bytes, `<P>_OFF` offsets (n+1), `N_<P>`; when `indexed`, also the open-addressing
/// index `<P>_IDX` and `<P>_MASK`.
fn strings(blob: &mut Vec<u8>, layout: &mut String, p: &str, keys: &[&str], indexed: bool) {
    sec(&format!("{p}_STR"), blob, layout);
    let base = blob.len() as u32;
    let mut offs = vec![0u32];
    for k in keys {
        blob.extend_from_slice(k.as_bytes());
        offs.push(blob.len() as u32 - base);
    }
    pad4(blob);
    sec(&format!("{p}_OFF"), blob, layout);
    for o in offs {
        put32(blob, o);
    }
    writeln!(layout, "pub const N_{p}: usize = {};", keys.len()).unwrap();
    if !indexed {
        return;
    }
    let ix = index(keys);
    sec(&format!("{p}_IDX"), blob, layout);
    for s in &ix {
        put32(blob, *s);
    }
    writeln!(layout, "const {p}_MASK: usize = {};", ix.len() - 1).unwrap();
}

/// Groups section: `<P>_GOFF` (n+1 record offsets) and `<P>_REC` (8-byte records: tag, src, 0, 0, id).
fn groups(blob: &mut Vec<u8>, layout: &mut String, p: &str, g: &[Vec<(u8, u8, u32)>]) {
    sec(&format!("{p}_GOFF"), blob, layout);
    let mut n = 0u32;
    put32(blob, 0);
    for x in g {
        n += x.len() as u32;
        put32(blob, n);
    }
    sec(&format!("{p}_REC"), blob, layout);
    for x in g {
        for &(tag, src, id) in x {
            blob.extend_from_slice(&[tag, src, 0, 0]);
            put32(blob, id);
        }
    }
}

/// `lexicon-ext.tsv` (form, tag, lemma or `=`, sources) and `acronyms.tsv` (acronym, expansion, source) → blob and
/// layout: forms (XF) with their analyses (tag, sources, lemma), lemmas (XL), lower-case lemmas (XP) with their
/// paradigms (tag, form), acronyms (XA) with their expansions (XE: «expansion\tsource»).
fn extension(ext: &str, acr: &str) -> (Vec<u8>, String) {
    let mut forms: Vec<&str> = Vec::new();
    let mut fid: HashMap<&str, u32> = HashMap::new();
    let mut lemmas: Vec<&str> = Vec::new();
    let mut lid: HashMap<&str, u32> = HashMap::new();
    let mut plow: Vec<String> = Vec::new();
    let mut pid: HashMap<String, u32> = HashMap::new();
    let mut ana: Vec<Vec<(u8, u8, u32)>> = Vec::new();
    let mut par: Vec<Vec<(u8, u8, u32)>> = Vec::new();
    for line in ext.lines().filter(|l| !l.starts_with('#') && !l.is_empty()) {
        let c: Vec<&str> = line.split('\t').collect();
        assert!(c.len() >= 4, "lexicon-ext.tsv: bad line {line}");
        let tag = TAGS.iter().position(|t| *t == c[1]).unwrap_or_else(|| panic!("lexicon-ext.tsv: unknown tag {}", c[1])) as u8;
        let lemma = if c[2] == "=" { c[0] } else { c[2] };
        let src = ext_src(c[3]);
        let f = *fid.entry(c[0]).or_insert_with(|| {
            forms.push(c[0]);
            ana.push(Vec::new());
            forms.len() as u32 - 1
        });
        let l = *lid.entry(lemma).or_insert_with(|| {
            lemmas.push(lemma);
            lemmas.len() as u32 - 1
        });
        ana[f as usize].push((tag, src, l));
        let low = lemma.to_lowercase();
        let p = match pid.get(&low) {
            Some(&p) => p,
            None => {
                plow.push(low.clone());
                par.push(Vec::new());
                pid.insert(low, plow.len() as u32 - 1);
                plow.len() as u32 - 1
            }
        };
        par[p as usize].push((tag, src, f));
    }
    let mut akeys: Vec<&str> = Vec::new();
    let mut aid: HashMap<&str, u32> = HashMap::new();
    let mut exps: Vec<String> = Vec::new();
    let mut ag: Vec<Vec<(u8, u8, u32)>> = Vec::new();
    for line in acr.lines().filter(|l| !l.starts_with('#') && !l.is_empty()) {
        let c: Vec<&str> = line.split('\t').collect();
        assert!(c.len() >= 3, "acronyms.tsv: bad line {line}");
        let a = *aid.entry(c[0]).or_insert_with(|| {
            akeys.push(c[0]);
            ag.push(Vec::new());
            akeys.len() as u32 - 1
        });
        exps.push(format!("{}\t{}", c[1], c[2]));
        ag[a as usize].push((0, ext_src(c[2]), exps.len() as u32 - 1));
    }
    let mut blob = Vec::new();
    let mut layout = String::new();
    strings(&mut blob, &mut layout, "XF", &forms, true);
    strings(&mut blob, &mut layout, "XL", &lemmas, false);
    let pl: Vec<&str> = plow.iter().map(String::as_str).collect();
    strings(&mut blob, &mut layout, "XP", &pl, true);
    strings(&mut blob, &mut layout, "XA", &akeys, true);
    let ex: Vec<&str> = exps.iter().map(String::as_str).collect();
    strings(&mut blob, &mut layout, "XE", &ex, false);
    groups(&mut blob, &mut layout, "XF", &ana);
    groups(&mut blob, &mut layout, "XP", &par);
    groups(&mut blob, &mut layout, "XA", &ag);
    (blob, layout)
}

/// Start of a blob section — a constant in layout.
fn sec(name: &str, blob: &[u8], layout: &mut String) {
    writeln!(layout, "const {name}: usize = {};", blob.len()).unwrap();
}

#[cfg(test)]
mod wave27 {
    use super::*;
    #[test]
    fn index_and_record_layout() {
        assert_eq!(index(&[]), [0, 0]);
        let mut s: Vec<_> = index(&["alpha", "beta", "alpha"])
            .into_iter()
            .filter(|&x| x != 0)
            .collect();
        s.sort();
        assert_eq!(s, [1, 2, 3]);
        let mut b = Vec::new();
        put32(&mut b, 0x12345678);
        b.push(9);
        pad4(&mut b);
        assert_eq!(b, [0x78, 0x56, 0x34, 0x12, 9, 0, 0, 0]);
        assert_eq!(src_bits("uua?rb"), 15);
        assert_eq!(src_bits("xyz"), 0);
    }
}
