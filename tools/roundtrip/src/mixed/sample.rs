//! The sampler: sentences of many languages and styles, drawn with a fixed seed, and the streams made of them —
//! one variety a line (`mono-<variety>`), every variety shuffled one sentence a line (`lines`), and documents where an
//! English sentence quotes one or two pieces of Hebrew or Aramaic (`docs`), as a commentary does.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use langseg::{Lang, Rng, fnv};

use super::Route;

/// Where the sources are.
#[derive(Clone, Debug)]
pub struct Paths {
    /// the school's lessons (`$MOVA_DATA/school/courses`, the school data)
    pub school: PathBuf,
    /// the 0.3 tales treebank (`mova-tales-silver-en.conllu`)
    pub tales: PathBuf,
    /// `corpora/judaism`: `dict/wlc-trees`, `dict/tanakh-jps1917.tsv`, `dict/tanakh-en.tsv`, `text/…`
    pub judaism: PathBuf,
    /// `corpora/emode/text/shakespeare.tsv`, if there
    pub shakespeare: PathBuf,
    /// UD 2.18 (`UD_Hebrew-IAHLTwiki`, `nc/UD_Hebrew-HTB`): Modern Hebrew, read for measuring only
    pub ud: PathBuf,
    /// the Hebrew parser's models (`pointed/`, `unpointed/`)
    pub hbo: PathBuf,
    /// the language segmenter's model
    pub langseg: PathBuf,
    /// the `gparse` binary
    pub gparse: String,
}

/// A variety of the sample: its name, the code `langseg` should give it, the route it should take, its source.
pub struct Variety {
    pub name: &'static str,
    pub lang: Lang,
    pub route: Route,
    pub source: &'static str,
}

pub const VARIETIES: [Variety; 16] = [
    Variety { name: "en-plain", lang: Lang::En, route: Route::En, source: "school lessons, explain/fact (own)" },
    Variety { name: "en-slang", lang: Lang::En, route: Route::En, source: "school slang tests, the question (own)" },
    Variety { name: "en-formal", lang: Lang::En, route: Route::En, source: "school formal exams, the question (own)" },
    Variety { name: "en-tale", lang: Lang::En, route: Route::En, source: "0.3 tales treebank text (PD books)" },
    Variety { name: "en-jps", lang: Lang::En, route: Route::En, source: "JPS 1917, Ruth and Jonah (PD)" },
    Variety { name: "en-talmud", lang: Lang::En, route: Route::En, source: "Yerushalmi, Guggenheimer's English (CC BY)" },
    Variety { name: "en-shk", lang: Lang::En, route: Route::En, source: "Shakespeare (PD)" },
    Variety { name: "hbo-accented", lang: Lang::Hbo, route: Route::Heb, source: "WLC Ruth, Jonah as written (CC BY)" },
    Variety { name: "hbo-pointed", lang: Lang::Hbo, route: Route::Heb, source: "WLC Ruth, Jonah, accents off" },
    Variety { name: "hbo-unpointed", lang: Lang::Hbo, route: Route::Heb, source: "WLC Ruth, Jonah, letters only" },
    Variety { name: "hbo-x-mishnaic", lang: Lang::HboMishnaic, route: Route::Heb, source: "Yerushalmi Hebrew segments (CC BY)" },
    Variety { name: "jpa", lang: Lang::Jpa, route: Route::Text, source: "Yerushalmi Aramaic segments (CC BY)" },
    Variety { name: "arc-x-targum", lang: Lang::ArcTargum, route: Route::Text, source: "Targum Onkelos (PD)" },
    Variety { name: "arc", lang: Lang::Arc, route: Route::Text, source: "WLC Biblical Aramaic (CC BY)" },
    Variety { name: "he-wiki", lang: Lang::He, route: Route::Heb, source: "UD IAHLTwiki dev (CC BY-SA, measure only)" },
    Variety { name: "he-htb", lang: Lang::He, route: Route::Heb, source: "UD HTB dev (CC BY-NC, measure only)" },
];

pub fn variety(name: &str) -> Option<usize> {
    VARIETIES.iter().position(|v| v.name == name)
}

