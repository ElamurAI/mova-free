//! One Whale run: pin an explicit task queue, call the teacher at most once per task, gate the
//! reply through the configured contract and publish immutable versioned bundles with receipts.
use crate::{
    contract,
    originality,
    source::{self, Adapter},
    spec::{self, MetadataSource, Spec},
    util::{error, file_hash, hash, lease, load, now, normal, save, storage_available, storage_floor, valid_hash, write_sync},
    Result,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

pub const RUN_SCHEMA: &str = "mova-shark-run-v1";
const JSON_MAX: usize = 2 << 20;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Item {
    pub kind: String,
    pub id: String,
    pub path: PathBuf,
    pub sha256: String,
    pub format: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub binding: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub licence: String,
    #[serde(default)]
    pub local_only: bool,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub metadata: Value,
}
/// Bounded teacher transport. The production implementation is the shared Codex transport.
pub trait Teacher {
    fn ask(&self, dir: &Path, model: &str, effort: &str, prompt: &str) -> Result<Value>;
}
pub struct CodexTeacher;
impl Teacher for CodexTeacher {
    fn ask(&self, dir: &Path, model: &str, effort: &str, prompt: &str) -> Result<Value> {
        mova_teacher::ask(dir, model, effort, prompt).map_err(|e| error(e.to_string()))
    }
}
fn path(v: &Value, k: &str) -> Result<PathBuf> {
    crate::util::absolute(v, k)
}
/// Configured teacher identity. No model is built in.
pub fn teacher(c: &Value) -> Result<(String, String)> {
    let model = c["teacher"]["model"].as_str().ok_or("config requires teacher.model; no default model is built in")?;
    let effort = c["teacher"]["effort"].as_str().ok_or("config requires teacher.effort")?;
    mova_teacher::validate_model(model).map_err(|e| error(e.to_string()))?;
    if !matches!(effort, "low" | "medium" | "high" | "xhigh") {
        return Err(error("teacher.effort must be low, medium, high or xhigh"));
    }
    Ok((model.into(), effort.into()))
}
fn items(c: &Value, spec: &Spec) -> Result<Vec<Item>> {
    let items: Vec<Item> = serde_json::from_value(c["items"].clone())?;
    let max_items = c["max_items"].as_u64().unwrap_or(64);
    if !(1..=256).contains(&max_items) || items.is_empty() || items.len() > max_items as usize {
        return Err(error("explicit items must fit max_items (default 64, maximum 256)"));
    }
    let mut seen = BTreeSet::new();
    for i in &items {
        let kind = spec.kind(&i.kind)?;
        if i.id.is_empty() || i.id.len() > 256 || !i.path.is_absolute() || !valid_hash(&i.sha256) || !seen.insert((i.kind.clone(), i.id.clone())) {
            return Err(error("invalid/duplicate source item"));
        }
        if !kind.formats.contains_key(&i.format) {
            return Err(error("source format is not admitted for the item kind"));
        }
        if !i.metadata.is_null() && !i.metadata.is_object() {
            return Err(error("item metadata must be an object"));
        }
        match &kind.binding {
            Some(b) if !(normal(&i.binding) && i.binding.starts_with(&b.prefix) && i.binding.ends_with(&b.suffix) && i.binding.len() > b.prefix.len() + b.suffix.len()) => {
                return Err(error("item requires a normal relative binding inside the configured prefix/suffix"));
            }
            None if !i.binding.is_empty() => return Err(error("item binding is not admitted for this kind")),
            _ => {}
        }
    }
    Ok(items)
}
fn adapter(spec: &Spec, item: &Item) -> Result<Adapter> {
    Adapter::parse(&spec.kind(&item.kind)?.formats[&item.format])
}
/// Pinned JSONL metadata rows for the selected export-kind items, keyed by `id`.
fn exported(c: &Value, spec: &Spec, items: &[Item]) -> Result<BTreeMap<String, (Value, String)>> {
    let ids: BTreeSet<_> = items.iter().filter(|i| spec.kinds[&i.kind].metadata == MetadataSource::Export).map(|i| i.id.clone()).collect();
    if ids.is_empty() {
        return Ok(BTreeMap::new());
    }
    let pin = &c["metadata_export"];
    let p = PathBuf::from(pin["path"].as_str().ok_or("export-kind items require metadata_export {path, sha256}")?);
    let expected = pin["sha256"].as_str().ok_or("missing metadata_export.sha256")?;
    if !p.is_absolute() || !valid_hash(expected) {
        return Err(error("metadata_export requires an absolute path and SHA-256"));
    }
    use sha2::Digest;
    let mut h = sha2::Sha256::new();
    let mut out = BTreeMap::new();
    let mut line = Vec::new();
    let mut r = BufReader::new(File::open(p)?);
    loop {
        line.clear();
        if r.read_until(b'\n', &mut line)? == 0 {
            break;
        }
        if line.len() > 1 << 20 {
            return Err(error("oversized metadata row"));
        }
        h.update(&line);
        let v: Value = serde_json::from_slice(&line)?;
        if let Some(id) = v["id"].as_str() {
            if ids.contains(id) && out.insert(id.into(), (v.clone(), hash(&line))).is_some() {
                return Err(error("duplicate selected metadata ID"));
            }
        }
    }
    if format!("{:x}", h.finalize()) != expected {
        return Err(error("metadata export hash mismatch"));
    }
    if out.len() != ids.len() {
        return Err(error("item ID missing from pinned metadata export"));
    }
    Ok(out)
}
fn inline_metadata(item: &Item) -> Value {
    let mut m = if item.metadata.is_object() { item.metadata.clone() } else { json!({}) };
    let o = m.as_object_mut().expect("object");
    o.entry("id").or_insert(json!(item.id));
    o.entry("title").or_insert(json!(if item.title.is_empty() { &item.id } else { &item.title }));
    if !item.licence.is_empty() {
        o.entry("licence").or_insert(json!(item.licence));
    }
    m
}
/// Pin explicit local inputs and publish disjoint worker configs; no teacher call.
pub fn prepare(c: &Value) -> Result<Value> {
    let root = path(c, "run_root")?;
    let store = path(c, "store")?;
    let spec = Spec::load(c)?;
    let (model, effort) = teacher(c)?;
    fs::create_dir_all(&root)?;
    fs::create_dir_all(&store)?;
    if store.join(".git").exists() {
        return Err(error("bundle store must remain outside Git"));
    }
    let inputs = items(c, &spec)?;
    spec.originality_config(&c["originality"])?;
    let workers = c["workers"].as_u64().unwrap_or(2);
    if !(1..=8).contains(&workers) {
        return Err(error("workers must be 1..8"));
    }
    let mut base = c.clone();
    base.as_object_mut().ok_or("config must be object")?.remove("worker_index");
    let identity = hash(serde_json::to_vec(&json!({"config":base,"spec_sha256":spec.sha256,"template_sha256":spec.template_sha256,"schema":RUN_SCHEMA}))?);
    let mp = root.join("manifest.json");
    if mp.exists() {
        let m = load(&mp, 64 << 20)?;
        if m["identity_sha256"] != identity {
            return Err(error("Whale input identity changed; use a new run root"));
        }
        return Ok(m);
    }
    // Prepared workers only read the immutable manifest; they must not contend for this lease.
    let _lock = lease(&root.join("prepare.lock"))?;
    if mp.exists() {
        return Err(error("concurrent preparation completed; inspect and invoke again"));
    }
    let rows = exported(c, &spec, &inputs)?;
    fs::create_dir_all(root.join("jobs"))?;
    let mut tasks = Vec::new();
    for (index, item) in inputs.iter().enumerate() {
        if file_hash(&item.path)? != item.sha256 {
            return Err(error("source SHA mismatch"));
        }
        let kind = spec.kind(&item.kind)?;
        let ad = adapter(&spec, item)?;
        let (meta, rowsha) = if kind.metadata == MetadataSource::Export {
            let (v, s) = rows.get(&item.id).ok_or("missing metadata")?;
            if let (Some(f), Adapter::Text) = (&kind.export_path_field, ad) {
                let p = PathBuf::from(v[f].as_str().ok_or("export row lacks configured path field")?);
                if p.canonicalize()? != item.path.canonicalize()? {
                    return Err(error("item path differs from authoritative exported path"));
                }
            }
            (v.clone(), s.clone())
        } else {
            (inline_metadata(item), hash(serde_json::to_vec(item)?))
        };
        let admission = spec::admission(kind, item.local_only, &meta);
        let mut bundle_meta = spec::project(&meta, &spec.bundle.metadata_fields, usize::MAX);
        bundle_meta["id"] = json!(item.id);
        let (context, coverage) = source::extract(&item.path, ad, spec.context_max_bytes)?;
        if context.trim().is_empty() {
            return Err(error("empty source context"));
        }
        if file_hash(&item.path)? != item.sha256 {
            return Err(error("source changed during context extraction"));
        }
        let shown = json!({"id":item.id,"kind":item.kind,"source_sha256":item.sha256,"source_format":item.format,"coverage":coverage,"metadata":spec.prompt_metadata(&meta)});
        let p = spec.prompt(&shown, &context)?;
        let key = hash(serde_json::to_vec(&json!({"kind":item.kind,"id":item.id,"source":item.sha256,"metadata":rowsha,"prompt":hash(&p),"teacher":model,"effort":effort,"spec":spec.sha256}))?);
        let job = root.join("jobs").join(&key);
        fs::create_dir(&job)?;
        write_sync(&job.join("prompt.txt"), p.as_bytes())?;
        write_sync(&job.join("context.txt"), context.as_bytes())?;
        save(&job.join("source-metadata.json"), &meta)?;
        tasks.push(json!({"index":index,"worker_index":index as u64 % workers,"key":key,"item":item,"bundle_metadata":bundle_meta,"admission":admission,"metadata_row_sha256":rowsha,"context_sha256":hash(&context),"prompt_sha256":hash(&p),"prompt_bytes":p.len(),"coverage":coverage,"length_profile":spec.output.length.profile}));
    }
    let mut configs = Vec::new();
    for index in 0..workers {
        let dir = root.join("workers").join(format!("{index:02}"));
        fs::create_dir_all(&dir)?;
        let mut worker = c.clone();
        worker["worker_index"] = json!(index);
        let p = dir.join("config.json");
        save(&p, &worker)?;
        configs.push(json!({"worker_index":index,"config":p,"sha256":file_hash(&p)?}));
    }
    let manifest = json!({"schema":RUN_SCHEMA,"spec":{"id":spec.id,"path":spec.path,"sha256":spec.sha256,"template_sha256":spec.template_sha256},"length_profile":spec.output.length.profile,"identity_sha256":identity,"manifest_path":mp,"teacher":{"model":model,"effort":effort,"transport":"mova-teacher Codex CLI; no fallback"},"workers":workers,"worker_configs":configs,"tasks":tasks,"created_unix":now(),"publication":"external draft bundles; admission/local_only gates retained"});
    save(&mp, &manifest)?;
    Ok(manifest)
}
fn worker_status(root: &Path, state: &str, job: Option<&str>, detail: &str, model: &str, effort: &str) -> Result<()> {
    save(&root.join("status.json"), &json!({"state":state,"job":job,"detail":detail,"heartbeat_unix":now(),"pid":std::process::id(),"teacher_model":model,"teacher_effort":effort,"automatic_uncertain_retry":false}))
}
fn disk_ok(c: &Value) -> Result<bool> {
    let floor = storage_floor(c)?;
    Ok(storage_available(&path(c, "run_root")?, floor)? && storage_available(&path(c, "store")?, floor)?)
}
fn publish(c: &Value, spec: &Spec, task: &Value, body: &Value, reply: &Value, projection: Option<&Value>) -> Result<Value> {
    let store = path(c, "store")?;
    let key = task["key"].as_str().ok_or("missing task key")?;
    let item: Item = serde_json::from_value(task["item"].clone())?;
    // Inspect the complete raw reply before editorial length projection can remove prose.
    let raw: Value = serde_json::from_str(reply["result"].as_str().ok_or("missing raw reply result")?)?;
    let originality = if spec.originality.enabled {
        (|| -> Result<Value> {
            let primary = source::full_primary(&item.id, &item.path, &item.sha256, adapter(spec, &item)?)?;
            originality::check(&spec.originality_config(&c["originality"])?, &originality::narrative(&raw, &spec.originality.non_narrative_fields), &[primary])
        })()
        .unwrap_or_else(|e| json!({"schema":2,"decision":"pending_incomplete","publication_allowed":false,"reason":e.to_string()}))
    } else {
        json!({"schema":2,"decision":"not_required_by_specification","publication_allowed":true})
    };
    save(&path(c, "run_root")?.join("jobs").join(key).join("originality.json"), &originality)?;
    if originality["publication_allowed"] != true {
        return Err(error(format!("originality gate: {}", originality["decision"])));
    }
    let l = &spec.output.length;
    let measured = body.pointer(&l.pointer).and_then(Value::as_str).ok_or("length field")?;
    let length_measure = contract::measure(measured, l);
    let version = hash(serde_json::to_vec(&json!({"task":key,"reply_sha256":hash(serde_json::to_vec(reply)?),"length_projection":projection}))?);
    let bucket = store.join(&spec.bundle.store_prefix).join(&item.kind).join(hash(&item.id));
    fs::create_dir_all(&bucket)?;
    let final_dir = bucket.join(&version);
    if final_dir.exists() {
        return Err(error("immutable output version already exists; inspect instead of overwrite"));
    }
    let mut rendered = body.clone();
    contract::rewrite_statuses(spec, &mut rendered)?;
    let (model, effort) = teacher(c)?;
    let receipt = json!({"provider":reply["provider"],"model":reply["model"],"configured_model":model,"effort":effort,"usage":reply["usage"],"secs":reply["secs"],"cost_reported":reply["cost_reported"],"reply_sha256":hash(serde_json::to_vec(reply)?),"prompt_sha256":task["prompt_sha256"]});
    let evidence = json!({"length_projection":projection,"context":task["coverage"],"context_sha256":task["context_sha256"],"input_sha256":item.sha256,"metadata_row_sha256":task["metadata_row_sha256"],"teacher_result":"validated configured contract, length and provenance only; correctness not independently established","length_measure":length_measure,"originality":originality,"verbatim_guard":"configured context run guard; full local lexical gate reported separately; no universal originality proof","execution_evidence":"no source execution or reproduction performed by the Whale"});
    let mut files = contract::artifacts(spec, &rendered, &task["bundle_metadata"])?;
    files.push(("originality.json".into(), serde_json::to_vec_pretty(&originality)?));
    files.push(("evidence.json".into(), serde_json::to_vec_pretty(&evidence)?));
    files.push(("teacher-receipt.json".into(), serde_json::to_vec_pretty(&receipt)?));
    let stage = bucket.join(format!(".{version}.part-{}", std::process::id()));
    fs::create_dir(&stage)?;
    let mut hashes = Vec::new();
    for (name, bytes) in files {
        write_sync(&stage.join(&name), &bytes)?;
        hashes.push(json!({"path":name,"sha256":hash(&bytes)}));
    }
    let mut source = json!({"id":item.id,"kind":item.kind,"sha256":item.sha256,"format":item.format});
    if !item.binding.is_empty() {
        source["binding"] = json!(item.binding);
    }
    let mut manifest = json!({"schema":1,"id":format!("{}{}",spec.bundle.id_prefix,hash(&item.id)),"kind":spec.bundle.manifest_kind,"status":spec.bundle.manifest_status,"source":source,"admission":task["admission"],"local_only":task["admission"]["local_only"],"public_export_allowed":task["admission"]["public_export_allowed"],"coverage":task["coverage"],"length_measure":length_measure,"length_profile":l.profile,"artifacts":hashes,"teacher":receipt,"version_sha256":version,"task_sha256":key,"spec":{"id":spec.id,"sha256":spec.sha256,"template_sha256":spec.template_sha256}});
    for (k, v) in &spec.bundle.manifest_extra {
        manifest[k] = v.clone();
    }
    save(&stage.join("manifest.json"), &manifest)?;
    File::open(&stage)?.sync_all()?;
    fs::rename(&stage, &final_dir)?;
    File::open(&bucket)?.sync_all()?;
    let mp = final_dir.join("manifest.json");
    Ok(json!({"manifest":mp,"manifest_sha256":file_hash(&mp)?,"version_sha256":version,"length_measure":length_measure,"local_only":task["admission"]["local_only"]}))
}
/// Gate one parsed reply: verbatim guard on the raw body, projection, contract, publication.
fn admit(c: &Value, spec: &Spec, task: &Value, reply: &Value, context: &str, (model, _effort): (&str, &str)) -> Result<Value> {
    if reply["model"] != model || reply["provider"] != "codex" || reply["is_error"] != false {
        return Err(error("teacher identity/error contract mismatch"));
    }
    let text = reply["result"].as_str().ok_or("missing teacher result")?;
    if text.len() > spec.output.max_reply_bytes {
        return Err(error("teacher result too large"));
    }
    let mut v: Value = serde_json::from_str(text)?;
    if contract::copied_run(context, &v, spec.output.verbatim_run_tokens) {
        return Err(error("original reply source overlap"));
    }
    let projection = contract::project(&mut v, &spec.output.length)?;
    contract::validate(spec, &v, context)?;
    publish(c, spec, task, &v, reply, projection.as_ref())
}
fn marker(root: &Path, own: &Path, name: &str) -> bool {
    root.join(name).exists() || own.join(name).exists()
}
pub fn run_with(c: &Value, once: bool, teacher_impl: &impl Teacher) -> Result<()> {
    let manifest = prepare(c)?;
    let spec = Spec::load(c)?;
    let (model, effort) = teacher(c)?;
    let root = path(c, "run_root")?;
    let workers = manifest["workers"].as_u64().ok_or("workers")?;
    let index = c["worker_index"].as_u64().unwrap_or(0);
    if index >= workers {
        return Err(error("worker_index outside worker range"));
    }
    let own = root.join("workers").join(format!("{index:02}"));
    let _lease = lease(&own.join("worker.lock"))?;
    let st = |state: &str, job: Option<&str>, detail: &str| worker_status(&own, state, job, detail, &model, &effort);
    loop {
        if marker(&root, &own, "STOP") {
            st("stopped", None, "STOP requested; no new calls")?;
            return Ok(());
        }
        if marker(&root, &own, "PAUSE") {
            st("paused", None, "PAUSE requested")?;
            if once {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(5));
            continue;
        }
        if !disk_ok(c)? {
            st("paused_storage", None, "configured free-space floor; evidence retained")?;
            return Ok(());
        }
        let mut pending = None;
        for t in manifest["tasks"].as_array().ok_or("tasks")? {
            if t["worker_index"] != index {
                continue;
            }
            let key = t["key"].as_str().ok_or("task key")?;
            if !valid_hash(key) {
                return Err(error("invalid task hash"));
            }
            let job = root.join("jobs").join(key);
            let receipt = job.join("receipt.json");
            if receipt.exists() {
                let r = load(&receipt, JSON_MAX)?;
                if r["task_sha256"] != key {
                    return Err(error("receipt/task identity mismatch"));
                }
                if !matches!(r["outcome"].as_str(), Some("completed" | "quarantined" | "uncertain" | "local_only_no_remote_teacher")) {
                    return Err(error("invalid receipt outcome; inspect instead of retry"));
                }
                if r["outcome"] == "uncertain" {
                    st("paused_uncertain", Some(key), "teacher attempt uncertain; inspect instead of retry")?;
                    return Err(error("uncertain teacher reservation"));
                }
                continue;
            }
            if job.join("reservation.json").exists() || job.join("reply.json").exists() {
                st("paused_uncertain", Some(key), "unreceipted reservation/output; never automatically resubmitted")?;
                return Err(error("unresolved Whale reservation"));
            }
            pending = Some(t.clone());
            break;
        }
        let Some(task) = pending else {
            st("awaiting_queue", None, "immutable queue exhausted; not replayed")?;
            if once {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(5));
            continue;
        };
        let key = task["key"].as_str().ok_or("task key")?;
        let job = root.join("jobs").join(key);
        let item: Item = serde_json::from_value(task["item"].clone())?;
        if spec.kind(&item.kind)?.withhold_teacher_when_local_only && task["admission"]["local_only"] != false {
            save(&job.join("receipt.json"), &json!({"task_sha256":key,"outcome":"local_only_no_remote_teacher","reason":"local-only content is not admitted to the remote teacher","source_sha256":item.sha256,"admission":task["admission"],"metadata_row_sha256":task["metadata_row_sha256"],"teacher_called":false,"automatic_retry":false}))?;
            st("idle", Some(key), "local_only_no_remote_teacher")?;
            if once {
                return Ok(());
            }
            continue;
        }
        if file_hash(&item.path)? != item.sha256 {
            return Err(error("source changed since preparation; no teacher called"));
        }
        let prompt = fs::read_to_string(job.join("prompt.txt"))?;
        let context = fs::read_to_string(job.join("context.txt"))?;
        if hash(&prompt) != task["prompt_sha256"] || hash(&context) != task["context_sha256"] || prompt.len() > spec.prompt_max_bytes {
            return Err(error("prepared context/prompt changed"));
        }
        if marker(&root, &own, "STOP") || marker(&root, &own, "PAUSE") {
            continue;
        }
        let mut reservation = OpenOptions::new().create_new(true).write(true).open(job.join("reservation.json"))?;
        reservation.write_all(serde_json::to_string(&json!({"task_sha256":key,"worker_index":index,"model":model,"effort":effort,"prompt_sha256":task["prompt_sha256"],"reserved_unix":now()}))?.as_bytes())?;
        reservation.sync_all()?;
        File::open(&job)?.sync_all()?;
        st("running", Some(key), "one pinned teacher request; active call allowed to finish")?;
        let reply = match teacher_impl.ask(&job, &model, &effort, &prompt) {
            Ok(v) => v,
            Err(e) => {
                save(&job.join("receipt.json"), &json!({"task_sha256":key,"outcome":"uncertain","reason":e.to_string(),"automatic_retry":false}))?;
                st("paused_uncertain", Some(key), "teacher failed; no retry")?;
                return Err(e);
            }
        };
        save(&job.join("reply.json"), &reply)?;
        let receipt = match admit(c, &spec, &task, &reply, &context, (&model, &effort)) {
            Ok(output) => json!({"task_sha256":key,"outcome":"completed","output":output,"teacher_reply_sha256":file_hash(&job.join("reply.json"))?,"automatic_retry":false}),
            Err(e) => json!({"task_sha256":key,"outcome":"quarantined","reason":e.to_string(),"teacher_reply_sha256":file_hash(&job.join("reply.json"))?,"automatic_reprompt":false}),
        };
        save(&job.join("receipt.json"), &receipt)?;
        st("idle", Some(key), receipt["outcome"].as_str().unwrap_or("invalid"))?;
        if once {
            return Ok(());
        }
    }
}
/// Run one disjoint worker through the shared Codex teacher transport.
pub fn run(c: &Value, once: bool) -> Result<()> {
    run_with(c, once, &CodexTeacher)
}
/// Read actual local receipts/status. No teacher request, source extraction or process launch.
pub fn status(c: &Value) -> Result<Value> {
    let root = path(c, "run_root")?;
    let mp = root.join("manifest.json");
    if !mp.exists() {
        return Ok(json!({"state":"not_prepared","run_root":root}));
    }
    let m = load(&mp, 64 << 20)?;
    let mut outcomes = BTreeMap::<String, usize>::new();
    let mut outputs = Vec::new();
    for task in m["tasks"].as_array().ok_or("tasks")? {
        let key = task["key"].as_str().ok_or("task key")?;
        let job = root.join("jobs").join(key);
        if job.join("length-projection-receipt.json").exists() {
            let projected = load(&job.join("length-projection-receipt.json"), JSON_MAX)?;
            *outcomes.entry("length_projected".into()).or_default() += 1;
            if let Some(v) = projected.get("output") {
                outputs.push(v.clone());
            }
        }
        if job.join("receipt.json").exists() {
            let r = load(&job.join("receipt.json"), JSON_MAX)?;
            *outcomes.entry(r["outcome"].as_str().unwrap_or("invalid_receipt").into()).or_default() += 1;
            if let Some(v) = r.get("output") {
                outputs.push(v.clone());
            }
        } else {
            let state = if job.join("reservation.json").exists() { "reserved_uncertain" } else { "pending" };
            *outcomes.entry(state.into()).or_default() += 1;
        }
    }
    let mut workers = Vec::new();
    for i in 0..m["workers"].as_u64().ok_or("workers")? {
        let p = root.join("workers").join(format!("{i:02}/status.json"));
        workers.push(if p.exists() { load(&p, JSON_MAX)? } else { json!({"worker_index":i,"state":"not_started"}) });
    }
    Ok(json!({"schema":1,"run_root":root,"identity_sha256":m["identity_sha256"],"spec":m["spec"],"teacher":m["teacher"],"outcomes":outcomes,"workers":workers,"outputs":outputs,"proof_inferred":false,"publication_automatic":false}))
}
/// Explicit local-only editorial projection of length-gate quarantines; never calls a teacher
/// or mutates original evidence.
pub fn project_quarantined_lengths(c: &Value) -> Result<Value> {
    let m = prepare(c)?;
    let spec = Spec::load(c)?;
    let (model, effort) = teacher(c)?;
    let root = path(c, "run_root")?;
    let _lease = lease(&root.join("length-projection.lock"))?;
    if !disk_ok(c)? {
        return Err(error("projection storage floor"));
    }
    let mut projected = Vec::new();
    let mut skipped = Vec::new();
    for task in m["tasks"].as_array().ok_or("tasks")? {
        let key = task["key"].as_str().ok_or("key")?;
        if !valid_hash(key) {
            return Err(error("invalid task hash"));
        }
        let job = root.join("jobs").join(key);
        let receipt_path = job.join("receipt.json");
        if !receipt_path.exists() {
            continue;
        }
        let old = load(&receipt_path, JSON_MAX)?;
        if old["outcome"] != "quarantined" || !old["reason"].as_str().unwrap_or("").starts_with(contract::LENGTH_GATE) {
            continue;
        }
        let derived = job.join("length-projection-receipt.json");
        if !derived.exists() && job.join("length-projection-reservation.json").exists() {
            return Err(error("uncertain existing length projection; inspect without replay"));
        }
        let original_receipt_sha = file_hash(&receipt_path)?;
        let reply_path = job.join("reply.json");
        let reply_sha = file_hash(&reply_path)?;
        if old["task_sha256"] != key || old["teacher_reply_sha256"] != reply_sha {
            return Err(error("original quarantine/reply changed"));
        }
        let reservation = load(&job.join("reservation.json"), JSON_MAX)?;
        if reservation["task_sha256"] != key || reservation["model"] != model || reservation["effort"] != effort || reservation["prompt_sha256"] != task["prompt_sha256"] {
            return Err(error("original teacher reservation mismatch"));
        }
        let item: Item = serde_json::from_value(task["item"].clone())?;
        if spec.kind(&item.kind)?.withhold_teacher_when_local_only && task["admission"]["local_only"] != false {
            return Err(error("local-only item cannot have remote projection"));
        }
        let prompt = fs::read_to_string(job.join("prompt.txt"))?;
        let context = fs::read_to_string(job.join("context.txt"))?;
        if file_hash(&item.path)? != item.sha256 || hash(&context) != task["context_sha256"] || hash(&prompt) != task["prompt_sha256"] || prompt.len() > spec.prompt_max_bytes {
            return Err(error("projection source/context pins changed"));
        }
        let reply = load(&reply_path, JSON_MAX.max(spec.output.max_reply_bytes * 2))?;
        if reply["model"] != model.as_str() || reply["provider"] != "codex" || reply["is_error"] != false {
            return Err(error("projection teacher identity mismatch"));
        }
        // Only pure content eligibility errors are isolated per task; I/O and integrity stay fatal.
        let checked = (|| -> Result<(Value, Value)> {
            let text = reply["result"].as_str().ok_or("teacher result")?;
            if text.len() > spec.output.max_reply_bytes {
                return Err(error("teacher result too large"));
            }
            let mut body: Value = serde_json::from_str(text)?;
            if contract::copied_run(&context, &body, spec.output.verbatim_run_tokens) {
                return Err(error("original reply source overlap cannot be repaired"));
            }
            let trace = contract::project(&mut body, &spec.output.length)?.ok_or("length field is not oversized")?;
            contract::validate(&spec, &body, &context)?;
            Ok((body, trace))
        })();
        let (body, trace) = match checked {
            Ok(v) => v,
            Err(e) => {
                skipped.push(json!({"task_sha256":key,"outcome":"skipped_content_ineligible","reason":e.to_string(),"teacher_called":false,"original_receipt_sha256":original_receipt_sha,"teacher_reply_sha256":reply_sha}));
                continue;
            }
        };
        if derived.exists() {
            let prior = load(&derived, JSON_MAX)?;
            if prior["task_sha256"] != key || prior["original_receipt_sha256"] != original_receipt_sha || prior["teacher_reply_sha256"] != reply_sha || prior["projection"] != trace || file_hash(&path(&prior["output"], "manifest")?)? != prior["output"]["manifest_sha256"] {
                return Err(error("existing length projection evidence changed"));
            }
            projected.push(prior);
            continue;
        }
        let mut claim = OpenOptions::new().create_new(true).write(true).open(job.join("length-projection-reservation.json"))?;
        claim.write_all(serde_json::to_string(&json!({"task_sha256":key,"original_receipt_sha256":original_receipt_sha,"teacher_reply_sha256":reply_sha,"projection":trace}))?.as_bytes())?;
        claim.sync_all()?;
        File::open(&job)?.sync_all()?;
        if file_hash(&receipt_path)? != original_receipt_sha || file_hash(&reply_path)? != reply_sha {
            return Err(error("original evidence changed during projection"));
        }
        let output = publish(c, &spec, task, &body, &reply, Some(&trace))?;
        let result = json!({"schema":1,"task_sha256":key,"outcome":"length_projected","original_receipt_sha256":original_receipt_sha,"teacher_reply_sha256":reply_sha,"projection":trace,"output":output,"teacher_model":model,"teacher_called":false});
        crate::util::create_bytes(&derived, &serde_json::to_vec(&result)?)?;
        projected.push(result);
    }
    Ok(json!({"schema":1,"teacher_called":false,"length_projected":projected,"skipped":skipped}))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{contract::tests::body, spec::tests::fixture_spec};
    use std::{cell::Cell, sync::atomic::{AtomicU64, Ordering}};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    const MODEL: &str = "gpt-fixture";
    struct Fixture {
        dir: PathBuf,
        c: Value,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }
    /// Scalar-unit fixture with a 40..50 length gate; inline `note` items.
    fn fixture(n: usize, workers: u64) -> Fixture {
        fixture_unit(n, workers, "unicode_scalars")
    }
    fn fixture_unit(n: usize, workers: u64, unit: &str) -> Fixture {
        let base = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        fs::create_dir_all(&base).unwrap();
        let dir = base.join(format!("whale-run-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir(&dir).unwrap();
        let spec = fixture_spec(&dir, unit, 40, 50);
        let mut items = Vec::new();
        for i in 0..n {
            let p = dir.join(format!("note{i}.txt"));
            fs::write(&p, format!("Note {i}: the counter increases by one per step.\n")).unwrap();
            items.push(json!({"kind":"note","id":format!("note-{i}"),"title":"Fixture note","path":p,"sha256":file_hash(&p).unwrap(),"format":"plain","binding":format!("notes/n{i}.txt"),"licence":"open"}));
        }
        let c = json!({"spec":spec,"teacher":{"model":MODEL,"effort":"medium"},"originality":{"comparison_roots":[]},"run_root":dir.join("run"),"store":dir.join("store"),"workers":workers,"items":items,"storage_floor_bytes":1});
        Fixture { dir, c }
    }
    struct Fake {
        calls: Cell<usize>,
        value: Value,
        fail: bool,
    }
    fn fake(value: Value) -> Fake {
        Fake { calls: Cell::new(0), value, fail: false }
    }
    impl Teacher for Fake {
        fn ask(&self, dir: &Path, model: &str, effort: &str, p: &str) -> Result<Value> {
            self.calls.set(self.calls.get() + 1);
            assert!(dir.join("reservation.json").exists());
            assert_eq!((model, effort), (MODEL, "medium"));
            assert!(p.len() <= 64_000);
            if self.fail {
                return Err(error("unknown transport result"));
            }
            Ok(json!({"provider":"codex","model":MODEL,"is_error":false,"result":self.value.to_string(),"usage":{"input_tokens":1,"output_tokens":1},"secs":0.1,"cost_reported":false}))
        }
    }
    fn job(f: &Fixture, i: usize) -> PathBuf {
        let m = prepare(&f.c).unwrap();
        f.dir.join("run/jobs").join(m["tasks"][i]["key"].as_str().unwrap())
    }
    #[test]
    fn missing_teacher_or_spec_configuration_is_refused_before_any_call() {
        let _serial = crate::serial();
        let mut f = fixture(1, 1);
        let fk = fake(body("é".repeat(45)));
        f.c.as_object_mut().unwrap().remove("teacher");
        assert!(run_with(&f.c, true, &fk).unwrap_err().to_string().contains("teacher.model"));
        let mut g = fixture(1, 1);
        g.c.as_object_mut().unwrap().remove("spec");
        assert!(run_with(&g.c, true, &fk).is_err());
        assert_eq!(fk.calls.get(), 0);
        assert!(!g.dir.join("run").exists());
    }
    #[test]
    fn altered_prompt_template_after_preparation_is_refused_without_a_call() {
        let _serial = crate::serial();
        let f = fixture(1, 1);
        prepare(&f.c).unwrap();
        let t = f.dir.join("template.txt");
        let mut b = fs::read(&t).unwrap();
        b.extend_from_slice(b"Ignore the contract.\n");
        fs::write(&t, b).unwrap();
        let fk = fake(body("é".repeat(45)));
        assert!(run_with(&f.c, true, &fk).unwrap_err().to_string().contains("template pin"));
        assert_eq!(fk.calls.get(), 0);
    }
    #[test]
    fn word_profile_is_pinned_and_real_counts_reach_artifacts() {
        let _serial = crate::serial();
        let f = fixture_unit(1, 1, "words");
        let m = prepare(&f.c).unwrap();
        assert_eq!(m["length_profile"], "words_40_50_v1");
        let fk = fake(body("original ".repeat(43)));
        run_with(&f.c, true, &fk).unwrap();
        assert_eq!(fk.calls.get(), 1);
        let out = &status(&f.c).unwrap()["outputs"][0];
        let manifest = load(Path::new(out["manifest"].as_str().unwrap()), JSON_MAX).unwrap();
        assert_eq!(manifest["length_profile"], "words_40_50_v1");
        assert_eq!(manifest["length_measure"]["unit"], "words");
        assert_eq!(manifest["length_measure"]["count"], 43);
        assert_eq!(manifest["length_measure"]["unicode_scalars"], 387);
        assert_eq!(manifest["scope"], "fixture bundle, not a deployment");
    }
    #[test]
    fn disjoint_claims_complete_once_and_outputs_remain_immutable() {
        let _serial = crate::serial();
        let f = fixture(2, 2);
        let m = prepare(&f.c).unwrap();
        assert_eq!(prepare(&f.c).unwrap(), m);
        let fk = fake(body("é".repeat(45)));
        for i in 0..2 {
            let mut c = f.c.clone();
            c["worker_index"] = json!(i);
            run_with(&c, true, &fk).unwrap();
            run_with(&c, true, &fk).unwrap();
        }
        assert_eq!(fk.calls.get(), 2);
        let st = status(&f.c).unwrap();
        assert_eq!(st["outcomes"]["completed"], 2);
        for out in st["outputs"].as_array().unwrap() {
            let p = PathBuf::from(out["manifest"].as_str().unwrap());
            assert_eq!(file_hash(&p).unwrap(), out["manifest_sha256"]);
            let m = load(&p, JSON_MAX).unwrap();
            assert_eq!((m["local_only"].clone(), m["kind"].clone()), (json!(false), json!("fixture_bundle")));
            assert!(m["source"]["binding"].as_str().unwrap().starts_with("notes/"));
            for a in m["artifacts"].as_array().unwrap() {
                let b = fs::read(p.parent().unwrap().join(a["path"].as_str().unwrap())).unwrap();
                assert_eq!(hash(&b), a["sha256"]);
            }
            assert_eq!(fs::read_to_string(p.parent().unwrap().join("summary.md")).unwrap().chars().count(), 45);
            let claims = load(&p.parent().unwrap().join("claims.json"), JSON_MAX).unwrap();
            assert_eq!(claims["claims"][0]["teacher_claimed_status"], "observed");
            assert_eq!(claims["claims"][0]["status"], "hypothesis");
            assert_eq!(claims["proof_status"], "not_proved");
        }
    }
    #[test]
    fn invalid_reply_is_terminal_quarantine_without_reprompt() {
        let _serial = crate::serial();
        let f = fixture(1, 1);
        let fk = fake(body("short".into()));
        run_with(&f.c, true, &fk).unwrap();
        run_with(&f.c, true, &fk).unwrap();
        assert_eq!(fk.calls.get(), 1);
        assert_eq!(status(&f.c).unwrap()["outcomes"]["quarantined"], 1);
    }
    #[test]
    fn unresolved_and_failed_transport_never_resubmit() {
        let _serial = crate::serial();
        let f = fixture(1, 1);
        let mut fk = fake(body("é".repeat(45)));
        fk.fail = true;
        assert!(run_with(&f.c, true, &fk).is_err());
        assert!(run_with(&f.c, true, &fk).is_err());
        assert_eq!(fk.calls.get(), 1);
        let g = fixture(1, 1);
        fs::write(job(&g, 0).join("reservation.json"), "{}").unwrap();
        let fk = fake(body("é".repeat(45)));
        assert!(run_with(&g.c, true, &fk).is_err());
        assert_eq!(fk.calls.get(), 0);
    }
    #[test]
    fn local_only_admission_withholds_the_teacher() {
        let _serial = crate::serial();
        let mut f = fixture(1, 1);
        f.c["items"][0]["licence"] = json!("restricted");
        let m = prepare(&f.c).unwrap();
        assert_eq!(m["tasks"][0]["admission"]["local_only"], true);
        let fk = fake(body("é".repeat(45)));
        run_with(&f.c, true, &fk).unwrap();
        run_with(&f.c, true, &fk).unwrap();
        assert_eq!(fk.calls.get(), 0);
        let j = job(&f, 0);
        assert_eq!(load(&j.join("receipt.json"), JSON_MAX).unwrap()["outcome"], "local_only_no_remote_teacher");
        assert!(!j.join("reservation.json").exists() && !j.join("reply.json").exists());
    }
    #[test]
    fn exported_metadata_is_pinned_and_path_bound() {
        let _serial = crate::serial();
        let mut f = fixture(1, 1);
        let p = f.dir.join("loose.txt");
        fs::write(&p, "A local text about a bounded method.\n").unwrap();
        let export = f.dir.join("rows.jsonl");
        fs::write(&export, format!("{}\n", json!({"id":"loose1","title":"Bounded","text_path":p}))).unwrap();
        f.c["items"] = json!([{"kind":"loose","id":"loose1","path":p,"sha256":file_hash(&p).unwrap(),"format":"plain"}]);
        assert!(prepare(&f.c).unwrap_err().to_string().contains("metadata_export"));
        f.c["metadata_export"] = json!({"path":export,"sha256":"0".repeat(64)});
        assert!(prepare(&f.c).unwrap_err().to_string().contains("hash mismatch"));
        f.c["metadata_export"]["sha256"] = json!(file_hash(&export).unwrap());
        let m = prepare(&f.c).unwrap();
        assert_eq!(m["tasks"][0]["admission"]["rule"], "no admission rule in specification");
        assert_eq!(m["tasks"][0]["bundle_metadata"]["title"], "Bounded");
    }
    #[test]
    fn epub_items_use_the_configured_adapter() {
        let _serial = crate::serial();
        let mut f = fixture(1, 1);
        let p = f.dir.join("book.epub");
        fs::write(&p, crate::source::tests::epub()).unwrap();
        f.c["items"][0]["path"] = json!(p);
        f.c["items"][0]["format"] = json!("book");
        f.c["items"][0]["sha256"] = json!(file_hash(&p).unwrap());
        let m = prepare(&f.c).unwrap();
        assert_eq!(m["tasks"][0]["coverage"]["adapter"], "epub");
        let prompt = fs::read_to_string(job(&f, 0).join("prompt.txt")).unwrap();
        assert!(prompt.contains("Second chapter") && !prompt.contains("private files"));
    }
    #[test]
    fn context_prompt_bounds_and_pause_stop_prevent_new_calls() {
        let _serial = crate::serial();
        let mut f = fixture(1, 1);
        let p = PathBuf::from(f.c["items"][0]["path"].as_str().unwrap());
        fs::write(&p, "🦈".repeat(48_000 / 4 + 1)).unwrap();
        f.c["items"][0]["sha256"] = json!(file_hash(&p).unwrap());
        let m = prepare(&f.c).unwrap();
        assert_eq!(m["tasks"][0]["coverage"]["partial"], true);
        assert!(m["tasks"][0]["prompt_bytes"].as_u64().unwrap() <= 64_000);
        fs::write(f.dir.join("run/PAUSE"), "operator\n").unwrap();
        let fk = fake(body("é".repeat(45)));
        run_with(&f.c, true, &fk).unwrap();
        assert_eq!(fk.calls.get(), 0);
        fs::remove_file(f.dir.join("run/PAUSE")).unwrap();
        fs::write(f.dir.join("run/STOP"), "operator\n").unwrap();
        run_with(&f.c, true, &fk).unwrap();
        assert_eq!(fk.calls.get(), 0);
    }
    fn old_length_quarantine(f: &Fixture, index: usize, mut b: Value) -> (String, String) {
        let m = prepare(&f.c).unwrap();
        let task = &m["tasks"][index];
        let key = task["key"].as_str().unwrap();
        let j = f.dir.join("run/jobs").join(key);
        // Explicit mock of a retained historical teacher result, never a teacher invocation.
        b["summary"] = json!(format!("{}\n\n{}", "é".repeat(45), "x".repeat(20)));
        save(&j.join("reservation.json"), &json!({"task_sha256":key,"model":MODEL,"effort":"medium","prompt_sha256":task["prompt_sha256"]})).unwrap();
        save(&j.join("reply.json"), &json!({"model":MODEL,"provider":"codex","is_error":false,"result":b.to_string()})).unwrap();
        save(&j.join("receipt.json"), &json!({"task_sha256":key,"outcome":"quarantined","reason":format!("{} fixture",contract::LENGTH_GATE),"teacher_reply_sha256":file_hash(&j.join("reply.json")).unwrap()})).unwrap();
        (file_hash(&j.join("receipt.json")).unwrap(), file_hash(&j.join("reply.json")).unwrap())
    }
    #[test]
    fn explicit_projection_has_no_teacher_and_preserves_quarantine_duplicate_and_claim_gates() {
        let _serial = crate::serial();
        let f = fixture(1, 1);
        let (receipt, reply) = old_length_quarantine(&f, 0, body("unused".into()));
        let first = project_quarantined_lengths(&f.c).unwrap();
        assert_eq!(first["length_projected"].as_array().unwrap().len(), 1);
        assert_eq!(first["teacher_called"], false);
        assert_eq!(project_quarantined_lengths(&f.c).unwrap(), first);
        assert_eq!(first["length_projected"][0]["original_receipt_sha256"], receipt);
        assert_eq!(first["length_projected"][0]["teacher_reply_sha256"], reply);
        let st = status(&f.c).unwrap();
        assert_eq!((st["outcomes"]["quarantined"].clone(), st["outcomes"]["length_projected"].clone()), (json!(1), json!(1)));
        let h = fixture(1, 1);
        let fk = fake(body(format!("{}\n\n{}", "é".repeat(45), "x".repeat(20))));
        run_with(&h.c, true, &fk).unwrap();
        assert_eq!(status(&h.c).unwrap()["outcomes"]["completed"], 1);
        let retained = load(&job(&h, 0).join("reply.json"), JSON_MAX).unwrap();
        let raw: Value = serde_json::from_str(retained["result"].as_str().unwrap()).unwrap();
        assert_eq!(raw["summary"].as_str().unwrap().chars().count(), 67);
        let g = fixture(1, 1);
        let mut bad = body("unused".into());
        bad["review"]["state"] = json!("reviewed");
        old_length_quarantine(&g, 0, bad);
        let rejected = project_quarantined_lengths(&g.c).unwrap();
        assert_eq!(rejected["skipped"].as_array().unwrap().len(), 1);
        assert!(rejected["length_projected"].as_array().unwrap().is_empty());
    }
    #[test]
    fn content_ineligible_task_does_not_block_independent_projection_or_mutate_originals() {
        let _serial = crate::serial();
        let mut f = fixture(2, 1);
        let sentence = "one two three four five six seven eight nine ten eleven twelve thirteen fourteen";
        let p = PathBuf::from(f.c["items"][0]["path"].as_str().unwrap());
        fs::write(&p, sentence).unwrap();
        f.c["items"][0]["sha256"] = json!(file_hash(&p).unwrap());
        let mut bad = body("unused".into());
        bad["method"] = json!(sentence);
        let a = old_length_quarantine(&f, 0, bad);
        let b = old_length_quarantine(&f, 1, body("unused".into()));
        let first = project_quarantined_lengths(&f.c).unwrap();
        assert_eq!(first["skipped"].as_array().unwrap().len(), 1);
        assert!(first["skipped"][0]["reason"].as_str().unwrap().contains("overlap"));
        assert_eq!(first["length_projected"].as_array().unwrap().len(), 1);
        assert_eq!(first, project_quarantined_lengths(&f.c).unwrap());
        for (i, pins) in [a, b].iter().enumerate() {
            let j = job(&f, i);
            assert_eq!(file_hash(&j.join("receipt.json")).unwrap(), pins.0);
            assert_eq!(file_hash(&j.join("reply.json")).unwrap(), pins.1);
            if i == 0 {
                assert!(!j.join("length-projection-reservation.json").exists());
            }
        }
        fs::write(&p, "tampered").unwrap();
        assert!(project_quarantined_lengths(&f.c).is_err());
    }
    #[test]
    fn binding_paths_are_required_normal_relative_and_scoped() {
        let _serial = crate::serial();
        let f = fixture(1, 1);
        let spec = Spec::load(&f.c).unwrap();
        for bad in ["", "/notes/x.txt", "notes/../x.txt", "notes/./x.txt", "notes//x.txt", "other/x.txt", "notes/x.md", "notes/.txt"] {
            let mut c = f.c.clone();
            c["items"][0]["binding"] = json!(bad);
            assert!(items(&c, &spec).is_err(), "{bad}");
        }
        let mut c = f.c.clone();
        c["items"][0]["kind"] = json!("unknown");
        assert!(items(&c, &spec).is_err());
    }
    #[test]
    fn larger_explicit_queue_requires_bounded_opt_in_and_unknown_receipt_is_not_retried() {
        let _serial = crate::serial();
        let mut f = fixture(65, 2);
        let spec = Spec::load(&f.c).unwrap();
        assert!(items(&f.c, &spec).is_err());
        f.c["max_items"] = json!(256);
        assert_eq!(items(&f.c, &spec).unwrap().len(), 65);
        f.c["max_items"] = json!(257);
        assert!(items(&f.c, &spec).is_err());
        let g = fixture(1, 1);
        let j = job(&g, 0);
        let key = j.file_name().unwrap().to_str().unwrap().to_owned();
        save(&j.join("receipt.json"), &json!({"task_sha256":key,"outcome":"unknown"})).unwrap();
        let fk = fake(body("é".repeat(45)));
        assert!(run_with(&g.c, true, &fk).is_err());
        assert_eq!(fk.calls.get(), 0);
    }
    #[test]
    fn full_primary_tail_and_unconfigured_corpus_quarantine_without_reprompt() {
        let _serial = crate::serial();
        let f = fixture(1, 1);
        let p = PathBuf::from(f.c["items"][0]["path"].as_str().unwrap());
        let tail = "Twelve distinct source tokens are intentionally preserved here beyond the narrow context boundary";
        fs::write(&p, format!("{}\n{tail}", "x ".repeat(48_000))).unwrap();
        let mut c = f.c.clone();
        c["items"][0]["sha256"] = json!(file_hash(&p).unwrap());
        let mut b = body("é".repeat(45));
        b["method"] = json!(tail);
        let fk = fake(b);
        run_with(&c, true, &fk).unwrap();
        assert_eq!(fk.calls.get(), 1);
        let key = prepare(&c).unwrap()["tasks"][0]["key"].as_str().unwrap().to_owned();
        let j = f.dir.join("run/jobs").join(key);
        assert_eq!(load(&j.join("receipt.json"), JSON_MAX).unwrap()["outcome"], "quarantined");
        assert_eq!(load(&j.join("originality.json"), JSON_MAX).unwrap()["decision"], "blocked_overlap");
        let g = fixture(1, 1);
        let mut c = g.c.clone();
        c.as_object_mut().unwrap().remove("originality");
        run_with(&c, true, &fake(body("é".repeat(43)))).unwrap();
        assert_eq!(status(&c).unwrap()["outcomes"]["quarantined"], 1);
    }
    #[test]
    fn source_changes_and_duplicate_worker_lease_fail_closed() {
        let _serial = crate::serial();
        let f = fixture(1, 1);
        prepare(&f.c).unwrap();
        let prepare_lease = lease(&f.dir.join("run/prepare.lock")).unwrap();
        assert!(prepare(&f.c).is_ok());
        drop(prepare_lease);
        let held = lease(&f.dir.join("run/workers/00/worker.lock")).unwrap();
        let fk = fake(body("é".repeat(45)));
        assert!(run_with(&f.c, true, &fk).is_err());
        drop(held);
        fs::write(PathBuf::from(f.c["items"][0]["path"].as_str().unwrap()), "changed").unwrap();
        assert!(run_with(&f.c, true, &fk).is_err());
        assert_eq!(fk.calls.get(), 0);
    }
}
