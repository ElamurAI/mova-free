//! Register VM v1 for mlab (26.09): a compiler from the parse tree to register bytecode, and an executor.
//!
//! The design follows Lua 5 (Ierusalimschy, de Figueiredo, Celes, «The Implementation of Lua 5.0», 2005): registers per
//! function frame, constants in a pool, the dispatcher is a `match` in a loop; superinstructions for hot pairs. A register value is
//! a [`Reg`]: real and logical scalars without memory allocation, everything else — `Rc<Value>` with copy-on-write
//! (`B = A` does not copy, `B(1) = 0` copies only when someone else still holds the matrix).
//!
//! **Fallback is transparent.** Whatever does not compile is executed by the existing tree-walker:
//! - a statement (`switch`, `try`, commands, struct fields, cell lists, calls to `exist`/`clear`/`feval`…) — `Fallback`;
//! - an expression in which a variable's type is known only at run time (indexing a table or a string array) — `DynEval`.
//!
//! Variables move registers → `HashMap` frame → registers by move (no copies), and only those mentioned in
//! the statement. Semantics and error texts come from the same `ops`, `index` and builtin registry, so the output is the same;
//! the equivalence gate is `tests/vm.rs` (both frozen suites, byte for byte).

use crate::ast::{BinOp, Expr, LValue, PostOp, Program, Stmt, StmtKind, UnOp};
use crate::builtins::{BuiltinFn, Registry};
use crate::index::{self, IdxArg};
use crate::interp::{EndCtx, Flow, Interp, MError};
use crate::ops::{self, RangeSpec};
use crate::value::{Class, Func, Mat, Value};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Register number.
pub type R = u16;
const NO: R = u16::MAX;
const NOJ: u32 = u32::MAX;

/// Builtins that see frame variables by name (or can call such a builtin via a string/handle): a statement with
/// them is executed by the tree with full synchronization of registers and frame.
const FRAME_SENSITIVE: &[&str] = &["exist", "clear", "feval", "arrayfun", "cellfun", "eval", "evalin", "assignin", "who", "whos", "inputname"];

fn frame_sensitive(name: &str) -> bool {
    FRAME_SENSITIVE.contains(&name)
}

// ---------------------------------------------------------------- values

/// Register: `U` — variable undefined (the name then means a function), `F` — real double scalar, `L` — logical
/// scalar, `V` — everything else (matrices, complex, char, handles, tables).
#[derive(Clone, Debug, Default)]
pub enum Reg {
    #[default]
    U,
    F(f64),
    L(bool),
    V(Rc<Value>),
}

impl Reg {
    #[inline]
    pub fn from_value(v: Value) -> Reg {
        if let Value::Mat(m) = &v {
            if m.rows == 1 && m.cols == 1 && m.im.is_none() {
                match m.class {
                    Class::Double => return Reg::F(m.re[0]),
                    Class::Logical => {
                        let b = m.re[0].to_bits();
                        if b == 0 {
                            return Reg::L(false);
                        }
                        if b == 1f64.to_bits() {
                            return Reg::L(true);
                        }
                    }
                    Class::Char => {}
                }
            }
        }
        Reg::V(Rc::new(v))
    }
    fn from_mat(m: Mat) -> Reg {
        Reg::from_value(Value::Mat(m))
    }
    pub fn to_value(&self) -> Option<Value> {
        match self {
            Reg::U => None,
            Reg::F(x) => Some(Value::num(*x)),
            Reg::L(b) => Some(Value::boolean(*b)),
            Reg::V(v) => Some((**v).clone()),
        }
    }
    pub fn into_value(self) -> Option<Value> {
        match self {
            Reg::U => None,
            Reg::F(x) => Some(Value::num(x)),
            Reg::L(b) => Some(Value::boolean(b)),
            Reg::V(v) => Some(unwrap_rc(v)),
        }
    }
    #[inline]
    fn real(&self) -> Option<f64> {
        match *self {
            Reg::F(x) => Some(x),
            Reg::L(b) => Some(if b { 1.0 } else { 0.0 }),
            _ => None,
        }
    }
    #[inline]
    fn is_mat(&self) -> bool {
        match self {
            Reg::F(_) | Reg::L(_) => true,
            Reg::V(v) => matches!(**v, Value::Mat(_)),
            Reg::U => false,
        }
    }
    #[inline]
    fn is_def(&self) -> bool {
        !matches!(self, Reg::U)
    }
    fn class_name(&self) -> &'static str {
        match self {
            Reg::U => "undefined",
            Reg::F(_) => "double",
            Reg::L(_) => "logical",
            Reg::V(v) => v.class_name(),
        }
    }
}

fn unwrap_rc(v: Rc<Value>) -> Value {
    Rc::try_unwrap(v).unwrap_or_else(|v| (*v).clone())
}

fn into_mat(r: Reg) -> Option<Mat> {
    match r {
        Reg::F(x) => Some(Mat::scalar(x)),
        Reg::L(b) => Some(Mat::boolean(b)),
        Reg::V(v) => match unwrap_rc(v) {
            Value::Mat(m) => Some(m),
            _ => None,
        },
        Reg::U => None,
    }
}

fn mat_cow(r: &Reg) -> Option<Cow<'_, Mat>> {
    match r {
        Reg::F(x) => Some(Cow::Owned(Mat::scalar(*x))),
        Reg::L(b) => Some(Cow::Owned(Mat::boolean(*b))),
        Reg::V(v) => match &**v {
            Value::Mat(m) => Some(Cow::Borrowed(m)),
            _ => None,
        },
        Reg::U => None,
    }
}

/// Slow-path operand: a borrowed register value or a temporarily built one (scalar, function call).
enum Opnd<'a> {
    B(&'a Value),
    O(Value),
}

impl std::ops::Deref for Opnd<'_> {
    type Target = Value;
    fn deref(&self) -> &Value {
        match self {
            Opnd::B(v) => v,
            Opnd::O(v) => v,
        }
    }
}

// ---------------------------------------------------------------- bytecode

/// Instruction. Registers — `R`, jumps — instruction number, the rest — indices into the prototype's tables.
#[derive(Clone, Copy, Debug)]
pub enum Op {
    LoadF(R, f64),
    LoadK(R, u32),
    /// dst ← variable; undefined — call the function with the same name (as in the tree).
    GetVar(R, R),
    Move(R, R),
    EndVal(R),
    Bin(BinOp, R, R, R),
    BinK(BinOp, R, R, f64),
    /// dst ← constant op b (constant on the left; operand order as in the tree).
    KBin(BinOp, R, f64, R),
    /// Superinstruction: dst ← a + constant.
    AddK(R, R, f64),
    /// Gate negative control (`--fault vm:add`): scalar addition with a +1 error.
    AddBug(R, R, R),
    Un(UnOp, R, R),
    Transp(bool, R, R),
    /// dst, a, s (NO — no step), b.
    Range(R, R, R, R),
    Concat(R, u32),
    /// dst ← logical scalar for `&&` (true) or `||` (false), with the tree's checks.
    ScBool(R, R, bool),
    Jmp(u32),
    JmpFalse(R, u32),
    /// Jump if the register is a logical scalar with the given value (short-circuit).
    JmpIfL(R, bool, u32),
    /// Superinstruction «compare and jump»: jump if NOT (a op b).
    JmpCmp(BinOp, R, R, u32),
    JmpCmpK(BinOp, R, f64, u32),
    /// Loop over a range without materialization: slot, variable, a, s, b, exit.
    ForRange(u16, R, R, R, R, u32),
    /// Loop over the columns of a value: slot, variable, value, exit.
    ForGen(u16, R, R, u32),
    /// Superinstruction «loop step and bound check»: slot, variable, start of the body.
    ForNext(u16, R, u32),
    /// Variable is a matrix, handle or undefined → continue; otherwise (table, strings) → jump to `DynEval`.
    TypeSwitch(R, u32),
    TypeIsMat(R, u32),
    PushEnd(R, u8, u8),
    PopEnd,
    /// An index argument of a matrix variable must be numeric or logical.
    CheckIdx(R, R),
    CheckArg(R),
    /// dst, variable, call site: matrix → read, handle → call, undefined → function by name.
    Index(R, R, u32),
    Call(R, u32),
    CallMulti(u32),
    TakeMulti(R, u16),
    DynEval(R, u32),
    AsgCheck(R, R, u32),
    ReadCheck(R, u32),
    IndexAsg(R, R, u32),
    Compound(BinOp, R, R, u32),
    GetField(R, R, u32),
    MakeClosure(R, u32),
    Display(R, u32),
    /// Statement `x`: variable defined — display it; otherwise call function `x` and put the result into `ans`.
    ShowOrCall(R, u32, bool),
    SetAns(R, bool),
    Fallback(u32),
    Ret,
}

#[derive(Clone, Copy, Debug)]
pub enum Target {
    User(usize),
    Builtin(BuiltinFn),
    Missing,
}

#[derive(Debug)]
pub struct Site {
    pub name: String,
    pub args: Vec<R>,
    pub nargout: usize,
    /// Whether the value is needed (expression) — otherwise an empty result is allowed (statement).
    pub need: bool,
    /// A call `name(args)` in the tree records the names of variable arguments (`table(x, y)`).
    pub set_names: bool,
    pub arg_names: Vec<Option<String>>,
    pub target: Target,
    /// Arguments — directly the registers of variables and constants (do not take, undefined → function call);
    /// then the variable type is checked by the instruction itself (`dyn_idx` — fallback for tables and strings).
    pub direct: bool,
    pub dyn_idx: u32,
}

#[derive(Debug)]
pub struct Closure {
    pub params: Vec<String>,
    pub body: Rc<Expr>,
    pub caps: Vec<(String, R)>,
}

#[derive(Debug)]
pub struct Dyn {
    pub expr: Expr,
    pub sync: Vec<(String, R)>,
    pub nargout: usize,
    pub need: bool,
}

#[derive(Debug)]
pub struct Fb {
    pub stmt: Stmt,
    pub sync: Vec<(String, R)>,
    pub brk: u32,
    pub cont: u32,
}

/// Compiled unit: a script or a function.
#[derive(Debug, Default)]
pub struct Proto {
    pub name: String,
    pub code: Vec<Op>,
    pub nregs: usize,
    pub nvars: usize,
    pub nloops: usize,
    pub consts: Vec<Rc<Value>>,
    pub strs: Vec<String>,
    pub reg_names: Vec<String>,
    pub vars: Vec<(String, R)>,
    pub params: Vec<R>,
    pub outputs: Vec<R>,
    pub output_names: Vec<String>,
    pub nargin_reg: R,
    pub nargout_reg: R,
    pub ans_reg: R,
    pub sites: Vec<Site>,
    pub concats: Vec<Vec<Vec<R>>>,
    pub closures: Vec<Closure>,
    pub dyns: Vec<Dyn>,
    pub fallbacks: Vec<Fb>,
    /// Instructions compiled / handed over to the tree.
    pub native: usize,
    pub fallback: usize,
}

#[derive(Debug, Default)]
pub struct VmProgram {
    pub protos: Vec<Proto>,
    pub index: HashMap<String, usize>,
}

