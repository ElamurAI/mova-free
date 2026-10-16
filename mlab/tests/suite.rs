//! End-to-end tests: the frozen suite (hash and result), negative controls (a broken builtin,
//! a wrong answer injected before the gate), suite errata.

use mlab::{Interp, suite};

const SUITE: &str = include_str!("../data/suite-v1.txt");
/// Suite hash, recorded in the worklog on 26.09 before the interpreter code (re-computed 01.10 after translating comments and titles to English; expected outputs unchanged).
const FROZEN_SHA256: &str = "2d79564ac44e9592a21eb58a23c1d10a9222193ef3c0003dacb0b0a5864246e2";
/// Scenarios with an error in the suite itself (not in the interpreter); see README «Errata».
const ERRATA: &[&str] = &["la-18"];

fn sha256_hex(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01,
        0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
        0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116, 0x1e376c08,
        0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut h: [u32; 8] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19];
    let mut msg = data.to_vec();
    let bitlen = (data.len() as u64) * 8;
    msg.push(0x80);
    while msg.len() % 64 != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bitlen.to_be_bytes());
    for chunk in msg.chunks(64) {
        let mut w = [0u32; 64];
        for t in 0..16 {
            w[t] = u32::from_be_bytes([chunk[4 * t], chunk[4 * t + 1], chunk[4 * t + 2], chunk[4 * t + 3]]);
        }
        for t in 16..64 {
            let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
            let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
            w[t] = w[t - 16].wrapping_add(s0).wrapping_add(w[t - 7]).wrapping_add(s1);
        }
        let mut v = h;
        for t in 0..64 {
            let s1 = v[4].rotate_right(6) ^ v[4].rotate_right(11) ^ v[4].rotate_right(25);
            let ch = (v[4] & v[5]) ^ (!v[4] & v[6]);
            let t1 = v[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[t]).wrapping_add(w[t]);
            let s0 = v[0].rotate_right(2) ^ v[0].rotate_right(13) ^ v[0].rotate_right(22);
            let maj = (v[0] & v[1]) ^ (v[0] & v[2]) ^ (v[1] & v[2]);
            let t2 = s0.wrapping_add(maj);
            v = [t1.wrapping_add(t2), v[0], v[1], v[2], v[3].wrapping_add(t1), v[4], v[5], v[6]];
        }
        for k in 0..8 {
            h[k] = h[k].wrapping_add(v[k]);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

#[test]
fn sha256_self_check() {
    assert_eq!(sha256_hex(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
}

#[test]
fn suite_is_frozen() {
    assert_eq!(sha256_hex(SUITE.as_bytes()), FROZEN_SHA256, "data/suite-v1.txt changed after freezing");
}

#[test]
fn frozen_suite_passes_except_errata() {
    let sc = suite::parse(SUITE);
    assert_eq!(sc.len(), 206);
    let res = suite::run(&sc, None);
    let failed: Vec<&str> = res.iter().filter(|r| !r.pass).map(|r| r.id.as_str()).collect();
    for r in res.iter().filter(|r| !r.pass && !ERRATA.contains(&r.id.as_str())) {
        eprintln!("✗ {}: {:?}\n{}", r.id, r.first_diff, r.got);
    }
    assert_eq!(failed, ERRATA, "red scenarios outside the errata");
}

#[test]
fn errata_la18_corrected_passes() {
    // la-18 with a nonsingular A (det = 6): identity det(AB) = det(A)det(B) and inv(A)·A = I
    let src = "A = [2 0 1; 1 3 2; 1 1 2];\nB = [1 2 0; 0 1 3; 2 0 1];\ndisp(abs(det(A * B) - det(A) * det(B)) < 1e-10)\ndisp(norm(inv(A) * A - eye(3)) < 1e-12)\n";
    assert_eq!(mlab::run_capture(src), "1\n1\n");
}

#[test]
fn negative_control_broken_builtin_turns_suite_red() {
    let sc = suite::parse(SUITE);
    for (fault, must_fail) in [("sum", "red-01"), ("max", "red-03"), ("det", "la-01"), ("sqrt", "elem-01")] {
        let res = suite::run(&sc, Some(fault));
        let failed: Vec<&str> = res.iter().filter(|r| !r.pass && !ERRATA.contains(&r.id.as_str())).map(|r| r.id.as_str()).collect();
        assert!(failed.contains(&must_fail), "broken {fault}: {must_fail} should have turned red, red: {failed:?}");
    }
}

fn run_with_fault(src: &str, fault: Option<&str>) -> (String, usize) {
    let mut it = Interp::capture();
    it.fault = fault.map(|s| s.to_string());
    it.run(src);
    let n = it.gate_warnings;
    (it.take_output(), n)
}

#[test]
fn gates_silent_on_good_answers_and_loud_on_wrong_ones() {
    let cases = [
        ("gate:mldivide", "x = [4 -2 1; -2 4 -2; 1 -2 4] \\ [11; -16; 17];", "gate mldivide"),
        ("gate:mldivide", "x = [1 0; 1 1; 1 2] \\ [1; 3; 5];", "gate mldivide: least squares"),
        ("gate:inv", "B = inv([4 7; 2 6]);", "gate inv"),
        ("gate:eig", "[V, D] = eig([1 2; 3 4]);", "gate eig: |A*V - V*D|"),
        ("gate:eig", "[V, D] = eig([2 1; 1 2]);", "gate eig: |A*V - V*D|"),
        ("gate:eig", "e = eig(magic(4));", "gate eig: |sum(eig(A)) - trace(A)|"),
        ("gate:svd", "[U, S, V] = svd([3 0; 0 4; 0 0]);", "gate svd: |U*S*V' - A|"),
        ("gate:svd", "s = svd(magic(3));", "gate svd: |sum(s.^2)"),
    ];
    for (fault, src, needle) in cases {
        let (clean, n0) = run_with_fault(src, None);
        assert_eq!(n0, 0, "the gate should stay silent on a correct answer: {src}\n{clean}");
        assert!(!clean.contains("gate"), "{clean}");
        let (bad, n1) = run_with_fault(src, Some(fault));
        assert!(n1 >= 1 && bad.contains(needle), "the gate did not catch the injected error ({fault}) in «{src}»: {bad}");
    }
}

#[test]
fn gate_residual_catches_wrong_solution_directly() {
    // injected wrong answer: x(1) += 1 + |x(1)| before the gate
    let (out, n) = run_with_fault("x = [2 1; 1 3] \\ [3; 5]", Some("gate:mldivide"));
    assert_eq!(n, 1);
    assert!(out.starts_with("warning: gate mldivide: |A*x - b| / (|A|*|x| + |b|) = "), "{out}");
}

#[test]
fn parse_error_runs_nothing() {
    let out = mlab::run_capture("disp(1)\nx = (1 + 2\n");
    assert_eq!(out, "parse error near line 2, column 11: expected ')'\n");
}

#[test]
fn deterministic_rand_default_seed() {
    let a = mlab::run_capture("x = rand(1, 3); disp(x)");
    let b = mlab::run_capture("x = rand(1, 3); disp(x)");
    assert_eq!(a, b);
    let c = mlab::run_capture("rng(5); x = rand(1, 3); rng(5); y = rand(1, 3); disp(isequal(x, y))");
    assert_eq!(c, "1\n");
}

#[test]
fn magic_squares_are_magic() {
    for n in 3..=12 {
        let src = format!(
            "M = magic({n}); s = {n} * ({n}^2 + 1) / 2; disp(all(sum(M) == s) && all(sum(M, 2) == s) && trace(M) == s && sum(diag(fliplr(M))) == s && isequal(sort(M(:))', 1:{n}^2))"
        );
        assert_eq!(mlab::run_capture(&src), "1\n", "magic({n})");
    }
}

// ---------- v2: tables, files, statistics ----------

const SUITE_V2: &str = include_str!("../data/suite-v2.txt");
/// Re-hashed 01.10 after translating comments, titles and the expected statistical report text to English.
const FROZEN_V2_SHA256: &str = "c9380bf75154a2506c9538b2ac8e2d5318692f19c682af24f2ab77057d9e7c87";
/// File suite (vfs, 26.09): written, expected output checked by hand, frozen before the WASI host (re-hashed 01.10 after translating comments and titles to English).
const SUITE_FS: &str = include_str!("../data/suite-fs.txt");
const FROZEN_FS_SHA256: &str = "5e22079e12706934980708f899f905e073ee6fdfc6a8bce022af451fb21c1c5f";

#[test]
fn suite_v2_is_frozen() {
    assert_eq!(sha256_hex(SUITE_V2.as_bytes()), FROZEN_V2_SHA256, "data/suite-v2.txt changed after freezing");
}

#[test]
fn suite_fs_is_frozen() {
    assert_eq!(sha256_hex(SUITE_FS.as_bytes()), FROZEN_FS_SHA256, "data/suite-fs.txt changed after freezing");
}

#[test]
fn frozen_suite_v2_passes() {
    let sc = suite::parse(SUITE_V2);
    assert_eq!(sc.len(), 44);
    let red: Vec<String> = suite::run(&sc, None).into_iter().filter(|r| !r.pass).map(|r| format!("{} {:?}", r.id, r.first_diff)).collect();
    assert!(red.is_empty(), "red: {red:?}");
}

#[test]
fn negative_control_v2_broken_builtins_turn_red() {
    let sc = suite::parse(SUITE_V2);
    for f in ["mean", "median", "quantile", "skewness", "normcdf", "tinv", "chi2cdf", "binopdf", "ttest", "datenum", "histcounts"] {
        let red = suite::run(&sc, Some(f)).iter().filter(|r| !r.pass).count();
        assert!(red >= 1, "broken {f} did not turn suite v2 red");
    }
}

#[test]
fn mc_gate_green_on_true_p_and_red_on_wrong_p() {
    let rows = mlab::mcgate::run(20000, 20260926, None);
    let red: Vec<String> = rows.iter().filter(|r| !r.ok).map(|r| format!("{} α={} {:.4}", r.test, r.alpha, r.frac)).collect();
    assert!(red.is_empty(), "Monte Carlo gate is red on correct p-values: {red:?}");
    for t in ["ttest", "ttest2", "anova1", "corr", "fitlm", "chi2test"] {
        let rows = mlab::mcgate::run(5000, 20260926, Some(&format!("p:{t}")));
        assert!(rows.iter().any(|r| !r.ok), "wrong p in {t} not caught");
    }
}

#[test]
fn fitlm_gate_silent_on_good_and_loud_on_wrong_coefficients() {
    let code = "x = [1;2;3;4;5]; y = [1.1;2.3;2.8;4.2;5.1]; C = fitlm(x, y, 'Display', 'off');\n";
    let (_, gw) = run_with_fault(code, None);
    assert_eq!(gw, 0);
    let (out, gw) = run_with_fault(code, Some("gate:fitlm"));
    assert!(gw >= 1 && out.contains("gate fitlm"), "{out}");
}

#[test]
fn csv_and_json_errors_name_line_and_column() {
    let e = |r: Result<mlab::value::Table, mlab::MError>| r.err().map(|e| e.msg).unwrap_or_default();
    assert_eq!(e(mlab::fileio::read_csv_table("a,b\n1,2\n3,\"x\n", ',', "f")), "f: line 3, column 3: unterminated quoted field");
    assert_eq!(e(mlab::fileio::read_csv_table("a,b\n1\n", ',', "f")), "f: line 2, column 2: expected 2 fields (as in header), found 1");
    assert_eq!(e(mlab::fileio::read_csv_table("a,b\n\"x\"y,2\n", ',', "f")), "f: line 2, column 4: unexpected character after closing quote");
    let j = mlab::fileio::parse_json("[{\"a\": 1},\n {\"a\": }]", "f").err().map(|e| e.msg).unwrap_or_default();
    assert_eq!(j, "f: line 2, column 8: invalid JSON value");
}

#[test]
fn csv_round_trip_keeps_types_and_tricky_text() {
    let code = r#"f = [tempname() '.csv'];
T = table([1.5; NaN; -3], {'a,b', 'say "hi"', ' pad'}, [true; false; true], {'007', 'true', ''}, 'VariableNames', {'x','s','ok','code'});
T.d = datenum({'2026-09-26', '2024-02-29', '1970-01-01'});
writetable(T, f); R = readtable(f);
printf('%d %d %d\n', isequal(R.x(1), 1.5), isnan(R.x(2)), R.x(3) == -3);
printf('%d %d %d\n', R.s(1) == "a,b", R.s(2) == 'say "hi"', R.s(3) == " pad");
printf('%d %d %d\n', isequal(R.ok, T.ok), R.code(1) == "007", R.code(3) == "");
printf('%d\n', isequal(R.d, T.d));
disp(class(R.code))
"#;
    let (out, _) = run_with_fault(code, None);
    assert_eq!(out, "1 1 1\n1 1 1\n1 1 1\n1\nstring\n", "{out}");
}
