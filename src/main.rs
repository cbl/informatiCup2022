use rstrain::annealer::Annealer;
use rstrain::cost::{cost, delays};
use rstrain::parser::parse;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing input!");
    } else {
        let mut tt = parse(&args[1]);

        let mut annealer = Annealer::new();
        annealer.repeated(&mut tt, 3);
        annealer.plot(&"plots/costs.png".to_owned());

        println!("\n{}", tt.to_string());
        println!("cost {}", cost(&tt));
        println!("delays {}", delays(&tt));
    }
}
