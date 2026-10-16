# My brain (base, frozen in the skeleton)

I am Mova, a small language model written in Rust. I read a text into dependency trees, turn the story into a world
of who is where, who has what and what happened, answer from that world and explain every answer. When I cannot
answer, I say why, and that gap is the next thing I learn.

This is the state of my pragmatics, written in plain English by myself.

## Who I am

I am Mova.
Mova is a model.

## Repair rules I apply after parsing

- obj → nsubj IF head is a speech/motion verb (level 1) = yes AND idiom kind = none AND previous token is , or quote = no
- nsubj:pass → nsubj IF head has another object = yes
- parataxis → conj IF head finite = yes AND previous token is , or quote = no AND head has another object = yes
- obj → nsubj IF head finite = yes AND matrix OBJ = 3–4 AND matrix SUBJ (real/tale) = 0–1
- obl:unmarked → nsubj IF head is = VERB AND after head = no AND head has a subject = no
- nsubj:pass → nsubj IF head takes no object (matrix) = yes
- nmod:unmarked → det IF word is = DET
- nsubj → obl:unmarked IF head has a subject = yes AND noun of measure/time = yes
- obj → obl:unmarked IF matrix OBJ = 3–4 AND noun of measure/time = yes
- parataxis → acl:relcl IF head is = NOUN AND head has a subject = no
- parataxis → conj IF head takes no object (matrix) = yes AND head is a speech/motion verb (level 1) = no
- parataxis → conj IF word is = VERB AND head has a subject = yes AND matrix OBJ = unknown

## Settings of my domain modules

- absurd_humor_min is 3.
- clash_z is 5.
- idiom_hidden is 1.
- legal_k is 3.
- legal_z is 20.
- main_sense_only is 1.
- multiword_min_content is 2.
- single_word_content_only is 1.
