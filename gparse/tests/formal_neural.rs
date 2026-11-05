//! Run the Lean specifications against a preinstalled Mathlib project; no downloads.
//! LEAN_PROJECT=<project> cargo test --release -p gparse --test formal_neural -- --ignored --nocapture
use std::{path::{Path,PathBuf},process::{Command,Output}};

fn check(file:&Path,project:&Path,lake:&Path)->Output {
    Command::new("timeout").arg("90").arg(lake).args(["env","lean"]).arg(file)
        .current_dir(project).output().expect("run installed Lean")
}
fn text(out:&Output)->String {
    format!("{}{}",String::from_utf8_lossy(&out.stdout),String::from_utf8_lossy(&out.stderr))
}

#[test]
#[ignore = "requires LEAN_PROJECT with installed Lean 4.35.0-rc3 and cached Mathlib"]
fn proofs_and_false_formula_controls() {
    run_specs(&[
        ("Complementer.lean","|modeled - observed| ≤ modelError + observationError := by",
                         "|modeled - observed| ≤ 0 := by"),
        ("Softmax.lean","p i * (g i - ∑ j, p j * g j) := by\n  simp [sub_mul",
                        "p i * (g i + ∑ j, p j * g j) := by\n  simp [sub_mul"),
        ("Twin.lean","prediction + leverage / (1 + leverage) * (observation - prediction)",
                     "prediction - leverage / (1 + leverage) * (observation - prediction)"),
        ("StudyComparison.lean","model && assessment && total","model && true && total"),
        ("Retention.lean","(repeats - 1) * replay - repeats * load - save","(repeats - 1) * replay - repeats * load + save"),
        ("Enrichment.lean","annotation && dictionary && lesson && exam", "annotation"),
        ("ExerciseReuse.lean","∑ i, w i * phi i = ∑ i, v i * phi i := by",
                              "∑ i, w i * phi i = (∑ i, v i * phi i) + 1 := by"),
        ("School.lean","fun i => w i + rate * (right i - chosen i)",
                       "fun i => w i - rate * (right i - chosen i)"),
        ("Activation.lean","|gated f x - gated g x| ≤ |x| / 2 * epsilon",
                           "|gated f x - gated g x| ≤ 0"),
    ]);
}


#[test]
#[ignore = "requires LEAN_PROJECT with installed Lean and cached Mathlib"]
fn frozen_union_preimage_and_false_formula_control() {
    run_specs(&[("FrozenUnion.lean",
        "A ⊆ O ∧ O \\ A ⊆ P ∧ P ⊆ O",
        "A ⊆ O ∧ O \\ A ⊆ P ∧ P ⊆ A")]);
}

fn run_specs(specs: &[(&str, &str, &str)]) {
    let project=PathBuf::from(std::env::var("LEAN_PROJECT").expect("LEAN_PROJECT"));
    let lake=std::env::var_os("LEAN_LAKE").map(PathBuf::from)
        .unwrap_or_else(||PathBuf::from(std::env::var_os("HOME").unwrap()).join(".elan/bin/lake"));
    let root=Path::new(env!("CARGO_MANIFEST_DIR")).join("../formal");
    let stamp=std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dir=std::env::temp_dir().join(format!("mova-formal-{}-{stamp}",std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    for (name,from,to) in specs {
        let original=root.join(name);
        let out=check(&original,&project,&lake);
        let log=text(&out);
        println!("{name}:\n{log}");
        assert!(out.status.success(),"{name}: Lean rejected the specification");
        assert!(!log.contains("sorryAx")&&!log.contains("declaration uses 'sorry'"),"incomplete proof");
        let mut checked=log.lines().filter(|l|l.contains("does not depend on any axioms")).count();
        for line in log.lines().filter(|l|l.contains("depends on axioms:")) {
            let list=line.split_once("depends on axioms:").unwrap().1.trim();
            let names=list.strip_prefix('[').and_then(|x|x.strip_suffix(']')).expect("Lean axiom list");
            for name in names.split(',').map(str::trim).filter(|x|!x.is_empty()) {
                assert!(["propext","Classical.choice","Quot.sound"].contains(&name),
                    "unexpected axiom {name} in {line}");
            }
            checked+=1;
        }
        assert_eq!(checked,match *name { "FrozenUnion.lean" => 1, "Complementer.lean" => 9, "Softmax.lean" => 5, "Twin.lean" | "Enrichment.lean" | "Retention.lean" | "StudyComparison.lean" => 4, "School.lean" | "ExerciseReuse.lean" => 3, _ => 2 },"missing axiom reports");
        let source=std::fs::read_to_string(&original).unwrap();
        assert_eq!(source.matches(*from).count(),1,"negative control must alter exactly one formula");
        let false_file=dir.join(name);
        std::fs::write(&false_file,source.replacen(*from,to,1)).unwrap();
        let bad=check(&false_file,&project,&lake);
        assert!(!bad.status.success(),"false {name} passed");
        assert_ne!(bad.status.code(),Some(124),"negative control timed out");
        assert!(text(&bad).contains("error"),"negative control failed outside Lean");
        println!("{name}: false-formula control rejected");
    }
    std::fs::remove_dir_all(dir).unwrap();
}
