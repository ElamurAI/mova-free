//! Three-way JSON merges inside a git merge, source-binding refresh and groupant byte-range rebinding.

use crate::{compact, insert_key, load_for_edit, load_str, render, save, write_atomic, Doc, Res, Str, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn sha_hex(b: &[u8]) -> String {
    Sha256::digest(b).iter().map(|x| format!("{x:02x}")).collect()
}

pub fn git(repo: &Path, args: &[&str]) -> Res<Vec<u8>> {
    let o = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .map_err(|e| format!("git: {e}"))?;
    if !o.status.success() {
        return Err(format!("git {}: {}", args.join(" "), String::from_utf8_lossy(&o.stderr).trim()));
    }
    Ok(o.stdout)
}

fn git_text(repo: &Path, args: &[&str]) -> Res<String> {
    String::from_utf8(git(repo, args)?).map_err(|_| format!("git {}: output is not UTF-8", args.join(" ")))
}

// ---------------------------------------------------------------- sequence matching

/// Matching blocks (a_start, b_start, len) of a longest common subsequence, in order, without the sentinel.
/// Common prefix and suffix are trimmed; the middle uses an O(n*m) table bounded by `MAX_CELLS`.
pub fn matching_blocks<T: PartialEq>(a: &[T], b: &[T]) -> Res<Vec<(usize, usize, usize)>> {
    let mut pre = 0;
    while pre < a.len() && pre < b.len() && a[pre] == b[pre] {
        pre += 1;
    }
    let mut suf = 0;
    while suf < a.len() - pre && suf < b.len() - pre && a[a.len() - 1 - suf] == b[b.len() - 1 - suf] {
        suf += 1;
    }
    let (am, bm) = (&a[pre..a.len() - suf], &b[pre..b.len() - suf]);
    let (n, m) = (am.len(), bm.len());
    let mut pairs: Vec<(usize, usize)> = (0..pre).map(|i| (i, i)).collect();
    if n > 0 && m > 0 {
        if (n + 1) * (m + 1) > MAX_CELLS {
            return Err(format!("diff middle {n}x{m} exceeds {MAX_CELLS} table cells"));
        }
        // l[i][j] = LCS length of am[i..], bm[j..]
        let w = m + 1;
        let mut l = vec![0u32; (n + 1) * w];
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                l[i * w + j] = if am[i] == bm[j] { l[(i + 1) * w + j + 1] + 1 } else { l[(i + 1) * w + j].max(l[i * w + j + 1]) };
            }
        }
        let (mut i, mut j) = (0, 0);
        while i < n && j < m {
            if am[i] == bm[j] {
                pairs.push((pre + i, pre + j));
                i += 1;
                j += 1;
            } else if l[(i + 1) * w + j] >= l[i * w + j + 1] {
                i += 1;
            } else {
                j += 1;
            }
        }
    }
    pairs.extend((0..suf).map(|k| (a.len() - suf + k, b.len() - suf + k)));
    let mut blocks: Vec<(usize, usize, usize)> = Vec::new();
    for (x, y) in pairs {
        match blocks.last_mut() {
            Some(bl) if bl.0 + bl.2 == x && bl.1 + bl.2 == y => bl.2 += 1,
            _ => blocks.push((x, y, 1)),
        }
    }
    Ok(blocks)
}

pub const MAX_CELLS: usize = 40_000_000;

/// A change of `base[s..e]` into `repl`.
#[derive(Clone, Debug, PartialEq)]
struct Hunk {
    s: usize,
    e: usize,
    repl: Vec<Value>,
}

fn hunks(base: &[Value], side: &[Value]) -> Res<Vec<Hunk>> {
    let mut out = Vec::new();
    let (mut pa, mut pb) = (0, 0);
    let mut blocks = matching_blocks(base, side)?;
    blocks.push((base.len(), side.len(), 0));
    for (a, b, n) in blocks {
        if a > pa || b > pb {
            out.push(Hunk { s: pa, e: a, repl: side[pb..b].to_vec() });
        }
        pa = a + n;
        pb = b + n;
    }
    Ok(out)
}

fn overlaps(a: &Hunk, b: &Hunk) -> bool {
    a.s.max(b.s) < a.e.min(b.e) || a.s == b.s || (a.s == a.e && b.s < a.s && a.s < b.e) || (b.s == b.e && a.s < b.s && b.s < a.e)
}

