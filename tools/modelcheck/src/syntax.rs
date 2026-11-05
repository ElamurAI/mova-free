//! Source-bound partial Rust AST projection. Token bodies and semantic analysis are out of scope.
use anyhow::{Context, Result, ensure};
use proc_macro2::{LineColumn, Span};
use serde::Serialize;
use std::{fs, path::Path};
use syn::{
    spanned::Spanned,
    visit::{self, Visit},
};

#[derive(Debug, Serialize)]
pub struct Projection {
    pub schema: u32,
    pub kind: &'static str,
    pub source: Source,
    pub parser: &'static str,
    pub range_unit: &'static str,
    pub limitations: Vec<&'static str>,
    pub nodes: Vec<Node>,
}
#[derive(Debug, Serialize)]
pub struct Source {
    pub path: String,
    pub sha256: String,
    pub bytes: usize,
}
#[derive(Debug, Serialize)]
pub struct Node {
    pub id: usize,
    pub parent: Option<usize>,
    pub category: &'static str,
    pub variant: &'static str,
    pub start: usize,
    pub end: usize,
    pub opaque: bool,
}

/// Read only Git-discovered, confined Rust sources; hash exactly the parsed bytes.
pub fn project(root: &Path, source: &str) -> Result<Projection> {
    ensure!(
        crate::relative(source)
            && source.ends_with(".rs")
            && source.split('/').all(|p| !matches!(p, "" | "." | "..")),
        "expected a normalized repository-relative Rust source"
    );
    let root = fs::canonicalize(root)?;
    ensure!(
        crate::discovered_source(&crate::rust_files(&root)?, source),
        "source is not a Git-discovered Rust file"
    );
    let path = crate::confined_file(&root, &root, source)?;
    let bytes = fs::read(path)?;
    let text = std::str::from_utf8(&bytes).context("Rust source must be UTF-8")?;
    parse(source, text)
}
pub(crate) fn parse(path: &str, text: &str) -> Result<Projection> {
    let ast = syn::parse_file(text).context("cannot parse Rust syntax")?;
    let mut builder = Builder::new(text);
    builder.nodes.push(Node {
        id: 0,
        parent: None,
        category: "file",
        variant: "File",
        start: 0,
        end: text.len(),
        opaque: false,
    });
    builder.stack.push(0);
    builder.visit_file(&ast);
    ensure!(
        !builder.invalid_span,
        "parser returned an invalid source span"
    );
    Ok(Projection {
        schema: 1,
        kind: "rust_syntax_projection",
        source: Source {
            path: path.to_owned(),
            sha256: crate::sha(text.as_bytes()),
            bytes: text.len(),
        },
        parser: "syn-2",
        range_unit: "utf8_bytes_half_open",
        limitations: vec![
            "partial_ast_not_lossless_syntax",
            "macro_token_bodies_opaque_no_expansion",
            "no_name_resolution_or_type_inference",
            "no_cfg_evaluation_or_external_module_loading",
            "attributes_comments_tokens_and_some_ast_fields_not_projected",
            "verbatim_and_unknown_variants_opaque",
            "source_read_is_not_a_repository_snapshot",
        ],
        nodes: builder.nodes,
    })
}
struct Builder {
    // proc_macro2 locations use Unicode scalar columns, not UTF-8 bytes.
    lines: Vec<Vec<usize>>,
    nodes: Vec<Node>,
    stack: Vec<usize>,
    invalid_span: bool,
}
impl Builder {
    fn new(text: &str) -> Self {
        // syn::parse_file strips a leading BOM before creating spans.
        let (text, base) = text.strip_prefix('\u{feff}').map_or((text, 0), |s| (s, 3));
        let mut lines = vec![vec![base]];
        for (offset, ch) in text.char_indices() {
            let offset = offset + base;
            if ch == '\n' {
                lines.push(vec![offset + 1]);
            } else {
                lines.last_mut().unwrap().push(offset + ch.len_utf8());
            }
        }
        Self {
            lines,
            nodes: vec![],
            stack: vec![],
            invalid_span: false,
        }
    }
    fn offset(&mut self, pos: LineColumn) -> usize {
        if let Some(offset) = pos
            .line
            .checked_sub(1)
            .and_then(|i| self.lines.get(i))
            .and_then(|line| line.get(pos.column))
        {
            *offset
        } else {
            self.invalid_span = true;
            0
        }
    }
    fn enter(&mut self, category: &'static str, variant: &'static str, span: Span) {
        let start = self.offset(span.start());
        let end = self.offset(span.end());
        let parent = self.stack.last().copied();
        if start > end
            || parent.is_some_and(|p| start < self.nodes[p].start || end > self.nodes[p].end)
        {
            self.invalid_span = true;
        }
        let id = self.nodes.len();
        self.nodes.push(Node {
            id,
            parent,
            category,
            variant,
            start,
            end,
            opaque: matches!(variant, "Verbatim" | "Unknown" | "Macro"),
        });
        self.stack.push(id);
    }
    fn leave(&mut self) {
        self.stack.pop();
    }
}
macro_rules! variant {
    ($value:expr, $enum:ident, $($name:ident),+ $(,)?) => {
        match $value { $(syn::$enum::$name(_) => stringify!($name),)+ _ => "Unknown" }
    };
}
impl<'ast> Visit<'ast> for Builder {
    fn visit_item(&mut self, node: &'ast syn::Item) {
        let kind = variant!(
            node,
            Item,
            Const,
            Enum,
            ExternCrate,
            Fn,
            ForeignMod,
            Impl,
            Macro,
            Mod,
            Static,
            Struct,
            Trait,
            TraitAlias,
            Type,
            Union,
            Use,
            Verbatim
        );
        self.enter("item", kind, node.span());
        visit::visit_item(self, node);
        self.leave();
    }
    fn visit_impl_item(&mut self, node: &'ast syn::ImplItem) {
        self.enter(
            "impl_item",
            variant!(node, ImplItem, Const, Fn, Type, Macro, Verbatim),
            node.span(),
        );
        visit::visit_impl_item(self, node);
        self.leave();
    }
    fn visit_trait_item(&mut self, node: &'ast syn::TraitItem) {
        self.enter(
            "trait_item",
            variant!(node, TraitItem, Const, Fn, Type, Macro, Verbatim),
            node.span(),
        );
        visit::visit_trait_item(self, node);
        self.leave();
    }
    fn visit_foreign_item(&mut self, node: &'ast syn::ForeignItem) {
        self.enter(
            "foreign_item",
            variant!(node, ForeignItem, Fn, Static, Type, Macro, Verbatim),
            node.span(),
        );
        visit::visit_foreign_item(self, node);
        self.leave();
    }
    fn visit_stmt(&mut self, node: &'ast syn::Stmt) {
        let kind = match node {
            syn::Stmt::Local(_) => "Local",
            syn::Stmt::Item(_) => "Item",
            syn::Stmt::Expr(_, None) => "Expr",
            syn::Stmt::Expr(_, Some(_)) => "Semi",
            syn::Stmt::Macro(_) => "Macro",
        };
        self.enter("statement", kind, node.span());
        visit::visit_stmt(self, node);
        self.leave();
    }
    fn visit_expr(&mut self, node: &'ast syn::Expr) {
        self.enter(
            "expression",
            variant!(
                node, Expr, Array, Assign, Async, Await, Binary, Block, Break, Call, Cast, Closure,
                Const, Continue, Field, ForLoop, Group, If, Index, Infer, Let, Lit, Loop, Macro,
                Match, MethodCall, Paren, Path, Range, RawAddr, Reference, Repeat, Return, Struct,
                Try, TryBlock, Tuple, Unary, Unsafe, Verbatim, While, Yield
            ),
            node.span(),
        );
        visit::visit_expr(self, node);
        self.leave();
    }
    fn visit_pat(&mut self, node: &'ast syn::Pat) {
        self.enter(
            "pattern",
            variant!(
                node,
                Pat,
                Const,
                Ident,
                Lit,
                Macro,
                Or,
                Paren,
                Path,
                Range,
                Reference,
                Rest,
                Slice,
                Struct,
                Tuple,
                TupleStruct,
                Type,
                Verbatim,
                Wild
            ),
            node.span(),
        );
        visit::visit_pat(self, node);
        self.leave();
    }
    fn visit_type(&mut self, node: &'ast syn::Type) {
        self.enter(
            "type",
            variant!(
                node,
                Type,
                Array,
                BareFn,
                Group,
                ImplTrait,
                Infer,
                Macro,
                Never,
                Paren,
                Path,
                Ptr,
                Reference,
                Slice,
                TraitObject,
                Tuple,
                Verbatim
            ),
            node.span(),
        );
        visit::visit_type(self, node);
        self.leave();
    }
    fn visit_block(&mut self, node: &'ast syn::Block) {
        self.enter("block", "Block", node.span());
        visit::visit_block(self, node);
        self.leave();
    }
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.enter("macro", "Macro", node.span());
        self.leave();
    }
    // Attributes may contain expression-shaped arguments. Keep them outside this projection.
    fn visit_attribute(&mut self, _: &'ast syn::Attribute) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        process::Command,
        sync::atomic::{AtomicUsize, Ordering},
    };
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    struct Repo(std::path::PathBuf);
    impl Repo {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "mova-syntax-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&path).unwrap();
            assert!(
                Command::new("git")
                    .arg("init")
                    .arg("-q")
                    .arg(&path)
                    .status()
                    .unwrap()
                    .success()
            );
            Self(path)
        }
    }
    impl Drop for Repo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn nested_projection_is_deterministic_and_source_bound() {
        let text = "mod inner { trait T { fn f(&self) -> u8; } struct S; impl T for S { fn f(&self) -> u8 { let x: u8 = 1; if x > 0 { x } else { 0 } } } }";
        let a = parse("src/lib.rs", text).unwrap();
        let b = parse("src/lib.rs", text).unwrap();
        assert_eq!(
            serde_json::to_vec(&a).unwrap(),
            serde_json::to_vec(&b).unwrap()
        );
        assert_eq!(a.source.sha256, crate::sha(text.as_bytes()));
        for (category, variant) in [
            ("item", "Mod"),
            ("trait_item", "Fn"),
            ("impl_item", "Fn"),
            ("statement", "Local"),
            ("expression", "If"),
            ("pattern", "Ident"),
            ("type", "Path"),
        ] {
            assert!(
                a.nodes
                    .iter()
                    .any(|n| n.category == category && n.variant == variant)
            );
        }
        for n in &a.nodes[1..] {
            let p = &a.nodes[n.parent.unwrap()];
            assert!(p.id < n.id && p.start <= n.start && n.end <= p.end);
            assert!(text.get(n.start..n.end).is_some());
        }
    }
    #[test]
    fn unicode_crlf_spans_are_exact_byte_ranges() {
        let text = "// 🐙\r\nfn café() { let π = \"🦀\"; π; }\r\n";
        let p = parse("u.rs", text).unwrap();
        let snippets: Vec<_> = p.nodes.iter().map(|n| &text[n.start..n.end]).collect();
        assert!(snippets.contains(&"π"));
        assert!(snippets.contains(&"\"🦀\""));
        assert!(snippets.contains(&"let π = \"🦀\";"));
        let f = p.nodes.iter().find(|n| n.category == "item").unwrap();
        assert_eq!(&text[f.start..f.end], "fn café() { let π = \"🦀\"; π; }");
    }
    #[test]
    fn bom_and_shebang_keep_original_source_offsets() {
        for text in [
            "\u{feff}fn café() {}",
            "#!/usr/bin/env rust-script\nfn café() {}",
            "\u{feff}#!/usr/bin/env rust-script\r\nfn café() {}",
        ] {
            let p = parse("script.rs", text).unwrap();
            let f = p.nodes.iter().find(|n| n.category == "item").unwrap();
            assert_eq!(&text[f.start..f.end], "fn café() {}");
            assert_eq!(p.source.sha256, crate::sha(text.as_bytes()));
        }
    }
    #[test]
    fn omitted_attributes_macro_bodies_and_external_modules_preserve_shape() {
        let shape = |text: &str| {
            parse("shape.rs", text)
                .unwrap()
                .nodes
                .into_iter()
                .map(|n| (n.parent, n.category, n.variant, n.opaque))
                .collect::<Vec<_>>()
        };
        assert_eq!(shape("fn f() {}"), shape("#[cfg(any())] fn f() {}"));
        assert_eq!(
            shape("fn f() { opaque! { a + b }; }"),
            shape("fn f() { opaque! { @@@ arbitrary tokens }; }")
        );
        let repo = Repo::new();
        fs::write(
            repo.0.join("a.rs"),
            "mod absent; extern \"C\" { fn foreign(); }",
        )
        .unwrap();
        let projection = project(&repo.0, "a.rs").unwrap();
        assert!(
            projection
                .nodes
                .iter()
                .any(|n| n.category == "foreign_item" && n.variant == "Fn")
        );
        assert_eq!(
            projection
                .nodes
                .iter()
                .filter(|n| n.category == "item" && n.variant == "Mod")
                .count(),
            1
        );
        assert!(!repo.0.join("absent.rs").exists());
    }

    #[test]
    fn macros_are_opaque_and_invalid_rust_fails() {
        let p = parse("m.rs", "fn f() { strange! { not Rust at all @@@ }; }").unwrap();
        let m = p.nodes.iter().find(|n| n.category == "macro").unwrap();
        assert!(m.opaque);
        assert!(!p.nodes.iter().any(|n| n.parent == Some(m.id)));
        assert!(parse("bad.rs", "fn missing(").is_err());
        assert_eq!(parse("empty.rs", "").unwrap().nodes.len(), 1);
    }
    #[test]
    fn discovery_and_exclusive_output_reject_unsupported_paths() {
        let repo = Repo::new();
        fs::write(repo.0.join("a.rs"), "fn f() {}").unwrap();
        fs::write(repo.0.join("notes.txt"), "fn f() {}").unwrap();
        fs::write(repo.0.join(".gitignore"), "ignored.rs\n").unwrap();
        fs::write(repo.0.join("ignored.rs"), "fn f() {}").unwrap();
        assert!(project(&repo.0, "a.rs").is_ok());
        for path in [
            "../a.rs",
            "/a.rs",
            "./a.rs",
            "notes.txt",
            "ignored.rs",
            "missing.rs",
            "a.rs/../a.rs",
        ] {
            assert!(project(&repo.0, path).is_err(), "{path}");
        }
        fs::write(repo.0.join("bad.rs"), [0xff]).unwrap();
        assert!(project(&repo.0, "bad.rs").is_err());
        let out = repo.0.join("output.json");
        crate::write_new(&out, b"first").unwrap();
        assert!(crate::write_new(&out, b"second").is_err());
        assert_eq!(fs::read(out).unwrap(), b"first");
    }
    #[cfg(unix)]
    #[test]
    fn source_symlink_escape_is_rejected() {
        let repo = Repo::new();
        let outside = Repo::new();
        fs::write(outside.0.join("outside.rs"), "fn f() {}").unwrap();
        std::os::unix::fs::symlink(outside.0.join("outside.rs"), repo.0.join("escape.rs")).unwrap();
        assert!(project(&repo.0, "escape.rs").is_err());
    }
}

