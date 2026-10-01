//! Architect's changes after round 1 (Opus, `runs/…/rounds/r1/architect.md`), each behind its own switch:
//! - `a1:transcribe` — rewriting given code: mlab loop → vector expression via the AST (accumulation,
//!   counting, building and printing patterns), C fragment → mlab by transliteration;
//! - `a2:func-multi` — "write a function NAME(…)" tasks (body — the same motivated plan, calls on the test
//!   inputs) and multi-goal tasks (several printf in the description → several subgoals);
//! - `a3:struct-ops` — structural operators: row/column reductions of a matrix, sorting by a key,
//!   set operations on two vectors.

use mlab::ast::{BinOp, Expr, LValue, PostOp, Stmt, StmtKind, UnOp};

use crate::lemma;
use crate::memory::Bm25;
use crate::motiv::Model;
use crate::tasks::Public;
use crate::v0::Solution;
use crate::v1::{self, Flags, Step, Val};

// ---------- printing the AST back as mlab ----------

fn op_str(op: &BinOp, elem: bool) -> &'static str {
    match op {
        BinOp::Add => "+",
        BinOp::Sub => "-",
        BinOp::Mul => {
            if elem {
                ".*"
            } else {
                "*"
            }
        }
        BinOp::Div => {
            if elem {
                "./"
            } else {
                "/"
            }
        }
        BinOp::LDiv => "\\",
        BinOp::Pow => {
            if elem {
                ".^"
            } else {
                "^"
            }
        }
        BinOp::EMul => ".*",
        BinOp::EDiv => "./",
        BinOp::ELDiv => ".\\",
        BinOp::EPow => ".^",
        BinOp::Eq => "==",
        BinOp::Ne => "~=",
        BinOp::Lt => "<",
        BinOp::Le => "<=",
        BinOp::Gt => ">",
        BinOp::Ge => ">=",
        BinOp::And => "&",
        BinOp::Or => "|",
        BinOp::AndAnd => {
            if elem {
                "&"
            } else {
                "&&"
            }
        }
        BinOp::OrOr => {
            if elem {
                "|"
            } else {
                "||"
            }
        }
    }
}

fn quote(s: &str, dq: bool) -> String {
    if dq {
        format!("\"{}\"", s.replace('\\', "\\\\").replace('\n', "\\n").replace('\t', "\\t").replace('"', "\\\""))
    } else {
        format!("'{}'", s.replace('\'', "''"))
    }
}

/// Expression → text; `elem` — element-wise operators; `ren` — variable renaming (loop → vector).
pub fn show(e: &Expr, elem: bool, ren: &(&str, &str)) -> String {
    let sh = |x: &Expr| show(x, elem, ren);
    match e {
        Expr::Num(_, t) | Expr::Imag(_, t) => t.clone(),
        Expr::Str(s, dq) => quote(s, *dq),
        Expr::Ident(n) => {
            if n == ren.0 {
                ren.1.to_string()
            } else {
                n.clone()
            }
        }
        Expr::Colon => ":".into(),
        Expr::End => "end".into(),
        Expr::Paren(x) => format!("({})", sh(x)),
        Expr::Unary(op, x) => {
            let o = match op {
                UnOp::Neg => "-",
                UnOp::Plus => "+",
                UnOp::Not => "~",
            };
            format!("{o}{}", wrap(x, elem, ren))
        }
        Expr::Postfix(op, x) => format!("{}{}", wrap(x, elem, ren), if matches!(op, PostOp::CTranspose) { "'" } else { ".'" }),
        Expr::Binary(op, a, b) => format!("{} {} {}", wrap(a, elem, ren), op_str(op, elem), wrap(b, elem, ren)),
        Expr::Range(a, s, b) => match s {
            Some(s) => format!("{}:{}:{}", wrap(a, elem, ren), wrap(s, elem, ren), wrap(b, elem, ren)),
            None => format!("{}:{}", wrap(a, elem, ren), wrap(b, elem, ren)),
        },
        Expr::Index(b, args) => format!("{}({})", sh(b), args.iter().map(|a| sh(a)).collect::<Vec<_>>().join(", ")),
        Expr::Matrix(rows) => format!("[{}]", rows.iter().map(|r| r.iter().map(|x| sh(x)).collect::<Vec<_>>().join(", ")).collect::<Vec<_>>().join("; ")),
        Expr::CellList(v) => format!("{{{}}}", v.iter().map(|x| sh(x)).collect::<Vec<_>>().join(", ")),
        Expr::AnonFn(ps, body) => format!("@({}) {}", ps.join(", "), show(body, elem, ren)),
        Expr::FuncHandle(n) => format!("@{n}"),
        Expr::Field(x, f) => format!("{}.{f}", sh(x)),
        #[allow(unreachable_patterns)]
        _ => "?".into(),
    }
}

fn wrap(e: &Expr, elem: bool, ren: &(&str, &str)) -> String {
    match e {
        Expr::Binary(..) | Expr::Range(..) => format!("({})", show(e, elem, ren)),
        _ => show(e, elem, ren),
    }
}

fn lhs_str(l: &LValue, ren: &(&str, &str)) -> String {
    match l {
        LValue::Var(v) => v.clone(),
        LValue::Index(v, a) => format!("{v}({})", a.iter().map(|x| show(x, false, ren)).collect::<Vec<_>>().join(", ")),
        LValue::Field(v, f) => format!("{v}.{f}"),
        LValue::Tilde => "~".into(),
    }
}

