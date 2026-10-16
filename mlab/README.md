# mlab — a computing language in the spirit of MATLAB/Octave

**Goal** (26.09): copy the logic of Octave and build a mini-competitor to MATLAB. v1 is an interpreter for a subset
of the language in Rust: matrices, 1-based indexing, colon, `end`, element-wise operations, functions, control flow, linear
algebra, polynomials, FFT, numerical methods. Every linear algebra result passes a **verification gate**:
a wrong answer produces a warning, not silence.

This is the natural plan language for the LLM (Opus writes such code fluently), while the SLM executes it (`docs/context.md`, "Mini-lab").
Our difference from Octave is the gates now, and an exact mode and steps later.

## Clean room: legal

- **Octave is GPL.** We take only behaviour from the public MATLAB/Octave documentation: syntax, semantics, output
  format. Octave code was not read, copied or translated, and no MATLAB code was taken either. The output format rules
  below are written down as our own specification based on the documentation and output examples.
- **Language functionality and APIs are not protected by copyright:** EU — SAS Institute v WPL (2012),
  US — Google v. Oracle (2021).
- **The name "MATLAB" is not used in the product** — it is a trademark. The crate and CLI are `mlab`, files are `.m`
  (a file extension is not a trademark).
- **Dependencies are permissive only:** the table below; each license was checked against the crates.io API.
- Algorithms come from published papers (Brent, Nelder–Mead / Lagarias, Dormand–Prince, Gauss–Kronrod, Hager /
  Higham, Shampine); constants (quadrature nodes and weights, Butcher coefficients) are mathematical facts.

## How it works

| module | what it does |
|---|---|
| `lexer.rs` | tokens with position and a "space before" flag; an apostrophe is a transpose or a string depending on context; `%`/`#`, `%{ … %}`, `...` |
| `parser.rs` | recursive descent with the language's precedences; a space inside `[...]` separates elements (`[1 -2]` — two, `[1 - 2]` — one); `end` in an index; command syntax (`format long`) |
| `ast.rs` | tree: expressions, statements, functions |
| `value.rs` | `Value`: `Mat` (f64 column-major + class double/logical/char + optional imaginary part), `Func` (named/anonymous with capture), **`Table`** (v2: named columns of different types), **`Exact`** (v2: exact mode; an empty enum — the slot already exists in every `match`) |
| `interp.rs` | workspace and call frames, execution, `end` context, calls (variable → script function → builtin), output |
| `ops.rs` | operators with implicit expansion (broadcasting), complex numbers, matrix multiplication (faer for large ones), division, power, ranges, concatenation |
| `index.rs` | reading `A(I)`, `A(I,J)`; assignment with growth; deletion `A(I) = []` |
| `display.rs` | Octave-style `x = …` and `disp` output; printing of anonymous functions |
| `builtins/` | **an extensible registry** (`Registry::register`): `core`, `elem`, `reduce`, `io`, `linalg`, `poly`, `fft`, `numeric`; v2 — `table`, `stats` |
| `table.rs`, `fileio.rs`, `mcgate.rs` | v2: tables (columns, output, keys, dates), CSV/TSV/JSON, a Monte Carlo gate for p-values |
| `rng.rs` | xoshiro256** + splitmix64, normals by Marsaglia's polar method; deterministic with a seed |
| `suite.rs`, `main.rs` | scenario suite; CLI |
| `vm.rs` | **VM v1** (26.09): a compiler from the tree to register bytecode and an executor; fallback to the tree is transparent (section "VM v1") |
| `builtins/files.rs` | **vfs** (26.09): files and folders via the `vfs::FileSystem` trait; all file builtins go through `Interp::fs` (section "File system: vfs") |

## The v1 language

- Numbers (`3`, `.5`, `1e-3`, `2i`), strings `'…'` (`''` is a quote) and `"…"` (escapes `\n \t \\`, as in Octave).
- Matrices `[1 2; 3 4]`: spaces and commas separate elements, `;` and newline separate rows; concatenation with size checks.
- Ranges `a:b`, `a:s:b` (the end is exactly `b`), character ranges `'a':'e'`.
- Indexing `A(i,j)`, `A(:,2)`, `A(:)`, `end` (including in function arguments inside an index: `v(min(3,end))`),
  logical masks, linear; result shape by the language rules; assignment with growth (`x(5) = 1`, `A(3,3) = 9`),
  deletion (`A(:,2) = []`), `+= -= *= /=`, `x++`/`x--` (Octave).
- Operators `+ - * / \ ^ .* ./ .\ .^ ' .'`, comparisons (`~=` and `!=`), `& | ~ !`, `&& ||` (scalar, short-
  circuit); power is left-associative, `-2^2 = -4`, `2^-1`.
- Anonymous functions `@(x) x.^2` capturing values at creation; handles `@sin`, `feval`, `arrayfun`, `func2str`.
- `if/elseif/else`, `for` (over columns), `while`, `do … until`, `break`, `continue`, `return`, `switch` (numbers,
  strings, lists `{…}`), `try/catch` (with an identifier — the message string), `lasterr`.
- User functions at the end of a script (like MATLAB local functions) or anywhere: multiple outputs, `[~, i] = …`,
  `nargin`/`nargout`, recursion (depth up to 256), terminated by `end`/`endfunction` or without them.
- Octave terminators: `endif endfor endwhile endswitch endfunction end_try_catch`.
- `;` suppresses output; otherwise `x = 3` or an `A =` block with aligned columns; wider than 80 characters — in blocks
  `Columns 1 through 16:`.
- Values: f64 matrices, logical, char; complex — arithmetic, `abs real imag conj angle`, `sqrt`/`log`
  of negatives, power, output, solving square systems; `roots` and `eig` of non-symmetric matrices.
- Not in v1: cell arrays, structs, N-dimensional arrays, integer types (`int8`…), `single` (becomes double), `string` objects.
  These give a clear runtime error, not a crash.

### Builtins (229 in v1; 292 with v2; 300 with vfs)

`mlab builtins` prints the list. Groups:
- **creation:** `zeros ones eye rand randn randi randperm rng linspace logspace magic diag triu tril repmat reshape cat horzcat vertcat meshgrid colon true false`, constants `pi e Inf NaN eps i j realmax realmin`;
- **sizes and types:** `size numel length ndims rows columns isempty isscalar isvector isrow iscolumn issquare class isa isnumeric ischar islogical isreal double logical char isequal exist`;
- **reductions:** `sum prod cumsum cumprod max min mean median std var sort find any all unique diff nnz fliplr flipud flip`;
- **element-wise:** `abs sqrt exp log log2 log10 log1p expm1 sin cos tan asin acos atan atan2 sinh cosh tanh sec csc cot hypot floor ceil round fix mod rem sign gcd lcm factorial nchoosek primes isprime nthroot gamma real imag conj angle complex isnan isinf isfinite xor`;
- **output and strings:** `disp fprintf printf sprintf num2str int2str mat2str strcmp strcmpi upper lower strrep strtrim strcat strfind str2double str2num`;
- **linear algebra:** `\ / inv det lu qr chol eig svd norm rank trace kron dot cross pinv cond`;
- **polynomials:** `polyval polyfit roots conv deconv poly polyder polyint`;
- **spectrum:** `fft ifft` (rustfft);
- **numerical methods:** `integral` (adaptive Gauss–Kronrod 7–15; infinite limits; for finite ones — Shampine's
  substitution, which weakens endpoint singularities), `fzero` (Brent; an interval or an interval search from a point),
  `fminsearch` (Nelder–Mead, Lagarias coefficients, initial simplex +5 %), `ode45` (Dormand–Prince 5(4),
  RelTol 1e-3, AbsTol 1e-6), `trapz`;
