//! Equivalence gate of the register VM v1: the same output as the tree-walker — byte for byte, including
//! warnings and errors. Frozen suites v1 (206) and v2 (44), edge cases, negative controls.

use mlab::{Interp, suite};

const SUITE_V1: &str = include_str!("../data/suite-v1.txt");
const SUITE_V2: &str = include_str!("../data/suite-v2.txt");

fn run(src: &str, vm: bool, fault: Option<&str>) -> (String, Interp) {
    let mut it = Interp::capture();
    it.vm_mode = vm;
    it.fault = fault.map(|s| s.to_string());
    it.run(src);
    (it.take_output(), it)
}

#[test]
fn vm_gate_frozen_suites_byte_identical() {
    for (name, text) in [("suite-v1", SUITE_V1), ("suite-v2", SUITE_V2)] {
        let sc = suite::parse(text);
        let r = suite::vm_gate(&sc, None);
        if let Some((id, line, t, v)) = &r.mismatch {
            panic!("{name}: VM diverged from the tree in {id}, line {line}: tree «{t}», VM «{v}»");
        }
        assert_eq!(r.same, sc.len(), "{name}");
        eprintln!(
            "{name}: {} scenarios, same output {}, fully in VM {}, instructions in VM {}, fallback {} (times {}, DynEval {})",
            r.total, r.same, r.fully_native, r.native, r.fallback, r.fallback_runs, r.dyn_runs
        );
    }
}

#[test]
fn vm_gate_equal_under_builtin_faults_too() {
    // a broken builtin breaks the tree and the VM alike — the gate compares behaviour, not «correctness»
    for fault in ["sum", "max", "gate:mldivide", "mean"] {
        for text in [SUITE_V1, SUITE_V2] {
            let r = suite::vm_gate(&suite::parse(text), Some(fault));
            assert!(r.mismatch.is_none(), "fault {fault}: {:?}", r.mismatch);
        }
    }
}

#[test]
fn vm_gate_negative_control_turns_red() {
    // An error injected into the VM (scalar addition +1) must turn the gate red: a gate that cannot show red is not a gate.
    let sc = suite::parse(SUITE_V1);
    let r = suite::vm_gate(&sc, Some("vm:add"));
    assert!(r.mismatch.is_some(), "the gate did not see the broken VM");
    let red = sc
        .iter()
        .filter(|s| run(&s.code, false, Some("vm:add")).0 != run(&s.code, true, Some("vm:add")).0)
        .count();
    eprintln!("vm:add → red v1 scenarios: {red}");
    assert!(red >= 5, "too few red: {red}");
}

const PRELUDE_FNS: &str = r#"
function r = fib(n)
  if n < 2
    r = n;
  else
    r = fib(n-1) + fib(n-2);
  end
end
function r = addb(a, b)
  if nargin < 2
    b = 10;
  end
  r = a + b;
end
function [a, b] = two()
  a = nargout;
  b = 2;
end
function [a, b] = half()
  a = 1;
end
function noret(x)
  y = x;
end
function r = deep(n)
  if n == 0
    r = 0;
  else
    r = 1 + deep(n - 1);
  end
end
function r = first_big(v)
  r = -1;
  for k = 1:numel(v)
    if v(k) > 2
      r = k;
      return;
    end
  end
end
function r = sq(x)
  r = x.^2;
end
function r = kk()
  r = 2;
end
function r = kk2()
  r = 4;
end
function r = later2(n)
  r = n * 100;
end
function r = later()
  if false
    h = 1;
  end
  r = h;
end
"#;

