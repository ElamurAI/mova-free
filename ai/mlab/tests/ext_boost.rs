//! Boost.Math test data (Boost Software License 1.0, `tests/data/ext/boost/LICENSE_1_0.txt`): values of
//! special functions with 35–40 significant digits (computed by Boost in multiprecision arithmetic). The `.ipp` files are as is from
//! `boost_1_92_0/libs/math/test/`. Metric — the largest relative error in units of eps = 2^−52 over the points
//! where the exact value is a normal double; the input is taken as the nearest double (the same x as in Boost).
//!
//! `cargo test --release --test ext_boost -- --nocapture` prints the table.

use mlab::Interp;

const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/ext/boost");

/// Array `name` from the file: lines `{{ SC_(a), SC_(b), SC_(c) }}`.
fn table(file: &str, name: &str) -> Vec<Vec<f64>> {
    let text = std::fs::read_to_string(format!("{DIR}/{file}")).unwrap_or_else(|e| panic!("{file}: {e}"));
    let start = text.find(&format!("> {name} = ")).unwrap_or_else(|| panic!("{file}: array {name}"));
    let body = &text[start..];
    let end = body.find("} };").unwrap_or(body.len());
    body[..end]
        .lines()
        .filter(|l| l.contains("SC_("))
        .map(|l| {
            l.split("SC_(")
                .skip(1)
                .map(|t| t[..t.find(')').unwrap()].trim().parse::<f64>().unwrap_or_else(|_| panic!("{file}: number in «{l}»")))
                .collect()
        })
        .collect()
}

/// Values of the function `f` at the points `xs` via the interpreter (`printf('%.17g')`).
fn mlab_eval(f: &str, xs: &[f64], fault: Option<&str>) -> Vec<f64> {
    let mut code = String::from("x = [");
    for x in xs {
        code += &format!("{x:e} ");
    }
    code += &format!("];\nprintf('%.17g\\n', {f}(x));\n");
    let mut it = Interp::capture();
    it.fault = fault.map(str::to_string);
    it.run(&code);
    let out = it.take_output();
    let v: Vec<f64> = out
        .lines()
        .map(|l| match l.trim() {
            "Inf" => f64::INFINITY,
            "-Inf" => f64::NEG_INFINITY,
            "NaN" => f64::NAN,
            t => t.parse().unwrap_or_else(|_| panic!("{f}: line «{l}»")),
        })
        .collect();
    assert_eq!(v.len(), xs.len(), "{f}: {out}");
    v
}

struct Stat {
    n: usize,
    max_eps: f64,
    worst_x: f64,
}

fn measure(f: &str, rows: &[Vec<f64>], col: usize, fault: Option<&str>) -> Stat {
    let rows: Vec<&Vec<f64>> = rows.iter().filter(|r| r[col].is_normal()).collect();
    let xs: Vec<f64> = rows.iter().map(|r| r[0]).collect();
    let got = mlab_eval(f, &xs, fault);
    let mut st = Stat { n: rows.len(), max_eps: 0.0, worst_x: f64::NAN };
    for (r, g) in rows.iter().zip(&got) {
        let want = r[col];
        let e = if g.is_nan() { f64::INFINITY } else { ((g - want) / want).abs() / f64::EPSILON };
        if e > st.max_eps || e.is_nan() {
            st.max_eps = e;
            st.worst_x = r[0];
        }
    }
    st
}

/// (function, file, array, value column, error bound in eps)
const CASES: &[(&str, &str, &str, usize, f64)] = &[
    ("erf", "erf_small_data.ipp", "erf_small_data", 1, 4.0),
    ("erfc", "erf_small_data.ipp", "erf_small_data", 2, 4.0),
    ("erf", "erf_data.ipp", "erf_data", 1, 4.0),
    ("erfc", "erf_data.ipp", "erf_data", 2, 4.0),
    ("erf", "erf_large_data.ipp", "erf_large_data", 1, 4.0),
    ("erfc", "erf_large_data.ipp", "erf_large_data", 2, 4.0),
    ("gamma", "test_gamma_data.ipp", "factorials", 1, 4.0),
    ("gamma", "test_gamma_data.ipp", "near_0", 1, 4.0),
    ("gamma", "test_gamma_data.ipp", "near_1", 1, 4.0),
    ("gamma", "test_gamma_data.ipp", "near_2", 1, 4.0),
    ("gamma", "test_gamma_data.ipp", "near_m10", 1, 4.0),
    ("gamma", "test_gamma_data.ipp", "near_m55", 1, 4.0),
];

#[test]
fn boost_special_functions() {
    let mut bad = Vec::new();
    println!("function\tBoost array\tpoints\tmax. error, eps\tcorrect digits\tworst x\tbound, eps");
    for &(f, file, name, col, limit) in CASES {
        let st = measure(f, &table(file, name), col, None);
        let digits = if st.max_eps == 0.0 { 16.0 } else { -(st.max_eps * f64::EPSILON).log10() };
        println!("{f}\t{name}\t{}\t{:.1}\t{digits:.1}\t{:e}\t{limit}", st.n, st.max_eps, st.worst_x);
        if !(st.max_eps <= limit) {
            bad.push(format!("{f} on {name}: {:.3e} eps (x = {:e}) > {limit}", st.max_eps, st.worst_x));
        }
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// Negative control: a broken `erf` (+1 to the first result) must turn red.
#[test]
fn boost_negative_control_broken_erf_is_red() {
    let st = measure("erf", &table("erf_data.ipp", "erf_data"), 1, Some("erf"));
    assert!(st.max_eps > 1e6, "broken erf did not turn red: {}", st.max_eps);
}