#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub native: usize,
    pub fallback: usize,
    pub fallback_runs: u64,
    pub dyn_runs: u64,
}

#[derive(Clone, Debug, Default)]
pub enum LoopState {
    #[default]
    Idle,
    Range { spec: RangeSpec, k: usize },
    Cols { m: Rc<Value>, j: usize, n: usize },
    Single,
}

/// Registers and loop states of one call (from a pool — no memory allocation per call).
#[derive(Debug, Default)]
pub struct FrameBuf {
    regs: Vec<Reg>,
    loops: Vec<LoopState>,
}

// ---------------------------------------------------------------- compiler

#[derive(Default)]
struct LoopCx {
    brk: Vec<usize>,
    cont: Vec<usize>,
    fbs: Vec<usize>,
}

struct Cx<'a> {
    user: &'a HashMap<String, usize>,
    registry: &'a Registry,
    fault_add: bool,
    p: Proto,
    vars: HashMap<String, R>,
    top: usize,
    loops: Vec<LoopCx>,
}

fn assigned(stmts: &[Stmt], add: &mut dyn FnMut(&str)) {
    for s in stmts {
        match &s.kind {
            StmtKind::Assign { lhs, .. } => {
                for lv in lhs {
                    match lv {
                        LValue::Var(n) | LValue::Index(n, _) | LValue::Field(n, _) => add(n),
                        LValue::Tilde => {}
                    }
                }
            }
            StmtKind::For { var, body, .. } => {
                add(var);
                assigned(body, add);
            }
            StmtKind::If { clauses, else_body } => {
                for (_, b) in clauses {
                    assigned(b, add);
                }
                if let Some(b) = else_body {
                    assigned(b, add);
                }
            }
            StmtKind::While { body, .. } | StmtKind::DoUntil { body, .. } => assigned(body, add),
            StmtKind::Switch { cases, otherwise, .. } => {
                for (_, b) in cases {
                    assigned(b, add);
                }
                if let Some(b) = otherwise {
                    assigned(b, add);
                }
            }
            StmtKind::Try { body, ident, catch_body } => {
                assigned(body, add);
                if let Some(i) = ident {
                    add(i);
                }
                assigned(catch_body, add);
            }
            _ => {}
        }
    }
}

fn has_end(e: &Expr) -> bool {
    match e {
        Expr::End => true,
        Expr::Paren(x) | Expr::Unary(_, x) | Expr::Postfix(_, x) | Expr::Field(x, _) => has_end(x),
        Expr::Binary(_, a, b) => has_end(a) || has_end(b),
        Expr::Range(a, s, b) => has_end(a) || s.as_deref().is_some_and(has_end) || has_end(b),
        Expr::Index(b, args) => has_end(b) || args.iter().any(has_end),
        Expr::Matrix(rows) => rows.iter().flatten().any(has_end),
        Expr::CellList(items) => items.iter().any(has_end),
        _ => false,
    }
}

/// Names the tree may touch while executing an expression (variables and calls); `fs` — there is a call that sees the frame.
fn names_expr(e: &Expr, out: &mut HashSet<String>, fs: &mut bool) {
    match e {
        Expr::Ident(n) => {
            if frame_sensitive(n) {
                *fs = true;
            }
            out.insert(n.clone());
        }
        Expr::FuncHandle(n) => {
            if frame_sensitive(n) {
                *fs = true;
            }
        }
        Expr::Paren(x) | Expr::Unary(_, x) | Expr::Postfix(_, x) | Expr::Field(x, _) => names_expr(x, out, fs),
        Expr::Binary(_, a, b) => {
            names_expr(a, out, fs);
            names_expr(b, out, fs);
        }
        Expr::Range(a, s, b) => {
            names_expr(a, out, fs);
            if let Some(s) = s {
                names_expr(s, out, fs);
            }
            names_expr(b, out, fs);
        }
        Expr::Index(b, args) => {
            names_expr(b, out, fs);
            for a in args {
                names_expr(a, out, fs);
            }
        }
        Expr::Matrix(rows) => {
            for x in rows.iter().flatten() {
                names_expr(x, out, fs);
            }
        }
        Expr::CellList(items) => {
            for x in items {
                names_expr(x, out, fs);
            }
        }
        Expr::AnonFn(_, body) => {
            let mut inner = HashSet::new();
            crate::interp::collect_idents(body, &mut inner);
            for n in inner {
                if frame_sensitive(&n) {
                    *fs = true;
                }
                out.insert(n);
            }
        }
        _ => {}
    }
}

fn names_stmts(stmts: &[Stmt], out: &mut HashSet<String>, fs: &mut bool) {
    for s in stmts {
        names_stmt(s, out, fs);
    }
}

fn names_stmt(s: &Stmt, out: &mut HashSet<String>, fs: &mut bool) {
    match &s.kind {
        StmtKind::Expr { expr, .. } => {
            out.insert("ans".into());
            names_expr(expr, out, fs);
        }
        StmtKind::Assign { lhs, rhs, .. } => {
            for lv in lhs {
                match lv {
                    LValue::Var(n) | LValue::Field(n, _) => {
                        out.insert(n.clone());
                    }
                    LValue::Index(n, args) => {
                        out.insert(n.clone());
                        for a in args {
                            names_expr(a, out, fs);
                        }
                    }
                    LValue::Tilde => {}
                }
            }
            names_expr(rhs, out, fs);
        }
        StmtKind::If { clauses, else_body } => {
            for (c, b) in clauses {
                names_expr(c, out, fs);
                names_stmts(b, out, fs);
            }
            if let Some(b) = else_body {
                names_stmts(b, out, fs);
            }
        }
        StmtKind::For { var, iter, body } => {
            out.insert(var.clone());
            names_expr(iter, out, fs);
            names_stmts(body, out, fs);
        }
        StmtKind::While { cond, body } | StmtKind::DoUntil { body, cond } => {
            names_expr(cond, out, fs);
            names_stmts(body, out, fs);
        }
        StmtKind::Switch { subject, cases, otherwise } => {
            names_expr(subject, out, fs);
            for (c, b) in cases {
                names_expr(c, out, fs);
                names_stmts(b, out, fs);
            }
            if let Some(b) = otherwise {
                names_stmts(b, out, fs);
            }
        }
        StmtKind::Try { body, ident, catch_body } => {
            names_stmts(body, out, fs);
            if let Some(i) = ident {
                out.insert(i.clone());
            }
            names_stmts(catch_body, out, fs);
        }
        StmtKind::Command { .. } => {
            *fs = true;
        }
        StmtKind::Break | StmtKind::Continue | StmtKind::Return => {}
    }
}

fn arg_names(args: &[Expr]) -> Vec<Option<String>> {
    // an empty list and a list of None are the same for `table` («Var1», «Var2»…), so no memory allocation
    if !args.iter().any(|a| matches!(a, Expr::Ident(_))) {
        return Vec::new();
    }
    args.iter().map(|a| if let Expr::Ident(n) = a { Some(n.clone()) } else { None }).collect()
}

impl<'a> Cx<'a> {
    fn emit(&mut self, op: Op) -> usize {
        self.p.code.push(op);
        self.p.code.len() - 1
    }
    fn here(&self) -> u32 {
        self.p.code.len() as u32
    }
    fn patch(&mut self, at: usize, t: u32) {
        match &mut self.p.code[at] {
            Op::Jmp(x)
            | Op::JmpFalse(_, x)
            | Op::JmpIfL(_, _, x)
            | Op::JmpCmp(_, _, _, x)
            | Op::JmpCmpK(_, _, _, x)
            | Op::ForRange(_, _, _, _, _, x)
            | Op::ForGen(_, _, _, x)
            | Op::ForNext(_, _, x)
            | Op::TypeSwitch(_, x)
            | Op::TypeIsMat(_, x) => *x = t,
            other => unreachable!("patch {other:?}"),
        }
    }
    fn patch_here(&mut self, at: usize) {
        let h = self.here();
        self.patch(at, h);
    }
    fn tmp(&mut self) -> R {
        let r = self.top;
        self.top += 1;
        if self.top > self.p.nregs {
            self.p.nregs = self.top;
        }
        assert!(self.top < NO as usize, "mlab vm: too many registers");
        r as R
    }
    fn var(&self, n: &str) -> Option<R> {
        self.vars.get(n).copied()
    }
    fn str(&mut self, s: &str) -> u32 {
        if let Some(k) = self.p.strs.iter().position(|x| x == s) {
            return k as u32;
        }
        self.p.strs.push(s.to_string());
        (self.p.strs.len() - 1) as u32
    }
    fn load_value(&mut self, d: R, v: Value) {
        match Reg::from_value(v) {
            Reg::F(x) => {
                self.emit(Op::LoadF(d, x));
            }
            Reg::V(v) => {
                self.p.consts.push(v);
                let k = (self.p.consts.len() - 1) as u32;
                self.emit(Op::LoadK(d, k));
            }
            other => {
                // logical constant (true/false as numbers do not appear in the tree, but just in case)
                self.p.consts.push(Rc::new(other.into_value().unwrap()));
                let k = (self.p.consts.len() - 1) as u32;
                self.emit(Op::LoadK(d, k));
            }
        }
    }
    fn site(&mut self, name: &str, args: Vec<R>, nargout: usize, need: bool, set_names: bool, names: Vec<Option<String>>) -> u32 {
        let target = if let Some(&i) = self.user.get(name) {
            Target::User(i)
        } else if let Some(f) = self.registry.get(name) {
            Target::Builtin(f)
        } else {
            Target::Missing
        };
        self.p.sites.push(Site { name: name.to_string(), args, nargout, need, set_names, arg_names: names, target, direct: false, dyn_idx: NOJ });
        (self.p.sites.len() - 1) as u32
    }
    fn sync_list(&self, names: &HashSet<String>, fs: bool) -> Vec<(String, R)> {
        self.p.vars.iter().filter(|(n, _)| fs || names.contains(n)).cloned().collect()
    }

    // ---- what the compiler supports

    fn ok_expr(&self, e: &Expr) -> bool {
        match e {
            Expr::Num(..) | Expr::Imag(..) | Expr::Str(..) | Expr::Colon | Expr::End | Expr::AnonFn(..) => true,
            Expr::FuncHandle(n) | Expr::Ident(n) => !frame_sensitive(n),
            Expr::Paren(x) | Expr::Unary(_, x) | Expr::Postfix(_, x) | Expr::Field(x, _) => self.ok_expr(x),
            Expr::Binary(_, a, b) => self.ok_expr(a) && self.ok_expr(b),
            Expr::Range(a, s, b) => self.ok_expr(a) && s.as_deref().is_none_or(|s| self.ok_expr(s)) && self.ok_expr(b),
            Expr::Index(b, args) => matches!(&**b, Expr::Ident(n) if !frame_sensitive(n)) && args.iter().all(|a| self.ok_expr(a)),
            Expr::Matrix(rows) => rows.iter().flatten().all(|x| self.ok_expr(x)),
            Expr::CellList(_) => false,
        }
    }

