//! Dangling sidecar references: Markdown inside `*.groupant/` sidecars that names a
//! sibling or parent `.md`/`.json` file which does not exist (for example a former
//! `evidence.md` merged into `model.md`), or a `.md#fragment` whose heading is absent.
//! `relink` rewrites such names to the `model.md#section` that now holds the former
//! file's block. Lexical scan only: no Markdown rendering, no content changes.
use crate::consolidate::{git_files, join, parent, relpath};
use crate::sections::{self, anchor, heading_level};
use anyhow::{Context, Result, ensure};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;

/// File names the public tree omits on purpose (regenerable syntax projection).
pub const REGENERABLE: [&str; 1] = ["syntax.json"];
const MODEL: &str = "model.md";
const HISTORY: &str = "History";

/// A path-like token ending in `.md` or `.json`, with an optional `#fragment`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub start: usize,
    /// End of the path (before any fragment).
    pub end: usize,
    /// End of the fragment, equal to `end` without one.
    pub fragment_end: usize,
}

fn path_char(c: u8) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, b'_' | b'.' | b'/' | b'-')
}

/// Path-like tokens ending in `.md` or `.json`. A token directly after `$`, `~`, `:`,
/// `@` or `*`, starting with `/`, or followed by `*` is external (environment roots,
/// absolute paths, URLs, globs) and skipped. Trailing sentence dots are dropped.
pub fn tokens(text: &str) -> Vec<Token> {
    let b = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if !path_char(b[i]) || (i > 0 && path_char(b[i - 1])) {
            i += 1;
            continue;
        }
        let start = i;
        while i < b.len() && path_char(b[i]) {
            i += 1;
        }
        let mut end = i;
        while end > start && b[end - 1] == b'.' {
            end -= 1;
        }
        let tok = &text[start..end];
        let name = tok.rsplit('/').next().unwrap_or(tok);
        let ext = name.ends_with(".md") || name.ends_with(".json");
        let external = (start > 0 && matches!(b[start - 1], b'$' | b'~' | b':' | b'@' | b'*'))
            || tok.starts_with('/')
            || b.get(i) == Some(&b'*');
        if !ext || external || name.starts_with('.') {
            continue;
        }
        let mut fragment_end = end;
        if end == i && b.get(end) == Some(&b'#') {
            let mut j = end + 1;
            while j < b.len() && (b[j].is_ascii_alphanumeric() || matches!(b[j], b'_' | b'-')) {
                j += 1;
            }
            if j > end + 1 {
                fragment_end = j;
                i = j;
            }
        }
        out.push(Token {
            start,
            end,
            fragment_end,
        });
    }
    out
}

/// Sidecar-style tokens: bare names, `./` and `../` paths, and any path through a
/// `.groupant/` directory. Other paths (`docs/x.md`) are repository references.
fn sidecar_style(tok: &str) -> bool {
    !tok.contains('/')
        || tok.starts_with("./")
        || tok.starts_with("../")
        || tok.contains(".groupant/")
}

/// Anchors of every ATX heading outside fences.
pub fn heading_anchors(text: &str) -> BTreeSet<String> {
    sections::lines(text)
        .into_iter()
        .filter(|l| !l.fenced)
        .filter_map(|l| {
            heading_level(l.text).map(|n| anchor(l.text.trim_start()[n..].trim_end_matches('#')))
        })
        .collect()
}

/// Repository view used by both the check and the rewrite.
pub struct Tree {
    files: BTreeSet<String>,
    sidecar_json: BTreeSet<String>,
    texts: BTreeMap<String, String>,
    /// Paths deleted anywhere in history (former sidecar files among them).
    deleted: BTreeSet<String>,
}

impl Tree {
    pub fn load(root: &Path) -> Result<Self> {
        let files: BTreeSet<String> = git_files(root)?.into_iter().collect();
        let sidecar_json = files
            .iter()
            .filter(|f| f.ends_with(".json") && parent(f).ends_with(".groupant"))
            .map(|f| f.rsplit('/').next().unwrap_or(f).to_string())
            .collect();
        let mut texts = BTreeMap::new();
        for f in files.iter().filter(|f| f.ends_with(".md")) {
            if let Ok(t) = fs::read_to_string(root.join(f)) {
                texts.insert(f.clone(), t);
            }
        }
        Ok(Tree {
            files,
            sidecar_json,
            texts,
            deleted: deleted_paths(root)?,
        })
    }

