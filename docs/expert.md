# An expert system for English grammar: seeds and rules

Design brief (25.09.2026):

> With a large body of English grammar literature and free dictionaries at hand, build an expert system that tests and assists the annotator: it checks the annotator's output, drawing on solid knowledge of grammar and of the structure of word parts.

> The expert system itself must learn from materials, i.e. scientific papers and dictionaries, not from examples as the annotator does. The expert system can be trained by Opus 5.5 high.

> In parallel with other sub-agents, read books and scientific papers from the library, summarize them, and record the summaries as seeds in the English folder.

## How it works

1. **Seeds**: `en/seeds/<section>/<name>.md`. A seed is like the stone of a cherry: the kernel a rule grows from, not a bone; hence the folder name `seeds`. Each holds one piece of knowledge about English, with a source: a book page or a paper id. Sections:
   - `morph`: inflection and word structure: stems, prefixes, suffixes;
   - `nominal`: the noun phrase;
   - `verbal`: the verb and the clause;
   - `lexicon`: dictionaries and valency;
   - `errors`: typical errors of parsers, LLMs and learners.
2. **Rule**: a ```` ```rule ```` block in a seed, in the language below. It is executed by a deterministic Rust engine (`en::expert`).
3. **Self-check of rules against gold.** Every rule is run over UD EWT:
   - a rule that fires often on gold is either wrong or catches gold errors; Opus reviews these cases and refines the rule;
   - examples only check rules, they do not generate them (the "examples are not truth" principle).
4. **Uses:**
   - checking annotation from Opus, Rust and silver;
   - hints for Opus in edit mode: "word 5 violates rule X";
   - hard constraints for the parser beam.

## Seed format

```markdown
# <short title>

**Gist.** 1–3 sentences: what the grammar says.
**Conditions and exceptions.** …
**Examples.** 2–4 short English examples, original or quotes of up to ~10 words.
**In UD.** How the phenomenon looks in UD 2.18 (EWT) annotation: UPOS, XPOS, FEATS, relations.
**Sources.** Jespersen MEG VI §3.2 (books/j/je/…, vol. 5, p. 123); Santorini 1990, VBZ; UD `docs/_en/…`; `2023.udw-1.7`.

