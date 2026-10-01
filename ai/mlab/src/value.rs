//! Language values: f64 matrices (column-major) with a class (double, logical, char) and an optional
//! imaginary part; function handles; placeholders for v2 — tables and exact (rational) matrices.

use crate::ast::Expr;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Double,
    Logical,
    Char,
}

/// Numeric/logical/char matrix. Data is column-major: element (i, j) — `re[j * rows + i]`.
#[derive(Clone, Debug)]
pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    pub re: Vec<f64>,
    pub im: Option<Vec<f64>>,
    pub class: Class,
}

/// Function handle: named (`@sin`, `@myfun`) or anonymous (`@(x) x.^2`) with captured variables.
#[derive(Debug)]
pub enum Func {
    Named(String),
    Anon { params: Vec<String>, body: Rc<Expr>, captured: Vec<(String, Value)> },
}

/// Table column type. Num and Date — double `Mat` (a date is a datenum, NaN — missing), Bool — logical `Mat`,
/// Text — `Value::Str` (None — missing).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColKind {
    Num,
    Bool,
    Text,
    Date,
}

/// v2: table — named columns of different types, each a column vector of the same length; data is stored
/// by columns in memory, so `T.c = T.a .* T.b` is a single vector operation.
#[derive(Clone, Debug)]
pub struct Table {
    pub names: Vec<String>,
    pub columns: Vec<Value>,
    pub kinds: Vec<ColKind>,
}

/// v2: string array (class `string`), column-major; `None` — missing (`<missing>`). From `{'a','b'}` and text
/// table columns.
#[derive(Clone, Debug)]
pub struct StrArr {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Option<String>>,
}

impl StrArr {
    pub fn col(data: Vec<Option<String>>) -> StrArr {
        StrArr { rows: data.len(), cols: 1, data }
    }
    pub fn row(data: Vec<Option<String>>) -> StrArr {
        StrArr { rows: if data.is_empty() { 0 } else { 1 }, cols: data.len(), data }
    }
}

/// v2: exact mode — rational matrices via the `math` crate (after the math v2 merge).
/// Empty enum: a value of this type cannot be created in v1, and every `match` on `Value` already has a branch for it.
#[derive(Clone, Debug)]
pub enum Exact {}

#[derive(Clone, Debug)]
pub enum Value {
    Mat(Mat),
    Func(Rc<Func>),
    Table(Rc<Table>),
    Str(Rc<StrArr>),
    Exact(Exact),
}

impl Mat {
    pub fn new(rows: usize, cols: usize, re: Vec<f64>) -> Mat {
        debug_assert_eq!(re.len(), rows * cols);
        Mat { rows, cols, re, im: None, class: Class::Double }
    }
    pub fn complex(rows: usize, cols: usize, re: Vec<f64>, im: Vec<f64>) -> Mat {
        let mut m = Mat { rows, cols, re, im: Some(im), class: Class::Double };
        m.narrow();
        m
    }
    pub fn scalar(x: f64) -> Mat {
        Mat::new(1, 1, vec![x])
    }
    pub fn cscalar(re: f64, im: f64) -> Mat {
        Mat::complex(1, 1, vec![re], vec![im])
    }
    pub fn boolean(b: bool) -> Mat {
        Mat { rows: 1, cols: 1, re: vec![if b { 1.0 } else { 0.0 }], im: None, class: Class::Logical }
    }
    pub fn zeros(rows: usize, cols: usize) -> Mat {
        Mat::new(rows, cols, vec![0.0; rows * cols])
    }
    pub fn filled(rows: usize, cols: usize, x: f64) -> Mat {
        Mat::new(rows, cols, vec![x; rows * cols])
    }
    pub fn empty() -> Mat {
        Mat::new(0, 0, vec![])
    }
    pub fn row(v: Vec<f64>) -> Mat {
        let n = v.len();
        Mat::new(1, n, v)
    }
    pub fn col(v: Vec<f64>) -> Mat {
        let n = v.len();
        Mat::new(n, 1, v)
    }
    pub fn str(s: &str) -> Mat {
        let re: Vec<f64> = s.chars().map(|c| c as u32 as f64).collect();
        let n = re.len();
        Mat { rows: if n == 0 { 0 } else { 1 }, cols: n, re, im: None, class: Class::Char }
    }
    pub fn with_class(mut self, class: Class) -> Mat {
        self.class = class;
        self
    }
    pub fn numel(&self) -> usize {
        self.rows * self.cols
    }
    pub fn is_empty(&self) -> bool {
        self.rows == 0 || self.cols == 0
    }
    pub fn is_scalar(&self) -> bool {
        self.rows == 1 && self.cols == 1
    }
    pub fn is_vector(&self) -> bool {
        (self.rows == 1 || self.cols == 1) && self.numel() >= 1
    }
    pub fn is_complex(&self) -> bool {
        self.im.is_some()
    }
    pub fn at(&self, i: usize, j: usize) -> f64 {
        self.re[j * self.rows + i]
    }
    pub fn im_at(&self, k: usize) -> f64 {
        self.im.as_ref().map_or(0.0, |v| v[k])
    }
    /// An imaginary part that is all zeros is dropped (like automatic narrowing in Octave).
    pub fn narrow(&mut self) {
        if let Some(im) = &self.im {
            if im.iter().all(|&x| x == 0.0) {
                self.im = None;
            }
        }
    }
    pub fn im_or_zeros(&self) -> Vec<f64> {
        self.im.clone().unwrap_or_else(|| vec![0.0; self.re.len()])
    }
    pub fn to_string_lossy(&self) -> String {
        // rows of a char matrix are read row by row
        let mut s = String::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                let c = self.at(i, j);
                s.push(char::from_u32(c.max(0.0) as u32).unwrap_or('\u{FFFD}'));
            }
        }
        s
    }
    pub fn row_string(&self, i: usize) -> String {
        (0..self.cols).map(|j| char::from_u32(self.at(i, j).max(0.0) as u32).unwrap_or('\u{FFFD}')).collect()
    }
    pub fn transpose(&self) -> Mat {
        let (r, c) = (self.rows, self.cols);
        let mut re = vec![0.0; r * c];
        for j in 0..c {
            for i in 0..r {
                re[i * c + j] = self.re[j * r + i];
            }
        }
        let im = self.im.as_ref().map(|v| {
            let mut out = vec![0.0; r * c];
            for j in 0..c {
                for i in 0..r {
                    out[i * c + j] = v[j * r + i];
                }
            }
            out
        });
        Mat { rows: c, cols: r, re, im, class: self.class }
    }
    pub fn dims_str(&self) -> String {
        format!("{}x{}", self.rows, self.cols)
    }
}

impl Value {
    pub fn num(x: f64) -> Value {
        Value::Mat(Mat::scalar(x))
    }
    pub fn boolean(b: bool) -> Value {
        Value::Mat(Mat::boolean(b))
    }
    pub fn str(s: &str) -> Value {
        Value::Mat(Mat::str(s))
    }
    pub fn class_name(&self) -> &'static str {
        match self {
            Value::Mat(m) => match m.class {
                Class::Double => "double",
                Class::Logical => "logical",
                Class::Char => "char",
            },
            Value::Func(_) => "function_handle",
            Value::Table(_) => "table",
            Value::Str(_) => "string",
            Value::Exact(e) => match *e {},
        }
    }
}