    /// Markdown files inside some `*.groupant/` directory.
    pub fn sidecar_markdown(&self) -> Vec<&String> {
        self.texts
            .keys()
            .filter(|f| f.contains(".groupant/"))
            .collect()
    }

    /// Existing file a token written in `dir` names, if any. A bare name is read from
    /// `dir` and each enclosing sidecar ("parent receipt.json", "file-root
    /// evidence.json"); a path from `dir` and every ancestor directory up to the
    /// repository root (source-directory and repository-relative spellings).
    fn existing(&self, dir: &str, tok: &str) -> Option<String> {
        let bases: Vec<String> = if tok.contains('/') {
            let mut out = vec![dir.to_string()];
            let mut d = dir;
            while !d.is_empty() {
                d = parent(d);
                out.push(d.to_string());
            }
            out
        } else {
            let mut out = vec![dir.to_string()];
            out.extend(sidecar_chain(dir));
            out
        };
        bases
            .iter()
            .filter_map(|b| join(b, tok))
            .find(|p| self.files.contains(p))
    }
}

/// One finding of the check.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub file: String,
    pub line: usize,
    pub token: String,
    /// "missing_file" or "missing_heading".
    pub kind: &'static str,
    /// Up to 100 bytes of the line on each side of the token.
    pub context: String,
}

/// True when the token is followed by its SHA-256 (`name: hash`, `` `name`: `hash` ``,
/// `name, SHA256 hash`): a provenance record of a former file's bytes, not a link.
pub fn hash_record(after: &str) -> bool {
    let mut s = after.trim_start_matches('`');
    s = s
        .strip_prefix(':')
        .or_else(|| s.strip_prefix(','))
        .unwrap_or(s);
    s = s.trim_start_matches([' ', '\t']);
    for word in ["SHA-256", "SHA256", "sha256", "sha-256"] {
        if let Some(rest) = s.strip_prefix(word) {
            s = rest.trim_start_matches(':').trim_start_matches([' ', '\t']);
            break;
        }
    }
    s = s.trim_start_matches('`');
    s.len() >= 64
        && s.as_bytes()[..64]
            .iter()
            .all(|c| matches!(c, b'0'..=b'9' | b'a'..=b'f'))
        && s.as_bytes()
            .get(64)
            .is_none_or(|c| !c.is_ascii_alphanumeric())
}

/// Up to 100 bytes of the token's line on each side, cut at character boundaries.
fn context(text: &str, start: usize, end: usize) -> String {
    let line_start = text[..start].rfind('\n').map_or(0, |i| i + 1);
    let line_end = text[end..].find('\n').map_or(text.len(), |i| end + i);
    let mut a = start.saturating_sub(100).max(line_start);
    while !text.is_char_boundary(a) {
        a += 1;
    }
    let mut b = (end + 100).min(line_end);
    while !text.is_char_boundary(b) {
        b -= 1;
    }
    text[a..b].to_string()
}

/// True when the token at `start` is listed as content of an external run directory:
/// its sentence (from the last `. `, `; ` or line start) holds `$MOVA_DATA/` followed
/// by ` contains ` before the token (`$MOVA_DATA/runs/x/ contains prospective.md, …`).
pub fn run_listing(text: &str, start: usize) -> bool {
    let before = &text[..start];
    let line = before.rfind('\n').map_or(0, |i| i + 1);
    let sentence = [before.rfind(". "), before.rfind("; ")]
        .into_iter()
        .flatten()
        .map(|i| i + 2)
        .chain([line])
        .max()
        .unwrap_or(0);
    let s = &before[sentence..];
    s.find("$MOVA_DATA/")
        .is_some_and(|i| s[i..].contains(" contains "))
}

/// Findings in one sidecar Markdown text; hash provenance records are counted apart.
pub fn scan(tree: &Tree, file: &str, text: &str) -> Vec<Finding> {
    scan_counting(tree, file, text, &mut 0)
}

