use crate::connection::Id as CId;
use crate::connection::Name as CName;
use crate::passenger::Location as PLocation;
use crate::station::Id as SId;
use crate::train::{Id as TId, Location as TLocation};
use crate::types::{Capacity, Time};
use std::collections::HashMap;

#[derive(Clone)]
pub struct State {
    pub s_capacity: Vec<Capacity>,
    pub c_capacity: HashMap<CName, Capacity>,
    pub t_capacity: Vec<Capacity>,
    pub t_location: Vec<TLocation>,
    pub p_location: Vec<PLocation>,
}

impl State {
    //
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
