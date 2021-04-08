use crate::metropolis;
use crate::salesman::Itinerary;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointStyle};
use plotlib::view::ContinuousView;
use rand::prelude::*;

// Schedule describes an annealing schedule.
pub struct Schedule {
    temperature: f64,
    itinerary: Itinerary,
    temperature_step: f64,
    min_temperature: f64,
}

impl Schedule {
    // Generate a new annealing schedule.
    pub fn new(
        start_temperature: f64,
        temperature_step: f64,
        min_temperature: f64,
        num_cities: usize,
    ) -> Schedule {
        Schedule {
            temperature: start_temperature,
            itinerary: Itinerary::new(num_cities),
            temperature_step: temperature_step,
            min_temperature: min_temperature,
        }
    }

    pub fn run(&mut self, sample_size: usize) -> Itinerary {
        let mut results = Vec::new();
        loop {
            let (values, _) = metropolis::sample(self, sample_size);
            self.itinerary = values[values.len() - 1].clone();
            results.push((self.temperature, self.itinerary.1 as f64));

            self.temperature -= self.temperature_step;
            if self.temperature < self.min_temperature {
                break;
            }
            println!("Current temperature: {}", self.temperature);
        }

        let s1: Plot = Plot::new(results).point_style(PointStyle::new());
        let v = ContinuousView::new()
            .add(s1)
            .x_label("Temperature")
            .y_label("Manhattan travel distance");
        Page::single(&v).save("scatter.png").unwrap();
        self.itinerary.clone()
    }
}

impl metropolis::Simulatable<Itinerary> for Schedule {
    fn pdf(&self, randv: &Itinerary) -> f64 {
        return std::f64::consts::E.powf(-1.0 * randv.1 as f64 / self.temperature);
    }

    fn generate(&self, current: Option<&Itinerary>) -> Itinerary {
        match current {
            None => self.itinerary.clone(),
            Some(cur) => cur.generate_new(),
        }
    }

    fn accept(&self, current: Option<&Itinerary>, proposed: &Itinerary) -> bool {
        if let Some(cur) = current {
            let (new_cost, old_cost) = (proposed.1, cur.1);
            if new_cost < old_cost {
                return true;
            }

            let mut rng = thread_rng();
            let acceptance_cutoff: f64 = rng.gen_range(0.0..1.0);
            let prob =
                std::f64::consts::E.powf(-1.0 * (new_cost - old_cost) as f64 / self.temperature);
            if acceptance_cutoff < prob {
                return true;
            }

            return false;
        } else {
            return true;
        }
    }
}