fn scan_counting(tree: &Tree, file: &str, text: &str, records: &mut usize) -> Vec<Finding> {
    let dir = parent(file);
    let mut out = Vec::new();
    for t in tokens(text) {
        let tok = &text[t.start..t.end];
        let name = tok.rsplit('/').next().unwrap_or(tok);
        if REGENERABLE.contains(&name) || !sidecar_style(tok) {
            continue;
        }
        if hash_record(&text[t.fragment_end..]) {
            *records += 1;
            continue;
        }
        if !tok.contains('/') && run_listing(text, t.start) {
            continue;
        }
        // A bare name is a sidecar reference when such a file exists, or existed in this
        // sidecar chain (JSON: when some sidecar holds that name). Other bare names
        // (`native-commands.json`, an external run's `prospective.md`) are run artifacts.
        if !tok.contains('/')
            && tree.existing(dir, tok).is_none()
            && !former_sidecar_file(tree, dir, tok)
            && !(name.ends_with(".json") && tree.sidecar_json.contains(name))
        {
            continue;
        }
        let line = text[..t.start].bytes().filter(|b| *b == b'\n').count() + 1;
        let full = &text[t.start..t.fragment_end];
        match tree.existing(dir, tok) {
            None => out.push(Finding {
                file: file.to_string(),
                line,
                token: full.to_string(),
                kind: "missing_file",
                context: context(text, t.start, t.fragment_end),
            }),
            Some(path) if t.fragment_end > t.end => {
                let fragment = &text[t.end + 1..t.fragment_end];
                let target = if path == file {
                    Some(text)
                } else {
                    tree.texts.get(&path).map(String::as_str)
                };
                if let Some(target) = target
                    && !heading_anchors(target).contains(&fragment.to_lowercase())
                {
                    out.push(Finding {
                        file: file.to_string(),
                        line,
                        token: full.to_string(),
                        kind: "missing_heading",
                        context: context(text, t.start, t.fragment_end),
                    });
                }
            }
            Some(_) => {}
        }
    }
    out
}

fn findings_json(findings: &[Finding]) -> Value {
    let mut by_token: BTreeMap<&str, usize> = BTreeMap::new();
    let mut files: BTreeSet<&str> = BTreeSet::new();
    for f in findings {
        *by_token.entry(&f.token).or_default() += 1;
        files.insert(&f.file);
    }
    json!({
        "files_with_dangling": files.len(),
        "dangling_references": findings.len(),
        "by_token": by_token,
        "dangling": findings.iter().map(|d| json!({"file": d.file, "line": d.line, "token": d.token, "kind": d.kind, "context": d.context})).collect::<Vec<_>>(),
    })
}

/// Scans every Markdown file inside a `*.groupant/` directory of the repository.
pub fn check(root: &Path) -> Result<Value> {
    let tree = Tree::load(root)?;
    let mut findings = Vec::new();
    let mut records = 0;
    let scanned = tree.sidecar_markdown();
    for f in &scanned {
        findings.extend(scan_counting(&tree, f, &tree.texts[*f], &mut records));
    }
    let mut report = findings_json(&findings);
    report["hash_provenance_records_skipped"] = json!(records);
    report["schema"] = json!(1);
    report["kind"] = json!("groupant_sidecar_links");
    report["valid"] = json!(findings.is_empty());
    report["scanned_markdown_files"] = json!(scanned.len());
    report["allowed_absent"] = json!(REGENERABLE);
    report["verification_inferred"] = json!(false);
    Ok(report)
}

/// Paths of files deleted anywhere in the repository history (renames count as deletes).
pub fn deleted_paths(root: &Path) -> Result<BTreeSet<String>> {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args([
            "log",
            "--diff-filter=D",
            "--no-renames",
            "--name-only",
            "--format=",
        ])
        .output()?;
    ensure!(out.status.success(), "git log failed");
    Ok(String::from_utf8(out.stdout)?
        .lines()
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect())
}

/// Sidecar directories enclosing `dir`, innermost first (including `dir` itself).
fn sidecar_chain(dir: &str) -> Vec<String> {
    let mut out = Vec::new();
    let parts: Vec<&str> = dir.split('/').collect();
    for n in (1..=parts.len()).rev() {
        if parts[n - 1].ends_with(".groupant") {
            out.push(parts[..n].join("/"));
        }
    }
    out
}

/// Where a former file's block lives now.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Relink {
    /// Sidecar holding the model.md.
    pub sidecar: String,
    /// H2 heading text the reference now points at.
    pub heading: String,
    /// "own_section", "former_file_section", "former_file_history".
    pub via: &'static str,
}