- **control:** `error warning lasterr assert feval arrayfun func2str str2func format clear tic toc`.
- **v2 — tables and files:** `table height width summary sortrows groupsummary grouptransform pivot innerjoin join ismissing rmmissing movmean movsum movmax movmin movmedian datenum datestr readtable writetable readmatrix writematrix jsonencode fileread writelines tempname string`;
- **vfs — files and folders:** `isfile isfolder mkdir rmdir delete movefile copyfile readdir`, `exist(…, 'file'|'dir')`;
- **v2 — statistics:** `mode quantile prctile skewness kurtosis cov corrcoef corr histcounts normpdf normcdf norminv tpdf tcdf tinv chi2pdf chi2cdf chi2inv fpdf fcdf finv binopdf binocdf binoinv poisspdf poisscdf poissinv erf erfc ttest ttest2 chi2test crosstab anova1 fitlm`.

### Output format (our specification in the style of Octave `format short`)

- Integers as integers; column width = digits of the largest + 1 (room for the sign), 2 spaces between columns: `   1  -2`.
- Non-integers: d = digits before the point after rounding to 5 significant digits; d ≥ 1 → 5−d decimals (`3.1416`, `12.500`, `100.50`,
  `1234.5`); d = 0 → 4 (`0.5000`); d = −1 → 6 (`0.050000`); d ≤ −2 or d ≥ 5 → e-format (`1.0000e-03`, `1.2346e+04`).
- A matrix has a common format based on the largest and smallest non-zero; an exact zero is `0`; a common format that is too long
  → e-format. `format long` is the same with 16 significant digits (`3.141592653589793`).
- Logical — width 1 (`  1  0  1`); char — as rows; empty — `[](0x3)`; complex — `3 - 4i`,
  `0.5000 + 1.2500i`; anonymous functions — as in Octave: `@(x) sin (x) + 1`.

## Result verification gates

Checking is easier than generating, so the check is a gate. The threshold is 1000 · n · eps; stable algorithms lie
orders of magnitude below, so the gate is silent on correct answers and shouts on wrong ones.

| operation | gate |
|---|---|
| `A \ b` (square) | backward error ‖A·x − b‖ / (‖A‖‖x‖ + ‖b‖) |
| `A \ b` (least squares, singular) | normal equations ‖Aᵀ(A·x − b)‖ / (‖A‖₁(‖A‖‖x‖ + ‖b‖)) |
| `inv(A)` | ‖A·inv(A) − I‖ / (‖A‖‖inv(A)‖) |
| `[V, D] = eig(A)` | ‖A·V − V·D‖ / (‖A‖‖V‖); eigenvalues only — Σλ versus the trace |
| `[U, S, V] = svd(A)` | ‖U·S·Vᵀ − A‖ / ‖A‖; singular values only — Σσ² versus ‖A‖²_F |
| `lu`, `qr`, `chol` | ‖P·A − L·U‖, ‖Q·R − A‖, ‖RᵀR − A‖ relative to ‖A‖ |