/// Semantic edge cases: each line is a separate program (an error stops only that one).
const EDGE: &[&str] = &[
    "x = sin(1); sin = 5; y = sin(1)\nclear sin\nz = sin(0)",
    "a = fib + 1",
    "r = later()",
    "v = 1:5; v(min(3,end))\nv(end-1:end)\nA = magic(4); A(end, end-1)\nv(end+1) = 6",
    "for k=1:5\n try\n  if k==3\n   break;\n  end\n catch\n end\n disp(k)\nend\ndisp(k)",
    "for k=1:4\n switch k\n  case 2\n   continue;\n  case 3\n   break;\n end\n disp(k)\nend",
    "deep(200)\ndeep(300)",
    "addb(1)\naddb(1, 2)\naddb(1, 2, 3)",
    "[m, i] = max([3 1 4 1 5])\n[~, i] = sort([3 1 2])\n[a, b] = size(zeros(2,3))",
    "[p, q] = two()\n[x, y] = half()",
    "x = noret(3)",
    "noret(3)\nnoret(3);\nans",
    "a = 2; g = @(x) x*a; a = 3; g(2)\nf = @(x) @(y) x + y; h = f(2); h(3)",
    "x += 1",
    "x = 5\nx\n3 + 4\n[1 2 3]\ntrue\n'abc'\nx(2) = 7\ny = x';\ny",
    "v = 1:5; v(v > 2)\nv(logical([1 0 1 0 1])) = 0\nv(v > 2) = -1",
    "z(3) = 1\nM(2,3) = 5\nw = []; w(end+1) = 1; w(end+1) = 2",
    "k = 0;\ndo\n k++;\n if k == 2\n  continue;\n end\n disp(k)\nuntil k >= 4",
    "k = 0; s = 0;\nwhile k < 5 && s < 100\n k = k + 1; s = s + k^3;\nend\n[k s]",
    "if [] disp(1); else disp(0); end\nif [1 1 0] disp(1); else disp(0); end\nif 'a' disp(1); end",
    "'a' + 1\n'a':'e'\nc = 'hello'; c(1) = 'J'",
    "z = 3 + 4i; abs(z)\n(-8)^(1/3)\nz'\nw = [1+2i 3]; w(2)",
    "for c = [1 2; 3 4]\n disp(c')\nend\nfor k = []\n disp(1)\nend\nfor k = zeros(0,3)\n disp(2)\nend\nfor k = 7\n disp(k)\nend\nfor ch = 'ab'\n disp(ch)\nend",
    "n = 3;\nfor k = 1:n\n n = 10; k = k * 100;\n disp(k)\nend\nk",
    "undefined_fn(3)",
    "x = [1 2 3]; x(5)",
    "x = [1 2 3]; x(0)",
    "x = [1 2 3]; x(1.5)",
    "[1 2] + [1 2 3]",
    "3; ans + 1",
    "f = @sin; f(0)\nh = @fib; h(10)\nfeval(@fib, 12)\narrayfun(@sq, 1:3)",
    "T = table([1;2;3], [4;5;6], 'VariableNames', {'a','b'});\ns = 0;\nfor k = 1:3\n s = s + T.a(k) * T.b(k);\nend\ns\nT(2, :)",
    "first_big([1 2 3 4])\nfirst_big([1 1])",
    "x = 1; x(2,2) = 5\nA = eye(3); B = A; B(1,1) = 9; disp(A(1,1)); disp(B(1,1))",
    "s = 0; for k = 1:0.1:2\n s = s + k;\nend\nprintf(\"%.17g\\n\", s)\nfor k = 10:-3:1\n disp(k)\nend",
    "x = 1; exist('x')\nexist('nothing_here')",
    "clear x\nx = 3;\nclear x\nx",
    "a.b = 1",
    "v = [1 2 3]; v(2) = []\nv(:) = []\nx = 5; x(1) = []",
    "x = 5; x(1)\nx(1,1)\nx(2)",
    "try\n fib()\ncatch e\n disp(e)\nend\nlasterr",
    "format long\npi\nx = 1/3\nformat short\nx",
    "1:3 == 1:3\nif 1:3 == 1:3 disp('eq'); end\nif [1 2] < [3 4] disp('lt'); end\nif [1 5] < [3 4] disp('no'); else disp('else'); end",
    "if [1 2] < [1 2 3] disp(1); end",
    "x = 3; x'\n~isempty([])\n!true\nclass(-true)\nclass(+true)\n-[1 2]",
    "for k = 1:3\nend\nk\nclear k\nfor k = []\nend\nk",
    "disp(1); break; disp(2)",
    "for i = 1:2\n z = i * 2;\nend\nz\nclear i\nq = i * 2",
    "e\ne = 5; e + 1",
    "v = [1 2 3]; v(2) += 5\nv(end) *= 2\nv(4) -= 1",
    "x = 2; [x, x+1; 2*x, x^2]\nA = magic(3); A(:, 1)\nA(2, :) = []\nA(:)'",
    "if [] && true disp(1); end",
    "[1 2] || 1",
    "max([1 5 3])\n[1 2 3](2)",
    "x = 1; x(3) = 2\ny = true; y(3) = true\nclass(y)",
    "m = zeros(3); for i = 1:3\n for j = 1:3\n  m(i, j) = i * 10 + j;\n end\nend\nm\nsum(m(:))",
    "t = 0; for k = 1:10\n if mod(k, 2) == 0\n  continue\n end\n t = t + k;\nend\nt",
    "a = 5; b = a; b += 1; [a b]\nc = 'x'; c += 1",
    "x = int8(5)",
    "s = struct('a', 1)",
    "q = {1, 2}",
    "function_does_not_exist",
    "x = 10; while x > 0\n x = x - 3;\n if x < 4, break; end\nend\nx",
    "r = 0; for k = 1:3\n for m = 1:3\n  if m == 2, break; end\n  r = r + k * m;\n end\nend\nr",
    "p = 2^0.5\nq = (-2)^2\nr = (-2)^0.5\nNaN == NaN\nInf - Inf",
    "x = 0; x(2) = 1i\nclass(x)",
    "v = 1:3; v(2)++\nv",
    "A = [1 2; 3 4]; A(2, 3) = 7\nA(:, end+1) = [8; 9]",
    "u = 1; u(1, 1, 1) = 3\nu(1, 1, 2) = 4",
    "g = @(x) x + 1; g(1, 2)",
    "fib(1, 2)",
    "[a, b, c] = fib(3)",
    "v = [10 20 30];\nv(kk)\nkk = 1;\nv(kk)\nw(kk) = 5\nw(kk2) = 6\nkk2 = 3;",
    "x = 2; x(x) = 5\nA = [3 1 2]; A(A)\nA(A) = [7 8 9]",
    "T = table([1;2;3], [4;5;6], 'VariableNames', {'a','b'});\nk = 2; j = 1;\nT(k, j)\nT(k, :)\nT(1, 1) = 5",
    "S = {'aa', 'bb', 'cc'};\nk = 2;\nS(k)\nS(k) = 1",
    "g = @(a, b) a + b; x = 1; y = 2; g(x, y)\ng(x)\nh = @() 42; h()",
    "p = later2(3)\nlater2 = [5 6 7];\nq = later2(3)",
    "M = magic(4); i = 2; j = 3;\nM(i, j)\nM(i, :)\nM(:, j)'\nM(i, j) = -1;\nM(j, i) = true;\nM(5, 5) = 1\nM(0, 1)",
    "L = logical([1 0 1]); k = 2; L(k)\nL(k) = 1\nclass(L)\nL(k) = 0.5\nclass(L)",
    "C = 'hello'; k = 1; C(k)\nC(k) = 72\nZ = [1+2i 3]; k = 1; Z(k)\nZ(k) = 5",
    "x = 1:5; y = 2 * x\nz = 1 - x\nw = 2 ^ x\nq = 10 / 4\nr = 1 - true\ns = 2 ^ 0.5",
    "k = 3; v = zeros(1, k); for i = 1:k\n v(i) = i^2;\nend\nv\nv(k+1)",
];

