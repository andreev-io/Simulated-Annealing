use rand::{distributions::Uniform, prelude::*};

#[derive(Debug)]
// A city is its ID plus two coordinates.
pub struct City(usize, pub f64, pub f64);

impl Clone for City {
    fn clone(&self) -> Self {
        return City(self.0, self.1, self.2);
    }
}

#[derive(Debug)]
// Itinerary is simply a sequence of cities and the total cost of visiting them
// in this order.
pub struct Itinerary(pub Vec<City>, pub f64);

impl Itinerary {
    // Generate a new random itinerary given the number of cities to visit. The
    // side of the square in which the cities are positioned is N^1/2.
    pub fn new(num_cities: usize) -> Itinerary {
        let mut rng = thread_rng();
        let mut cities: Vec<City> = Vec::new();

        let uniform = Uniform::from(1.0..=(num_cities as f64).sqrt());
        for i in 0..num_cities {
            cities.push(City(i, uniform.sample(&mut rng), uniform.sample(&mut rng)));
        }

        let cost = Itinerary::cost(&cities);
        Itinerary(cities, cost)
    }

    // Generate a new itinerary by picking two indices at random and reversing
    // the subsequence bounded by these indices.
    pub fn generate_new(&self) -> Itinerary {
        let mut new_itinerary = self.clone();
        let (mut index_one, mut index_two) = Itinerary::generate_swap_indices(self.0.len());

        while index_one < index_two {
            let temp = new_itinerary.0[index_one].clone();
            new_itinerary.0[index_one] = new_itinerary.0[index_two].clone();
            new_itinerary.0[index_two] = temp;
            index_one += 1;
            index_two -= 1;
        }

        new_itinerary.1 = Itinerary::cost(&new_itinerary.0);

        new_itinerary
    }

    // Calculate the cost of an itinerary.
    fn cost(cities: &Vec<City>) -> f64 {
        let len = cities.len();
        let mut cost: f64 = Itinerary::manhattan(&cities[0], &cities[len - 1]);
        for i in 1..len {
            cost += Itinerary::manhattan(&cities[i], &cities[i - 1]);
        }

        cost
    }

    pub fn average_step_length(&self) -> f64 {
        self.1 as f64 / self.0.len() as f64
    }

    // Calculate the Manhattan distance between two cities. I have tried
    // optimizing this function with a cache, but empirically lookups in a Rust
    // HashMap take longer than a few extra FLOPS.
    fn manhattan(start: &City, finish: &City) -> f64 {
        let mut cost: f64 = if start.1 > finish.1 {
            start.1 - finish.1
        } else {
            finish.1 - start.1
        };

        cost += if start.2 > finish.2 {
            start.2 - finish.2
        } else {
            finish.2 - start.2
        };

        cost
    }

    // Randomly pick two non-equal numbers in the range 0..=len-1.
    fn generate_swap_indices(len: usize) -> (usize, usize) {
        loop {
            let mut rng = thread_rng();
            let index_one: usize = rng.gen_range(0..len);
            let index_two: usize = rng.gen_range(0..len);
            if index_one != index_two {
                if index_one < index_two {
                    return (index_one, index_two);
                } else {
                    return (index_two, index_one);
                }
            }
        }
    }
}

impl Clone for Itinerary {
    fn clone(&self) -> Self {
        Itinerary(self.0.clone(), self.1)
    }
}
