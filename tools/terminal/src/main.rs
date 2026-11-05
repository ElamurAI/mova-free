use std::{io::Read, path::Path, time::Duration};

const USAGE: &str = "usage: mova-terminal status PROFILE | ask PROFILE [--wait] (question on stdin) | \
answer PROFILE ID | abandon PROFILE ID REASON";

fn run(args: &[String]) -> terminal::Result<serde_json::Value> {
    let (cmd, profile) = match args {
        [c, p, ..] => (c.as_str(), terminal::Profile::load(Path::new(p))?),
        _ => return Err(USAGE.into()),
    };
    match (cmd, &args[2..]) {
        ("status", []) => terminal::status(&profile),
        ("ask", rest) if rest.is_empty() || rest == ["--wait"] => {
            let mut q = String::new();
            std::io::stdin()
                .take(profile.question_max_bytes as u64 + 1)
                .read_to_string(&mut q)?;
            if rest.is_empty() {
                let a = terminal::ask(&profile, &q)?;
                Ok(serde_json::json!({"sent": a.id, "answer_path": a.answer_path}))
            } else {
                let (id, text) = terminal::ask_wait(&profile, &q, Duration::from_millis(500))?;
                Ok(serde_json::json!({"id": id, "answer": text}))
            }
        }
        ("answer", [id]) => Ok(serde_json::json!({"id": id, "answer": terminal::answer(&profile, id)?})),
        ("abandon", [id, reason]) => {
            terminal::abandon(&profile, id, reason)?;
            Ok(serde_json::json!({"abandoned": id}))
        }
        _ => Err(USAGE.into()),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(v) => println!("{v}"),
        Err(e) => {
            eprintln!("mova-terminal: {e}");
            std::process::exit(1);
        }
    }
}
