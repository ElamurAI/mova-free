//! Wikidata lexemes (CC0) → TSV (zstd): lemmas, forms with grammatical features, senses with glosses,
//! sense links — item (P5137 etc.), translation (P5972), synonym (P5973), antonym (P5974), synset
//! WordNet (P8814).
//!
//! The lexeme dump is ours (`$MOVA_DATA/raw/wikidata/20260923/`), unpacked by `lbzip2 -dc -n K`, parsed in a single
//! stream: 1.7 million lexemes, a matter of minutes.

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use anyhow::{bail, Context, Result};
use serde::de::{Deserializer, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

use crate::wdx::{num_id, rank_code, thousands, Stmt, Terms, Tsv, Val};

/// Sense or form id: `L123-S2` → (123, 2).
pub fn sub_id(s: &str, letter: u8) -> Option<(u32, u32)> {
    let (l, rest) = s.split_once('-')?;
    Some((num_id(l, b'L')?, num_id(rest, letter)?))
}

/// Claims of any properties, split by value type.
#[derive(Default)]
struct AnyClaims<'a> {
    /// (P, Q, rank)
    items: Vec<(u32, u32, u8)>,
    /// (P, lexeme, sense, rank) — a value that references another lexeme sense
    senses: Vec<(u32, u32, u32, u8)>,
    /// (P, string, rank) — only `STR_PROPS`
    strs: Vec<(u32, Cow<'a, str>, u8)>,
}

/// String properties of senses that we take: P8814 — WordNet 3.1 synset.
const STR_PROPS: &[u32] = &[8814];

impl<'de: 'a, 'a> Deserialize<'de> for AnyClaims<'a> {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V<'a>(PhantomData<&'a ()>);
        impl<'de: 'a, 'a> Visitor<'de> for V<'a> {
            type Value = AnyClaims<'a>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("claims")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Self::Value, A::Error> {
                while a.next_element::<IgnoredAny>()?.is_some() {}
                Ok(AnyClaims::default())
            }
            fn visit_map<A: MapAccess<'de>>(self, mut m: A) -> Result<Self::Value, A::Error> {
                let mut c = AnyClaims::default();
                while let Some(k) = m.next_key::<&'a str>()? {
                    let Some(p) = num_id(k, b'P') else {
                        m.next_value::<IgnoredAny>()?;
                        continue;
                    };
                    for s in m.next_value::<Vec<Stmt<'a>>>()? {
                        let r = rank_code(&s.rank);
                        let Some(dv) = s.mainsnak.datavalue else { continue };
                        match dv.value {
                            Val::Obj(o) => {
                                let Some(id) = o.id.as_deref() else { continue };
                                if let Some(q) = num_id(id, b'Q') {
                                    c.items.push((p, q, r));
                                } else if let Some((l, sn)) = sub_id(id, b'S') {
                                    c.senses.push((p, l, sn, r));
                                }
                            }
                            Val::Str(x) if STR_PROPS.contains(&p) => c.strs.push((p, x, r)),
                            _ => {}
                        }
                    }
                }
                Ok(c)
            }
        }
        d.deserialize_any(V(PhantomData))
    }
}

#[derive(Deserialize)]
struct Form<'a> {
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow, default)]
    representations: Terms<'a>,
    #[serde(rename = "grammaticalFeatures", borrow, default)]
    feats: Vec<Cow<'a, str>>,
}

#[derive(Deserialize)]
struct Sense<'a> {
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow, default)]
    glosses: Terms<'a>,
    #[serde(borrow, default)]
    claims: AnyClaims<'a>,
}

#[derive(Deserialize)]
struct Lex<'a> {
    #[serde(rename = "type", borrow)]
    typ: Cow<'a, str>,
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow, default)]
    lemmas: Terms<'a>,
    #[serde(rename = "lexicalCategory", borrow, default)]
    cat: Cow<'a, str>,
    #[serde(borrow, default)]
    language: Cow<'a, str>,
    #[serde(borrow, default)]
    claims: AnyClaims<'a>,
    #[serde(borrow, default)]
    forms: Vec<Form<'a>>,
    #[serde(borrow, default)]
    senses: Vec<Sense<'a>>,
}

