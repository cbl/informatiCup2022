use crate::connection::Id as CId;
use crate::passenger::Location as PLocation;
use crate::station::Id as SId;
use crate::types;

pub type Id = types::Id;
pub type Speed = f64;
pub type Capacity = types::Capacity;

/// The time a train has been on a connection.
pub type Progress = usize;

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
    // - CId: connection id
    // - SId: destination id,
    // - Progress: time the train has been on the station
    Connection(CId, SId, Progress),
    Station(SId),
    Nothing,
}

impl Location {
    pub fn is_station(&self) -> bool {
        match self {
            Location::Station(_) => true,
            _ => false,
        }
    }

    pub fn is_connection(&self) -> bool {
        match self {
            Location::Connection(_, _, _) => true,
            _ => false,
        }
    }

    pub fn is_nothing(&self) -> bool {
        match self {
            Location::Nothing => true,
            _ => false,
        }
    }

    pub fn matches_passenger_station(&self, p_location: &PLocation) -> bool {
        match p_location {
            &PLocation::Station(p_s_id) => match self {
                &Location::Station(l_s_id) => p_s_id == l_s_id,
                _ => false,
            },
            _ => false,
        }
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
