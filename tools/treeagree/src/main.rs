//! treeagree — agreed UD trees for the school corpus.
//!
//! `treeagree agree <haiku.conllu> <parse run dir> <out.conllu> <report dir>`
//!   Haiku's silver tree is the default; the rules in `rules.rs` change words where the best open parsers
//!   (Stanza, Trankit, spaCy trf from `<run>/out/`) give written evidence. Edits are applied in rule order and
//!   kept only if the tree stays valid and gains no new gate violation (`tree.rs`); a rejected edit is logged.
//!   Changed words carry `Agree=<rule>|Haiku=<head>:<rel>` in MISC. Writes `<report dir>/edits.tsv`,
//!   `rejected.tsv`, `agree.md`.
//! `treeagree check <file.conllu>` — the verifier: gate violations of every tree in a file.
//! `TREEAGREE_LEX=<file.conllu>` — learn retag FEATS and lemmas from that corpus instead of the reference.

mod dispute;
mod rules;
mod tree;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use en::gram::Rel;
use en::udreg::Registry;

use rules::{Edit, Rule, View};
use tree::{Bad, Sent, Tok, Wit};

const PARSERS: [&str; 3] = ["stanza", "trankit", "spacy-trf"];

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let r = match a.get(1).map(String::as_str) {
        Some("agree") if a.len() == 6 => agree(&a[2], Path::new(&a[3]), &a[4], Path::new(&a[5])),
        Some("check") if a.len() == 3 => check(&a[2]),
        Some("dispute-ask") if a.len() >= 5 => dispute_ask(&a[2], Path::new(&a[3]), &a[4], a.get(5).and_then(|x| x.parse().ok()).unwrap_or(8)),
        Some("dispute-apply") if a.len() == 7 => dispute_apply(&a[2], Path::new(&a[3]), &a[4], &a[5], &a[6]),
        _ => Err("usage: treeagree agree <haiku.conllu> <run dir> <out.conllu> <report dir> | treeagree check <file.conllu> | treeagree dispute-ask <draft.conllu> <parsers dir> <prompts.jsonl> [per] | treeagree dispute-apply <draft.conllu> <parsers dir> <prompts.jsonl> <answers.jsonl> <out.conllu>".into()),
    };
    if let Err(e) = r {
        eprintln!("treeagree: {e}");
        std::process::exit(1);
    }
}

fn read(p: impl AsRef<Path>) -> Result<String, String> {
    fs::read_to_string(p.as_ref()).map_err(|e| format!("{}: {e}", p.as_ref().display()))
}

fn witnesses(run: &Path, n: usize) -> Result<Vec<Vec<Vec<Wit>>>, String> {
    let mut out = Vec::new();
    for p in ["stanza", "trankit"] {
        let w = tree::read_wit(&read(run.join(format!("out/{p}.conllu")))?);
        if w.len() != n {
            return Err(format!("{p}: {} sentences, the draft has {n}", w.len()));
        }
        out.push(w);
    }
    Ok(out)
}

/// `dispute-ask`: the teacher's questions — only disputed words, as choices (`dispute.rs`).
fn dispute_ask(draft: &str, run: &Path, out: &str, per: usize) -> Result<(), String> {
    let sents = tree::read_ref(&read(draft)?)?;
    let wits = witnesses(run, sents.len())?;
    let gs = dispute::groups(&sents, &wits, per);
    let mut t = String::new();
    let mut q = 0;
    for (k, (var, ids)) in gs.iter().enumerate() {
        q += var.lines().filter(|l| l.starts_with("  ")).count();
        let v = format!("{{\"id\":{k},\"sents\":[{}],\"fixed\":{},\"var\":{}}}", ids.iter().map(|x| json_str(x)).collect::<Vec<_>>().join(","), json_str(dispute::FIXED), json_str(var));
        t.push_str(&v);
        t.push('\n');
    }
    fs::write(out, t).map_err(|e| e.to_string())?;
    let n_disp: usize = gs.iter().map(|g| g.1.len()).sum();
    println!("{} sentences, {n_disp} with disputes, {q} disputed words, {} prompts → {out}", sents.len(), gs.len());
    Ok(())
}

