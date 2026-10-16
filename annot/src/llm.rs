//! Opus via `claude -p`: annotation of a batch of sentences and a second look at two annotations. The
//! call runs in an empty directory (no project context), without tools; output is JSON with the result,
//! time, tokens and cost.

use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

use anyhow::{Context, Result, bail};
use en::gram::{Tag, UPos};
use en::udreg::Registry;
use serde::{Deserialize, Serialize};

use crate::ann::{Ann, Item, diff, write_block};

/// Accounting of one call.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Call {
    pub step: String,
    pub sentences: usize,
    pub secs: f64,
    pub cost_usd: f64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_read_tokens: u64,
    #[serde(default)]
    pub cache_write_tokens: u64,
}

pub struct Opus {
    pub model: String,
    pub effort: String,
    /// empty directory for the call
    pub cwd: std::path::PathBuf,
}

impl Opus {
    fn ask(&self, step: &str, n: usize, prompt: &str) -> Result<(String, Call)> {
        std::fs::create_dir_all(&self.cwd)?;
        let t0 = Instant::now();
        let mut child = Command::new("claude")
            .args(["-p", "--model", &self.model, "--effort", &self.effort, "--output-format", "json"])
            // the machine-wide settings.json sets CLAUDE_CODE_EFFORT_LEVEL=max and overrides --effort; --settings takes precedence
            .args(["--settings", &format!("{{\"env\":{{\"CLAUDE_CODE_EFFORT_LEVEL\":\"{}\"}}}}", self.effort)])
            .args(["--disallowedTools", "Bash,Edit,Write,Read,Glob,Grep,WebFetch,WebSearch,Agent,NotebookEdit"])
            .envs(api_env())
            .current_dir(&self.cwd)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("claude -p")?;
        child.stdin.take().context("stdin")?.write_all(prompt.as_bytes())?;
        let out = child.wait_with_output()?;
        if !out.status.success() {
            bail!("claude -p ({step}): {}", String::from_utf8_lossy(&out.stderr));
        }
        let v: serde_json::Value = serde_json::from_slice(&out.stdout).context("JSON from claude -p")?;
        if v["is_error"].as_bool() == Some(true) {
            bail!("claude -p ({step}): {}", v["result"]);
        }
        let u = &v["usage"];
        let tok = |k: &str| u[k].as_u64().unwrap_or(0);
        let call = Call {
            step: step.to_string(),
            sentences: n,
            secs: t0.elapsed().as_secs_f64(),
            cost_usd: v["total_cost_usd"].as_f64().unwrap_or(0.0),
            input_tokens: tok("input_tokens"),
            output_tokens: tok("output_tokens"),
            cache_read_tokens: tok("cache_read_input_tokens"),
            cache_write_tokens: tok("cache_creation_input_tokens"),
        };
        Ok((v["result"].as_str().unwrap_or_default().to_string(), call))
    }

    /// First look: annotation of a batch of sentences with fixed tokens.
    pub fn annotate(&self, items: &[&Item]) -> Result<(String, Call)> {
        let (fixed, var) = annotate_prompt(items);
        self.ask("annotate", items.len(), &(fixed + &var))
    }

    /// Second look: own annotation (A), the Rust annotator's annotation (B), disagreements → final.
    pub fn adjudicate(&self, items: &[(&Item, &Ann, &Ann)]) -> Result<(String, Call)> {
        let mut p = rules(
            "You are an expert UD annotator reviewing two annotations of the same English sentences (UD v2.18, EWT guidelines). Annotation A is your own first pass; annotation B comes from a statistical parser. Produce the FINAL correct annotation. For every disagreement choose A, B, or a third option if both are wrong. Also re-check tokens where A and B agree — both can be wrong. Do not trust A just because it is yours.",
        );
        p.push_str("\nSentences:\n\n");
        for (it, a, b) in items {
            p.push_str(&format!("=== {} — {}\n--- A (your first pass)\n{}--- B (statistical parser)\n{}", it.id, it.text, write_block(&it.id, a), write_block(&it.id, b)));
            let d = diff(a, b);
            p.push_str(&format!("--- disagreements ({}):\n{}\n\n", d.len(), if d.is_empty() { "none".to_string() } else { d.join("\n") }));
        }
        p.push_str("Now output ONLY the final CoNLL-U blocks, one per sentence, each starting with its `# sent_id = ` line.\n");
        self.ask("adjudicate", items.len(), &p)
    }
}