    fn stmt_ok(&self, s: &Stmt) -> bool {
        match &s.kind {
            StmtKind::Expr { expr, .. } => self.ok_expr(expr),
            StmtKind::Assign { lhs, rhs, op, .. } => {
                if lhs.len() == 1 {
                    let lhs_ok = match &lhs[0] {
                        LValue::Var(n) => !frame_sensitive(n),
                        LValue::Index(n, args) => !frame_sensitive(n) && args.iter().all(|a| self.ok_expr(a)),
                        LValue::Tilde => op.is_none(),
                        LValue::Field(..) => false,
                    };
                    return lhs_ok && self.ok_expr(rhs);
                }
                if op.is_some() || !lhs.iter().all(|lv| matches!(lv, LValue::Var(_) | LValue::Tilde)) {
                    return false;
                }
                match rhs {
                    Expr::Ident(n) => self.var(n).is_none() && !frame_sensitive(n),
                    Expr::Index(b, args) => {
                        matches!(&**b, Expr::Ident(n) if self.var(n).is_none() && !frame_sensitive(n))
                            && args.iter().all(|a| self.ok_expr(a))
                    }
                    _ => false,
                }
            }
            StmtKind::If { clauses, .. } => clauses.iter().all(|(c, _)| self.ok_expr(c)),
            StmtKind::For { iter, .. } => self.ok_expr(iter),
            StmtKind::While { cond, .. } | StmtKind::DoUntil { cond, .. } => self.ok_expr(cond),
            StmtKind::Break | StmtKind::Continue | StmtKind::Return => true,
            StmtKind::Switch { .. } | StmtKind::Try { .. } | StmtKind::Command { .. } => false,
        }
    }

    // ---- statements

    fn block(&mut self, b: &[Stmt]) {
        for s in b {
            self.stmt(s);
        }
    }

    fn stmt(&mut self, s: &Stmt) {
        let top = self.top;
        if self.stmt_ok(s) {
            self.p.native += 1;
            self.stmt_native(s);
        } else {
            self.p.fallback += 1;
            self.fallback(s);
        }
        self.top = top;
    }

    fn fallback(&mut self, s: &Stmt) {
        let mut names = HashSet::new();
        let mut fs = false;
        names_stmt(s, &mut names, &mut fs);
        let sync = self.sync_list(&names, fs);
        let idx = self.p.fallbacks.len();
        self.p.fallbacks.push(Fb { stmt: s.clone(), sync, brk: NOJ, cont: NOJ });
        if let Some(l) = self.loops.last_mut() {
            l.fbs.push(idx);
        }
        self.emit(Op::Fallback(idx as u32));
    }

    fn close_loop(&mut self, exit: u32, cont: u32) {
        let l = self.loops.pop().unwrap();
        for j in l.brk {
            self.patch(j, exit);
        }
        for j in l.cont {
            self.patch(j, cont);
        }
        for f in l.fbs {
            self.p.fallbacks[f].brk = exit;
            self.p.fallbacks[f].cont = cont;
        }
    }

    fn stmt_native(&mut self, s: &Stmt) {
        match &s.kind {
            StmtKind::Expr { expr, print } => self.expr_stmt(expr, *print),
            StmtKind::Assign { lhs, rhs, print, op } => {
                if lhs.len() == 1 {
                    self.assign1(&lhs[0], rhs, *print, *op);
                } else {
                    self.assign_multi(lhs, rhs, *print);
                }
            }
            StmtKind::If { clauses, else_body } => {
                let mut ends = Vec::new();
                for (c, b) in clauses {
                    let jf = self.cond_false(c);
                    self.block(b);
                    ends.push(self.emit(Op::Jmp(NOJ)));
                    self.patch_here(jf);
                }
                if let Some(b) = else_body {
                    self.block(b);
                }
                for e in ends {
                    self.patch_here(e);
                }
            }
            StmtKind::For { var, iter, body } => {
                let v = self.var(var).unwrap();
                let slot = self.p.nloops as u16;
                self.p.nloops += 1;
                let top = self.top;
                let mut it = iter;
                while let Expr::Paren(x) = it {
                    it = x;
                }
                let prep = if let Expr::Range(a, st, b) = it {
                    let ta = self.tmp();
                    self.expr(a, ta);
                    let ts = match st {
                        Some(st) => {
                            let t = self.tmp();
                            self.expr(st, t);
                            t
                        }
                        None => NO,
                    };
                    let tb = self.tmp();
                    self.expr(b, tb);
                    self.emit(Op::ForRange(slot, v, ta, ts, tb, NOJ))
                } else {
                    let t = self.tmp();
                    self.expr(iter, t);
                    self.emit(Op::ForGen(slot, v, t, NOJ))
                };
                self.top = top;
                let body_start = self.here();
                self.loops.push(LoopCx::default());
                self.block(body);
                let cont = self.here();
                self.emit(Op::ForNext(slot, v, body_start));
                let exit = self.here();
                self.patch(prep, exit);
                self.close_loop(exit, cont);
            }
            StmtKind::While { cond, body } => {
                let start = self.here();
                let jf = self.cond_false(cond);
                self.loops.push(LoopCx::default());
                self.block(body);
                self.emit(Op::Jmp(start));
                let exit = self.here();
                self.patch(jf, exit);
                self.close_loop(exit, start);
            }
            StmtKind::DoUntil { body, cond } => {
                let start = self.here();
                self.loops.push(LoopCx::default());
                self.block(body);
                let cont = self.here();
                let jf = self.cond_false(cond);
                self.patch(jf, start);
                let exit = self.here();
                self.close_loop(exit, cont);
            }
            StmtKind::Break | StmtKind::Continue => {
                if self.loops.is_empty() {
                    // outside a loop the tree ends the script or function
                    self.emit(Op::Ret);
                } else {
                    let j = self.emit(Op::Jmp(NOJ));
                    let l = self.loops.last_mut().unwrap();
                    if matches!(s.kind, StmtKind::Break) {
                        l.brk.push(j);
                    } else {
                        l.cont.push(j);
                    }
                }
            }
            StmtKind::Return => {
                self.emit(Op::Ret);
            }
            StmtKind::Switch { .. } | StmtKind::Try { .. } | StmtKind::Command { .. } => unreachable!(),
        }
    }

    fn expr_stmt(&mut self, e: &Expr, print: bool) {
        match e {
            Expr::Ident(n) => {
                let s = self.site(n, vec![], 0, false, false, vec![]);
                match self.var(n) {
                    Some(v) => {
                        self.emit(Op::ShowOrCall(v, s, print));
                    }
                    None => {
                        let t = self.tmp();
                        self.emit(Op::Call(t, s));
                        self.emit(Op::SetAns(t, print));
                    }
                }
            }
            Expr::Index(b, args) if matches!(&**b, Expr::Ident(_)) => {
                let Expr::Ident(n) = &**b else { unreachable!() };
                let t = self.tmp();
                self.index(n, args, t, 0, false);
                self.emit(Op::SetAns(t, print));
            }
            _ => {
                let t = self.tmp();
                self.expr(e, t);
                self.emit(Op::SetAns(t, print));
            }
        }
    }

    fn assign1(&mut self, lv: &LValue, rhs: &Expr, print: bool, op: Option<BinOp>) {
        match lv {
            LValue::Var(n) => {
                let v = self.var(n).unwrap();
                match op {
                    None => self.expr(rhs, v),
                    Some(o) => {
                        let t = self.tmp();
                        self.expr(rhs, t);
                        let nm = self.str(n);
                        self.emit(Op::Compound(o, v, t, nm));
                    }
                }
                if print {
                    let nm = self.str(n);
                    self.emit(Op::Display(v, nm));
                }
            }
            LValue::Index(n, args) if op.is_none() && !args.iter().any(has_end) && args.iter().all(|a| self.simple_idx(a)) => {
                // superinstruction: element store; checks (value, variable, indices) — on the slow path
                let v = self.var(n).unwrap();
                let nm = self.str(n);
                let t = self.tmp();
                self.expr(rhs, t);
                let regs = self.direct_args(args);
                let s = self.site(n, regs, 1, true, false, vec![]);
                self.p.sites[s as usize].direct = true;
                self.emit(Op::IndexAsg(v, t, s));
                if print {
                    self.emit(Op::Display(v, nm));
                }
            }
            LValue::Index(n, args) => {
                let v = self.var(n).unwrap();
                let nm = self.str(n);
                let t = self.tmp();
                self.expr(rhs, t);
                if let Some(o) = op {
                    self.emit(Op::ReadCheck(v, nm));
                    let old = self.tmp();
                    let regs = self.idx_args(v, args, true);
                    let s = self.site(n, regs, 1, true, false, vec![]);
                    self.emit(Op::Index(old, v, s));
                    self.emit(Op::Bin(o, t, old, t));
                }
                self.emit(Op::AsgCheck(v, t, nm));
                let regs = self.idx_args(v, args, false);
                let s = self.site(n, regs, 1, true, false, vec![]);
                self.emit(Op::IndexAsg(v, t, s));
                if print {
                    self.emit(Op::Display(v, nm));
                }
            }
            LValue::Tilde => {
                let t = self.tmp();
                self.expr(rhs, t);
            }
            LValue::Field(..) => unreachable!(),
        }
    }