fn unique(v: &[Value]) -> bool {
    let set: BTreeSet<String> = v.iter().map(compact).collect();
    set.len() == v.len()
}

// ---------------------------------------------------------------- 3-way merge

#[derive(Clone, Debug, PartialEq)]
pub struct Conflict {
    pub at: String,
    pub what: String,
}

const KEY_FIELDS: [&str; 5] = ["path", "id", "section", "type", "name"];

fn elem_key(v: &Value) -> Option<String> {
    let Value::Obj(_) = v else { return None };
    let parts: Vec<String> = KEY_FIELDS.iter().filter_map(|f| v.field(f).map(|x| format!("{f}={}", compact(x)))).collect();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(","))
    }
}

/// Keys of an array of objects when every element has a unique (path,id,section,type,name) key.
fn keyed(a: &[Value]) -> Option<Vec<String>> {
    let ks: Vec<String> = a.iter().map(elem_key).collect::<Option<_>>()?;
    let set: BTreeSet<&String> = ks.iter().collect();
    (set.len() == ks.len()).then_some(ks)
}

fn path_sorted(a: &[Value]) -> bool {
    a.iter().all(|x| x.field("path").and_then(Value::as_str).is_some())
        && a.windows(2).all(|w| w[0].field("path").and_then(Value::as_str) <= w[1].field("path").and_then(Value::as_str))
}

fn short(v: Option<&Value>) -> String {
    match v {
        None => "(absent)".into(),
        Some(v) => {
            let s = compact(v);
            if s.chars().count() > 120 {
                format!("{}...", s.chars().take(117).collect::<String>())
            } else {
                s
            }
        }
    }
}

/// Merges `ours` (HEAD) and `theirs` (MERGE_HEAD) against `base`. Clean when only one side changed a
/// value, both made the same change, or the changes touch different keys / keyed elements / non-overlapping
/// array regions. Ours' order is kept; theirs' additions go to their sorted place (sorted objects or
/// path-sorted arrays) or after their predecessor. Conflicts are collected; the ours value stands in.
pub struct Merger {
    pub conflicts: Vec<Conflict>,
    /// sha256 values recomputed from the working tree to settle a both-sides hash change.
    pub resolved: Vec<String>,
    /// (repository root, manifest directory) for resolving `path`+`sha256` hash conflicts; None disables it.
    pub files: Option<(PathBuf, PathBuf)>,
}

impl Merger {
    pub fn new(files: Option<(PathBuf, PathBuf)>) -> Merger {
        Merger { conflicts: Vec::new(), resolved: Vec::new(), files }
    }

    fn conflict(&mut self, at: &str, what: String) {
        self.conflicts.push(Conflict { at: at.to_string(), what });
    }

    fn three(&mut self, at: &str, base: Option<&Value>, ours: &Value, theirs: &Value) {
        self.conflict(at, format!("base {} | ours {} | theirs {}", short(base), short(Some(ours)), short(Some(theirs))));
    }

    pub fn merge(&mut self, base: Option<&Value>, ours: &Value, theirs: &Value, at: &str) -> Value {
        if ours == theirs || base == Some(theirs) {
            return ours.clone();
        }
        if base == Some(ours) {
            return theirs.clone();
        }
        match (ours, theirs) {
            (Value::Obj(o), Value::Obj(t)) => {
                let b = match base {
                    Some(Value::Obj(b)) => b.as_slice(),
                    _ => &[],
                };
                Value::Obj(self.merge_obj(b, o, t, at))
            }
            (Value::Arr(o), Value::Arr(t)) => {
                let b = match base {
                    Some(Value::Arr(b)) => b.as_slice(),
                    _ => &[],
                };
                Value::Arr(self.merge_arr(b, o, t, at))
            }
            _ => {
                self.three(at, base, ours, theirs);
                ours.clone()
            }
        }
    }

