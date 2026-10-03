# Documentation

How Mova works, explained through its *seeds*: short notes from which rules
and knowledge grow. Each page covers one folder of seeds, with one section per
seed: what it says, when it applies, examples, sources, and the rules or
machine-readable blocks it defines.

| Page | Folder | What |
|---|---|---|
| [global-seeds.md](global-seeds.md) | `ai/global/seeds` | the global level: values, concepts and categories, verb classes, links, relations, learned knowledge; compiled into Rust tables by `build.rs` |
| [en-seeds-morph.md](en-seeds-morph.md) | `ai/en/seeds/morph` | English morphology: tags, lemmas, word formation |
| [en-seeds-lexicon.md](en-seeds-lexicon.md) | `ai/en/seeds/lexicon` | word classes the parser needs to know (copula-like verbs, indirect-object verbs, …) |
| [en-seeds-nominal.md](en-seeds-nominal.md) | `ai/en/seeds/nominal` | noun phrases: determiners, pronouns, numbers, names |
| [en-seeds-verbal.md](en-seeds-verbal.md) | `ai/en/seeds/verbal` | verbs and clauses: auxiliaries, objects, complements |
| [en-seeds-errors.md](en-seeds-errors.md) | `ai/en/seeds/errors` | typical annotation errors and the rules that catch them |
| [dialect-mova.md](dialect-mova.md) | `ai/dialects/mova` | Mova's dialect of Universal Dependencies and the converters to and from it |
| [absurdity.md](absurdity.md) | `ai/global/data`, `ai/world` | the absurdity matrix: plausibility of events, debug and working modes, repairs |
| [induce.md](induce.md) | `ai/world` | tree-repair rules the model induces itself; knowledge ablation; hidden rules coming back |
| [selfplay.md](selfplay.md) | `ai/world` | self-modification experiments, the experiment journal, tuning any component |
| [derive.md](derive.md) | `ai/world` | dependency trees by logical derivation |
| [pragmatics-expressions.md](pragmatics-expressions.md) | `ai/global/data`, `ai/prag`, `ai/world` | expressions with real meanings, register and variety labels, domain modules |

Grammar seeds in `ai/en/seeds` carry ```` ```rule ```` blocks that the rule
engine `en::expert` loads at run time (`en expert-check`); a test gate checks
that every seed parses. Global seeds are compiled at build time.