    fn assign_multi(&mut self, lhs: &[LValue], rhs: &Expr, print: bool) {
        let (n, args, is_index): (&String, &[Expr], bool) = match rhs {
            Expr::Ident(n) => (n, &[], false),
            Expr::Index(b, a) => match &**b {
                Expr::Ident(n) => (n, &a[..], true),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };
        let mut regs = Vec::with_capacity(args.len());
        for a in args {
            let t = self.tmp();
            self.expr(a, t);
            regs.push(t);
        }
        let names = if is_index { arg_names(args) } else { vec![] };
        let s = self.site(n, regs, lhs.len(), false, is_index, names);
        self.emit(Op::CallMulti(s));
        for (k, lv) in lhs.iter().enumerate() {
            if let LValue::Var(x) = lv {
                let v = self.var(x).unwrap();
                self.emit(Op::TakeMulti(v, k as u16));
            }
        }
        if print {
            for lv in lhs {
                if let LValue::Var(x) = lv {
                    let v = self.var(x).unwrap();
                    let nm = self.str(x);
                    self.emit(Op::Display(v, nm));
                }
            }
        }
    }

    /// Condition: evaluates it and emits a jump «if false» (returns its index for patching).
    fn cond_false(&mut self, e: &Expr) -> usize {
        let top = self.top;
        let mut e = e;
        while let Expr::Paren(x) = e {
            e = x;
        }
        let j = match e {
            Expr::Binary(o @ (BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge | BinOp::Eq | BinOp::Ne), l, r) => {
                if let Expr::Num(k, _) = &**r {
                    let a = self.operand(l);
                    self.emit(Op::JmpCmpK(*o, a, *k, NOJ))
                } else {
                    let a = self.left(l, r);
                    let b = self.operand(r);
                    self.emit(Op::JmpCmp(*o, a, b, NOJ))
                }
            }
            _ => {
                let a = self.operand(e);
                self.emit(Op::JmpFalse(a, NOJ))
            }
        };
        self.top = top;
        j
    }

    // ---- expressions

    /// Variable as an operand — its register directly (undefined → function call on the slow path).
    fn direct(&self, e: &Expr) -> Option<R> {
        match e {
            Expr::Ident(n) => self.var(n),
            Expr::Paren(x) => self.direct(x),
            _ => None,
        }
    }
    /// Side-effect free: a constant, a string or a variable.
    fn simple(&self, e: &Expr) -> bool {
        match e {
            Expr::Num(..) | Expr::Str(..) => true,
            Expr::Ident(n) => self.var(n).is_some(),
            Expr::Paren(x) => self.simple(x),
            _ => false,
        }
    }
    fn operand(&mut self, e: &Expr) -> R {
        if let Some(v) = self.direct(e) {
            return v;
        }
        let t = self.tmp();
        self.expr(e, t);
        t
    }
    /// Left operand: directly only when the right one has no side effects (evaluation order as in the tree).
    fn left(&mut self, l: &Expr, r: &Expr) -> R {
        match self.direct(l) {
            Some(v) if self.simple(r) => v,
            _ => {
                let t = self.tmp();
                self.expr(l, t);
                t
            }
        }
    }

    /// Evaluates an expression into register `d`; `d` is written only by the last instruction (so `x = x + y` is safe).
    fn expr(&mut self, e: &Expr, d: R) {
        let top = self.top;
        match e {
            Expr::Num(v, _) => {
                self.emit(Op::LoadF(d, *v));
            }
            Expr::Imag(v, _) => self.load_value(d, Value::Mat(Mat::cscalar(0.0, *v))),
            Expr::Str(s, _) => self.load_value(d, Value::str(s)),
            Expr::Colon => self.load_value(d, Value::str(":")),
            Expr::End => {
                self.emit(Op::EndVal(d));
            }
            Expr::FuncHandle(n) => self.load_value(d, Value::Func(Rc::new(Func::Named(n.clone())))),
            Expr::Ident(n) => match self.var(n) {
                Some(v) => {
                    self.emit(Op::GetVar(d, v));
                }
                None => {
                    let s = self.site(n, vec![], 1, true, false, vec![]);
                    self.emit(Op::Call(d, s));
                }
            },
            Expr::Paren(x) => self.expr(x, d),
            Expr::Unary(o, x) => {
                let a = self.operand(x);
                self.emit(Op::Un(*o, d, a));
            }
            Expr::Postfix(o, x) => {
                let a = self.operand(x);
                self.emit(Op::Transp(*o == PostOp::CTranspose, d, a));
            }
            Expr::Binary(o @ (BinOp::AndAnd | BinOp::OrOr), l, r) => {
                let and = *o == BinOp::AndAnd;
                let t = self.tmp();
                self.expr(l, t);
                self.emit(Op::ScBool(t, t, and));
                let j = self.emit(Op::JmpIfL(t, !and, NOJ));
                self.expr(r, t);
                self.emit(Op::ScBool(t, t, and));
                self.patch_here(j);
                self.emit(Op::Move(d, t));
            }
            Expr::Binary(o, l, r) => {
                if let Expr::Num(k, _) = &**r {
                    let a = self.operand(l);
                    self.emit(if *o == BinOp::Add { Op::AddK(d, a, *k) } else { Op::BinK(*o, d, a, *k) });
                } else if let Expr::Num(k, _) = &**l {
                    let b = self.operand(r);
                    self.emit(Op::KBin(*o, d, *k, b));
                } else {
                    let a = self.left(l, r);
                    let b = self.operand(r);
                    self.emit(if *o == BinOp::Add && self.fault_add { Op::AddBug(d, a, b) } else { Op::Bin(*o, d, a, b) });
                }
            }
            Expr::Range(a, s, b) => {
                let ta = self.tmp();
                self.expr(a, ta);
                let ts = match s {
                    Some(s) => {
                        let t = self.tmp();
                        self.expr(s, t);
                        t
                    }
                    None => NO,
                };
                let tb = self.tmp();
                self.expr(b, tb);
                self.emit(Op::Range(d, ta, ts, tb));
            }
            Expr::Index(b, args) => match &**b {
                Expr::Ident(n) => self.index(n, args, d, 1, true),
                _ => unreachable!("ok_expr"),
            },
            Expr::Matrix(rows) => {
                let mut shape = Vec::with_capacity(rows.len());
                for row in rows {
                    let mut rr = Vec::with_capacity(row.len());
                    for x in row {
                        let t = self.tmp();
                        self.expr(x, t);
                        rr.push(t);
                    }
                    shape.push(rr);
                }
                self.p.concats.push(shape);
                let k = (self.p.concats.len() - 1) as u32;
                self.emit(Op::Concat(d, k));
            }
            Expr::AnonFn(params, body) => {
                let mut names = HashSet::new();
                crate::interp::collect_idents(body, &mut names);
                let mut sorted: Vec<&String> = names.iter().collect();
                sorted.sort();
                let caps = sorted
                    .into_iter()
                    .filter(|n| !params.contains(n))
                    .filter_map(|n| self.var(n).map(|r| (n.clone(), r)))
                    .collect();
                self.p.closures.push(Closure { params: params.clone(), body: body.clone(), caps });
                let k = (self.p.closures.len() - 1) as u32;
                self.emit(Op::MakeClosure(d, k));
            }
            Expr::Field(base, name) => {
                let t = self.tmp();
                self.expr(base, t);
                let nm = self.str(name);
                self.emit(Op::GetField(d, t, nm));
            }
            Expr::CellList(_) => unreachable!("ok_expr"),
        }
        self.top = top;
    }

    /// Index arguments of a variable: each into its own register; `end` — the variable's size context.
    fn idx_args(&mut self, v: R, args: &[Expr], read: bool) -> Vec<R> {
        let n = args.len();
        let mut regs = Vec::with_capacity(n);
        for (k, a) in args.iter().enumerate() {
            let t = self.tmp();
            if has_end(a) && !matches!(a, Expr::Colon) {
                self.emit(Op::PushEnd(v, k as u8, n as u8));
                self.expr(a, t);
                self.emit(Op::PopEnd);
            } else {
                self.expr(a, t);
            }
            self.emit(if read { Op::CheckIdx(v, t) } else { Op::CheckArg(t) });
            regs.push(t);
        }
        regs
    }

    /// Simple index argument: a variable, a number or `:` — no side effects (except a function call in place of
    /// an undefined variable, which the instruction itself then makes, in the same order).
    fn simple_idx(&self, a: &Expr) -> bool {
        match a {
            Expr::Num(..) | Expr::Colon => true,
            Expr::Ident(n) => self.var(n).is_some(),
            _ => false,
        }
    }
    fn direct_args(&mut self, args: &[Expr]) -> Vec<R> {
        let mut regs = Vec::with_capacity(args.len());
        for a in args {
            match a {
                Expr::Ident(n) => regs.push(self.var(n).unwrap()),
                _ => {
                    let t = self.tmp();
                    self.expr(a, t);
                    regs.push(t);
                }
            }
        }
        regs
    }

    /// `name(args)`: variable — runtime dispatch by type; otherwise — a function call.
    fn index(&mut self, name: &str, args: &[Expr], d: R, nargout: usize, need: bool) {
        let top = self.top;
        if let Some(v) = self.var(name) {
            let end = args.iter().any(has_end);
            let e = Expr::Index(Box::new(Expr::Ident(name.to_string())), args.to_vec());
            let mut names = HashSet::new();
            let mut fs = false;
            names_expr(&e, &mut names, &mut fs);
            let sync = self.sync_list(&names, fs);
            self.p.dyns.push(Dyn { expr: e, sync, nargout, need });
            let di = (self.p.dyns.len() - 1) as u32;
            if !end && args.iter().all(|a| self.simple_idx(a)) {
                // superinstruction: element read directly from the index variables' registers
                let regs = self.direct_args(args);
                let s = self.site(name, regs, nargout, need, true, arg_names(args));
                self.p.sites[s as usize].direct = true;
                self.p.sites[s as usize].dyn_idx = di;
                self.emit(Op::Index(d, v, s));
                self.top = top;
                return;
            }
            let sw = self.emit(if end { Op::TypeIsMat(v, NOJ) } else { Op::TypeSwitch(v, NOJ) });
            let regs = self.idx_args(v, args, true);
            let s = self.site(name, regs, nargout, need, true, arg_names(args));
            self.emit(Op::Index(d, v, s));
            let j = self.emit(Op::Jmp(NOJ));
            self.patch_here(sw);
            self.emit(Op::DynEval(d, di));
            self.patch_here(j);
        } else {
            let mut regs = Vec::with_capacity(args.len());
            for a in args {
                let t = self.tmp();
                self.expr(a, t);
                regs.push(t);
            }
            let s = self.site(name, regs, nargout, need, true, arg_names(args));
            self.emit(Op::Call(d, s));
        }
        self.top = top;
    }
}

fn compile_unit(
    user: &HashMap<String, usize>,
    registry: &Registry,
    fault_add: bool,
    name: &str,
    params: &[String],
    outputs: &[String],
    body: &[Stmt],
    extra: &[String],
    is_fn: bool,
) -> Proto {
    let mut order: Vec<String> = Vec::new();
    {
        let mut seen: HashSet<String> = HashSet::new();
        let mut add = |n: &str| {
            if n != "~" && seen.insert(n.to_string()) {
                order.push(n.to_string());
            }
        };
        for p in params {
            add(p);
        }
        for o in outputs {
            add(o);
        }
        if is_fn {
            add("nargin");
            add("nargout");
        }
        add("ans");
        assigned(body, &mut add);
        for x in extra {
            add(x);
        }
    }
    let vars: HashMap<String, R> = order.iter().enumerate().map(|(i, n)| (n.clone(), i as R)).collect();
    let mut p = Proto { name: name.to_string(), nvars: order.len(), nregs: order.len(), ..Default::default() };
    p.vars = order.iter().map(|n| (n.clone(), vars[n])).collect();
    p.reg_names = order.clone();
    p.params = params.iter().map(|n| if n == "~" { NO } else { vars[n] }).collect();
    p.outputs = outputs.iter().map(|n| vars[n]).collect();
    p.output_names = outputs.to_vec();
    p.nargin_reg = if is_fn { vars["nargin"] } else { NO };
    p.nargout_reg = if is_fn { vars["nargout"] } else { NO };
    p.ans_reg = vars["ans"];
    let mut cx = Cx { user, registry, fault_add, p, vars, top: order.len(), loops: Vec::new() };
    cx.block(body);
    cx.emit(Op::Ret);
    cx.p
}

/// Compiles all interpreter functions and the script body. `extra` — variables already in the workspace.
pub fn compile_program(it: &Interp, script: &[Stmt], extra: &[String]) -> (VmProgram, Proto) {
    let mut names: Vec<&String> = it.functions.keys().collect();
    names.sort();
    let index: HashMap<String, usize> = names.iter().enumerate().map(|(i, n)| ((*n).clone(), i)).collect();
    let fault_add = it.fault.as_deref() == Some("vm:add");
    let registry = &*it.registry;
    let protos = names
        .iter()
        .map(|n| {
            let f = &it.functions[*n];
            compile_unit(&index, registry, fault_add, &f.name, &f.params, &f.outputs, &f.body, &[], true)
        })
        .collect();
    let script = compile_unit(&index, registry, fault_add, "<script>", &[], &[], script, extra, false);
    (VmProgram { protos, index }, script)
}

// ---------------------------------------------------------------- execution

/// Registers → frame (by move): only the variables mentioned in the statement.
fn sync_out(it: &mut Interp, sync: &[(String, R)], regs: &mut [Reg]) {
    let frame = it.frames.last_mut().unwrap();
    for (name, r) in sync {
        match std::mem::take(&mut regs[*r as usize]).into_value() {
            Some(v) => {
                frame.insert(name.clone(), v);
            }
            None => {
                frame.remove(name);
            }
        }
    }
}

/// Frame → registers (by move).
fn sync_in(it: &mut Interp, sync: &[(String, R)], regs: &mut [Reg]) {
    let frame = it.frames.last_mut().unwrap();
    for (name, r) in sync {
        regs[*r as usize] = frame.remove(name).map(Reg::from_value).unwrap_or(Reg::U);
    }
}

/// Running a program on the VM (from `Interp::run`): the same output and error protocol as the tree.
pub fn run_program(it: &mut Interp, prog: &Program) -> bool {
    let mut extra: Vec<String> = it.frames.last().unwrap().keys().cloned().collect();
    extra.sort();
    let (vp, script) = compile_program(it, &prog.body, &extra);
    it.vm_stats.native += script.native + vp.protos.iter().map(|p| p.native).sum::<usize>();
    it.vm_stats.fallback += script.fallback + vp.protos.iter().map(|p| p.fallback).sum::<usize>();
    let vp = Rc::new(vp);
    it.vm = Some(vp.clone());
    let mut regs = vec![Reg::U; script.nregs];
    let mut loops = vec![LoopState::Idle; script.nloops];
    sync_in(it, &script.vars, &mut regs);
    let r = exec(it, &vp, &script, &mut regs, &mut loops);
    sync_out(it, &script.vars, &mut regs);
    match r {
        Ok(()) | Err(Flow::Return) | Err(Flow::Break) | Err(Flow::Continue) => true,
        Err(Flow::Err(e)) => {
            it.last_error = e.msg.clone();
            it.err_out(&format!("error: {}\n", e.msg));
            false
        }
    }
}

/// Calling a compiled function from the tree (`Interp::call_user`, argument count checks already done).
pub(crate) fn call_from_tree(it: &mut Interp, vp: &Rc<VmProgram>, idx: usize, args: Vec<Value>, nargout: usize) -> Result<Vec<Value>, Flow> {
    let callee = &vp.protos[idx];
    it.enter()?;
    let mut fr = it.vm_pool.pop().unwrap_or_default();
    fr.regs.resize(callee.nregs, Reg::U);
    let nargin = args.len();
    for (k, a) in args.into_iter().enumerate() {
        let pr = callee.params[k];
        if pr != NO {
            fr.regs[pr as usize] = Reg::from_value(a);
        }
    }
    let outs = finish_call(it, vp, callee, fr, nargin, nargout, |c, r| collect_outputs(c, r, nargout))?;
    Ok(outs.into_iter().filter_map(Reg::into_value).collect())
}

/// Arguments of a user function call: take from temporary registers or already collected.
enum Args<'r> {
    Take(&'r mut [Reg], &'r [R]),
    Owned(Vec<Reg>),
}

