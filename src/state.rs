use crate::connection::Id as CId;
use crate::connection::Name as CName;
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
    fitness: f64,
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
        let fitness = -1.0;

        State {
            t,
            s_capacity,
            c_capacity,
            t_capacity,
            t_location,
            p_location,
            moves,
            fitness,
        }
    }

    pub fn fitness(&self) -> f64 {
        if self.fitness >= 0.0 {
            return self.fitness;
        }

        // TODO

        0.0
    }

    pub fn next_null(&self) -> State {
        State::new(
            self.t + 1,
            self.s_capacity.clone(),
            self.c_capacity.clone(),
            self.t_capacity.clone(),
            self.t_location.clone(),
            self.p_location.clone(),
        )
    }

    pub fn neighbourhood(&self) -> Vec<State> {}

    pub fn boardings(&self) -> Vec<Move> {}

    pub fn detrains(&self) -> Vec<Move> {}

    pub fn departments(&self) -> Vec<Move> {}

    pub fn make_move(&mut self, m: Move, model: Model) {
        match m {
            Move::Board((t, p)) => {
                self.t_capacity -= 
            }
        }
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.hash(state);
        self.s_capacity.hash(state);
        for elt in self.c_capacity {
            elt.hash(state)
        }
        self.t_capacity.hash(state);
        self.t_location.hash(state);
        self.p_location.hash(state);
        self.moves.hash(state);
    }
}

#[derive(PartialEq)]
pub enum Start {
    Station(SId),
    Nothing,
}

#[derive(PartialEq)]
pub enum Boarding {
    Some((SId, TId)),
    Nothing,
}

#[derive(PartialEq)]
pub enum Departure {
    Connection(CId),
    Nothing,
}

#[derive(PartialEq)]
pub enum Detrain {
    Ok,
    Nothing,
}
