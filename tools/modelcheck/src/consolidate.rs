//! One Markdown file per groupant: merges the Markdown files of each `*.groupant/`
//! sidecar into its `model.md`, one block (H2 section) per former file, and moves every
//! repository reference to the new locations. Mechanics only: no model content changes.
//!
//! Former files that some JSON record selects by `section` stay verbatim (their own H2
//! sections remain addressable) under a `## <stem>` heading; other files become
//! `## <stem>` sections with their headings demoted below H2. A sidecar whose H2 anchors
//! would collide, or whose text ends inside a code fence, stays unmerged and is reported.
//! Hash-bound JSON references that were fresh are rebound to the new bytes (a section's
//! bytes for `model.md#Section`); stale ones keep their stale hash and are reported.
use crate::sections::{self, anchor, demote, h2_names, min_heading_level, split_reference};
use crate::sha;
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;

/// Line added under the title of a model.md whose former files had fresh hash-bound
/// references that were rebound.
pub const MIGRATION_LINE: &str = "Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.";
/// Fixed block order; other files follow by name.
pub const ORDER: [&str; 11] = [
    "descriptant",
    "model",
    "approximant",
    "invariant",
    "cost",
    "composition",
    "catalog",
    "models",
    "tests",
    "evidence",
    "prospective",
];
/// JSON keys whose `.md` strings name files outside the sidecar (evidence run
/// directories), so they are never rewritten.
const EXTERNAL_KEYS: [&str; 1] = ["file"];
const MODEL: &str = "model.md";

pub(crate) fn git_files(repo: &Path) -> Result<Vec<String>> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
        ])
        .output()?;
    ensure!(out.status.success(), "cannot list repository files");
    let mut files = BTreeSet::new();
    for bytes in out.stdout.split(|b| *b == 0).filter(|b| !b.is_empty()) {
        let path = std::str::from_utf8(bytes)?.to_string();
        if repo.join(&path).is_file() {
            files.insert(path);
        }
    }
    Ok(files.into_iter().collect())
}

pub(crate) fn parent(path: &str) -> &str {
    path.rsplit_once('/').map_or("", |(d, _)| d)
}
fn base(path: &str) -> &str {
    path.rsplit_once('/').map_or(path, |(_, b)| b)
}
/// Lexical join and normalization of a relative path; None for absolute or escaping paths.
pub(crate) fn join(dir: &str, rel: &str) -> Option<String> {
    if rel.starts_with('/') || rel.contains("://") {
        return None;
    }
    let mut parts: Vec<&str> = dir.split('/').filter(|p| !p.is_empty()).collect();
    for part in rel.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            p => parts.push(p),
        }
    }
    Some(parts.join("/"))
}
/// Relative path from directory `from` to repository path `to`.
pub(crate) fn relpath(from: &str, to: &str) -> String {
    let a: Vec<&str> = from.split('/').filter(|p| !p.is_empty()).collect();
    let b: Vec<&str> = to.split('/').filter(|p| !p.is_empty()).collect();
    let common = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let mut out: Vec<&str> = vec![".."; a.len() - common];
    out.extend(&b[common..]);
    out.join("/")
}

#[derive(Clone)]
struct Target {
    sidecar: String,
    stem: String,
    verbatim: bool,
}
impl Target {
    fn model(&self) -> String {
        format!("{}/{MODEL}", self.sidecar)
    }
}

/// A string literal inside a JSON text: byte span (with quotes), decoded value,
/// containing object (if any) and member key (if a member value).
pub(crate) struct Literal {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) value: String,
    pub(crate) object: Option<usize>,
    pub(crate) key: Option<String>,
}

/// Lists string values (not keys) of a JSON text with their object and key.
pub(crate) fn json_literals(text: &str) -> Result<Vec<Literal>> {
    enum Frame {
        Object {
            id: usize,
            key: Option<String>,
            expect_key: bool,
        },
        Array,
    }
    let bytes = text.as_bytes();
    let mut stack: Vec<Frame> = Vec::new();
    let mut out = Vec::new();
    let mut objects = 0;
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'{' => {
                stack.push(Frame::Object {
                    id: objects,
                    key: None,
                    expect_key: true,
                });
                objects += 1;
                i += 1;
            }
            b'[' => {
                stack.push(Frame::Array);
                i += 1;
            }
            b'}' | b']' => {
                stack.pop();
                i += 1;
            }
            b',' => {
                if let Some(Frame::Object {
                    expect_key, key, ..
                }) = stack.last_mut()
                {
                    *expect_key = true;
                    *key = None;
                }
                i += 1;
            }
            b'"' => {
                let start = i;
                i += 1;
                while i < bytes.len() && bytes[i] != b'"' {
                    i += if bytes[i] == b'\\' { 2 } else { 1 };
                }
                ensure!(i < bytes.len(), "unterminated JSON string");
                i += 1;
                let value: String = serde_json::from_str(&text[start..i])?;
                match stack.last_mut() {
                    Some(Frame::Object {
                        key, expect_key, ..
                    }) if *expect_key => {
                        *key = Some(value);
                        *expect_key = false;
                    }
                    Some(Frame::Object { id, key, .. }) => out.push(Literal {
                        start,
                        end: i,
                        value,
                        object: Some(*id),
                        key: key.clone(),
                    }),
                    _ => out.push(Literal {
                        start,
                        end: i,
                        value,
                        object: None,
                        key: None,
                    }),
                }
            }
            _ => i += 1,
        }
    }
    Ok(out)
}

/// A path-like token ending in `.md` inside a string: span of the path, and the
/// suffix span (query and/or fragment) that follows it.
struct Token {
    start: usize,
    end: usize,
    suffix_end: usize,
}
fn md_tokens(s: &str) -> Vec<Token> {
    let b = s.as_bytes();
    let path_char = |c: u8| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'.' | b'/' | b'-');
    let word_char = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(pos) = s[from..].find(".md") {
        let dot = from + pos;
        let end = dot + 3;
        from = end;
        if b.get(end).is_some_and(|c| word_char(*c)) {
            continue;
        }
        let mut start = dot;
        while start > 0 && path_char(b[start - 1]) {
            start -= 1;
        }
        if start == dot || out.last().is_some_and(|t: &Token| t.suffix_end > start) {
            continue;
        }
        let mut suffix_end = end;
        if b.get(suffix_end) == Some(&b'?') {
            suffix_end += 1;
            while b.get(suffix_end).is_some_and(|c| {
                c.is_ascii_alphanumeric() || matches!(c, b'_' | b'=' | b'&' | b'-')
            }) {
                suffix_end += 1;
            }
        }
        if b.get(suffix_end) == Some(&b'#') {
            suffix_end += 1;
            while b
                .get(suffix_end)
                .is_some_and(|c| c.is_ascii_alphanumeric() || matches!(c, b'_' | b'-'))
            {
                suffix_end += 1;
            }
        }
        out.push(Token {
            start,
            end,
            suffix_end,
        });
    }
    out
}