/// Former locations of a token written in `dir`, as (former path, sidecar now holding
/// its block): read from `dir`, then from each enclosing sidecar outward, then from the
/// source directory of the outermost sidecar. A module `prospective.md` that existed
/// was folded into its `mod.rs.groupant/model.md`.
fn candidates(tree: &Tree, dir: &str, tok: &str) -> Vec<(String, String)> {
    let chain = sidecar_chain(dir);
    let mut bases: Vec<String> = chain.clone();
    if let Some(outer) = chain.last() {
        bases.push(parent(outer).to_string());
    }
    let mut out = Vec::new();
    for base in &bases {
        let Some(former) = join(base, tok) else {
            continue;
        };
        let mut side = parent(&former).to_string();
        if !side.ends_with(".groupant")
            && former.ends_with("prospective.md")
            && former.rsplit('/').next() == Some("prospective.md")
            && tree.deleted.contains(&former)
        {
            side = if side.is_empty() {
                "mod.rs.groupant".into()
            } else {
                format!("{side}/mod.rs.groupant")
            };
        }
        if side.ends_with(".groupant") && !out.iter().any(|(f, _)| *f == former) {
            out.push((former, side));
        }
    }
    out
}

/// True when some candidate former location of the token existed in history: the
/// bare name then denotes a former sidecar file, not a file of an external run.
fn former_sidecar_file(tree: &Tree, dir: &str, tok: &str) -> bool {
    candidates(tree, dir, tok)
        .iter()
        .any(|(f, _)| tree.deleted.contains(f))
}

/// Resolves a dangling token written in `dir` to a section of a sidecar's model.md.
/// The nearest candidate former location (`candidates`) whose file existed in Git
/// history decides: its sidecar's H2 whose anchor is the former file's stem, else its
/// `History` H2 when a `· stem` group heading is there. Without any former file, an
/// H2 of the writing sidecar's own model.md with that anchor.
pub fn resolve(tree: &Tree, dir: &str, tok: &str) -> Option<Relink> {
    let stem = tok.rsplit('/').next()?.strip_suffix(".md")?;
    let want = anchor(stem);
    let section = |side: &str| -> Option<String> {
        let model = tree.texts.get(&format!("{side}/{MODEL}"))?;
        sections::h2_names(model)
            .into_iter()
            .find(|h| anchor(h) == want)
            .map(str::to_string)
    };
    let dated = |side: &str| -> bool {
        let Some(model) = tree.texts.get(&format!("{side}/{MODEL}")) else {
            return false;
        };
        let history = sections::section_bytes(model, HISTORY).unwrap_or_default();
        sections::lines(history).iter().any(|l| {
            !l.fenced
                && heading_level(l.text).is_some_and(|n| n > 2)
                && l.text
                    .rsplit_once(" · ")
                    .is_some_and(|(_, s)| anchor(s) == want)
        })
    };
    // The nearest former file that existed decides.
    for (former, side) in candidates(tree, dir, tok) {
        if !tree.deleted.contains(&former) {
            continue;
        }
        if let Some(heading) = section(&side) {
            return Some(Relink {
                sidecar: side,
                heading,
                via: "former_file_section",
            });
        }
        if dated(&side) {
            return Some(Relink {
                sidecar: side,
                heading: HISTORY.to_string(),
                via: "former_file_history",
            });
        }
    }
    // No former file: a section of the writing sidecar's own model.md.
    section(dir).map(|heading| Relink {
        sidecar: dir.to_string(),
        heading,
        via: "own_section",
    })
}

/// New text for a relinked token written in `dir`: `model.md#anchor` relative to `dir`.
pub fn replacement(dir: &str, r: &Relink) -> String {
    let rel = relpath(dir, &r.sidecar);
    let prefix = if rel.is_empty() {
        String::new()
    } else {
        format!("{rel}/")
    };
    format!("{prefix}{MODEL}#{}", anchor(&r.heading))
}

/// Rewrites dangling former-file names in one text. A token that is a whole Markdown
/// link label (`[evidence.md](...)`) becomes the heading name instead. Returns the new
/// text, per-token rewrites and tokens left unresolved.
pub fn relink_text(tree: &Tree, file: &str, text: &str) -> (String, Vec<Value>, Vec<Finding>) {
    let dir = parent(file);
    let findings = scan(tree, file, text);
    let dangling: BTreeSet<usize> = findings.iter().map(|f| f.line).collect();
    let mut out = String::with_capacity(text.len() + 64);
    let mut last = 0;
    let mut done = Vec::new();
    let mut left = Vec::new();
    for t in tokens(text) {
        let full = &text[t.start..t.fragment_end];
        let line = text[..t.start].bytes().filter(|b| *b == b'\n').count() + 1;
        let Some(finding) = dangling
            .contains(&line)
            .then(|| findings.iter().find(|f| f.line == line && f.token == full))
            .flatten()
        else {
            continue;
        };
        let tok = &text[t.start..t.end];
        let relink = (finding.kind == "missing_file" && t.fragment_end == t.end)
            .then(|| resolve(tree, dir, tok))
            .flatten();
        let Some(r) = relink else {
            left.push(finding.clone());
            continue;
        };
        let label = text[..t.start].ends_with('[') && text[t.end..].starts_with("](");
        let new = if label {
            r.heading.clone()
        } else {
            replacement(dir, &r)
        };
        out.push_str(&text[last..t.start]);
        out.push_str(&new);
        last = t.end;
        done.push(json!({"file": file, "line": line, "from": tok, "to": new, "via": r.via}));
    }
    out.push_str(&text[last..]);
    (out, done, left)
}

