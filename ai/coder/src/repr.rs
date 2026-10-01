//! Two code representations (design note: "directly — the MMM sees the code as text…; and as a representation of the parsed AST structure"):
//! 1. **text** — mlab lexer tokens and token n-grams (variable names → VAR, numbers → NUM, strings → STR,
//!    built-ins — by their own name); robust to invalid code (if the lexer fails — split by character classes);
//! 2. **AST** — nodes, "parent > child" edges, paths from the statement root (length 3), paths between leaves
//!    via the lowest common ancestor (like code2vec or paths in a UD dependency tree), scopes and
//!    data flow "where defined → where used".
//!
//! Invalid code: partial parsing with recovery (the tree-sitter idea) — the program is split into blocks by
//! keywords; a block that does not parse goes line by line; whatever parses goes in as a tree.

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use mlab::ast::{BinOp, Expr, LValue, Program, Stmt, StmtKind};

fn builtins() -> &'static HashSet<&'static str> {
    static B: OnceLock<HashSet<&'static str>> = OnceLock::new();
    B.get_or_init(|| {
        let mut s: HashSet<&'static str> = mlab::builtins::Registry::with_defaults().names().into_iter().collect();
        for x in ["printf", "fprintf", "disp", "end", "pi", "true", "false", "Inf", "NaN"] {
            s.insert(x);
        }
        s
    })
}

fn norm_ident(s: &str) -> String {
    if builtins().contains(s) { s.to_string() } else { "VAR".into() }
}

/// Text representation: tokens and 2-, 3-grams.
pub fn text_feats(code: &str) -> Vec<String> {
    let toks: Vec<String> = match mlab::lexer::lex(code) {
        Ok(ts) => ts
            .iter()
            .map(|t| match &t.tok {
                mlab::lexer::Tok::Ident(s) => norm_ident(s),
                mlab::lexer::Tok::Num(..) | mlab::lexer::Tok::Imag(..) => "NUM".into(),
                mlab::lexer::Tok::Str(..) => "STR".into(),
                other => format!("{other:?}").split('(').next().unwrap_or("").to_string(),
            })
            .filter(|s| !s.is_empty())
            .collect(),
        Err(_) => rough_tokens(code),
    };
    let mut f: Vec<String> = toks.iter().map(|t| format!("t:{t}")).collect();
    for w in toks.windows(2) {
        f.push(format!("t2:{} {}", w[0], w[1]));
    }
    for w in toks.windows(3) {
        f.push(format!("t3:{} {} {}", w[0], w[1], w[2]));
    }
    f
}

/// Coarse tokens for code the lexer does not accept (unterminated string etc.).
fn rough_tokens(code: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let flush = |cur: &mut String, out: &mut Vec<String>| {
        if !cur.is_empty() {
            let t = if cur.chars().next().is_some_and(|c| c.is_ascii_digit()) { "NUM".to_string() } else { norm_ident(cur) };
            out.push(t);
            cur.clear();
        }
    };
    for c in code.chars() {
        if c.is_alphanumeric() || c == '_' || (c == '.' && cur.chars().next().is_some_and(|d| d.is_ascii_digit())) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out);
            if !c.is_whitespace() {
                out.push(c.to_string());
            }
        }
    }
    flush(&mut cur, &mut out);
    out
}

const OPEN: &[&str] = &["for", "while", "if", "switch", "function", "try", "do", "parfor"];
const CLOSE: &[&str] = &["end", "endfor", "endwhile", "endif", "endswitch", "endfunction", "end_try_catch", "until"];

fn first_word(l: &str) -> &str {
    l.trim_start().split(|c: char| !(c.is_alphanumeric() || c == '_')).next().unwrap_or("")
}

