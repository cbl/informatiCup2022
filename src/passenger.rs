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

#[derive(Clone)]
pub struct Journey {
    pub locations: Vec<Location>,
}

impl Journey {
    pub fn at(&self, t: types::Time) -> Location {
        return self.locations[t];
    }
}

impl FromIterator<Location> for Journey {
    fn from_iter<I: IntoIterator<Item = Location>>(iter: I) -> Self {
        let mut locations: Vec<Location> = vec![];

        for l in iter {
            locations.push(l);
        }

        Journey { locations }
    }
}
