use crate::model::Model;
use crate::move_::{Move, None};
use crate::solution::Solution;
use crate::state::State;
use crate::types::{Fitness, Time, TimeDiff};
use fxhash::hash64;
use linked_hash_set::LinkedHashSet;
use rand::seq::SliceRandom;
use rand::Rng;
use std::cmp::max;
use std::ops::Range;
use std::time::Instant;

const STEPS_PER_TEMP: usize = 10;
const COOLING_FACTOR: f64 = 0.96;
const INITIAL_TEMP: f64 = 0.999;
const RANGE: f64 = 0.3;

pub struct TabuSearch {
    tabu: LinkedHashSet<u64>,
    max_millis: u128,
    tabu_size: usize,
    track_fitness: bool,
    pub fitness: Vec<TimeDiff>,
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

    /// Find the best neighbour for the given state
    fn find_neighbour(&mut self, state: &mut State, model: &Model) {
        let mut rnd = rand::thread_rng();

        // list of possible states
        let mut moves: Vec<Move> = vec![];

        let move_none = Move::None(None());

        // the best move
        let mut best_move: Move = move_none;

        for t_id in 0..model.trains.len() {
            moves = state.get_moves(t_id, model);

            if moves.is_empty() {
                continue;
            }

            best_move = move_none;

            // shuffling the moves somehow leads to finding good solutions much
            // faster...
            moves.shuffle(&mut rnd);

            // find neighbour with best cost that is not tabu
            for m in moves.into_iter() {
                self.checked_moves += 1;

                if !m.is_gt(&best_move, state, model) || !m.is_gt(&move_none, state, model) {
                    continue;
                }

                state.push(m, model);

                if !self.tabu.contains(&hash64(state)) {
                    best_move = m;
                }

                state.pop(model);
            }

            if let Move::None(_) = best_move {
            } else {
                state.push(best_move, model);
            }

            // add to tabu list
            self.add_tabu(state);
        }
    }

    /// Add state to tabu list
    fn add_tabu(&mut self, state: &State) {
        self.tabu.insert(hash64(state));

        if self.tabu.len() > self.tabu_size {
            self.tabu.pop_back();
        }
    }

    pub fn search(&mut self, model: &Model) -> (Solution, u128) {
        // random generator
        let mut rnd = rand::thread_rng();

        // the system temperature
        let mut temperature = INITIAL_TEMP;

        // start system time
        let start_time = Instant::now();

        // the current solution
        let mut solution: Solution = Solution::new();

        // the best solution
        let mut best_solution: Solution = Solution::new();
        let mut min_delay = TimeDiff::MAX;

        // the current state
        let mut state: State = model.initial_state();

        //
        let mut start: usize = 0;

        let mut overloads = 0;

        while best_solution.fitness() > 0 {
            while state.t < model.t_max {
                self.find_neighbour(&mut state, model);
                solution.0.push(state.clone());

                if self.track_fitness {
                    if solution.fitness() < min_delay {
                        min_delay = solution.fitness();
                    }
                    self.fitness.push(min_delay);
                }

                state.next(model);

                if state.has_station_overload() {
                    overloads += 1;
                    break;
                    // println!("overloads: {}", overloads);
                    // solution.0.push(state);
                    // return (solution, 0);
                }

                if state.p_arrived.len() == model.passengers.len() {
                    break;
                }
            }

            // return (solution, start_time.elapsed().as_millis());
            // std::process::exit(1);

            if solution.fitness() < best_solution.fitness() {
                best_solution = solution.clone();
            } else {
                solution = best_solution.clone();
            }

            // start = rnd.gen_range(self.get_range(&solution, &temperature));
            start = rnd.gen_range(0..solution.0.len());

            if start == 0 {
                state = model.initial_state();
            } else {
                state.clone_from(&solution.0[start - 1]);
                state.next(model);
            }

            solution.0.drain(start..);

            if self.max_millis < start_time.elapsed().as_millis() {
                break;
            }
        }

        // println!("overloads: {}", overloads);
        // std::process::exit(1);

        (best_solution, start_time.elapsed().as_millis())
    }
}
