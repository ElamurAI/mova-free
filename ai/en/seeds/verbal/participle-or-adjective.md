# Participle or adjective

**Gist.** The *-ed/-en* and *-ing* forms stand between verb and adjective. The verbal nature shows when there is tense, an object, an agent, adverbials: *the letters written yesterday*, *was worn by Joseph*. The adjectival nature — when the word has degrees of comparison or intensifiers (*very tired, more interesting*), a negative prefix *un-* with no corresponding verb (*unexpected, uninteresting*), stands after *seem, become*. A separate case is *-ed* adjectives from nouns (*talented, skilled, gifted, kind-hearted*): these are not participles at all; rules for them are in `morph/wf-ed-adjectives.md`.

**Conditions and exceptions.** *Very* nearby is not yet proof: *very much appreciated* — *very* goes with *much*. The prefix *un-* also occurs in true verbs (*undo, unlock → unlocked*). In *The coat was badly worn* it is an adjective; *The coat was worn by Joseph* is a passive.

**Examples.** *I am very interested* (ADJ). — *He was interested by the offer* (VERB, passive). — *a talented singer* (ADJ from the noun talent).

**In UD.** Adjectival use — `ADJ` (`JJ`), with *be* — `cop`; verbal — `VERB` (`VBN`/`VBG`), with *be* — `aux:pass`/`aux`. The intensifier *very* attaches to `ADJ`, not to `VERB`.

**Sources.** Poutsma 1923, *Participles*, §§7–13 (verbal and adjectival nature; vol. 2, pp. 199–202), §17 (attributive position, p. 204), §§21–22 (un-, intensifiers with -ing, p. 210), §§33, 35 (un-, very with -ed, pp. 229–230), §42 (-ed adjectives from nouns: aged, gifted, talented, p. 236); Reed & Kellogg, Higher Lessons, Lesson 129 (worn: passive vs adjective complement); https://universaldependencies.org/en/feat/Voice.html.

```rule
rule: en.verbal.very-participle
what: the intensifier very on a participle signals an adjective (ADJ)
match: v[upos=VERB, xpos=VBN|VBG]; a[form=very, rel=advmod, head=v]
require: not v[upos=VERB]
severity: warn
source: Poutsma 1923 Participles §§22, 35
```
