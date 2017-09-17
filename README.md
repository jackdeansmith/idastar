# Iterative Deepening A\* Search in Rust

Recently, a professor of mine made a succinct description of the conditions
needed for a problem to be solvable with this method:

* Each state has defined successor states
* There exists some goal state
* Each transition has unit costs
* There exists some admissible heuristic for cost until goal

I thought it would be an interesting idea to leverage Rust's type system to make
a generic version of this algorithm.

As is, this repo is the minimal working product of that idea. My end goal is to
take what I've learned working on this to create a library for all kinds of
search strategies and the traits necessary to use them generically. 
