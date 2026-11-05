//! Re-checking the list with each new gparse model, and measuring it before and after an Eel round.
//!
//! **The judge** of one tree, without gold (the Wikipedia corpus's judges, applied to gparse):
//! `roots` (not exactly one root), `head_outside`, `grammar` (a UD English grammar constraint broken,
//! `world::induce::violations`), `nonprojective` (never for gparse, whose Eisner decoder is
//! projective; kept for the drafts), and `witness` (Stanza, run once on the same tokens, attaches
//! fewer than 80 % of the non-punctuation words to the same heads). A tree that none flags is read
//! well. This measures disagreement and broken constraints, not correctness.
//!
//! **The fixed sample** (`<lane_root>/measure/sample.jsonl`): built once and never changed, from the
//! cases with a draft tree, the `measure` split first, then by key; Stanza's heads stored with it.
//! A new sample needs a new name.
//!
//! **A measurement** of a model directory: gparse (CPU) on the sample's tokens, the judge's pass
//! rate (the drafts' own rate beside it), UAS/LAS against the validated Fish trees of sample cases
//! (silver, review-only; reported for the never-trained `measure` split apart), and LAS on the
//! pinned EWT dev. Each run lands in `measure/runs/` and one line in `measure/log.tsv`.
//!
//! **Recheck**: every case open for gparse (and, so regressions show, every case it learned) with a
//! draft tree is judged with the deployed gparse; read well → `learned` for gparse, failing a
//! learned case again → `open`. The `en` view (the corpora's own rechecks) is never touched.
use crate::config::Config;
use crate::{read_json, sha, write_atomic};
use anyhow::{ensure, Context, Result};
use en::annotate::Word;
use en::gram::{Feats, Rel, Tag, UPos};
use hard_case_list::{Case, GPARSE};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const WITNESS_UAS: f64 = 0.8;

/// (form, upos, head, rel) per word of a block.
pub fn words(block: &str) -> Vec<(String, String, usize, String)> {
    crate::rows(block).iter().map(|c| (c[1].to_string(), c[3].to_string(), c[6].parse().unwrap_or(usize::MAX), c[7].to_string())).collect()
}

/// The judge's reasons for one tree; `witness`: Stanza's heads on the same tokens, when known.
pub fn judge(block: &str, witness: Option<&[usize]>) -> Vec<String> {
    let w = words(block);
    let mut k = vec![];
    if w.is_empty() {
        return vec!["no_words".into()];
    }
    if w.iter().filter(|x| x.2 == 0).count() != 1 {
        k.push("roots".to_string());
    }
    if w.iter().any(|x| x.2 > w.len()) {
        k.push("head_outside".to_string());
        return k;
    }
    let ws: Vec<Word> = crate::rows(block)
        .iter()
        .map(|c| Word { form: c[1].into(), lemma: c[2].into(), upos: UPos::parse(c[3]).unwrap_or(UPos::X), tag: Tag::parse(c[4]).unwrap_or(Tag::NN), feats: Feats::default(), head: c[6].parse().unwrap_or(0), rel: Rel::parse(c[7]).unwrap_or(Rel::Dep) })
        .collect();
    if world::induce::violations(&ws) > 0 {
        k.push("grammar".into());
    }
    if !roundtrip::tree::projective(&roundtrip::tree::of_words(&ws)) {
        k.push("nonprojective".into());
    }
    if let Some(h) = witness {
        if let Some(u) = uas(&w, h) {
            if u < WITNESS_UAS {
                k.push("witness".into());
            }
        }
    }
    k
}

/// Unlabelled agreement over non-punctuation words (None when the lengths differ or nothing compares).
pub fn uas(w: &[(String, String, usize, String)], heads: &[usize]) -> Option<f64> {
    if w.len() != heads.len() {
        return None;
    }
    let (mut n, mut same) = (0, 0);
    for (x, h) in w.iter().zip(heads) {
        if x.1 == "PUNCT" {
            continue;
        }
        n += 1;
        same += usize::from(x.2 == *h);
    }
    (n > 0).then(|| same as f64 / n as f64)
}

