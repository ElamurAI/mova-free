//! Run only explicitly supplied benchmark commands; no shell, API calls or automatic search.
use anyhow::{Context, Result, bail, ensure};
use mova_twin::{Model, Prediction};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::process::CommandExt,
    path::{Component, Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Spec {
    schema: u32,
    family: String,
    /// Hardware, source revision, warm/cold regime and other declared boundaries.
    context: BTreeMap<String, String>,
    executable: PathBuf,
    cwd: PathBuf,
    args: Vec<String>,
    env: BTreeMap<String, String>,
    /// Only these environment settings may vary inside a cohort.
    axes: Vec<String>,
    inputs: BTreeMap<String, PathBuf>,
    feature_source: String,
    features: BTreeMap<String, f64>,
    prior: BTreeMap<String, f64>,
    ridge: f64,
    timeout_seconds: u64,
    outputs: Vec<OutputCheck>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OutputCheck {
    /// Relative to the new run directory. stdout.txt/stderr.txt are also available.
    path: PathBuf,
    #[serde(default)]
    contains: Vec<String>,
    sha256: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
struct Fingerprint {
    binary: String,
    inputs: BTreeMap<String, String>,
}
#[derive(Serialize, Deserialize)]
struct Forecast {
    schema: u32,
    created_unix_ms: u128,
    domain: String,
    fingerprint: Fingerprint,
    spec: Spec,
    parent_checkpoint: Option<String>,
    model_before: Model,
    prediction: Prediction,
}
#[derive(Serialize, Deserialize)]
struct Observation {
    schema: u32,
    exit_code: Option<i32>,
    seconds: f64,
    accepted: bool,
    errors: Vec<String>,
    artifacts: BTreeMap<String, String>,
    residual_seconds: f64,
    absolute_relative_error: f64,
}
#[derive(Serialize, Deserialize)]
struct Checkpoint {
    schema: u32,
    domain: String,
    forecast_sha256: String,
    observation_sha256: String,
    model: Model,
}
fn hash_bytes(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash_file(p: &Path) -> Result<String> {
    let mut f = File::open(p).with_context(|| format!("open {}", p.display()))?;
    let mut h = Sha256::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}
fn hash_tree(p: &Path) -> Result<String> {
    let meta = fs::symlink_metadata(p)?;
    ensure!(
        !meta.file_type().is_symlink(),
        "input symlink must be resolved explicitly: {}",
        p.display()
    );
    if meta.is_file() {
        return hash_file(p);
    }
    ensure!(meta.is_dir(), "input must be a regular file/directory");
    let mut entries = BTreeMap::new();
    for e in fs::read_dir(p)? {
        let e = e?;
        entries.insert(
            e.file_name()
                .into_string()
                .map_err(|_| anyhow::anyhow!("non-UTF8 input"))?,
            hash_tree(&e.path())?,
        );
    }
    Ok(hash_bytes(&serde_json::to_vec(&entries)?))
}
fn fingerprint(s: &Spec) -> Result<Fingerprint> {
    Ok(Fingerprint {
        binary: hash_file(&s.executable)?,
        inputs: s
            .inputs
            .iter()
            .map(|(k, p)| Ok((k.clone(), hash_tree(p)?)))
            .collect::<Result<_>>()?,
    })
}
fn json<T: for<'de> Deserialize<'de>>(p: &Path) -> Result<T> {
    serde_json::from_slice(&fs::read(p)?).with_context(|| format!("read {}", p.display()))
}
fn save<T: Serialize>(p: &Path, v: &T) -> Result<()> {
    let mut f = OpenOptions::new().write(true).create_new(true).open(p)?;
    f.write_all(&serde_json::to_vec_pretty(v)?)?;
    f.write_all(b"\n")?;
    f.sync_all()?;
    Ok(())
}
fn validate(s: &Spec) -> Result<()> {
    ensure!(
        s.schema == 1
            && !s.family.is_empty()
            && !s.context.is_empty()
            && !s.feature_source.is_empty(),
        "missing schema/cohort/feature provenance"
    );
    ensure!(
        s.executable.is_absolute() && s.cwd.is_absolute() && s.cwd.is_dir(),
        "absolute executable and existing cwd required"
    );
    ensure!(
        s.inputs.values().all(|p| p.is_absolute()),
        "absolute input paths required"
    );
    ensure!(
        s.features.keys().eq(s.prior.keys()),
        "prior and feature names differ"
    );
    ensure!(
        s.features.values().all(|v| v.is_finite()) && s.features.values().any(|v| *v != 0.),
        "invalid features"
    );
    ensure!(
        s.axes.iter().all(|a| s.env.contains_key(a)),
        "axis absent from explicit environment"
    );
    ensure!(
        s.timeout_seconds > 0 && s.timeout_seconds <= 86400,
        "timeout must be 1..86400 seconds"
    );
    ensure!(
        !s.outputs.is_empty(),
        "at least one output check is required"
    );
    for o in &s.outputs {
        ensure!(
            !o.path.as_os_str().is_empty()
                && o.path
                    .components()
                    .all(|c| matches!(c, Component::Normal(_))),
            "output must be a relative file path"
        );
        ensure!(
            ![
                "forecast.json",
                "observation.json",
                "updated.json",
                "executable"
            ]
            .contains(&o.path.to_string_lossy().as_ref()),
            "reserved output path"
        );
    }
    Model::new(s.prior.values().copied().collect(), s.ridge)?;
    Ok(())
}
fn domain(s: &Spec, f: &Fingerprint) -> Result<String> {
    // Variable settings and feature values are the hypothesis being tested.
    // All fixed settings, input bytes and binary bytes remain part of the boundary.
    let fixed_env: BTreeMap<_, _> = s.env.iter().filter(|(k, _)| !s.axes.contains(k)).collect();
    Ok(hash_bytes(&serde_json::to_vec(&(
        s.schema,
        &s.family,
        &s.context,
        &s.cwd,
        &s.args,
        fixed_env,
        &s.axes,
        &s.feature_source,
        s.features.keys().collect::<Vec<_>>(),
        &s.prior,
        s.ridge,
        &s.outputs,
        f,
    ))?))
}
fn prepare(s: Spec, parent: Option<&Path>) -> Result<Forecast> {
    validate(&s)?;
    let fp = fingerprint(&s)?;
    let d = domain(&s, &fp)?;
    let (m, link) = if let Some(p) = parent {
        let c: Checkpoint = json(&p.join("updated.json"))?;
        ensure!(
            c.schema == 1 && c.domain == d,
            "parent belongs to a different cohort; start a new chain"
        );
        ensure!(
            c.forecast_sha256 == hash_file(&p.join("forecast.json"))?
                && c.observation_sha256 == hash_file(&p.join("observation.json"))?,
            "parent evidence was modified"
        );
        let o: Observation = json(&p.join("observation.json"))?;
        ensure!(o.accepted, "cannot learn from a failed run");
        c.model.validate()?;
        let previous: Forecast = json(&p.join("forecast.json"))?;
        ensure!(
            previous.domain == c.domain && o.schema == 1,
            "inconsistent parent evidence"
        );
        let mut replay = previous.model_before;
        replay.observe(
            &previous.spec.features.values().copied().collect::<Vec<_>>(),
            o.seconds,
        )?;
        ensure!(
            same_model(&replay, &c.model),
            "parent model differs from replayed update"
        );
        (c.model, Some(hash_file(&p.join("updated.json"))?))
    } else {
        (
            Model::new(s.prior.values().copied().collect(), s.ridge)?,
            None,
        )
    };
    let x: Vec<f64> = s.features.values().copied().collect();
    let prediction = m.predict(&x)?;
    Ok(Forecast {
        schema: 1,
        created_unix_ms: SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
        domain: d,
        fingerprint: fp,
        spec: s,
        parent_checkpoint: link,
        model_before: m,
        prediction,
    })
}
fn execute(f: &Forecast, dir: &Path) -> Result<Observation> {
    let s = &f.spec;
    let replace = |v: &str| v.replace("{run}", &dir.to_string_lossy());
    let mut cmd = Command::new(dir.join("executable"));
    cmd.args(s.args.iter().map(|v| replace(v)))
        .current_dir(&s.cwd)
        .env_clear()
        .env("PATH", "/usr/local/bin:/usr/bin:/bin")
        .env("LANG", "C.UTF-8")
        .envs(s.env.iter().map(|(k, v)| (k, replace(v))))
        .stdin(Stdio::null())
        .stdout(File::create(dir.join("stdout.txt"))?)
        .stderr(File::create(dir.join("stderr.txt"))?)
        .process_group(0);
    let start = Instant::now();
    let mut errors = Vec::new();
    let mut code = None;
    match cmd.spawn() {
        Err(e) => errors.push(format!("spawn: {e}")),
        Ok(mut child) => loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    code = status.code();
                    if !status.success() {
                        errors.push(format!("process: {status}"));
                    }
                    break;
                }
                Ok(None) if start.elapsed() < Duration::from_secs(s.timeout_seconds) => {
                    std::thread::sleep(Duration::from_millis(5))
                }
                other => {
                    // The benchmark owns this new process group; stop descendants too.
                    unsafe {
                        libc::kill(-(child.id() as i32), libc::SIGKILL);
                    }
                    let _ = child.wait();
                    errors.push(match other {
                        Err(e) => format!("wait: {e}"),
                        _ => "timeout".into(),
                    });
                    break;
                }
            }
        },
    }
    let seconds = start.elapsed().as_secs_f64();
    let mut artifacts = BTreeMap::new();
    for name in ["stdout.txt", "stderr.txt"] {
        artifacts.insert(name.into(), hash_file(&dir.join(name))?);
    }
    for o in &s.outputs {
        let p = dir.join(&o.path);
        let check = (|| -> Result<()> {
            ensure!(
                p.canonicalize()?.starts_with(dir),
                "output escaped the run directory"
            );
            ensure!(
                p.is_file() && p.metadata()?.len() > 0,
                "missing/empty output {}",
                o.path.display()
            );
            let h = hash_file(&p)?;
            if let Some(expected) = &o.sha256 {
                ensure!(h == *expected, "output hash differs: {}", o.path.display());
            }
            if !o.contains.is_empty() {
                let text = fs::read_to_string(&p)?;
                for needle in &o.contains {
                    ensure!(text.contains(needle), "missing output marker: {needle}");
                }
            }
            artifacts.insert(o.path.to_string_lossy().into_owned(), h);
            Ok(())
        })();
        if let Err(e) = check {
            errors.push(e.to_string());
        }
    }
    match fingerprint(s) {
        Ok(after) if after == f.fingerprint => (),
        Ok(_) => errors.push("inputs or source executable changed during the run".into()),
        Err(e) => errors.push(format!("post-run fingerprint: {e}")),
    }
    if hash_file(&dir.join("executable"))? != f.fingerprint.binary {
        errors.push("run executable changed".into());
    }
    let residual = seconds - f.prediction.seconds;
    Ok(Observation {
        schema: 1,
        exit_code: code,
        seconds,
        accepted: errors.is_empty(),
        errors,
        artifacts,
        residual_seconds: residual,
        absolute_relative_error: residual.abs() / seconds,
    })
}
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("report") {
        ensure!(args.len() > 2, "report needs run directories");
        let mut rows = Vec::new();
        for path in &args[2..] {
            let dir = Path::new(path);
            let f: Forecast = json(&dir.join("forecast.json"))?;
            let o: Observation = json(&dir.join("observation.json"))?;
            if dir.join("updated.json").exists() {
                let c: Checkpoint = json(&dir.join("updated.json"))?;
                ensure!(
                    c.forecast_sha256 == hash_file(&dir.join("forecast.json"))?
                        && c.observation_sha256 == hash_file(&dir.join("observation.json"))?,
                    "evidence changed: {path}"
                );
            }
            rows.push(serde_json::json!({
                "run":path, "family":f.spec.family, "features":f.spec.features,
                "observations_before":f.prediction.observations,
                "outside_observed_box":f.prediction.outside_observed_box,
                "forecast_seconds":f.prediction.seconds,
                "actual_seconds":o.seconds, "absolute_relative_error":o.absolute_relative_error,
                "accepted":o.accepted, "errors":o.errors, "artifacts":o.artifacts
            }));
        }
        println!("{}", serde_json::to_string_pretty(&rows)?);
        return Ok(());
    }
    let (mode, spec_path, dir, parent) = match args.as_slice() {
        [_, mode, spec] if mode == "predict" => (mode, spec, None, None),
        [_, mode, spec, parent] if mode == "predict" => (mode, spec, None, Some(Path::new(parent))),
        [_, mode, spec, dir] if mode == "run" => (mode, spec, Some(Path::new(dir)), None),
        [_, mode, spec, dir, parent] if mode == "run" => {
            (mode, spec, Some(Path::new(dir)), Some(Path::new(parent)))
        }
        _ => bail!(
            "usage: mova-twin predict SPEC.json [PARENT_RUN] | run SPEC.json NEW_RUN [PARENT_RUN]"
        ),
    };
    let f = prepare(json(Path::new(spec_path))?, parent)?;
    if mode == "predict" {
        println!("{}", serde_json::to_string_pretty(&f)?);
        return Ok(());
    }
    let dir = dir.unwrap();
    fs::create_dir(dir).context("run directory must be new; never overwrite evidence")?;
    let dir = dir.canonicalize()?;
    fs::copy(&f.spec.executable, dir.join("executable"))?;
    ensure!(
        hash_file(&dir.join("executable"))? == f.fingerprint.binary,
        "binary changed before snapshot"
    );
    // The forecast is durable before launching the real system.
    save(&dir.join("forecast.json"), &f)?;
    File::open(&dir)?.sync_all()?;
    let o = execute(&f, &dir)?;
    save(&dir.join("observation.json"), &o)?;
    println!("{}", serde_json::to_string_pretty(&o)?);
    ensure!(
        o.accepted,
        "run rejected; evidence retained, model not updated"
    );
    let mut model = f.model_before;
    model.observe(
        &f.spec.features.values().copied().collect::<Vec<_>>(),
        o.seconds,
    )?;
    let c = Checkpoint {
        schema: 1,
        domain: f.domain,
        model,
        forecast_sha256: hash_file(&dir.join("forecast.json"))?,
        observation_sha256: hash_file(&dir.join("observation.json"))?,
    };
    save(&dir.join("updated.json"), &c)?;
    File::open(&dir)?.sync_all()?;
    Ok(())
}

// Permit only serialization-scale roundoff for ledgers created before float_roundtrip.
fn same_model(a: &Model, b: &Model) -> bool {
    let av = a
        .theta
        .iter()
        .chain(a.p.iter().flatten())
        .chain(&a.min)
        .chain(&a.max)
        .chain(std::iter::once(&a.squared_error));
    let bv = b
        .theta
        .iter()
        .chain(b.p.iter().flatten())
        .chain(&b.min)
        .chain(&b.max)
        .chain(std::iter::once(&b.squared_error));
    a.n == b.n
        && av
            .zip(bv)
            .all(|(x, y)| (x - y).abs() <= 1e-12 * (1. + x.abs()))
}
