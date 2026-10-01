# Questions: inversion and the fronted wh-word

**Gist.** Yes/no questions are built by inversion: the first auxiliary goes before the subject — *Are you going?*, *Has he left?*; if there is no auxiliary, *do* is added — *Did you see it?*. In wh-questions the question word comes first but keeps its role in the clause: *What did you see?* (*What* — object), *Who called?* (*Who* — subject, no inversion), *Where do you live?* (adverbial). In archaic and poetic style the verb itself is inverted: *Have you any thoughts?*, *What say ye?*.

**Conditions and exceptions.** If the question word is the subject or its modifier, the order is normal (*Which boy won?*). An indirect question (*I wonder where he lives*) has no inversion. Questions with copular *be*: the head becomes the interrogative predicative (*What is that?* — `cop(What, is)`, `nsubj(What, that)`).

**Examples.** *What did you see?* — `obj(see, What)`, `aux(see, did)`. — *Who is he?* — *How much notification would you give?* (fronted object phrase).

**In UD.** The dependency annotation is the same as in a declarative sentence, only the order changes: a fronted `obj` stands to the left of the verb, `aux` to the left of `nsubj`. Order rules (e.g. `iobj-before-obj`) give false warnings for questions.

**Sources.** Reed & Kellogg, Higher Lessons, Lesson 55 "Arrangement — Interrogative Sentences" (the subject stands "after the first word of it when it is compound"); Brown 1851, Part II, Ch. VI, "V. Form of Question"; Jespersen, MEG V, ch. XXV "Questions", §25.1x (two types of questions; text-1, p. 492); https://universaldependencies.org/en/dep/cop.html (*What is that?*).
