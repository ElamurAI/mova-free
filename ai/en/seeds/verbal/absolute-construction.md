# Absolute construction and independent participles

**Gist.** An absolute construction is a noun or pronoun together with a participle, grammatically unconnected to the rest of the sentence: *His master being absent, the business was neglected*. It has its own "subject" and serves as an adverbial (reason, time). Independent participial or infinitive phrases — *generally speaking*, *to confess the truth* — also have no link to the subject of the sentence.

**Conditions and exceptions.** The construction is often introduced by *with/without*: *With the kids in school, I have free time*. It also occurs without a verb (*most of them women and children*). Brown required the nominative in it (*he being absent*, not *him*). A participle without its own subject that does not refer to the subject of the sentence (*Walking home, the rain started*) is "dangling": a stylistic flaw, but the annotation is the same (`advcl`).

**Examples.** *His master being absent, the business was neglected.* — *With some used as building material and others as urinals.* — *The attack killed 600 Iraqis, most of them women and children.*

**In UD.** The head of the construction (participle, adjective, noun) is `advcl` on the predicate of the sentence; the "subject" of the construction is `nsubj`/`nsubj:pass` of that head; *with* is `mark`. A passive participle in the construction has `nsubj:pass` without `aux:pass`.

**Sources.** Reed & Kellogg, Higher Lessons, Lesson 44 (absolute phrase, independent participle and infinitive; ); Brown 1851, Rule VIII "Nom. Absolute" and Rule XX "Participles"; https://universaldependencies.org/en/dep/advcl.html (*With the kids in school…*); https://universaldependencies.org/en/specific-syntax.html (Nonverbal predicates with no copular verb: absolute).