/// (UAS words agreeing, LAS words agreeing, words) of a tree against a reference on the same forms.
pub fn attach(t: &str, gold: &str) -> Option<(usize, usize, usize)> {
    let (a, b) = (words(t), words(gold));
    if a.len() != b.len() || a.iter().zip(&b).any(|(x, y)| x.0 != y.0) {
        return None;
    }
    let u = a.iter().zip(&b).filter(|(x, y)| x.2 == y.2).count();
    let l = a.iter().zip(&b).filter(|(x, y)| x.2 == y.2 && x.3 == y.3).count();
    Some((u, l, a.len()))
}

/// Stanza's heads for blocks (CPU, the existing bridge on Mova's own tokens). None when it fails.
pub fn stanza(cfg: &Config, blocks: &[String], work: &Path) -> Result<Option<Vec<Vec<usize>>>> {
    let w = cfg.witness.as_ref().context("the configuration has no witness section")?;
    let _ = std::fs::remove_dir_all(work);
    std::fs::create_dir_all(work.join("mova"))?;
    let mut s = String::new();
    for b in blocks {
        s += &format!("# sent_id = {}\n", crate::block_id(b).unwrap_or(""));
        for c in crate::rows(b) {
            s += &format!("{}\t{}\t{}\t{}\t_\t_\t{}\t{}\t_\t_\n", c[0], c[1], c[2], c[3], c[6], c[7]);
        }
        s.push('\n');
    }
    std::fs::write(work.join("mova/run.conllu"), s)?;
    let out = Command::new(&w.venv).arg("-I").arg(&w.bridge).arg(work).env("CUDA_VISIBLE_DEVICES", "").env("OMP_NUM_THREADS", "4").env("MKL_NUM_THREADS", "4").output()?;
    if !out.status.success() {
        eprintln!("hard-cases: Stanza bridge failed: {}", String::from_utf8_lossy(&out.stderr).lines().last().unwrap_or(""));
        return Ok(None);
    }
    let got: Vec<Vec<usize>> = crate::blocks(&std::fs::read_to_string(work.join("stanza/run.conllu"))?).iter().map(|b| crate::rows(b).iter().map(|c| c[6].parse().unwrap_or(usize::MAX)).collect()).collect();
    Ok((got.len() == blocks.len()).then_some(got))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SampleCase {
    pub key: String,
    pub split: String,
    pub track: String,
    pub source: String,
    pub text: String,
    /// The draft (the source's tree) with id `m:<key>`.
    pub draft: String,
    pub stanza: Option<Vec<usize>>,
}

fn measure_dir(cfg: &Config) -> PathBuf {
    cfg.lane_root.join("measure")
}

/// The fixed sample: read, or built once.
pub fn sample(cfg: &Config, name: &str) -> Result<Vec<SampleCase>> {
    let p = measure_dir(cfg).join(format!("{name}.jsonl"));
    if p.exists() {
        return std::fs::read_to_string(&p)?.lines().filter(|l| !l.is_empty()).map(|l| Ok(serde_json::from_str(l)?)).collect();
    }
    let st = hard_case_list::state(&cfg.list)?;
    let mut c: Vec<&Case> = st.values().filter(|c| c.found.draft_conllu.as_deref().is_some_and(|d| !crate::rows(d).is_empty())).collect();
    let split = |c: &Case| cfg.policy.split(&c.found.source, &c.found.document.id);
    c.sort_by_key(|c| (split(c) != "measure", c.key.clone()));
    c.truncate(cfg.measure.sample);
    let drafts: Vec<String> = c.iter().map(|c| crate::with_id(&crate::blocks(c.found.draft_conllu.as_deref().unwrap())[0], &format!("m:{}", c.key))).collect();
    let heads = stanza(cfg, &drafts, &measure_dir(cfg).join("work/stanza"))?;
    let out: Vec<SampleCase> = c
        .iter()
        .zip(&drafts)
        .enumerate()
        .map(|(i, (c, d))| SampleCase { key: c.key.clone(), split: split(c).into(), track: cfg.policy.track(&c.found.licence).into(), source: c.found.source.clone(), text: c.found.text.clone(), draft: d.clone(), stanza: heads.as_ref().map(|h| h[i].clone()) })
        .collect();
    let body: String = out.iter().map(|s| serde_json::to_string(s).unwrap() + "\n").collect();
    crate::write_new(&p, body.as_bytes())?;
    crate::write_new(&measure_dir(cfg).join(format!("{name}.json")), &serde_json::to_vec_pretty(&json!({"cases": out.len(), "list_sha256": crate::file_sha(&cfg.list)?, "stanza": heads.is_some(), "sha256": sha(body.as_bytes())}))?)?;
    Ok(out)
}

/// gparse (CPU) on blocks, by id.
pub fn gparse(cfg: &Config, model: &Path, blocks: &[String], work: &Path) -> Result<BTreeMap<String, String>> {
    std::fs::create_dir_all(work)?;
    let (i, o) = (work.join("in.conllu"), work.join("out.conllu"));
    std::fs::write(&i, blocks.iter().map(|b| format!("{b}\n")).collect::<String>())?;
    let mut cmd = Command::new(&cfg.gparse.executable);
    cmd.arg("annotate").arg(model).arg(&i).arg(&o).env("GP_CPU", "1");
    if let Some(lex) = &cfg.gparse.lex {
        cmd.env("GP_LEX", lex);
    }
    let r = cmd.output()?;
    ensure!(r.status.success(), "gparse annotate: {}", String::from_utf8_lossy(&r.stderr).lines().last().unwrap_or(""));
    Ok(crate::blocks(&std::fs::read_to_string(&o)?).into_iter().filter_map(|b| Some((crate::block_id(&b)?.to_string(), b))).collect())
}

/// LAS/UAS of a model on a pinned treebank (`gparse eval`, CPU).
pub fn eval(cfg: &Config, model: &Path, pin: &crate::config::Pin) -> Result<Value> {
    let p = pin.check()?;
    let r = Command::new(&cfg.gparse.executable).arg("eval").arg(model).arg(p).env("GP_CPU", "1").output()?;
    ensure!(r.status.success(), "gparse eval: {}", String::from_utf8_lossy(&r.stderr).lines().last().unwrap_or(""));
    let text = String::from_utf8_lossy(&r.stdout).to_string() + &String::from_utf8_lossy(&r.stderr);
    let num = |k: &str| text.split(k).nth(1).and_then(|x| x.trim_start_matches([' ', '=', ':']).split(|c: char| !(c.is_ascii_digit() || c == '.')).next()?.parse::<f64>().ok());
    Ok(json!({"UAS": num("UAS"), "LAS": num("LAS"), "output": text.lines().filter(|l| l.contains("LAS")).collect::<Vec<_>>()}))
}

/// The model directory's identity: `gparse:<first 16 hex of its weights' SHA-256>`.
pub fn model_id(model: &Path) -> Result<String> {
    Ok(format!("gparse:{}", &crate::file_sha(&model.join("model.safetensors"))?[..16]))
}

/// One measurement of a model directory on the fixed sample.
pub fn measure(cfg: &Config, model: &Path, name: &str, label: &str) -> Result<Value> {
    let s = sample(cfg, name)?;
    ensure!(!s.is_empty(), "empty sample");
    let id = model_id(model)?;
    let drafts: Vec<String> = s.iter().map(|c| c.draft.clone()).collect();
    let got = gparse(cfg, model, &drafts, &measure_dir(cfg).join("work/gparse"))?;
    let silver: BTreeMap<String, crate::harvest::Annotation> = std::fs::read_to_string(cfg.lane_root.join("learn/annotations.jsonl")).unwrap_or_default().lines().filter_map(|l| serde_json::from_str::<crate::harvest::Annotation>(l).ok()).map(|a| (a.key.clone(), a)).collect();
    let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
    let (mut pass, mut draft_pass) = (0, 0);
    let mut att: BTreeMap<&str, (usize, usize, usize, usize)> = BTreeMap::new();
    let mut per = vec![];
    for c in &s {
        let t = got.get(&format!("m:{}", c.key)).context("gparse lost a sample sentence")?;
        let k = judge(t, c.stanza.as_deref());
        let kd = judge(&c.draft, c.stanza.as_deref());
        pass += usize::from(k.is_empty());
        draft_pass += usize::from(kd.is_empty());
        for x in &k {
            *kinds.entry(x.clone()).or_default() += 1;
        }
        if let Some(a) = silver.get(&c.key) {
            if let Some((u, l, n)) = attach(t, &a.tree) {
                for g in ["all", if a.split == "measure" { "measure_split" } else { "trained_splits" }] {
                    let e = att.entry(g).or_default();
                    *e = (e.0 + u, e.1 + l, e.2 + n, e.3 + 1);
                }
            }
        }
        per.push(json!({"key": c.key, "pass": k.is_empty(), "why": k}));
    }
    let n = s.len();
    let silver_v: Value = att.iter().map(|(g, (u, l, w, k))| (g.to_string(), json!({"sentences": k, "words": w, "UAS": 100.0 * *u as f64 / (*w).max(1) as f64, "LAS": 100.0 * *l as f64 / (*w).max(1) as f64}))).collect::<serde_json::Map<_, _>>().into();
    let ewt = match &cfg.measure.ewt_dev {
        Some(p) => eval(cfg, model, p)?,
        None => json!(null),
    };
    let v = json!({
        "label": label, "model": id, "model_dir": model, "sample": name, "cases": n,
        "pass": pass, "pass_rate": pass as f64 / n as f64, "draft_pass": draft_pass, "draft_pass_rate": draft_pass as f64 / n as f64,
        "kinds": kinds, "silver": silver_v, "ewt_dev": ewt, "per_case_sha256": sha(serde_json::to_vec(&per)?),
        "judge": {"witness_uas": WITNESS_UAS, "stanza": s.iter().filter(|c| c.stanza.is_some()).count()},
    });
    let d = measure_dir(cfg);
    write_atomic(&d.join("runs").join(format!("{label}-{}.json", &id[7..])), &serde_json::to_vec_pretty(&json!({"summary": v, "cases": per}))?)?;
    let log = d.join("log.tsv");
    if !log.exists() {
        std::fs::write(&log, "label\tmodel\tsample\tcases\tpass\tpass_rate\tdraft_pass_rate\tsilver_las_measure_split\tewt_las\n")?;
    }
    use std::io::Write;
    writeln!(
        std::fs::OpenOptions::new().append(true).open(&log)?,
        "{label}\t{id}\t{name}\t{n}\t{pass}\t{:.4}\t{:.4}\t{}\t{}",
        pass as f64 / n as f64,
        draft_pass as f64 / n as f64,
        v["silver"]["measure_split"]["LAS"].as_f64().map_or("-".into(), |x| format!("{x:.2}")),
        v["ewt_dev"]["LAS"].as_f64().map_or("-".into(), |x| format!("{x:.2}"))
    )?;
    Ok(v)
}

/// Re-checks the list with the deployed gparse (or `model`). Skips when this model already
/// re-checked it, unless `force`.
pub fn recheck(cfg: &Config, model: Option<&Path>, force: bool) -> Result<Value> {
    let model = model.unwrap_or(&cfg.gparse.model_dir).to_path_buf();
    let id = model_id(&model)?;
    let stamp = cfg.lane_root.join("recheck.json");
    if !force && read_json(&stamp).is_ok_and(|v| v["model"] == id.as_str()) {
        return Ok(json!({"model": id, "skipped": "already re-checked with this model"}));
    }
    let mut no_tree = 0;
    let (judged, learned, reopened) = hard_case_list::recheck(&cfg.list, None, GPARSE, &id, "hard-cases-recheck", true, &mut |cases: &[&Case]| {
        let drafts: Vec<Option<String>> = cases.iter().map(|c| c.found.draft_conllu.as_deref().and_then(|d| crate::blocks(d).into_iter().next()).map(|b| crate::with_id(&b, &format!("r:{}", c.key)))).collect();
        let have: Vec<String> = drafts.iter().flatten().cloned().collect();
        let got = if have.is_empty() { BTreeMap::new() } else { gparse(cfg, &model, &have, &cfg.lane_root.join("work/recheck"))? };
        let wit = if have.is_empty() { None } else { stanza(cfg, &have, &cfg.lane_root.join("work/recheck-stanza"))? };
        let wit_by: BTreeMap<String, Vec<usize>> = wit.map(|w| have.iter().filter_map(|b| crate::block_id(b).map(String::from)).zip(w).collect()).unwrap_or_default();
        Ok(cases
            .iter()
            .zip(&drafts)
            .map(|(c, d)| match d {
                // No tokens to parse: not judged, the case stays as it is.
                None => {
                    no_tree += 1;
                    None
                }
                Some(_) => {
                    let rid = format!("r:{}", c.key);
                    Some(got.get(&rid).map_or(vec!["gparse_failed".into()], |t| judge(t, wit_by.get(&rid).map(Vec::as_slice))))
                }
            })
            .collect())
    })?;
    let v = json!({"model": id, "judged": judged, "learned": learned, "reopened": reopened, "without_tokens": no_tree, "at_unix": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs()});
    write_atomic(&stamp, &serde_json::to_vec_pretty(&v)?)?;
    Ok(v)
}

/// Every `every` seconds until `<lane_root>/STOP`: when the deployed gparse weights change,
/// measure the new model on the fixed sample, then re-check the list.
pub fn watch(cfg: &Config, name: &str, every: u64) -> Result<()> {
    loop {
        if cfg.lane_root.join("STOP").exists() {
            eprintln!("hard-cases: STOP found");
            return Ok(());
        }
        let id = model_id(&cfg.gparse.model_dir)?;
        let last = read_json(&cfg.lane_root.join("recheck.json")).ok().and_then(|v| v["model"].as_str().map(String::from));
        if last.as_deref() != Some(id.as_str()) {
            eprintln!("hard-cases: model {id}: measure {}", measure(cfg, &cfg.gparse.model_dir, name, "deployed")?);
            eprintln!("hard-cases: recheck {}", recheck(cfg, None, false)?);
        }
        let mut waited = 0;
        while waited < every && !cfg.lane_root.join("STOP").exists() {
            std::thread::sleep(std::time::Duration::from_secs(10));
            waited += 10;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOOD: &str = "# sent_id = x\n# text = Dogs bark .\n1\tDogs\tdog\tNOUN\tNNS\t_\t2\tnsubj\t_\t_\n2\tbark\tbark\tVERB\tVBP\t_\t0\troot\t_\t_\n3\t.\t.\tPUNCT\t.\t_\t2\tpunct\t_\t_\n";

    #[test]
    fn judge_shows_red_and_green() {
        assert!(judge(GOOD, Some(&[2, 0, 2])).is_empty());
        // red: two roots; a head outside; two subjects; a witness that disagrees
        assert!(judge(&GOOD.replace("\t2\tnsubj", "\t0\tnsubj"), None).contains(&"roots".to_string()));
        assert!(judge(&GOOD.replace("\t2\tnsubj", "\t9\tnsubj"), None).contains(&"head_outside".to_string()));
        let two = "# sent_id = y\n1\tDogs\tdog\tNOUN\tNNS\t_\t3\tnsubj\t_\t_\n2\tcats\tcat\tNOUN\tNNS\t_\t3\tnsubj\t_\t_\n3\tbark\tbark\tVERB\tVBP\t_\t0\troot\t_\t_\n";
        assert_eq!(judge(two, None), vec!["grammar".to_string()]);
        assert_eq!(judge(GOOD, Some(&[0, 1, 2])), vec!["witness".to_string()]);
        assert_eq!(attach(GOOD, GOOD), Some((3, 3, 3)));
        assert_eq!(attach(GOOD, &GOOD.replace("\tnsubj", "\tobj")), Some((3, 2, 3)));
        assert_eq!(attach(GOOD, two), None);
    }
}
