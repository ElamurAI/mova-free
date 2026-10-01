//! Parser: recursive descent with the language's precedences (from lowest):
//! `||` < `&&` < `|` < `&` < comparison < `:` < `+ -` < `* / \ .* ./ .\` < unary `+ - ~` < `^ .^` < postfix `' .'` and indexing.
//! Power is left-associative (`2^3^2 = 64`), `-2^2 = -4`, `2^-1` — unary in the exponent.
//! In `[...]` a space separates elements: `[1 -2]` — two elements, `[1 - 2]` — a difference, `[a (1)]` — two elements.

use crate::ast::*;
use crate::lexer::{ParseError, Tok, Token, is_keyword};
use std::rc::Rc;

pub struct Parser {
    toks: Vec<Token>,
    pos: usize,
    in_matrix: bool,
    index_depth: usize,
}

const COMMAND_WORDS: &[&str] = &["format", "clc", "clear", "close", "more", "warning", "pkg", "hold", "figure", "clearvars"];

pub fn parse_program(src: &str) -> Result<Program, ParseError> {
    let toks = crate::lexer::lex(src)?;
    let mut p = Parser { toks, pos: 0, in_matrix: false, index_depth: 0 };
    p.program()
}

type PResult<T> = Result<T, ParseError>;

impl Parser {
    fn peek(&self) -> &Tok {
        &self.toks[self.pos].tok
    }
    fn peek_at(&self, k: usize) -> &Tok {
        let i = (self.pos + k).min(self.toks.len() - 1);
        &self.toks[i].tok
    }
    fn tok(&self) -> &Token {
        &self.toks[self.pos]
    }
    fn tok_at(&self, k: usize) -> &Token {
        let i = (self.pos + k).min(self.toks.len() - 1);
        &self.toks[i]
    }
    fn advance(&mut self) -> Token {
        let t = self.toks[self.pos].clone();
        if self.pos < self.toks.len() - 1 {
            self.pos += 1;
        }
        t
    }
    fn err<T>(&self, msg: impl Into<String>) -> PResult<T> {
        let t = self.tok();
        Err(ParseError { line: t.line, col: t.col, msg: msg.into() })
    }
    fn expect(&mut self, want: Tok, what: &str) -> PResult<()> {
        if *self.peek() == want {
            self.advance();
            Ok(())
        } else {
            self.err(format!("expected {what}"))
        }
    }
    fn is_kw(&self, k: &str) -> bool {
        matches!(self.peek(), Tok::Ident(s) if s == k)
    }
    fn is_kw_any(&self, ks: &[&str]) -> bool {
        matches!(self.peek(), Tok::Ident(s) if ks.contains(&s.as_str()))
    }
    fn skip_separators(&mut self) {
        while matches!(self.peek(), Tok::Newline | Tok::Semi | Tok::Comma) {
            self.advance();
        }
    }
    fn skip_newlines(&mut self) {
        while matches!(self.peek(), Tok::Newline) {
            self.advance();
        }
    }

    fn program(&mut self) -> PResult<Program> {
        let mut prog = Program::default();
        loop {
            self.skip_separators();
            if *self.peek() == Tok::Eof {
                break;
            }
            if self.is_kw("function") {
                let f = self.function()?;
                prog.functions.push(Rc::new(f));
            } else {
                let s = self.statement()?;
                prog.body.push(s);
            }
        }
        Ok(prog)
    }

    fn block(&mut self, terms: &[&str]) -> PResult<Vec<Stmt>> {
        let mut out = Vec::new();
        loop {
            self.skip_separators();
            if *self.peek() == Tok::Eof {
                return self.err(format!("missing '{}'", terms[0]));
            }
            if self.is_kw_any(terms) {
                return Ok(out);
            }
            if self.is_kw("function") {
                return self.err("nested function definitions are not supported");
            }
            out.push(self.statement()?);
        }
    }