/// Lexeme tables and their columns — checked against `sql/wikidata-load.sql` by a test.
#[cfg_attr(not(test), allow(dead_code))]
pub const LEX_COLUMNS: &[(&str, &str)] = &[
    ("lexemes", "l, lang, cat, lemma, lemma_code, n_forms, n_senses"),
    ("lemmas", "l, code, lemma"),
    ("forms", "l, f, code, rep, feats"),
    ("glosses", "l, s, code, gloss"),
    ("sense_items", "l, s, p, q, rank"),
    ("sense_links", "l, s, p, l2, s2, rank"),
    ("sense_strs", "l, s, p, value, rank"),
    ("lex_claims", "l, p, q, rank"),
];

#[derive(Default)]
pub struct LexStats {
    pub lexemes: u64,
    pub bad: u64,
    pub bad_examples: Vec<String>,
    pub lemmas: u64,
    pub forms: u64,
    pub form_reps: u64,
    pub senses: u64,
    pub glosses: u64,
    pub sense_items: HashMap<u32, u64>,
    pub sense_links: HashMap<u32, u64>,
    pub sense_strs: u64,
    pub lex_claims: u64,
    /// lexeme language (Q) → (lexemes, forms, senses)
    pub by_lang: HashMap<u32, [u64; 3]>,
    pub wall: f64,
}

struct LexOuts {
    lexemes: Tsv,
    lemmas: Tsv,
    forms: Tsv,
    glosses: Tsv,
    sense_items: Tsv,
    sense_links: Tsv,
    sense_strs: Tsv,
    lex_claims: Tsv,
}

impl LexOuts {
    fn create(dir: &Path, level: i32) -> Result<LexOuts> {
        let f = |t: &str| Tsv::create(dir.join(format!("lex-{t}.tsv.zst")), level);
        Ok(LexOuts {
            lexemes: f("lexemes")?,
            lemmas: f("lemmas")?,
            forms: f("forms")?,
            glosses: f("glosses")?,
            sense_items: f("sense_items")?,
            sense_links: f("sense_links")?,
            sense_strs: f("sense_strs")?,
            lex_claims: f("lex_claims")?,
        })
    }

    fn finish(self) -> Result<Vec<(String, PathBuf, u64, u64, u64)>> {
        let mut v = Vec::new();
        for (n, t) in [
            ("lexemes", self.lexemes),
            ("lemmas", self.lemmas),
            ("forms", self.forms),
            ("glosses", self.glosses),
            ("sense_items", self.sense_items),
            ("sense_links", self.sense_links),
            ("sense_strs", self.sense_strs),
            ("lex_claims", self.lex_claims),
        ] {
            let (p, r, raw, z) = t.finish()?;
            v.push((n.to_string(), p, r, raw, z));
        }
        Ok(v)
    }
}

fn lex_line(line: &[u8], o: &mut LexOuts, st: &mut LexStats) -> Result<()> {
    let mut l = line;
    while let Some((&last, rest)) = l.split_last() {
        if matches!(last, b'\r' | b',' | b' ' | b'\n') {
            l = rest;
        } else {
            break;
        }
    }
    if l.first() != Some(&b'{') {
        return Ok(());
    }
    let x: Lex = match serde_json::from_slice(l) {
        Ok(x) => x,
        Err(e) => {
            st.bad += 1;
            if st.bad_examples.len() < 10 {
                st.bad_examples.push(format!("{e} — {}", String::from_utf8_lossy(&l[..l.len().min(60)])));
            }
            return Ok(());
        }
    };
    if x.typ != "lexeme" {
        return Ok(());
    }
    let Some(lid) = num_id(&x.id, b'L') else {
        st.bad += 1;
        return Ok(());
    };
    st.lexemes += 1;
    let lang = num_id(&x.language, b'Q').unwrap_or(0);
    let cat = num_id(&x.cat, b'Q').unwrap_or(0);
    let (code0, lemma0) = x.lemmas.0.first().map(|(c, v)| (*c, v.as_ref())).unwrap_or(("", ""));
    o.lexemes.n(lid as u64).t().n(lang as u64).t().n(cat as u64).t().s(lemma0).t().s(code0).t().n(x.forms.len() as u64).t().n(x.senses.len() as u64).end()?;
    let bl = st.by_lang.entry(lang).or_default();
    bl[0] += 1;
    bl[1] += x.forms.len() as u64;
    bl[2] += x.senses.len() as u64;
    for (c, v) in &x.lemmas.0 {
        o.lemmas.n(lid as u64).t().s(c).t().s(v).end()?;
        st.lemmas += 1;
    }
    for &(p, q, r) in &x.claims.items {
        o.lex_claims.n(lid as u64).t().n(p as u64).t().n(q as u64).t().n(r as u64).end()?;
        st.lex_claims += 1;
    }
    for f in &x.forms {
        let Some((_, fno)) = sub_id(&f.id, b'F') else { continue };
        st.forms += 1;
        let feats: Vec<String> = f.feats.iter().filter_map(|q| num_id(q, b'Q')).map(|q| q.to_string()).collect();
        let feats = feats.join(",");
        for (c, v) in &f.representations.0 {
            o.forms.n(lid as u64).t().n(fno as u64).t().s(c).t().s(v).t().s(&feats).end()?;
            st.form_reps += 1;
        }
    }
    for s in &x.senses {
        let Some((_, sno)) = sub_id(&s.id, b'S') else { continue };
        st.senses += 1;
        for (c, v) in &s.glosses.0 {
            o.glosses.n(lid as u64).t().n(sno as u64).t().s(c).t().s(v).end()?;
            st.glosses += 1;
        }
        for &(p, q, r) in &s.claims.items {
            o.sense_items.n(lid as u64).t().n(sno as u64).t().n(p as u64).t().n(q as u64).t().n(r as u64).end()?;
            *st.sense_items.entry(p).or_default() += 1;
        }
        for &(p, l2, s2, r) in &s.claims.senses {
            o.sense_links.n(lid as u64).t().n(sno as u64).t().n(p as u64).t().n(l2 as u64).t().n(s2 as u64).t().n(r as u64).end()?;
            *st.sense_links.entry(p).or_default() += 1;
        }
        for (p, v, r) in &s.claims.strs {
            o.sense_strs.n(lid as u64).t().n(sno as u64).t().n(*p as u64).t().s(v).t().n(*r as u64).end()?;
            st.sense_strs += 1;
        }
    }
    Ok(())
}

