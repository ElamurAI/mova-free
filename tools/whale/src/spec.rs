//! Pinned Whale specification: prompt template, source kinds, admission, output contract,
//! originality parameters and bundle layout. Public code knows no concrete task domain;
//! every domain choice arrives through this hash-pinned file and its pinned template.
use crate::{
    source::Adapter,
    util::{error, hash, label, normal, pinned, valid_hash},
    Result,
};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

pub const SPEC_SCHEMA: &str = "mova-shark-spec-v1";
pub const METADATA_SLOT: &str = "{{METADATA_JSON}}";
pub const SOURCE_SLOT: &str = "{{SOURCE_TEXT}}";
const SPEC_MAX: usize = 1 << 20;
const TEMPLATE_MAX: usize = 256 * 1024;

fn d_prompt_max() -> usize {
    64_000
}
fn d_context_max() -> usize {
    48_000
}
fn d_text_max() -> usize {
    12_000
}
fn d_list_min() -> usize {
    1
}
fn d_list_max() -> usize {
    24
}
fn d_item_max() -> usize {
    4_000
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pin {
    pub path: String,
    pub sha256: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BindingRule {
    /// Item field `binding` must be a normal relative path with this prefix and suffix.
    pub prefix: String,
    pub suffix: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Admission {
    /// Metadata field holding the licence name.
    pub licence_field: String,
    /// Admitted names after generic normalization (lowercase, unified dashes, single spaces).
    pub admitted: Vec<String>,
    #[serde(default)]
    pub ignored_suffixes: Vec<String>,
    #[serde(default)]
    pub prefix_rewrites: Vec<(String, String)>,
    #[serde(default)]
    pub require_true: Vec<String>,
    #[serde(default)]
    pub require_false: Vec<String>,
}
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MetadataSource {
    Inline,
    Export,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KindSpec {
    /// Item format name -> adapter (`text` or `epub`).
    pub formats: BTreeMap<String, String>,
    pub metadata: MetadataSource,
    /// For exported metadata: a row field whose canonical path must equal a `text` item path.
    #[serde(default)]
    pub export_path_field: Option<String>,
    #[serde(default)]
    pub binding: Option<BindingRule>,
    #[serde(default)]
    pub admission: Option<Admission>,
    /// A local-only item of this kind is receipted without any teacher call.
    #[serde(default)]
    pub withhold_teacher_when_local_only: bool,
}
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    Words,
    UnicodeScalars,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LengthRule {
    pub pointer: String,
    pub unit: Unit,
    pub min: usize,
    pub max: usize,
    /// Versioned profile identifier; a changed rule needs a new identifier.
    pub profile: String,
    #[serde(default)]
    pub forbid: Vec<String>,
    #[serde(default)]
    pub project_oversized: bool,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum FieldRule {
    Equals {
        pointer: String,
        value: Value,
    },
    Text {
        pointer: String,
        #[serde(default = "d_text_max")]
        max_bytes: usize,
    },
    StringList {
        pointer: String,
        #[serde(default = "d_list_min")]
        min: usize,
        #[serde(default = "d_list_max")]
        max: usize,
        #[serde(default = "d_item_max")]
        item_max_bytes: usize,
    },
    ObjectList {
        pointer: String,
        min: usize,
        max: usize,
        #[serde(default)]
        text_fields: Vec<String>,
        #[serde(default = "d_text_max")]
        text_max_bytes: usize,
        #[serde(default)]
        string_list_fields: Vec<String>,
        #[serde(default)]
        enum_fields: BTreeMap<String, Vec<String>>,
        #[serde(default)]
        distinct_by: Vec<String>,
        #[serde(default)]
        must_contain: BTreeMap<String, Vec<String>>,
    },
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OutputContract {
    pub max_reply_bytes: usize,
    pub length: LengthRule,
    pub fields: Vec<FieldRule>,
    /// Contiguous output tokens shared with the teacher context that quarantine a reply; 0 disables.
    #[serde(default)]
    pub verbatim_run_tokens: usize,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginalitySpec {
    pub enabled: bool,
    #[serde(default)]
    pub non_narrative_fields: Vec<String>,
    #[serde(default)]
    pub thresholds: Map<String, Value>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PathPointer {
    pub path: String,
    pub pointer: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonArtifact {
    pub path: String,
    pub fields: BTreeMap<String, String>,
    #[serde(default)]
    pub constants: Map<String, Value>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StatusRewrite {
    pub list: String,
    pub field: String,
    pub keep: Vec<String>,
    pub otherwise: String,
    pub claimed_field: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleSpec {
    pub store_prefix: String,
    pub manifest_kind: String,
    pub manifest_status: String,
    pub id_prefix: String,
    #[serde(default)]
    pub text_artifacts: Vec<PathPointer>,
    #[serde(default)]
    pub list_artifacts: Vec<PathPointer>,
    #[serde(default)]
    pub json_artifacts: Vec<JsonArtifact>,
    #[serde(default)]
    pub status_rewrites: Vec<StatusRewrite>,
    pub metadata_artifact: String,
    pub metadata_fields: Vec<String>,
    #[serde(default)]
    pub manifest_extra: Map<String, Value>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SpecFile {
    schema: String,
    id: String,
    prompt_template: Pin,
    #[serde(default = "d_prompt_max")]
    prompt_max_bytes: usize,
    #[serde(default = "d_context_max")]
    context_max_bytes: usize,
    prompt_metadata_fields: Vec<String>,
    kinds: BTreeMap<String, KindSpec>,
    output: OutputContract,
    originality: OriginalitySpec,
    bundle: BundleSpec,
}
/// Loaded, validated specification with its pins.
#[derive(Clone, Debug)]
pub struct Spec {
    pub id: String,
    pub path: PathBuf,
    pub sha256: String,
    pub template_sha256: String,
    template: String,
    pub prompt_max_bytes: usize,
    pub context_max_bytes: usize,
    prompt_metadata_fields: Vec<String>,
    pub kinds: BTreeMap<String, KindSpec>,
    pub output: OutputContract,
    pub originality: OriginalitySpec,
    pub bundle: BundleSpec,
}
/// Generated bundle files that a specification cannot claim for its own artifacts.
pub const RESERVED_ARTIFACTS: [&str; 4] = ["manifest.json", "teacher-receipt.json", "evidence.json", "originality.json"];
fn artifact_name(s: &str) -> bool {
    !s.is_empty() && s.len() <= 128 && !s.starts_with('.') && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}
fn pointer(s: &str) -> bool {
    s.is_empty() || (s.starts_with('/') && s.len() <= 512)
}
impl Spec {
    /// Load the configured `spec` pin. The template pin may be relative to the spec file.
    pub fn load(config: &Value) -> Result<Self> {
        let pin = &config["spec"];
        if !pin.is_object() {
            return Err(error("config requires spec {path, sha256}; there is no built-in specification"));
        }
        if !Path::new(pin["path"].as_str().unwrap_or("")).is_absolute() {
            return Err(error("spec.path must be absolute"));
        }
        let (path, bytes) = pinned(pin, None, SPEC_MAX).map_err(|e| error(format!("spec pin: {e}")))?;
        let f: SpecFile = serde_json::from_slice(&bytes).map_err(|e| error(format!("spec contract: {e}")))?;
        if f.schema != SPEC_SCHEMA || !label(&f.id) {
            return Err(error(format!("spec schema must be {SPEC_SCHEMA} with a label id")));
        }
        let tpin = json!({"path":f.prompt_template.path,"sha256":f.prompt_template.sha256});
        let (_, tb) = pinned(&tpin, path.parent(), TEMPLATE_MAX).map_err(|e| error(format!("prompt template pin: {e}")))?;
        let template = String::from_utf8(tb)?;
        for slot in [METADATA_SLOT, SOURCE_SLOT] {
            if template.matches(slot).count() != 1 {
                return Err(error(format!("prompt template must contain {slot} exactly once")));
            }
        }
        let s = Spec {
            id: f.id,
            sha256: hash(&bytes),
            path,
            template_sha256: f.prompt_template.sha256,
            template,
            prompt_max_bytes: f.prompt_max_bytes,
            context_max_bytes: f.context_max_bytes,
            prompt_metadata_fields: f.prompt_metadata_fields,
            kinds: f.kinds,
            output: f.output,
            originality: f.originality,
            bundle: f.bundle,
        };
        s.validate()?;
        Ok(s)
    }
    fn validate(&self) -> Result<()> {
        if !(1..=1_000_000).contains(&self.prompt_max_bytes) || !(1..=1_000_000).contains(&self.context_max_bytes) {
            return Err(error("prompt/context bounds must be 1..1000000 bytes"));
        }
        if self.kinds.is_empty() || self.kinds.len() > 32 {
            return Err(error("spec needs 1..32 kinds"));
        }
        for (k, v) in &self.kinds {
            if !label(k) || v.formats.is_empty() {
                return Err(error("kind names must be labels with at least one format"));
            }
            for (fmt, a) in &v.formats {
                Adapter::parse(a)?;
                if !label(fmt) {
                    return Err(error("format names must be labels"));
                }
            }
            if let Some(b) = &v.binding {
                if b.prefix.is_empty() || !normal(b.prefix.trim_end_matches('/')) {
                    return Err(error("binding prefix must be a normal relative directory"));
                }
            }
            if let Some(a) = &v.admission {
                if a.licence_field.is_empty() || a.admitted.is_empty() {
                    return Err(error("admission needs licence_field and admitted names"));
                }
            }
        }
        let o = &self.output;
        let l = &o.length;
        if !(1..=8 << 20).contains(&o.max_reply_bytes) || !pointer(&l.pointer) || l.pointer.is_empty() || l.min == 0 || l.min > l.max || !label(&l.profile.replace('.', "_")) {
            return Err(error("output length rule or reply bound invalid"));
        }
        if o.verbatim_run_tokens == 1 || o.verbatim_run_tokens > 256 || o.fields.len() > 128 {
            return Err(error("verbatim_run_tokens must be 0 or 2..256; at most 128 field rules"));
        }
        for f in &o.fields {
            let (p, ok) = match f {
                FieldRule::Equals { pointer, .. } => (pointer, true),
                FieldRule::Text { pointer, max_bytes } => (pointer, *max_bytes > 0),
                FieldRule::StringList { pointer, min, max, item_max_bytes } => (pointer, min <= max && *item_max_bytes > 0 && *max <= 4096),
                FieldRule::ObjectList { pointer, min, max, .. } => (pointer, min <= max && *max <= 4096),
            };
            if !ok || !pointer(p) {
                return Err(error("invalid output field rule"));
            }
        }
        let b = &self.bundle;
        if !label(&b.store_prefix) || b.manifest_kind.is_empty() || b.manifest_status.is_empty() || b.id_prefix.len() > 64 {
            return Err(error("bundle store_prefix/manifest_kind/status/id_prefix invalid"));
        }
        let mut names = BTreeSet::new();
        let all = b.text_artifacts.iter().map(|x| &x.path).chain(b.list_artifacts.iter().map(|x| &x.path)).chain(b.json_artifacts.iter().map(|x| &x.path)).chain([&b.metadata_artifact]);
        for n in all {
            if !artifact_name(n) || RESERVED_ARTIFACTS.contains(&n.as_str()) || !names.insert(n.clone()) {
                return Err(error(format!("bundle artifact name invalid, reserved or duplicate: {n}")));
            }
        }
        for p in b.text_artifacts.iter().chain(&b.list_artifacts).map(|x| &x.pointer).chain(b.json_artifacts.iter().flat_map(|j| j.fields.values())).chain(b.status_rewrites.iter().map(|r| &r.list)) {
            if !pointer(p) {
                return Err(error("bundle pointers must be JSON pointers"));
            }
        }
        for k in b.manifest_extra.keys() {
            if MANIFEST_KEYS.contains(&k.as_str()) {
                return Err(error(format!("manifest_extra cannot override generic key {k}")));
            }
        }
        Ok(())
    }
    pub fn kind(&self, k: &str) -> Result<&KindSpec> {
        self.kinds.get(k).ok_or_else(|| error(format!("source kind {k:?} is not admitted by the specification")))
    }
    /// Render the pinned template. Slots are located in the template, never in substituted text.
    pub fn prompt(&self, metadata: &Value, context: &str) -> Result<String> {
        let meta = serde_json::to_string(metadata)?;
        let m = self.template.find(METADATA_SLOT).ok_or("metadata slot")?;
        let s = self.template.find(SOURCE_SLOT).ok_or("source slot")?;
        let mut parts = [(m, METADATA_SLOT.len(), meta.as_str()), (s, SOURCE_SLOT.len(), context)];
        parts.sort_by_key(|x| x.0);
        let mut out = String::new();
        let mut at = 0;
        for (pos, len, text) in parts {
            out.push_str(&self.template[at..pos]);
            out.push_str(text);
            at = pos + len;
        }
        out.push_str(&self.template[at..]);
        if out.len() > self.prompt_max_bytes {
            return Err(error(format!("prepared prompt exceeds {} UTF-8 bytes", self.prompt_max_bytes)));
        }
        Ok(out)
    }
    /// Teacher-visible metadata projection; long strings are cut to 1000 bytes.
    pub fn prompt_metadata(&self, meta: &Value) -> Value {
        project(meta, &self.prompt_metadata_fields, 1000)
    }
    /// Originality configuration: specification thresholds plus deployment corpus keys only.
    pub fn originality_config(&self, deployment: &Value) -> Result<Value> {
        let mut c = Value::Object(self.originality.thresholds.clone());
        if let Some(m) = deployment.as_object() {
            for (k, v) in m {
                if !CORPUS_KEYS.contains(&k.as_str()) {
                    return Err(error(format!("deployment originality key {k} is not a corpus/bound key")));
                }
                c[k] = v.clone();
            }
        }
        Ok(c)
    }
}
const CORPUS_KEYS: [&str; 7] = ["comparison_roots", "comparison_extensions", "cache_path", "max_entries", "max_bytes", "max_document_bytes", "max_output_bytes"];
/// Generic manifest keys written by the bundle publisher.
pub const MANIFEST_KEYS: [&str; 16] = ["schema", "id", "kind", "status", "source", "admission", "local_only", "public_export_allowed", "coverage", "length_measure", "length_profile", "artifacts", "teacher", "version_sha256", "task_sha256", "spec"];
pub fn project(meta: &Value, fields: &[String], cap: usize) -> Value {
    let mut out = Map::new();
    for k in fields {
        if let Some(v) = meta.get(k) {
            out.insert(k.clone(), match v {
                Value::String(s) => json!(crate::util::prefix(s, cap)),
                v => v.clone(),
            });
        }
    }
    Value::Object(out)
}
/// Generic licence normalization: lowercase, unified dashes, single spaces, then the
/// specification's suffix removals and prefix rewrites. Exact matching only.
pub fn normalize_licence(name: &str, a: &Admission) -> String {
    let n = name.to_lowercase().replace(['‐', '‑', '‒', '–', '—', '−'], "-").split_whitespace().collect::<Vec<_>>().join(" ");
    let mut n = a.ignored_suffixes.iter().find_map(|s| n.strip_suffix(&s.to_lowercase()).map(str::to_owned)).unwrap_or(n);
    for (from, to) in &a.prefix_rewrites {
        if let Some(rest) = n.strip_prefix(&from.to_lowercase()) {
            n = format!("{}{rest}", to.to_lowercase());
            break;
        }
    }
    n
}
/// Admission record: `local_only` withholds publication (and, per kind, the teacher call).
pub fn admission(kind: &KindSpec, item_local_only: bool, meta: &Value) -> Value {
    let Some(a) = &kind.admission else {
        return json!({"name":meta["licence"].as_str().unwrap_or("unspecified"),"local_only":item_local_only,"public_export_allowed":!item_local_only,"rule":"no admission rule in specification","decision":"metadata policy, not legal adjudication; no automatic publication"});
    };
    let name = meta[&a.licence_field].as_str().unwrap_or("");
    let normalized = normalize_licence(name, a);
    let open = a.admitted.iter().any(|x| x.to_lowercase() == normalized);
    let flags = a.require_true.iter().all(|f| meta[f] == true) && a.require_false.iter().all(|f| meta[f] == false);
    let local = item_local_only || !open || !flags;
    json!({"name":if name.is_empty(){"unknown"}else{name},"normalized":normalized,"local_only":local,"public_export_allowed":!local,"decision":"metadata policy, not legal adjudication; no automatic publication"})
}
pub fn valid_key(s: &str) -> bool {
    valid_hash(s)
}
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::fs;
    /// A generic, domain-free fixture specification for tests: one `note` kind of text sources,
    /// a `summary` length field and two list fields. Not a deployment specification.
    pub fn fixture_spec(dir: &Path, unit: &str, min: usize, max: usize) -> Value {
        let t = dir.join("template.txt");
        fs::write(&t, "Return one JSON object. Metadata: {{METADATA_JSON}}\nBEGIN SOURCE DATA\n{{SOURCE_TEXT}}\nEND SOURCE DATA\n").unwrap();
        let spec = json!({"schema":SPEC_SCHEMA,"id":"fixture","prompt_template":{"path":"template.txt","sha256":crate::util::file_hash(&t).unwrap()},
            "prompt_metadata_fields":["id","title"],
            "kinds":{"note":{"formats":{"plain":"text","book":"epub"},"metadata":"inline","binding":{"prefix":"notes/","suffix":".txt"},
                "admission":{"licence_field":"licence","admitted":["open"],"require_true":[],"require_false":[]},"withhold_teacher_when_local_only":true},
                "loose":{"formats":{"plain":"text"},"metadata":"export","export_path_field":"text_path"}},
            "output":{"max_reply_bytes":256000,"verbatim_run_tokens":12,
                "length":{"pointer":"/summary","unit":unit,"min":min,"max":max,"profile":format!("{unit}_{min}_{max}_v1"),"forbid":["```"],"project_oversized":true},
                "fields":[{"kind":"equals","pointer":"/schema","value":1},{"kind":"string_list","pointer":"/points"},{"kind":"text","pointer":"/method"},
                    {"kind":"object_list","pointer":"/claims","min":1,"max":4,"text_fields":["type","statement"],"string_list_fields":["support"],"enum_fields":{"status":["observed","hypothesis","missing"]},"distinct_by":["type","statement"],"must_contain":{"type":["primary"]}},
                    {"kind":"equals","pointer":"/review/state","value":"unreviewed"}]},
            "originality":{"enabled":true,"non_narrative_fields":["schema","status","type"]},
            "bundle":{"store_prefix":"notes","manifest_kind":"fixture_bundle","manifest_status":"generated_unverified","id_prefix":"fixture-",
                "text_artifacts":[{"path":"summary.md","pointer":"/summary"},{"path":"method.md","pointer":"/method"}],
                "list_artifacts":[{"path":"points.md","pointer":"/points"}],
                "json_artifacts":[{"path":"claims.json","fields":{"claims":"/claims"},"constants":{"proof_status":"not_proved"}}],
                "status_rewrites":[{"list":"/claims","field":"status","keep":["missing"],"otherwise":"hypothesis","claimed_field":"teacher_claimed_status"}],
                "metadata_artifact":"metadata.json","metadata_fields":["id","title","licence"],"manifest_extra":{"scope":"fixture bundle, not a deployment"}}});
        let p = dir.join("spec.json");
        fs::write(&p, serde_json::to_vec_pretty(&spec).unwrap()).unwrap();
        json!({"path":p,"sha256":crate::util::file_hash(&p).unwrap()})
    }
    fn dir(tag: &str) -> PathBuf {
        let base = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let d = base.join(format!("whale-spec-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }
    #[test]
    fn missing_or_altered_spec_and_template_pins_are_refused() {
        let d = dir("pins");
        let pin = fixture_spec(&d, "words", 5, 10);
        let s = Spec::load(&json!({"spec":pin})).unwrap();
        assert_eq!(s.id, "fixture");
        assert!(Spec::load(&json!({})).unwrap_err().to_string().contains("no built-in"));
        // Negative control: one changed template byte must be refused.
        let t = d.join("template.txt");
        let mut b = fs::read(&t).unwrap();
        b.push(b' ');
        fs::write(&t, &b).unwrap();
        assert!(Spec::load(&json!({"spec":pin})).unwrap_err().to_string().contains("template pin"));
        b.pop();
        fs::write(&t, &b).unwrap();
        assert!(Spec::load(&json!({"spec":pin})).is_ok());
        let mut forged = pin.clone();
        forged["sha256"] = json!("0".repeat(64));
        assert!(Spec::load(&json!({"spec":forged})).unwrap_err().to_string().contains("spec pin"));
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn template_slots_are_located_in_the_template_not_in_substituted_text() {
        let d = dir("slots");
        let s = Spec::load(&json!({"spec":fixture_spec(&d, "words", 5, 10)})).unwrap();
        let p = s.prompt(&json!({"title":"{{SOURCE_TEXT}}"}), "body {{METADATA_JSON}}").unwrap();
        assert!(p.contains("\"title\":\"{{SOURCE_TEXT}}\""));
        assert!(p.contains("body {{METADATA_JSON}}"));
        let long = "x".repeat(70_000);
        assert!(s.prompt(&json!({}), &long).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn licence_normalization_is_exact_after_configured_rewrites() {
        let a: Admission = serde_json::from_value(json!({"licence_field":"licence","admitted":["cc by 4.0","cc0 1.0","public domain"],"ignored_suffixes":[" (archive)"],"prefix_rewrites":[["cc-by ","cc by "]],"require_true":["public"],"require_false":["text_local_only"]})).unwrap();
        let kind = KindSpec { formats: BTreeMap::new(), metadata: MetadataSource::Inline, export_path_field: None, binding: None, admission: Some(a), withhold_teacher_when_local_only: true };
        for name in ["unknown", "CC BY-ND 4.0", "CC BY-SA 4.0", "CC BY Non-Commercial 4.0", "CC BY/NC", "CC BY–NC 4.0", "Not public domain", "CC BY 4.0 with additional restrictions"] {
            assert_eq!(admission(&kind, false, &json!({"licence":name,"public":true,"text_local_only":false}))["local_only"], true, "{name}");
        }
        for name in ["CC BY 4.0", "CC BY 4.0 (archive)", "CC-BY 4.0", "CC–BY 4.0", "CC0 1.0", "Public domain"] {
            assert_eq!(admission(&kind, false, &json!({"licence":name,"public":true,"text_local_only":false}))["local_only"], false, "{name}");
        }
        assert_eq!(admission(&kind, false, &json!({"licence":"CC BY 4.0","public":true}))["local_only"], true);
        assert_eq!(admission(&kind, true, &json!({"licence":"CC BY 4.0","public":true,"text_local_only":false}))["local_only"], true);
    }
}
