//! Compare many parsers with the reference (Haiku) trees of the school corpus.
//!
//!   parsecmp <run dir>
//!
//! The run dir holds `in/haiku.conllu` (reference), `in/course.txt` (course per sentence, same order) and
//! `out/` with the parser outputs (see `SYSTEMS`, `CONSTITUENCY`). Sentences are aligned by order; a
//! sentence whose token count differs from the reference is skipped for that parser and counted.
//! Writes `report.md`, `suspects.tsv` (tokens where a strong majority of parsers disagrees with Haiku)
//! and `per-course.tsv` into the run dir.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

/// Dependency systems: (name, path under out/, labels are UD).
/// Only the strong ones (design note, 15.10: slow and weak parsers dropped — UDPipe 1, CoreNLP, Malt, spaCy sm/md).
const SYSTEMS: &[(&str, &str, bool)] = &[
    ("stanza", "stanza.conllu", true),
    ("trankit", "trankit.conllu", true),
    ("spacy-lg", "spacy-lg.conllu", false),
    ("spacy-trf", "spacy-trf.conllu", false),
    ("supar-dep", "supar-dep.conllu", false),
    ("supar-crf2o", "supar-crf2o.conllu", false),
];

/// Constituency systems: one bracketed tree per line.
const CONSTITUENCY: &[(&str, &str)] = &[
    ("stanza-con", "stanza-con.ptb"),
    ("benepar", "benepar.ptb"),
    ("supar-con", "supar-con.ptb"),
];

/// Share of available systems that must agree on a head other than Haiku's to flag a token.
const STRONG: f64 = 0.75;

#[derive(Clone)]
struct Tok {
    form: String,
    upos: String,
    head: u32,
    rel: String,
}

struct Sent {
    id: String,
    text: String,
    toks: Vec<Tok>,
}

fn read_conllu(path: &Path) -> Vec<Sent> {
    let data = fs::read_to_string(path).unwrap_or_default();
    let mut out = Vec::new();
    let mut cur = Sent { id: String::new(), text: String::new(), toks: Vec::new() };
    for line in data.lines() {
        if line.is_empty() {
            if !cur.toks.is_empty() {
                out.push(std::mem::replace(&mut cur, Sent { id: String::new(), text: String::new(), toks: Vec::new() }));
            }
            continue;
        }
        if let Some(v) = line.strip_prefix("# sent_id = ") {
            cur.id = v.to_string();
        } else if let Some(v) = line.strip_prefix("# text = ") {
            cur.text = v.to_string();
        } else if !line.starts_with('#') {
            let c: Vec<&str> = line.split('\t').collect();
            if c.len() < 8 || c[0].contains('-') || c[0].contains('.') {
                continue;
            }
            cur.toks.push(Tok {
                form: c[1].to_string(),
                upos: c[3].to_string(),
                head: c[6].parse().unwrap_or(u32::MAX),
                rel: c[7].to_lowercase(),
            });
        }
    }
    if !cur.toks.is_empty() {
        out.push(cur);
    }
    out
}

/// Universal part of a relation: `acl:relcl` → `acl`.
fn base(rel: &str) -> &str {
    rel.split(':').next().unwrap_or(rel)
}

#[derive(Default, Clone, Copy)]
struct Score {
    n: u64,
    uas: u64,
    las: u64,
    n_np: u64,
    uas_np: u64,
}

impl Score {
    fn add(&mut self, g: &Tok, s: &Tok, ud: bool) {
        let h = g.head == s.head;
        self.n += 1;
        self.uas += h as u64;
        self.las += (h && ud && base(&g.rel) == base(&s.rel)) as u64;
        if g.upos != "PUNCT" {
            self.n_np += 1;
            self.uas_np += h as u64;
        }
    }
}

fn pct(a: u64, b: u64) -> String {
    if b == 0 { "—".into() } else { format!("{:.1}", 100.0 * a as f64 / b as f64) }
}

