use crate::connection::Name as CName;
use crate::model::Model;
use crate::move_::Move;
use crate::passenger::{Id as PId, Location as PLocation};
use crate::station::Id as SId;
use crate::train::{Id as TId, Location as TLocation};
use crate::types::{Capacity, Fitness, Time, TimeDiff};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

const LAMBDA_DELAY: f64 = 5.0;
const LAMBDA_NEW_PASSENGERS: f64 = 0.001;
const LAMBDA_ARRIVAL: f64 = 0.45;

#[derive(Clone, PartialEq)]
pub struct State {
    pub t: Time,
    pub s_capacity: Vec<Capacity>,
    pub c_capacity: HashMap<CName, Capacity>,
    pub t_capacity: Vec<Capacity>,
    pub t_location: Vec<TLocation>,
    pub p_location: Vec<PLocation>,
    pub t_passengers: Vec<Vec<PId>>,
    pub s_passengers: Vec<Vec<PId>>,
    pub p_arrived: Vec<PId>,
    pub p_delays: Vec<TimeDiff>,
    pub new_passengers: Vec<PId>,
    pub moves: Vec<Move>,
}

impl State {
    //

    pub fn new(
        t: Time,
        s_capacity: Vec<Capacity>,
        c_capacity: HashMap<CName, Capacity>,
        t_capacity: Vec<Capacity>,
        t_location: Vec<TLocation>,
        p_location: Vec<PLocation>,
        p_delays: Vec<TimeDiff>,
    ) -> State {
        let t_len = t_location.len();
        let s_len = s_capacity.len();
        let p_len = p_location.len();
        let s_passengers: Vec<Vec<PId>> = (0..s_len)
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
            t_passengers: (0..t_len).map(|_| vec![]).collect(),
            s_passengers,
            p_arrived: vec![],
            moves: vec![],
            p_delays,
            new_passengers: (0..p_len).collect(),
        }
    }

    pub fn fitness(&self, model: &Model) -> Fitness {
        let mut fitness = 0.0;

        // number of passengers on a station
        // fitness += self
        //     .s_passengers
        //     .iter()
        //     .map(|x| x.len() as Fitness)
        //     .sum::<Fitness>();

        // arrived passengers
        // fitness += (self.p_location.len() - self.p_arrived.len()) as Fitness * 2.0;

        // delays
        fitness += self
            .p_delays
            .iter()
            .map(|delay| *delay as Fitness / model.max_arrival as Fitness)
            .sum::<Fitness>()
            * 0.5;

        fitness += self.p_delays.iter().filter(|d| **d > 0).sum::<i32>() as f64 * LAMBDA_DELAY;

        // add train to station for all passengers that havent moved yet
        for p_id in self.new_passengers.iter() {
            for (t_id, location) in self.t_location.iter().enumerate() {
                match location {
                    TLocation::Station(s_id) => {
                        if *s_id == model.passengers[*p_id].start {
                            fitness += (model.passengers[*p_id].size - self.t_capacity[t_id])
                                as Fitness
                                / model.passengers[*p_id].size as Fitness
                                * LAMBDA_NEW_PASSENGERS
                        }
                    }
                    _ => (),
                }
            }
        }

        // train locations
        for (t_id, location) in self.t_location.iter().enumerate() {
            if let Some(s_id) = location.next_station() {
                for p_id in self.t_passengers[t_id].iter() {
                    fitness -= 1.0
                        / (model.normalize_distance(
                            model.distance(s_id, model.passengers[*p_id].destination),
                        ) + 1.0);

                    fitness += model.normalized_arrival(*p_id) * LAMBDA_ARRIVAL;
                }
            }
        }

        fitness
    }

    /// Gets a list of the passengers that have arrived.
    pub fn arrived_passengers(&self) -> Vec<PId> {
        self.p_arrived.clone()
    }

    pub fn next_null(&self, model: &Model) -> State {
        let mut state = self.clone();

        state.t += 1;
        state.moves = vec![];

        // check for arrived trains
        for (t_id, location) in state.t_location.clone().iter().enumerate() {
            if let TLocation::Connection(c_id, s_id, p_id) = location {
                // Zeiteinheiten * Geschwindigkeit >= Streckenlänge
                // https://github.com/informatiCup/informatiCup2022/issues/7
                if ((*p_id + 1) as f64 * model.trains[t_id].speed)
                    >= model.connections[c_id].distance
                {
                    // todo: store arrived trains?
                    state.t_location[t_id] = TLocation::Station(*s_id);
                    // decrease station capacity
                    state.s_capacity[*s_id] -= 1;
                    // increase connection capacity
                    *state
                        .c_capacity
                        .get_mut(&model.connections[&c_id].name)
                        .unwrap() += 1;
                } else {
                    state.t_location[t_id] = TLocation::Connection(*c_id, *s_id, p_id + 1);
                }
            }
        }

        state
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
            .map(|p_id| Move::Board(t_id, *p_id, s_id))
            .collect()
    }

    pub fn detrains(&self, t_id: TId, s_id: SId) -> Vec<Move> {
        self.t_passengers[t_id]
            .iter()
            .map(|p_id| Move::Detrain(t_id, *p_id, s_id))
            .collect()
    }

    pub fn departments(&self, t_id: TId, s_id: SId, model: &Model) -> Vec<Move> {
        self.c_capacity
            .iter()
            .filter_map(|(c_name, &capacity)| {
                if capacity > 0 {
                    if let Some((&c, _)) = model
                        .connections
                        .iter()
                        .find(|(c_id, connection)| c_id.0 == s_id && connection.name == *c_name)
                    {
                        return Some(Move::Depart(t_id, s_id, c));
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            })
            .collect()
    }

    /// gets the available train starts for the train
    pub fn train_starts(&self, t: TId) -> Vec<Move> {
        self.s_capacity
            .iter()
            .enumerate()
            .filter_map(|(s, &capacity)| {
                if capacity > 0 {
                    return Some(Move::TrainStart(t, s));
                } else {
                    return None;
                }
            })
            .collect()
    }

    /// Make move and apply the corresponding changes to the state.
    pub fn make_move(&mut self, m: Move, model: &Model) {
        self.moves.push(m);

        match m {
            Move::Board(t_id, p_id, s_id) => {
                // update train capacity
                self.t_capacity[t_id] -= model.passengers[p_id].size;

                // set passenger location
                self.p_location[p_id] = PLocation::Train(t_id);

                // remember train passengers
                self.t_passengers[t_id].push(p_id);
                // forget station passenger
                let index = self.s_passengers[s_id]
                    .iter()
                    .position(|p| *p == p_id)
                    .unwrap();
                self.s_passengers[s_id].remove(index);

                // remove new passenger
                let index = self.new_passengers.iter().position(|p| *p == p_id).unwrap();
                self.new_passengers.remove(index);
            }
            Move::Detrain(t_id, p_id, s) => {
                // update train capacity
                self.t_capacity[t_id] += model.passengers[p_id].size;

                // set passenger location
                if s == model.passengers[p_id].destination {
                    self.p_location[p_id] = PLocation::Arrived;
                    self.p_arrived.push(p_id);
                    self.p_delays[p_id] = self.t as i32 - model.passengers[p_id].arrival as i32;
                } else {
                    self.p_location[p_id] = PLocation::Station(s);
                }

                // forget train passenger
                let index = self.t_passengers[t_id]
                    .iter()
                    .position(|p| *p == p_id)
                    .unwrap();
                self.t_passengers[t_id].remove(index);
            }
            Move::Depart(t, s, c) => {
                // set train location
                // todo: check if 0 is right
                if let Some(destination) = model.get_destination(s, c) {
                    self.t_location[t] = TLocation::Connection(c, destination, 0);
                }

                // increase station capacity
                self.s_capacity[s] += 1;

                // decrease connection capacity
                *self
                    .c_capacity
                    .get_mut(&model.connections[&c].name)
                    .unwrap() -= 1;
            }
            Move::TrainStart(t, s) => {
                self.s_capacity[s] -= 1;
                self.t_location[t] = TLocation::Station(s);
            }
        }
    }

    /// Gets the move for the given train.
    pub fn train_move(&self, t_id: TId) -> Option<&Move> {
        self.moves.iter().find(|m| match m {
            Move::Board(t, _, _) => *t == t_id,
            Move::Detrain(t, _, _) => *t == t_id,
            Move::Depart(t, _, _) => *t == t_id,
            Move::TrainStart(t, _) => *t == t_id,
        })
    }

    /// Gets the move for the given passenger.
    pub fn passenger_move(&self, p_id: TId) -> Option<&Move> {
        self.moves.iter().find(|m| match m {
            Move::Board(_, p, _) => *p == p_id,
            Move::Detrain(_, p, _) => *p == p_id,
            _ => false,
        })
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.hash(state);
        self.s_capacity.hash(state);
        for elt in self.c_capacity.clone() {
            elt.hash(state)
        }
        self.t_capacity.hash(state);
        self.t_location.hash(state);
        self.p_location.hash(state);
        self.moves.hash(state);
    }
}