impl Opus {
    /// "Draft" mode (run the Rust annotator first and give its output to Opus as a starting point, so
    /// Opus does not annotate from scratch): Opus receives the Rust annotation and corrects it in one pass.
    pub fn correct(&self, items: &[(&Item, &Ann)]) -> Result<(String, Call)> {
        let mut p = rules(
            "You are an expert UD annotator (UD v2.18, English EWT guidelines). Below is a DRAFT annotation of each sentence produced by our statistical parser. The draft is often wrong: typical errors are PP attachment (obl vs nmod), coordination, clausal relations, UPOS/XPOS of ambiguous words and FEATS. Check EVERY token and every column and output the FINAL correct annotation. Keep what is right, fix what is wrong; do not trust the draft.",
        );
        p.push_str("\nSentences with drafts:\n\n");
        for (it, b) in items {
            p.push_str(&format!("=== {} — {}\n--- draft (statistical parser)\n{}", it.id, it.text, write_block(&it.id, b)));
        }
        p.push_str("Now output ONLY the final CoNLL-U blocks, one per sentence, each starting with its `# sent_id = ` line.\n");
        self.ask("correct", items.len(), &p)
    }
}

impl Opus {
    /// "Patches only" mode (for millions of sentences): Opus sees the Rust draft and emits only
    /// patches — `<sent_id> <word No.> <COLUMN>=<value>` or `<sent_id> OK`. Rust applies the patches,
    /// the gates check the result.
    /// `hints` — seed-rule hints for the drafts (`crate::hints`), by sentence id.
    pub fn patch(&self, items: &[(&Item, &Ann)], hints: &std::collections::BTreeMap<String, Vec<String>>) -> Result<(String, Call)> {
        let (fixed, var) = patch_prompt(items, hints);
        self.ask("patch", items.len(), &(fixed + &var))
    }
}

/// The first-look prompt (annotation from scratch, no draft) in two parts: fixed instructions (cacheable) and the
/// sentences of this batch.
pub fn annotate_prompt(items: &[&Item]) -> (String, String) {
    let mut p = rules("You are an expert linguist annotating English in Universal Dependencies (UD v2.18) following the English EWT guidelines.");
    p.push_str("\nSentences (tokens are fixed, one per numbered line):\n\n");
    let mut v = String::new();
    for it in items {
        v.push_str(&format!("# sent_id = {}\n", it.id));
        for (i, f) in it.forms.iter().enumerate() {
            v.push_str(&format!("{} {}\n", i + 1, f));
        }
        v.push('\n');
    }
    (p, v)
}

/// ANNOT_CONTEXT=<world.tsv> (`prag world`: sent_id, summary): the temporary world before each sentence — a page
/// summary instead of the pages (design note, 15.10: a window must stay far below 70k tokens).
fn world_context() -> &'static std::collections::BTreeMap<String, String> {
    static W: std::sync::OnceLock<std::collections::BTreeMap<String, String>> = std::sync::OnceLock::new();
    W.get_or_init(|| {
        std::env::var("ANNOT_CONTEXT")
            .ok()
            .and_then(|f| std::fs::read_to_string(f).ok())
            .map(|t| t.lines().skip(1).filter_map(|l| l.split_once('\t')).map(|(a, b)| (a.to_string(), b.to_string())).collect())
            .unwrap_or_default()
    })
}

/// Pragmatic fields and their closed sets (the prag schema, `prag/src/schema.rs`).
pub const PRAG_FIELDS: [&str; 6] = ["form", "act", "indirect", "hedge", "polarity", "voice"];
pub const PRAG_FORM: [&str; 8] = ["decl", "q", "wh", "imp", "excl", "frag", "intj", "other"];
pub const PRAG_ACT: [&str; 16] = ["assert", "narrate", "ask", "request", "offer", "suggest", "promise", "thank", "apologize", "greet", "agree", "disagree", "evaluate", "express", "warn", "other"];
pub const PRAG_POL: [&str; 3] = ["pos", "neg", "none"];
pub const PRAG_VOICE: [&str; 3] = ["narrator", "character", "none"];

/// Is `value` allowed for the pragmatic `field`?
pub fn prag_ok(field: &str, value: &str) -> bool {
    match field {
        "form" => PRAG_FORM.contains(&value),
        "act" => PRAG_ACT.contains(&value),
        "indirect" | "hedge" => matches!(value, "0" | "1"),
        "polarity" => PRAG_POL.contains(&value),
        "voice" => PRAG_VOICE.contains(&value),
        _ => false,
    }
}

