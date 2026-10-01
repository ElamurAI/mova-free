//! Interpreter: workspace, statement execution, expression evaluation, indexing, calls,
//! output. Errors — `error: …`, warnings — `warning: …` (in the console — to stderr).

use crate::ast::*;
use crate::builtins::Registry;
use crate::display::{self, Format};
use crate::index::{self, IdxArg};
use crate::ops;
use crate::rng::Rng;
use crate::value::{Class, Func, Mat, Value};
use std::collections::{HashMap, HashSet};
use std::io::Write;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct MError {
    pub msg: String,
    pub identifier: String,
}

impl MError {
    pub fn new(msg: impl Into<String>) -> MError {
        MError { msg: msg.into(), identifier: String::new() }
    }
    pub fn with_id(mut self, id: impl Into<String>) -> MError {
        self.identifier = id.into();
        self
    }
}

pub enum Flow {
    Err(MError),
    Break,
    Continue,
    Return,
}

impl From<MError> for Flow {
    fn from(e: MError) -> Flow {
        Flow::Err(e)
    }
}

pub type R<T> = Result<T, Flow>;

pub enum Sink {
    /// Merged stream (scenario suite, tests).
    Capture(String),
    /// Console: output to stdout, warnings and errors — to stderr.
    Console,
}

pub(crate) struct EndCtx {
    pub(crate) rows: usize,
    pub(crate) cols: usize,
    pub(crate) pos: usize,
    pub(crate) nargs: usize,
}

pub struct Interp {
    pub(crate) frames: Vec<HashMap<String, Value>>,
    pub(crate) functions: HashMap<String, Rc<FunctionDef>>,
    pub(crate) registry: Rc<Registry>,
    sink: Sink,
    pub fmt: Format,
    pub rng: Rng,
    pub last_error: String,
    pub warnings_on: bool,
    /// Negative controls: `Some("sum")` breaks the builtin sum (+1 to the first element of the result);
    /// `Some("gate:mldivide")` / `gate:inv` / `gate:eig` / `gate:svd` injects a wrong answer before the gate.
    pub fault: Option<String>,
    pub(crate) end_stack: Vec<EndCtx>,
    pub(crate) depth: usize,
    pub tic: Option<std::time::Instant>,
    /// Counter of gate triggers (for tests and the report).
    pub gate_warnings: usize,
    /// v2: names of the variable arguments of the last call (`table(x, y)` takes column names from here).
    pub arg_names: Vec<Option<String>>,
    /// VM v1: `run` compiles the program to register bytecode (`vm.rs`); whatever does not compile — tree-walker.
    pub vm_mode: bool,
    pub(crate) vm: Option<Rc<crate::vm::VmProgram>>,
    pub(crate) vm_pool: Vec<crate::vm::FrameBuf>,
    /// VM v1: how many instructions were compiled and how many times the fallback to the tree fired.
    pub vm_stats: crate::vm::Stats,
    /// File system for file builtins (`readtable`, `writetable`, `fileread`…). By default — in memory
    /// (`vfs::Vfs`): foreign and generated code does not see the real FS. `--fs dir:<dir>` / `--fs real` — only explicitly.
    pub fs: Box<dyn vfs::FileSystem>,
}

impl Interp {
    pub fn new(sink: Sink) -> Interp {
        Interp {
            frames: vec![HashMap::new()],
            functions: HashMap::new(),
            registry: Rc::new(Registry::with_defaults()),
            sink,
            fmt: Format { long: false },
            rng: Rng::new(crate::rng::DEFAULT_SEED),
            last_error: String::new(),
            warnings_on: true,
            fault: None,
            end_stack: Vec::new(),
            depth: 0,
            tic: None,
            gate_warnings: 0,
            arg_names: Vec::new(),
            vm_mode: false,
            vm: None,
            vm_pool: Vec::new(),
            vm_stats: crate::vm::Stats::default(),
            fs: Box::new(vfs::Vfs::new()),
        }
    }
    /// Replace the file system (`--fs`).
    pub fn with_fs(mut self, fs: Box<dyn vfs::FileSystem>) -> Interp {
        self.fs = fs;
        self
    }
    pub fn capture() -> Interp {
        Interp::new(Sink::Capture(String::new()))
    }
    pub fn console() -> Interp {
        Interp::new(Sink::Console)
    }
    pub fn take_output(&mut self) -> String {
        match &mut self.sink {
            Sink::Capture(s) => std::mem::take(s),
            Sink::Console => String::new(),
        }
    }

