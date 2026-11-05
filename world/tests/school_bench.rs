//! Cheap CLI gates; actual timing/quality checks use the staged benchmark itself.
use std::{fs,path::PathBuf,process::Command};
use serde_json::Value;
struct Scratch(PathBuf);
impl Scratch {
    fn new()->Self {
        let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let p=std::env::temp_dir().join(format!("school-bench-{}-{stamp}",std::process::id()));
        fs::create_dir(&p).unwrap(); Self(p)
    }
    fn command(&self)->Command {
        let mut c=Command::new(env!("CARGO_BIN_EXE_world"));
        c.env("MOVA_EXEC","cpu").env("MOVA_HOT_ROOT",self.0.join("hot"))
            .env("MOVA_BENCH_ROOT",self.0.join("bench")).env("WORLD_EN_MODEL",self.0.join("missing.bin"))
            .env_remove("MOVA_STATE").env_remove("PRAG_OFF");
        c
    }
}
impl Drop for Scratch {fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
#[test]
fn plan_requires_no_model_and_periodic_full_keeps_all_questions() {
    let s=Scratch::new();
    let mut plans=Vec::new();
    for round in ["1","5","20"] {
        let out=s.command().args(["knowledge","bench","mysticism","--round",round,"--plan"]).output().unwrap();
        assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));
        plans.push(serde_json::from_slice::<Value>(&out.stdout).unwrap());
    }
    assert_eq!(plans[0]["tier"],"quick");
    assert_eq!(plans[1]["tier"],"medium");
    assert_eq!(plans[2]["tier"],"full");
    assert!(plans[0]["lessons"].as_array().unwrap().len()<plans[1]["lessons"].as_array().unwrap().len());
    assert_eq!(plans[2]["lessons"].as_array().unwrap().len() as u64,plans[2]["total_lessons"].as_u64().unwrap());
    assert_eq!(plans[2]["tests"],313);
    assert_eq!(plans[2]["exams"],112);
    assert!(!s.0.join("bench").exists());
}
#[test]
fn failed_probe_cannot_advance_cadence_or_replace_published_results() {
    let s=Scratch::new();
    let published=s.0.join("hot/school/mysticism/results.txt");
    fs::create_dir_all(published.parent().unwrap()).unwrap();
    fs::write(&published,"published reference").unwrap();
    let out=s.command().args(["knowledge","bench","mysticism"]).output().unwrap();
    assert!(!out.status.success());
    assert!(!s.0.join("bench/mysticism/seed-42/completed-round").exists());
    assert!(!s.0.join("bench/mysticism/seed-42/running").exists());
    assert_eq!(fs::read_to_string(published).unwrap(),"published reference");
    assert!(!s.command().args(["knowledge","bench","mysticism","--round","0"]).output().unwrap().status.success());
}

#[test]
fn wave29_bench_command_isolation_and_cleanup_without_spawn() {
    let path;
    {
        let s = Scratch::new();
        path = s.0.clone();
        let c = s.command();
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
        assert_eq!(
            vars["MOVA_BENCH_ROOT"].as_deref(),
            s.0.join("bench").to_str()
        );
        assert_eq!(vars["MOVA_HOT_ROOT"].as_deref(), s.0.join("hot").to_str());
        assert_eq!(
            vars["WORLD_EN_MODEL"].as_deref(),
            s.0.join("missing.bin").to_str()
        );
        assert_eq!(vars["MOVA_STATE"], None);
        assert_eq!(vars["PRAG_OFF"], None);
        assert!(c.get_args().next().is_none());
        assert!(!s.0.join("bench").exists());
        fs::write(s.0.join("sentinel"), b"synthetic").unwrap();
    }
    assert!(!path.exists());
}
