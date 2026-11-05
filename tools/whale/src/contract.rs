//! Configured output contract: length measure and editorial projection, field rules,
//! verbatim-context guard and bundle artifact rendering. Gate errors carry a stable prefix.
use crate::{
    spec::{FieldRule, LengthRule, Spec, Unit},
    util::{error, hash},
    Result,
};
use serde_json::{json, Value};
use std::collections::BTreeSet;

/// Every length-gate quarantine reason starts with this prefix; projection selects on it.
pub const LENGTH_GATE: &str = "length gate:";
pub const WORD_TOKENIZATION: &str = "unicode_whitespace_tokens_containing_unicode_alphanumeric_v1";

pub fn word_count(s: &str) -> usize {
    s.split_whitespace().filter(|t| t.chars().any(char::is_alphanumeric)).count()
}
pub fn count(s: &str, u: Unit) -> usize {
    match u {
        Unit::Words => word_count(s),
        Unit::UnicodeScalars => s.chars().count(),
    }
}
fn unit_id(u: Unit) -> &'static str {
    match u {
        Unit::Words => "words",
        Unit::UnicodeScalars => "unicode_scalars",
    }
}
fn tokenization(u: Unit) -> &'static str {
    match u {
        Unit::Words => WORD_TOKENIZATION,
        Unit::UnicodeScalars => "unicode_scalars_v1",
    }
}
pub fn measure(s: &str, l: &LengthRule) -> Value {
    json!({"unit":unit_id(l.unit),"count":count(s,l.unit),"minimum":l.min,"maximum":l.max,"profile":l.profile,"tokenization":tokenization(l.unit),"words":word_count(s),"unicode_scalars":s.chars().count()})
}
fn at<'a>(v: &'a Value, p: &str) -> &'a Value {
    v.pointer(p).unwrap_or(&Value::Null)
}
fn text<'a>(v: &'a Value, p: &str, max: usize) -> Result<&'a str> {
    let s = at(v, p).as_str().ok_or_else(|| error(format!("missing string {p}")))?;
    if s.trim().is_empty() || s.len() > max {
        return Err(error(format!("empty/oversized {p}")));
    }
    Ok(s)
}
fn strings(v: &Value, p: &str, min: usize, max: usize, item: usize) -> Result<()> {
    let a = at(v, p).as_array().ok_or_else(|| error(format!("{p} must be a string array")))?;
    if a.len() < min || a.len() > max || a.iter().any(|x| x.as_str().is_none_or(|s| s.trim().is_empty() || s.len() > item)) {
        return Err(error(format!("{p} must hold {min}..{max} nonempty strings of at most {item} bytes")));
    }
    Ok(())
}
fn words(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_alphanumeric()).filter(|s| !s.is_empty()).map(str::to_lowercase).collect()
}
/// True when any `n` consecutive normalized output words also occur consecutively in `context`.
pub fn copied_run(context: &str, output: &Value, n: usize) -> bool {
    fn collect(v: &Value, out: &mut String) {
        match v {
            Value::String(s) => {
                out.push_str(s);
                out.push(' ');
            }
            Value::Array(a) => a.iter().for_each(|v| collect(v, out)),
            Value::Object(m) => m.values().for_each(|v| collect(v, out)),
            _ => {}
        }
    }
    if n == 0 {
        return false;
    }
    let mut t = String::new();
    collect(output, &mut t);
    let source = words(context);
    let grams: BTreeSet<_> = source.windows(n).map(|x| x.join(" ")).collect();
    words(&t).windows(n).any(|x| grams.contains(&x.join(" ")))
}
/// Validate a parsed reply against the configured contract. Returns the length-field count.
pub fn validate(spec: &Spec, v: &Value, context: &str) -> Result<usize> {
    let o = &spec.output;
    let l = &o.length;
    let s = at(v, &l.pointer).as_str().ok_or_else(|| error(format!("{LENGTH_GATE} missing string {}", l.pointer)))?;
    let n = count(s, l.unit);
    if !(l.min..=l.max).contains(&n) || s.trim().is_empty() || l.forbid.iter().any(|f| s.contains(f.as_str())) {
        return Err(error(format!("{LENGTH_GATE} {} must contain {}..{} {} without forbidden substrings", l.pointer, l.min, l.max, unit_id(l.unit))));
    }
    for f in &o.fields {
        match f {
            FieldRule::Equals { pointer, value } => {
                if at(v, pointer) != value {
                    return Err(error(format!("{pointer} must equal {value}")));
                }
            }
            FieldRule::Text { pointer, max_bytes } => {
                text(v, pointer, *max_bytes)?;
            }
            FieldRule::StringList { pointer, min, max, item_max_bytes } => strings(v, pointer, *min, *max, *item_max_bytes)?,
            FieldRule::ObjectList { pointer, min, max, text_fields, text_max_bytes, string_list_fields, enum_fields, distinct_by, must_contain } => {
                let list = at(v, pointer).as_array().ok_or_else(|| error(format!("{pointer} must be an array")))?;
                if list.len() < *min || list.len() > *max {
                    return Err(error(format!("{pointer} must contain {min}..{max} objects")));
                }
                let mut seen = BTreeSet::new();
                let mut values: Vec<BTreeSet<String>> = vec![BTreeSet::new(); must_contain.len()];
                for m in list {
                    for k in text_fields {
                        text(m, &format!("/{k}"), *text_max_bytes)?;
                    }
                    for k in string_list_fields {
                        strings(m, &format!("/{k}"), 1, 24, 4000)?;
                    }
                    for (k, allowed) in enum_fields {
                        if !m[k].as_str().is_some_and(|x| allowed.iter().any(|a| a == x)) {
                            return Err(error(format!("{pointer}/*/{k} outside admitted values")));
                        }
                    }
                    if !distinct_by.is_empty() && !seen.insert(distinct_by.iter().map(|k| m[k].clone()).collect::<Vec<_>>().iter().map(Value::to_string).collect::<Vec<_>>()) {
                        return Err(error(format!("{pointer} entries must be distinct by {distinct_by:?}")));
                    }
                    for (i, k) in must_contain.keys().enumerate() {
                        if let Some(x) = m[k].as_str() {
                            values[i].insert(x.to_owned());
                        }
                    }
                }
                for (i, (k, need)) in must_contain.iter().enumerate() {
                    if need.iter().any(|x| !values[i].contains(x)) {
                        return Err(error(format!("{pointer} must contain {k} values {need:?}")));
                    }
                }
            }
        }
    }
    if serde_json::to_vec(v)?.len() > o.max_reply_bytes {
        return Err(error("generated output exceeds configured output budget"));
    }
    if copied_run(context, v, o.verbatim_run_tokens) {
        return Err(error(format!("generated output contains a {}-word source overlap; quarantine instead of copying", o.verbatim_run_tokens)));
    }
    Ok(n)
}
/// Editorial projection of an oversized length field to the last whole paragraph, else the
/// last sentence, whose prefix count is admitted. No padding, no rewrite, no teacher call.
pub fn project(body: &mut Value, l: &LengthRule) -> Result<Option<Value>> {
    let original = at(body, &l.pointer).as_str().ok_or_else(|| error(format!("missing {}", l.pointer)))?.to_owned();
    let input = count(&original, l.unit);
    if input <= l.max || !l.project_oversized {
        return Ok(None);
    }
    if l.forbid.iter().any(|f| original.contains(f.as_str())) {
        return Err(error("field with forbidden substrings cannot be projected"));
    }
    let admitted = |end: usize| (l.min..=l.max).contains(&count(&original[..end], l.unit));
    let paragraph = original.match_indices("\n\n").map(|(n, _)| n).filter(|&n| admitted(n)).last();
    let sentence = original
        .char_indices()
        .filter_map(|(n, c)| {
            let end = n + c.len_utf8();
            (matches!(c, '.' | '!' | '?') && original[end..].chars().next().is_none_or(char::is_whitespace) && admitted(end)).then_some(end)
        })
        .last();
    let (end, boundary) = paragraph
        .map(|n| (n, "paragraph"))
        .or_else(|| sentence.map(|n| (n, "sentence")))
        .ok_or_else(|| error(format!("{LENGTH_GATE} oversized field has no whole paragraph/sentence boundary within {}..{}", l.min, l.max)))?;
    let output = original[..end].to_owned();
    let out_n = count(&output, l.unit);
    let trace = json!({"schema":1,"pointer":l.pointer,"profile":l.profile,"unit":unit_id(l.unit),"tokenization":tokenization(l.unit),
        "input_count":input,"output_count":out_n,"removed_count":input-out_n,
        "input_unicode_scalars":original.chars().count(),"output_unicode_scalars":output.chars().count(),
        "input_sha256":hash(&original),"output_sha256":hash(&output),"boundary":boundary,"prefix_bytes":end,
        "partial":true,"semantic_preservation":"not_established"});
    *body.pointer_mut(&l.pointer).ok_or("length pointer")? = json!(output);
    Ok(Some(trace))
}
/// Apply configured status rewrites (teacher claims are retained beside the downgraded value).
pub fn rewrite_statuses(spec: &Spec, body: &mut Value) -> Result<()> {
    for r in &spec.bundle.status_rewrites {
        let Some(list) = body.pointer_mut(&r.list).and_then(Value::as_array_mut) else { continue };
        for m in list {
            let claimed = m[&r.field].clone();
            m[&r.claimed_field] = claimed.clone();
            let keep = claimed.as_str().is_some_and(|c| r.keep.iter().any(|k| k == c));
            if !keep {
                m[&r.field] = json!(r.otherwise);
            }
        }
    }
    Ok(())
}
/// Render configured artifacts from a validated (and rewritten) body.
pub fn artifacts(spec: &Spec, body: &Value, metadata: &Value) -> Result<Vec<(String, Vec<u8>)>> {
    let b = &spec.bundle;
    let mut out = Vec::new();
    for a in &b.text_artifacts {
        out.push((a.path.clone(), at(body, &a.pointer).as_str().ok_or_else(|| error(format!("artifact text {}", a.pointer)))?.as_bytes().to_vec()));
    }
    for a in &b.list_artifacts {
        let list = at(body, &a.pointer).as_array().ok_or_else(|| error(format!("artifact list {}", a.pointer)))?;
        let s: String = list.iter().map(|v| v.as_str().map(|s| format!("- {s}\n")).ok_or("artifact list entries must be strings")).collect::<std::result::Result<_, _>>()?;
        out.push((a.path.clone(), s.into_bytes()));
    }
    for a in &b.json_artifacts {
        let mut m = serde_json::Map::new();
        for (k, p) in &a.fields {
            m.insert(k.clone(), at(body, p).clone());
        }
        for (k, v) in &a.constants {
            m.insert(k.clone(), v.clone());
        }
        out.push((a.path.clone(), serde_json::to_vec_pretty(&Value::Object(m))?));
    }
    out.push((b.metadata_artifact.clone(), serde_json::to_vec_pretty(metadata)?));
    Ok(out)
}
#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::spec::tests::fixture_spec;
    use std::{fs, path::PathBuf};
    pub fn body(summary: String) -> Value {
        json!({"schema":1,"summary":summary,"points":["A conditional observation."],"method":"Trace finite observations.","claims":[{"type":"primary","statement":"The value increases.","support":["Observed line"],"status":"observed"},{"type":"secondary","statement":"Some states are not recoverable.","support":["Discarded state"],"status":"missing"}],"review":{"state":"unreviewed"}})
    }
    fn spec(tag: &str, unit: &str, min: usize, max: usize) -> (PathBuf, Spec) {
        let base = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let d = base.join(format!("whale-contract-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        let s = Spec::load(&json!({"spec":fixture_spec(&d, unit, min, max)})).unwrap();
        (d, s)
    }
    #[test]
    fn word_unit_counts_alphanumeric_whitespace_tokens_not_characters() {
        assert_eq!(word_count("can't re-enter αβ 42 — !!! 🦈"), 4);
        let (d, s) = spec("words", "words", 40, 50);
        for n in [40, 43, 50] {
            assert_eq!(validate(&s, &body("модель ".repeat(n)), "short").unwrap(), n);
        }
        for n in [39, 51] {
            let e = validate(&s, &body("word ".repeat(n)), "short").unwrap_err().to_string();
            assert!(e.starts_with(LENGTH_GATE), "{e}");
        }
        fs::remove_dir_all(d).unwrap();
    }
    /// The bounds of the deployment's survey profile (4,000..5,000 words) on a fixture spec.
    fn length_gate_holds(s: &Spec) {
        let l = &s.output.length;
        for n in [l.min, l.min + 1, l.max] {
            assert_eq!(validate(s, &body("word ".repeat(n)), "short").unwrap(), n, "{n} must pass");
        }
        // Negative controls: one word below and one above the bounds are refused by the gate.
        for n in [l.min - 1, l.max + 1] {
            let e = validate(s, &body("word ".repeat(n)), "short").unwrap_err().to_string();
            assert!(e.starts_with(LENGTH_GATE), "{n}: {e}");
        }
        // As configured (project_oversized), the run first projects an oversized reply to a whole
        // sentence inside the bounds; without such a boundary it stays a length-gate quarantine.
        let mut v = body(format!("{}end. {}", "word ".repeat(l.max - 1), "tail ".repeat(2)));
        let trace = project(&mut v, l).unwrap().unwrap();
        assert_eq!((trace["input_count"].as_u64(), trace["output_count"].as_u64()), (Some(l.max as u64 + 2), Some(l.max as u64)));
        assert_eq!(validate(s, &v, "short").unwrap(), l.max);
        let e = project(&mut body("word ".repeat(l.max + 1)), l).unwrap_err().to_string();
        assert!(e.starts_with(LENGTH_GATE), "{e}");
        let mut short = body("word ".repeat(l.min - 1));
        assert!(project(&mut short, l).unwrap().is_none());
        assert!(validate(s, &short, "short").is_err());
    }
    #[test]
    fn survey_word_bounds_reject_3999_and_5001_words() {
        let (d, s) = spec("survey-words", "words", 4000, 5000);
        assert_eq!((s.output.length.min, s.output.length.max, s.output.length.profile.as_str()), (4000, 5000, "words_4000_5000_v1"));
        length_gate_holds(&s);
        fs::remove_dir_all(d).unwrap();
    }
    /// Opt-in check of a deployment specification (never part of this crate's data): set
    /// MOVA_WHALE_SPEC_CHECK to `PATH` or `PATH:SHA256`; without it the test reports a skip.
    #[test]
    fn deployment_spec_length_gate_when_configured() {
        let Some(arg) = std::env::var_os("MOVA_WHALE_SPEC_CHECK") else {
            eprintln!("SKIPPED: MOVA_WHALE_SPEC_CHECK not configured");
            return;
        };
        let arg = arg.to_string_lossy().to_string();
        let (path, sha) = match arg.rsplit_once(':') {
            Some((p, h)) if crate::util::valid_hash(h) => (p.to_string(), h.to_string()),
            _ => (arg.clone(), crate::util::file_hash(std::path::Path::new(&arg)).unwrap()),
        };
        let s = Spec::load(&json!({"spec":{"path":path,"sha256":sha}})).unwrap();
        eprintln!("checked {} {}: {}..{} {:?}", s.id, s.output.length.profile, s.output.length.min, s.output.length.max, s.output.length.unit);
        assert!(matches!(s.output.length.unit, Unit::Words));
        assert_eq!((s.output.length.min, s.output.length.max), (4000, 5000));
        // Only the length gate is exercised; the other configured fields fail later by design.
        for n in [3999usize, 5001] {
            let mut doc = json!({});
            set(&mut doc, &s.output.length.pointer, json!("word ".repeat(n)));
            let e = validate(&s, &doc, "").unwrap_err().to_string();
            assert!(e.starts_with(LENGTH_GATE), "{n}: {e}");
        }
        let mut doc = json!({});
        set(&mut doc, &s.output.length.pointer, json!("word ".repeat(4500)));
        let e = validate(&s, &doc, "").err().map(|e| e.to_string()).unwrap_or_default();
        assert!(!e.starts_with(LENGTH_GATE), "4500 words must pass the length gate: {e}");
    }
    fn set(doc: &mut Value, pointer: &str, v: Value) {
        let mut at = doc;
        let parts: Vec<&str> = pointer.trim_start_matches('/').split('/').collect();
        for (i, k) in parts.iter().enumerate() {
            if i + 1 == parts.len() {
                at[*k] = v;
                return;
            }
            if at[*k].is_null() {
                at[*k] = json!({});
            }
            at = &mut at[*k];
        }
    }
    #[test]
    fn scalar_unit_counts_unicode_scalars_and_equality_claims_fail() {
        let (d, s) = spec("scalars", "unicode_scalars", 40, 50);
        for n in [40, 45, 50] {
            assert_eq!(validate(&s, &body("🦈".repeat(n)), "short").unwrap(), n);
        }
        assert!(validate(&s, &body("é".repeat(51)), "").is_err());
        let mut v = body("é".repeat(45));
        v["review"]["state"] = json!("reviewed");
        assert!(validate(&s, &v, "").is_err());
        let mut v = body("é".repeat(45));
        v["summary"] = json!(format!("```{}", "é".repeat(44)));
        assert!(validate(&s, &v, "").is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn object_rules_require_distinct_entries_admitted_values_and_required_members() {
        let (d, s) = spec("objects", "unicode_scalars", 40, 50);
        let mut v = body("é".repeat(45));
        let mut extra = v["claims"][0].clone();
        extra["statement"] = json!("A narrower statement.");
        v["claims"].as_array_mut().unwrap().push(extra.clone());
        assert!(validate(&s, &v, "").is_ok());
        v["claims"].as_array_mut().unwrap().push(extra);
        assert!(validate(&s, &v, "").unwrap_err().to_string().contains("distinct"));
        let mut v = body("é".repeat(45));
        v["claims"][0]["type"] = json!("other");
        assert!(validate(&s, &v, "").unwrap_err().to_string().contains("must contain"));
        let mut v = body("é".repeat(45));
        v["claims"][0]["status"] = json!("proved");
        assert!(validate(&s, &v, "").is_err());
        assert!(validate(&s, &body(" ".repeat(45)), "").is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn long_verbatim_context_runs_are_quarantined() {
        let (d, s) = spec("verbatim", "unicode_scalars", 40, 50);
        let sentence = "one two three four five six seven eight nine ten eleven twelve thirteen fourteen";
        let mut v = body("é".repeat(45));
        v["method"] = json!(sentence);
        assert!(validate(&s, &v, sentence).unwrap_err().to_string().contains("overlap"));
        assert!(!copied_run(sentence, &v, 0));
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn projection_uses_exact_prefix_and_rejects_no_boundary_or_short_input() {
        let (d, s) = spec("projection", "words", 40, 50);
        let raw = format!("{}\n\n{}", "слово ".repeat(43), "tail ".repeat(9));
        let mut v = body(raw.clone());
        let trace = project(&mut v, &s.output.length).unwrap().unwrap();
        assert_eq!((trace["unit"].as_str(), trace["input_count"].as_u64(), trace["output_count"].as_u64()), (Some("words"), Some(52), Some(43)));
        assert_eq!(trace["boundary"], "paragraph");
        assert!(raw.starts_with(v["summary"].as_str().unwrap()));
        assert_eq!(validate(&s, &v, "short").unwrap(), 43);
        let mut sentence = body(format!("{}end. {}", "w ".repeat(44), "x ".repeat(20)));
        assert_eq!(project(&mut sentence, &s.output.length).unwrap().unwrap()["boundary"], "sentence");
        let mut short = body("word ".repeat(39));
        assert!(project(&mut short, &s.output.length).unwrap().is_none());
        assert!(project(&mut body("word ".repeat(60)), &s.output.length).is_err());
        fs::remove_dir_all(d).unwrap();
    }
    #[test]
    fn status_rewrite_retains_claims_and_artifacts_follow_configuration() {
        let (d, s) = spec("artifacts", "unicode_scalars", 40, 50);
        let mut v = body("é".repeat(45));
        rewrite_statuses(&s, &mut v).unwrap();
        assert_eq!(v["claims"][0]["status"], "hypothesis");
        assert_eq!(v["claims"][0]["teacher_claimed_status"], "observed");
        assert_eq!(v["claims"][1]["status"], "missing");
        let a = artifacts(&s, &v, &json!({"id":"x"})).unwrap();
        let names: Vec<_> = a.iter().map(|x| x.0.as_str()).collect();
        assert_eq!(names, ["summary.md", "method.md", "points.md", "claims.json", "metadata.json"]);
        assert_eq!(a[2].1, b"- A conditional observation.\n");
        fs::remove_dir_all(d).unwrap();
    }
}