/// One sampled sentence.
#[derive(Clone, Debug)]
pub struct Item {
    pub id: String,
    pub var: usize,
    pub text: String,
    /// the Bible verse (`RUT.1.1`) of a WLC or JPS sentence
    pub verse: Option<String>,
}

/// A gold span of a line: the bytes of one item, or glue (`item: None`: the quote marks and spaces a document adds).
#[derive(Clone, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub item: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Stream {
    pub name: String,
    pub lines: Vec<String>,
    pub gold: Vec<Vec<Span>>,
}

fn words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn usable(s: &str) -> bool {
    let n = words(s);
    (3..=40).contains(&n) && !s.contains(['\t', '\n', '\r'])
}

/// English sentences of a value: cut after . ! ? (and a closing quote) before a space and a capital or an opening quote.
pub fn sentences(s: &str) -> Vec<String> {
    let cs: Vec<(usize, char)> = s.char_indices().collect();
    let mut out = Vec::new();
    let mut start = 0;
    for k in 0..cs.len() {
        let (i, c) = cs[k];
        if !matches!(c, '.' | '!' | '?') {
            continue;
        }
        let mut j = k + 1;
        while j < cs.len() && matches!(cs[j].1, '"' | '\'' | '’' | '”' | '»' | ')') {
            j += 1;
        }
        if j + 1 < cs.len() && cs[j].1 == ' ' && (cs[j + 1].1.is_uppercase() || matches!(cs[j + 1].1, '"' | '“' | '«' | '\'')) {
            let end = cs[j].0;
            let t = s[start..end].trim();
            if !t.is_empty() {
                out.push(t.to_string());
            }
            start = cs[j].0 + 1;
        }
        let _ = i;
    }
    let t = s[start..].trim();
    if !t.is_empty() {
        out.push(t.to_string());
    }
    out
}

fn read(p: &Path) -> Result<String> {
    std::fs::read_to_string(p).with_context(|| format!("{}", p.display()))
}

/// `# text` (and `# verse`, `# sent_id`) of a CoNLL-U file.
fn conllu_texts(p: &Path) -> Result<Vec<(String, String, Option<String>)>> {
    let mut out = Vec::new();
    let (mut id, mut verse) = (String::new(), None);
    for l in read(p)?.lines() {
        if let Some(x) = l.strip_prefix("# sent_id = ") {
            id = x.to_string();
            verse = None;
        } else if let Some(x) = l.strip_prefix("# verse = ") {
            verse = Some(x.split_whitespace().next().unwrap_or("").to_string());
        } else if let Some(x) = l.strip_prefix("# text = ") {
            out.push((id.clone(), x.to_string(), verse.clone()));
        }
    }
    Ok(out)
}

fn tsv_rows(p: &Path) -> Result<Vec<Vec<String>>> {
    Ok(read(p)?.lines().map(|l| l.split('\t').map(String::from).collect()).collect())
}

