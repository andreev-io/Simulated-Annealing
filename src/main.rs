extern crate clap;
use clap::{App, Arg};

mod annealing;
mod metropolis;
mod salesman;

fn main() {
    let matches = App::new("Simulated Annealing for Traveling Salesman")
        .version("1.0")
        .author("Ilya Andreev <iandre3@illiois.edu>")
        .arg(
            Arg::with_name("start-temperature")
                .long("start-temp")
                .value_name("START_TEMPERATURE")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("min-temperature")
                .long("min-temp")
                .value_name("MIN_TEMPERATURE")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("temperature-step")
                .long("temp-step")
                .value_name("TEMPERATURE_STEP")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("sample-size")
                .long("sample-size")
                .value_name("SAMPLE_SIZE")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("num-cities")
                .long("num-cities")
                .value_name("NUM_CITIES")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let mut schedule = annealing::Schedule::new(
        matches
            .value_of("start-temperature")
            .unwrap()
            .parse()
            .unwrap(),
        matches
            .value_of("temperature-step")
            .unwrap()
            .parse()
            .unwrap(),
        matches
            .value_of("min-temperature")
            .unwrap()
            .parse()
            .unwrap(),
        matches.value_of("num-cities").unwrap().parse().unwrap(),
    );

    let final_itinerary = schedule.run(matches.value_of("sample-size").unwrap().parse().unwrap());
    println!(
        "Average step size: {}",
        final_itinerary.average_step_length()
    );
}