/// Opt-in artifact integrity checks, not AST/source semantic correspondence.
pub(crate) fn validate_artifact(
    root: &Path,
    source: &Path,
    expected_sha: Option<&str>,
    v: &serde_json::Value,
) -> Result<()> {
    ensure!(
        v["schema"] == 1 && v["kind"] == "rust_syntax_projection",
        "expected schema 1 rust_syntax_projection"
    );
    let relative = source
        .strip_prefix(root)?
        .to_str()
        .context("source path must be UTF-8")?;
    ensure!(
        v["source"]["path"].as_str() == Some(relative),
        "projection source path mismatch"
    );
    ensure!(
        expected_sha.is_some() && v["source"]["sha256"].as_str() == expected_sha,
        "projection source SHA-256 missing or stale"
    );
    let text = fs::read_to_string(source)?;
    ensure!(
        Some(crate::sha(text.as_bytes()).as_str()) == expected_sha,
        "source changed during artifact validation"
    );
    ensure!(
        v["source"]["bytes"].as_u64() == Some(text.len() as u64)
            && v["range_unit"] == "utf8_bytes_half_open",
        "projection byte count or range unit mismatch"
    );
    ensure!(v["parser"] == "syn-2", "unsupported projection parser");
    ensure!(
        v["limitations"]
            .as_array()
            .is_some_and(|a| !a.is_empty() && a.iter().all(|x| crate::nonempty(x).is_some())),
        "projection limitations missing"
    );
    let nodes = v["nodes"]
        .as_array()
        .context("projection nodes must be an array")?;
    ensure!(!nodes.is_empty(), "projection root missing");
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for (index, node) in nodes.iter().enumerate() {
        ensure!(
            node["id"].as_u64() == Some(index as u64),
            "projection node IDs must equal their array indices"
        );
        let start = node["start"]
            .as_u64()
            .and_then(|n| usize::try_from(n).ok())
            .context("invalid node start")?;
        let end = node["end"]
            .as_u64()
            .and_then(|n| usize::try_from(n).ok())
            .context("invalid node end")?;
        ensure!(
            start <= end && text.get(start..end).is_some(),
            "invalid UTF-8 byte range"
        );
        ensure!(
            crate::nonempty(&node["category"]).is_some()
                && crate::nonempty(&node["variant"]).is_some()
                && node["opaque"].is_boolean(),
            "missing node category, variant or opacity"
        );
        if index == 0 {
            ensure!(
                node.get("parent").is_some_and(serde_json::Value::is_null)
                    && node["category"] == "file"
                    && node["variant"] == "File"
                    && start == 0
                    && end == text.len(),
                "invalid projection root"
            );
        } else {
            let parent = node["parent"]
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .context("invalid node parent")?;
            ensure!(parent < index, "projection parent must precede child");
            let (ps, pe) = ranges[parent];
            ensure!(ps <= start && end <= pe, "projection child escapes parent");
        }
        ranges.push((start, end));
    }
    Ok(())
}

