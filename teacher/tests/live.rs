#[test]
#[ignore = "calls the authenticated Codex teacher once"]
fn authenticated_teacher_smoke() {
    let dir = std::env::temp_dir().join(format!("mova-teacher-live-{}", std::process::id()));
    let r = mova_teacher::ask(&dir, &mova_teacher::model("MOVA_TEACHER_MODEL"), "medium", "Return exactly MOVA_TEACHER_OK with no other text. Do not use tools.").unwrap();
    assert_eq!(r["result"].as_str().unwrap().trim(), "MOVA_TEACHER_OK");
    println!("provider={} model={} usage={}", r["provider"], r["model"], r["usage"]);
    std::fs::remove_dir_all(dir).unwrap();
}