On failure — `warning: gate mldivide: |A*x - b| / (|A|*|x| + |b|) = 2.3e-01 exceeds 4.4e-13`. A gate warning
is not suppressed by `warning off`. Separately, as in Octave, — `warning: matrix singular to machine precision` for
a singular matrix (rcond estimate by the Hager–Higham method < eps): then `\` gives the minimum-norm solution (SVD), and
`inv` of an exactly singular matrix gives `Inf`.

**Negative control of the gate:** `Interp::fault = Some("gate:mldivide")` (`gate:inv`, `gate:eig`, `gate:svd`) plants
a wrong answer before the check; the test `gates_silent_on_good_answers_and_loud_on_wrong_ones` requires silence on
correct answers and a warning on planted ones.

## CLI

```
cargo build --release
../target/release/mlab run file.m            # script; output to stdout, warning/error to stderr; exit code 1 on error
../target/release/mlab -e "A = magic(4); rank(A)"
../target/release/mlab test data/suite-v1.txt [--fault sum] [-v]   # scenario suite, summary by category
../target/release/mlab builtins              # list of builtins
../target/release/mlab run examples/demo.m --fault gate:mldivide   # see how the gate catches a planted error
../target/release/mlab run file.m --vm       # the same on the register VM v1 (--vm also works with -e and test)
../target/release/mlab vmgate data/suite-v1.txt data/suite-v2.txt [--fault vm:add]   # VM equivalence gate
../target/release/mlab vmbench examples/vmbench   # tree versus VM, 3 runs, best
../target/release/mlab vmdump file.m         # bytecode listing
../target/release/mlab run file.m --fs mem   # (default) files only in memory; see "File system: vfs"
../target/release/mlab run file.m --fs-import data:/in --fs-export out:/out [--fs-limits file=64M] [--fs-journal j.tsv]
../target/release/mlab run file.m --fs dir:./data   # a real folder, the root is the boundary
../target/release/mlab run file.m --fs real  # as before vfs — only explicitly
```

`examples/demo.m` is a short demo: indexing, `\` with the gate, `eig`, `roots`, `integral` to ∞, user
functions, `ode45`. `--fault NAME` for `run`/`-e`/`test` is a negative control (a broken builtin or a wrong
answer before the gate).

## v1 measurements (26.09)

### Scenario suite

`data/suite-v1.txt` — 206 scenarios with expected output (syntax 15, output 20, matrices 15, indexing 19,
operators 19, control 13, functions 14, reductions 20, element-wise 9, output functions 10, linear algebra 20,
gates 3, polynomials 7, spectrum 6, numerical methods 9, errors 7). Written from the language documentation (clean room),
values come from identities and exact answers (A·inv(A) = I, A·V = V·D, U·S·Vᵀ = A, Fibonacci via A^10, exact
roots, Parseval); where a number depends on rounding, the scenario prints a "< tolerance" check. Octave is not on the box
and cannot be installed. **Frozen by hash before the interpreter code**:
sha256 `2c37701df94db6328546388cc21145c4c8fde48ea4f18aeb00e2e2333ca0b0e6` (test `suite_is_frozen`).

| run | scenarios | correct | red |
|---|---|---|---|
| first, before any fixes | 206 | 205 | 1 — la-18, an error in the suite |
| after the probe and fixes | 206 | 205 | the same la-18 |

**Suite errata.** la-18 takes A = [2 0 1; 1 3 2; 1 1 1], but it is singular (det = 0) — an error in the suite.
The interpreter is right: it warns "matrix singular to machine precision", and the identity inv(A)·A = I does not hold.
The suite is frozen and not edited; the corrected scenario (det = 6) is the test `errata_la18_corrected_passes`.

**Caveat.** The suite and the interpreter were written by the same author, so 205/206 is conformance to our specification,
not an independent yardstick of Octave compatibility. An independent check is in "What next".

**Probe after the code** (5 scripts, ~150 lines, `$MOVA_DATA/runs/mlab-2026-09-26/probe/`): found and fixed —
an extra blank line after column blocks; `x++` was not parsed; `s.a = 1` caused a parse error for the whole
script (now a runtime error "structs are not supported"); `integral` on singularities (1/√x on [0, 1]:
1.9999988 → 1.9999999999998 after Shampine's substitution); `x = f()` for a function without an output; `[1 2] && 1`
(now an error, as in MATLAB).

### Negative controls

- **A broken builtin** (`--fault NAME`: +1 to the first element of the result) turns the suite red: `sum` → 199/206,
  `max` → 195/206, `det` → 204/206, `sqrt` — `elem-01` red (test `negative_control_broken_builtin_turns_suite_red`).
- **A wrong answer planted before the gate**: `gate:mldivide` → 7 scenarios red with a gate warning,
  `gate:eig` → 7, `gate:inv` → 2, `gate:svd` → 1; on correct answers the gates are silent.
- `magic(n)` for n = 3…12: row, column and diagonal sums and a permutation of 1…n² (test `magic_squares_are_magic`).

### Linear algebra backend: faer versus nalgebra

`bench/` is a separate crate, single thread (faer — `Par::Seq`), best of 3 runs, `nice`; raw numbers —
`bench/result-2026-09-26.tsv`.

| task | faer 0.24, s | nalgebra 0.35, s | faer / nalgebra |
|---|---|---|---|
| multiplication 1000×1000 | 0.034 | 0.037 | 0.90 |
| solve 1000×1000 (LU) | 0.017 | 0.103 | 0.17 |
| eig 500×500 non-symmetric, eigenvalues only | 0.080 | 0.213 | 0.38 |
| eig 500×500 non-symmetric, values and vectors | 0.130 | — | nalgebra cannot |
| eig 500×500 symmetric, values and vectors | 0.032 | 0.105 | 0.30 |

Residuals of the same order (‖Ax − b‖ 1.8e-11 / 1.6e-11; max|AV − VD| 1.9e-14 / 4.1e-14).
**faer was chosen:** faster everywhere, and only it gives eigenvectors of non-symmetric matrices (`[V, D] = eig(A)`).

### Interpreter speed (release, single thread)

| task | time |
|---|---|
| loop of 10⁶ scalar additions | 0.28 s |
| 10⁵ assignments `v(k) = k^2` | 0.03 s |
| `A * B`, 1000×1000 | 0.07 s |
| `A \ b`, 1000×1000, with rcond estimate and gate | 0.04 s |
| `eig`, 500×500, values only / with vectors, with gates | 0.11 / 0.21 s |
| the whole suite (206 scenarios) | 0.01 s |

## v2 (26.09): tables, files, statistics

Requested: statistics on top and Excel-like work with tables, export and import; table computations in RAM.

### Tables — columnar, in memory

- **`table`**: named column vectors of different types — numbers (`NaN` is missing), logical, text (class `string`,
  `<missing>`), dates (datenum, as in MATLAB: 719529 = 1970-01-01; printed as ISO). `table(x, y)` takes the names from the
  variable names, `'VariableNames', {'a','b'}`. `{'a','b'}` in v2 is a string array (there are no real cell arrays yet).
- **Access:** `T.x`, `T.x(k)`, `T(rows, vars)` — rows by number, mask, `end`, `:`; variables by name,
  `{'a','b'}`, number. `height width size summary`.
- **Vector operations on columns:** `T.c = T.a .* T.b`, `T.big = T.c > 8`, conditionals via masks
  (`T.d = T.c .* T.big + (-1) .* ~T.big`), `T.x += 1`; a scalar expands to all rows. String arrays —
  `==`/`~=` (filter `T(T.reg == "N", :)`).
- **Operations:** filtering by mask; `sortrows` (stable; multiple keys; `'descend'`; a matrix with a −column);
  `groupsummary` by one or more keys (`sum mean median mode var std min max range nummissing nnz
  numunique`, missing values ignored, `GroupCount`); `grouptransform` (a handle — `@cumsum`,
  `@(v) movmean(v, 3)` — or `meancenter`/`zscore`) within a group; `pivot(T, 'Rows', r, 'Columns', c,
  'DataVariable', v, 'Method', 'sum')`; `innerjoin` (sorts by key), `join` (order of the left table; every key must
  be found, right keys are unique); `ismissing rmmissing`; moving windows `movmean movsum movmax movmin movmedian`
  (endpoints "shrink", as in MATLAB); dates `datenum datestr`.
- **Output** (our specification): `  3x3 table`, names, underlines, numbers right-aligned (column format as for
  matrices), text left-aligned, logical `true`/`false`; more than 30 rows — the first 10 and last 5.

### Import and export

- **CSV/TSV** — our own parser (RFC 4180: quotes, doubled quotes, newlines and delimiters inside quotes, BOM, empty
  lines skipped). The column type is inferred from the data: numbers → logical (`true/false`) → ISO dates → text; an empty unquoted
  field is missing; a quoted field is always text (so `"007"` stays text). Names — as
  `makeValidName`. **An error names the line and column:** `readtable: line 3, column 3: unterminated quoted
  field`, `line 3, column 5: expected 2 fields (as in header), found 3`, `unexpected character after closing
  quote`. Writing quotes text that would otherwise be read as a number, logical or date, so the round trip preserves
  types.
- **JSON** — an array of records (`[{"n": 1, "s": "a"}, …]`, like MATLAB's `jsonencode` of a table); our own parser with
  line and column in errors, `\uXXXX` and surrogate pairs; `jsonencode`.
- `readtable / writetable` (by extension: `.csv .txt` — comma, `.tsv .tab` — tab, `.json`;
  `'Delimiter'`, `'FileType'`), `readmatrix / writematrix` (header skipped, non-numeric becomes `NaN`),
  `fileread writelines tempname string`.
- **XLSX/ODS — not yet** (a clear error; in "What next").

### Statistics

- **Descriptive:** `mode` (ties — the smaller), `quantile / prctile` (MATLAB method: sorted values are the quantiles
  (k − 0.5)/n, linear in between, min/max outside), `skewness / kurtosis` (biased and `flag = 0`), `cov`,
  `corrcoef` (`[R, P]`), `histcounts` (edges or number of bins); `NaN` is missing. v1: `mean median std var`.
- **Distributions** (`statrs`, MIT; scalar parameters expand): `normpdf normcdf norminv tpdf tcdf tinv
  chi2pdf chi2cdf chi2inv fpdf fcdf finv binopdf binocdf binoinv poisspdf poisscdf poissinv erf erfc`.
  **A finding of the suite:** in `statrs` 0.19.1 `erfc` has a relative error of ~5·10⁻¹¹ (`normcdf(−1.96)` =
  0.02499789514709759 versus 0.024997895148220435) — scenario `dist-01` turned red. The normal distribution is now
  our own: `erf` — a series with positive terms (Abramowitz–Stegun 7.1.6), `erfc` for x ≥ 2 — a continued fraction
  (7.1.14, Lentz's method), `norminv` — statrs start + 2 Newton steps; error ~10⁻¹⁵.
- **Tests with p-values:** `[h, p, ci, stats] = ttest(x)`, `ttest(x, m)`, `ttest(x, y)` (paired);
  `ttest2(x, y)` (equal variances by default, as in MATLAB) and `'Vartype', 'unequal'` (Welch–Satterthwaite);
  `[p, chi2, df, E] = chi2test(M)` (χ² test of independence), `[tbl, chi2, p] = crosstab(x, y)`;
  `[p, tbl] = anova1(y, group, 'off')`; `[rho, p] = corr(x, y)`; options `'Alpha'`, `'Tail'`
  (`both/right/left`), `'Display', 'off'`. `stats` is a one-row table (`st.tstat`, `st.df`).
- **OLS regression:** `[C, S] = fitlm(X, y)`, `fitlm(T, 'y ~ a + b')`, `'y ~ a - 1'` (no intercept),
  `fitlm(T)`. `C` is a table `Name Estimate SE tStat pValue`; `S` — `N DFE R2 AdjR2 RMSE F pF SSE`. OLS via
  Householder QR; SE from R⁻¹.
- **Transparency:** every test and `fitlm` without `'Display','off'` prints a report — H0/H1 and α; assumptions (what was
  checked from the data: n, variance ratio, expected frequencies < 5 — and what was not: independence); steps with
  numbers; conclusion and confidence interval. Example:

```
ttest2: two-sample t-test for independent samples (Welch)
  H0: μx = μy; H1: μx ≠ μy (two-sided), α = 0.05
  assumptions:
    independence of observations within and between samples — not checked from the data
    normality of both populations (or large n) — nx = 5, ny = 6
    equal variances not required (Welch); max s²/min s² = 1.80556
  steps:
    x̄ = 5.6, ȳ = 4.4, s²x = 0.325, s²y = 0.18
    SE = √(s²x/nx + s²y/ny) = 0.308221
    t = (x̄ − ȳ)/SE = 3.89331, df (Welch — Satterthwaite) = 7.3003
    p = 2·P(T ≥ |t|) = 0.00548141
  conclusion: p < α → reject H0 (h = 1); 95% CI for μx − μy: [0.477208, 1.92279]
