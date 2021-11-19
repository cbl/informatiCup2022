use crate::cost::cost;
use crate::solution::Solution;
use crate::timetable::Timetable;
use plotters::prelude::*;
use rand::Rng;
use std::f64::consts::E;

/// Problem specific Boltzman's constant may have to adjust if your global
/// value function changes the sizes of the numbers it produces. It is important
/// that jumps seem random at the begining of the run, and rare at the end of a
/// run, and this is a knob to tweak that.
const BOLTZMANN_CONSTANT: f64 = 0.45;

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
    costs: Vec<f64>,
}

impl Annealer {
    pub fn new() -> Annealer {
        Annealer { costs: vec![] }
    }

    pub fn repeated(&mut self, tt: &mut Timetable, n_samples: i32) {
        let mut best_solution: Solution = tt.solution.clone();
        let mut best_cost: f64 = cost(&tt);
        let mut cost_now: f64;
        let mut c_tt = tt.clone();

        for _ in 0..n_samples {
            self.anneal(&mut c_tt);
            cost_now = cost(&c_tt);
            if cost_now < best_cost {
                best_cost = cost_now;
                best_solution = c_tt.solution.clone();
            }
        }

        tt.solution = best_solution;
    }

    pub fn anneal(&mut self, tt: &mut Timetable) {
        let mut rng = rand::thread_rng();

        // the current system temperature
        let mut temperature: f64 = INITIAL_TEMPERATURE;

        // value of current state
        let mut current_cost: f64 = 0.0;

        // value after swap
        let mut new_cost: f64 = 0.0;
        let mut delta: f64 = 0.0;

        // hold wap accept conditions
        let (mut merit, mut flip): (f64, f64) = (0.0, 0.0);

        // neighbor
        let mut neighbor: Solution;

        current_cost = cost(&tt);

        for _ in 1..COOLING_STEPS {
            temperature *= COOLING_FRACTION;

            for i in 1..STEPS_PER_TEMP {
                neighbor = tt.solution.clone();

                tt.transition();

                flip = rng.gen_range(0.0..1.0);
                new_cost = cost(&tt);
                delta = new_cost - current_cost;
                // -(current_value - delta) / (BOLTZMANN_CONSTANT * temperature)
                merit = E.powf((-delta) / (BOLTZMANN_CONSTANT * temperature));

                if i % 10 == 0 {
                    println!(
                        "current_cost: {}, new_cost: {}, delta {}, merit: {}, flip {}",
                        current_cost, new_cost, delta, merit, flip
                    );
                }

                if delta < 0.0 {
                    // Accept win
                    current_cost = new_cost;
                } else if merit > flip {
                    // Accept loss
                    current_cost = new_cost;
                } else {
                    // Reject
                    tt.solution = neighbor;
                }

                self.costs.push(current_cost);
            }
        }
    }

    pub fn plot(&self, file_name: &String) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(file_name, (1024, 800)).into_drawing_area();
        root.fill(&WHITE)?;

        root.fill(&WHITE)?;
        let min = self
            .costs
            .iter()
            .fold(f64::INFINITY, |a, &b| a.min(b))
            .min((-0.0));

        let max = self.costs.iter().fold(0.0, |a, &b| match a > b {
            true => a,
            false => b,
        });

        let mut chart = ChartBuilder::on(&root)
            .set_label_area_size(LabelAreaPosition::Left, 50)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .caption("Costs By Iteration", ("sans-serif", 40))
            .build_cartesian_2d(-10..(self.costs.len() as i32), (min - 2.0)..(max + 2.0))
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(self.costs.iter().enumerate().map(|(x, y)| {
                Circle::new((x as i32, *y), 1, Into::<ShapeStyle>::into(&BLACK).filled())
            }))
            .unwrap();

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", file_name);

        Ok(())
    }
}
