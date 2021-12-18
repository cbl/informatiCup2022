use crate::model::Model;
use crate::move_::Move;
use crate::solution::Solution;
use crate::state::State;
use fxhash::hash64;
use rand::Rng;

const MAX_TABU: usize = 10000;
const REPEATS: i32 = 100;
const NO_CHANGES_MAX: i32 = 10;

pub struct TabuSearch {
    tabu: Vec<u64>,
    pub fitness: Vec<f64>,
}

impl TabuSearch {
    pub fn new() -> TabuSearch {
        TabuSearch {
            tabu: vec![],
            fitness: vec![],
        }
    }

    fn find_neighbour(&mut self, state: &State, model: &Model) -> State {
        // list of possible states
        let mut moves: Vec<Move> = vec![];

        // the best neighbour
        let mut neighbour = state.clone();

        // null state
        let mut null: State = neighbour.clone();

        // the next neighbour that is being checked
        let mut next: State;

        for t_id in 0..model.trains.len() {
            moves = neighbour.get_moves(t_id, model);

            if moves.is_empty() {
                continue;
            }

            null = neighbour.clone();

            // find neighbour with best cost that is not tabu
            for m in moves.iter() {
                next = null.clone();
                next.make_move(*m, model);

                if self.tabu.contains(&hash64(&next)) {
                    continue;
                }

                if next.fitness(model) <= neighbour.fitness(model) {
                    self.fitness.push(next.fitness(model));
                    neighbour = next.clone();
                }
            }

            // add to tabu list
            self.add_tabu_state(&neighbour);
        }

        neighbour
    }

    fn add_tabu_state(&mut self, state: &State) {
        self.tabu.push(hash64(state));

        if self.tabu.len() > MAX_TABU {
            self.tabu.pop();
        }
    }

    pub fn search(&mut self, model: &Model) -> Solution {
        // current time
        let mut t_max = model.max_arrival + 1;

        // the current solution
        let mut solution: Solution = Solution::new();

        // the best solution
        let mut best_solution: Solution = Solution::new();

        // the current state
        let mut next: State = model.initial_state();

        //
        let mut start: usize = 0;

        let mut best_fitness = f64::MAX;

        let mut no_changes = 0;

        for _ in 0..REPEATS {
            best_fitness = f64::MAX;
            for t in start..t_max {
                next = self.find_neighbour(&next.next_null(model), model);

                solution.0.push(next.clone());

                if next.arrived_passengers().len() == model.passengers.len() {
                    break;
                } else if t == t_max {
                    t_max += 1;
                }

                if next.fitness(model) < best_fitness {
                    best_fitness = next.fitness(model);
                    no_changes = 0;
                } else if no_changes > NO_CHANGES_MAX {
                    break;
                } else {
                    no_changes += 1;
                }
            }

            if solution.fitness(model) <= best_solution.fitness(model) {
                best_solution = solution.clone();
            } else {
                solution = best_solution.clone();
            }

            start = rand::thread_rng().gen_range(0..solution.0.len());

            if start == 0 {
                next = model.initial_state();
            } else {
                next = best_solution.0[start - 1].clone().next_null(model);
            }

            solution.0.drain(start..);
        }

        best_solution
    }
}