```

### v2 gates

- **Monte Carlo for p** (`mlab mcgate [-k 20000] [--fault p:ttest]`, `src/mcgate.rs`, seed 20260926): under H0
  a correct p is uniform, so the fraction of p ≤ α over K = 20,000 simulations on small samples must be α ± 4.5·√(α(1 − α)/K)
  (+0.004 for approximate tests: Welch, χ²). 8 tests × α ∈ {0.01, 0.05, 0.1, 0.5}: ttest (n = 5), right-tailed (n = 6),
  ttest2 (4 and 7), Welch (6 and 9, σ 1 and 2), anova1 (3 × 4), corr (n = 6), fitlm slope p (n = 7), χ² 2×3
  (n = 300) — **32/32 green**, 0.10 s. Example: ttest n = 5 — 0.0512 at α = 0.05 (±0.0069).
- **A wrong p → red:** `p:ttest` (normal instead of t) — 7 red rows, `p:ttest2` — 7, `p:anova1`
  (χ²/df1 instead of F) — 4, `p:corr` — 4, `p:fitlm` — 4, `p:chi2test` (df + 1) — 3.
- **fitlm versus `\`:** QR coefficients are compared with the interpreter's `X \ y` (with its own gate); a discrepancy
  > 10⁻⁸ gives `warning: gate fitlm: …`. Control `--fault gate:fitlm` — the warning appears; on correct answers — silence.

### v2 suite and controls

`data/suite-v2.txt` — **44 scenarios** (tables 12, files 7, descriptive 6, distributions 5, tests 9, regression 3,
errors 2), written and **frozen before the code** (commit `adce85c1`): sha256
`012d330d0a428cf4242bc90a45f4978f222549fdd64c943030adf1a1bd1508c1` (test `suite_v2_is_frozen`). Expected values come from
exact formulas and identities (t² = F, correlation p = slope p, χ² 2×2 = n(ad − bc)²/(r1·r2·c1·c2), F(1, ν) = t(ν)²,
binomial and Poisson fractions) and from textbook and R documentation examples: sleep (Student 1908),
PlantGrowth, Agresti (2007), Anscombe's quartet.

| run | correct | red |
|---|---|---|
| first, before any fixes | 43/44 | dist-01: `normcdf(1.96)` at the 12th digit — the `erfc` error in statrs |
| after our own `erfc` | **44/44** | — |
| v1 regression | 205/206 | the same la-18 (errata) |

**Negative controls** (`--fault NAME`: +1 to the first result): `mean`, `median`, `tinv`, `ttest`, `datenum`,
`histcounts` → 43/44; `quantile`, `skewness`, `normcdf`, `chi2cdf`, `binopdf` → 42/44 (test
`negative_control_v2_broken_builtins_turn_red`). A CSV file with an error gives a clear error with line and column
(scenarios io-04, io-05 and test `csv_and_json_errors_name_line_and_column`). CSV round trip with tricky text
(comma, quotes, leading space, `"007"`, empty string versus missing, a leap-year date) — test
`csv_round_trip_keeps_types_and_tricky_text`. `cargo test` — 21 tests, green.

### Benchmark: 1M rows in memory

`examples/bench_table.m`, release, single thread, `nice`:

| operation (1,000,000 rows) | time |
|---|---|
| `table` from three columns | 0.011 s |
| **`groupsummary` sum by key (1000 groups) + `sortrows` of the result** | **0.120 s** (target < 1 s) |
| `sortrows` of the whole table by a numeric column | 0.140 s |
| `groupsummary` sum + mean by two keys (12,000 groups) | 0.250 s |
| two computed columns (`T.y = T.x .* 2 + T.g`, `T.big = T.x > 0.5`) | 0.010 s |
| `grouptransform` `@cumsum` by key (1000 handle calls) | 0.128 s |
| mask filter `T(T.big & T.r == 3, :)` | 0.022 s |

Grouping is a (stable) sort of indices by keys and a pass over runs; joins use a hash index of the right table.
Assigning a column does not copy the table (`Rc::make_mut`).

### v2 differences from MATLAB

- Text columns are our `string` class; `{…}` is only a list of strings; `T{…}` and `T.Properties` — no.
- Dates are datenum numbers in a column of type "date" (printed as ISO); there is no `datetime`; dates come from files,
  `T.d = datenum(…)` gives a numeric column.
- `histcounts(x, n)` — n equal bins from min to max (MATLAB picks "round" edges); without n — Sturges' rule.
- The test report is printed by default (in MATLAB tests are silent); `'Display','off'` suppresses it.
- `anova1` without a plot; `tbl` is a table `Source SS df MS F p`. `ttest2` with `'Vartype','unequal'`: `stats` has
  `sdx`, `sdy` instead of an `sd` vector.
- `groupsummary` skips `NaN` in methods; `outerjoin`, `'omitnan'` in v1 `mean` — not yet.

## VM v1 (26.09): register bytecode

Design decision (26.09): pick whichever classic design is faster → a register VM, as in Lua 5 (`docs/context.md`, "Execution sandbox").
`mlab run file.m --vm` compiles the program to bytecode and executes it; whatever the compiler does not handle is executed by the tree walker,
transparently for the user. Code — `src/vm.rs`, gate — `tests/vm.rs`.

### How it works

- **Registers per function frame.** A unit's variables (script or function) are fixed registers `0..n`, temporaries are a stack above
  them. A name that is assigned somewhere is a register; an undefined register at runtime means a function with the same
  name, as in the tree (`x = sin(1); sin = 5; sin(1)`).
- **Register value** `Reg`: `F(f64)` — a real double scalar, `L(bool)` — a logical scalar, both without
  allocation; `V(Rc<Value>)` — everything else, with copy-on-write (`B = A` does not copy the matrix,
  `B(1) = 0` copies only if someone else still holds it). Matrices and operations are the existing mlab types and functions.
- **Bytecode:** 45 instructions, the dispatcher is a `match` in a loop; numbers are inline in the instruction (`LoadF`), other constants are
  in a pool (`LoadK`). Calling a user function goes from VM to VM: arguments are passed in registers, frames come from a pool.
- **Superinstructions** for hot pairs:
  - `AddK`, `BinK`, `KBin` — an operation with a constant (`s + 1`, `x^2`, `2*x`);
  - `JmpCmp`, `JmpCmpK` — "compare and jump" (`if n < 2`, `while k <= n`);
  - `ForNext` — "loop step, bound check, new value of the variable"; a loop over `a:s:b` runs without materializing the
    range (`ops::RangeSpec::at` — the same formula as in `range_values`);
  - `Index` / `IndexAsg` with simple arguments (variables, numbers, `:`) — reading and writing an element `A(i,j)` directly from
    registers, without copies or checks in the loop body; checks, errors and atypical cases go through the slow path
    via the same `index::read` / `index::assign`.
- **Fallback to the tree, two levels.**
  - A statement the compiler does not handle (`switch`, `try`, commands like `format long`, fields `T.x = …`,
    cell lists `{…}`, calls to `exist`/`clear`/`feval`/`arrayfun`/`cellfun`, which see the frame by name) becomes `Fallback`:
    the variables it mentions move registers → `HashMap` frame → registers **by move**, without copies;
    `break`/`continue`/`return` from inside become VM jumps.
  - An expression in which a variable's type is known only at runtime (indexing a table or a string array) becomes `DynEval`.
- **The same semantics:** operators, indexing, builtins (including `--fault`), output and error texts are the same
  functions `ops`, `index`, the registry. The VM has its own code only for scalar fast paths, and those are checked by the gate.

### Equivalence gate

`mlab vmgate` and the test `vm_gate_frozen_suites_byte_identical`: every scenario runs through the tree walker and the VM; all output, including
warnings and errors, must match **byte for byte**; the first discrepancy is red and stops.

| suite | scenarios | same output | entirely in VM | statements in VM | fallback (statements / hits) | `DynEval` |
|---|---|---|---|---|---|---|
| suite-v1 | 206 | **206** | 199 | 574 | 9 / 11 | 0 |
| suite-v2 | 44 | **44** | 22 | 195 | 47 / 47 | 5 |

- v2 falls back more often: table column assignments `T.c = …` and lists `{'a','b'}`.
- Under broken builtins (`--fault sum`, `max`, `mean`, `gate:mldivide`) the tree and the VM also give the same output.
- `mlab test … --vm`: 205/206 (the same errata la-18) and 44/44.
- **Negative control** (`--fault vm:add`: scalar addition in the VM gives +1) — the gate turns red on the very first
  scenario with an addition (syn-08: tree "24", VM "26"); 7 red in v1 in total (test
  `vm_gate_negative_control_turns_red`).
- **Edge cases** — 83 programs (`vm_edge_cases_match_tree`): a name that is sometimes a variable, sometimes a function; `end` in nested
  calls; `break` from `try`/`switch` in fallback; the recursion limit 256; `nargin`/`nargout`; multiple outputs;
  capture in anonymous functions; growth and deletion; logical masks; complex and char; loops over a matrix,
  an empty value, a scalar, a string; tables in a loop; copy-on-write; error texts.

### Speed (release, single thread, `nice`, best of 3; including parsing and compilation)

`mlab vmbench examples/vmbench` (the output of both modes is compared):

| task | tree, s | VM, s | speedup |
|---|---|---|---|
| fib(27) recursively (≈ 630K calls) | 0.417 | 0.053 | **7.9×** |
| loop of 10⁷ scalar additions | 1.877 | 0.129 | **14.5×** |
| nested loops over a 1000×1000 matrix, `B(i,j) = x*x + 2*x + 1` | 0.837 | 0.067 | **12.6×** |
| mixed: `\`, `*`, `norm`, `sum` in a loop + 2·10⁵ scalar `mod`/`sqrt`/`abs` | 0.163 | 0.080 | 2.0× |

- **The target of ≥ 3× on scalar loops is reached** (7.9–14.5×). The 10⁷ loop in the tree also materializes the range
  (80 MB), the VM does not.
- **Mixed is only 2×, and that is honest.**
  - The matrix part alone — 0.027 versus 0.026 s (1.0×): the time is in the builtins (`\` with rcond estimate and gate, `*` on
    faer), and those are the same.
  - Scalar builtins alone — 2.2×: the builtin ABI is `&[Value]`, so every scalar is wrapped into a `Mat`
    (an allocation) and back.
  - Next: fast paths for element-wise builtins on scalars (the same `f64 → f64` function as in `elem.rs`) and
    an ABI on `Reg`.
- **Stack:** recursion of depth 250 in release — 1 MB for the VM versus 2 MB for the tree. In a debug build even 8 MB is
  not enough for the tree, so the edge-case test runs in a separate thread with a larger stack.

### WASM

`rustup target add wasm32-wasip1` → `cargo build --release --target wasm32-wasip1` builds without changes:
`mlab.wasm` is 2,367,782 bytes (2.3 MB, without `wasm-opt`); with vfs (26.09) — 2,543,685 bytes, and it runs in our
WASI host (section "File system: vfs"). Under WASI in Node 24 (V8): fib(27) — 0.630 s tree versus
0.069 s VM (9.1×), the 10⁷ loop — 2.747 versus 0.219 s (12.5×); 1.3–1.7 times slower than the native build.

### v1 limitations

- These fall back: `switch`, `try`, commands, fields and tables on the left-hand side, cell lists, an index not on a name
  (`f(x)(2)`), multiple outputs with a variable on the right-hand side. Correct, but in a hot loop it runs at tree speed
  plus variable moves.
- Anonymous function bodies are executed by the tree (`@(x) x.^2` in `arrayfun`, `integral`).
- Next: `switch` and `try` in bytecode; fast scalar builtins; WASM modules via `wasmi` / `wasmtime` —
  as a separate step.

## Independent tests (tests3, 26.09)

Requested: many more tests devised through Opus, plus something found on the internet. The v1/v2 suites were written by the
same author as the code, so they measure conformance to the specification. Here there are two independent sources. Worklog —
`$MOVA_DATA/worklog/2026-09-26/tests3.md`, runs — `$MOVA_DATA/runs/tests3-2026-09-26/`.

### LLM scenarios (`tests/data/vmm/`, `tests/vmm_suites.rs`)

- **The author is Opus** (`claude-opus-5-5`, effort high, `math/src/bin/vmmtests.rs` — reuses `math::vmm::Vmm`, cap and log).
  It did not see the mlab code: the prompt is the language documentation (what exists, what does not, deliberate differences) and rules for unambiguous
  output (`printf`, `mat2str`, tolerance checks for iterative methods). Prompts — `tests/data/vmm/prompts/`.
- **400 scenarios**, 10 areas of 40: indexing, operators, strings, control flow and functions, reductions and element-wise,
  linear algebra, polynomials and numerical methods, tables, statistics, edge cases. Each header has
  `src:` (identity, exact formula, documentation, manual calculation).
- **Frozen by hashes before the first run** (commit `1f4581225`; the hashes are in the test and the worklog).
- **First run on the baseline (before fixes): 382/400**, the VM — the same 382. After fixes — **392/400**; the
  tree ↔ VM equivalence gate — **400/400 byte for byte**.

| class | count | what |
|---|---|---|
| **defect, fixed** | 10 | `det(zeros(3))` = NaN (faer divides 0/0) → 0; `complex(1, 0)` was narrowed to real; `linspace(a, b, 0)` gave 1×1; `printf('%g', -0)` without the minus; `upper`/`lower` flattened a char matrix into a row; `polyder(a, b)` ignored the second argument; `mode(A, 2)` ignored the dimension; `cov(x, 1)` took 1 as Y; `dot` of complex numbers — an error; `nchoosek(v, k)` — "not supported" |
| **defect, in "What next"** | 4 | `A.^2'` (must be `(A.^2)'`), the loop variable after `for k = 1:0` (must be 1×0), `T.x(2) = 10` is not parsed, command syntax `disp hello` |
| test error | 2 | `trace(A*B)` = 28, not 29 (checked by hand with both products); `anova1(…, 'Display', 'off')` — the mistake was suggested by the prompt (in MATLAB the third argument is `'off'`) |
| difference (documented) | 2 | `strrep` with overlap — we behave like Octave (`'ba'`), MATLAB gives `'bb'`; `fitlm` returns a table, not a `LinearModel` object |

### NIST StRD (`tests/data/ext/nist/`, `tests/ext_nist.rs`)

NIST Statistical Reference Datasets (public domain, `.dat` files as is): 11 regressions, 11 ANOVA, 9 descriptive.
The yardstick is LRE, the number of correct significant digits (McCullough 1998), capped at 15. Minimum per dataset:

| dataset | difficulty | fitlm | `\` before → after | anova1 before → after |
|---|---|---|---|---|
| Norris, NoInt1/2 | lower | 13.3; 14.7; 15.0 | 13.0; 14.7; 15.0 | |
| Pontius | lower | 12.2 | **4.9 → 13.5** | |
| Longley | higher | 12.8 | **9.1 → 11.4** | |
| Wampler1–5 | lower–higher | 9.6; 12.9; 9.0; 7.7; 5.7 | 9.4 → 9.6; **10.3 → 12.5**; 9.5 → 9.6; 8.1; 6.1 | |
| Filip (degree-10 polynomial) | higher | **7.3** | **0 silently → warning "rank deficient"** (as in MATLAB) | |
| SiRstv, SmLs01–03 | lower | | | 12.4–14.4 → 13.1–15.0 |
| AtmWtAg, SmLs04–06 | average | | | 6.5–8.5 → 9.9–10.2 |
| SmLs07–09 (10¹² + 0.x) | higher | | | **2.7 / 2.0 / 0.0 → 4.0 / 3.9 / 3.9** |
| PiDigits … NumAcc2 | lower | | | mean/std/r(1): 13.0–15.0 |
| NumAcc3, NumAcc4 | average, higher | | | 9.5; 8.3 |

- **`\` for rectangular matrices** used truncated SVD: SVD is not invariant to column scaling (raw powers of x), so
  Pontius had 4.9 digits, and Filip 0 without any warning. Now it is **Householder QR with column pivoting**
  (`lstsq_qr`), rank by |R_kk| > max(m, n)·eps·|R₁₁|; rank deficiency — minimum-norm SVD with a warning.
- **anova1** summed 10¹² values directly. Now — a shift by the first observation (SS do not depend on the shift) and
  a correction pass for the means. SmLs07–09 — ~4 digits, that is the ceiling: the very representation of 1000000000000.4 in double has an error of
  ~6·10⁻⁵ with deviations of ~0.1.
- Wampler5 5.7 and NumAcc4 8.3 are within double precision for these data; the test thresholds are the lower bound of what was measured.
- Negative control: a broken `mean` → LRE < 2; a separate test — Filip via `\` must warn.

### Boost.Math (`tests/data/ext/boost/`, `tests/ext_boost.rs`)

Boost 1.92.0 test data (BSL-1.0, `LICENSE_1_0.txt` alongside): values with 35–40 digits. Largest relative error:

| function, Boost sample | points | before, eps | after, eps |
|---|---|---|---|
| erf, erf_small_data | 149 | **4.5·10¹⁵** (erf(1.4·10⁻⁴⁵) = 0) | 1.0 |
| erf, erf_data | 500 | 15.3 | 0.7 |
| erfc, erf_data | 500 | **410.7** (near x = 2) | 1.3 |
| erf / erfc, erf_large_data | 300 / 68 | 0.0 / 3.3 | 0.0 / 1.1 |
| gamma, factorials | 198 | **296** (x ≈ 100) | 2.1 |
| gamma, near_0 / near_1 / near_2 | 40 / 41 / 41 | 3.4 / 2.8 / 6.0 | 2.0 / 2.2 / 1.6 |
| gamma, near_m10 / near_m55 | 40 / 40 | **6.3·10⁵ / 6.4·10⁶** | 2.2 / 2.0 |

Causes: `erf = 1 − erfc` (cancellation near zero), `erfc = 1 − series` up to x = 2, terms cancelling each other in the Lanczos sum with g = 7,
the `sin(πx)` reflection without argument reduction. Now — `libm` 0.2.16 (MIT, a port of musl/fdlibm; it was already
in the tree via statrs): `erf`, `erfc`, `tgamma`. mlab has no beta function yet — in "What next".

### Octave tests — an external measurement (`tests/octave_ext.rs`, `#[ignore]`)

