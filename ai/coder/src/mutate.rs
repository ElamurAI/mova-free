//! Judge control on subtle flaws (separate from training): the task's reference and one mutant of it — number ±1,
//! reversed comparison, "+" ↔ "−", a built-in from a pair (sum ↔ prod, max ↔ min, floor ↔ ceil, mod → rem).
//! The real result is execution in the sandbox (some mutants are equivalent and pass). The judge sees
//! the task, the expected output and the program — without the reference: pure judgement by reading code.

use mlab::lexer::{Tok, lex};

use crate::util::sha256_hex;

fn offset(code: &str, line: usize, col: usize) -> Option<usize> {
    let mut off = 0usize;
    for (i, l) in code.split('\n').enumerate() {
        if i + 1 == line {
            let ch: Vec<(usize, char)> = l.char_indices().collect();
            return ch.get(col - 1).map(|(b, _)| off + b);
        }
        off += l.len() + 1;
    }
    None
}

/// All mutation candidates: (kind, new program).
pub fn candidates(code: &str) -> Vec<(String, String)> {
    let Ok(toks) = lex(code) else { return Vec::new() };
    let mut out = Vec::new();
    for t in &toks {
        let (text, repl, kind): (String, String, &str) = match &t.tok {
            Tok::Num(v, s) if v.fract() == 0.0 && *v >= 0.0 && *v < 1e6 => (s.clone(), format!("{}", *v as i64 + 1), "literal+1"),
            Tok::Lt => ("<".into(), "<=".into(), "compare"),
            Tok::Le => ("<=".into(), "<".into(), "compare"),
            Tok::Gt => (">".into(), ">=".into(), "compare"),
            Tok::Ge => (">=".into(), ">".into(), "compare"),
            Tok::Plus if t.space_before => ("+".into(), "-".into(), "arith"),
            Tok::Minus if t.space_before => ("-".into(), "+".into(), "arith"),
            Tok::Ident(n) => {
                let r = match n.as_str() {
                    "sum" => "prod",
                    "prod" => "sum",
                    "max" => "min",
                    "min" => "max",
                    "floor" => "ceil",
                    "ceil" => "floor",
                    "mod" => "rem",
                    _ => continue,
                };
                (n.clone(), r.to_string(), "builtin")
            }
            _ => continue,
        };
        let Some(o) = offset(code, t.line, t.col) else { continue };
        if !code[o..].starts_with(&text) {
            continue;
        }
        let m = format!("{}{}{}", &code[..o], repl, &code[o + text.len()..]);
        if mlab::parser::parse_program(&m).is_ok() {
            out.push((kind.to_string(), m));
        }
    }
    out
}

/// One mutant per task: kind in rotation (literal, compare, arith, builtin) from the id hash, then position from the hash.
pub fn pick(code: &str, id: &str) -> Option<(String, String)> {
    let c = candidates(code);
    if c.is_empty() {
        return None;
    }
    let h = u64::from_str_radix(&sha256_hex(id.as_bytes())[..12], 16).unwrap_or(0);
    let kinds = ["literal+1", "compare", "arith", "builtin"];
    for k in 0..kinds.len() {
        let want = kinds[(h as usize + k) % kinds.len()];
        let of: Vec<&(String, String)> = c.iter().filter(|(kd, _)| kd == want).collect();
        if !of.is_empty() {
            return Some(of[(h >> 8) as usize % of.len()].clone());
        }
    }
    None
}
