# wikt-extract-wikitext

## descriptant

### wikt-extract-wikitext

#### Descriptant

Written with the code (tools/wikt-extract, first version). Wikitext of one definition, example or list line → plain text plus side records (labels, form-of lemma, nyms, examples, gaps, dropped). Template calls are found with brace nesting, split at top-level pipes (not inside nested templates or links) into positional and named parameters; each known name has a handler printing what the template prints (links → alt or term, w → display or article, taxlink → name, gloss/qualifier → bracketed text, form-of → «Kind lemma», place → «A kind in holonyms», ux → example recorded); category, sense-id and maintenance templates print nothing and are counted as dropped; any other name prints nothing and is counted as a gap. Links [[a|b]] → b, external [url text] → text, bold/italic quotes, comments, references and HTML tags removed, entities decoded, whitespace collapsed.

The forward model below is source-derived; the tests in model.md#tests exercise the actual functions on small synthetic pages, and the full English pass (docs/data.md, «Wiktionary») is the corpus-scale observation. Executable percentage approximants are not supported: the outputs are categorical text records with no useful error metric.

## approximant

### Forward relation

Wikitext of one definition, example or list line → plain text plus side records (labels, form-of lemma, nyms, examples, gaps, dropped). Template calls are found with brace nesting, split at top-level pipes (not inside nested templates or links) into positional and named parameters; each known name has a handler printing what the template prints (links → alt or term, w → display or article, taxlink → name, gloss/qualifier → bracketed text, form-of → «Kind lemma», place → «A kind in holonyms», ux → example recorded); category, sense-id and maintenance templates print nothing and are counted as dropped; any other name prints nothing and is counted as a gap. Links [[a|b]] → b, external [url text] → text, bold/italic quotes, comments, references and HTML tags removed, entities decoded, whitespace collapsed.

#### Domain

Well-formed UTF-8 wikitext/XML of the English Wiktionary dump export format (2026-09-01 run); bounded resources.

## models

# Typed source models

Partial source-derived relations at this manifest snapshot; one call under the stated domain; no universal semantic proof.

## behavior_complement

Never executes Lua modules or expands templates from the dump's Template namespace: unknown templates contribute no text (only a gap count), categories are not reproduced, inline modifiers other than the first term segment are dropped, nested templates inside dropped templates are not visited, unclosed {{ or [[ are left as literal text.

Domain: Well-formed UTF-8 wikitext/XML of the English Wiktionary dump export format (2026-09-01 run); bounded resources.

## reverse_reconstruction

Many wikitexts render to the same text (any template with no handler, any drop-listed template, bold vs plain, [[a|b]] vs b); from a rendered text and its side records one can recover the set of compatible lines only up to these erasures: labels and nyms are recoverable as values, not as template names or positions.

Domain: Well-formed UTF-8 wikitext/XML of the English Wiktionary dump export format (2026-09-01 run); bounded resources. Preimages are sets, not unique inverses: the rendering is not injective.

## gap_accounting

Every template call reached by render with no handler increments out.gaps[name] exactly once; drop-listed and R:/RQ:/quote-/cite-/rf* names increment out.dropped instead. A template nested in a parameter that a handler renders is visited; one inside an unrendered parameter is not.

## parameter_split

split_params returns n+1 slices for n top-level pipes; a pipe inside {{…}} or [[…]] does not split; parse_call treats k=v as named only when k has no [[, {{ or <.

## tests

### Native evidence

Package/bin: `wikt-extract`. Selected tests (`cargo test --release -p wikt-extract`), all passing:

- `wikitext::tests::links_bold_and_templates`
- `wikitext::tests::labels_examples_nyms_and_form_of`
- `wikitext::tests::unknown_templates_are_counted_as_gaps`
- `wikitext::tests::nested_parameters_and_place`

Negative control: unknown_templates_are_counted_as_gaps: a name with no handler is counted in gaps and a category template in dropped, not in gaps; removing the gap branch's counter turns it red.

Scope: actual functions on synthetic in-memory inputs; the corpus-scale pass is an observation, not a proof. Lua module semantics, full template coverage and the correctness of every printed sense are not established.

## invariant-and-cost

### Invariant and cost

#### Invariant

render is pure and deterministic (BTreeMap counts); output has no tab or newline; a template either prints text, is dropped (counted) or is a gap (counted), never silently removed.

#### Cost

O(n) passes over a line of n bytes per nesting level (close_braces rescans nested bodies; parameters rendered recursively): worst case O(n·d) for depth d; tidy is O(n) with a few replace passes. Memory O(n).

## History

### 2026-11-04 · descriptant

Heading `#### Epoch 2026-11-04` stays in descriptant as `#### Descriptant`.

### 2026-11-04 · tests

Heading `### Native evidence — 2026-11-04` stays in tests as `### Native evidence`.
