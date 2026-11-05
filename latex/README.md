# latex — built-in LaTeX support

**Goal:** built-in, compiled LaTeX support. Math-mode formulas → our own tree (enums) → back to canonical
LaTeX or plain text. Everything is in the binary, no external TeX needed; on the website formulas are drawn by
KaTeX. Its place in the mini-lab is described in [docs/context.md](../docs/context.md#mini-lab-a-matlaboctave-style-language-and-built-in-latex).

## How it is built

| file | what it does |
|---|---|
| `data/commands.txt` + `build.rs` | **a single command table** → enums `Sym` (TeX class, Unicode, whether it is a delimiter), `FracKind`, `Accent`, `Font`, `TextStyle`, `Stack`, `XArrow`, `EnvKind` and constant `match`es: name → `Entry`, character (`≤`, `α`) → `Sym`, environment → `EnvKind`. The first form is canonical for printing (`\leq` → `\le`). A duplicate form or an unknown kind or class is a build error |
| `table.rs` | module `table`: includes the code that `build.rs` generates from `data/commands.txt` (the enums above; lookups `lookup_cmd`, `lookup_char`, `lookup_env`) and holds the hand-written enums `Class` and `Entry` |
| `lex.rs` | math-mode lexer (TeX rules: `\alpha` eats spaces, `%` is a comment, spaces merge) and a stack stream onto which a macro pushes its body |
| `macros.rs` | `\newcommand`/`\renewcommand`/`\providecommand` with `[n][default]`, `\def` with `#1…#9` (and with delimiters, as in TeX: `\def\pair(#1,#2){…}`), `\let` (keeps the old value), `\DeclareMathOperator(*)`; `Macros::scan(tex)` collects definitions from a document |
| `parse.rs` | parser: tokens → `Node`; errors with a byte position (`Error::column`, `Error::context` — a `⟦` mark in the source) |
| `ast.rs` | tree: `Row`, `Num`, `Char`, `Sym`, `Group`, `Frac`, `Sqrt`, `Scripts` (base, scripts, primes), `OpName`, `LeftRight`/`Middle`, `Env` (tables), `Font`, `Text`, `Accent`, `Stack`, `XArrow`, `Class`, `Not`, `Unknown` (name, arguments, position). `Span` does not take part in `==` |
| `print.rs` | `to_latex` (canonical LaTeX), `to_text` (Unicode: `(a + b)/2`, `x²`, `ℝⁿ`, matrix `(a b; c d)`), `to_sexpr` (debugging), `to_latex_faulty` (deliberate defects for the negative control) |
| `gate.rs` | round-trip gate `parse(print(parse(x))) == parse(x)`; `affected` — whether a defect touches the tree (for a precise negative control) |
| `extract.rs` | formulas from `.tex`: `$…$`, `$$…$$`, `\(…\)`, `\[…\]`, `equation`, `align`, `alignat{n}`, `gather`, `multline`, `eqnarray`, `flalign`, `displaymath`, `math` (and starred forms); comments, `\$`, `\verb`, `verbatim` are skipped; `$…$` does not cross an empty line |
| `tomath.rs` (feature `math`) | bridge to `math::Expr` and back (see below) |
| `main.rs`, `corpus.rs` | CLI: `parse`, `tex`, `corpus` (arXiv metadata `.xml.gz`, `.tex`) |

### What is supported

- numbers (`3.14`; a space breaks a number), Latin and Greek letters (including `\var…`, `\Alpha…`, Unicode `α`, `≤`, `ℝ`);
- `\frac \dfrac \tfrac \cfrac \binom \dbinom \tbinom \genfrac`, infix `\over \choose \atop \brack \brace`;
  `\sqrt[n]{}`, `\root n \of x`; superscripts and subscripts (as in TeX — to the preceding atom, `^{14}C` — to `{}`),
  primes `f''`, groups `{}`;
- 493 symbols (800 command names together with the other kinds, 24 environment names): operators and relations (`+ - \cdot \times \div = \ne \le \ge < > \in \subset \to \mapsto …`,
  `\not X` → negated symbol), arrows, delimiters, punctuation, `\dots`;
- functions `\sin \cos \log \ln \exp … \lim \max \sup \det \gcd \argmax`, `\operatorname{…}`/`\operatorname*`;
  big operators with limits `\sum \prod \int \iint \oint \bigcup …` (`\limits`/`\nolimits` are dropped);
- `\left … \middle … \right` (with `.`); sizes `\big…` are dropped;
- tables: `matrix pmatrix bmatrix Bmatrix vmatrix Vmatrix smallmatrix` (and `*`), `array`, `subarray`, `cases`,
  `dcases`, `rcases`, `aligned`, `alignedat`, `gathered`, `split`, `\substack`; plain TeX `\pmatrix{…\cr…}`,
  AMS-TeX `\aligned…\endaligned`; `&` and `\\` at the top level — implicit `aligned` (the body of `align`);
- fonts `\mathbb \mathbf \mathrm \mathcal \mathscr \mathfrak \mathit \mathsf \mathtt \boldsymbol \bm` (and
  old-style `{\bf x}`, `{\cal A}`, `\Bbb`), text `\text \mbox \textrm \textbf …` (with `$…$` inside),
  accents `\hat \bar \vec \tilde \dot \overline \widehat \underbrace \boxed \cancel \pmod …`,
  `\overset \underset \stackrel`, `\xrightarrow[…]{…}`, `\mathrel{…}` etc., `\unicode{x…}`;
- spacing (`\, \; \quad \hspace{…} \kern…`), style, labels (`\label \tag \nonumber \eqno`), colour, `\phantom` —
  dropped; `\ensuremath`, `\smash`, `\rlap` — content is merged into the row;
- **an unknown command becomes an `Unknown(name, group arguments, position)` node**; an unknown environment becomes an `Env` with
  its name and position. An argument of an unknown command that does not parse as a formula (`\setpres{…}{$x$ for all $s$}`)
  is kept as text. Nothing is lost silently;
- unbalanced input (`{`, `}`, `\left`/`\right`, `\begin`/`\end`, `[`), a double script, `$` inside a formula — an error
  with a position. Lenient, like MathJax: `$#P$` → `\#P`, `CP^2#2` → `\#2`.

## Bridge to `math::Expr` (feature `math`)

`to_expr` covers a subset: numbers, variables, `+ − · × / ÷` and implicit multiplication, unary minus, fractions, roots
(`\sqrt[3]{x}` → `x^(1/3)`), powers, `\sin \cos \tan \exp \ln`, absolute value `|x|`, parentheses (`(x+1)^2` — in TeX the script
attaches to `)`, and the bridge accounts for this), `e`, `\pi`. `\sin 2x` = sin(2x), `\sin x\cos x` = sin(x)·cos(x),
`\sin^2 x` = (sin x)². `\frac{1}{2}` (an irreducible fraction of integers) is a rational literal; `\frac{2}{4}` stays
a division (no step is lost). `to_relation` handles a chain `a = b ≤ c`. A refusal carries the kind of reason ("variable",
"script", "function", "construct", "relation", "syntax", …). `from_expr` is the reverse side: printing `math` steps and
answers in LaTeX; a test checks `Expr → LaTeX → tree → Expr` on a set of expressions (with normalization
`Div(1,2)` ≡ `1/2`). The bridge targets the `math eval` expression tree (`math::Expr`), not the `mathsolve` plan language.

## CLI

```
cargo build --release [--features math]
../target/release/latex parse '\frac{a+b}{2} \le \sqrt[3]{x}'   # tree, LaTeX, text, unknowns, round trip (and math::Expr)
../target/release/latex tex main.tex chap1.tex -v                # document formulas with macros from all files
../target/release/latex corpus $MOVA_DATA/raw/math/arxiv-oai --out DIR [--math] [--fault swapfrac|dropsup|droplast] [--limit N]
```

## Real formulas

`latex corpus` reads arXiv abstracts and titles (`$MOVA_DATA/raw/math/arxiv-oai/`, OAI-PMH arXivRaw metadata,
`.xml.gz`; this source has no `.tex`), removes duplicates between sets and reports for each formula: parsed or error,
`Unknown` nodes, round trip, and the bridge to `math::Expr` with the reason for each refusal. It writes the list of
unknown commands (`unknown.tsv` in `--out`). Abstracts have no preamble, so author macros (`\R`, `\Z`, `\eps`, …)
show up as unknowns; in `.tex` documents they are expanded by `Macros::scan`.

### Negative controls

- unbalanced input → an error with the exact position (cases in `tests/gate.rs`);
- a corrupted printer (`--fault swapfrac|dropsup|droplast`: numerator ↔ denominator, dropped superscripts, the last
  atom of a row of three or more dropped) must turn the gate red exactly on the formulas the defect touches (`affected`)
  and nowhere else;
- a macro loop (`\def\a{\a}`) is an error, not a hang; random token inputs must not panic, and everything parsed must
  pass the round trip; bridge: a different expression gives a different tree.

`cargo test` and `cargo test --features math` run unit, end-to-end, bridge and property tests on random inputs.

## Independent tests

`tests/data/vmm/latex.tsv` holds cases from an independent author (the teacher, which did not see the code; prompts
alongside), frozen by hash before they were run. Kinds: `same` and `diff` (tree equality without positions), `err`
(must be an error), `ok` (no `Unknown`, round trip), `value` (value through the bridge to `math::Expr`, only with
`--features math`), `count` (formulas in a document fragment), `macro`. The runner is `tests/vmm_latex.rs`; it reads
"\n" in the format as a line break only when it is not followed by a letter, so `\newcommand` survives. Its negative
control swaps `same` and `diff`.

## Licenses

- Code — the Elamur Cosmic License (`LICENSE.md` at the repo root). The command table is our own, from our own knowledge of
  LaTeX/amsmath/amssymb; the KaTeX function list (MIT) served only as a reference for what to support (no KaTeX files were taken).
- Dependencies: `flate2` (MIT/Apache-2.0, feature `gz` — only the CLI for `.xml.gz`), `math` (our own, feature `math`).
- arXiv abstracts are used only for measurement and do not go into the repo.

## Limits

- `\sideset`, `\tensor`, `\prescript`, `CD` diagrams (`@>>>`), `\genfrac` with non-standard delimiters — as `\left…\right`.
- Text in `\text{…}` is stored as a canonicalized string; its inner `$…$` are not parsed into the tree.
- A preset of common author shorthands (`\R` → `\mathbb{R}`, `\eps` → `\varepsilon`) — only as an explicit option, not by
  default: it is a guess about the author's intent. Not implemented.
- The bridge is limited by the `math` variable table; scripts (`x_1`), sums, integrals, matrices are refused.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
