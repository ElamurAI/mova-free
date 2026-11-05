//! Fail-closed prompt guard and audit controller.
use annotation_review_worker::{annotation, execute};
mod controller;
use std::ffi::OsString;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, ExitCode, Stdio};

const LIMIT: u64 = 25_000;

fn guard<R: Read>(input: R, executable: &Path, args: &[OsString]) -> io::Result<i32> {
    let mut bytes = Vec::new();
    input.take(LIMIT + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > LIMIT {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "prompt exceeds 25000 UTF-8 bytes; Codex was not started",
        ));
    }
    std::str::from_utf8(&bytes).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "prompt is not UTF-8; Codex was not started",
        )
    })?;
    let mut child = Command::new(executable)
        .args(args)
        .stdin(Stdio::piped())
        .spawn()?;
    let written = child.stdin.take().expect("piped stdin").write_all(&bytes);
    let status = child.wait()?;
    written?;
    Ok(status.code().unwrap_or(1))
}

fn main() -> ExitCode {
    let args: Vec<OsString> = std::env::args_os().skip(1).collect();
    if args.first().and_then(|a| a.to_str()) == Some("annotation-contract") {
        if args.len() != 1 {
            eprintln!("annotation-contract takes no arguments");
            return ExitCode::FAILURE;
        }
        println!(
            "{}",
            serde_json::json!({"schema":1,"purpose":"syntax_ast_and_pragmatics_only","default_profile":"sonnet","rendered_request_cap":100000,"implementation_bindings":annotation::bindings()})
        );
        return ExitCode::SUCCESS;
    }
    if args.first().and_then(|a| a.to_str()) == Some("annotation-execute") {
        return match execute::dispatch(&args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("annotation execution: {e:#}");
                ExitCode::FAILURE
            }
        };
    }
    if args.first().and_then(|a| a.to_str()) == Some("annotation-prepare") {
        return match annotation::dispatch(&args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("annotation contract: {e}");
                ExitCode::FAILURE
            }
        };
    }
    if matches!(
        args.first().and_then(|x| x.to_str()),
        Some("run" | "config" | "status")
    ) {
        return match controller::dispatch(&args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("controller: {e}");
                ExitCode::FAILURE
            }
        };
    }
    let Some(codex) = std::env::var_os("MOVA_REVIEW_CODEX_REAL") else {
        eprintln!("annotation review guard: MOVA_REVIEW_CODEX_REAL is required");
        return ExitCode::FAILURE;
    };
    if !Path::new(&codex).is_absolute() {
        eprintln!("annotation review guard: Codex path must be absolute");
        return ExitCode::FAILURE;
    }
    match guard(io::stdin().lock(), Path::new(&codex), &args) {
        Ok(code) => ExitCode::from(u8::try_from(code).unwrap_or(1)),
        Err(e) => {
            eprintln!("annotation review guard: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    #[test]
    fn guard_passes_exact_input_and_arguments_and_blocks_oversize() {
        let dir = std::env::temp_dir().join(format!("review-guard-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let child = dir.join("child");
        let received = dir.join("received");
        std::fs::write(
            &child,
            format!(
                "#!/bin/sh\nprintf '%s\\n' \"$1\" > '{}'\ncat >> '{}'\n",
                received.display(),
                received.display()
            ),
        )
        .unwrap();
        std::fs::set_permissions(&child, std::fs::Permissions::from_mode(0o700)).unwrap();
        assert_eq!(
            guard("é".as_bytes(), &child, &["argument".into()]).unwrap(),
            0
        );
        assert_eq!(std::fs::read(&received).unwrap(), "argument\né".as_bytes());
        std::fs::remove_file(&received).unwrap();
        assert!(guard(vec![b'x'; LIMIT as usize + 1].as_slice(), &child, &[]).is_err());
        assert!(
            !received.exists(),
            "negative control: oversized input must not launch child"
        );
        assert!(guard(&[0xff][..], &child, &[]).is_err());
        assert!(!received.exists());
        assert_eq!(
            guard(vec![b'x'; LIMIT as usize].as_slice(), &child, &[]).unwrap(),
            0
        );
        std::fs::remove_dir_all(dir).unwrap();
    }
}