#[derive(Default)]
struct Plan {
    /// Former file → its block target, for merged sidecars only.
    targets: BTreeMap<String, Target>,
    /// Sidecar → ordered former files.
    merged: BTreeMap<String, Vec<String>>,
    unmerged: BTreeMap<String, String>,
    /// Fold-in module prospective file → sidecar.
    foldins: BTreeMap<String, String>,
    /// Verbatim former file → its H2 headings renamed to stay unique (`X` → `X (stem)`).
    renames: BTreeMap<String, BTreeMap<String, String>>,
}

enum Mode {
    Json,
    Markdown,
}

/// Resolves a token path written in `dir` to a former file.
fn resolve<'p>(
    plan: &'p Plan,
    dir: &str,
    path: &str,
    repo_relative: bool,
) -> Option<(String, &'p Target)> {
    let local = join(dir, path).and_then(|p| plan.targets.get_key_value(&p));
    let global = if repo_relative && path.contains('/') {
        join("", path).and_then(|p| plan.targets.get_key_value(&p))
    } else {
        None
    };
    local.or(global).map(|(k, t)| (k.clone(), t))
}

/// Rewrites references to former files inside one string. `fragment` picks the
/// fragment added to a target without one; returns the new text and rewrite count.
fn rewrite(
    plan: &Plan,
    text: &str,
    dir: &str,
    mode: Mode,
    fragment: &dyn Fn(&str, &Target) -> Option<String>,
) -> (String, usize) {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    let mut count = 0;
    for t in md_tokens(text) {
        let path = &text[t.start..t.end];
        let suffix = &text[t.end..t.suffix_end];
        let qualifies = match mode {
            Mode::Json => true,
            Mode::Markdown => {
                path.contains(".groupant/")
                    || text[..t.start].ends_with("](")
                    || text[..t.start].ends_with("](<")
                    || text[..t.start].ends_with("]: ")
            }
        };
        let repo_relative = matches!(mode, Mode::Json) || path.contains(".groupant/");
        let Some((former, target)) = qualifies
            .then(|| resolve(plan, dir, path, repo_relative))
            .flatten()
        else {
            continue;
        };
        // Replace the basename by the relative location of model.md from the former
        // file's directory: "model.md", or "mod.rs.groupant/model.md" for a fold-in.
        let keep = &path[..path.len() - base(path).len()];
        let mut new = format!("{keep}{}", relpath(parent(&former), &target.model()));
        if suffix.contains('#') {
            new.push_str(suffix);
        } else {
            new.push_str(suffix);
            if let Some(f) = fragment(&former, target) {
                new.push('#');
                new.push_str(&f);
            }
        }
        if new != text[t.start..t.suffix_end] {
            count += 1;
        }
        out.push_str(&text[last..t.start]);
        out.push_str(&new);
        last = t.suffix_end;
    }
    out.push_str(&text[last..]);
    (out, count)
}

fn md_fragment(_former: &str, target: &Target) -> Option<String> {
    Some(anchor(&target.stem))
}

/// Renames plain H2 headings outside fences (`## old` → `## new`); other bytes unchanged.
fn rename_h2(text: &str, renames: &BTreeMap<String, String>) -> String {
    let mut out = String::with_capacity(text.len() + 32);
    for l in sections::lines(text) {
        let raw = &text[l.start..l.end];
        match (!l.fenced)
            .then(|| l.text.strip_prefix("## "))
            .flatten()
            .and_then(|n| renames.get(n))
        {
            Some(new) => {
                out.push_str("## ");
                out.push_str(new);
                out.push_str(&raw[l.text.len()..]);
            }
            None => out.push_str(raw),
        }
    }
    out
}

/// Reverse of `rename_h2` for one block: `## X (stem)` outside fences becomes `## X`.
fn unrename_h2(text: &str, stem: &str) -> String {
    let suffix = format!(" ({stem})");
    let mut out = String::with_capacity(text.len());
    for l in sections::lines(text) {
        let raw = &text[l.start..l.end];
        match (!l.fenced)
            .then(|| {
                l.text
                    .strip_prefix("## ")
                    .and_then(|n| n.strip_suffix(suffix.as_str()))
            })
            .flatten()
        {
            Some(old) => {
                out.push_str("## ");
                out.push_str(old);
                out.push_str(&raw[l.text.len()..]);
            }
            None => out.push_str(raw),
        }
    }
    out
}

fn order_key(stem: &str) -> (usize, String) {
    (
        ORDER.iter().position(|s| *s == stem).unwrap_or(ORDER.len()),
        stem.to_string(),
    )
}

