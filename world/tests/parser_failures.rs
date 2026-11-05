//! CLI failures must not look like successful school measurements.
use std::path::PathBuf;
use std::process::Command;

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("mova-parser-test-{}-{stamp}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        Self(dir)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

fn world(s: &Scratch, model: &std::path::Path, mode: &str) -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_world"));
    c.current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("WORLD_EN_MODEL", model)
        .env("MOVA_HOT_ROOT", s.0.join("hot"))
        .env("MOVA_EXEC", mode)
        .env_remove("MOVA_STATE")
        .env_remove("PRAG_OFF");
    c
}

fn rejects_bad_models(mode: &str) {
    let s = Scratch::new();
    let missing = s.0.join("missing.bin");
    let corrupt = s.0.join("corrupt.bin");
    std::fs::write(&corrupt, b"not a parser model").unwrap();
    let trace = s.0.join("trace.tsv");
    let diagnosis = s.0.join("diagnosis.md");
    for model in [&missing, &corrupt] {
        for args in [
            vec!["knowledge", "course", "mysticism", "--trace", trace.to_str().unwrap()],
            vec!["knowledge", "trace", "mysticism", "1", "1"],
            vec!["knowledge", "diagnose", diagnosis.to_str().unwrap(), "mysticism"],
        ] {
            let out = world(&s, model, mode).args(&args).output().unwrap();
            assert!(!out.status.success(), "{mode}: {args:?} silently accepted {}", model.display());
            let err = String::from_utf8_lossy(&out.stderr);
            assert!(err.contains("en model") && err.contains(model.to_str().unwrap()), "{err}");
            assert!(out.stdout.is_empty(), "failed parsing produced a report");
            assert!(!trace.exists() && !diagnosis.exists(), "failed parsing wrote a report");
            assert!(!s.0.join("hot/school").exists(), "failed parsing saved a score");
        }
    }
    let out = world(&s, &missing, mode).args(["knowledge", "show", "mysticism", "1"]).output().unwrap();
    assert!(out.status.success(), "showing lesson data must not require a model");
    assert!(!out.stdout.is_empty());
}

#[test]
fn cpu_school_rejects_missing_and_corrupt_models() { rejects_bad_models("cpu"); }

#[cfg(feature = "gpu")]
#[test]
fn gpu_school_rejects_missing_and_corrupt_en_models() { rejects_bad_models("gpu"); }

#[test]
#[ignore = "requires WORLD_EN_MODEL; run explicitly in CPU and GPU builds"]
fn batch_parsing_preserves_order_empty_inputs_and_repeats() {
    let texts: Vec<String> = [
        "Mary went to the kitchen.",
        "",
        "The red box is in the kitchen. The blue box is in the garden.",
        "   ",
        "Nora picked up the pear and carried it to the hallway.",
        "Mary went to the kitchen.",
    ].into_iter().map(String::from).collect();
    let first = world::exec::parse_many(&texts).unwrap();
    let second = world::exec::parse_many(&texts).unwrap();
    assert_eq!(first, second, "repeated batches differ");
    assert_eq!(first.len(), texts.len());
    assert!(first[1].is_empty() && first[3].is_empty());
    assert_eq!(first[0], first[5]);
    assert!(!first[0].is_empty() && !first[2].is_empty() && !first[4].is_empty());
    assert!(world::exec::parse_many(&[]).unwrap().is_empty());
    assert!(world::exec::parse_many(&[String::new(), "  ".into()]).unwrap().iter().all(Vec::is_empty));
    if world::exec::mode() == world::exec::Mode::Cpu {
        let singles: Vec<_> = texts.iter().map(|t| world::frames::parse_words(t)).collect();
        assert_eq!(first, singles, "batched CPU parsing differs from the reference");
    }
}

#[cfg(feature = "gpu")]
#[test]
#[ignore = "requires WORLD_EN_MODEL and CUDA"]
fn missing_gpu_model_returns_an_error() {
    let s = Scratch::new();
    let model = world::tree::model_path();
    let out = world(&s, &model, "gpu")
        .env("MOVA_GPARSE_MODEL", s.0.join("missing-gpu"))
        .args(["knowledge", "course", "mysticism"]).output().unwrap();
    assert!(!out.status.success());
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("GPU parser"), "{err}");
    assert!(out.stdout.is_empty());
    assert!(!s.0.join("hot/school").exists());
}

#[test]
fn wave29_command_isolation_and_cleanup_without_spawn() {
    let path;
    {
        let s = Scratch::new();
        path = s.0.clone();
        let model = s.0.join("synthetic-missing.bin");
        let c = world(&s, &model, "cpu");
        let vars: std::collections::BTreeMap<_, _> = c
            .get_envs()
            .map(|(k, v)| {
                (
                    k.to_string_lossy().into_owned(),
                    v.map(|x| x.to_string_lossy().into_owned()),
                )
            })
            .collect();
        assert_eq!(vars["MOVA_EXEC"].as_deref(), Some("cpu"));
        assert_eq!(vars["WORLD_EN_MODEL"].as_deref(), model.to_str());
        assert_eq!(vars["MOVA_HOT_ROOT"].as_deref(), s.0.join("hot").to_str());
        assert_eq!(vars["MOVA_STATE"], None);
        assert_eq!(vars["PRAG_OFF"], None);
        assert!(c.get_args().next().is_none());
        assert_eq!(
            c.get_current_dir(),
            Some(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
        );
        assert!(!model.exists());
        std::fs::write(s.0.join("sentinel"), b"synthetic").unwrap();
    }
    assert!(!path.exists());
}
