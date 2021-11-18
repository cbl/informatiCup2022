use crate::cost::cost;
use crate::solution::Solution;
use crate::timetable::Timetable;
use rand::Rng;
use std::f64::consts::E;

const BOLTZMANN_CONSTANT: f64 = 1.0;
const COOLING_STEPS: i32 = 1000;

// Used for temperature decrement - Typically 0.8 <= a <= 0.99
const COOLING_FRACTION: f64 = 0.99;

// Initial system temperature
const INITIAL_TEMPERATURE: f64 = 1.0;

// Number of steps between temperature change - Typically 100 to 1000
const STEPS_PER_TEMP: i32 = 200;

pub struct Annealer {
    // The duration in ms
// duration: i32,
}

impl Annealer {
    pub fn anneal(&self, tt: &mut Timetable) {
        let mut rng = rand::thread_rng();

        // the current system temperature
        let mut temperature: f64 = INITIAL_TEMPERATURE;

        // value of current state
        let mut current_cost: f64 = 0.0;
        let mut current_value: f64 = 0.0;

        // value at start of loop
        let mut start_cost: f64 = 0.0;

        // value after swap
        let mut new_cost: f64 = 0.0;
        let mut delta: f64 = 0.0;

        // hold wap accept conditions
        let (mut merit, mut flip): (f64, f64) = (0.0, 0.0);

        // exponent for energy function
        let mut exponent: f64 = 0.0;

        // neighbor
        let mut neighbor: Solution;

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
                exponent = (-delta / current_value) / (BOLTZMANN_CONSTANT * temperature);
                merit = E.powf(exponent);

                // println!(
                //     "current_cost: {}, new_cost: {}, current_valule {}, delta {}, merit: {}, flip {}, exponent {}, test {}, p {}",
                //     current_cost,
                //     new_cost,
                //     current_value,
                //     delta,
                //     merit,
                //     flip,
                //     exponent,
                //     (-delta / current_value),
                //     (BOLTZMANN_CONSTANT * temperature)
                // );

                if delta < 0.0 {
                    // Accept win
                    current_cost = new_cost;
                    current_value += delta;
                } else if merit > flip {
                    // Accept loss
                    current_cost = new_cost;
                    current_value += delta;
                } else {
                    // Reject
                    tt.solution = neighbor;
                }
            }
        }
    }
}
