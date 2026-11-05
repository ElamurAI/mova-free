//! Every relative link in the public Markdown resolves: the target file or folder exists, and a
//! `#fragment` names a heading (GitHub slug rules) or an explicit `id`/`name` anchor of the target.
//! Public means every Git-tracked `.md` outside `peta/`. External URLs are not fetched.
//! Negative control: `a_broken_path_and_a_missing_anchor_are_reported`.
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path, PathBuf};
use std::process::Command;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").canonicalize().unwrap()
}

/// GitHub's heading slug: lowercase; keep letters, digits, `-` and `_`; a space becomes `-`.
fn slug(heading: &str) -> String {
    let mut s = String::new();
    for c in plain(heading).trim().chars() {
        if c.is_alphanumeric() || c == '-' || c == '_' {
            s.extend(c.to_lowercase());
        } else if c == ' ' {
            s.push('-');
        }
    }
    s
}

/// Heading text without inline markup: links keep their text, code spans their content.
fn plain(t: &str) -> String {
    let mut out = String::new();
    let b: Vec<char> = t.chars().collect();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            '`' | '*' => i += 1,
            '!' if b.get(i + 1) == Some(&'[') => i += 1,
            '[' => i += 1,
            ']' if b.get(i + 1) == Some(&'(') => {
                let mut depth = 0;
                i += 1;
                while i < b.len() {
                    match b[i] {
                        '(' => depth += 1,
                        ')' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
                i += 1;
            }
            ']' => i += 1,
            '<' => {
                // drop inline HTML tags
                while i < b.len() && b[i] != '>' {
                    i += 1;
                }
                i += 1;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    out
}

/// Lines outside fenced code blocks, with their 1-based numbers.
fn prose(text: &str) -> Vec<(usize, &str)> {
    let mut fence: Option<(char, usize)> = None;
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let t = line.trim_start();
        let ch = t.chars().next().unwrap_or(' ');
        if ch == '`' || ch == '~' {
            let run = t.chars().take_while(|&c| c == ch).count();
            if run >= 3 {
                match fence {
                    None => {
                        fence = Some((ch, run));
                        continue;
                    }
                    Some((f, r)) if f == ch && run >= r && t[run..].trim().is_empty() => {
                        fence = None;
                        continue;
                    }
                    _ => {}
                }
            }
        }
        if fence.is_none() {
            out.push((n + 1, line));
        }
    }
    out
}

/// The anchors a Markdown file offers: heading slugs (duplicates numbered) and explicit ids.
fn anchors(text: &str) -> BTreeSet<String> {
    let mut seen: BTreeMap<String, usize> = BTreeMap::new();
    let mut out = BTreeSet::new();
    for (_, line) in prose(text) {
        let t = line.trim_start();
        let hashes = t.chars().take_while(|&c| c == '#').count();
        if (1..=6).contains(&hashes) && t[hashes..].starts_with(' ') {
            let base = slug(t[hashes..].trim().trim_end_matches('#'));
            let k = seen.entry(base.clone()).or_insert(0);
            out.insert(if *k == 0 { base.clone() } else { format!("{base}-{k}") });
            *k += 1;
        }
        for key in ["id=\"", "name=\""] {
            let mut rest = line;
            while let Some(p) = rest.find(key) {
                rest = &rest[p + key.len()..];
                if let Some(e) = rest.find('"') {
                    out.insert(rest[..e].to_string());
                }
            }
        }
    }
    out
}

/// Line without inline code spans.
fn strip_code(line: &str) -> String {
    let mut out = String::new();
    let mut parts = line.split('`');
    let mut inside = false;
    while let Some(p) = parts.next() {
        if !inside {
            out.push_str(p);
        } else {
            out.push(' ');
        }
        inside = !inside;
    }
    out
}

/// Relative link destinations of a file: `](dest)`, `[ref]: dest`, `href="dest"`, `src="dest"`.
fn links(text: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (n, raw) in prose(text) {
        let line = strip_code(raw);
        let mut rest = line.as_str();
        while let Some(p) = rest.find("](") {
            rest = &rest[p + 2..];
            let mut depth = 1;
            let mut end = rest.len();
            for (i, c) in rest.char_indices() {
                match c {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            end = i;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            let dest = rest[..end].trim();
            let dest = dest.trim_start_matches('<');
            let dest = dest.split(['>', ' ']).next().unwrap_or("");
            out.push((n, dest.to_string()));
            rest = &rest[end.min(rest.len())..];
        }
        let t = line.trim_start();
        if t.starts_with('[') {
            if let Some(p) = t.find("]: ") {
                if !t[..p].contains("](") {
                    out.push((n, t[p + 3..].split_whitespace().next().unwrap_or("").to_string()));
                }
            }
        }
        for key in ["href=\"", "src=\""] {
            let mut rest = line.as_str();
            while let Some(p) = rest.find(key) {
                rest = &rest[p + key.len()..];
                if let Some(e) = rest.find('"') {
                    out.push((n, rest[..e].to_string()));
                }
            }
        }
    }
    out.retain(|(_, d)| !d.is_empty() && !d.contains("://") && !d.starts_with("mailto:") && !d.starts_with("tel:"));
    out
}

fn decode(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Lexical join that keeps the path inside `root` (`/x` is root-relative, as on GitHub).
fn resolve(root: &Path, file: &Path, dest: &str) -> Option<PathBuf> {
    let base = if dest.starts_with('/') { root.to_path_buf() } else { file.parent()?.to_path_buf() };
    let mut p = base;
    for c in Path::new(dest.trim_start_matches('/')).components() {
        match c {
            Component::ParentDir => {
                if !p.pop() || !p.starts_with(root) {
                    return None;
                }
            }
            Component::Normal(x) => p.push(x),
            _ => {}
        }
    }
    Some(p)
}

/// Broken links of the given files, as `file:line: dest (reason)`.
fn check(root: &Path, files: &[PathBuf]) -> Vec<String> {
    let mut cache: BTreeMap<PathBuf, BTreeSet<String>> = BTreeMap::new();
    let mut bad = Vec::new();
    for f in files {
        let Ok(text) = std::fs::read_to_string(f) else { continue };
        let rel = f.strip_prefix(root).unwrap_or(f).display().to_string();
        for (n, dest) in links(&text) {
            let (path, frag) = match dest.split_once('#') {
                Some((p, fr)) => (p, Some(fr)),
                None => (dest.as_str(), None),
            };
            let path = decode(path.split('?').next().unwrap_or(""));
            let target = if path.is_empty() {
                Some(f.clone())
            } else {
                resolve(root, f, &path)
            };
            let Some(target) = target else {
                bad.push(format!("{rel}:{n}: {dest} (outside the repository)"));
                continue;
            };
            if !target.exists() {
                bad.push(format!("{rel}:{n}: {dest} (no such file)"));
                continue;
            }
            let Some(frag) = frag.filter(|x| !x.is_empty()) else { continue };
            if target.extension().is_some_and(|e| e == "md") {
                let a = cache
                    .entry(target.clone())
                    .or_insert_with(|| anchors(&std::fs::read_to_string(&target).unwrap_or_default()));
                let frag = decode(frag).to_lowercase();
                if !a.contains(&frag) {
                    bad.push(format!("{rel}:{n}: {dest} (no anchor #{frag})"));
                }
            }
        }
    }
    bad
}

fn public_markdown(root: &Path) -> Vec<PathBuf> {
    let out = Command::new("git").arg("-C").arg(root).args(["ls-files", "-z", "--", "*.md"]).output().expect("git ls-files");
    assert!(out.status.success(), "git ls-files failed");
    out.stdout
        .split(|&b| b == 0)
        .filter(|p| !p.is_empty())
        .map(|p| String::from_utf8_lossy(p).into_owned())
        .filter(|p| !p.starts_with("peta/") && !p.starts_with("tera/"))
        .map(|p| root.join(p))
        .collect()
}

#[test]
fn public_markdown_links_resolve() {
    let root = repo();
    let files = public_markdown(&root);
    assert!(files.len() > 100, "too few Markdown files found: {}", files.len());
    let bad = check(&root, &files);
    assert!(bad.is_empty(), "{} broken links in {} files:\n{}", bad.len(), files.len(), bad.join("\n"));
}

#[test]
fn a_broken_path_and_a_missing_anchor_are_reported() {
    let dir = std::env::temp_dir().join(format!("mova-doc-links-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&dir);
    let a = dir.join("a.md");
    let b = dir.join("b.md");
    std::fs::write(&b, "# The `b` page — here\n\n## Same\n\n## Same\n\n<a id=\"x\"></a>\n```\n# Not a heading\n```\n").unwrap();
    std::fs::write(
        &a,
        "[ok](b.md#the-b-page--here) [ok](b.md#same-1) [ok](b.md#x) [ok](#top) `[code](gone.md)`\n# Top\n\
         [bad](gone.md) [bad](b.md#not-a-heading) [bad](b.md#missing) [ext](https://example.org/x)\n\
         ```\n[fenced](gone.md)\n```\n[bad]: ../../../../../../../../../../../../../../nowhere.md\n",
    )
    .unwrap();
    let root = dir.canonicalize().unwrap();
    let bad = check(&root, &[root.join("a.md")]);
    assert_eq!(bad.len(), 4, "{bad:#?}");
    assert!(bad[0].contains("gone.md (no such file)"));
    assert!(bad[1].contains("#not-a-heading"));
    assert!(bad[2].contains("#missing"));
    assert!(bad[3].contains("outside the repository"));
    assert!(check(&root, &[root.join("b.md")]).is_empty());
}
