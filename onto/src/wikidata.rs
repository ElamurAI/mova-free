//! Wikidata — streaming, one core, read-only.
//!
//! - `probe`: the first N bytes of the compressed full dump (`latest-all.json.bz2` in the data directory) → P31/P279 and English
//!   labels; measures decompression and parsing time separately to estimate a full pass. The dump is not changed, nothing is
//!   written into the dump's folder.
//! - `lexemes`: lexeme dump (CC0) → English senses with P5137 «item for this sense» and P8814 «WordNet 3.1
//!   synset»; statistics for all languages.
//!
//! Parsing borrows strings from the buffer (`Cow`) and skips the remaining fields. Empty maps in the Wikidata dump are written as
//! `[]` (a PHP legacy) — `map_or_seq` accepts them.

use std::borrow::Cow;
use std::fmt;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::marker::PhantomData;
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result};
use serde::de::{self, Deserializer, IgnoredAny, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

/// Accepts a JSON object as a struct, and an empty array `[]` as the default value.
fn map_or_seq<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    struct V<T>(PhantomData<T>);
    impl<'de, T: Deserialize<'de> + Default> Visitor<'de> for V<T> {
        type Value = T;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("an object or an empty array")
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<T, A::Error> {
            while a.next_element::<IgnoredAny>()?.is_some() {}
            Ok(T::default())
        }
        fn visit_map<A: MapAccess<'de>>(self, a: A) -> Result<T, A::Error> {
            T::deserialize(de::value::MapAccessDeserializer::new(a))
        }
    }
    d.deserialize_any(V(PhantomData))
}

#[derive(Deserialize)]
struct Text<'a> {
    #[serde(borrow)]
    value: Cow<'a, str>,
}

#[derive(Deserialize, Default)]
struct EnLabel<'a> {
    #[serde(borrow, default)]
    en: Option<Text<'a>>,
}

#[derive(Deserialize)]
struct ItemVal<'a> {
    #[serde(borrow, default)]
    id: Option<Cow<'a, str>>,
}

#[derive(Deserialize)]
struct ItemDv<'a> {
    #[serde(borrow)]
    value: ItemVal<'a>,
}

#[derive(Deserialize)]
struct ItemSnak<'a> {
    #[serde(borrow, default)]
    datavalue: Option<ItemDv<'a>>,
}

#[derive(Deserialize)]
struct ItemStmt<'a> {
    #[serde(borrow)]
    mainsnak: ItemSnak<'a>,
    #[serde(borrow, default)]
    rank: Option<Cow<'a, str>>,
}

#[derive(Deserialize)]
struct StrDv<'a> {
    #[serde(borrow)]
    value: Cow<'a, str>,
}

#[derive(Deserialize)]
struct StrSnak<'a> {
    #[serde(borrow, default)]
    datavalue: Option<StrDv<'a>>,
}

#[derive(Deserialize)]
struct StrStmt<'a> {
    #[serde(borrow)]
    mainsnak: StrSnak<'a>,
}

#[derive(Deserialize, Default)]
struct ClassClaims<'a> {
    #[serde(rename = "P31", borrow, default)]
    p31: Vec<ItemStmt<'a>>,
    #[serde(rename = "P279", borrow, default)]
    p279: Vec<ItemStmt<'a>>,
}

#[derive(Deserialize)]
struct Entity<'a> {
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow, default, deserialize_with = "map_or_seq")]
    labels: EnLabel<'a>,
    #[serde(borrow, default, deserialize_with = "map_or_seq")]
    claims: ClassClaims<'a>,
}

