use crate::station::Id as SId;
use crate::train::Id as TId;
use crate::types;

pub type Id = types::Id;
pub type Capacity = types::Capacity;
pub type GroupSize = Capacity;
pub type ArrivalTime = types::Time;

#[derive(Clone, PartialEq, Copy, Hash)]
pub enum Location {
    Arrived,
    Train(TId),
    Station(SId),
}

#[derive(Clone, Debug)]
pub struct Passenger {
    pub name: String,
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