/// Every candidate of every variety, in a fixed order.
fn candidates(p: &Paths) -> Result<BTreeMap<usize, Vec<Item>>> {
    let mut c: BTreeMap<usize, Vec<Item>> = BTreeMap::new();
    let mut push = |var: &str, id: String, text: String, verse: Option<String>| {
        let t = text.trim().to_string();
        if usable(&t) {
            let v = variety(var).unwrap();
            c.entry(v).or_default().push(Item { id, var: v, text: t, verse });
        }
    };
    // the school: explain/fact values, test and exam questions (before the first « | »)
    let mut courses: Vec<PathBuf> = std::fs::read_dir(&p.school).with_context(|| format!("{}", p.school.display()))?.flatten().map(|e| e.path()).filter(|x| x.is_dir()).collect();
    courses.sort();
    for course in courses {
        let mut ls: Vec<PathBuf> = std::fs::read_dir(&course)?.flatten().map(|e| e.path()).filter(|x| x.extension().is_some_and(|e| e == "lesson")).collect();
        ls.sort();
        for f in ls {
            let name = f.strip_prefix(&p.school).unwrap_or(&f).display().to_string();
            for (n, line) in read(&f)?.lines().enumerate() {
                let Some((k, v)) = line.split_once(": ") else { continue };
                let id = format!("school:{name}:{n}");
                match k {
                    "explain" | "fact" => {
                        for (j, s) in sentences(v).into_iter().enumerate() {
                            push("en-plain", format!("{id}:{j}"), s, None);
                        }
                    }
                    "test" => push("en-slang", id, v.split(" | ").next().unwrap_or("").to_string(), None),
                    "exam" => push("en-formal", id, v.split(" | ").next().unwrap_or("").to_string(), None),
                    _ => {}
                }
            }
        }
    }
    for (id, t, _) in conllu_texts(&p.tales)? {
        push("en-tale", id, t, None);
    }
    // JPS 1917 of Ruth and Jonah, with the verse of each sentence
    let d = p.judaism.join("dict");
    let verses: BTreeMap<String, String> = tsv_rows(&d.join("tanakh-jps1917.tsv.verses"))?.into_iter().filter(|r| r.len() >= 2).map(|r| (r[0].clone(), r[1].clone())).collect();
    for r in tsv_rows(&d.join("tanakh-jps1917.tsv"))? {
        if r.len() >= 2 && (r[0].starts_with("jps:RUT:") || r[0].starts_with("jps:JON:")) {
            push("en-jps", r[0].clone(), r[1].clone(), verses.get(&r[0]).cloned());
        }
    }
    // the Yerushalmi: English sentences; the original's segments by their language
    let y = p.judaism.join("text/yerushalmi-guggenheimer");
    let mut ys: Vec<PathBuf> = std::fs::read_dir(&y)?.flatten().map(|e| e.path()).filter(|x| x.extension().is_some_and(|e| e == "tsv")).collect();
    ys.sort();
    for f in ys {
        let fname = f.file_name().unwrap().to_string_lossy().to_string();
        if fname.starts_with("index-") {
            continue;
        }
        let rows = tsv_rows(&f)?;
        if fname.ends_with(".en.tsv") {
            for r in rows.iter().filter(|r| r.len() >= 2) {
                for (j, s) in sentences(&r[1]).into_iter().enumerate() {
                    push("en-talmud", format!("yer-en:{}:{j}", r[0]), s, None);
                }
            }
        } else {
            for (n, r) in rows.iter().enumerate().filter(|(_, r)| r.len() >= 3) {
                match r[1].as_str() {
                    "hbo-x-mishnaic" => push("hbo-x-mishnaic", format!("yer:{}:{n}", r[0]), r[2].clone(), None),
                    "jpa" => push("jpa", format!("yer:{}:{n}", r[0]), r[2].clone(), None),
                    _ => {}
                }
            }
        }
    }
    if p.shakespeare.exists() {
        for r in tsv_rows(&p.shakespeare)? {
            if r.len() >= 2 {
                push("en-shk", r[0].clone(), r[1].clone(), None);
            }
        }
    }
    // the Hebrew Bible: Ruth and Jonah, the Hebrew parser's dev books; each verse in one of three pointings
    let mut k = 0;
    for book in ["RUT", "JON"] {
        for (id, t, verse) in conllu_texts(&d.join(format!("wlc-trees/{book}.conllu")))? {
            let (var, text) = match k % 3 {
                0 => ("hbo-accented", t.clone()),
                1 => ("hbo-pointed", hbo::text::norm(&t, hbo::text::Level::Pointed)),
                _ => ("hbo-unpointed", hbo::text::norm(&t, hbo::text::Level::Unpointed)),
            };
            k += 1;
            push(var, id, text, verse);
        }
    }
    let arc = p.judaism.join("text/tanakh-arc-macula");
    for book in ["DAN", "EZR", "JER"] {
        for (id, t, verse) in conllu_texts(&arc.join(format!("{book}.conllu")))? {
            push("arc", id, t, verse);
        }
    }
    let tg = p.judaism.join("text/targum-onkelos");
    for book in ["GEN", "EXO", "LEV", "NUM", "DEU"] {
        for r in tsv_rows(&tg.join(format!("{book}.tsv")))? {
            if r.len() >= 2 {
                push("arc-x-targum", format!("onkelos:{}", r[0]), r[1].clone(), Some(r[0].clone()));
            }
        }
    }
    for (id, t, _) in conllu_texts(&p.ud.join("UD_Hebrew-IAHLTwiki/he_iahltwiki-ud-dev.conllu"))? {
        push("he-wiki", id, t, None);
    }
    for (id, t, _) in conllu_texts(&p.ud.join("nc/UD_Hebrew-HTB/he_htb-ud-dev.conllu"))? {
        push("he-htb", id, t, None);
    }
    Ok(c)
}