/// ANNOT_PRAG=<agreed.tsv> (`prag agree`: sent_id + the six fields): the pragmatic reading shown to the teacher next
/// to the tree, and the base its PRAG patches apply to.
pub fn prag_labels() -> &'static std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>> {
    static P: std::sync::OnceLock<std::collections::BTreeMap<String, std::collections::BTreeMap<String, String>>> = std::sync::OnceLock::new();
    P.get_or_init(|| {
        let Some(t) = std::env::var("ANNOT_PRAG").ok().and_then(|f| std::fs::read_to_string(f).ok()) else { return Default::default() };
        let mut lines = t.lines();
        let head: Vec<&str> = lines.next().unwrap_or("").split('\t').collect();
        lines
            .filter_map(|l| {
                let c: Vec<&str> = l.split('\t').collect();
                let m: std::collections::BTreeMap<String, String> = head.iter().zip(&c).filter(|(h, _)| PRAG_FIELDS.contains(h)).map(|(h, v)| (h.to_string(), v.to_string())).collect();
                c.first().map(|id| (id.to_string(), m))
            })
            .collect()
    })
}

/// The "patches only" prompt in two parts: the fixed instructions (identical for every batch, so the
/// Batches API can cache them) and the sentences of this batch. `claude -p` gets the two concatenated.
pub fn patch_prompt(items: &[(&Item, &Ann)], hints: &std::collections::BTreeMap<String, Vec<String>>) -> (String, String) {
    let mut p = rules(
        "You are an expert UD annotator (UD v2.18, English EWT guidelines). Below is a DRAFT annotation of each sentence produced by our statistical parser. It is often wrong: PP attachment (obl vs nmod), coordination, clausal relations, UPOS/XPOS of ambiguous words, FEATS. Check every token and column.",
    );
    p.push_str("\nOUTPUT FORMAT (overrides the CoNLL-U output instruction above): do NOT output CoNLL-U. Output ONLY corrections, one per line:\n<sent_id> <token number> <COLUMN>=<new value>\nwhere COLUMN is one of LEMMA UPOS XPOS FEATS HEAD DEPREL. Several wrong columns of one token → several lines. If a sentence needs no correction, output one line: <sent_id> OK. Every sentence must get at least one line. No explanations.\n\nSentences with drafts:\n\n");
    let mut v = String::new();
    if !hints.is_empty() {
        v.push_str("Some drafts are followed by HINTS: violations of English grammar rules that our rule engine found in the draft. On our tests about 9 of 10 such hints point at a real error in the draft, but a rule can be wrong: fix the draft where the hint is right, ignore it where it is not. Hints cover only a small part of the errors: still check every token.\n\n");
    }
    let world = world_context();
    if !world.is_empty() {
        v.push_str("Some sentences are preceded by a WORLD line: a summary of the story or lesson before this sentence (salient entities with what is known of them, the last speaker, recent events), made by our pragmatic reader. Use it for pronouns, speakers and roles; do not annotate it.\n\n");
    }
    let prag = prag_labels();
    if !prag.is_empty() {
        v.push_str(&format!("Each sentence also has a PRAG line: our pragmatic reading of it (after the tree). Check it too. Fields and values: form = {}; act = {}; indirect = 0|1 (the form is not the usual one for the act, e.g. «Can you open the window?» = q but request); hedge = 0|1 (maybe, I think, sort of); polarity = {} (the speaker's evaluation; grammatical negation alone is not neg); voice = {} (direct speech is labelled by the speech itself). To correct a field output a line <sent_id> PRAG <field>=<value>.\n\n", PRAG_FORM.join("|"), PRAG_ACT.join("|"), PRAG_POL.join("|"), PRAG_VOICE.join("|")));
    }
    for (it, b) in items {
        if let Some(w) = world.get(&it.id) {
            v.push_str(&format!("WORLD: {w}\n"));
        }
        v.push_str(&format!("=== {} — {}\n{}", it.id, it.text, write_block(&it.id, b)));
        if let Some(p) = prag.get(&it.id) {
            v.push_str(&format!("PRAG: {}\n\n", PRAG_FIELDS.iter().map(|f| format!("{f}={}", p.get(*f).map(String::as_str).unwrap_or("?"))).collect::<Vec<_>>().join(" ")));
        }
        if let Some(h) = hints.get(&it.id) {
            v.push_str("HINTS:\n");
            for x in h {
                v.push_str(&format!("- {x}\n"));
            }
            v.push('\n');
        }
    }
    (p, v)
}