/// Partial parsing: (statements, functions, share of parsed non-empty lines, whether the parse is complete).
pub fn parse_partial(code: &str) -> (Vec<Stmt>, Vec<std::rc::Rc<mlab::ast::FunctionDef>>, f64, bool) {
    if let Ok(p) = mlab::parser::parse_program(code) {
        return (p.body, p.functions, 1.0, true);
    }
    let lines: Vec<&str> = code.lines().collect();
    let total = lines.iter().filter(|l| !l.trim().is_empty()).count().max(1);
    // blocks by keyword depth
    let mut groups: Vec<Vec<&str>> = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    let mut depth = 0i32;
    for l in &lines {
        let w = first_word(l);
        if OPEN.contains(&w) {
            depth += 1;
        }
        // "end" inside an index is not counted: only as the first word of a line
        if CLOSE.contains(&w) {
            depth -= 1;
        }
        cur.push(l);
        if depth <= 0 {
            groups.push(std::mem::take(&mut cur));
            depth = 0;
        }
    }
    if !cur.is_empty() {
        groups.push(cur);
    }
    let mut body = Vec::new();
    let mut funcs = Vec::new();
    let mut ok_lines = 0usize;
    for g in groups {
        let text = g.join("\n") + "\n";
        if let Ok(p) = mlab::parser::parse_program(&text) {
            ok_lines += g.iter().filter(|l| !l.trim().is_empty()).count();
            body.extend(p.body);
            funcs.extend(p.functions);
            continue;
        }
        for l in g {
            let w = first_word(l);
            if OPEN.contains(&w) || CLOSE.contains(&w) || l.trim().is_empty() {
                continue;
            }
            if let Ok(p) = mlab::parser::parse_program(&format!("{l}\n")) {
                ok_lines += 1;
                body.extend(p.body);
            }
        }
    }
    (body, funcs, ok_lines as f64 / total as f64, false)
}

fn bin_name(op: &BinOp) -> String {
    format!("{op:?}")
}

/// Expression node type.
fn etype(e: &Expr) -> String {
    match e {
        Expr::Num(..) | Expr::Imag(..) => "Num".into(),
        Expr::Str(..) => "Str".into(),
        Expr::Ident(s) => {
            if builtins().contains(s.as_str()) {
                format!("Id:{s}")
            } else {
                "Var".into()
            }
        }
        Expr::Colon => "Colon".into(),
        Expr::End => "End".into(),
        Expr::Paren(_) => "Paren".into(),
        Expr::Unary(op, _) => format!("Un:{op:?}"),
        Expr::Postfix(op, _) => format!("Post:{op:?}"),
        Expr::Binary(op, _, _) => format!("Bin:{}", bin_name(op)),
        Expr::Range(..) => "Range".into(),
        Expr::Index(b, _) => match b.as_ref() {
            Expr::Ident(s) if builtins().contains(s.as_str()) => format!("Call:{s}"),
            Expr::Ident(_) => "IndexVar".into(),
            _ => "Index".into(),
        },
        Expr::Matrix(_) => "Matrix".into(),
        Expr::CellList(_) => "CellList".into(),
        Expr::AnonFn(..) => "AnonFn".into(),
        Expr::FuncHandle(_) => "Handle".into(),
        Expr::Field(..) => "Field".into(),
        #[allow(unreachable_patterns)]
        _ => "Other".into(),
    }
}

fn children(e: &Expr) -> Vec<&Expr> {
    match e {
        Expr::Paren(a) | Expr::Unary(_, a) | Expr::Postfix(_, a) | Expr::Field(a, _) => vec![a],
        Expr::Binary(_, a, b) => vec![a, b],
        Expr::Range(a, s, b) => {
            let mut v: Vec<&Expr> = vec![a];
            if let Some(s) = s {
                v.push(s);
            }
            v.push(b);
            v
        }
        Expr::Index(b, args) => {
            let mut v: Vec<&Expr> = Vec::new();
            if !matches!(b.as_ref(), Expr::Ident(_)) {
                v.push(b);
            }
            v.extend(args.iter());
            v
        }
        Expr::Matrix(rows) => rows.iter().flatten().collect(),
        Expr::CellList(v) => v.iter().collect(),
        Expr::AnonFn(_, body) => vec![body.as_ref()],
        _ => vec![],
    }
}

/// Variables used in an expression (with context — the parent node type).
fn uses<'a>(e: &'a Expr, parent: &str, out: &mut Vec<(String, String)>) {
    match e {
        Expr::Ident(s) if !builtins().contains(s.as_str()) => out.push((s.clone(), parent.to_string())),
        Expr::Index(b, _) => {
            if let Expr::Ident(s) = b.as_ref() {
                if !builtins().contains(s.as_str()) {
                    out.push((s.clone(), "IndexVar".into()));
                }
            }
            let t = etype(e);
            for c in children(e) {
                uses(c, &t, out);
            }
        }
        _ => {
            let t = etype(e);
            for c in children(e) {
                uses(c, &t, out);
            }
        }
    }
}

