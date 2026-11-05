//! Plain English -> actual native UD -> bounded read-only tools. No LLM/shell fallback.
use anyhow::{Context, Result, bail, ensure};
use en::annotate::{Annotator, Word};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
};
use world::tree::Tree;
pub mod commands;
pub mod frame;
pub mod realize;
pub mod relay;
pub mod slots;
pub mod socket;
use slots::{SlotKind, SlotPolicy, SlotRule};
pub fn digest(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
const LOG_MAX: u64 = 65536;
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub model: PathBuf,
    pub model_sha256: String,
    pub capabilities: BTreeMap<String, Capability>,

    pub ledger: PathBuf,
    pub followup_queue: PathBuf,
    /// Address policy for typed prompt arguments; omitted (and not serialized) when default.
    #[serde(default, skip_serializing_if = "SlotPolicy::is_default")]
    pub slot_policy: SlotPolicy,
    /// Collective names over several addressable entities, e.g. "dolphin" over the aliases of
    /// each configured Dolphin. A singular collective name without one member's alias is refused
    /// as ambiguous; the plural ("the Dolphins") may address a capability of the group itself.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub group_aliases: BTreeMap<String, Vec<String>>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Capability {
    pub handler: String,
    pub parameters: Value,
    pub intent: IntentRule,
    /// Typed arguments bound from the prompt through the parse (`slots.rs`); none by default.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<SlotRule>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntentRule {
    pub request_predicates: Vec<String>,
    pub observations: Vec<String>,
    pub entities: Vec<String>,
    #[serde(default)]
    pub subordinate_predicates: Vec<String>,
    /// The capability belongs to the addressee of the whole channel: when no entity alias is said,
    /// the first alias is meant ("Fetch X." said to Sea Spider). Off by default.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub addressee: bool,
    /// Observations matched only by the native lemma of a word, never by dictionary variants
    /// (the variants of "a" include "can"); for short function words such as modals. None by default.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub native_lemma_observations: Vec<String>,
    /// The entity aliases name a group and bind only in the plural ("the Dolphins"). Off by default.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub collective: bool,
    /// The capability only reads state (a status, a count, a stored page, a permission check), so
    /// a question may select it. Without it the capability acts, and only a request (an
    /// imperative or a modal request: "Stop Eel.", "Could you stop Eel?") selects it; a polar or
    /// wh question about the action ("Is Eel stopped?", "How do I stop Eel?") never runs it.
    /// Off by default: an undeclared capability refuses questions rather than acting on them.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub observes: bool,
}
impl Config {
    pub fn validate(&self) -> Result<()> {
        ensure!(
            self.model.is_absolute()
                && self.ledger.is_absolute()
                && self.followup_queue.is_absolute(),
            "absolute trusted configuration required"
        );
        ensure!(
            self.capabilities.len() <= 64 && !self.capabilities.is_empty(),
            "require 1..64 configured capabilities"
        );
        for (name, c) in &self.capabilities {
            ensure!(id(name) && id(&c.handler), "invalid capability/handler id");
            for terms in [
                &c.intent.request_predicates,
                &c.intent.observations,
                &c.intent.entities,
                &c.intent.subordinate_predicates,
                &c.intent.native_lemma_observations,
            ] {
                ensure!(
                    terms.len() <= 64
                        && terms.iter().all(|s| !s.is_empty()
                            && s.len() <= 64
                            && s.bytes().all(|b| b.is_ascii_alphabetic())),
                    "intent terms must be bounded lexical items"
                );
            }
            ensure!(
                !c.intent.request_predicates.is_empty()
                    && !c.intent.observations.is_empty()
                    && !c.intent.entities.is_empty(),
                "intent rule requires predicates, observations and entity aliases"
            );
            if c.handler == "crab_control" {
                control_request(&c.parameters, "check")?;
            }
            if c.handler == "crab_consult" {
                consult_operation(&c.parameters, &c.slots)?;
            }
            if c.handler == relay::HANDLER {
                relay::validate(&c.parameters, &c.slots)?;
            }
            if c.handler == "crab_flow" {
                ensure!(c.slots.is_empty(), "crab_flow takes no prompt arguments");
                flow_request(&c.parameters, "check")?;
            }
            // The deputy's own acting operations can never be declared observing: a question must
            // not start, stop or restart a worker, enqueue a consultation or relay a task.
            let acting = match c.handler.as_str() {
                "crab_control" => c.parameters["operation"] != "worker_status",
                "crab_consult" => c.parameters["operation"] == "enqueue",
                h => h == relay::HANDLER,
            };
            ensure!(!(c.intent.observes && acting), "capability {name} acts; it cannot be declared observing");
            slots::validate_rules(&c.slots).map_err(anyhow::Error::msg)?;
        }
        ensure!(self.model_sha256.len() == 64, "model identity required");
        ensure!(self.group_aliases.len() <= 8, "at most eight group aliases");
        for (g, members) in &self.group_aliases {
            ensure!(
                !g.is_empty() && g.len() <= 32 && g.bytes().all(|b| b.is_ascii_lowercase()),
                "a group alias is one lowercase word"
            );
            ensure!(
                members.len() >= 2 && members.len() <= 32,
                "a group names 2..32 member aliases"
            );
            for m in members {
                ensure!(
                    self.capabilities.values().any(|c| c.intent.entities.contains(m)) && m != g,
                    "group member {m:?} must be a configured entity alias"
                );
            }
        }
        Ok(())
    }
}
fn id(s: &str) -> bool {
    s.as_bytes().first().is_some_and(u8::is_ascii_alphanumeric)
        && s.len() <= 80
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Action {
    pub capability: String,
    pub matched_entities: Vec<String>,
    pub matched_observations: Vec<String>,
    /// Zero-based native UD node witnesses; argument observations descend from request heads.
    pub native_request_heads: Vec<usize>,
    pub native_observation_nodes: Vec<usize>,
    /// Validated prompt arguments by slot name, in prompt order (empty and omitted without slots).
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub arguments: BTreeMap<String, Vec<String>>,
    /// Zero-based native nodes of the argument placeholders, per sentence.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub native_argument_nodes: Vec<usize>,
}
/// Public extension boundary. Implementations are installed Rust modules, never prompt-supplied code.
/// In-process Crab observation for a deputy embedded in Crab: same read-only operations
/// and output shape as `CrabObservation`, answered by a callback into Crab's own dispatch.
pub struct CrabInProcess<'a>(pub &'a dyn Fn(&Value) -> std::result::Result<Value, String>);
impl CapabilityHandler for CrabInProcess<'_> {
    fn execute(&self, p: &Value, _: &Action, invocation_id: &str) -> Result<Value> {
        let op = p["operation"].as_str().context("observation operation required")?;
        ensure!(["status", "read_reports"].contains(&op), "adapter admits only read-only Crab operations");
        let q = json!({"schema":1,"id":invocation_id,"op":op,"limit":3});
        let v = (self.0)(&q).map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(json!({"request":q,"response_sha256":digest(&serde_json::to_vec(&v)?),"response":v,"health_inference":"not_established_by_transport","transport":"in_process"}))
    }
}
/// Explicit worker control for a deputy embedded in Crab. The operation and worker come from
/// trusted capability parameters, never from prompt text; the prompt only selects the
/// configured capability. Crab validates the worker name against its own configuration.
pub struct CrabControl<'a>(pub &'a dyn Fn(&Value) -> std::result::Result<Value, String>);
pub const CONTROL_OPERATIONS: [&str; 4] =
    ["worker_status", "worker_start", "worker_stop", "worker_restart"];
