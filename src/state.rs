use crate::connection::Id as CId;
use crate::connection::Name as CName;
use crate::model::Model;
use crate::move_::Move;
use crate::passenger::{Id as PId, Location as PLocation};
use crate::station::Id as SId;
use crate::train::{Id as TId, Location as TLocation};
use crate::types::{Capacity, Time};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq)]
pub struct State {
    pub t: Time,
    pub s_capacity: Vec<Capacity>,
    pub c_capacity: HashMap<CName, Capacity>,
    pub t_capacity: Vec<Capacity>,
    pub t_location: Vec<TLocation>,
    pub p_location: Vec<PLocation>,
    pub moves: Vec<Move>,
}

impl State {
    //

    pub fn new(
        t: Time,
        s_capacity: Vec<Capacity>,
        c_capacity: HashMap<CName, Capacity>,
        t_capacity: Vec<i32>,
        t_location: Vec<TLocation>,
        p_location: Vec<PLocation>,
    ) -> State {
        let moves = vec![];

        State {
            t,
            s_capacity,
            c_capacity,
            t_capacity,
            t_location,
            p_location,
            moves,
        }
    }

    pub fn fitness(&self, model: &Model) -> f64 {
        let mut fitness = 0.0;

        fitness += (self.p_location.len() - self.arrived_passengers().len()) as f64;

        if self.moves.len() == 0 {
            fitness += 1.0;
        }

        // self.fitness += self.empty_train_departments(model);

        // self.fitness += self.transits(model);

        for m in self.moves.iter() {
            match m {
                Move::Detrain(_, p_id, s_id) => {
                    if *s_id != model.passengers[*p_id].destination {
                        fitness += 1.0;
                    }
                }
                Move::Depart(t_id, _, _) => {
                    if self.t_capacity[*t_id] == model.trains[*t_id].capacity {
                        fitness += 1.0;
                    }
                }
                _ => (),
            }
        }

        fitness
    }

    fn transits(&self, model: &Model) -> f64 {
        self.moves
            .iter()
            .filter(|(&m)| {
                if let Move::Detrain(_, p_id, s_id) = m {
                    return *s_id != model.passengers[*p_id].destination;
                } else {
                    return false;
                }
            })
            .count() as f64
    }

    fn empty_train_departments(&self, model: &Model) -> f64 {
        self.moves
            .iter()
            .filter(|(&m)| {
                if let Move::Depart(t_id, _, _) = m {
                    return self.t_capacity[*t_id] == model.trains[*t_id].capacity;
                } else {
                    return false;
                }
            })
            .count() as f64
    }

    /// Gets a list of the passengers that have arrived.
    pub fn arrived_passengers(&self) -> Vec<PId> {
        self.p_location
            .iter()
            .enumerate()
            .filter_map(|(p_id, location)| match location {
                PLocation::Arrived => Some(p_id),
                _ => None,
            })
            .collect()
    }

    pub fn next_null(&self, model: &Model) -> State {
        let mut state = State::new(
            self.t + 1,
            self.s_capacity.clone(),
            self.c_capacity.clone(),
            self.t_capacity.clone(),
            self.t_location.clone(),
            self.p_location.clone(),
        );

        // check for arrived trains
        for (t, location) in state.t_location.clone().iter().enumerate() {
            if let TLocation::Connection(c, s, p) = location {
                // Zeiteinheiten * Geschwindigkeit >= Streckenlänge
                // https://github.com/informatiCup/informatiCup2022/issues/7
                if ((*p + 1) as f64 * model.trains[t].speed) >= model.connections[c].distance {
                    // todo: store arrived trains?
                    state.t_location[t] = TLocation::Station(*s);
                    // decrease station capacity
                    state.s_capacity[*s] -= 1;
                    // increase connection capacity
                    *state
                        .c_capacity
                        .get_mut(&model.connections[&c].name)
                        .unwrap() += 1;
                } else {
                    state.t_location[t] = TLocation::Connection(*c, *s, p + 1);
                }
            }
        }

        state
    }

