use crate::{cost, state, transition};
use rand::Rng;
use std::f64::consts::E;

const BOLTZMANN_CONSTANT: f64 = 1.380649e-23f64;
const COOLING_STEPS: i32 = 100000;

// Used for temperature decrement - Typically 0.8 <= a <= 0.99
const COOLING_FRACTION: f64 = 0.99;

// Initial system temperature
const INITIAL_TEMPERATURE: f64 = 1.0;

// Number of steps between temperature change - Typically 100 to 1000
const STEPS_PER_TEMP: i32 = 100;

pub struct Annealer {
    state: state::State,
    data: state::Data,
}

impl Annealer {
    pub fn anneal(&mut self) {
        let mut rng = rand::thread_rng();

        // the current system temperature
        let mut temperature: f64 = INITIAL_TEMPERATURE;

        // value of current state
        let mut current_value: f64 = 0.0;

        // value at start of loop
        let mut start_value: f64 = 0.0;

        // value after swap
        let mut delta: f64 = 0.0;

        // hold wap accept conditions
        let (mut merit, mut flip): (f64, f64) = (0.0, 0.0);

        // exponent for energy function
        //let mut exponent: f64 = 0.0;

        // Initial neighbor
        let mut neighbor;

        current_value = cost::cost(&self.state, &self.data);

        for i in 1..COOLING_STEPS {
            temperature *= COOLING_FRACTION;
            start_value = current_value;

            for j in 1..STEPS_PER_TEMP {
                neighbor = self.state.clone();
                transition::transition(&mut self.state, &self.data);

                flip = rng.gen_range(0.0..1.0);
                delta = cost::cost(&self.state, &self.data);
                // -(current_value - delta) / (BOLTZMANN_CONSTANT * temperature)
                merit = E.powf((-delta / current_value) / (BOLTZMANN_CONSTANT * temperature));

                if delta < 0.0 {
                    // Accept win
                    current_value = current_value + delta;
                } else if merit > flip {
                    // Accept loss
                    current_value = current_value + delta;
                } else {
                    // Reject
                    self.state = neighbor;
                }
            }
        }
    }
}