fn ids<'a>(v: &'a [ItemStmt<'a>]) -> impl Iterator<Item = &'a str> {
    v.iter()
        .filter(|s| s.rank.as_deref() != Some("deprecated"))
        .filter_map(|s| s.mainsnak.datavalue.as_ref().and_then(|d| d.value.id.as_deref()))
}

/// Dump line → entity JSON: without `[`/`]` and the trailing comma.
fn entity_json(line: &str) -> Option<&str> {
    let t = line.trim_end();
    let t = t.strip_suffix(',').unwrap_or(t);
    if t.len() < 2 || !t.starts_with('{') {
        return None;
    }
    Some(t)
}

/// Measuring a pass over the first `max_bytes` of the compressed dump: decompression and parsing separately.
pub fn probe(dump: &Path, max_bytes: u64, out_tsv: &Path, report: &Path) -> Result<()> {
    let f = File::open(dump).with_context(|| format!("cannot open {}", dump.display()))?;
    let total_size = f.metadata()?.len();
    let dec = bzip2::read::MultiBzDecoder::new(BufReader::with_capacity(1 << 20, f.take(max_bytes)));
    let mut rd = BufReader::with_capacity(1 << 22, dec);
    let mut out = BufWriter::new(File::create(out_tsv)?);
    writeln!(out, "# id\ten_label\tP31\tP279")?;
    let (mut t_read, mut t_parse) = (0f64, 0f64);
    let (mut lines, mut ents, mut bad, mut with31, mut with279, mut with_en, mut json_bytes) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut n31 = 0u64;
    let mut n279 = 0u64;
    let mut line = String::new();
    let t0 = Instant::now();
    let mut stop = String::new();
    loop {
        line.clear();
        let t = Instant::now();
        let r = rd.read_line(&mut line);
        t_read += t.elapsed().as_secs_f64();
        match r {
            Ok(0) => break,
            Ok(n) => json_bytes += n as u64,
            Err(e) => {
                // the end of a cut piece is a truncated bzip2 block: this is expected
                stop = format!("reading stopped: {e}");
                break;
            }
        }
        lines += 1;
        let Some(js) = entity_json(&line) else { continue };
        let t = Instant::now();
        let parsed: Result<Entity, _> = serde_json::from_str(js);
        t_parse += t.elapsed().as_secs_f64();
        let Ok(e) = parsed else {
            bad += 1;
            continue;
        };
        ents += 1;
        let p31: Vec<&str> = ids(&e.claims.p31).collect();
        let p279: Vec<&str> = ids(&e.claims.p279).collect();
        n31 += p31.len() as u64;
        n279 += p279.len() as u64;
        with31 += !p31.is_empty() as u64;
        with279 += !p279.is_empty() as u64;
        let en = e.labels.en.as_ref().map(|t| t.value.as_ref()).unwrap_or("");
        with_en += !en.is_empty() as u64;
        if !p31.is_empty() || !p279.is_empty() {
            writeln!(out, "{}\t{}\t{}\t{}", e.id, en.replace(['\t', '\n'], " "), p31.join(","), p279.join(","))?;
        }
    }
    out.flush()?;
    let wall = t0.elapsed().as_secs_f64();
    let frac = max_bytes.min(total_size) as f64 / total_size as f64;
    let est_wall_h = wall / frac / 3600.0;
    let est_ents = ents as f64 / frac;
    let mut r = BufWriter::new(File::create(report)?);
    let s = format!(
        "# Wikidata: streaming pass measurement (P31/P279 + English labels)\n\n\
         - dump: `{}`, {:.1} GB compressed; read the first {:.2} GB ({:.3}% of the file)\n\
         - decompressed JSON: {:.2} GB (compression ×{:.1}), lines {lines}, entities {ents}, unparsed {bad}\n\
         - with P31: {with31} ({n31} statements), with P279: {with279} ({n279} statements), with an English label: {with_en}\n\
         - time: total {wall:.1} s; bzip2 decompression {t_read:.1} s, JSON parsing {t_parse:.1} s (1 core, nice)\n\
         - speed: {:.2} MB/s compressed, {:.0} entities/s\n\
         - {stop}\n\n\
         **Estimated full pass on one core:** {est_wall_h:.1} h, ~{:.0} M entities.\n",
        dump.display(),
        total_size as f64 / 1e9,
        max_bytes.min(total_size) as f64 / 1e9,
        frac * 100.0,
        json_bytes as f64 / 1e9,
        json_bytes as f64 / max_bytes.min(total_size) as f64,
        max_bytes.min(total_size) as f64 / 1e6 / wall,
        ents as f64 / wall,
        est_ents / 1e6,
    );
    r.write_all(s.as_bytes())?;
    print!("{s}");
    Ok(())
}

// ── lexemes ──────────────────────────────────────────────────────────────────────────────────────

#[derive(Deserialize, Default)]
struct LexLemmas<'a> {
    #[serde(borrow, default)]
    en: Option<Text<'a>>,
    #[serde(rename = "en-gb", borrow, default)]
    en_gb: Option<Text<'a>>,
    #[serde(rename = "en-us", borrow, default)]
    en_us: Option<Text<'a>>,
}