    fn function(&mut self) -> PResult<FunctionDef> {
        self.advance(); // function
        let mut outputs = Vec::new();
        if *self.peek() == Tok::LBracket {
            self.advance();
            loop {
                match self.peek().clone() {
                    Tok::RBracket => {
                        self.advance();
                        break;
                    }
                    Tok::Comma => {
                        self.advance();
                    }
                    Tok::Ident(s) => {
                        self.advance();
                        outputs.push(s);
                    }
                    _ => return self.err("expected output name"),
                }
            }
            self.expect(Tok::Assign, "'='")?;
        } else if matches!(self.peek(), Tok::Ident(_)) && *self.peek_at(1) == Tok::Assign {
            if let Tok::Ident(s) = self.advance().tok {
                outputs.push(s);
            }
            self.advance();
        }
        let name = match self.advance().tok {
            Tok::Ident(s) => s,
            _ => return self.err("expected function name"),
        };
        let mut params = Vec::new();
        if *self.peek() == Tok::LParen {
            self.advance();
            loop {
                match self.peek().clone() {
                    Tok::RParen => {
                        self.advance();
                        break;
                    }
                    Tok::Comma => {
                        self.advance();
                    }
                    Tok::Ident(s) => {
                        self.advance();
                        params.push(s);
                    }
                    Tok::Not => {
                        self.advance();
                        params.push("~".into());
                    }
                    _ => return self.err("expected parameter name"),
                }
            }
        }
        // body — up to end/endfunction, up to the next function or to the end of the file (functions without end)
        let mut body = Vec::new();
        loop {
            self.skip_separators();
            if *self.peek() == Tok::Eof || self.is_kw("function") {
                break;
            }
            if self.is_kw_any(&["end", "endfunction"]) {
                self.advance();
                break;
            }
            body.push(self.statement()?);
        }
        Ok(FunctionDef { name, params, outputs, body })
    }

    fn end_of_statement(&mut self) -> PResult<bool> {
        // returns print: true if not «;»
        match self.peek() {
            Tok::Semi => {
                self.advance();
                Ok(false)
            }
            Tok::Comma | Tok::Newline => {
                self.advance();
                Ok(true)
            }
            Tok::Eof => Ok(true),
            _ => {
                // allow a block keyword right after a statement (e.g. «if x, y = 1 end» is not allowed)
                let d = self.peek().describe();
                self.err(format!("unexpected '{d}'"))
            }
        }
    }

