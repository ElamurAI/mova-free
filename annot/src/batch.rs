//! Prepared teacher requests executed through the local Codex CLI queue.
//! The existing JSONL envelopes remain compatible with historical batch-apply readers.
//! ANNOT_NO_WAIT=1 starts/polls a local worker; final results appear only after every request succeeds.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use en::conllu;
use en::ud::Ud;
use en::udreg::Registry;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::ann::{Ann, Item};
use crate::llm::{self, Call};
use crate::{Outcome, batch_size, chunk_hints, finish, load_hints};

const MAX_TOKENS: u64 = 32000;

/// Everything `batch-apply` needs, written by `batch-prepare`.
#[derive(Serialize, Deserialize)]
struct State {
    model: String,
    effort: String,
    /// training files of the Rust annotator (needed again only for ANNOT_FEATS=rust)
    train: Vec<PathBuf>,
    /// silver corpus to append to; None for calib
    corpus: Option<PathBuf>,
    items: Vec<Item>,
    /// Rust drafts, in the order of `items`
    drafts: Vec<Ann>,
    /// item indices of each request; the request's custom_id is its index here
    chunks: Vec<Vec<usize>>,
    hints_on: bool,
    hint_count: (usize, usize),
    /// "patch" (Haiku corrects the SLM draft) or "blind" (Haiku annotates from scratch, the draft is only for the report)
    #[serde(default = "patch_flow")]
    flow: String,
}

fn patch_flow() -> String {
    "patch".into()
}

/// One Batches request: the fixed instructions first, as their own block marked for prompt caching,
/// then the sentences of this batch. Concatenated, the two blocks are exactly the `claude -p` prompt.
/// No `thinking` field: Opus 5.5 always thinks adaptively; `output_config.effort` matches ANNOT_EFFORT
/// of the live flow.
fn request(custom_id: &str, model: &str, effort: &str, fixed: &str, var: &str) -> Value {
    json!({
        "custom_id": custom_id,
        "params": {
            "model": model,
            "max_tokens": MAX_TOKENS,
            "output_config": {"effort": effort},
            "messages": [{"role": "user", "content": [
                {"type": "text", "text": fixed, "cache_control": {"type": "ephemeral"}},
                {"type": "text", "text": var}
            ]}]
        }
    })
}

#[allow(clippy::too_many_arguments)]
pub fn prepare(
    b: &BTreeMap<String, Ann>,
    train: &[PathBuf],
    items: &[Item],
    out: &Path,
    corpus: Option<&Path>,
    model_id: &str,
    effort: &str,
) -> Result<()> {
    mova_teacher::validate_model(model_id)?;
    if std::env::var("ANNOT_FLOW").as_deref() != Ok("patch") {
        bail!("batch-prepare supports only ANNOT_FLOW=patch");
    }
    let hints = load_hints(true)?;
    let mut lines = String::new();
    let mut chunks = Vec::new();
    let mut hint_count = (0usize, 0usize);
    for (k, chunk) in items.chunks(batch_size()).enumerate() {
        let pairs: Vec<(&Item, &Ann)> = chunk.iter().map(|it| (it, &b[&it.id])).collect();
        let hs = chunk_hints(hints.as_ref(), chunk, b);
        hint_count.0 += hs.values().map(Vec::len).sum::<usize>();
        hint_count.1 += hs.len();
        let (fixed, var) = llm::patch_prompt(&pairs, &hs);
        lines.push_str(&serde_json::to_string(&request(
            &k.to_string(),
            model_id,
            effort,
            &fixed,
            &var,
        ))?);
        lines.push('\n');
        let start = k * batch_size();
        chunks.push((start..start + chunk.len()).collect());
    }
    let state = State {
        model: model_id.to_string(),
        effort: effort.to_string(),
        train: train.to_vec(),
        corpus: corpus.map(Path::to_path_buf),
        items: items.to_vec(),
        drafts: items.iter().map(|it| b[&it.id].clone()).collect(),
        chunks,
        hints_on: hints.is_some(),
        hint_count,
        flow: patch_flow(),
    };
    std::fs::write(out.join("batch-requests.jsonl"), &lines)?;
    std::fs::write(out.join("batch-state.json"), serde_json::to_string(&state)?)?;
    println!(
        "{} sentences in {} requests → {} (model {model_id}, effort {effort})",
        items.len(),
        state.chunks.len(),
        out.join("batch-requests.jsonl").display()
    );
    Ok(())
}

