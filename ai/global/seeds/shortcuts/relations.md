# Relations of space, size and time

**Gist.** Relations between things have properties that need not be learned from every story: the inverse
("A is north of B" means "B is south of A"), direction on the plane (north is up, left is to the left),
transitivity ("bigger than" through a chain), the order of parts of the day. A story gives only facts — conclusions
come from here (level 3 does not invent knowledge).

**Conditions and exceptions.** "Fits inside" is the same as "smaller than"; "above/below" for shapes is the same
vertical as north/south on a map, but with a different name. Time order is within one narrative:
yesterday is earlier than this morning. "Today" and "tomorrow" follow yesterday as whole days
(`day`). Days of the week have their own order (`week`), separate from the parts of
a day: "on monday" and "on tuesday" stamp two different times of a story (SVAMP: "played tag with 5 kids on monday").

**Examples.** "The kitchen is north of the garden" → from the garden to the kitchen is north (n).
"The box fits inside the chest; the chest is smaller than the suitcase" → the box is smaller than the suitcase.

**Sources.** Weston et al. 2015, bAbI (arXiv:1502.05698) — tasks 4, 14, 17, 18, 19; Allen's interval
algebra (1983) for time.

```relation
north_of: inverse=south_of vec=0,1 step=n
south_of: inverse=north_of vec=0,-1 step=s
east_of: inverse=west_of vec=1,0 step=e
west_of: inverse=east_of vec=-1,0 step=w
above: inverse=below vec=0,1
below: inverse=above vec=0,-1
left_of: inverse=right_of vec=-1,0
right_of: inverse=left_of vec=1,0
bigger_than: inverse=smaller_than transitive=yes
smaller_than: inverse=bigger_than transitive=yes
fit_inside: same=smaller_than
fit_in: same=smaller_than
yesterday: order=0
morning: order=1
afternoon: order=2
evening: order=3
today: day=1
tomorrow: day=2
monday: week=1
tuesday: week=2
wednesday: week=3
thursday: week=4
friday: week=5
saturday: week=6
sunday: week=7
```
