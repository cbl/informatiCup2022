use rstrain::parser::parse;
use rstrain::plotter::Plotter;
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
        let plotter = Plotter { path: "plots" };

        let solution = tabu.search(&model);

        println!("\n{}", solution.to_string(&model, verbose));
        println!("Fitness: {}", solution.fitness(&model));
        println!("Delays: {:?}", solution.delays());
        println!("Total Delays: {:?}", solution.delays().iter().sum::<i32>());
        println!(
            "Arrived Passengers: {}/{}",
            solution.0[solution.0.len() - 1].arrived_passengers().len(),
            model.passengers.len()
        );

        plotter.plot_fitness(&tabu);

        // println!("delays {}", delays(&tt));
    }
}