/// VM-to-VM call: arguments are passed in registers (scalars — without memory allocation).
fn call_user_vm<T>(
    it: &mut Interp,
    prog: &VmProgram,
    idx: usize,
    args: Args,
    nargout: usize,
    collect: impl FnOnce(&Proto, &mut [Reg]) -> Result<T, Flow>,
) -> Result<T, Flow> {
    let callee = &prog.protos[idx];
    let nargin = match &args {
        Args::Take(_, a) => a.len(),
        Args::Owned(v) => v.len(),
    };
    if nargin > callee.params.len() {
        return Err(MError::new(format!("{}: function called with too many inputs", callee.name)).into());
    }
    if nargout > callee.outputs.len() {
        return Err(MError::new(format!("{}: function called with too many outputs", callee.name)).into());
    }
    it.enter()?;
    let mut fr = it.vm_pool.pop().unwrap_or_default();
    fr.regs.resize(callee.nregs, Reg::U);
    match args {
        Args::Take(regs, a) => {
            for (k, &r) in a.iter().enumerate() {
                let v = std::mem::take(&mut regs[r as usize]);
                let pr = callee.params[k];
                if pr != NO {
                    fr.regs[pr as usize] = v;
                }
            }
        }
        Args::Owned(v) => {
            for (k, x) in v.into_iter().enumerate() {
                let pr = callee.params[k];
                if pr != NO {
                    fr.regs[pr as usize] = x;
                }
            }
        }
    }
    finish_call(it, prog, callee, fr, nargin, nargout, collect)
}

fn finish_call<T>(
    it: &mut Interp,
    prog: &VmProgram,
    callee: &Proto,
    mut fr: FrameBuf,
    nargin: usize,
    nargout: usize,
    collect: impl FnOnce(&Proto, &mut [Reg]) -> Result<T, Flow>,
) -> Result<T, Flow> {
    fr.loops.resize(callee.nloops, LoopState::Idle);
    fr.regs[callee.nargin_reg as usize] = Reg::F(nargin as f64);
    fr.regs[callee.nargout_reg as usize] = Reg::F(nargout as f64);
    it.frames.push(HashMap::new());
    let saved = std::mem::take(&mut it.end_stack);
    let r = exec(it, prog, callee, &mut fr.regs, &mut fr.loops);
    it.end_stack = saved;
    it.frames.pop();
    it.depth -= 1;
    let out = match r {
        Ok(()) | Err(Flow::Return) | Err(Flow::Break) | Err(Flow::Continue) => collect(callee, &mut fr.regs),
        Err(e) => Err(e),
    };
    fr.regs.clear();
    fr.loops.clear();
    it.vm_pool.push(fr);
    out
}

fn collect_outputs(callee: &Proto, regs: &mut [Reg], nargout: usize) -> Result<Vec<Reg>, Flow> {
    let mut outs = Vec::with_capacity(nargout.max(1));
    for (k, &o) in callee.outputs.iter().enumerate() {
        if k >= nargout.max(1) {
            break;
        }
        match &regs[o as usize] {
            Reg::U => {
                if k < nargout {
                    return Err(undefined_output(callee, k));
                }
                break;
            }
            v => outs.push(v.clone()),
        }
    }
    Ok(outs)
}

/// First result (nargout ≤ 1) — without a `Vec`; `U` — no result.
fn collect_first(callee: &Proto, regs: &mut [Reg], nargout: usize) -> Result<Reg, Flow> {
    match callee.outputs.first() {
        None => Ok(Reg::U),
        Some(&o) => match std::mem::take(&mut regs[o as usize]) {
            Reg::U if nargout > 0 => Err(undefined_output(callee, 0)),
            v => Ok(v),
        },
    }
}

fn undefined_output(callee: &Proto, k: usize) -> Flow {
    MError::new(format!("value on right hand side of assignment undefined: '{}' undefined in {}", callee.output_names[k], callee.name)).into()
}

fn first_val(vals: Vec<Value>, need: bool) -> Result<Reg, MError> {
    match vals.into_iter().next() {
        Some(v) => Ok(Reg::from_value(v)),
        None if need => Err(MError::new("value on right hand side of assignment undefined")),
        None => Ok(Reg::U),
    }
}

/// Undefined variable as a value — call the function with that name (like `eval(Ident)` in the tree).
#[inline(never)]
fn resolve_undef(it: &mut Interp, p: &Proto, r: R) -> Result<Value, MError> {
    let name = p.reg_names.get(r as usize).cloned().unwrap_or_default();
    match it.call_function(&name, vec![], 1) {
        Ok(vals) => vals.into_iter().next().ok_or_else(|| MError::new("value on right hand side of assignment undefined")),
        Err(Flow::Err(e)) => Err(e),
        Err(_) => Err(MError::new("value on right hand side of assignment undefined")),
    }
}

fn opnd<'a>(it: &mut Interp, p: &Proto, regs: &'a [Reg], r: R) -> Result<Opnd<'a>, MError> {
    Ok(match &regs[r as usize] {
        Reg::V(v) => Opnd::B(v),
        Reg::F(x) => Opnd::O(Value::num(*x)),
        Reg::L(b) => Opnd::O(Value::boolean(*b)),
        Reg::U => Opnd::O(resolve_undef(it, p, r)?),
    })
}

#[inline(always)]
fn scalar_bin(op: BinOp, x: f64, y: f64) -> Option<Reg> {
    Some(match op {
        BinOp::Add => Reg::F(x + y),
        BinOp::Sub => Reg::F(x - y),
        BinOp::Mul | BinOp::EMul => Reg::F(x * y),
        BinOp::Div | BinOp::EDiv => Reg::F(x / y),
        BinOp::LDiv | BinOp::ELDiv => Reg::F(y / x),
        BinOp::Pow | BinOp::EPow => {
            if x < 0.0 && y != y.trunc() {
                return None; // complex result — slow path
            }
            Reg::F(x.powf(y))
        }
        BinOp::Eq => Reg::L(x == y),
        BinOp::Ne => Reg::L(x != y),
        BinOp::Lt => Reg::L(x < y),
        BinOp::Le => Reg::L(x <= y),
        BinOp::Gt => Reg::L(x > y),
        BinOp::Ge => Reg::L(x >= y),
        BinOp::And => Reg::L(x != 0.0 && y != 0.0),
        BinOp::Or => Reg::L(x != 0.0 || y != 0.0),
        BinOp::AndAnd | BinOp::OrOr => return None,
    })
}

#[inline(always)]
fn scalar_cmp(op: BinOp, x: f64, y: f64) -> bool {
    match op {
        BinOp::Lt => x < y,
        BinOp::Le => x <= y,
        BinOp::Gt => x > y,
        BinOp::Ge => x >= y,
        BinOp::Eq => x == y,
        _ => x != y,
    }
}

#[inline(never)]
fn slow_bin(it: &mut Interp, p: &Proto, regs: &mut [Reg], op: BinOp, d: R, a: R, b: R) -> Result<(), MError> {
    let v = {
        let av = opnd(it, p, regs, a)?;
        let bv = opnd(it, p, regs, b)?;
        ops::binary(it, op, &av, &bv)?
    };
    // release temporary operands so as not to hold extra references (copy-on-write)
    for r in [a, b] {
        if r as usize >= p.nvars && r != d {
            regs[r as usize] = Reg::U;
        }
    }
    regs[d as usize] = Reg::from_value(v);
    Ok(())
}