struct Walk {
    f: Vec<String>,
    /// variable → how it was defined (type of the right-hand side)
    defs: HashMap<String, String>,
}

impl Walk {
    fn expr(&mut self, e: &Expr, path: &mut Vec<String>, leaves: &mut Vec<(Vec<String>, String)>) {
        let t = etype(e);
        if let Some(p) = path.last() {
            self.f.push(format!("a2:{p}>{t}"));
        }
        if path.len() >= 2 {
            self.f.push(format!("a3:{}>{}>{t}", path[path.len() - 2], path[path.len() - 1]));
        }
        self.f.push(format!("a:{t}"));
        path.push(t.clone());
        let ch = children(e);
        if ch.is_empty() {
            leaves.push((path.clone(), t.clone()));
        }
        for c in ch {
            self.expr(c, path, leaves);
        }
        path.pop();
    }

    fn stmts(&mut self, body: &[Stmt], scope: &str) {
        for s in body {
            self.stmt(s, scope);
        }
    }

    fn stmt(&mut self, s: &Stmt, scope: &str) {
        let (name, exprs): (String, Vec<&Expr>) = match &s.kind {
            StmtKind::Expr { expr, .. } => ("S:Expr".into(), vec![expr]),
            StmtKind::Assign { rhs, op, .. } => (if op.is_some() { "S:AssignOp".into() } else { "S:Assign".into() }, vec![rhs]),
            StmtKind::If { clauses, .. } => ("S:If".into(), clauses.iter().map(|c| &c.0).collect()),
            StmtKind::For { iter, .. } => ("S:For".into(), vec![iter]),
            StmtKind::While { cond, .. } => ("S:While".into(), vec![cond]),
            StmtKind::DoUntil { cond, .. } => ("S:DoUntil".into(), vec![cond]),
            StmtKind::Switch { subject, .. } => ("S:Switch".into(), vec![subject]),
            StmtKind::Try { .. } => ("S:Try".into(), vec![]),
            StmtKind::Break => ("S:Break".into(), vec![]),
            StmtKind::Continue => ("S:Continue".into(), vec![]),
            StmtKind::Return => ("S:Return".into(), vec![]),
            StmtKind::Command { .. } => ("S:Command".into(), vec![]),
        };
        self.f.push(format!("a:{name}"));
        self.f.push(format!("sc:{scope}>{name}"));
        for e in &exprs {
            let mut path = vec![name.clone()];
            let mut leaves = Vec::new();
            self.expr(e, &mut path, &mut leaves);
            // paths between leaves via the lowest common ancestor (first 6 leaves)
            for i in 0..leaves.len().min(6) {
                for j in i + 1..leaves.len().min(6) {
                    let (a, b) = (&leaves[i].0, &leaves[j].0);
                    let k = a.iter().zip(b.iter()).take_while(|(x, y)| x == y).count();
                    let up: Vec<&str> = a[k..].iter().rev().map(String::as_str).collect();
                    let down: Vec<&str> = b[k..].iter().map(String::as_str).collect();
                    let lca = if k > 0 { a[k - 1].as_str() } else { "^" };
                    self.f.push(format!("lp:{}^{lca}_{}", up.join("^"), down.join("_")));
                }
            }
            let mut u = Vec::new();
            uses(e, &name, &mut u);
            for (v, ctx) in u {
                let d = self.defs.get(&v).cloned().unwrap_or_else(|| "undef".into());
                self.f.push(format!("df:{d}->{ctx}"));
            }
        }
        match &s.kind {
            StmtKind::Assign { lhs, rhs, op, .. } => {
                let rt = etype(rhs);
                for l in lhs {
                    let v = match l {
                        LValue::Var(v) | LValue::Index(v, _) | LValue::Field(v, _) => v.clone(),
                        LValue::Tilde => continue,
                    };
                    // accumulation: x = x + … or x += …
                    let mut u = Vec::new();
                    uses(rhs, "", &mut u);
                    if op.is_some() || u.iter().any(|(n, _)| *n == v) {
                        self.f.push(format!("df:accumulate:{rt}"));
                    }
                    self.defs.insert(v, rt.clone());
                }
            }
            StmtKind::For { var, body, .. } => {
                self.defs.insert(var.clone(), "loopvar".into());
                self.stmts(body, "for");
            }
            StmtKind::While { body, .. } | StmtKind::DoUntil { body, .. } => self.stmts(body, "while"),
            StmtKind::If { clauses, else_body } => {
                for (_, b) in clauses {
                    self.stmts(b, "if");
                }
                if let Some(b) = else_body {
                    self.stmts(b, "else");
                }
            }
            StmtKind::Switch { cases, otherwise, .. } => {
                for (_, b) in cases {
                    self.stmts(b, "case");
                }
                if let Some(b) = otherwise {
                    self.stmts(b, "case");
                }
            }
            StmtKind::Try { body, catch_body, .. } => {
                self.stmts(body, "try");
                self.stmts(catch_body, "catch");
            }
            _ => {}
        }
    }
}