    fn merge_obj(&mut self, b: &[(Str, Value)], o: &[(Str, Value)], t: &[(Str, Value)], at: &str) -> Vec<(Str, Value)> {
        let find = |m: &[(Str, Value)], k: &str| m.iter().find(|(x, _)| x.s == k).map(|(_, v)| v.clone());
        let mut out: Vec<(Str, Value)> = Vec::new();
        for (k, ov) in o {
            let here = format!("{at}.{}", k.s);
            let bv = find(b, &k.s);
            match find(t, &k.s) {
                Some(tv) => {
                    if k.s == "sha256" && ov != &tv && bv.as_ref() != Some(ov) && bv.as_ref() != Some(&tv) {
                        if let Some(h) = self.rehash(o, &here) {
                            out.push((k.clone(), Value::string(h)));
                            continue;
                        }
                    }
                    let v = self.merge(bv.as_ref(), ov, &tv, &here);
                    out.push((k.clone(), v));
                }
                None => match bv {
                    None => out.push((k.clone(), ov.clone())),
                    Some(bv) if &bv == ov => {}
                    Some(bv) => {
                        self.conflict(&here, format!("deleted by theirs, changed by ours: base {} | ours {}", short(Some(&bv)), short(Some(ov))));
                        out.push((k.clone(), ov.clone()));
                    }
                },
            }
        }
        for (ti, (k, tv)) in t.iter().enumerate() {
            if o.iter().any(|(x, _)| x.s == k.s) {
                continue;
            }
            match find(b, &k.s) {
                None => {
                    let sorted = out.windows(2).all(|w| w[0].0.s <= w[1].0.s);
                    if sorted {
                        insert_key(&mut out, k.clone(), tv.clone());
                    } else {
                        let after = t[..ti].iter().rev().find_map(|(p, _)| out.iter().position(|(x, _)| x.s == p.s));
                        out.insert(after.map_or(out.len(), |i| i + 1), (k.clone(), tv.clone()));
                    }
                }
                Some(bv) if &bv == tv => {}
                Some(bv) => self.conflict(
                    &format!("{at}.{}", k.s),
                    format!("deleted by ours, changed by theirs: base {} | theirs {}", short(Some(&bv)), short(Some(tv))),
                ),
            }
        }
        out
    }

    /// Settle a both-sides sha256 change of an object with a `path` by hashing the working-tree file.
    fn rehash(&mut self, obj: &[(Str, Value)], at: &str) -> Option<String> {
        let (repo, dir) = self.files.as_ref()?;
        let p = obj.iter().find(|(k, _)| k.s == "path").and_then(|(_, v)| v.as_str())?;
        let file = resolve_bound(repo, dir, p)?;
        let h = sha_hex(&std::fs::read(&file).ok()?);
        self.resolved.push(format!("{at}: both sides changed; recomputed from {}", file.display()));
        Some(h)
    }

    fn merge_arr(&mut self, b: &[Value], o: &[Value], t: &[Value], at: &str) -> Vec<Value> {
        if let (Some(ok), Some(tk), Some(bk)) = (keyed(o), keyed(t), keyed(b)) {
            return self.merge_keyed(b, &bk, o, &ok, t, &tk, at);
        }
        self.merge_seq(b, o, t, at)
    }

    #[allow(clippy::too_many_arguments)]
    fn merge_keyed(&mut self, b: &[Value], bk: &[String], o: &[Value], ok: &[String], t: &[Value], tk: &[String], at: &str) -> Vec<Value> {
        let bmap: BTreeMap<&str, &Value> = bk.iter().map(String::as_str).zip(b).collect();
        let tmap: BTreeMap<&str, &Value> = tk.iter().map(String::as_str).zip(t).collect();
        let oset: BTreeSet<&str> = ok.iter().map(String::as_str).collect();
        let mut out: Vec<(String, Value)> = Vec::new();
        for (k, ov) in ok.iter().zip(o) {
            let here = format!("{at}[{k}]");
            match tmap.get(k.as_str()) {
                Some(tv) => {
                    let v = self.merge(bmap.get(k.as_str()).copied(), ov, tv, &here);
                    out.push((k.clone(), v));
                }
                None => match bmap.get(k.as_str()) {
                    None => out.push((k.clone(), ov.clone())),
                    Some(bv) if *bv == ov => {}
                    Some(bv) => {
                        self.conflict(&here, format!("deleted by theirs, changed by ours: base {} | ours {}", short(Some(bv)), short(Some(ov))));
                        out.push((k.clone(), ov.clone()));
                    }
                },
            }
        }
        let sorted = path_sorted(o);
        for (ti, (k, tv)) in tk.iter().zip(t).enumerate() {
            if oset.contains(k.as_str()) {
                continue;
            }
            match bmap.get(k.as_str()) {
                None => {
                    let i = if sorted && tv.field("path").and_then(Value::as_str).is_some() {
                        let p = tv.field("path").and_then(Value::as_str);
                        out.iter().position(|(_, x)| x.field("path").and_then(Value::as_str) > p).unwrap_or(out.len())
                    } else {
                        let after = tk[..ti].iter().rev().find_map(|p| out.iter().position(|(x, _)| x == p));
                        after.map_or(out.len(), |i| i + 1)
                    };
                    out.insert(i, (k.clone(), tv.clone()));
                }
                Some(bv) if *bv == tv => {}
                Some(bv) => self.conflict(
                    &format!("{at}[{k}]"),
                    format!("deleted by ours, changed by theirs: base {} | theirs {}", short(Some(bv)), short(Some(tv))),
                ),
            }
        }
        out.into_iter().map(|(_, v)| v).collect()
    }