#[inline(never)]
fn slow_bin_k(it: &mut Interp, p: &Proto, regs: &mut [Reg], op: BinOp, d: R, a: R, k: f64) -> Result<(), MError> {
    let v = {
        let av = opnd(it, p, regs, a)?;
        ops::binary(it, op, &av, &Value::num(k))?
    };
    if a as usize >= p.nvars && a != d {
        regs[a as usize] = Reg::U;
    }
    regs[d as usize] = Reg::from_value(v);
    Ok(())
}

#[inline(never)]
fn slow_kbin(it: &mut Interp, p: &Proto, regs: &mut [Reg], op: BinOp, d: R, k: f64, b: R) -> Result<(), MError> {
    let v = {
        let bv = opnd(it, p, regs, b)?;
        ops::binary(it, op, &Value::num(k), &bv)?
    };
    if b as usize >= p.nvars && b != d {
        regs[b as usize] = Reg::U;
    }
    regs[d as usize] = Reg::from_value(v);
    Ok(())
}

#[inline(never)]
fn slow_truthy(it: &mut Interp, p: &Proto, regs: &[Reg], r: R) -> Result<bool, MError> {
    let v = opnd(it, p, regs, r)?;
    ops::truthy(&v)
}

#[inline(never)]
fn slow_cmp(it: &mut Interp, p: &Proto, regs: &[Reg], op: BinOp, a: R, b: Result<R, f64>) -> Result<bool, MError> {
    let av = opnd(it, p, regs, a)?;
    let v = match b {
        Ok(b) => {
            let bv = opnd(it, p, regs, b)?;
            ops::binary(it, op, &av, &bv)?
        }
        Err(k) => ops::binary(it, op, &av, &Value::num(k))?,
    };
    ops::truthy(&v)
}

#[inline(never)]
fn sc_bool_slow(it: &mut Interp, p: &Proto, regs: &[Reg], r: R, and: bool) -> Result<bool, MError> {
    let op = if and { "&&" } else { "||" };
    let v = opnd(it, p, regs, r)?;
    match &*v {
        Value::Mat(m) => {
            if m.is_empty() {
                return Err(MError::new(format!("invalid conversion from empty value to real scalar (operator {op})")));
            }
            if !m.is_scalar() {
                return Err(MError::new(format!(
                    "binary operator '{op}': operands must be convertible to logical scalar values (got {}x{})",
                    m.rows, m.cols
                )));
            }
            Ok(m.re[0] != 0.0 || m.im_at(0) != 0.0)
        }
        other => Err(MError::new(format!("binary operator '{op}' not implemented for '{}'", other.class_name()))),
    }
}

fn col_of(v: &Value, j: usize) -> Reg {
    let Value::Mat(m) = v else { unreachable!() };
    if m.rows == 1 && m.im.is_none() && m.class == Class::Double {
        return Reg::F(m.re[j]);
    }
    let mut c = if m.rows == 1 {
        Mat { rows: 1, cols: 1, re: vec![m.re[j]], im: m.im.as_ref().map(|v| vec![v[j]]), class: m.class }
    } else {
        let rg = j * m.rows..(j + 1) * m.rows;
        Mat { rows: m.rows, cols: 1, re: m.re[rg.clone()].to_vec(), im: m.im.as_ref().map(|v| v[rg].to_vec()), class: m.class }
    };
    c.narrow();
    Reg::from_mat(c)
}

/// Start of a loop over a value (by columns); returns false if there are no iterations.
fn start_cols(regs: &mut [Reg], loops: &mut [LoopState], slot: u16, var: R, v: Reg) -> bool {
    match v {
        Reg::V(rc) => {
            let dims = if let Value::Mat(m) = &*rc { Some((m.rows, m.cols)) } else { None };
            match dims {
                Some((r, c)) => {
                    if r == 0 || c == 0 {
                        return false;
                    }
                    regs[var as usize] = col_of(&rc, 0);
                    loops[slot as usize] = LoopState::Cols { m: rc, j: 0, n: c };
                }
                None => {
                    // a single value (handle, table) — a single iteration
                    regs[var as usize] = Reg::V(rc);
                    loops[slot as usize] = LoopState::Single;
                }
            }
            true
        }
        Reg::U => false,
        other => {
            regs[var as usize] = other;
            loops[slot as usize] = LoopState::Single;
            true
        }
    }
}

#[inline(never)]
fn for_range_slow(it: &mut Interp, p: &Proto, regs: &mut [Reg], a: R, s: R, b: R) -> Result<Reg, MError> {
    let v = {
        let av = opnd(it, p, regs, a)?;
        let sv = if s == NO { None } else { Some(opnd(it, p, regs, s)?) };
        let bv = opnd(it, p, regs, b)?;
        ops::range(&av, sv.as_deref(), &bv)?
    };
    Ok(Reg::from_value(v))
}

fn idx_arg(r: Reg) -> Result<IdxArg, MError> {
    match r {
        Reg::F(x) => Ok(IdxArg::Vals(Mat::scalar(x))),
        Reg::L(b) => Ok(IdxArg::Vals(Mat::boolean(b))),
        Reg::V(v) => match unwrap_rc(v) {
            Value::Mat(m) => {
                if m.class == Class::Char && m.numel() == 1 && m.re[0] == ':' as u32 as f64 {
                    Ok(IdxArg::All)
                } else {
                    Ok(IdxArg::Vals(m))
                }
            }
            other => Err(MError::new(format!("subscript indices must be numeric or logical, not {}", other.class_name()))),
        },
        Reg::U => Err(MError::new("subscript indices must be numeric or logical, not undefined")),
    }
}

fn take_idx_args(regs: &mut [Reg], args: &[R]) -> Result<Vec<IdxArg>, MError> {
    args.iter().map(|&a| idx_arg(std::mem::take(&mut regs[a as usize]))).collect()
}

fn take_values(regs: &mut [Reg], args: &[R]) -> Vec<Value> {
    args.iter()
        .map(|&a| std::mem::take(&mut regs[a as usize]).into_value().unwrap_or_else(|| Value::Mat(Mat::empty())))
        .collect()
}

/// Direct arguments: register copies; an undefined variable — a call of the function with its name (in order, as in the tree).
fn gather(it: &mut Interp, p: &Proto, regs: &[Reg], args: &[R], check_idx: bool) -> Result<Vec<Reg>, MError> {
    let mut out = Vec::with_capacity(args.len());
    for &a in args {
        let r = match &regs[a as usize] {
            Reg::U => Reg::from_value(resolve_undef(it, p, a)?),
            r => r.clone(),
        };
        if check_idx && !r.is_mat() {
            return Err(MError::new(format!("subscript indices must be numeric or logical, not {}", r.class_name())));
        }
        out.push(r);
    }
    Ok(out)
}

fn call_builtin(it: &mut Interp, s: &Site, f: BuiltinFn, argv: Vec<Value>) -> Result<Vec<Value>, Flow> {
    if s.set_names {
        it.arg_names = s.arg_names.clone();
    }
    let r = f(it, &argv, s.nargout);
    if s.set_names {
        it.arg_names.clear();
    }
    let mut vals = r?;
    if it.fault.as_deref() == Some(s.name.as_str()) {
        // negative control: broken builtin — +1 to the first element of the first result (as in the tree)
        if let Some(Value::Mat(m)) = vals.get_mut(0) {
            if let Some(x) = m.re.first_mut() {
                *x += 1.0;
                m.class = Class::Double;
            }
        }
    }
    Ok(vals)
}

fn call_user_named<T>(
    it: &mut Interp,
    prog: &VmProgram,
    s: &Site,
    idx: usize,
    args: Args,
    collect: impl FnOnce(&Proto, &mut [Reg]) -> Result<T, Flow>,
) -> Result<T, Flow> {
    if s.set_names {
        it.arg_names = s.arg_names.clone();
    }
    let r = call_user_vm(it, prog, idx, args, s.nargout, collect);
    if s.set_names {
        it.arg_names.clear();
    }
    r
}

fn missing(it: &mut Interp, s: &Site) -> Flow {
    if s.set_names {
        it.arg_names.clear();
    }
    MError::new(format!("'{}' undefined", s.name)).into()
}

fn site_args<'r>(it: &mut Interp, p: &Proto, regs: &'r mut [Reg], s: &'r Site) -> Result<Args<'r>, MError> {
    Ok(if s.direct { Args::Owned(gather(it, p, regs, &s.args, false)?) } else { Args::Take(regs, &s.args) })
}

fn args_values(args: Args) -> Vec<Value> {
    match args {
        Args::Take(regs, a) => take_values(regs, a),
        Args::Owned(v) => v.into_iter().map(|r| r.into_value().unwrap_or_else(|| Value::Mat(Mat::empty()))).collect(),
    }
}

/// Call by site with all results (`[a, b] = f(…)`).
#[inline(never)]
fn do_call(it: &mut Interp, prog: &VmProgram, p: &Proto, regs: &mut [Reg], s: &Site) -> Result<Vec<Reg>, Flow> {
    let args = site_args(it, p, regs, s)?;
    match s.target {
        Target::User(idx) => call_user_named(it, prog, s, idx, args, |c, r| collect_outputs(c, r, s.nargout)),
        Target::Builtin(f) => {
            let argv = args_values(args);
            Ok(call_builtin(it, s, f, argv)?.into_iter().map(Reg::from_value).collect())
        }
        Target::Missing => Err(missing(it, s)),
    }
}

/// Call by site with the first result (`U` — none): a script function (VM to VM), a builtin, or an error.
#[inline(never)]
fn do_call1(it: &mut Interp, prog: &VmProgram, p: &Proto, regs: &mut [Reg], s: &Site) -> Result<Reg, Flow> {
    let args = site_args(it, p, regs, s)?;
    match s.target {
        Target::User(idx) => call_user_named(it, prog, s, idx, args, |c, r| collect_first(c, r, s.nargout)),
        Target::Builtin(f) => {
            let argv = args_values(args);
            Ok(call_builtin(it, s, f, argv)?.into_iter().next().map(Reg::from_value).unwrap_or(Reg::U))
        }
        Target::Missing => Err(missing(it, s)),
    }
}

fn need_value(r: Reg, need: bool) -> Result<Reg, MError> {
    if need && !r.is_def() {
        return Err(MError::new("value on right hand side of assignment undefined"));
    }
    Ok(r)
}

#[inline(never)]
fn dyn_eval(it: &mut Interp, p: &Proto, regs: &mut [Reg], d: R, di: u32) -> Result<(), Flow> {
    let ds = &p.dyns[di as usize];
    it.vm_stats.dyn_runs += 1;
    sync_out(it, &ds.sync, regs);
    let r = it.eval_multi(&ds.expr, ds.nargout);
    sync_in(it, &ds.sync, regs);
    regs[d as usize] = first_val(r?, ds.need)?;
    Ok(())
}