#[derive(Deserialize, Default)]
struct SenseClaims<'a> {
    #[serde(rename = "P5137", borrow, default)]
    p5137: Vec<ItemStmt<'a>>,
    #[serde(rename = "P8814", borrow, default)]
    p8814: Vec<StrStmt<'a>>,
}

#[derive(Deserialize)]
struct LexSense<'a> {
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow, default, deserialize_with = "map_or_seq")]
    glosses: EnLabel<'a>,
    #[serde(borrow, default, deserialize_with = "map_or_seq")]
    claims: SenseClaims<'a>,
}

#[derive(Deserialize)]
struct Lexeme<'a> {
    #[serde(borrow)]
    id: Cow<'a, str>,
    #[serde(borrow, default, deserialize_with = "map_or_seq")]
    lemmas: LexLemmas<'a>,
    #[serde(rename = "lexicalCategory", borrow, default)]
    category: Cow<'a, str>,
    #[serde(borrow, default)]
    language: Cow<'a, str>,
    #[serde(borrow, default)]
    senses: Vec<LexSense<'a>>,
}

/// English — Q1860.
const ENGLISH: &str = "Q1860";

/// Wikidata lexical category → OEWN part of speech.
pub fn category_pos(q: &str) -> Option<char> {
    Some(match q {
        "Q1084" | "Q147276" => 'n', // noun, proper noun
        "Q24905" => 'v',
        "Q34698" => 'a',
        "Q380057" => 'r',
        _ => return None,
    })
}

/// Lexeme dump → `wd-senses-en.tsv` (English senses) and a statistics report.
pub fn lexemes(dump: &Path, out_tsv: &Path, report: &Path) -> Result<()> {
    let f = File::open(dump).with_context(|| format!("cannot open {}", dump.display()))?;
    let dec = bzip2::read::MultiBzDecoder::new(BufReader::with_capacity(1 << 20, f));
    let mut rd = BufReader::with_capacity(1 << 22, dec);
    let mut out = BufWriter::new(File::create(out_tsv)?);
    writeln!(out, "# lexeme\tlemma\tcategory\tsense\tP5137\tP8814\tgloss_en")?;
    let t0 = Instant::now();
    let (mut lexs, mut bad, mut senses, mut s5137, mut s8814) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut en_lex, mut en_senses, mut en_5137, mut en_8814, mut en_gloss) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let mut en_by_cat: std::collections::BTreeMap<String, (u64, u64)> = Default::default();
    let mut qs: std::collections::HashSet<String> = Default::default();
    let mut line = String::new();
    loop {
        line.clear();
        if rd.read_line(&mut line)? == 0 {
            break;
        }
        let Some(js) = entity_json(&line) else { continue };
        let Ok(l) = serde_json::from_str::<Lexeme>(js) else {
            bad += 1;
            continue;
        };
        lexs += 1;
        let en = l.language == ENGLISH;
        en_lex += en as u64;
        let lemma = l.lemmas.en.as_ref().or(l.lemmas.en_gb.as_ref()).or(l.lemmas.en_us.as_ref()).map(|t| t.value.as_ref()).unwrap_or("");
        for s in &l.senses {
            senses += 1;
            let q: Vec<&str> = ids(&s.claims.p5137).collect();
            let wn: Vec<&str> = s.claims.p8814.iter().filter_map(|x| x.mainsnak.datavalue.as_ref().map(|d| d.value.as_ref())).collect();
            s5137 += !q.is_empty() as u64;
            s8814 += !wn.is_empty() as u64;
            if !en {
                continue;
            }
            en_senses += 1;
            en_5137 += !q.is_empty() as u64;
            en_8814 += !wn.is_empty() as u64;
            let gloss = s.glosses.en.as_ref().map(|t| t.value.as_ref()).unwrap_or("");
            en_gloss += !gloss.is_empty() as u64;
            let c = en_by_cat.entry(l.category.to_string()).or_default();
            c.0 += 1;
            c.1 += !q.is_empty() as u64;
            for x in &q {
                qs.insert(x.to_string());
            }
            if !q.is_empty() || !wn.is_empty() {
                writeln!(
                    out,
                    "{}\t{}\t{}\t{}\t{}\t{}\t{}",
                    l.id,
                    lemma.replace(['\t', '\n'], " "),
                    l.category,
                    s.id,
                    q.join(","),
                    wn.join(","),
                    gloss.replace(['\t', '\n'], " ")
                )?;
            }
        }
    }
    out.flush()?;
    let wall = t0.elapsed().as_secs_f64();
    let mut cats: Vec<_> = en_by_cat.into_iter().collect();
    cats.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    let mut s = format!(
        "# Wikidata lexemes (CC0): statistics\n\n\
         - dump: `{}`; time {wall:.0} s (1 core)\n\
         - lexemes {lexs} (unparsed {bad}), senses {senses}; with P5137 «item for this sense» {s5137}, with P8814 «WordNet 3.1 synset» {s8814}\n\
         - English lexemes {en_lex}, senses {en_senses}; with P5137 {en_5137} ({} distinct Q), with P8814 {en_8814}, with an English gloss {en_gloss}\n\n\
         | category (English) | senses | with P5137 |\n|---|---:|---:|\n",
        dump.display(),
        qs.len()
    );
    for (c, (n, q)) in cats.iter().take(12) {
        s.push_str(&format!("| {c} | {n} | {q} |\n"));
    }
    std::fs::write(report, &s)?;
    print!("{s}");
    Ok(())
}

