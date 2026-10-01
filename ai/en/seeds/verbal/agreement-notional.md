# Notional agreement: collectives, quantity nouns, measures

**Gist.** Sometimes the verb agrees not with the form but with the meaning of the subject. A collective noun, when thought of as a plurality of persons, takes the plural: *The committee **were** divided* (more often British). Quantity nouns with *of* (*a number of, a lot of, a couple of, a handful of, a majority of*) make the verb agree with what follows *of*: *A number of groups **have** become active*. Conversely, names with a plural form, sums and measures take the singular: *Ten dollars **is** enough*, *The United States **is***.

**Conditions and exceptions.** *The number of X* is singular (*The number of cases **is** rising*). The same form can be used with either number depending on the speaker's view — so these cases are an exception for agreement rules, not an error. Collective nouns as such — `nominal/collective-nouns.md`.

**Examples.** *A large number of them **are** Bangladeshis.* — *There **are** a wealth of references* (colloquial). — *Three hours **isn't** far.*

**In UD.** The head of the subject is a quantity noun (*number, lot, couple*) with `Number=Sing`; the `Number` feature is set by word form, not meaning (UD changes: "the wordform-based feature is to be preferred"). So the `agreement-*.md` rules give false warnings here; a desired extension of the rule language is a "lemma not in list" condition (`lemma!=`), to exclude *number, lot, couple, majority, handful, bunch, variety, wealth, total, rest, half, percent*.

**Sources.** Brown 1851, Rule XV (a collective noun conveying "the idea of plurality" takes a plural verb); Rule XIV, Note II (six months' interest was); Jespersen, MEG II (Syntax I), chapters on number (text-2); UD docs/changes.md` "Morphosyntactic Features" (notional agreement).
