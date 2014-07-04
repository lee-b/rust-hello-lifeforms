rust-hello-lifeforms
====================

Hello Lifeforms.  A genetic algorithm, written in Rust.

This is just a quick port / rewrite of something I wrote in Python and C++
previously.  The Rust version is VERY basic so far (compared to the Python
version, which evolved a readable string, or the C++ version which evolved
working programs, in a virtual CPU), but I'll probably make it do more
as time goes on.


Details
-------

This... wait for it... counts up to a big number! ;)  [currently, I mean]

The algorithm is generic though, and should have pluggable Individuals (things to evolve), Evaluators (goals), and breeding soon.  This will make it capable of doing lots of things, in lots of different ways.

What the algorithm does is:

* Generate a random population of Individuals as a starting Population.
* Evaluate that population for fitness (i.e., how close all individuals are to the goal)
* Decides whether fitness is sufficient.  If so, it quits.  Otherwise, it evolves the population, and tries the evaluation again.
* The evolution involves creating new Populations over time (new generations), by naturally selecting two pairs of individuals.
* Natural selection involves selecting four individuals, then choosing the fittest two (based on how fit an Evaluator says they are), and breeding them together.
* Breeding is customisable, depending on the problem, and the nature of the Individuals being evolved.  For example, breeding two numbers could involve simple or complex math.  Breeding two images could involve a bitmap operation or other graphical effect.

Right now, it's a little inefficient, because the entire population is evaluated for fitness, to check when the loop ends, and then I evaluate four again, for natural selection. Probably I should select which four I want to compete, then get do all the fitness evaluation, and do the selection of those four within the same loop. It's pretty damn fast so far anyway, though.

I just realised that my C++ version probably could've been optimised a lot, just by having a better fitness evaluation. Might try that virtual CPU stuff in Rust some time.



Building
--------

For a normal build, just run 'cargo build'

For an optimised build, run ./opt_build.sh


Usage
-----

Run target/lforms


