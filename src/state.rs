use crate::model::Model;
use crate::move_::Move;
use crate::passenger::{Id as PId, Location as PLocation};
use crate::station::Id as SId;
use crate::train::{Id as TId, Location as TLocation};
use crate::types::{Capacity, Fitness, IdSet, Time, TimeDiff};

use itertools::Itertools;
use std::hash::{Hash, Hasher};

const LAMBDA_DELAY: Fitness = 5.0;
const LAMBDA_NEW_PASSENGERS: Fitness = 0.001;
const LAMBDA_ARRIVAL: Fitness = 0.45;

#[derive(Clone, PartialEq)]
pub struct State {
    pub t: Time,
    pub s_capacity: Vec<Capacity>,
    pub c_capacity: Vec<Capacity>,
    pub t_capacity: Vec<Capacity>,
    pub t_location: Vec<TLocation>,
    pub p_location: Vec<PLocation>,
    pub t_passengers: Vec<IdSet>,
    pub s_passengers: Vec<IdSet>,
    pub p_arrived: IdSet,
    pub p_delays: Vec<TimeDiff>,
    pub new_passengers: Vec<PId>,
    pub moves: Vec<Move>,
}

impl State {
    //

    pub fn new(
        t: Time,
        s_capacity: Vec<Capacity>,
        c_capacity: Vec<Capacity>,
        t_capacity: Vec<Capacity>,
        t_location: Vec<TLocation>,
        p_location: Vec<PLocation>,
        p_delays: Vec<TimeDiff>,
    ) -> State {
        let t_len = t_location.len();
        let s_len = s_capacity.len();
        let p_len = p_location.len();
        let s_passengers: Vec<IdSet> = (0..s_len)
            .map(|s_id| {
                p_location
                    .clone()
                    .iter()
                    .enumerate()
                    .filter(|(_, location)| match location {
                        PLocation::Station(s) => s_id == *s,
                        _ => false,
                    })
                    .map(|(p_id, _)| p_id)
                    .collect()
            })
            .collect();

        State {
            t,
            s_capacity,
            c_capacity,
            t_capacity,
            t_location,
            p_location,
            t_passengers: (0..t_len).map(|_| IdSet::new()).collect(),
            s_passengers,
            p_arrived: IdSet::new(),
            moves: vec![],
            p_delays,
            new_passengers: (0..p_len).collect(),
        }
    }

    pub fn arriving_trains(&self, t: Time, s_id: SId, model: &Model) -> usize {
        self.t_location
            .iter()
            .enumerate()
            .filter(|(t_id, l)| match l {
                TLocation::Connection(c_id, to, t_start) => {
                    if s_id != *to {
                        return false;
                    }

                    let p = ((self.t - *t_start + t) as f64 * model.trains[*t_id].speed)
                        / model.connections[*c_id].distance;

                    p >= 1.0
                }
                _ => false,
            })
            .count()
    }

    pub fn has_station_overload(&self) -> bool {
        self.s_capacity.iter().filter(|&c| *c < 0).count() > 0
    }

    pub fn fitness(&self, model: &Model) -> Fitness {
        0.0
    }

    pub fn next(&mut self, model: &Model) {
        self.t += 1;
        self.moves = vec![];

        // check for arrived trains
        for (t_id, location) in self.t_location.clone().iter().enumerate() {
            if let TLocation::Connection(c_id, s_id, t_start) = location {
                // Zeiteinheiten * Geschwindigkeit >= Streckenlänge
                // https://github.com/informatiCup/informatiCup2022/issues/7

                // progress in percentage
                let p = ((self.t - *t_start) as f64 * model.trains[t_id].speed)
                    / model.connections[*c_id].distance;

                if p >= 1.0 {
                    // todo: store arrived trains?
                    self.t_location[t_id] = TLocation::Station(*s_id);
                    // decrease station capacity
                    self.s_capacity[*s_id] -= 2;
                    // increase connection capacity
                    self.c_capacity[*c_id] += 1;
                }
            }
        }
    }

    /// Gets a list of legal train moves.
    pub fn get_moves(&self, t_id: TId, model: &Model) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        if self.t == 0 {}

        match self.t_location[t_id] {
            // when the train has not started on any station yet, it will be
            // placed
            TLocation::Nothing => {
                moves.append(&mut self.train_starts(t_id));
            }
            // when the train is on a station, 3 types of moves are possible:
            // board, detrain, depart
            TLocation::Station(s_id) => {
                if self.t > 0 {
                    moves.append(&mut self.boardings(t_id, s_id, model));
                    moves.append(&mut self.detrains(t_id, s_id));
                    moves.append(&mut self.departments(t_id, s_id, model));
                }
            }
            // no moves possible when the given train is on a connection.
            TLocation::Connection(_, _, _) => (),
        };

