use crate::model::Model;
use crate::move_::Move;
use crate::state::State;
use crate::train::{Location as TLocation, Speed};
use crate::types::{IdSet, TimeDiff};

use rust_decimal::Decimal;

/// The soltion holds a list of states at any given point in
/// time.
#[derive(Clone)]
pub struct Solution(pub Vec<State>);

impl Solution {
    pub fn new() -> Solution {
        let states = vec![];

        Solution(states)
    }

    /// Gets a list of the arrived passengers.
    pub fn arrived_passengers(&self) -> &IdSet {
        &self.0[self.0.len() - 1].p_arrived
    }

    /// Gets a list of delays for each passenger.
    pub fn delays(&self) -> Vec<i32> {
        self.0[self.0.len() - 1].p_delays.clone()
    }

    /// Determines whether the latest state has is legal.
    pub fn is_legal(&self) -> bool {
        let len = self.0.len();

        if len == 0 {
            false
        } else {
            self.0[len - 1].is_legal()
        }
    }

    /// Gets the total delays of the solution
    pub fn fitness(&self) -> TimeDiff {
        let len = self.0.len();

        if len == 0 {
            return TimeDiff::MAX;
        }

        self.0[len - 1]
            .p_delays
            .iter()
            .filter(|d| **d > 0)
            .sum::<TimeDiff>()
    }

    fn to_string_verbose(&self, model: &Model) -> String {
        let mut string: String = "".to_owned();

        self.0.iter().enumerate().for_each(|(t, state)| {
            string.push_str(&format!(
                "[Time:{}][Fitness:({})]\n",
                t,
                state.p_delays.iter().filter(|d| **d > 0).sum::<TimeDiff>()
            ));
            string.push_str(&format!("t_passengers: {:?}\n", state.t_passengers));

            for m in &state.moves {
                string.push_str(&m.to_string(model));
                string.push_str(&"\n");
            }

            for (t_id, location) in state.t_location.iter().enumerate() {
                if let TLocation::Connection(c_id, s_id, t_start) = location {
                    if t - *t_start == 0 {
                        continue;
                    }

                    string.push_str(&format!(
                        "{} on {} to {} at {:.2}%\n",
                        model.trains[t_id].name,
                        model.connections[*c_id].name,
                        model.stations[*s_id].name,
                        (Speed::from(t - *t_start) * model.trains[t_id].speed)
                            / model.connections[*c_id].distance
                            * Decimal::from(100)
                    ));
                }
            }

            string.push_str(&"\n");
        });

        string
    }

    pub fn to_string(&self, model: &Model, verbose: bool) -> String {
        if verbose {
            return self.to_string_verbose(model);
        }

        let mut string: String = "".to_owned();

        model.trains.iter().enumerate().for_each(|(t_id, train)| {
            string.push_str(&format!("[Train:{}]\n", train.name));

            self.0.iter().enumerate().for_each(|(t, state)| {
                if let Some(m) = state.train_move(t_id) {
                    match m {
                        Move::Start(t_start) => {
                            string.push_str(&format!(
                                "{} Start {}\n",
                                t, model.stations[t_start.s_id].name
                            ));
                        }
                        Move::Depart(depart) => {
                            string.push_str(&format!(
                                "{} Depart {}\n",
                                t, model.connections[depart.c_id].name
                            ));
                        }
                        _ => (),
                    }
                }
            });

            string.push_str(&"\n");
        });

        model
            .passengers
            .iter()
            .enumerate()
            .for_each(|(p_id, passenger)| {
                string.push_str(&format!("[Passenger:{}]\n", passenger.name));

                self.0.iter().enumerate().for_each(|(t, state)| {
                    if let Some(m) = state.passenger_move(p_id) {
                        match m {
                            Move::Board(board) => {
                                string.push_str(&format!(
                                    "{} Board {}\n",
                                    t, model.trains[board.t_id].name
                                ));
                            }
                            Move::Detrain(_) => {
                                string.push_str(&format!("{} Detrain\n", t));
                            }
                            _ => (),
                        }
                    }
                });

                string.push_str(&"\n");
            });

        string
    }
}