#[cfg(test)]
mod artifact_tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn declared_projection_rejects_stale_wrong_source_and_bad_ranges() {
        let root =
            std::env::temp_dir().join(format!("mova-syntax-artifact-{}", std::process::id()));
        let dir = root.join("a.rs.groupant");
        fs::create_dir_all(&dir).unwrap();
        let text = "fn café() {}";
        let source = root.join("a.rs");
        fs::write(&source, text).unwrap();
        let hash = crate::sha(text.as_bytes());
        let projection = serde_json::to_value(parse("a.rs", text).unwrap()).unwrap();
        fs::write(dir.join("model.md"), "model").unwrap();
        let manifest = json!({"schema":1,"kind":"groupant","id":"a","status":"partial",
            "source":{"path":"../a.rs","sha256":hash},
            "components":{"ast":{"status":"partial","format":"rust_syntax_projection","path":"syntax.json"},
            "approximant":{"status":"partial","path":"model.md"},
            "invariant":{"status":"partial","path":"model.md"},
            "descriptant":{"status":"partial","path":"model.md"},
            "obernants":[{"id":"c","type":"behavior_complement","status":"not_materialized"},
            {"id":"r","type":"reverse_reconstruction","status":"not_materialized"}]}});
        let check = |artifact: &serde_json::Value, manifest: &serde_json::Value, sha: &str| {
            fs::write(
                dir.join("syntax.json"),
                serde_json::to_vec(artifact).unwrap(),
            )
            .unwrap();
            let mut errors = vec![];
            crate::validate_groupant(&root, &dir, &source, Some(sha), manifest, &mut errors);
            errors
        };
        assert!(check(&projection, &manifest, &hash).is_empty());
        for (pointer, value) in [
            ("/source/path", json!("other.rs")),
            ("/source/sha256", json!("stale")),
            ("/kind", json!("full_ast")),
            ("/source/bytes", json!(0)),
            ("/nodes/1/parent", json!(1)),
            ("/nodes/1/end", json!(100)),
            ("/nodes/1/start", json!(7)), // Inside é's UTF-8 encoding.
        ] {
            let mut bad = projection.clone();
            *bad.pointer_mut(pointer).unwrap() = value;
            assert!(!check(&bad, &manifest, &hash).is_empty(), "{pointer}");
        }
        fs::write(&source, "fn changed() {}").unwrap();
        let new_hash = crate::sha(b"fn changed() {}");
        let mut rebound = manifest.clone();
        rebound["source"]["sha256"] = json!(new_hash);
        assert!(!check(&projection, &rebound, &new_hash).is_empty());
        fs::remove_dir_all(root).unwrap();
    }
}