        moves
    }

    pub fn boardings(&self, t_id: TId, s_id: SId, model: &Model) -> Vec<Move> {
        self.s_passengers[s_id]
            .iter()
            .filter(|&p_id| model.passengers[*p_id].size <= self.t_capacity[t_id])
            .map(|p_id| Move::Board((t_id, *p_id, s_id)))
            .collect()
    }

    pub fn detrains(&self, t_id: TId, s_id: SId) -> Vec<Move> {
        self.t_passengers[t_id]
            .iter()
            .map(|p_id| Move::Detrain((t_id, *p_id, s_id)))
            .collect()
    }

    pub fn departments(&self, t_id: TId, s_id: SId, model: &Model) -> Vec<Move> {
        model.station_connections[s_id]
            .iter()
            .filter(|&c_id| self.c_capacity[*c_id] > 0)
            .map(|&c_id| Move::Depart((t_id, (s_id, model.get_destination(s_id, c_id)), c_id)))
            .collect()
    }

    /// gets the available train starts for the train
    pub fn train_starts(&self, t: TId) -> Vec<Move> {
        self.s_capacity
            .iter()
            .enumerate()
            .filter_map(|(s, &capacity)| {
                if capacity > 0 {
                    return Some(Move::TrainStart((t, s)));
                } else {
                    return None;
                }
            })
            .collect()
    }

    /// Make move and apply the corresponding changes to the state.
    pub fn push(&mut self, m: Move, model: &Model) {
        self.moves.push(m);

        match m {
            Move::Board((t_id, p_id, s_id)) => {
                // update train capacity
                self.t_capacity[t_id] -= model.passengers[p_id].size;

                // set passenger location
                self.p_location[p_id] = PLocation::Train(t_id);

                // remember train passengers
                self.t_passengers[t_id].insert(p_id);
                // forget station passenger
                self.s_passengers[s_id].remove(&p_id);

                // remove new passenger
                let index = self.new_passengers.iter().position(|p| *p == p_id).unwrap();
                self.new_passengers.remove(index);
            }
            Move::Detrain((t_id, p_id, s_id)) => {
                // update train capacity
                self.t_capacity[t_id] += model.passengers[p_id].size;

                // set passenger location
                if s_id == model.passengers[p_id].destination {
                    self.p_location[p_id] = PLocation::Arrived;
                    self.p_arrived.insert(p_id);
                    self.p_delays[p_id] = self.t as i32 - model.passengers[p_id].arrival as i32;
                } else {
                    self.p_location[p_id] = PLocation::Station(s_id);
                }

                self.t_passengers[t_id].remove(&p_id);
            }
            Move::Depart((t_id, (from, to), c_id)) => {
                self.t_location[t_id] = TLocation::Connection(c_id, to, self.t);
                self.s_capacity[from] += 1;
                self.c_capacity[c_id] -= 1;
            }
            Move::TrainStart((t, s)) => {
                self.s_capacity[s] -= 1;
                self.t_location[t] = TLocation::Station(s);
            }
            _ => (),
        }
    }

    pub fn pop(&mut self, model: &Model) -> Option<Move> {
        if let Some(m) = self.moves.pop() {
            match m {
                Move::Board((t_id, p_id, s_id)) => {
                    self.t_capacity[t_id] += model.passengers[p_id].size;
                    self.p_location[p_id] = PLocation::Station(s_id);
                    self.t_passengers[t_id].remove(&p_id);
                    self.s_passengers[s_id].insert(p_id);
                    if s_id == model.passengers[p_id].start {
                        self.new_passengers.push(p_id);
                    }
                }
                Move::Detrain((t_id, p_id, s_id)) => {
                    // update train capacity
                    self.t_capacity[t_id] -= model.passengers[p_id].size;
                    self.p_location[p_id] = PLocation::Train(t_id);

                    // set passenger location
                    if s_id == model.passengers[p_id].destination {
                        self.p_arrived.remove(&p_id);
                        self.p_delays[p_id] = model.t_max as TimeDiff;
                    }

                    self.t_passengers[t_id].insert(p_id);
                }
                Move::Depart((t_id, (from, to), c_id)) => {
                    self.t_location[t_id] = TLocation::Station(from);
                    self.s_capacity[from] -= 1;
                    self.c_capacity[c_id] += 1;
                }
                Move::TrainStart((t, s)) => {
                    self.s_capacity[s] += 1;
                    self.t_location[t] = TLocation::Nothing;
                }
                _ => (),
            }

            return Some(m);
        }

        None
    }

    /// Gets the move for the given train.
    pub fn train_move(&self, t_id: TId) -> Option<&Move> {
        self.moves.iter().find(|m| match m {
            Move::Board((t, _, _)) => *t == t_id,
            Move::Detrain((t, _, _)) => *t == t_id,
            Move::Depart((t, _, _)) => *t == t_id,
            Move::TrainStart((t, _)) => *t == t_id,
            _ => false,
        })
    }

    /// Gets the move for the given passenger.
    pub fn passenger_move(&self, p_id: TId) -> Option<&Move> {
        self.moves.iter().find(|m| match m {
            Move::Board((_, p, _)) => *p == p_id,
            Move::Detrain((_, p, _)) => *p == p_id,
            _ => false,
        })
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.hash(state);
        // self.s_capacity.hash(state);
        // self.c_capacity.hash(state);
        // self.t_capacity.hash(state);
        self.t_location.hash(state);
        self.p_location.hash(state);
        self.moves.hash(state);
    }
}
