//! Durable CUDA parser training from reviewed DB snapshots and real parser agreement.
use crate::Result;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::Write,
    os::{fd::AsRawFd, unix::process::CommandExt},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn file_hash(p: &Path) -> Result<String> {
    Ok(hash(&fs::read(p)?))
}
fn field<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    v[k].as_str()
        .ok_or_else(|| format!("trainer: missing string {k}").into())
}
fn path(v: &Value, k: &str) -> Result<PathBuf> {
    let p = PathBuf::from(field(v, k)?);
    if !p.is_absolute() {
        return Err(format!("trainer: {k} must be absolute").into());
    }
    Ok(p)
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn atomic(p: &Path, b: &[u8]) -> Result<()> {
    let t = p.with_extension("peta-tmp");
    let mut f = OpenOptions::new().create_new(true).write(true).open(&t)?;
    f.write_all(b)?;
    f.sync_all()?;
    drop(f);
    fs::rename(t, p)?;
    File::open(p.parent().ok_or("no parent")?)?.sync_all()?;
    Ok(())
}
fn write_json(p: &Path, v: &Value) -> Result<()> {
    atomic(p, &serde_json::to_vec_pretty(v)?)
}
fn load(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn status(r: &Path, s: &str, d: &str, g: Option<&str>) -> Result<()> {
    write_json(
        &r.join("status.json"),
        &json!({"state":s,"detail":d,"generation":g,"heartbeat_unix":now(),"parser_training_only":true}),
    )
}
struct Lock(File);
fn lock(p: &Path) -> Result<Option<Lock>> {
    fs::create_dir_all(p.parent().ok_or("no lock parent")?)?;
    let f = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .open(p)?;
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        let e = std::io::Error::last_os_error();
        if e.kind() == std::io::ErrorKind::WouldBlock {
            return Ok(None);
        }
        return Err(e.into());
    }
    Ok(Some(Lock(f)))
}
fn inherit_gpu_lock(command: &mut Command, fd: std::os::fd::RawFd) {
    // The parent stays CLOEXEC even while unrelated threads spawn other children.
    // Only this GPU child's post-fork descriptor copy survives exec.
    unsafe {
        command.pre_exec(move || {
            let flags = libc::fcntl(fd, libc::F_GETFD);
            if flags < 0 || libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
}
#[derive(Clone, Debug)]
struct Tree {
    id: String,
    text: String,
    block: String,
    words: usize,
}
fn trees(s: &str) -> Result<Vec<Tree>> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for b in s.split("\n\n").filter(|b| !b.trim().is_empty()) {
        let ids: Vec<_> = b
            .lines()
            .filter_map(|l| l.strip_prefix("# sent_id = "))
            .collect();
        let ts: Vec<_> = b
            .lines()
            .filter_map(|l| l.strip_prefix("# text = "))
            .collect();
        if ids.len() != 1
            || ts.len() != 1
            || ids[0].is_empty()
            || ts[0].is_empty()
            || !seen.insert(ids[0].to_owned())
        {
            return Err("trainer: missing/duplicate/joined ID or text".into());
        }
        let mut hs = Vec::new();
        for l in b.lines().filter(|l| !l.starts_with('#') && !l.is_empty()) {
            let c: Vec<_> = l.split('\t').collect();
            if c.len() != 10 {
                return Err("trainer: expected ten columns".into());
            }
            if c[0].contains('-') || c[0].contains('.') {
                continue;
            }
            if c[0].parse::<usize>()? != hs.len() + 1 {
                return Err("trainer: noncontiguous tokens".into());
            }
            let h = c[6].parse::<usize>()?;
            if (h == 0) != (c[7] == "root") {
                return Err("trainer: root label/head mismatch".into());
            }
            hs.push(h);
        }
        let n = hs.len();
        if n == 0 || hs.iter().filter(|&&h| h == 0).count() != 1 {
            return Err("trainer: invalid roots".into());
        }
        for i in 0..n {
            let mut j = i + 1;
            let mut trail = BTreeSet::new();
            while j != 0 {
                if j > n || !trail.insert(j) {
                    return Err("trainer: cycle or out-of-range head".into());
                }
                j = hs[j - 1];
            }
        }
        out.push(Tree {
            id: ids[0].into(),
            text: ts[0].into(),
            block: format!("{}\n\n", b.trim()),
            words: n,
        });
    }
    Ok(out)
}
fn metadata(p: &Path) -> Result<(BTreeSet<String>, BTreeSet<String>)> {
    let s = fs::read_to_string(p)?;
    Ok((
        s.lines()
            .filter_map(|l| l.strip_prefix("# sent_id = ").map(String::from))
            .collect(),
        s.lines()
            .filter_map(|l| l.strip_prefix("# text = ").map(|t| hash(t.as_bytes())))
            .collect(),
    ))
}
fn lesson(id: &str) -> &str {
    id.rsplit_once(':').map(|x| x.0).unwrap_or(id)
}
fn render(ts: &[Tree]) -> String {
    ts.iter().map(|t| t.block.as_str()).collect()
}
fn tree_hashes(ts: &[Tree]) -> BTreeMap<String, String> {
    // Count changed supervision, not changed comments, MISC or provenance alone.
    ts.iter()
        .map(|t| {
            let targets = t
                .block
                .lines()
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .filter_map(|l| {
                    let c: Vec<_> = l.split('\t').collect();
                    c[0].parse::<usize>()
                        .ok()
                        .map(|_| [c[1], c[3], c[4], c[6], c[7]].join("\t"))
                })
                .collect::<Vec<_>>()
                .join("\n");
            (t.id.clone(), hash(targets.as_bytes()))
        })
        .collect()
}
fn cfg(m: &Path) -> Result<BTreeMap<String, String>> {
    let mut v = BTreeMap::new();
    for l in fs::read_to_string(m.join("cfg.txt"))?.lines() {
        let (k, x) = l.split_once('=').ok_or("trainer: malformed cfg")?;
        if v.insert(k.into(), x.into()).is_some() {
            return Err("trainer: duplicate cfg key".into());
        }
    }
    v.entry("gelu".into()).or_insert("erf".into());
    Ok(v)
}
fn vocabulary(ts: &[Tree], m: &Path) -> Result<()> {
    for (n, k) in [("xpos.txt", 4), ("upos.txt", 3), ("rels.txt", 7)] {
        let v: BTreeSet<_> = fs::read_to_string(m.join(n))?
            .lines()
            .skip(1)
            .map(String::from)
            .collect();
        for t in ts {
            for l in t
                .block
                .lines()
                .filter(|l| !l.starts_with('#') && !l.is_empty())
            {
                let c: Vec<_> = l.split('\t').collect();
                if c[0].contains('-') || c[0].contains('.') {
                    continue;
                }
                if !v.contains(c[k]) {
                    return Err(format!(
                        "trainer: unknown/reserved {n} target {} at {}",
                        c[k], t.id
                    )
                    .into());
                }
            }
        }
    }
    Ok(())
}
fn compatible(a: &Path, b: &Path) -> Result<()> {
    if cfg(a)? != cfg(b)? {
        return Err("trainer: candidate cfg differs".into());
    }
    for n in ["words.txt", "xpos.txt", "upos.txt", "rels.txt"] {
        if fs::read(a.join(n))? != fs::read(b.join(n))? {
            return Err(format!("trainer: candidate {n} differs").into());
        }
    }
    if fs::metadata(b.join("model.safetensors"))?.len() == 0 {
        return Err("trainer: empty weights".into());
    }
    Ok(())
}
fn checked(c: &Value, k: &str, h: &str) -> Result<PathBuf> {
    let p = path(c, k)?;
    if file_hash(&p)? != field(c, h)? {
        return Err(format!("trainer: {k} hash mismatch").into());
    }
    Ok(p)
}
fn command(c: &Value, gpu_fd: Option<std::os::fd::RawFd>) -> Result<Command> {
    let executable = path(c, "gparse")?;
    let resolved = executable.canonicalize()?;
    let own = std::env::current_exe()?.canonicalize()?;
    // The legacy alias remains accepted so existing private configurations are unchanged.
    let explicit = c["peta_executable"].as_str().map(PathBuf::from);
    let controller = c["controller_executable"].as_str().map(PathBuf::from);
    for wrapper in std::iter::once(own).chain(explicit).chain(controller) {
        if resolved == wrapper.canonicalize()? || file_hash(&resolved)? == file_hash(&wrapper)? {
            return Err(
                "trainer: gparse must be the native parser, not the Peta GPU-lock wrapper".into(),
            );
        }
    }
    let mut cmd = Command::new(executable);
    cmd.process_group(0);
    for (k, _) in std::env::vars_os() {
        if k.to_string_lossy().starts_with("GP_") {
            cmd.env_remove(k);
        }
    }
    cmd.env_remove("CYCLE_TRAIN_DBS");
    if let Some(fd) = gpu_fd {
        inherit_gpu_lock(&mut cmd, fd);
    }
    Ok(cmd)
}
fn invoke(mut c: Command, log: &Path, r: &Path, phase: &str, g: &str, secs: u64) -> Result<String> {
    let f = OpenOptions::new().create_new(true).write(true).open(log)?;
    c.stdin(Stdio::null())
        .stdout(Stdio::from(f.try_clone()?))
        .stderr(Stdio::from(f));
    let mut child = c.spawn()?;
    let start = Instant::now();
    let mut hb = Instant::now();
    loop {
        if let Some(s) = child.try_wait()? {
            if !s.success() {
                return Err(format!("trainer: {phase} failed({s}); {}", log.display()).into());
            }
            break;
        }
        if start.elapsed() > Duration::from_secs(secs) {
            unsafe {
                libc::kill(-(child.id() as i32), libc::SIGKILL);
            }
            let _ = child.wait();
            return Err(format!("trainer: {phase} timed out; no automatic retry").into());
        }
        if hb.elapsed() > Duration::from_secs(5) {
            status(r, phase, "child active", Some(g))?;
            hb = Instant::now();
        }
        thread::sleep(Duration::from_millis(100));
    }
    Ok(fs::read_to_string(log)?)
}
#[derive(Clone, Debug)]
struct Score {
    words: usize,
    uas: f64,
    las: f64,
    xpos: f64,
    upos: f64,
}
fn scores(s: &str, ps: &[PathBuf]) -> Result<Vec<Score>> {
    let mut out = Vec::new();
    for p in ps {
        let prefix = format!("{}: words ", p.display());
        let rows: Vec<_> = s.lines().filter_map(|l| l.strip_prefix(&prefix)).collect();
        if rows.len() != 1 {
            return Err("trainer: missing/duplicate exact dev metric line".into());
        }
        let f: Vec<_> = rows[0].split([',', ';']).map(str::trim).collect();
        if f.len() != 6 {
            return Err("trainer: malformed dev metrics".into());
        }
        let mut vs = Vec::new();
        for (i, n) in [(1, "UAS "), (2, "LAS "), (3, "XPOS "), (4, "UPOS ")] {
            let x = f[i]
                .strip_prefix(n)
                .ok_or("trainer: metric order")?
                .parse::<f64>()?;
            if !x.is_finite() || !(0.0..=100.0).contains(&x) {
                return Err("trainer: invalid dev score".into());
            }
            vs.push(x);
        }
        let words = f[0].parse()?;
        if words == 0 {
            return Err("trainer: empty dev".into());
        }
        out.push(Score {
            words,
            uas: vs[0],
            las: vs[1],
            xpos: vs[2],
            upos: vs[3],
        });
    }
    Ok(out)
}
#[derive(Debug)]
struct CandidateRejected {
    kind: &'static str,
    detail: String,
}
impl std::fmt::Display for CandidateRejected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.detail)
    }
}
impl std::error::Error for CandidateRejected {}
fn sample(c: &Value) -> Result<f64> {
    let s = if c["gp_sample"].is_null() {
        1.0
    } else {
        c["gp_sample"]
            .as_f64()
            .ok_or("trainer: GP_SAMPLE must be numeric")?
    };
    if !s.is_finite() || s <= 0.0 || s > 1.0 {
        return Err("trainer: GP_SAMPLE must be in (0,1]".into());
    }
    Ok(s)
}
fn gate(b: &[Score], a: &[Score]) -> Result<()> {
    if b.len() != a.len() || b.is_empty() {
        return Err("trainer: score cardinality".into());
    }
    for (i, (b, a)) in b.iter().zip(a).enumerate() {
        if b.words != a.words {
            return Err("trainer: dev token count changed".into());
        }
        if a.las + (if i == 0 { 0.5 } else { 0.0 }) + 1e-8 < b.las {
            return Err(CandidateRejected {
                kind: "development_las_gate",
                detail: format!("trainer: dev LAS regression set{i}: {} to {}", b.las, a.las),
            }
            .into());
        }
    }
    Ok(())
}
fn anchor_gate(c: &Value, after: &[Score]) -> Result<()> {
    if c["ewt_las_floor"].is_null() {
        return Ok(());
    }
    let floor = c["ewt_las_floor"]
        .as_f64()
        .ok_or("trainer: invalid absolute EWT LAS floor")?;
    if !floor.is_finite() || !(0.0..=100.0).contains(&floor) {
        return Err("trainer: invalid absolute EWT LAS floor".into());
    }
    let score = after
        .first()
        .ok_or("trainer: missing fixed EWT developer score")?;
    if score.las + 1e-8 < floor {
        return Err(CandidateRejected {
            kind: "absolute_anchor_gate",
            detail: format!(
                "trainer: absolute EWT LAS anchor failed: {} below {floor}",
                score.las
            ),
        }
        .into());
    }
    Ok(())
}
fn finite_cuda(s: &str, epochs: usize) -> Result<()> {
    if !s
        .lines()
        .any(|l| l.starts_with("model: ") && l.contains("device Cuda"))
    {
        return Err("trainer: CUDA not established".into());
    }
    let rows: Vec<_> = s.lines().filter(|l| l.starts_with("epoch ")).collect();
    if rows.len() != epochs {
        return Err("trainer: wrong epoch count".into());
    }
    for l in &rows {
        let n = l
            .split_once(": loss ")
            .ok_or("trainer: loss missing")?
            .1
            .split(',')
            .next()
            .ok_or("loss")?
            .parse::<f64>()?;
        if !n.is_finite() || n < 0.0 {
            return Err("trainer: nonfinite loss".into());
        }
    }
    if !rows.iter().any(|l| l.ends_with(" *")) {
        return Err("trainer: no saved checkpoint".into());
    }
    Ok(())
}
fn copy_model(a: &Path, b: &Path) -> Result<()> {
    fs::create_dir(b)?;
    for n in [
        "cfg.txt",
        "words.txt",
        "xpos.txt",
        "upos.txt",
        "rels.txt",
        "model.safetensors",
    ] {
        if !fs::symlink_metadata(a.join(n))?.is_file() {
            return Err("trainer: regular model files required".into());
        }
        fs::copy(a.join(n), b.join(n))?;
        File::open(b.join(n))?.sync_all()?;
    }
    File::open(b)?.sync_all()?;
    Ok(())
}
fn install(m: &Path, c: &Path, old: &str, g: &str) -> Result<String> {
    let _l = lock(&m.join(".peta-model.lock"))?.ok_or("trainer: model installer active")?;
    compatible(m, c)?;
    if file_hash(&m.join("model.safetensors"))? != old {
        return Err("trainer: concurrent model change".into());
    }
    if !fs::symlink_metadata(m.join("model.safetensors"))?.is_file() {
        return Err("trainer: regular weights required".into());
    }
    let b = fs::read(c.join("model.safetensors"))?;
    let h = hash(&b);
    let p = m.join(format!(".peta-{g}.part"));
    let mut f = OpenOptions::new().create_new(true).write(true).open(&p)?;
    f.write_all(&b)?;
    f.sync_all()?;
    drop(f);
    if file_hash(&m.join("model.safetensors"))? != old {
        return Err("trainer: model changed before activation".into());
    }
    fs::rename(p, m.join("model.safetensors"))?;
    File::open(m)?.sync_all()?;
    if file_hash(&m.join("model.safetensors"))? != h {
        return Err("trainer: installed hash mismatch".into());
    }
    Ok(h)
}