/// Token usage of one reply.
#[derive(Debug, Default, Clone, PartialEq)]
struct Usage {
    input: u64,
    output: u64,
    cache_read: u64,
    cache_write: u64,
}

/// One succeeded reply: the text blocks joined (thinking blocks skipped), stop reason, usage.
#[derive(Debug, Clone, PartialEq)]
struct Reply {
    text: String,
    stop: String,
    usage: Usage,
}

/// Batches results JSONL → reply or error text by custom_id (results come in any order).
fn parse_results(text: &str) -> Result<BTreeMap<String, std::result::Result<Reply, String>>> {
    let mut out = BTreeMap::new();
    for (i, line) in text.lines().enumerate().filter(|x| !x.1.trim().is_empty()) {
        let v: Value =
            serde_json::from_str(line).with_context(|| format!("results line {}", i + 1))?;
        let id = v["custom_id"]
            .as_str()
            .with_context(|| format!("results line {}: no custom_id", i + 1))?
            .to_string();
        let r = &v["result"];
        let res = match r["type"].as_str() {
            Some("succeeded") => {
                let m = &r["message"];
                let text: String = m["content"]
                    .as_array()
                    .map(|c| {
                        c.iter()
                            .filter(|b| b["type"] == "text")
                            .filter_map(|b| b["text"].as_str())
                            .collect::<Vec<_>>()
                            .join("")
                    })
                    .unwrap_or_default();
                let u = &m["usage"];
                let tok = |k: &str| u[k].as_u64().unwrap_or(0);
                Ok(Reply {
                    text,
                    stop: m["stop_reason"].as_str().unwrap_or_default().to_string(),
                    usage: Usage {
                        input: tok("input_tokens"),
                        output: tok("output_tokens"),
                        cache_read: tok("cache_read_input_tokens"),
                        cache_write: tok("cache_creation_input_tokens"),
                    },
                })
            }
            Some(t) => Err(format!("{t}: {}", r["error"])),
            None => Err("no result type".into()),
        };
        out.insert(id, res);
    }
    Ok(out)
}

/// API-equivalent cost of one batch reply, in USD, from the Anthropic pricing page (2026-10). Per million
/// tokens: base input, 5-minute cache write, cache read (hit), output. The Batches API charges 50% of every
/// one of them. Haiku 5.5 is priced by prompt size: above 100,000 input tokens (input + cache) it costs 5× more.
/// Models not in the table: 0 (their tokens are still recorded).
fn batch_cost(model: &str, u: &Usage) -> f64 {
    let id = model
        .rsplit_once('-')
        .filter(|(_, d)| d.len() == 8 && d.bytes().all(|b| b.is_ascii_digit()))
        .map_or(model, |(i, _)| i);
    let prompt = u.input + u.cache_read + u.cache_write;
    let (inp, cw, cr, out) = match id {
        "claude-fable-5-1" | "claude-mythos-5-1" => (10.0, 12.5, 0.25, 50.0),
        "claude-fable-5" | "claude-mythos-5" => (10.0, 12.5, 1.0, 50.0),
        "claude-opus-5-5" => (4.0, 5.0, 0.20, 20.0),
        "claude-opus-5" | "claude-opus-4-8" | "claude-opus-4-7" | "claude-opus-4-6"
        | "claude-opus-4-5" => (5.0, 6.25, 0.50, 25.0),
        "claude-opus-4-1" | "claude-opus-4" => (15.0, 18.75, 1.50, 75.0),
        "claude-sonnet-5-5" => (2.0, 2.5, 0.10, 10.0),
        "claude-sonnet-5" => (2.0, 2.5, 0.20, 10.0),
        "claude-sonnet-4-6" | "claude-sonnet-4-5" | "claude-sonnet-4" => (3.0, 3.75, 0.30, 15.0),
        "claude-haiku-5-5" if prompt > 100_000 => (0.50, 0.625, 0.05, 2.50),
        "claude-haiku-5-5" => (0.10, 0.125, 0.01, 0.50),
        "claude-haiku-4-5" => (1.0, 1.25, 0.10, 5.0),
        "claude-haiku-3-5" => (0.80, 1.0, 0.08, 4.0),
        _ => return 0.0,
    };
    let std = (u.input as f64 * inp
        + u.output as f64 * out
        + u.cache_read as f64 * cr
        + u.cache_write as f64 * cw)
        / 1e6;
    std * 0.5
}

