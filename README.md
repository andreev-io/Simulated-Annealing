# Optimization by Simulated Annealing

### Implementation of the eponymous 1983 Kirkpatrick et al. paper.


By drawing an analogy between combinatorial optimization problems and the
statistical mechanics model of a cooling metal, we can build up heuristics
useful to approach the global optimum of the problem in question.

This code sets up a traveling salesman problem and searches for the optimal
solution using the simulated annealing approach. **The goal of this project is for
me to better prepare for my *University of Illinois Artificial Intelligence and
Data Analytics Group* presentation**: [https://www.youtube.com/watch?v=Fus4UYd9o3o](https://www.youtube.com/watch?v=Fus4UYd9o3o).

Under the hood, the implementation models the Boltzmann distribution using the
Metropolis algorithm.

## Visualizations

**Travel distance during computation for 500 cities:**


![Manhattan Distance](https://github.com/andreev-io/Simulated-Annealing/blob/master/plots/scatterT0N500S1000.png?raw=true)


**Initial random path through 500 cities:**


![Initial Path](https://github.com/andreev-io/Simulated-Annealing/blob/master/plots/pathT10N500S1000.png?raw=true)


**Optimized travel path through 500 cities:**


![Optimized Path](https://github.com/andreev-io/Simulated-Annealing/blob/master/plots/pathT0.009900000009966804N500S1000.png?raw=true)

## Usage
To obtain the same results as above, run
```
cargo build --release
./target/release/simulated-annealing --min-temp=0.01 --num-cities=500 --sample-size=1000 --start-temp=10 --temp-step=0.0001
```

## Links
See https://paperswithcode.com/paper/optimization-by-simulated-annealing.