    /// diff3 over arrays: non-overlapping hunks of both sides apply; identical hunks apply once; two pure
    /// insertions at one point concatenate (ours first) only when the whole result keeps unique elements.
    fn merge_seq(&mut self, b: &[Value], o: &[Value], t: &[Value], at: &str) -> Vec<Value> {
        let (ho, ht) = match (hunks(b, o), hunks(b, t)) {
            (Ok(x), Ok(y)) => (x, y),
            (Err(e), _) | (_, Err(e)) => {
                self.conflict(at, format!("array diff: {e}"));
                return o.to_vec();
            }
        };
        let mut chosen: Vec<Hunk> = Vec::new();
        let (mut i, mut j) = (0, 0);
        let mut joined = false;
        while i < ho.len() || j < ht.len() {
            let take_o = j == ht.len() || (i < ho.len() && (ho[i].s, ho[i].e) <= (ht[j].s, ht[j].e));
            let h = if take_o { &ho[i] } else { &ht[j] };
            let other = if take_o { ht.get(j) } else { ho.get(i) };
            match other {
                Some(x) if overlaps(h, x) => {
                    if h == x {
                        chosen.push(h.clone());
                    } else if h.s == h.e && x.s == x.e && h.s == x.s {
                        let (oh, th) = if take_o { (h, x) } else { (x, h) };
                        let mut repl = oh.repl.clone();
                        repl.extend(th.repl.iter().cloned());
                        chosen.push(Hunk { s: h.s, e: h.e, repl });
                        joined = true;
                    } else {
                        self.conflict(at, format!("overlapping array changes: base {} | ours {} | theirs {}", short(Some(&Value::Arr(b.to_vec()))), short(Some(&Value::Arr(o.to_vec()))), short(Some(&Value::Arr(t.to_vec())))));
                        return o.to_vec();
                    }
                    i += 1;
                    j += 1;
                }
                _ => {
                    chosen.push(h.clone());
                    if take_o {
                        i += 1
                    } else {
                        j += 1
                    }
                }
            }
        }
        let mut out = Vec::new();
        let mut pos = 0;
        for h in &chosen {
            out.extend_from_slice(&b[pos..h.s]);
            out.extend(h.repl.iter().cloned());
            pos = h.e;
        }
        out.extend_from_slice(&b[pos..]);
        if joined && !unique(&out) {
            self.conflict(at, format!("both sides inserted at one place and the joined array repeats an element: ours {} | theirs {}", short(Some(&Value::Arr(o.to_vec()))), short(Some(&Value::Arr(t.to_vec())))));
            return o.to_vec();
        }
        out
    }
}

/// A bound path: `./` or `../` paths are relative to the manifest directory; others are tried at the
/// repository root first, then the manifest directory. A `#fragment` is ignored.
pub fn resolve_bound(repo: &Path, dir: &Path, p: &str) -> Option<PathBuf> {
    let p = p.split('#').next().unwrap_or(p);
    if p.is_empty() {
        return None;
    }
    let cands: Vec<PathBuf> =
        if p.starts_with("./") || p.starts_with("../") { vec![dir.join(p)] } else { vec![repo.join(p), dir.join(p)] };
    cands.into_iter().find(|c| c.is_file())
}

/// Refresh every `{path, sha256}` object from the working tree; returns (changed, unresolved paths).
pub fn rebind_all(v: &mut Value, repo: &Path, dir: &Path) -> (usize, Vec<String>) {
    let mut changed = 0;
    let mut missing = Vec::new();
    walk_rebind(v, repo, dir, &mut changed, &mut missing);
    (changed, missing)
}