    /// Gets the neighbourhood for the given train and model.
    pub fn neighbourhood(&self, t: TId, model: &Model) -> Vec<Move> {
        let mut moves: Vec<Move> = vec![];

        match self.t_location[t] {
            // when the train has not started on any station yet, it will be
            // placed
            TLocation::Nothing => {
                moves.append(&mut self.train_starts(t));
            }
            // when the train is on a station, 3 types of moves are possible:
            // board, detrain, depart
            TLocation::Station(s) => {
                moves.append(&mut self.boardings(t, s, model));
                moves.append(&mut self.detrains(t, s, model));
                moves.append(&mut self.departments(t, s, model));
            }
            // no moves possible when the given train is on a connection.
            TLocation::Connection(_, _, _) => (),
        };

        moves

        // moves
        //     .into_iter()
        //     .map(|m| self.next_null(model).make_move(m, model))
        //     .collect()
    }

    pub fn boardings(&self, t: TId, s: SId, model: &Model) -> Vec<Move> {
        self.p_location
            .iter()
            .enumerate()
            .filter_map(|(p, location)| {
                if let PLocation::Station(p_s) = location {
                    if *p_s == s && model.passengers[p].size <= self.t_capacity[t] {
                        return Some(Move::Board(t, p, s));
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            })
            .collect()
    }

    pub fn detrains(&self, t: TId, s: SId, model: &Model) -> Vec<Move> {
        self.p_location
            .iter()
            .enumerate()
            .filter_map(|(p, location)| {
                if let PLocation::Train(p_t) = location {
                    if *p_t == t {
                        return Some(Move::Detrain(t, p, s));
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            })
            .collect()
    }

    pub fn departments(&self, t: TId, s: SId, model: &Model) -> Vec<Move> {
        self.c_capacity
            .iter()
            .filter_map(|(c_name, &capacity)| {
                if capacity > 0 {
                    if let Some((&c, _)) = model
                        .connections
                        .iter()
                        .find(|(c_id, connection)| c_id.0 == s && connection.name == *c_name)
                    {
                        return Some(Move::Depart(t, s, c));
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

    pub fn is_legal(&self, m: &Move, model: &Model) -> bool {
        match m {
            Move::Board(t, p, s) => {
                if let PLocation::Station(p_s) = self.p_location[*p] {
                    return self.t_capacity[*t] >= model.passengers[*p].size && p_s == *s;
                } else {
                    return false;
                }
            }
            Move::Detrain(t, p, s) => {
                if let PLocation::Train(p_t) = self.p_location[*p] {
                    return self.s_capacity[*s] >= 1 && p_t == *t;
                } else {
                    return false;
                }
            }
            Move::Depart(t, s, c) => {
                if let TLocation::Station(t_s) = self.t_location[*t] {
                    return self.c_capacity[&model.connections[&c].name] >= 1 && t_s == *s;
                } else {
                    return false;
                }
            }
            Move::TrainStart(t, s) => {
                if let TLocation::Nothing = self.t_location[*t] {
                    return self.s_capacity[*s] >= 1;
                } else {
                    return false;
                }
            }
        }
    }

    pub fn make_move(&mut self, m: Move, model: &Model) {
        self.moves.push(m);

        match m {
            Move::Board(t, p, _) => {
                // update train capacity
                self.t_capacity[t] -= model.passengers[p].size;

                // set passenger location
                self.p_location[p] = PLocation::Train(t);
            }
            Move::Detrain(t, p, s) => {
                // update train capacity
                self.t_capacity[t] += model.passengers[p].size;

                // set passenger location
                if s == model.passengers[p].destination {
                    self.p_location[p] = PLocation::Arrived;
                } else {
                    self.p_location[p] = PLocation::Station(s);
                }
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

    pub fn train_move(&self, t_id: TId) -> Option<&Move> {
        self.moves.iter().find(|m| match m {
            Move::Board(t, _, _) => *t == t_id,
            Move::Detrain(t, _, _) => *t == t_id,
            Move::Depart(t, _, _) => *t == t_id,
            Move::TrainStart(t, _) => *t == t_id,
        })
    }

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