/// At most `n` items of every variety, drawn with the seed; items in variety order.
pub fn sample(p: &Paths, n: usize, seed: u64) -> Result<Vec<Item>> {
    let mut out = Vec::new();
    for (v, mut items) in candidates(p)? {
        items.sort_by(|a, b| a.id.cmp(&b.id));
        items.dedup_by(|a, b| a.text == b.text);
        Rng(seed ^ fnv(VARIETIES[v].name)).shuffle(&mut items);
        items.truncate(n);
        items.sort_by(|a, b| a.id.cmp(&b.id));
        out.extend(items);
    }
    Ok(out)
}

fn line_of(items: &[Item], k: usize) -> (String, Vec<Span>) {
    let t = items[k].text.clone();
    let n = t.len();
    (t, vec![Span { start: 0, end: n, item: Some(k) }])
}

/// Glue and pieces into one line with its gold spans.
struct Doc {
    text: String,
    gold: Vec<Span>,
}

impl Doc {
    fn new() -> Doc {
        Doc { text: String::new(), gold: Vec::new() }
    }
    fn glue(&mut self, s: &str) {
        let a = self.text.len();
        self.text.push_str(s);
        self.gold.push(Span { start: a, end: self.text.len(), item: None });
    }
    fn piece(&mut self, s: &str, item: usize) {
        let a = self.text.len();
        self.text.push_str(s);
        self.gold.push(Span { start: a, end: self.text.len(), item: Some(item) });
    }
}

/// The streams of a sample.
pub fn streams(items: &[Item], seed: u64, docs: usize) -> Vec<Stream> {
    let mut out = Vec::new();
    for (v, var) in VARIETIES.iter().enumerate() {
        let ks: Vec<usize> = (0..items.len()).filter(|&k| items[k].var == v).collect();
        if ks.is_empty() {
            continue;
        }
        let (lines, gold) = ks.iter().map(|&k| line_of(items, k)).unzip();
        out.push(Stream { name: format!("mono-{}", var.name), lines, gold });
    }
    let mut all: Vec<usize> = (0..items.len()).collect();
    Rng(seed ^ 0x11e5).shuffle(&mut all);
    let (lines, gold) = all.iter().map(|&k| line_of(items, k)).unzip();
    out.push(Stream { name: "lines".into(), lines, gold });
    // documents: an English carrier with one or two quotes of Hebrew or Aramaic, and the verse pairs of Ruth and Jonah
    let mut rng = Rng(seed ^ 0xd0c5);
    let en: Vec<usize> = (0..items.len()).filter(|&k| VARIETIES[items[k].var].route == Route::En && items[k].verse.is_none()).collect();
    let foreign: Vec<usize> = (0..items.len()).filter(|&k| VARIETIES[items[k].var].route != Route::En).collect();
    let mut s = Stream { name: "docs".into(), lines: Vec::new(), gold: Vec::new() };
    if !en.is_empty() && !foreign.is_empty() {
        let (mut ei, mut fi) = (en.clone(), foreign.clone());
        rng.shuffle(&mut ei);
        rng.shuffle(&mut fi);
        for d in 0..docs {
            let e = ei[d % ei.len()];
            let q1 = fi[(2 * d) % fi.len()];
            let q2 = fi[(2 * d + 1) % fi.len()];
            let t = &items[e].text;
            let cuts: Vec<usize> = t.match_indices(' ').map(|(i, _)| i).collect();
            let mut doc = Doc::new();
            match d % 3 {
                // the quote inside the sentence
                0 if cuts.len() >= 2 => {
                    let c = cuts[1 + rng.below(cuts.len() - 1)];
                    doc.piece(&t[..c], e);
                    doc.glue(" «");
                    doc.piece(&items[q1].text, q1);
                    doc.glue("» ");
                    doc.piece(&t[c + 1..], e);
                }
                // two quotes after the sentence
                1 => {
                    doc.piece(t, e);
                    doc.glue(" «");
                    doc.piece(&items[q1].text, q1);
                    doc.glue("» (");
                    doc.piece(&items[q2].text, q2);
                    doc.glue(")");
                }
                // one quote after it
                _ => {
                    doc.piece(t, e);
                    doc.glue(" «");
                    doc.piece(&items[q1].text, q1);
                    doc.glue("»");
                }
            }
            s.lines.push(doc.text);
            s.gold.push(doc.gold);
        }
        // a verse of Ruth or Jonah in English, then in Hebrew: the pairs whose names the world may join
        let mut by_verse: BTreeMap<&str, (Vec<usize>, Option<usize>)> = BTreeMap::new();
        for (k, it) in items.iter().enumerate() {
            if let Some(v) = &it.verse {
                match VARIETIES[it.var].name {
                    "en-jps" => by_verse.entry(v).or_default().0.push(k),
                    "hbo-accented" | "hbo-pointed" | "hbo-unpointed" => by_verse.entry(v).or_default().1 = Some(k),
                    _ => {}
                }
            }
        }
        for (_, (es, h)) in by_verse {
            let Some(h) = h else { continue };
            if es.is_empty() {
                continue;
            }
            let mut doc = Doc::new();
            for (j, &e) in es.iter().enumerate() {
                if j > 0 {
                    doc.glue(" ");
                }
                doc.piece(&items[e].text, e);
            }
            doc.glue(" «");
            doc.piece(&items[h].text, h);
            doc.glue("»");
            s.lines.push(doc.text);
            s.gold.push(doc.gold);
        }
    }
    out.push(s);
    out
}

