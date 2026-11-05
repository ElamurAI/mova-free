//! End-to-end tests against a fake LLM in a real tmux server on a private socket.
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};
use terminal::{ask, ask_wait, journal, pending, status, Profile};

const FAKE: &str = r#"#!/bin/bash
# A fake LLM terminal: answers each question into the answer file and prints the marker.
mode="$1"; marker="$2"
stty -echo 2>/dev/null
echo "$marker"
file=""
while IFS= read -r line; do
  case "$line" in
    "Answer file: "*) file="${line#Answer file: }";;
    "End of question.")
      if [ "$mode" = silent ]; then echo "thinking (esc to interrupt)"; continue; fi
      printf 'An answer from %s.\n' "$mode" > "$file"
      echo "$marker";;
  esac
done
"#;

struct Fx {
    dir: PathBuf,
    socket: String,
}
impl Drop for Fx {
    fn drop(&mut self) {
        let _ = Command::new("tmux").args(["-L", &self.socket, "kill-server"]).output();
        let _ = fs::remove_dir_all(&self.dir);
    }
}
fn fx(name: &str) -> Fx {
    let socket = format!("mova-term-{}-{}", name, std::process::id());
    let dir = std::env::temp_dir().join(&socket);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("fake.sh"), FAKE).unwrap();
    Fx { dir, socket }
}
impl Fx {
    fn spawn(&self, session: &str, owner: &str, mode: &str) {
        let script = self.dir.join("fake.sh");
        let ok = Command::new("tmux")
            .args(["-L", &self.socket, "new-session", "-d", "-s", session, "-x", "200", "-y", "50", "bash"])
            .arg(&script)
            .args([mode, "TERMINAL READY"])
            .status()
            .unwrap()
            .success();
        assert!(ok);
        Command::new("tmux")
            .args(["-L", &self.socket, "set-option", "-t", session, "@mova_crab_owner", owner])
            .status()
            .unwrap();
        for _ in 0..50 {
            let out = Command::new("tmux")
                .args(["-L", &self.socket, "capture-pane", "-p", "-t", &format!("={session}:")])
                .output()
                .unwrap();
            if String::from_utf8_lossy(&out.stdout).contains("TERMINAL READY") {
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        panic!("fake terminal did not start");
    }
    fn profile(&self, animal: &str, session: &str, owner: &str) -> Profile {
        let p = Profile {
            animal: animal.into(),
            session: session.into(),
            tmux: which_tmux(),
            socket: Some(self.socket.clone()),
            pane_command: "bash".into(),
            owner_marker: owner.into(),
            idle_marker: "TERMINAL READY".into(),
            idle_window: 2,
            root: self.dir.join(animal),
            question_max_bytes: 100_000,
            answer_max_bytes: 400_000,
            answer_timeout_seconds: 10,
        };
        p.validate().unwrap();
        p
    }
}
fn which_tmux() -> PathBuf {
    for p in ["/usr/bin/tmux", "/usr/local/bin/tmux", "/bin/tmux"] {
        if Path::new(p).exists() {
            return p.into();
        }
    }
    panic!("tmux not found");
}

#[test]
fn each_animal_gets_its_own_answer_from_its_own_terminal() {
    let f = fx("own");
    f.spawn("whale-1-sol", "mova-whale-1-sol", "whale-1");
    f.spawn("whale-2-sol", "mova-whale-2-sol", "whale-2");
    let a = f.profile("whale-1", "whale-1-sol", "mova-whale-1-sol");
    let b = f.profile("whale-2", "whale-2-sol", "mova-whale-2-sol");
    let (_, x) = ask_wait(&a, "What is a dependency tree?", Duration::from_millis(100)).unwrap();
    let (_, y) = ask_wait(&b, "What is a frame?", Duration::from_millis(100)).unwrap();
    assert_eq!(x, "An answer from whale-1.\n");
    assert_eq!(y, "An answer from whale-2.\n");
    let s = status(&a).unwrap();
    assert_eq!((s["sent"].as_u64(), s["answered"].as_u64()), (Some(1), Some(1)));
    assert_eq!(s["state"], "idle");
    // Second question to the same terminal works once the first is answered.
    let (_, z) = ask_wait(&a, "And a frame?", Duration::from_millis(100)).unwrap();
    assert_eq!(z, "An answer from whale-1.\n");
}

#[test]
fn another_animals_terminal_is_refused_and_nothing_is_sent() {
    let f = fx("foreign");
    f.spawn("whale-2-sol", "mova-whale-2-sol", "whale-2");
    // whale-1 points at whale-2's session but keeps its own owner marker: refused.
    let wrong = f.profile("whale-1", "whale-2-sol", "mova-whale-1-sol");
    let e = ask(&wrong, "Hello?").unwrap_err().to_string();
    assert!(e.contains("owner marker"), "{e}");
    assert!(journal(&wrong).unwrap().is_empty());
}

#[test]
fn an_unanswered_question_is_never_replayed() {
    let f = fx("silent");
    f.spawn("turtle-sol", "mova-turtle-sol", "silent");
    let p = f.profile("turtle", "turtle-sol", "mova-turtle-sol");
    let a = ask(&p, "Is this question clear?").unwrap();
    assert_eq!(pending(&p).unwrap().as_deref(), Some(a.id.as_str()));
    let e = ask(&p, "Another one?").unwrap_err().to_string();
    assert!(e.contains("never replayed"), "{e}");
    assert_eq!(journal(&p).unwrap().iter().filter(|v| v["event"] == "sent").count(), 1);
}

#[test]
fn a_busy_terminal_gets_no_question() {
    let f = fx("busy");
    f.spawn("nautilus-sol", "mova-nautilus-sol", "silent");
    let p = f.profile("nautilus", "nautilus-sol", "mova-nautilus-sol");
    ask(&p, "First?").unwrap();
    std::thread::sleep(Duration::from_millis(300));
    // Clear the pending record by abandoning it: the screen still shows work, so no question goes.
    let id = pending(&p).unwrap().unwrap();
    terminal::abandon(&p, &id, "test").unwrap();
    let e = ask(&p, "Second?").unwrap_err().to_string();
    assert!(e.contains("not idle"), "{e}");
    assert_eq!(journal(&p).unwrap().iter().filter(|v| v["event"] == "sent").count(), 1);
}
