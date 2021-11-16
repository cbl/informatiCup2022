use crate::connection::Connections;
use crate::passenger::{ArrivalTime as PArrivalTime, Location as PLocation, Passenger};
use crate::solution::Solution;
use crate::state::State;
use crate::station::Station;
use crate::train::Train;
use crate::types::Time;

/// The entities struct holds all existing entities and the corresponding meta
/// data. This includes a list of stations, connections, trains and passengers.
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

        let s_capacity = self.stations.iter().map(|s| s.capacity).collect();
        let t_capacity = self.trains.iter().map(|t| t.capacity).collect();

        let t_location = self
            .trains
            .iter()
            .map(|train| train.start.to_location())
            .collect();

        let p_location = self
            .passengers
            .iter()
            .map(|passenger| PLocation::Station(passenger.start))
            .collect();

        Solution(
            (0..latest_arrival)
                .into_iter()
                .map(|t: Time| State {
                    s_capacity,
                    t_capacity,
                    t_location,
                    p_location,
                })
                .collect(),
        )
    }
}