pub fn show_stmt(s: &Stmt, ind: &str, out: &mut Vec<String>) {
    let no = ("", "");
    let semi = |p: bool| if p { "" } else { ";" };
    match &s.kind {
        StmtKind::Expr { expr, print } => out.push(format!("{ind}{}{}", show(expr, false, &no), semi(*print))),
        StmtKind::Assign { lhs, rhs, print, op } => {
            let l = if lhs.len() == 1 { lhs_str(&lhs[0], &no) } else { format!("[{}]", lhs.iter().map(|x| lhs_str(x, &no)).collect::<Vec<_>>().join(", ")) };
            let o = match op {
                Some(op) => format!("{}=", op_str(op, false)),
                None => "=".into(),
            };
            out.push(format!("{ind}{l} {o} {}{}", show(rhs, false, &no), semi(*print)));
        }
        StmtKind::For { var, iter, body } => {
            out.push(format!("{ind}for {var} = {}", show(iter, false, &no)));
            for b in body {
                show_stmt(b, &format!("{ind}  "), out);
            }
            out.push(format!("{ind}end"));
        }
        StmtKind::While { cond, body } => {
            out.push(format!("{ind}while {}", show(cond, false, &no)));
            for b in body {
                show_stmt(b, &format!("{ind}  "), out);
            }
            out.push(format!("{ind}end"));
        }
        StmtKind::DoUntil { body, cond } => {
            out.push(format!("{ind}do"));
            for b in body {
                show_stmt(b, &format!("{ind}  "), out);
            }
            out.push(format!("{ind}until {}", show(cond, false, &no)));
        }
        StmtKind::If { clauses, else_body } => {
            for (i, (c, b)) in clauses.iter().enumerate() {
                out.push(format!("{ind}{} {}", if i == 0 { "if" } else { "elseif" }, show(c, false, &no)));
                for x in b {
                    show_stmt(x, &format!("{ind}  "), out);
                }
            }
            if let Some(b) = else_body {
                out.push(format!("{ind}else"));
                for x in b {
                    show_stmt(x, &format!("{ind}  "), out);
                }
            }
            out.push(format!("{ind}end"));
        }
        StmtKind::Break => out.push(format!("{ind}break;")),
        StmtKind::Continue => out.push(format!("{ind}continue;")),
        StmtKind::Return => out.push(format!("{ind}return;")),
        StmtKind::Command { name, args, .. } => out.push(format!("{ind}{name} {}", args.join(" "))),
        StmtKind::Switch { .. } | StmtKind::Try { .. } => out.push(format!("{ind}% (switch/try not reprinted)")),
    }
}

fn uses_var(e: &Expr, v: &str) -> bool {
    match e {
        Expr::Ident(n) => n == v,
        Expr::Paren(x) | Expr::Unary(_, x) | Expr::Postfix(_, x) | Expr::Field(x, _) => uses_var(x, v),
        Expr::Binary(_, a, b) => uses_var(a, v) || uses_var(b, v),
        Expr::Range(a, s, b) => uses_var(a, v) || s.as_ref().is_some_and(|s| uses_var(s, v)) || uses_var(b, v),
        Expr::Index(b, a) => uses_var(b, v) || a.iter().any(|x| uses_var(x, v)),
        Expr::Matrix(r) => r.iter().flatten().any(|x| uses_var(x, v)),
        Expr::AnonFn(_, b) => uses_var(b, v),
        _ => false,
    }
}

fn call_of<'a>(e: &'a Expr, name: &[&str]) -> Option<&'a Vec<Expr>> {
    if let Expr::Index(b, a) = e {
        if let Expr::Ident(n) = b.as_ref() {
            if name.contains(&n.as_str()) {
                return Some(a);
            }
        }
    }
    None
}

