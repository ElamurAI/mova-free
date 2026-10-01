# Math world scripts

2,680 text word problems with a script that tells, step by step, what the
problem's world is: who has what, what changes, what is asked. One JSON object
per line: `id`, `set` (`svamp-train` or `gsm8k-train`), `question`, `gold`
(answer), `script`.

Example script:

```
@1 set philip.orange = 87                          k=given
@1 set philip.banana = 290                         k=given
@2 set philip.banana_group = 2                     k=given
@3 set philip.banana_per_group = philip.banana / philip.banana_group   k=unit
@3 ask philip.banana_per_group
```

`@N` is the sentence the line comes from; `k=` is the kind of step.

**Sources:** problems from the training splits of
[SVAMP](https://github.com/arkilpatel/SVAMP) (Patel et al. 2021, MIT) and
[GSM8K](https://github.com/openai/grade-school-math) (Cobbe et al. 2021, MIT).

**How made:** a large language model wrote a script for each problem; Mova ran
it with its exact arithmetic core and kept only scripts whose result equals the
gold answer.

**License:** problems MIT (as the original sets); scripts ours, Apache-2.0 OR MIT.

**How Mova uses it:** training data for Mova's own world writer in `ai/math`
(`qworld`): it learns to label each number with its role in the world, so that
the judge of the step-by-step solver can check a candidate against the world.