fn json_str(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// The value of `"key":` in a one-line JSON object (string or array of strings) — enough for our own JSONL.
fn json_field(line: &str, key: &str) -> Option<String> {
    let at = line.find(&format!("\"{key}\":"))? + key.len() + 3;
    let rest = line[at..].trim_start();
    let mut out = String::new();
    let mut chars = rest.chars();
    match chars.next()? {
        '"' => {
            let mut esc = false;
            for c in chars {
                if esc {
                    out.push(match c {
                        'n' => '\n',
                        't' => '\t',
                        x => x,
                    });
                    esc = false;
                } else if c == '\\' {
                    esc = true;
                } else if c == '"' {
                    return Some(out);
                } else {
                    out.push(c);
                }
            }
            None
        }
        '[' => Some(rest[1..rest.find(']')?].to_string()),
        c => Some(std::iter::once(c).chain(chars.take_while(|x| x.is_ascii_digit())).collect()),
    }
}

/// `dispute-apply`: the teacher's choices on the draft, sentence by sentence through the gates.
fn dispute_apply(draft: &str, run: &Path, prompts: &str, answers: &str, out: &str) -> Result<(), String> {
    let reg = Registry::load();
    let mut sents = tree::read_ref(&read(draft)?)?;
    let wits = witnesses(run, sents.len())?;
    let disp = dispute::by_id(&sents, &wits);
    let groups: BTreeMap<String, Vec<String>> = read(prompts)?
        .lines()
        .filter_map(|l| Some((json_field(l, "id")?, json_field(l, "sents")?.split(',').map(|x| x.trim().trim_matches('"').to_string()).collect())))
        .collect();
    let mut picks: BTreeMap<String, Vec<(usize, usize)>> = BTreeMap::new();
    for l in read(answers)?.lines() {
        let (Some(id), Some(text)) = (json_field(l, "id"), json_field(l, "text")) else { continue };
        let Some(ids) = groups.get(&id) else { continue };
        for (n, w, k) in dispute::parse_answers(&text) {
            if let Some(sid) = ids.get(n.wrapping_sub(1)) {
                picks.entry(sid.clone()).or_default().push((w, k));
            }
        }
    }
    let (mut changed_words, mut changed_sents, mut asked) = (0, 0, 0);
    let mut text = String::new();
    for s in sents.iter_mut() {
        let ds = disp.get(&s.id).cloned().unwrap_or_default();
        asked += ds.len();
        let before: Vec<(usize, Rel)> = s.toks.iter().map(|t| (t.head, t.rel)).collect();
        let n = dispute::apply(s, &ds, picks.get(&s.id).map(Vec::as_slice).unwrap_or(&[]), &reg);
        changed_words += n;
        changed_sents += (n > 0) as usize;
        for c in &s.comments {
            writeln!(text, "{c}").unwrap();
        }
        for (j, t) in s.toks.iter().enumerate() {
            let mut c = t.cols.clone();
            c[6] = t.head.to_string();
            c[7] = t.rel.name().to_string();
            if (t.head, t.rel) != before[j] {
                c[9] = format!("Agree=teacher-choice|Before={}:{}", before[j].0, before[j].1);
            }
            writeln!(text, "{}", c.join("\t")).unwrap();
        }
        text.push('\n');
    }
    fs::write(out, text).map_err(|e| e.to_string())?;
    println!("{asked} disputed words; the teacher changed {changed_words} words in {changed_sents} sentences → {out}");
    Ok(())
}

/// The verifier.
fn check(file: &str) -> Result<(), String> {
    let reg = Registry::load();
    let sents = tree::read_ref(&read(file)?)?;
    let mut kinds: BTreeMap<&str, u64> = BTreeMap::new();
    let mut bad_sents = 0;
    for s in &sents {
        let heads: Vec<usize> = s.toks.iter().map(|t| t.head).collect();
        let rels: Vec<Rel> = s.toks.iter().map(|t| t.rel).collect();
        let v = tree::violations(&s.toks, &heads, &rels, &reg);
        bad_sents += !v.is_empty() as u64;
        for b in &v {
            *kinds.entry(b.kind()).or_default() += 1;
        }
    }
    println!("{file}: {} sentences, {} with gate violations", sents.len(), bad_sents);
    for (k, n) in kinds {
        println!("  {k}\t{n}");
    }
    Ok(())
}

/// Apply edits in order; each must keep the tree valid and bring no new violation. An edit that breaks the
/// shape alone (a cycle, a second root) is retried together with the other pending edits of the words involved:
/// swaps of head and dependent need both. An edit refused together with its retag is retried without it.
fn apply(s: &Sent, edits: &[Edit], reg: &Registry) -> (Vec<Tok>, Vec<usize>, Vec<Rel>, Vec<Edit>, Vec<(Edit, Bad)>) {
    let mut toks = s.toks.clone();
    let mut heads: Vec<usize> = s.toks.iter().map(|t| t.head).collect();
    let mut rels: Vec<Rel> = s.toks.iter().map(|t| t.rel).collect();
    let base = tree::violations(&s.toks, &heads, &rels, reg);
    let mut edits = edits.to_vec();
    let mut done = vec![false; edits.len()];
    let (mut kept, mut rejected) = (Vec::new(), Vec::new());
    for i in 0..edits.len() {
        if done[i] {
            continue;
        }
        let try_set = |edits: &[Edit], set: &[usize], toks: &[Tok], heads: &[usize], rels: &[Rel]| {
            let (mut t, mut h, mut r) = (toks.to_vec(), heads.to_vec(), rels.to_vec());
            for &k in set {
                let e = &edits[k];
                h[e.pos - 1] = e.head;
                r[e.pos - 1] = e.rel;
                if let Some(f) = &e.fix {
                    t[e.pos - 1].retag(f);
                }
            }
            tree::no_worse(&t, &h, &r, &base, reg).map(|_| (t, h, r))
        };
        let attempt = |edits: &[Edit], done: &[bool], toks: &[Tok], heads: &[usize], rels: &[Rel]| {
            let mut set = vec![i];
            let mut res = try_set(edits, &set, toks, heads, rels);
            if matches!(res, Err(Bad::Cycle(_) | Bad::Roots(_))) {
                // bundle: pending edits of the words on the new head's path to the root, and of other roots
                let mut words = Vec::new();
                let mut j = edits[i].head;
                for _ in 0..heads.len() {
                    if j == 0 || j == edits[i].pos {
                        break;
                    }
                    words.push(j);
                    j = heads[j - 1];
                }
                words.extend(heads.iter().enumerate().filter(|(_, &h)| h == 0).map(|(k, _)| k + 1));
                set.extend((0..edits.len()).filter(|&k| k != i && !done[k] && words.contains(&edits[k].pos)));
                if set.len() > 1 {
                    res = try_set(edits, &set, toks, heads, rels);
                }
            }
            (set, res)
        };
        let (mut set, mut res) = attempt(&edits, &done, &toks, &heads, &rels);
        if res.is_err() && edits[i].fix.is_some() && edits[i].rule != Rule::PosAgree {
            let mut plain = edits.clone();
            plain[i].fix = None;
            let (s2, r2) = attempt(&plain, &done, &toks, &heads, &rels);
            if r2.is_ok() {
                edits = plain;
                set = s2;
                res = r2;
            }
        }
        match res {
            Ok((t, h, r)) => {
                toks = t;
                heads = h;
                rels = r;
                for &k in &set {
                    done[k] = true;
                    kept.push(edits[k].clone());
                }
            }
            Err(b) => {
                done[i] = true;
                rejected.push((edits[i].clone(), b));
            }
        }
    }
    kept.sort_by_key(|e| e.pos);
    (toks, heads, rels, kept, rejected)
}

fn agree(haiku: &str, run: &Path, out: &str, rep: &Path) -> Result<(), String> {
    let reg = Registry::load();
    let sents = tree::read_ref(&read(haiku)?)?;
    // FEATS and lemmas for retagged words: from the reference itself, or from a bigger corpus (TREEAGREE_LEX) when the
    // reference is a small batch of drafts
    let lex = match std::env::var("TREEAGREE_LEX") {
        Ok(p) => tree::Lex::learn(&tree::read_ref(&read(&p)?)?),
        Err(_) => tree::Lex::learn(&sents),
    };
    let mut wits: Vec<Vec<Vec<Wit>>> = Vec::new();
    for p in PARSERS {
        let w = tree::read_wit(&read(run.join(format!("out/{p}.conllu")))?);
        if w.len() != sents.len() {
            return Err(format!("{p}: {} sentences, Haiku has {}", w.len(), sents.len()));
        }
        wits.push(w);
    }
    fs::create_dir_all(rep).map_err(|e| e.to_string())?;
    let mut text = String::new();
    let mut edits_tsv = String::from("sent_id\tcourse\tword\tform\trule\thaiku\tagreed\tstanza\ttrankit\tspacy_trf\ttext\n");
    let mut rej_tsv = String::from("sent_id\tcourse\tword\tform\trule\thaiku\tproposed\treason\ttext\n");
    let (mut proposed, mut kept_n, mut rej_n): (BTreeMap<Rule, u64>, BTreeMap<Rule, u64>, BTreeMap<Rule, u64>) = Default::default();
    let mut rej_why: BTreeMap<String, u64> = BTreeMap::new();
    let mut kept_rel: BTreeMap<(Rule, String), u64> = BTreeMap::new();
    let (mut skipped, mut changed, mut words) = (0u64, 0u64, 0u64);
    let mut course_edits: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    // UAS of Haiku and of the agreed trees against each witness: (haiku agrees, agreed agrees)
    let mut uas = [(0u64, 0u64); 3];
    let mut retagged = 0u64;
    let mut pos_moves: BTreeMap<String, u64> = BTreeMap::new();
    for (i, s) in sents.iter().enumerate() {
        let n = s.toks.len();
        words += n as u64;
        let ws: Vec<&[Wit]> = wits.iter().map(|w| w[i].as_slice()).collect();
        let aligned = ws.iter().all(|w| w.len() == n);
        let (toks, heads, rels, kept) = if aligned {
            let v = View { h: &s.toks, s: ws[0], t: ws[1], p: Some(ws[2]), lex: &lex };
            let edits = rules::propose(&v);
            for e in &edits {
                *proposed.entry(e.rule).or_default() += 1;
            }
            let (toks, heads, rels, kept, rejected) = apply(s, &edits, &reg);
            for (e, why) in &rejected {
                *rej_n.entry(e.rule).or_default() += 1;
                *rej_why.entry(format!("{} — {}", e.rule.id(), why.kind())).or_default() += 1;
                let t = &s.toks[e.pos - 1];
                writeln!(rej_tsv, "{}\t{}\t{}\t{}\t{}\t{}:{}:{}\t{}:{}:{}\t{}\t{}", s.id, s.course, e.pos, t.form(), e.rule.id(), t.head, t.rel, t.upos,
                    e.head, e.rel, e.fix.as_ref().map_or(t.upos, |f| f.upos), why, s.text).unwrap();
            }
            for e in &kept {
                *kept_n.entry(e.rule).or_default() += 1;
                let t = &s.toks[e.pos - 1];
                if e.rule != Rule::PosAgree {
                    *kept_rel.entry((e.rule, t.rel.base().name().to_string())).or_default() += 1;
                }
                if let Some(f) = &e.fix {
                    retagged += 1;
                    *pos_moves.entry(format!("{} → {}", t.upos, f.upos)).or_default() += 1;
                }
                let w = |k: usize| format!("{}:{}:{}", ws[k][e.pos - 1].head, ws[k][e.pos - 1].rel, ws[k][e.pos - 1].upos);
                writeln!(edits_tsv, "{}\t{}\t{}\t{}\t{}\t{}:{}:{}/{}\t{}:{}:{}\t{}\t{}\t{}\t{}", s.id, s.course, e.pos, t.form(), e.rule.id(), t.head, t.rel, t.upos, t.cols[4],
                    e.head, e.rel, e.fix.as_ref().map_or(format!("{}/{}", t.upos, t.cols[4]), |f| format!("{}/{}", f.upos, f.tag)), w(0), w(1), w(2), s.text).unwrap();
            }
            for (k, w) in ws.iter().enumerate() {
                for (j, x) in w.iter().enumerate() {
                    uas[k].0 += (x.head == s.toks[j].head) as u64;
                    uas[k].1 += (x.head == heads[j]) as u64;
                }
            }
            (toks, heads, rels, kept)
        } else {
            skipped += 1;
            (s.toks.clone(), s.toks.iter().map(|t| t.head).collect(), s.toks.iter().map(|t| t.rel).collect(), Vec::new())
        };
        let ce = course_edits.entry(s.course.clone()).or_default();
        ce.0 += n as u64;
        ce.1 += kept.len() as u64;
        changed += !kept.is_empty() as u64;
        // write the block
        for c in &s.comments {
            if c.starts_with("# annotation = ") {
                let by: BTreeMap<Rule, usize> = kept.iter().fold(BTreeMap::new(), |mut m, e| {
                    *m.entry(e.rule).or_default() += 1;
                    m
                });
                writeln!(text, "{c}; agreed by treeagree rules (Stanza, Trankit, spaCy trf){}", if kept.is_empty() { String::from(": no change") } else {
                    format!(": {}", by.iter().map(|(r, n)| format!("{} ×{n}", r.id())).collect::<Vec<_>>().join(", "))
                }).unwrap();
            } else {
                writeln!(text, "{c}").unwrap();
            }
        }
        for (j, t) in s.toks.iter().enumerate() {
            let mut c = toks[j].cols.clone();
            c[6] = heads[j].to_string();
            c[7] = rels[j].name().to_string();
            if let Some(e) = kept.iter().find(|e| e.pos == j + 1) {
                c[9] = format!("Agree={}|Haiku={}:{}", e.rule.id(), t.head, t.rel);
                if e.fix.is_some() {
                    write!(c[9], "|HaikuPos={}:{}:{}", t.upos, t.cols[4], t.cols[5]).unwrap();
                }
            }
            writeln!(text, "{}", c.join("\t")).unwrap();
        }
        text.push('\n');
    }
    fs::write(out, &text).map_err(|e| e.to_string())?;
    fs::write(rep.join("edits.tsv"), edits_tsv).map_err(|e| e.to_string())?;
    fs::write(rep.join("rejected.tsv"), rej_tsv).map_err(|e| e.to_string())?;

    // the verifier over what was written: no tree may be worse than Haiku's
    let back = tree::read_ref(&text)?;
    let (mut bad_h, mut bad_a) = (0u64, 0u64);
    for (s, a) in sents.iter().zip(&back) {
        let v = |x: &Sent| {
            let h: Vec<usize> = x.toks.iter().map(|t| t.head).collect();
            let r: Vec<Rel> = x.toks.iter().map(|t| t.rel).collect();
            tree::violations(&x.toks, &h, &r, &reg).len() as u64
        };
        bad_h += v(s);
        bad_a += v(a);
    }

    let pct = |a: u64, b: u64| if b == 0 { 0.0 } else { 100.0 * a as f64 / b as f64 };
    let mut md = String::new();
    writeln!(md, "# Agreed trees for the school\n").unwrap();
    writeln!(md, "Haiku silver `{haiku}` + Stanza, Trankit, spaCy trf (`{}`) → `{out}`.\n", run.display()).unwrap();
    writeln!(md, "- {} sentences, {} words; {} not aligned with a parser (left as Haiku).", sents.len(), words, skipped).unwrap();
    let total: u64 = kept_n.values().sum();
    writeln!(md, "- Changed: {} words ({:.2}%) in {} sentences ({:.1}%).", total, pct(total, words), changed, pct(changed, sents.len() as u64)).unwrap();
    writeln!(md, "- Gate violations (registry, shape, non-projective punct): Haiku {bad_h}, agreed {bad_a}.\n").unwrap();
    writeln!(md, "| rule | proposed | kept | rejected |\n|---|---|---|---|").unwrap();
    for r in Rule::ALL {
        let g = |m: &BTreeMap<Rule, u64>| m.get(&r).copied().unwrap_or(0);
        writeln!(md, "| {} | {} | {} | {} |", r.id(), g(&proposed), g(&kept_n), g(&rej_n)).unwrap();
    }
    writeln!(md, "\nRejections:\n").unwrap();
    for (k, n) in &rej_why {
        writeln!(md, "- {k}: {n}").unwrap();
    }
    let mut pm: Vec<_> = pos_moves.into_iter().collect();
    pm.sort_by(|a, b| b.1.cmp(&a.1));
    writeln!(md, "\nRetagged (UPOS, XPOS, FEATS, lemma — R8, alone or with the word's other edit): {retagged} words. Top: {}.",
        pm.iter().take(12).map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", ")).unwrap();
    writeln!(md, "\nHaiku relation of the changed word, top per rule:\n").unwrap();
    for r in Rule::ALL {
        let mut v: Vec<_> = kept_rel.iter().filter(|((rr, _), _)| *rr == r).map(|((_, rel), n)| (rel.as_str(), *n)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        writeln!(md, "- {}: {}", r.id(), v.iter().take(8).map(|(r, n)| format!("`{r}` {n}")).collect::<Vec<_>>().join(", ")).unwrap();
    }
    writeln!(md, "\nHead agreement with each parser (UAS, all words): Haiku → agreed.\n").unwrap();
    for (k, p) in PARSERS.iter().enumerate() {
        writeln!(md, "- {p}: {:.1} → {:.1}", pct(uas[k].0, words), pct(uas[k].1, words)).unwrap();
    }
    writeln!(md, "\n| course | words | changed % |\n|---|---|---|").unwrap();
    let mut cs: Vec<_> = course_edits.into_iter().collect();
    cs.sort_by(|a, b| pct(b.1 .1, b.1 .0).partial_cmp(&pct(a.1 .1, a.1 .0)).unwrap());
    for (c, (w, e)) in cs {
        writeln!(md, "| {c} | {w} | {:.2} |", pct(e, w)).unwrap();
    }
    fs::write(rep.join("agree.md"), &md).map_err(|e| e.to_string())?;
    print!("{md}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use en::gram::{Feats, UPos};

    fn tok(form: &str, upos: UPos, head: usize, rel: Rel) -> Tok {
        let cols = vec!["0", form, &form.to_lowercase(), upos.name(), "_", "_", "0", rel.name(), "_", "_"].into_iter().map(String::from).collect();
        Tok { cols, upos, feats: Feats::parse("_").0, head, rel }
    }
    fn w(head: usize, rel: &str) -> Wit {
        Wit { head, rel: rel.into(), ..Default::default() }
    }

    /// "cats , dogs and birds sleep" with Haiku's UD 1 habits: cc and comma on the first conjunct, conj chained.
    fn sent() -> Vec<Tok> {
        vec![
            tok("cats", UPos::NOUN, 6, Rel::Nsubj),
            tok(",", UPos::PUNCT, 1, Rel::Punct),
            tok("dogs", UPos::NOUN, 1, Rel::Conj),
            tok("and", UPos::CCONJ, 1, Rel::Cc),
            tok("birds", UPos::NOUN, 3, Rel::Conj),
            tok("sleep", UPos::VERB, 0, Rel::Root),
        ]
    }
    fn ud_parse() -> Vec<Wit> {
        vec![w(6, "nsubj"), w(3, "punct"), w(1, "conj"), w(5, "cc"), w(1, "conj"), w(0, "root")]
    }

    #[test]
    fn rules_fire_on_evidence_and_only_then() {
        let h = sent();
        let (s, t) = (ud_parse(), ud_parse());
        let p = vec![w(6, "nsubj"), w(1, "punct"), w(1, "conj"), w(3, "cc"), w(3, "conj"), w(0, "ROOT")];
        let lex = tree::Lex::default();
        let v = View { h: &h, s: &s, t: &t, p: Some(&p), lex: &lex };
        let e = rules::propose(&v);
        let got: Vec<(usize, usize, Rule)> = e.iter().map(|e| (e.pos, e.head, e.rule)).collect();
        assert_eq!(got, vec![(2, 3, Rule::PunctNext), (4, 5, Rule::CcNext), (5, 1, Rule::ConjFirst)]);
        // negative control: parsers agree with Haiku → nothing fires
        let mirror: Vec<Wit> = h.iter().map(|t| w(t.head, t.rel.name())).collect();
        let v = View { h: &h, s: &mirror, t: &mirror, p: Some(&mirror), lex: &lex };
        assert!(rules::propose(&v).is_empty());
        // R4 needs spaCy too: without the third witness the subject move does not fire
        let mut s2 = mirror.clone();
        s2[0] = w(3, "nsubj");
        let v = View { h: &h, s: &s2, t: &s2, p: Some(&mirror), lex: &lex };
        assert!(rules::propose(&v).is_empty());
    }

    #[test]
    fn gates_reject_bad_trees() {
        let reg = Registry::load();
        let h = sent();
        let heads: Vec<usize> = h.iter().map(|t| t.head).collect();
        let rels: Vec<Rel> = h.iter().map(|t| t.rel).collect();
        assert!(tree::violations(&h, &heads, &rels, &reg).is_empty());
        // root relation without head 0
        let mut nohead = heads.clone();
        nohead[5] = 1;
        assert_eq!(tree::shape(&nohead, &rels), Err(Bad::RootRel(6)));
        // no root left (cats → sleep → cats is a cycle)
        let mut r2 = rels.clone();
        r2[5] = Rel::Obj;
        assert_eq!(tree::shape(&nohead, &r2), Err(Bad::Roots(0)));
        // crossing arcs: b → d and c → a interleave
        let cross = vec![0usize, 4, 1, 1];
        assert!(tree::crossed(&cross, 3));
        assert!(!tree::crossed(&heads, 2));
        // a comma reaching over "dogs" (on "cats") to "sleep" crosses nothing; one from "dogs" to "sleep"
        // while "birds" hangs on "dogs" is crossed
        let mut np = heads.clone();
        np[3] = 6; // "and" → "sleep", spans 4..6, while "birds" → "dogs" spans 3..5
        assert!(tree::crossed(&np, 4));
        // cop with a lemma other than "be" is refused by the registry
        let mut bad = rels.clone();
        bad[0] = Rel::Cop;
        assert!(tree::violations(&h, &heads, &bad, &reg).iter().any(|b| matches!(b, Bad::Registry(1, _))));
        // amod on a NOUN does not fit its tag
        let mut am = rels.clone();
        am[2] = Rel::Amod;
        assert!(tree::violations(&h, &heads, &am, &reg).contains(&Bad::PosRel(3)));
    }

    #[test]
    fn swap_needs_both_edits() {
        let reg = Registry::load();
        // "run fast": Haiku makes "fast" the root; both edits together turn it round
        let h = vec![tok("run", UPos::VERB, 2, Rel::Advcl), tok("fast", UPos::ADV, 0, Rel::Root)];
        let s = Sent { comments: vec![], id: "t".into(), course: "c".into(), text: "run fast".into(), toks: h };
        let edits = vec![
            Edit { pos: 1, head: 0, rel: Rel::Root, rule: Rule::Strong, fix: None },
            Edit { pos: 2, head: 1, rel: Rel::Advmod, rule: Rule::Strong, fix: None },
        ];
        let (_, heads, _, kept, rej) = apply(&s, &edits, &reg);
        assert_eq!(heads, vec![0, 1]);
        assert_eq!(kept.len(), 2);
        assert!(rej.is_empty());
        // alone, the first edit makes two roots and is refused
        let (_, heads, _, kept, rej) = apply(&s, &edits[..1], &reg);
        assert_eq!(heads, vec![2, 0]);
        assert!(kept.is_empty() && rej.len() == 1);
    }

    #[test]
    fn retag_rides_with_the_relation() {
        let reg = Registry::load();
        // "Desert foxes run": Haiku tags "Desert" ADJ/JJ with amod; both parsers say NOUN/NN compound
        let mut desert = tok("Desert", UPos::ADJ, 2, Rel::Amod);
        desert.cols[4] = "JJ".into();
        let mut foxes = tok("foxes", UPos::NOUN, 3, Rel::Nsubj);
        foxes.cols[4] = "NNS".into();
        let mut sand = tok("sand", UPos::NOUN, 3, Rel::Obj);
        sand.cols[4] = "NN".into();
        sand.cols[5] = "Number=Sing".into();
        let run = tok("run", UPos::VERB, 0, Rel::Root);
        let h = vec![desert, foxes, run.clone()];
        let lex = tree::Lex::learn(&[Sent { comments: vec![], id: "l".into(), course: "c".into(), text: String::new(), toks: vec![sand, run] }]);
        let tag = |u: &str, x: &str, head, rel: &str| Wit { upos: u.into(), xpos: x.into(), head, rel: rel.into() };
        let s = vec![tag("NOUN", "NN", 2, "compound"), tag("NOUN", "NNS", 3, "nsubj"), tag("VERB", "VBP", 0, "root")];
        let v = View { h: &h, s: &s, t: &s, p: Some(&s), lex: &lex };
        let e = rules::propose(&v);
        assert_eq!(e.len(), 1);
        assert_eq!((e[0].rule, e[0].rel), (Rule::RelAgree, Rel::Compound));
        let f = e[0].fix.as_ref().expect("retag");
        assert_eq!((f.upos, f.feats.as_str(), f.lemma.as_str()), (UPos::NOUN, "Number=Sing", "desert"));
        let sent = Sent { comments: vec![], id: "t".into(), course: "c".into(), text: String::new(), toks: h.clone() };
        let (toks, _, rels, kept, rej) = apply(&sent, &e, &reg);
        assert!(rej.is_empty() && kept.len() == 1);
        assert_eq!((toks[0].upos, rels[0]), (UPos::NOUN, Rel::Compound));
        // negative control: the parsers disagree on the tag → no retag; the relation still moves (compound fits ADJ)
        let mut t2 = s.clone();
        t2[0].upos = "ADJ".into();
        t2[0].xpos = "JJ".into();
        let v = View { h: &h, s: &s, t: &t2, p: Some(&s), lex: &lex };
        let e = rules::propose(&v);
        assert!(e.iter().all(|e| e.fix.is_none()));
        let (_, _, rels, _, rej) = apply(&sent, &e, &reg);
        assert_eq!(rels[0], Rel::Compound);
        assert!(rej.is_empty());
    }
}
