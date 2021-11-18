use crate::station::Id as SId;
use crate::train::Id as TId;
use crate::types;

pub type Id = types::Id;
pub type Speed = f64;
pub type Capacity = types::Capacity;
pub type GroupSize = i32;
pub type ArrivalTime = types::Time;

#[derive(Clone, PartialEq, Copy)]
pub enum Location {
    Arrived,
    Train(TId),
    Station(SId),
}

impl Location {
    /// Determines whether the passenger location is a station.
    pub fn is_station(&self) -> bool {
        match self {
            Location::Station(_) => true,
            _ => false,
        }
    }

    /// Determines whether the passenger location is a train.
    pub fn is_train(&self) -> bool {
        match self {
            Location::Train(_) => true,
            _ => false,
        }
    }

    /// Determines whether the passenger location is arrived.
    pub fn is_arrived(&self) -> bool {
        match self {
            Location::Arrived => true,
            _ => false,
        }
    }

    pub fn matches_train(&self, t_id: TId) -> bool {
        match self {
            Location::Train(id) => *id == t_id,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Passenger {
    pub name: &'static str,
    pub start: SId,
    pub destination: SId,
    pub size: GroupSize,
    pub arrival: ArrivalTime,
}

#[derive(Clone, PartialEq)]
pub enum LocationType {
    Nothing,
    Station,
    Train,
    Arrived,
}