    fn statement(&mut self) -> PResult<Stmt> {
        let line = self.tok().line;
        if let Tok::Ident(w) = self.peek().clone() {
            if is_keyword(&w) {
                let kind = match w.as_str() {
                    "if" => self.if_stmt()?,
                    "for" | "parfor" => self.for_stmt()?,
                    "while" => self.while_stmt()?,
                    "do" => self.do_stmt()?,
                    "switch" => self.switch_stmt()?,
                    "try" => self.try_stmt()?,
                    "break" => {
                        self.advance();
                        StmtKind::Break
                    }
                    "continue" => {
                        self.advance();
                        StmtKind::Continue
                    }
                    "return" => {
                        self.advance();
                        StmtKind::Return
                    }
                    "global" | "persistent" => return self.err(format!("'{w}' variables are not supported in mlab v1")),
                    _ => return self.err(format!("unexpected '{w}'")),
                };
                return Ok(Stmt { kind, line });
            }
            // command syntax: format long, clear x, warning off
            if COMMAND_WORDS.contains(&w.as_str()) {
                let next = self.tok_at(1).clone();
                let is_cmd = match &next.tok {
                    Tok::Newline | Tok::Semi | Tok::Comma | Tok::Eof => true,
                    Tok::Ident(_) | Tok::Num(..) => next.space_before && !matches!(self.peek_at(2), Tok::Assign | Tok::LParen),
                    _ => false,
                };
                if is_cmd {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.peek(), Tok::Newline | Tok::Semi | Tok::Comma | Tok::Eof) {
                        let t = self.advance();
                        args.push(match t.tok {
                            Tok::Ident(s) => s,
                            Tok::Num(_, s) => s,
                            Tok::Str(s, _) => s,
                            other => other.text().to_string(),
                        });
                    }
                    let print = self.end_of_statement()?;
                    return Ok(Stmt { kind: StmtKind::Command { name: w, args, print }, line });
                }
            }
        }
        // x++ / x-- (Octave): like x += 1 / x -= 1
        if let Tok::Ident(name) = self.peek().clone() {
            let (t1, t2) = (self.tok_at(1).clone(), self.tok_at(2).clone());
            let inc = matches!((&t1.tok, &t2.tok), (Tok::Plus, Tok::Plus) | (Tok::Minus, Tok::Minus)) && !t2.space_before;
            if inc && matches!(self.tok_at(3).tok, Tok::Newline | Tok::Semi | Tok::Comma | Tok::Eof) {
                let op = if t1.tok == Tok::Plus { BinOp::Add } else { BinOp::Sub };
                self.advance();
                self.advance();
                self.advance();
                let print = self.end_of_statement()?;
                return Ok(Stmt {
                    kind: StmtKind::Assign { lhs: vec![LValue::Var(name)], rhs: Expr::Num(1.0, "1".into()), print, op: Some(op) },
                    line,
                });
            }
        }
        // assignment?
        let save = self.pos;
        if let Some(lhs) = self.try_lhs()? {
            let op = match self.peek() {
                Tok::Assign => None,
                Tok::PlusEq => Some(BinOp::Add),
                Tok::MinusEq => Some(BinOp::Sub),
                Tok::StarEq => Some(BinOp::Mul),
                Tok::SlashEq => Some(BinOp::Div),
                _ => unreachable!(),
            };
            self.advance();
            let rhs = self.expr()?;
            let print = self.end_of_statement()?;
            return Ok(Stmt { kind: StmtKind::Assign { lhs, rhs, print, op }, line });
        }
        self.pos = save;
        let e = self.expr()?;
        let print = self.end_of_statement()?;
        Ok(Stmt { kind: StmtKind::Expr { expr: e, print }, line })
    }

    /// Tries to parse the left-hand side of an assignment; if `=`/`+=`… does not follow, returns None (the caller restores the position).
    fn try_lhs(&mut self) -> PResult<Option<Vec<LValue>>> {
        let save = self.pos;
        let is_assign = |t: &Tok| matches!(t, Tok::Assign | Tok::PlusEq | Tok::MinusEq | Tok::StarEq | Tok::SlashEq);
        match self.peek().clone() {
            Tok::LBracket => {
                self.advance();
                let mut lv = Vec::new();
                loop {
                    match self.peek().clone() {
                        Tok::RBracket => {
                            self.advance();
                            break;
                        }
                        Tok::Comma => {
                            self.advance();
                        }
                        Tok::Not => {
                            self.advance();
                            lv.push(LValue::Tilde);
                        }
                        Tok::Ident(s) if !is_keyword(&s) => {
                            self.advance();
                            if *self.peek() == Tok::LParen && !self.tok().space_before {
                                match self.index_args() {
                                    Ok(args) => lv.push(LValue::Index(s, args)),
                                    Err(_) => {
                                        self.pos = save;
                                        return Ok(None);
                                    }
                                }
                            } else {
                                lv.push(LValue::Var(s));
                            }
                        }
                        _ => {
                            self.pos = save;
                            return Ok(None);
                        }
                    }
                }
                if *self.peek() == Tok::Assign {
                    Ok(Some(lv))
                } else {
                    self.pos = save;
                    Ok(None)
                }
            }
            Tok::Ident(s) if !is_keyword(&s) => {
                self.advance();
                if is_assign(self.peek()) {
                    return Ok(Some(vec![LValue::Var(s)]));
                }
                if *self.peek() == Tok::Dot {
                    if let Tok::Ident(field) = self.peek_at(1).clone() {
                        if is_assign(self.peek_at(2)) {
                            self.advance();
                            self.advance();
                            return Ok(Some(vec![LValue::Field(s, field)]));
                        }
                    }
                }
                if *self.peek() == Tok::LParen {
                    match self.index_args() {
                        Ok(args) => {
                            if is_assign(self.peek()) {
                                return Ok(Some(vec![LValue::Index(s, args)]));
                            }
                        }
                        Err(_) => {}
                    }
                }
                self.pos = save;
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn if_stmt(&mut self) -> PResult<StmtKind> {
        self.advance();
        let mut clauses = Vec::new();
        let cond = self.expr()?;
        let body = self.block(&["elseif", "else", "end", "endif"])?;
        clauses.push((cond, body));
        let mut else_body = None;
        loop {
            if self.is_kw("elseif") {
                self.advance();
                self.skip_newlines();
                let c = self.expr()?;
                let b = self.block(&["elseif", "else", "end", "endif"])?;
                clauses.push((c, b));
            } else if self.is_kw("else") {
                self.advance();
                else_body = Some(self.block(&["end", "endif"])?);
            } else {
                self.advance(); // end / endif
                break;
            }
        }
        Ok(StmtKind::If { clauses, else_body })
    }

    fn for_stmt(&mut self) -> PResult<StmtKind> {
        self.advance();
        let paren = *self.peek() == Tok::LParen;
        if paren {
            self.advance();
        }
        let var = match self.advance().tok {
            Tok::Ident(s) if !is_keyword(&s) => s,
            _ => return self.err("expected loop variable"),
        };
        self.expect(Tok::Assign, "'='")?;
        let iter = if paren {
            let saved = self.in_matrix;
            self.in_matrix = false;
            let e = self.expr()?;
            self.in_matrix = saved;
            self.expect(Tok::RParen, "')'")?;
            e
        } else {
            self.expr()?
        };
        let body = self.block(&["end", "endfor", "endparfor"])?;
        self.advance();
        Ok(StmtKind::For { var, iter, body })
    }

    fn while_stmt(&mut self) -> PResult<StmtKind> {
        self.advance();
        let cond = self.expr()?;
        let body = self.block(&["end", "endwhile"])?;
        self.advance();
        Ok(StmtKind::While { cond, body })
    }

    fn do_stmt(&mut self) -> PResult<StmtKind> {
        self.advance();
        let body = self.block(&["until"])?;
        self.advance();
        let cond = self.expr()?;
        Ok(StmtKind::DoUntil { body, cond })
    }

    fn switch_stmt(&mut self) -> PResult<StmtKind> {
        self.advance();
        let subject = self.expr()?;
        let mut cases = Vec::new();
        let mut otherwise = None;
        self.skip_separators();
        loop {
            if self.is_kw("case") {
                self.advance();
                let v = self.expr()?;
                let b = self.block(&["case", "otherwise", "end", "endswitch"])?;
                cases.push((v, b));
            } else if self.is_kw("otherwise") {
                self.advance();
                otherwise = Some(self.block(&["case", "otherwise", "end", "endswitch"])?);
            } else if self.is_kw_any(&["end", "endswitch"]) {
                self.advance();
                break;
            } else {
                return self.err("expected 'case', 'otherwise' or 'end' in switch");
            }
        }
        Ok(StmtKind::Switch { subject, cases, otherwise })
    }

    fn try_stmt(&mut self) -> PResult<StmtKind> {
        self.advance();
        if *self.peek() == Tok::Comma {
            self.advance();
        }
        let body = self.block(&["catch", "end", "end_try_catch"])?;
        let mut ident = None;
        let mut catch_body = Vec::new();
        if self.is_kw("catch") {
            let catch_line = self.tok().line;
            self.advance();
            if let Tok::Ident(s) = self.peek().clone() {
                let t = self.tok();
                if t.line == catch_line && !is_keyword(&s) && matches!(self.peek_at(1), Tok::Newline | Tok::Semi | Tok::Eof) {
                    self.advance();
                    ident = Some(s);
                }
            }
            catch_body = self.block(&["end", "end_try_catch"])?;
        }
        self.advance();
        Ok(StmtKind::Try { body, ident, catch_body })
    }

    // ---------------- expressions ----------------

    pub fn expr(&mut self) -> PResult<Expr> {
        self.oror()
    }

    fn oror(&mut self) -> PResult<Expr> {
        let mut l = self.andand()?;
        while *self.peek() == Tok::OrOr {
            self.advance();
            let r = self.andand()?;
            l = Expr::Binary(BinOp::OrOr, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn andand(&mut self) -> PResult<Expr> {
        let mut l = self.or()?;
        while *self.peek() == Tok::AndAnd {
            self.advance();
            let r = self.or()?;
            l = Expr::Binary(BinOp::AndAnd, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn or(&mut self) -> PResult<Expr> {
        let mut l = self.and()?;
        while *self.peek() == Tok::Or {
            self.advance();
            let r = self.and()?;
            l = Expr::Binary(BinOp::Or, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn and(&mut self) -> PResult<Expr> {
        let mut l = self.cmp()?;
        while *self.peek() == Tok::And {
            self.advance();
            let r = self.cmp()?;
            l = Expr::Binary(BinOp::And, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn cmp(&mut self) -> PResult<Expr> {
        let mut l = self.range()?;
        loop {
            let op = match self.peek() {
                Tok::EqEq => BinOp::Eq,
                Tok::Ne => BinOp::Ne,
                Tok::Lt => BinOp::Lt,
                Tok::Le => BinOp::Le,
                Tok::Gt => BinOp::Gt,
                Tok::Ge => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let r = self.range()?;
            l = Expr::Binary(op, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn range(&mut self) -> PResult<Expr> {
        let a = self.additive()?;
        if *self.peek() == Tok::Colon {
            self.advance();
            let b = self.additive()?;
            if *self.peek() == Tok::Colon {
                self.advance();
                let c = self.additive()?;
                return Ok(Expr::Range(Box::new(a), Some(Box::new(b)), Box::new(c)));
            }
            return Ok(Expr::Range(Box::new(a), None, Box::new(b)));
        }
        Ok(a)
    }
    /// In `[...]`, «+»/«-» with a space before and no space after starts a new element.
    fn matrix_sign_boundary(&self) -> bool {
        self.in_matrix && self.tok().space_before && !self.tok_at(1).space_before
    }
    fn additive(&mut self) -> PResult<Expr> {
        let mut l = self.mult()?;
        loop {
            let op = match self.peek() {
                Tok::Plus => BinOp::Add,
                Tok::Minus => BinOp::Sub,
                _ => break,
            };
            if self.matrix_sign_boundary() {
                break;
            }
            self.advance();
            let r = self.mult()?;
            l = Expr::Binary(op, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn mult(&mut self) -> PResult<Expr> {
        let mut l = self.unary()?;
        loop {
            let op = match self.peek() {
                Tok::Star => BinOp::Mul,
                Tok::Slash => BinOp::Div,
                Tok::Backslash => BinOp::LDiv,
                Tok::DotStar => BinOp::EMul,
                Tok::DotSlash => BinOp::EDiv,
                Tok::DotBackslash => BinOp::ELDiv,
                _ => break,
            };
            self.advance();
            let r = self.unary()?;
            l = Expr::Binary(op, Box::new(l), Box::new(r));
        }
        Ok(l)
    }
    fn unary(&mut self) -> PResult<Expr> {
        let op = match self.peek() {
            Tok::Minus => Some(UnOp::Neg),
            Tok::Plus => Some(UnOp::Plus),
            Tok::Not => Some(UnOp::Not),
            _ => None,
        };
        if let Some(op) = op {
            self.advance();
            let e = self.unary()?;
            return Ok(Expr::Unary(op, Box::new(e)));
        }
        self.power()
    }
    fn power(&mut self) -> PResult<Expr> {
        let mut base = self.postfix()?;
        loop {
            let op = match self.peek() {
                Tok::Caret => BinOp::Pow,
                Tok::DotCaret => BinOp::EPow,
                _ => break,
            };
            self.advance();
            let e = self.power_operand()?;
            base = Expr::Binary(op, Box::new(base), Box::new(e));
        }
        Ok(base)
    }
    fn power_operand(&mut self) -> PResult<Expr> {
        let op = match self.peek() {
            Tok::Minus => Some(UnOp::Neg),
            Tok::Plus => Some(UnOp::Plus),
            Tok::Not => Some(UnOp::Not),
            _ => None,
        };
        if let Some(op) = op {
            self.advance();
            let e = self.power_operand()?;
            return Ok(Expr::Unary(op, Box::new(e)));
        }
        self.postfix()
    }
    fn postfix(&mut self) -> PResult<Expr> {
        let mut e = self.primary()?;
        loop {
            match self.peek() {
                Tok::LParen => {
                    if self.in_matrix && self.tok().space_before {
                        break;
                    }
                    let args = self.index_args()?;
                    e = Expr::Index(Box::new(e), args);
                }
                Tok::Quote => {
                    self.advance();
                    e = Expr::Postfix(PostOp::CTranspose, Box::new(e));
                }
                Tok::DotQuote => {
                    self.advance();
                    e = Expr::Postfix(PostOp::Transpose, Box::new(e));
                }
                Tok::Dot => {
                    if self.in_matrix && self.tok().space_before {
                        break;
                    }
                    if let Tok::Ident(name) = self.peek_at(1).clone() {
                        self.advance();
                        self.advance();
                        e = Expr::Field(Box::new(e), name);
                    } else {
                        return self.err("unexpected '.'");
                    }
                }
                Tok::LBrace if !(self.in_matrix && self.tok().space_before) => {
                    return self.err("cell arrays are not supported in mlab v1");
                }
                _ => break,
            }
        }
        Ok(e)
    }
    fn index_args(&mut self) -> PResult<Vec<Expr>> {
        self.advance(); // (
        let saved = self.in_matrix;
        self.in_matrix = false;
        self.index_depth += 1;
        let mut args = Vec::new();
        if *self.peek() == Tok::RParen {
            self.advance();
        } else {
            loop {
                if *self.peek() == Tok::Colon && matches!(self.peek_at(1), Tok::Comma | Tok::RParen) {
                    self.advance();
                    args.push(Expr::Colon);
                } else {
                    args.push(self.expr()?);
                }
                match self.peek() {
                    Tok::Comma => {
                        self.advance();
                    }
                    Tok::RParen => {
                        self.advance();
                        break;
                    }
                    _ => {
                        self.index_depth -= 1;
                        self.in_matrix = saved;
                        return self.err("expected ')'");
                    }
                }
            }
        }
        self.index_depth -= 1;
        self.in_matrix = saved;
        Ok(args)
    }
    fn primary(&mut self) -> PResult<Expr> {
        let t = self.tok().clone();
        match t.tok {
            Tok::Num(v, s) => {
                self.advance();
                Ok(Expr::Num(v, s))
            }
            Tok::Imag(v, s) => {
                self.advance();
                Ok(Expr::Imag(v, s))
            }
            Tok::Str(s, dq) => {
                self.advance();
                Ok(Expr::Str(s, dq))
            }
            Tok::Ident(s) => {
                if s == "end" && self.index_depth > 0 {
                    self.advance();
                    return Ok(Expr::End);
                }
                if is_keyword(&s) {
                    return self.err(format!("unexpected '{s}'"));
                }
                self.advance();
                Ok(Expr::Ident(s))
            }
            Tok::LParen => {
                self.advance();
                let saved = self.in_matrix;
                self.in_matrix = false;
                let e = self.expr()?;
                self.in_matrix = saved;
                self.expect(Tok::RParen, "')'")?;
                Ok(Expr::Paren(Box::new(e)))
            }
            Tok::LBracket => {
                let rows = self.matrix(Tok::RBracket)?;
                Ok(Expr::Matrix(rows))
            }
            Tok::LBrace => {
                let rows = self.matrix(Tok::RBrace)?;
                Ok(Expr::CellList(rows.into_iter().flatten().collect()))
            }
            Tok::At => {
                self.advance();
                match self.peek().clone() {
                    Tok::LParen => {
                        self.advance();
                        let mut params = Vec::new();
                        loop {
                            match self.advance().tok {
                                Tok::RParen => break,
                                Tok::Comma => {}
                                Tok::Ident(s) => params.push(s),
                                Tok::Not => params.push("~".into()),
                                _ => return self.err("expected parameter name"),
                            }
                        }
                        let saved_m = self.in_matrix;
                        let saved_d = self.index_depth;
                        self.in_matrix = false;
                        self.index_depth = 0;
                        let body = self.expr()?;
                        self.in_matrix = saved_m;
                        self.index_depth = saved_d;
                        Ok(Expr::AnonFn(params, Rc::new(body)))
                    }
                    Tok::Ident(s) => {
                        self.advance();
                        Ok(Expr::FuncHandle(s))
                    }
                    _ => self.err("expected function name or parameter list after '@'"),
                }
            }
            Tok::Colon if self.index_depth > 0 => {
                self.advance();
                Ok(Expr::Colon)
            }
            other => self.err(format!("unexpected '{}'", other.describe())),
        }
    }
    fn matrix(&mut self, close: Tok) -> PResult<Vec<Vec<Expr>>> {
        self.advance(); // [ or {
        let saved = self.in_matrix;
        self.in_matrix = true;
        let mut rows: Vec<Vec<Expr>> = Vec::new();
        let mut cur: Vec<Expr> = Vec::new();
        loop {
            let t = self.peek().clone();
            if t == close {
                self.advance();
                break;
            }
            match t {
                Tok::Semi | Tok::Newline => {
                    self.advance();
                    if !cur.is_empty() {
                        rows.push(std::mem::take(&mut cur));
                    }
                }
                Tok::Comma => {
                    self.advance();
                }
                Tok::Eof => {
                    self.in_matrix = saved;
                    return self.err(format!("expected '{}'", close.text()));
                }
                _ => {
                    let e = self.expr()?;
                    cur.push(e);
                    let nt = self.tok();
                    let ok = matches!(nt.tok, Tok::Comma | Tok::Semi | Tok::Newline) || nt.tok == close || nt.space_before;
                    if !ok {
                        let d = nt.tok.describe();
                        self.in_matrix = saved;
                        return self.err(format!("unexpected '{d}' in matrix"));
                    }
                }
            }
        }
        if !cur.is_empty() {
            rows.push(cur);
        }
        self.in_matrix = saved;
        Ok(rows)
    }
}