fn control_request(p: &Value, invocation_id: &str) -> Result<Value> {
    let op = p["operation"].as_str().context("control operation required")?;
    ensure!(
        CONTROL_OPERATIONS.contains(&op),
        "crab_control admits only worker_status, worker_start, worker_stop and worker_restart"
    );
    let mut q = json!({"schema":1,"id":invocation_id,"op":op});
    match p["worker"].as_str() {
        Some(w) => {
            ensure!(id(w) && w.len() <= 64, "invalid configured worker name");
            q["worker"] = json!(w);
        }
        None => ensure!(op == "worker_status", "worker required for {op}"),
    }
    match p.get("immediate") {
        None => {}
        Some(Value::Bool(b)) => {
            ensure!(op == "worker_stop", "immediate belongs only to worker_stop");
            q["immediate"] = json!(b);
        }
        Some(_) => anyhow::bail!("immediate must be a boolean"),
    }
    ensure!(
        p.as_object()
            .is_some_and(|m| m.keys().all(|k| ["operation", "worker", "immediate"].contains(&k.as_str()))),
        "crab_control parameters are operation, worker and immediate only"
    );
    Ok(q)
}
impl CapabilityHandler for CrabControl<'_> {
    fn execute(&self, p: &Value, _: &Action, invocation_id: &str) -> Result<Value> {
        let q = control_request(p, invocation_id)?;
        let v = (self.0)(&q).map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(json!({"request":q,"response_sha256":digest(&serde_json::to_vec(&v)?),"response":v,
            "transport":"in_process","completion":"request recorded; observe worker status","health_inference":"not_established_by_transport"}))
    }
}
/// The Octopus's channel to its named Dolphins through Crab. `enqueue` hands the quoted text
/// bound by the parse (slot `task`, kind text, exactly one) to Crab's consult queue for the
/// configured Dolphin, which delivers it only when that Dolphin's terminal is idle and never
/// overwrites an active task; `state` reads that Dolphin's queue, pending consultation and latest
/// report; `list` lists every Dolphin with its state. Operation and Dolphin name come from trusted
/// capability parameters; only the quoted text comes from the prompt.
pub struct CrabConsult<'a>(pub &'a dyn Fn(&Value) -> std::result::Result<Value, String>);
pub const CONSULT_OPERATIONS: [&str; 3] = ["enqueue", "state", "list"];
fn consult_operation(p: &Value, slots: &[SlotRule]) -> Result<&'static str> {
    let op = p["operation"].as_str().context("consult operation required")?;
    let op = CONSULT_OPERATIONS
        .into_iter()
        .find(|o| *o == op)
        .context("crab_consult admits only enqueue, state and list")?;
    ensure!(
        p.as_object().is_some_and(|m| m.keys().all(|k| k == "operation" || k == "dolphin")),
        "crab_consult parameters are operation and dolphin only"
    );
    match p.get("dolphin") {
        Some(Value::String(d)) => ensure!(op != "list" && id(d) && d.len() <= 64, "invalid Dolphin name for {op}"),
        None => ensure!(op == "list", "{op} needs the configured Dolphin name"),
        Some(_) => anyhow::bail!("dolphin must be a name"),
    }
    let task_slot = slots
        .iter()
        .any(|s| s.name == "task" && s.kinds == [SlotKind::Text] && s.min == 1 && s.max == 1);
    match op {
        "enqueue" => ensure!(task_slot && slots.len() == 1, "enqueue needs exactly one slot: task, kind text, 1..1"),
        _ => ensure!(slots.is_empty(), "{op} takes no prompt arguments"),
    }
    Ok(op)
}
impl CapabilityHandler for CrabConsult<'_> {
    fn execute(&self, p: &Value, a: &Action, invocation_id: &str) -> Result<Value> {
        let op = p["operation"].as_str().context("consult operation required")?;
        let dolphin = &p["dolphin"];
        let q = match op {
            "enqueue" => {
                let [task] = a.arguments.get("task").map(Vec::as_slice).unwrap_or_default() else {
                    anyhow::bail!("enqueue needs exactly one quoted task bound by the parse");
                };
                json!({"schema":1,"id":invocation_id,"op":"consult_enqueue","dolphin":dolphin,"task":task})
            }
            "state" => json!({"schema":1,"id":invocation_id,"op":"consult_state","dolphin":dolphin}),
            "list" => json!({"schema":1,"id":invocation_id,"op":"dolphin_list"}),
            _ => anyhow::bail!("crab_consult admits only enqueue, state and list"),
        };
        let v = (self.0)(&q).map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(json!({"request":q,"response_sha256":digest(&serde_json::to_vec(&v)?),"response":v,
            "transport":"in_process","advice_standing":"evidence for the Octopus, never an action"}))
    }
}
/// Work-flow observation through Crab: `status` (optionally one configured animal kind, the
/// `group`) answers "Do the Fish have work?", "Is any worker clogged?" and "Show the Fish
/// queues."; `attention` answers "What needs my attention?". Read-only; the operation and kind
/// come from trusted capability parameters, never from prompt text.
pub struct CrabFlow<'a>(pub &'a dyn Fn(&Value) -> std::result::Result<Value, String>);
fn flow_request(p: &Value, invocation_id: &str) -> Result<Value> {
    ensure!(
        p.as_object().is_some_and(|m| m.keys().all(|k| k == "operation" || k == "group")),
        "crab_flow parameters are operation and group only"
    );
    match (p["operation"].as_str(), p.get("group")) {
        (Some("status"), None) => Ok(json!({"schema":1,"id":invocation_id,"op":"flow_status"})),
        (Some("status"), Some(Value::String(g))) => {
            ensure!(id(g) && g.len() <= 64, "invalid animal kind");
            Ok(json!({"schema":1,"id":invocation_id,"op":"flow_status","group":g}))
        }
        (Some("attention"), None) => Ok(json!({"schema":1,"id":invocation_id,"op":"attention"})),
        _ => anyhow::bail!("crab_flow admits status (with an optional group) and attention"),
    }
}
impl CapabilityHandler for CrabFlow<'_> {
    fn execute(&self, p: &Value, _: &Action, invocation_id: &str) -> Result<Value> {
        let q = flow_request(p, invocation_id)?;
        let v = (self.0)(&q).map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(json!({"request":q,"response_sha256":digest(&serde_json::to_vec(&v)?),"response":v,
            "transport":"in_process","health_inference":"not_established_by_transport"}))
    }
}
pub trait CapabilityHandler {
    fn execute(&self, parameters: &Value, action: &Action, invocation_id: &str) -> Result<Value>;
}
pub struct CrabObservation;
impl CapabilityHandler for CrabObservation {
    fn execute(&self, p: &Value, _: &Action, invocation_id: &str) -> Result<Value> {
        let socket = Path::new(p["socket"].as_str().context("trusted socket required")?);
        let op = p["operation"]
            .as_str()
            .context("observation operation required")?;
        ensure!(
            ["status", "read_reports"].contains(&op),
            "adapter admits only read-only Crab operations"
        );
        let q = json!({"schema":1,"id":invocation_id,"op":op,"limit":3});
        let v = mova_crab::angel::call(socket, &q).map_err(|e| anyhow::anyhow!("{e}"))?;
        Ok(
            json!({"request":q,"response_sha256":digest(&serde_json::to_vec(&v)?),"response":v,"health_inference":"not_established_by_transport"}),
        )
    }
}
pub struct LogDiagnosis;
impl CapabilityHandler for LogDiagnosis {
    fn execute(&self, p: &Value, _: &Action, _: &str) -> Result<Value> {
        let path = Path::new(p["path"].as_str().context("trusted log path required")?);
        ensure!(path.is_absolute(), "absolute trusted log path required");
        diagnose(path)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeWord {
    pub form: String,
    pub lemma: String,
    pub upos: String,
    pub tag: String,
    pub feats_decimal: String,
    pub head: usize,
    pub rel: String,
}
impl From<&Word> for NativeWord {
    fn from(w: &Word) -> Self {
        Self {
            form: w.form.clone(),
            lemma: w.lemma.clone(),
            upos: format!("{:?}", w.upos),
            tag: format!("{:?}", w.tag),
            feats_decimal: format!("{}", w.feats.0),
            head: w.head,
            rel: format!("{:?}", w.rel),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub text: String,
    pub words: Vec<NativeWord>,
    pub native_question_frame: String,
    /// The deputy's own role frame from the native parse (`frame.rs`): mood, subject, verb.
    pub deputy_frame: frame::Frame,
    pub syntax_admitted: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Failure {
    pub stage: String,
    pub reason: String,
    /// For an ambiguity: the configured readings the prompt could mean (entity aliases or
    /// capabilities), so the asker can say which one. Empty and omitted otherwise.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub candidates: Vec<String>,
}
fn failure(stage: &str, reason: impl Into<String>) -> Failure {
    Failure {
        stage: stage.into(),
        reason: reason.into(),
        candidates: vec![],
    }
}
fn ambiguity(reason: impl Into<String>, candidates: impl IntoIterator<Item = String>) -> Failure {
    let candidates: BTreeSet<String> = candidates.into_iter().collect();
    let list: Vec<String> = candidates.into_iter().collect();
    Failure {
        stage: "pragmatic_ambiguity".into(),
        reason: format!("{}; candidates: {}", reason.into(), list.join(", ")),
        candidates: list,
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub schema: u32,
    pub engine: String,
    pub native_sources: Value,
    pub source_sha256: String,
    pub prompt: String,
    pub prompt_sha256: String,
    pub config_sha256: String,
    pub model_sha256: String,
    pub observations: Vec<Observation>,
    pub pragmatic_engine: Value,
    pub plan: Vec<Action>,
    pub results: Vec<Value>,
    pub failure: Option<Failure>,
    pub followups: Vec<Value>,
    pub training_authorized: bool,
}
pub struct Deputy {
    config: Config,
    annotator: Annotator,
    handlers: BTreeMap<String, Box<dyn CapabilityHandler>>,
}
impl Deputy {
    pub fn load(config: Config) -> Result<Self> {
        config.validate()?;
        ensure!(
            std::env::var_os("WORLD_NO_REPAIR").is_none(),
            "native repair override is not admitted by this deputy profile"
        );
        let bytes = fs::read(&config.model)?;
        ensure!(
            digest(&bytes) == config.model_sha256,
            "syntax model hash mismatch"
        );
        let annotator = Annotator::from_bytes(&bytes)?;
        let handlers: BTreeMap<String, Box<dyn CapabilityHandler>> = BTreeMap::from([
            (
                "crab_observe".into(),
                Box::new(CrabObservation) as Box<dyn CapabilityHandler>,
            ),
            (
                "log_tail".into(),
                Box::new(LogDiagnosis) as Box<dyn CapabilityHandler>,
            ),
            (
                relay::HANDLER.into(),
                Box::new(relay::AnimalRelay) as Box<dyn CapabilityHandler>,
            ),
        ]);
        Ok(Self {
            config,
            annotator,
            handlers,
        })
    }
    pub fn with_handler(mut self, id: String, handler: Box<dyn CapabilityHandler>) -> Result<Self> {
        ensure!(
            self.config.capabilities.values().any(|c| c.handler == id),
            "handler is not configured"
        );
        ensure!(
            !self.handlers.contains_key(&id),
            "handler already registered"
        );
        self.handlers.insert(id, handler);
        Ok(self)
    }
    pub fn review(&self, prompt: &str) -> Report {
        let mut r = Report {
            schema: 1, engine: "Mova en::Annotator + world::tree::Tree::parse_cased + world::ans::parse_question; bounded native deputy v1".into(),
            native_sources: json!({
                "en_annotate_sha256":digest(include_bytes!("../../../en/src/annotate.rs")),
                "world_tree_sha256":digest(include_bytes!("../../../world/src/tree.rs")),
                "world_rerank_sha256":digest(include_bytes!("../../../world/src/rerank.rs")),
                "world_question_frame_sha256":digest(include_bytes!("../../../world/src/ans.rs")),
                "cli_sha256":digest(include_bytes!("main.rs"))
            }),
            source_sha256: digest(include_bytes!("lib.rs")),
            prompt: prompt.into(), prompt_sha256: digest(prompt.as_bytes()),
            config_sha256: digest(&serde_json::to_vec(&self.config).unwrap()),
            model_sha256: self.config.model_sha256.clone(), observations: vec![],
            pragmatic_engine: json!({"status":"missing","reason":"No trained pragmatic inference invoked. Native question frame is a separate deterministic reader observation; intent resolver is a bounded skeleton rule."}),
            plan: vec![], results: vec![], failure: None, followups: vec![], training_authorized: false,
        };
        // Typed arguments are masked only when some capability declares slots, so configurations
        // without slots read exactly the text they did before.
        let slots_on = self.config.capabilities.values().any(|c| !c.slots.is_empty());
        let ids_on = self.config.capabilities.values().flat_map(|c| &c.slots).any(|s| s.kinds.contains(&SlotKind::Id));
        let masked = if slots_on {
            slots::mask_with(prompt, &self.config.slot_policy, ids_on)
        } else {
            Ok(slots::Masked { text: prompt.to_string(), arguments: vec![] })
        };
        let (text, arguments) = match masked {
            Ok(m) => (m.text, m.arguments),
            Err(reason) => {
                r.failure = Some(failure("argument_invalid", reason));
                (String::new(), vec![])
            }
        };
        let mut bound_arguments = BTreeSet::new();
        // Each sentence with whether its own terminator is a question mark (placeholders have
        // already masked the question marks inside URLs and quotations).
        let sentences: Vec<(&str, bool)> = text
            .split_inclusive(['.', '!', '?', '\n'])
            .map(|s| (s.trim_end_matches(['.', '!', '?', '\n']).trim(), s.ends_with('?')))
            .filter(|(s, _)| !s.is_empty())
            .collect();
        if r.failure.is_some() {
        } else if prompt.len() > 8192
            || sentences.is_empty()
            || sentences.len() > 8
            || prompt.contains('\0')
        {
            r.failure = Some(failure(
                "input_boundary",
                "Require 1..8 ordinary sentences, at most 8192 UTF-8 bytes",
            ));
        } else {
            let request_verbs: BTreeSet<String> = self
                .config
                .capabilities
                .values()
                .flat_map(|c| c.intent.request_predicates.iter().cloned())
                .collect();
            let mut context = None;
            for (s, asked) in sentences {
                let tree = Tree::parse_cased(&self.annotator, s);
                let admitted = syntax_valid(&tree.w);
                // The deputy reasons over the words after its own frame repairs (an imperative read
                // as a noun phrase); the observation keeps the native parse.
                let (words, deputy_frame) = if admitted {
                    frame::read_repairing(&tree.w, &request_verbs)
                } else {
                    (vec![], frame::read(&[]))
                };
                let obs = Observation {
                    text: s.into(),
                    words: tree.w.iter().map(NativeWord::from).collect(),
                    native_question_frame: world::ans::frame_str(&world::ans::parse_question(s)),
                    deputy_frame: deputy_frame.clone(),
                    syntax_admitted: admitted,
                };
                let decision = if s.contains(['"', '“', '”']) {
                    Err(failure(
                        "pragmatic_ambiguity",
                        "Quoted instruction scope is not admitted",
                    ))
                } else if admitted {
                    // A bare group name is reported as ambiguous even when nothing else matched.
                    match resolve(&words, &deputy_frame, asked, &self.config.capabilities, &mut context) {
                        Ok(actions) => ambiguous_group(&words, &self.config.group_aliases, actions),
                        Err(f) => ambiguous_group(&words, &self.config.group_aliases, vec![]).and(Err(f)),
                    }
                        .and_then(|actions| {
                        if slots_on {
                            bind(&words, actions, &self.config.capabilities, &arguments, &mut bound_arguments)
                        } else {
                            Ok(actions)
                        }
                    })
                } else {
                    Err(failure(
                        "syntax_unadmitted",
                        "Native UD is empty, cyclic, over budget, or has invalid roots/heads",
                    ))
                };
                r.observations.push(obs);
                match decision {
                    Ok(actions) => r.plan.extend(actions),
                    Err(f) => {
                        if r.failure.is_none() {
                            r.failure = Some(f);
                        }
                    }
                }
            }
            if r.plan.len() > 8 {
                r.failure = Some(failure("plan_boundary", "At most eight actions"));
            }
            if r.failure.is_none() && bound_arguments.len() != arguments.len() {
                r.failure = Some(failure(
                    "argument_unbound",
                    "A URL or host of the request is not a parsed argument of a requested action",
                ));
            }
        }
        if let Some(f) = &r.failure {
            let target = match f.stage.as_str() {
                "syntax_unadmitted" => "fish_ast_review",
                "pragmatic_ambiguity" | "missing_context" | "argument_unbound" | "argument_ambiguous" => {
                    "fish_pragmatics_review"
                }
                _ => "skeleton_capability_review",
            };
            let identity = digest(
                &serde_json::to_vec(&json!([
                    r.source_sha256,
                    r.model_sha256,
                    r.prompt_sha256,
                    r.config_sha256,
                    r.native_sources,
                    "native_default_no_override",
                    f
                ]))
                .unwrap(),
            );
            let dedupe_key = dedupe_key(&r.prompt_sha256, &f.stage);
            r.followups.push(json!({"schema":1,"candidate_id":identity,"dedupe_key":dedupe_key,"priority":"operator_highest","status":"review_only","suggested_lane":target,
                "prompt":r.prompt,"prompt_sha256":r.prompt_sha256,"model_sha256":r.model_sha256,"source_sha256":r.source_sha256,
                "context":{"config_sha256":r.config_sha256,"repair_profile":"native_default_no_override","native_sources":r.native_sources,"observations":r.observations,"pragmatic_engine":r.pragmatic_engine},
                "failure":f,"heldout_admission":"not_checked","privacy_review":"required","training_authorized":false}));
        }
        r
    }
    /// The review queue holds one file per (prompt, failure stage): `<dedupe_key>.json`. The first
    /// candidate is kept byte-for-byte in its fields; every build (source, model, configuration and
    /// native sources) that reproduced the same refusal is listed once in `reproduced_by`, with how
    /// often it did. Report followups keep their per-build `candidate_id`.
    pub fn publish_followups(&self, r: &Report) -> Result<()> {
        if r.followups.is_empty() {
            return Ok(());
        }
        fs::create_dir_all(&self.config.followup_queue)?;
        for v in &r.followups {
            let key = v["dedupe_key"].as_str().context("missing dedupe key")?;
            let path = self.config.followup_queue.join(format!("{key}.json"));
            let build = json!({"candidate_id":v["candidate_id"],"source_sha256":v["source_sha256"],
                "model_sha256":v["model_sha256"],"config_sha256":v["context"]["config_sha256"],
                "native_sources_sha256":digest(&serde_json::to_vec(&v["context"]["native_sources"])?),
                "reason":v["failure"]["reason"],"seen":1});
            let merged = if path.exists() {
                let mut old: Value = serde_json::from_slice(&bounded_file(&path, 4_000_000)?)?;
                ensure!(
                    old["dedupe_key"] == v["dedupe_key"] && old["prompt_sha256"] == v["prompt_sha256"],
                    "review queue key conflict at {}",
                    path.display()
                );
                let list = old["reproduced_by"].as_array_mut().context("reproduced_by list")?;
                match list.iter_mut().find(|b| b["candidate_id"] == v["candidate_id"]) {
                    Some(b) => b["seen"] = json!(b["seen"].as_u64().unwrap_or(1) + 1),
                    None => {
                        ensure!(list.len() < 256, "review candidate reproduced by too many builds");
                        list.push(build)
                    }
                }
                old
            } else {
                let mut first = v.clone();
                first["reproduced_by"] = json!([build]);
                first
            };
            replace_file(&path, &serde_json::to_vec_pretty(&merged)?)?;
        }
        File::open(&self.config.followup_queue)?.sync_all()?;
        Ok(())
    }
    /// Reserve before any tool call; an attempt without a receipt is never replayed.
    pub fn run(&self, prompt: &str, request_id: &str, execute: bool) -> Result<Report> {
        self.run_with(prompt, request_id, execute, &BTreeMap::new())
    }
    /// As `run`, with per-call handler overrides by handler name. An embedding process (Crab)
    /// uses this to answer `crab_observe` in-process instead of over its own busy socket.
    pub fn run_with(
        &self,
        prompt: &str,
        request_id: &str,
        execute: bool,
        overrides: &BTreeMap<String, &dyn CapabilityHandler>,
    ) -> Result<Report> {
        ensure!(id(request_id), "invalid invocation id");
        let mut r = self.review(prompt);
        self.publish_followups(&r)?;
        if !execute || r.failure.is_some() {
            return Ok(r);
        }
        fs::create_dir_all(&self.config.ledger)?;
        let dir = self.config.ledger.join(request_id);
        match fs::create_dir(&dir) {
            Ok(()) => {
                File::open(&self.config.ledger)?.sync_all()?;
                write_new(&dir.join("reservation.json"), &serde_json::to_vec(&r)?)?;
                File::open(&dir)?.sync_all()?;
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                let saved: Report = serde_json::from_slice(&bounded_file(
                    &dir.join("reservation.json"),
                    2_000_000,
                )?)?;
                ensure!(
                    saved.prompt_sha256 == r.prompt_sha256
                        && saved.config_sha256 == r.config_sha256
                        && saved.source_sha256 == r.source_sha256
                        && saved.native_sources == r.native_sources,
                    "invocation id conflict"
                );
                if !dir.join("receipt.json").exists() {
                    bail!("uncertain prior attempt: reservation has no receipt; no replay");
                }
                return Ok(serde_json::from_slice(&bounded_file(
                    &dir.join("receipt.json"),
                    2_000_000,
                )?)?);
            }
            Err(e) => return Err(e.into()),
        }
        for (i, a) in r.plan.iter().enumerate() {
            let c = &self.config.capabilities[&a.capability];
            let tool=overrides.get(&c.handler).copied()
                .or_else(|| self.handlers.get(&c.handler).map(|h| h.as_ref()))
                .context("configured handler is not installed")
                .and_then(|h|h.execute(&c.parameters,a,&format!("octopus-{request_id}-{i}")))
                .map(|v|json!({"action":a,"output_sha256":digest(&serde_json::to_vec(&v).unwrap()),"output":v}));
            match tool {
                Ok(v) => r.results.push(v),
                Err(e) => {
                    r.failure = Some(failure(
                        "execution_failure",
                        format!("{e:#}; no automatic replay"),
                    ));
                    break;
                }
            }
        }
        write_new(&dir.join("receipt.json"), &serde_json::to_vec_pretty(&r)?)?;
        File::open(&dir)?.sync_all()?;
        Ok(r)
    }
}

fn syntax_valid(w: &[Word]) -> bool {
    if w.is_empty() || w.len() > 160 || w.iter().filter(|x| x.head == 0).count() != 1 {
        return false;
    }
    for i in 0..w.len() {
        let mut seen = BTreeSet::new();
        let mut at = i;
        loop {
            if !seen.insert(at) || w[at].head > w.len() {
                return false;
            }
            if w[at].head == 0 {
                break;
            }
            at = w[at].head - 1;
        }
    }
    true
}
fn variants(w: &Word) -> BTreeSet<String> {
    let mut out: BTreeSet<_> = world::ans::lemmas(&w.form.to_lowercase())
        .into_iter()
        .collect();
    out.insert(w.lemma.to_lowercase());
    out
}
fn has(w: &[Word], words: &[&str]) -> bool {
    w.iter()
        .any(|x| variants(x).iter().any(|s| words.contains(&s.as_str())))
}
fn descends(w: &[Word], mut node: usize, ancestor: usize) -> bool {
    for _ in 0..w.len() {
        if node == ancestor {
            return true;
        }
        if w[node].head == 0 {
            return false;
        }
        node = w[node].head - 1;
    }
    false
}
/// A singular group name ("the Dolphin") without one member's alias does not say which entity is
/// meant: refused, never routed to a default member. Only a capability that names the singular
/// word itself as its own entity may take it (an explicit configuration, not a guess).
fn plural(x: &Word, g: &str) -> bool {
    matches!(x.tag, en::gram::Tag::NNS | en::gram::Tag::NNPS) || x.form.to_lowercase() == format!("{g}s")
}
fn ambiguous_group(
    w: &[Word],
    groups: &BTreeMap<String, Vec<String>>,
    actions: Vec<Action>,
) -> std::result::Result<Vec<Action>, Failure> {
    for (g, members) in groups {
        let singular = w.iter().any(|x| variants(x).contains(g) && !plural(x, g));
        let named = members.iter().any(|m| has(w, &[m.as_str()]));
        let explicit = actions.iter().any(|a| a.matched_entities.contains(g));
        if singular && !named && !explicit {
            return Err(ambiguity(
                format!("\"{g}\" names {} configured entities; say which one", members.len()),
                members.iter().cloned(),
            ));
        }
    }
    Ok(actions)
}
/// "What is the status of X?": a root "what"/"which" with a copula asks about its nominal
/// subject, so the subject noun stands for the question's predicate.
fn copular_wh_subject<'a>(w: &'a [Word], p: &Word) -> Option<&'a Word> {
    let root = w.iter().position(|x| std::ptr::eq(x, p))? + 1;
    let wh = matches!(p.lemma.to_lowercase().as_str(), "what" | "which");
    let cop = w.iter().any(|x| x.head == root && x.rel.base() == en::gram::Rel::Cop);
    if !(wh && cop) {
        return None;
    }
    w.iter().find(|x| x.head == root && x.rel.base() == en::gram::Rel::Nsubj && x.upos == en::gram::UPos::NOUN)
}
fn resolve(
    w: &[Word],
    frame: &frame::Frame,
    asked: bool,
    capabilities: &BTreeMap<String, Capability>,
    context: &mut Option<String>,
) -> std::result::Result<Vec<Action>, Failure> {
    if has(w, &["not", "never", "unless", "without", "if", "either"])
        || w.iter().any(|x| x.form.contains("n't"))
    {
        return Err(failure(
            "pragmatic_ambiguity",
            "Negation, alternatives and conditional scope are not admitted",
        ));
    }
    // A repaired polar frame ("Is the documenter running?" parsed with «documenter» as root) asks
    // with its verb slot: the subject noun is not a predicate, the recovered verb or adjective is.
    let polar = frame.mood == frame::Mood::Polar;
    let repaired_subject = if polar && frame.repaired { frame.subject } else { None };
    let frame_verb = if polar && frame.repaired { frame.verb } else { None };
    let predicates: Vec<(usize, &Word)> = w
        .iter()
        .enumerate()
        .filter(|(i, x)| {
            Some(*i) != repaired_subject
                && (matches!(x.upos, en::gram::UPos::VERB | en::gram::UPos::AUX)
                    || x.head == 0
                    || Some(*i) == frame_verb)
        })
        .collect();
    let mut allowed: BTreeSet<String> = [
        "be", "do", "have", "can", "could", "would", "should", "please",
    ]
    .map(str::to_string)
    .into_iter()
    .collect();
    for c in capabilities.values() {
        allowed.extend(c.intent.request_predicates.iter().cloned());
        allowed.extend(c.intent.subordinate_predicates.iter().cloned());
    }
    // A polar (yes/no) question — subject-auxiliary inversion ("Is Crab running normally?",
    // "Are the fish okay?") — asks to observe a state, like "what"/"how" questions. Native
    // signal: the first word is a non-past AUX attached to the root clause (the native word list
    // carries no punctuation, so inversion itself marks the question). Past tense asks about
    // history, not current state, and stays refused.
    // The deputy frame also marks inversion when the parser hung the auxiliary on a name inside
    // the subject («Is Opus Dolphin busy?»: «is» → «opus» → «dolphin»).
    let polar_question = w.first().is_some_and(|x| {
        x.upos == en::gram::UPos::AUX
            && x.tag != en::gram::Tag::VBD
            && (x.head == 0
                || w.get(x.head - 1).is_some_and(|h| h.head == 0)
                || frame.mood == frame::Mood::Polar)
    });
    for &(pi, p) in &predicates {
        let clause_head = p.head == 0 || Some(pi) == frame_verb;
        if clause_head && p.tag == en::gram::Tag::VBD {
            return Err(failure(
                "pragmatic_ambiguity",
                "Past narrative is not an instruction",
            ));
        }
        let observed = |x: &Word| {
            capabilities.values().any(|c| {
                variants(x)
                    .iter()
                    .any(|s| c.intent.observations.contains(s))
            })
        };
        let nominal_question = clause_head
            && (has(w, &["what", "which", "how"]) || polar_question)
            && (observed(p)
                || copular_wh_subject(w, p).is_some_and(observed)
                // A polar frame asks about its subject: an observed subject noun admits the
                // frame's predicate ("Is the queue empty?" asks about the configured observation
                // «queue»), whether the parse put «empty» at the root or needed the repair.
                || (polar && Some(pi) == frame.verb && frame.subject.is_some_and(|s| observed(&w[s]))));
        if !nominal_question && !variants(p).iter().any(|s| allowed.contains(s)) {
            return Err(failure(
                "unsupported_capability",
                format!(
                    "Native predicate {:?} ({:?}) has no configured capability",
                    p.form, p.rel
                ),
            ));
        }
    }
    if !polar_question
        && !has(w, &["what", "which", "how", "could", "can", "would"])
        && w.iter().any(|x| {
            x.rel.base() == en::gram::Rel::Nsubj
                && x.head > 0
                && w[x.head - 1].head == 0
                && x.form != "you"
        })
    {
        return Err(failure(
            "pragmatic_ambiguity",
            "Declarative subject scope is not admitted as an instruction",
        ));
    }
    let mut actions = vec![];
    let mut entities = BTreeSet::new();
    let mut observed = false;
    // A polar or wh question asks about a state; only an observing capability answers it. An
    // acting capability that a question would otherwise select is withheld, never run. A sentence
    // that ends in a question mark is a question too whatever its parse ("Have the Whales
    // resumed?" read as the causative imperative), unless it is a modal request ("Could you ...?").
    let question = matches!(frame.mood, frame::Mood::Polar | frame::Mood::Wh)
        || (asked && frame.mood != frame::Mood::ModalRequest);
    let mut withheld = false;
    for (name, c) in capabilities {
        let lemma_is = |x: &Word, s: &str| x.lemma.eq_ignore_ascii_case(s);
        let observations: Vec<_> = c
            .intent
            .observations
            .iter()
            .filter(|s| has(w, &[s.as_str()]))
            .chain(c.intent.native_lemma_observations.iter().filter(|s| w.iter().any(|x| lemma_is(x, s))))
            .cloned()
            .collect();
        if observations.is_empty() {
            continue;
        }
        observed = true;
        // A collective capability addresses its group only in the plural ("the Dolphins").
        let direct: Vec<_> = c
            .intent
            .entities
            .iter()
            .filter(|s| {
                if c.intent.collective {
                    w.iter().any(|x| variants(x).contains(s.as_str()) && plural(x, s))
                } else {
                    has(w, &[s.as_str()])
                }
            })
            .cloned()
            .collect();
        let bound = if direct.is_empty() && has(w, &["its", "their", "those"]) {
            context
                .as_ref()
                .filter(|s| c.intent.entities.contains(s))
                .map(|s| vec![s.clone()])
                .unwrap_or_default()
        } else {
            direct
        };
        let bound = if bound.is_empty() && c.intent.addressee {
            vec![c.intent.entities[0].clone()]
        } else {
            bound
        };
        if bound.is_empty() {
            continue;
        }
        if bound.len() > 1 {
            return Err(ambiguity("Multiple entity aliases match a capability", bound));
        }
        let request_heads: Vec<_> = w
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                (matches!(p.upos, en::gram::UPos::VERB | en::gram::UPos::AUX)
                    && variants(p)
                        .iter()
                        .any(|s| c.intent.request_predicates.contains(s)))
                    || (p.head == 0 && (has(w, &["what", "which", "how"]) || polar_question))
            })
            .map(|(i, _)| i)
            .collect();
        let observation_nodes: Vec<_> = w
            .iter()
            .enumerate()
            .filter(|(_, p)| {
                variants(p).iter().any(|s| observations.contains(s) && c.intent.observations.contains(s))
                    || c.intent.native_lemma_observations.iter().any(|s| observations.contains(s) && lemma_is(p, s))
            })
            .map(|(i, _)| i)
            .collect();
        let related = observation_nodes
            .iter()
            .any(|n| request_heads.iter().any(|h| descends(w, *n, *h)));
        if !related {
            continue;
        }
        if question && !c.intent.observes {
            withheld = true;
            continue;
        }
        entities.extend(bound.iter().cloned());
        actions.push(Action {
            capability: name.clone(),
            matched_entities: bound,
            matched_observations: observations,
            native_request_heads: request_heads,
            native_observation_nodes: observation_nodes,
            arguments: BTreeMap::new(),
            native_argument_nodes: vec![],
        });
    }
    if actions.is_empty() && withheld {
        return Err(failure(
            "pragmatic_ambiguity",
            "A question about an action does not request it; only observing capabilities answer questions",
        ));
    }
    if actions.is_empty() {
        return Err(failure(
            if observed {
                "missing_context"
            } else {
                "unsupported_capability"
            },
            if observed {
                "Configured observation lacks an unambiguous entity/request binding"
            } else {
                "No configured capability matches these native observations"
            },
        ));
    }
    // The most specific reading wins: a candidate whose matched observations are a strict subset
    // of another candidate's for the same entity is dropped ("Stop Eel now." is the urgent stop,
    // not also the ordinary one). Equal or crossing matches stay ambiguous below.
    // With equal matches the reading that the request explains more completely wins: the share
    // of the capability's configured observations that were actually said ("Stop Eel." fully
    // describes the ordinary stop and only a third of the urgent one).
    let coverage = |a: &Action| {
        let i = &capabilities[&a.capability].intent;
        let total = (i.observations.len() + i.native_lemma_observations.len()).max(1);
        a.matched_observations.len() as f64 / total as f64
    };
    let dominated: BTreeSet<usize> = (0..actions.len())
        .filter(|&i| {
            (0..actions.len()).any(|j| {
                let (a, b) = (&actions[i], &actions[j]);
                let subset = a.matched_observations.iter().all(|s| b.matched_observations.contains(s));
                j != i
                    && a.matched_entities == b.matched_entities
                    && subset
                    && (a.matched_observations.len() < b.matched_observations.len()
                        || (a.matched_observations.len() == b.matched_observations.len()
                            && coverage(a) < coverage(b)))
            })
        })
        .collect();
    let actions: Vec<Action> = actions
        .into_iter()
        .enumerate()
        .filter(|(i, _)| !dominated.contains(i))
        .map(|(_, a)| a)
        .collect();
    // Independent candidates for the same observation/entity are ambiguous, not additive.
    for i in 0..actions.len() {
        for j in i + 1..actions.len() {
            if actions[i].matched_entities == actions[j].matched_entities
                && actions[i]
                    .matched_observations
                    .iter()
                    .any(|s| actions[j].matched_observations.contains(s))
            {
                return Err(ambiguity(
                    "Several configured capabilities claim the same observed request",
                    [actions[i].capability.clone(), actions[j].capability.clone()],
                ));
            }
        }
    }
    *context = if entities.len() == 1 {
        entities.into_iter().next()
    } else {
        None
    };
    Ok(actions)
}
/// Give each action its typed arguments through the parse. Every placeholder of this sentence must
/// reach exactly one action's request head through argument relations (`slots::bound_head`), and
/// that action's capability must have a slot of the argument's kind; slot counts must then fit. An
/// argument (quoted text, identifier, URL or host) that a colon introduces after the request is
/// bound by `slots::colon_head` when the parse alone does not bind it.
fn bind(
    w: &[Word],
    mut actions: Vec<Action>,
    capabilities: &BTreeMap<String, Capability>,
    arguments: &[slots::Argument],
    bound: &mut BTreeSet<usize>,
) -> std::result::Result<Vec<Action>, Failure> {
    for (ai, arg) in arguments.iter().enumerate() {
        let nodes: Vec<usize> = (0..w.len())
            .filter(|&i| w[i].form.eq_ignore_ascii_case(&arg.placeholder))
            .collect();
        let node = match nodes.as_slice() {
            [] => continue, // another sentence's argument
            [n] => *n,
            _ => return Err(failure("argument_ambiguous", "An argument placeholder occurs twice")),
        };
        let heads: Vec<usize> = actions.iter().flat_map(|a| a.native_request_heads.iter().copied()).collect();
        // The parse decides first; an argument that a colon hands to the preceding request is that
        // request's argument whatever relation the parser gave it (`slots::colon_head`).
        let colon = || slots::colon_head(w, node, &heads);
        let head = slots::bound_head(w, node, &heads).or_else(colon).ok_or_else(|| {
            failure(
                "argument_unbound",
                format!("{} is not an argument of a requested action in the parse", arg.value),
            )
        })?;
        let takers: Vec<usize> = (0..actions.len())
            .filter(|&i| {
                actions[i].native_request_heads.contains(&head)
                    && capabilities[&actions[i].capability].slots.iter().any(|s| s.kinds.contains(&arg.kind))
            })
            .collect();
        let [taker] = takers.as_slice() else {
            return Err(failure(
                if takers.is_empty() { "argument_unbound" } else { "argument_ambiguous" },
                format!("{} has {} candidate actions in the parse", arg.value, takers.len()),
            ));
        };
        let rule = capabilities[&actions[*taker].capability]
            .slots
            .iter()
            .find(|s| s.kinds.contains(&arg.kind))
            .expect("taker has a slot of this kind");
        actions[*taker].arguments.entry(rule.name.clone()).or_default().push(arg.value.clone());
        actions[*taker].native_argument_nodes.push(node);
        bound.insert(ai);
    }
    for a in &actions {
        for rule in &capabilities[&a.capability].slots {
            let n = a.arguments.get(&rule.name).map_or(0, Vec::len);
            if n < rule.min || n > rule.max {
                let kinds: Vec<&str> = rule
                    .kinds
                    .iter()
                    .map(|k| match k {
                        SlotKind::Url => "URL",
                        SlotKind::Host => "host",
                        SlotKind::Text => "quoted text",
                        SlotKind::Id => "identifier",
                    })
                    .collect();
                return Err(failure(
                    "argument_count",
                    format!(
                        "{} needs {}..{} {} as {}; the parse gives {n}",
                        a.capability,
                        rule.min,
                        rule.max,
                        kinds.join(" or "),
                        rule.name
                    ),
                ));
            }
        }
    }
    Ok(actions)
}
fn bounded_file(p: &Path, max: u64) -> Result<Vec<u8>> {
    let f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(p)?;
    ensure!(
        f.metadata()?.is_file() && f.metadata()?.len() <= max,
        "bounded regular file required"
    );
    let mut b = vec![];
    f.take(max + 1).read_to_end(&mut b)?;
    ensure!(b.len() as u64 <= max, "file grew beyond budget");
    Ok(b)
}
/// The deduplication key of a review candidate: the prompt and the stage at which it failed.
pub fn dedupe_key(prompt_sha256: &str, stage: &str) -> String {
    digest(&serde_json::to_vec(&json!(["review-candidate-v1", prompt_sha256, stage])).unwrap())
}
/// Write through a private temporary file and rename over the target.
fn replace_file(p: &Path, b: &[u8]) -> Result<()> {
    let tmp = p.with_extension(format!("tmp-{}", std::process::id()));
    write_new(&tmp, b)?;
    fs::rename(&tmp, p)?;
    Ok(())
}
fn write_new(p: &Path, b: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(p)?;
    f.write_all(b)?;
    f.sync_all()?;
    Ok(())
}
pub fn diagnose(path: &Path) -> Result<Value> {
    let mut f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)?;
    let m = f.metadata()?;
    ensure!(m.is_file(), "trusted log must be a regular file");
    let start = m.len().saturating_sub(LOG_MAX);
    f.seek(SeekFrom::Start(start))?;
    let mut bytes = vec![];
    // Observe at most the length seen on this descriptor, even if the file grows.
    f.take(m.len() - start).read_to_end(&mut bytes)?;
    // The first partial line is outside this observation. Drop it before decoding:
    // the byte window may begin inside a valid UTF-8 code point.
    let skipped = if start == 0 {
        0
    } else {
        bytes
            .iter()
            .position(|b| *b == b'\n')
            .map_or(bytes.len(), |i| i + 1)
    };
    let text =
        std::str::from_utf8(&bytes[skipped..]).context("admitted log lines must be UTF-8")?;
    let mut offset = start + skipped as u64;
    let mut signals = vec![];
    for line in text.split_inclusive('\n') {
        if signals.len() < 32 {
            let lower = line.to_lowercase();
            let markers: Vec<_> = ["error", "failed", "panic", "timeout", "refused", "invalid"]
                .into_iter()
                .filter(|m| lower.contains(m))
                .collect();
            if !markers.is_empty() {
                signals.push(json!({"byte_start":offset,"byte_end":offset+line.len() as u64,"markers":markers,"text":line.chars().take(1024).collect::<String>(),"interpretation":"lexical error candidate; causation and current health unverified"}));
            }
        }
        offset += line.len() as u64;
    }
    Ok(
        json!({"tail_sha256":digest(&bytes),"file_size_observed":m.len(),"byte_start":start,"bytes_read":bytes.len(),"skipped_prefix_bytes":skipped,"decoded_byte_start":start+skipped as u64,"signal_limit":32,"signals":signals,"scope":"bounded lexical diagnosis; not full history or semantic error proof","causal_findings":[],"healthy":null}),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_is_not_admitted() {
        assert!(!syntax_valid(&[]));
    }
    #[test]
    fn bounded_log_does_not_assert_health() {
        let p = std::env::temp_dir().join(format!("octopus-log-{}", std::process::id()));
        fs::write(&p, "started\nerror: worker refused request\nrecovered\n").unwrap();
        let v = diagnose(&p).unwrap();
        assert_eq!(v["signals"].as_array().unwrap().len(), 1);
        assert_eq!(v["healthy"], Value::Null);
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn log_tail_drops_partial_line_before_utf8_validation() {
        let p = std::env::temp_dir().join(format!("octopus-utf8-tail-{}", std::process::id()));
        let last = "error: retained\n";
        let prefix = format!("α{}\n", "x".repeat(LOG_MAX as usize - last.len() - 2));
        let log = format!("{prefix}{last}");
        assert_eq!(log.len(), LOG_MAX as usize + 1);
        assert!(std::str::from_utf8(&log.as_bytes()[1..]).is_err());
        fs::write(&p, &log).unwrap();
        let v = diagnose(&p).unwrap();
        assert_eq!(v["bytes_read"], LOG_MAX);
        assert_eq!(v["tail_sha256"], digest(&log.as_bytes()[1..]));
        assert_eq!(v["decoded_byte_start"], prefix.len());
        assert_eq!(v["signals"][0]["byte_start"], prefix.len());
        assert_eq!(v["signals"][0]["text"], last);
        assert_eq!(v["healthy"], Value::Null);
        // A long single line has no admitted complete suffix, even with a valid full log.
        fs::write(&p, format!("α{}", "x".repeat(LOG_MAX as usize - 1))).unwrap();
        let v = diagnose(&p).unwrap();
        assert!(v["signals"].as_array().unwrap().is_empty());
        assert_eq!(v["skipped_prefix_bytes"], LOG_MAX);
        // Invalid bytes in an admitted line are never repaired by lossy decoding.
        fs::write(&p, b"error: invalid \xff\n").unwrap();
        assert!(diagnose(&p).is_err());
        let mut malformed = vec![b'x'; LOG_MAX as usize + 1];
        let n = malformed.len();
        malformed[n - 4..].copy_from_slice(b"\n\xffx\n");
        fs::write(&p, malformed).unwrap();
        assert!(diagnose(&p).is_err());
        fs::remove_file(p).unwrap();
    }
    #[test]
    fn crab_flow_admits_only_status_and_attention() {
        let seen = std::cell::RefCell::new(vec![]);
        let cb = |q: &Value| -> std::result::Result<Value, String> {
            seen.borrow_mut().push(q.clone());
            Ok(json!({"ok":true}))
        };
        let h = CrabFlow(&cb);
        let a = Action { capability: "x".into(), matched_entities: vec![], matched_observations: vec![], native_request_heads: vec![],
            native_observation_nodes: vec![], arguments: BTreeMap::new(), native_argument_nodes: vec![] };
        h.execute(&json!({"operation":"status","group":"fish"}), &a, "i1").unwrap();
        h.execute(&json!({"operation":"status"}), &a, "i2").unwrap();
        h.execute(&json!({"operation":"attention"}), &a, "i3").unwrap();
        assert_eq!(seen.borrow()[0], json!({"schema":1,"id":"i1","op":"flow_status","group":"fish"}));
        assert_eq!(seen.borrow()[2]["op"], "attention");
        for bad in [json!({"operation":"pause"}), json!({"operation":"attention","group":"fish"}), json!({"operation":"status","group":"../x"}),
            json!({"operation":"status","worker":"eel"}), json!({"operation":"status","group":3})] {
            assert!(h.execute(&bad, &a, "b").is_err(), "{bad}");
        }
        assert_eq!(seen.borrow().len(), 3, "a refused parameter set never reaches Crab");
    }
    #[test]
    fn crab_control_admits_only_configured_worker_operations() {
        let seen = std::cell::RefCell::new(vec![]);
        let cb = |q: &Value| -> std::result::Result<Value, String> {
            seen.borrow_mut().push(q.clone());
            Ok(json!({"ok":true}))
        };
        let h = CrabControl(&cb);
        let a = Action {
            capability: "x".into(),
            matched_entities: vec![],
            matched_observations: vec![],
            native_request_heads: vec![],
            native_observation_nodes: vec![],
            arguments: BTreeMap::new(),
            native_argument_nodes: vec![],
        };
        let ok = h
            .execute(&json!({"operation":"worker_restart","worker":"eel"}), &a, "i1")
            .unwrap();
        assert_eq!(ok["request"]["op"], "worker_restart");
        assert_eq!(ok["request"]["worker"], "eel");
        assert!(h.execute(&json!({"operation":"worker_status"}), &a, "i2").is_ok());
        for bad in [
            json!({"operation":"consult_opus","worker":"eel"}),
            json!({"operation":"stop","worker":"eel"}),
            json!({"operation":"start_whales"}),
            json!({"operation":"worker_start"}),
            json!({"operation":"worker_start","worker":"../eel"}),
            json!({"operation":"worker_start","worker":"eel","argv":["/bin/sh"]}),
        ] {
            assert!(h.execute(&bad, &a, "i3").is_err(), "{bad}");
        }
        assert_eq!(seen.borrow().len(), 2, "refused parameters never reach Crab");
    }
    #[test]
    fn aliases_cannot_be_paths() {
        assert!(!id("../../etc/passwd"));
        assert!(!id(""));
    }
}

#[cfg(test)]
mod native_controls {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::os::unix::net::UnixListener;
    fn test_capabilities(root: &Path, log: PathBuf) -> BTreeMap<String, Capability> {
        let pred = vec![
            "ask", "check", "inspect", "review", "read", "show", "look", "get", "tell", "see",
            "diagnose", "find", "know", "report", "take", "have",
        ];
        let mut out = BTreeMap::new();
        // Worker control vocabulary is configuration, not code: one capability per operation.
        for (name, operation, preds, obs) in [
            ("eel_start", "worker_start", vec!["start", "launch"], vec!["start", "launch"]),
            ("eel_restart", "worker_restart", vec!["restart"], vec!["restart"]),
            ("eel_stop", "worker_stop", vec!["stop"], vec!["stop"]),
            ("eel_status", "worker_status", pred.clone(), vec!["status", "run", "work"]),
            ("eel_stop_now", "worker_stop", vec!["stop"], vec!["stop", "now", "immediately"]),
        ] {
            let mut parameters = json!({"operation":operation,"worker":"eel"});
            if name == "eel_stop_now" {
                parameters["immediate"] = json!(true);
            }
            out.insert(
                name.into(),
                Capability {
                    slots: vec![],
                    handler: "crab_control".into(),
                    parameters,
                    intent: IntentRule {
                        request_predicates: preds.iter().map(|s| s.to_string()).collect(),
                        observations: obs.into_iter().map(str::to_string).collect(),
                        entities: vec!["eel".into()],
                        subordinate_predicates: vec!["run".into(), "work".into()],
                        addressee: false,
                        native_lemma_observations: vec![],
                        collective: false,
                        observes: operation == "worker_status",
                    },
                },
            );
        }
        for (name, handler, parameters, entity, obs) in [
            (
                "control",
                "crab_observe",
                json!({"socket":root.join("control.sock"),"operation":"status"}),
                "crab",
                vec![
                    "status", "health", "normal", "normally", "run", "work", "doing",
                ],
            ),
            (
                "reports",
                "crab_observe",
                json!({"socket":root.join("control.sock"),"operation":"read_reports"}),
                "crab",
                vec!["report"],
            ),
            (
                "diagnose",
                "log_tail",
                json!({"path":log}),
                "dolphin",
                vec!["log"],
            ),
        ] {
            out.insert(
                name.into(),
                Capability {
                    slots: vec![],
                    handler: handler.into(),
                    parameters,
                    intent: IntentRule {
                        request_predicates: pred.iter().map(|s| s.to_string()).collect(),
                        observations: obs.into_iter().map(str::to_string).collect(),
                        entities: vec![entity.into()],
                        subordinate_predicates: vec!["run".into(), "work".into()],
                        addressee: false,
                        native_lemma_observations: vec![],
                        collective: false,
                        observes: true,
                    },
                },
            );
        }
        out
    }
    /// A pod of four named Dolphins: every name reaches its own Dolphin, only the quoted text is
    /// handed over, a bare "Dolphin" is refused as ambiguous, the plural lists the pod.
    #[test]
    fn real_native_question_frames_dedupe_and_execution_failures() {
        let Some(path) = std::env::var_os("MOVA_OCTOPUS_TEST_MODEL") else {
            eprintln!("SKIPPED native question frames: MOVA_OCTOPUS_TEST_MODEL not configured");
            return;
        };
        let root = std::env::temp_dir().join(format!("octopus-frames-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let model = PathBuf::from(path);
        let mut capabilities = test_capabilities(&root, root.join("dolphin.log"));
        capabilities.get_mut("eel_status").unwrap().intent.observations.push("busy".into());
        for n in ["documenter", "opus"] {
            capabilities.insert(format!("{n}_status"), Capability {
                slots: vec![],
                handler: "crab_control".into(),
                parameters: json!({"operation":"worker_status","worker":format!("dolphin-{n}")}),
                intent: IntentRule {
                    request_predicates: vec!["check".into(), "ask".into()],
                    observations: vec!["status".into(), "run".into(), "busy".into()],
                    entities: vec![n.into()],
                    subordinate_predicates: vec!["run".into()],
                    addressee: false,
                    native_lemma_observations: vec![],
                    collective: false,
                    observes: true,
                },
            });
        }
        let config = Config {
            model_sha256: digest(&fs::read(&model).unwrap()),
            model,
            capabilities,
            ledger: root.join("ledger"),
            followup_queue: root.join("review"),
            slot_policy: SlotPolicy::default(),
            group_aliases: BTreeMap::from([("dolphin".to_string(), vec!["documenter".to_string(), "opus".to_string()])]),
        };
        let d = Deputy::load(config.clone()).unwrap();
        let plan = |p: &str| {
            let r = d.review(p);
            (r.plan.iter().map(|a| a.capability.clone()).collect::<Vec<_>>(), r.failure, r.observations)
        };
        let mut repaired = 0;
        // Real refused or misread prompts of 2026-11-04 (live Crab, en.bin 0.4).
        for (p, cap, frame) in [
            ("Is Crab running normally?", "control", "polar | subj «crab» | verb run"),
            ("Is the documenter running?", "documenter_status", "polar repaired | subj «the documenter» | verb run"),
            ("Is Documenter running?", "documenter_status", "polar | subj «documenter» | verb run"),
            ("Is Eel busy?", "eel_status", "polar repaired | subj «eel» | verb busy"),
            ("Is the Eel running?", "eel_status", "polar repaired | subj «the eel» | verb running"),
            ("Is Opus Dolphin busy?", "opus_status", "polar repaired | subj «opus dolphin» | verb busy"),
            ("Could you start Eel?", "eel_start", "modal_request | subj «you» | verb start"),
            ("Could you restart Eel, please?", "eel_restart", "modal_request | subj «you» | verb restart"),
            // 2026-11-05 (en.bin 2483516a and the model before it): one noun phrase, «show» a noun.
            ("Show Crab's recent reports.", "reports", "imperative repaired | subj «» | verb show"),
            ("Show Crab's reports.", "reports", "imperative repaired | subj «» | verb show"),
        ] {
            let (caps, f, obs) = plan(p);
            assert_eq!(caps, [cap], "{p}: {f:?}");
            // Whether a frame needed its repair depends on the model's parse (en.bin 2483516a reads
            // «Is Crab running normally?» with «crab» as the root); the roles must not.
            let roles = |t: &str| t.replacen(" repaired", "", 1);
            assert_eq!(roles(&obs[0].deputy_frame.text), roles(frame), "{p}");
            repaired += usize::from(obs[0].deputy_frame.repaired);
        }
        assert!(repaired >= 2, "the repairs are exercised: {repaired}");
        // Controls: past questions, statements and a bare group name stay refused.
        assert!(plan("Was Crab running normally?").1.is_some());
        assert_eq!(plan("Eel is running.").1.unwrap().stage, "pragmatic_ambiguity");
        let f = plan("Is the Dolphin running?").1.unwrap();
        assert_eq!((f.stage.as_str(), f.candidates.clone()), ("pragmatic_ambiguity", vec!["documenter".to_string(), "opus".to_string()]));
        assert!(f.reason.contains("candidates: documenter, opus"), "{}", f.reason);
        // A question about an action never runs it: only observing capabilities answer questions
        // (`IntentRule::observes`). Each of these planned the action before 2026-11-05.
        // A question mark makes a question of any parse ("Start Eel?", a bare verb).
        for p in ["Is Eel stopped?", "Is Eel started?", "Has Eel stopped?", "Did you stop Eel?", "Can Eel restart?", "How do I stop Eel?", "Start Eel?"] {
            let (caps, f, _) = plan(p);
            assert!(caps.is_empty(), "{p}: {caps:?}");
            assert_eq!(f.map(|f| f.stage), Some("pragmatic_ambiguity".to_string()), "{p}");
        }
        // The same verbs as requests still act.
        for (p, cap) in [("Stop Eel.", "eel_stop"), ("Could you stop Eel?", "eel_stop")] {
            assert_eq!(plan(p).0, [cap], "{p}");
        }
        // A capability whose operation acts cannot be declared observing.
        let mut lying = config.clone();
        lying.capabilities.get_mut("eel_stop").unwrap().intent.observes = true;
        assert!(Deputy::load(lying).is_err());
        // One review file per prompt and failure stage; every reproducing build is listed once.
        d.run("Kill Eel.", "dedupe-1", false).unwrap();
        d.run("Kill Eel.", "dedupe-2", false).unwrap();
        let mut other = config.clone();
        other.capabilities.get_mut("eel_status").unwrap().intent.observations.push("idle".into());
        Deputy::load(other).unwrap().run("Kill Eel.", "dedupe-3", false).unwrap();
        let files: Vec<_> = fs::read_dir(root.join("review")).unwrap().map(|e| e.unwrap().path()).collect();
        assert_eq!(files.len(), 1, "{files:?}");
        let v: Value = serde_json::from_slice(&fs::read(&files[0]).unwrap()).unwrap();
        let builds = v["reproduced_by"].as_array().unwrap();
        assert_eq!(builds.len(), 2);
        assert_eq!((builds[0]["seen"].as_u64(), builds[1]["seen"].as_u64()), (Some(2), Some(1)));
        // An understood plan whose execution fails is its own reading, not a misunderstanding.
        let crab = |_: &Value| -> std::result::Result<Value, String> { Err("worker is in unadopted_conflict".into()) };
        let control = CrabControl(&crab);
        let handlers: BTreeMap<String, &dyn CapabilityHandler> =
            BTreeMap::from([("crab_control".to_owned(), &control as &dyn CapabilityHandler)]);
        let r = d.run_with("Start Eel.", "exec-fail-1", true, &handlers).unwrap();
        assert_eq!(r.failure.as_ref().unwrap().stage, "execution_failure");
        assert!(r.followups.is_empty());
        assert_eq!(
            commands::reading(&r),
            commands::Reading::ExecutedFailed { capabilities: vec!["eel_start".into()] }
        );
        let _ = fs::remove_dir_all(&root);
    }
    #[test]
    fn real_native_dolphin_pod_routes_by_name() {
        let Some(path) = std::env::var_os("MOVA_OCTOPUS_TEST_MODEL") else {
            eprintln!("SKIPPED native Dolphin pod: MOVA_OCTOPUS_TEST_MODEL not configured");
            return;
        };
        let root = std::env::temp_dir().join(format!("octopus-dolphins-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let model = PathBuf::from(path);
        let names = ["opus", "copilot", "documenter", "sol"];
        let mut capabilities = BTreeMap::new();
        let intent = |preds: &[&str], obs: &[&str], ent: &str, collective| IntentRule {
            request_predicates: preds.iter().map(|s| s.to_string()).collect(),
            observations: obs.iter().map(|s| s.to_string()).collect(),
            entities: vec![ent.into()],
            subordinate_predicates: vec!["run".into()],
            addressee: false,
            native_lemma_observations: vec![],
            collective,
            observes: false,
        };
        let ask = ["ask", "tell", "send", "give", "consult"];
        let look = ["ask", "check", "show", "read", "tell", "see", "know"];
        for n in names {
            capabilities.insert(format!("{n}-ask"), Capability {
                handler: "crab_consult".into(),
                parameters: json!({"operation":"enqueue","dolphin":n}),
                intent: intent(&ask, &ask, n, false),
                slots: vec![SlotRule { name: "task".into(), kinds: vec![SlotKind::Text], min: 1, max: 1 }],
            });
            capabilities.insert(format!("{n}-start"), Capability {
                handler: "crab_control".into(),
                parameters: json!({"operation":"worker_start","worker":format!("dolphin-{n}")}),
                intent: intent(&["start", "launch"], &["start", "launch"], n, false),
                slots: vec![],
            });
            capabilities.insert(format!("{n}-busy"), Capability {
                handler: "crab_consult".into(),
                parameters: json!({"operation":"state","dolphin":n}),
                intent: IntentRule { observes: true, ..intent(&look, &["busy", "idle"], n, false) },
                slots: vec![],
            });
        }
        capabilities.insert("dolphins-list".into(), Capability {
            handler: "crab_consult".into(),
            parameters: json!({"operation":"list"}),
            intent: IntentRule { observes: true, ..intent(&["list", "show", "ask", "check"], &["list", "run", "dolphin"], "dolphin", true) },
            slots: vec![],
        });
        let config = Config {
            model_sha256: digest(&fs::read(&model).unwrap()),
            model,
            capabilities,
            ledger: root.join("ledger"),
            followup_queue: root.join("review"),
            slot_policy: SlotPolicy::default(),
            group_aliases: BTreeMap::from([("dolphin".to_string(), names.iter().map(|s| s.to_string()).collect())]),
        };
        let d = Deputy::load(config.clone()).unwrap();
        let sent = std::cell::RefCell::new(Vec::<Value>::new());
        let crab = |q: &Value| -> std::result::Result<Value, String> {
            sent.borrow_mut().push(q.clone());
            Ok(json!({"ok":true}))
        };
        let consult = CrabConsult(&crab);
        let control = CrabControl(&crab);
        let handlers: BTreeMap<String, &dyn CapabilityHandler> = BTreeMap::from([
            ("crab_consult".to_owned(), &consult as &dyn CapabilityHandler),
            ("crab_control".to_owned(), &control as &dyn CapabilityHandler),
        ]);
        let mut k = 0;
        let mut run = |p: &str| {
            k += 1;
            sent.borrow_mut().clear();
            let id = format!("pod-{k}");
            let r = d.run_with(p, &id, true, &handlers).unwrap();
            let q = sent.borrow().clone();
            (r, q, id)
        };
        for n in names {
            let title = format!("{}{}", n[..1].to_uppercase(), &n[1..]);
            let (r, q, id) = run(&format!("Ask the {title} Dolphin: «Task for {n}; do not stop Eel.» Thanks."));
            assert!(r.failure.is_none(), "{n}: {:?}", r.failure);
            assert_eq!(q.len(), 1, "{n}");
            assert!(q[0]["id"].as_str().unwrap().contains(&id), "{n}");
            assert_eq!((&q[0]["op"], &q[0]["dolphin"]), (&json!("consult_enqueue"), &json!(n)), "{n}");
            assert_eq!(q[0]["task"], format!("Task for {n}; do not stop Eel."), "only the quoted text: {n}");
            let (r, q, _) = run(&format!("Start the {title} Dolphin."));
            assert!(r.failure.is_none(), "{n}: {:?}", r.failure);
            assert_eq!(q[0]["worker"], format!("dolphin-{n}"));
            // The quotation closes the sentence after a colon; the 2026-11-05 en.bin (and the one
            // before it) attach it as a vocative, which the colon rule binds to the ask.
            for p in [format!("Ask the {title} Dolphin: «Check this diff.»"), format!("Ask the {title} Dolphin: \"Check this diff.\"")] {
                let (r, q, _) = run(&p);
                assert!(r.failure.is_none(), "{p}: {:?}", r.failure);
                assert_eq!((&q[0]["dolphin"], &q[0]["task"]), (&json!(n), &json!("Check this diff.")), "{p}");
            }
        }
        // Negative controls: a quotation that is not handed over by a colon after the request is
        // never sent.
        for p in ["«Check this diff.» Ask the Opus Dolphin.", "Ask the Opus Dolphin. Note: «Check this diff.»", "Note: «Check this diff.» Ask the Opus Dolphin."] {
            let (r, q, _) = run(p);
            assert!(r.failure.is_some() && q.is_empty(), "{p}: {:?}", r.failure);
        }
        for p in ["Ask the Dolphin: «Review the plan.»", "Start the Dolphin.", "Is the Dolphin busy?"] {
            let (r, q, _) = run(p);
            assert_eq!(r.failure.as_ref().map(|f| f.stage.as_str()), Some("pragmatic_ambiguity"), "{p}");
            assert!(q.is_empty(), "a bare Dolphin reaches no Dolphin: {p}");
            assert_eq!(r.followups[0]["status"], "review_only");
        }
        let (r, q, _) = run("Which Dolphins are running?");
        assert!(r.failure.is_none(), "{:?}", r.failure);
        assert_eq!(q[0]["op"], "dolphin_list");
        // The quoted text must be bound by the parse to the ask; outside text is never sent.
        let (r, q, _) = run("Ask the Opus Dolphin to stop Eel.");
        assert!(r.failure.is_some() && q.is_empty());
        // Configuration checks: enqueue needs its text slot and a Dolphin name; list takes none.
        for bad in [json!({"operation":"enqueue"}), json!({"operation":"list","dolphin":"opus"}), json!({"operation":"forget","dolphin":"opus"})] {
            assert!(consult_operation(&bad, &[SlotRule { name: "task".into(), kinds: vec![SlotKind::Text], min: 1, max: 1 }]).is_err(), "{bad}");
        }
        assert!(consult_operation(&json!({"operation":"enqueue","dolphin":"opus"}), &[]).is_err());
        let mut bad = config.clone();
        bad.group_aliases.insert("whale".into(), vec!["moby".into(), "opus".into()]);
        assert!(bad.validate().is_err(), "group members must be configured aliases");
        let _ = fs::remove_dir_all(&root);
    }
    #[test]
    fn real_native_plaintext_and_protocol_controls() {
        let Some(path) = std::env::var_os("MOVA_OCTOPUS_TEST_MODEL") else {
            eprintln!("SKIPPED native controls: MOVA_OCTOPUS_TEST_MODEL not configured");
            return;
        };
        let root = std::env::temp_dir().join(format!("octopus-native-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let model = PathBuf::from(path);
        let log = root.join("worker.log");
        fs::write(&log, "started\nerror: prior worker stopped\nrecovered\n").unwrap();
        let d = Deputy::load(Config {
            model_sha256: digest(&fs::read(&model).unwrap()),
            model,
            capabilities: test_capabilities(&root, log),
            ledger: root.join("ledger"),
            followup_queue: root.join("review"),
            slot_policy: SlotPolicy::default(),
            group_aliases: BTreeMap::new(),
        })
        .unwrap();
        let positives = [
            ("Ask Crab for its status", 1),
            ("What is Crab's current status?", 1),
            ("Have a look at Dolphin's logs.", 1),
            ("Please check Crab's status.", 1),
            (
                "Could you ask Crab whether everything is running normally?",
                1,
            ),
            ("Read Dolphin's recent logs.", 1),
            (
                "Have a look at Dolphin’s recent logs and ask Crab whether everything is running normally.",
                2,
            ),
            (
                "Please inspect Dolphin's logs. Then ask Crab for its status.",
                2,
            ),
            ("Read Dolphin's logs. Check its logs.", 2),
            ("Show Crab's recent reports.", 1),
            ("Is Crab running normally?", 1),
            ("Is Crab working?", 1),
            ("Is Crab running normally", 1),
            ("Start Eel.", 1),
            ("Restart Eel, please.", 1),
            ("Is Eel running?", 1),
            ("Stop Eel.", 1),
            ("Please launch Eel.", 1),
            ("Check Eel's status.", 1),
            ("Stop Eel now.", 1),
            ("Stop Eel immediately!", 1),
        ];
        let mut observations = vec![];
        for (p, n) in positives {
            let r = d.review(p);
            observations.push(serde_json::to_value(&r).unwrap());
            assert!(r.failure.is_none(), "{p}: {:?}", r.failure);
            assert_eq!(r.plan.len(), n, "{p}");
            assert!(!r.observations[0].words.is_empty());
        }
        for p in [
            "Delete Dolphin's logs and ask Crab for its status.",
            "Do not ask Crab for its status.",
            "Read its logs.",
            "Ask someone whether things are fine.",
            "Train the parser on those logs.",
            "If Crab is working, inspect Dolphin's logs.",
            "She checks Crab's status.",
            "He said ask Crab for its status.",
            "Crab is running normally.",
            "Is Crab not running?",
            "Was Crab running normally?",
            "Do not stop Eel.",
            "If Eel is running, restart it.",
            "Delete Eel.",
            "Eel stopped.",
            "Stop Whale.",
            "Kill Eel.",
        ] {
            let r = d.review(p);
            observations.push(serde_json::to_value(&r).unwrap());
            assert!(r.failure.is_some(), "negative accepted: {p}");
            assert_eq!(r.results.len(), 0);
            assert_eq!(r.followups[0]["priority"], "operator_highest");
            assert_eq!(r.followups[0]["training_authorized"], false);
        }
        let listener = UnixListener::bind(root.join("control.sock")).unwrap();
        let h = std::thread::spawn(move || {
            let (mut s, _) = listener.accept().unwrap();
            let mut line = String::new();
            BufReader::new(s.try_clone().unwrap())
                .read_line(&mut line)
                .unwrap();
            let q: Value = serde_json::from_str(&line).unwrap();
            assert_eq!(q["op"], "status");
            writeln!(
                s,
                "{}",
                json!({"schema":1,"id":q["id"],"ok":true,"data":{"observed":"fixture"}})
            )
            .unwrap();
        });
        let first = d.run("Ask Crab for its status", "once", true).unwrap();
        h.join().unwrap();
        assert_eq!(first.results.len(), 1);
        let cached = d.run("Ask Crab for its status", "once", true).unwrap();
        assert_eq!(
            serde_json::to_value(&first).unwrap(),
            serde_json::to_value(&cached).unwrap()
        );
        assert!(d.run("Read Dolphin's logs", "once", true).is_err());
        fs::remove_file(d.config.ledger.join("once/receipt.json")).unwrap();
        assert!(
            d.run("Ask Crab for its status", "once", true)
                .unwrap_err()
                .to_string()
                .contains("no replay")
        );
        let refused = d.run("Delete Dolphin's logs", "refused", true).unwrap();
        assert!(refused.failure.is_some());
        assert!(!d.config.ledger.join("refused").exists());
        // English control prompts reach Crab's worker operations in-process.
        let sent = std::cell::RefCell::new(vec![]);
        let immediate = std::cell::RefCell::new(vec![]);
        let cb = |q: &Value| -> std::result::Result<Value, String> {
            sent.borrow_mut().push((q["op"].clone(), q["worker"].clone()));
            immediate.borrow_mut().push(q["immediate"].clone());
            Ok(json!({"schema":1,"id":q["id"],"ok":true,"data":{"fixture":true}}))
        };
        let control = CrabControl(&cb);
        let overrides: BTreeMap<String, &dyn CapabilityHandler> =
            BTreeMap::from([("crab_control".to_owned(), &control as &dyn CapabilityHandler)]);
        for (i, (p, op)) in [
            ("Start Eel.", "worker_start"),
            ("Restart Eel, please.", "worker_restart"),
            ("Is Eel running?", "worker_status"),
            ("Stop Eel.", "worker_stop"),
            ("Stop Eel now.", "worker_stop"),
        ]
        .into_iter()
        .enumerate()
        {
            let r = d.run_with(p, &format!("ctl{i}"), true, &overrides).unwrap();
            assert!(r.failure.is_none(), "{p}: {:?}", r.failure);
            assert_eq!(r.results.len(), 1, "{p}");
            assert_eq!(sent.borrow().last().unwrap(), &(json!(op), json!("eel")), "{p}");
        }
        let refused = d.run_with("Do not stop Eel.", "ctl-neg", true, &overrides).unwrap();
        assert!(refused.failure.is_some());
        assert_eq!(sent.borrow().len(), 5, "a refused prompt never reaches Crab");
        // The most specific reading: "Stop Eel." waits for the safe point, "Stop Eel now." interrupts.
        assert_eq!(immediate.borrow()[3], Value::Null);
        assert_eq!(immediate.borrow()[4], json!(true));
        // Without the in-process override the control handler is absent: execution fails closed.
        let r = d.run("Stop Eel.", "ctl-absent", true).unwrap();
        assert_eq!(r.failure.as_ref().unwrap().stage, "execution_failure");
        if let Some(dest) = std::env::var_os("MOVA_OCTOPUS_TEST_OBSERVATIONS") {
            fs::write(dest, serde_json::to_vec_pretty(&observations).unwrap()).unwrap();
        }
        fs::remove_dir_all(root).unwrap();
    }
}