`%!assert`, `%!test`, `%!error` blocks from the GNU Octave 11.3.0 source files (GPL: only in
`$MOVA_DATA/raw/tests-octave/`, they do not enter the repo; the tool takes only `%!` lines, the implementation code is not
read). Each block is a separate `mlab run` process, limit 10 s. The measurement includes blocks all of whose calls are mlab builtins.

| | blocks | measured | pass | assert | other error | infinite loop | **crash** |
|---|---|---|---|---|---|---|---|
| before fixes | 15 235 | 2 798 | 1 409 | 247 | 1 132 | 0 | **10** |
| after | 15 235 | 2 798 | 1 419 | 242 | 1 137 | 0 | **0** |

Crashes (a panic instead of an error) — `trapz(h, Y)` with a scalar step, `polyfit(x, y, Inf)`, `primes(Inf)`,
`corr` of constant data (a panic inside statrs), `corrcoef([])` — fixed. "Other error" is mostly N-dimensional
arrays, cells, structs; the 242 red `assert`s have not been analysed yet — "What next".

## File system: vfs (26.09)

Requested (26.09): a small model could already write a virtual file system and embed it by porting the code to
WASM/WASI. The `vfs` crate (`../vfs/README.md`) is an in-memory file system with snapshots. mlab accesses files only
through it.

