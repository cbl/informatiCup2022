use crate::connection::Id as CId;
use crate::station::Id as SId;
use crate::types;
use std::fmt;

pub type Id = types::Id;
pub type Speed = f64;
pub type Capacity = types::Capacity;

#[derive(Clone, PartialEq)]
pub enum StartStation {
    Any,
    Station(SId),
}

impl StartStation {
    pub fn to_location(&self) -> Location {
        match self {
            StartStation::Station(s_id) => Location::Station(*s_id),
            StartStation::Any => Location::Nothing,
        }
    }
}

#[derive(Clone, PartialEq, Copy, Hash)]
pub enum Location {
    /// - CId: connection id
    /// - SId: destination id,
    /// - Time: the start time
    Connection(CId, SId, types::Time),
    Station(SId),
    Nothing,
}

impl Location {
    pub fn next_station(&self) -> Option<SId> {
        match self {
            &Location::Connection(_, s_id, _) => Some(s_id),
            &Location::Station(s_id) => Some(s_id),
            _ => None,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Location::Connection(_, _, _) => fmt.write_str("Connection"),
            Location::Station(_) => fmt.write_str("Station"),
            Location::Nothing => fmt.write_str("None"),
        };

        Ok(())
    }
}

#[derive(Clone)]
pub struct Train {
    pub name: String,
    pub start: StartStation,
    pub speed: Speed,
    pub capacity: Capacity,
}

#[derive(Clone, PartialEq)]
pub enum LocationType {
    Nothing,
    Station,
    Connection,
}
