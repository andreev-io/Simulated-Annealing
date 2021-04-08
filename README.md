# Simulated Annealing

### Implementation of the eponymous 1983 Kirkpatrick et al. paper.


By drawing an analogy between combinatorial optimization problems and the
statistical mechanics model of a cooling metal, we can build up heuristics
useful to approach the global optimum of the problem in question.

This code sets up a traveling salesman problem and searches for the optimal
solution using the simulated annealing approach. **The goal of this project is for
me to better prepare for my *University of Illinois Artificial Intelligence and
Data Analytics Group* presentation.**

Under the hood, the implementation models the Boltzmann distribution using the
Metropolis algorithm.

## Visualizations

**Travel distance during computation for 500 cities:**
![Manhattan Distance](https://github.com/andreev-io/Simulated-Annealing/blob/master/plots/scatterT0.09900000000010267N500S1000.png?raw=true)


**Initial random path through 500 cities:**
![Initial Path](https://github.com/andreev-io/Simulated-Annealing/blob/master/plots/pathT10N500S1000.png?raw=true)


**Optimized travel path through 500 cities:**
![Optimized Path](https://github.com/andreev-io/Simulated-Annealing/blob/master/plots/pathT0.09900000000010267N500S1000.png?raw=true)

If interested, you can easily tweak the problem setup in `main.rs`. You can edit
the number of cities, start and end temperature, temperature step, and sample
size at each temperature.
