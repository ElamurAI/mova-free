# Stative verbs hardly take the progressive

**Gist.** The progressive (*be + -ing*) describes an action that goes on and develops through will or activity. So verbs describing a state, knowledge, possession, feeling-attitude are hardly used in it: *I know*, not *\*I am knowing*; *It belongs to me*, not *\*is belonging*. Poutsma gives groups: perception (*see, hear, smell, taste* in the literal sense — unlike *look, listen*), attitude (*like, love, hate, prefer, esteem*), desire (*desire, wish*), thinking (*believe, know, suppose, understand*), possession and composition (*belong, contain, consist, possess, resemble*), appearance (*seem, appear*).

**Conditions and exceptions.** With a shade of activity the progressive is possible: *I'm seeing a doctor* (visiting), *You're being silly* (behaving), *I'm loving it* (modern colloquial), *I've been meaning to call*. So only the most stable stative verbs are taken into the rule, and it only warns: either the text is unusual, or the *-ing* is mis-annotated (for example an adjective or gerund taken for a progressive).

**Examples.** *I know the answer.* — *\*I am knowing the answer.* — *She is being very kind* (copula with a shade of behaviour).

**In UD.** A `VBG` head with `aux` *be* has no lemma from the list.

**Sources.** Poutsma 1921, *The Expanded Form*, §38 («contrary to the force of the Expanded Form»; vol. 1, p. 96), §§39–42 (perception, feeling, thinking; pp. 97–99); Jespersen, MEG IV, ch. XIV «Expanded Tenses Concluded» (text-4).

```rule
rule: en.lexicon.stative-progressive
what: stative verbs in the progressive — unusual (or a wrong -ing annotation)
match: v[upos=VERB, xpos=VBG, lemma=know|believe|understand|suppose|belong|contain|consist|own|possess|resemble|seem|prefer|matter|exist|deserve|recognize]; a[lemma=be, rel=aux, head=v]
require: not v[xpos=VBG]
severity: warn
source: Poutsma 1921 Expanded Form §§38–42
```