/// Runs the migration below `dir` ("." for the whole repository). Writes files unless
/// `dry_run`; never deletes: the report's `remove` list is for `git rm`.
pub fn consolidate(repo: &Path, dir: &str, dry_run: bool) -> Result<Value> {
    let files = git_files(repo)?;
    let prefix = match dir.trim_end_matches('/') {
        "." | "" => String::new(),
        d => format!("{d}/"),
    };
    let read = |p: &str| -> Result<String> {
        fs::read_to_string(repo.join(p)).with_context(|| format!("cannot read UTF-8 {p}"))
    };
    // 1. Sidecars with Markdown files and module prospective fold-ins.
    let mut sidecars: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for f in &files {
        if f.ends_with(".md") && parent(f).ends_with(".groupant") && f.starts_with(&prefix) {
            sidecars
                .entry(parent(f).to_string())
                .or_default()
                .push(f.clone());
        }
    }
    let mut plan = Plan::default();
    for f in &files {
        let d = parent(f);
        let side = format!("{d}/mod.rs.groupant");
        if base(f) == "prospective.md" && !d.ends_with(".groupant") && sidecars.contains_key(&side)
        {
            plan.foldins.insert(f.clone(), side);
        }
    }
    let mut texts: BTreeMap<String, String> = BTreeMap::new();
    for f in sidecars.values().flatten().chain(plan.foldins.keys()) {
        texts.insert(f.clone(), read(f)?);
    }
    // 2. Verbatim files: selected by `section` from some JSON record.
    let json_files: Vec<&String> = files
        .iter()
        .filter(|f| f.ends_with(".json") && base(f) != "syntax.json")
        .collect();
    let mut json_texts: BTreeMap<String, String> = BTreeMap::new();
    for f in &json_files {
        let bytes = fs::read(repo.join(f))?;
        if bytes.windows(3).any(|w| w == b".md") {
            json_texts.insert((*f).clone(), String::from_utf8(bytes)?);
        }
    }
    let mut verbatim: BTreeSet<String> = BTreeSet::new();
    let mut literals: BTreeMap<String, Vec<Literal>> = BTreeMap::new();
    for (f, text) in &json_texts {
        let lits = json_literals(text).with_context(|| format!("cannot scan JSON {f}"))?;
        let mut by_object: BTreeMap<usize, BTreeMap<&str, &str>> = BTreeMap::new();
        for l in &lits {
            if let (Some(o), Some(k)) = (l.object, l.key.as_deref()) {
                by_object.entry(o).or_default().insert(k, &l.value);
            }
        }
        for fields in by_object.values() {
            let (Some(path), Some(section)) = (fields.get("path"), fields.get("section")) else {
                continue;
            };
            for candidate in [join(parent(f), path), join("", path)]
                .into_iter()
                .flatten()
            {
                if let Some(text) = texts.get(&candidate)
                    && sections::heading_count(text, section) == 1
                {
                    verbatim.insert(candidate);
                }
            }
        }
        literals.insert(f.clone(), lits);
    }
    // 3. Mergeability per sidecar.
    let mut overflow_files = BTreeMap::new();
    for (side, members) in &sidecars {
        let foldin: Vec<&String> = plan
            .foldins
            .iter()
            .filter(|(_, s)| *s == side)
            .map(|(f, _)| f)
            .collect();
        if members.len() == 1 && base(&members[0]) == MODEL && foldin.is_empty() {
            continue;
        }
        let mut reason = None;
        let mut anchors: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for f in members.iter().chain(foldin.iter().copied()) {
            let text = &texts[f];
            if sections::unclosed_fence(text) {
                reason = Some(format!("{f} ends inside a code fence"));
            }
            let stem = base(f).trim_end_matches(".md");
            let stem = if foldin.contains(&f) {
                "prospective"
            } else {
                stem
            };
            anchors
                .entry(anchor(stem))
                .or_default()
                .insert(format!("{stem} heading"));
            if verbatim.contains(f) {
                if foldin.contains(&f) || (stem == "prospective" && !foldin.is_empty()) {
                    reason = Some(format!(
                        "{f} is section-referenced and needs a prospective fold-in"
                    ));
                }
                for name in h2_names(text) {
                    anchors.entry(anchor(name)).or_default().insert(f.clone());
                }
            }
            if foldin.contains(&f) && text.contains("](") {
                reason = Some(format!("{f} has links that would need rebasing"));
            }
        }
        if let Some(r) = reason {
            plan.unmerged.insert(side.clone(), r);
            continue;
        }
        // The fold-in shares the prospective slot and follows the sidecar's own file.
        let mut list: Vec<String> = members
            .iter()
            .chain(foldin.iter().copied())
            .cloned()
            .collect();
        list.sort_by_key(|f| {
            let folded = plan.foldins.contains_key(f);
            let stem = if folded {
                "prospective"
            } else {
                base(f).trim_end_matches(".md")
            };
            (order_key(stem), folded)
        });
        // Colliding anchors: block headings (`## stem`) never move; among verbatim H2
        // headings the first in block order keeps its name unless a block heading holds
        // the anchor, and every other one becomes `X (stem)` of its own former file.
        let mut renames: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
        for (a, from) in &anchors {
            if from.len() < 2 {
                continue;
            }
            let block_heading = from.iter().any(|s| s.ends_with(" heading"));
            let mut keep_first = !block_heading;
            for f in list.iter().filter(|f| from.contains(*f)) {
                for name in h2_names(&texts[f]) {
                    if anchor(name) != *a {
                        continue;
                    }
                    if keep_first {
                        keep_first = false;
                        continue;
                    }
                    let stem = base(f).trim_end_matches(".md");
                    renames
                        .entry(f.clone())
                        .or_default()
                        .insert(name.to_string(), format!("{name} ({stem})"));
                }
            }
        }
        // After renaming every anchor must be unique, or the sidecar stays unmerged.
        let mut after: BTreeMap<String, usize> = BTreeMap::new();
        for f in &list {
            let stem = if plan.foldins.contains_key(f) {
                "prospective"
            } else {
                base(f).trim_end_matches(".md")
            };
            if !(plan.foldins.contains_key(f)
                && list
                    .iter()
                    .any(|g| !plan.foldins.contains_key(g) && base(g) == "prospective.md"))
            {
                *after.entry(anchor(stem)).or_default() += 1;
            }
            if verbatim.contains(f) {
                let mut seen = BTreeSet::new();
                for name in h2_names(&texts[f]) {
                    let new = renames
                        .get(f)
                        .and_then(|r| r.get(name))
                        .map_or(name, String::as_str);
                    // a heading repeated inside one former file was already ambiguous there
                    if seen.insert(anchor(new)) {
                        *after.entry(anchor(new)).or_default() += 1;
                    }
                }
            }
        }
        let clashes: Vec<String> = after
            .iter()
            .filter(|(_, n)| **n > 1)
            .map(|(a, _)| format!("#{a}"))
            .collect();
        if !clashes.is_empty() {
            plan.unmerged.insert(
                side.clone(),
                format!("H2 anchor collision after renaming: {}", clashes.join("; ")),
            );
            continue;
        }
        plan.renames.extend(renames);
        for f in &list {
            let stem = if plan.foldins.contains_key(f) {
                "prospective".to_string()
            } else {
                base(f).trim_end_matches(".md").to_string()
            };
            plan.targets.insert(
                f.clone(),
                Target {
                    sidecar: side.clone(),
                    stem,
                    verbatim: verbatim.contains(f),
                },
            );
        }
        plan.merged.insert(side.clone(), list);
    }
    plan.foldins.retain(|f, _| plan.targets.contains_key(f));
    // 4. JSON rewriting and hash bindings.
    let mut rebinds: Vec<Value> = Vec::new();
    let mut stale: Vec<Value> = Vec::new();
    let mut whole_verbatim: Vec<Value> = Vec::new();
    let mut by_key: BTreeMap<String, usize> = BTreeMap::new();
    let mut skipped_external = 0usize;
    let mut migrated: BTreeSet<String> = BTreeSet::new();
    // (json file, literal index) → new value, and pending hash rebinds.
    let mut json_edits: BTreeMap<String, BTreeMap<usize, String>> = BTreeMap::new();
    struct Pending {
        file: String,
        sha_literal: usize,
        reference: String,
        old_sha: String,
    }
    let mut pending: Vec<Pending> = Vec::new();
    for (f, lits) in &literals {
        let dir = parent(f);
        let mut fields: BTreeMap<usize, BTreeMap<&str, usize>> = BTreeMap::new();
        for (i, l) in lits.iter().enumerate() {
            if let (Some(o), Some(k)) = (l.object, l.key.as_deref()) {
                fields.entry(o).or_default().insert(k, i);
            }
        }
        for (i, l) in lits.iter().enumerate() {
            if !l.value.contains(".md") {
                continue;
            }
            let key = l.key.clone().unwrap_or_default();
            let siblings = l.object.and_then(|o| fields.get(&o));
            let sibling = |k: &str| {
                siblings
                    .and_then(|s| s.get(k))
                    .map(|j| lits[*j].value.as_str())
            };
            let whole = md_tokens(&l.value)
                .first()
                .is_some_and(|t| t.start == 0 && t.suffix_end == l.value.len());
            let hashed = whole && key == "path" && sibling("sha256").is_some();
            let fragment = |former: &str, target: &Target| -> Option<String> {
                if !target.verbatim {
                    return Some(target.stem.clone());
                }
                // Verbatim: a hash-bound record whose section names one of the former
                // file's headings is narrowed to that section.
                let section = sibling("section")?;
                (hashed && sections::heading_count(&texts[former], section) == 1).then(|| {
                    plan.renames
                        .get(former)
                        .and_then(|r| r.get(section))
                        .cloned()
                        .unwrap_or_else(|| section.to_string())
                })
            };
            if EXTERNAL_KEYS.contains(&key.as_str()) {
                if md_tokens(&l.value)
                    .iter()
                    .any(|t| resolve(&plan, dir, &l.value[t.start..t.end], true).is_some())
                {
                    skipped_external += 1;
                }
                continue;
            }
            let (new, count) = rewrite(&plan, &l.value, dir, Mode::Json, &fragment);
            let resolved = whole
                .then(|| {
                    let t = &md_tokens(&l.value)[0];
                    resolve(&plan, dir, &l.value[t.start..t.end], true)
                })
                .flatten();
            if count > 0 {
                *by_key.entry(key.clone()).or_default() += count;
                json_edits
                    .entry(f.clone())
                    .or_default()
                    .insert(i, new.clone());
            }
            if let (true, Some((former, target))) = (hashed, resolved) {
                let sha_index = siblings
                    .and_then(|s| s.get("sha256"))
                    .copied()
                    .context("sha256 literal")?;
                let old_sha = lits[sha_index].value.clone();
                let record = json!({"file": f, "path": l.value, "new_path": new, "former": former});
                if sha(texts[&former].as_bytes()) == old_sha {
                    migrated.insert(target.sidecar.clone());
                    if target.verbatim && !new.contains('#') {
                        whole_verbatim.push(record.clone());
                    }
                    let reference = match split_reference(&new).1 {
                        Some(section) => format!("{}#{section}", target.model()),
                        None => target.model(),
                    };
                    pending.push(Pending {
                        file: f.clone(),
                        sha_literal: sha_index,
                        reference,
                        old_sha,
                    });
                } else {
                    stale.push(record);
                }
            }
        }
    }
    // A `section` field naming a renamed heading follows the rename.
    let mut section_renames = 0usize;
    for (f, lits) in &literals {
        let dir = parent(f);
        let mut fields: BTreeMap<usize, BTreeMap<&str, usize>> = BTreeMap::new();
        for (i, l) in lits.iter().enumerate() {
            if let (Some(o), Some(k)) = (l.object, l.key.as_deref()) {
                fields.entry(o).or_default().insert(k, i);
            }
        }
        for keys in fields.values() {
            let (Some(p), Some(s)) = (keys.get("path"), keys.get("section")) else {
                continue;
            };
            let path = split_reference(&lits[*p].value).0;
            let Some((former, _)) = resolve(&plan, dir, path, true) else {
                continue;
            };
            if let Some(new) = plan
                .renames
                .get(&former)
                .and_then(|r| r.get(&lits[*s].value))
            {
                json_edits
                    .entry(f.clone())
                    .or_default()
                    .insert(*s, new.clone());
                section_renames += 1;
            }
        }
    }
    // 5. Merged model.md texts.
    let mut outputs: BTreeMap<String, String> = BTreeMap::new();
    let mut verbatim_count = 0;
    let mut wrapped_count = 0;
    for (side, list) in &plan.merged {
        let title = fs::read(repo.join(side).join("manifest.json"))
            .ok()
            .and_then(|b| serde_json::from_slice::<Value>(&b).ok())
            .and_then(|m| m["id"].as_str().map(str::to_string))
            .unwrap_or_else(|| base(side).to_string());
        let mut out = format!("# {title}\n\n");
        if migrated.contains(side) {
            out.push_str(MIGRATION_LINE);
            out.push_str("\n\n");
        }
        let has_own_prospective = list
            .iter()
            .any(|f| !plan.foldins.contains_key(f) && base(f) == "prospective.md");
        let has_foldin = list.iter().any(|f| plan.foldins.contains_key(f));
        for f in list {
            let target = &plan.targets[f];
            let (text, _) = rewrite(&plan, &texts[f], parent(f), Mode::Markdown, &md_fragment);
            let foldin = plan.foldins.contains_key(f);
            let shared = has_own_prospective && has_foldin && target.stem == "prospective";
            if !(foldin && has_own_prospective) {
                out.push_str(&format!("## {}\n\n", target.stem));
            }
            if shared {
                out.push_str(if foldin {
                    "### Module prospective\n\n"
                } else {
                    "### Groupant prospective\n\n"
                });
            }
            let block = if target.verbatim {
                verbatim_count += 1;
                match plan.renames.get(f) {
                    Some(r) => rename_h2(&text, r),
                    None => text,
                }
            } else {
                wrapped_count += 1;
                let floor: usize = if shared { 4 } else { 3 };
                let shift = min_heading_level(&text).map_or(0, |m| floor.saturating_sub(m));
                let (demoted, overflow) = demote(&text, shift);
                if overflow > 0 {
                    overflow_files.insert(f.clone(), overflow);
                }
                demoted
            };
            out.push_str(&block);
            if !block.ends_with('\n') {
                out.push('\n');
            }
            out.push('\n');
        }
        while out.ends_with("\n\n") {
            out.pop();
        }
        outputs.insert(format!("{side}/{MODEL}"), out);
    }
    // 6. Other Markdown files: references to former files.
    let removed: BTreeSet<&String> = plan.targets.keys().filter(|f| base(f) != MODEL).collect();
    let mut md_edits: BTreeMap<String, String> = BTreeMap::new();
    for f in files.iter().filter(|f| f.ends_with(".md")) {
        if plan.targets.contains_key(f) {
            continue;
        }
        let text = match texts.get(f) {
            Some(t) => t.clone(),
            None => match fs::read_to_string(repo.join(f)) {
                Ok(t) => t,
                Err(_) => continue,
            },
        };
        if !text.contains(".md") {
            continue;
        }
        let (new, count) = rewrite(&plan, &text, parent(f), Mode::Markdown, &md_fragment);
        if count > 0 {
            md_edits.insert(f.clone(), new);
        }
    }
    // 7. Hash rebinds against the final texts.
    for p in &pending {
        let (file, section) = split_reference(&p.reference);
        let text = outputs
            .get(file)
            .context("rebind target is a merged model.md")?;
        let bytes = match section {
            Some(s) => sections::section_bytes(text, s)
                .with_context(|| format!("rebind section {s} missing in {file}"))?,
            None => text.as_str(),
        };
        let new_sha = sha(bytes.as_bytes());
        json_edits
            .entry(p.file.clone())
            .or_default()
            .insert(p.sha_literal, new_sha.clone());
        rebinds.push(json!({"file":p.file,"reference":p.reference,"old_sha256":p.old_sha,"new_sha256":new_sha}));
    }
    // 8. Hash-bound references to other files this run changes. A Markdown file whose
    // only change is moved reference targets is rebound when its binding was fresh; a
    // binding to a rewritten JSON file is reported, never refreshed.
    let rebound: BTreeSet<(String, usize)> = pending
        .iter()
        .map(|p| (p.file.clone(), p.sha_literal))
        .collect();
    let json_changed: BTreeSet<String> = json_edits.keys().cloned().collect();
    let mut newly_stale: Vec<Value> = Vec::new();
    let mut relinked: Vec<Value> = Vec::new();
    for (f, lits) in &literals {
        let mut fields: BTreeMap<usize, BTreeMap<&str, usize>> = BTreeMap::new();
        for (i, l) in lits.iter().enumerate() {
            if let (Some(o), Some(k)) = (l.object, l.key.as_deref()) {
                fields.entry(o).or_default().insert(k, i);
            }
        }
        for keys in fields.values() {
            let (Some(p), Some(h)) = (keys.get("path"), keys.get("sha256")) else {
                continue;
            };
            if rebound.contains(&(f.clone(), *h)) {
                continue;
            }
            let path = &lits[*p].value;
            for candidate in [join(parent(f), path), join("", path)]
                .into_iter()
                .flatten()
            {
                let new = match (md_edits.get(&candidate), outputs.get(&candidate)) {
                    (Some(n), _) | (None, Some(n)) => n,
                    _ if json_changed.contains(&candidate) => {
                        let old = fs::read(repo.join(&candidate)).unwrap_or_default();
                        if sha(&old) == lits[*h].value {
                            newly_stale.push(json!({"file": f, "path": path, "target": candidate}));
                        } else {
                            stale.push(json!({"file": f, "path": path, "note": "already stale; bound JSON is rewritten"}));
                        }
                        break;
                    }
                    _ => continue,
                };
                let old = fs::read(repo.join(&candidate)).unwrap_or_default();
                if sha(&old) == lits[*h].value && old != new.as_bytes() {
                    let new_sha = sha(new.as_bytes());
                    json_edits
                        .entry(f.clone())
                        .or_default()
                        .insert(*h, new_sha.clone());
                    relinked.push(json!({"file": f, "path": path, "old_sha256": lits[*h].value, "new_sha256": new_sha}));
                }
                break;
            }
        }
    }
    // JSON → JSON bindings: render the bound files first, then rebind fresh references
    // to them in files that are not themselves bound (one level; deeper chains are reported).
    let mut json_relinks: Vec<(String, usize, String, String)> = Vec::new();
    {
        let mut kept = Vec::new();
        for item in newly_stale.drain(..) {
            let (f, target) = (
                item["file"].as_str().unwrap_or_default().to_string(),
                item["target"].as_str().unwrap_or_default().to_string(),
            );
            let lits = &literals[&f];
            let index = lits.iter().enumerate().find(|(_, l)| {
                l.key.as_deref() == Some("sha256")
                    && lits.iter().any(|p| {
                        p.object == l.object
                            && p.key.as_deref() == Some("path")
                            && p.value == item["path"].as_str().unwrap_or_default()
                    })
            });
            let fresh = fs::read(repo.join(&target)).map(|b| sha(&b));
            match index {
                Some((i, l)) if fresh.as_deref().ok() == Some(l.value.as_str()) => {
                    json_relinks.push((f, i, target, l.value.clone()))
                }
                _ => kept.push(item),
            }
        }
        newly_stale = kept;
    }
    let render = |f: &str, edits: &BTreeMap<usize, String>| -> Result<String> {
        let text = &json_texts[f];
        let lits = &literals[f];
        let mut out = String::with_capacity(text.len());
        let mut last = 0;
        for (i, value) in edits {
            out.push_str(&text[last..lits[*i].start]);
            out.push_str(&serde_json::to_string(value)?);
            last = lits[*i].end;
        }
        out.push_str(&text[last..]);
        serde_json::from_str::<Value>(&out)
            .with_context(|| format!("rewritten JSON {f} does not parse"))?;
        Ok(out)
    };
    let mut json_outputs: BTreeMap<String, String> = BTreeMap::new();
    for (f, edits) in &json_edits {
        json_outputs.insert(f.clone(), render(f, edits)?);
    }
    let bound: BTreeSet<&String> = json_relinks.iter().map(|r| &r.2).collect();
    for (f, i, target, old_sha) in &json_relinks {
        if bound.contains(f) {
            newly_stale
                .push(json!({"file": f, "target": target, "reason": "chained JSON binding"}));
            continue;
        }
        let new_sha = sha(json_outputs[target].as_bytes());
        json_edits
            .entry(f.clone())
            .or_default()
            .insert(*i, new_sha.clone());
        json_outputs.insert(f.clone(), render(f, &json_edits[f])?);
        relinked
            .push(json!({"file": f, "path": target, "old_sha256": old_sha, "new_sha256": new_sha}));
    }
    if !dry_run {
        for (f, text) in outputs.iter().chain(&md_edits).chain(&json_outputs) {
            fs::write(repo.join(f), text)?;
        }
    }
    Ok(json!({
        "schema": 1,
        "kind": "groupant_consolidation",
        "dir": dir,
        "dry_run": dry_run,
        "sidecars_with_markdown": sidecars.len(),
        "merged_sidecars": plan.merged.len(),
        "unmerged": plan.unmerged,
        "former_files": plan.targets.len(),
        "verbatim_blocks": verbatim_count,
        "wrapped_blocks": wrapped_count,
        "foldins": plan.foldins,
        "heading_overflow": overflow_files,
        "markdown_files_changed": md_edits.keys().collect::<Vec<_>>(),
        "json_files_changed": json_outputs.keys().collect::<Vec<_>>(),
        "json_rewrites_by_key": by_key,
        "external_keys_skipped": skipped_external,
        "hash_rebinds": rebinds,
        "stale_hash_references_kept": stale,
        "whole_file_hash_rebinds_to_verbatim": whole_verbatim,
        "migration_lines": migrated,
        "heading_renames": plan.renames,
        "section_field_renames": section_renames,
        "newly_stale_unhandled": newly_stale,
        "rebound_after_reference_rewrite": relinked,
        "remove": removed,
        "verification_inferred": false,
    }))
}

