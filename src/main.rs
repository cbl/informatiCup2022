use clap::{App, Arg};
use rstrain::debug::debug;
use rstrain::parser::parse;
use rstrain::tabu::TabuSearch;

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
                    "Size of tabu list, increase for large models, required memory is <TABU> * 64bit (default 8000000)",
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
                .help("Print detailed information about the result"),
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
        .unwrap_or("30000")
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
        .unwrap_or("8000000")
        .to_string()
        .parse::<usize>()
    {
        Ok(tabu) => tabu,
        Err(_) => {
            eprintln!("error: invalid input for [tabu-size]");
            return;
        }
    };

    let model = parse(&matches.value_of("INPUT").unwrap().to_string());
    let mut tabu = TabuSearch::new(max_millis, tabu_size);

    let (solution, duration) = tabu.search(&model);

    if matches.is_present("DEBUG") {
        debug(model, solution, duration);
    } else {
        println!("{}", solution.to_string(&model, false));
    }
}