/// Rewrites dangling former-file names in every sidecar Markdown file and rebinds
/// fresh hash-bound JSON references (`path` plus `sha256`) to the changed texts with
/// the `undate` rebinder; already-stale bindings stay untouched and are reported.
/// Writes unless `dry_run`; never deletes.
pub fn relink(root: &Path, dry_run: bool) -> Result<Value> {
    let tree = Tree::load(root)?;
    let mut changed: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut rewrites = Vec::new();
    let mut left = Vec::new();
    let mut by_via: BTreeMap<String, usize> = BTreeMap::new();
    for f in tree.sidecar_markdown() {
        let text = &tree.texts[f];
        let (new, done, rest) = relink_text(&tree, f, text);
        for d in &done {
            *by_via
                .entry(d["via"].as_str().unwrap_or_default().to_string())
                .or_default() += 1;
        }
        rewrites.extend(done);
        left.extend(rest);
        if new != *text {
            changed.insert(f.clone(), vec![text.clone(), new]);
        }
    }
    let changed_markdown: Vec<String> = changed.keys().cloned().collect();
    let json_files: Vec<String> = tree
        .files
        .iter()
        .filter(|f| f.ends_with(".json"))
        .cloned()
        .collect();
    let mut json_texts = BTreeMap::new();
    for f in &json_files {
        json_texts.insert(
            f.clone(),
            fs::read_to_string(root.join(f)).with_context(|| f.clone())?,
        );
    }
    let (rebinds, stale) = crate::undate::rebind(&mut changed, &json_files, &mut json_texts)?;
    if !dry_run {
        for (f, versions) in &changed {
            let text = versions.last().context("versions")?;
            fs::write(root.join(f), text).with_context(|| format!("cannot write {f}"))?;
        }
    }
    let json_changed: Vec<&String> = changed.keys().filter(|f| f.ends_with(".json")).collect();
    Ok(json!({
        "schema": 1,
        "kind": "groupant_sidecar_relink",
        "dry_run": dry_run,
        "changed_files": changed_markdown,
        "rewrites": rewrites.len(),
        "rewrites_by_via": by_via,
        "rewrite_list": rewrites,
        "json_files_changed": json_changed,
        "hash_rebinds": rebinds,
        "stale_bindings_untouched": stale,
        "unresolved": findings_json(&left),
        "verification_inferred": false,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_skip_external_and_keep_fragments() {
        let t = "See evidence.md. Raw: $MOVA_DATA/runs/x/a.md, /abs/b.md, https://h/c.md, d*.json; [x.md](model.md#tests) e.mdx";
        let got: Vec<&str> = tokens(t)
            .iter()
            .map(|k| &t[k.start..k.fragment_end])
            .collect();
        assert_eq!(got, ["evidence.md", "x.md", "model.md#tests"]);
        let listing =
            "Receipts: $MOVA_DATA/runs/w/ contains prospective.md, a.log. See prospective.md.";
        assert!(run_listing(listing, listing.find("prospective").unwrap()));
        assert!(!run_listing(listing, listing.rfind("prospective").unwrap()));
        let beside = "Saved in $MOVA_DATA/runs/w/prospective.md and retained beside this source as prospective.md.";
        assert!(!run_listing(beside, beside.rfind("prospective").unwrap()));
        assert!(hash_record(&format!("`: `{}`", "f".repeat(64))));
        assert!(hash_record(&format!(", SHA256 {}.", "0".repeat(64))));
        assert!(!hash_record(" for details"));
    }

    fn fixture(name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("mova-links-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("a.rs.groupant/c.groupant")).unwrap();
        let git = |args: &[&str]| {
            assert!(
                Command::new("git")
                    .arg("-C")
                    .arg(&root)
                    .args(args)
                    .output()
                    .unwrap()
                    .status
                    .success()
            );
        };
        git(&["init", "-q"]);
        git(&["config", "user.email", "t@example.invalid"]);
        git(&["config", "user.name", "t"]);
        fs::write(root.join("a.rs"), "fn a() {}\n").unwrap();
        fs::write(root.join("a.rs.groupant/evidence.md"), "old\n").unwrap();
        fs::write(root.join("a.rs.groupant/wave1-evidence.md"), "old\n").unwrap();
        git(&["add", "."]);
        git(&["commit", "-qm", "before"]);
        git(&[
            "rm",
            "-q",
            "a.rs.groupant/evidence.md",
            "a.rs.groupant/wave1-evidence.md",
        ]);
        fs::write(
            root.join("a.rs.groupant/model.md"),
            "# a\n\n## Forward\n\nSee evidence.md; syntax.json is regenerable; runs keep native-commands.json.\n\n- `evidence.md`: `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`\n\n## evidence\n\nold\n\n## History\n\n### 2026-10-30 · wave1-evidence\n\nold\n",
        )
        .unwrap();
        fs::write(
            root.join("a.rs.groupant/c.groupant/model.md"),
            "# c\n\nParent ../evidence.md and parent evidence.md; wave1-evidence.md; [evidence.md](../model.md#evidence); ../gone.md; ../model.md#nothere; external run prospective.md\n\n## evidence\n\nOwn section; the former file was the parent's.\n",
        )
        .unwrap();
        let forward = "## Forward\n\nSee evidence.md; syntax.json is regenerable; runs keep native-commands.json.\n\n- `evidence.md`: `aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`";
        let manifest = format!(
            "{{\"bindings\": [{{\"path\": \"model.md#Forward\", \"sha256\": \"{}\"}}, {{\"path\": \"model.md#Forward\", \"sha256\": \"{}\"}}]}}\n",
            crate::sha(forward.as_bytes()),
            "0".repeat(64)
        );
        fs::write(root.join("a.rs.groupant/manifest.json"), manifest).unwrap();
        git(&["add", "."]);
        git(&["commit", "-qm", "after"]);
        root
    }

    #[test]
    fn check_is_red_then_relink_makes_it_green_except_unresolved() {
        let root = fixture("relink");
        let red = check(&root).unwrap();
        assert_eq!(red["valid"], false);
        let tokens: Vec<&str> = red["dangling"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["token"].as_str().unwrap())
            .collect();
        assert_eq!(
            tokens,
            [
                "../evidence.md",
                "evidence.md",
                "wave1-evidence.md",
                "evidence.md",
                "../gone.md",
                "../model.md#nothere",
                "evidence.md",
            ]
        );
        let report = relink(&root, false).unwrap();
        assert_eq!(report["rewrites"], 5);
        let parent = fs::read_to_string(root.join("a.rs.groupant/model.md")).unwrap();
        assert!(parent.contains("See model.md#evidence; syntax.json"));
        // The hash provenance record of the former file keeps its name.
        assert!(parent.contains("- `evidence.md`: `aaaa"));
        // The fresh section binding follows the new bytes; the stale one stays.
        let manifest: Value =
            serde_json::from_slice(&fs::read(root.join("a.rs.groupant/manifest.json")).unwrap())
                .unwrap();
        let section = sections::section_bytes(&parent, "Forward").unwrap();
        assert_eq!(
            manifest["bindings"][0]["sha256"],
            crate::sha(section.as_bytes())
        );
        assert_eq!(manifest["bindings"][1]["sha256"], "0".repeat(64));
        assert_eq!(report["hash_rebinds"].as_array().unwrap().len(), 1);
        assert_eq!(
            report["stale_bindings_untouched"].as_array().unwrap().len(),
            1
        );
        let child = fs::read_to_string(root.join("a.rs.groupant/c.groupant/model.md")).unwrap();
        assert_eq!(
            child,
            "# c\n\nParent ../model.md#evidence and parent ../model.md#evidence; ../model.md#history; [evidence](../model.md#evidence); ../gone.md; ../model.md#nothere; external run prospective.md\n\n## evidence\n\nOwn section; the former file was the parent's.\n"
        );
        let after = check(&root).unwrap();
        let left: Vec<&str> = after["dangling"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["token"].as_str().unwrap())
            .collect();
        assert_eq!(left, ["../gone.md", "../model.md#nothere"]);
        // A second run changes nothing.
        assert_eq!(relink(&root, false).unwrap()["rewrites"], 0);
        let _ = fs::remove_dir_all(&root);
    }
}