#[inline(never)]
fn index_slow(it: &mut Interp, prog: &VmProgram, p: &Proto, regs: &mut [Reg], d: R, var: R, si: u32) -> Result<(), Flow> {
    let s = &p.sites[si as usize];
    if regs[var as usize].is_mat() {
        let iargs = if s.direct {
            gather(it, p, regs, &s.args, true)?.into_iter().map(idx_arg).collect::<Result<Vec<_>, _>>()?
        } else {
            take_idx_args(regs, &s.args)?
        };
        let r = {
            let m = mat_cow(&regs[var as usize]).unwrap();
            index::read(&m, &iargs, &s.name)?
        };
        regs[d as usize] = Reg::from_mat(r);
        return Ok(());
    }
    let func = match &regs[var as usize] {
        Reg::V(v) => match &**v {
            Value::Func(f) => Some(f.clone()),
            _ => None,
        },
        _ => None,
    };
    if let Some(f) = func {
        let argv = if s.direct {
            gather(it, p, regs, &s.args, false)?.into_iter().map(|r| r.into_value().unwrap_or_else(|| Value::Mat(Mat::empty()))).collect()
        } else {
            take_values(regs, &s.args)
        };
        let fs = matches!(&*f, Func::Named(n) if frame_sensitive(n));
        if fs {
            sync_out(it, &p.vars, regs);
        }
        let r = it.call_func_value(&f, argv, s.nargout);
        if fs {
            sync_in(it, &p.vars, regs);
        }
        let v = first_val(r?, s.need)?;
        regs[d as usize] = v;
        return Ok(());
    }
    if !regs[var as usize].is_def() {
        let r = do_call1(it, prog, p, regs, s)?;
        regs[d as usize] = need_value(r, s.need)?;
        return Ok(());
    }
    if s.dyn_idx != NOJ {
        // table, string array: the tree executes the whole expression (arguments are simple — not yet evaluated)
        return dyn_eval(it, p, regs, d, s.dyn_idx);
    }
    Err(MError::new(format!("indexing a {} is not supported in mlab v1", regs[var as usize].class_name())).into())
}

/// Fast scalar assignment within a real double matrix (the same result as `index::assign`).
#[inline(always)]
fn asg_fast(regs: &mut [Reg], v: R, x: f64, i: f64, j: Option<f64>) -> bool {
    let Reg::V(rc) = &mut regs[v as usize] else { return false };
    let (rows, cols) = match &**rc {
        Value::Mat(m) if m.im.is_none() && m.class == Class::Double => (m.rows, m.cols),
        _ => return false,
    };
    let k = match j {
        None => {
            if !(i >= 1.0 && i == i.trunc() && i <= (rows * cols) as f64) {
                return false;
            }
            i as usize - 1
        }
        Some(j) => {
            if !(i >= 1.0 && i == i.trunc() && i <= rows as f64 && j >= 1.0 && j == j.trunc() && j <= cols as f64) {
                return false;
            }
            (j as usize - 1) * rows + (i as usize - 1)
        }
    };
    if let Value::Mat(m) = Rc::make_mut(rc) {
        m.re[k] = x;
    }
    true
}

#[inline(never)]
fn index_asg_slow(it: &mut Interp, p: &Proto, regs: &mut [Reg], v: R, val: R, si: u32) -> Result<(), Flow> {
    let s = &p.sites[si as usize];
    // check order — as in the tree (`assign_index`): value, variable, then indices
    if !regs[val as usize].is_mat() {
        return Err(MError::new(format!(
            "operator = undefined for 'matrix' by '{}' operations (indexed assignment of this type is not supported in mlab v1)",
            regs[val as usize].class_name()
        ))
        .into());
    }
    if let Reg::V(x) = &regs[v as usize] {
        if !matches!(**x, Value::Mat(_)) {
            return Err(MError::new(format!("indexed assignment into a {} is not supported in mlab v1", x.class_name())).into());
        }
    }
    let iargs = if s.direct {
        gather(it, p, regs, &s.args, true)?.into_iter().map(idx_arg).collect::<Result<Vec<_>, _>>()?
    } else {
        take_idx_args(regs, &s.args)?
    };
    let vm = into_mat(std::mem::take(&mut regs[val as usize])).unwrap_or_else(Mat::empty);
    let vi = v as usize;
    match &regs[vi] {
        Reg::U => {
            let mut m = Mat::empty();
            index::assign(&mut m, false, &iargs, &vm, &s.name)?;
            regs[vi] = Reg::from_mat(m);
        }
        Reg::F(_) | Reg::L(_) => {
            let mut m = into_mat(regs[vi].clone()).unwrap();
            index::assign(&mut m, true, &iargs, &vm, &s.name)?;
            regs[vi] = Reg::from_mat(m);
        }
        Reg::V(_) => {
            let scalar = match &mut regs[vi] {
                Reg::V(rc) => match Rc::make_mut(rc) {
                    Value::Mat(m) => {
                        index::assign(m, true, &iargs, &vm, &s.name)?;
                        m.rows == 1 && m.cols == 1 && m.im.is_none()
                    }
                    _ => false,
                },
                _ => false,
            };
            if scalar {
                let r = std::mem::take(&mut regs[vi]);
                regs[vi] = Reg::from_value(r.into_value().unwrap());
            }
        }
    }
    Ok(())
}

fn show(it: &mut Interp, name: &str, r: &Reg) {
    match r {
        Reg::V(v) => it.display(name, v),
        Reg::F(x) => it.display(name, &Value::num(*x)),
        Reg::L(b) => it.display(name, &Value::boolean(*b)),
        Reg::U => {}
    }
}

fn exec(it: &mut Interp, prog: &VmProgram, p: &Proto, regs: &mut [Reg], loops: &mut [LoopState]) -> Result<(), Flow> {
    let base = it.end_stack.len();
    let r = exec_inner(it, prog, p, regs, loops);
    if r.is_err() {
        it.end_stack.truncate(base);
    }
    r
}