/// Labelled spans of a bracketed tree, without preterminals and the root wrapper.
fn spans(tree: &str) -> Vec<(String, u32, u32)> {
    let mut out = Vec::new();
    let mut stack: Vec<(String, u32, bool)> = Vec::new(); // label, start, has child phrase
    let mut pos = 0u32;
    let chars: Vec<char> = tree.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '(' => {
                let mut j = i + 1;
                while j < chars.len() && !chars[j].is_whitespace() && chars[j] != '(' && chars[j] != ')' {
                    j += 1;
                }
                let label: String = chars[i + 1..j].iter().collect();
                let label = label.split(['-', '=']).next().unwrap_or("").to_string();
                if let Some(top) = stack.last_mut() {
                    top.2 = true;
                }
                stack.push((label, pos, false));
                i = j;
            }
            ')' => {
                if let Some((label, start, phrase)) = stack.pop() {
                    if phrase && !stack.is_empty() && !matches!(label.as_str(), "ROOT" | "TOP" | "") {
                        out.push((label, start, pos));
                    }
                }
                i += 1;
            }
            c if c.is_whitespace() => i += 1,
            _ => {
                while i < chars.len() && !chars[i].is_whitespace() && chars[i] != ')' {
                    i += 1;
                }
                pos += 1;
            }
        }
    }
    out.sort();
    out
}

fn bracket_f1(a: &[(String, u32, u32)], b: &[(String, u32, u32)]) -> (u64, u64, u64) {
    let mut ma: BTreeMap<&(String, u32, u32), i64> = BTreeMap::new();
    for s in a {
        *ma.entry(s).or_default() += 1;
    }
    let mut hit = 0u64;
    for s in b {
        if let Some(c) = ma.get_mut(s) {
            if *c > 0 {
                *c -= 1;
                hit += 1;
            }
        }
    }
    (hit, a.len() as u64, b.len() as u64)
}