/// Lexeme dump → `lex-*.tsv.zst`. Fail-fast: more than 100 unparsed lines or an lbzip2 error — refusal.
pub fn extract(dump: &Path, lbzip2: &Path, bz_threads: usize, out: &Path, level: i32) -> Result<(LexStats, Vec<(String, PathBuf, u64, u64, u64)>)> {
    std::fs::create_dir_all(out)?;
    let t0 = Instant::now();
    let mut child = Command::new(lbzip2)
        .args(["-dc", "-n", &bz_threads.to_string()])
        .arg(dump)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("cannot start {}", lbzip2.display()))?;
    let mut rd = BufReader::with_capacity(1 << 22, child.stdout.take().context("stdout")?);
    let mut o = LexOuts::create(out, level)?;
    let mut st = LexStats::default();
    let mut line = Vec::with_capacity(1 << 20);
    loop {
        line.clear();
        if rd.read_until(b'\n', &mut line)? == 0 {
            break;
        }
        lex_line(&line, &mut o, &mut st)?;
        if st.bad > 100 {
            let _ = child.kill();
            bail!("more than 100 unparsed lexemes: {:?}", st.bad_examples);
        }
    }
    let status = child.wait()?;
    let mut err = String::new();
    if let Some(mut e) = child.stderr.take() {
        use std::io::Read;
        let _ = e.read_to_string(&mut err);
    }
    if !status.success() {
        bail!("lbzip2: {status} {err}");
    }
    let files = o.finish()?;
    st.wall = t0.elapsed().as_secs_f64();
    Ok((st, files))
}