/// AST representation (partial parsing for invalid code). Returns (features, share parsed).
pub fn ast_feats(code: &str) -> (Vec<String>, f64) {
    let (body, funcs, frac, _) = parse_partial(code);
    let mut w = Walk { f: Vec::new(), defs: HashMap::new() };
    w.stmts(&body, "script");
    for fd in &funcs {
        w.f.push(format!("sc:function/{}in/{}out", fd.params.len(), fd.outputs.len()));
        let saved = std::mem::take(&mut w.defs);
        for p in &fd.params {
            w.defs.insert(p.clone(), "param".into());
        }
        w.stmts(&fd.body, "function");
        w.defs = saved;
    }
    (w.f, frac)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Text,
    Ast,
    Both,
}

pub fn feats(code: &str, mode: Mode) -> Vec<String> {
    match mode {
        Mode::Text => text_feats(code),
        Mode::Ast => ast_feats(code).0,
        Mode::Both => {
            let mut f = text_feats(code);
            f.extend(ast_feats(code).0);
            f
        }
    }
}

/// The move a statement makes (label for "move choice" on code).
pub fn stmt_move(s: &Stmt) -> &'static str {
    fn call_name(e: &Expr) -> Option<&str> {
        if let Expr::Index(b, _) = e {
            if let Expr::Ident(n) = b.as_ref() {
                return Some(n.as_str());
            }
        }
        None
    }
    match &s.kind {
        StmtKind::For { .. } | StmtKind::While { .. } | StmtKind::DoUntil { .. } => "Iterate",
        StmtKind::If { .. } | StmtKind::Switch { .. } | StmtKind::Try { .. } => "Branch",
        StmtKind::Expr { expr, print } => match call_name(expr) {
            Some("printf" | "fprintf" | "disp" | "display") => "Output",
            _ if *print => "Output",
            _ => "Arith",
        },
        StmtKind::Assign { rhs, .. } => {
            let n = call_name(rhs).unwrap_or("");
            match n {
                "sum" | "prod" | "max" | "min" | "mean" | "median" | "numel" | "nnz" | "any" | "all" | "length" | "std" | "var" | "mode" | "norm" | "det" | "trace" | "rank" => {
                    "Reduce"
                }
                "sort" | "unique" | "sortrows" | "fliplr" | "flipud" => "Sort",
                "find" | "strfind" | "ismember" => "Search",
                "num2str" | "sprintf" | "upper" | "lower" | "strrep" | "strtrim" | "mat2str" | "str2num" | "str2double" | "strcat" | "char" | "double" | "int2str" => "Convert",
                _ => match rhs {
                    Expr::Num(..) | Expr::Str(..) | Expr::Matrix(_) => "Bind",
                    Expr::Range(..) => "Range",
                    Expr::Index(b, args) if matches!(b.as_ref(), Expr::Ident(v) if !builtins().contains(v.as_str())) && args.iter().any(|a| matches!(a, Expr::Binary(op, _, _) if matches!(op, BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge | BinOp::And | BinOp::Or))) => "Filter",
                    Expr::Binary(op, _, _) if matches!(op, BinOp::EMul | BinOp::EDiv | BinOp::EPow) => "Map",
                    _ if n.is_empty() => "Arith",
                    _ => "Map",
                },
            }
        }
        _ => "Arith",
    }
}

pub fn program(code: &str) -> Option<Program> {
    mlab::parser::parse_program(code).ok()
}