fn main() {
    let run = PathBuf::from(std::env::args().nth(1).expect("usage: parsecmp <run dir>"));
    let gold = read_conllu(&run.join("in/haiku.conllu"));
    let courses: Vec<String> = fs::read_to_string(run.join("in/course.txt")).unwrap().lines().map(String::from).collect();
    assert_eq!(gold.len(), courses.len(), "in/course.txt and in/haiku.conllu differ in length");

    // Load systems; keep only those with a full output.
    let mut sys: Vec<(&str, bool, Vec<Option<Vec<Tok>>>, u64)> = Vec::new();
    for &(name, file, ud) in SYSTEMS {
        let s = read_conllu(&run.join("out").join(file));
        if s.len() != gold.len() {
            eprintln!("{name}: {} sentences, reference has {} — skipped", s.len(), gold.len());
            continue;
        }
        let mut miss = 0;
        let aligned = s
            .into_iter()
            .zip(&gold)
            .map(|(s, g)| if s.toks.len() == g.toks.len() { Some(s.toks) } else { miss += 1; None })
            .collect();
        sys.push((name, ud, aligned, miss));
    }

    let mut report = String::new();
    writeln!(report, "# School corpus: open parsers vs Haiku trees\n").unwrap();
    writeln!(report, "Reference: `in/haiku.conllu`, {} sentences, {} tokens. Sentences aligned by order and token count.\n",
        gold.len(), gold.iter().map(|s| s.toks.len()).sum::<usize>()).unwrap();

    // 1. Each system vs Haiku.
    writeln!(report, "## Each parser vs Haiku\n").unwrap();
    writeln!(report, "UAS and LAS over all tokens; LAS only for parsers with UD labels (universal part of the relation). `UAS−p` excludes punctuation.\n").unwrap();
    writeln!(report, "| parser | UD labels | UAS | UAS−p | LAS | sentences with exact tree | skipped (token mismatch) |\n|---|---|---|---|---|---|---|").unwrap();
    let mut per_course: BTreeMap<(String, &str), Score> = BTreeMap::new();
    let mut rel_recall: BTreeMap<(&str, String), (u64, u64)> = BTreeMap::new();
    for (name, ud, al, miss) in &sys {
        let mut sc = Score::default();
        let mut exact = 0u64;
        for (i, s) in al.iter().enumerate() {
            let Some(s) = s else { continue };
            let g = &gold[i];
            let mut all = true;
            let pc = per_course.entry((courses[i].clone(), name)).or_default();
            for (gt, st) in g.toks.iter().zip(s) {
                sc.add(gt, st, *ud);
                pc.add(gt, st, *ud);
                let ok = gt.head == st.head && (!*ud || base(&gt.rel) == base(&st.rel));
                all &= ok;
                if *ud {
                    let e = rel_recall.entry((name, base(&gt.rel).to_string())).or_default();
                    e.0 += 1;
                    e.1 += (gt.head == st.head && base(&gt.rel) == base(&st.rel)) as u64;
                }
            }
            exact += all as u64;
        }
        writeln!(report, "| {name} | {} | {} | {} | {} | {} | {miss} |", if *ud { "yes" } else { "no" },
            pct(sc.uas, sc.n), pct(sc.uas_np, sc.n_np), if *ud { pct(sc.las, sc.n) } else { "—".into() },
            pct(exact, gold.len() as u64 - miss)).unwrap();
    }

    // 2. Majority of parsers vs Haiku, and suspects.
    let mut sus = String::from("sent_id\tcourse\ttoken\tform\thaiku_head\thaiku_rel\tmajority_head\tsupport\tvoters\tspacy_trf\ttext\n");
    let trf = sys.iter().find(|s| s.0 == "spacy-trf" && s.2.iter().any(|t| t.is_some())).map(|s| &s.2);
    let mut trf_ok = 0u64;
    let mut trf_rel: BTreeMap<String, u64> = BTreeMap::new();
    let (mut voted, mut maj_agree, mut flagged) = (0u64, 0u64, 0u64);
    let mut flagged_sents: BTreeMap<String, u64> = BTreeMap::new();
    let mut flagged_rel: BTreeMap<String, u64> = BTreeMap::new();
    let mut flagged_course: BTreeMap<String, u64> = BTreeMap::new();
    let mut toks_course: BTreeMap<String, u64> = BTreeMap::new();
    // Recurring Haiku habits among the suspects: (cc to the left conjunct, conj chained, punct, root)
    let (mut cc_left, mut conj_chain, mut punct_s, mut root_s) = (0u64, 0u64, 0u64, 0u64);
    for (i, g) in gold.iter().enumerate() {
        *toks_course.entry(courses[i].clone()).or_default() += g.toks.len() as u64;
        for (k, gt) in g.toks.iter().enumerate() {
            let mut votes: BTreeMap<u32, u32> = BTreeMap::new();
            let mut n = 0u32;
            for (_, ud, al, _) in &sys {
                // Only UD parsers vote (Stanza, Trankit): the others use Stanford/ClearNLP head rules (case, cop, aux heads differ).
                if !*ud {
                    continue;
                }
                if let Some(s) = &al[i] {
                    *votes.entry(s[k].head).or_default() += 1;
                    n += 1;
                }
            }
            if n < 2 {
                continue;
            }
            voted += 1;
            let (&mh, &mc) = votes.iter().max_by_key(|(h, c)| (**c, std::cmp::Reverse(**h))).unwrap();
            maj_agree += (mh == gt.head) as u64;
            if mh != gt.head && mc as f64 >= STRONG * n as f64 {
                flagged += 1;
                *flagged_sents.entry(g.id.clone()).or_default() += 1;
                *flagged_rel.entry(base(&gt.rel).to_string()).or_default() += 1;
                *flagged_course.entry(courses[i].clone()).or_default() += 1;
                let pos = k as u32 + 1;
                let rel = base(&gt.rel);
                if rel == "cc" && gt.head < pos && mh > pos {
                    cc_left += 1;
                } else if rel == "conj" && gt.head > 0 && base(&g.toks[gt.head as usize - 1].rel) == "conj"
                    && g.toks[gt.head as usize - 1].head == mh {
                    conj_chain += 1;
                } else if rel == "punct" {
                    punct_s += 1;
                } else if gt.head == 0 || mh == 0 {
                    root_s += 1;
                }
                // Third witness: spaCy trf (ClearNLP rules). A pobj hangs on its preposition there; UD hangs it on the preposition's head.
                let trf = trf.and_then(|t| t[i].as_ref()).map(|t| {
                    let h = t[k].head;
                    if t[k].rel == "pobj" && h > 0 { t[h as usize - 1].head } else { h }
                });
                let confirmed = trf == Some(mh);
                trf_ok += confirmed as u64;
                if confirmed {
                    *trf_rel.entry(base(&gt.rel).to_string()).or_default() += 1;
                }
                writeln!(sus, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}", g.id, courses[i], k + 1, gt.form, gt.head, gt.rel, mh, mc, n,
                    match trf { None => "-", Some(_) if confirmed => "yes", Some(_) => "no" }, g.text).unwrap();
            }
        }
    }
    writeln!(report, "\n## Majority of parsers vs Haiku\n").unwrap();
    writeln!(report, "Head chosen by most UD parsers per token ({} UD parsers vote; spaCy and SuPar use other head rules and do not vote).\n", sys.iter().filter(|s| s.1).count()).unwrap();
    writeln!(report, "- Majority head equals Haiku's: {}% of {} tokens.", pct(maj_agree, voted), voted).unwrap();
    writeln!(report, "- **Suspects**: at least {:.0}% of parsers agree on another head — {} tokens ({}%) in {} sentences ({}% of sentences). List: `suspects.tsv`.",
        STRONG * 100.0, flagged, pct(flagged, voted), flagged_sents.len(), pct(flagged_sents.len() as u64, gold.len() as u64)).unwrap();
    let mut fr: Vec<_> = flagged_rel.into_iter().collect();
    fr.sort_by(|a, b| b.1.cmp(&a.1));
    writeln!(report, "- Haiku relations most often contradicted: {}.",
        fr.iter().take(12).map(|(r, c)| format!("`{r}` {c}")).collect::<Vec<_>>().join(", ")).unwrap();
    if trf.is_some() {
        let mut tr: Vec<_> = trf_rel.into_iter().collect();
        tr.sort_by(|a, b| b.1.cmp(&a.1));
        writeln!(report, "- **Confirmed by spaCy trf** (a third parser with other head rules picks the same head; `pobj` lifted to the preposition's head): {} tokens ({}% of suspects); `spacy_trf` column in `suspects.tsv`. Top: {}.",
            trf_ok, pct(trf_ok, flagged), tr.iter().take(10).map(|(r, c)| format!("`{r}` {c}")).collect::<Vec<_>>().join(", ")).unwrap();
    }

    writeln!(report, "\nSuspects by kind (a token counts once, first matching kind):\n").unwrap();
    writeln!(report, "| kind | tokens | share of suspects |\n|---|---|---|").unwrap();
    let other = flagged - cc_left - conj_chain - punct_s - root_s;
    for (k, n) in [
        ("`cc` on the left conjunct (UD 2 puts it on the following one)", cc_left),
        ("`conj` chained to the previous conjunct (UD 2: all on the first)", conj_chain),
        ("`punct` attachment", punct_s),
        ("root of the sentence differs", root_s),
        ("other attachments", other),
    ] {
        writeln!(report, "| {k} | {n} | {} |", pct(n, flagged)).unwrap();
    }

    // 3. Per course.
    writeln!(report, "\n## Per course\n").unwrap();
    writeln!(report, "Suspect rate = suspect tokens / tokens; then UAS−p of each parser on that course. Middle-school courses (programming, philosophy) are marked *.\n").unwrap();
    let mut hdr = String::from("| course | sentences | suspects % |");
    let mut sep = String::from("|---|---|---|");
    for (name, ..) in &sys {
        hdr.push_str(&format!(" {name} |"));
        sep.push_str("---|");
    }
    writeln!(report, "{hdr}\n{sep}").unwrap();
    let mut sents_course: BTreeMap<&str, u64> = BTreeMap::new();
    for c in &courses {
        *sents_course.entry(c).or_default() += 1;
    }
    let mut tsv = String::from("course\tparser\ttokens\tuas\tuas_nopunct\tlas\n");
    let mut cs: Vec<_> = sents_course.iter().collect();
    cs.sort_by(|a, b| b.1.cmp(a.1));
    for (c, n) in cs {
        let fl = *flagged_course.get(*c).unwrap_or(&0);
        let mark = if matches!(*c, "programming" | "philosophy") { "*" } else { "" };
        let mut row = format!("| {c}{mark} | {n} | {} |", pct(fl, toks_course[*c]));
        for (name, ud, ..) in &sys {
            let s = per_course.get(&(c.to_string(), name)).copied().unwrap_or_default();
            row.push_str(&format!(" {} |", pct(s.uas_np, s.n_np)));
            writeln!(tsv, "{c}\t{name}\t{}\t{}\t{}\t{}", s.n, pct(s.uas, s.n), pct(s.uas_np, s.n_np),
                if *ud { pct(s.las, s.n) } else { "—".into() }).unwrap();
        }
        writeln!(report, "{row}").unwrap();
    }

    // 4. Relation recall for UD parsers.
    writeln!(report, "\n## Haiku relations: how often UD parsers reproduce them (head + relation)\n").unwrap();
    let ud_sys: Vec<&str> = sys.iter().filter(|s| s.1).map(|s| s.0).collect();
    let mut rels: BTreeMap<String, u64> = BTreeMap::new();
    for g in &gold {
        for t in &g.toks {
            *rels.entry(base(&t.rel).to_string()).or_default() += 1;
        }
    }
    let mut rv: Vec<_> = rels.into_iter().collect();
    rv.sort_by(|a, b| b.1.cmp(&a.1));
    writeln!(report, "| relation | count | {} |\n|---|---|{}", ud_sys.join(" | "), "---|".repeat(ud_sys.len())).unwrap();
    for (r, n) in rv.iter().take(30) {
        let cells: Vec<String> = ud_sys.iter().map(|s| {
            let (a, b) = rel_recall.get(&(*s, r.clone())).copied().unwrap_or((0, 0));
            pct(b, a)
        }).collect();
        writeln!(report, "| `{r}` | {n} | {} |", cells.join(" | ")).unwrap();
    }

    // 5. Pairwise UAS between parsers and Haiku.
    writeln!(report, "\n## Pairwise head agreement (UAS of row vs column, tokens of sentences both have)\n").unwrap();
    let names: Vec<&str> = std::iter::once("haiku").chain(sys.iter().map(|s| s.0)).collect();
    writeln!(report, "| | {} |\n|---|{}", names.join(" | "), "---|".repeat(names.len())).unwrap();
    let get = |k: usize, i: usize| -> Option<&Vec<Tok>> {
        if k == 0 { Some(&gold[i].toks) } else { sys[k - 1].2[i].as_ref() }
    };
    for a in 0..names.len() {
        let mut row = format!("| {} |", names[a]);
        for b in 0..names.len() {
            let (mut n, mut h) = (0u64, 0u64);
            if a != b {
                for i in 0..gold.len() {
                    if let (Some(x), Some(y)) = (get(a, i), get(b, i)) {
                        for (p, q) in x.iter().zip(y) {
                            n += 1;
                            h += (p.head == q.head) as u64;
                        }
                    }
                }
            }
            row.push_str(&format!(" {} |", if a == b { "—".into() } else { pct(h, n) }));
        }
        writeln!(report, "{row}").unwrap();
    }

    // 6. Constituency parsers among themselves.
    let cons: Vec<(&str, Vec<String>)> = CONSTITUENCY.iter().filter_map(|(n, f)| {
        let d = fs::read_to_string(run.join("out").join(f)).ok()?;
        let l: Vec<String> = d.lines().map(String::from).collect();
        (l.len() == gold.len()).then_some((*n, l))
    }).collect();
    if cons.len() >= 2 {
        writeln!(report, "\n## Constituency parsers: labelled bracket F1 between each pair\n").unwrap();
        writeln!(report, "| | {} |\n|---|{}", cons.iter().map(|c| c.0).collect::<Vec<_>>().join(" | "), "---|".repeat(cons.len())).unwrap();
        let sp: Vec<Vec<Vec<(String, u32, u32)>>> = cons.iter().map(|c| c.1.iter().map(|t| spans(t)).collect()).collect();
        for a in 0..cons.len() {
            let mut row = format!("| {} |", cons[a].0);
            for b in 0..cons.len() {
                if a == b {
                    row.push_str(" — |");
                    continue;
                }
                let (mut h, mut na, mut nb) = (0, 0, 0);
                for i in 0..gold.len() {
                    let (x, y, z) = bracket_f1(&sp[a][i], &sp[b][i]);
                    h += x;
                    na += y;
                    nb += z;
                }
                let f = if na + nb == 0 { 0.0 } else { 200.0 * h as f64 / (na + nb) as f64 };
                row.push_str(&format!(" {f:.1} |"));
            }
            writeln!(report, "{row}").unwrap();
        }
    }

    // 7. Rule-based coverage: ERG (MRS) and Link Grammar.
    writeln!(report, "\n## Rule-based parsers: coverage\n").unwrap();
    let mut erg = (0u64, 0u64);
    if let Ok(rd) = fs::read_dir(run.join("out/erg")) {
        for e in rd.flatten() {
            let d = fs::read_to_string(e.path()).unwrap_or_default();
            for l in d.lines() {
                if l.starts_with("SENT: ") { erg.0 += 1 }
                if l.starts_with("SKIP: ") { erg.1 += 1 }
            }
        }
    }
    writeln!(report, "- ACE + ERG 2018: parsed {} of {} attempted ({}%), timeout 20 s per sentence (a sample: the run was stopped as too slow).", erg.0, erg.0 + erg.1, pct(erg.0, erg.0 + erg.1)).unwrap();
    let (mut full, mut part) = (0u64, 0u64);
    if let Ok(rd) = fs::read_dir(run.join("out/lg")) {
        for e in rd.flatten() {
            let d = fs::read_to_string(e.path()).unwrap_or_default();
            for l in d.lines() {
                if let Some(r) = l.trim_start().strip_prefix("Linkage 1, cost vector = (UNUSED=") {
                    if r.starts_with("0 ") { full += 1 } else { part += 1 }
                }
            }
        }
    }
    let none = (gold.len() as u64).saturating_sub(full + part);
    writeln!(report, "- Link Grammar 5.12.5: complete linkage for {} of {} sentences ({}%); {} need null links (words left out); {} got no linkage at all (timeout 10 s).",
        full, gold.len(), pct(full, gold.len() as u64), part, none).unwrap();

    fs::write(run.join("report.md"), report).unwrap();
    fs::write(run.join("suspects.tsv"), sus).unwrap();
    fs::write(run.join("per-course.tsv"), tsv).unwrap();
    eprintln!("wrote report.md, suspects.tsv, per-course.tsv");
}
