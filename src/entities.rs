use crate::connection::{Connections, Name as CName};
use crate::passenger::{ArrivalTime as PArrivalTime, Location as PLocation, Passenger};
use crate::solution::Solution;
use crate::state::State;
use crate::station::Station;
use crate::train::{Location as TLocation, Train};
use crate::types::{Capacity, Time};
use std::collections::HashMap;

/// The entities struct holds all existing entities and the corresponding meta
/// data. This includes a list of stations, connections, trains and passengers.
#[derive(Clone)]
pub struct Entities {
    pub stations: Vec<Station>,
    pub connections: Connections,
    pub trains: Vec<Train>,
    pub passengers: Vec<Passenger>,
}

impl Entities {
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

    /// Gets the initial solution from the all entities.
    pub fn init_solution(&self) -> Solution {
        let latest_arrival: Time = self.latest_arrival();

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

        Solution(
            (0..latest_arrival + 1)
                .into_iter()
                .map(|t: Time| State {
                    s_capacity: s_capacity.clone(),
                    c_capacity: c_capacity.clone(),
                    t_capacity: t_capacity.clone(),
                    t_location: t_location.clone(),
                    p_location: p_location.clone(),
                })
                .collect::<Vec<State>>(),
        )
    }
}
