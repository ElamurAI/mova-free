//! latex — built-in LaTeX support for Mova: parsing math-mode formulas into a tree (own enums,
//! the command table compiled from `data/commands.txt`), document macros (`\newcommand`, `\def` with
//! `#1…#9`), printing to canonical LaTeX and plain text, a round-trip gate, formula extraction from `.tex`.
//! No external TeX is needed. The bridge to `math::Expr` is the `tomath` module (feature `math`).
//!
//! ```
//! let t = latex::parse(r"\frac{a}{b} \le \sqrt[3]{x}").unwrap();
//! assert_eq!(latex::to_latex(&t), r"\frac{a}{b} \le \sqrt[3]{x}");
//! assert_eq!(latex::to_text(&t), "a/b ≤ ∛x");
//! assert!(latex::gate::check(&t).is_ok());
//! ```

pub mod ast;
pub mod extract;
pub mod gate;
pub mod lex;
pub mod macros;
pub mod parse;
pub mod print;
pub mod table;
#[cfg(feature = "math")]
pub mod tomath;

pub use ast::{Env, EnvName, Node, Scripts, Span, Unknown};
pub use extract::{Formula, extract};
pub use macros::Macros;
pub use parse::{Error, ErrorKind, parse, parse_with};
pub use print::{to_latex, to_text};