    // ---------- output ----------

    pub fn out(&mut self, s: &str) {
        match &mut self.sink {
            Sink::Capture(buf) => buf.push_str(s),
            Sink::Console => {
                let mut o = std::io::stdout().lock();
                let _ = o.write_all(s.as_bytes());
                let _ = o.flush();
            }
        }
    }
    pub(crate) fn err_out(&mut self, s: &str) {
        match &mut self.sink {
            Sink::Capture(buf) => buf.push_str(s),
            Sink::Console => {
                let _ = std::io::stdout().lock().flush();
                let mut e = std::io::stderr().lock();
                let _ = e.write_all(s.as_bytes());
            }
        }
    }
    pub fn warn(&mut self, msg: &str) {
        if self.warnings_on {
            self.err_out(&format!("warning: {msg}\n"));
        }
    }
    /// Result-checking gate warning — always visible (the gate is not silenced by `warning off`).
    pub fn gate_warn(&mut self, msg: &str) {
        self.gate_warnings += 1;
        self.err_out(&format!("warning: {msg}\n"));
    }
    pub fn display(&mut self, name: &str, v: &Value) {
        let s = display::format_named(name, v, self.fmt);
        self.out(&s);
    }

    // ---------- variables ----------

    pub fn get_var(&self, name: &str) -> Option<&Value> {
        self.frames.last().and_then(|f| f.get(name))
    }
    pub fn set_var(&mut self, name: &str, v: Value) {
        self.frames.last_mut().unwrap().insert(name.to_string(), v);
    }
    pub fn clear_vars(&mut self, names: &[String]) {
        let f = self.frames.last_mut().unwrap();
        if names.is_empty() || names.iter().any(|n| n == "all" || n == "-all") {
            f.clear();
        } else {
            for n in names {
                f.remove(n);
            }
        }
    }
    pub fn is_function_name(&self, name: &str) -> bool {
        self.functions.contains_key(name) || self.registry.get(name).is_some()
    }

    // ---------- running ----------

    /// Registers the program's functions (for `mlab vmdump`).
    pub fn load_functions(&mut self, prog: &Program) {
        for f in &prog.functions {
            self.functions.insert(f.name.clone(), f.clone());
        }
    }

    /// Parses and executes the program text. Prints `parse error …` / `error: …` to the error stream.
    /// Returns true if execution finished without an error.
    pub fn run(&mut self, src: &str) -> bool {
        let prog = match crate::parser::parse_program(src) {
            Ok(p) => p,
            Err(e) => {
                self.err_out(&format!("{e}\n"));
                return false;
            }
        };
        for f in &prog.functions {
            self.functions.insert(f.name.clone(), f.clone());
        }
        if self.vm_mode {
            return crate::vm::run_program(self, &prog);
        }
        match self.exec_block(&prog.body) {
            Ok(()) | Err(Flow::Return) | Err(Flow::Break) | Err(Flow::Continue) => true,
            Err(Flow::Err(e)) => {
                self.last_error = e.msg.clone();
                self.err_out(&format!("error: {}\n", e.msg));
                false
            }
        }
    }

    fn exec_block(&mut self, stmts: &[Stmt]) -> R<()> {
        for s in stmts {
            self.exec(s)?;
        }
        Ok(())
    }

