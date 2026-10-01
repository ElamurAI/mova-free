//! v2, tier 4: formal proofs in Lean 4 + Mathlib. The statement is fixed (from the set), the LLM writes only
//! the proof body after `:= by`; the MMM assembles the file itself (so the statement cannot be swapped), runs
//! `lake env lean` in the project folder `data/lean/mathproofs` (offline: no `lake update`,
//! `cache get`) and accepts a proof only when Lean reports no errors and no `sorry`, and the text contains no
//! `sorry`/`admit`/`axiom`/`native_decide`/`unsafe`. If it fails, Lean's errors go back to the LLM (≤ 2 retries).

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Theorem {
    pub id: String,
    pub split: String,
    pub header: String,
    /// `theorem … :=`
    pub statement: String,
    pub nl: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct LeanVerdict {
    pub ok: bool,
    pub errors: Vec<String>,
    pub secs: f64,
    pub file: String,
}

pub const BANNED: &[&str] = &["sorry", "admit", "axiom ", "native_decide", "unsafe", "implemented_by", "extern", "set_option debug", "decide := true", "ofReduceBool"];

pub fn project() -> PathBuf {
    PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("lean/mathproofs")
}

pub fn assemble(t: &Theorem, proof: &str) -> String {
    let stmt = t.statement.trim_end();
    let stmt = stmt.strip_suffix(":=").unwrap_or(stmt).trim_end();
    let body: String = proof.lines().map(|l| format!("  {l}\n")).collect();
    format!("{}\n{stmt} := by\n{body}", t.header.trim_end())
}

/// Check proofs with Lean. `dir` — where to put the files (may be outside the project).
pub fn check(t: &Theorem, proof: &str, dir: &Path, tag: &str, timeout_s: u64) -> LeanVerdict {
    let t0 = Instant::now();
    let mut v = LeanVerdict::default();
    for b in BANNED {
        if proof.contains(b) {
            v.errors.push(format!("forbidden in a proof: «{}»", b.trim()));
        }
    }
    if !v.errors.is_empty() {
        v.secs = t0.elapsed().as_secs_f64();
        return v;
    }
    let src = assemble(t, proof);
    let _ = std::fs::create_dir_all(dir);
    let safe: String = t.id.chars().map(|c| if c.is_alphanumeric() { c } else { '_' }).collect();
    let file = dir.join(format!("{safe}-{tag}.lean"));
    if std::fs::write(&file, &src).is_err() {
        v.errors.push("failed to write the file".into());
        return v;
    }
    v.file = file.display().to_string();
    let home = std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into());
    let path = format!("{home}/.elan/bin:{}", std::env::var("PATH").unwrap_or_default());
    let out = Command::new("timeout")
        .arg(timeout_s.to_string())
        .arg(format!("{home}/.elan/bin/lake"))
        .args(["env", "lean"])
        .arg(&file)
        .current_dir(project())
        .env("PATH", path)
        .output();
    v.secs = t0.elapsed().as_secs_f64();
    match out {
        Err(e) => v.errors.push(format!("running Lean: {e}")),
        Ok(o) => {
            let text = format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr));
            if o.status.code() == Some(124) {
                v.errors.push(format!("Lean: time limit {timeout_s} s"));
            }
            for block in text.split(&format!("{}:", file.display())).skip(1) {
                let b = block.trim();
                if b.contains("error") || b.contains("declaration uses 'sorry'") {
                    v.errors.push(b.chars().take(700).collect());
                }
            }
            if !o.status.success() && v.errors.is_empty() {
                v.errors.push(text.chars().take(700).collect());
            }
        }
    }
    v.ok = v.errors.is_empty();
    v
}

/// Prompt: proofs for fixed statements; the answer is blocks `=== id` … `=== end`.
pub fn prompt(ts: &[(Theorem, Option<(String, Vec<String>)>)]) -> String {
    let mut s = String::from(
        "You are an expert Lean 4 + Mathlib prover (Lean 4.35, current Mathlib). For each theorem below write a complete proof.\n\
         The statement is FIXED: the engine inserts your text after `:= by`, so output ONLY the tactic proof body (it will be indented by the engine).\n\
         Forbidden: sorry, admit, axioms, native_decide. Prefer robust tactics (simp, ring, linarith, nlinarith, omega, norm_num, aesop, exact?-style explicit lemma names you are sure exist).\n\
         Output format — for each theorem exactly:\n=== <id>\n<proof lines>\n=== end\nNo other text.\n\n",
    );
    for (t, prev) in ts {
        s.push_str(&format!("### {}\nInformal: {}\n```lean\n{}\n{}\n```\n", t.id, t.nl.trim(), t.header.trim(), t.statement.trim()));
        if let Some((p, errs)) = prev {
            s.push_str(&format!("Your previous proof FAILED to check:\n```lean\n{p}\n```\nLean said:\n{}\nFix it.\n", errs.join("\n---\n")));
        }
        s.push('\n');
    }
    s
}

pub fn parse_proofs(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut cur: Option<(String, Vec<String>)> = None;
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("=== ") {
            let rest = rest.trim();
            if rest == "end" {
                if let Some((id, ls)) = cur.take() {
                    let body = ls.join("\n");
                    let body = body.trim_matches('\n').trim_start_matches("```lean").trim_start_matches("```").trim_end_matches("```").trim_matches('\n').to_string();
                    out.push((id, body));
                }
            } else {
                cur = Some((rest.to_string(), Vec::new()));
            }
        } else if let Some((_, ls)) = cur.as_mut() {
            ls.push(line.to_string());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_blocks_and_ban() {
        let t = "=== a|b\n  intro x\n  simp\n=== end\n=== c\nexact foo\n=== end\n";
        let ps = parse_proofs(t);
        assert_eq!(ps.len(), 2);
        assert_eq!(ps[0].0, "a|b");
        let th = Theorem { id: "x".into(), split: "dev".into(), header: "import Mathlib".into(), statement: "theorem x : 1 + 1 = 2 :=".into(), nl: String::new() };
        let v = check(&th, "sorry", std::path::Path::new("."), "t", 5);
        assert!(!v.ok, "sorry must be rejected before Lean");
        assert!(assemble(&th, "norm_num").contains("theorem x : 1 + 1 = 2 := by\n  norm_num"));
    }
}