fn exec_inner(it: &mut Interp, prog: &VmProgram, p: &Proto, regs: &mut [Reg], loops: &mut [LoopState]) -> Result<(), Flow> {
    let code = &p.code[..];
    let mut pc = 0usize;
    let mut multi: Vec<Reg> = Vec::new();
    loop {
        let op = code[pc];
        pc += 1;
        match op {
            Op::LoadF(d, x) => regs[d as usize] = Reg::F(x),
            Op::LoadK(d, k) => regs[d as usize] = Reg::V(p.consts[k as usize].clone()),
            Op::GetVar(d, v) => {
                let r = match &regs[v as usize] {
                    Reg::U => Reg::from_value(resolve_undef(it, p, v)?),
                    r => r.clone(),
                };
                regs[d as usize] = r;
            }
            Op::Move(d, s) => regs[d as usize] = regs[s as usize].clone(),
            Op::EndVal(d) => {
                let c = it.end_stack.last().ok_or_else(|| MError::new("'end': nonconformant arguments"))?;
                let v = if c.nargs == 1 {
                    c.rows * c.cols
                } else if c.pos == 0 {
                    c.rows
                } else if c.pos == 1 {
                    c.cols
                } else {
                    1
                };
                regs[d as usize] = Reg::F(v as f64);
            }
            Op::Bin(o, d, a, b) => {
                if let (Some(x), Some(y)) = (regs[a as usize].real(), regs[b as usize].real()) {
                    if let Some(r) = scalar_bin(o, x, y) {
                        regs[d as usize] = r;
                        continue;
                    }
                }
                slow_bin(it, p, regs, o, d, a, b)?;
            }
            Op::BinK(o, d, a, k) => {
                if let Some(x) = regs[a as usize].real() {
                    if let Some(r) = scalar_bin(o, x, k) {
                        regs[d as usize] = r;
                        continue;
                    }
                }
                slow_bin_k(it, p, regs, o, d, a, k)?;
            }
            Op::KBin(o, d, k, b) => {
                if let Some(y) = regs[b as usize].real() {
                    if let Some(r) = scalar_bin(o, k, y) {
                        regs[d as usize] = r;
                        continue;
                    }
                }
                slow_kbin(it, p, regs, o, d, k, b)?;
            }
            Op::AddK(d, a, k) => {
                if let Reg::F(x) = regs[a as usize] {
                    regs[d as usize] = Reg::F(x + k);
                    continue;
                }
                if let Reg::L(b) = regs[a as usize] {
                    regs[d as usize] = Reg::F(if b { 1.0 } else { 0.0 } + k);
                    continue;
                }
                slow_bin_k(it, p, regs, BinOp::Add, d, a, k)?;
            }
            Op::AddBug(d, a, b) => {
                if let (Some(x), Some(y)) = (regs[a as usize].real(), regs[b as usize].real()) {
                    regs[d as usize] = Reg::F(x + y + 1.0);
                    continue;
                }
                slow_bin(it, p, regs, BinOp::Add, d, a, b)?;
            }
            Op::Un(o, d, s) => {
                let r = match (o, &regs[s as usize]) {
                    (UnOp::Neg, Reg::F(x)) => Reg::F(-*x),
                    (UnOp::Plus, Reg::F(x)) => Reg::F(*x),
                    (UnOp::Not, Reg::F(x)) => Reg::L(*x == 0.0),
                    (UnOp::Neg, Reg::L(b)) => Reg::F(-(if *b { 1.0 } else { 0.0 })),
                    (UnOp::Plus, Reg::L(b)) => Reg::F(if *b { 1.0 } else { 0.0 }),
                    (UnOp::Not, Reg::L(b)) => Reg::L(!*b),
                    _ => {
                        let v = {
                            let v = opnd(it, p, regs, s)?;
                            ops::unary(o, &v)?
                        };
                        Reg::from_value(v)
                    }
                };
                regs[d as usize] = r;
            }
            Op::Transp(conj, d, s) => {
                let r = match &regs[s as usize] {
                    Reg::F(x) => Reg::F(*x),
                    Reg::L(b) => Reg::L(*b),
                    _ => {
                        let v = {
                            let v = opnd(it, p, regs, s)?;
                            ops::transpose(&v, conj)?
                        };
                        Reg::from_value(v)
                    }
                };
                regs[d as usize] = r;
            }
            Op::Range(d, a, s, b) => {
                let r = for_range_slow(it, p, regs, a, s, b)?;
                regs[d as usize] = r;
            }
            Op::Concat(d, k) => {
                let shape = &p.concats[k as usize];
                let rows: Vec<Vec<Value>> = shape.iter().map(|row| take_values(regs, row)).collect();
                regs[d as usize] = Reg::from_value(ops::concat(rows)?);
            }
            Op::ScBool(d, s, and) => {
                let b = match regs[s as usize] {
                    Reg::F(x) => x != 0.0,
                    Reg::L(b) => b,
                    _ => sc_bool_slow(it, p, regs, s, and)?,
                };
                regs[d as usize] = Reg::L(b);
            }
            Op::Jmp(t) => pc = t as usize,
            Op::JmpFalse(r, t) => {
                let b = match regs[r as usize] {
                    Reg::F(x) => x != 0.0,
                    Reg::L(b) => b,
                    _ => slow_truthy(it, p, regs, r)?,
                };
                if !b {
                    pc = t as usize;
                }
            }
            Op::JmpIfL(r, want, t) => {
                if let Reg::L(b) = regs[r as usize] {
                    if b == want {
                        pc = t as usize;
                    }
                }
            }
            Op::JmpCmp(o, a, b, t) => {
                let res = match (regs[a as usize].real(), regs[b as usize].real()) {
                    (Some(x), Some(y)) => scalar_cmp(o, x, y),
                    _ => slow_cmp(it, p, regs, o, a, Ok(b))?,
                };
                if !res {
                    pc = t as usize;
                }
            }
            Op::JmpCmpK(o, a, k, t) => {
                let res = match regs[a as usize].real() {
                    Some(x) => scalar_cmp(o, x, k),
                    None => slow_cmp(it, p, regs, o, a, Err(k))?,
                };
                if !res {
                    pc = t as usize;
                }
            }
            Op::ForRange(slot, var, a, s, b, exit) => {
                let st = if s == NO { Some(1.0) } else { regs[s as usize].real() };
                match (regs[a as usize].real(), st, regs[b as usize].real()) {
                    (Some(x), Some(st), Some(y)) => {
                        let spec = ops::range_spec(x, st, y);
                        if spec.n == 0 {
                            pc = exit as usize;
                        } else {
                            regs[var as usize] = Reg::F(spec.at(0));
                            loops[slot as usize] = LoopState::Range { spec, k: 0 };
                        }
                    }
                    _ => {
                        let v = for_range_slow(it, p, regs, a, s, b)?;
                        if !start_cols(regs, loops, slot, var, v) {
                            pc = exit as usize;
                        }
                    }
                }
            }
            Op::ForGen(slot, var, t, exit) => {
                let v = std::mem::take(&mut regs[t as usize]);
                if !start_cols(regs, loops, slot, var, v) {
                    pc = exit as usize;
                }
            }
            Op::ForNext(slot, var, body) => {
                let sl = &mut loops[slot as usize];
                let next = match sl {
                    LoopState::Range { spec, k } => {
                        *k += 1;
                        if *k < spec.n { Some(Reg::F(spec.at(*k))) } else { None }
                    }
                    LoopState::Cols { m, j, n } => {
                        *j += 1;
                        if *j < *n { Some(col_of(m, *j)) } else { None }
                    }
                    LoopState::Single | LoopState::Idle => None,
                };
                match next {
                    Some(r) => {
                        regs[var as usize] = r;
                        pc = body as usize;
                    }
                    None => *sl = LoopState::Idle,
                }
            }
            Op::TypeSwitch(v, t) => {
                if let Reg::V(x) = &regs[v as usize] {
                    if !matches!(**x, Value::Mat(_) | Value::Func(_)) {
                        pc = t as usize;
                    }
                }
            }
            Op::TypeIsMat(v, t) => {
                if !regs[v as usize].is_mat() {
                    pc = t as usize;
                }
            }
            Op::PushEnd(v, pos, nargs) => {
                let (rows, cols) = match &regs[v as usize] {
                    Reg::F(_) | Reg::L(_) => (1, 1),
                    Reg::V(x) => match &**x {
                        Value::Mat(m) => (m.rows, m.cols),
                        _ => (0, 0),
                    },
                    Reg::U => (0, 0),
                };
                it.end_stack.push(EndCtx { rows, cols, pos: pos as usize, nargs: nargs as usize });
            }
            Op::PopEnd => {
                it.end_stack.pop();
            }
            Op::CheckIdx(v, t) => {
                if regs[v as usize].is_mat() && !regs[t as usize].is_mat() {
                    return Err(MError::new(format!("subscript indices must be numeric or logical, not {}", regs[t as usize].class_name())).into());
                }
            }
            Op::CheckArg(t) => {
                if !regs[t as usize].is_mat() {
                    return Err(MError::new(format!("subscript indices must be numeric or logical, not {}", regs[t as usize].class_name())).into());
                }
            }
            Op::Index(d, v, si) => {
                let s = &p.sites[si as usize];
                if let Reg::V(x) = &regs[v as usize] {
                    if let Value::Mat(m) = &**x {
                        if m.im.is_none() && m.class != Class::Char {
                            let k = match s.args.len() {
                                1 => match regs[s.args[0] as usize] {
                                    Reg::F(i) if i >= 1.0 && i == i.trunc() && i <= m.re.len() as f64 => Some(i as usize - 1),
                                    _ => None,
                                },
                                2 => match (&regs[s.args[0] as usize], &regs[s.args[1] as usize]) {
                                    (Reg::F(i), Reg::F(j))
                                        if *i >= 1.0
                                            && *i == i.trunc()
                                            && *i <= m.rows as f64
                                            && *j >= 1.0
                                            && *j == j.trunc()
                                            && *j <= m.cols as f64 =>
                                    {
                                        Some((*j as usize - 1) * m.rows + (*i as usize - 1))
                                    }
                                    _ => None,
                                },
                                _ => None,
                            };
                            if let Some(k) = k {
                                let e = m.re[k];
                                let r = if m.class == Class::Double {
                                    Some(Reg::F(e))
                                } else if e.to_bits() == 0 {
                                    Some(Reg::L(false))
                                } else if e.to_bits() == 1f64.to_bits() {
                                    Some(Reg::L(true))
                                } else {
                                    None
                                };
                                if let Some(r) = r {
                                    regs[d as usize] = r;
                                    continue;
                                }
                            }
                        }
                    }
                }
                index_slow(it, prog, p, regs, d, v, si)?;
            }
            Op::Call(d, si) => {
                let s = &p.sites[si as usize];
                let r = do_call1(it, prog, p, regs, s)?;
                regs[d as usize] = need_value(r, s.need)?;
            }
            Op::CallMulti(si) => {
                multi = do_call(it, prog, p, regs, &p.sites[si as usize])?;
            }
            Op::TakeMulti(v, k) => match multi.get(k as usize) {
                Some(x) => regs[v as usize] = x.clone(),
                None => return Err(MError::new(format!("element number {} undefined in return list", k + 1)).into()),
            },
            Op::DynEval(d, di) => dyn_eval(it, p, regs, d, di)?,
            Op::AsgCheck(v, val, _) => {
                if !regs[val as usize].is_mat() {
                    return Err(MError::new(format!(
                        "operator = undefined for 'matrix' by '{}' operations (indexed assignment of this type is not supported in mlab v1)",
                        regs[val as usize].class_name()
                    ))
                    .into());
                }
                if let Reg::V(x) = &regs[v as usize] {
                    if !matches!(**x, Value::Mat(_)) {
                        return Err(MError::new(format!("indexed assignment into a {} is not supported in mlab v1", x.class_name())).into());
                    }
                }
            }
            Op::ReadCheck(v, nm) => {
                if !regs[v as usize].is_mat() {
                    return Err(MError::new(format!("'{}' undefined", p.strs[nm as usize])).into());
                }
            }
            Op::IndexAsg(v, val, si) => {
                let s = &p.sites[si as usize];
                if let Some(x) = regs[val as usize].real() {
                    let done = match s.args.len() {
                        1 => match regs[s.args[0] as usize] {
                            Reg::F(i) => asg_fast(regs, v, x, i, None),
                            _ => false,
                        },
                        2 => match (&regs[s.args[0] as usize], &regs[s.args[1] as usize]) {
                            (Reg::F(i), Reg::F(j)) => {
                                let (i, j) = (*i, *j);
                                asg_fast(regs, v, x, i, Some(j))
                            }
                            _ => false,
                        },
                        _ => false,
                    };
                    if done {
                        continue;
                    }
                }
                index_asg_slow(it, p, regs, v, val, si)?;
            }
            Op::Compound(o, v, val, nm) => {
                if !regs[v as usize].is_def() {
                    return Err(MError::new(format!("'{}' undefined", p.strs[nm as usize])).into());
                }
                if let (Some(x), Some(y)) = (regs[v as usize].real(), regs[val as usize].real()) {
                    if let Some(r) = scalar_bin(o, x, y) {
                        regs[v as usize] = r;
                        continue;
                    }
                }
                slow_bin(it, p, regs, o, v, v, val)?;
            }
            Op::GetField(d, s, nm) => {
                let name = &p.strs[nm as usize];
                let col = match &regs[s as usize] {
                    Reg::V(x) => match &**x {
                        Value::Table(t) => Some(t.columns[crate::table::var_index(t, name)?].clone()),
                        _ => None,
                    },
                    _ => None,
                };
                match col {
                    Some(c) => regs[d as usize] = Reg::from_value(c),
                    None => {
                        return Err(MError::new(format!(
                            "invalid use of a N_-D array / field access '.{name}': structs are not supported in mlab v1"
                        ))
                        .into());
                    }
                }
            }
            Op::MakeClosure(d, k) => {
                let c = &p.closures[k as usize];
                let captured: Vec<(String, Value)> =
                    c.caps.iter().filter_map(|(n, r)| regs[*r as usize].to_value().map(|v| (n.clone(), v))).collect();
                let f = Func::Anon { params: c.params.clone(), body: c.body.clone(), captured };
                regs[d as usize] = Reg::V(Rc::new(Value::Func(Rc::new(f))));
            }
            Op::Display(r, nm) => show(it, &p.strs[nm as usize], &regs[r as usize]),
            Op::ShowOrCall(v, si, print) => {
                if regs[v as usize].is_def() {
                    if print {
                        show(it, &p.reg_names[v as usize], &regs[v as usize]);
                    }
                } else {
                    let x = do_call1(it, prog, p, regs, &p.sites[si as usize])?;
                    if x.is_def() {
                        if print {
                            show(it, "ans", &x);
                        }
                        regs[p.ans_reg as usize] = x;
                    }
                }
            }
            Op::SetAns(t, print) => {
                let x = std::mem::take(&mut regs[t as usize]);
                if x.is_def() {
                    if print {
                        show(it, "ans", &x);
                    }
                    regs[p.ans_reg as usize] = x;
                }
            }
            Op::Fallback(i) => {
                let fb = &p.fallbacks[i as usize];
                it.vm_stats.fallback_runs += 1;
                sync_out(it, &fb.sync, regs);
                let r = it.exec(&fb.stmt);
                sync_in(it, &fb.sync, regs);
                match r {
                    Ok(()) => {}
                    Err(Flow::Break) => {
                        if fb.brk == NOJ {
                            return Ok(());
                        }
                        pc = fb.brk as usize;
                    }
                    Err(Flow::Continue) => {
                        if fb.cont == NOJ {
                            return Ok(());
                        }
                        pc = fb.cont as usize;
                    }
                    Err(Flow::Return) => return Ok(()),
                    Err(e) => return Err(e),
                }
            }
            Op::Ret => return Ok(()),
        }
    }
}

/// Bytecode listing (for debugging and documentation): `mlab vmdump file.m`.
pub fn dump(p: &Proto) -> String {
    let mut s = format!("; {} — registers {}, variables {}, loops {}, instructions {} (fallback {})\n", p.name, p.nregs, p.nvars, p.nloops, p.native, p.fallback);
    for (i, op) in p.code.iter().enumerate() {
        s.push_str(&format!("{i:4}  {op:?}\n"));
    }
    s
}