/// `annot batch-cost <out-dir>`: tokens and API-equivalent cost of any finished batch (`batch-results.jsonl`), each
/// reply priced by the model its request named in `batch-requests.jsonl` — for the flows outside annot (the dictionary
/// builder, macula) that only send requests through `batch-run`.
pub fn cost(out: &Path) -> Result<()> {
    let models: BTreeMap<String, String> =
        std::fs::read_to_string(out.join("batch-requests.jsonl"))?
            .lines()
            .filter_map(|l| serde_json::from_str::<Value>(l).ok())
            .map(|v| {
                (
                    v["custom_id"].as_str().unwrap_or_default().to_string(),
                    v["params"]["model"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                )
            })
            .collect();
    let res = parse_results(&std::fs::read_to_string(out.join("batch-results.jsonl"))?)?;
    let (mut tot, mut cost, mut failed, mut cut) = (Usage::default(), 0.0, 0usize, 0usize);
    for (id, r) in &res {
        match r {
            Ok(r) => {
                cost += batch_cost(models.get(id).map_or("", String::as_str), &r.usage);
                cut += usize::from(r.stop != "end_turn");
                tot.input += r.usage.input;
                tot.output += r.usage.output;
                tot.cache_read += r.usage.cache_read;
                tot.cache_write += r.usage.cache_write;
            }
            Err(_) => failed += 1,
        }
    }
    let names: std::collections::BTreeSet<&String> = models.values().collect();
    if names.iter().any(|m| m.starts_with("gpt-")) {
        println!(
            "Codex subscription dollar cost is not reported; the numeric compatibility cost is zero."
        );
    }
    println!(
        "batch: {} results ({names:?}), failed {failed}, not ended by end_turn {cut}; tokens: input {}, cache read {}, cache write {}, output {} (incl. thinking); API equivalent at batch price ${cost:.3}",
        res.len(),
        tot.input,
        tot.cache_read,
        tot.cache_write,
        tot.output
    );
    Ok(())
}

pub fn apply(out: &Path, results: &Path) -> Result<()> {
    let state: State = serde_json::from_str(
        &std::fs::read_to_string(out.join("batch-state.json"))
            .context("batch-state.json (run batch-prepare first)")?,
    )?;
    let res = parse_results(&std::fs::read_to_string(results)?)?;
    let reg = Registry::load();
    let b: BTreeMap<String, Ann> = state
        .items
        .iter()
        .zip(&state.drafts)
        .map(|(it, d)| (it.id.clone(), d.clone()))
        .collect();
    let mut o = Outcome {
        hints_on: state.hints_on,
        hint_count: state.hint_count,
        ..Outcome::default()
    };
    let (mut tot, mut cost, mut failed) = (Usage::default(), 0.0, 0usize);
    for (k, idxs) in state.chunks.iter().enumerate() {
        let pairs: Vec<(&Item, &Ann)> = idxs
            .iter()
            .map(|&i| (&state.items[i], &b[&state.items[i].id]))
            .collect();
        match res.get(&k.to_string()) {
            Some(Ok(r)) => {
                let c = batch_cost(&state.model, &r.usage);
                cost += c;
                tot.input += r.usage.input;
                tot.output += r.usage.output;
                tot.cache_read += r.usage.cache_read;
                tot.cache_write += r.usage.cache_write;
                if r.stop != "end_turn" {
                    o.notes
                        .push(format!("batch request {k}: stop reason {}", r.stop));
                }
                o.calls.push(Call {
                    step: format!("{} (batch)", state.flow),
                    sentences: pairs.len(),
                    secs: 0.0,
                    cost_usd: c,
                    input_tokens: r.usage.input,
                    output_tokens: r.usage.output,
                    cache_read_tokens: r.usage.cache_read,
                    cache_write_tokens: r.usage.cache_write,
                });
                if state.flow == "blind" {
                    let refs: Vec<&Item> = pairs.iter().map(|p| p.0).collect();
                    let f = crate::collect(&r.text, &refs, &reg, &mut o.bad, "A");
                    o.ta.push_str(&r.text);
                    o.ta.push('\n');
                    o.a.extend(f.clone());
                    o.fin.extend(f);
                } else {
                    o.add_patch_reply(&r.text, &pairs, &reg);
                }
            }
            other => {
                failed += 1;
                let why = match other {
                    Some(Err(e)) => format!("batch request {k} failed: {e}"),
                    _ => format!("batch request {k}: no result"),
                };
                for (it, _) in &pairs {
                    o.bad.insert(format!("P {}", it.id), why.clone());
                }
            }
        }
    }
    if state.model.starts_with("gpt-") {
        o.notes.push(
            "Codex subscription dollar cost is not reported; numeric compatibility cost is zero."
                .into(),
        );
    }
    o.notes.push(format!(
        "batch: {} requests, failed {failed}; tokens: input {}, cache read {}, cache write {}, output {} (incl. thinking); API equivalent at batch price ${cost:.3}",
        state.chunks.len(),
        tot.input,
        tot.cache_read,
        tot.cache_write,
        tot.output
    ));
    if let Some(t) = processing_time(out) {
        o.notes
            .push(format!("batch processing time (created → ended): {t:.0} s"));
    }
    let ud = if std::env::var("ANNOT_FEATS").as_deref() == Ok("rust") {
        let mut sents = Vec::new();
        for p in &state.train {
            sents.extend(conllu::read(p)?.into_iter().filter(|s| s.tagged()));
        }
        Some(Ud::train(&sents))
    } else {
        None
    };
    let transport = if state.model.starts_with("gpt-") {
        "Codex CLI; subscription cost not reported"
    } else {
        "historical Batches API"
    };
    let effort = format!("{}, {transport}", state.effort);
    finish(
        ud.as_ref(),
        &state.items,
        &b,
        o,
        out,
        state.corpus.as_deref(),
        &state.flow,
        &state.model,
        &effort,
    )
}

/// `annot batch-blind <patch dir> <out dir>`: the same sentences as a prepared patch batch, but Haiku annotates them
/// from scratch, without the SLM draft — to measure how much the draft steers the teacher (anchoring). Nothing goes
/// into the corpus; the drafts stay in the state only for the report (final vs Rust).
pub fn blind(from: &Path, out: &Path) -> Result<()> {
    let mut state: State = serde_json::from_str(
        &std::fs::read_to_string(from.join("batch-state.json"))
            .context("batch-state.json of the patch batch")?,
    )?;
    std::fs::create_dir_all(out)?;
    let mut lines = String::new();
    for (k, idxs) in state.chunks.iter().enumerate() {
        let refs: Vec<&Item> = idxs.iter().map(|&i| &state.items[i]).collect();
        let (fixed, var) = llm::annotate_prompt(&refs);
        lines.push_str(&serde_json::to_string(&request(
            &k.to_string(),
            &state.model,
            &state.effort,
            &fixed,
            &var,
        ))?);
        lines.push('\n');
    }
    state.flow = "blind".into();
    state.corpus = None;
    state.hints_on = false;
    state.hint_count = (0, 0);
    std::fs::write(out.join("batch-requests.jsonl"), &lines)?;
    std::fs::write(out.join("batch-state.json"), serde_json::to_string(&state)?)?;
    println!(
        "{} sentences in {} blind requests → {} (model {}, effort {})",
        state.items.len(),
        state.chunks.len(),
        out.join("batch-requests.jsonl").display(),
        state.model,
        state.effort
    );
    Ok(())
}

/// Seconds between `created_at` and `ended_at` of the saved batch status.
fn processing_time(out: &Path) -> Option<f64> {
    let v: Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("batch-status.json")).ok()?).ok()?;
    Some(epoch(v["ended_at"].as_str()?)? - epoch(v["created_at"].as_str()?)?)
}