#[test]
fn vm_edge_cases_match_tree() {
    // recursion to the limit of 256 in a debug build of the tree needs > 8 MB of stack (the VM — less), hence a separate thread
    std::thread::Builder::new().stack_size(512 << 20).spawn(edge_cases).unwrap().join().unwrap();
}

fn edge_cases() {
    let mut red = Vec::new();
    for (k, body) in EDGE.iter().enumerate() {
        let src = format!("{body}\n{PRELUDE_FNS}");
        let (t, _) = run(&src, false, None);
        let (v, _) = run(&src, true, None);
        if t != v {
            red.push(format!("#{k} «{}»\n--- tree ---\n{t}--- VM ---\n{v}", body.lines().next().unwrap_or("")));
        }
    }
    assert!(red.is_empty(), "mismatches ({}):\n{}", red.len(), red.join("\n"));
}

#[test]
fn vm_really_compiles_hot_loops() {
    // a scalar loop, nested loops over a matrix and recursion — without a single fallback to the tree
    let src = "s = 0;\nfor k = 1:1000\n s = s + k;\nend\nA = ones(20); B = zeros(20);\nfor i = 1:20\n for j = 1:20\n  B(i,j) = A(i,j) * 2 + 1;\n end\nend\ndisp(s); disp(sum(B(:))); disp(fib(15))\n"
        .to_string()
        + PRELUDE_FNS;
    let (out, it) = run(&src, true, None);
    assert_eq!(out, "500500\n1200\n610\n");
    assert_eq!(it.vm_stats.fallback_runs, 0);
    assert_eq!(it.vm_stats.dyn_runs, 0);
    assert_eq!(it.vm_stats.fallback, 0);
}

#[test]
fn vm_workspace_is_visible_after_run() {
    let mut it = Interp::capture();
    it.vm_mode = true;
    it.run("x = 41;");
    it.run("x = x + 1; disp(x)");
    assert_eq!(it.take_output(), "42\n");
}
