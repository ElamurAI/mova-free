# Things, measures and parts (from the snake's "teach me" reports, 2026-09-30)

**Gist.** General human categories of nouns that the snake met in problems and did not know: time units, measures, part and whole, countable things. The category says how to treat the number: the same category is added and subtracted; part and whole are multiplied or divided by a rate ("pages per chapter"); time is converted by constants (week = 7 days).

**Conditions and exceptions.** "Cup" is both a vessel and a measure; in recipes it is a measure.

**Examples.** "6 cups of flour" — a measure; "4 chapters, 20 pages each" — part and whole with a rate; "3 weeks" → 21 days.

**Sources.** General knowledge; the snake's reports on SVAMP (`mathsolve steps --explain`).

```category
time_unit: second minute hour day week month year decade century
measure_unit: cup inch foot feet yard mile meter centimeter kilometer gram kilogram pound ounce liter gallon g kg km cm mm
whole_part: book chapter page box sack bag basket pack row shelf team class group
countable: cookie peach apple orange banana marble crayon goldfish shirt cake piece candy egg flower pencil sticker card toy bottle ball book cup
```

```link
time_unit -> convert : time units — conversion by constants (hour = 60 min, week = 7 days)
whole_part -> rate : part of a whole — a rate ("pages per chapter", "candies per pack")
measure_unit -> total : the same measure — add and subtract
countable -> total : countable things of one category — add and subtract
```

```concept
convert: convert
```

```link
convert -> multiply : conversion to a smaller unit — multiplication by a constant
```

Verbs from the reports are concepts, not classes: extending the verb classes changes the snake's features and lowered SVAMP accuracy from 57.3% to 54.0% (regression gate). Concepts only explain steps.

```concept
put_in: put place pour add_in
learn_gain: learn memorize
join_group: join enroll
activity: sit visit jump
fit_into: fit
```

```link
put_in -> increase : put into — there is more where it was put
learn_gain -> increase : learned — knows more
join_group -> increase : joined — there are more in the group
```