/// Reads `rev:path` blobs through one `git cat-file --batch` process.
fn git_blobs(repo: &Path, rev: &str, paths: &[String]) -> Result<BTreeMap<String, String>> {
    use std::io::Write;
    let mut child = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["cat-file", "--batch"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let mut input = String::new();
    for p in paths {
        input.push_str(&format!("{rev}:{p}\n"));
    }
    let mut stdin = child.stdin.take().context("git stdin")?;
    let writer = std::thread::spawn(move || stdin.write_all(input.as_bytes()));
    let out = child.wait_with_output()?;
    writer
        .join()
        .map_err(|_| anyhow::anyhow!("git writer panicked"))??;
    ensure!(out.status.success(), "git cat-file failed");
    let mut blobs = BTreeMap::new();
    let mut at = 0;
    for p in paths {
        let header_end = at
            + out.stdout[at..]
                .iter()
                .position(|b| *b == b'\n')
                .context("git header")?;
        let header = std::str::from_utf8(&out.stdout[at..header_end])?;
        let size: usize = header
            .rsplit(' ')
            .next()
            .and_then(|s| s.parse().ok())
            .with_context(|| format!("missing blob {rev}:{p}: {header}"))?;
        let start = header_end + 1;
        blobs.insert(
            p.clone(),
            String::from_utf8(out.stdout[start..start + size].to_vec())?,
        );
        at = start + size + 1;
    }
    Ok(blobs)
}

/// Replaces every `.md` path token (with its query/fragment) by one marker, so texts
/// compare equal when they differ only in where their Markdown references point.
fn normalize(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut last = 0;
    for t in md_tokens(text) {
        out.push_str(&text[last..t.start]);
        out.push('\u{1}');
        last = t.suffix_end;
    }
    out.push_str(&text[last..]);
    out
}

/// Independent gate for a finished migration against `base`: every former file's text
/// (modulo `.md` reference targets and heading demotion) occurs exactly once in its
/// sidecar's model.md, model.md holds nothing else but the scaffold (title, migration
/// line, block headings, blank lines), its H2 anchors are unique, and no other Markdown
/// file is left in a merged sidecar.
pub fn check(repo: &Path, base_rev: &str) -> Result<Value> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["diff", "--name-status", "--no-renames", "-z", base_rev])
        .output()?;
    ensure!(out.status.success(), "git diff failed");
    let fields: Vec<String> = out
        .stdout
        .split(|b| *b == 0)
        .filter(|b| !b.is_empty())
        .map(|b| String::from_utf8_lossy(b).into_owned())
        .collect();
    let mut originals: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for pair in fields.chunks(2) {
        let [status, path] = pair else { continue };
        let dir = parent(path);
        let side = if dir.ends_with(".groupant") {
            dir.to_string()
        } else if base(path) == "prospective.md" {
            format!("{dir}/mod.rs.groupant")
        } else {
            continue;
        };
        let former =
            (status == "D" && path.ends_with(".md")) || (status == "M" && base(path) == MODEL);
        if former {
            originals.entry(side).or_default().push(path.clone());
        }
    }
    // A sidecar was consolidated when some former file left it; a model.md changed
    // only by moved links elsewhere is not a merge.
    originals.retain(|_, list| list.iter().any(|p| base(p) != MODEL));
    let all: Vec<String> = originals.values().flatten().cloned().collect();
    let blobs = git_blobs(repo, base_rev, &all)?;
    let files = git_files(repo)?;
    let mut failures: Vec<Value> = Vec::new();
    for (side, list) in &originals {
        let model_path = format!("{side}/{MODEL}");
        let Ok(new) = fs::read_to_string(repo.join(&model_path)) else {
            failures.push(json!({"sidecar":side,"failure":"model.md missing"}));
            continue;
        };
        let leftovers: Vec<&String> = files
            .iter()
            .filter(|f| parent(f) == side && f.ends_with(".md") && base(f) != MODEL)
            .collect();
        if !leftovers.is_empty() {
            failures.push(
                json!({"sidecar":side,"failure":"other Markdown files remain","files":leftovers}),
            );
        }
        // Reverse the merge block by block: every former file owns the block under
        // `## <stem>`; its body must equal the former text, possibly demoted.
        let stem_of = |p: &str| -> String {
            if parent(p).ends_with(".groupant") {
                base(p).trim_end_matches(".md").to_string()
            } else {
                "prospective".to_string()
            }
        };
        let stems: BTreeSet<String> = list.iter().map(|p| stem_of(p)).collect();
        let all_lines = sections::lines(&new);
        let mut heads: Vec<(usize, String)> = Vec::new();
        for (i, l) in all_lines.iter().enumerate() {
            if let Some(name) = l
                .text
                .strip_prefix("## ")
                .filter(|n| !l.fenced && stems.contains(*n))
            {
                heads.push((i, name.to_string()));
            }
        }
        let preamble_end = heads.first().map_or(all_lines.len(), |h| h.0);
        for (i, l) in all_lines[..preamble_end].iter().enumerate() {
            let ok = l.text.trim().is_empty()
                || (i == 0 && l.text.starts_with("# "))
                || l.text == MIGRATION_LINE;
            if !ok {
                failures.push(json!({"sidecar":side,"failure":"text not traced to a former file","line":l.text}));
            }
        }
        // a verbatim block may carry headings renamed `X (stem)` to stay unique
        let matches = |body: &str, stem: &str, original: &str| -> bool {
            let body = normalize(unrename_h2(body, stem).trim_end());
            (0..=3).any(|shift| normalize(demote(original, shift).0.trim_end()) == body)
        };
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for (k, (line, stem)) in heads.iter().enumerate() {
            if !seen.insert(stem.clone()) {
                failures
                    .push(json!({"sidecar":side,"failure":"duplicate block heading","stem":stem}));
                continue;
            }
            let start = all_lines[*line].end;
            let end = heads.get(k + 1).map_or(new.len(), |h| all_lines[h.0].start);
            let body = new[start..end]
                .strip_prefix('\n')
                .unwrap_or(&new[start..end]);
            let owners: Vec<&String> = list.iter().filter(|p| stem_of(p) == *stem).collect();
            let ok = match owners.as_slice() {
                [one] => matches(body, stem, &blobs[*one]),
                [first, second] => {
                    let (own, module) = if parent(first).ends_with(".groupant") {
                        (first, second)
                    } else {
                        (second, first)
                    };
                    body.strip_prefix("### Groupant prospective\n\n")
                        .and_then(|b| b.split_once("\n### Module prospective\n\n"))
                        .is_some_and(|(a, b)| {
                            matches(a, stem, &blobs[*own]) && matches(b, stem, &blobs[*module])
                        })
                }
                _ => false,
            };
            if !ok {
                failures.push(json!({"sidecar":side,"failure":"block differs from its former files","stem":stem,"originals":owners}));
            }
        }
        for stem in stems.difference(&seen) {
            failures.push(json!({"sidecar":side,"failure":"former file has no block","stem":stem}));
        }
        let mut anchors: BTreeMap<String, usize> = BTreeMap::new();
        for name in h2_names(&new) {
            *anchors.entry(anchor(name)).or_default() += 1;
        }
        let duplicated: Vec<&String> = anchors
            .iter()
            .filter(|(a, n)| {
                **n > 1
                    && !list.iter().any(|p| {
                        h2_names(&blobs[p])
                            .iter()
                            .filter(|h| anchor(h) == **a)
                            .count()
                            == **n
                    })
            })
            .map(|(a, _)| a)
            .collect();
        if !duplicated.is_empty() {
            failures.push(
                json!({"sidecar":side,"failure":"duplicate H2 anchors","anchors":duplicated}),
            );
        }
    }
    Ok(json!({
        "schema": 1,
        "kind": "groupant_consolidation_check",
        "base": base_rev,
        "sidecars": originals.len(),
        "originals": all.len(),
        "failures": failures,
        "valid": failures.is_empty(),
        "scope": "Text preservation modulo Markdown reference targets and heading demotion; not a semantic check of the models.",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn git(root: &Path, args: &[&str]) {
        let ok = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(args)
            .output()
            .unwrap();
        assert!(
            ok.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&ok.stderr)
        );
    }

    fn fixture(name: &str) -> std::path::PathBuf {
        let root =
            std::env::temp_dir().join(format!("mova-consolidate-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let side = root.join("a.rs.groupant");
        fs::create_dir_all(&side).unwrap();
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(root.join("a.rs"), "fn a() {}\n").unwrap();
        fs::write(
            side.join("descriptant.md"),
            "# A\n\nWhat a does; see [tests](tests.md).\n",
        )
        .unwrap();
        fs::write(side.join("model.md"), "# A model\n\n## Forward\n\nf(x)=x\n").unwrap();
        fs::write(
            side.join("tests.md"),
            "# Tests\n\n## Cases\n\n```\n## not a heading\n```\n",
        )
        .unwrap();
        let tests_sha = sha(b"# Tests\n\n## Cases\n\n```\n## not a heading\n```\n");
        fs::write(
            side.join("manifest.json"),
            format!(
                "{{\"id\":\"a\",\"components\":{{\"approximant\":{{\"path\":\"model.md\",\"section\":\"Forward\"}},\
\"descriptant\":{{\"path\":\"descriptant.md\"}},\"tests\":{{\"path\":\"tests.md\",\"sha256\":\"{tests_sha}\"}}}}}}\n"
            ),
        )
        .unwrap();
        fs::write(root.join("docs/x.md"), "See [the tests](../a.rs.groupant/tests.md#cases) and `a.rs.groupant/descriptant.md`.\n").unwrap();
        // An already single-file sidecar whose model.md links into a merged one changes
        // only by a moved link; the check must not treat it as a merge.
        fs::create_dir_all(root.join("b.rs.groupant")).unwrap();
        fs::write(root.join("b.rs"), "fn b() {}\n").unwrap();
        fs::write(
            root.join("b.rs.groupant/model.md"),
            "# B\n\nUses [a tests](../a.rs.groupant/tests.md).\n",
        )
        .unwrap();
        git(&root, &["init", "-q"]);
        git(&root, &["add", "-A"]);
        git(
            &root,
            &[
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "commit",
                "-q",
                "-m",
                "base",
            ],
        );
        root
    }

    #[test]
    fn merges_rewrites_rebinds_and_checks() {
        let root = fixture("merge");
        let dry = consolidate(&root, ".", true).unwrap();
        assert_eq!(dry, consolidate(&root, ".", true).unwrap()); // deterministic
        assert!(root.join("a.rs.groupant/tests.md").exists()); // dry run writes nothing
        let report = consolidate(&root, ".", false).unwrap();
        assert_eq!(report["merged_sidecars"], 1);
        assert_eq!(
            report["remove"],
            json!(["a.rs.groupant/descriptant.md", "a.rs.groupant/tests.md"])
        );
        let model = fs::read_to_string(root.join("a.rs.groupant/model.md")).unwrap();
        assert_eq!(
            model,
            format!(
                "# a\n\n{MIGRATION_LINE}\n\n## descriptant\n\n### A\n\nWhat a does; see [tests](model.md#tests).\n\n\
## model\n\n# A model\n\n## Forward\n\nf(x)=x\n\n## tests\n\n### Tests\n\n#### Cases\n\n```\n## not a heading\n```\n"
            )
        );
        let manifest: Value = serde_json::from_str(
            &fs::read_to_string(root.join("a.rs.groupant/manifest.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            manifest["components"]["descriptant"]["path"],
            "model.md#descriptant"
        );
        assert_eq!(
            manifest["components"]["approximant"],
            json!({"path":"model.md","section":"Forward"})
        );
        let tests = sections::section_bytes(&model, "tests").unwrap();
        assert_eq!(
            manifest["components"]["tests"],
            json!({"path":"model.md#tests","sha256":sha(tests.as_bytes())})
        );
        let (_, bound) =
            crate::reference_bytes(&root, &root.join("a.rs.groupant"), "model.md#tests").unwrap();
        assert_eq!(bound, tests.as_bytes());
        assert!(
            crate::reference_bytes(&root, &root.join("a.rs.groupant"), "model.md#Missing").is_err()
        );
        assert_eq!(
            fs::read_to_string(root.join("docs/x.md")).unwrap(),
            "See [the tests](../a.rs.groupant/model.md#cases) and `a.rs.groupant/model.md#descriptant`.\n"
        );
        assert_eq!(
            fs::read_to_string(root.join("b.rs.groupant/model.md")).unwrap(),
            "# B\n\nUses [a tests](../a.rs.groupant/model.md#tests).\n"
        );
        for f in report["remove"].as_array().unwrap() {
            git(&root, &["rm", "-q", f.as_str().unwrap()]);
        }
        git(&root, &["add", "-A"]);
        assert_eq!(check(&root, "HEAD").unwrap()["valid"], true);
        // Idempotent: a second run finds nothing to merge.
        assert_eq!(consolidate(&root, ".", true).unwrap()["merged_sidecars"], 0);
        // Negative control: one changed word in a block fails the check.
        fs::write(
            root.join("a.rs.groupant/model.md"),
            model.replace("f(x)=x", "f(x)=y"),
        )
        .unwrap();
        let bad = check(&root, "HEAD").unwrap();
        assert_eq!(bad["valid"], false);
        assert_eq!(bad["failures"][0]["stem"], "model");
        // Negative control: invented text outside any block fails too.
        fs::write(
            root.join("a.rs.groupant/model.md"),
            model.replacen("\n\n## descriptant", "\n\ninvented\n\n## descriptant", 1),
        )
        .unwrap();
        assert_eq!(check(&root, "HEAD").unwrap()["valid"], false);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn collisions_rename_and_open_fences_stay_unmerged() {
        let root = fixture("unmerged");
        // A verbatim model.md with a `## tests` heading collides with the tests block:
        // the heading becomes `## tests (model)` and its section reference follows.
        fs::write(
            root.join("a.rs.groupant/model.md"),
            "## Forward\n\nf\n\n## tests\n\nx\n",
        )
        .unwrap();
        let manifest = fs::read_to_string(root.join("a.rs.groupant/manifest.json")).unwrap();
        fs::write(
            root.join("a.rs.groupant/manifest.json"),
            manifest.replacen(
                "\"components\":{",
                "\"components\":{\"cost\":{\"path\":\"model.md\",\"section\":\"tests\"},",
                1,
            ),
        )
        .unwrap();
        git(&root, &["add", "-A"]);
        git(
            &root,
            &[
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "commit",
                "-q",
                "-m",
                "collide",
            ],
        );
        let report = consolidate(&root, ".", false).unwrap();
        assert_eq!(report["merged_sidecars"], 1);
        assert_eq!(
            report["heading_renames"]["a.rs.groupant/model.md"]["tests"],
            "tests (model)"
        );
        let model = fs::read_to_string(root.join("a.rs.groupant/model.md")).unwrap();
        assert!(
            model.contains("\n## tests (model)\n\nx\n")
                && model.contains("\n## tests\n\n### Tests\n"),
            "{model}"
        );
        let m: Value = serde_json::from_str(
            &fs::read_to_string(root.join("a.rs.groupant/manifest.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            m["components"]["cost"],
            json!({"path":"model.md","section":"tests (model)"})
        );
        for f in report["remove"].as_array().unwrap() {
            git(&root, &["rm", "-q", f.as_str().unwrap()]);
        }
        git(&root, &["add", "-A"]);
        assert_eq!(check(&root, "HEAD").unwrap()["valid"], true);
        // negative control: a renamed heading under another stem's suffix is not reversed
        fs::write(
            root.join("a.rs.groupant/model.md"),
            model.replace("## tests (model)", "## tests (other)"),
        )
        .unwrap();
        assert_eq!(check(&root, "HEAD").unwrap()["valid"], false);
        fs::remove_dir_all(&root).unwrap();
        let root = fixture("fence");
        fs::write(
            root.join("a.rs.groupant/model.md"),
            "## Forward\n\n~~~\nopen\n",
        )
        .unwrap();
        let report = consolidate(&root, ".", true).unwrap();
        assert!(
            report["unmerged"]["a.rs.groupant"]
                .as_str()
                .unwrap()
                .contains("code fence")
        );
        fs::remove_dir_all(root).unwrap();
    }
}
