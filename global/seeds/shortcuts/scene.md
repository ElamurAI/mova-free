# Scene: shapes, colours, sizes, proximity

**Gist.** A thing in a picture is described by size, colour and shape ("small blue circle"). An incomplete description
("the blue square", "the medium thing") points to an already mentioned thing compatible with it. Proximity
(near, far) and contact (touching) are symmetric: "A near B" is also "B near A". Unlike directions,
they are not transitive.

**Conditions and exceptions.** "Thing", "object", "one", "shape" — a thing of any shape. "Top", "over" —
the same as "above"; "bottom", "under", "beneath" — the same as "below".

**Examples.** "Near and below the large yellow square is a large black square" → the black square is
near the yellow one and below it.

**Sources.** Mirzaee et al. 2021, SpartQA (NAACL); Suhr et al. 2019, NLVR.

```category
scene_color: red blue green yellow black white brown gray grey orange purple pink
scene_size: small medium large big little tiny huge
scene_shape: square circle triangle oval rectangle star diamond pentagon hexagon
scene_any: thing things object objects one ones shape shapes item items
```

```relation
near: symmetric=yes
far: symmetric=yes
touching: symmetric=yes
```