```rule
…
```
```

## Rule language (v1)

```rule
rule: en.errors.singular-subject-vbp
what: singular noun subject before a VBP verb: missing -s or wrong subject
match: v[xpos=VBP]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN|PROPN, feats.Number=Sing, before=v]
require: exists c[rel=conj, head=s]
unless: s[lemma=lot|number|majority|rest|half|percent]; s[lemma=family|team|staff|government]
severity: warn
source: P17-1074 (VERB:SVA); Fowler MEU 1926, NUMBER
```

- **`match`.** One or more nodes separated by `;`. A node is written as `name[condition, condition, …]`: all conditions together. The engine enumerates all bindings of distinct words in the sentence that satisfy the conditions.
- **`require`.** What must hold for each binding; if not, it is a violation. Clauses are separated by `;` and all must hold. A clause can list alternatives with ` or `; one is enough: `require: s[feats.Number=Plur] or exists c[rel=conj, head=s]`. Clause forms:
  - `name[conditions]`: on an already bound node;
  - `not name[conditions]`: prohibition;
  - `exists c[conditions]`: another, unbound word with these conditions exists; it may refer to nodes from `match`;
  - `none c[conditions]`: no such word exists.
- **`unless`** (v1). Exceptions that cancel a violation: clauses separated by `;`, same forms as in `require`. If at least one clause holds for a binding, there is no violation.
- **Conditions:**

  | condition | meaning |
  | --- | --- |
  | `upos=NOUN`, `upos=NOUN\|PROPN` | part of speech (one of several) |
  | `xpos=VBZ` | PTB tag |
  | `lemma=be`, `form=that` | lemma, form (case-insensitive) |
  | `feats.Number=Sing`, `feats.Number!=Plur` | feature value equals / does not equal |
  | `feats.Number`, `!feats.Number` | feature present / absent |
  | `rel=nsubj:pass` | exact relation, with subtype |
  | `rel~nsubj` | relation by base: nsubj, nsubj:pass, nsubj:outer |
  | `head=v`, `head=0` | the word's head is node v / the root |
  | `before=v`, `after=v` | word order |
  | `suffix=ness`, `prefix=un` | end / start of the form, case-insensitive |
  | v1: `upos!=`, `xpos!=`, `lemma!=`, `form!=`, `rel!=`, `head!=0`… | negation of any condition; `lemma!=lot\|number` means "none of the values" |
  | v1: `rel!~nmod` | relation not of this base |
  | v1: `feats.Person=@s`, `feats.Number!=@s` | feature same as on node `s` / different |
  | v1: `lemma=@form` | lemma equals form, case-insensitive |
  | v1: `lemma.suffix=y`, `lemma.prefix=un` | end / start of the lemma |
  | v1: `next=v`, `prev=v` | word immediately after / immediately before v |
- **`severity`:** `error`: the annotation is definitely wrong; `warn`: a suspicion.
- **`source`:** where the knowledge comes from. Mandatory.

**v1 examples:**
- `a[upos=ADJ, rel=amod, after=n, lemma!=involved|due|elect, lemma.suffix!=able|ible]`: a postpositive adjective, except fixed ones;
- `require: r[feats.Person=@s]; r[feats.Number=@s] or not s[feats.Number]`: a reflexive pronoun agrees with the subject without spelling out each person;
- `match: t[form=mr.|dr.]; n[upos=PROPN, next=t]`: a title immediately before a name;
- `unless: none c[before=v, upos!=PUNCT|CCONJ|ADV|INTJ]`: before the verb the clause has only punctuation, conjunctions, adverbs and interjections: the subject is omitted at the start.

**How the engine resolves ambiguity** (the simplest solution):
- `unless` is checked only for bindings where `require` failed; it does not reduce the "fired" count.
- There is no "and" between conditions on different nodes within one `unless` clause. A needed "and" is written in `require` as several clauses with `or`, i.e. in conjunctive normal form. For example, "same number, or Ptan with a plural demonstrative": `require: n[feats.Number=@d] or d[feats.Number=Plur]; n[feats.Number=@d] or n[feats.Number=Ptan]`.
- `feats.X=@s`: if `s` lacks feature X, the condition is false; `feats.X!=@s` is its negation and is then true. `@` occurs only in `feats.X` and in `lemma=@form`; node `s` comes from `match`.
- `next`/`prev` are neighbors by CoNLL-U word numbers; punctuation marks are words too.
- `;` and ` or ` split only outside `[…]`, `,` inside a node; a comma inside a value (`PronType=Int,Rel`) is not supported.
- A feature outside the `Feat` list in `feats.X` / `!feats.X` (e.g. Aspect) behaves as in v0: it is always absent, because the CoNLL-U reader drops unknown pairs. In `feats.X=@s` such a feature is a parse error.
- A parse error, not silence (v1):
  - unknown field, say `unles:`;
  - a field given twice;
  - a node given twice in `match`;
  - a `head=`, `before=`, `after=`, `next=`, `prev=` or `@` reference to a node not from `match`.

**`en expert-check`** prints up to three examples per rule. With `EN_EXPERT_LIST=1` it also prints all violations as a list at the end: sent_id, rule, severity, with `--gold` whether it is true or false, nodes `name=number:form`, sentence text.

Mova builds the engine and the gold check of rules (`en::expert`, `en expert-check`). Sub-agents write seeds and rules.

## Status (26.09.2026)

Five sub-agents (Opus) read the grammars and papers in the library and wrote 230 seeds with 442 rules. After v1 there are 422 rules: duplicates merged, agreement rewritten with `@` (below). The engine parses all rules and there are no duplicate ids; this is a `cargo test` gate (`seeds_all_parse`).

| section | seeds | rules (25.09 → v1) | of which error |
|---|---:|---:|---:|
| `morph`: inflection, word formation | 59 | 151 | 112 |
| `nominal`: noun phrase | 57 | 106 → 99 | 57 |
| `verbal`: verb and clause | 50 | 81 → 82 | 44 |
| `lexicon`: valency (VerbNet, PropBank) | 9 | 10 | 0 |
| `errors`: typical errors of parsers, LLMs and learners | 55 | 94 → 80 | 20 → 15 |

Sub-agent reports: `$MOVA_DATA/runs/bones-2026-09-25/<section>.md`.

**Self-check against gold.** On EWT train (12,544 sentences, 5.7 s), only 16 of 213 error-level rules produce violations. That is 1–17 cases per rule, and nearly all are errors in EWT itself: `compound` to the right of its head, *what* as `det:predet`, imperative *Don't* with `Mood=Ind`.

**On our annotation** (`en expert-check en/seeds --gold <gold> <file>`):
- Final Opus high annotation, 292 sentences. Error-rule violations are real errors in 36 cases out of 36. But they cover only 6.4% of erroneous tokens; warn rules add another 3.9%. The most useful rules:
  - `en.errors.tmod-npmod`: 12 of 12. This led to the `:tmod` → `:unmarked` normalization in `annot`;
  - `passive-no-active-subject`: 7 of 7;
  - `nummod-before-noun`: 5 of 5.
- Bronze, i.e. Rust alone on the seed examples, 326 sentences: 66 error-rule violations. Most frequent: "one subject per predicate".

Conclusion: the rules are precise but only see the ungrammatical. So they work as hints for Opus in edit mode ("word 5 violates rule X") and for picking sentences worth promoting to silver. They are not full control.

**Language v1, as requested by the sub-agents, done 26.09** (see "Rule language (v1)" above):
- negation of any condition: `lemma!=`, `form!=`, `upos!=`, `rel!=`, `rel!~`, for exception lists;
- "or" in `require` (` or `) and `unless:`, exceptions that cancel a violation;
- comparing features of two nodes (`feats.Person=@s`): agreement without spelling out each person;
- `lemma=@form`, `lemma.suffix=`, `lemma.prefix=`: spelling;
- "immediately after / before" (`next=v`, `prev=v`);
- up to three examples in the `expert-check` report and a full violation list (`EN_EXPERT_LIST=1`).

Not done yet: access to MISC (`Typo=Yes` requires `CorrectForm`) and "has a dependent with rel=X" without a separate node; for now that is `exists c[rel=X, head=v]`.

**Duplicates merged (26.09).** 17 pairs from the `errors.md` list: the rule stays in one seed, and in the other the block is replaced by the line "Rule: `<id>` in `<file>`"; seed text is unchanged.
- Identical pairs, and pairs where the `nominal` version is broader, stay in the topical seed: `nominal/nummod.md` (two), `nmod-poss.md`, `flat-names.md`, `compound.md`, `there-expletive.md`.
- Where the rules differed, the broader or more precise one was kept and the other's conditions were merged into it:
  - `prp-poss-rel` +WP$;
  - `double-comparison` +RBR;
  - `finite-subject-nom` +WP;
  - `relative-that-not-mark` +PRON;
  - `mass-no-plural` +4 lemmas and forms in -s;
  - `lexicon.iobj-verb`: merged list, 171 lemmas;
  - `errors.title-desc`: title immediately before PROPN;
  - `dem-sing` and `dem-plur` → one `nominal.dem-number` via `@`;
  - the errors versions of `a-singular` and `finite-subject-nominative-auxcop` were kept: more sensitive, with precise exceptions.

**Refining the noisiest rules with v1 (26.09).** Exceptions are taken from the seed text itself and from sources, not from examples:
- `finite-has-subject`: subject omission at the start of a sentence (Jespersen 1924, prosiopesis), fixed *thank/hope*, and clauses with *as/than* (Curme §5 b, d). Asyndetic coordination moved to a new `parataxis-has-subject`: the subject can be taken from the previous clause.
- Subject agreement: quantity and collective nouns (`agreement-notional.md`, Poutsma III p. 304, Curme §8 I 1 d), coordinated antecedent, measures with a numeral, *each/every X and Y*, conjunct after the verb.
- `postposed-adj-has-deps`: fixed postpositives (Curme §10 I 1 a) and *-able/-ible*.
- `reflexive-person-1/2/3` and `reflexive-number-plur` → one `reflexive-agreement` via `@`.
- `passive-subject-aux`: absolute construction and headline.
- `missing-copula`: telegraphic style, when the sentence has no finite verb.
- `bare-count-noun`: article dropped at the start of a sentence; *room* and *company* have a mass reading.

| measurement | before | after |
|---|---:|---:|
| EWT train, violations of all rules | 966 | 576 |
| `finite-has-subject` on EWT train | 257 | 9, plus 17 in `parataxis-has-subject` |
| `final.conllu` (Opus, 292 sentences): violations / real | 78 / 63 | 60 / 57 |
| `final.conllu`: coverage of erroneous tokens error / warn | 36 (6.4%) / 22 (3.9%) | 36 (6.4%) / 21 (3.7%) |
| bronze, violations | 144 | 130 |

Real violations dropped by 6:
- 5: double counting from the `nummod-before-noun` duplicate. Those tokens are still covered, by the same rule from `nominal`.
- 1: *the rest of us pay*: notional agreement, and the seed itself names this example as an exception. The real error there is the head *pay*, and the agreement rule hit it by accident. Hence warn coverage −1 token.
