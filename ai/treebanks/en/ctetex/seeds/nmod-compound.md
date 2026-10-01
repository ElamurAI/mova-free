# CTeTex — a noun before a noun: half `nmod`, half `compound`

**Gist.** An English noun before a head noun (*water use*, *Lag Frames*) is `compound` in UD. EWT annotates 4750 of 4784 such pairs this way. CTeTex uses `compound` only in 324 of 638 pairs; in 307 it is `nmod` without a preposition. Among the `nmod` pairs there are many capitalized terms (*Subscription Version*, *Redundancy Management*, *Provider ID*); in EWT such terms are `compound`.

**Conditions and exceptions.**
- Adjacent nouns where the left one depends on the right one:
  - `compound` 324;
  - `nmod` 307;
  - `amod` 6;
  - `nsubj` 1.
- The most frequent `nmod` pairs: *Subscription Version* 8, *Redundancy Management* 7, *Provider ID* 5, *water use* 4, *User Location* 4, *System Initialization* 4, *RAM scrub* 4.
- The CTeTex guideline (Hassert et al. 2021) does not explain this choice. It seems to treat terms as modifiers rather than parts of a compound, as in the Italian TUT (`../../partut/seeds/nmod-compound.md`).

**Examples.** `199` *Lag Frames – The BE shall receive LTA or Speed Dump Lag Frames…* — *Lag*: `nmod` → *Frames*, both times. In EWT — `compound`.

**In UD.**

| rule | CTeTex | ParTUT | EWT |
|---|---:|---:|---:|
| `tb.en.nn-compound`: a noun immediately before a head noun not `compound` | 314 / 726 | 862 / 974 | 19 / 5376 |

In the first column the rule counts matches — all pairs of adjacent NOUN — and violations — those that depend on the right neighbour other than as `compound`.

**Sources.** https://universaldependencies.org/en/dep/compound.html, https://universaldependencies.org/u/dep/nmod.html (`nmod` — usually with `case`); Hassert et al. 2021, `2021.udw-1.5`, sec. 4.4 (specialized vocabulary).
