use crate::{train, types};
use std::iter::FromIterator;

pub type Id = types::Id;
pub type Speed = f64;
pub type Capacity = types::Capacity;
pub type GroupSize = i32;
pub type ArrivalTime = types::Time;
pub type LocationId = types::OptionalId;

#[derive(Clone)]
pub struct Location {
    pub typ: LocationType,
    pub id: LocationId,
}

pub struct Passenger {
    // id: Id,
    pub name: &'static str,
    pub start: train::Id,
    pub destination: train::Id,
    pub size: GroupSize,
    pub arrival: ArrivalTime,
}

#[derive(Clone)]
pub enum LocationType {
    Nothing,
    Station,
    Train,
    Arrived,
}