/// A single `for` loop → vector equivalent (lines) and motive; None — pattern not recognised.
fn vectorize_for(var: &str, iter: &Expr, body: &[Stmt]) -> Option<(Vec<String>, String)> {
    let vs = format!("{var}s");
    let ren = (var, vs.as_str());
    let head = format!("{vs} = {};", show(iter, false, &("", "")));
    if body.len() != 1 {
        return None;
    }
    let s = &body[0];
    // accumulation: acc = acc + E(k) / acc += E(k) / acc = acc * E(k)
    if let StmtKind::Assign { lhs, rhs, op, .. } = &s.kind {
        if let [LValue::Var(acc)] = lhs.as_slice() {
            let (agg, term): (&str, Option<&Expr>) = match (op, rhs) {
                (Some(BinOp::Add), e) => ("sum", Some(e)),
                (Some(BinOp::Mul), e) => ("prod", Some(e)),
                (None, Expr::Binary(BinOp::Add, a, b)) if matches!(a.as_ref(), Expr::Ident(n) if n == acc) => ("sum", Some(b.as_ref())),
                (None, Expr::Binary(BinOp::Mul, a, b)) if matches!(a.as_ref(), Expr::Ident(n) if n == acc) => ("prod", Some(b.as_ref())),
                _ => ("", None),
            };
            if let Some(e) = term {
                if !uses_var(e, acc) {
                    let op = if agg == "sum" { "+" } else { "*" };
                    let ev = if uses_var(e, var) { show(e, true, &ren) } else { format!("{} * ones(size({vs}))", wrap(e, true, &ren)) };
                    return Some((vec![head, format!("{acc} = {acc} {op} {agg}({ev});")], format!("the loop {} one term per index, so build all terms at once and {agg} them", if agg == "sum" { "adds" } else { "multiplies" })));
                }
            }
        }
        // building: v(k) = E(k)
        if let [LValue::Index(v, idx)] = lhs.as_slice() {
            if idx.len() == 1 && matches!(&idx[0], Expr::Ident(n) if n == var) && op.is_none() && !uses_var(rhs, v) {
                return Some((vec![head, format!("{v}({vs}) = {};", show(rhs, true, &ren))], "each index gets its own value by one rule, so assign them all at once".into()));
            }
            if idx.len() == 1 && matches!(&idx[0], Expr::Binary(BinOp::Add, a, b) if matches!(a.as_ref(), Expr::End) && matches!(b.as_ref(), Expr::Num(..))) && op.is_none() && !uses_var(rhs, v) {
                return Some((vec![head, format!("{v} = [{v}, {}];", show(rhs, true, &ren))], "the loop appends one value per index, so append the whole vector at once".into()));
            }
        }
    }
    // printing in a loop: printf(FMT, E1(k), E2(k)…)
    if let StmtKind::Expr { expr, .. } = &s.kind {
        if let Some(args) = call_of(expr, &["printf", "fprintf"]) {
            if args.len() >= 2 && matches!(&args[0], Expr::Str(..)) {
                let f = show(&args[0], false, &("", ""));
                let parts: Vec<String> = args[1..].iter().map(|a| if uses_var(a, var) { show(a, true, &ren) } else { format!("{} * ones(size({vs}))", wrap(a, true, &ren)) }).collect();
                let data = if parts.len() == 1 { parts[0].clone() } else { format!("[{}]", parts.join("; ")) };
                return Some((vec![head, format!("printf({f}, {data});")], "printf repeats its format over all columns, so print every index in one call".into()));
            }
        }
    }
    // condition in the body: if C(k), (counting | building | printing)
    if let StmtKind::If { clauses, else_body: None } = &s.kind {
        if clauses.len() == 1 && clauses[0].1.len() == 1 {
            let cond = show(&clauses[0].0, true, &ren);
            let inner = &clauses[0].1[0];
            let km = format!("{var}m");
            let renm = (var, km.as_str());
            let sel = format!("{km} = {vs}({cond});");
            match &inner.kind {
                StmtKind::Assign { lhs, rhs, op, .. } => {
                    if let [LValue::Var(c)] = lhs.as_slice() {
                        let is_inc = matches!((op, rhs), (Some(BinOp::Add), Expr::Num(v, _)) if *v == 1.0)
                            || matches!((op, rhs), (None, Expr::Binary(BinOp::Add, a, b)) if matches!(a.as_ref(), Expr::Ident(n) if n == c) && matches!(b.as_ref(), Expr::Num(v, _) if *v == 1.0));
                        if is_inc {
                            return Some((vec![head, format!("{c} = {c} + nnz({cond});")], "the loop counts indices that pass a test, so test them all at once and count the true ones".into()));
                        }
                        let term = match (op, rhs) {
                            (Some(BinOp::Add), e) => Some(e),
                            (None, Expr::Binary(BinOp::Add, a, b)) if matches!(a.as_ref(), Expr::Ident(n) if n == c) => Some(b.as_ref()),
                            _ => None,
                        };
                        if let Some(e) = term {
                            if !uses_var(e, c) {
                                return Some((vec![head, sel, format!("{c} = {c} + sum({});", show(e, true, &renm))], "only indices that pass the test add a term, so select them first and sum".into()));
                            }
                        }
                    }
                    if let [LValue::Index(v, idx)] = lhs.as_slice() {
                        if idx.len() == 1 && matches!(&idx[0], Expr::Binary(BinOp::Add, a, _) if matches!(a.as_ref(), Expr::End)) && op.is_none() {
                            return Some((vec![head, sel, format!("{v} = [{v}, {}];", show(rhs, true, &renm))], "only indices that pass the test are appended, so select them first".into()));
                        }
                    }
                }
                StmtKind::Expr { expr, .. } => {
                    if let Some(args) = call_of(expr, &["printf", "fprintf"]) {
                        if args.len() >= 2 && matches!(&args[0], Expr::Str(..)) {
                            let f = show(&args[0], false, &("", ""));
                            let parts: Vec<String> = args[1..].iter().map(|a| if uses_var(a, var) { show(a, true, &renm) } else { format!("{} * ones(size({km}))", wrap(a, true, &renm)) }).collect();
                            let data = if parts.len() == 1 { parts[0].clone() } else { format!("[{}]", parts.join("; ")) };
                            return Some((vec![head, sel, format!("printf({f}, {data});")], "only indices that pass the test are printed, so select them first and print in one call".into()));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    None
}

/// C → mlab by transliteration (types dropped, for(;;) → range, {} → end, % → mod, a[i] → a(i+1)).
pub fn c_to_mlab(c: &str) -> Option<String> {
    let mut s = c.replace("int main()", "").replace("int main(void)", "").replace("return 0;", "");
    for inc in ["#include <stdio.h>", "#include<stdio.h>"] {
        s = s.replace(inc, "");
    }
    // arrays: int a[] = {1, 2, 3}; → a = [1, 2, 3];
    let mut out = String::new();
    let toks: Vec<char> = s.chars().collect();
    let mut i = 0;
    // first split into "statements" at ; { }
    let mut units: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut paren = 0i32;
    let mut brace_init = false;
    while i < toks.len() {
        let ch = toks[i];
        if ch == '(' {
            paren += 1;
        }
        if ch == ')' {
            paren -= 1;
        }
        if ch == '=' && toks.get(i + 1).is_some_and(|c| *c == ' ' || *c == '{') {
            let rest: String = toks[i + 1..].iter().collect();
            if rest.trim_start().starts_with('{') {
                brace_init = true;
            }
        }
        if brace_init {
            cur.push(match ch {
                '{' => '[',
                '}' => ']',
                c => c,
            });
            if ch == ';' {
                brace_init = false;
                units.push(std::mem::take(&mut cur));
            }
            i += 1;
            continue;
        }
        if paren == 0 && (ch == ';' || ch == '{' || ch == '}') {
            let t = cur.trim().to_string();
            if !t.is_empty() {
                units.push(t);
            }
            if ch == '{' || ch == '}' {
                units.push(ch.to_string());
            }
            cur.clear();
            i += 1;
            continue;
        }
        cur.push(ch);
        i += 1;
    }
    if !cur.trim().is_empty() {
        units.push(cur.trim().to_string());
    }
    let types = ["unsigned ", "long ", "int ", "double ", "float ", "char ", "const ", "bool "];
    let strip_types = |u: &str| -> String {
        let mut t = u.trim().to_string();
        loop {
            let before = t.clone();
            for ty in types {
                if let Some(r) = t.strip_prefix(ty) {
                    t = r.trim_start().to_string();
                }
            }
            if t == before {
                break;
            }
        }
        // «int a[] = [..]» → «a = [..]»
        t = t.replace("[] =", " =").replace("[]=", " =");
        t
    };
    let fix_expr = |e: &str| -> String {
        let mut e = e.to_string();
        // a[i] → a(i+1)
        let mut r = String::new();
        let cs: Vec<char> = e.chars().collect();
        let mut k = 0;
        while k < cs.len() {
            if cs[k] == '[' && k > 0 && (cs[k - 1].is_alphanumeric() || cs[k - 1] == '_') {
                let mut d = 1;
                let mut j = k + 1;
                while j < cs.len() && d > 0 {
                    if cs[j] == '[' {
                        d += 1;
                    }
                    if cs[j] == ']' {
                        d -= 1;
                    }
                    j += 1;
                }
                let inner: String = cs[k + 1..j - 1].iter().collect();
                r.push_str(&format!("({} + 1)", inner.trim()));
                k = j;
                continue;
            }
            r.push(cs[k]);
            k += 1;
        }
        e = r;
        // X % Y (simple operands) → mod(X, Y)
        while let Some(p) = e.find(" % ") {
            let left: String = e[..p].chars().rev().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '.' || *c == ')' || *c == '(').collect::<String>().chars().rev().collect();
            let right: String = e[p + 3..].chars().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '.').collect();
            if left.is_empty() || right.is_empty() || left.contains('(') != left.contains(')') {
                return String::new();
            }
            e = format!("{}mod({left}, {right}){}", &e[..p - left.len()], &e[p + 3 + right.len()..]);
        }
        e.replace("!=", "~=").replace("true", "true").replace("false", "false")
    };
    let mut lines: Vec<String> = Vec::new();
    let mut open: Vec<bool> = Vec::new(); // whether the block was opened by a construct (for/if/while)
    let mut k = 0;
    while k < units.len() {
        let u = units[k].trim().to_string();
        k += 1;
        if u == "{" {
            open.push(false);
            continue;
        }
        if u == "}" {
            let was = open.pop().unwrap_or(false);
            // else / else if after }
            if let Some(nx) = units.get(k) {
                let n = nx.trim();
                if n.starts_with("else if") {
                    let cond = n.trim_start_matches("else if").trim().trim_start_matches('(').trim_end_matches(')').to_string();
                    lines.push(format!("elseif {}", fix_expr(&cond)));
                    k += 1;
                    if units.get(k).is_some_and(|x| x == "{") {
                        k += 1;
                        open.push(true);
                    }
                    continue;
                }
                if n == "else" {
                    lines.push("else".into());
                    k += 1;
                    if units.get(k).is_some_and(|x| x == "{") {
                        k += 1;
                        open.push(true);
                    }
                    continue;
                }
            }
            if was {
                lines.push("end".into());
            }
            continue;
        }
        let u = strip_types(&u);
        if let Some(rest) = u.strip_prefix("for") {
            let inner = rest.trim().trim_start_matches('(').trim_end_matches(')');
            let parts: Vec<&str> = inner.split(';').map(str::trim).collect();
            if parts.len() != 3 {
                return None;
            }
            let init = strip_types(parts[0]);
            let (v, a) = init.split_once('=')?;
            let (v, a) = (v.trim(), a.trim());
            let cond = parts[1];
            let step = parts[2].replace(' ', "");
            let (op, b) = ["<=", ">=", "<", ">"].iter().find_map(|op| cond.split_once(op).map(|(_, b)| (*op, b.trim())))?;
            let st: String = if step == format!("{v}++") || step == format!("++{v}") {
                "1".into()
            } else if step == format!("{v}--") || step == format!("--{v}") {
                "-1".into()
            } else if let Some(x) = step.strip_prefix(&format!("{v}+=")) {
                x.to_string()
            } else if let Some(x) = step.strip_prefix(&format!("{v}-=")) {
                format!("-{x}")
            } else {
                return None;
            };
            let end = match op {
                "<=" | ">=" => fix_expr(b),
                "<" => format!("{} - 1", fix_expr(b)),
                _ => format!("{} + 1", fix_expr(b)),
            };
            lines.push(if st == "1" { format!("for {v} = {}:{end}", fix_expr(a)) } else { format!("for {v} = {}:{st}:{end}", fix_expr(a)) });
            if units.get(k).is_some_and(|x| x == "{") {
                k += 1;
                open.push(true);
            } else {
                // single-statement body
                if let Some(b) = units.get(k) {
                    lines.push(format!("{};", fix_expr(&strip_types(b))));
                    k += 1;
                }
                lines.push("end".into());
            }
            continue;
        }
        for kw in ["while", "if"] {
            if u.starts_with(kw) && u[kw.len()..].trim_start().starts_with('(') {
                let cond = u[kw.len()..].trim().trim_start_matches('(').trim_end_matches(')');
                lines.push(format!("{kw} {}", fix_expr(cond)));
                if units.get(k).is_some_and(|x| x == "{") {
                    k += 1;
                    open.push(true);
                } else {
                    if let Some(b) = units.get(k) {
                        lines.push(format!("{};", fix_expr(&strip_types(b))));
                        k += 1;
                    }
                    lines.push("end".into());
                }
                break;
            }
        }
        if u.starts_with("while") || u.starts_with("if") {
            continue;
        }
        let e = fix_expr(&u);
        if e.is_empty() {
            return None;
        }
        lines.push(format!("{e};"));
    }
    out.push_str(&lines.join("\n"));
    out.push('\n');
    if mlab::parser::parse_program(&out).is_ok() { Some(out) } else { None }
}

fn step(diff: &'static str, kind: &str, goal: &str, motive: &str, fast: Vec<String>, stepw: Option<Vec<String>>) -> Step {
    Step { diff, kind: kind.into(), goal: goal.into(), motive: motive.into(), why: "architect change".into(), fast, step: stepw }
}

/// a1: rewrite given code (mlab or C) — vectorise loops via the AST.
fn rewrite(t: &Public) -> Option<Solution> {
    let lit = lemma::literals(&t.description);
    let code = lit.code.iter().max_by_key(|c| c.len())?.clone();
    let is_c = code.contains('{') || code.contains("int ") || code.contains("for (") || code.contains("#include");
    let (src, from_c) = if is_c { (c_to_mlab(&code)?, true) } else { (code.replace("; ", ";\n"), false) };
    let prog = mlab::parser::parse_program(&src).ok()?;
    let mut steps: Vec<Step> = Vec::new();
    if from_c {
        steps.push(step("represent", "Convert", "transliterate the C fragment into mlab", "the same statements exist in mlab: types vanish, for(;;) becomes a range, braces become end, % becomes mod, a[i] becomes a(i+1)", vec!["% (C fragment transliterated below)".into()], None));
    }
    let mut vectorized = 0;
    let mut kept = 0;
    for s in &prog.body {
        match &s.kind {
            StmtKind::For { var, iter, body } => match vectorize_for(var, iter, body) {
                Some((lines, why)) => {
                    vectorized += 1;
                    let mut loop_lines = Vec::new();
                    show_stmt(s, "", &mut loop_lines);
                    steps.push(step("transform", "Map", &format!("replace the loop over {var} by array operations"), &why, lines, Some(loop_lines)));
                }
                None => {
                    kept += 1;
                    let mut l = Vec::new();
                    show_stmt(s, "", &mut l);
                    steps.push(step("repeat", "Iterate", &format!("keep the loop over {var} (no vector pattern recognised)"), "its body is not a single accumulate, count, build or print step, so a vector form is not certain", l, None));
                }
            },
            _ => {
                let mut l = Vec::new();
                show_stmt(s, "", &mut l);
                steps.push(step("given", "Bind", "keep this statement", "it does not loop, so it is already in array form", l.clone(), Some(l)));
            }
        }
    }
    let sp = v1::understand(&t.description);
    let mut sol = v1::finish(t, &sp, steps, vec!["a1:transcribe".into()]);
    sol.explanation = format!("Rewrite of the given {} code: {vectorized} loop(s) vectorised by AST patterns (accumulate, count, build, print), {kept} kept as loops.\n{}", if from_c { "C" } else { "mlab" }, sol.explanation);
    Some(sol)
}

/// a2: "function NAME(params)" — body via the motivated plan, calls on the test inputs.
fn function_task(t: &Public, model: &Model, flags: &Flags) -> Option<Solution> {
    let d = &t.description;
    let low = d.to_lowercase();
    let p = low.find("function")?;
    let rest = &d[p + "function".len()..];
    // first "name(" after the word function
    let open = rest.find('(')?;
    let name: String = rest[..open].split_whitespace().last()?.trim_matches(|c: char| !(c.is_alphanumeric() || c == '_')).to_string();
    if name.is_empty() || !name.chars().next()?.is_alphabetic() || ["printf", "disp"].contains(&name.as_str()) {
        return None;
    }
    let close = rest[open..].find(')')? + open;
    let params: Vec<String> = rest[open + 1..close].split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    if params.is_empty() || params.iter().any(|p| !p.chars().all(|c| c.is_alphanumeric() || c == '_')) {
        return None;
    }
    // body: text after the signature up to the first period
    let body_txt: String = rest[close + 1..].split(". ").next().unwrap_or("").to_string();
    let lit = lemma::literals(d);
    // calls: printf with NAME( in the description — verbatim; otherwise numbers after "on/for/with"
    let calls: Vec<String> = lit.prints.iter().filter(|c| c.contains(&format!("{name}("))).map(|c| format!("{c};")).collect();
    let fmt = lit.formats.first().cloned().unwrap_or_else(|| "%d\\n".into());
    let harness = if calls.is_empty() && flags.on("b3:func-harness") { crate::arch3::harness_calls(d, &name, &params, &fmt) } else { None };
    let used_harness = harness.is_some();
    let calls = if let Some(h) = harness {
        h
    } else if calls.is_empty() {
        let tail = low.rsplit_once(" on ").or_else(|| low.rsplit_once(" for ")).or_else(|| low.rsplit_once(" with ")).map(|x| x.1.to_string()).unwrap_or_default();
        let nums: Vec<String> = tail.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-')).filter(|x| x.parse::<f64>().is_ok()).map(|x| x.trim_end_matches('.').to_string()).collect();
        if nums.is_empty() {
            return None;
        }
        if params.len() == 1 {
            nums.iter().map(|n| format!("printf('{fmt}', {name}({n}));")).collect()
        } else {
            nums.chunks(params.len()).filter(|c| c.len() == params.len()).map(|c| format!("printf('{fmt}', {name}({}));", c.join(", "))).collect()
        }
    } else {
        calls
    };
    // body: plan on the body text, the given value is the first parameter
    let mut bsp = v1::understand(&body_txt);
    // range with a parameter: "from 1 to n"
    if bsp.range.is_none() {
        let w: Vec<&str> = body_txt.split(|c: char| !(c.is_alphanumeric() || c == '_')).filter(|x| !x.is_empty()).collect();
        for i in 0..w.len().saturating_sub(3) {
            if (w[i] == "from" || w[i] == "between") && (w[i + 2] == "to" || w[i + 2] == "and") && (params.iter().any(|p| p == w[i + 3]) || w[i + 3].parse::<f64>().is_ok()) {
                bsp.range = Some((w[i + 1].to_string(), w[i + 3].to_string()));
            }
        }
    }
    // b1: body described by imperative sentences — translate the clauses (text up to "call it / test it / print")
    if flags.on("b1:procedure") {
        let full_body = rest[close + 1..].to_string();
        let low_b = full_body.to_lowercase();
        let cut = ["call it", "test it", "apply it", "then print", "print "].iter().filter_map(|k| low_b.find(k)).min().unwrap_or(full_body.len());
        if let Some((lines, ok, cues)) = crate::arch3::transcribe(&full_body[..cut]) {
            let has_out = lines.iter().any(|l| l.trim_start().starts_with("out = "));
            let last = lines.iter().rev().find_map(|l| l.trim().split_once(" = ").map(|(a, _)| a.trim().to_string())).unwrap_or_else(|| params[0].clone());
            let sig = format!("function out = {name}({})", params.join(", "));
            let mut body = vec![sig, "  % carry out the procedure exactly as the task states it: each clause becomes one statement.".into()];
            body.extend(lines.iter().map(|l| format!("  {l}")));
            if !has_out {
                body.push(format!("  out = {last};"));
            }
            body.push("end".into());
            let steps = vec![
                step("present", "Output", "call the function on the requested inputs and print each result", "the task asks for the results of the function on concrete inputs", calls.clone(), Some(calls.clone())),
                step("reuse", "Define", &format!("define {name} once for all inputs"), "the same computation is needed for several inputs, so wrap it in a function", body, None),
            ];
            let sp = v1::understand(d);
            let mut u = vec!["a2:func-multi".to_string(), "a2:function".into(), "b1:procedure".into()];
            if used_harness {
                u.push("b3:func-harness".into());
            }
            let mut sol = v1::finish(t, &sp, steps, u);
            sol.explanation = format!("Function task: {name}({}); body transcribed from {ok} of {cues} procedural clauses.\n{}", params.join(", "), sol.explanation);
            return Some(sol);
        }
    }
    let val = if body_txt.contains("string") || body_txt.contains("text") { Val::Str } else if body_txt.contains("vector") || body_txt.contains("array") || body_txt.contains("elements") { Val::Vec } else { Val::Scalar };
    let start = if bsp.range.is_some() { None } else { Some((params[0].clone(), val)) };
    let mut body = v1::plan(&body_txt, &bsp, model, start).ok()?;
    // last step (presentation) → out = …
    body.pop();
    let last = body.iter().rev().flat_map(|s| s.fast.iter().rev()).find_map(|l| l.split_once(" = ").map(|(a, _)| a.trim().to_string())).unwrap_or_else(|| params[0].clone());
    let mut steps: Vec<Step> = Vec::new();
    steps.push(step("present", "Output", "call the function on the requested inputs and print each result", "the task asks for the results of the function on concrete inputs", calls.clone(), Some(calls.clone())));
    let sig = format!("function out = {name}({})", params.join(", "));
    let mut fast = vec![sig.clone()];
    let mut stepw = vec![sig];
    for s in &body {
        fast.push(format!("  % {} -> {}: {}. Motive: {}.", s.diff, s.kind, s.goal, s.motive.split(" (math:").next().unwrap_or("")));
        stepw.push(format!("  % {} -> {}: {}.", s.diff, s.kind, s.goal));
        fast.extend(s.fast.iter().map(|l| format!("  {l}")));
        stepw.extend(s.step.as_ref().unwrap_or(&s.fast).iter().map(|l| format!("  {l}")));
    }
    fast.push(format!("  out = {last};"));
    stepw.push(format!("  out = {last};"));
    fast.push("end".into());
    stepw.push("end".into());
    let has_step = body.iter().any(|s| s.step.as_ref().is_some_and(|l| l != &s.fast));
    steps.push(step("reuse", "Define", &format!("define {name} once for all inputs"), "the same computation is needed for several inputs, so wrap it in a function", fast, if has_step { Some(stepw) } else { None }));
    let sp = v1::understand(d);
    let mut u = vec!["a2:func-multi".to_string(), "a2:function".into()];
    if used_harness {
        u.push("b3:func-harness".into());
    }
    let mut sol = v1::finish(t, &sp, steps, u);
    sol.explanation = format!("Function task: {name}({}) with body planned from «{}».\n{}", params.join(", "), body_txt.chars().take(120).collect::<String>(), sol.explanation);
    Some(sol)
}

/// a2: several printf in the description — several subgoals, each from the same data.
fn multi_goal(t: &Public, model: &Model) -> Option<Solution> {
    let d = &t.description;
    let lit = lemma::literals(d);
    let prints: Vec<&String> = lit.prints.iter().filter(|p| p.contains('\'')).collect();
    if prints.len() < 2 {
        return None;
    }
    let full = v1::understand(d);
    let (dname, dval) = full.data.first()?.clone();
    let val = if dval.starts_with('\'') { Val::Str } else if dval.starts_with('[') { Val::Vec } else { Val::Scalar };
    let mut steps: Vec<Step> = Vec::new();
    let given: Vec<String> = full.data.iter().map(|(n, v)| format!("{n} = {v};")).collect();
    steps.push(step("given", "Bind", "name the given data", "name the data first, so every later step can refer to it", given.clone(), Some(given)));
    let mut from = 0usize;
    let mut done = 0;
    for (i, p) in prints.iter().enumerate() {
        let Some(pos) = d[from..].find(p.as_str()).map(|x| x + from) else { continue };
        let seg = &d[from..pos + p.len()];
        from = pos + p.len();
        let ssp = v1::understand(seg);
        let copy = format!("{dname}{}", i + 1);
        let mut seg_steps = match v1::plan(seg, &ssp, model, Some((copy.clone(), val.clone()))) {
            Ok(s) => s,
            Err(_) => continue,
        };
        done += 1;
        steps.push(step("given", "Bind", &format!("start goal {} from a copy of the data", i + 1), "each goal starts from the original data, so earlier goals must not change it", vec![format!("{copy} = {dname};")], Some(vec![format!("{copy} = {dname};")])));
        steps.append(&mut seg_steps);
    }
    if done < 2 {
        return None;
    }
    let mut sol = v1::finish(t, &full, steps, vec!["a2:func-multi".into(), "a2:multi-goal".into()]);
    sol.explanation = format!("Multi-goal task: {} print requests, {done} planned as separate goals.\n{}", prints.len(), sol.explanation);
    Some(sol)
}

/// a3: structural operators (matrix by rows/columns, sorting by a key, set operations on two vectors).
fn struct_ops(t: &Public, flags: &Flags) -> Option<Solution> {
    let d = &t.description;
    let low = lemma::strip_literals(d).to_lowercase();
    let full = v1::understand(d);
    let lit = &full.lit;
    let fmt = lit.formats.first().cloned().unwrap_or_else(|| "%d\\n".into());
    let print = |x: &str| lit.prints.iter().find(|p| p.contains('\'')).map(|p| format!("{p};")).unwrap_or_else(|| format!("printf('{fmt}', {x});"));
    let given: Vec<String> = full.data.iter().map(|(n, v)| format!("{n} = {v};")).collect();
    // b2: replacing elements by a condition: "replace every negative entry with 0"
    if flags.on("b2:literal-repair") && low.contains("replace") && !full.conds.is_empty() {
        if let Some((m, _)) = full.data.iter().find(|(_, v)| v.starts_with('[')) {
            let k = low.split(" with ").nth(1).and_then(|x| x.split_whitespace().next()).map(|x| x.trim_end_matches([',', '.']).to_string()).filter(|x| x.parse::<f64>().is_ok());
            if let Some(k) = k {
                let c = full.conds.iter().map(|(_, c)| format!("({})", c.replace('X', m))).collect::<Vec<_>>().join(" & ");
                let mut lines = vec![format!("{m}({c}) = {k};")];
                let prints: Vec<String> = lit.prints.iter().filter(|p| p.contains('\'')).map(|p| format!("{p};")).collect();
                let prints = if prints.is_empty() { vec![format!("disp(mat2str({m}));")] } else { prints };
                if low.contains("count") || low.contains("how many") {
                    lines.insert(0, format!("cnt = nnz({c});"));
                }
                let stepw = vec![format!("for i = 1:numel({m})"), format!("  if {}", full.conds.iter().map(|(_, c)| format!("({})", c.replace('X', &format!("{m}(i)")))).collect::<Vec<_>>().join(" && ")), format!("    {m}(i) = {k};"), "  end".into(), "end".into()];
                let steps = vec![
                    step("given", "Bind", "name the given matrix", "name the data first, so every later step can refer to it", given.clone(), Some(given.clone())),
                    step("select", "Filter", "overwrite the entries that meet the condition", "only some entries qualify, and a logical mask addresses exactly them without changing the shape", lines, if low.contains("count") { None } else { Some(stepw) }),
                    step("present", "Output", "print the answer in the requested format", "the answer must be shown exactly in the requested format", prints.clone(), Some(prints)),
                ];
                return Some(v1::finish(t, &full, steps, vec!["a3:struct-ops".into(), "b2:literal-repair".into()]));
            }
        }
    }
    // matrix: reduction by rows/columns
    if let Some((m, _)) = full.data.iter().find(|(_, v)| v.starts_with('[') && v.contains(';')) {
        let dim = if low.contains("each row") || low.contains("every row") || low.contains("row sums") || low.contains("per row") {
            Some(("2", "row"))
        } else if low.contains("each column") || low.contains("every column") || low.contains("column sums") || low.contains("per column") {
            Some(("1", "column"))
        } else {
            None
        };
        if let (Some((dm, what)), Some(agg)) = (dim, full.agg.clone()) {
            let f = if agg == "count" { "numel".to_string() } else { agg.clone() };
            let r = format!("r = {f}({m}, {dm});");
            let r = if f == "max" || f == "min" { format!("r = {f}({m}, [], {dm});") } else { r };
            let stepw = if agg == "sum" {
                let (outer, inner, acc) = if dm == "2" { ("i = 1:size(M, 1)", "j = 1:size(M, 2)", "r(i) = r(i) + M(i, j);") } else { ("j = 1:size(M, 2)", "i = 1:size(M, 1)", "r(j) = r(j) + M(i, j);") };
                Some(vec![format!("r = zeros(1, size({m}, {}));", if dm == "2" { "1" } else { "2" }), format!("for {}", outer.replace('M', m)), format!("  for {}", inner.replace('M', m)), format!("    {}", acc.replace('M', m)), "  end".into(), "end".into()])
            } else {
                None
            };
            let steps = vec![
                step("given", "Bind", "name the given matrix", "name the data first, so every later step can refer to it", given.clone(), Some(given.clone())),
                step("aggregate", "Reduce", &format!("reduce each {what} to one number ({agg})"), &format!("many values in each {what} must become one number; the dimension argument does it for all {what}s at once"), vec![r], stepw),
                step("present", "Output", "print the answer in the requested format", "the answer must be shown exactly in the requested format", vec![print("r")], Some(vec![print("r")])),
            ];
            return Some(v1::finish(t, &full, steps, vec!["a3:struct-ops".into(), "a3:matrix".into()]));
        }
    }
    let vecs: Vec<&(String, String)> = full.data.iter().filter(|(_, v)| v.starts_with('[') && !v.contains(';')).collect();
    if vecs.len() >= 2 {
        let (a, b) = (&vecs[0].0, &vecs[1].0);
        // sorting by a key: "X sorted/ordered by Y"
        let wide = flags.on("b2:literal-repair") && (low.contains("highest to lowest") || low.contains("lowest to highest") || low.contains("ranked by") || low.contains("largest to smallest") || low.contains("smallest to largest"));
        if wide || low.contains("sorted by") || low.contains("ordered by") || low.contains("in order of") || low.contains("according to") {
            let desc = low.contains("descend") || low.contains("highest") || low.contains("largest first") || low.contains("decreasing");
            let dir = if desc { ", 'descend'" } else { "" };
            let steps = vec![
                step("given", "Bind", "name the given vectors", "name the data first, so every later step can refer to it", given.clone(), Some(given.clone())),
                step("order", "Sort", &format!("order {a} by the keys in {b}"), "the order is decided by another vector, so sort the keys and reuse their permutation", vec![format!("[~, idx] = sort({b}{dir});"), format!("r = {a}(idx);")], None),
                step("present", "Output", "print the answer in the requested format", "the answer must be shown exactly in the requested format", vec![print("r")], Some(vec![print("r")])),
            ];
            let mut u = vec!["a3:struct-ops".to_string(), "a3:keyed-sort".into()];
            if wide {
                u.push("b2:literal-repair".into());
            }
            return Some(v1::finish(t, &full, steps, u));
        }
        let setop = if low.contains("intersection") || low.contains("in both") || low.contains("common") {
            Some(("intersection", format!("r = {a}(arrayfun(@(x) any({b} == x), {a}));")))
        } else if low.contains("not in") || low.contains("difference") || low.contains("only in") || low.contains("missing from") {
            Some(("difference", format!("r = {a}(~arrayfun(@(x) any({b} == x), {a}));")))
        } else if low.contains("union") {
            Some(("union", format!("r = [{a}, {b}(~arrayfun(@(x) any({a} == x), {b}))];")))
        } else {
            None
        };
        if let Some((name, line)) = setop {
            let steps = vec![
                step("given", "Bind", "name the given vectors", "name the data first, so every later step can refer to it", given.clone(), Some(given.clone())),
                step("select", "Filter", &format!("keep the {name} of the two vectors, in the original order"), "membership decides which items qualify, so test every item against the other vector", vec![line], None),
                step("present", "Output", "print the answer in the requested format", "the answer must be shown exactly in the requested format", vec![print("r")], Some(vec![print("r")])),
            ];
            return Some(v1::finish(t, &full, steps, vec!["a3:struct-ops".into(), "a3:set".into()]));
        }
    }
    None
}

/// Entry point for the architect's changes (by switches).
pub fn try_changes(t: &Public, _bm: &Bm25, model: &Model, flags: &Flags) -> Option<Solution> {
    let has_code = !lemma::literals(&t.description).code.is_empty();
    if flags.on("a1:transcribe") && has_code {
        if let Some(s) = rewrite(t) {
            return Some(s);
        }
    }
    if flags.on("a2:func-multi") {
        if let Some(s) = function_task(t, model, flags) {
            return Some(s);
        }
    }
    if flags.on("b1:procedure") && !has_code {
        if let Some(s) = crate::arch3::procedure_task(t) {
            return Some(s);
        }
    }
    if flags.on("a3:struct-ops") {
        if let Some(s) = struct_ops(t, flags) {
            return Some(s);
        }
    }
    if flags.on("a2:func-multi") {
        if let Some(s) = multi_goal(t, model) {
            return Some(s);
        }
    }
    None
}