fn development(
    c: &Value,
    held: &(BTreeSet<String>, BTreeSet<String>),
    snapshot: &Path,
    gen: &Path,
) -> Result<(Vec<PathBuf>, Vec<Tree>)> {
    let defs = c["development"]
        .as_array()
        .ok_or("trainer: pinned development path/hash objects required")?;
    if defs.is_empty() {
        return Err("trainer: fixed development set required".into());
    }
    let mut ps = Vec::new();
    let mut all = Vec::new();
    for (i, d) in defs.iter().enumerate() {
        let p = path(d, "path")?;
        if file_hash(&p)? != field(d, "sha256")? {
            return Err("trainer: development hash mismatch".into());
        }
        let ts = trees(&fs::read_to_string(&p)?)?;
        if ts.is_empty()
            || ts
                .iter()
                .any(|t| held.0.contains(&t.id) || held.1.contains(&hash(t.text.as_bytes())))
        {
            return Err("trainer: empty/protected development material".into());
        }
        let frozen = gen.join(format!("development-{i}.conllu"));
        fs::write(&frozen, render(&ts))?;
        ps.push(frozen);
        all.extend(ts);
    }
    let ts = trees(&fs::read_to_string(snapshot)?)?;
    if ts.is_empty()
        || ts
            .iter()
            .any(|t| held.0.contains(&t.id) || held.1.contains(&hash(t.text.as_bytes())))
    {
        return Err("trainer: empty/protected snapshot dev".into());
    }
    ps.push(snapshot.into());
    all.extend(ts);
    Ok((ps, all))
}
fn baseline_hashes(c: &Value, r: &Path) -> Result<BTreeMap<String, String>> {
    let p = r.join("trained.json");
    if p.exists() {
        return Ok(serde_json::from_value(load(&p)?["train_hashes"].clone())?);
    }
    if c["baseline_train"].is_null() {
        return Ok(BTreeMap::new());
    }
    Ok(tree_hashes(&trees(&fs::read_to_string(checked(
        c,
        "baseline_train",
        "baseline_train_sha256",
    )?)?)?))
}
fn same_tokens(a: &Tree, b: &Tree) -> bool {
    let forms = |t: &Tree| {
        t.block
            .lines()
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .filter_map(|l| {
                let c: Vec<_> = l.split('\t').collect();
                c[0].parse::<usize>().ok().map(|_| c[1].to_string())
            })
            .collect::<Vec<_>>()
    };
    a.id == b.id && a.text == b.text && forms(a) == forms(b)
}
fn agreement(
    c: &Value,
    export: &Value,
    gen: &Path,
    r: &Path,
    g: &str,
    timeout: u64,
) -> Result<PathBuf> {
    let input = path(export, "agreed_input_train_path")?;
    let parserdir = path(export, "parsers_dir")?;
    let reference = trees(&fs::read_to_string(&input)?)?;
    if reference.is_empty() {
        return Err("trainer: no fully witnessed training records".into());
    }
    for name in ["stanza", "trankit", "spacy-trf"] {
        let ws = trees(&fs::read_to_string(
            parserdir.join("out").join(format!("{name}.conllu")),
        )?)?;
        if reference.len() != ws.len() || reference.iter().zip(&ws).any(|(a, b)| !same_tokens(a, b))
        {
            return Err(format!("trainer: {name} identity/token alignment failed").into());
        }
    }
    let output = gen.join("agreed-train.conllu");
    let reports = gen.join("agreement-report");
    let mut cmd = Command::new(path(c, "treeagree")?);
    cmd.process_group(0).env_remove("TREEAGREE_LEX");
    if !c["treeagree_lex"].is_null() {
        cmd.env(
            "TREEAGREE_LEX",
            checked(c, "treeagree_lex", "treeagree_lex_sha256")?,
        );
    }
    cmd.arg("agree")
        .arg(&input)
        .arg(&parserdir)
        .arg(&output)
        .arg(&reports);
    invoke(cmd, &gen.join("agreement.log"), r, "agreement", g, timeout)?;
    let agreed = trees(&fs::read_to_string(&output)?)?;
    if reference.len() != agreed.len()
        || reference
            .iter()
            .zip(&agreed)
            .any(|(a, b)| !same_tokens(a, b))
    {
        return Err("trainer: agreement changed ID/text/tokenization".into());
    }
    write_json(
        &gen.join("agreement.json"),
        &json!({"teacher_input_sha256":file_hash(&input)?,"agreed_train_sha256":file_hash(&output)?,"fully_witnessed_train_ids":reference.len(),"missing_training_witness_ids":export["missing_training_witness_ids"],"algorithm":"existing treeagree with actual stanza/trankit/spacy-trf witnesses","semantic_gold":false}),
    )?;
    Ok(output)
}
/// The id prefixes fresh training may carry: `fresh_train_prefixes` (each a nonempty `name:`), or
/// `school:` alone when absent. A deployment admits more kinds of reviewed annotation explicitly.
fn fresh_prefixes(c: &Value) -> Result<Vec<String>> {
    if c["fresh_train_prefixes"].is_null() {
        return Ok(vec!["school:".into()]);
    }
    let a = c["fresh_train_prefixes"]
        .as_array()
        .ok_or("trainer: fresh_train_prefixes must be an array")?;
    let mut v = Vec::new();
    for p in a {
        let p = p.as_str().ok_or("trainer: fresh_train_prefixes are strings")?;
        if p.len() < 2
            || !p.ends_with(':')
            || !p[..p.len() - 1]
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        {
            return Err("trainer: a fresh_train_prefix is a lowercase name followed by ':'".into());
        }
        v.push(p.to_string());
    }
    if v.is_empty() {
        return Err("trainer: fresh_train_prefixes must not be empty".into());
    }
    Ok(v)
}
fn prepare(
    c: &Value,
    export: &Value,
    gen: &Path,
    model: &Path,
    agreed: &Path,
) -> Result<(Vec<Tree>, Vec<PathBuf>, Value)> {
    let held = metadata(&checked(c, "heldout", "heldout_sha256")?)?;
    let (snapshot_dev, snapshot_train) = (path(export, "dev_path")?, path(export, "train_path")?);
    let (devpaths, devtrees) = development(c, &held, &snapshot_dev, gen)?;
    let dev_ids: BTreeSet<_> = devtrees.iter().map(|t| t.id.clone()).collect();
    let dev_text: BTreeSet<_> = devtrees.iter().map(|t| hash(t.text.as_bytes())).collect();
    let dev_lessons: BTreeSet<_> = devtrees.iter().map(|t| lesson(&t.id).to_string()).collect();
    let max = cfg(model)?
        .get("max_len")
        .ok_or("trainer: max_len missing")?
        .parse::<usize>()?
        .min(256);
    let source: BTreeMap<_, _> = trees(&fs::read_to_string(snapshot_train)?)?
        .into_iter()
        .map(|t| (t.id.clone(), t))
        .collect();
    let prefixes = fresh_prefixes(c)?;
    let mut train = Vec::new();
    let mut excluded = Vec::new();
    for t in trees(&fs::read_to_string(agreed)?)? {
        if !source.get(&t.id).is_some_and(|s| same_tokens(s, &t)) {
            return Err("trainer: agreed record absent from selected DB train view".into());
        }
        if !prefixes.iter().any(|p| t.id.starts_with(p.as_str()))
            || held.0.contains(&t.id)
            || held.1.contains(&hash(t.text.as_bytes()))
            || dev_ids.contains(&t.id)
            || dev_text.contains(&hash(t.text.as_bytes()))
            || dev_lessons.contains(lesson(&t.id))
        {
            return Err("trainer: nonteaching/heldout/dev overlap in fresh training".into());
        }
        if t.words >= max {
            excluded.push(t.id);
        } else {
            train.push(t);
        }
    }
    if train.is_empty() {
        return Err("trainer: empty trainable agreement output".into());
    }
    vocabulary(&train, model)?;
    // The entire selected DB train view is excluded, including currently unwitnessed records.
    let mut replay = Vec::new();
    let mut replay_excluded = 0;
    if !c["replay_train"].is_null() {
        let p = checked(c, "replay_train", "replay_sha256")?;
        for t in trees(&fs::read_to_string(p)?)? {
            if source.contains_key(&t.id)
                || dev_ids.contains(&t.id)
                || held.0.contains(&t.id)
                || dev_text.contains(&hash(t.text.as_bytes()))
                || held.1.contains(&hash(t.text.as_bytes()))
                || dev_lessons.contains(lesson(&t.id))
                || t.id.starts_with("question:")
                || t.id.starts_with("math:")
                || t.words >= max
            {
                replay_excluded += 1;
            } else {
                replay.push(t);
            }
        }
    }
    vocabulary(&replay, model)?;
    let fresh = gen.join("eligible-train.conllu");
    fs::write(&fresh, render(&train))?;
    let mut args = Vec::new();
    if !replay.is_empty() {
        let p = gen.join("replay.conllu");
        fs::write(&p, render(&replay))?;
        args.push(p);
    }
    args.push(fresh);
    let frozen_hashes: BTreeMap<String, String> = args
        .iter()
        .chain(devpaths.iter())
        .map(|p| Ok((p.to_string_lossy().into_owned(), file_hash(p)?)))
        .collect::<Result<_>>()?;
    let info = json!({"fresh_train_ids":train.len(),"replay_ids":replay.len(),"length_excluded":excluded,"replay_excluded":replay_excluded,"train_hashes":tree_hashes(&train),"train_files":args,"development_files":devpaths,"frozen_file_sha256":frozen_hashes,"agreed_train_sha256":file_hash(agreed)?,"historical_checkpoint_exposure":"not undone by current filtering"});
    Ok((train, devpaths, info))
}

