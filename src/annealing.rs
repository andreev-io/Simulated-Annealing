use crate::metropolis;
use crate::salesman::Itinerary;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, PointStyle};
use plotlib::view::ContinuousView;
use rand::prelude::*;

// Schedule describes an annealing schedule.
pub struct Schedule {
    itinerary: Itinerary,
    temperature_step: f64,
    start_temperature: f64,
    current_temperature: f64,
    min_temperature: f64,
    num_cities: usize,
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
            start_temperature: start_temperature,
            current_temperature: start_temperature,
            itinerary: Itinerary::new(num_cities),
            temperature_step: temperature_step,
            min_temperature: min_temperature,
            num_cities: num_cities,
        }
    }

    pub fn run(&mut self, sample_size: usize) -> Itinerary {
        self.plot_path(sample_size);

        let mut results = Vec::new();
        while self.current_temperature > self.min_temperature {
            let values = metropolis::sample(self, sample_size);
            self.itinerary = values[values.len() - 1].0.clone();
            results.push((self.current_temperature, self.itinerary.1 as f64));
            self.current_temperature -= self.temperature_step;
            println!(
                "Current temperature: {}. Current average travel leg: {}.",
                self.current_temperature,
                self.itinerary.average_step_length()
            );
        }

        self.plot_scatter(results, sample_size);
        self.plot_path(sample_size);
        self.itinerary.clone()
    }

    fn plot_scatter(&self, results: Vec<(f64, f64)>, sample_size: usize) {
        let s1: Plot = Plot::new(results).point_style(PointStyle::new().colour("#35C788"));
        let v = ContinuousView::new()
            .add(s1)
            .x_label(format!(
                "Temperature. Start temp: {}, temp step: {}, sample size: {}, cities: {}",
                self.start_temperature, self.temperature_step, sample_size, self.num_cities
            ))
            .y_label("Manhattan travel distance");
        Page::single(&v)
            .save(format!(
                "plots/scatterT{}N{}S{}.svg",
                self.current_temperature, self.num_cities, sample_size
            ))
            .unwrap();
    }

    fn plot_path(&self, sample_size: usize) {
        let mut points: Vec<(f64, f64)> = self
            .itinerary
            .0
            .iter()
            .map(|city| (city.1 as f64, city.2 as f64))
            .collect();
        points.push((self.itinerary.0[0].1 as f64, self.itinerary.0[0].2 as f64));

        let s1: Plot = Plot::new(points.clone()).point_style(PointStyle::new().colour("#35C788"));
        let s2: Plot = Plot::new(points).line_style(LineStyle::new().colour("#35C788"));
        let v = ContinuousView::new()
            .add(s1)
            .add(s2)
            .x_range(0.0, (self.num_cities as f64).sqrt() + 2.0)
            .y_range(0.0, (self.num_cities as f64).sqrt() + 2.0)
            .x_label(format!(
                "X coordinate. Cities: {}, temperature: {}",
                self.num_cities, self.current_temperature
            ))
            .y_label("Y coordinate");
        Page::single(&v)
            .save(format!(
                "plots/pathT{}N{}S{}.svg",
                self.current_temperature, self.num_cities, sample_size
            ))
            .unwrap();
    }
}

impl metropolis::Simulatable<Itinerary> for Schedule {
    fn pdf(&self, randv: &Itinerary) -> f64 {
        return std::f64::consts::E.powf(-1.0 * randv.1 as f64 / self.current_temperature);
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
            let prob = std::f64::consts::E
                .powf(-1.0 * (new_cost - old_cost) as f64 / self.current_temperature);
            if acceptance_cutoff < prob {
                return true;
            }

            return false;
        } else {
            return true;
        }
    }
}
