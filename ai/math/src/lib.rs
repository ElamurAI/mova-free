//! math — the MMM's (small language model's) base mathematics module: an exact symbolic core (rationals, expression
//! trees, units with dimensions and affine temperature), steps and independent checks, understanding of English
//! questions via the UD tree from `en`. Deterministic, on the CPU, no LLM.

pub mod err;
pub mod eval;
pub mod expr;
pub mod fcheck;
pub mod rat;
pub mod units;
pub mod lex;
pub mod understand;
pub mod suite;

// v2 (26.09): exact core on big integers and rationals (dashu), number theory, combinatorics,
// polynomials, systems, derivatives, the LLM plan language, a solver with checks and memory.
pub mod big;
pub mod nt;
pub mod comb;
pub mod calc;
pub mod poly;
pub mod linsys;
pub mod deriv;
pub mod plan;
pub mod vmm;
pub mod curric;
pub mod memory;
pub mod solve;
// v3 (30.09): word problems — the slow model builds a tree of subproblems, the fast core computes.
pub mod arith;
// v4 (30.09): a transition system over the text's numbers with UD features.
pub mod steps;
// v5 (30.09): a subproblem graph (DAG) — a pool of quantities, steps over any pair.
pub mod deduct;
// 30.09: the snake writes the problem's quantitative world itself (ARIS-like).
pub mod qworld;
pub mod qread;
pub mod lean;
