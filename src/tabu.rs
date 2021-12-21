use crate::model::Model;
use crate::move_::Move;
use crate::solution::Solution;
use crate::state::State;
use crate::types::Fitness;
use fxhash::hash64;
use linked_hash_set::LinkedHashSet;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Instant;

const STEPS_PER_TEMP: usize = 5;
const COOLING_FACTOR: f64 = 0.95;
const INITIAL_TEMP: f64 = 0.999;

pub struct TabuSearch {
    tabu: LinkedHashSet<u64>,
    max_millis: u128,
    tabu_size: usize,
    track_fitness: bool,
    pub fitness: Vec<Fitness>,
    pub checked_moves: usize,
}

impl TabuSearch {
    pub fn new(max_millis: u128, tabu_size: usize, track_fitness: bool) -> TabuSearch {
        TabuSearch {
            tabu: LinkedHashSet::new(),
            fitness: vec![],
            max_millis,
            tabu_size,
            track_fitness,
            checked_moves: 0,
        }
    }

    fn find_neighbour(&mut self, state: &mut State, model: &Model) {
        let mut rnd = rand::thread_rng();

        // list of possible states
        #[warn(unused_assignments)]
        let mut moves: Vec<Move> = vec![];
        let mut best_move: Move = Move::None;

        for t_id in 0..model.trains.len() {
            moves = state.get_moves(t_id, model);

            if moves.is_empty() {
                continue;
            }

            best_move = Move::None;

            // shuffling the moves somehow leads to finding good solutions much
            // faster...
            moves.shuffle(&mut rnd);

            // find neighbour with best cost that is not tabu
            for m in moves.into_iter() {
                state.push(m, model);

                if m.is_gt(&best_move, state, model) && !self.tabu.contains(&hash64(&state)) {
                    best_move = m;
                }

                state.pop(model);

                self.checked_moves += 1;
            }

            if let Move::None = best_move {
            } else {
                state.push(best_move, model);
            }

            // add to tabu list
            self.add_tabu_state(&state);
        }
    }

    fn add_tabu_state(&mut self, state: &State) {
        self.tabu.insert(hash64(state));

        if self.tabu.len() > self.tabu_size {
            self.tabu.pop_back();
            // self.tabu.pop();
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
        let mut min_delay = Fitness::MAX;

        // the current state
        let mut next: State = model.initial_state();

        //
        let mut start: usize = 0;

        let mut temperature = INITIAL_TEMP;

        while best_solution.fitness() > 0.0 {
            for _ in 0..STEPS_PER_TEMP {
                for _ in start..model.t_max {
                    self.find_neighbour(&mut next, model);
                    solution.0.push(next.clone());

                    if self.track_fitness {
                        if solution.fitness() < min_delay {
                            min_delay = solution.fitness();
                        }
                        self.fitness.push(min_delay);
                    }

                    if next.p_arrived.len() == model.passengers.len() {
                        break;
                    }

                    next.next(model);
                }

                if solution.fitness() < best_solution.fitness() {
                    best_solution = solution.clone();
                } else {
                    solution = best_solution.clone();
                }

                let mid = (solution.0.len() as f64 * (1.0 - temperature));
                let a = std::cmp::max(0, (mid - solution.0.len() as f64 * 0.01) as usize);
                let b = std::cmp::min(
                    solution.0.len(),
                    (mid + solution.0.len() as f64 * 0.01) as usize,
                );

                println!("range {}..{} ({}deg)", a, b, temperature,);

                // start = rnd.gen_range(0..solution.0.len());
                start = rnd.gen_range(a..b);

                if start == 0 {
                    next = model.initial_state();
                } else {
                    next.clone_from(&solution.0[start - 1]);
                    next.next(model);
                }

                solution.0.drain(start..);
            }

            if self.max_millis < start_time.elapsed().as_millis() {
                break;
            }

            temperature *= COOLING_FACTOR;
        }

        (best_solution, start_time.elapsed().as_millis())
    }
}
