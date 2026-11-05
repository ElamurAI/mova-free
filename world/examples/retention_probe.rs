//! Compare a neutral-focus live learner with a cold restored graph.
//! The separate CLI test is the process-isolation check.
use anyhow::{Result, ensure};
use std::{env, fs, path::Path};
use world::knowledge::retention::{Retained, parse_material};
fn main() -> Result<()> {
    probe(&env::args().collect::<Vec<_>>())
}

fn probe(args: &[String]) -> Result<()> {
    ensure!(args.len() == 4, "retention_probe TRAIN EXAM NEW_STATE");
    let train = parse_material(&fs::read_to_string(&args[1])?, &args[1])?;
    let exam = parse_material(&fs::read_to_string(&args[2])?, &args[2])?;
    let mut st = Retained::new(&train.course);
    let training = st.train(&train, false)?;
    let mut live = st.exam(&exam)?;
    st.save(Path::new(&args[3]))?;
    let t = std::time::Instant::now();
    let restored = Retained::load(Path::new(&args[3]))?;
    let load_seconds = t.elapsed().as_secs_f64();
    let mut cold = restored.exam(&exam)?;
    let live_seconds = live["seconds"].clone();
    let cold_seconds = cold["seconds"].clone();
    live.as_object_mut().unwrap().remove("seconds");
    cold.as_object_mut().unwrap().remove("seconds");
    ensure!(live == cold, "restored answer or witness differs");
    let blank = Retained::new(&train.course).exam(&exam)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "training":training,"same_answers_and_witnesses":true,"result":cold,
            "live_exam_seconds":live_seconds,"cold_exam_seconds":cold_seconds,
            "load_seconds":load_seconds,"no_training_right":blank["right"],
            "checkpoint_bytes":fs::metadata(&args[3])?.len()
        }))?
    );
    Ok(())
}

#[cfg(test)]
mod wave29_worldlearn {
    use super::*;
    #[test]
    fn arity_gate_precedes_input_reads() {
        let err = probe(&["probe".into()]).unwrap_err().to_string();
        assert!(err.contains("retention_probe TRAIN EXAM NEW_STATE"));
    }
    #[test]
    fn malformed_training_precedes_missing_exam_and_state_write() {
        let dir = env::temp_dir().join(format!("wave29-worldlearn-{}-probe", std::process::id()));
        fs::create_dir(&dir).unwrap();
        let train = dir.join("train.txt");
        let exam = dir.join("absent.txt");
        let state = dir.join("state.json");
        fs::write(&train, "").unwrap();
        let err = probe(&[
            "probe".into(),
            train.to_string_lossy().into_owned(),
            exam.to_string_lossy().into_owned(),
            state.to_string_lossy().into_owned(),
        ])
        .unwrap_err()
        .to_string();
        assert!(
            err.contains("material needs course and lesson number"),
            "{err}"
        );
        assert!(!state.exists());
        fs::remove_dir_all(dir).unwrap();
    }
}