// A decode memo, never a validation or filesystem cache. Every caller confines
// and rereads the artifact; every returned Value still undergoes validation.
const MAX_RETAINED_ARTIFACT_BYTES: usize = 8 * 1024 * 1024;
pub(crate) struct ArtifactReader {
    reference: bool,
    last: Option<(Vec<u8>, std::rc::Rc<serde_json::Value>)>,
    /// Projections rebuilt in memory because their `syntax.json` was absent (see `projection`).
    pub(crate) regenerated: usize,
}
impl ArtifactReader {
    pub(crate) fn new() -> Self {
        Self {
            reference: std::env::var_os("MOVA_MODELCHECK_REFERENCE_JSON").is_some_and(|v| v == "1"),
            last: None,
            regenerated: 0,
        }
    }
    pub(crate) fn read(&mut self, path: &Path) -> Result<std::rc::Rc<serde_json::Value>> {
        let bytes = fs::read(path)?;
        if !self.reference {
            if let Some((saved, value)) = &self.last {
                if *saved == bytes {
                    return Ok(std::rc::Rc::clone(value));
                }
            }
        }
        let value = std::rc::Rc::new(serde_json::from_slice(&bytes)?);
        if !self.reference && bytes.len() <= MAX_RETAINED_ARTIFACT_BYTES {
            self.last = Some((bytes, std::rc::Rc::clone(&value)));
        } else {
            self.last = None;
        }
        Ok(value)
    }
    /// A `syntax.json` projection. One that is absent is regenerable, not an error: public exports omit
    /// projections, and `mova-modelcheck syntax REPO SOURCE NEW_JSON` writes them again. It is rebuilt in
    /// memory from the current source and validated like a stored one (a stale manifest hash still fails).
    /// Any other name, or a present file, is read as usual.
    pub(crate) fn projection(
        &mut self,
        root: &Path,
        dir: &Path,
        name: &str,
        source: &Path,
    ) -> Result<std::rc::Rc<serde_json::Value>> {
        let candidate = dir.join(name);
        let regenerable = !name.trim().is_empty()
            && !Path::new(name).is_absolute()
            && Path::new(name).file_name().is_some_and(|f| f == "syntax.json")
            && fs::symlink_metadata(&candidate).is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound);
        if !regenerable {
            let artifact = crate::confined_file(root, dir, name)?;
            return self.read(&artifact);
        }
        let parent = fs::canonicalize(candidate.parent().context("projection parent")?)?;
        ensure!(parent.starts_with(root), "projection reference leaves the repository");
        let relative = source.strip_prefix(root)?.to_str().context("source path must be UTF-8")?;
        let text = fs::read_to_string(source)?;
        self.regenerated += 1;
        Ok(std::rc::Rc::new(serde_json::to_value(parse(relative, &text)?)?))
    }
}