// ── trial mapping of lexeme senses to OEWN synsets ─────────────────────────────────────────

const STOP: &[&str] = &[
    "a", "an", "the", "of", "or", "and", "in", "on", "to", "for", "with", "by", "as", "at", "from", "that", "which", "who", "is", "are", "be", "its", "it", "any", "some",
    "usually", "especially", "often", "one", "used", "etc", "something", "someone", "person", "thing", "being", "having", "has", "not", "into", "this", "their", "such",
];

pub(crate) fn toks(s: &str) -> HashSet<String> {
    s.split(|c: char| !c.is_alphanumeric())
        .map(|t| t.to_lowercase())
        .filter(|t| t.len() > 2 && !STOP.contains(&t.as_str()))
        .map(|t| t.trim_end_matches('s').to_string())
        .collect()
}

use std::collections::{HashMap, HashSet};

/// Senses of English lexemes with P5137 → OEWN synsets: P8814 (WordNet 3.1 synset — the offset matches the
/// OEWN id), unambiguous (one sense on both sides), by overlap of gloss words (≥2 shared and a lead
/// over the second). The rest are ambiguous, no guessing.
pub fn map_trial(g: &crate::model::Graph, tsv: &Path, out: &Path, report: &Path) -> Result<()> {
    let text = std::fs::read_to_string(tsv).with_context(|| format!("cannot read {} — run wd-lexemes first", tsv.display()))?;
    let rows: Vec<Vec<&str>> = text.lines().filter(|l| !l.starts_with('#')).map(|l| l.split('\t').collect()).filter(|f: &Vec<&str>| f.len() >= 7).collect();
    let mut per_lex: HashMap<&str, usize> = HashMap::new();
    for f in &rows {
        if !f[4].is_empty() {
            *per_lex.entry(f[0]).or_default() += 1;
        }
    }
    let mut w = BufWriter::new(File::create(out)?);
    writeln!(w, "# Q\tsynset\tlemma\tmethod\tscore\twd_gloss\toewn_gloss")?;
    let mut counts: std::collections::BTreeMap<&str, usize> = Default::default();
    let mut synsets: HashSet<u32> = HashSet::new();
    let mut by_gloss: Vec<String> = Vec::new();
    let mut key_words: Vec<String> = Vec::new();
    let keys = ["bird", "crow", "fox", "apple", "cheese", "lion", "mouse", "hare", "tortoise", "whale", "penguin", "meat", "genie"];
    for f in &rows {
        let (lemma, cat, qs, wn, gloss) = (f[1], f[2], f[4], f[5], f[6]);
        if qs.is_empty() {
            continue;
        }
        let q = qs.split(',').next().unwrap_or("");
        let Some(pc) = category_pos(cat) else {
            *counts.entry("other category").or_default() += 1;
            continue;
        };
        let pos = crate::model::Pos::from_wn(&pc.to_string()).expect("n v a r");
        let mut res: Option<(crate::model::Cid, &str, usize)> = None;
        if !wn.is_empty() {
            let id = format!("oewn-{}", wn.split(',').next().unwrap_or(""));
            if let Some(c) = g.find_synset(&id) {
                res = Some((c, "P8814", 0));
            }
        }
        let found = crate::query::lookup(g, lemma, pos);
        if res.is_none() {
            match &found {
                None => {
                    *counts.entry("lemma not in OEWN").or_default() += 1;
                    continue;
                }
                Some(fd) if fd.senses.len() == 1 && per_lex.get(f[0]) == Some(&1) => res = Some((fd.senses[0], "unambiguous", 0)),
                Some(fd) => {
                    let wt = toks(gloss);
                    let mut sc: Vec<(usize, crate::model::Cid)> = fd
                        .senses
                        .iter()
                        .map(|&c| {
                            let mut ot = toks(g.gloss_of(c));
                            for m in g.members(c) {
                                ot.extend(toks(g.word(*m)));
                            }
                            ot.remove(&lemma.to_lowercase());
                            (wt.intersection(&ot).count(), c)
                        })
                        .collect();
                    sc.sort_by(|a, b| b.0.cmp(&a.0));
                    let second = sc.get(1).map(|x| x.0).unwrap_or(0);
                    if sc[0].0 >= 2 && sc[0].0 > second {
                        res = Some((sc[0].1, "by gloss", sc[0].0));
                    }
                }
            }
        }
        let Some((c, how, score)) = res else {
            *counts.entry("ambiguous").or_default() += 1;
            continue;
        };
        *counts.entry(how).or_default() += 1;
        synsets.insert(c);
        let line = format!("{q}\t{}\t{lemma}\t{how}\t{score}\t{}\t{}", g.cid_str(c), gloss.replace('\t', " "), g.gloss_of(c));
        writeln!(w, "{line}")?;
        if how == "by gloss" {
            by_gloss.push(line.clone());
        }
        if keys.contains(&lemma) {
            key_words.push(format!("| {lemma} | {q} | {} | {how} | {} |", g.show(c), gloss.chars().take(70).collect::<String>()));
        }
    }
    w.flush()?;
    let total: usize = counts.values().sum();
    let mut s = format!("# Trial mapping: Wikidata lexeme senses (P5137) → OEWN synsets\n\nEnglish senses with P5137: {total}. Distinct synsets with Q: {}.\n\n| method | senses |\n|---|---:|\n", synsets.len());
    for (k, v) in &counts {
        let _ = writeln!(s, "| {k} | {v} ({:.0}%) |", 100.0 * *v as f64 / total.max(1) as f64);
    }
    s.push_str("\n## Fable keywords\n\n| lemma | Q | synset | method | Wikidata gloss |\n|---|---|---|---|---|\n");
    for k in &key_words {
        s.push_str(k);
        s.push('\n');
    }
    // sample for manual checking: every N-th mapping by gloss
    s.push_str("\n## «By gloss» sample for manual checking (every N-th)\n\n");
    let step = (by_gloss.len() / 20).max(1);
    for (i, l) in by_gloss.iter().step_by(step).take(20).enumerate() {
        let _ = writeln!(s, "{}. `{}`", i + 1, l.replace('\t', " · "));
    }
    std::fs::write(report, &s)?;
    print!("{s}");
    Ok(())
}

