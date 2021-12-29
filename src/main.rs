#![warn(unused_extern_crates)]

use clap::{App, Arg, ArgMatches};
use rstrain::debug::debug;
use rstrain::parser::parse;
use rstrain::plotter::Plotter;
use rstrain::tabu::TabuSearch;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

fn get_std_in() -> String {
    let stdin = io::stdin();
    let mut input = String::new();

    for line in stdin.lock().lines() {
        input += &line.unwrap();
        input += &"\n".to_string();
    }

    input
}

fn parse_arg<T: FromStr>(
    matches: &ArgMatches,
    name: &'static str,
    default: &'static str,
) -> Result<T, T::Err> {
    matches
        .value_of(name)
        .unwrap_or(default)
        .to_string()
        .parse()
}

fn main() {
    let matches = App::new("rstrain")
        .version("0.0.1")
        .author("Lennart Carstens-Behrens")
        .about("Train routing optimization program")
        .arg(
            Arg::with_name("TABU")
                .short("s")
                .long("tabu-size")
                .takes_value(true)
                .help(
                    "Size of tabu list, increase for large models (default 8000000)",
                ),
        )
        .arg(
            Arg::with_name("TIME")
                .short("t")
                .long("time")
                .takes_value(true)
                .help("Max search duration in milliseconds (default 600000)"),
        )
        .arg(
            Arg::with_name("DEBUG")
                .short("d")
                .long("debug")
                .takes_value(false)
                .help("Prints detailed information about the result"),
        )
        .arg(
            Arg::with_name("TMAX")
                .short("m")
                .long("t-max")
                .takes_value(true)
                .help("The latest time, increase when a solution with a total delay of 0 cannot be found, default value is the latest arrival time of all passengers"),
        )
        .arg(
            Arg::with_name("PLOT")
                .short("p")
                .long("plot")
                .takes_value(false)
                .help("Plots the fitness progress, plots are located in ./plots"),
        )
        .get_matches();

    // parse arguments
    let max_millis = parse_arg(&matches, "TIME", "600000").expect("invalid input for [TIME]");
    let tabu_size = parse_arg(&matches, "TABU", "8000000").expect("invalid input for [TABU]");
    let t_max = parse_arg(&matches, "TMAX", "0").expect("invalid input for [TMAX]");
    let track_fitness = matches.is_present("PLOT");

    // build model
    let mut model = parse(&get_std_in());
    model.t_max = std::cmp::max(model.t_max, t_max);

    // construct tabu search
    let mut tabu = TabuSearch::new(max_millis, tabu_size, track_fitness);

    // run tabu search
    let (solution, duration) = tabu.search(&model);

    // print result
    if matches.is_present("DEBUG") {
        debug(model, solution, duration, tabu.checked_moves);
    } else {
        println!("{}", solution.to_string(&model, false));
    }

    // plot fitness
    if matches.is_present("PLOT") {
        #[allow(unused_must_use)]
        {
            (Plotter { path: "plots" }).plot_fitness(&tabu);
        }
    }
}
