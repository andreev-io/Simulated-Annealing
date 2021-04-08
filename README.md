# Simulated Annealing

### Implementation of the eponymous 1983 Kirkpatrick et al. paper.


By drawing an analogy between combinatorial optimization problem and the statistical mechanics model of a cooling metal, we can build up some useful heuristics that can help us approach the global optimum of the problem in question.

This code sets up a traveling salesman problem and searches for the optimal solution using the simulated annealing approach.

Travel distance during the computation for 500 cities:
![Manhattan Distance](https://github.com/andreev-io/Simulated-Annealing/plots/scatterT0.09900000000010267N500S1000.svg)

Initial random path through 500 cities:
![Initial Path](https://github.com/andreev-io/Simulated-Annealing/plots/pathT10N500S1000.svg)

Optimized travel path through 500 cities:
![Optimized Path](https://github.com/andreev-io/Simulated-Annealing/plots/pathT0.09900000000010267N500S1000.svg)