#[cfg(test)]
mod t {
    use super::*;

    /// Empty dump maps — `[]`; P31/P279 with deprecated rank — dropped; English label.
    #[test]
    fn dump_quirks() {
        let e: Entity = serde_json::from_str(r#"{"type":"item","id":"Q1","labels":[],"claims":[]}"#).unwrap();
        assert!(e.labels.en.is_none() && e.claims.p31.is_empty());
        let js = r#"{"id":"Q146","labels":{"fr":{"language":"fr","value":"chat"},"en":{"language":"en","value":"house cat"}},
            "claims":{"P31":[{"mainsnak":{"datavalue":{"value":{"id":"Q55983715"}}},"rank":"normal"}],
            "P279":[{"mainsnak":{"datavalue":{"value":{"id":"Q39201"}}},"rank":"normal"},{"mainsnak":{"datavalue":{"value":{"id":"Q999"}}},"rank":"deprecated"}],
            "P18":[{"mainsnak":{"datavalue":{"value":"Cat.jpg"}}}]}}"#;
        let e: Entity = serde_json::from_str(js).unwrap();
        assert_eq!(e.labels.en.as_ref().unwrap().value, "house cat");
        assert_eq!(ids(&e.claims.p31).collect::<Vec<_>>(), ["Q55983715"]);
        assert_eq!(ids(&e.claims.p279).collect::<Vec<_>>(), ["Q39201"]);
        assert_eq!(entity_json("{\"id\":\"Q1\"},\n"), Some("{\"id\":\"Q1\"}"));
        assert_eq!(entity_json("[\n"), None);
    }
}
