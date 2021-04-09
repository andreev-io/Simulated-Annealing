use rand::prelude::*;
use rand_distr::StandardNormal;

// Defines a trait for distributions that can be simulated with the Metropolis
// algorithm.
pub trait Simulatable<T: Clone> {
    // This function needs to return a value proportional to the probability of
    // obtaining a given sample.
    fn pdf(&self, randv: &T) -> f64;
    // This function generates a new sample based on the current sample.
    fn generate(&self, current: Option<&T>) -> T;
    // This function yields whether to accept a proposed sample.
    fn accept(&self, current: Option<&T>, proposed: &T) -> bool;
}

// This function samples from a simulatable distribution n times. It returns a
// vector of sampled values and the associated probabilities of obtaining them.
pub fn sample<T: Clone>(s: &impl Simulatable<T>, n: usize) -> Vec<(T, f64)> {
    let mut results = Vec::new();

    let seed_value = s.generate(None);
    let seed_probability = s.pdf(&seed_value);
    results.push((seed_value, seed_probability));
    for i in 1..n {
        let proposed = s.generate(Some(&results[i - 1].0));
        if s.accept(Some(&results[i - 1].0), &proposed) {
            let p = s.pdf(&proposed);
            results.push((proposed, p));
        } else {
            results.push(results[i - 1].clone());
        }
    }

    results
}

pub struct Exponential {}

impl Simulatable<f64> for Exponential {
    fn pdf(&self, randv: &f64) -> f64 {
        if *randv < 0.0 {
            return 0.0;
        } else {
            return std::f64::consts::E.powf(-1.0 * randv);
        }
    }

    fn generate(&self, current: Option<&f64>) -> f64 {
        let drift: f64 = thread_rng().sample(StandardNormal);
        if let Some(cur) = current {
            return cur + drift;
        } else {
            return drift;
        }
    }

    fn accept(&self, current: Option<&f64>, proposed: &f64) -> bool {
        let mut rng = thread_rng();
        if let Some(cur) = current {
            let proposed_prob = self.pdf(proposed);
            let acceptance_ratio = proposed_prob / self.pdf(cur);
            let acceptance_cutoff: f64 = rng.gen_range(0.0..1.0);
            return acceptance_cutoff < acceptance_ratio;
        } else {
            return true;
        }
    }
}