/// Lexeme report. `lang_names` — language Q → name (from `items`, if present).
pub fn report(st: &LexStats, files: &[(String, PathBuf, u64, u64, u64)], dump: &Path) -> String {
    use std::fmt::Write as _;
    let mut s = String::from("# Wikidata lexemes → TSV\n\n");
    let _ = writeln!(s, "- dump `{}`; {:.0} s; unparsed {}", dump.display(), st.wall, st.bad);
    let _ = writeln!(
        s,
        "- lexemes {}, lemmas (with spelling variants) {}, forms {} (representations {}), senses {}, glosses {}, lexeme claims with an item {}",
        thousands(st.lexemes),
        thousands(st.lemmas),
        thousands(st.forms),
        thousands(st.form_reps),
        thousands(st.senses),
        thousands(st.glosses),
        thousands(st.lex_claims)
    );
    let name = |p: u32| match p {
        5137 => "P5137 item for this sense",
        5972 => "P5972 translation",
        5973 => "P5973 synonym",
        5974 => "P5974 antonym",
        _ => "",
    };
    let mut si: Vec<_> = st.sense_items.iter().collect();
    si.sort_by(|a, b| b.1.cmp(a.1));
    let _ = writeln!(s, "- sense → item: {}", si.iter().take(8).map(|(p, n)| format!("P{p} {} {}", name(**p), thousands(**n))).collect::<Vec<_>>().join(" · "));
    let mut sl: Vec<_> = st.sense_links.iter().collect();
    sl.sort_by(|a, b| b.1.cmp(a.1));
    let _ = writeln!(s, "- sense → sense: {}", sl.iter().take(8).map(|(p, n)| format!("P{p} {} {}", name(**p), thousands(**n))).collect::<Vec<_>>().join(" · "));
    let _ = writeln!(s, "- P8814 WordNet 3.1 synset on senses: {}", st.sense_strs);
    let mut bl: Vec<_> = st.by_lang.iter().collect();
    bl.sort_by(|a, b| b.1[0].cmp(&a.1[0]));
    let _ = writeln!(s, "- lexeme languages {}; first 20 (language Q: lexemes / forms / senses): {}", bl.len(), bl.iter().take(20).map(|(q, c)| format!("Q{q} {}/{}/{}", c[0], c[1], c[2])).collect::<Vec<_>>().join(" · "));
    let _ = writeln!(s, "\n| table | rows | TSV, MB | zstd, MB |\n|---|---:|---:|---:|");
    for (t, _, r, raw, z) in files {
        let _ = writeln!(s, "| lex-{t} | {} | {:.1} | {:.1} |", thousands(*r), *raw as f64 / 1e6, *z as f64 / 1e6);
    }
    if !st.bad_examples.is_empty() {
        let _ = writeln!(s, "\nUnparsed: {:?}", st.bad_examples);
    }
    s
}

#[cfg(test)]
mod t {
    use super::*;