#[cfg(test)]
mod wave34_tests {
    use super::*;
    use std::{hint::black_box, rc::Rc, time::Instant};

    #[test]
    fn artifact_decode_memo_matches_reference_and_observes_file_changes() {
        let dir = std::env::temp_dir().join(format!("mova-decode-memo-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("artifact.json");
        let mut memo = ArtifactReader {
            reference: false,
            last: None,
            regenerated: 0,
        };
        let mut reference = ArtifactReader {
            reference: true,
            last: None,
            regenerated: 0,
        };
        for bytes in [
            b"{\"v\":1}".as_slice(),
            b"{\"v\":2}",
            b"{malformed",
            b"{\"v\":2}",
            b"null",
            "[1,true,\"π\"]".as_bytes(),
            b"\xff",
            b"{\"v\":1}",
        ] {
            fs::write(&path, bytes).unwrap();
            for _ in 0..2 {
                let actual = memo
                    .read(&path)
                    .map(|v| (*v).clone())
                    .map_err(|e| e.to_string());
                let expected = reference
                    .read(&path)
                    .map(|v| (*v).clone())
                    .map_err(|e| e.to_string());
                assert_eq!(actual, expected, "bytes={bytes:?}");
            }
        }
        let first = memo.read(&path).unwrap();
        let second = memo.read(&path).unwrap();
        assert!(
            Rc::ptr_eq(&first, &second),
            "identical bytes should share decoded value"
        );
        fs::remove_file(&path).unwrap();
        assert!(
            memo.read(&path).is_err(),
            "missing file cannot reuse success"
        );
        fs::create_dir(&path).unwrap();
        assert!(memo.read(&path).is_err(), "directory cannot reuse success");
        fs::remove_dir(&path).unwrap();
        fs::write(&path, b"{\"v\":9}").unwrap();
        assert_eq!(memo.read(&path).unwrap()["v"], 9);
        let oversized = format!("\"{}\"", "x".repeat(MAX_RETAINED_ARTIFACT_BYTES));
        fs::write(&path, oversized).unwrap();
        assert_eq!(
            memo.read(&path).unwrap().as_str().unwrap().len(),
            MAX_RETAINED_ARTIFACT_BYTES
        );
        assert!(memo.last.is_none(), "oversized artifacts are not retained");
        assert!(reference.last.is_none());
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn cached_decode_still_validates_source_and_nodes_and_measures_cost() {
        let dir =
            std::env::temp_dir().join(format!("mova-decode-validation-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let source = dir.join("a.rs");
        let artifact = dir.join("syntax.json");
        let text = (0..160)
            .map(|i| format!("fn f{i}() -> usize {{ let x = {i}; x + 1 }}\n"))
            .collect::<String>();
        fs::write(&source, &text).unwrap();
        let hash = crate::sha(text.as_bytes());
        let value = serde_json::to_value(parse("a.rs", &text).unwrap()).unwrap();
        let bytes = serde_json::to_vec(&value).unwrap();
        fs::write(&artifact, &bytes).unwrap();
        let mut memo = ArtifactReader {
            reference: false,
            last: None,
            regenerated: 0,
        };
        let initial = memo.read(&artifact).unwrap();
        validate_artifact(&dir, &source, Some(&hash), &initial).unwrap();
        fs::write(&source, text.replace("x + 1", "x + 2")).unwrap();
        let cached = memo.read(&artifact).unwrap();
        assert!(Rc::ptr_eq(&initial, &cached));
        assert!(
            validate_artifact(&dir, &source, Some(&hash), &cached)
                .unwrap_err()
                .to_string()
                .contains("source changed")
        );
        fs::write(&source, &text).unwrap();
        for (pointer, bad, diagnostic) in [
            ("/schema", serde_json::json!(2), "schema"),
            ("/source/path", serde_json::json!("b.rs"), "path mismatch"),
            ("/source/sha256", serde_json::json!("stale"), "SHA-256"),
            ("/nodes/1/id", serde_json::json!(99), "array indices"),
            ("/nodes/1/parent", serde_json::json!(1), "precede"),
            ("/nodes/1/start", serde_json::json!(text.len() + 1), "UTF-8"),
        ] {
            let mut changed = value.clone();
            *changed.pointer_mut(pointer).unwrap() = bad;
            fs::write(&artifact, serde_json::to_vec(&changed).unwrap()).unwrap();
            let observed = memo.read(&artifact).unwrap();
            let error = validate_artifact(&dir, &source, Some(&hash), &observed)
                .unwrap_err()
                .to_string();
            assert!(error.contains(diagnostic), "{pointer}: {error}");
        }
        fs::write(&artifact, &bytes).unwrap();
        let measure = |reference: bool| {
            let mut reader = ArtifactReader {
                reference,
                last: None,
                regenerated: 0,
            };
            let start = Instant::now();
            let mut total = 0;
            for _ in 0..12 {
                let current = reader.read(black_box(&artifact)).unwrap();
                validate_artifact(&dir, &source, Some(&hash), black_box(&current)).unwrap();
                assert_eq!(*current, value);
                total += current["nodes"].as_array().unwrap().len();
            }
            (start.elapsed().as_nanos(), total)
        };
        for sample in 0..4 {
            let (reference, cached) = if sample % 2 == 0 {
                let r = measure(true);
                (r, measure(false))
            } else {
                let c = measure(false);
                (measure(true), c)
            };
            assert_eq!(reference.1, cached.1);
            println!(
                "TIMING sample={sample} bytes={} repetitions=12 reference_ns={} memo_ns={} nodes={}",
                bytes.len(),
                reference.0,
                cached.0,
                cached.1
            );
        }
        fs::remove_dir_all(dir).unwrap();
    }
}