fn execute(
    c: &Value,
    export: &Value,
    r: &Path,
    gen: &Path,
    g: &str,
    gpu_fd: Option<std::os::fd::RawFd>,
) -> Result<Value> {
    sample(c)?;
    let model = path(c, "model_dir")?;
    let backup = gen.join("before-model");
    let timeout = c["train_timeout_seconds"]
        .as_u64()
        .unwrap_or(600)
        .clamp(1, 7200);
    let agreed = agreement(c, export, gen, r, g, timeout)?;
    let (fresh, devpaths, info) = prepare(c, export, gen, &model, &agreed)?;
    let previous = baseline_hashes(c, r)?;
    let current_hashes = tree_hashes(&fresh);
    let new_train = current_hashes
        .iter()
        .filter(|(id, h)| previous.get(*id) != Some(*h))
        .count();
    let min = c["min_new_train"].as_u64().unwrap_or(256) as usize;
    if new_train < min && !c["force_train_once"].as_bool().unwrap_or(false) {
        return Ok(
            json!({"outcome":"below_threshold","priority_diagnostic_id":export["priority_diagnostic_id"],"new_train":new_train,"minimum":min,"generation":g,"inputs":info}),
        );
    }
    if !storage_ok(r, c["trainer_max_bytes"].as_u64().unwrap_or(50_000_000_000))? {
        return Err("trainer: storage guard before backup".into());
    }
    copy_model(&model, &backup)?;
    let old = file_hash(&backup.join("model.safetensors"))?;
    let mut cmd = command(c, gpu_fd)?;
    cmd.arg("eval").arg(&backup).args(&devpaths);
    let before = scores(
        &invoke(cmd, &gen.join("baseline.log"), r, "baseline", g, timeout)?,
        &devpaths,
    )?;
    let epochs = c["epochs"].as_u64().unwrap_or(2) as usize;
    if !(1..=10).contains(&epochs) {
        return Err("trainer: epochs must be1..10".into());
    }
    let lr = c["lr"].as_f64().unwrap_or(0.0001);
    if !lr.is_finite() || lr <= 0.0 || lr > 0.001 {
        return Err("trainer: invalid continuation learning rate".into());
    }
    let tokens = c["tokenbudget"].as_u64().unwrap_or(1000);
    if tokens == 0 || tokens > 16000 {
        return Err("trainer: invalid token budget".into());
    }
    let part = gen.join("candidate.part");
    let mut cmd = command(c, gpu_fd)?;
    cmd.arg("train").arg(&part);
    for p in info["train_files"].as_array().ok_or("train_files")? {
        cmd.arg(p.as_str().ok_or("train path")?);
    }
    cmd.arg("--")
        .args(&devpaths)
        .env("GP_INIT", &backup)
        .env("GP_EPOCHS", epochs.to_string())
        .env("GP_SAMPLE", sample(c)?.to_string())
        .env("GP_LR", lr.to_string())
        .env("GP_TOKENS", tokens.to_string())
        .env("GP_SEED", c["seed"].as_u64().unwrap_or(42).to_string())
        .env("GP_DEV_EVERY", "1")
        .env("GP_DEV_QUICK", "256");
    if !storage_ok(r, c["trainer_max_bytes"].as_u64().unwrap_or(50_000_000_000))? {
        return Err("trainer: storage guard before CUDA training".into());
    }
    let start = Instant::now();
    let trainlog = invoke(cmd, &gen.join("training.log"), r, "training", g, timeout)?;
    let seconds = start.elapsed().as_secs_f64();
    finite_cuda(&trainlog, epochs)?;
    compatible(&backup, &part)?;
    let saved = gen.join("candidate");
    fs::rename(part, &saved)?;
    File::open(gen)?.sync_all()?;
    let mut cmd = command(c, gpu_fd)?;
    cmd.arg("eval").arg(&saved).args(&devpaths);
    let after = scores(
        &invoke(
            cmd,
            &gen.join("candidate-eval.log"),
            r,
            "candidate_reload",
            g,
            timeout,
        )?,
        &devpaths,
    )?;
    gate(&before, &after)?;
    anchor_gate(c, &after)?;
    let installed = install(&model, &saved, &old, g)?;
    let sj = |ss: &[Score]| {
        ss.iter()
            .map(|s| json!({"words":s.words,"UAS":s.uas,"LAS":s.las,"XPOS":s.xpos,"UPOS":s.upos}))
            .collect::<Vec<_>>()
    };
    let result = json!({"outcome":"installed","generation":g,"new_train":new_train,"old_weights_sha256":old,"installed_weights_sha256":installed,"training_seconds":seconds,"experiment":{"seed":c["seed"].as_u64().unwrap_or(42),"gp_sample":sample(c)?,"epochs":epochs,"lr":lr},"before":sj(&before),"after":sj(&after),"ewt_las_floor":c["ewt_las_floor"],"inputs":info,"parser_training_only":true,"development_interpretation":"selection-biased silver regression: the same development sets select checkpoints; not heldout generalization"});
    write_json(
        &r.join("trained.json"),
        &json!({"generation":g,"train_hashes":current_hashes,"installed_weights_sha256":installed}),
    )?;
    Ok(result)
}
fn generation_once(
    c: &Value,
    export: &Value,
    r: &Path,
    gpu_fd: Option<std::os::fd::RawFd>,
) -> Result<()> {
    let g = field(export, "generation_sha256")?;
    if g.len() != 64
        || !g
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
    {
        return Err("trainer: invalid generation hash".into());
    }
    let gen = r.join("generations").join(g);
    if gen.exists() {
        if gen.join("receipt.json").exists() {
            status(
                r,
                "already_processed",
                "terminal generation; no retry",
                Some(g),
            )?;
            return Ok(());
        }
        status(
            r,
            "paused_uncertain",
            "reservation lacks terminal receipt; inspect child/model",
            Some(g),
        )?;
        return Err("trainer: uncertain generation".into());
    }
    fs::create_dir(&gen)?;
    write_json(
        &gen.join("reservation.json"),
        &json!({"generation":g,"export":export,"config_sha256":hash(&serde_json::to_vec(c)?),"reserved_unix":now()}),
    )?;
    match execute(c, export, r, &gen, g, gpu_fd) {
        Ok(mut v) => {
            v["priority_diagnostic_id"] = export["priority_diagnostic_id"].clone();
            write_json(&gen.join("receipt.json"), &v)?;
            status(r, field(&v, "outcome")?, "generation completed", Some(g))?;
            Ok(())
        }
        Err(e) => {
            let mut failure = e.to_string();
            if let Some(rejection) = e.downcast_ref::<CandidateRejected>() {
                let audit = (|| -> Result<(String, String)> {
                    let old = file_hash(&gen.join("before-model/model.safetensors"))?;
                    let current = file_hash(&path(c, "model_dir")?.join("model.safetensors"))?;
                    if current != old {
                        return Err("deployed model diverged before quality receipt".into());
                    }
                    Ok((old, file_hash(&gen.join("candidate/model.safetensors"))?))
                })();
                if let Ok((old, candidate_hash)) = audit.as_ref() {
                    write_json(
                        &gen.join("receipt.json"),
                        &json!({"outcome":"rejected","priority_diagnostic_id":export["priority_diagnostic_id"],"reason_kind":rejection.kind,"generation":g,"error":rejection.detail,"automatic_retry":false,"model_unchanged":true,"old_weights_sha256":old,"candidate_weights_sha256":candidate_hash,"experiment":{"seed":c["seed"].as_u64().unwrap_or(42),"gp_sample":sample(c)?,"epochs":c["epochs"].as_u64().unwrap_or(2),"lr":c["lr"].as_f64().unwrap_or(0.0001)},"ewt_las_floor":c["ewt_las_floor"],"candidate_preserved":true}),
                    )?;
                    status(r,"rejected","quality gate rejected candidate; model retained; identical experiment will not retry",Some(g))?;
                    return Ok(());
                }
                if let Err(audit_error) = audit {
                    failure = format!(
                        "{failure}; rejected-candidate resource audit failed: {audit_error}"
                    );
                }
            }
            write_json(
                &gen.join("receipt.json"),
                &json!({"outcome":"failed","generation":g,"error":failure,"automatic_retry":false,"inspect_model_before_recovery":true}),
            )?;
            if !r.join("PAUSE").exists() {
                atomic(
                    &r.join("PAUSE"),
                    b"trainer-failure: inspect retained candidate and model before resuming\n",
                )?;
            }
            status(
                r,
                "failed_paused",
                "candidate retained; manual review required before later generations",
                Some(g),
            )?;
            Ok(())
        }
    }
}
fn storage_ok(root: &Path, limit: u64) -> Result<bool> {
    fn bytes(p: &Path) -> Result<u64> {
        let mut n = 0u64;
        for e in fs::read_dir(p)? {
            let e = e?;
            let kind = e.file_type()?;
            if kind.is_dir() {
                n = n.saturating_add(bytes(&e.path())?)
            } else if kind.is_file() {
                n = n.saturating_add(e.metadata()?.len());
            } else {
                return Err("trainer: special file in run storage".into());
            }
        }
        Ok(n)
    }
    let p = std::ffi::CString::new(root.as_os_str().as_encoded_bytes())?;
    let mut s: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(p.as_ptr(), &mut s) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    Ok(
        (s.f_bavail as u64).saturating_mul(s.f_frsize as u64) >= 10_000_000_000
            && bytes(root)? < limit,
    )
}
/// Poll a stable DB fingerprint, reserve once, agree, gate CUDA training, install checked weights.
pub fn run(c: &Value, once: bool, data: &mut impl crate::TrainingData) -> Result<()> {
    if c["force_train_once"] == true && !once {
        return Err("trainer: force_train_once requires --once".into());
    }
    let runroot = path(c, "run_root")?;
    let r = runroot.join("trainer");
    fs::create_dir_all(r.join("generations"))?;
    fs::create_dir_all(r.join("exports"))?;
    let _owner = lock(&r.join("controller.lock"))?.ok_or("trainer already active")?;
    let identity = hash(&serde_json::to_vec(c)?);
    let p = r.join("config-sha256");
    if p.exists() {
        if fs::read_to_string(p)? != identity {
            return Err("trainer: config changed; use new run root".into());
        }
    } else {
        atomic(&p, identity.as_bytes())?;
    }
    let mut seq = 0u64;
    loop {
        if r.join("STOP").exists() || runroot.join("STOP").exists() {
            status(&r, "stopped", "STOP present", None)?;
            return Ok(());
        }
        if r.join("PAUSE").exists() || runroot.join("PAUSE").exists() {
            status(&r, "paused", "PAUSE present", None)?;
            if once {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(5));
            continue;
        }
        if !storage_ok(
            &r,
            c["trainer_max_bytes"].as_u64().unwrap_or(50_000_000_000),
        )? {
            status(
                &r,
                "paused_storage",
                "10 GB free floor or trainer storage cap reached; evidence retained",
                None,
            )?;
            if once {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(30));
            continue;
        }
        let (current, priority) = match data.priority(c)? {
            crate::PriorityWork::None => (data.status(c)?, None),
            crate::PriorityWork::AwaitingReview { diagnostic_id } => {
                if !crate::evidence::is_sha256(&diagnostic_id) {
                    return Err("trainer: invalid priority diagnostic identity".into());
                }
                write_json(
                    &r.join("status.json"),
                    &json!({
                        "state":"waiting_for_review","priority_diagnostic_id":diagnostic_id,
                        "detail":"urgent diagnostic is not admitted training data; ordinary work waits",
                        "heartbeat_unix":now(),"parser_training_only":true
                    }),
                )?;
                if once {
                    return Ok(());
                }
                thread::sleep(Duration::from_secs(5));
                continue;
            }
            crate::PriorityWork::Reviewed {
                diagnostic_id,
                mut export,
            } => {
                if !crate::evidence::is_sha256(&diagnostic_id) {
                    return Err("trainer: invalid priority diagnostic identity".into());
                }
                export["priority_diagnostic_id"] = json!(diagnostic_id);
                (export.clone(), Some((diagnostic_id, export)))
            }
        };
        let g = field(&current, "generation_sha256")?;
        if !crate::evidence::is_sha256(g) {
            return Err("trainer: invalid polling generation hash".into());
        }
        let gen = r.join("generations").join(g);
        if gen.join("receipt.json").exists() {
            if let Some((id, _)) = &priority {
                let receipt = load(&gen.join("receipt.json"))?;
                if matches!(
                    receipt["outcome"].as_str(),
                    Some("installed" | "rejected" | "below_threshold")
                ) {
                    if receipt["priority_diagnostic_id"] != id.as_str() {
                        return Err(
                            "trainer: priority identity does not match terminal generation".into(),
                        );
                    }
                    data.priority_terminal(c, id, &receipt)?;
                }
            }
            status(
                &r,
                "waiting_for_data",
                "unchanged terminal generation",
                Some(g),
            )?;
            if once {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(
                c["poll_seconds"].as_u64().unwrap_or(30).clamp(1, 300),
            ));
            continue;
        }
        if gen.exists() {
            status(
                &r,
                "paused_uncertain",
                "incomplete generation reservation",
                Some(g),
            )?;
            return Err("trainer: uncertain reservation".into());
        }
        let Some(gpu) = lock(&path(c, "gpu_lock")?)? else {
            status(
                &r,
                "waiting_for_gpu",
                "cooperating parser/trainer owns GPU",
                Some(g),
            )?;
            if once {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(5));
            continue;
        };
        let out = r
            .join("exports")
            .join(format!("{}-{}-{seq}", now(), std::process::id()));
        seq += 1;
        let (id, export) = match priority {
            Some((id, export)) => (Some(id), export),
            None => (None, data.export(c, &out)?),
        };
        generation_once(c, &export, &r, Some(gpu.0.as_raw_fd()))?;
        if let Some(id) = id {
            let receipt = load(
                &r.join("generations")
                    .join(field(&export, "generation_sha256")?)
                    .join("receipt.json"),
            )?;
            if matches!(
                receipt["outcome"].as_str(),
                Some("installed" | "rejected" | "below_threshold")
            ) {
                if receipt["priority_diagnostic_id"] != id.as_str() {
                    return Err(
                        "trainer: priority identity does not match terminal generation".into(),
                    );
                }
                data.priority_terminal(c, &id, &receipt)?;
            }
        }
        if once {
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        os::unix::fs::PermissionsExt,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        dir: PathBuf,
        c: Value,
        export: Value,
        root: PathBuf,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }
    fn tree(id: &str, text: &str) -> String {
        format!(
            "# sent_id = {id}\n# text = {text}\n1\t{text}\t{text}\tNOUN\tNN\t_\t0\troot\t_\t_\n\n"
        )
    }
    fn script(p: &Path, s: &str) {
        fs::write(p, s).unwrap();
        fs::set_permissions(p, fs::Permissions::from_mode(0o700)).unwrap();
    }
    fn fixture(mode: &str) -> Fixture {
        let base = std::env::var_os("CARGO_TARGET_DIR")
            .map(PathBuf::from)
            .map(|p| p.join("peta-test-temp"))
            .unwrap_or_else(std::env::temp_dir);
        fs::create_dir_all(&base).unwrap();
        let dir = base.join(format!(
            "peta-trainer-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        let root = dir.join("trainer");
        fs::create_dir_all(root.join("generations")).unwrap();
        let model = dir.join("model");
        fs::create_dir(&model).unwrap();
        for (n, s) in [
            ("cfg.txt", "max_len=256\ngelu=erf\n"),
            ("words.txt", "_\n<unk>\nFresh\n"),
            ("xpos.txt", "_\nNN\n"),
            ("upos.txt", "_\nNOUN\n"),
            ("rels.txt", "_\nroot\n"),
            ("model.safetensors", "oldweights"),
        ] {
            fs::write(model.join(n), s).unwrap();
        }
        let train = dir.join("train.conllu");
        let dev = dir.join("dev.conllu");
        let fixed = dir.join("fixed-dev.conllu");
        let held = dir.join("protected.conllu");
        fs::write(&train, tree("school:course:train:1", "Fresh")).unwrap();
        fs::write(&dev, tree("school:course:dev:1", "Silver")).unwrap();
        fs::write(&fixed, tree("fixed:ewtdev:1", "Fixed")).unwrap();
        fs::write(&held, tree("held:test:1", "Protected")).unwrap();
        let parsers = dir.join("parsers");
        fs::create_dir_all(parsers.join("out")).unwrap();
        for n in ["stanza", "trankit", "spacy-trf"] {
            fs::copy(&train, parsers.join("out").join(format!("{n}.conllu"))).unwrap();
        }
        let gp = dir.join("fake-gparse");
        let calls = dir.join("calls");
        let device = if mode == "cpu" {
            "Cpu"
        } else {
            "Cuda { gpu_id: 0 }"
        };
        script(&gp,&format!("#!/bin/sh\nset -eu\nprintf '%s\\n' \"$*\" >> '{}'\nif test \"$1\" = train; then\n mkdir \"$2\"\n cp \"$GP_INIT\"/* \"$2\"/\n printf newweights > \"$2/model.safetensors\"\n echo 'model: config, device {device}'\n n=1\n while test \"$n\" -le \"$GP_EPOCHS\"; do echo \"epoch  $n: loss 0.5, 0 s *\"; n=$((n+1)); done\nelse\n m=\"$2\"\n shift 2\n score=90\n if test '{mode}' = regress; then case \"$m\" in */candidate) score=20;; esac; fi\n for p do printf '%s: words 1, UAS 91, LAS %s, XPOS 92, UPOS 93; 1 words/s\\n' \"$p\" \"$score\"; done\nfi\n",calls.display()));
        let agree = dir.join("fake-treeagree");
        script(&agree,"#!/bin/sh\nset -eu\ntest \"$1\" = agree\ncp \"$2\" \"$4\"\nmkdir \"$5\"\nprintf finite > \"$5/agree.md\"\n");
        let c = json!({"run_root":dir,"model_dir":model,"gparse":gp,"treeagree":agree,"heldout":held,"heldout_sha256":file_hash(&held).unwrap(),"development":[{"path":fixed,"sha256":file_hash(&fixed).unwrap()}],"min_new_train":1,"epochs":2,"train_timeout_seconds":5,"gpu_lock":dir.join("gpu.lock")});
        let export = json!({"generation_sha256":"a".repeat(64),"train_path":train,"dev_path":dev,"agreed_input_train_path":train,"parsers_dir":parsers,"missing_training_witness_ids":[]});
        Fixture {
            dir,
            c,
            export,
            root,
        }
    }
    struct FixtureData {
        export: Value,
        poll: String,
        calls: Vec<&'static str>,
    }
    impl crate::TrainingData for FixtureData {
        fn status(&mut self, _: &Value) -> Result<Value> {
            self.calls.push("status");
            Ok(json!({"generation_sha256":self.poll}))
        }
        fn export(&mut self, _: &Value, destination: &Path) -> Result<Value> {
            self.calls.push("export");
            assert_eq!(
                destination.parent().unwrap().file_name().unwrap(),
                "exports"
            );
            Ok(self.export.clone())
        }
    }
    #[test]
    fn public_eel_entrypoint_drives_native_training_and_records_actual_installation() {
        let mut f = fixture("ok");
        let smi = f.dir.join("fake-nvidia-smi");
        script(
            &smi,
            "#!/bin/sh\ncase \"$*\" in *query-compute-apps*) :;; *) echo 0;; esac\n",
        );
        f.c["nvidia_smi"] = json!(smi);
        f.c["ewt_las_floor"] = json!(84.94);
        let mut data = FixtureData {
            // Export may legitimately observe a newer generation than polling.
            export: f.export.clone(),
            poll: "b".repeat(64),
            calls: Vec::new(),
        };
        crate::run(&f.c, true, &mut data).unwrap();
        assert_eq!(data.calls, ["status", "export"]);
        assert_eq!(
            fs::read(f.dir.join("model/model.safetensors")).unwrap(),
            b"newweights"
        );
        let receipt = load(&f.dir.join("eel/receipts/00000000.json")).unwrap();
        let e = crate::evidence::inspect(&receipt).unwrap();
        assert_eq!(
            e.training_evidence,
            crate::evidence::TrainingEvidence::InstallationRecorded
        );
        assert_eq!(e.generation, Some("a".repeat(64)));
        let calls = fs::read_to_string(f.dir.join("calls")).unwrap();
        assert_eq!(calls.lines().count(), 3); // baseline, train, reload
        assert!(calls.lines().nth(1).unwrap().starts_with("train "));
        // The next invocation is cooldown-only: no new export or command replay.
        crate::run(&f.c, true, &mut data).unwrap();
        assert_eq!(data.calls.len(), 2);
        assert_eq!(fs::read_to_string(f.dir.join("calls")).unwrap(), calls);
    }
    #[test]
    fn malformed_poll_identity_is_rejected_before_export_or_native_command() {
        let f = fixture("ok");
        let mut data = FixtureData {
            export: f.export.clone(),
            poll: "../escape".into(),
            calls: Vec::new(),
        };
        assert!(run(&f.c, true, &mut data)
            .unwrap_err()
            .to_string()
            .contains("invalid polling generation"));
        assert_eq!(data.calls, ["status"]);
        assert!(!f.dir.join("calls").exists());
    }
    struct PriorityFixture {
        work: crate::PriorityWork,
        acknowledgements: usize,
        fail_ack: bool,
    }
    impl crate::TrainingData for PriorityFixture {
        fn priority(&mut self, _: &Value) -> Result<crate::PriorityWork> {
            Ok(self.work.clone())
        }
        fn status(&mut self, _: &Value) -> Result<Value> {
            panic!("ordinary status must not outrank urgent work")
        }
        fn export(&mut self, _: &Value, _: &Path) -> Result<Value> {
            panic!("ordinary export must not outrank urgent work")
        }
        fn priority_terminal(&mut self, _: &Value, id: &str, receipt: &Value) -> Result<()> {
            assert_eq!(id, "c".repeat(64));
            assert_eq!(receipt["priority_diagnostic_id"], id);
            assert_eq!(receipt["outcome"], "installed");
            if self.fail_ack {
                return Err("synthetic durable acknowledgement failure".into());
            }
            self.acknowledgements += 1;
            self.work = crate::PriorityWork::None;
            Ok(())
        }
    }
    #[test]
    fn reviewed_priority_export_precedes_ordinary_data_and_is_acknowledged() {
        let f = fixture("ok");
        let mut data = PriorityFixture {
            work: crate::PriorityWork::Reviewed {
                diagnostic_id: "c".repeat(64),
                export: f.export.clone(),
            },
            acknowledgements: 0,
            fail_ack: false,
        };
        run(&f.c, true, &mut data).unwrap();
        assert_eq!(data.acknowledgements, 1);
        assert!(matches!(data.work, crate::PriorityWork::None));
        assert_eq!(
            fs::read_to_string(f.dir.join("calls"))
                .unwrap()
                .lines()
                .count(),
            3
        );
        assert_eq!(
            fs::read(f.dir.join("model/model.safetensors")).unwrap(),
            b"newweights"
        );
    }
    #[test]
    fn unreviewed_priority_work_blocks_ordinary_training_without_claiming_training() {
        let f = fixture("ok");
        let mut data = PriorityFixture {
            work: crate::PriorityWork::AwaitingReview {
                diagnostic_id: "c".repeat(64),
            },
            acknowledgements: 0,
            fail_ack: false,
        };
        run(&f.c, true, &mut data).unwrap();
        assert_eq!(
            load(&f.root.join("status.json")).unwrap()["state"],
            "waiting_for_review"
        );
        assert!(!f.dir.join("calls").exists());
        assert_eq!(
            fs::read(f.dir.join("model/model.safetensors")).unwrap(),
            b"oldweights"
        );
        // Existing Eel receipt handling represents this as deferred, never an installation.
        let receipt = super::super::complete_receipt(&f.dir).unwrap();
        assert_eq!(receipt["outcome"], "deferred");
        assert_eq!(
            crate::evidence::inspect(&receipt)
                .unwrap()
                .training_evidence,
            crate::evidence::TrainingEvidence::NoTrainingRecorded
        );
    }
    #[test]
    fn priority_ack_failure_retains_terminal_receipt_and_does_not_replay_compute() {
        let f = fixture("ok");
        let mut data = PriorityFixture {
            work: crate::PriorityWork::Reviewed {
                diagnostic_id: "c".repeat(64),
                export: f.export.clone(),
            },
            acknowledgements: 0,
            fail_ack: true,
        };
        assert!(run(&f.c, true, &mut data).is_err());
        let calls = fs::read_to_string(f.dir.join("calls")).unwrap();
        let receipt = f
            .root
            .join("generations")
            .join("a".repeat(64))
            .join("receipt.json");
        assert_eq!(load(&receipt).unwrap()["outcome"], "installed");
        assert!(run(&f.c, true, &mut data).is_err()); // idempotent ack retry only
        assert_eq!(fs::read_to_string(f.dir.join("calls")).unwrap(), calls);
    }
    #[test]
    fn priority_below_threshold_acknowledges_without_native_gpu_commands() {
        struct Data {
            export: Value,
            ack: usize,
        }
        impl crate::TrainingData for Data {
            fn priority(&mut self, _: &Value) -> Result<crate::PriorityWork> {
                Ok(crate::PriorityWork::Reviewed {
                    diagnostic_id: "c".repeat(64),
                    export: self.export.clone(),
                })
            }
            fn status(&mut self, _: &Value) -> Result<Value> {
                panic!("ordinary source")
            }
            fn export(&mut self, _: &Value, _: &Path) -> Result<Value> {
                panic!("ordinary source")
            }
            fn priority_terminal(&mut self, _: &Value, id: &str, receipt: &Value) -> Result<()> {
                assert_eq!(id, "c".repeat(64));
                assert_eq!(receipt["priority_diagnostic_id"], id);
                assert_eq!(receipt["outcome"], "below_threshold");
                self.ack += 1;
                Ok(())
            }
        }
        let mut f = fixture("ok");
        f.c["min_new_train"] = json!(10);
        let mut data = Data {
            export: f.export.clone(),
            ack: 0,
        };
        run(&f.c, true, &mut data).unwrap();
        assert_eq!(data.ack, 1);
        assert!(!f.dir.join("calls").exists());
        assert_eq!(
            fs::read(f.dir.join("model/model.safetensors")).unwrap(),
            b"oldweights"
        );
        let receipt = load(
            &f.root
                .join("generations")
                .join("a".repeat(64))
                .join("receipt.json"),
        )
        .unwrap();
        assert_eq!(
            crate::evidence::inspect(&receipt)
                .unwrap()
                .training_evidence,
            crate::evidence::TrainingEvidence::NoTrainingRecorded
        );
    }
    #[test]
    fn ordinary_historical_receipt_cannot_acknowledge_a_different_priority_identity() {
        let f = fixture("ok");
        let gen = f.root.join("generations").join("a".repeat(64));
        fs::create_dir(&gen).unwrap();
        write_json(
            &gen.join("receipt.json"),
            &json!({"outcome":"installed","generation":"a".repeat(64)}),
        )
        .unwrap();
        let mut data = PriorityFixture {
            work: crate::PriorityWork::Reviewed {
                diagnostic_id: "c".repeat(64),
                export: f.export.clone(),
            },
            acknowledgements: 0,
            fail_ack: false,
        };
        let error = run(&f.c, true, &mut data).unwrap_err().to_string();
        assert!(error.contains("priority identity does not match"));
        assert_eq!(data.acknowledgements, 0);
        assert!(!f.dir.join("calls").exists());
    }
    #[test]
    fn storage_cap_is_a_real_negative_gate() {
        let f = fixture("ok");
        assert!(!storage_ok(&f.dir, 0).unwrap());
        // The disk floor is independently environment-dependent, so do not claim its green branch.
    }
    #[test]
    fn native_parser_cannot_resolve_to_peta_wrapper() {
        let mut f = fixture("ok");
        assert!(command(&f.c, None).is_ok());
        f.c["peta_executable"] = f.c["gparse"].clone();
        assert!(command(&f.c, None).is_err());
    }
    #[test]
    fn fresh_train_prefixes_default_to_school_and_admit_only_what_is_named() {
        assert_eq!(fresh_prefixes(&json!({})).unwrap(), vec!["school:".to_string()]);
        assert_eq!(
            fresh_prefixes(&json!({"fresh_train_prefixes":["hard:"]})).unwrap(),
            vec!["hard:".to_string()]
        );
        for bad in [json!([]), json!(["hard"]), json!([":"]), json!(["Hard:"]), json!("hard:")] {
            assert!(fresh_prefixes(&json!({"fresh_train_prefixes":bad})).is_err());
        }
        // red: a hard-case id is refused by default, admitted when named
        let mut f = fixture("prefix");
        let agreed = f.dir.join("hard-agreed.conllu");
        fs::write(&agreed, tree("hard:abc:1", "Fresh")).unwrap();
        let train = f.dir.join("hard-train.conllu");
        fs::write(&train, tree("hard:abc:1", "Fresh")).unwrap();
        f.export["train_path"] = json!(train);
        let gen = f.dir.join("gen-prefix");
        fs::create_dir_all(&gen).unwrap();
        let model = path(&f.c, "model_dir").unwrap();
        let e = prepare(&f.c, &f.export, &gen, &model, &agreed).unwrap_err();
        assert!(e.to_string().contains("nonteaching"));
        f.c["fresh_train_prefixes"] = json!(["school:", "hard:"]);
        let (t, _, _) = prepare(&f.c, &f.export, &gen, &model, &agreed).unwrap();
        assert_eq!(t.len(), 1);
    }
    #[test]
    fn replay_excludes_selected_records_missing_witnesses() {
        let mut f = fixture("ok");
        let agreed = f.dir.join("only-agreed.conllu");
        fs::copy(f.dir.join("train.conllu"), &agreed).unwrap();
        let missing = tree("school:course:missing:1", "MissingWitness");
        fs::write(
            f.dir.join("train.conllu"),
            tree("school:course:train:1", "Fresh") + &missing,
        )
        .unwrap();
        let replay = f.dir.join("replay.conllu");
        fs::write(&replay, missing + &tree("ewt:train:1", "Safe")).unwrap();
        f.c["replay_train"] = json!(replay);
        f.c["replay_sha256"] = json!(file_hash(&replay).unwrap());
        let gen = f.dir.join("prepare");
        fs::create_dir(&gen).unwrap();
        let (_, _, info) = prepare(&f.c, &f.export, &gen, &f.dir.join("model"), &agreed).unwrap();
        assert_eq!(info["fresh_train_ids"], 1);
        assert_eq!(info["replay_ids"], 1);
        assert_eq!(info["replay_excluded"], 1);
    }
    #[test]
    fn sample_defaults_and_bounds_are_explicit() {
        assert_eq!(sample(&json!({})).unwrap(), 1.0);
        assert_eq!(sample(&json!({"gp_sample":0.15})).unwrap(), 0.15);
        for v in [json!(0), json!(-0.1), json!(1.1), json!("NaN")] {
            assert!(sample(&json!({"gp_sample":v})).is_err());
        }
    }
    #[test]
    fn absolute_anchor_prevents_cumulative_local_gate_drift() {
        let score = |las| Score {
            words: 1,
            uas: 90.0,
            las,
            xpos: 90.0,
            upos: 90.0,
        };
        assert!(gate(&[score(85.44)], &[score(85.0)]).is_ok());
        assert!(anchor_gate(&json!({"ewt_las_floor":84.94}), &[score(85.0)]).is_ok());
        assert!(gate(&[score(85.0)], &[score(84.6)]).is_ok());
        let rejected = anchor_gate(&json!({"ewt_las_floor":84.94}), &[score(84.6)]).unwrap_err();
        assert_eq!(
            rejected.downcast_ref::<CandidateRejected>().unwrap().kind,
            "absolute_anchor_gate"
        );
        assert!(anchor_gate(&json!({}), &[score(84.6)]).is_ok());
        assert!(anchor_gate(&json!({"ewt_las_floor":101}), &[score(100.0)]).is_err());
    }
    #[test]
    fn metric_protocol_and_cuda_controls() {
        let ps = vec![PathBuf::from("/dev/known")];
        let line = "/dev/known: words 5, UAS 91, LAS 90, XPOS 92, UPOS 93; 1 words/s\n";
        assert!(scores(&("noise\n".to_owned() + line), &ps).is_ok());
        assert!(scores(&(line.to_owned() + line), &ps).is_err());
        assert!(scores(&line.replace("LAS 90", "LAS NaN"), &ps).is_err());
        assert!(scores("noise", &ps).is_err());
        let s = "model: x device Cuda { gpu_id: 0 }\nepoch  1: loss 0.5, 0 s *\n";
        assert!(finite_cuda(s, 1).is_ok());
        assert!(finite_cuda(&s.replace("Cuda { gpu_id: 0 }", "Cpu"), 1).is_err());
        assert!(finite_cuda(&s.replace("0.5", "NaN"), 1).is_err());
    }
    #[test]
    fn generation_success_and_exact_repeat() {
        let f = fixture("ok");
        generation_once(&f.c, &f.export, &f.root, None).unwrap();
        let gen = f.root.join("generations").join("a".repeat(64));
        let receipt = load(&gen.join("receipt.json")).unwrap();
        assert_eq!(receipt["outcome"], "installed");
        assert_eq!(
            fs::read(f.dir.join("model/model.safetensors")).unwrap(),
            b"newweights"
        );
        let before = fs::read(f.dir.join("calls")).unwrap();
        assert!(!String::from_utf8_lossy(&before).contains("protected.conllu"));
        generation_once(&f.c, &f.export, &f.root, None).unwrap();
        assert_eq!(fs::read(f.dir.join("calls")).unwrap(), before);
        assert!(gen.join("agreement.json").exists());
        assert_eq!(
            fs::read(gen.join("before-model/model.safetensors")).unwrap(),
            b"oldweights"
        );
    }
    #[test]
    fn quality_rejects_keep_old_model_while_resource_failures_pause() {
        for mode in ["cpu", "regress", "malformed"] {
            let f = fixture(mode);
            if mode == "malformed" {
                let p = f.dir.join("fake-gparse");
                let s = fs::read_to_string(&p).unwrap();
                fs::write(p, s.replace("score=90", "score=NaN")).unwrap();
            }
            generation_once(&f.c, &f.export, &f.root, None).unwrap();
            let gen = f.root.join("generations").join("a".repeat(64));
            assert_eq!(
                load(&gen.join("receipt.json")).unwrap()["outcome"],
                if mode == "regress" {
                    "rejected"
                } else {
                    "failed"
                }
            );
            assert_eq!(
                fs::read(f.dir.join("model/model.safetensors")).unwrap(),
                b"oldweights"
            );
            assert_eq!(f.root.join("PAUSE").exists(), mode != "regress");
            let calls = fs::read(f.dir.join("calls")).unwrap();
            generation_once(&f.c, &f.export, &f.root, None).unwrap();
            assert_eq!(calls, fs::read(f.dir.join("calls")).unwrap());
        }
    }
    #[test]
    fn replay_filter_and_unknown_target() {
        let mut f = fixture("ok");
        let gen = f.dir.join("prepare");
        fs::create_dir(&gen).unwrap();
        let p = f.dir.join("replay.conllu");
        let rows = tree("school:course:train:1", "Fresh")
            + &tree("question:exam:1", "Question")
            + &tree("school:course:dev:2", "OtherDevLesson")
            + &tree("other:alias:1", "Protected")
            + &tree("ewt:train:1", "Safe");
        fs::write(&p, rows).unwrap();
        f.c["replay_train"] = json!(p);
        f.c["replay_sha256"] = json!(file_hash(&p).unwrap());
        let (_, _, info) = prepare(
            &f.c,
            &f.export,
            &gen,
            &f.dir.join("model"),
            &f.dir.join("train.conllu"),
        )
        .unwrap();
        assert_eq!(info["replay_ids"], 1);
        assert_eq!(info["replay_excluded"], 4);
        let t = tree("school:course:train:1", "Fresh").replace("\tNN\t", "\tBAD\t");
        assert!(vocabulary(&trees(&t).unwrap(), &f.dir.join("model")).is_err());
    }
    #[test]
    fn below_threshold_does_not_copy_weights_or_launch_gpu() {
        let mut f = fixture("ok");
        f.c["min_new_train"] = json!(2);
        generation_once(&f.c, &f.export, &f.root, None).unwrap();
        let gen = f.root.join("generations").join("a".repeat(64));
        assert_eq!(
            load(&gen.join("receipt.json")).unwrap()["outcome"],
            "below_threshold"
        );
        assert!(!gen.join("before-model").exists());
        assert!(!f.dir.join("calls").exists());
    }
    #[test]
    fn uncertain_generation_never_submits_child() {
        let f = fixture("ok");
        fs::create_dir(f.root.join("generations").join("a".repeat(64))).unwrap();
        assert!(generation_once(&f.c, &f.export, &f.root, None).is_err());
        assert!(!f.dir.join("calls").exists());
    }
    #[test]
    fn supervision_signature_ignores_metadata_but_not_labels() {
        let a = trees(&tree("school:x:1", "Fresh")).unwrap();
        let b =
            trees(&("# annotation = changed\n".to_owned() + &tree("school:x:1", "Fresh"))).unwrap();
        assert_eq!(tree_hashes(&a), tree_hashes(&b));
        let c = trees(&tree("school:x:1", "Fresh").replace("\tNN\t", "\tNNP\t")).unwrap();
        assert_ne!(tree_hashes(&a), tree_hashes(&c));
    }
    #[test]
    fn parallel_unrelated_child_never_inherits_gpu_lease() {
        let f = fixture("ok");
        let p = f.dir.join("parallel-lease");
        let owner = lock(&p).unwrap().unwrap();
        let fd = owner.0.as_raw_fd();
        let gate = std::sync::Arc::new(std::sync::Barrier::new(2));
        let a = gate.clone();
        let gpu = std::thread::spawn(move || {
            let mut c = Command::new("/bin/sh");
            c.args(["-c", "read _"]).stdin(Stdio::piped());
            inherit_gpu_lock(&mut c, fd);
            a.wait();
            c.spawn().unwrap()
        });
        let other = std::thread::spawn(move || {
            gate.wait();
            Command::new("/bin/sh")
                .args(["-c", "read _"])
                .stdin(Stdio::piped())
                .spawn()
                .unwrap()
        });
        let mut gpu = gpu.join().unwrap();
        let mut other = other.join().unwrap();
        assert_ne!(
            unsafe { libc::fcntl(fd, libc::F_GETFD) } & libc::FD_CLOEXEC,
            0
        );
        drop(owner);
        let gpu_held = lock(&p).unwrap().is_none();
        drop(gpu.stdin.take());
        gpu.wait().unwrap();
        let unrelated_alive = other.try_wait().unwrap().is_none();
        let reclaimed = lock(&p).unwrap().is_some();
        drop(other.stdin.take());
        other.wait().unwrap();
        assert!(gpu_held);
        assert!(unrelated_alive);
        assert!(reclaimed, "unrelated live child must not keep GPU lease");
    }
    #[test]
    fn inherited_gpu_lease_survives_parent_descriptor_close() {
        let f = fixture("ok");
        let p = f.dir.join("lease");
        let owner = lock(&p).unwrap().unwrap();
        let mut command = Command::new("/bin/sh");
        command.args(["-c", "read _"]).stdin(Stdio::piped());
        inherit_gpu_lock(&mut command, owner.0.as_raw_fd());
        assert_ne!(
            unsafe { libc::fcntl(owner.0.as_raw_fd(), libc::F_GETFD) } & libc::FD_CLOEXEC,
            0
        );
        let mut child = command.spawn().unwrap();
        drop(owner);
        let held = lock(&p).unwrap().is_none();
        drop(child.stdin.take());
        child.wait().unwrap();
        assert!(held);
        assert!(lock(&p).unwrap().is_some());
    }
}
