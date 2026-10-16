# Seeds, explained

A **seed** is a short Markdown note from which a rule or a piece of knowledge grows. Each page here covers one folder of seeds, with one section per seed: what it says, when it applies, examples, sources, and the rules or machine-readable blocks it defines.

| page | folder | what |
|---|---|---|
| [global-seeds.md](global-seeds.md) | `global/seeds` | the global level: values, concepts and categories, verb classes, links, relations, measures, senses of words written alike; compiled into Rust tables by `build.rs` |
| [en-seeds-morph.md](en-seeds-morph.md) | `en/seeds/morph` | English morphology: tags, lemmas, word formation |
| [en-seeds-lexicon.md](en-seeds-lexicon.md) | `en/seeds/lexicon` | word classes the parser needs to know (copula-like verbs, indirect-object verbs, …) |
| [en-seeds-nominal.md](en-seeds-nominal.md) | `en/seeds/nominal` | noun phrases: determiners, pronouns, numbers, names |
| [en-seeds-verbal.md](en-seeds-verbal.md) | `en/seeds/verbal` | verbs and clauses: auxiliaries, objects, complements |
| [en-seeds-errors.md](en-seeds-errors.md) | `en/seeds/errors` | typical annotation errors and the rules that catch them |
| [dialect-mova.md](dialect-mova.md) | `dialects/mova` | Mova's dialect of Universal Dependencies and the converters to and from it |

Grammar seeds in `en/seeds` carry ```` ```rule ```` blocks that the rule engine `en::expert` loads at run time (`en expert-check`); a test gate checks that every seed parses. Global seeds are compiled at build time.

The pages were written for release 0.3 (2026-10-03). The grammar pages cover every seed in their folders as of 2026-10-15. On the global page, `measures`, `selection` and `homographs` were added on 2026-10-15.

Related: [../absurdity.md](../absurdity.md) (the absurdity matrix), [../induce.md](../induce.md) (rules the model induces itself), [../selfplay.md](../selfplay.md), [../derive.md](../derive.md), [../pragmatics-expressions.md](../pragmatics-expressions.md).