    pub(crate) fn exec(&mut self, s: &Stmt) -> R<()> {
        match &s.kind {
            StmtKind::Expr { expr, print } => {
                if let Expr::Ident(name) = expr {
                    if let Some(v) = self.get_var(name) {
                        if *print {
                            let v = v.clone();
                            self.display(name, &v);
                        }
                        return Ok(());
                    }
                }
                let vals = self.eval_multi(expr, 0)?;
                if let Some(v) = vals.into_iter().next() {
                    if *print {
                        self.display("ans", &v);
                    }
                    self.set_var("ans", v);
                }
                Ok(())
            }
            StmtKind::Assign { lhs, rhs, print, op } => self.exec_assign(lhs, rhs, *print, *op),
            StmtKind::If { clauses, else_body } => {
                for (cond, body) in clauses {
                    let c = self.eval(cond)?;
                    if ops::truthy(&c)? {
                        return self.exec_block(body);
                    }
                }
                if let Some(b) = else_body {
                    self.exec_block(b)?;
                }
                Ok(())
            }
            StmtKind::For { var, iter, body } => {
                let v = self.eval(iter)?;
                let m = match v {
                    Value::Mat(m) => m,
                    other => {
                        // a single value — a single iteration
                        self.set_var(var, other);
                        return match self.exec_block(body) {
                            Ok(()) | Err(Flow::Break) | Err(Flow::Continue) => Ok(()),
                            Err(e) => Err(e),
                        };
                    }
                };
                if m.rows == 0 {
                    return Ok(());
                }
                for j in 0..m.cols {
                    let col = if m.rows == 1 {
                        let mut c = Mat { rows: 1, cols: 1, re: vec![m.re[j]], im: m.im.as_ref().map(|v| vec![v[j]]), class: m.class };
                        c.narrow();
                        c
                    } else {
                        let rg = j * m.rows..(j + 1) * m.rows;
                        let mut c = Mat { rows: m.rows, cols: 1, re: m.re[rg.clone()].to_vec(), im: m.im.as_ref().map(|v| v[rg].to_vec()), class: m.class };
                        c.narrow();
                        c
                    };
                    self.set_var(var, Value::Mat(col));
                    match self.exec_block(body) {
                        Ok(()) | Err(Flow::Continue) => {}
                        Err(Flow::Break) => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            StmtKind::While { cond, body } => {
                loop {
                    let c = self.eval(cond)?;
                    if !ops::truthy(&c)? {
                        break;
                    }
                    match self.exec_block(body) {
                        Ok(()) | Err(Flow::Continue) => {}
                        Err(Flow::Break) => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(())
            }
            StmtKind::DoUntil { body, cond } => {
                loop {
                    match self.exec_block(body) {
                        Ok(()) | Err(Flow::Continue) => {}
                        Err(Flow::Break) => break,
                        Err(e) => return Err(e),
                    }
                    let c = self.eval(cond)?;
                    if ops::truthy(&c)? {
                        break;
                    }
                }
                Ok(())
            }
            StmtKind::Switch { subject, cases, otherwise } => {
                let sv = self.eval(subject)?;
                for (cv, body) in cases {
                    let hit = match cv {
                        Expr::CellList(items) => {
                            let mut h = false;
                            for it in items {
                                let v = self.eval(it)?;
                                if switch_match(&sv, &v) {
                                    h = true;
                                    break;
                                }
                            }
                            h
                        }
                        e => {
                            let v = self.eval(e)?;
                            switch_match(&sv, &v)
                        }
                    };
                    if hit {
                        return self.exec_block(body);
                    }
                }
                if let Some(b) = otherwise {
                    self.exec_block(b)?;
                }
                Ok(())
            }
            StmtKind::Try { body, ident, catch_body } => match self.exec_block(body) {
                Err(Flow::Err(e)) => {
                    self.last_error = e.msg.clone();
                    if let Some(id) = ident {
                        self.set_var(id, Value::str(&e.msg));
                    }
                    self.exec_block(catch_body)
                }
                other => other,
            },
            StmtKind::Break => Err(Flow::Break),
            StmtKind::Continue => Err(Flow::Continue),
            StmtKind::Return => Err(Flow::Return),
            StmtKind::Command { name, args, print } => {
                let argv: Vec<Value> = args.iter().map(|a| Value::str(a)).collect();
                let vals = self.call_function(name, argv, 0)?;
                if let Some(v) = vals.into_iter().next() {
                    if *print {
                        self.display("ans", &v);
                    }
                    self.set_var("ans", v);
                }
                Ok(())
            }
        }
    }

    fn exec_assign(&mut self, lhs: &[LValue], rhs: &Expr, print: bool, op: Option<BinOp>) -> R<()> {
        if lhs.len() == 1 {
            let mut v = self.eval(rhs)?;
            match &lhs[0] {
                LValue::Tilde => {}
                LValue::Field(name, field) => {
                    if !matches!(self.get_var(name), Some(Value::Table(_))) {
                        return Err(MError::new(format!("invalid use of '{name}.{field}': structs are not supported in mlab v1")).into());
                    }
                    if let Some(op) = op {
                        let cur = match self.get_var(name) {
                            Some(Value::Table(t)) => t.columns[crate::table::var_index(t, field)?].clone(),
                            _ => unreachable!(),
                        };
                        v = ops::binary(self, op, &cur, &v)?;
                    }
                    if let Some(Value::Table(t)) = self.frames.last_mut().unwrap().get_mut(name.as_str()) {
                        crate::table::set_var(Rc::make_mut(t), field, &v)?;
                    }
                    if print {
                        let v = self.get_var(name).unwrap().clone();
                        self.display(name, &v);
                    }
                }
                LValue::Var(name) => {
                    if let Some(op) = op {
                        let cur = self.get_var(name).cloned().ok_or_else(|| MError::new(format!("'{name}' undefined")))?;
                        v = ops::binary(self, op, &cur, &v)?;
                    }
                    self.set_var(name, v);
                    if print {
                        let v = self.get_var(name).unwrap().clone();
                        self.display(name, &v);
                    }
                }
                LValue::Index(name, args) => {
                    if let Some(op) = op {
                        let old = self.index_var_read(name, args)?;
                        v = ops::binary(self, op, &old, &v)?;
                    }
                    self.assign_index(name, args, v)?;
                    if print {
                        let v = self.get_var(name).unwrap().clone();
                        self.display(name, &v);
                    }
                }
            }
            return Ok(());
        }
        let want = lhs.len();
        let vals = self.eval_multi(rhs, want)?;
        for (k, lv) in lhs.iter().enumerate() {
            if matches!(lv, LValue::Tilde) {
                continue;
            }
            let v = match vals.get(k) {
                Some(v) => v.clone(),
                None => return Err(MError::new(format!("element number {} undefined in return list", k + 1)).into()),
            };
            match lv {
                LValue::Var(name) => self.set_var(name, v),
                LValue::Index(name, args) => self.assign_index(name, args, v)?,
                LValue::Field(name, field) => {
                    return Err(MError::new(format!("invalid use of '{name}.{field}': structs are not supported in mlab v1")).into());
                }
                LValue::Tilde => {}
            }
        }
        if print {
            for lv in lhs {
                let name = match lv {
                    LValue::Var(n) | LValue::Index(n, _) => n,
                    LValue::Tilde | LValue::Field(..) => continue,
                };
                let v = self.get_var(name).unwrap().clone();
                self.display(name, &v);
            }
        }
        Ok(())
    }

    fn index_var_read(&mut self, name: &str, args: &[Expr]) -> R<Value> {
        let (rows, cols) = match self.get_var(name) {
            Some(Value::Mat(m)) => (m.rows, m.cols),
            _ => return Err(MError::new(format!("'{name}' undefined")).into()),
        };
        let iargs = self.eval_index_args(args, rows, cols)?;
        match self.get_var(name) {
            Some(Value::Mat(m)) => Ok(Value::Mat(index::read(m, &iargs, name)?)),
            _ => Err(MError::new(format!("'{name}' undefined")).into()),
        }
    }

    fn assign_index(&mut self, name: &str, args: &[Expr], v: Value) -> R<()> {
        let vm = match v {
            Value::Mat(m) => m,
            other => {
                return Err(MError::new(format!(
                    "operator = undefined for '{}' by '{}' operations (indexed assignment of this type is not supported in mlab v1)",
                    "matrix",
                    other.class_name()
                ))
                .into());
            }
        };
        let (defined, rows, cols) = match self.get_var(name) {
            Some(Value::Mat(m)) => (true, m.rows, m.cols),
            Some(other) => {
                return Err(MError::new(format!("indexed assignment into a {} is not supported in mlab v1", other.class_name())).into());
            }
            None => (false, 0, 0),
        };
        let iargs = self.eval_index_args(args, rows, cols)?;
        let frame = self.frames.last_mut().unwrap();
        if !defined {
            let mut m = Mat::empty();
            index::assign(&mut m, false, &iargs, &vm, name)?;
            frame.insert(name.to_string(), Value::Mat(m));
        } else if let Some(Value::Mat(m)) = frame.get_mut(name) {
            index::assign(m, true, &iargs, &vm, name)?;
        }
        Ok(())
    }

    fn eval_index_args(&mut self, args: &[Expr], rows: usize, cols: usize) -> R<Vec<IdxArg>> {
        let nargs = args.len();
        let mut out = Vec::with_capacity(nargs);
        for (pos, a) in args.iter().enumerate() {
            if matches!(a, Expr::Colon) {
                out.push(IdxArg::All);
                continue;
            }
            self.end_stack.push(EndCtx { rows, cols, pos, nargs });
            let r = self.eval(a);
            self.end_stack.pop();
            match r? {
                Value::Mat(m) => {
                    if m.class == Class::Char && m.numel() == 1 && m.re[0] == ':' as u32 as f64 {
                        out.push(IdxArg::All);
                    } else {
                        out.push(IdxArg::Vals(m));
                    }
                }
                other => return Err(MError::new(format!("subscript indices must be numeric or logical, not {}", other.class_name())).into()),
            }
        }
        Ok(out)
    }

    // ---------- expressions ----------

    pub fn eval(&mut self, e: &Expr) -> R<Value> {
        match e {
            Expr::Num(v, _) => Ok(Value::num(*v)),
            Expr::Imag(v, _) => Ok(Value::Mat(Mat::cscalar(0.0, *v))),
            Expr::Str(s, _) => Ok(Value::str(s)),
            Expr::Ident(name) => {
                if let Some(v) = self.get_var(name) {
                    return Ok(v.clone());
                }
                let vals = self.call_function(name, vec![], 1)?;
                vals.into_iter().next().ok_or_else(|| MError::new("value on right hand side of assignment undefined").into())
            }
            Expr::Colon => Ok(Value::str(":")),
            Expr::End => {
                let c = self.end_stack.last().ok_or_else(|| MError::new("'end': nonconformant arguments"))?;
                let v = if c.nargs == 1 {
                    c.rows * c.cols
                } else if c.pos == 0 {
                    c.rows
                } else if c.pos == 1 {
                    c.cols
                } else {
                    1
                };
                Ok(Value::num(v as f64))
            }
            Expr::Paren(e) => self.eval(e),
            Expr::Unary(op, e) => {
                let v = self.eval(e)?;
                Ok(ops::unary(*op, &v)?)
            }
            Expr::Postfix(op, e) => {
                let v = self.eval(e)?;
                Ok(ops::transpose(&v, *op == PostOp::CTranspose)?)
            }
            Expr::Binary(BinOp::AndAnd, l, r) | Expr::Binary(BinOp::OrOr, l, r) => {
                let is_and = matches!(e, Expr::Binary(BinOp::AndAnd, ..));
                let lv = self.eval(l)?;
                let lb = self.sc_bool(&lv, if is_and { "&&" } else { "||" })?;
                if is_and && !lb {
                    return Ok(Value::boolean(false));
                }
                if !is_and && lb {
                    return Ok(Value::boolean(true));
                }
                let rv = self.eval(r)?;
                let rb = self.sc_bool(&rv, if is_and { "&&" } else { "||" })?;
                Ok(Value::boolean(rb))
            }
            Expr::Binary(op, l, r) => {
                let lv = self.eval(l)?;
                let rv = self.eval(r)?;
                Ok(ops::binary(self, *op, &lv, &rv)?)
            }
            Expr::Range(a, s, b) => {
                let av = self.eval(a)?;
                let sv = match s {
                    Some(s) => Some(self.eval(s)?),
                    None => None,
                };
                let bv = self.eval(b)?;
                Ok(ops::range(&av, sv.as_ref(), &bv)?)
            }
            Expr::Index(..) => {
                let vals = self.eval_multi(e, 1)?;
                vals.into_iter().next().ok_or_else(|| MError::new("value on right hand side of assignment undefined").into())
            }
            Expr::Matrix(rows) => {
                let mut vrows = Vec::with_capacity(rows.len());
                for row in rows {
                    let mut vr = Vec::with_capacity(row.len());
                    for x in row {
                        vr.push(self.eval(x)?);
                    }
                    vrows.push(vr);
                }
                Ok(ops::concat(vrows)?)
            }
            Expr::CellList(items) => {
                // v2: {'a', 'b'} — a list of strings becomes a string array; other cell arrays are not supported
                let mut data = Vec::with_capacity(items.len());
                for x in items {
                    match self.eval(x)? {
                        Value::Mat(m) if m.class == Class::Char && m.rows <= 1 => data.push(Some(m.to_string_lossy())),
                        Value::Str(s) if s.data.len() == 1 => data.push(s.data[0].clone()),
                        _ => return Err(MError::new("cell arrays are not supported in mlab v1 (only {'text', ...} lists of strings)").into()),
                    }
                }
                Ok(Value::Str(Rc::new(crate::value::StrArr::row(data))))
            }
            Expr::AnonFn(params, body) => {
                let mut names = HashSet::new();
                collect_idents(body, &mut names);
                let mut captured = Vec::new();
                let mut sorted: Vec<&String> = names.iter().collect();
                sorted.sort();
                for n in sorted {
                    if params.contains(n) {
                        continue;
                    }
                    if let Some(v) = self.get_var(n) {
                        captured.push((n.clone(), v.clone()));
                    }
                }
                Ok(Value::Func(Rc::new(Func::Anon { params: params.clone(), body: body.clone(), captured })))
            }
            Expr::FuncHandle(name) => Ok(Value::Func(Rc::new(Func::Named(name.clone())))),
            Expr::Field(base, name) => match self.eval(base)? {
                Value::Table(t) => Ok(t.columns[crate::table::var_index(&t, name)?].clone()),
                _ => Err(MError::new(format!("invalid use of a N_-D array / field access '.{name}': structs are not supported in mlab v1")).into()),
            },
        }
    }

    fn sc_bool(&self, v: &Value, op: &str) -> R<bool> {
        match v {
            Value::Mat(m) => {
                if m.is_empty() {
                    return Err(MError::new(format!("invalid conversion from empty value to real scalar (operator {op})")).into());
                }
                if !m.is_scalar() {
                    return Err(MError::new(format!(
                        "binary operator '{op}': operands must be convertible to logical scalar values (got {}x{})",
                        m.rows, m.cols
                    ))
                    .into());
                }
                Ok(m.re[0] != 0.0 || m.im_at(0) != 0.0)
            }
            other => Err(MError::new(format!("binary operator '{op}' not implemented for '{}'", other.class_name())).into()),
        }
    }

    /// Evaluation with multiple results (function calls); other expressions give one.
    pub fn eval_multi(&mut self, e: &Expr, nargout: usize) -> R<Vec<Value>> {
        match e {
            Expr::Ident(name) if self.get_var(name).is_none() => self.call_function(name, vec![], nargout),
            Expr::Index(base, args) => {
                if let Expr::Ident(name) = &**base {
                    match self.get_var(name) {
                        Some(Value::Mat(m)) => {
                            let (rows, cols) = (m.rows, m.cols);
                            let iargs = self.eval_index_args(args, rows, cols)?;
                            if let Some(Value::Mat(m)) = self.get_var(name) {
                                return Ok(vec![Value::Mat(index::read(m, &iargs, name)?)]);
                            }
                            unreachable!()
                        }
                        Some(Value::Func(f)) => {
                            let f = f.clone();
                            let argv = self.eval_args(args)?;
                            return self.call_func_value(&f, argv, nargout);
                        }
                        Some(Value::Table(t)) => {
                            let t = t.clone();
                            return Ok(vec![self.index_table(&t, args, name)?]);
                        }
                        Some(Value::Str(s)) => {
                            let s = s.clone();
                            return Ok(vec![self.index_str(&s, args, name)?]);
                        }
                        Some(other) => {
                            return Err(MError::new(format!("indexing a {} is not supported in mlab v1", other.class_name())).into());
                        }
                        None => {
                            let argv = self.eval_args(args)?;
                            self.arg_names = args.iter().map(|a| if let Expr::Ident(n) = a { Some(n.clone()) } else { None }).collect();
                            let r = self.call_function(name, argv, nargout);
                            self.arg_names.clear();
                            return r;
                        }
                    }
                }
                let bv = self.eval(base)?;
                match bv {
                    Value::Mat(m) => {
                        let iargs = self.eval_index_args(args, m.rows, m.cols)?;
                        Ok(vec![Value::Mat(index::read(&m, &iargs, "index")?)])
                    }
                    Value::Func(f) => {
                        let argv = self.eval_args(args)?;
                        self.call_func_value(&f, argv, nargout)
                    }
                    Value::Table(t) => Ok(vec![self.index_table(&t, args, "index")?]),
                    Value::Str(s) => Ok(vec![self.index_str(&s, args, "index")?]),
                    other => Err(MError::new(format!("indexing a {} is not supported in mlab v1", other.class_name())).into()),
                }
            }
            _ => Ok(vec![self.eval(e)?]),
        }
    }

    /// v2: `T(rows, vars)` — rows by number/mask/`end`/`:`, variables by name, list of names, numbers.
    fn index_table(&mut self, t: &Rc<crate::value::Table>, args: &[Expr], name: &str) -> R<Value> {
        use crate::table as tb;
        if args.len() != 2 {
            return Err(MError::new(format!("{name}: table subscripts must be T(rows, vars)")).into());
        }
        let (h, w) = (tb::height(t), t.names.len());
        let mut sel: Vec<Option<Value>> = Vec::with_capacity(2);
        for (pos, a) in args.iter().enumerate() {
            if matches!(a, Expr::Colon) {
                sel.push(None);
                continue;
            }
            self.end_stack.push(EndCtx { rows: h, cols: w, pos, nargs: 2 });
            let r = self.eval(a);
            self.end_stack.pop();
            let v = r?;
            let all = matches!(&v, Value::Mat(m) if m.class == Class::Char && m.to_string_lossy() == ":");
            sel.push(if all { None } else { Some(v) });
        }
        let vars: Vec<usize> = match &sel[1] {
            None => (0..w).collect(),
            Some(v) => tb::var_list(t, v)?,
        };
        let rows: Vec<usize> = match &sel[0] {
            None => (0..h).collect(),
            Some(Value::Mat(m)) if m.class != Class::Char => {
                let (idx, _) = index::to_indices(&IdxArg::Vals(m.clone()), h, name, 0, 2)?;
                if let Some(&bad) = idx.iter().find(|&&i| i >= h) {
                    return Err(MError::new(format!("{name}({},_): out of bound {h} (table has {h} rows)", bad + 1)).into());
                }
                idx
            }
            Some(other) => return Err(MError::new(format!("{name}: table row subscripts must be numeric or logical, not {}", other.class_name())).into()),
        };
        Ok(Value::Table(Rc::new(tb::take_rows(&tb::select_vars(t, &vars), &rows))))
    }

    /// v2: string array indexing — the same semantics as for matrices (via a matrix of positions).
    fn index_str(&mut self, s: &Rc<crate::value::StrArr>, args: &[Expr], name: &str) -> R<Value> {
        let pos = Mat::new(s.rows, s.cols, (0..s.data.len()).map(|k| k as f64).collect());
        let iargs = self.eval_index_args(args, s.rows, s.cols)?;
        let r = index::read(&pos, &iargs, name)?;
        let data = r.re.iter().map(|&k| s.data[k as usize].clone()).collect();
        Ok(Value::Str(Rc::new(crate::value::StrArr { rows: r.rows, cols: r.cols, data })))
    }

    fn eval_args(&mut self, args: &[Expr]) -> R<Vec<Value>> {
        let mut out = Vec::with_capacity(args.len());
        for a in args {
            out.push(self.eval(a)?);
        }
        Ok(out)
    }

    // ---------- calls ----------

    pub fn call_function(&mut self, name: &str, args: Vec<Value>, nargout: usize) -> R<Vec<Value>> {
        if let Some(f) = self.functions.get(name).cloned() {
            return self.call_user(&f, args, nargout);
        }
        if let Some(b) = self.registry.get(name) {
            let mut vals = b(self, &args, nargout)?;
            if self.fault.as_deref() == Some(name) {
                // negative control: broken builtin — +1 to the first element of the first result
                if let Some(Value::Mat(m)) = vals.get_mut(0) {
                    if let Some(x) = m.re.first_mut() {
                        *x += 1.0;
                        m.class = Class::Double;
                    }
                }
            }
            return Ok(vals);
        }
        Err(MError::new(format!("'{name}' undefined")).into())
    }

    /// Calling a function value (for builtins that accept handles: integral, fzero, arrayfun…).
    pub fn call_value(&mut self, f: &Value, args: Vec<Value>, nargout: usize) -> Result<Vec<Value>, MError> {
        match f {
            Value::Func(func) => match self.call_func_value(&func.clone(), args, nargout) {
                Ok(v) => Ok(v),
                Err(Flow::Err(e)) => Err(e),
                Err(_) => Ok(vec![]),
            },
            Value::Mat(m) if m.class == Class::Char => {
                let name = m.to_string_lossy();
                match self.call_function(&name, args, nargout) {
                    Ok(v) => Ok(v),
                    Err(Flow::Err(e)) => Err(e),
                    Err(_) => Ok(vec![]),
                }
            }
            other => Err(MError::new(format!("expected a function handle, got {}", other.class_name()))),
        }
    }

    /// Convenience: a function call with one result.
    pub fn call1(&mut self, f: &Value, args: Vec<Value>) -> Result<Value, MError> {
        self.call_value(f, args, 1)?
            .into_iter()
            .next()
            .ok_or_else(|| MError::new("function returned no value"))
    }

    pub(crate) fn call_func_value(&mut self, f: &Rc<Func>, args: Vec<Value>, nargout: usize) -> R<Vec<Value>> {
        match &**f {
            Func::Named(name) => self.call_function(name, args, nargout),
            Func::Anon { params, body, captured } => {
                if args.len() > params.len() {
                    return Err(MError::new("@<anonymous>: function called with too many inputs").into());
                }
                self.enter()?;
                let mut frame: HashMap<String, Value> = captured.iter().cloned().collect();
                for (p, a) in params.iter().zip(args) {
                    if p != "~" {
                        frame.insert(p.clone(), a);
                    }
                }
                self.frames.push(frame);
                let saved_end = std::mem::take(&mut self.end_stack);
                let r = self.eval_multi(body, nargout);
                self.end_stack = saved_end;
                self.frames.pop();
                self.depth -= 1;
                r
            }
        }
    }

    pub(crate) fn enter(&mut self) -> R<()> {
        self.depth += 1;
        if self.depth > 256 {
            self.depth -= 1;
            return Err(MError::new("max_recursion_depth exceeded").into());
        }
        Ok(())
    }

    fn call_user(&mut self, f: &Rc<FunctionDef>, args: Vec<Value>, nargout: usize) -> R<Vec<Value>> {
        if args.len() > f.params.len() {
            return Err(MError::new(format!("{}: function called with too many inputs", f.name)).into());
        }
        if nargout > f.outputs.len() {
            return Err(MError::new(format!("{}: function called with too many outputs", f.name)).into());
        }
        if let Some(vp) = self.vm.clone() {
            if let Some(&idx) = vp.index.get(f.name.as_str()) {
                return crate::vm::call_from_tree(self, &vp, idx, args, nargout);
            }
        }
        self.enter()?;
        let nargin = args.len();
        let mut frame = HashMap::new();
        for (p, a) in f.params.iter().zip(args) {
            if p != "~" {
                frame.insert(p.clone(), a);
            }
        }
        frame.insert("nargin".to_string(), Value::num(nargin as f64));
        frame.insert("nargout".to_string(), Value::num(nargout as f64));
        self.frames.push(frame);
        let saved_end = std::mem::take(&mut self.end_stack);
        let r = self.exec_block(&f.body);
        self.end_stack = saved_end;
        let frame = self.frames.pop().unwrap();
        self.depth -= 1;
        match r {
            Ok(()) | Err(Flow::Return) | Err(Flow::Break) | Err(Flow::Continue) => {}
            Err(e) => return Err(e),
        }
        let mut outs = Vec::new();
        for (k, o) in f.outputs.iter().enumerate() {
            if k >= nargout.max(1) {
                break;
            }
            match frame.get(o) {
                Some(v) => outs.push(v.clone()),
                None => {
                    if k < nargout {
                        return Err(MError::new(format!("value on right hand side of assignment undefined: '{o}' undefined in {}", f.name)).into());
                    }
                    break;
                }
            }
        }
        Ok(outs)
    }
}

fn switch_match(subject: &Value, case: &Value) -> bool {
    match (subject, case) {
        (Value::Mat(s), Value::Mat(c)) => {
            if s.class == Class::Char || c.class == Class::Char {
                return s.class == Class::Char && c.class == Class::Char && s.to_string_lossy() == c.to_string_lossy() && s.rows <= 1 && c.rows <= 1;
            }
            if s.is_empty() || c.is_empty() {
                return false;
            }
            if c.is_scalar() {
                return s.is_scalar() && s.re[0] == c.re[0] && s.im_at(0) == c.im_at(0);
            }
            false
        }
        _ => false,
    }
}

pub(crate) fn collect_idents(e: &Expr, out: &mut HashSet<String>) {
    match e {
        Expr::Ident(n) => {
            out.insert(n.clone());
        }
        Expr::Paren(x) | Expr::Unary(_, x) | Expr::Postfix(_, x) | Expr::Field(x, _) => collect_idents(x, out),
        Expr::Binary(_, a, b) => {
            collect_idents(a, out);
            collect_idents(b, out);
        }
        Expr::Range(a, s, b) => {
            collect_idents(a, out);
            if let Some(s) = s {
                collect_idents(s, out);
            }
            collect_idents(b, out);
        }
        Expr::Index(f, args) => {
            collect_idents(f, out);
            for a in args {
                collect_idents(a, out);
            }
        }
        Expr::Matrix(rows) => {
            for r in rows {
                for x in r {
                    collect_idents(x, out);
                }
            }
        }
        Expr::CellList(items) => {
            for x in items {
                collect_idents(x, out);
            }
        }
        Expr::AnonFn(params, body) => {
            let mut inner = HashSet::new();
            collect_idents(body, &mut inner);
            for n in inner {
                if !params.contains(&n) {
                    out.insert(n);
                }
            }
        }
        _ => {}
    }
}

/// Execute text and return the merged output (for the scenario suite and tests).
pub fn run_capture(src: &str) -> String {
    let mut it = Interp::capture();
    it.run(src);
    it.take_output()
}