fn walk_rebind(v: &mut Value, repo: &Path, dir: &Path, changed: &mut usize, missing: &mut Vec<String>) {
    match v {
        Value::Obj(m) => {
            let p = m.iter().find(|(k, _)| k.s == "path").and_then(|(_, v)| v.as_str()).map(str::to_string);
            if let (Some(p), Some(slot)) = (p, m.iter_mut().find(|(k, v)| k.s == "sha256" && v.as_str().is_some())) {
                match resolve_bound(repo, dir, &p).and_then(|f| std::fs::read(f).ok()) {
                    Some(bytes) => {
                        let h = sha_hex(&bytes);
                        if slot.1.as_str() != Some(h.as_str()) {
                            slot.1 = Value::string(h);
                            *changed += 1;
                        }
                    }
                    None => missing.push(p),
                }
            }
            for (_, x) in m.iter_mut() {
                walk_rebind(x, repo, dir, changed, missing);
            }
        }
        Value::Arr(a) => a.iter_mut().for_each(|x| walk_rebind(x, repo, dir, changed, missing)),
        _ => {}
    }
}

pub struct MergeReport {
    pub conflicts: Vec<Conflict>,
    pub resolved: Vec<String>,
    pub rebound: usize,
    pub unresolved_paths: Vec<String>,
    pub written: bool,
}

/// Merge FILE (path relative to the current directory) inside a git merge in progress and write it to the
/// working tree when there is no conflict. The layout comes from HEAD's version (MERGE_HEAD's if HEAD's
/// is not reproduced byte for byte).
pub fn merge3_file(file: &str, rebind_every: bool, reformat: bool) -> Res<MergeReport> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let root = PathBuf::from(git_text(&cwd, &["rev-parse", "--show-toplevel"])?.trim());
    let pre = git_text(&cwd, &["rev-parse", "--show-prefix"])?.trim().to_string();
    git(&root, &["rev-parse", "-q", "--verify", "MERGE_HEAD"]).map_err(|_| "no merge in progress (MERGE_HEAD missing)".to_string())?;
    let rel = format!("{pre}{file}");
    let base_rev = git_text(&root, &["merge-base", "HEAD", "MERGE_HEAD"])?.trim().to_string();
    let show = |rev: &str| -> Res<Option<String>> {
        match git(&root, &["show", &format!("{rev}:{rel}")]) {
            Ok(b) => String::from_utf8(b).map(Some).map_err(|_| format!("{rev}:{rel} is not UTF-8")),
            Err(_) => Ok(None),
        }
    };
    let ours_t = show("HEAD")?.ok_or_else(|| format!("HEAD has no {rel}"))?;
    let theirs_t = show("MERGE_HEAD")?.ok_or_else(|| format!("MERGE_HEAD has no {rel}"))?;
    let base_d = match show(&base_rev)? {
        Some(t) => Some(load_str(&t).map_err(|e| format!("base {rel}: {e}"))?),
        None => None,
    };
    let ours = load_str(&ours_t).map_err(|e| format!("HEAD {rel}: {e}"))?;
    let theirs = load_str(&theirs_t).map_err(|e| format!("MERGE_HEAD {rel}: {e}"))?;
    let style = if ours.canonical {
        ours.style
    } else if theirs.canonical {
        theirs.style
    } else if reformat {
        ours.style
    } else {
        return Err(format!("{rel}: neither side's layout is reproduced byte for byte; rerun with --reformat"));
    };
    let path = root.join(&rel);
    let dir = path.parent().unwrap_or(&root).to_path_buf();
    let mut m = Merger::new(Some((root.clone(), dir.clone())));
    let mut merged = m.merge(base_d.as_ref().map(|d| &d.value), &ours.value, &theirs.value, "$");
    let (mut rebound, mut unresolved) = (0, Vec::new());
    if rebind_every && m.conflicts.is_empty() {
        (rebound, unresolved) = rebind_all(&mut merged, &root, &dir);
    }
    let mut written = false;
    if m.conflicts.is_empty() {
        let text = render(&merged, style);
        if std::fs::read(&path).ok().as_deref() != Some(text.as_bytes()) {
            write_atomic(&path, text.as_bytes())?;
            written = true;
        }
    }
    Ok(MergeReport { conflicts: m.conflicts, resolved: m.resolved, rebound, unresolved_paths: unresolved, written })
}