    #[test]
    fn lexeme_rows() {
        let js = r#"{"type":"lexeme","id":"L7","lemmas":{"de":{"language":"de","value":"Apfel"}},"lexicalCategory":"Q1084","language":"Q188","claims":{"P5185":[{"mainsnak":{"datavalue":{"value":{"entity-type":"item","id":"Q1775461"}}},"rank":"normal"}]},"forms":[{"id":"L7-F1","representations":{"de":{"language":"de","value":"Apfel"}},"grammaticalFeatures":["Q131105","Q110786"],"claims":[]},{"id":"L7-F2","representations":{"de":{"language":"de","value":"Äpfel"}},"grammaticalFeatures":["Q146233"],"claims":[]}],"senses":[{"id":"L7-S1","glosses":{"de":{"language":"de","value":"Frucht des Apfelbaums"},"en":{"language":"en","value":"apple"}},"claims":{"P5137":[{"mainsnak":{"datavalue":{"value":{"entity-type":"item","id":"Q89"}}},"rank":"normal"}],"P5972":[{"mainsnak":{"datavalue":{"value":{"entity-type":"sense","id":"L3257-S1"}}},"rank":"normal"}],"P8814":[{"mainsnak":{"datavalue":{"value":"07755101-n"}}}]}}]},"#;
        let dir = std::env::temp_dir().join(format!("onto-wdlex-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let mut o = LexOuts::create(&dir, 3).unwrap();
        let mut st = LexStats::default();
        lex_line(js.as_bytes(), &mut o, &mut st).unwrap();
        lex_line(b"[", &mut o, &mut st).unwrap();
        let files = o.finish().unwrap();
        let get = |t: &str| {
            let p = &files.iter().find(|f| f.0 == t).unwrap().1;
            String::from_utf8(zstd::stream::decode_all(std::fs::File::open(p).unwrap()).unwrap()).unwrap()
        };
        assert_eq!(get("lexemes"), "7\t188\t1084\tApfel\tde\t2\t1\n");
        assert_eq!(get("forms"), "7\t1\tde\tApfel\t131105,110786\n7\t2\tde\tÄpfel\t146233\n");
        assert_eq!(get("glosses"), "7\t1\tde\tFrucht des Apfelbaums\n7\t1\ten\tapple\n");
        assert_eq!(get("sense_items"), "7\t1\t5137\t89\t1\n");
        assert_eq!(get("sense_links"), "7\t1\t5972\t3257\t1\t1\n");
        assert_eq!(get("sense_strs"), "7\t1\t8814\t07755101-n\t1\n");
        assert_eq!(get("lex_claims"), "7\t5185\t1775461\t1\n");
        assert_eq!((st.lexemes, st.bad), (1, 0));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ids() {
        assert_eq!(sub_id("L123-S2", b'S'), Some((123, 2)));
        assert_eq!(sub_id("L123-F14", b'F'), Some((123, 14)));
        assert_eq!(sub_id("L123-F14", b'S'), None);
        assert_eq!(sub_id("Q5", b'S'), None);
    }
}

/// English senses with P5137 or P8814 in the `wd-senses-en.tsv` format (as in `onto wd-lexemes`) — from our
/// `lex-*.tsv.zst`, so that `onto wd-map` matches against the same data as in the database. Checked against the previous
/// run: the same 33,387 senses.
pub fn senses_en(dir: &Path, out: &Path) -> Result<(usize, usize)> {
    use crate::wdgraph::for_each_row;
    use std::collections::BTreeMap;
    use std::fmt::Write as _;
    const EN: u32 = 1860;
    let mut lex: HashMap<u32, (u32, String)> = HashMap::new(); // l → (category, en lemma)
    for_each_row(&dir.join("lex-lexemes.tsv.zst"), |f| {
        if f.len() >= 7 && f[1].parse::<u32>()? == EN {
            lex.insert(f[0].parse()?, (f[2].parse()?, String::new()));
        }
        Ok(())
    })?;
    // lemma: en → en-gb → en-us (as in wd-lexemes)
    let mut best: HashMap<u32, (u8, String)> = HashMap::new();
    for_each_row(&dir.join("lex-lemmas.tsv.zst"), |f| {
        let l: u32 = f[0].parse()?;
        if !lex.contains_key(&l) {
            return Ok(());
        }
        let r = match f[1] {
            "en" => 0,
            "en-gb" => 1,
            "en-us" => 2,
            _ => return Ok(()),
        };
        let e = best.entry(l).or_insert((9, String::new()));
        if r < e.0 {
            *e = (r, f[2].to_string());
        }
        Ok(())
    })?;
    for (l, (_, lemma)) in best {
        if let Some(x) = lex.get_mut(&l) {
            x.1 = lemma;
        }
    }
    let mut p5137: BTreeMap<(u32, u32), Vec<u32>> = BTreeMap::new();
    for_each_row(&dir.join("lex-sense_items.tsv.zst"), |f| {
        let l: u32 = f[0].parse()?;
        if f[2] == "5137" && f[4] != "0" && lex.contains_key(&l) {
            p5137.entry((l, f[1].parse()?)).or_default().push(f[3].parse()?);
        }
        Ok(())
    })?;
    let mut p8814: BTreeMap<(u32, u32), Vec<String>> = BTreeMap::new();
    for_each_row(&dir.join("lex-sense_strs.tsv.zst"), |f| {
        let l: u32 = f[0].parse()?;
        if f[2] == "8814" && lex.contains_key(&l) {
            p8814.entry((l, f[1].parse()?)).or_default().push(f[3].to_string());
        }
        Ok(())
    })?;
    let mut gloss: HashMap<(u32, u32), String> = HashMap::new();
    for_each_row(&dir.join("lex-glosses.tsv.zst"), |f| {
        let l: u32 = f[0].parse()?;
        if f[2] == "en" && lex.contains_key(&l) {
            gloss.insert((l, f[1].parse()?), f[3].to_string());
        }
        Ok(())
    })?;
    let mut keys: Vec<(u32, u32)> = p5137.keys().chain(p8814.keys()).copied().collect();
    keys.sort_unstable();
    keys.dedup();
    let mut s = String::from("# lexeme\tlemma\tcategory\tsense\tP5137\tP8814\tgloss_en\n");
    let mut with_q = 0;
    for (l, sn) in &keys {
        let (cat, lemma) = &lex[l];
        let qs = p5137.get(&(*l, *sn)).map(|v| v.iter().map(|q| format!("Q{q}")).collect::<Vec<_>>().join(",")).unwrap_or_default();
        with_q += !qs.is_empty() as usize;
        let wn = p8814.get(&(*l, *sn)).map(|v| v.join(",")).unwrap_or_default();
        let _ = writeln!(s, "L{l}\t{lemma}\tQ{cat}\tL{l}-S{sn}\t{qs}\t{wn}\t{}", gloss.get(&(*l, *sn)).map(|g| g.as_str()).unwrap_or(""));
    }
    std::fs::write(out, s)?;
    Ok((keys.len(), with_q))
}