- **All file builtins go through the `vfs::FileSystem` trait** (`Interp::fs`):
  - existing: `readtable writetable readmatrix writematrix fileread writelines tempname`;
  - new: `isfile isfolder mkdir rmdir delete movefile copyfile readdir`;
  - `exist(name)` now also sees files (2) and folders (7), as well as `exist(name, 'var'|'builtin'|'file'|'dir')`.
- **Modes** (`--fs` for `run`, `-e`, `test`):

  | mode | what the program sees | when |
  |---|---|---|
  | `--fs mem` (default) | only the in-memory file system: `/` and `/tmp`; the real FS is not visible | foreign or generated code, tests, suites |
  | `--fs dir:<folder>` | a real folder as the root (chroot): `..` above the root and links pointing outside are errors | operator data |
  | `--fs real` | the real FS without restrictions, as before vfs | only explicitly, for backward compatibility |

  `Interp::new` also uses `mem`: the library is sandboxed by default.
- **Memory contents from folders and to folders, only explicitly** (only with `--fs mem`):
  - `--fs-import DIR[:VIRT]` — before the run, by default into `/`;
  - `--fs-export DIR[:VIRT]` — after the run, **only if the script finished without an error**: only approved results
    leave the sandbox. Otherwise — a "skipped" message.
- `--fs-limits total=256M,file=64M,files=65536` — memory limits. A violation is an error with an explanation:
  `writematrix: cannot write '/big.csv': vfs limit: file '/big.csv' would be 12345 bytes, max_file_size = 2000`.