// ---------------------------------------------------------------- source bindings

/// Refresh `source_bindings[].sha256` of one manifest for the given repository paths; with `add`, insert
/// missing bindings at their sorted place. Returns the number of bindings changed or added.
pub fn rebind_manifest(repo: &Path, manifest: &Path, paths: &[String], add: bool, reformat: bool) -> Res<usize> {
    let mut doc: Doc = load_for_edit(manifest, reformat)?;
    let at = manifest.display().to_string();
    let Some(Value::Arr(b)) = doc.value.field_mut("source_bindings") else {
        return Err(format!("{at}: no source_bindings array"));
    };
    let mut changed = 0;
    for p in paths {
        let file = repo.join(p);
        let idx = b.iter().position(|x| x.field("path").and_then(Value::as_str) == Some(p.as_str()));
        match idx {
            Some(i) => {
                let bytes = std::fs::read(&file).map_err(|e| format!("{at}: bound {p} unreadable: {e}"))?;
                let h = sha_hex(&bytes);
                let slot = b[i].field_mut("sha256").ok_or_else(|| format!("{at}: binding {p} has no sha256"))?;
                if slot.as_str() != Some(h.as_str()) {
                    *slot = Value::string(h);
                    changed += 1;
                }
            }
            None if add => {
                let bytes = std::fs::read(&file).map_err(|e| format!("{at}: cannot add {p}: {e}"))?;
                let i = b.iter().position(|x| x.field("path").and_then(Value::as_str).is_some_and(|q| q > p.as_str())).unwrap_or(b.len());
                b.insert(i, Value::Obj(vec![(Str::new("path"), Value::string(p.clone())), (Str::new("sha256"), Value::string(sha_hex(&bytes)))]));
                changed += 1;
            }
            None => {}
        }
    }
    if changed > 0 {
        save(manifest, &doc)?;
    }
    Ok(changed)
}

// ---------------------------------------------------------------- groupant byte ranges

fn line_starts(t: &[u8]) -> Vec<usize> {
    let mut o = vec![0];
    for (i, c) in t.iter().enumerate() {
        if *c == b'\n' && i + 1 < t.len() {
            o.push(i + 1);
        }
    }
    o.push(t.len());
    if t.is_empty() {
        o.truncate(1);
    }
    o
}

fn lines(t: &[u8], starts: &[usize]) -> Vec<Vec<u8>> {
    starts.windows(2).map(|w| t[w[0]..w[1]].to_vec()).collect()
}

/// Byte blocks (old, new, len) of unchanged lines plus the end sentinel.
pub fn byte_blocks(old: &[u8], new: &[u8]) -> Res<Vec<(usize, usize, usize)>> {
    let (os, ns) = (line_starts(old), line_starts(new));
    let (ol, nl) = (lines(old, &os), lines(new, &ns));
    let mut out: Vec<(usize, usize, usize)> =
        matching_blocks(&ol, &nl)?.into_iter().map(|(a, b, n)| (os[a], ns[b], os[a + n] - os[a])).collect();
    out.push((old.len(), new.len(), 0));
    Ok(out)
}

/// Map an old byte offset to the new text: exact inside unchanged lines; otherwise a start goes to the end
/// of the previous unchanged block and an end to the start of the next one.
pub fn map_offset(blocks: &[(usize, usize, usize)], o: usize, end: bool) -> usize {
    for &(a, b, n) in blocks {
        if (a <= o && o < a + n) || (end && n > 0 && o == a + n) {
            return b + (o - a);
        }
    }
    if end {
        blocks.iter().filter(|x| x.0 >= o).min_by_key(|x| x.0).map_or(0, |x| x.1)
    } else {
        blocks.iter().filter(|x| x.0 + x.2 <= o).max_by_key(|x| x.0).map_or(0, |x| x.1 + x.2)
    }
}

fn manifests_under(dir: &Path, out: &mut Vec<PathBuf>) -> Res<()> {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            manifests_under(&p, out)?;
        } else if p.file_name().is_some_and(|n| n == "manifest.json") {
            out.push(p);
        }
    }
    Ok(())
}

