use crate::model::Model;
use crate::move_::Move;
use crate::solution::Solution;
use crate::state::State;
use fxhash::hash64;
use std::process;

const MAX_TABU: usize = 10000;
const REPEATS: i32 = 5;
const PATHS_PER_T: i32 = 15;

pub struct TabuSearch {
    costs: Vec<f64>,
}

impl TabuSearch {
    pub fn new() -> TabuSearch {
        TabuSearch { costs: vec![] }
    }

    pub fn search(&mut self, model: &Model) -> Solution {
        // current time
        let t_max = model.latest_arrival() + 1;

        // the current solution
        let mut solution: Solution = Solution::new();
        let mut best_solution: Solution = Solution::new();

        // list of possible states
        let mut moves: Vec<Move> = vec![];

        // the states that are tabu
        let mut tabu: Vec<u64> = vec![];

        // the current state
        let mut best_neighbour: State = model.initial_state();
        let mut null_state: State = best_neighbour.clone();
        let mut check_state: State;

        for _ in 0..REPEATS {
            for i in 0..t_max {
                for _ in 0..PATHS_PER_T {
                    if i == 0 {
                        null_state = model.initial_state();
                    } else {
                        null_state = best_solution.0[i - 1].clone().next_null(model);
                    }

                    best_neighbour = null_state;
                    solution.0.drain(i..);

                    for t in i..t_max {
                        // choose move for each train
                        for t_id in 0..model.trains.len() {
                            moves = best_neighbour.neighbourhood(t_id, model);

                            if moves.is_empty() {
                                continue;
                            }

                            // set null_state for each train
                            null_state = best_neighbour.clone();

                            // find neighbour with best cost that is not tabu
                            for m in moves.iter() {
                                check_state = best_neighbour.clone();
                                check_state.make_move(*m, model);

                                if tabu.contains(&hash64(&check_state)) {
                                    continue;
                                }

                                println!("{} ({})", m.to_string(model), check_state.fitness(model));

                                if check_state.fitness(model) < best_neighbour.fitness(model) {
                                    best_neighbour = check_state;
                                }
                            }

                            // add to tabu list
                            let hash = hash64(&best_neighbour);
                            if best_neighbour != null_state && !tabu.contains(&hash) {
                                tabu.push(hash);

                                if tabu.len() > MAX_TABU {
                                    tabu.pop();
                                }
                            }
                        }

                        solution.0.push(best_neighbour.clone());

                        null_state = best_neighbour.next_null(model);
                        best_neighbour = null_state;
                    }

                    println!("{}", solution.to_string(model, true));

                    process::exit(1);

                    if solution.fitness(model) <= best_solution.fitness(model) {
                        best_solution = solution.clone();
                    } else {
                        solution = best_solution.clone();
                    }
                }
            }
        }

        best_solution
    }
}