- `--fs-journal FILE.tsv` — an operation journal (TSV): what was read and written, errors, attempts to escape the root.
- **`tempname` in `mem` and `dir:`** — `/tmp/mlab_<k>`, deterministic. In `real` — as before: `temp_dir()/mlab_<pid>_<k>_<ns>`.
- **Error texts are as before:** "No such file or directory (os error 2)". Escaping the root — "path escapes the
  sandbox root: '../x'".

### vfs gates (`tests/fs.rs`, `tests/suite.rs`)

- **The frozen suites 206 + 44 under `--fs mem` give byte-for-byte the same output as before vfs.** A run of main (before vfs,
  `std::fs`) and of the branch (memory) — tree and VM, 500 runs: sha256 of both outputs
  `846d93aab89d0419e4de184ee2f9fde5b98f17e011c001be6b1bcafb2f283a3a`. The test
  `frozen_suites_same_output_in_mem_real_and_dir` requires identical output of every scenario in `mem`, `real` and `dir:`
  (a fresh folder in `tmp/`), tree and VM.
- **The file suite `data/suite-fs.txt`** — 25 scenarios:
  - what it checks: files, folders, paths with `./` and `../`, `readdir` in byte order, OS errors, 50 files in a folder,
    a non-Latin name, a tricky CSV;
  - expected output checked by hand against MATLAB/Octave semantics;
  - frozen: sha256 `782a847e4d64c4fe85b0de2d92e740b78c972d72aae0043b846e2d23c30f0e94` (test
    `suite_fs_is_frozen`);
  - `mem` 25/25; `dir:` — the same output.
- **Escaping the root — red tests:**
  - `../../../../etc/passwd`, `/../etc/passwd`, `/tmp/../../x.csv`, writing and `mkdir` above the root — an error in `mem` and
    `dir:`;
  - in `dir:` — also real links pointing outside: to a file, to a folder, dangling;
  - a file outside the root does not appear.
- **Limits:** file size and number of files — errors with an explanation.
- **CLI:**
  - `--fs-import` → `groupsummary` → `--fs-export`: only `/out` is exported, the journal contains `import`,
    `write` and `export`;
  - a script error — export skipped;
  - also: an unknown mode, import without `mem`, a limit from the CLI, `dir:`.
- **Negative controls** are in `vfs`: `ClampDotDot`, `NoLimits`, `RestoreForgetsClock`, `NoConfine` turn their
  gates red.

### mlab.wasm in our WASI host

`wasi-host` (`../vfs/README.md`, section "WASI host") runs `mlab.wasm` on `wasmi`. WASI calls go to the same
`vfs::Vfs`. The guest is started with `--fs real`: the guest's `std::fs` → WASI → the host's `Vfs`.

| suite | runs | same output as native mlab with `--fs mem` |
|---|---|---|
| `suite-fs` (tree and VM) | 50 | **50** |
| `suite-v1` + `suite-v2` | 250 | **249**; la-18 (errata, a singular matrix) — the faer build for wasm32 gives "svd: failed to converge", V8 too |

The file suite in the guest — 0.03 s, the 206 v1 scenarios — 0.12 s (native — 0.01 s).

## Known differences from Octave/MATLAB

- `rand`/`randn` — our own stream (xoshiro256**), not the Mersenne Twister: deterministic, but different numbers. Seeding:
  `rand('seed'|'state'|'twister', s)`, `rng(s)`, `rng('default')`.
- `eye`/`diag` — full matrices (Octave shows "Diagonal Matrix").
- `ode45` returns only accepted steps (Refine = 1; in MATLAB/Octave — 4); `odeset` options — after structs.
- `A \ b` for a singular square matrix — a warning and the minimum-norm solution (verified by the gate).
- e-format thresholds and error texts are our specification in the style of Octave (`error: operator +: nonconformant
  arguments (op1 is 1x2, op2 is 1x3)`, `error: 'x' undefined`, `error: v(5): out of bound 3 (dimensions are 1x3)`).
- `unique` returns indices of first occurrences; `mat2str` of strings uses double quotes (as Octave);
  `num2str` of non-integer matrices has simplified alignment; `asin`/`acos` outside [−1, 1] give NaN (not complex).

## Licenses

- Code — the Elamur Cosmic License (`LICENSE.md` at the repo root).
- The scenario suite and the format specification are our own.
- Test data (tests3): NIST StRD — public domain (US government works, 17 U.S.C. § 105; `tests/data/ext/nist/`); Boost.Math — Boost Software License 1.0 (`tests/data/ext/boost/LICENSE_1_0.txt`, `.ipp` files with the author line unchanged); LLM scenarios — our own (generated by Opus for us). Octave tests (GPL) and Matrix Market — only in `$MOVA_DATA/raw/`, not in the repo.
- `vfs` (26.09) is our own crate without dependencies; no new third-party crates were added to mlab.

Dependencies (the `cargo tree` tree; each crate's license was checked against the crates.io API `/api/v1/crates/<name>/<version>`
on 26.09; the check — `$MOVA_DATA/runs/mlab-2026-09-26/deps-cratesio.tsv`):