/// Format rules — the same for both looks: full UD 2.18 for English (EWT); closed inventories
/// and the feature registry per part of speech come from `en::udreg`.
fn rules(role: &str) -> String {
    let reg = Registry::load();
    let tags: Vec<&str> = Tag::ALL.iter().filter(|t| !matches!(t, Tag::DQuote | Tag::X)).map(|t| t.name()).collect();
    let feats: String = UPos::ALL
        .iter()
        .map(|&u| {
            let f: Vec<&str> = reg.allowed(u).into_iter().map(|x| x.name()).collect();
            format!("  {u}: {}", if f.is_empty() { "_ (no features)".to_string() } else { f.join(" ") })
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "{role}\n\
         Tokenization is FIXED: never merge, split or change tokens.\n\
         For each sentence output a CoNLL-U block: first the line `# sent_id = <id>`, then one line per token with 10 TAB-separated columns: ID FORM LEMMA UPOS XPOS FEATS HEAD DEPREL DEPS MISC.\n\
         - UPOS: universal POS: ADJ ADP ADV AUX CCONJ DET INTJ NOUN NUM PART PRON PROPN PUNCT SCONJ SYM VERB X.\n\
         - XPOS: Penn Treebank tag exactly as in UD English EWT: {}\n\
         - FEATS: UD features exactly as in EWT, `|`-separated, sorted alphabetically by feature name (case-insensitive), `_` if none. Only these Feature=Value pairs are allowed per UPOS (UD 2.18 registry for English):\n{}\n\
         - AUX only for these lemmas: {}; `cop` only for lemma `be`.\n\
         - LEMMA: EWT conventions (n't → not; 's → be / have / 's by function; I stays I; proper nouns keep case; other words lowercase).\n\
         - Possessive pronouns (my, your, his, her, its, our, their, whose) and possessive nouns before 's attach as `nmod:poss`, never `det:poss`.\n\
         - Bare noun phrase modifiers (temporal: `last week`, `every day`; measure: `two feet long`) are `obl:unmarked` / `nmod:unmarked` (EWT since UD 2.15), never `:tmod` or `:npmod`.\n\
         - HEAD: number of the head token, 0 for the root; exactly one root per sentence.\n\
         - DEPREL: one of (UD 2.18 English, subtypes included): {}\n\
         - DEPS and MISC: write _ .\n\
         Separate blocks with a blank line. Output ONLY CoNLL-U blocks: no explanations, no code fences.\n",
        tags.join(" "),
        feats,
        reg.auxiliaries().join(" "),
        reg.deprels().into_iter().filter(|d| crate::ann::ewt_rel(d).is_none()).collect::<Vec<_>>().join(" ")
    )
}

/// Writes the call log.
pub fn log_calls(path: &Path, calls: &[Call]) -> Result<()> {
    let mut s = String::new();
    for c in calls {
        s.push_str(&serde_json::to_string(c)?);
        s.push('\n');
    }
    std::fs::write(path, s)?;
    Ok(())
}

/// API key for `claude -p` (calls go through the API by default): the line
/// `ANTHROPIC_API_KEY=…` in `~/Mova/local.md` (outside git, guarded by `secret-guard.sh`). No such line, or
/// `MOVA_SUBSCRIPTION=1` — the call goes through the subscription, as before. The key is never printed.
fn api_env() -> Vec<(String, String)> {
    if std::env::var("MOVA_SUBSCRIPTION").as_deref() == Ok("1") {
        return Vec::new();
    }
    local_key().map(|k| vec![("ANTHROPIC_API_KEY".to_string(), k)]).unwrap_or_default()
}

/// The key line of `~/Mova/local.md`, if any.
fn local_key() -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let text = std::fs::read_to_string(std::path::Path::new(&home).join("Mova/local.md")).ok()?;
    text.lines().find_map(|l| l.strip_prefix("ANTHROPIC_API_KEY=")).map(str::trim).filter(|k| !k.is_empty()).map(str::to_string)
}

/// API key for direct API calls (Message Batches): `~/Mova/local.md`, else env `ANTHROPIC_API_KEY`.
/// Never printed or written anywhere.
pub fn api_key() -> Option<String> {
    local_key().or_else(|| std::env::var("ANTHROPIC_API_KEY").ok().filter(|k| !k.trim().is_empty()))
}
