use clap::{App, Arg};
use rstrain::debug::debug;
use rstrain::parser::parse;
use rstrain::plotter::Plotter;
use rstrain::tabu::TabuSearch;
use std::cmp;

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
                    "Size of tabu list, increase for large models, required memory is <TABU> * 32bit (default 80000000)",
                ),
        )
        .arg(
            Arg::with_name("TIME")
                .short("t")
                .long("time")
                .takes_value(true)
                .help("Max search duration in milliseconds (default 30000)"),
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
                .help("Plots the fitness progress"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Input model")
                .required(true)
                .index(1),
        )
        .get_matches();

    let max_millis: u128 = match matches
        .value_of("TIME")
        .unwrap_or("600000")
        .to_string()
        .parse::<u128>()
    {
        Ok(time) => time,
        Err(_) => {
            eprintln!("error: invalid input for [time]");
            return;
        }
    };

    let tabu_size = match matches
        .value_of("TABU")
        .unwrap_or("80000000")
        .to_string()
        .parse::<usize>()
    {
        Ok(tabu) => tabu,
        Err(_) => {
            eprintln!("error: invalid input for [tabu-size]");
            return;
        }
    };

    let mut model = parse(&matches.value_of("INPUT").unwrap().to_string());

    model.t_max = match matches
        .value_of("TMAX")
        .unwrap_or("0")
        .to_string()
        .parse::<usize>()
    {
        Ok(t_max) => cmp::max(t_max, model.t_max),
        Err(_) => {
            eprintln!("error: invalid input for [t-max]");
            return;
        }
    };

    let mut tabu = TabuSearch::new(max_millis, tabu_size, matches.is_present("PLOT"));

    let (solution, duration) = tabu.search(&model);

    if matches.is_present("DEBUG") {
        debug(model, solution, duration, tabu.checked_moves);
    } else {
        println!("{}", solution.to_string(&model, false));
    }

    // plot the fitness
    if matches.is_present("PLOT") {
        #[allow(unused_must_use)]
        {
            (Plotter { path: "plots" }).plot_fitness(&tabu);
        }
    }
}