| crate | version | license (crates.io) | role |
|---|---|---|---|
| `autocfg` | 1.5.1 | Apache-2.0 OR MIT | build only (proc-macro / build.rs) |
| `bitflags` | 2.13.2 | MIT OR Apache-2.0 | in the binary |
| `bytemuck` | 1.25.2 | Zlib OR Apache-2.0 OR MIT | in the binary |
| `cfg-if` | 1.0.5 | MIT OR Apache-2.0 | in the binary |
| `defer` | 0.2.1 | MIT OR Apache-2.0 | build only (proc-macro / build.rs) |
| `dyn-stack-macros` | 0.1.3 | MIT | build only (proc-macro / build.rs) |
| `dyn-stack` | 0.13.2 | MIT | in the binary |
| `equator-macro` | 0.2.1 | MIT | build only (proc-macro / build.rs) |
| `equator-macro` | 0.6.0 | MIT | build only (proc-macro / build.rs) |
| `equator` | 0.2.2 | MIT | in the binary |
| `equator` | 0.6.0 | MIT | in the binary |
| `faer-traits` | 0.24.0 | MIT | in the binary |
| `faer` | 0.24.4 | MIT | direct; in the binary |
| `gemm-c32` | 0.19.0 | MIT | in the binary |
| `gemm-c64` | 0.19.0 | MIT | in the binary |
| `gemm-common` | 0.19.0 | MIT | in the binary |
| `gemm-f32` | 0.19.0 | MIT | in the binary |
| `gemm-f64` | 0.19.0 | MIT | in the binary |
| `gemm` | 0.19.0 | MIT | in the binary |
| `generativity` | 1.2.1 | MIT OR Apache-2.0 | in the binary |
| `interpol` | 0.2.1 | MIT | build only (proc-macro / build.rs) |
| `libc` | 0.2.189 | MIT OR Apache-2.0 | in the binary |
| `libm` | 0.2.16 | MIT | tests3: direct (erf, erfc, tgamma); in the binary |
| `nano-gemm-c32` | 0.2.1 | MIT | in the binary |
| `nano-gemm-c64` | 0.2.1 | MIT | in the binary |
| `nano-gemm-codegen` | 0.2.1 | MIT | build only (proc-macro / build.rs) |
| `nano-gemm-core` | 0.2.1 | MIT | in the binary |
| `nano-gemm-f32` | 0.2.1 | MIT | in the binary |
| `nano-gemm-f64` | 0.2.1 | MIT | in the binary |
| `nano-gemm` | 0.2.2 | MIT | in the binary |
| `num-complex` | 0.4.6 | MIT OR Apache-2.0 | in the binary |
| `num_cpus` | 1.17.0 | MIT OR Apache-2.0 | in the binary |
| `num-integer` | 0.1.47 | MIT OR Apache-2.0 | in the binary |
| `num-traits` | 0.2.19 | MIT OR Apache-2.0 | in the binary |
| `once_cell` | 1.21.4 | MIT OR Apache-2.0 | in the binary |
| `paste` | 1.0.15 | MIT OR Apache-2.0 | build only (proc-macro / build.rs) |
| `primal-check` | 0.3.4 | MIT OR Apache-2.0 | in the binary |
| `private-gemm-x86` | 0.1.20 | MIT | in the binary |
| `proc-macro2` | 1.0.107 | MIT OR Apache-2.0 | build only (proc-macro / build.rs) |
| `pulp` | 0.22.3 | MIT | in the binary |
| `pulp-wasm-simd-flag` | 0.1.1 | MIT | in the binary |
| `qd` | 0.8.0 | MIT | in the binary |
| `quote` | 1.0.47 | MIT OR Apache-2.0 | build only (proc-macro / build.rs) |
| `raw-cpuid` | 11.6.0 | MIT | in the binary |
| `reborrow` | 0.5.5 | MIT | in the binary |
| `rustfft` | 6.4.1 | MIT OR Apache-2.0 | direct; in the binary |
| `seq-macro` | 0.3.6 | MIT OR Apache-2.0 | build only (proc-macro / build.rs) |
| `strength_reduce` | 0.2.4 | MIT OR Apache-2.0 | in the binary |
| `syn` | 1.0.109 | MIT OR Apache-2.0 | build only (proc-macro / build.rs) |
| `syn` | 2.0.119 | MIT OR Apache-2.0 | build only (proc-macro / build.rs) |
| `transpose` | 0.2.3 | MIT OR Apache-2.0 | in the binary |
| `unicode-ident` | 1.0.26 | (MIT OR Apache-2.0) AND Unicode-3.0 | build only (proc-macro / build.rs) |
| `version_check` | 0.9.5 | MIT OR Apache-2.0 | build only (proc-macro / build.rs) |
| `statrs` | 0.19.1 | MIT | v2, direct; in the binary (without default features: no nalgebra or rand) |
| `approx` | 0.5.1 | Apache-2.0 | v2; in the binary |
| `thiserror` | 2.0.21 | MIT OR Apache-2.0 | v2; in the binary |
| `thiserror-impl` | 2.0.21 | MIT OR Apache-2.0 | v2; build only (proc-macro) |

In total v1 has 53 crates (38 in the binary, 15 build-only), all matching crates.io; v2 added 4 (checked against the
crates.io API on 26.09: statrs — MIT, approx — Apache-2.0, thiserror and thiserror-impl — MIT OR Apache-2.0). CSV and
JSON are our own code, without crates. Summary: MIT — 29, MIT OR Apache-2.0 — 22, `bytemuck` — Zlib OR Apache-2.0 OR MIT (we take MIT), `unicode-ident` — (MIT OR Apache-2.0) AND Unicode-3.0. Unicode-3.0 is the permissive Unicode data license (OSI), and this crate is used only at build time (the proc-macro2 → syn chain), it does not end up in the binary. GPL/LGPL — none. The benchmark (`bench/`) additionally pulls in `nalgebra` (Apache-2.0) and its dependencies — only for measurement, not in mlab.

## What next

1. **Exact mode** — after merging `math` v2: the `Value::Exact` variant is already in the value type. Rational matrices and
   big integers from `math`; `det`, `inv`, `\`, `rank`, `rref` over the rationals without rounding; a
   `format rat`/`exact` switch; steps and proofs as in `math`.
2. **Cell arrays and structs** — unlock `odeset`, `err.message`, `[p, S] = polyfit`, `cellfun`,
   `strsplit`, `UniformOutput=false`, `varargin`/`varargout`.
3. **Plots** — `plot`/`bar`/`hist` to SVG (a permissive crate, e.g. `plotters` — MIT) for the website; data —
   KaTeX/LaTeX alongside.
4. **Built-in LaTeX** (`docs/context.md`): printing results and steps in LaTeX, parsing formulas into an expression tree.
5. **More on v2** (v2 was done on 26.09 — section above):
   - XLSX/ODS: reading with `calamine` (MIT), writing with `rust_xlsxwriter` (MIT/Apache) — check the licenses of their trees (zip,
     quick-xml…) against crates.io before adding; DuckDB — through our CLI;
   - **a "table mathematics" pilot** (requested 26.09): 50–60 of our own problems over tables with exact answers
     (revenue by region, monthly average, share within a group, growth in %, moving and weighted averages, trend
     forecast), frozen by hash; the LLM (`claude -p`, API, `api_env()` as in `math/src`, effort high via
     `--settings` env, cap $15) writes a solution in the mlab language → the SLM executes it in memory → independent check →
     solution memory (JSONL in runs and `$MOVA_DATA/db/mathsolve.duckdb`, as in math v2) → how many plans the perceptron
     already guesses;
   - `datetime`, cell arrays (real `{}`), `T{…}`, `outerjoin`, `'omitnan'`, Fisher's exact test and
     non-parametric tests (Mann–Whitney, Wilcoxon — port from HypothesisTests.jl, MIT), p-value corrections (MultipleTesting.jl).
6. **An independent compatibility check** — scenarios from another author (a human or a model) based on the documentation, blind to the
   code; today's suite measures only conformance to our specification.
7. ~~Speed — bytecode instead of tree walking~~ — VM v1 (26.09), section above; next — `switch`/`try` in
   bytecode, scalar builtins without wrapping. N-dimensional arrays; integer types.
8. **After the independent tests (tests3, 26.09):**
   - 4 defects found by the LLM: `A.^2'` (transpose and power — one precedence level, left to right), the loop variable after
     an empty `for` (1×0; tree and VM together), `T.x(k) = v`, command syntax for any function;
   - analyse the 242 red `assert`s of the Octave tests (`$MOVA_DATA/runs/tests3-2026-09-26/octave/blocks.tsv`):
     defect, MATLAB/Octave difference or out of scope;
   - `beta`, `gammaln`, `betainc` and a check against Boost's `beta_*_data.ipp`; Matrix Market (5 matrices already in raw:
     bcsstk01/02, nos4 — SPD: `chol`, λ > 0, trace = Σλ; west0067, gre__115 — non-symmetric: `\`, `det`, `eig`); a sample of numpy/SciPy tests (BSD)
     from PyPI archives — not done in time;
   - a second LLM wave for ~$5: table problems, formatted `disp` output of matrices, `format long`.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
