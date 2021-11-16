use crate::passenger::Location as PLocation;
use crate::train::Location as TLocation;
use crate::types::Capacity;

#[derive(Clone)]
pub struct State {
    pub s_capacity: Vec<Capacity>,
    pub t_capacity: Vec<Capacity>,
    pub t_location: Vec<TLocation>,
    pub p_location: Vec<PLocation>,
}

impl State {
    //
}
