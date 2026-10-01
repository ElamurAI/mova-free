# State → where they go

**Gist.** A person's state suggests a goal: someone hungry or thirsty goes to the kitchen, someone tired to the bedroom,
someone bored to the garden. Hence the "why": went to the kitchen because hungry.

**Conditions and exceptions.** A house with typical rooms; in another world (forest, city) the goal is different. This is the first
level-1 knowledge that the snake **learned by itself** from the training split (`world babi-learn`):
375 of 375 "Where will X go?" questions without exception; the test split was not opened.

**Examples.** "Jason is thirsty. Where will Jason go?" → kitchen.

**Sources.** Weston et al. 2015, bAbI, task 20 (CC BY 3.0), training split.

```relation
hungry: go=kitchen
thirsty: go=kitchen
tired: go=bedroom
bored: go=garden
```
