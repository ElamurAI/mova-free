# mlab — a computing language in the spirit of MATLAB/Octave

mlab follows the logic of Octave as a small alternative to MATLAB. It is an interpreter for a subset
of the language in Rust: matrices, 1-based indexing, colon, `end`, element-wise operations, functions, control flow, linear
algebra, polynomials, FFT, numerical methods. Every linear algebra result passes a **verification gate**:
a wrong answer produces a warning, not silence.

This is the natural plan language for the LLM (the teacher writes such code fluently), while the SLM executes it ([docs/context.md](../docs/context.md#mini-lab-a-matlaboctave-style-language-and-built-in-latex)).
Our difference from Octave is the gates. There is no exact mode and no step output yet (see "Limits").

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
| `value.rs` | `Value`: `Mat` (f64 column-major + class double/logical/char + optional imaginary part), `Func` (named/anonymous with capture), **`Table`** (named columns of different types), **`Exact`** (reserved for an exact mode; an empty enum — the slot already exists in every `match`) |
| `interp.rs` | workspace and call frames, execution, `end` context, calls (variable → script function → builtin), output |
| `ops.rs` | operators with implicit expansion (broadcasting), complex numbers, matrix multiplication (faer for large ones), division, power, ranges, concatenation |
| `index.rs` | reading `A(I)`, `A(I,J)`; assignment with growth; deletion `A(I) = []` |
| `display.rs` | Octave-style `x = …` and `disp` output; printing of anonymous functions |
| `builtins/` | **an extensible registry** (`Registry::register`): `core`, `elem`, `reduce`, `io`, `linalg`, `poly`, `fft`, `numeric`, `table`, `stats` |
| `table.rs`, `fileio.rs`, `mcgate.rs` | tables (columns, output, keys, dates), CSV/TSV/JSON, a Monte Carlo gate for p-values |
| `rng.rs` | xoshiro256** + splitmix64, normals by Marsaglia's polar method; deterministic with a seed |
| `suite.rs`, `main.rs` | scenario suite; CLI |
| `vm.rs` | **VM**: a compiler from the tree to register bytecode and an executor; fallback to the tree is transparent (section "Register VM") |
| `builtins/files.rs` | **vfs**: files and folders via the `vfs::FileSystem` trait; all file builtins go through `Interp::fs` (section "File system: vfs") |

## The language

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
- Not supported: cell arrays, structs, N-dimensional arrays, integer types (`int8`…), `single` (becomes double), `string` objects.
  These give a clear runtime error, not a crash.

### Builtins

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
- **tables and files:** `table height width summary sortrows groupsummary grouptransform pivot innerjoin join ismissing rmmissing movmean movsum movmax movmin movmedian datenum datestr readtable writetable readmatrix writematrix jsonencode fileread writelines tempname string`;
- **vfs — files and folders:** `isfile isfolder mkdir rmdir delete movefile copyfile readdir`, `exist(…, 'file'|'dir')`;
- **statistics:** `mode quantile prctile skewness kurtosis cov corrcoef corr histcounts normpdf normcdf norminv tpdf tcdf tinv chi2pdf chi2cdf chi2inv fpdf fcdf finv binopdf binocdf binoinv poisspdf poisscdf poissinv erf erfc ttest ttest2 chi2test crosstab anova1 fitlm`.

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
../target/release/mlab run file.m --vm       # the same on the register VM (--vm also works with -e and test)
../target/release/mlab vmgate data/suite-v1.txt data/suite-v2.txt [--fault vm:add]   # VM equivalence gate
../target/release/mlab vmbench examples/vmbench   # tree versus VM, 3 runs, best
../target/release/mlab vmdump file.m         # bytecode listing
../target/release/mlab run file.m --fs mem   # (default) files only in memory; see "File system: vfs"
../target/release/mlab run file.m --fs-import data:/in --fs-export out:/out [--fs-limits file=64M] [--fs-journal j.tsv]
../target/release/mlab run file.m --fs dir:./data   # a real folder, the root is the boundary
../target/release/mlab run file.m --fs real  # the real FS without restrictions — only explicitly
```

`examples/demo.m` is a short demo: indexing, `\` with the gate, `eig`, `roots`, `integral` to ∞, user
functions, `ode45`. `--fault NAME` for `run`/`-e`/`test` is a negative control (a broken builtin or a wrong
answer before the gate).

## Scenario suites

- **`data/suite-v1.txt`** — the core language: syntax, output, matrices, indexing, operators, control, functions,
  reductions, element-wise, output functions, linear algebra, gates, polynomials, spectrum, numerical methods, errors.
  Written from the language documentation (clean room); expected values come from identities and exact answers
  (A·inv(A) = I, A·V = V·D, U·S·Vᵀ = A, Fibonacci via A^10, exact roots, Parseval); where a number depends on rounding,
  the scenario prints a "< tolerance" check. Frozen by hash before the interpreter code: sha256
  `2c37701df94db6328546388cc21145c4c8fde48ea4f18aeb00e2e2333ca0b0e6` (test `suite_is_frozen`).
- **Errata la-18.** The scenario takes A = [2 0 1; 1 3 2; 1 1 1], which is singular (det = 0); the interpreter warns
  "matrix singular to machine precision", so the scenario fails by design. The suite is frozen and not edited; the
  corrected scenario (det = 6) is the test `errata_la18_corrected_passes`.
- **`data/suite-v2.txt`** — tables, files and statistics (section "Suite `suite-v2`").
- The suites and the interpreter share an author, so they check conformance to our specification; the independent
  checks are in "Independent tests".
- **Negative control:** `--fault NAME` adds 1 to the first element of a builtin's result and must turn the suite red
  (test `negative_control_broken_builtin_turns_suite_red`). `magic(n)` for n = 3…12 is checked for its sums and
  permutation (test `magic_squares_are_magic`).

**Linear algebra backend: faer.** It gives eigenvectors of non-symmetric matrices (`[V, D] = eig(A)`), which nalgebra
does not; [`bench/`](bench/README.md) is the measurement crate that compares the two.

## Tables, files, statistics

Statistics and Excel-like work with tables, export and import; table computations in RAM.

### Tables — columnar, in memory

- **`table`**: named column vectors of different types — numbers (`NaN` is missing), logical, text (class `string`,
  `<missing>`), dates (datenum, as in MATLAB: 719529 = 1970-01-01; printed as ISO). `table(x, y)` takes the names from the
  variable names, `'VariableNames', {'a','b'}`. `{'a','b'}` is a string array (there are no real cell arrays).
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
- **XLSX/ODS — not supported** (a clear error).

### Statistics

- **Descriptive:** `mode` (ties — the smaller), `quantile / prctile` (MATLAB method: sorted values are the quantiles
  (k − 0.5)/n, linear in between, min/max outside), `skewness / kurtosis` (biased and `flag = 0`), `cov`,
  `corrcoef` (`[R, P]`), `histcounts` (edges or number of bins); `NaN` is missing. Also `mean median std var`.
- **Distributions** (`statrs`, MIT; scalar parameters expand): `normpdf normcdf norminv tpdf tcdf tinv
  chi2pdf chi2cdf chi2inv fpdf fcdf finv binopdf binocdf binoinv poisspdf poisscdf poissinv erf erfc`.
  `erf`, `erfc` (and `gamma`) come from `libm`, not `statrs`, whose `erfc` is less accurate (scenario `dist-01`, and
  "Boost.Math" below); `norminv` is a statrs start + 2 Newton steps on the exact Φ.
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

### Statistics gates

- **Monte Carlo for p** (`mlab mcgate [-k 20000] [--fault p:ttest]`, `src/mcgate.rs`, seed 20260926): under H0
  a correct p is uniform, so the fraction of p ≤ α over K = 20,000 simulations on small samples must be α ± 4.5·√(α(1 − α)/K)
  (+0.004 for approximate tests: Welch, χ²). 8 tests × α ∈ {0.01, 0.05, 0.1, 0.5}: ttest (n = 5), right-tailed (n = 6),
  ttest2 (4 and 7), Welch (6 and 9, σ 1 and 2), anova1 (3 × 4), corr (n = 6), fitlm slope p (n = 7), χ² 2×3
  (n = 300).
- **Negative control:** `--fault p:<test>` plants a wrong p (`p:ttest` — normal instead of t, `p:anova1` — χ²/df1
  instead of F, `p:chi2test` — df + 1; also `p:ttest2`, `p:corr`, `p:fitlm`); the gate must turn red.
- **fitlm versus `\`:** QR coefficients are compared with the interpreter's `X \ y` (with its own gate); a discrepancy
  > 10⁻⁸ gives `warning: gate fitlm: …`. Control `--fault gate:fitlm` — the warning appears; on correct answers — silence.

### Suite `suite-v2`

`data/suite-v2.txt` covers tables, files, descriptive statistics, distributions, tests, regression and errors. It was
written and frozen before the code: sha256 `012d330d0a428cf4242bc90a45f4978f222549fdd64c943030adf1a1bd1508c1` (test
`suite_v2_is_frozen`). Expected values come from exact formulas and identities (t² = F, correlation p = slope p,
χ² 2×2 = n(ad − bc)²/(r1·r2·c1·c2), F(1, ν) = t(ν)², binomial and Poisson fractions) and from textbook and R
documentation examples: sleep (Student 1908), PlantGrowth, Agresti (2007), Anscombe's quartet.

- Negative control: broken builtins under `--fault NAME` turn it red (test `negative_control_v2_broken_builtins_turn_red`).
- A CSV file with an error gives a clear error with line and column (scenarios io-04, io-05; test
  `csv_and_json_errors_name_line_and_column`).
- The CSV round trip keeps types and tricky text (comma, quotes, leading space, `"007"`, empty string versus missing,
  a leap-year date): test `csv_round_trip_keeps_types_and_tricky_text`.

Grouping is a (stable) sort of indices by keys and a pass over runs; joins use a hash index of the right table.
Assigning a column does not copy the table (`Rc::make_mut`). `examples/bench_table.m` is a 1M-row table benchmark.

### Table and statistics differences from MATLAB

- Text columns are our `string` class; `{…}` is only a list of strings; `T{…}` and `T.Properties` — no.
- Dates are datenum numbers in a column of type "date" (printed as ISO); there is no `datetime`; dates come from files,
  `T.d = datenum(…)` gives a numeric column.
- `histcounts(x, n)` — n equal bins from min to max (MATLAB picks "round" edges); without n — Sturges' rule.
- The test report is printed by default (in MATLAB tests are silent); `'Display','off'` suppresses it.
- `anova1` without a plot; `tbl` is a table `Source SS df MS F p`. `ttest2` with `'Vartype','unequal'`: `stats` has
  `sdx`, `sdy` instead of an `sd` vector.
- `groupsummary` skips `NaN` in methods; `outerjoin` and `'omitnan'` in `mean` are not supported.

## Register VM

A register VM, as in Lua 5 ([docs/context.md](../docs/context.md#execution-sandbox-the-slm-programs-bytecode-an-in-memory-file-system)).
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

- `suite-v2` falls back more often than `suite-v1`: table column assignments `T.c = …` and lists `{'a','b'}`.
- `mlab test … --vm` runs the suites on the VM; under broken builtins (`--fault`) the tree and the VM must also agree.
- **Negative control:** `--fault vm:add` (scalar addition in the VM gives +1) must turn the gate red (test
  `vm_gate_negative_control_turns_red`).
- **Edge cases** (`vm_edge_cases_match_tree`): a name that is sometimes a variable, sometimes a function; `end` in
  nested calls; `break` from `try`/`switch` in fallback; the recursion limit 256; `nargin`/`nargout`; multiple outputs;
  capture in anonymous functions; growth and deletion; logical masks; complex and char; loops over a matrix, an empty
  value, a scalar, a string; tables in a loop; copy-on-write; error texts. In a debug build the tree walker needs a
  deep stack, so this test runs in a separate thread with a larger stack.

`mlab vmbench examples/vmbench` times the tree and the VM (best of 3, including parsing and compilation) and checks
that both give the same output.

### WASM

`rustup target add wasm32-wasip1` → `cargo build --release --target wasm32-wasip1` builds `mlab.wasm` without changes
(with vfs); it runs in our WASI host (section "mlab.wasm in our WASI host").

### VM limits

- These fall back: `switch`, `try`, commands, fields and tables on the left-hand side, cell lists, an index not on a name
  (`f(x)(2)`), multiple outputs with a variable on the right-hand side. Correct, but in a hot loop it runs at tree speed
  plus variable moves.
- Anonymous function bodies are executed by the tree (`@(x) x.^2` in `arrayfun`, `integral`).
- Scalar builtins go through the `&[Value]` ABI, so every scalar is wrapped into a `Mat` and back.

## Independent tests

The `suite-v1`/`suite-v2` suites share an author with the code, so they check conformance to the specification. These
tests use independent sources: scenarios written by the teacher and external reference data.

### LLM scenarios (`tests/data/vmm/`, `tests/vmm_suites.rs`)

- **The author is the teacher** (`math/src/bin/vmmtests.rs` — reuses `math::vmm::Vmm`, cap and log). It did not see
  the mlab code: the prompt is the language documentation (what exists, what does not, deliberate differences) and
  rules for unambiguous output (`printf`, `mat2str`, tolerance checks for iterative methods). Prompts —
  `tests/data/vmm/prompts/`.
- 10 areas: indexing, operators, strings, control flow and functions, reductions and element-wise, linear algebra,
  polynomials and numerical methods, tables, statistics, edge cases. Each header has `src:` (identity, exact formula,
  documentation, manual calculation).
- Frozen by hashes before they were run (the hashes are in the test). Every scenario also runs through the tree ↔ VM
  equivalence gate.

### NIST StRD (`tests/data/ext/nist/`, `tests/ext_nist.rs`)

NIST Statistical Reference Datasets (public domain, `.dat` files as is): regressions, ANOVA and descriptive
statistics. The yardstick is LRE, the number of correct significant digits (McCullough 1998), capped at 15; each
dataset has a minimum LRE threshold in the test.

- **`\` for rectangular matrices** is **Householder QR with column pivoting** (`lstsq_qr`), rank by
  |R_kk| > max(m, n)·eps·|R₁₁|; rank deficiency — minimum-norm SVD with a warning. Truncated SVD is not used: it is not
  invariant to column scaling (raw powers of x).
- **anova1** shifts the data by the first observation (SS do not depend on the shift) and makes a correction pass for the
  means. On data such as SmLs07–09 (10¹² + 0.x) the double representation itself limits the attainable digits.
- Negative control: a broken `mean` must fall below the threshold; Filip via `\` must warn "rank deficient" (as in
  MATLAB).

### Boost.Math (`tests/data/ext/boost/`, `tests/ext_boost.rs`)

Boost 1.92.0 test data (BSL-1.0, `LICENSE_1_0.txt` alongside): values with 35–40 digits for `erf`, `erfc` and `gamma`;
the test bounds the relative error in eps. `erf`, `erfc` and `tgamma` come from `libm` 0.2.16 (MIT, a port of
musl/fdlibm; already in the tree via statrs), not from home-grown formulas, which lose accuracy through cancellation
near zero and near negative integers. mlab has no beta function.

### Octave tests (`tests/octave_ext.rs`, `#[ignore]`)

`%!assert`, `%!test`, `%!error` blocks from the GNU Octave 11.3.0 source files (GPL: only in
`$MOVA_DATA/raw/tests-octave/`, they do not enter the repo; the tool takes only `%!` lines, the implementation code is
not read). Each block is a separate `mlab run` process with a limit of 10 s; only blocks all of whose calls are mlab
builtins are run. A panic instead of an error counts as a crash; the regression cases (`trapz(h, Y)` with a scalar
step, `polyfit(x, y, Inf)`, `primes(Inf)`, `corr` of constant data, `corrcoef([])`) are errors.

## File system: vfs

The `vfs` crate (`../vfs/README.md`) is an in-memory file system with snapshots. mlab accesses files only
through it.

- **All file builtins go through the `vfs::FileSystem` trait** (`Interp::fs`):
  - `readtable writetable readmatrix writematrix fileread writelines tempname`;
  - `isfile isfolder mkdir rmdir delete movefile copyfile readdir`;
  - `exist(name)` also sees files (2) and folders (7), as well as `exist(name, 'var'|'builtin'|'file'|'dir')`.
- **Modes** (`--fs` for `run`, `-e`, `test`):

  | mode | what the program sees | when |
  |---|---|---|
  | `--fs mem` (default) | only the in-memory file system: `/` and `/tmp`; the real FS is not visible | foreign or generated code, tests, suites |
  | `--fs dir:<folder>` | a real folder as the root (chroot): `..` above the root and links pointing outside are errors | operator data |
  | `--fs real` | the real FS without restrictions | only explicitly |

  `Interp::new` also uses `mem`: the library is sandboxed by default.
- **Memory contents from folders and to folders, only explicitly** (only with `--fs mem`):
  - `--fs-import DIR[:VIRT]` — before the run, by default into `/`;
  - `--fs-export DIR[:VIRT]` — after the run, **only if the script finished without an error**: only approved results
    leave the sandbox. Otherwise — a "skipped" message.
- `--fs-limits total=256M,file=64M,files=65536` — memory limits. A violation is an error with an explanation:
  `writematrix: cannot write '/big.csv': vfs limit: file '/big.csv' would be 12345 bytes, max_file_size = 2000`.
- `--fs-journal FILE.tsv` — an operation journal (TSV): what was read and written, errors, attempts to escape the root.
- **`tempname` in `mem` and `dir:`** — `/tmp/mlab_<k>`, deterministic. In `real` — `temp_dir()/mlab_<pid>_<k>_<ns>`.
- **Error texts are those of the OS:** "No such file or directory (os error 2)". Escaping the root — "path escapes the
  sandbox root: '../x'".

### vfs gates (`tests/fs.rs`, `tests/suite.rs`)

- **The frozen suites give byte-for-byte the same output under `--fs mem` as with `std::fs`:** the test
  `frozen_suites_same_output_in_mem_real_and_dir` requires identical output of every scenario in `mem`, `real` and `dir:`
  (a fresh folder in `tmp/`), tree and VM.
- **The file suite `data/suite-fs.txt`:**
  - what it checks: files, folders, paths with `./` and `../`, `readdir` in byte order, OS errors, 50 files in a folder,
    a non-Latin name, a tricky CSV;
  - expected output checked by hand against MATLAB/Octave semantics;
  - frozen: sha256 `782a847e4d64c4fe85b0de2d92e740b78c972d72aae0043b846e2d23c30f0e94` (test
    `suite_fs_is_frozen`);
  - `dir:` must give the same output as `mem`.
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

The guest must give the same output as native mlab with `--fs mem` on `suite-fs`, `suite-v1` and `suite-v2` (tree
and VM). The exception is the errata scenario la-18 (a singular matrix): the faer build for wasm32 reports "svd: failed
to converge".

## Known differences from Octave/MATLAB

- `rand`/`randn` — our own stream (xoshiro256**), not the Mersenne Twister: deterministic, but different numbers. Seeding:
  `rand('seed'|'state'|'twister', s)`, `rng(s)`, `rng('default')`.
- `eye`/`diag` — full matrices (Octave shows "Diagonal Matrix").
- `ode45` returns only accepted steps (Refine = 1; in MATLAB/Octave — 4); `odeset` options — after structs.
- `A \ b` for a singular square matrix — a warning and the minimum-norm solution (verified by the gate).
- e-format thresholds and error texts are our specification in the style of Octave (`error: operator +: nonconformant
  arguments (op1 is 1x2, op2 is 1x3)`, `error: 'x' undefined`, `error: v(5): out of bound 3 (dimensions are 1x3)`).
- `strrep` with overlapping matches behaves like Octave (`'ba'`); MATLAB gives `'bb'`. `fitlm` returns a table, not a
  `LinearModel` object.
- `unique` returns indices of first occurrences; `mat2str` of strings uses double quotes (as Octave);
  `num2str` of non-integer matrices has simplified alignment; `asin`/`acos` outside [−1, 1] give NaN (not complex).

## Licenses

- Code — the Elamur Cosmic License (`LICENSE.md` at the repo root).
- The scenario suite and the format specification are our own.
- Test data: NIST StRD — public domain (US government works, 17 U.S.C. § 105; `tests/data/ext/nist/`); Boost.Math — Boost Software License 1.0 (`tests/data/ext/boost/LICENSE_1_0.txt`, `.ipp` files with the author line unchanged); LLM scenarios — our own (generated by the teacher for us). Octave tests (GPL) and Matrix Market — only in `$MOVA_DATA/raw/`, not in the repo.
- `vfs` is our own crate without dependencies; no new third-party crates were added to mlab.

Dependencies (the `cargo tree` tree; each crate's license was checked against the crates.io API
`/api/v1/crates/<name>/<version>` on 2026-09-26):

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
| `libm` | 0.2.16 | MIT | direct (erf, erfc, tgamma); in the binary |
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
| `statrs` | 0.19.1 | MIT | direct; in the binary (without default features: no nalgebra or rand) |
| `approx` | 0.5.1 | Apache-2.0 | in the binary |
| `thiserror` | 2.0.21 | MIT OR Apache-2.0 | in the binary |
| `thiserror-impl` | 2.0.21 | MIT OR Apache-2.0 | build only (proc-macro) |

All crates match crates.io; statrs — MIT, approx — Apache-2.0, thiserror and
thiserror-impl — MIT OR Apache-2.0. CSV and
JSON are our own code, without crates. Summary: MIT — 29, MIT OR Apache-2.0 — 22, `bytemuck` — Zlib OR Apache-2.0 OR MIT (we take MIT), `unicode-ident` — (MIT OR Apache-2.0) AND Unicode-3.0. Unicode-3.0 is the permissive Unicode data license (OSI), and this crate is used only at build time (the proc-macro2 → syn chain), it does not end up in the binary. GPL/LGPL — none. The benchmark (`bench/`) additionally pulls in `nalgebra` (Apache-2.0) and its dependencies — only for measurement, not in mlab.

## Limits

- **No exact mode.** The `Value::Exact` slot exists; rational matrices and big integers from `math`, exact `det`, `inv`,
  `\`, `rank`, `rref`, `format rat` and step output are not implemented.
- **No cell arrays, structs, N-dimensional arrays or integer types**, so no `odeset`, `err.message`,
  `[p, S] = polyfit`, `cellfun`, `strsplit`, `varargin`/`varargout`.
- **No plots, no XLSX/ODS, no `datetime`, `T{…}`, `outerjoin`, `'omitnan'`, Fisher's exact test, non-parametric
  tests, p-value corrections, `beta`/`gammaln`/`betainc`.**
- **Open defects:** `A.^2'` (transpose and power — one precedence level, left
  to right), the loop variable after an empty `for` (must be 1×0), `T.x(k) = v`, command syntax for any function.
- Failing `assert` blocks of the Octave tests are not classified (defect, MATLAB/Octave difference or out of scope).
- The VM falls back to the tree for `switch`, `try` and the cases listed under "VM limits".

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
