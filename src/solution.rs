use crate::model::Model;
use crate::move_::Move;
use crate::state::State;
use std::fmt;

/// The soltion holds a list of states for all entities at any given point in
/// time.
#[derive(Clone)]
pub struct Solution(pub Vec<State>);

impl Solution {
    pub fn new() -> Solution {
        let states = vec![];

        Solution(states)
    }

    pub fn fitness(&mut self, model: &Model) -> f64 {
        let len = self.0.len();

        if len == 0 {
            return f64::MAX;
        }

        self.0[len - 1].fitness(model)
    }

    fn to_string_verbose(&self, model: &Model) -> String {
        let mut string: String = "".to_owned();

        self.0.iter().enumerate().for_each(|(t, state)| {
            //
            string.push_str(&format!("[Time:{}]\n", t));

            for m in &state.moves {
                string.push_str(&m.to_string(model));
                string.push_str(&"\n");
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
                        Move::TrainStart(_, s_id) => {
                            string
                                .push_str(&format!("{} Start {}\n", t, model.stations[*s_id].name));
                        }
                        Move::Depart(_, _, c_id) => {
                            if let Some(connection) = model.connections.get(&c_id) {
                                string.push_str(&format!("{} Depart {}\n", t, connection.name));
                            }
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
                string.push_str(&format!("[Passenger:{}]", passenger.name));

                self.0.iter().enumerate().for_each(|(t, state)| {
                    if let Some(m) = state.passenger_move(p_id) {
                        match m {
                            Move::Board(t_id, _, _) => {
                                string.push_str(&format!(
                                    "{} Board {}\n",
                                    t, model.trains[*t_id].name
                                ));
                            }
                            Move::Detrain(_, _, _) => {
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
