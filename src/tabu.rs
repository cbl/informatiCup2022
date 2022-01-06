use crate::model::Model;
use crate::move_::{Move, None};
use crate::solution::Solution;
use crate::state::State;
use crate::types::TimeDiff;
use fxhash::hash32;
use fxhash::FxBuildHasher;
use linked_hash_set::LinkedHashSet;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Instant;

const STOP_AT_NO_IMPROVEMENTS: i32 = 25000;

/// Tabu-enhanced genetic search.
pub struct TabuGeneticSearch {
    /// A HashSet that holds states that have been visited before. States are
    /// popped when the maximum number of states have been added to the set.
    tabu: LinkedHashSet<u32, FxBuildHasher>,

    /// The maximum number of milli seconds the algorithm list should run.
    max_millis: u128,

    /// The maxmimum number of items in the tabu list.
    tabu_size: usize,

    /// Wether to track fitness or not.
    track_fitness: bool,

    /// A vector containing the best sum of delays fo all iterations.
    pub fitness: Vec<TimeDiff>,

    /// The number of moves that have been checked.
    pub checked_moves: usize,
}

impl TabuGeneticSearch {
    /// Constructs a new TabuGeneticSearch struct.
    pub fn new(max_millis: u128, tabu_size: usize, track_fitness: bool) -> TabuGeneticSearch {
        TabuGeneticSearch {
            tabu: LinkedHashSet::<u32, FxBuildHasher>::default(),
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

        for t_id in 0..(model.used_trains) {
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

                if !self.tabu.contains(&hash32(state)) {
                    best_move = m;
                }

                state.pop(model);
            }

            if let Move::None(_) = best_move {
            } else {
                state.push(best_move, model);

                // add to tabu list
                self.add_to_tabu_list(state);
            }
        }
    }

    /// Add state to tabu list
    fn add_to_tabu_list(&mut self, state: &State) {
        self.tabu.insert(hash32(state));

        if self.tabu.len() > self.tabu_size {
            self.tabu.pop_back();
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
        let mut min_delay = TimeDiff::MAX;

        // the current state
        let mut state: State = model.initial_state();

        // The start time for the next iteration.
        let mut start: usize = 0;

        let mut no_improvements = 0;

        let mut illegal = 0;

        while best_solution.fitness() > 0 || !best_solution.is_legal() {
            while state.t <= model.t_max {
                self.find_neighbour(&mut state, model);
                solution.0.push(state.clone());

                if self.track_fitness {
                    if solution.fitness() < min_delay {
                        min_delay = solution.fitness();
                    }
                    self.fitness.push(min_delay);
                }

                state.next(model);

                if !state.is_legal() || state.p_arrived.len() == model.passengers.len() {
                    break;
                }
            }

            if solution.fitness() < best_solution.fitness() {
                best_solution = solution.clone();
                no_improvements = 0;
            } else {
                solution = best_solution.clone();
                no_improvements += 1;
            }

            if no_improvements > STOP_AT_NO_IMPROVEMENTS {
                break;
            }

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

        (best_solution, start_time.elapsed().as_millis())
    }
}
