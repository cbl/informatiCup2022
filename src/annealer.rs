use crate::cost::cost;
use crate::solution::Solution;
use crate::timetable::Timetable;
use plotters::prelude::*;
use rand::Rng;
use std::f64::consts::E;

/// Problem specific Boltzman's constant May have to adjust if your global
/// value function changes the sizes of the numbers it produces. It is important
/// that jumps seem random at the begining of the run, and rare at the end of a
/// run, and this is a knob to tweak that.
const BOLTZMANN_CONSTANT: f64 = 0.25;

/// how many times do we cool -- make higher to improve quality, lower to speed
/// the program up. Move in tandem with the COOLING_FRACTION
const COOLING_STEPS: i32 = 100;

//
/// How much to cool each time -- make higher to improve quality, lower to speed
/// the program up. Typically 0.8 <= a <= 0.99
const COOLING_FRACTION: f64 = 0.95;

// Start temperature.
const INITIAL_TEMPERATURE: f64 = 1.0;

/// Number of steps between temperature change -- Lower makes it faster, higher
/// makes it potentially better. Typically 100 to 1000
const STEPS_PER_TEMP: i32 = 100;

pub struct Annealer {
    // The duration in ms
// duration: i32,
}

impl Annealer {
    pub fn anneal(&self, tt: &mut Timetable) {
        let mut rng = rand::thread_rng();

        let mut costs: Vec<f64> = vec![];

        // the current system temperature
        let mut temperature: f64 = INITIAL_TEMPERATURE;

        // value of current state
        let mut current_cost: f64 = 0.0;
        // let mut current_value: f64 = 0.1;

        // value at start of loop
        let mut start_cost: f64 = 0.0;
        let mut best_cost: f64 = f64::INFINITY;

        // value after swap
        let mut new_cost: f64 = 0.0;
        let mut delta: f64 = 0.0;

        // hold wap accept conditions
        let (mut merit, mut flip): (f64, f64) = (0.0, 0.0);

        // exponent for energy function
        let mut exponent: f64 = 0.0;

        // neighbor
        let mut neighbor: Solution;
        let mut best: Solution = tt.solution.clone();

        current_cost = cost(&tt);

        for _ in 1..COOLING_STEPS {
            temperature *= COOLING_FRACTION;
            start_cost = current_cost;

            for _ in 1..STEPS_PER_TEMP {
                neighbor = tt.solution.clone();

                tt.transition();

                flip = rng.gen_range(0.0..1.0);
                new_cost = cost(&tt);
                delta = new_cost - current_cost;
                // -(current_value - delta) / (BOLTZMANN_CONSTANT * temperature)
                exponent = (-delta) / (BOLTZMANN_CONSTANT * temperature);
                merit = E.powf(exponent);

                println!(
                    "current_cost: {}, new_cost: {}, delta {}, merit: {}, flip {}",
                    current_cost,
                    new_cost,
                    // current_value,
                    delta,
                    merit,
                    flip // exponent,
                         // (-delta / current_value),
                         // (BOLTZMANN_CONSTANT * temperature)
                );

                if best_cost > new_cost {
                    best_cost = new_cost;
                    best = tt.solution.clone();
                }

                if delta < 0.0 {
                    // Accept win
                    current_cost = new_cost;
                    // current_value += delta;
                } else if merit > flip {
                    // Accept loss
                    current_cost = new_cost;
                    // current_value += delta;
                } else {
                    // Reject
                    tt.solution = neighbor;
                }

                costs.push(current_cost);
            }
        }
        plot(&costs);
        tt.solution = best;
    }
}
const OUT_FILE_NAME: &'static str = "plots/costs.png";

fn plot(costs: &Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    root.fill(&WHITE)?;
    // let min = costs.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = costs.iter().fold(0.0, |a, &b| match a > b {
        true => a,
        false => b,
    });

    let mut chart = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .caption("Costs By Iteration", ("sans-serif", 40))
        .build_cartesian_2d(-10..(costs.len() as i32), -1.0..(max + 2.0))
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(costs.iter().enumerate().map(|(x, y)| {
            Circle::new((x as i32, *y), 1, Into::<ShapeStyle>::into(&BLACK).filled())
        }))
        .unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
