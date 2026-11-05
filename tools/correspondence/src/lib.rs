//! Typed finite relations. Formula equality never proves Rust correspondence.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
pub const MAX_FORMULAS: usize = 4096;
pub const MAX_UNIVERSE: usize = 64;
pub const MAX_DEPTH: usize = 32;
pub const MAX_NODES: usize = 4096;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Forward,
    ReverseReconstruction,
    BehaviorComplement,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct Sort {
    pub name: String,
    pub values: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct Contract {
    pub domain: Sort,
    pub codomain: Sort,
    pub observation: String,
    pub assumptions: Vec<String>,
    pub effects: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SourceBinding {
    pub path: String,
    pub sha256: String,
    pub symbol: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ArtifactBinding {
    pub path: String,
    pub sha256: String,
    pub section: String,
}
/// Composition executes first then then; complement uses the full inferred rectangle.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
pub enum Expr {
    Atom {
        name: String,
        domain: Sort,
        codomain: Sort,
        pairs: Vec<[usize; 2]>,
    },
    Identity {
        sort: Sort,
    },
    Converse {
        arg: Box<Expr>,
    },
    Complement {
        arg: Box<Expr>,
    },
    Compose {
        first: Box<Expr>,
        then: Box<Expr>,
    },
    Union {
        args: Vec<Expr>,
    },
    Intersection {
        args: Vec<Expr>,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Formula {
    pub id: String,
    pub groupant_id: String,
    pub role: Role,
    pub contract: Contract,
    pub expr: Expr,
    pub source: SourceBinding,
    pub model: ArtifactBinding,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub formulas: Vec<Formula>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Comparison {
    pub left: String,
    pub right: String,
    pub status: String,
    pub structural_match: bool,
    pub finite_equal: bool,
    pub counterexample: Option<[usize; 2]>,
    pub scope: String,
}
fn text_ok(s: &str) -> bool {
    !s.trim().is_empty() && s.len() <= 4096
}
fn check_sort(s: &Sort) -> Result<(), String> {
    if !text_ok(&s.name) || s.values.len() > MAX_UNIVERSE {
        return Err("invalid named sort or universe cap".into());
    }
    let mut seen = BTreeSet::new();
    if s.values.iter().any(|v| !text_ok(v) || !seen.insert(v)) {
        return Err("invalid or duplicate finite coordinate".into());
    }
    Ok(())
}
type AtomDefinition = (Sort, Sort, BTreeSet<[usize; 2]>);
fn infer(
    e: &Expr,
    depth: usize,
    count: &mut usize,
    atoms: &mut BTreeMap<String, AtomDefinition>,
) -> Result<(Sort, Sort), String> {
    *count += 1;
    if depth > MAX_DEPTH || *count > MAX_NODES {
        return Err("formula depth/node budget exceeded".into());
    }
    match e {
        Expr::Atom {
            name,
            domain,
            codomain,
            pairs,
        } => {
            check_sort(domain)?;
            check_sort(codomain)?;
            if !text_ok(name)
                || pairs.len() > MAX_UNIVERSE * MAX_UNIVERSE
                || pairs
                    .iter()
                    .any(|p| p[0] >= domain.values.len() || p[1] >= codomain.values.len())
            {
                return Err("invalid atom name,coordinate or pair budget".into());
            }
            let def = (
                domain.clone(),
                codomain.clone(),
                pairs.iter().copied().collect(),
            );
            if atoms.get(name).is_some_and(|old| old != &def) {
                return Err("conflicting atom bindings".into());
            }
            atoms.insert(name.clone(), def);
            Ok((domain.clone(), codomain.clone()))
        }
        Expr::Identity { sort } => {
            check_sort(sort)?;
            Ok((sort.clone(), sort.clone()))
        }
        Expr::Converse { arg } => {
            let (a, b) = infer(arg, depth + 1, count, atoms)?;
            Ok((b, a))
        }
        Expr::Complement { arg } => infer(arg, depth + 1, count, atoms),
        Expr::Compose { first, then } => {
            let (a, b) = infer(first, depth + 1, count, atoms)?;
            let (c, d) = infer(then, depth + 1, count, atoms)?;
            if b != c {
                return Err("composition intermediate sort mismatch".into());
            }
            Ok((a, d))
        }
        Expr::Union { args } | Expr::Intersection { args } => {
            if args.is_empty() || args.len() > MAX_NODES {
                return Err("set operator arity outside budget".into());
            }
            let ends = infer(&args[0], depth + 1, count, atoms)?;
            for arg in &args[1..] {
                if infer(arg, depth + 1, count, atoms)? != ends {
                    return Err("set operator endpoint mismatch".into());
                }
            }
            Ok(ends)
        }
    }
}
/// Validates bounded typed algebra, not source/model filesystem bindings.
pub fn validate_formula(f: &Formula) -> Result<(), String> {
    for s in [
        &f.id,
        &f.groupant_id,
        &f.contract.observation,
        &f.source.path,
        &f.source.symbol,
        &f.model.path,
        &f.model.section,
    ] {
        if !text_ok(s) {
            return Err("invalid formula metadata".into());
        }
    }
    for hash in [&f.source.sha256, &f.model.sha256] {
        if hash.len() != 64 || !hash.bytes().all(|x| x.is_ascii_hexdigit()) {
            return Err("SHA requires 64 hexadecimal digits".into());
        }
    }
    for fields in [&f.contract.assumptions, &f.contract.effects] {
        if fields.len() > MAX_NODES || fields.iter().any(|s| !text_ok(s)) {
            return Err("invalid contract metadata".into());
        }
    }
    check_sort(&f.contract.domain)?;
    check_sort(&f.contract.codomain)?;
    if infer(&f.expr, 1, &mut 0, &mut BTreeMap::new())?
        != (f.contract.domain.clone(), f.contract.codomain.clone())
    {
        return Err("root endpoint contract mismatch".into());
    }
    Ok(())
}

// Laws of typed relations only: no inverse cancellation or complement composition.
// Lean RelationComposition covers associativity, converse of composition, involutions
// and identity units. Converse/complement commutation on the full rectangle and converse
// over union/intersection are finite-checked here only.
fn height(e: &Expr) -> usize {
    match e {
        Expr::Atom { .. } | Expr::Identity { .. } => 1,
        Expr::Converse { arg } | Expr::Complement { arg } => 1 + height(arg),
        Expr::Compose { first, then } => 1 + height(first).max(height(then)),
        Expr::Union { args } | Expr::Intersection { args } => {
            1 + args.iter().map(height).max().unwrap_or(0)
        }
    }
}
fn size(e: &Expr) -> usize {
    match e {
        Expr::Atom { .. } | Expr::Identity { .. } => 1,
        Expr::Converse { arg } | Expr::Complement { arg } => 1 + size(arg),
        Expr::Compose { first, then } => 1 + size(first) + size(then),
        Expr::Union { args } | Expr::Intersection { args } => {
            1 + args.iter().map(size).sum::<usize>()
        }
    }
}
fn complement(e: Expr) -> Expr {
    match e {
        Expr::Complement { arg } => *arg,
        other => Expr::Complement {
            arg: Box::new(other),
        },
    }
}
fn flatten(e: Expr, out: &mut Vec<Expr>) {
    match e {
        Expr::Compose { first, then } => {
            flatten(*first, out);
            flatten(*then, out);
        }
        other => out.push(other),
    }
}
/// Longest prefix of heights[i..] that fits one alphabetic tree of height at most b.
fn reach(i: usize, b: usize, h: &[usize], memo: &mut [Vec<Option<usize>>]) -> usize {
    if i >= h.len() || h[i] > b {
        return i;
    }
    if let Some(r) = memo[i][b] {
        return r;
    }
    let mut best = i + 1;
    if b >= 2 {
        let left = reach(i, b - 1, h, memo);
        best = best.max(left);
        if left > i {
            best = best.max(reach(left, b - 1, h, memo));
        }
    }
    memo[i][b] = Some(best);
    best
}
fn build(
    leaves: &mut [Option<Expr>],
    i: usize,
    end: usize,
    b: usize,
    h: &[usize],
    memo: &mut [Vec<Option<usize>>],
) -> Expr {
    if end - i == 1 {
        return leaves[i].take().expect("each factor is placed once");
    }
    // Greedy maximal left part; any suffix of a fitting run fits as well.
    let k = reach(i, b - 1, h, memo).min(end - 1);
    Expr::Compose {
        first: Box::new(build(leaves, i, k, b - 1, h, memo)),
        then: Box::new(build(leaves, k, end, b - 1, h, memo)),
    }
}
/// Ordered factors (identity units removed) as the deterministic minimum-height
/// alphabetic tree. Any accepted association is one such tree, so height never grows.
fn chain(parts: Vec<Expr>) -> Expr {
    let mut all = Vec::new();
    for p in parts {
        flatten(p, &mut all);
    }
    let mut unit = None;
    let mut factors = Vec::new();
    for f in all {
        if matches!(f, Expr::Identity { .. }) {
            unit.get_or_insert(f);
        } else {
            factors.push(f);
        }
    }
    match factors.len() {
        0 => return unit.expect("composition has factors"),
        1 => return factors.pop().unwrap(),
        _ => {}
    }
    let h: Vec<usize> = factors.iter().map(height).collect();
    let n = factors.len();
    let top = h.iter().copied().max().unwrap() + usize::BITS as usize;
    let mut memo = vec![vec![None; top + 1]; n];
    let mut b = h.iter().copied().max().unwrap() + 1;
    while reach(0, b, &h, &mut memo) < n {
        b += 1;
    }
    let mut leaves: Vec<_> = factors.into_iter().map(Some).collect();
    build(&mut leaves, 0, n, b, &h, &mut memo)
}
fn set_op(union: bool, args: Vec<Expr>) -> Expr {
    let mut out = BTreeSet::new();
    for a in args {
        match a {
            Expr::Union { args } if union => out.extend(args),
            Expr::Intersection { args } if !union => out.extend(args),
            other => {
                out.insert(other);
            }
        }
    }
    let args: Vec<_> = out.into_iter().collect();
    if args.len() == 1 {
        args.into_iter().next().unwrap()
    } else if union {
        Expr::Union { args }
    } else {
        Expr::Intersection { args }
    }
}
/// Converse of an expression already normalized with converse pushed to atoms.
fn converse(e: Expr) -> Expr {
    match e {
        Expr::Converse { arg } => *arg,
        Expr::Identity { .. } => e,
        Expr::Atom { .. } => Expr::Converse { arg: Box::new(e) },
        Expr::Complement { arg } => complement(converse(*arg)),
        Expr::Compose { .. } => {
            let mut factors = Vec::new();
            flatten(e, &mut factors);
            chain(factors.into_iter().rev().map(converse).collect())
        }
        Expr::Union { args } => set_op(true, args.into_iter().map(converse).collect()),
        Expr::Intersection { args } => set_op(false, args.into_iter().map(converse).collect()),
    }
}
/// push=true places converse on atoms; push=false only cancels double converse and
/// never adds nodes. Both modes keep height and atom identities.
fn normalize_mode(e: &Expr, push: bool) -> Expr {
    match e {
        Expr::Atom {
            name,
            domain,
            codomain,
            pairs,
        } => Expr::Atom {
            name: name.clone(),
            domain: domain.clone(),
            codomain: codomain.clone(),
            pairs: pairs
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
        },
        Expr::Identity { .. } => e.clone(),
        Expr::Converse { arg } => match normalize_mode(arg, push) {
            a if push => converse(a),
            Expr::Converse { arg } => *arg,
            a => Expr::Converse { arg: Box::new(a) },
        },
        Expr::Complement { arg } => complement(normalize_mode(arg, push)),
        Expr::Compose { first, then } => chain(vec![
            normalize_mode(first, push),
            normalize_mode(then, push),
        ]),
        Expr::Union { args } | Expr::Intersection { args } => set_op(
            matches!(e, Expr::Union { .. }),
            args.iter().map(|a| normalize_mode(a, push)).collect(),
        ),
    }
}
/// Pushing converse onto atoms can multiply converse nodes beyond MAX_NODES; the
/// bounded mode is then used so canonical output always revalidates.
fn normalize(e: &Expr) -> Expr {
    let full = normalize_mode(e, true);
    if size(&full) <= MAX_NODES {
        full
    } else {
        normalize_mode(e, false)
    }
}
/// Expression-only key: caller must index the contract separately. Free atom
/// names retain their declared identities; there is no name-based alpha rewrite.
pub fn canonical(f: &Formula) -> Result<String, String> {
    validate_formula(f)?;
    serde_json::to_string(&normalize(&f.expr)).map_err(|e| e.to_string())
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct Matrix {
    rows: Vec<u64>,
    columns: usize,
}
fn mask(n: usize) -> u64 {
    if n == 64 { u64::MAX } else { (1u64 << n) - 1 }
}
fn matrix_compose(a: &Matrix, b: &Matrix) -> Matrix {
    let mut rows = vec![0; a.rows.len()];
    for (i, &bits) in a.rows.iter().enumerate() {
        for (j, &row) in b.rows.iter().enumerate() {
            if bits & (1u64 << j) != 0 {
                rows[i] |= row;
            }
        }
    }
    Matrix {
        rows,
        columns: b.columns,
    }
}
fn eval(e: &Expr) -> Matrix {
    match e {
        Expr::Atom {
            domain,
            codomain,
            pairs,
            ..
        } => {
            let mut rows = vec![0; domain.values.len()];
            for &[a, b] in pairs {
                rows[a] |= 1u64 << b;
            }
            Matrix {
                rows,
                columns: codomain.values.len(),
            }
        }
        Expr::Identity { sort } => Matrix {
            rows: (0..sort.values.len()).map(|i| 1u64 << i).collect(),
            columns: sort.values.len(),
        },
        Expr::Converse { arg } => {
            let a = eval(arg);
            let mut rows = vec![0; a.columns];
            for (i, &bits) in a.rows.iter().enumerate() {
                for (j, row) in rows.iter_mut().enumerate() {
                    if bits & (1u64 << j) != 0 {
                        *row |= 1u64 << i;
                    }
                }
            }
            Matrix {
                rows,
                columns: a.rows.len(),
            }
        }
        Expr::Complement { arg } => {
            let mut a = eval(arg);
            let u = mask(a.columns);
            for row in &mut a.rows {
                *row = !*row & u;
            }
            a
        }
        Expr::Compose { first, then } => matrix_compose(&eval(first), &eval(then)),
        Expr::Union { args } | Expr::Intersection { args } => {
            let mut out = eval(&args[0]);
            for arg in &args[1..] {
                let b = eval(arg);
                for (a, b) in out.rows.iter_mut().zip(b.rows) {
                    if matches!(e, Expr::Union { .. }) {
                        *a |= b;
                    } else {
                        *a &= b;
                    }
                }
            }
            out
        }
    }
}
const SCOPE: &str =
    "Declared finite models only; source correspondence unproved by this comparison";
/// Exhaustive table equality is finite model evidence, never a general Rust proof.
pub fn compare(left: &Formula, right: &Formula) -> Result<Comparison, String> {
    validate_formula(left)?;
    validate_formula(right)?;
    let mut out = Comparison {
        left: left.id.clone(),
        right: right.id.clone(),
        status: "incompatible_contract".into(),
        structural_match: false,
        finite_equal: false,
        counterexample: None,
        scope: SCOPE.into(),
    };
    if left.contract != right.contract {
        return Ok(out);
    }
    let a = eval(&left.expr);
    let b = eval(&right.expr);
    out.structural_match = normalize(&left.expr) == normalize(&right.expr);
    out.finite_equal = a == b;
    out.status = if out.structural_match {
        "normalized_formula_match"
    } else if out.finite_equal {
        "finite_coincidence"
    } else {
        "finite_counterexample"
    }
    .into();
    for (i, (&x, &y)) in a.rows.iter().zip(&b.rows).enumerate() {
        let diff = x ^ y;
        if diff != 0 {
            out.counterexample = Some([i, diff.trailing_zeros() as usize]);
            break;
        }
    }
    Ok(out)
}
/// Finite composite properties. functional: univalent (at most one output per input);
/// injective: converse univalent; total: every domain value has an output (vacuously
/// true for an empty domain); surjective: every codomain value is hit (vacuously true
/// for an empty codomain). An incompatible contract leaves every property false.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CompositionSummary {
    pub left: String,
    pub right: String,
    pub status: String,
    pub cardinality: usize,
    pub functional: bool,
    pub injective: bool,
    pub identity: bool,
    pub total: bool,
    pub surjective: bool,
    pub scope: String,
}
/// Compose pure finite relations when intermediate sorts and assumptions agree.
/// Stateful composition needs an explicit common-state adapter and is excluded.
pub fn compose_summary(left: &Formula, right: &Formula) -> Result<CompositionSummary, String> {
    validate_formula(left)?;
    validate_formula(right)?;
    let mut out = CompositionSummary {
        left: left.id.clone(),
        right: right.id.clone(),
        status: "incompatible_contract".into(),
        cardinality: 0,
        functional: false,
        injective: false,
        identity: false,
        total: false,
        surjective: false,
        scope: SCOPE.into(),
    };
    if left.contract.codomain != right.contract.domain
        || left.contract.observation != right.contract.observation
        || left.contract.assumptions != right.contract.assumptions
        || left.contract.effects != right.contract.effects
        || !left.contract.effects.is_empty()
    {
        return Ok(out);
    }
    let m = matrix_compose(&eval(&left.expr), &eval(&right.expr));
    out.status = "finite_composition".into();
    out.cardinality = m.rows.iter().map(|r| r.count_ones() as usize).sum();
    out.functional = m.rows.iter().all(|r| r.count_ones() <= 1);
    let mut seen = 0;
    out.injective = true;
    for &r in &m.rows {
        if seen & r != 0 {
            out.injective = false;
        }
        seen |= r;
    }
    out.total = m.rows.iter().all(|&r| r != 0);
    out.surjective = seen == mask(m.columns);
    out.identity = left.contract.domain == right.contract.codomain
        && m.rows.iter().enumerate().all(|(i, &r)| r == 1u64 << i);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sort(name: &str, n: usize) -> Sort {
        Sort {
            name: name.into(),
            values: (0..n).map(|i| i.to_string()).collect(),
        }
    }
    fn atom(name: &str, a: &Sort, b: &Sort, bits: usize) -> Expr {
        Expr::Atom {
            name: name.into(),
            domain: a.clone(),
            codomain: b.clone(),
            pairs: (0..a.values.len())
                .flat_map(|i| (0..b.values.len()).map(move |j| [i, j]))
                .filter(|[i, j]| bits & (1usize << (i * b.values.len() + j)) != 0)
                .collect(),
        }
    }
    fn rev(e: Expr) -> Expr {
        Expr::Converse { arg: Box::new(e) }
    }
    fn comp(a: Expr, b: Expr) -> Expr {
        Expr::Compose {
            first: Box::new(a),
            then: Box::new(b),
        }
    }
    fn not(e: Expr) -> Expr {
        Expr::Complement { arg: Box::new(e) }
    }
    fn formula(e: Expr) -> Formula {
        let (domain, codomain) = infer(&e, 1, &mut 0, &mut BTreeMap::new()).unwrap();
        Formula {
            id: "fixture".into(),
            groupant_id: "fixture".into(),
            role: Role::Forward,
            contract: Contract {
                domain,
                codomain,
                observation: "finite relation".into(),
                assumptions: vec![],
                effects: vec![],
            },
            expr: e,
            source: SourceBinding {
                path: "fixture.rs".into(),
                sha256: "0".repeat(64),
                symbol: "fixture".into(),
            },
            model: ArtifactBinding {
                path: "fixture.md".into(),
                sha256: "0".repeat(64),
                section: "Forward".into(),
            },
        }
    }
    #[test]
    fn small_relations_normalization_is_sound_and_idempotent() {
        let s = sort("bool", 2);
        for r in 0..16 {
            for t in 0..16 {
                let a = atom("r", &s, &s, r);
                let b = atom("s", &s, &s, t);
                let lhs = rev(comp(a.clone(), b.clone()));
                let rhs = comp(rev(b), rev(a.clone()));
                assert_eq!(eval(&lhs), eval(&rhs));
                assert!(
                    compare(&formula(lhs.clone()), &formula(rhs))
                        .unwrap()
                        .structural_match
                );
                for e in [
                    lhs,
                    rev(rev(a.clone())),
                    not(not(a.clone())),
                    comp(a.clone(), Expr::Identity { sort: s.clone() }),
                ] {
                    assert_eq!(eval(&e), eval(&normalize(&e)));
                    assert_eq!(normalize(&normalize(&e)), normalize(&e));
                }
            }
        }
    }
    #[test]
    fn rectangular_converse_preserves_reversed_endpoints() {
        let x = sort("x", 2);
        let y = sort("y", 3);
        let z = sort("z", 1);
        for r in 0..64 {
            for t in 0..8 {
                let a = atom("r", &x, &y, r);
                let b = atom("s", &y, &z, t);
                let lhs = formula(rev(comp(a.clone(), b.clone())));
                let rhs = formula(comp(rev(b), rev(a)));
                assert!(compare(&lhs, &rhs).unwrap().finite_equal);
                assert_eq!(lhs.contract.domain, z);
                assert_eq!(lhs.contract.codomain, x);
            }
        }
    }
    #[test]
    fn wrong_complement_composition_and_inverse_laws_show_red() {
        let s = sort("bool", 2);
        let id = Expr::Identity { sort: s.clone() };
        let wrong = compare(
            &formula(not(comp(id.clone(), id.clone()))),
            &formula(comp(not(id.clone()), not(id.clone()))),
        )
        .unwrap();
        assert!(!wrong.finite_equal);
        assert!(wrong.counterexample.is_some());
        let constant = atom("constant", &s, &s, 5);
        let fibers = comp(constant.clone(), rev(constant.clone()));
        assert!(
            !compare(&formula(fibers), &formula(id))
                .unwrap()
                .finite_equal
        );
        let summary = compose_summary(&formula(constant.clone()), &formula(rev(constant))).unwrap();
        assert_eq!(summary.cardinality, 4);
        assert!(!summary.functional);
        assert!(!summary.identity);
    }
    #[test]
    fn theories_effects_and_intermediate_sorts_are_checked() {
        let q = sort("rational", 2);
        let p = sort("field_998244353", 2);
        let a = formula(atom("a", &q, &q, 9));
        let b = formula(atom("b", &p, &p, 9));
        assert_eq!(compare(&a, &b).unwrap().status, "incompatible_contract");
        let mut e = a.clone();
        e.contract.effects.push("writes_state".into());
        assert_eq!(compare(&a, &e).unwrap().status, "incompatible_contract");
        assert_eq!(
            compose_summary(&e, &e).unwrap().status,
            "incompatible_contract"
        );
        let mut bad = a.clone();
        bad.expr = comp(a.expr, b.expr);
        assert!(validate_formula(&bad).is_err());
    }
    #[test]
    fn finite_coincidence_retains_free_symbol_identity() {
        let s = sort("bool", 2);
        let c = compare(
            &formula(atom("one_source", &s, &s, 9)),
            &formula(atom("another_source", &s, &s, 9)),
        )
        .unwrap();
        assert_eq!(c.status, "finite_coincidence");
        assert!(!c.structural_match);
    }
    #[test]
    fn malformed_budget_and_unknown_nodes_are_rejected() {
        assert!(serde_json::from_str::<Expr>(r#"{"op":"inverse"}"#).is_err());
        let s = sort("bool", 2);
        let mut f = formula(atom("a", &s, &s, 0));
        for _ in 0..MAX_DEPTH {
            f.expr = rev(f.expr);
        }
        assert!(validate_formula(&f).is_err());
        let mut bad = formula(atom("a", &s, &s, 0));
        if let Expr::Atom { pairs, .. } = &mut bad.expr {
            pairs.push([2, 0]);
        }
        assert!(validate_formula(&bad).is_err());
        bad.expr = Expr::Union { args: vec![] };
        assert!(validate_formula(&bad).is_err());
    }
    #[test]
    fn bit_width_and_empty_endpoint_boundaries() {
        let s = sort("u64_coordinates", 64);
        let id = Expr::Identity { sort: s.clone() };
        let f = formula(not(not(id.clone())));
        validate_formula(&f).unwrap();
        assert_eq!(eval(&f.expr), eval(&id));
        let empty = sort("empty", 0);
        let e = formula(rev(atom("empty", &empty, &s, 0)));
        validate_formula(&e).unwrap();
        assert_eq!(eval(&e.expr).rows, vec![0; 64]);
    }
    #[test]
    fn repeated_atom_name_cannot_change_meaning() {
        let s = sort("bool", 2);
        let mut f = formula(atom("a", &s, &s, 0));
        f.expr = Expr::Union {
            args: vec![atom("a", &s, &s, 0), atom("a", &s, &s, 1)],
        };
        assert!(validate_formula(&f).is_err());
    }
    fn balanced_chain(leaf: &Expr, n: usize) -> Expr {
        if n == 1 {
            leaf.clone()
        } else {
            comp(balanced_chain(leaf, n / 2), balanced_chain(leaf, n - n / 2))
        }
    }
    fn reformula(f: &Formula) -> Formula {
        let mut g = f.clone();
        g.expr = serde_json::from_str(&canonical(f).unwrap()).unwrap();
        g
    }
    #[test]
    fn canonical_output_stays_within_validation_budgets() {
        let s = sort("one", 1);
        let f = formula(rev(balanced_chain(&atom("a", &s, &s, 1), MAX_NODES / 2)));
        validate_formula(&f).unwrap();
        let g = reformula(&f);
        validate_formula(&g).unwrap();
        assert_eq!(eval(&f.expr), eval(&g.expr));
    }
    #[test]
    fn associativity_and_converse_complement_commutation_match_structurally() {
        let s = sort("bool", 2);
        for r in 0..16 {
            for t in 0..16 {
                for u in [0, 3, 6, 9, 15] {
                    let (a, b, c) = (
                        atom("a", &s, &s, r),
                        atom("b", &s, &s, t),
                        atom("c", &s, &s, u),
                    );
                    let left = comp(comp(a.clone(), b.clone()), c.clone());
                    let right = comp(a.clone(), comp(b.clone(), c.clone()));
                    let rr = comp(comp(rev(c.clone()), rev(b.clone())), rev(a.clone()));
                    for (x, y) in [
                        (left.clone(), right.clone()),
                        (rev(left), rr),
                        (rev(not(a.clone())), not(rev(a.clone()))),
                        (
                            rev(Expr::Union {
                                args: vec![a.clone(), b.clone()],
                            }),
                            Expr::Union {
                                args: vec![rev(b.clone()), rev(a.clone())],
                            },
                        ),
                    ] {
                        let cmp = compare(&formula(x), &formula(y)).unwrap();
                        assert_eq!(cmp.status, "normalized_formula_match");
                        assert!(cmp.finite_equal);
                    }
                }
            }
        }
        // All five associations of four factors share one canonical tree.
        let f: Vec<_> = (0..4)
            .map(|i| atom(&format!("f{i}"), &s, &s, 1 + 3 * i))
            .collect();
        let shapes = [
            comp(
                comp(comp(f[0].clone(), f[1].clone()), f[2].clone()),
                f[3].clone(),
            ),
            comp(
                comp(f[0].clone(), comp(f[1].clone(), f[2].clone())),
                f[3].clone(),
            ),
            comp(
                comp(f[0].clone(), f[1].clone()),
                comp(f[2].clone(), f[3].clone()),
            ),
            comp(
                f[0].clone(),
                comp(comp(f[1].clone(), f[2].clone()), f[3].clone()),
            ),
            comp(
                f[0].clone(),
                comp(f[1].clone(), comp(f[2].clone(), f[3].clone())),
            ),
        ];
        let keys: BTreeSet<_> = shapes
            .iter()
            .map(|e| canonical(&formula(e.clone())).unwrap())
            .collect();
        assert_eq!(keys.len(), 1);
    }
    #[test]
    fn invalid_inverse_and_complement_laws_stay_nonstructural() {
        let s = sort("bool", 2);
        let id = Expr::Identity { sort: s.clone() };
        let (mut red_inverse, mut red_complement, mut red_union) = (0, 0, 0);
        for r in 0..16 {
            for t in 0..16 {
                let (a, b) = (atom("a", &s, &s, r), atom("b", &s, &s, t));
                let inverse = compare(
                    &formula(comp(a.clone(), rev(a.clone()))),
                    &formula(id.clone()),
                )
                .unwrap();
                let distribute = compare(
                    &formula(not(comp(a.clone(), b.clone()))),
                    &formula(comp(not(a.clone()), not(b.clone()))),
                )
                .unwrap();
                let de_morgan_wrong = compare(
                    &formula(not(Expr::Union {
                        args: vec![a.clone(), b.clone()],
                    })),
                    &formula(Expr::Union {
                        args: vec![not(a.clone()), not(b.clone())],
                    }),
                )
                .unwrap();
                for c in [&inverse, &distribute, &de_morgan_wrong] {
                    assert!(!c.structural_match);
                }
                red_inverse += !inverse.finite_equal as usize;
                red_complement += !distribute.finite_equal as usize;
                red_union += !de_morgan_wrong.finite_equal as usize;
            }
        }
        assert!(red_inverse > 0 && red_complement > 0 && red_union > 0);
    }
    struct Lcg(u64);
    impl Lcg {
        fn next(&mut self, n: u64) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 33) % n) as usize
        }
    }
    fn random_expr(g: &mut Lcg, sorts: &[Sort], from: usize, to: usize, depth: usize) -> Expr {
        let choice = if depth == 0 { 0 } else { g.next(7) };
        match choice {
            1 => rev(random_expr(g, sorts, to, from, depth - 1)),
            2 => not(random_expr(g, sorts, from, to, depth - 1)),
            3 | 4 => {
                let mid = g.next(sorts.len() as u64);
                comp(
                    random_expr(g, sorts, from, mid, depth - 1),
                    random_expr(g, sorts, mid, to, depth - 1),
                )
            }
            5 => {
                let n = 2 + g.next(2);
                let args = (0..n)
                    .map(|_| random_expr(g, sorts, from, to, depth - 1))
                    .collect();
                if g.next(2) == 0 {
                    Expr::Union { args }
                } else {
                    Expr::Intersection { args }
                }
            }
            6 if from == to => Expr::Identity {
                sort: sorts[from].clone(),
            },
            _ => {
                let width = sorts[from].values.len() * sorts[to].values.len();
                let bits = g.next(1 << width);
                atom(
                    &format!("r{from}{to}_{bits}"),
                    &sorts[from],
                    &sorts[to],
                    bits,
                )
            }
        }
    }
    #[test]
    fn canonical_is_idempotent_sound_and_never_deeper() {
        let sorts = [sort("x", 2), sort("y", 3), sort("z", 1)];
        let mut g = Lcg(36_37);
        let (mut changed, mut checked) = (0, 0);
        for _ in 0..3000 {
            let (from, to) = (g.next(3), g.next(3));
            let e = random_expr(&mut g, &sorts, from, to, 6);
            let f = formula(e.clone());
            validate_formula(&f).unwrap();
            let n = normalize(&e);
            assert_eq!(eval(&e), eval(&n));
            assert_eq!(normalize(&n), n);
            assert!(height(&n) <= height(&e));
            let g2 = reformula(&f);
            validate_formula(&g2).unwrap();
            assert_eq!(canonical(&g2).unwrap(), canonical(&f).unwrap());
            changed += (n != e) as usize;
            checked += 1;
        }
        assert_eq!(checked, 3000);
        assert!(changed > 1000);
    }
    #[test]
    fn deep_chains_rebalance_within_depth_budget() {
        let s = sort("bool", 2);
        // Irreducible deep factor: alternating union/intersection nests, height 19.
        let mut deep = atom("d", &s, &s, 6);
        for level in 0..18 {
            let args = vec![deep, atom(&format!("w{level}"), &s, &s, 1 + level % 14)];
            deep = if level % 2 == 0 {
                Expr::Union { args }
            } else {
                Expr::Intersection { args }
            };
        }
        assert_eq!(height(&normalize(&deep)), 19);
        // Left-deep chain at the depth limit with the deep factor first, and the
        // right-deep association of the same factors with the deep factor innermost.
        let factors: Vec<_> = (0..13)
            .map(|i| atom(&format!("k{i}"), &s, &s, 1 + i % 15))
            .collect();
        let mut e = deep.clone();
        for k in &factors {
            e = comp(e, k.clone());
        }
        assert_eq!(height(&e), MAX_DEPTH);
        let mut right = factors[12].clone();
        for k in factors[..12].iter().rev() {
            right = comp(k.clone(), right);
        }
        let right = comp(deep, right);
        let f = formula(e.clone());
        validate_formula(&f).unwrap();
        validate_formula(&formula(right.clone())).unwrap();
        let n = normalize(&e);
        assert!(height(&n) <= MAX_DEPTH && size(&n) <= MAX_NODES);
        assert!(height(&n) < height(&e));
        assert_eq!(n, normalize(&right));
        validate_formula(&reformula(&f)).unwrap();
        assert_eq!(eval(&e), eval(&n));
        // One factor too deep makes the original invalid, as before.
        let mut over = f.clone();
        over.expr = comp(e, atom("extra", &s, &s, 1));
        assert!(validate_formula(&over).is_err());
    }
    #[test]
    fn large_converse_chain_uses_bounded_mode_idempotently() {
        let s = sort("one", 1);
        let e = rev(balanced_chain(&atom("a", &s, &s, 1), MAX_NODES / 2));
        assert_eq!(size(&e), MAX_NODES);
        assert!(size(&normalize_mode(&e, true)) > MAX_NODES);
        let n = normalize(&e);
        assert!(size(&n) <= MAX_NODES);
        assert_eq!(normalize(&n), n);
        // Just under the budget the converse is pushed onto atoms.
        let small = rev(balanced_chain(&atom("a", &s, &s, 1), 64));
        assert!(!matches!(normalize(&small), Expr::Converse { .. }));
    }
    fn summary(a: Expr, b: Expr) -> CompositionSummary {
        compose_summary(&formula(a), &formula(b)).unwrap()
    }
    #[test]
    fn composition_totality_surjectivity_and_empty_sets() {
        let (x, y, z, e) = (sort("x", 2), sort("y", 3), sort("z", 2), sort("empty", 0));
        // x0->y0, x1->y2 then y0->z1, y1->z0: partial, univalent, injective.
        let partial = summary(atom("r", &x, &y, 0b100001), atom("s", &y, &z, 0b000110));
        assert_eq!(partial.cardinality, 1);
        assert!(partial.functional && partial.injective);
        assert!(!partial.total && !partial.surjective);
        // Constant total map: total, not surjective, not injective.
        let constant = summary(atom("c", &x, &y, 0b001001), atom("t", &y, &z, 0b010101));
        assert!(constant.total && constant.functional);
        assert!(!constant.surjective && !constant.injective);
        // Rectangular bijection composed with its converse is identity on x.
        let bij = atom("b", &x, &z, 0b0110);
        let round = summary(bij.clone(), rev(bij));
        assert!(round.identity && round.total && round.surjective);
        // Empty domain: vacuously total and functional; surjective only if codomain empty.
        let empty_dom = summary(atom("e", &e, &x, 0), atom("f", &x, &z, 0b1111));
        assert!(empty_dom.total && empty_dom.functional && empty_dom.injective);
        assert!(!empty_dom.surjective && empty_dom.cardinality == 0);
        // Empty codomain: nonempty domain is not total; surjectivity is vacuous.
        let empty_cod = summary(atom("g", &x, &y, 0b111111), atom("h", &y, &e, 0));
        assert!(!empty_cod.total && empty_cod.surjective && empty_cod.functional);
        // Incompatible contracts claim nothing.
        let mut l = formula(atom("u", &x, &x, 0b1001));
        l.contract.observation = "other".into();
        let bad = compose_summary(&l, &formula(atom("v", &x, &x, 0b1001))).unwrap();
        assert_eq!(bad.status, "incompatible_contract");
        assert!(!bad.total && !bad.surjective && !bad.identity);
    }
}
