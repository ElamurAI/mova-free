//! Executable self-study joins on synthetic local artifacts; no teacher call or learning.
use mova_enrich::{
    Claim, Packet, Question, Review, Sentence,
    assessment::{AssessmentContract, compare},
    hash, stage, status,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
use world::knowledge::{
    Lesson,
    retention::{parse_material as parse, split_lesson},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Run(PathBuf);
impl Drop for Run {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
impl Run {
    fn new() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "mova-groupant-pipeline-{}-{nonce}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn write(&self, name: &str, bytes: impl AsRef<[u8]>) {
        fs::write(self.0.join(name), bytes).unwrap();
    }
    fn save_completed(&self, events: &str) -> anyhow::Result<()> {
        let reply = mova_teacher::decode_events(events)?;
        self.write("review.json", reply["result"].as_str().unwrap());
        self.write("teacher-reply.json", serde_json::to_vec(&reply).unwrap());
        Ok(())
    }
}
fn prepared() -> (Run, Review) {
    let run = Run::new();
    let packet = Packet {
        self_study: None,
        schema: 1,
        document: "fixture:fox".into(),
        enrichment_partition: "train".into(),
        school_sha256: String::new(),
        dictionary_sha256: "synthetic-dictionary".into(),
        sentences: (0..24)
            .map(|i| Sentence {
                id: format!("fixture:fox:{i}"),
                text: format!("The fox number {i} eats apple number {i}."),
                tree: String::new(),
                prag: BTreeMap::new(),
                world: String::new(),
            })
            .collect(),
        words: vec![],
        role_proposals: vec![],
        related_lessons: vec![],
    };
    let bytes = serde_json::to_vec(&packet).unwrap();
    let digest = hash(&bytes);
    run.write("packet.json", bytes);
    run.write("packet.sha256", &digest);
    run.write("review-prompt.txt", "Synthetic local fixture only.");
    run.write("prompt.sha256", hash(b"Synthetic local fixture only."));
    let claim = Claim {
        text: packet.sentences[0].text.clone(),
        source_id: packet.sentences[0].id.clone(),
        quote: packet.sentences[0].text.clone(),
    };
    let review = Review {
        packet_sha256: digest,
        title: "Synthetic fox passages".into(),
        explain: vec![claim.clone(); 3],
        facts: vec![claim; 5],
        dictionary: vec![],
        questions: packet
            .sentences
            .iter()
            .enumerate()
            .map(|(i, sentence)| Question {
                question: format!("According to source example {i}, what does the fox eat?"),
                answer: format!("apple number {i}"),
                wrong: ["a pear", "a plum", "a nut"].map(str::to_owned),
                source_id: sentence.id.clone(),
                evidence: sentence.text.clone(),
                kind: "action".into(),
                phase: if i < 12 {
                    "exercise"
                } else if i < 20 {
                    "test"
                } else {
                    "exam"
                }
                .into(),
            })
            .collect(),
        gaps: vec![],
        corrections: vec!["Synthetic local fixture.".into()],
    };
    (run, review)
}
fn message(text: &str) -> Value {
    json!({"type":"item.completed","item":{"type":"agent_message","text":text}})
}
fn completed() -> Value {
    json!({"type":"turn.completed","usage":{"input_tokens":11,"output_tokens":7}})
}
fn transcript(events: &[Value]) -> String {
    events
        .iter()
        .map(|event| serde_json::to_string(event).unwrap() + "\n")
        .collect()
}
fn valid_trace(review: &Review) -> String {
    transcript(&[
        json!({"type":"thread.started","thread_id":"synthetic"}),
        json!({"type":"turn.started"}),
        message(&serde_json::to_string(review).unwrap()),
        completed(),
    ])
}
fn grade(
    lesson: &Lesson,
    contract: &AssessmentContract,
    model: &str,
    correct_count: usize,
) -> Value {
    let mut cursor = 0;
    let mut rows = Vec::new();
    for (kind, questions) in [("test", &lesson.tests), ("exam", &lesson.exams)] {
        for (index, (question, right, wrong)) in questions.iter().enumerate() {
            let correct = cursor < correct_count;
            rows.push(
                json!({"kind":kind,"index":index,"question":question,"right":right,
                "choice":if correct {right} else {&wrong[0]},"correct":correct}),
            );
            cursor += 1;
        }
    }
    assert!(correct_count <= cursor);
    json!({"state_sha256":model,"assessment_sha256":contract.identity(),
        "lesson_reads":0,"exercise_updates":0,"rows":rows,"right":correct_count,"total":cursor})
}

#[test]
fn decoded_review_stages_and_splits_without_publication() {
    let (run, review) = prepared();
    run.save_completed(&valid_trace(&review)).unwrap();
    stage(&run.0).unwrap();
    let receipt = status(&run.0).unwrap();
    assert_eq!(receipt["state"], "staged");
    assert_eq!(receipt["schema"], 2);
    assert_eq!(receipt["published"], false);
    assert_eq!(receipt["complete"], false);
    assert_eq!(receipt["outputs"].as_object().unwrap().len(), 6);
    let text = fs::read_to_string(run.0.join("staged/books.lesson")).unwrap();
    let (training_text, assessment_text) = split_lesson(&text).unwrap();
    let training = parse(&training_text, "synthetic-training.lesson").unwrap();
    let assessment = parse(&assessment_text, "synthetic-assessment.lesson").unwrap();
    assert_eq!(training.exercises.len(), 12);
    assert!(training.tests.is_empty() && training.exams.is_empty());
    assert_eq!((assessment.tests.len(), assessment.exams.len()), (8, 4));
    assert!(
        assessment.explain.is_empty()
            && assessment.facts.is_empty()
            && assessment.exercises.is_empty()
    );
    assert!(AssessmentContract::new(&training).is_err());
    assert!(AssessmentContract::new(&assessment).is_ok());
    let original = fs::read(run.0.join("staged/books.lesson")).unwrap();
    assert!(stage(&run.0).is_err());
    assert_eq!(
        fs::read(run.0.join("staged/books.lesson")).unwrap(),
        original
    );
    assert_eq!(status(&run.0).unwrap(), receipt);
}
#[test]
fn temporal_countermodels_cannot_create_review_artifacts() {
    let (run, review) = prepared();
    let answer = message(&serde_json::to_string(&review).unwrap());
    for events in [
        vec![completed(), answer.clone()],
        vec![answer.clone(), completed(), answer.clone()],
        vec![answer.clone(), completed(), answer.clone(), completed()],
        vec![answer.clone(), completed(), json!({"type":"turn.failed"})],
        vec![
            json!({"type":"item.completed","item":{"type":"command_execution"}}),
            answer.clone(),
            completed(),
        ],
        vec![answer],
    ] {
        assert!(run.save_completed(&transcript(&events)).is_err());
        assert!(!run.0.join("review.json").exists());
        assert!(!run.0.join("teacher-reply.json").exists());
        assert!(!run.0.join("staged").exists());
    }
    assert_eq!(status(&run.0).unwrap()["state"], "prepared");
}

#[test]
fn semantic_json_equality_does_not_replace_review_byte_identity() {
    let (run, review) = prepared();
    run.save_completed(&valid_trace(&review)).unwrap();
    let original = fs::read(run.0.join("review.json")).unwrap();
    let reformatted = serde_json::to_vec_pretty(&review).unwrap();
    assert_ne!(original, reformatted);
    assert_eq!(
        serde_json::from_slice::<Value>(&original).unwrap(),
        serde_json::from_slice::<Value>(&reformatted).unwrap()
    );
    run.write("review.json", reformatted);
    let error = stage(&run.0).unwrap_err().to_string();
    assert!(error.contains("review differs"), "{error}");
    assert!(!run.0.join("staged").exists());
    assert!(!run.0.join("stage-receipt.json").exists());
}
#[test]
fn staged_question_projection_binds_identity_before_score_comparison() {
    let (run, review) = prepared();
    run.save_completed(&valid_trace(&review)).unwrap();
    stage(&run.0).unwrap();
    let text = fs::read_to_string(run.0.join("staged/books.lesson")).unwrap();
    let (_, questions) = split_lesson(&text).unwrap();
    let lesson = parse(&questions, "synthetic-assessment.lesson").unwrap();
    let contract = AssessmentContract::new(&lesson).unwrap();
    let before_report = grade(&lesson, &contract, "baseline", 0);
    let after_report = grade(&lesson, &contract, "candidate", 1);
    let before = contract.bind(&before_report, "baseline").unwrap();
    let after = contract.bind(&after_report, "candidate").unwrap();
    // Same assessment in both axes is admitted; this tests wiring, not held-out quality.
    let decision = compare(&before, &after, &before, &after).unwrap();
    assert_eq!(decision.gain, 1);
    assert!(decision.regression_preserved && decision.recommend);
    let foreign = parse(
        &questions.replace("source example 12", "foreign source 12"),
        "synthetic-foreign.lesson",
    )
    .unwrap();
    let foreign_contract = AssessmentContract::new(&foreign).unwrap();
    assert_ne!(contract.identity(), foreign_contract.identity());
    assert!(foreign_contract.bind(&after_report, "candidate").is_err());
    let other = contract
        .bind(
            &grade(&lesson, &contract, "other-candidate", 1),
            "other-candidate",
        )
        .unwrap();
    assert!(compare(&before, &after, &before, &other).is_err());
    let mut active = after_report;
    active["exercise_updates"] = json!(1);
    assert!(contract.bind(&active, "candidate").is_err());
}