/// Rebind one changed Rust source against BASE: its sidecar manifests' source hashes, coverage byte ranges
/// and covered-bytes hashes, and every tracked `*.groupant/**/manifest.json` binding `{path: FILE,
/// sha256: old}`. Returns report lines; a fragment whose covered text changed is marked TEXT CHANGED.
pub fn rebind_groupant(repo: &Path, base: &str, file: &str, reformat: bool) -> Res<Vec<String>> {
    let old = git(repo, &["show", &format!("{base}:{file}")])?;
    let new = std::fs::read(repo.join(file)).map_err(|e| format!("{file}: {e}"))?;
    let (old_sha, new_sha) = (sha_hex(&old), sha_hex(&new));
    let mut report = Vec::new();
    if old_sha == new_sha {
        report.push(format!("{file}: unchanged since {base}"));
        return Ok(report);
    }
    let blocks = byte_blocks(&old, &new)?;
    let side = repo.join(format!("{file}.groupant"));
    let mut ms = Vec::new();
    if side.is_dir() {
        manifests_under(&side, &mut ms)?;
    } else {
        report.push(format!("{file}: no sidecar {}", side.display()));
    }
    for p in &ms {
        let mut doc = load_for_edit(p, reformat)?;
        let v = &mut doc.value;
        if let Some(s) = v.field_mut("source").and_then(|s| s.field_mut("sha256")) {
            if s.as_str() == Some(old_sha.as_str()) {
                *s = Value::string(new_sha.clone());
            }
        }
        if let Some(cov) = v.field_mut("coverage") {
            // A null byte_range covers the whole file: only the source hash moves.
            let range = cov
                .field("byte_range")
                .filter(|r| !matches!(r, Value::Null))
                .map(|r| (r.field("start").and_then(num), r.field("end").and_then(num)));
            if let Some((Some(s), Some(e))) = range {
                if s > e || e > old.len() {
                    return Err(format!("{}: byte_range [{s},{e}) outside the {}-byte base source", p.display(), old.len()));
                }
                let (ns, ne) = (map_offset(&blocks, s, false), map_offset(&blocks, e, true));
                let ne = ne.max(ns);
                let same = old[s..e] == new[ns..ne];
                let r = cov.field_mut("byte_range").expect("checked");
                *r.field_mut("start").expect("checked") = Value::Num(ns.to_string());
                *r.field_mut("end").expect("checked") = Value::Num(ne.to_string());
                if let Some(h) = cov.field_mut("source_bytes_sha256") {
                    *h = Value::string(sha_hex(&new[ns..ne]));
                }
                report.push(format!("{}: [{s},{e}) -> [{ns},{ne}) {}", p.display(), if same { "same text" } else { "TEXT CHANGED" }));
            } else if range.is_some() {
                return Err(format!("{}: byte_range without integer start/end", p.display()));
            }
        }
        if save(p, &doc)? {
            report.push(format!("{}: written", p.display()));
        }
    }
    let own = format!("{file}.groupant/");
    let listed = git(repo, &["ls-files", "-z"])?;
    for name in listed.split(|c| *c == 0).filter_map(|n| std::str::from_utf8(n).ok()) {
        if !name.ends_with("manifest.json") || !name.contains(".groupant/") || name.starts_with(&own) {
            continue;
        }
        let gp = repo.join(name);
        let Ok(text) = std::fs::read_to_string(&gp) else { continue };
        if !text.contains(&old_sha) {
            continue;
        }
        let mut doc = load_for_edit(&gp, reformat)?;
        let n = bind_walk(&mut doc.value, file, &old_sha, &new_sha);
        if n > 0 && save(&gp, &doc)? {
            report.push(format!("{name}: bound {file} ({n})"));
        }
    }
    Ok(report)
}

fn num(v: &Value) -> Option<usize> {
    match v {
        Value::Num(n) => n.parse().ok(),
        _ => None,
    }
}

fn bind_walk(v: &mut Value, file: &str, old: &str, new: &str) -> usize {
    match v {
        Value::Obj(m) => {
            let mut n = 0;
            let hit = m.iter().any(|(k, x)| k.s == "path" && x.as_str() == Some(file));
            if hit {
                if let Some((_, s)) = m.iter_mut().find(|(k, x)| k.s == "sha256" && x.as_str() == Some(old)) {
                    *s = Value::string(new);
                    n += 1;
                }
            }
            n + m.iter_mut().map(|(_, x)| bind_walk(x, file, old, new)).sum::<usize>()
        }
        Value::Arr(a) => a.iter_mut().map(|x| bind_walk(x, file, old, new)).sum(),
        _ => 0,
    }
}
