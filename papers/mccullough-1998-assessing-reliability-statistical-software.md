# Assessing the Reliability of Statistical Software: Part I

**Authors:** B. D. McCullough · **Year:** 1998 · **Venue:** The American Statistician, vol. 52, no. 4
**Link:** unknown
**License of the paper:** unknown

## Summary
The paper proposes a systematic methodology for testing the numerical accuracy of statistical software packages. It argues that users tend to trust the numbers a package prints, although numerical errors in estimation, random number generation and statistical distributions can be serious. The method has three parts: tests of estimation (using the NIST Statistical Reference Datasets with certified values for univariate statistics, ANOVA, linear and nonlinear regression), tests of random number generators, and tests of statistical distribution functions. Accuracy is summarised with the log relative error (LRE), which roughly counts the number of correct significant digits in a computed result compared with the certified value. Datasets are graded by difficulty, so a package can be judged by how gracefully its accuracy degrades on ill-conditioned problems. The work became a standard reference for benchmarking statistical and spreadsheet software.

## How Mova uses it
- `ai/mlab/tests/ext_nist.rs` — the MATLAB-compatible numeric engine `mlab` is tested on NIST StRD datasets (11 regressions, 11 ANOVA, 9 univariate; data files under `ai/mlab/tests/data/ext/nist/`).
- The yardstick is LRE as defined in this paper, capped at 15 digits; the test prints a table of minimum LRE per dataset for `fitlm`, the backslash solver and `anova1`.
- The methodology is adopted directly (certified values, difficulty grades, LRE); no part of the paper is reimplemented beyond the metric.

## Effectiveness in Mova
This paper supplies a metric rather than a model idea, so its value is in the defects it exposed. From the `mlab` README (minimum LRE, before → after fixes): backslash on Pontius 4.9 → 13.5, Longley 9.1 → 11.4, Wampler2 10.3 → 12.5; on Filip (degree-10 polynomial) the solver went from silently returning 0 correct digits to a "rank deficient" warning; `anova1` on SmLs07–09 improved from 2.7 / 2.0 / 0.0 to 4.0 / 3.9 / 3.9, and on AtmWtAg/SmLs04–06 from 6.5–8.5 to 9.9–10.2.

---

👨‍🔬💥
