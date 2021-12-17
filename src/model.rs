use crate::connection::{Connections, Id as CId, Name as CName};
use crate::passenger::{ArrivalTime as PArrivalTime, Location as PLocation, Passenger};
use crate::solution::Solution;
use crate::state::State;
use crate::station::{Id as SId, Station};
use crate::train::{Location as TLocation, Train};
use crate::types::{Capacity, Time};
use std::collections::HashMap;

/// The model struct holds all existing entities and the corresponding meta
/// data. This includes a list of stations, connections, trains and passengers.
#[derive(Clone)]
pub struct Model {
    pub stations: Vec<Station>,
    pub connections: Connections,
    pub trains: Vec<Train>,
    pub passengers: Vec<Passenger>,
}

impl Model {
    /// Gets the latest arrival time of all passengers.
    pub fn latest_arrival(&self) -> PArrivalTime {
        let mut t = 0;

        for p in &self.passengers {
            if p.arrival > t {
                t = p.arrival;
            }
        }

        return t;
    }

    pub fn get_destination(&self, s: SId, c: CId) -> Option<SId> {
        if c.0 == s {
            return Some(c.1);
        } else if c.1 == s {
            return Some(c.0);
        }

        None
    }

    /// Gets the initial state from the model.
    pub fn initial_state(&self) -> State {
        let s_capacity: Vec<Capacity> = self
            .stations
            .clone()
            .into_iter()
            .map(|station| station.capacity)
            .collect::<Vec<Capacity>>();

        let t_capacity = self
            .trains
            .clone()
            .into_iter()
            .map(|train| train.capacity)
            .collect::<Vec<Capacity>>();

        let t_location = self
            .trains
            .clone()
            .into_iter()
            .map(|train| train.start.to_location())
            .collect::<Vec<TLocation>>();

        let p_location = self
            .passengers
            .clone()
            .into_iter()
            .map(|passenger| PLocation::Station(passenger.start))
            .collect::<Vec<PLocation>>();

        let c_capacity = self
            .connections
            .clone()
            .into_iter()
            .map(|(_, connection)| (connection.name, connection.capacity))
            .collect::<HashMap<CName, Capacity>>();

        State::new(
            0,
            s_capacity.clone(),
            c_capacity.clone(),
            t_capacity.clone(),
            t_location.clone(),
            p_location.clone(),
        )
    }
}