/// The streams as files: `<name>.txt` and `<name>.gold.tsv` (line, start, end, variety or glue, item id).
pub fn write(dir: &Path, items: &[Item], streams: &[Stream]) -> Result<()> {
    std::fs::create_dir_all(dir)?;
    let mut it = String::from("id\tvariety\tverse\ttext\n");
    for x in items {
        it += &format!("{}\t{}\t{}\t{}\n", x.id, VARIETIES[x.var].name, x.verse.clone().unwrap_or_default(), x.text);
    }
    std::fs::write(dir.join("items.tsv"), it)?;
    for s in streams {
        let mut txt = String::new();
        let mut g = String::from("line\tstart\tend\tvariety\titem\n");
        for (i, l) in s.lines.iter().enumerate() {
            txt += l;
            txt.push('\n');
            for sp in &s.gold[i] {
                let (v, id) = sp.item.map(|k| (VARIETIES[items[k].var].name, items[k].id.as_str())).unwrap_or(("glue", "-"));
                g += &format!("{i}\t{}\t{}\t{v}\t{id}\n", sp.start, sp.end);
            }
        }
        std::fs::write(dir.join(format!("{}.txt", s.name)), txt)?;
        std::fs::write(dir.join(format!("{}.gold.tsv", s.name)), g)?;
    }
    Ok(())
}
#[cfg(test)]
mod wave32_tests {
    use super::*;
    #[test]
    fn synthetic_stream_spans() {
        assert_eq!(
            sentences("One. Two? «Three!»"),
            ["One.", "Two?", "«Three!»"]
        );
        assert!(!usable("one two"));
        assert!(usable("one two three"));
        let items = vec![
            Item {
                id: "e".into(),
                var: 0,
                text: "One two three four.".into(),
                verse: None,
            },
            Item {
                id: "h".into(),
                var: 7,
                text: "אב גד הו".into(),
                verse: None,
            },
        ];
        let a = streams(&items, 7, 3);
        let b = streams(&items, 7, 3);
        assert_eq!(
            a.iter().map(|s| &s.lines).collect::<Vec<_>>(),
            b.iter().map(|s| &s.lines).collect::<Vec<_>>()
        );
        assert_eq!(a.last().unwrap().lines.len(), 3);
        for st in &a {
            for (line, spans) in st.lines.iter().zip(&st.gold) {
                let mut at = 0;
                for sp in spans {
                    assert_eq!(sp.start, at);
                    assert!(line.is_char_boundary(sp.start) && line.is_char_boundary(sp.end));
                    at = sp.end;
                }
                assert_eq!(at, line.len());
            }
        }
        assert_eq!(streams(&[], 0, 3).len(), 2);
    }
}
