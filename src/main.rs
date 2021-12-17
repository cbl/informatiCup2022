use rstrain::parser::parse;
use rstrain::tabu::TabuSearch;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let verbose = true;

    if args.len() < 2 {
        println!("Missing input!");
    } else {
        let model = parse(&args[1]);

        let mut tabu = TabuSearch::new();
        let solution = tabu.search(&model);
        // solution.plot(&"plots/costs.png".to_owned());

        println!("\n{}", solution.to_string(&model, verbose));
        println!(
            "Arrived Passengers: {}/{}",
            solution.0[solution.0.len() - 1].arrived_passengers().len(),
            model.passengers.len()
        );
        // println!("delays {}", delays(&tt));
    }
}
