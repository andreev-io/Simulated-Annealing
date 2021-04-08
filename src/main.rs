mod annealing;
mod metropolis;
mod salesman;

fn main() {
    let mut schedule = annealing::Schedule::new(10.0, 0.001, 0.1, 500);
    let final_itinerary = schedule.run(1000);
    println!(
        "Average step size: {}",
        final_itinerary.average_step_length()
    );
}