/// RFC 3339 UTC timestamp (`2026-10-01T12:34:56.789Z` or `+00:00`) → seconds since 1970.
fn epoch(t: &str) -> Option<f64> {
    let (y, mo, d) = (
        t.get(0..4)?.parse::<i64>().ok()?,
        t.get(5..7)?.parse::<i64>().ok()?,
        t.get(8..10)?.parse::<i64>().ok()?,
    );
    let (h, mi, s) = (
        t.get(11..13)?.parse::<i64>().ok()?,
        t.get(14..16)?.parse::<i64>().ok()?,
        t.get(17..19)?.parse::<i64>().ok()?,
    );
    let frac: f64 = t
        .get(19..)
        .filter(|r| r.starts_with('.'))
        .map(|r| {
            r.chars()
                .take_while(|c| *c == '.' || c.is_ascii_digit())
                .collect::<String>()
        })
        .and_then(|f| format!("0{f}").parse().ok())
        .unwrap_or(0.0);
    // days from civil (Howard Hinnant)
    let y2 = if mo <= 2 { y - 1 } else { y };
    let era = y2.div_euclid(400);
    let yoe = y2 - era * 400;
    let doy = (153 * (if mo > 2 { mo - 3 } else { mo + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some((days * 86400 + h * 3600 + mi * 60 + s) as f64 + frac)
}

/// Run or poll a local Codex batch; no external Batches API is used.
pub fn run(out: &Path) -> Result<()> {
    crate::codex_batch::run(out, std::env::var("ANNOT_NO_WAIT").as_deref() == Ok("1"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ann::Tok;
    use en::gram::{Feats, Rel, Tag, UPos};

    fn item() -> (Item, Ann) {
        let forms = ["Dogs", "bark", "."];
        let a: Ann = vec![
            Tok {
                form: "Dogs".into(),
                lemma: "dog".into(),
                upos: UPos::parse("NOUN").unwrap(),
                tag: Tag::parse("NNS").unwrap(),
                feats: Feats::parse("Number=Plur").0,
                head: 2,
                rel: Rel::parse("nsubj").unwrap(),
            },
            Tok {
                form: "bark".into(),
                lemma: "bark".into(),
                upos: UPos::parse("NOUN").unwrap(),
                tag: Tag::parse("NN").unwrap(),
                feats: Feats::parse("Number=Sing").0,
                head: 0,
                rel: Rel::parse("root").unwrap(),
            },
            Tok {
                form: ".".into(),
                lemma: ".".into(),
                upos: UPos::parse("PUNCT").unwrap(),
                tag: Tag::parse(".").unwrap(),
                feats: Feats::default(),
                head: 2,
                rel: Rel::parse("punct").unwrap(),
            },
        ];
        (
            Item {
                id: "t-1".into(),
                text: "Dogs bark.".into(),
                forms: forms.iter().map(|x| x.to_string()).collect(),
                gold: Some(a.clone()),
            },
            a,
        )
    }

    #[test]
    fn request_has_cached_fixed_block_and_same_prompt() {
        let (it, a) = item();
        let pairs = vec![(&it, &a)];
        let (fixed, var) = llm::patch_prompt(&pairs, &BTreeMap::new());
        let r = request("0", "claude-opus-5-5", "medium", &fixed, &var);
        assert_eq!(r["custom_id"], "0");
        assert_eq!(r["params"]["model"], "claude-opus-5-5");
        assert_eq!(r["params"]["max_tokens"], 32000);
        assert_eq!(r["params"]["output_config"]["effort"], "medium");
        let c = &r["params"]["messages"][0]["content"];
        assert_eq!(c[0]["cache_control"]["type"], "ephemeral");
        assert!(c[1].get("cache_control").is_none());
        // the two blocks are the whole live prompt; the fixed one has no sentence in it
        let joined = format!(
            "{}{}",
            c[0]["text"].as_str().unwrap(),
            c[1]["text"].as_str().unwrap()
        );
        assert_eq!(joined, fixed.clone() + &var);
        assert!(!fixed.contains("t-1") && var.contains("# sent_id = t-1"));
        // the fixed block is identical across batches (prompt caching)
        let (fixed2, _) = llm::patch_prompt(&[], &BTreeMap::new());
        assert_eq!(fixed, fixed2);
    }

    #[test]
    fn results_parse_and_patch() {
        let ok = r#"{"custom_id":"0","result":{"type":"succeeded","message":{"stop_reason":"end_turn","content":[{"type":"thinking","thinking":"","signature":"x"},{"type":"text","text":"t-1 2 UPOS=VERB\nt-1 2 XPOS=VBP\nt-1 2 FEATS=Mood=Ind|Number=Plur|Person=3|Tense=Pres|VerbForm=Fin\n"}],"usage":{"input_tokens":120,"output_tokens":900,"cache_read_input_tokens":3000,"cache_creation_input_tokens":0}}}}"#;
        let err = r#"{"custom_id":"1","result":{"type":"errored","error":{"type":"error","error":{"type":"overloaded_error","message":"x"}}}}"#;
        let res = parse_results(&format!("{err}\n{ok}\n")).unwrap();
        let r = res["0"].as_ref().unwrap();
        assert_eq!(r.stop, "end_turn");
        assert_eq!(
            r.usage,
            Usage {
                input: 120,
                output: 900,
                cache_read: 3000,
                cache_write: 0
            }
        );
        assert!(r.text.starts_with("t-1 2 UPOS=VERB"));
        assert!(res["1"].as_ref().unwrap_err().starts_with("errored"));
        // half of $4/$20/$0.20 per MTok
        assert!(
            (batch_cost("claude-opus-5-5", &r.usage)
                - (120.0 * 4.0 + 900.0 * 20.0 + 3000.0 * 0.2) / 2e6)
                .abs()
                < 1e-12
        );
        // the reply goes through the live patch code and gates
        let (it, a) = item();
        let mut o = Outcome::default();
        o.add_patch_reply(&r.text, &[(&it, &a)], &Registry::load());
        assert!(o.bad.is_empty(), "{:?}", o.bad);
        assert_eq!(o.fin["t-1"][1].upos, UPos::parse("VERB").unwrap());
        // negative control: a reply that skips the sentence is rejected
        let mut o2 = Outcome::default();
        o2.add_patch_reply("other-id OK\n", &[(&it, &a)], &Registry::load());
        assert!(o2.fin.is_empty() && o2.bad.contains_key("P t-1"));
    }

    #[test]
    fn state_roundtrip_and_epoch() {
        let (it, a) = item();
        let s = State {
            model: "m".into(),
            effort: "medium".into(),
            train: vec![],
            corpus: None,
            items: vec![it],
            drafts: vec![a.clone()],
            chunks: vec![vec![0]],
            hints_on: false,
            hint_count: (0, 0),
            flow: patch_flow(),
        };
        let back: State = serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
        assert_eq!(back.drafts[0], a);
        assert_eq!(back.items[0].gold.as_ref().unwrap(), &a);
        assert_eq!(epoch("1970-01-02T00:00:01Z"), Some(86401.0));
        assert_eq!(
            epoch("2026-10-01T12:00:30.5+00:00").unwrap() - epoch("2026-09-30T12:00:00Z").unwrap(),
            86430.5
        );
    }
}

/// Run the same Codex queue synchronously.
pub fn direct_run(out: &Path) -> Result<()> {
    crate::codex_batch::run(out, false)
}

#[cfg(test)]
mod wave27_tests {
    use super::*;
    #[test]
    fn duplicate_result_last_wins_and_price_boundary() {
        let text = "{\"custom_id\":\"x\",\"result\":{\"type\":\"errored\"}}\n{\"custom_id\":\"x\",\"result\":{\"type\":\"succeeded\",\"message\":{\"content\":[{\"type\":\"thinking\",\"text\":\"hidden\"},{\"type\":\"text\",\"text\":\"kept\"}]}}}";
        let results = parse_results(text).unwrap();
        assert_eq!(results.len(), 1);
        let reply = results["x"].as_ref().unwrap();
        assert_eq!(reply.text, "kept");
        assert_eq!(reply.usage, Usage::default());
        assert!(parse_results("{}").is_err());
        let low = Usage {
            input: 100_000,
            ..Usage::default()
        };
        let high = Usage {
            input: 100_001,
            ..Usage::default()
        };
        assert!((batch_cost("claude-haiku-5-5", &low) - 0.005).abs() < 1e-12);
        assert!((batch_cost("claude-haiku-5-5", &high) - 0.02500025).abs() < 1e-12);
        assert_eq!(batch_cost("gpt-6.1-terra", &high), 0.0);
    }
}
