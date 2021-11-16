use crate::{train, types};
use std::iter::FromIterator;

pub type Id = types::Id;
pub type Speed = f64;
pub type Capacity = types::Capacity;
pub type LocationId = types::OptionalId;
pub type StartStationId = types::OptionalId;

#[derive(Clone)]
pub struct Location {
    pub typ: LocationType,
    pub id: LocationId,
}

pub struct Train {
    // id: Id,
    pub name: &'static str,
    pub start: StartStationId,
    pub speed: Speed,
    pub capacity: Capacity,
}

#[derive(Clone)]
pub enum LocationType {
    Nothing,
    Station,
    Connection,
}
