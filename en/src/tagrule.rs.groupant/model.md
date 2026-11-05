# en-tagrule

## descriptant

### en-tagrule

Modern English context tag rules given to the tagger before it tags: R1 so of purpose → IN.

## approximant

### Forward approximant

so_purpose(words, i): «so» at i > 0, the previous word not punctuation or a coordinator, the next word not «that»; a subject pronoun at i+1 puts the modal at i+2, a determiner/possessive at i+1 and an alphabetic word at i+2 put it at i+3; the modal is can or could and the word after it is not a negation. force(words)[i] = Some(IN) when R1 holds, else None.

Domain/state: None (pure functions over the sentence).

Executable surrogate: unsupported. The outputs are categorical (analyses, tags); no calibrated percentage metric. No automatic substitution and no claimed 1/2/5/10 percent profile.

## invariant

### Invariant and cost

force(words).len() == words.len(); only Tag::IN or None; pure function of lowercased words at i-1..i+4.

Cost: O(n) per sentence with constant work per word (lowercasing up to five neighbours).

Limits: The rule tags; the parser decides mark vs advmod; in-memory ud-eval and parse-eval do not apply forced tags; A bare noun subject and so + will are left to the model

## composition

### Composition

so_purpose(words, i): «so» at i > 0, the previous word not punctuation or a coordinator, the next word not «that»; a subject pronoun at i+1 puts the modal at i+2, a determiner/possessive at i+1 and an alphabetic word at i+2 put it at i+3; the modal is can or could and the word after it is not a negation. force(words)[i] = Some(IN) when R1 holds, else None.

Consumers: annotate::tags merges force with emode::force (EModE first) before PTagger::tag_forced; the parser and UD tables then see IN.

## models

## behavior_complement

Never fires on: «so» first in the sentence, after a comma or coordinator, before «that»; a bare noun subject (so wolves can); modals will, would, may, might, must; a negated can/could; «so» of degree (so tired).

## reverse_reconstruction

R(force = Some(IN) at i) is every sentence with the R1 context at i; force = None maps back to any sentence without it. The tag does not recover whether the treebank would write SCONJ: EWT 2.18 train has 18 SCONJ/mark and 4 ADV/advmod in this context.

## treebank_counts

EWT 2.18 train: 22 contexts, 18 SCONJ/mark; GUM train 10, 8; GUM dev 2, 2; ESLSpok train 2, 1. The expert rule en.verbal.so-purpose-mark (seeds/verbal/purpose-clause.md) flags 4 of 20 bindings on EWT train gold (warn).

## tests

### Evidence

Checks: tagrule::tests::so_of_purpose.

cargo test --release -p en --lib: all en library tests passed (exit 0), run twice with identical outcomes. Negative controls are inside the tests (malformed derive lines must fail to parse; misspellings, blocked rivals, short or rare bases, names and EModE endings must give no analysis; contexts with a comma, a coordinator, a result modal or a negation must not force IN). A compiled mutation control is recorded in the branch report. No corpus, training, held-out set, GPU, teacher or API was run by the tests.

## History

### 2026-11-04 · descriptant

2026-11-04 modeling epoch (en-morph): new source; tests written with the code, each with negative controls; measurements in en/README.md (Measurements 2026-11-04). A registered leaf is partial coverage, not a proof.

### 2026-11-04 · tests

Heading `### Evidence — 2026-11-04` stays in tests as `### Evidence`.
