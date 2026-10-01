//! mlab — a computing language in the spirit of MATLAB/Octave: a clean reimplementation from public documentation
//! (syntax, semantics, output format), an interpreter in Rust. Linear algebra — faer, FFT — rustfft.
//! Every linear algebra result passes a checking gate; an error gives a warning, not silence.

pub mod ast;
pub mod builtins;
pub mod display;
pub mod fileio;
pub mod index;
pub mod interp;
pub mod lexer;
pub mod mcgate;
pub mod ops;
pub mod parser;
pub mod rng;
pub mod suite;
pub mod table;
pub mod value;
pub mod vm;

pub use interp::{Interp, MError, run_capture};
pub use value::{Class, Mat, Value};
