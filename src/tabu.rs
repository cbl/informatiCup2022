use crate::model::Model;
use crate::move_::Move;
use crate::solution::Solution;
use crate::state::State;
use crate::types::Fitness;
use fxhash::hash64;
use rand::Rng;
use std::time::Instant;

const NO_CHANGES_MAX: i32 = 15;

pub struct TabuSearch {
    tabu: Vec<u64>,
    max_millis: u128,
    tabu_size: usize,
}

impl TabuSearch {
    pub fn new(max_millis: u128, tabu_size: usize) -> TabuSearch {
        TabuSearch {
            tabu: vec![],
            max_millis,
            tabu_size,
        }
    }

    fn find_neighbour(&mut self, state: &State, model: &Model) -> State {
        // list of possible states
        #[warn(unused_assignments)]
        let mut moves: Vec<Move> = vec![];

        // the best neighbour
        let mut neighbour = state.clone();

        // null state
        #[warn(unused_assignments)]
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

                // println!("{} ({})", m.to_string(model), next.fitness(model));
                // println!("arrived: {:?}", next.p_arrived);
                // println!("delays: {:?}", next.p_delays);

                if next.fitness(model) <= neighbour.fitness(model) {
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

        if self.tabu.len() > self.tabu_size {
            self.tabu.pop();
        }
    }

    pub fn search(&mut self, model: &Model) -> (Solution, u128) {
        // random generator
        let mut rnd = rand::thread_rng();

        // start system time
        let start_time = Instant::now();

        // the current solution
        let mut solution: Solution = Solution::new();

        // the best solution
        let mut best_solution: Solution = Solution::new();

        // the current state
        let mut next: State = model.initial_state();

        //
        let mut start: usize = 0;

        #[warn(unused_assignments)]
        let mut best_fitness = Fitness::MAX;

        let mut no_changes = 0;

        while best_solution.fitness() > 0.0 {
            best_fitness = Fitness::MAX;

            for _ in start..model.t_max {
                next = self.find_neighbour(&next, model);

                solution.0.push(next.clone());

                if next.arrived_passengers().len() == model.passengers.len() {
                    break;
                }

                if next.fitness(model) < best_fitness {
                    best_fitness = next.fitness(model);
                    no_changes = 0;
                } else if no_changes > NO_CHANGES_MAX {
                    break;
                } else {
                    no_changes += 1;
                }

                next = next.next_null(model);
            }

            // std::process::exit(1);

            // remebering best solution by state fitness leads to finding the
            // best solution faster than just checking the solution fitness.
            if solution.state_fitness(model) < best_solution.state_fitness(model) {
                best_solution = solution.clone();
            } else {
                solution = best_solution.clone();
            }

            start = rnd.gen_range(0..solution.0.len());

            if start == 0 {
                next = model.initial_state();
            } else {
                next = solution.0[start - 1].next_null(model);
            }

            solution.0.drain(start..);

            if self.max_millis < start_time.elapsed().as_millis() {
                break;
            }
        }

        (best_solution, start_time.elapsed().as_millis())
    }
}
